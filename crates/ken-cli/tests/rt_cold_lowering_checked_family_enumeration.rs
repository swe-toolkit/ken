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
//! basis does NOT transfer here and assuming it would have been wrong**: this
//! source has NO substitution placeholder. It is a fixed program whose `main`
//! dispatches to a named entry directly.
//!
//! So admissibility is defined differently, and the population is bounded by a
//! different fact: the source contains exactly ONE declaration bearing the
//! entry signature `proc <name> (cap : Cap AFull)` -- the shape `main`'s
//! `MkProgramCaps cap |-> <entry> cap` position requires. Every other
//! declaration takes a resource, a buffer or an outcome, so none is admissible
//! at that site. One admissible entry and no placeholder to vary means one
//! program for this source.
//!
//! # The family is one content identity
//!
//! Four tests carry byte-identical source content under different constant
//! names: `rt_capture_projection_grow`, `rt_exactint_carried_observe`,
//! `rt_resource_release_carried_observe`, and
//! `rt_branched_scrutinee_unit_body_port`. Names and reporting labels do not
//! create programs. The family therefore contains one program, identified by
//! its source bytes. The permanent population pin below rejects two rows with
//! the same bytes, so labels cannot inflate the census.
//!
//! # Why this population cannot be bounded the way the sibling one was
//!
//! `rt_parity` had 11 entries, so one pass surfaced five distinct mechanisms --
//! the refusals were spread across independent programs. Here the family is a
//! population of ONE, and a single compile returns a single refusal. **Whatever
//! sits behind the first refusal is invisible until that refusal is fixed.**
//! Enumeration has no purchase on a population of one; for this family, depth
//! can only be discovered serially, which is precisely the condition the
//! enumeration discipline exists to avoid. That is a finding about the
//! FIXTURES, and it is reported rather than worked around.

#![cfg(target_os = "linux")]

const CHECKED_FAMILY_SOURCE: &str = r#"program capabilities FS AFull
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

/// The checked-program family: exactly one row per distinct source content.
/// See the closure argument above for why this is the population and not a
/// selection from it.
const FAMILY: &[(&str, &str)] = &[(
    "checked capture with branched scrutinee (shared by four pins)",
    CHECKED_FAMILY_SOURCE,
)];

/// The terminal disposition each content-distinct program must reach.
///
/// `RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION` retires the advancing-refusal
/// sentinel: the closed program now completes through the static checked-IH
/// dispatcher and its transported positional environment.
const EXPECTED: &[(&str, &str)] = &[(
    "checked capture with branched scrutinee (shared by four pins)",
    "OK",
)];

/// DURABLE INVARIANT: adding a genuinely byte-distinct source remains green;
/// adding another label for existing bytes or an unpaired expectation reds.
///
/// MEASURED: `FAMILY` row count equals its byte-slice set cardinality, labels
/// are unique, and `EXPECTED` has exactly the same unique labels.
/// CLAIMED: reporting labels cannot inflate the declared program population,
/// and every source-content identity has exactly one terminal disposition.
/// THE GAP: this pin guards the declared family, not discovery of fixture files;
/// the closure argument above independently grounds the current four fixture
/// copies as one byte-identical source identity.
#[test]
fn family_rows_are_unique_by_source_content_and_have_exact_expectations() {
    let distinct_sources: std::collections::BTreeSet<&[u8]> =
        FAMILY.iter().map(|(_, source)| source.as_bytes()).collect();
    assert_eq!(
        distinct_sources.len(),
        FAMILY.len(),
        "each checked-family row must name a distinct source-content identity; \
         labels cannot inflate the program population"
    );

    let expected: std::collections::BTreeSet<&str> =
        EXPECTED.iter().map(|(name, _)| *name).collect();
    let family: std::collections::BTreeSet<&str> = FAMILY.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        family.len(),
        FAMILY.len(),
        "each content-distinct program must also have one distinct reporting label"
    );
    assert_eq!(
        expected.len(),
        EXPECTED.len(),
        "each terminal disposition must have one distinct reporting label"
    );
    assert_eq!(
        expected, family,
        "the disposition table and the content-distinct family have diverged"
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
            .expect(
                "covered by family_rows_are_unique_by_source_content_and_have_exact_expectations",
            );
        if key == "OK" && outcome != "OK" {
            mismatches.push(format!(
                "{name}: expected the closed checked-IH representation to complete, got: \
                 {outcome}"
            ));
        } else if key != "OK" && !outcome.contains(key) {
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
