#![cfg(target_os = "linux")]

use std::os::unix::ffi::OsStringExt;
use std::sync::Mutex;

// Spec: 42 §3.3, §5; 45 §4. PX11 Inc2 entry 15 (Architect ruling relayed in
// evt_3eddv6t6byx5f). Promise class: durable native/interpreter differential.
// MEASURED: a generated continuation context's worker body calls
// `bytes_to_list` on a carried `Bytes` (the second read's payload) and its
// result list selects the exit (Nil -> 57, Cons head _ -> head); ordered FS
// operations, stdout, stderr, exit, terminal state and FS delta agree with the
// independently executed interpreter on empty and nonempty payloads.
// CLAIMED: the planner issues the synthesized Nil/Cons records for the
// generated context's enclosing specialization, so native lowering of that
// call does not refuse with N0003. THE GAP: one generated-context shape and
// one primitive; the exchange covers the producing program at scale.
//
// Baseline provisioning: the harness and its stated 256 MiB Builder stack are
// the nested-read suite's, which this fixture is derived from; it provisions
// the same workload, not a default-stack claim.
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
        .name(format!("gen-ctx-btl-{row}"))
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
    let build = ken_cli::build_native_program_with_roots(
        source,
        ken_cli::SourceFormat::Ken,
        &checked_catalog_roots(),
        &format!("rt-gen-ctx-btl-{row}"),
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
        &checked_catalog_roots(),
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
fn generated_context_bytes_to_list_matches_interpreter() {
    assert_row(
        "bytes-to-list",
        include_str!("fixtures/rt_generated_context_bytes_to_list.ken"),
        [57, 1, 9, 255],
        2,
        false,
    );
}

fn checked_catalog_roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")]
}
