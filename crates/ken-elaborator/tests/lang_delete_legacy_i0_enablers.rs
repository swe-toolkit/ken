//! I0 enablers: ordinary catalog imports and checked raw root identities.
//! Promise class: durable invariant. The provider's checked identity is the
//! imported identity, not an equal spelling in the mutable globals table.

use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

const HOME_IMPORT: &str = "import Capability.Process.Exit (host_pure)\n";
const CLIENT: &str = "fn selected (a : Auth) (r : Type) (x : r) : HostIO a r = host_pure a r x\n";

fn checked_head(env: &ElabEnv, id: GlobalId) -> GlobalId {
    let (_, mut term) = env
        .env
        .transparent_body(id)
        .expect("checked transparent client");
    loop {
        match term {
            Term::Lam(_, body) | Term::App(body, _) => term = *body,
            Term::Const { id, .. } => return id,
            other => panic!("client did not reference its selected provider: {other:?}"),
        }
    }
}

/// MEASURED: the ordinary file and declaration entries load their explicit
/// provider and emit a checked reference to the pre-source home id.
/// CLAIMED: the same home identity is selected across ordinary calls.
/// THE GAP: an importless Legacy fallback also emits that ID; the I0 census
/// scratch control must remove this import and fail on terminal S-BARE.
#[test]
fn ordinary_file_and_decl_imports_select_pre_source_home_id() {
    let roots = [catalog_root()];
    let mut file = ElabEnv::new().unwrap();
    let expected = file.globals["host_pure"];
    eprintln!("I0 base pre-source host_pure id={expected:?}");
    file.set_catalog_roots_v1(&roots).unwrap();
    file.set_catalog_roots_v1(&roots)
        .expect("same root is idempotent");
    file.elaborate_file_v1(&format!("{HOME_IMPORT}{CLIENT}"))
        .unwrap();
    assert_eq!(checked_head(&file, file.globals["selected"]), expected);

    let mut decl = ElabEnv::new().unwrap();
    assert_eq!(decl.globals["host_pure"], expected);
    decl.set_catalog_roots_v1(&roots).unwrap();
    assert!(decl
        .elaborate_decl_results_v1(HOME_IMPORT)
        .unwrap()
        .is_empty());
    let selected = decl.elaborate_decl_v1(CLIENT).unwrap();
    assert_eq!(checked_head(&decl, selected.def_id), expected);

    // A mutable same-spelling flat entry must not choose the imported ID.
    let mut shadow = ElabEnv::new().unwrap();
    let home = shadow.globals["host_pure"];
    let other = shadow.globals["host_bind"];
    assert_ne!(home, other);
    shadow.globals.insert("host_pure".to_string(), other);
    shadow.set_catalog_roots_v1(&roots).unwrap();
    shadow.elaborate_file_v1(HOME_IMPORT).unwrap();
    shadow.elaborate_file_v1(CLIENT).unwrap();
    assert_eq!(checked_head(&shadow, shadow.globals["selected"]), home);
}

/// MEASURED: the actual roots entry loads a file client and resolves the
/// imported checked home identity. CLAIMED: roots and ordinary file entries
/// agree on the pre-source home, regardless of their separate owner routes.
/// THE GAP: the census scratch deletion control rules out Legacy passthrough.
#[test]
fn roots_entry_client_imports_the_same_pre_source_home_id() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("catalog/packages");
    std::fs::create_dir_all(root.join("Capability/Process")).unwrap();
    std::fs::copy(
        catalog_root().join("Capability/Process/Exit.ken.md"),
        root.join("Capability/Process/Exit.ken.md"),
    )
    .unwrap();
    std::fs::write(
        root.join("Client.ken"),
        format!("{HOME_IMPORT}pub {CLIENT}"),
    )
    .unwrap();
    let mut env = ElabEnv::new().unwrap();
    let expected = env.globals["host_pure"];
    env.elaborate_module_from_roots_v1(&[root], "Client")
        .unwrap();
    assert_eq!(checked_head(&env, env.globals["Client.selected"]), expected);
}

/// MEASURED: with no root even a real module is refused at the import edge.
/// CLAIMED: I0 does not change rootless environments or stale inline imports.
/// THE GAP: this row tests an actual non-inline provider, not a missing file.
#[test]
fn rootless_ordinary_import_retains_missing_provider_refusal() {
    let mut env = ElabEnv::new().unwrap();
    match env.elaborate_file_v1(&format!("{HOME_IMPORT}{CLIENT}")) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "Capability.Process.Exit");
            assert_eq!((span.start, span.end), (0, HOME_IMPORT.trim_end().len()));
        }
        other => panic!("rootless file must refuse at import: {other:?}"),
    }
}

/// MEASURED: a raw declaration has a checked identity through a later
/// ordinary source, while a same-named import and raw declaration cannot
/// silently replace one another in either order. CLAIMED: R-RAW binds the
/// returned id in the root scope and rejects takeover before minting.
/// THE GAP: before/after next_global_id proves the rejected raw collision
/// did not allocate a hidden kernel declaration.
#[test]
fn raw_checked_root_identity_cannot_be_replaced_by_an_import() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("catalog/packages");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("Provider.ken"), "pub const raw_piece : Int = 0\n").unwrap();
    let roots = [root];
    let mut raw_first = ElabEnv::new().unwrap();
    let int_id = raw_first.globals["Int"];
    let raw_id = raw_first
        .declare_postulate_raw("raw_piece", Term::const_(int_id, vec![]))
        .unwrap();
    let selected = raw_first
        .elaborate_decl_v1("const used_raw : Int = raw_piece")
        .unwrap();
    assert_eq!(checked_head(&raw_first, selected.def_id), raw_id);
    raw_first.set_catalog_roots_v1(&roots).unwrap();
    match raw_first.elaborate_decl_results_v1("import Provider (raw_piece)") {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "raw_piece"),
        other => panic!("raw must not be taken over by imported provider: {other:?}"),
    }
    assert_eq!(raw_first.globals["raw_piece"], raw_id);
    let used_again = raw_first
        .elaborate_decl_v1("const still_raw : Int = raw_piece")
        .unwrap();
    assert_eq!(checked_head(&raw_first, used_again.def_id), raw_id);
    let next_id = raw_first.env.next_global_id();
    match raw_first.elaborate_decl_v1("const raw_piece : Int = 1") {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "raw_piece"),
        other => panic!("later same-spelled source must refuse before takeover: {other:?}"),
    }
    assert_eq!(raw_first.env.next_global_id(), next_id);
    assert_eq!(raw_first.globals["raw_piece"], raw_id);

    // This is the discriminating R-RAW arm: Strict has NO flat-global
    // fallback. Only the new checked root binding can admit this use.
    let mut strict = ElabEnv::new().unwrap();
    strict
        .prepare_isolated_source_strict_v1(&[], "const strict_raw : Int = raw_piece", false)
        .unwrap();
    let int_id = strict.globals["Int"];
    let strict_id = strict
        .declare_postulate_raw("raw_piece", Term::const_(int_id, vec![]))
        .unwrap();
    let strict_client = strict
        .elaborate_decl_v1("const strict_raw : Int = raw_piece")
        .unwrap();
    assert_eq!(checked_head(&strict, strict_client.def_id), strict_id);

    let mut imported_first = ElabEnv::new().unwrap();
    imported_first.set_catalog_roots_v1(&roots).unwrap();
    imported_first
        .elaborate_decl_results_v1("import Provider (raw_piece)")
        .unwrap();
    let provider_id = imported_first.globals["Provider.raw_piece"];
    let next_id = imported_first.env.next_global_id();
    match imported_first.declare_postulate_raw("raw_piece", Term::const_(int_id, vec![])) {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "raw_piece"),
        other => panic!("imported checked identity must not be taken over by raw: {other:?}"),
    }
    assert_eq!(imported_first.env.next_global_id(), next_id);
    let selected = imported_first
        .elaborate_decl_v1("const still_provider : Int = raw_piece")
        .unwrap();
    assert_eq!(checked_head(&imported_first, selected.def_id), provider_id);
}

/// MEASURED: N2, changed-root and opposite-mode requests fail without
/// replacing a checked provider. CLAIMED: the setter shares the roots cache
/// and its mode lock with the other entries, not a second mutable root plane.
#[test]
fn ordinary_roots_setting_is_single_and_mode_locked() {
    let mut env = ElabEnv::new().unwrap();
    let roots = [catalog_root()];
    assert!(matches!(
        env.set_catalog_roots_v1(&[]),
        Err(ElabError::ParseError { .. })
    ));
    assert!(matches!(
        env.set_catalog_roots_v1(&[roots[0].clone(), roots[0].clone()]),
        Err(ElabError::ParseError { .. })
    ));
    env.set_catalog_roots_v1(&roots).unwrap();
    let other = [roots[0].join("other")];
    assert!(matches!(
        env.set_catalog_roots_v1(&other),
        Err(ElabError::ParseError { .. })
    ));
    env.elaborate_file_v1("import Capability.Process.Exit (host_pure)\n")
        .unwrap();
    match env.elaborate_isolated_file_strict_v1(&roots, "const after : Nat = Zero\n") {
        Err(ElabError::Internal(msg)) => assert!(msg.contains("resolution mode cannot change")),
        other => panic!("Legacy roots must not switch to Strict: {other:?}"),
    }
}
