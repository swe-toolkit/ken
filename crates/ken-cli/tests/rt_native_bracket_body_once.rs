//! Checked bracket bodies perform each reached host effect once, in source
//! order, under both execution engines. The interpreter is the independent
//! observation oracle; the expected prints prevent two empty traces from
//! agreeing vacuously.

#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_os = "linux")]
fn assert_bracket_body_observation(
    name: &str,
    source: &str,
    expected_prints: &[&[u8]],
    expected_releases: usize,
) {
    let dir = tempfile::Builder::new()
        .prefix(&format!("rt-bracket-body-once-{name}-"))
        .tempdir()
        .expect("creates isolated bracket fixture root");
    std::fs::write(dir.path().join("held-0.bin"), b"held resource 0")
        .expect("provisions checked resource");

    let mut host = ken_interp::PosixHost::new_at(dir.path());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        dir.path().as_os_str().as_bytes(),
        &mut host,
    )
    .expect("checked bracket executes in the interpreter");
    let output = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        &format!("rt-bracket-body-once-{name}"),
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("checked bracket builds as a linked native artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked native bracket executes without a trap");

    assert_eq!(
        (
            native.exit_status,
            &native.terminal_error,
            &native.terminal_exit,
            &native.stdout,
            &native.stderr,
            &native.effect_trace,
        ),
        (
            interpreted.exit_status,
            &interpreted.terminal_error,
            &interpreted.terminal_exit,
            &interpreted.stdout,
            &interpreted.stderr,
            &interpreted.effect_trace,
        ),
        "{name}: full ordered native/interpreter observation, including resource identities",
    );
    assert_eq!((native.exit_status, native.terminal_error), (0, None));
    let prints = native
        .effect_trace
        .iter()
        .filter_map(|event| match &event.request {
            ken_runtime::CanonicalRequestV1::ConsoleWrite { bytes, .. } => Some(bytes.as_slice()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        prints, expected_prints,
        "{name}: each intended print occurs once"
    );
    let releases = native
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .count();
    assert_eq!(releases, expected_releases, "{name}: bracket releases once");
}

/// Promise class: durable behavioral invariant (`42 §6.2–6.4`, `45 §1`).
/// MEASURED: the exact checked resource body has two ordered console
/// performances and one release in both
/// engines. CLAIMED: neither the inline route nor the returned-Vis owner
/// re-performs a body effect. THE GAP: these fixed source shapes do not exhaust
/// arbitrary bracket bodies; the three-print, buffer and lookalike siblings
/// below vary that route while CI guards the stable observation relation.
#[cfg(target_os = "linux")]
#[test]
fn resource_bracket_body_performs_two_prints_once() {
    assert_bracket_body_observation(
        "resource",
        include_str!("fixtures/rt_bracket_body_once_resource.ken"),
        &[b"x\n", b"y\n"],
        1,
    );
}

/// Promise class: durable behavioral invariant (`42 §6.4`, `45 §1`).
/// The third print discriminates a cascading suffix replay from one repeated
/// call site; the ordered host trace is the oracle, not a planner edge count.
#[cfg(target_os = "linux")]
#[test]
fn resource_bracket_body_performs_three_prints_once() {
    assert_bracket_body_observation(
        "three",
        include_str!("fixtures/rt_bracket_body_once_three.ken"),
        &[b"x\n", b"y\n", b"z\n"],
        1,
    );
}

/// Promise class: durable behavioral invariant (`42 §6.4`, `45 §1`).
/// A buffer-backed bracket uses the same perform-once rule as the FS handle.
#[cfg(target_os = "linux")]
#[test]
fn buffer_bracket_body_performs_two_prints_once() {
    assert_bracket_body_observation(
        "buffer",
        include_str!("fixtures/rt_bracket_body_once_buffer.ken"),
        &[b"x\n", b"y\n"],
        1,
    );
}

/// Promise class: durable behavioral invariant (`42 §6.4`, `45 §1`).
/// A source-level open/body/close lookalike inside a real resource bracket
/// must not duplicate its synthetic close effect.
#[cfg(target_os = "linux")]
#[test]
fn bracket_free_lookalike_inside_resource_performs_once() {
    assert_bracket_body_observation(
        "lookalike",
        include_str!("fixtures/rt_bracket_body_once_lookalike.ken"),
        &[b"o1\n", b"c1\n"],
        1,
    );
}

/// Promise class: durable behavioral invariant (`42 §6.4`, `45 §1`).
/// This negative control shares two Console Vis nodes but has no bracket
/// owner, and therefore neither acquires nor releases a resource.
#[cfg(target_os = "linux")]
#[test]
fn unbracketed_two_prints_do_not_acquire_or_release() {
    assert_bracket_body_observation(
        "no-bracket",
        include_str!("fixtures/rt_bracket_body_once_no_bracket.ken"),
        &[b"x\n", b"y\n"],
        0,
    );
}
