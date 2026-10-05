use std::fs;

use ken_elaborator::{ElabEnv, render_open_obligations};
use tempfile::tempdir;

const OPEN_FILE: &str = r#"const ac0_need : String requires Equal Int 0 0 = "ac0-run"
const ac0_use : String = ac0_need
const main : String = ac0_use
"#;

const NONE_FILE: &str = "const ac0_use : String = \"ac0-run\"\nconst main : String = ac0_use\n";

const DISCHARGED_FILE: &str = r#"const ac0_need : String requires Equal Int 0 0 = "ac0-run"
fn ac0_use (p : Top) : String = ac0_need
const main : String = ac0_use Proved
"#;

const LITERATE: &str = r#"# Open obligation roles

```ken
const ac0_need : String requires Equal Int 0 0 = "ac0-run"
```

```ken example
const ac0_use : String = ac0_need
```

```ken reject
const ac0_bad : Bool = missing_ac0_name
```
"#;

/// Promise class: durable invariant.
/// MEASURED: V1 returns one open result for the same source whose result-row
/// counts are `[0, 1, 0]`; both ID-only wrappers project those exact IDs.
/// CLAIMED: API callers can report every open result without drifting from the
/// legacy ID set. THE GAP: the compiler driver still has separate integration
/// checks for target selection and package metadata.
#[test]
fn file_and_literate_v1_results_preserve_open_obligations_and_wrapper_ids() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env.elaborate_file_v1(OPEN_FILE).expect("one open call");
    let counts = results
        .iter()
        .map(|result| result.obligations.len())
        .collect::<Vec<_>>();
    assert_eq!(counts, vec![0, 1, 0]);
    let reports = render_open_obligations(&results);
    assert_eq!(reports.len(), 1);
    assert!(reports[0].starts_with("unknown ac0_use.requires."));

    let mut legacy = ElabEnv::new().expect("prelude");
    let ids = legacy.elaborate_file(OPEN_FILE).expect("ID wrapper");
    assert_eq!(ids, results.iter().map(|result| result.def_id).collect::<Vec<_>>());

    for source in [NONE_FILE, DISCHARGED_FILE] {
        let mut control = ElabEnv::new().expect("prelude");
        let control_results = control.elaborate_file_v1(source).expect("control elaborates");
        assert!(control_results
            .iter()
            .all(|result| result.obligations.is_empty()));
        assert!(render_open_obligations(&control_results).is_empty());
    }

    let mut literate = ElabEnv::new().expect("prelude");
    let literate_results = literate
        .elaborate_ken_md_file_v1(LITERATE)
        .expect("ken example succeeds and ken reject fails");
    assert_eq!(literate_results.len(), 2, "reject fences are excluded");
    assert_eq!(
        literate_results
            .iter()
            .map(|result| result.obligations.len())
            .sum::<usize>(),
        1
    );
    let literate_reports = render_open_obligations(&literate_results);
    assert_eq!(literate_reports.len(), 1);

    let mut literate_legacy = ElabEnv::new().expect("prelude");
    let literate_ids = literate_legacy
        .elaborate_ken_md_file(LITERATE)
        .expect("ID wrapper shares the V1 path");
    assert_eq!(
        literate_ids,
        literate_results
            .iter()
            .map(|result| result.def_id)
            .collect::<Vec<_>>()
    );
}

/// Promise class: durable invariant.
/// MEASURED: Dependency owns one open call-site hole. Entry loads it first;
/// a second roots call loads Other through the cached Dependency and still
/// returns that hole. The loaded literate entry's example result is returned by
/// the separate fence V1 operation.
/// CLAIMED: roots V1 reports dependency obligations on both fresh and cached
/// routes. THE GAP: the fixture has one shared dependency edge, not a wide DAG.
#[test]
fn module_roots_and_loaded_literate_fences_return_their_elaboration_results() {
    let root = tempdir().expect("module root");
    fs::write(
        root.path().join("Dependency.ken"),
        "pub const ac0_need : String requires Equal Int 0 0 = \"ac0-run\"\n\
         pub const ac0_use : String = ac0_need\n",
    )
    .expect("write dependency");
    fs::write(
        root.path().join("Entry.ken"),
        "import Dependency\nconst entry_value : String = Dependency.ac0_use\n",
    )
    .expect("write entry");
    fs::write(
        root.path().join("Other.ken"),
        "import Dependency\nconst other_value : String = Dependency.ac0_use\n",
    )
    .expect("write second root sharing the cached dependency");

    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")
        .expect("root and dependency elaborate");
    assert_eq!(results.len(), 3, "both dependency results are included");
    assert_eq!(
        results
            .iter()
            .map(|result| result.obligations.len())
            .sum::<usize>(),
        1
    );
    assert_eq!(render_open_obligations(&results).len(), 1);
    let cached_results = env
        .elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Other")
        .expect("second root retains the cached dependency's V1 results");
    assert_eq!(cached_results.len(), 3);
    assert_eq!(render_open_obligations(&cached_results).len(), 1);
    let cached_ids = env
        .elaborate_module_from_roots(&[root.path().to_path_buf()], "Other")
        .expect("cached dependency ID wrapper projects the V1 results");
    assert_eq!(
        cached_ids,
        cached_results
            .iter()
            .map(|result| result.def_id)
            .collect::<Vec<_>>()
    );

    let mut legacy = ElabEnv::new().expect("prelude");
    let ids = legacy
        .elaborate_module_from_roots(&[root.path().to_path_buf()], "Entry")
        .expect("ID-only roots wrapper");
    assert_eq!(ids, results.iter().map(|result| result.def_id).collect::<Vec<_>>());

    let literate_source = root.path().join("LiterateEntry.ken.md");
    fs::write(&literate_source, LITERATE).expect("write literate entry");
    let mut literate_env = ElabEnv::new().expect("prelude");
    let root_results = literate_env
        .elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "LiterateEntry")
        .expect("literate module root loads without running fence roles");
    assert_eq!(root_results.len(), 1);
    let fence_results = literate_env
        .execute_loaded_entry_checked_fences_v1("LiterateEntry")
        .expect("loaded entry's example fence runs");
    assert_eq!(fence_results.len(), 1);
    assert_eq!(fence_results[0].obligations.len(), 1);
    assert_eq!(render_open_obligations(&fence_results).len(), 1);
}
