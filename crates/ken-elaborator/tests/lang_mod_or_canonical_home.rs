//! Acceptance pins for `LANG-MOD-OR-CANONICAL-HOME`, implementing the module
//! identity and flat-Sigma rules in `spec/30-surface/33-declarations.md` §3-4.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::ElabEnv;
use ken_kernel::env::Decl;
use ken_kernel::{Level, Term};

const OR_KEN_MD: &str = include_str!("../../../catalog/packages/Core/Logic/Or.ken.md");

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-or-home-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create canonical-Or fixture root");
        Self(path)
    }

    fn write(&self, relative: &str, source: &str) {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent");
        }
        fs::write(path, source).expect("write canonical-Or fixture");
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn applied_head(term: &Term) -> &Term {
    match term {
        Term::App(function, _) => applied_head(function),
        Term::Pi(_, codomain) | Term::Lam(_, codomain) => applied_head(codomain),
        other => other,
    }
}

/// Promise class: normative compatibility vector. The canonical catalog
/// declaration retains the former prelude's exact parameter, result, and
/// constructor-field shape while receiving a newly allocated catalog identity.
///
/// **MEASURED:** the real strict roots loader admits `Core.Logic.Or`, exposes
/// its three public identities, and emits the exact kernel inductive telescope.
/// **CLAIMED:** the catalog family is the field-for-field proof-relevant
/// replacement for the retired prelude family. **THE GAP:** matching names or
/// declaration counts would not establish the sort and field identity; the
/// assertions inspect the emitted `InductiveDecl` by its fresh `GlobalId`.
#[test]
fn core_logic_or_loads_standalone_with_exact_relevant_shape() {
    let mut env = ElabEnv::new().expect("base environment");
    assert!(!env.globals.contains_key("Or"));
    assert!(!env.globals.contains_key("Inl"));
    assert!(!env.globals.contains_key("Inr"));
    let pre_catalog_decls = env.env.decls().count();
    let trusted_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_module_from_roots_strict(&[catalog_root()], "Core.Logic.Or")
        .expect("Core.Logic.Or must load standalone through strict roots");

    let or_id = env.globals["Core.Logic.Or.Or"];
    let inl_id = env.globals["Core.Logic.Or.Inl"];
    let inr_id = env.globals["Core.Logic.Or.Inr"];
    assert!(or_id.0 as usize >= pre_catalog_decls);
    let or_decl = env
        .env
        .inductive(or_id)
        .expect("catalog Or must be an inductive family");
    assert_eq!(or_decl.params, vec![Term::omega(Level::Zero); 2]);
    assert!(or_decl.indices.is_empty());
    assert_eq!(or_decl.level, Level::Zero);
    assert_eq!(or_decl.constructors.len(), 2);
    assert_eq!(or_decl.constructors[0].id, inl_id);
    assert_eq!(or_decl.constructors[0].args, vec![Term::var(1)]);
    assert_eq!(or_decl.constructors[1].id, inr_id);
    assert_eq!(or_decl.constructors[1].args, vec![Term::var(0)]);
    assert_eq!(
        trusted_before,
        env.env.trusted_base().into_iter().collect(),
        "ordinary catalog data and exports add no trusted declaration",
    );
}

/// Promise class: durable invariant. Selective imports preserve one provider
/// identity across independent units and do not recreate ambient bare names.
///
/// **MEASURED:** every independently roots-loaded checked-source consumer unit
/// emits a witness type and body whose applied heads are the same provider family
/// and constructor `GlobalId`s. **CLAIMED:** every legal selective-import edge
/// reuses one catalog `Or` identity. **THE GAP:** successful imports alone could
/// hide per-consumer copies; comparing the emitted heads to the provider
/// identities rules those copies out, while alternating constructors exercises
/// both public exports.
#[test]
fn every_selective_consumer_reuses_one_catalog_global_identity() {
    let root = FixtureRoot::new("all-consumers");
    root.write("Core/Logic/Or.ken.md", OR_KEN_MD);
    let consumers = [
        "Core.Classes.LawfulClasses",
        "Core.Logic.EmptyDec",
        "Data.Collections.Derived",
        "Data.Collections.Map",
        "Capability.Formatting.Doc",
        "Data.Numeric.Nat.Order",
        "Tooling.Verification.FoKripke",
    ];
    for (index, module) in consumers.iter().enumerate() {
        let relative = format!("{}.ken", module.replace('.', "/"));
        let witness = if index % 2 == 0 {
            "pub fn witness (x : Bool) : Or (Eq Bool x x) (Eq Bool x x) = \
             Inl (Eq Bool x x) (Eq Bool x x) Refl"
        } else {
            "pub fn witness (x : Bool) : Or (Eq Bool x x) (Eq Bool x x) = \
             Inr (Eq Bool x x) (Eq Bool x x) Refl"
        };
        root.write(
            &relative,
            &format!("import Core.Logic.Or (Or, Inl, Inr)\n{witness}"),
        );
    }

    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for module in consumers {
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], module)
            .unwrap_or_else(|error| panic!("strict consumer {module} failed: {error}"));
    }

    let or_id = env.globals["Core.Logic.Or.Or"];
    let or_decl = env.env.inductive(or_id).expect("provider Or family");
    let inl_id = or_decl.constructors[0].id;
    let inr_id = or_decl.constructors[1].id;
    for (index, module) in consumers.iter().enumerate() {
        let witness_id = env.globals[&format!("{module}.witness")];
        let Decl::Transparent { ty, body, .. } = env
            .env
            .lookup(witness_id)
            .expect("consumer witness declaration")
        else {
            panic!("consumer witness must be transparent");
        };
        assert!(matches!(applied_head(ty), Term::IndFormer { id, .. } if *id == or_id));
        let expected_ctor = if index % 2 == 0 { inl_id } else { inr_id };
        assert!(matches!(applied_head(body), Term::Constructor { id, .. } if *id == expected_ctor));
    }
    assert!(!env.globals.contains_key("Or"));
    assert!(!env.globals.contains_key("Inl"));
    assert!(!env.globals.contains_key("Inr"));
    assert_eq!(trusted_before, env.env.trusted_base().into_iter().collect());
}
