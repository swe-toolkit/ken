//! Spec 36 §2.4, SLOTS Increments 1 and 3. Promise class: durable invariant.
//! MEASURED: a performing proc's tree is bound before a match scrutinee in
//! checked and inferred modes; the selected callback runs once, and nested
//! matches retain outer references. The s12 fixture executes identically on
//! the interpreter and native backend. CLAIMED: scrutinee-slot sequencing
//! preserves the explicit-let result, latent row and execution order.
//! THE GAP: these fixtures exercise checked/inferred call, returned closure,
//! nested outer-reference and recursive-result shapes, not every IH carrier;
//! the elaborator's separate core-identity pins check the let twins structurally.

use std::path::Path;
use std::process::{Command, Output};

const PRE: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
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
proc pick (r : Rose) : Box visits [Console] = match r {
  Leaf ↦ MkBox action ;
  Node b ↦ match b {
    Empty ↦ MkBox action ;
    One x ↦ pick x ;
    Join xs ys ↦ MkBox action } }
"#;
const MAIN_HEAD: &str = "proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n  : HostIO APartial ExitCode visits [Console] =\n";
const MAIN_USE: &str = "  let bx = usep (Node (One Rose Leaf)) in\n  match bx { MkBox g ↦ let ignored = g MkUnit in host_program APartial (print_line \"b\") }\n";
const RESOURCE_PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn invoke(cmd: &str, source: &Path, extra: &[&Path]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ken"));
    command.arg(cmd).arg(source);
    command.args(extra);
    command.output().expect("invoke checked Ken CLI")
}

fn assert_run(case: &str, source: &str, expected: &[u8], native: bool) {
    let parent = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp");
    std::fs::create_dir_all(&parent).expect("rooted fixture parent");
    let dir = tempfile::tempdir_in(parent).expect("isolated test fixture");
    let input = dir.path().join("program.ken");
    std::fs::write(&input, source).expect("write Ken source");
    let interpreted = invoke("run", &input, &[]);
    assert_eq!(
        interpreted.status.code(),
        Some(0),
        "{case}: interpreter rejected: {}",
        String::from_utf8_lossy(&interpreted.stderr)
    );
    assert_eq!(interpreted.stdout, expected, "{case}: interpreter output");
    if native {
        let profile = dir.path().join("profile.json");
        let artifact = dir.path().join("artifact");
        std::fs::write(&profile, RESOURCE_PROFILE).expect("write native resource profile");
        let built = invoke("native-build", &input, &[&artifact, &profile]);
        assert_eq!(
            built.status.code(),
            Some(0),
            "{case}: native-build rejected: {}",
            String::from_utf8_lossy(&built.stderr)
        );
        let executable = String::from_utf8(built.stdout).expect("native artifact path");
        let executed = Command::new(executable.trim())
            .output()
            .expect("run native artifact");
        assert_eq!(
            executed.status.code(),
            Some(0),
            "{case}: native execution: {}",
            String::from_utf8_lossy(&executed.stderr)
        );
        assert_eq!(executed.stdout, expected, "{case}: native output");
    }
}

/// The direct main scrutinee is the AC-1 execution and AC-2 mutation target.
#[test]
fn main_scrutinee_sequences_before_match_on_both_backends() {
    let source = format!(
        "{PRE}{MAIN_HEAD}  match pick (Node (One Rose Leaf)) {{ MkBox f ↦ \
         let ignored = f MkUnit in host_program APartial (print_line \"b\") }}\n"
    );
    assert_run("s12 main scrutinee", &source, b"ab\n", true);
}

#[test]
fn checked_callee_scrutinee_is_sequenced_when_called() {
    let source = format!(
        "{PRE}proc usep (r : Rose) : Box visits [Console] = \
         match pick r {{ MkBox f ↦ MkBox f }}\n{MAIN_HEAD}{MAIN_USE}"
    );
    assert_run("e2 callee scrutinee", &source, b"ab\n", false);
}

#[test]
fn inferred_main_scrutinee_sequences_before_callback() {
    let source = format!(
        "{PRE}{MAIN_HEAD}  let bx = match pick (Node (One Rose Leaf)) \
         {{ MkBox f ↦ MkBox f }} in\n  match bx {{ MkBox g ↦ \
         let ignored = g MkUnit in host_program APartial (print_line \"b\") }}\n"
    );
    assert_run("e1 inferred main scrutinee", &source, b"ab\n", false);
}

#[test]
fn nested_scrutinee_retains_outer_reference_and_callback_latent() {
    let source = format!(
        "{PRE}proc pickq (u : Unit) : Box visits [Console] = \
         let p = print_line \"q\" in MkBox quiet\n\
         proc usep (r : Rose) : Box visits [Console] = \
         match pick r {{ MkBox f ↦ match pickq MkUnit {{ MkBox g ↦ \
         match r {{ Leaf ↦ MkBox g ; Node b ↦ MkBox f }} }} }}\n\
         {MAIN_HEAD}{MAIN_USE}"
    );
    assert_run("e3 nested scrutinee", &source, b"q\nab\n", false);
}

fn recursive_result_program(join_body: &str) -> String {
    let pre = PRE.replacen(
        "Join xs ys ↦ MkBox action",
        &format!("Join xs ys ↦ {join_body}"),
        1,
    );
    assert_ne!(pre, PRE, "Join arm for recursive-result fixture");
    format!(
        "{pre}{MAIN_HEAD}  let bx = pick (Node (Join Rose (One Rose Leaf) (One Rose Leaf))) in\n  host_program APartial (print_line \"b\")\n"
    )
}

/// Promise class: durable invariant. The pure arm must be reached by a real
/// recursive call; an unchanged refusal or an uncalled declaration fails.
/// MEASURED: a Join call with `recursive result for xs` in a let executes.
/// CLAIMED: the selected IH result is sequenced through the let arm.
/// THE GAP: this Rose/Box example does not exercise every recursive carrier.
#[test]
fn recursive_result_pure_arm_let_executes() {
    let source = recursive_result_program(
        "let s = (recursive result for xs) in match s { MkBox f ↦ MkBox f }",
    );
    assert_run("s3 recursive result let", &source, b"b\n", false);
}

#[test]
fn recursive_result_pure_arm_scrutinee_executes() {
    let source = recursive_result_program("match (recursive result for xs) { MkBox f ↦ MkBox f }");
    assert_run("s2 recursive result scrutinee", &source, b"b\n", false);
}
