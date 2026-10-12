//! Entry ABI identities are captured before source elaboration, including
//! checked examples. Contract: spec 33 §3.2.1, 42 §6, 62 §3.2; Architect D0
//! `evt_45kr33z11554r` for VERIFY-ENTRY-ABI-CHECKED-IDENTITY.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const EXIT_MAIN: &str = "fn main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode = host_exit ANone Success";
const PRINT_MAIN: &str = "proc main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode visits [Console] = host_program ANone (print_line \"entered main\")";
const PROFILE: &str = "{\"runtime\":{\"invocation_epochs\":1024},\"call_events\":{\"event_generations\":2048,\"live_pending_slots\":1024},\"invocation\":{\"nodes\":100000,\"words\":100000,\"data_bytes\":1000000,\"native_int_limbs\":100000},\"persistent\":{\"nodes\":100000,\"words\":100000,\"data_bytes\":1000000,\"native_int_limbs\":100000}}";

#[derive(Clone, Copy)]
enum Fence {
    Example,
    Plain,
}

impl Fence {
    fn label(self) -> &'static str {
        match self {
            Self::Example => "example",
            Self::Plain => "plain",
        }
    }
}

fn fixture(main: &str, fence: Fence, name: &str) -> String {
    let shadow = format!("const {name} : Nat = Zero\n");
    match fence {
        Fence::Example => format!(
            "```ken\nprogram capabilities FS ANone\n{main}\n```\n```ken example\n{shadow}```\n"
        ),
        Fence::Plain => format!("```ken\nprogram capabilities FS ANone\n{main}\n{shadow}```\n"),
    }
}

fn write_fixture(dir: &Path, label: &str, source: &str) -> PathBuf {
    let path = dir.join(format!("{label}.ken.md"));
    fs::write(&path, source).expect("write Ken fixture");
    path
}

fn run(path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(path)
        .output()
        .expect("ken run")
}

fn native_build(path: &Path, output_dir: &Path, profile: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(path)
        .arg(output_dir)
        .arg(profile)
        .output()
        .expect("ken native-build")
}

fn native_run(input: &Path, output_dir: &Path, profile: &Path) -> Output {
    let built = native_build(input, output_dir, profile);
    assert!(
        built.status.success(),
        "native admission and emission from {input:?} must succeed: {built:?}"
    );
    Command::new(output_dir.join("ken-starter"))
        .output()
        .expect("run native starter")
}

/// Promise class: durable invariant. MEASURED: the complete adversary-reported
/// spelling set in both fence forms prints and exits like its same-main,
/// same-fence unrelated control; exit-example Success also exits successfully.
/// CLAIMED: source spelling cannot redirect reference Console/exit ABI roles.
/// THE GAP: native admission's separate descriptor is exercised below.
#[test]
fn run_uses_captured_prelude_roles_for_every_reported_shadow() {
    let temp = TempDir::new().expect("fixture directory");
    for fence in [Fence::Example, Fence::Plain] {
        let control = write_fixture(
            temp.path(),
            &format!("print-{}-control", fence.label()),
            &fixture(PRINT_MAIN, fence, "zz_unrelated"),
        );
        let expected = run(&control);
        assert!(expected.status.success(), "control must run: {expected:?}");
        assert_eq!(expected.stdout, b"entered main\n");
        for shadow in ["Stdout", "Write", "Vis", "Ret", "Success"] {
            let path = write_fixture(
                temp.path(),
                &format!("print-{}-{shadow}", fence.label()),
                &fixture(PRINT_MAIN, fence, shadow),
            );
            let actual = run(&path);
            assert_eq!(
                actual.status, expected.status,
                "shadow {shadow}: {actual:?}"
            );
            assert_eq!(
                actual.stdout, expected.stdout,
                "shadow {shadow}: {actual:?}"
            );
            assert_eq!(
                actual.stderr, expected.stderr,
                "shadow {shadow}: {actual:?}"
            );
        }
    }
    let control = write_fixture(
        temp.path(),
        "exit-example-control",
        &fixture(EXIT_MAIN, Fence::Example, "zz_unrelated"),
    );
    let shadow = write_fixture(
        temp.path(),
        "exit-example-Success",
        &fixture(EXIT_MAIN, Fence::Example, "Success"),
    );
    let expected = run(&control);
    let actual = run(&shadow);
    assert!(
        expected.status.success(),
        "exit control must run: {expected:?}"
    );
    assert_eq!(
        actual.status, expected.status,
        "shadowed exit ABI: {actual:?}"
    );
    assert_eq!(actual.stdout, expected.stdout);
    assert_eq!(actual.stderr, expected.stderr);
}

/// Promise class: durable invariant. MEASURED: shadowing either constructor
/// used to build the host-provided main arguments in a checked example or a
/// plain fence leaves the same reference result as its unrelated control.
/// CLAIMED: the runner cannot pass source-selected constructors to main.
/// THE GAP: the other effect-harvester arms are exercised above and by their
/// `&PreludeEnv` signature, not by this pair of constructor fixtures.
#[test]
fn runner_main_arguments_use_checked_prelude_constructors() {
    let temp = TempDir::new().expect("fixture directory");
    for fence in [Fence::Example, Fence::Plain] {
        let control = write_fixture(
            temp.path(),
            &format!("exit-{}-control", fence.label()),
            &fixture(EXIT_MAIN, fence, "zz_unrelated"),
        );
        let expected = run(&control);
        assert!(expected.status.success(), "control must run: {expected:?}");
        for shadow in ["MkProcessInput", "MkProgramCaps"] {
            let path = write_fixture(
                temp.path(),
                &format!("exit-{}-{shadow}", fence.label()),
                &fixture(EXIT_MAIN, fence, shadow),
            );
            let actual = run(&path);
            assert_eq!(
                actual.status, expected.status,
                "shadow {shadow}: {actual:?}"
            );
            assert_eq!(
                actual.stdout, expected.stdout,
                "shadow {shadow}: {actual:?}"
            );
            assert_eq!(
                actual.stderr, expected.stderr,
                "shadow {shadow}: {actual:?}"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: shadowed Success in all three
/// runnable forms builds and the linked starter returns the same stdout, stderr
/// and status as the control. Write, Vis and Ret remain runnable in both forms.
/// CLAIMED: native admission consumes the checked prelude Success id rather
/// than the mutable source binding. THE GAP: a separate stable-symbol defect
/// still refuses native Stdout, pinned in its own control below.
#[test]
fn native_admission_and_execution_keep_the_success_identity() {
    let temp = TempDir::new().expect("native fixture directory");
    let profile = temp.path().join("profile.json");
    fs::write(&profile, PROFILE).expect("native resource profile");
    for (main_kind, main, fence) in [
        ("exit", EXIT_MAIN, Fence::Example),
        ("print", PRINT_MAIN, Fence::Example),
        ("print", PRINT_MAIN, Fence::Plain),
    ] {
        let base = format!("{main_kind}-{}", fence.label());
        let control_input = write_fixture(
            temp.path(),
            &format!("{base}-control"),
            &fixture(main, fence, "zz_unrelated"),
        );
        let shadow_input = write_fixture(
            temp.path(),
            &format!("{base}-Success"),
            &fixture(main, fence, "Success"),
        );
        let expected = native_run(
            &control_input,
            &temp.path().join(format!("native-{base}-control")),
            &profile,
        );
        let actual = native_run(
            &shadow_input,
            &temp.path().join(format!("native-{base}-Success")),
            &profile,
        );
        assert!(
            expected.status.success(),
            "native control must run: {expected:?}"
        );
        assert_eq!(
            actual.status, expected.status,
            "shadowed native exit: {actual:?}"
        );
        assert_eq!(actual.stdout, expected.stdout);
        assert_eq!(actual.stderr, expected.stderr);
    }
    for fence in [Fence::Example, Fence::Plain] {
        for shadow in ["Write", "Vis", "Ret"] {
            let name = format!("print-{}-{shadow}", fence.label());
            let source = write_fixture(temp.path(), &name, &fixture(PRINT_MAIN, fence, shadow));
            let result = native_run(
                &source,
                &temp.path().join(format!("native-{name}")),
                &profile,
            );
            assert!(
                result.status.success(),
                "{name} native execution: {result:?}"
            );
            assert_eq!(result.stdout, b"entered main\n", "{name}");
            assert!(result.stderr.is_empty(), "{name}: {result:?}");
        }
    }
}

/// Promise class: durable invariant. MEASURED: reusing Success in the same
/// plain fence as a main whose body names it remains a TypeMismatch at
/// elaboration in both routes, unlike the unchanged unrelated control.
/// CLAIMED: ABI identity repair does not weaken plain-source type checking.
/// THE GAP: no runtime ABI reader is reached in this intentionally ill-typed row.
#[test]
fn plain_success_named_by_main_stays_a_kernel_rejection() {
    let temp = TempDir::new().expect("fixture directory");
    let profile = temp.path().join("profile.json");
    fs::write(&profile, PROFILE).expect("native resource profile");
    let control = write_fixture(
        temp.path(),
        "exit-plain-control",
        &fixture(EXIT_MAIN, Fence::Plain, "zz_unrelated"),
    );
    let shadow = write_fixture(
        temp.path(),
        "exit-plain-Success",
        &fixture(EXIT_MAIN, Fence::Plain, "Success"),
    );
    assert!(run(&control).status.success(), "control must run");
    let rejected = run(&shadow);
    assert!(!rejected.status.success(), "shadow must fail elaboration");
    assert!(
        String::from_utf8_lossy(&rejected.stderr).contains("KernelRejected { error: TypeMismatch"),
        "expected original typed elaboration error: {rejected:?}"
    );
    assert!(
        native_build(&control, &temp.path().join("native-control"), &profile)
            .status
            .success(),
        "native control must build"
    );
    let native = native_build(&shadow, &temp.path().join("native-shadow"), &profile);
    assert!(
        !native.status.success(),
        "native shadow must fail elaboration"
    );
    assert!(
        String::from_utf8_lossy(&native.stderr)
            .contains("elaboration failed: KernelRejected { error: TypeMismatch"),
        "expected same native elaboration error: {native:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a prelude constructor's
/// shadowed spelling in either fence form and its parent family's shadowed
/// spelling in the plain form build and execute like their unrelated control.
/// CLAIMED: native stable symbols name pre-source compiler identities, while
/// a colliding user declaration remains distinct. THE GAP: the producer unit
/// test checks the exact symbols; this test observes the native consumer.
#[test]
fn native_stdout_and_stream_shadows_run_like_unrelated_control() {
    let temp = TempDir::new().expect("native fixture directory");
    let profile = temp.path().join("profile.json");
    fs::write(&profile, PROFILE).expect("native resource profile");
    for fence in [Fence::Example, Fence::Plain] {
        let control = write_fixture(
            temp.path(),
            &format!("print-{}-control", fence.label()),
            &fixture(PRINT_MAIN, fence, "zz_unrelated"),
        );
        let expected = native_run(
            &control,
            &temp
                .path()
                .join(format!("native-{}-control", fence.label())),
            &profile,
        );
        assert!(expected.status.success(), "native control: {expected:?}");
        assert_eq!(expected.stdout, b"entered main\n");
        let shadows: &[&str] = match fence {
            Fence::Example => &["Stdout"],
            Fence::Plain => &["Stdout", "Stream"],
        };
        for shadow in shadows {
            let source = write_fixture(
                temp.path(),
                &format!("print-{}-{shadow}", fence.label()),
                &fixture(PRINT_MAIN, fence, shadow),
            );
            let actual = native_run(
                &source,
                &temp
                    .path()
                    .join(format!("native-{}-{shadow}", fence.label())),
                &profile,
            );
            assert_eq!(actual.status, expected.status, "{shadow}: {actual:?}");
            assert_eq!(actual.stdout, expected.stdout, "{shadow}: {actual:?}");
            assert_eq!(actual.stderr, expected.stderr, "{shadow}: {actual:?}");
        }
    }
}
