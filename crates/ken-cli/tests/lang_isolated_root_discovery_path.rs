//! Canonical CLI source discovery and loaded catalog-entry admission (33 §3.2).

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ken_elaborator::program_admission::{admit_selected_main, MainSelection};
use ken_elaborator::{ElabEnv, ElabError};

const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;
const ENTRY: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line)
import Capability.Process.Exit (host_program)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "catalog-main")
"#;
const PURE_MAIN: &str = r#"program capabilities FS APartial
import Capability.Process.Exit (host_pure)
fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode = host_pure APartial ExitCode ExitCode.Success
"#;

fn write(parent: &Path, relative: &str, source: &str) -> PathBuf {
    let path = parent.join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, source).unwrap();
    path
}

fn fixture_root() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("catalog/packages");
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    for relative in [
        "Capability/Console/Text.ken.md",
        "Capability/Process/Exit.ken.md",
    ] {
        let target = root.join(relative);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::copy(repo.join(relative), target).unwrap();
    }
    std::fs::write(dir.path().join("profile.json"), PROFILE).unwrap();
    (dir, root)
}

fn invoke(route: &str, source: &Path, dir: &Path, report: Option<&Path>) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ken"));
    cmd.arg(route);
    if let Some(report) = report {
        cmd.arg(format!("--resolution-report={}", report.display()));
    }
    cmd.arg(source);
    if route == "native-build" {
        cmd.arg(dir.join("artifact")).arg(dir.join("profile.json"));
    }
    cmd.output().expect("real Ken CLI")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Promise class: durable invariant. MEASURED: a cycle-free catalog entry's
/// own checked result is the selected main, and check/run/native-build each
/// reach that entry without reloading it as an isolated dependency. CLAIMED:
/// catalog dispatch preserves owning identity and runtime behavior. THE GAP:
/// the isolated twin shares exactly the source and imports but has a distinct
/// filesystem role; its stdout and exit are compared, not its object IDs.
#[test]
fn catalog_runnable_entry_owns_main_and_runs_and_native_builds() {
    let (dir, root) = fixture_root();
    let entry = write(&root, "Demo/App.ken", ENTRY);
    let isolated = write(dir.path(), "src/main.ken", ENTRY);
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict_v1(&[root], "Demo.App")
        .expect("load the entry once");
    let own = env
        .module_state
        .root_results_for_report("Demo.App")
        .unwrap();
    let only_main = own
        .iter()
        .find(|result| result.name == "Demo.App.main")
        .unwrap();
    let admitted = admit_selected_main(&env, MainSelection::LoadedUnit("Demo.App"))
        .expect("headed catalog unit has a checked main");
    assert_eq!(admitted.main, only_main.def_id);
    let check = invoke("check", &entry, dir.path(), None);
    assert!(check.status.success(), "catalog check: {}", stderr(&check));
    let run_report = dir.path().join("run-report.json");
    let catalog = invoke("run", &entry, dir.path(), Some(&run_report));
    let plain = invoke("run", &isolated, dir.path(), None);
    assert!(
        catalog.status.success(),
        "catalog run: {}",
        stderr(&catalog)
    );
    assert!(plain.status.success(), "isolated run: {}", stderr(&plain));
    assert_eq!(catalog.status.code(), plain.status.code());
    assert_eq!(catalog.stdout, plain.stdout);
    assert_eq!(catalog.stdout, b"catalog-main\n");
    let run_json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&run_report).unwrap()).unwrap();
    assert_eq!(run_json["module"], serde_json::Value::Null);
    assert_eq!(run_json["declarations"].as_array().unwrap().len(), 1);
    assert_eq!(run_json["declarations"][0]["declaration"], "Demo.App.main");
    let native_report = dir.path().join("native-report.json");
    let native = invoke("native-build", &entry, dir.path(), Some(&native_report));
    assert!(
        native.status.success(),
        "catalog native-build: {}",
        stderr(&native)
    );
    let artifact = PathBuf::from(String::from_utf8(native.stdout).unwrap().trim());
    assert!(
        artifact.is_file(),
        "native-build must name a real executable"
    );
    let native_json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&native_report).unwrap()).unwrap();
    assert_eq!(native_json["module"], serde_json::Value::Null);
    assert_eq!(native_json["declarations"].as_array().unwrap().len(), 1);
    assert_eq!(
        native_json["declarations"][0]["declaration"],
        "Demo.App.main"
    );
    let plain_native_dir = dir.path().join("plain-native");
    std::fs::create_dir_all(&plain_native_dir).unwrap();
    std::fs::write(plain_native_dir.join("profile.json"), PROFILE).unwrap();
    let plain_native = invoke("native-build", &entry, &plain_native_dir, None);
    assert!(
        plain_native.status.success(),
        "unflagged catalog native-build: {}",
        stderr(&plain_native)
    );
    let plain_artifact = PathBuf::from(String::from_utf8(plain_native.stdout).unwrap().trim());
    assert!(
        plain_artifact.is_file(),
        "unflagged native route must emit executable"
    );
}

/// Promise class: durable invariant. MEASURED: check runs a stale entry
/// rejection fence, while run/native-build load the same checked unit without
/// executing that document-only role. CLAIMED: catalog loading and document
/// checking are distinct routes. THE GAP: the identical headed entry reaches
/// all three routes; only check executes its fence.
#[test]
fn catalog_literate_entry_fences_execute_only_on_check() {
    let (dir, root) = fixture_root();
    let entry = write(
        &root,
        "Demo/Document.ken.md",
        &format!("# Document\n\n```ken\n{PURE_MAIN}```\n\n```ken reject\nconst stale : Nat = Zero\n```\n"),
    );
    let check = invoke("check", &entry, dir.path(), None);
    assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
    assert!(
        stderr(&check).contains("negative example is stale"),
        "{}",
        stderr(&check)
    );
    let run = invoke("run", &entry, dir.path(), None);
    assert!(
        run.status.success(),
        "catalog run must not execute fence: {}",
        stderr(&run)
    );
    let native = invoke("native-build", &entry, dir.path(), None);
    assert!(
        native.status.success(),
        "catalog native-build must not execute fence: {}",
        stderr(&native)
    );
}

/// Promise class: durable invariant. MEASURED: a catalog entry without main
/// refuses as MissingEntrypoint, even if its closure loads successfully.
/// CLAIMED: a valid dependency or program header cannot synthesize main.
/// THE GAP: the entry alone has no matching main result.
#[test]
fn catalog_unit_without_main_refuses_run_at_admission() {
    let (dir, root) = fixture_root();
    let source = write(
        &root,
        "Demo/NoMain.ken",
        "program capabilities FS APartial\nconst x : Nat = Zero\n",
    );
    let output = invoke("run", &source, dir.path(), None);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("missing entrypoint 'main'"),
        "{}",
        stderr(&output)
    );
}

/// Promise class: durable invariant. MEASURED: an imported dependency has a
/// real distinct checked main, but running the entry returns the entry's own
/// success rather than that dependency's failure. CLAIMED: selection uses the
/// entry's owned result, not a flat global or imported declaration. THE GAP:
/// the two main outcomes intentionally differ on the same run input.
#[test]
fn dependency_main_cannot_replace_entry_main() {
    let (dir, root) = fixture_root();
    write(
        &root,
        "Demo/Helper.ken",
        "import Capability.Process.Exit (host_pure)\n\
pub fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
  : HostIO APartial ExitCode = host_pure APartial ExitCode (ExitCode.Failure 7)\n",
    );
    let entry = write(
        &root,
        "Demo/App.ken",
        &format!(
            "{}{}",
            "program capabilities FS APartial\nimport Demo.Helper (main as helper_main)\n",
            &PURE_MAIN["program capabilities FS APartial\n".len()..]
        ),
    );
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict_v1(&[root], "Demo.App")
        .unwrap();
    let own = env
        .module_state
        .root_results_for_report("Demo.App")
        .unwrap();
    let selected = admit_selected_main(&env, MainSelection::LoadedUnit("Demo.App")).unwrap();
    assert_eq!(
        selected.main,
        own.iter()
            .find(|result| result.name == "Demo.App.main")
            .unwrap()
            .def_id
    );
    assert_ne!(selected.main, env.globals["Demo.Helper.main"]);
    let output = invoke("run", &entry, dir.path(), None);
    assert!(
        output.status.success(),
        "own main must succeed: {}",
        stderr(&output)
    );
}

/// Promise class: durable invariant. MEASURED: duplicate entry main produces
/// the qualified name `Demo.Duplicate.main` in the roots loader, while run
/// projects that specific error to DuplicateEntrypoint. CLAIMED: selecting a
/// catalog unit never disguises a repeated entrypoint as ordinary admission.
/// THE GAP: another duplicate must not be mistaken for the entry's own main.
#[test]
fn duplicate_catalog_entry_main_maps_to_duplicate_entrypoint() {
    let (dir, root) = fixture_root();
    let second_main = &PURE_MAIN[PURE_MAIN.find("fn main").unwrap()..];
    let entry = write(
        &root,
        "Demo/Duplicate.ken",
        &format!("{PURE_MAIN}{second_main}"),
    );
    let mut env = ElabEnv::new().unwrap();
    match env.elaborate_module_from_roots_strict_v1(&[root], "Demo.Duplicate") {
        Err(ElabError::DuplicateDefinition { name, .. }) => assert_eq!(name, "Demo.Duplicate.main"),
        other => panic!("wrong duplicate-name carrier: {other:?}"),
    }
    let output = invoke("run", &entry, dir.path(), None);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("duplicate entrypoint 'main'"),
        "{}",
        stderr(&output)
    );
}

/// Promise class: durable invariant. MEASURED: check, run and native-build
/// report the identical entry-rooted import cycle, not a dependency-rooted
/// cycle after re-elaborating the entry as an isolated file. CLAIMED: the
/// catalog unit is loaded once. THE GAP: the dependent unit points back to
/// the actual selected entry, so a wrong-rooted route changes the payload.
#[test]
fn all_routes_report_entry_rooted_catalog_cycle() {
    let (dir, root) = fixture_root();
    let entry = write(
        &root,
        "Demo/App.ken",
        "program capabilities FS APartial\nimport Demo.Helper\n",
    );
    write(
        &root,
        "Demo/Helper.ken",
        "import Demo.App\npub const value : Nat = Zero\n",
    );
    for route in ["check", "run", "native-build"] {
        let output = invoke(route, &entry, dir.path(), None);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{route}: {}",
            stderr(&output)
        );
        assert!(
            stderr(&output)
                .contains("ImportCycle { cycle: [\"Demo.App\", \"Demo.Helper\", \"Demo.App\"]"),
            "{route}: {}",
            stderr(&output)
        );
    }
}

/// Promise class: durable invariant. MEASURED: the real source reached through
/// a symlinked parent/.. spelling or a symlinked leaf selects the same root
/// and refusal as its direct spelling on check and run. CLAIMED: path reading
/// and discovery are keyed by one canonical source. THE GAP: the decoy root
/// admits this program if selected lexically, so green is not vacuous.
#[cfg(unix)]
#[test]
fn symlinked_parent_and_leaf_are_canonical_on_check_and_run() {
    use std::os::unix::fs::symlink;

    let (dir, root) = fixture_root();
    write(&root, "SourceChoice.ken", "pub const value : Nat = Zero\n");
    let outside = tempfile::tempdir().unwrap();
    let header = "program capabilities FS APartial\n";
    let source = format!(
        "{header}import SourceChoice (value)\n{}",
        &PURE_MAIN[header.len()..]
    );
    let actual = write(outside.path(), "a/sub/Entry.ken", &source);
    std::fs::create_dir_all(outside.path().join("a/b")).unwrap();
    let shortcut = dir.path().join("app/link");
    std::fs::create_dir_all(shortcut.parent().unwrap()).unwrap();
    symlink(outside.path().join("a/b"), &shortcut).unwrap();
    let indirect = dir.path().join("app/link/../sub/Entry.ken");
    let leaf = root.join("Demo/Alias.ken");
    std::fs::create_dir_all(leaf.parent().unwrap()).unwrap();
    symlink(&actual, &leaf).unwrap();
    assert_eq!(
        std::fs::canonicalize(&actual).unwrap(),
        std::fs::canonicalize(&indirect).unwrap()
    );
    assert_eq!(
        std::fs::canonicalize(&actual).unwrap(),
        std::fs::canonicalize(&leaf).unwrap()
    );
    for route in ["check", "run"] {
        let direct = invoke(route, &actual, dir.path(), None);
        assert_eq!(
            direct.status.code(),
            Some(1),
            "{route}: {}",
            stderr(&direct)
        );
        assert!(
            stderr(&direct).contains("UnboundName { name: \"SourceChoice\""),
            "{route}: {}",
            stderr(&direct)
        );
        for spelling in [&indirect, &leaf] {
            let linked = invoke(route, spelling, dir.path(), None);
            assert_eq!(
                linked.status.code(),
                direct.status.code(),
                "{route}: {}",
                stderr(&linked)
            );
            assert!(
                stderr(&linked).contains("UnboundName { name: \"SourceChoice\""),
                "{route}: {}",
                stderr(&linked)
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: canonicalization failures
/// become the existing read-error codes and unlink a pre-created report.
/// CLAIMED: missing source never falls through to a lexical/no-root loader.
/// THE GAP: the profile and report paths are valid, so source is the sole
/// failure; the spelled path appears in each diagnostic.
#[test]
fn absent_source_fails_closed_on_each_route_and_aborts_report() {
    let (dir, _) = fixture_root();
    let missing = dir.path().join("missing.ken");
    for (route, code) in [
        ("check", "C0019"),
        ("run", "C0020"),
        ("native-build", "C0010"),
    ] {
        let report = dir.path().join(format!("{route}-missing-report.json"));
        let output = invoke(route, &missing, dir.path(), Some(&report));
        assert_eq!(
            output.status.code(),
            Some(1),
            "{route}: {}",
            stderr(&output)
        );
        assert!(!report.exists(), "{route}: failed report is unlinked");
        let error = stderr(&output);
        assert!(
            error.contains(&format!("error[{code}]:")),
            "{route}: {error}"
        );
        assert!(
            error.contains(&missing.display().to_string()),
            "{route}: {error}"
        );
    }
}
