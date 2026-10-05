use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use tempfile::TempDir;

const OPEN: &str = r#"program capabilities FS APartial
const ac0_need : String requires Equal Int 0 0 = "ac0-run"
const ac0_use : String = ac0_need
proc main
      (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode
    visits [Console] =
  host_program APartial (print_line ac0_use)
"#;

const NONE: &str = r#"program capabilities FS APartial
const ac0_use : String = "ac0-run"
proc main
      (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode
    visits [Console] =
  host_program APartial (print_line ac0_use)
"#;

const DISCHARGED: &str = r#"program capabilities FS APartial
const ac0_need : String requires Equal Int 0 0 = "ac0-run"
fn ac0_use (p : Top) : String = ac0_need
proc main
      (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode
    visits [Console] =
  host_program APartial (print_line (ac0_use Proved))
"#;

const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn fixture(root: &Path, name: &str, source: &str) -> PathBuf {
    let path = root.join(name);
    std::fs::write(&path, source).expect("write Ken fixture");
    path
}

fn run(command: &str, args: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(command)
        .args(args)
        .output()
        .expect("ken command runs")
}

fn assert_one_open_report(stderr: &[u8]) {
    let stderr = String::from_utf8_lossy(stderr);
    let reports = stderr
        .lines()
        .filter(|line| line.starts_with("unknown ac0_use.requires."))
        .collect::<Vec<_>>();
    assert_eq!(reports.len(), 1, "stderr: {stderr}");
}

fn assert_no_open_report(stderr: &[u8]) {
    let stderr = String::from_utf8_lossy(stderr);
    assert!(
        !stderr
            .lines()
            .any(|line| line.starts_with("unknown ac0_use.requires.")),
        "stderr: {stderr}"
    );
}

fn repl(script: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("repl")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("ken repl starts");
    child
        .stdin
        .as_mut()
        .expect("repl stdin")
        .write_all(script.as_bytes())
        .expect("send repl input");
    child.wait_with_output().expect("repl exits")
}

/// Promise class: durable invariant.
/// MEASURED: a source with one open call-site `Requires` yields one `unknown`
/// line through each user-facing CLI/REPL route; discharged and absent-call
/// controls yield none and preserve their prior observable product output.
/// CLAIMED: these callers expose open obligations without turning them into a
/// failed check or a changed program result. THE GAP: compiler-driver semantic
/// maps and hashes are independently asserted in the elaborator tests.
#[test]
fn cli_and_repl_report_open_requires_and_keep_success_controls_unchanged() {
    let root = TempDir::new().expect("scratch fixture directory");
    let open_path = fixture(root.path(), "open.ken", OPEN);
    let none_path = fixture(root.path(), "none.ken", NONE);
    let discharged_path = fixture(root.path(), "discharged.ken", DISCHARGED);

    let checked = run("check", &[open_path.as_os_str()]);
    assert_eq!(checked.status.code(), Some(0), "stderr: {:?}", checked.stderr);
    assert!(checked.stdout.is_empty());
    assert_one_open_report(&checked.stderr);
    assert!(String::from_utf8_lossy(&checked.stderr)
        .contains("ken check: 1 open obligation(s), status unknown"));

    for path in [&none_path, &discharged_path] {
        let checked = run("check", &[path.as_os_str()]);
        assert_eq!(checked.status.code(), Some(0), "stderr: {:?}", checked.stderr);
        assert!(checked.stdout.is_empty());
        assert!(checked.stderr.is_empty());
    }

    let ran = run("run", &[open_path.as_os_str()]);
    assert_eq!(ran.status.code(), Some(0), "stderr: {:?}", ran.stderr);
    assert_eq!(ran.stdout, b"ac0-run\n");
    assert_one_open_report(&ran.stderr);

    for path in [&none_path, &discharged_path] {
        let ran = run("run", &[path.as_os_str()]);
        assert_eq!(ran.status.code(), Some(0), "stderr: {:?}", ran.stderr);
        assert_eq!(ran.stdout, b"ac0-run\n");
        assert!(ran.stderr.is_empty());
    }

    let open_repl = repl(
        ":def const ac0_need : String requires Equal Int 0 0 = \"ac0-run\"\n\
         :def const ac0_use : String = ac0_need\n\
         :list\n:quit\n",
    );
    assert_eq!(open_repl.status.code(), Some(0), "stderr: {:?}", open_repl.stderr);
    let open_repl_stdout = String::from_utf8_lossy(&open_repl.stdout);
    let repl_reports = open_repl_stdout
        .lines()
        .filter(|line| line.starts_with("unknown ac0_use.requires."))
        .collect::<Vec<_>>();
    assert_eq!(repl_reports.len(), 1, "stdout: {open_repl_stdout}");
    assert!(open_repl_stdout.contains("defined: ac0_use"));
    assert!(open_repl_stdout.ends_with("bye\n"));

    let module_repl = repl(
        ":def module Ac0 { const ac0_need : String requires Equal Int 0 0 = \"ac0-run\" const ac0_use : String = ac0_need const ac0_tail : String = \"tail\" }\n\
         :quit\n",
    );
    assert_eq!(module_repl.status.code(), Some(0), "stderr: {:?}", module_repl.stderr);
    let module_stdout = String::from_utf8_lossy(&module_repl.stdout);
    let module_reports = module_stdout
        .lines()
        .filter(|line| line.starts_with("unknown ") && line.contains("ac0_use.requires."))
        .collect::<Vec<_>>();
    assert_eq!(module_reports.len(), 1, "stdout: {module_stdout}");
    assert!(module_stdout.ends_with("bye\n"));

    for script in [
        ":def const ac0_use : String = \"ac0-run\"\n:list\n:quit\n",
        ":def const ac0_need : String requires Equal Int 0 0 = \"ac0-run\"\n\
         :def fn ac0_use (p : Top) : String = ac0_need\n:list\n:quit\n",
    ] {
        let control = repl(script);
        assert_eq!(control.status.code(), Some(0), "stderr: {:?}", control.stderr);
        assert!(control.stderr.is_empty());
        assert_no_open_report(&control.stdout);
        assert!(String::from_utf8_lossy(&control.stdout).ends_with("bye\n"));
    }

    let profile_path = root.path().join("resource-profile.json");
    std::fs::write(&profile_path, PROFILE).expect("write resource profile");
    for (name, path, expects_report) in [
        ("open", &open_path, true),
        ("none", &none_path, false),
        ("discharged", &discharged_path, false),
    ] {
        let output_dir = root.path().join(format!("native-{name}"));
        let built = run(
            "native-build",
            &[
                path.as_os_str(),
                output_dir.as_os_str(),
                profile_path.as_os_str(),
            ],
        );
        assert_eq!(built.status.code(), Some(0), "stderr: {:?}", built.stderr);
        let executable = output_dir.join("ken-starter");
        assert_eq!(built.stdout, format!("{}\n", executable.display()).as_bytes());
        assert!(executable.is_file());
        if expects_report {
            assert_one_open_report(&built.stderr);
        } else {
            // Test builds enable the runtime's unrelated planner census via
            // dev-feature unification; controls must omit only this task's row.
            assert_no_open_report(&built.stderr);
        }
    }
}
