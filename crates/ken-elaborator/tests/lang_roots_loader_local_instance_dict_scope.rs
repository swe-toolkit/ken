//! LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE acceptance.
//!
//! Promise class: durable invariants. These tests exercise the roots loader's
//! one synthesized-dictionary predicate at its local, export, and selective-
//! import faces. They compare resolved terms with the synthesis-installed
//! canonical `GlobalId`; no source-text shape is an oracle.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-roots-instance-dict-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create roots dictionary fixture");
        Self(path)
    }

    fn write(&self, relative: &str, source: &str) {
        fs::write(self.0.join(relative), source).expect("write roots dictionary fixture");
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

fn transparent_body(env: &ElabEnv, name: &str) -> Term {
    env.env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("{name} must be a transparent definition"))
        .1
}

fn assert_is_exact_const(term: &Term, expected: GlobalId) {
    assert!(
        matches!(term, Term::Const { id, .. } if *id == expected),
        "resolved dictionary must be the exact synthesis-installed GlobalId, found {term:?}"
    );
}

fn term_mentions_global(term: &Term, expected: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == expected =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == expected => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions_global(child, expected)),
    }
}

fn assert_unbound(error: ElabError, expected: &str) {
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. } if name == expected),
        "strict roots must reject unimported `{expected}` as UnboundName, found {error:?}"
    );
}

#[test]
fn strict_same_unit_instance_and_derive_names_select_the_synthesized_canonicals() {
    let root = FixtureRoot::new("same-unit");
    root.write("Classes.ken", "pub class C a {} pub class Derived a {}");
    root.write(
        "Entry.ken",
        "import Classes (C, Derived) \
         data Local = MkLocal \
         instance C Local {} \
         derive Derived for Local \
         theorem selected_instance : C Local = C_instance_Local \
         theorem selected_derive : Derived Local = Derived_instance_Local",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let trust_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("strict roots bind both generated dictionary names in their own unit");

    let instance = env.globals["C_instance_Entry.Local"];
    let derived = env.globals["Derived_instance_Entry.Local"];
    assert_is_exact_const(&transparent_body(&env, "Entry.selected_instance"), instance);
    assert_is_exact_const(&transparent_body(&env, "Entry.selected_derive"), derived);
    assert!(!env.globals.contains_key("C_instance_Local"));
    assert!(!env.globals.contains_key("Derived_instance_Local"));
    assert_eq!(env.env.trusted_base(), trust_before);
}

#[test]
fn same_file_module_import_is_available_before_just_in_time_dictionary_binding() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module Classes { pub class C a {} } \
         import Classes (C) \
         data Local = MkLocal \
         instance C Local {} \
         theorem selected : C Local = C_instance_Local",
    )
    .expect("the ordered module pass supplies the local-module class import");

    let canonical = env.globals["C_instance_Local"];
    assert_is_exact_const(&transparent_body(&env, "selected"), canonical);
}

#[test]
fn imported_head_uses_its_qualified_synthesis_identity_and_exports_that_identity() {
    let root = FixtureRoot::new("imported-head");
    root.write("Types.ken", "pub data Remote = MkRemote");
    root.write(
        "Owner.ken",
        "import Types (Remote) \
         class C a {} \
         pub class Derived a {} \
         instance C Remote {} \
         derive Derived for Bool \
         theorem local_selected : C Remote = C_instance_Remote \
         theorem local_derived : Derived Bool = Derived_instance_Bool \
         export C",
    );
    root.write(
        "Consumer.ken",
        "import Owner (C, Derived, C_instance_Remote, Derived_instance_Bool) \
         import Types (Remote) \
         theorem imported_selected : C Remote = C_instance_Remote \
         theorem imported_derived : Derived Bool = Derived_instance_Bool",
    );

    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Consumer")
        .expect("qualified generated identity is locally bound, exported, and imported");

    let canonical = env.globals["C_instance_Types.Remote"];
    let derived = env.globals["Derived_instance_Bool"];
    assert_is_exact_const(&transparent_body(&env, "Owner.local_selected"), canonical);
    assert_is_exact_const(&transparent_body(&env, "Owner.local_derived"), derived);
    assert_is_exact_const(
        &transparent_body(&env, "Consumer.imported_selected"),
        canonical,
    );
    assert_is_exact_const(
        &transparent_body(&env, "Consumer.imported_derived"),
        derived,
    );
    assert!(!env.globals.contains_key("C_instance_Remote"));
}

#[test]
fn selective_import_supports_explicit_argument_and_superclass_wiring() {
    let root = FixtureRoot::new("cross-module-shapes");
    root.write(
        "Provider.ken",
        "pub class C (f : Type -> Type) {} instance C Option {}",
    );
    root.write(
        "Consumer.ken",
        "import Provider (C, C_instance_Option) \
         fn accept (d : C Option) : Int = 0 \
         const explicit : Int = accept C_instance_Option \
         class Child (f : Type -> Type) { parent : C f } \
         instance Child Option { parent = C_instance_Option }",
    );

    let mut strict = ElabEnv::new().expect("strict environment");
    let strict_trust = strict.env.trusted_base();
    let strict_ids = strict
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Consumer")
        .expect("both cross-module dictionary use shapes resolve under strict roots");
    let dictionary = strict.globals["C_instance_Option"];
    assert!(term_mentions_global(
        &transparent_body(&strict, "Consumer.explicit"),
        dictionary
    ));
    assert!(term_mentions_global(
        &transparent_body(&strict, "Child_instance_Option"),
        dictionary
    ));
    assert_eq!(strict.env.trusted_base(), strict_trust);

    let mut legacy = ElabEnv::new().expect("legacy environment");
    let legacy_trust = legacy.env.trusted_base();
    let legacy_ids = legacy
        .elaborate_module_from_roots(&[root.0.clone()], "Consumer")
        .expect("the byte-equivalent legacy roots path remains green");
    assert_eq!(legacy_ids, strict_ids);
    assert_eq!(
        transparent_body(&legacy, "Consumer.explicit"),
        transparent_body(&strict, "Consumer.explicit")
    );
    assert_eq!(
        transparent_body(&legacy, "Child_instance_Option"),
        transparent_body(&strict, "Child_instance_Option")
    );
    assert_eq!(legacy.env.trusted_base(), legacy_trust);
}

#[test]
fn both_cross_module_shapes_require_the_dictionary_import() {
    let root = FixtureRoot::new("missing-import");
    root.write(
        "Provider.ken",
        "pub class C (f : Type -> Type) {} instance C Option {}",
    );
    root.write(
        "MissingExplicit.ken",
        "import Provider (C) \
         fn accept (d : C Option) : Int = 0 \
         const explicit : Int = accept C_instance_Option",
    );
    root.write(
        "MissingSuperclass.ken",
        "import Provider (C) \
         class Child (f : Type -> Type) { parent : C f } \
         instance Child Option { parent = C_instance_Option }",
    );

    for entry in ["MissingExplicit", "MissingSuperclass"] {
        let mut env = ElabEnv::new().expect("base environment");
        let error = env
            .elaborate_module_from_roots_strict(&[root.0.clone()], entry)
            .expect_err("loading a provider is not an implicit dictionary import");
        assert_unbound(error, "C_instance_Option");
    }
}

#[test]
fn undeclared_dictionary_import_is_rejected() {
    let root = FixtureRoot::new("undeclared-import");
    root.write("Provider.ken", "pub class C a {}");
    root.write(
        "Consumer.ken",
        "import Provider (C, C_instance_Option) const value : Int = 0",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Consumer")
        .expect_err("an undeclared dictionary must not enter the export surface");
    assert_unbound(error, "Provider.C_instance_Option");
}

#[test]
fn private_class_dictionary_stays_off_the_owner_surface() {
    let root = FixtureRoot::new("private-dictionary");
    root.write(
        "Provider.ken",
        "class Private a {} instance Private Bool {}",
    );
    root.write(
        "Consumer.ken",
        "import Provider (Private_instance_Bool) const value : Int = 0",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Consumer")
        .expect_err("a private class must not leak its synthesized dictionary");
    assert_unbound(error, "Provider.Private_instance_Bool");
}

#[test]
fn distinct_local_collision_rejects_before_allocating_or_trusting_either_name() {
    let root = FixtureRoot::new("collision");
    root.write("Classes.ken", "pub class C a {}");
    root.write(
        "Entry.ken",
        "import Classes (C) \
         data Local = MkLocal \
         axiom C_instance_Local : C Local \
         instance C Local {}",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let trust_before = env.env.trusted_base();
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect_err("a distinct local cannot shadow the synthesized dictionary alias");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "C_instance_Local"),
        "collision must be attributed to the dictionary surface name, found {error:?}"
    );
    assert!(!env.globals.contains_key("Entry.C_instance_Local"));
    assert!(!env.globals.contains_key("C_instance_Entry.Local"));
    assert_eq!(env.env.trusted_base(), trust_before);
}

#[test]
fn duplicate_structure_dictionary_is_rejected_by_exact_class_head() {
    let root = FixtureRoot::new("duplicate");
    root.write("Classes.ken", "pub class C a { value : a }");
    root.write(
        "Entry.ken",
        "import Classes (C) \
         data Local = MkLocal \
         instance C Local { value = MkLocal } \
         instance C Local { value = MkLocal }",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let trust_before = env.env.trusted_base();
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect_err("one synthesis canonical cannot be registered twice");
    assert!(
        matches!(
            error,
            ElabError::OverlappingInstances {
                ref class,
                ref head_type,
                ..
            } if class == "C" && head_type == "Entry.Local"
        ),
        "duplicate must be rejected at its exact class/head identity, found {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust_before);

    assert!(root.path().exists());
}
