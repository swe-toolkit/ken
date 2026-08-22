//! `RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION` -- the traversability
//! enumeration for the CHECKED-PROGRAM family, the sibling population to
//! `rt_cold_lowering_path_enumeration`.
//!
//! Measures and covers. Fixes nothing; touches neither validator.
//!
//! # The closure argument, RE-DERIVED rather than carried over
//!
//! The `rt_parity` population was closed because its harness substitutes an
//! entry at exactly one placeholder, so one name determined the program. **That
//! basis does NOT transfer here and assuming it would have been wrong**: these
//! sources have NO substitution placeholder. Each is a fixed program whose
//! `main` dispatches to a named entry directly.
//!
//! So admissibility is defined differently, and the population is bounded by a
//! different fact: each source contains exactly ONE declaration bearing the
//! entry signature `proc <name> (cap : Cap AFull)` -- the shape `main`'s
//! `MkProgramCaps cap |-> <entry> cap` position requires. Every other
//! declaration in these sources takes a resource, a buffer or an outcome, so
//! none is admissible at that site. One admissible entry per source, no
//! placeholder to vary, therefore **one program per source**.
//!
//! # The family is TWO programs, not one
//!
//! Four tests carry this family. Three share a byte-identical source
//! (`rt_capture_projection_grow`, `rt_exactint_carried_observe`,
//! `rt_resource_release_carried_observe`); the fourth
//! (`rt_branched_scrutinee_unit_body_port`) has a DISTINCT source under a
//! different constant name, whose entry happens to share the NAME
//! `rt_branched_stage` while having a different body.
//!
//! Taking "the source the four pins share" at face value would have enumerated
//! one program and missed the second -- the same shape as the `_stage`
//! name-suffix near-miss in the sibling report, and the same lesson: identity by
//! CONTENT, never by the name something is filed under.
//!
//! # Why this population cannot be bounded the way the sibling one was
//!
//! `rt_parity` had 11 entries, so one pass surfaced five distinct mechanisms --
//! the refusals were spread across independent programs. Here each program is a
//! population of ONE, and a single compile returns a single refusal. **Whatever
//! sits behind the first refusal in each program is invisible until that refusal
//! is fixed.** Enumeration has no purchase on a population of one; for this
//! family, depth can only be discovered serially, which is precisely the
//! condition the enumeration discipline exists to avoid. That is a finding about
//! the FIXTURES, and it is reported rather than worked around.

#![cfg(target_os = "linux")]

const CHECKED_CAPTURE_SOURCE: &str = r#"program capabilities FS AFull
fn rt_branched_body (_buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

proc rt_branched_endpoint_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer
      (MkBufferWindow (8 : Int) (4 : Int)))
    (\outcome. match outcome {
      Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)
    })

proc rt_branched_after_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
    Ok bracket |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_branched_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_branched_endpoint_buffer file))
    (\outcome. rt_branched_after_buffer outcome)

proc rt_branched_done
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> host_exit AFull (Failure 71);
    Ok bracket |-> host_exit AFull (Failure 72)
  }

proc rt_branched_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_branched_file)
    (\outcome. rt_branched_done outcome)

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> rt_branched_stage cap
  }
"#;

const BRANCHED_SCRUTINEE_SOURCE: &str = r#"program capabilities FS AFull
fn rt_branched_body (_buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

proc rt_branched_endpoint_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer
      (MkBufferWindow (8 : Int) (4 : Int)))
    (\outcome. match outcome {
      Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)
    })

proc rt_branched_after_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
    Ok bracket |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_branched_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_branched_endpoint_buffer file))
    (\outcome. rt_branched_after_buffer outcome)

proc rt_branched_done
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> host_exit AFull (Failure 71);
    Ok bracket |-> host_exit AFull (Failure 72)
  }

proc rt_branched_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_branched_file)
    (\outcome. rt_branched_done outcome)

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> rt_branched_stage cap
  }
"#;

/// The checked-program family: one entry per source, two distinct sources.
/// See the closure argument above for why this is the population and not a
/// selection from it.
const FAMILY: &[(&str, &str)] = &[
    ("checked-capture (shared by three pins)", CHECKED_CAPTURE_SOURCE),
    ("branched-scrutinee (distinct source)", BRANCHED_SCRUTINEE_SOURCE),
];

/// The refusal each program is expected to reach, and what retires it.
///
/// Same discipline as the sibling report: assert the exact disposition rather
/// than "everything completes", so a landing successor, a newly surfaced layer,
/// and a by-design refusal are three distinguishable reds instead of one
/// permanent one.
const IH_MARKER: &str =
    "computational IH invocation marker does not wrap a complete application";

/// Retired by `RT-IH-MARKER-PRODUCER-COMPLETE`, whose landing gate is a
/// mandatory re-run of this runner.
const EXPECTED: &[(&str, &str)] = &[
    ("checked-capture (shared by three pins)", IH_MARKER),
    ("branched-scrutinee (distinct source)", IH_MARKER),
];

#[test]
fn the_expectation_table_covers_exactly_the_family() {
    let expected: std::collections::BTreeSet<&str> =
        EXPECTED.iter().map(|(name, _)| *name).collect();
    let family: std::collections::BTreeSet<&str> =
        FAMILY.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        expected, family,
        "the disposition table and the enumerated family have diverged"
    );
}

#[test]
fn every_checked_family_program_reaches_its_expected_terminal_state() {
    let mut outcomes: Vec<(&str, String)> = Vec::new();
    for (name, source) in FAMILY {
        let root = tempfile::tempdir().expect("temporary native-build root");
        std::fs::write(root.path().join("source"), b"ab").unwrap();
        let outcome = match ken_cli::build_native_program(
            source,
            ken_cli::SourceFormat::Ken,
            "rt_cold_checked_family",
            root.path(),
        ) {
            Ok(_) => "OK".to_string(),
            Err(error) => format!("{error:?}"),
        };
        outcomes.push((name, outcome));
    }

    // Printed unconditionally: a refusal set only visible on failure is not a
    // report.
    eprintln!("RT_COLD_CHECKED population={}", FAMILY.len());
    for (name, outcome) in &outcomes {
        eprintln!("RT_COLD_CHECKED {name} => {outcome}");
    }

    let mut mismatches: Vec<String> = Vec::new();
    for (name, outcome) in &outcomes {
        let key = EXPECTED
            .iter()
            .find(|(entry, _)| entry == name)
            .map(|(_, key)| *key)
            .expect("covered by the_expectation_table_covers_exactly_the_family");
        if outcome == "OK" {
            mismatches.push(format!(
                "{name}: now COMPLETES. If RT-IH-MARKER-PRODUCER-COMPLETE landed, \
                 this row is retired and the program joins the completing set."
            ));
        } else if !outcome.contains(key) {
            mismatches.push(format!(
                "{name}: refuses for a DIFFERENT reason than the expected {key:?}. \
                 Note this population cannot bound its own depth -- one program per \
                 source means one refusal per source -- so a changed refusal is the \
                 only signal available that something moved. Got: {outcome}"
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "{} of {} checked-family programs reached an unexpected terminal state:\n{}",
        mismatches.len(),
        FAMILY.len(),
        mismatches.join("\n")
    );
}
