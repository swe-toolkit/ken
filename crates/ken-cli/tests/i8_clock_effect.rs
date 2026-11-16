//! I-8 wall-clock effect acceptance through the public injected runner.

use ken_cli::{run_program, ProgramOutcome, SourceFormat};
use ken_interp::{CaptureHost, ClockTrace, HostHandler};

#[path = "support/catalog_roots.rs"]
mod catalog_roots;

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

// Both host replies must reach the Ken continuation unchanged. The distinct
// failure statuses show which reply was corrupted, rather than trusting the
// host-side trace recorded before response reification.
const CLOCK_OBSERVE_BODY: &str = r#"
  host_bind APartial Instant ExitCode
    (host_clock APartial Instant wall_now)
    (\first .
      match first {
        Instant.MkInstant firstNanoseconds |->
          host_bind APartial Instant ExitCode
            (host_clock APartial Instant wall_now)
            (\second .
              match second {
                Instant.MkInstant secondNanoseconds |->
                  match eq_int firstNanoseconds 1700000000000000456 {
                    True |-> match eq_int secondNanoseconds 1700000000000000123 {
                      True |-> host_pure APartial ExitCode ExitCode.Success;
                      False |-> host_pure APartial ExitCode (ExitCode.Failure 13)
                    };
                    False |-> host_pure APartial ExitCode (ExitCode.Failure 12)
                  }
              })
      })
"#;

const HOST_IMPORT: &str = "import Capability.Process.Exit (host_bind, host_pure, host_clock)\n";
const WALL_IMPORT: &str = "import Capability.Time.WallClock (wall_now)\n";

fn clock_program_with_body(
    visits: &str,
    host_import: &str,
    wall_import: &str,
    body: &str,
) -> String {
    format!(
        "program capabilities FS APartial\n{host_import}{wall_import}\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode {visits} =\n{body}"
    )
}

fn clock_program(visits: &str, host_import: &str, wall_import: &str) -> String {
    clock_program_with_body(visits, host_import, wall_import, CLOCK_BODY)
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
    let composed = |visits: &str, wall: &str| clock_program(visits, HOST_IMPORT, wall);
    let imported = composed("visits [Clock]", WALL_IMPORT);
    let mut accepted = ken_elaborator::ElabEnv::new().expect("prelude");
    accepted
        .elaborate_isolated_file_strict_v1(&[catalog.clone()], &imported)
        .expect("both checked Clock homes import");
    let without_wall = composed("visits [Clock]", "");
    assert_ne!(
        without_wall, imported,
        "negative must differ from imported source"
    );
    assert!(
        !without_wall
            .lines()
            .any(|line| line.starts_with("import Capability.Time.WallClock")),
        "negative source must not import wall_now: {without_wall}"
    );
    let mut absent = ken_elaborator::ElabEnv::new().expect("prelude");
    match absent.elaborate_isolated_file_strict_v1(&[catalog.clone()], &without_wall) {
        Err(ken_elaborator::ElabError::UnboundName { name, .. }) => assert_eq!(name, "wall_now"),
        Err(error) => panic!("missing wall_now: wrong refusal {error:?}"),
        Ok(_) => panic!("wall_now available without import"),
    }
    let mut unrowed = ken_elaborator::ElabEnv::new().expect("prelude");
    match unrowed.elaborate_isolated_file_strict_v1(&[catalog], &composed("", WALL_IMPORT)) {
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

fn run_with_clock_script(
    samples: [i128; 2],
    body: &str,
) -> (ProgramOutcome, Vec<ClockTrace>, Vec<u8>) {
    let mut host = CaptureHost::new(Vec::new());
    host.set_clock_script(samples);
    let outcome = run_program(
        &clock_program_with_body("visits [Clock]", HOST_IMPORT, WALL_IMPORT, body),
        SourceFormat::Ken,
        &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")],
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

fn run_once() -> (ProgramOutcome, Vec<ClockTrace>, Vec<u8>) {
    run_with_clock_script(
        [1_700_000_000_000_000_123, 1_700_000_000_000_000_456],
        CLOCK_BODY,
    )
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
    env.set_catalog_roots_v1(&[
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
    ])
    .expect("checked catalog root");
    let error = env
        .elaborate_file(&clock_program("", HOST_IMPORT, WALL_IMPORT))
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

/// Promise class: durable invariant. MEASURED: loading the checked owner
/// adds no trust, and a real imported wall-clock program observes a decreasing
/// pair of host values in order, and the Ken continuation independently
/// matches both delivered payloads. CLAIMED: this package cannot justify a
/// monotonicity guarantee by adding trust or altering program-visible reads.
/// THE GAP: the host trace is pre-reification; distinct Ken exit statuses for
/// each wrong payload guard the response-delivery seam, while the separate
/// Strict import-and-row test guards access to the checked home.
#[test]
fn clock_package_is_structural_zero_trust_and_declares_no_ordering_law() {
    let mut env = ken_elaborator::ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base();
    catalog_roots::load_owner(&mut env, "Capability.Time.WallClock");
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
        let id = catalog_roots::selected_owner_id(&env, "Capability.Time.WallClock", name);
        assert!(
            !env.env.trusted_base().contains(&id),
            "{name} must be ordinary kernel-checked Ken"
        );
    }

    let later = 1_700_000_000_000_000_456_i128;
    let earlier = 1_700_000_000_000_000_123_i128;
    assert!(later > earlier, "the host script must move backwards");
    let (outcome, trace, _) = run_with_clock_script([later, earlier], CLOCK_OBSERVE_BODY);
    assert_eq!(outcome, ProgramOutcome { exit_status: 0 });
    assert_eq!(
        run_with_clock_script([later + 1, earlier], CLOCK_OBSERVE_BODY).0,
        ProgramOutcome { exit_status: 12 },
        "the first Ken-visible timestamp must be checked"
    );
    assert_eq!(
        run_with_clock_script([later, earlier + 1], CLOCK_OBSERVE_BODY).0,
        ProgramOutcome { exit_status: 13 },
        "the second Ken-visible timestamp must be checked"
    );
    assert_eq!(
        trace,
        vec![
            ClockTrace::WallNow {
                nanoseconds: later.into(),
            },
            ClockTrace::WallNow {
                nanoseconds: earlier.into(),
            },
        ],
        "a checked wall-clock client must see both reversed host samples"
    );
}
