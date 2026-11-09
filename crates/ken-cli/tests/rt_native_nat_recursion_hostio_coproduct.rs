//! Spec 36 §2.4 and 42 §3: admitted recursive performed values lower to one
//! native execution that agrees with the reference interpreter.
//! Promise class: durable invariant. MEASURED: s23 Nat and the HostIO-typed
//! cnth each write p then b, byte-equal on both backends, with s21 Rose as a
//! control and FSRA's written-Cap HostIO result as a second native control.
//! CLAIMED: checked operation layout and emitted-member direct-style lowering
//! preserve the performed result and host effects for these recursive shapes.
//! THE GAP: these four fixed programs do not cover all recursive data families
//! or every live result across a later host effect; the latter can fail closed.

use std::path::Path;
use std::process::{Command, Output};

const PRE: &str = r#"program capabilities FS APartial
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data PBox = MkPBox (Unit -> HostIO APartial Unit)
data Flag = On | Off
record Slot { bx : Box }
def Callback = Unit ->[Console] HostIO APartial Unit
data Bag (a : Type) : Type where { Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a }
data Rose = Leaf | Node (Bag Rose)
"#;
const MAIN: &str = "proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n  : HostIO APartial ExitCode visits [Console] =\n";
const RESOURCE_PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn invoke(command: &str, input: &Path, extra: &[&Path]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ken"));
    child.arg(command).arg(input).args(extra);
    child.output().expect("invoke checked Ken CLI")
}

fn assert_native_matches_run(name: &str, source: &str, expected: &[u8]) {
    let dir = tempfile::tempdir().expect("isolated native fixture");
    let source_path = dir.path().join("program.ken");
    std::fs::write(&source_path, source).expect("write Ken fixture");
    let run = invoke("run", &source_path, &[]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{name}: ken run rejected: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, expected, "{name}: reference bytes");

    let artifact = dir.path().join("artifact");
    let profile = dir.path().join("profile.json");
    std::fs::write(&profile, RESOURCE_PROFILE).expect("write native profile");
    let build = invoke("native-build", &source_path, &[&artifact, &profile]);
    assert_eq!(
        build.status.code(),
        Some(0),
        "{name}: native-build rejected: {}",
        String::from_utf8_lossy(&build.stderr)
    );
    let executable = String::from_utf8(build.stdout).expect("native artifact path");
    let executed = Command::new(executable.trim())
        .output()
        .expect("run native artifact");
    assert_eq!(
        executed.status.code(),
        Some(0),
        "{name}: native execution rejected: {}",
        String::from_utf8_lossy(&executed.stderr)
    );
    assert_eq!(
        executed.stdout, run.stdout,
        "{name}: native/reference bytes"
    );
}

#[test]
fn nat_recursive_io_performs_once_and_executes_byte_equal() {
    let source = format!(
        "{PRE}proc cntn (n : Nat) : Unit visits [Console] = match n {{\n\
         Zero ↦ let p = print_line \"p\" in MkUnit ;\n\
         Suc m ↦ cntn m }}\n{MAIN}\
         let u = cntn (Suc Zero) in host_program APartial (print_line \"b\")\n"
    );
    assert_native_matches_run("s23 Nat IO", &source, b"p\nb\n");
}

#[test]
fn nat_recursive_hostio_performs_once_and_executes_byte_equal() {
    let source = format!(
        "{PRE}proc cnth (n : Nat) : HostIO APartial Unit visits [Console] = match n {{\n\
         Zero ↦ let p = host_console APartial Unit (print_line \"p\") in \
         host_pure APartial Unit MkUnit ;\n\
         Suc m ↦ cnth m }}\n{MAIN}\
         let u = cnth (Suc Zero) in host_program APartial (print_line \"b\")\n"
    );
    assert_native_matches_run("cnth Nat HostIO", &source, b"p\nb\n");
}

#[test]
fn rose_recursive_control_still_executes_byte_equal() {
    let source = format!(
        "{PRE}proc cnt (r : Rose) : Unit visits [Console] = match r {{\n\
         Leaf ↦ let p = print_line \"p\" in MkUnit ;\n\
         Node b ↦ match b {{ Empty ↦ MkUnit ; One x ↦ cnt x ; Join xs ys ↦ MkUnit }} }}\n\
         {MAIN}let u = cnt (Node (One Rose Leaf)) in \
         host_program APartial (print_line \"b\")\n"
    );
    assert_native_matches_run("s21 Rose", &source, b"p\nb\n");
}

#[test]
fn clock_hostio_depth_three_leaf_executes_byte_equal() {
    let source = "program capabilities FS APartial\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
         : HostIO APartial ExitCode visits [Clock, Console] =\n\
         let now = host_clock APartial Instant wall_now in\n\
         host_program APartial (print_line \"clock\")\n";
    assert_native_matches_run("Clock depth-three HostIO leaf", source, b"clock\n");
}

#[test]
fn written_cap_fsra_hostio_result_still_executes_byte_equal() {
    let source = "program capabilities FS AFull\n\
         proc f (c : Cap AFull) : String visits [FS, Console] =\n\
         let r = print_line \"x\" in \"one\"\n\
         proc main (_input : ProcessInput) (caps : ProgramCaps AFull)\n\
         : HostIO AFull ExitCode visits [FS, Console] =\n\
         match caps { ProgramCaps.MkProgramCaps cap |->\n\
           let selected : String = f cap in\n\
           host_program AFull (print_line selected) }\n";
    assert_native_matches_run("FSRA P2", source, b"x\none\n");
}
