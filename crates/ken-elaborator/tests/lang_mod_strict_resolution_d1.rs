//! LANG-MOD-STRICT-RESOLUTION D1 strict-mode enforcement fixtures.
//!
//! The existing roots entry remains legacy until WP-4. These fixtures opt in
//! through `elaborate_module_from_roots_strict` and exercise the two resolver
//! chokes established by the D0 representation inventory.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Level, Term};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-strict-d1-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create D1 fixture root");
        Self(path)
    }

    fn write(&self, relative: &str, source: &str) {
        fs::write(self.0.join(relative), source).expect("write D1 fixture");
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

fn assert_unbound(error: ElabError, expected: &str) {
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. } if name == expected),
        "strict resolver must reject `{expected}` as UnboundName, found {error:?}"
    );
}

fn ambient_globals() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("ambient_value", int)
        .expect("declare ambient value");
    env.declare_postulate_raw("AmbientType", Term::ty(Level::Zero))
        .expect("declare ambient type");
    let true_id = env.globals["True"];
    env.globals.insert("AmbientTrue".to_string(), true_id);
    env.elaborate_file(
        "fn ambient_id (x : Int) : Int = x \
         proof stable for ambient_id (x : Int) : Eq Int (ambient_id x) x = Refl",
    )
    .expect("declare ambient attached-proof subject and proof");
    env
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum StrictRoute {
    ExprCon,
    TypeConElab,
    TypeConData,
    PatternCtor,
    AttachedDecl,
    AttachedExpr,
    ExportInScope,
    DeriveClass,
    DeriveData,
    InstanceClass,
    InstanceConstraint,
    ViewConstraint,
}

const GLOBAL_ROUTES: [StrictRoute; 7] = [
    StrictRoute::ExprCon,
    StrictRoute::TypeConElab,
    StrictRoute::TypeConData,
    StrictRoute::PatternCtor,
    StrictRoute::AttachedDecl,
    StrictRoute::AttachedExpr,
    StrictRoute::ExportInScope,
];

const CLASS_ROUTES: [StrictRoute; 5] = [
    StrictRoute::DeriveClass,
    StrictRoute::DeriveData,
    StrictRoute::InstanceClass,
    StrictRoute::InstanceConstraint,
    StrictRoute::ViewConstraint,
];

#[test]
fn strict_globals_choke_covers_every_d0_globals_representation() {
    let root = FixtureRoot::new("globals-routes");
    root.write("Expr.ken", "const leaked : Int = ambient_value");
    root.write(
        "Type.ken",
        "fn identity (x : AmbientType) : AmbientType = x",
    );
    root.write("Data.ken", "data Boxed = Box AmbientType");
    root.write(
        "Pattern.ken",
        "fn inspect (x : Bool) : Bool = match x { AmbientTrue |-> True ; False |-> False }",
    );
    root.write("Export.ken", "export ambient_value");
    root.write(
        "AttachedDecl.ken",
        "proof rooted for ambient_id (x : Int) : Eq Int (ambient_id x) x = Refl",
    );
    root.write(
        "AttachedExpr.ken",
        "theorem consume (x : Int) : Eq Int x x = ambient_id::stable x",
    );

    let cases = [
        ("Expr", "ambient_value", StrictRoute::ExprCon),
        ("Type", "AmbientType", StrictRoute::TypeConElab),
        ("Data", "AmbientType", StrictRoute::TypeConData),
        ("Pattern", "AmbientTrue", StrictRoute::PatternCtor),
        ("AttachedDecl", "ambient_id", StrictRoute::AttachedDecl),
        ("AttachedExpr", "ambient_id", StrictRoute::AttachedExpr),
        ("Export", "ambient_value", StrictRoute::ExportInScope),
    ];
    let mut covered = BTreeSet::new();
    for (entry, name, route) in cases {
        let mut strict = ambient_globals();
        let error = strict
            .elaborate_module_from_roots_strict(&[root.0.clone()], entry)
            .expect_err("strict roots must reject ambient globals");
        assert_unbound(error, name);
        covered.insert(route);

        let mut legacy = ambient_globals();
        legacy
            .elaborate_module_from_roots(&[root.0.clone()], entry)
            .unwrap_or_else(|error| panic!("legacy {entry} route remains live: {error}"));
    }
    assert_eq!(covered, GLOBAL_ROUTES.into_iter().collect());
}

fn ambient_classes() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "class AmbientDerive a {} \
         class AmbientInstance a {} \
         class AmbientConstraint a {} \
         data AmbientData = MkAmbientData",
    )
    .expect("declare ambient class/data family");
    env
}

#[test]
fn strict_class_choke_covers_every_d0_direct_class_representation() {
    let root = FixtureRoot::new("class-routes");
    root.write("DeriveClass.ken", "derive AmbientDerive for Bool");
    root.write(
        "DeriveData.ken",
        "class LocalClass a {} derive LocalClass for AmbientData",
    );
    root.write(
        "InstanceClass.ken",
        "data Local = MkLocal instance AmbientInstance Local {}",
    );
    root.write(
        "InstanceConstraint.ken",
        "class LocalInstance a {} data Local = MkLocal \
         instance LocalInstance Local where (d : AmbientConstraint Local) {}",
    );
    root.write(
        "ViewConstraint.ken",
        "fn constrained (x : Bool) : Bool where AmbientView Bool = x",
    );
    root.write(
        "ViewProvider.ken",
        "class AmbientView a {} instance AmbientView Bool {}",
    );

    let cases = [
        ("DeriveClass", "AmbientDerive", StrictRoute::DeriveClass),
        ("DeriveData", "AmbientData", StrictRoute::DeriveData),
        (
            "InstanceClass",
            "AmbientInstance",
            StrictRoute::InstanceClass,
        ),
        (
            "InstanceConstraint",
            "AmbientConstraint",
            StrictRoute::InstanceConstraint,
        ),
        ("ViewConstraint", "AmbientView", StrictRoute::ViewConstraint),
    ];
    let mut covered = BTreeSet::new();
    for (entry, name, route) in cases {
        let mut strict = ambient_classes();
        if entry == "ViewConstraint" {
            strict
                .elaborate_module_from_roots_strict(&[root.0.clone()], "ViewProvider")
                .expect("strict provider setup");
        }
        let error = strict
            .elaborate_module_from_roots_strict(&[root.0.clone()], entry)
            .expect_err("strict roots must reject ambient class/data names");
        assert_unbound(error, name);
        covered.insert(route);

        let mut legacy = ambient_classes();
        if entry == "ViewConstraint" {
            legacy
                .elaborate_module_from_roots(&[root.0.clone()], "ViewProvider")
                .expect("legacy provider setup");
        }
        legacy
            .elaborate_module_from_roots(&[root.0.clone()], entry)
            .unwrap_or_else(|error| panic!("legacy {entry} class route remains live: {error}"));
    }
    assert_eq!(covered, CLASS_ROUTES.into_iter().collect());
}

#[test]
fn strict_admits_local_counterparts_for_globals_and_class_representations() {
    let root = FixtureRoot::new("local-counterparts");
    root.write(
        "Entry.ken",
        "data Local = MkLocal \
         data Boxed = Box Local \
         const local_value : Int = 0 \
         fn local_id (x : Int) : Int = x \
         proof stable for local_id (x : Int) : Eq Int (local_id x) x = Refl \
         theorem consume (x : Int) : Eq Int (local_id x) x = local_id::stable x \
         fn type_id (x : Local) : Local = x \
         fn inspect_local (x : Local) : Bool = match x { MkLocal |-> True } \
         fn inspect (x : Bool) : Bool = match x { True |-> True ; False |-> False } \
         class LocalClass a {} \
         class Need a {} \
         instance Need Bool {} \
         derive LocalClass for Local \
         instance LocalClass Local where (d : Need Bool) {} \
         fn constrained (x : Bool) : Bool where Need Bool = x \
         export local_value",
    );
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("each strict representation accepts its local/floor counterpart");
}

#[test]
fn strict_admits_floor_kernel_locals_and_explicit_imports_without_new_trust() {
    let root = FixtureRoot::new("admitted");
    root.write(
        "Provider.ken",
        "pub const supplied : Int = 0 pub class SuppliedClass a {}",
    );
    root.write(
        "Entry.ken",
        "import Provider (supplied, SuppliedClass) \
         class LocalClass a {} \
         data Local = MkLocal \
         derive LocalClass for Local \
         instance SuppliedClass Local {} \
         const copied : Int = supplied \
         fn bool_id (x : Bool) : Bool = x \
         theorem reflexive (x : Bool) : Eq Bool x x = Refl \
         fn cast (a : Type) (b : Type) (e : Eq Type a b) (x : a) : b = \
           J (λb' _. b') x e",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("strict mode admits import, local, floor, and kernel vocabulary");
    assert_eq!(env.env.trusted_base(), trusted_before);

    let provider = env.globals["Provider.supplied"];
    let (_, copied) = env
        .env
        .transparent_body(env.globals["Entry.copied"])
        .expect("strict imported value is transparent");
    assert!(matches!(copied, Term::Const { id, .. } if id == provider));
}

#[test]
fn one_roots_run_cannot_mix_legacy_and_strict_cache_entries() {
    let root = FixtureRoot::new("mode-cache");
    root.write("Entry.ken", "fn bool_id (x : Bool) : Bool = x");
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root.0.clone()], "Entry")
        .expect("legacy roots entry works");
    assert!(matches!(
        env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry"),
        Err(ElabError::Internal(_))
    ));

    assert!(root.path().exists());
}
