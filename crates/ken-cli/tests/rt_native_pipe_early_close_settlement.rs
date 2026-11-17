//! Native/interpreter parity for both child pipe ends closed in the body
//! before waiting, compared with their no-close controls.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

use ken_host::{
    CanonicalOutcomeV1 as Outcome, CanonicalReplyV1 as Reply, CanonicalRequestV1 as Request,
    HostOpV1 as Op, ResourceBindingRole as Role, ResourceKindV1 as Kind,
    ResourceSettlementOutcomeV1 as Settlement,
};

const EARLY_CLOSE_READ: &str = include_str!("fixtures/rt_native_pipe_early_close_read.ken");
const NO_CLOSE_READ: &str = include_str!("fixtures/rt_native_pipe_no_close_read.ken");
const EARLY_CLOSE_WRITE: &str = include_str!("fixtures/rt_native_pipe_early_close_write.ken");
const NO_CLOSE_WRITE: &str = include_str!("fixtures/px10_take_stdin_no_close.ken");

fn checked_catalog_roots() -> Vec<std::path::PathBuf> {
    vec![std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn compare_pipe_end(
    source: &str,
    case: &str,
    expected_kind: Kind,
    expected_fd: u64,
    closes_before_wait: bool,
) {
    let root = tempfile::tempdir().expect("isolated early-close root");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let reference = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &checked_catalog_roots(),
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("reference interpreter runs the checked child bracket");
    let built = ken_cli::build_native_program_with_roots(
        source,
        ken_cli::SourceFormat::Ken,
        &checked_catalog_roots(),
        case,
        root.path().join("native"),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("native build admits the checked child bracket");
    let native = ken_runtime::run_bound_process_effect_observation(
        &built.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: built.plan_transport_hash,
        },
    )
    .expect("native child bracket runs");

    let first_difference = reference
        .effect_trace
        .iter()
        .zip(&native.effect_trace)
        .position(|(left, right)| left != right)
        .or_else(|| {
            (reference.effect_trace.len() != native.effect_trace.len())
                .then_some(reference.effect_trace.len().min(native.effect_trace.len()))
        });
    if let Some(index) = first_difference {
        panic!(
            "{case}: canonical effect mismatch at {index}: reference={:?}, native={:?}; exit status reference={} native={}; terminal exit reference={:?} native={:?}; terminal error reference={:?} native={:?}",
            reference.effect_trace.get(index), native.effect_trace.get(index),
            reference.exit_status, native.exit_status,
            reference.terminal_exit, native.terminal_exit,
            reference.terminal_error, native.terminal_error
        );
    }
    assert_eq!(
        native.exit_status, reference.exit_status,
        "{case}: exit status; reference trace={:?}, native trace={:?}",
        reference.effect_trace, native.effect_trace,
    );
    assert_eq!(
        native.terminal_exit, reference.terminal_exit,
        "{case}: terminal exit"
    );
    assert_eq!(
        native.terminal_error, reference.terminal_error,
        "{case}: terminal error"
    );
    assert_eq!(native.exit_status, 0, "{case}: bracket must succeed");
    assert_eq!(native.terminal_error, None, "{case}: terminal error");

    let trace = &native.effect_trace;
    let acquired = trace
        .iter()
        .enumerate()
        .filter_map(|(index, event)| match (&event.operation, &event.outcome) {
            (
                Op::ProcessTakePipe,
                Outcome::Success(Reply::ResourceAcquired {
                    resource_kind,
                    identity,
                    ..
                }),
            ) if *resource_kind == expected_kind => Some((index, *identity)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(acquired.len(), 1, "{case}: one acquired pipe end");
    assert_eq!(
        trace
            .iter()
            .filter(|event| event.operation == Op::ProcessTakePipe)
            .count(),
        1,
        "{case}: no other take may be hidden by the successful-acquisition filter"
    );
    let (take_index, pipe_identity) = acquired[0];
    let take = &trace[take_index];
    assert!(
        matches!(&take.request, Request::ProcessTakePipe { fd } if *fd == expected_fd),
        "{case}: take the requested child pipe end: {take:?}"
    );
    assert!(
        take.resource_bindings
            .contains(&(Role::Buffer, pipe_identity)),
        "{case}: acquisition binds the same pipe-end identity: {take:?}"
    );
    let child_identity = take
        .resource_bindings
        .iter()
        .find_map(|(role, identity)| (*role == Role::Target).then_some(*identity))
        .expect("stdout take must carry its child target");
    let waits = trace
        .iter()
        .enumerate()
        .filter(|(_, event)| event.operation == Op::ProcessWait)
        .collect::<Vec<_>>();
    assert_eq!(waits.len(), 1, "{case}: one completed wait");
    let (wait_index, wait) = waits[0];
    assert!(
        matches!(
            wait.outcome,
            Outcome::Success(Reply::ChildExit(ken_host::ChildExitV1::Exited(0)))
        ),
        "{case}: child exits normally: {wait:?}"
    );
    let releases = trace
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            event.operation == Op::ResourceRelease
                && event
                    .resource_bindings
                    .contains(&(Role::Target, pipe_identity))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        releases.len(),
        1,
        "{case}: body early close or bracket, never both, settles the taken identity once"
    );
    let (release_index, release) = releases[0];
    assert!(
        take_index < release_index,
        "{case}: settlement follows acquisition"
    );
    assert!(
        matches!(&release.outcome,
        Outcome::Success(Reply::ResourceSettlement(observed))
            if observed.resource_kind == expected_kind
                && observed.identity == pipe_identity
                && observed.outcome == Settlement::Released),
        "{case}: exact pipe-end settlement: {release:?}"
    );
    assert_eq!(
        release_index < wait_index,
        closes_before_wait,
        "{case}: body close precedes wait, bracket-only settlement follows it"
    );
    let child_releases = trace
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            event.operation == Op::ResourceRelease
                && event
                    .resource_bindings
                    .contains(&(Role::Target, child_identity))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        child_releases.len(),
        1,
        "{case}: bracket must also settle its child exactly once"
    );
    let (child_release_index, child_release) = child_releases[0];
    assert!(
        child_release_index > wait_index,
        "{case}: bracket settles the child after the body's wait"
    );
    assert!(
        release_index < child_release_index,
        "{case}: pipe-end release precedes ChildProcess settlement"
    );
    assert!(
        matches!(&child_release.outcome,
        Outcome::Success(Reply::ResourceSettlement(observed))
            if observed.resource_kind == Kind::ChildProcess
                && observed.identity == child_identity
                && observed.outcome == Settlement::Released),
        "{case}: bracket's exact child settlement: {child_release:?}"
    );
}

/// Promise class: durable invariant. MEASURED: one checked child stdout end
/// is acquired and closed before the wait, with identical complete ordered
/// canonical effects and terminal observations on both engines. CLAIMED: an
/// early close is performed once, and the bracket accepts its already-closed
/// end instead of releasing it again or returning a release error. THE GAP:
/// this one-child fixture does not witness a repeated body close.
#[test]
fn early_closed_stdout_settles_once_and_matches_interpreter() {
    // Existing PX10 Process smoke measured a 2308 KiB resident worker stack;
    // the stated 16 MiB provides at least 14076 KiB headroom to that baseline.
    std::thread::Builder::new()
        .name("rt-pipe-close-read".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            compare_pipe_end(
                EARLY_CLOSE_READ,
                "early-close-read",
                Kind::PipeRead,
                1,
                true,
            )
        })
        .expect("stated early-close worker")
        .join()
        .expect("early-close worker completed");
}

/// Promise class: durable invariant. MEASURED: without the explicit close,
/// the same checked child stdout acquisition and wait have native/interpreter
/// parity and the bracket releases the end exactly once after the wait.
/// CLAIMED: narrowing the relay key does not break the positive no-close
/// route. THE GAP: this fixture covers one child, not every endpoint context.
#[test]
fn no_close_stdout_settles_at_bracket_and_matches_interpreter() {
    std::thread::Builder::new()
        .name("rt-pipe-no-close-read".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| compare_pipe_end(NO_CLOSE_READ, "no-close-read", Kind::PipeRead, 1, false))
        .expect("stated no-close worker")
        .join()
        .expect("no-close worker completed");
}

/// Promise class: durable invariant. MEASURED: one checked child stdin end is
/// acquired as PipeWrite with fd 0 and closed before the wait. Both engines
/// agree on the entire ordered canonical trace and terminal result, including
/// the released pipe identity before the child's bracket settlement. CLAIMED:
/// an early close does not lose ownership or duplicate settlement. THE GAP:
/// this does not test a write after closing the pipe or a repeated body close.
#[test]
fn early_closed_stdin_settles_before_child_and_matches_interpreter() {
    std::thread::Builder::new()
        .name("rt-pipe-close-write".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            compare_pipe_end(
                EARLY_CLOSE_WRITE,
                "early-close-write",
                Kind::PipeWrite,
                0,
                true,
            )
        })
        .expect("stated write-end early-close worker")
        .join()
        .expect("write-end early-close worker completed");
}

/// Promise class: durable invariant. MEASURED: without explicit close, the
/// checked stdin PipeWrite end is settled once after the wait and before the
/// child, with interpreter/native full-trace parity. CLAIMED: narrowing the
/// relay key does not break the stdin no-close control. THE GAP: no write is
/// issued, so this does not witness ResourceStreamWrite or post-close use.
#[test]
fn no_close_stdin_settles_at_bracket_and_matches_interpreter() {
    std::thread::Builder::new()
        .name("rt-pipe-no-close-write".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| compare_pipe_end(NO_CLOSE_WRITE, "no-close-write", Kind::PipeWrite, 0, false))
        .expect("stated write-end no-close worker")
        .join()
        .expect("write-end no-close worker completed");
}
