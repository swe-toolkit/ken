//! I1: checked literate fences import providers through selected roots.
//! Promise class: durable invariant (33 §3.2.1; Architect evt_4f4yshmbkj59e).

use std::path::{Path, PathBuf};

use ken_elaborator::{literate::extract_ken_md, ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

fn root_with_provider() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temporary root");
    std::fs::write(dir.path().join("Q.ken"), "pub const q_value : Nat = Zero\n").expect("provider");
    dir
}

fn document(role: &str, provider: &str) -> String {
    format!(
        "```ken\npub const entry_value : Nat = Zero\n```\n\
         ```ken {role}\nimport {provider} (q_value)\nconst fence_value : Nat = q_value\n```\n"
    )
}

fn roots(path: &Path) -> [PathBuf; 1] {
    [path.to_path_buf()]
}

fn expected_provider_id(path: &Path, isolated: bool) -> GlobalId {
    let mut separate = ElabEnv::new().expect("independent provider environment");
    if isolated {
        separate
            .elaborate_isolated_file_strict_v1(&roots(path), "pub const entry_value : Nat = Zero\n")
            .expect("independent isolated root precedes Q");
    } else {
        separate
            .elaborate_module_from_roots_strict(&roots(path), "M")
            .expect("independent catalog root precedes Q");
    }
    let results = separate
        .elaborate_module_from_roots_strict_v1(&roots(path), "Q")
        .expect("Q roots-loads independently");
    results
        .iter()
        .find(|result| result.name == "Q.q_value")
        .expect("provider's checked module-path/member result")
        .def_id
}

fn assert_head(env: &ElabEnv, client: GlobalId, provider: GlobalId) {
    let (_, body) = env
        .env
        .transparent_body(client)
        .expect("checked fence client");
    assert!(
        matches!(body, Term::Const { id, .. } if id == provider),
        "fence client must refer to independently loaded checked provider {provider:?}: {body:?}"
    );
}

fn assert_stale_reject(err: ElabError, doc: &str) {
    let range = extract_ken_md(doc)
        .expect("valid checked roles")
        .reject_ranges[0]
        .clone();
    match err {
        ElabError::ParseError { span, .. } => {
            assert_eq!(
                (span.start, span.end),
                (range.start, range.end),
                "unexpectedly valid reject is attributed to that exact fence"
            );
        }
        other => panic!("valid reject must become the wrapper ParseError: {other:?}"),
    }
}

/// MEASURED: a catalog entry with no tangled Q import checks a fence-only Q
/// import and its client has Q.q_value's independently roots-loaded checked ID.
/// CLAIMED: catalog-entry example imports use the selected roots in Strict.
/// THE GAP: acceptance alone could come from Legacy's flat globals; the
/// checked-head assertion and the preload mutation discriminate that route.
#[test]
fn example_catalog_entry_imports_provider_outside_tangled_closure() {
    let dir = root_with_provider();
    let doc = document("example", "Q");
    std::fs::write(dir.path().join("M.ken.md"), &doc).unwrap();
    let provider = expected_provider_id(dir.path(), false);
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict(&roots(dir.path()), "M")
        .expect("entry loads without a Q import");
    assert_eq!(env.loaded_module_count(), 1, "Q is outside tangled closure");
    let examples = env
        .execute_loaded_entry_checked_fences_v1("M")
        .expect("fence-only import succeeds");
    let client = examples
        .iter()
        .find(|result| result.name == "fence_value")
        .expect("checked example result");
    assert_head(&env, client.def_id, provider);
    assert_eq!(env.loaded_module_count(), 2, "fence-only Q becomes checked");
}

/// MEASURED: the imported, well-typed reject is stale at its own byte range.
/// CLAIMED: a catalog reject cannot pass on a provider-load-history accident.
/// THE GAP: an absent-provider reject is the separate positive control.
#[test]
fn reject_catalog_entry_imports_provider_outside_tangled_closure() {
    let dir = root_with_provider();
    let doc = document("reject", "Q");
    std::fs::write(dir.path().join("M.ken.md"), &doc).unwrap();
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict(&roots(dir.path()), "M")
        .expect("entry loads before checked roles");
    assert_eq!(env.loaded_module_count(), 1);
    let err = env
        .execute_loaded_entry_checked_fences("M")
        .expect_err("Q makes this reject body valid and stale");
    assert_stale_reject(err, &doc);
}

/// MEASURED: the isolated literate entry checks its example's head against a
/// separate checked Q owner. CLAIMED: isolation does not restrict fence-only
/// imports to the tangled closure. THE GAP: the import-preload mutation must
/// turn this positive case red at Q, not merely alter the loaded count.
#[test]
fn example_isolated_literate_imports_provider_outside_tangled_closure() {
    let dir = root_with_provider();
    let doc = document("example", "Q");
    let provider = expected_provider_id(dir.path(), true);
    let mut env = ElabEnv::new().unwrap();
    let results = env
        .elaborate_isolated_ken_md_file_strict_v1(&roots(dir.path()), &doc)
        .expect("isolated checked example imports Q");
    let client = results
        .iter()
        .find(|result| result.name == "fence_value")
        .expect("checked example result");
    assert_head(&env, client.def_id, provider);
    assert_eq!(env.loaded_module_count(), 1, "only fence imports Q");
}

/// MEASURED: isolated rejected body becomes a wrapper ParseError at its range.
/// CLAIMED: an isolated reject cannot rely on a module remaining unloaded.
/// THE GAP: the absent-provider control distinguishes a genuine refusal.
#[test]
fn reject_isolated_literate_imports_provider_outside_tangled_closure() {
    let dir = root_with_provider();
    let doc = document("reject", "Q");
    let mut env = ElabEnv::new().unwrap();
    let err = env
        .elaborate_isolated_ken_md_file_strict_v1(&roots(dir.path()), &doc)
        .expect_err("Q makes this reject stale");
    assert_stale_reject(err, &doc);
}

/// MEASURED: missing Q rejects at the import edge on both routes, while its
/// ken reject fences pass. CLAIMED: the positive Q rows fail due to successful
/// loading, not because reject checking ceased. THE GAP: this controls absent
/// modules, not other errors within a present provider.
#[test]
fn absent_provider_reject_still_passes_on_both_routes() {
    let dir = root_with_provider();
    let doc = document("reject", "Absent");
    std::fs::write(dir.path().join("M.ken.md"), &doc).unwrap();
    let range = extract_ken_md(&doc).unwrap().reject_ranges[0].clone();
    let mut catalog = ElabEnv::new().unwrap();
    catalog
        .elaborate_module_from_roots_strict(&roots(dir.path()), "M")
        .unwrap();
    let errors = catalog
        .execute_loaded_entry_checked_fences_reporting_rejections("M")
        .expect("a genuinely missing provider keeps ken reject valid");
    assert_eq!(errors.len(), 1);
    match &errors[0].1 {
        ElabError::UnboundName { name, span } => {
            assert_eq!(name, "Absent");
            assert_eq!(span.start, 0, "import starts the checked body");
            assert_eq!(span.end, "import Absent (q_value)".len());
            assert_eq!(errors[0].0, range);
        }
        other => panic!("missing provider must refuse at import edge: {other:?}"),
    }
    let mut isolated = ElabEnv::new().unwrap();
    isolated
        .elaborate_isolated_ken_md_file_strict_v1(&roots(dir.path()), &doc)
        .expect("absent provider is a valid ken reject in isolated source");
    let mut direct = ElabEnv::new().unwrap();
    match direct.elaborate_isolated_file_strict_v1(&roots(dir.path()), &doc[range]) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "Absent");
            assert_eq!((span.start, span.end), (0, "import Absent (q_value)".len()));
        }
        other => panic!("isolated absent-provider import must refuse: {other:?}"),
    }
}
