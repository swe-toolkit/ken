//! Behavioral acceptance pins for `LANG-MOD-CATALOG-REALIZATION` Component A.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Term;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ken-catalog-realization-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("create catalog-realization fixture root");
        Self(root)
    }

    fn write(&self, relative: &str, source: &str) {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().expect("fixture path has a parent"))
            .expect("create fixture module directories");
        fs::write(path, source).expect("write catalog-realization fixture source");
    }

    fn roots(&self) -> Vec<PathBuf> {
        vec![self.0.clone()]
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn assert_alias_reuses(env: &ElabEnv, alias: &str, provider: &str) {
    let provider_id = env.globals[provider];
    let (_, body) = env
        .env
        .transparent_body(env.globals[alias])
        .unwrap_or_else(|| panic!("{alias} must be a transparent imported alias"));
    assert!(
        matches!(body, Term::Const { id, .. } if id == provider_id),
        "{alias} must reuse the exact provider GlobalId for {provider}"
    );
}

fn assert_body_uses_constructor(env: &ElabEnv, value: &str, provider: &str) {
    fn head_constructor(term: &Term) -> Option<ken_kernel::GlobalId> {
        match term {
            Term::Constructor { id, .. } => Some(*id),
            Term::App(function, _) => head_constructor(function),
            _ => None,
        }
    }

    let (_, body) = env
        .env
        .transparent_body(env.globals[value])
        .unwrap_or_else(|| panic!("{value} must have a transparent body"));
    assert_eq!(
        head_constructor(&body),
        Some(env.globals[provider]),
        "{value} must construct with the exact provider identity {provider}"
    );
}

/// Promise class: durable invariant. Component A's dependency-closed slice
/// loads standalone-strict without adding trust, while floor Nat resolves to
/// the compiler-installed family and constructors rather than a replacement.
///
/// **MEASURED:** strict roots elaborates each real selected module and a Nat
/// fixture, preserves the trusted-base set, and the fixture bodies use the
/// pre-existing `Nat` constructor ids. **CLAIMED:** the specified Nat-floor
/// inversion admits canonical identity without minting or widening beyond the
/// closed floor. **THE GAP:** whole-floor closure and the non-member reject live
/// in `lang_mod_nat_floor_realization`.
#[test]
fn strict_slice_stays_closed_and_floor_nat_reuses_existing_identity() {
    for module in [
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Tooling.Verification.ProofErasureBoundaryChecker",
    ] {
        let mut env = ElabEnv::new().expect("base environment");
        let trusted_before = env.env.trusted_base();
        env.elaborate_module_from_roots_strict(&[catalog_root()], module)
            .unwrap_or_else(|error| panic!("{module} must load standalone-strict: {error}"));
        assert_eq!(
            env.env.trusted_base(),
            trusted_before,
            "{module} must preserve the flat program's trusted base"
        );
    }

    let root = FixtureRoot::new("strict-floor-nat");
    root.write(
        "StrictNat.ken",
        "const zero : Nat = Zero\nconst one : Nat = Suc Zero",
    );
    let mut env = ElabEnv::new().expect("base environment");
    let nat = env.globals["Nat"];
    let zero = env.globals["Zero"];
    let suc = env.globals["Suc"];
    let declaration_count = env.env.declarations().len();
    let next_id = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();

    env.elaborate_module_from_roots_strict(&root.roots(), "StrictNat")
        .expect("strict roots must admit the canonical Nat floor identity");

    assert_eq!(env.globals["Nat"], nat);
    assert_eq!(env.globals["Zero"], zero);
    assert_eq!(env.globals["Suc"], suc);
    assert!(!env.globals.contains_key("StrictNat.Nat"));
    assert_body_uses_constructor(&env, "StrictNat.zero", "Zero");
    assert_body_uses_constructor(&env, "StrictNat.one", "Suc");
    assert_eq!(env.env.declarations().len(), declaration_count + 2);
    assert_eq!(env.env.next_global_id().0, next_id.0 + 2);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

/// Promise class: durable invariant. A real explicit-data provider publishes
/// its own constructors under legacy roots, and repeated reachability of those
/// same identities is idempotent.
///
/// **MEASURED:** canonical Or is loaded directly through legacy roots, then a
/// consumer imports its interface twice and aliases both constructors to their
/// provider GlobalIds. **CLAIMED:** a unit's own explicit-data constructors are
/// in local scope independently of external-fallback mode, without identity
/// duplication. **THE GAP:** ordinary-data and class declarations have their
/// own structural controls below.
#[test]
fn canonical_or_publishes_constructors_under_legacy_roots_idempotently() {
    const OR: &str = include_str!("../../../catalog/packages/Core/Logic/Or.ken.md");

    let root = FixtureRoot::new("legacy-or");
    root.write("Core/Logic/Or.ken.md", OR);
    root.write(
        "Entry.ken",
        "import Core.Logic.Or (Or, Inl, Inr)\n\
         import Core.Logic.Or (Or, Inl, Inr)\n\
         const left : Or (Equal Bool True True) (Equal Bool False False) =\n\
           Inl (Equal Bool True True) (Equal Bool False False) Proved\n\
         const right : Or (Equal Bool True True) (Equal Bool False False) =\n\
           Inr (Equal Bool True True) (Equal Bool False False) Proved",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots(&root.roots(), "Core.Logic.Or")
        .expect("real canonical Or must load directly through legacy roots");
    env.elaborate_module_from_roots(&root.roots(), "Entry")
        .expect("the same canonical Or interface must be import-idempotent");

    assert_body_uses_constructor(&env, "Entry.left", "Core.Logic.Or.Inl");
    assert_body_uses_constructor(&env, "Entry.right", "Core.Logic.Or.Inr");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

/// Promise class: durable invariant. Legacy scope formation covers ordinary
/// data constructors and class locals, not only explicit-data constructors.
///
/// **MEASURED:** a roots-loaded ordinary data unit exports its constructor, a
/// roots-loaded class unit exports and consumes its class identity, and a class
/// declaration colliding with the unshadowable prelude floor is rejected as an
/// ambiguous local. **CLAIMED:** all three ruled declaration kinds participate
/// in mode-independent local binding. **THE GAP:** canonical Or above supplies
/// the separate explicit-data witness.
#[test]
fn legacy_local_scope_covers_ordinary_data_and_classes() {
    let root = FixtureRoot::new("legacy-declaration-kinds");
    root.write(
        "LegacyData.ken",
        "data LegacyData = LegacyCtor\nexport LegacyData, LegacyCtor",
    );
    root.write(
        "LegacyClass.ken",
        "pub class LegacyClass a { keep : a -> a }",
    );
    root.write(
        "Entry.ken",
        "import LegacyData (LegacyData, LegacyCtor)\n\
         import LegacyClass (LegacyClass)\n\
         const made : LegacyData = LegacyCtor\n\
         fn keep_class (a : Type) (d : LegacyClass a) : LegacyClass a = d",
    );

    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&root.roots(), "Entry")
        .expect("legacy roots must bind ordinary-data constructors and class locals");
    assert_body_uses_constructor(&env, "Entry.made", "LegacyData.LegacyCtor");

    let error = ElabEnv::new()
        .expect("base environment")
        .elaborate_file("class Bool a { keep : a -> a }")
        .expect_err("a distinct local class must not shadow the prelude Bool identity");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "Bool"),
        "class/prelude collision must fail closed as AmbiguousReference, got {error:?}"
    );
}

/// Promise class: durable invariant. Different provider identities with the
/// same imported spelling remain ambiguous after local-scope repair.
///
/// **MEASURED:** either provider imports and executes alone, while importing
/// both produces `AmbiguousReference` for the shared name. **CLAIMED:** the
/// idempotence admitted above is identity-based, not spelling-based. **THE
/// GAP:** this controls distinct import identities; the prelude/local collision
/// direction is controlled by the class case above.
#[test]
fn distinct_import_identities_with_one_spelling_still_collide() {
    let root = FixtureRoot::new("distinct-import-collision");
    root.write("A.ken", "pub const item : Bool = True");
    root.write("B.ken", "pub const item : Bool = False");
    root.write("OnlyA.ken", "import A (item)\nconst selected : Bool = item");
    root.write("OnlyB.ken", "import B (item)\nconst selected : Bool = item");
    root.write(
        "Collision.ken",
        "import A (item)\nimport B (item)\nconst selected : Bool = item",
    );

    for entry in ["OnlyA", "OnlyB"] {
        ElabEnv::new()
            .expect("base environment")
            .elaborate_module_from_roots(&root.roots(), entry)
            .unwrap_or_else(|error| panic!("{entry} positive control must load: {error}"));
    }

    let error = ElabEnv::new()
        .expect("base environment")
        .elaborate_module_from_roots(&root.roots(), "Collision")
        .expect_err("different provider identities must not share one bare import");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "item"),
        "distinct import collision must name item, got {error:?}"
    );
}

/// Promise class: durable invariant. Real Arithmetic exposes `add` and `mul`
/// through the legacy roots loader without minting replacement identities.
///
/// **MEASURED:** a consumer selectively imports the two names from the real
/// provider and each transparent consumer body is the provider's GlobalId.
/// **CLAIMED:** the Arithmetic surface is public, fully elaborable, flat-Sigma,
/// identity-preserving, and zero-trust under the loader Component A ships.
/// **THE GAP:** Order's provider surface belongs to Component B because its
/// closure requires the canonical `OrdResult` home.
#[test]
fn arithmetic_operations_are_public_by_exact_provider_identity() {
    const ARITHMETIC: &str =
        include_str!("../../../catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md");
    const TRANSPORT: &str = include_str!("../../../catalog/packages/Core/Logic/Transport.ken.md");

    let root = FixtureRoot::new("arithmetic-identities");
    root.write("Data/Numeric/Nat/Arithmetic.ken.md", ARITHMETIC);
    root.write("Core/Logic/Transport.ken.md", TRANSPORT);
    root.write(
        "Entry.ken",
        "import Data.Numeric.Nat.Arithmetic (add, mul)\n\
         const imported_add : Nat -> Nat -> Nat = add\n\
         const imported_mul : Nat -> Nat -> Nat = mul",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots(&root.roots(), "Entry")
        .expect("real Arithmetic must be selectively importable through legacy roots");

    assert_alias_reuses(
        &env,
        "Entry.imported_add",
        "Data.Numeric.Nat.Arithmetic.add",
    );
    assert_alias_reuses(
        &env,
        "Entry.imported_mul",
        "Data.Numeric.Nat.Arithmetic.mul",
    );
    assert_eq!(env.env.trusted_base(), trusted_before);
}
