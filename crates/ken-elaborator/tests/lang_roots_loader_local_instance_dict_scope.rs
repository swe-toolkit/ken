//! LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE acceptance.
//!
//! Promise class: durable invariant. A locally synthesized instance dictionary
//! is a first-class local declaration in strict roots scope, resolves to the
//! same canonical global as legacy loading, and participates in ordinary
//! top-level collision rejection.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Term;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-roots-local-instance-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create fixture root");
        Self(path)
    }

    fn write(&self, relative: &str, source: &str) {
        fs::write(self.0.join(relative), source).expect("write fixture source");
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn instance_fixture(label: &str, deriving: bool) -> FixtureRoot {
    let root = FixtureRoot::new(label);
    root.write("Provider.ken", "pub class C a {}");
    let producer = if deriving {
        "derive C for Local"
    } else {
        "instance C Local {}"
    };
    root.write(
        "Entry.ken",
        &format!(
            "import Provider (C) \
             data Local = MkLocal \
             {producer} \
             theorem selected : C Local = C_instance_Local"
        ),
    );
    root
}

fn assert_selected_is_synthesized(env: &ElabEnv, dictionary_name: &str, selected_name: &str) {
    let dictionary = env.globals[dictionary_name];
    let selected = env.globals[selected_name];
    let (_, body) = env
        .env
        .transparent_body(selected)
        .expect("selected is transparent");
    assert!(
        matches!(body, Term::Const { id, .. } if id == dictionary),
        "strict resolution must select the synthesized dictionary's existing GlobalId"
    );
}

#[test]
fn strict_roots_binds_local_instance_dictionary_to_its_canonical_global() {
    let root = instance_fixture("instance-strict", false);
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("strict roots resolves the local synthesized dictionary");
    assert_eq!(env.env.trusted_base(), trusted_before);
    assert_selected_is_synthesized(&env, "C_instance_Entry.Local", "Entry.selected");
}

#[test]
fn strict_roots_binds_local_derived_dictionary_to_its_canonical_global() {
    let root = instance_fixture("derive-strict", true);
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("strict roots resolves the local derived dictionary");
    assert_selected_is_synthesized(&env, "C_instance_Entry.Local", "Entry.selected");
}

#[test]
fn strict_roots_uses_the_imported_head_canonical_in_the_dictionary_binding() {
    let root = FixtureRoot::new("imported-head");
    root.write("Provider.ken", "pub data Remote = MkRemote");
    root.write(
        "Entry.ken",
        "import Provider (Remote) \
         class C a {} \
         instance C Remote {} \
         theorem selected : C Remote = C_instance_Remote",
    );
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("strict roots resolves a class-owned instance for an imported head");
    assert_selected_is_synthesized(&env, "C_instance_Provider.Remote", "Entry.selected");
}

#[test]
fn legacy_flat_resolution_of_the_same_local_dictionary_stays_unchanged() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "class C a {} \
         data Local = MkLocal \
         instance C Local {} \
         theorem selected : C Local = C_instance_Local",
    )
    .expect("legacy flat resolution keeps resolving the local dictionary");
    assert_selected_is_synthesized(&env, "C_instance_Local", "selected");
}

#[test]
fn explicit_local_collision_with_synthesized_dictionary_is_rejected_pre_admission() {
    let root = FixtureRoot::new("collision");
    root.write("Provider.ken", "pub class C a {}");
    root.write(
        "Entry.ken",
        "import Provider (C) \
         data Local = MkLocal \
         axiom C_instance_Local : C Local \
         instance C Local {}",
    );
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect_err("a distinct explicit local cannot collide with the synthesized dictionary");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "C_instance_Local"),
        "the collision must be attributed to the generated local binding, found {error:?}"
    );
    assert_eq!(
        env.env.trusted_base(),
        trusted_before,
        "scope collision rejection must happen before the explicit axiom is admitted"
    );
}
