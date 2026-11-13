//! Measurement-only AC-0 probes for spec/30-surface/33-declarations.md §3.3.
use ken_elaborator::{ElabEnv, ElabError};
use std::path::PathBuf;

const SOURCE: &str = "const d0_probe : HostIO AFull Unit = host_pure AFull Unit MkUnit\n";

#[test]
fn d0_base_bare_file_accepts() {
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    let results = env
        .elaborate_file_v1(SOURCE)
        .expect("Legacy standalone file");
    assert_eq!(results.len(), 1);
    assert_eq!(env.globals["host_pure"], expected);
}

#[test]
fn d0_base_bare_decl_accepts() {
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    env.elaborate_decl_v1(SOURCE)
        .expect("Legacy standalone declaration");
    assert_eq!(env.globals["host_pure"], expected);
}

fn fixture() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("catalog/packages");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("D0.ken"), SOURCE).unwrap();
    (dir, root)
}

#[test]
fn d0_base_bare_roots_accepts() {
    let (_dir, root) = fixture();
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    let results = env
        .elaborate_module_from_roots_v1(&[root], "D0")
        .expect("Legacy roots entry");
    assert_eq!(results.len(), 1);
    assert_eq!(env.globals["host_pure"], expected);
}

#[test]
fn d0_r_flagged_file_panics_on_host_pure() {
    let mut env = ElabEnv::new().unwrap();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.elaborate_file_v1(SOURCE)
    }));
    let value = outcome.expect_err("flagged strict bare host_pure must census-panic");
    let message = value
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| value.downcast_ref::<&str>().map(|value| value.to_string()))
        .expect("panic text");
    assert!(
        message
            .contains("LEGACY_CENSUS site=S-BARE entry=elaborate_file_v1 name=host_pure raw=false"),
        "{message}"
    );
}

#[test]
fn d0_r_unflagged_isolated_refuses_without_panic() {
    let mut env = ElabEnv::new().unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.elaborate_isolated_file_strict_v1(&[], SOURCE)
    }))
    .expect("unflagged Strict path must not panic");
    match result {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "host_pure"),
        other => panic!("expected missing host_pure, got {other:?}"),
    }
}

const IMPORT: &str = "import Capability.Process.Exit (host_pure)\n";
const IMPORTED_SOURCE: &str = "import Capability.Process.Exit (host_pure)\nconst d0_probe : HostIO AFull Unit = host_pure AFull Unit MkUnit\n";

#[test]
fn d0_r_nested_module_inherits_flag() {
    let mut env = ElabEnv::new().unwrap();
    let source = "module Nested { const zz : HostIO AFull Unit = host_pure AFull Unit MkUnit }\n";
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.elaborate_file_v1(source)
    }));
    let value = outcome.expect_err("nested module must census-panic");
    let message = value
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| value.downcast_ref::<&str>().map(|value| value.to_string()))
        .expect("panic text");
    assert!(
        message
            .contains("LEGACY_CENSUS site=S-BARE entry=elaborate_file_v1 name=host_pure raw=false"),
        "{message}"
    );
}

#[test]
fn d0_r_raw_postulate_is_scoped_to_its_environment() {
    let source = "const d0_use : Nat = d0_raw\n";
    let mut raw_env = ElabEnv::new().unwrap();
    raw_env
        .declare_postulate_raw(
            "d0_raw",
            ken_kernel::Term::const_(raw_env.globals["Nat"], vec![]),
        )
        .unwrap();
    let mut fresh_env = ElabEnv::new().unwrap();
    for (env, expected) in [(&mut raw_env, "raw=true"), (&mut fresh_env, "raw=false")] {
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            env.elaborate_file_v1(source)
        }));
        let value = outcome.expect_err("flagged scope must census-panic");
        let message = value
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| value.downcast_ref::<&str>().map(|value| value.to_string()))
            .expect("panic text");
        assert!(
            message.contains(&format!(
                "LEGACY_CENSUS site=S-BARE entry=elaborate_file_v1 name=d0_raw {expected}"
            )),
            "{message}"
        );
    }
}

#[test]
fn d0_i_fresh_file_import_needs_catalog_roots() {
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    match env.elaborate_file_v1(IMPORTED_SOURCE) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "Capability.Process.Exit"),
        other => panic!("D0-I fresh no-root import outcome: {other:?}"),
    }
    println!(
        "D0_I fresh_file=UnboundName(Capability.Process.Exit) pre_source_host_pure={expected:?}"
    );
}

#[test]
fn d0_i_file_after_roots_load_resolves_pre_source_id() {
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let loaded = env.elaborate_module_from_roots_v1(&[root], "Capability.Process.Exit");
    println!("D0_I roots_loader={loaded:?}");
    loaded.expect("real catalog unit sets roots");
    let results = env.elaborate_file_v1(IMPORTED_SOURCE);
    println!("D0_I after_roots_file={results:?} pre_source_host_pure={expected:?}");
    let results = results.expect("import through known roots");
    assert_eq!(results.len(), 1);
    println!(
        "D0_I resolved_body={:?}",
        env.env.transparent_body(results[0].def_id)
    );
    assert_eq!(env.globals["host_pure"], expected);
}

#[test]
fn d0_i_public_prepare_sets_roots_without_loading_a_unit() {
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    assert!(!env
        .globals
        .contains_key("Capability.Process.Exit.exit_success"));
    env.prepare_isolated_source_strict_v1(&[root.clone()], "", false)
        .expect("empty root-bearing source prepares without a unit");
    assert!(
        !env.globals
            .contains_key("Capability.Process.Exit.exit_success"),
        "prepare must not load catalog unit"
    );
    let without_load = env.elaborate_file_v1(IMPORTED_SOURCE);
    match without_load {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "Capability.Process.Exit"),
        other => panic!("roots alone must not make a provider loaded: {other:?}"),
    }
    env.prepare_isolated_source_strict_v1(&[root], IMPORTED_SOURCE, false)
        .expect("preparing source import loads its provider");
    let results = env
        .elaborate_file_v1(IMPORTED_SOURCE)
        .expect("import after provider was actually loaded");
    assert_eq!(results.len(), 1);
    assert_eq!(env.globals["host_pure"], expected);
    println!("D0_I public_prepare_empty=roots_only, ordinary_file=UnboundName, source_prepare=provider_loaded, later_file=Ok pre_source_host_pure={expected:?}");
}

#[test]
fn d0_i_two_decl_calls_import_then_use() {
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    let first = env.elaborate_decl_v1(IMPORT);
    println!("D0_I declaration_import={first:?}");
    let second = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.elaborate_decl_v1(SOURCE)
    }));
    println!("D0_I declaration_use={second:?} pre_source_host_pure={expected:?}");
    let value = second.expect_err("no-root later use must census-panic");
    let message = value
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| value.downcast_ref::<&str>().map(|value| value.to_string()))
        .expect("panic text");
    assert!(
        message
            .contains("LEGACY_CENSUS site=S-BARE entry=elaborate_decl_v1 name=host_pure raw=false"),
        "{message}"
    );
}
