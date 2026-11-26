use super::*;
use crate::{
    AmbientScript, NativeTestedEvidence, confirm_native_tested_transition,
    scenario::{CheckedProgramEntry, Scenario, execute_scenario},
};
use ken_host::HostOpAvailabilityV1::NativeTested;

const DELAY: u64 = 20_000_000;

fn run(source: String) -> CanonicalDifferentialRun {
    execute_scenario(&Scenario {
        boundary_resource_profile: ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        process_input: Default::default(),
        ambient: AmbientScript {
            use_real_monotonic_clock: true,
            use_real_sleep: true,
            use_real_entropy: true,
            ..AmbientScript::default()
        },
        program_caps: Default::default(),
        entry: CheckedProgramEntry {
            catalog_roots: vec![
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages"),
            ],
            identity: "clock-promotion".into(),
            package_name: "clock-promotion".into(),
            source,
        },
        initial_filesystem: Vec::new(),
        expected_fs: Vec::new(),
    })
    .expect("both real lanes produce observations")
}

fn domain_source(deadline: &str) -> String {
    r#"program capabilities FS AFull
import Capability.Process.Exit (host_bind, host_clock, host_console, host_pure)
import Capability.Console.Text (print)
import Capability.Time.Monotonic (monotonic_now, sleep_until)
proc main (_input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [Clock, Console] =
  host_bind AFull MonotonicInstant ExitCode
    (host_clock AFull MonotonicInstant monotonic_now)
    (\_reading. host_bind AFull (Result IOError Unit) ExitCode
      (host_console AFull (Result IOError Unit) (print "x"))
      (\_marker. host_bind AFull Unit ExitCode
        (host_clock AFull Unit (sleep_until (Deadline.MkDeadline (MonotonicInstant.MkMonotonicInstant DEADLINE))))
        (\_unit. host_pure AFull ExitCode ExitCode.Success)))
"#.replace("DEADLINE", deadline)
}

/// Durable invariant: both refused operands reach the same trap class without
/// dispatching sleep. Per-lane terminal identity and diagnostics remain exact.
#[test]
fn d_neg_is_a_typed_pre_dispatch_trap() {
    let run = run(domain_source("(sub_int 0 1)"));
    run.compare_sleep_until_out_of_domain().unwrap();
    assert_eq!(run.interpreter.stdout, b"x");
    assert_eq!(run.interpreter.exit_status, 1);
}

#[test]
fn d_over_is_a_typed_pre_dispatch_trap() {
    let run = run(domain_source("18446744073709551616"));
    run.compare_sleep_until_out_of_domain().unwrap();
    assert_eq!(run.interpreter.stdout, b"x");
    assert_eq!(run.interpreter.exit_status, 1);
}

#[test]
fn d_zero_dispatches_unit_and_returns() {
    let run = run(domain_source("0"));
    let project = |lane, observation: &EffectObservation, window| {
        let mut observation = observation.clone();
        assert_eq!(observation.terminal_exit, TerminalExitClass::NormalReturn);
        assert_eq!(observation.terminal_error, None);
        assert_eq!(observation.exit_status, 0);
        assert!(observation.stderr.is_empty());
        assert_eq!(observation.stdout, b"x");
        let [before, marker, sleep] = observation.effect_trace.as_mut_slice() else {
            panic!("{lane}: expected the complete three-event trace");
        };
        project_monotonic_reading(lane, before, 0, window).unwrap();
        assert!(ambient_event(marker, 1, Op::ConsoleWrite));
        assert!(ambient_event(sleep, 2, Op::ClockSleepUntil));
        assert_eq!(sleep.request, Request::ClockSleepUntil { deadline: 0 });
        assert_eq!(sleep.outcome, Outcome::Success(Reply::Unit));
        observation
    };
    compare_canonical_exact(
        &project(
            "interpreter",
            &run.interpreter,
            run.interpreter_monotonic_window,
        ),
        &project("native", &run.native, run.native_monotonic_window),
    )
    .unwrap();
}

#[test]
fn domain_comparison_rejects_each_transport_perturbation() {
    use MonotonicReadingError as ReadError;
    use SleepUntilDomainDifferentialError as Error;
    let run = run(domain_source("(sub_int 0 1)"));
    run.compare_sleep_until_out_of_domain().unwrap();
    let check = |native: &EffectObservation| {
        compare_sleep_domain(
            &run.interpreter,
            run.interpreter_monotonic_window,
            native,
            run.native_monotonic_window,
        )
    };
    let mut native = run.native.clone();
    native.effect_trace.push(EffectEvent {
        sequence: 2,
        operation: Op::ClockSleepUntil,
        capability: None,
        resource_bindings: Vec::new(),
        request: Request::ClockSleepUntil { deadline: 0 },
        outcome: Outcome::Success(Reply::Unit),
    });
    assert!(matches!(
        check(&native),
        Err(Error::Reading(ReadError::TraceShape { lane: "native", .. }))
    ));

    let mut native = run.native.clone();
    native.effect_trace[0].outcome = Outcome::Success(Reply::MonotonicInstant(
        (BigInt::from(run.native_monotonic_window.end) + BigInt::from(1)).to_signed_bytes_be(),
    ));
    assert!(matches!(
        check(&native),
        Err(Error::Reading(ReadError::OutsidePlausibleWindow {
            lane: "native",
            ..
        }))
    ));

    let mut native = run.native.clone();
    let Request::ConsoleWrite { bytes, .. } = &mut native.effect_trace[1].request else {
        panic!()
    };
    bytes[0] ^= 1;
    assert_eq!(check(&native), Err(Error::TraceMismatch));

    for (start, length) in [
        (run.interpreter.stderr.len(), STARTER_LINE.len()),
        (
            run.interpreter.stderr.len() + STARTER_LINE.len(),
            run.native.stderr.len() - run.interpreter.stderr.len() - STARTER_LINE.len(),
        ),
    ] {
        let mut native = run.native.clone();
        native.stderr.drain(start..start + length);
        assert_eq!(check(&native), Err(Error::StderrMismatch));
    }
}

#[test]
fn real_monotonic_now_confirms_promotion() {
    use MonotonicNowDifferentialError as Error;
    let mut run = run(r#"program capabilities FS AFull
import Capability.Process.Exit (host_bind, host_clock, host_pure)
import Capability.Time.Monotonic (monotonic_now)
proc main (_input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [Clock] =
  host_bind AFull MonotonicInstant ExitCode
    (host_clock AFull MonotonicInstant monotonic_now)
    (\first. match first {
      MonotonicInstant.MkMonotonicInstant a |->
        host_bind AFull MonotonicInstant ExitCode
          (host_clock AFull MonotonicInstant monotonic_now)
          (\second. match second {
            MonotonicInstant.MkMonotonicInstant b |->
              match leq_int a b {
                False |-> host_pure AFull ExitCode (ExitCode.Failure 32);
                True |-> host_pure AFull ExitCode ExitCode.Success
              }
          })
    })
"#
    .into());
    assert_eq!(run.native.exit_status, 0);
    let evidence =
        |run: &CanonicalDifferentialRun| NativeTestedEvidence::from_monotonic_now_run(run);
    let check = |run: &CanonicalDifferentialRun| {
        confirm_native_tested_transition(Op::ClockMonotonicNow, evidence(run))
    };
    assert_eq!(check(&run), Ok(NativeTested));
    let original = run.native.clone();
    run.native.effect_trace[0].request = Request::ClockWallNow;
    assert!(matches!(
        run.compare_monotonic_now(),
        Err(Error::Reading(MonotonicReadingError::TraceShape { .. }))
    ));
    assert!(check(&run).is_err());
    run.native = original.clone();
    run.native.effect_trace[0].outcome = Outcome::Success(Reply::MonotonicInstant(Vec::new()));
    assert!(check(&run).is_err());
    run.native = original.clone();
    run.native.effect_trace[0].outcome = Outcome::Success(Reply::MonotonicInstant(vec![1]));
    run.native.effect_trace[1].outcome = Outcome::Success(Reply::MonotonicInstant(vec![0]));
    assert_eq!(
        run.compare_monotonic_now(),
        Err(Error::WentBackwards { lane: "native" })
    );
    assert!(check(&run).is_err());
    run.native = original;
    run.native.effect_trace[1].outcome = Outcome::Success(Reply::MonotonicInstant(vec![0x7f; 16]));
    assert!(matches!(
        run.compare_monotonic_now(),
        Err(Error::Reading(
            MonotonicReadingError::OutsidePlausibleWindow { .. }
        ))
    ));
    assert!(check(&run).is_err());
}

#[test]
fn real_sleep_honours_deadline_and_confirms_promotion() {
    let mut run = run(format!(
        r#"program capabilities FS AFull
import Capability.Process.Exit (host_bind, host_clock, host_pure)
import Capability.Time.Monotonic (monotonic_now, sleep_until)
proc main (_input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [Clock] =
  host_bind AFull MonotonicInstant ExitCode
    (host_clock AFull MonotonicInstant monotonic_now)
    (\first. match first {{
      MonotonicInstant.MkMonotonicInstant a |->
        host_bind AFull Unit ExitCode
          (host_clock AFull Unit (sleep_until (Deadline.MkDeadline (MonotonicInstant.MkMonotonicInstant (add_int a {DELAY})))))
          (\_unit. host_bind AFull MonotonicInstant ExitCode
            (host_clock AFull MonotonicInstant monotonic_now)
            (\_second. host_pure AFull ExitCode ExitCode.Success))
    }})
"#
    ));
    assert_eq!(run.native.exit_status, 0);
    let check = |run: &CanonicalDifferentialRun| {
        confirm_native_tested_transition(
            Op::ClockSleepUntil,
            NativeTestedEvidence::from_sleep_until_run(run, DELAY),
        )
    };
    assert_eq!(check(&run), Ok(NativeTested));
    let Request::ClockSleepUntil { deadline } = run.native.effect_trace[1].request else {
        panic!()
    };
    run.native.effect_trace[1].request = Request::ClockSleepUntil {
        deadline: deadline + 1,
    };
    assert!(check(&run).is_err());
    run.native.effect_trace[1].request = Request::ClockSleepUntil { deadline };
    let before = run.native.effect_trace[0].outcome.clone();
    run.native.effect_trace[2].outcome = before;
    assert_eq!(
        run.compare_sleep_until(DELAY),
        Err(SleepUntilDifferentialError::ReturnedBeforeDeadline { lane: "native" })
    );
    assert!(check(&run).is_err());
}

#[test]
fn real_entropy_checks_exact_lengths_and_confirms_promotion() {
    let excess = MAX_ENTROPY_REQUEST_BYTES_V1 + 1;
    let mut run = run(format!(
        r#"program capabilities FS AFull
import Capability.Process.Exit (host_bind, host_entropy, host_pure)
import Capability.System.Entropy (random_bytes)
proc main (_input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [Entropy] =
  host_bind AFull (Result IOError Bytes) ExitCode
    (host_entropy AFull (Result IOError Bytes) (random_bytes 32))
    (\_a. host_bind AFull (Result IOError Bytes) ExitCode
      (host_entropy AFull (Result IOError Bytes) (random_bytes 32))
      (\_b. host_bind AFull (Result IOError Bytes) ExitCode
        (host_entropy AFull (Result IOError Bytes) (random_bytes 0))
        (\_empty. host_bind AFull (Result IOError Bytes) ExitCode
          (host_entropy AFull (Result IOError Bytes) (random_bytes {excess}))
          (\high. match high {{
            Err _ |-> host_bind AFull (Result IOError Bytes) ExitCode
              (host_entropy AFull (Result IOError Bytes) (random_bytes (sub_int 0 1)))
              (\negative. match negative {{
                Err err |-> match err {{
                  IOError.InvalidInput |-> host_pure AFull ExitCode ExitCode.Success;
                  IOError.NotFound |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.PermissionDenied |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.CapabilityDenied |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.BrokenPipe |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.Interrupted |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.AlreadyExists |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.IsDirectory |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.NotDirectory |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.NotEmpty |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.Unsupported |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.Revoked |-> host_pure AFull ExitCode (ExitCode.Failure 34);
                  IOError.Other _ |-> host_pure AFull ExitCode (ExitCode.Failure 34)
                }};
                Ok _ |-> host_pure AFull ExitCode (ExitCode.Failure 35)
              }});
            Ok _ |-> host_pure AFull ExitCode (ExitCode.Failure 33)
          }}))))
"#
    ));
    assert_eq!(run.native.exit_status, 0);
    let check = |run: &CanonicalDifferentialRun| {
        confirm_native_tested_transition(
            Op::EntropyRandomBytes,
            NativeTestedEvidence::from_random_bytes_run(run),
        )
    };
    assert_eq!(check(&run), Ok(NativeTested));
    let original = run.native.clone();
    let first = run.native.effect_trace[0].outcome.clone();
    run.native.effect_trace[1].outcome = first;
    assert_eq!(
        run.compare_random_bytes(),
        Err(RandomBytesDifferentialError::RepeatedBytes { lane: "native" })
    );
    assert!(check(&run).is_err());
    run.native = original.clone();
    run.native.effect_trace[0].outcome = Outcome::Success(Reply::Bytes(vec![0; 31]));
    assert!(check(&run).is_err());
    run.native = original;
    run.native.effect_trace[3].outcome = Outcome::Success(Reply::Bytes(vec![0; excess as usize]));
    assert!(check(&run).is_err());
}
