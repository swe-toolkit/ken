//! Strict isolated entry: checked imports, root discovery, and entry ownership.
//! Contract: spec/30-surface/33-declarations.md §3.2 and Architect R1–R3.

use std::path::{Path, PathBuf};

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerDriverError, CompilerManifest, CompilerSource,
    CompilerTargetKind, TargetSelector,
};
use ken_elaborator::modules::{catalog_module_from_path, discover_source_path, SourcePathKind};
use ken_elaborator::{ElabEnv, ElabError};

fn discovered_isolated_root(path: &Path) -> Option<PathBuf> {
    match discover_source_path(path)
        .expect("existing source path")
        .kind
    {
        SourcePathKind::Isolated(root) => root,
        SourcePathKind::CatalogUnit(unit) => panic!("unexpected catalog unit: {unit:?}"),
    }
}

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
    std::fs::write(&isolated, "const local : Nat = Zero\n").unwrap();
    assert_eq!(discovered_isolated_root(&isolated), Some(nested.clone()));
    let outer_file = dir.path().join("src/root.ken");
    std::fs::create_dir_all(outer_file.parent().unwrap()).unwrap();
    std::fs::write(&outer_file, "const local : Nat = Zero\n").unwrap();
    assert_eq!(discovered_isolated_root(&outer_file), Some(outer));
    let missing = dir.path().join("never-created/root.ken");
    assert_eq!(
        discover_source_path(&missing).unwrap_err().kind(),
        std::io::ErrorKind::NotFound
    );
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
    std::fs::write(&source_path, literate).unwrap();
    let discovered = discovered_isolated_root(&source_path).unwrap();
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
    std::fs::create_dir_all(source_path.parent().unwrap()).unwrap();
    std::fs::write(
        &source_path,
        "import Lib (other)\nconst answer : Nat = other\n",
    )
    .unwrap();
    let roots = vec![discovered_isolated_root(&source_path).unwrap()];
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

/// Promise class: durable invariant (spec 33 §3.3). Each isolated source on
/// one environment starts from a fresh scope: a later source sees neither an
/// earlier source's imports nor its top-level declarations. A root change,
/// multiple roots and rootless-to-rooted selection still refuse.
#[test]
fn strict_env_keeps_root_locks_but_each_source_starts_fresh() {
    let (_dir, root) = root_with_lib();
    let mut env = ElabEnv::new().unwrap();
    load(
        &mut env,
        &[root.clone()],
        "import Lib (other)\nconst first : Nat = other\n",
    )
    .unwrap();
    load(
        &mut env,
        &[root.clone()],
        "import Lib (other)\nconst imported_second : Nat = other\n",
    )
    .unwrap();
    assert!(env.globals.contains_key("imported_second"));
    match load(&mut env, &[root.clone()], "const second : Nat = first\n") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "first"),
        other => panic!("expected missing earlier-source name, got {other:?}"),
    }
    assert!(!env.globals.contains_key("second"));
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
}

/// Promise class: durable invariant. MEASURED: three rootless entries reject
/// a library import at the module path, roots APIs reject zero roots, and a
/// bare pre-source Perm is unavailable. CLAIMED: every source route uses the
/// same strict boundary. THE GAP: the d1 chokes cover additional shapes.
#[test]
fn rootless_library_import_refuses_at_the_import() {
    let source = "import Core.Logic.Not (Not)\nconst witness : Bool = True";
    let mut isolated = ElabEnv::new().expect("pre-source environment");
    unbound(load(&mut isolated, &[], source), "Core.Logic.Not");
    let mut file = ElabEnv::new().expect("pre-source environment");
    unbound(file.elaborate_file_v1(source).map(|_| ()), "Core.Logic.Not");

    let package = "rootless";
    let symbol = StableSymbol::declaration(package, &[], "witness");
    let driver = compile_ken_package_sources(
        &CompilerManifest::new(package, Vec::new()),
        vec![CompilerSource::new("rootless.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![package.to_string()]),
            symbol,
            kind: CompilerTargetKind::Library,
        },
    );
    match driver {
        Err(CompilerDriverError::Elaboration(ElabError::UnboundName { name, .. })) => {
            assert_eq!(name, "Core.Logic.Not");
        }
        other => panic!("rootless driver import must refuse at module path: {other:?}"),
    }

    let mut roots_entry = ElabEnv::new().expect("pre-source environment");
    match roots_entry.elaborate_module_from_roots(&[], "Entry") {
        Err(ElabError::ParseError { msg, .. }) => {
            assert!(msg.contains("N2 requires exactly one populated catalog root, found 0"));
        }
        other => panic!("zero roots must refuse in the roots entry: {other:?}"),
    }
    let mut set_roots = ElabEnv::new().expect("pre-source environment");
    match set_roots.set_catalog_roots_v1(&[]) {
        Err(ElabError::ParseError { msg, .. }) => {
            assert!(msg.contains("N2 requires exactly one populated catalog root, found 0"));
        }
        other => panic!("zero roots must refuse in set_catalog_roots_v1: {other:?}"),
    }
    let mut perm = ElabEnv::new().expect("pre-source environment");
    match perm.elaborate_file_v1("fn f (xs : List Nat) : Prop = Perm Nat xs xs") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "Perm"),
        other => panic!("bare pre-source Perm must refuse: {other:?}"),
    }
}

/// Promise class: durable invariant. Discovery walks the source file's own
/// ancestors: a `..` component never makes a child directory an ancestor.
#[test]
fn discovery_never_takes_a_child_directory_through_parent_components() {
    let dir = tempfile::tempdir().unwrap();
    let child_root = dir.path().join("a/b/catalog/packages");
    std::fs::create_dir_all(&child_root).unwrap();
    // The file is <dir>/a/x.ken; <dir>/a/b is a child of its directory, so
    // discovery answers exactly as it does for the file's own spelling.
    std::fs::create_dir_all(dir.path().join("a/b/src")).unwrap();
    std::fs::write(dir.path().join("a/x.ken"), "const x : Nat = Zero\n").unwrap();
    std::fs::write(dir.path().join("a/b/x.ken"), "const x : Nat = Zero\n").unwrap();
    let through_parent = discovered_isolated_root(&dir.path().join("a/b/../x.ken"));
    assert_ne!(through_parent.as_ref(), Some(&child_root));
    assert_eq!(
        through_parent,
        discovered_isolated_root(&dir.path().join("a/x.ken"))
    );
    // The file is <dir>/a/b/x.ken, whose own directory holds the root.
    assert_eq!(
        discovered_isolated_root(&dir.path().join("a/b/src/../x.ken")),
        Some(child_root)
    );
}

/// Promise class: durable invariant. A relative source path names a file
/// below the working directory; its ancestors continue above that directory.
#[test]
fn discovery_from_a_relative_path_climbs_past_the_working_directory() {
    // Cargo runs integration tests in the package directory, two levels below
    // the repository root that holds catalog/packages.
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .unwrap();
    assert_eq!(
        discover_source_path(Path::new("tests/root.ken"))
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::NotFound
    );
    let dir = tempfile::Builder::new()
        .prefix("relative-source-root-")
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .unwrap();
    let relative = PathBuf::from(dir.path().file_name().unwrap()).join("root.ken");
    std::fs::write(&relative, "const root : Nat = Zero\n").unwrap();
    assert_eq!(discovered_isolated_root(&relative), Some(repo_root));
}
