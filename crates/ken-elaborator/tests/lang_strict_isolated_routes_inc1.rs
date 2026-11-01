//! Strict isolated entry: checked imports, root discovery, and entry ownership.
//! Contract: spec/30-surface/33-declarations.md §3.2 and Architect R1–R3.

use std::path::{Path, PathBuf};

use ken_elaborator::modules::{catalog_module_from_path, isolated_catalog_root};
use ken_elaborator::{ElabEnv, ElabError};

fn load(env: &mut ElabEnv, roots: &[PathBuf], src: &str) -> Result<(), ElabError> {
    env.elaborate_isolated_file_strict_v1(roots, src)
        .map(|_| ())
}

fn root_with_lib() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("catalog/packages");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("Lib.ken"),
        "pub const main : Nat = Suc Zero\npub const other : Nat = Zero\n",
    )
    .unwrap();
    (dir, root)
}

fn unbound(result: Result<(), ElabError>, expected: &str) {
    match result {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, expected);
            assert_ne!(span.start, span.end, "refuse at the import edge");
        }
        other => panic!("expected UnboundName({expected}), got {other:?}"),
    }
}

/// Promise class: durable invariant. A root is selected by the source path,
/// not a build location or environment variable; nearest eligible ancestor wins.
#[test]
fn nearest_source_ancestor_discovers_root_without_changing_catalog_path_routing() {
    let (dir, outer) = root_with_lib();
    let nested = dir.path().join("nested/catalog/packages");
    std::fs::create_dir_all(&nested).unwrap();
    let isolated = dir.path().join("nested/src/root.ken.md");
    std::fs::create_dir_all(isolated.parent().unwrap()).unwrap();
    assert_eq!(isolated_catalog_root(&isolated), Some(nested.clone()));
    let outer_file = dir.path().join("src/root.ken");
    assert_eq!(isolated_catalog_root(&outer_file), Some(outer));
    assert!(isolated_catalog_root(Path::new("/tmp/ken-no-catalog/root.ken")).is_none());
    let catalog = catalog_module_from_path(&nested.join("Lib.ken")).unwrap();
    assert_eq!(catalog.root, nested);
    assert_eq!(catalog.entry, "Lib");
}

/// Promise class: durable invariant. Without a root, local declarations and
/// inline imports still work, but external imports/facades refuse at the edge.
#[test]
fn no_root_refuses_only_external_edges_and_literate_edges_are_extracted() {
    let mut env = ElabEnv::new().unwrap();
    load(&mut env, &[], "const local : Nat = Zero\n").unwrap();
    let mut env = ElabEnv::new().unwrap();
    load(
        &mut env,
        &[],
        "module Local { pub const answer : Nat = Zero }\nimport Local (answer)\nconst own : Nat = answer\n",
    )
    .unwrap();
    for edge in ["import Missing.Unit\n", "export Missing.Unit\n"] {
        let mut env = ElabEnv::new().unwrap();
        unbound(load(&mut env, &[], edge), "Missing.Unit");
    }
    let mut env = ElabEnv::new().unwrap();
    let literate =
        "# Import text is not Ken\n\n```ken\nimport Lib (other)\nconst own : Nat = other\n```\n";
    let (dir, root) = root_with_lib();
    let source_path = dir.path().join("example.ken.md");
    let discovered = isolated_catalog_root(&source_path).unwrap();
    assert_eq!(discovered, root);
    env.elaborate_isolated_ken_md_file_strict_v1(&[discovered], literate)
        .unwrap();
    let mut no_root = ElabEnv::new().unwrap();
    match no_root.elaborate_isolated_ken_md_file_strict_v1(&[], literate) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "Lib");
            assert_ne!(span.start, span.end);
        }
        other => panic!("expected literate import-edge refusal, got {other:?}"),
    }
}

/// Promise class: durable invariant. Explicit loading selects only checked
/// provider exports; missing modules do not fall back to flat globals.
#[test]
fn discovered_root_loads_selected_provider_and_missing_unit_refuses() {
    let (dir, root) = root_with_lib();
    let source_path = dir.path().join("src/main.ken");
    let roots = vec![isolated_catalog_root(&source_path).unwrap()];
    assert_eq!(roots, vec![root]);
    let mut env = ElabEnv::new().unwrap();
    load(
        &mut env,
        &roots,
        "import Lib (other)\nconst answer : Nat = other\n",
    )
    .unwrap();
    assert!(env.globals.contains_key("answer"));
    assert!(env.globals.contains_key("Lib.other"));
    let mut env = ElabEnv::new().unwrap();
    unbound(
        load(
            &mut env,
            &roots,
            "import Missing.Unit (other)\nconst x : Nat = Zero\n",
        ),
        "Missing.Unit",
    );
    let mut env = ElabEnv::new().unwrap();
    unbound(load(&mut env, &roots, "const x : Nat = other\n"), "other");
}

/// Promise class: durable invariant. An imported declaration cannot be a
/// root main; own main remains distinct; selective import plus own main
/// refuses rather than silently replacing either checked identity.
#[test]
fn imported_main_does_not_become_the_roots_entrypoint() {
    let (_dir, root) = root_with_lib();
    let mut env = ElabEnv::new().unwrap();
    load(
        &mut env,
        &[root.clone()],
        "import Lib\nconst main : Nat = Zero\n",
    )
    .unwrap();
    let own = env.globals["main"];
    let imported = env.globals["Lib.main"];
    assert_ne!(own, imported);
    let mut env = ElabEnv::new().unwrap();
    match load(
        &mut env,
        &[root.clone()],
        "import Lib (main)\nconst main : Nat = Zero\n",
    ) {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "main"),
        other => panic!("expected selective main collision, got {other:?}"),
    }
    let mut env = ElabEnv::new().unwrap();
    load(
        &mut env,
        &[root],
        "import Lib (main)\nconst x : Nat = main\n",
    )
    .unwrap();
    assert_eq!(env.globals.get("main"), None);
    assert!(env.globals.contains_key("Lib.main"));
}

/// Promise class: durable invariant. A partially committed Legacy file is
/// still an active Legacy run: the first checked declaration survives a later
/// refusal even though no complete in-memory unit was recorded. Strict must
/// not replace its scope and quietly admit a second source.
#[test]
fn partial_legacy_admission_cannot_switch_to_strict() {
    let (_dir, root) = root_with_lib();
    let mut env = ElabEnv::new().unwrap();
    assert!(env
        .elaborate_file_v1("const leaked : Nat = Zero\nconst broken : Bool = Zero\n")
        .is_err());
    assert!(
        env.globals.contains_key("leaked"),
        "earlier Legacy declaration was admitted"
    );
    match load(&mut env, &[root], "const fresh : Nat = Zero\n") {
        Err(ElabError::Internal(msg)) => assert!(msg.contains("resolution mode cannot change")),
        other => panic!("partial Legacy admission must keep its mode lock, got {other:?}"),
    }
    let mut parse_only = ElabEnv::new().unwrap();
    assert!(parse_only
        .elaborate_file_v1("const invalid : Nat =")
        .is_err());
    load(&mut parse_only, &[], "const fresh : Nat = Zero\n").unwrap();
}

/// Promise class: durable invariant. A continuing strict scope retains
/// imports and locals. Root changes, multiple roots and a prior Legacy unit
/// refuse before a second source can reinterpret checked identities.
#[test]
fn strict_scope_continues_but_root_or_mode_cannot_change_mid_run() {
    let (_dir, root) = root_with_lib();
    let mut env = ElabEnv::new().unwrap();
    load(
        &mut env,
        &[root.clone()],
        "import Lib (other)\nconst first : Nat = other\n",
    )
    .unwrap();
    load(&mut env, &[root.clone()], "const second : Nat = first\n").unwrap();
    assert!(env.globals.contains_key("second"));
    let wrong = root.parent().unwrap().join("other-packages");
    std::fs::create_dir_all(&wrong).unwrap();
    match load(&mut env, &[wrong], "const new : Nat = Zero\n") {
        Err(ElabError::ParseError { msg, .. }) => {
            assert!(msg.contains("catalog roots cannot change"))
        }
        other => panic!("expected root-lock refusal, got {other:?}"),
    }
    match load(
        &mut env,
        &[root.clone(), root.clone()],
        "const new : Nat = Zero\n",
    ) {
        Err(ElabError::ParseError { msg, .. }) => assert!(msg.contains("N2")),
        other => panic!("expected N2 refusal, got {other:?}"),
    }
    let mut rootless = ElabEnv::new().unwrap();
    load(&mut rootless, &[], "const first : Nat = Zero\n").unwrap();
    match load(
        &mut rootless,
        &[root.clone()],
        "const second : Nat = Zero\n",
    ) {
        Err(ElabError::ParseError { msg, .. }) => {
            assert!(msg.contains("catalog roots cannot change"))
        }
        other => panic!("expected rootless-to-rooted lock refusal, got {other:?}"),
    }
    let mut legacy = ElabEnv::new().unwrap();
    legacy
        .elaborate_file_v1("const legacy : Nat = Zero\n")
        .unwrap();
    match load(&mut legacy, &[root.clone()], "const strict : Nat = Zero\n") {
        Err(ElabError::Internal(msg)) => assert!(msg.contains("resolution mode cannot change")),
        other => panic!("expected Legacy-to-Strict refusal, got {other:?}"),
    }
    let mut legacy_roots = ElabEnv::new().unwrap();
    legacy_roots
        .elaborate_module_from_roots_v1(&[root.clone()], "Lib")
        .unwrap();
    match load(&mut legacy_roots, &[root], "const strict : Nat = Zero\n") {
        Err(ElabError::Internal(msg)) => assert!(msg.contains("resolution mode cannot change")),
        other => panic!("expected roots mode-lock refusal, got {other:?}"),
    }
}
