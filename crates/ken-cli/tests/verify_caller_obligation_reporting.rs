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

fn assert_no_open_report(output: &[u8]) {
    let output = String::from_utf8_lossy(output);
    assert!(
        !output
            .lines()
            .any(|line| line.starts_with("unknown ac0_use.requires.")),
        "output: {output}"
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

fn assert_native_build_row(
    root: &Path,
    profile_path: &Path,
    name: &str,
    source_path: &Path,
    expects_report: bool,
) {
    let output_dir = root.join(format!("native-{name}"));
    let built = run(
        "native-build",
        &[
            source_path.as_os_str(),
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
        // Test builds enable an unrelated runtime planner census through dev
        // feature unification; the row under test is the open-obligation line.
        assert_no_open_report(&built.stderr);
    }
}

/// Promise class: durable invariant.
/// MEASURED: a source with one open call-site `Requires` yields one `unknown`
/// line through each user-facing route, and all callers keep their success
/// behavior. THE GAP: controls are exercised independently below so a failing
/// open row cannot skip their observations.
#[test]
fn cli_and_repl_report_open_requires_and_succeed() {
    let root = TempDir::new().expect("scratch fixture directory");
    let open_path = fixture(root.path(), "open.ken", OPEN);

    let checked = run("check", &[open_path.as_os_str()]);
    assert_eq!(checked.status.code(), Some(0), "stderr: {:?}", checked.stderr);
    assert!(checked.stdout.is_empty());
    assert_one_open_report(&checked.stderr);
    assert!(String::from_utf8_lossy(&checked.stderr)
        .contains("ken check: 1 open obligation(s), status unknown"));

    let ran = run("run", &[open_path.as_os_str()]);
    assert_eq!(ran.status.code(), Some(0), "stderr: {:?}", ran.stderr);
    assert_eq!(ran.stdout, b"ac0-run\n");
    assert_one_open_report(&ran.stderr);

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

    let profile_path = root.path().join("resource-profile.json");
    std::fs::write(&profile_path, PROFILE).expect("write resource profile");
    assert_native_build_row(root.path(), &profile_path, "open", &open_path, true);
}

/// Promise class: durable invariant.
/// MEASURED: closed sources keep check/run stdout and native-build's path, and
/// produce no open-obligation line. CLAIMED: reporting changes only open
/// obligations, not discharged or absent ones. THE GAP: test binaries emit an
/// unrelated planner record on stderr; driver hash controls are pinned in
/// ken-elaborator tests.
#[test]
fn cli_and_repl_closed_controls_remain_silent_and_keep_products_unchanged() {
    let root = TempDir::new().expect("scratch fixture directory");
    let none_path = fixture(root.path(), "none.ken", NONE);
    let discharged_path = fixture(root.path(), "discharged.ken", DISCHARGED);

    for path in [&none_path, &discharged_path] {
        let checked = run("check", &[path.as_os_str()]);
        assert_eq!(checked.status.code(), Some(0), "stderr: {:?}", checked.stderr);
        assert!(checked.stdout.is_empty());
        assert!(checked.stderr.is_empty());

        let ran = run("run", &[path.as_os_str()]);
        assert_eq!(ran.status.code(), Some(0), "stderr: {:?}", ran.stderr);
        assert_eq!(ran.stdout, b"ac0-run\n");
        assert!(ran.stderr.is_empty());
    }

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
    assert_native_build_row(root.path(), &profile_path, "none", &none_path, false);
    assert_native_build_row(
        root.path(),
        &profile_path,
        "discharged",
        &discharged_path,
        false,
    );
}
