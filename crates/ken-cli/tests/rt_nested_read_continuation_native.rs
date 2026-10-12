#![cfg(target_os = "linux")]

use std::os::unix::ffi::OsStringExt;
use std::sync::Mutex;

// Spec: 42 §3.3, §5; 45 §4. Promise class: durable native/interpreter
// differential over the selected second-read continuation, including its
// first-response capture. The checked single-read source and the nested-only
// result are separate controls. Each file is a fixed source fixture; the
// oracle is the independently executed interpreter, not a native constant.
// MEASURED: actual ordered FS operations, stdout, exit and terminal state on
// both empty and nonempty payloads for every row. CLAIMED: these reads run at
// interpreter parity instead of trapping on a carried byte or lost Ret.
// THE GAP: these six sources do not enumerate every generated-unit shape.
// The captured-read Ret is in a generated context with no selected terminal
// origins. Deleting root_unit alone leaves all six rows green; this suite does
// not claim to pin that conjunct. The direct scalar-consumer test
// `root_unit_and_result_origin_independently_guard_ret_scalar_join` pins it
// with a seeded selected nonroot/root pair. Surface-source reachability of a
// selected nonroot Ret scalar join remains review-enforced. Reinstating the
// prior object-wide Ret unwrap or either old byte-class guard reddens its
// respective rows; an intended new byte class can retain parity.
//
// Baseline provisioning: the landed single-read fixture uses 256 MiB because
// its native build exceeded the default 2 MiB and passed at 16 MiB. This
// stated Builder stack provisions the same workload, not a default-stack claim.
const BUILD_STACK_BYTES: usize = 256 * 1024 * 1024;
static BUILD_LOCK: Mutex<()> = Mutex::new(());

fn assert_row(
    row: &'static str,
    source: &'static str,
    exits: [i32; 4],
    read_count: usize,
    console: bool,
) {
    let _serial = BUILD_LOCK
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    std::thread::Builder::new()
        .name(format!("nested-read-{row}"))
        .stack_size(BUILD_STACK_BYTES)
        .spawn(move || assert_row_on_stated_stack(row, source, exits, read_count, console))
        .expect("spawn stated-stack fixture")
        .join()
        .expect("stated-stack fixture");
}

fn assert_row_on_stated_stack(
    row: &str,
    source: &str,
    exits: [i32; 4],
    read_count: usize,
    console: bool,
) {
    let dir = tempfile::tempdir().expect("private test directory");
    let build = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        &format!("rt-nested-read-{row}"),
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .unwrap_or_else(|error| panic!("{row}: native build: {error:?}"));
    for (index, (name, contents)) in [
        ("empty", Vec::new()),
        ("zero", vec![0u8]),
        ("nine", vec![9u8]),
        ("max", vec![255u8]),
    ]
    .into_iter()
    .enumerate()
    {
        std::fs::write(dir.path().join("ac0.bin"), contents).expect("FS input");
        let args = vec![std::ffi::OsString::from_vec(vec![2u8])];
        let native = ken_runtime::run_bound_process_effect_observation(
            &build.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: args,
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: build.plan_transport_hash,
            },
        )
        .unwrap_or_else(|error| panic!("{row}/{name}: native run: {error:?}"));
        let mut host = ken_interp::PosixHost::new_at(dir.path());
        let interpreted = ken_cli::run_program_effect_observation(
            source,
            ken_cli::SourceFormat::Ken,
            &[b"ken".to_vec(), vec![2u8]],
            &[],
            dir.path().as_os_str().as_encoded_bytes(),
            &mut host,
        )
        .unwrap_or_else(|error| panic!("{row}/{name}: interpreter: {error:?}"));
        assert_eq!(
            interpreted.exit_status, exits[index],
            "{row}/{name}: oracle arm"
        );
        assert_eq!(
            native.exit_status, interpreted.exit_status,
            "{row}/{name}: exit"
        );
        assert_eq!(native.stdout, interpreted.stdout, "{row}/{name}: stdout");
        assert_eq!(native.stderr, interpreted.stderr, "{row}/{name}: stderr");
        assert_eq!(
            native.terminal_error, interpreted.terminal_error,
            "{row}/{name}: terminal"
        );
        assert_eq!(
            native.effect_trace, interpreted.effect_trace,
            "{row}/{name}: ordered effects"
        );
        assert_eq!(
            native.filesystem_delta, interpreted.filesystem_delta,
            "{row}/{name}: FS delta"
        );
        let actual: Vec<_> = native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect();
        let mut expected = vec![ken_runtime::HostOpV1::FsReadFile; read_count];
        if console {
            expected.push(ken_runtime::HostOpV1::ConsoleWrite);
        }
        assert_eq!(actual, expected, "{row}/{name}: ordered operations");
    }
}

#[test]
fn at_more_reads_second_payload() {
    assert_row(
        "at-more",
        include_str!("fixtures/rt_nested_read_at_more.ken"),
        [51, 1, 9, 255],
        2,
        false,
    );
}

#[test]
fn at_b2_reads_captured_first_payload() {
    assert_row(
        "at-b2",
        include_str!("fixtures/rt_nested_read_at_b2.ken"),
        [52, 1, 9, 255],
        2,
        false,
    );
}

#[test]
fn length_more_reads_second_payload() {
    assert_row(
        "length-more",
        include_str!("fixtures/rt_nested_read_length_more.ken"),
        [54, 53, 53, 53],
        2,
        false,
    );
}

#[test]
fn captured_first_response_keeps_ret() {
    assert_row(
        "captured-read",
        include_str!("fixtures/rt_nested_read_captured_read.ken"),
        [56; 4],
        2,
        false,
    );
}

#[test]
fn nested_continuation_without_primitive_control() {
    assert_row(
        "nested-control",
        include_str!("fixtures/rt_nested_read_nested_control.ken"),
        [2; 4],
        2,
        false,
    );
}

#[test]
fn landed_single_read_control() {
    assert_row(
        "single-read-control",
        include_str!("fixtures/rt_nested_read_single_read_control.ken"),
        [7; 4],
        1,
        true,
    );
}
