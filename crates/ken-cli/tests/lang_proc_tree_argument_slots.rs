//! Spec 36 §2.4, SLOTS Increment 2. Promise class: durable invariant.
//! MEASURED: two performed arguments emit distinct lines in source order on
//! interpreter and native; an untaken match arm and an unapplied closure emit
//! none of their argument actions; a returned closure carried in an argument
//! works at the same boundary. A tree held as the first argument remains a
//! value while a later Unit argument performs. CLAIMED: slots sequence only when
//! evaluated, left to right, with the same denotation as explicit lets.
//! THE GAP: these programs measure concrete Console and returned-closure
//! shapes, not every effect family or the deferred recursive-result/IH slot;
//! the elaborator test compares the argument core with its let twin.

use std::path::Path;
use std::process::{Command, Output};

const HEAD: &str = "program capabilities FS APartial\n";
const EMIT: &str = "proc emit (msg : String) : Unit visits [Console] =\n  let r = print_line msg in MkUnit\nfn two (x : Unit) (y : Unit) : Unit = MkUnit\n";
const MAIN: &str = "proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n  : HostIO APartial ExitCode visits [Console] =\n";
const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn invoke(command: &str, source: &Path, extra: &[&Path]) -> Output {
    let mut process = Command::new(env!("CARGO_BIN_EXE_ken"));
    process.arg(command).arg(source).args(extra);
    process.output().expect("run Ken CLI")
}

fn assert_run(name: &str, source: &str, expected: &[u8], native: bool) {
    let parent = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp");
    std::fs::create_dir_all(&parent).expect("rooted fixture parent");
    let dir = tempfile::tempdir_in(parent).expect("isolated fixture");
    let path = dir.path().join("program.ken");
    let identifiers: std::collections::BTreeSet<_> = source
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .collect();
    let console = ["print_line", "write"]
        .into_iter()
        .filter(|name| identifiers.contains(name))
        .collect::<Vec<_>>();
    let exit = ["host_bind", "host_console", "host_program", "host_pure"]
        .into_iter()
        .filter(|name| identifiers.contains(name))
        .collect::<Vec<_>>();
    let mut imports = String::new();
    if !console.is_empty() {
        imports.push_str(&format!(
            "import Capability.Console.Text ({})\n",
            console.join(", ")
        ));
    }
    if !exit.is_empty() {
        imports.push_str(&format!(
            "import Capability.Process.Exit ({})\n",
            exit.join(", ")
        ));
    }
    let selected = source.replacen(HEAD, &format!("{HEAD}{imports}"), 1);
    std::fs::write(&path, selected).expect("write Ken source");
    let interpreted = invoke("run", &path, &[]);
    assert_eq!(
        interpreted.status.code(),
        Some(0),
        "{name} run: {}",
        String::from_utf8_lossy(&interpreted.stderr)
    );
    assert_eq!(interpreted.stdout, expected, "{name} interpreter output");
    if native {
        let profile = dir.path().join("profile.json");
        let artifact = dir.path().join("artifact");
        std::fs::write(&profile, PROFILE).expect("write profile");
        let built = invoke("native-build", &path, &[&artifact, &profile]);
        assert_eq!(
            built.status.code(),
            Some(0),
            "{name} native build: {}",
            String::from_utf8_lossy(&built.stderr)
        );
        let executable = String::from_utf8(built.stdout).expect("native path");
        let executed = Command::new(executable.trim())
            .output()
            .expect("execute native program");
        assert_eq!(
            executed.status.code(),
            Some(0),
            "{name} native execution: {}",
            String::from_utf8_lossy(&executed.stderr)
        );
        assert_eq!(executed.stdout, expected, "{name} native output");
    }
}

#[test]
fn two_performing_arguments_run_left_to_right_on_both_backends() {
    let src = format!("{HEAD}{EMIT}{MAIN}  let u = two (emit \"1\") (emit \"2\") in\n  host_program APartial (print_line \"b\")\n");
    assert_run("a0 arguments", &src, b"1\n2\nb\n", true);
}

#[test]
fn two_performing_arguments_in_changed_callee_run_left_to_right() {
    let src = format!("{HEAD}{EMIT}proc both (z : Unit) : Unit visits [Console] =\n  two (emit \"1\") (emit \"2\")\n{MAIN}  let u : Unit = both MkUnit in host_program APartial (print_line \"b\")\n");
    assert_run("a2 changed callee", &src, b"1\n2\nb\n", false);
}

#[test]
fn held_tree_argument_does_not_suppress_later_performed_slot() {
    let held = "host_console APartial (Result IOError Unit) (write ConsoleStream.Stdout (bytes_encode \"held\"))";
    let definitions =
        "fn take (tree : HostIO APartial (Result IOError Unit)) (unit : Unit) : Unit = unit\n";
    let direct = format!("{HEAD}{EMIT}{definitions}{MAIN}  let u = take ({held}) (emit \"later\") in host_program APartial (print_line \"done\")\n");
    let explicit = format!("{HEAD}{EMIT}{definitions}{MAIN}  let unit = emit \"later\" in let u = take ({held}) unit in host_program APartial (print_line \"done\")\n");
    assert_run(
        "held tree before performed argument",
        &direct,
        b"later\ndone\n",
        false,
    );
    assert_run(
        "explicit let with held tree",
        &explicit,
        b"later\ndone\n",
        false,
    );
}

/// Both arms return the declared tree. A mixed Unit/ITree match has no
/// checked value result and would refuse before locality can be observed.
#[test]
fn untaken_branch_does_not_hoist_argument_action() {
    let src = format!("{HEAD}{EMIT}data Flag = On | Off\n{MAIN}  match On {{ On ↦ host_program APartial (print_line \"b\") ; Off ↦ let u = two (emit \"1\") MkUnit in host_program APartial (print_line \"x\") }}\n");
    assert_run("untaken result branch", &src, b"b\n", true);
}

/// The closure returns a tree; a pure Unit-returning arrow may not silently
/// perform in its body even when that closure is never applied.
#[test]
fn unapplied_lambda_does_not_run_argument_action() {
    let src = format!("{HEAD}{EMIT}{MAIN}  let k : Unit -> HostIO APartial ExitCode = \\v. let u = two (emit \"1\") v in host_program APartial (print_line \"x\") in\n  host_program APartial (print_line \"b\")\n");
    assert_run("unapplied tree-result lambda", &src, b"b\n", false);
}

#[test]
fn returned_closure_can_pass_through_argument_slot() {
    let src = format!("{HEAD}data CBox = MkCBox (Unit ->[Console] HostIO APartial Unit)\nproc say (msg : String) : HostIO APartial Unit visits [Console] =\n  host_bind APartial (Result IOError Unit) Unit\n    (host_console APartial (Result IOError Unit)\n      (write ConsoleStream.Stdout (bytes_encode msg)))\n    (\\_. host_pure APartial Unit MkUnit)\nproc mk (msg : String) : CBox visits [Console] =\n  let r = print_line \"m\" in MkCBox (\\u. say msg)\nproc useb (c : CBox) (n : Unit) : CBox visits [Console] = c\n{MAIN}  let cb : CBox = useb (mk \"q\") MkUnit in match cb {{ MkCBox f ↦ let u = f MkUnit in\n    host_program APartial (print_line \"b\") }}\n");
    assert_run("c1 returned closure argument", &src, b"m\nqb\n", true);
}
