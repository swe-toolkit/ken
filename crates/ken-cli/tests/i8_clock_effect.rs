//! I-8 wall-clock effect acceptance through the public injected runner.

use ken_cli::{run_program, ProgramOutcome, SourceFormat};
use ken_interp::{CaptureHost, ClockTrace, HostHandler};

const CLOCK_PACKAGE: &str =
    include_str!("../../../catalog/packages/Capability/Time/WallClock.ken.md");

const CLOCK_BODY: &str = r#"
  host_bind APartial Instant ExitCode
    (host_clock APartial Instant wall_now)
    (\first .
      match first {
        Instant.MkInstant _firstNanoseconds |->
          host_bind APartial Instant ExitCode
            (host_clock APartial Instant wall_now)
            (\second .
              match second {
                Instant.MkInstant _secondNanoseconds |-> host_pure APartial ExitCode ExitCode.Success
              })
      })
"#;

fn clock_program(visits: &str) -> String {
    format!(
        "program capabilities FS APartial\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode {visits} =\n{CLOCK_BODY}"
    )
}

/// Promise class: durable invariant. MEASURED: the actual two-read Clock
/// source checks under Strict only with its exact Clock and HostIO package
/// imports, and still refuses an undeclared Clock row. CLAIMED: the H homes
/// preserve the checked effect contract without making wall_now ambient.
/// THE GAP: host values/order are asserted by the separate CaptureHost run.
#[test]
fn strict_clock_user_requires_homes_and_still_checks_the_clock_row() {
    let catalog = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let root_import = "import Capability.Process.Exit (host_bind, host_pure, host_clock)\n";
    let wall_import = "import Capability.Time.WallClock (wall_now)\n";
    let composed = |visits: &str, wall: &str| {
        clock_program(visits).replacen(
            "program capabilities FS APartial\n",
            &format!("program capabilities FS APartial\n{root_import}{wall}"),
            1,
        )
    };
    let mut accepted = ken_elaborator::ElabEnv::new().expect("prelude");
    accepted
        .elaborate_isolated_file_strict_v1(
            &[catalog.clone()],
            &composed("visits [Clock]", wall_import),
        )
        .expect("both checked Clock homes import");
    let mut absent = ken_elaborator::ElabEnv::new().expect("prelude");
    match absent
        .elaborate_isolated_file_strict_v1(&[catalog.clone()], &composed("visits [Clock]", ""))
    {
        Err(ken_elaborator::ElabError::UnboundName { name, .. }) => assert_eq!(name, "wall_now"),
        Err(error) => panic!("missing wall_now: wrong refusal {error:?}"),
        Ok(_) => panic!("wall_now available without import"),
    }
    let mut unrowed = ken_elaborator::ElabEnv::new().expect("prelude");
    match unrowed.elaborate_isolated_file_strict_v1(&[catalog], &composed("", wall_import)) {
        Err(ken_elaborator::ElabError::TypeMismatch { reason, .. }) => {
            assert!(
                reason.contains("EffectEscapes") && reason.contains("Clock"),
                "{reason}"
            )
        }
        Err(error) => panic!("wrong undeclared-clock refusal: {error:?}"),
        Ok(_) => panic!("Clock escaped the strict row check"),
    }
}

fn run_once() -> (ProgramOutcome, Vec<ClockTrace>, Vec<u8>) {
    let mut host = CaptureHost::new(Vec::new());
    host.set_clock_script([1_700_000_000_000_000_123, 1_700_000_000_000_000_456]);
    let outcome = run_program(
        &clock_program("visits [Clock]"),
        SourceFormat::Ken,
        &[],
        &[],
        b"",
        &mut host,
    )
    .expect("Clock program runs through the public injected driver");
    let trace = host.clock_trace().to_vec();
    let snapshot = format!("{outcome:?}|{trace:?}").into_bytes();
    (outcome, trace, snapshot)
}

#[test]
fn scripted_wall_clock_is_byte_identical_and_reaches_the_injected_host() {
    let first = run_once();
    let second = run_once();

    assert_eq!(first.0, ProgramOutcome { exit_status: 0 });
    assert_eq!(
        first.2, second.2,
        "independent snapshots must match bytewise"
    );
    assert_eq!(
        first.1,
        vec![
            ClockTrace::WallNow {
                nanoseconds: 1_700_000_000_000_000_123_i128.into(),
            },
            ClockTrace::WallNow {
                nanoseconds: 1_700_000_000_000_000_456_i128.into(),
            },
        ],
        "both reads must reach CaptureHost in program order"
    );
}

#[test]
fn fixed_clock_repeats_one_value_and_traces_each_read() {
    let mut host = CaptureHost::new(Vec::new());
    host.set_fixed_clock(-42);
    assert_eq!(host.clock_wall_now(), (-42_i128).into());
    assert_eq!(host.clock_wall_now(), (-42_i128).into());
    assert_eq!(
        host.clock_trace(),
        &[
            ClockTrace::WallNow {
                nanoseconds: (-42_i128).into(),
            },
            ClockTrace::WallNow {
                nanoseconds: (-42_i128).into(),
            },
        ]
    );
}

#[test]
fn undeclared_clock_row_is_a_named_effect_escape() {
    let mut env = ken_elaborator::ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&clock_program(""))
        .expect_err("calling wall_now without Clock in the row must reject");
    match error {
        ken_elaborator::ElabError::TypeMismatch { reason, .. } => {
            assert!(
                reason.contains("EffectEscapes"),
                "wrong rejection: {reason}"
            );
            assert!(reason.contains("Clock"), "missing Clock witness: {reason}");
        }
        other => panic!("expected named EffectEscapes rejection, got {other:?}"),
    }
}

#[test]
fn clock_package_is_structural_zero_trust_and_declares_no_ordering_law() {
    let extracted = ken_elaborator::literate::extract_ken_md(CLOCK_PACKAGE)
        .expect("Capability.Time.WallClock package must extract");
    for forbidden in [
        "Axiom",
        "postulate",
        "primitive",
        "opaque",
        "monotonic",
        "leq",
        "≤",
    ] {
        assert!(
            !extracted.source.contains(forbidden),
            "checked Clock package must not declare {forbidden}"
        );
    }
    assert!(
        CLOCK_PACKAGE.contains("declares no ordering or monotonicity law"),
        "the wall-clock absence of an ordering law must be stated explicitly"
    );

    let mut env = ken_elaborator::ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base();
    env.elaborate_ken_md_file(CLOCK_PACKAGE)
        .expect("ordinary Capability.Time.WallClock package elaborates");
    assert_eq!(before, env.env.trusted_base());
    for name in [
        "Instant",
        "Instant.MkInstant",
        "ClockOp",
        "WallNow",
        "clock_resp",
        "wall_now",
        "instant_nanoseconds",
        "replace_instant_nanoseconds",
    ] {
        let id = env.globals[name];
        assert!(
            !env.env.trusted_base().contains(&id),
            "{name} must be ordinary kernel-checked Ken"
        );
    }
}
