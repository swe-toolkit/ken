//! PX8-F linked checked `writeAll` reachability through PX8-N bounded Nat.

const WRITE_ALL: &str = r#"program capabilities FS AFull
fn body_from_write (outcome : Result ResourceError Unit)
  : ResourceBodyResult Unit Unit =
  match outcome {
    Err error |-> ResourceBodyErr Unit Unit MkUnit;
    Ok value |-> ResourceBodyOk Unit Unit MkUnit
  }

fn after_write (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (body_from_write outcome)

fn read_error_body (error : ResourceError)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)

fn read_eof_body (_unit : Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)

proc after_read
  (output : Resource FsHandle) (buffer : BufferHandle)
  (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> read_error_body error;
    Ok progress |-> match progress {
      ReadEof |-> read_eof_body MkUnit;
      ReadSome span count |->
        bind (Coproduct (FSOp AFull) AmbientOp)
          (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
          (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
          (writeAll AFull output (0 : Int) buffer span)
          (\written. after_write written)
    }
  }

proc buffer_body
  (input : Resource FsHandle) (output : Resource FsHandle)
  (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull input (0 : Int) buffer (MkBufferWindow (0 : Int) (6 : Int)))
    (\outcome. after_read output buffer outcome)

fn buffer_bracket_body
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : ResourceBodyResult Unit Unit =
  match outcome {
    Err error |-> ResourceBodyErr Unit Unit MkUnit;
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> ResourceBodyOk Unit Unit MkUnit;
      ResourceBracketBodyError error |-> ResourceBodyErr Unit Unit MkUnit;
      ResourceBracketReleaseError error |-> ResourceBodyErr Unit Unit MkUnit;
      ResourceBracketBodyAndReleaseError body_error release_error |->
        ResourceBodyErr Unit Unit MkUnit
    }
  }

fn after_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (buffer_bracket_body outcome)

proc output_body
  (input : Resource FsHandle) (output : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (6 : Int) (buffer_body input output))
    (\outcome. after_buffer outcome)

fn file_bracket_body
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : ResourceBodyResult Unit Unit =
  match outcome {
    Err error |-> ResourceBodyErr Unit Unit MkUnit;
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> ResourceBodyOk Unit Unit MkUnit;
      ResourceBracketBodyError error |-> ResourceBodyErr Unit Unit MkUnit;
      ResourceBracketReleaseError error |-> ResourceBodyErr Unit Unit MkUnit;
      ResourceBracketBodyAndReleaseError body_error release_error |->
        ResourceBodyErr Unit Unit MkUnit
    }
  }

fn after_output
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (file_bracket_body outcome)

proc input_body (cap : Cap AFull) (input : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withResource AFull Unit Unit cap (bytes_encode "output.bin")
      (ResourceWriteCreate CreateOrTruncate) (output_body input))
    (\outcome. after_output outcome)

fn finish (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 81);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> host_exit AFull Success;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 82);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 83);
      ResourceBracketBodyAndReleaseError body_error release_error |->
        host_exit AFull (Failure 84)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
        (withResource AFull Unit Unit cap (bytes_encode "input.bin")
          ResourceRead (input_body cap))
        (\outcome. finish outcome)
  }
"#;

#[cfg(target_os = "linux")]
fn build_short_pwrite_preload(dir: &std::path::Path) -> std::path::PathBuf {
    let source = dir.join("short_pwrite.c");
    let library = dir.join("libshort_pwrite.so");
    std::fs::write(
        &source,
        r#"#define _GNU_SOURCE
#include <dlfcn.h>
#include <stddef.h>
#include <sys/types.h>
#include <unistd.h>

ssize_t pwrite(int fd, const void *buf, size_t count, off_t offset) {
  static ssize_t (*next_pwrite)(int, const void *, size_t, off_t) = 0;
  if (!next_pwrite) {
    next_pwrite = dlsym(RTLD_NEXT, "pwrite");
  }
  size_t capped = count > 2 ? 2 : count;
  return next_pwrite(fd, buf, capped, offset);
}

ssize_t pwrite64(int fd, const void *buf, size_t count, off64_t offset) {
  static ssize_t (*next_pwrite64)(int, const void *, size_t, off64_t) = 0;
  if (!next_pwrite64) {
    next_pwrite64 = dlsym(RTLD_NEXT, "pwrite64");
  }
  size_t capped = count > 2 ? 2 : count;
  return next_pwrite64(fd, buf, capped, offset);
}
"#,
    )
    .unwrap();
    let status = std::process::Command::new("cc")
        .args(["-shared", "-fPIC", "-o"])
        .arg(&library)
        .arg(&source)
        .arg("-ldl")
        .status()
        .expect("compile short-pwrite preload");
    assert!(status.success(), "short-pwrite preload compilation failed");
    library
}

#[cfg(target_os = "linux")]
const RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD: &str =
    "KEN_RT_RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD";

#[cfg(target_os = "linux")]
const RETAINED_RESULT_CLOSURE_PROOF_MUTATION_CHILD: &str =
    "KEN_RT_RETAINED_RESULT_CLOSURE_PROOF_MUTATION_CHILD";

#[cfg(target_os = "linux")]
fn assert_retained_unit_call_target_mutation_child() {
    use ken_runtime::RetainedUnitCallTargetMutation as Mutation;

    let mode = std::env::var(RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD)
        .expect("retained-unit call-target mutation child mode");
    let (mutation, expected) = match mode.as_str() {
        "unrelated-owner-root" => (
            Mutation::SubstituteUnrelatedOwnerRoot,
            "retained body StaticOriginId(1236) has no graph-derived call target in this unit",
        ),
        "suppress-graph-claims" => (
            Mutation::SuppressGraphClaims,
            "retained body StaticOriginId(1236) has no graph-derived call target in this unit",
        ),
        "wrong-target" => (
            Mutation::SubstituteWrongTarget,
            "a retained-body graph claim for",
        ),
        "ambiguous-target" => (
            Mutation::DuplicateTargetClaim,
            "has more than one graph-derived call target",
        ),
        other => panic!("unknown retained-unit call-target mutation: {other}"),
    };
    let dir = tempfile::Builder::new()
        .prefix("ken-px8f-retained-target-control-")
        .tempdir()
        .unwrap();
    let result = ken_runtime::with_retained_unit_call_target_mutation(mutation, || {
        ken_cli::build_native_program(
            WRITE_ALL,
            ken_cli::SourceFormat::Ken,
            "px8f_write_all_retained_target_control",
            dir.path(),
        )
    });
    let error = match result {
        Ok(_) => panic!("{mode}: malformed retained-unit target derivation compiled"),
        Err(error) => error,
    };
    let rendered = format!("{error:?}");
    assert!(
        rendered.contains(expected),
        "{mode}: mutation missed intended refusal; error:\n{rendered}"
    );
    eprintln!("{mode}: {rendered}");
    assert!(
        ken_runtime::retained_unit_call_target_mutation_is_exact(),
        "{mode}: scoped retained-unit target mutation did not restore"
    );
}

#[cfg(target_os = "linux")]
fn assert_retained_result_closure_proof_mutation_child() {
    use ken_runtime::RetainedResultClosureProofMutation as Mutation;

    let mode = std::env::var(RETAINED_RESULT_CLOSURE_PROOF_MUTATION_CHILD)
        .expect("retained result-closure proof mutation child mode");
    let (mutation, expected) = match mode.as_str() {
        "exact" => (Mutation::Exact, None),
        "missing" => (
            Mutation::DropTypedOccurrence,
            Some("proof population omits an exact typed occurrence"),
        ),
        "duplicate" => (
            Mutation::DuplicateTypedOccurrence,
            Some("proof population duplicates an exact typed occurrence"),
        ),
        "wrong-owner" => (
            Mutation::SubstituteEveryOtherOwner,
            Some("proof changes the exact emission owner"),
        ),
        "wrong-body" => (
            Mutation::SubstituteEveryOtherBody,
            Some("proof changes the exact closure body"),
        ),
        "wrong-field" => (
            Mutation::SubstituteEveryOtherField,
            Some("proof changes the exact result constructor field"),
        ),
        "wrong-generated-target" => (
            Mutation::SubstituteEveryOtherTarget,
            Some("proof changes the exact generated continuation target"),
        ),
        "permuted-captures" => (
            Mutation::PermuteCaptureOrder,
            Some("proof changes the exact positional capture run"),
        ),
        "widened-population" => (
            Mutation::WidenToEveryOtherCapturedClosure,
            Some("proof population widens beyond an exact typed occurrence"),
        ),
        "missing-static-body-call-edge" => (
            Mutation::DropExactStaticBodyCallEdge,
            Some("proof has no unique exact static-body call edge"),
        ),
        "suppressed-result-authorization" => (
            Mutation::SuppressResultAuthorizationArm,
            Some("a closure cannot cross the boundary"),
        ),
        other => panic!("unknown retained result-closure proof mutation: {other}"),
    };
    let dir = tempfile::Builder::new()
        .prefix("ken-px8f-retained-result-closure-control-")
        .tempdir()
        .unwrap();
    let result = ken_runtime::with_retained_result_closure_proof_mutation(mutation, || {
        ken_cli::build_native_program(
            WRITE_ALL,
            ken_cli::SourceFormat::Ken,
            "px8f_write_all_retained_result_closure_control",
            dir.path(),
        )
    });
    match expected {
        None => {
            result.expect("the exact typed retained result-closure proof must compile");
            assert_eq!(
                ken_runtime::retained_result_closure_proof_mutation_applied(),
                0,
                "the exact positive must not perturb the proof population"
            );
        }
        Some(expected) => {
            let error = result.expect_err("a malformed retained result-closure proof compiled");
            let rendered = format!("{error:?}");
            assert!(
                rendered.contains(expected),
                "{mode}: mutation missed intended refusal; error:\n{rendered}"
            );
            assert!(
                ken_runtime::retained_result_closure_proof_mutation_applied() > 0,
                "{mode}: no exact retained-result relation or consumer was changed"
            );
            eprintln!("{mode}: {rendered}");
        }
    }
    assert!(
        ken_runtime::retained_result_closure_proof_mutation_is_exact(),
        "{mode}: scoped retained result-closure proof mutation did not restore"
    );
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. The native run and interpreter must agree
/// on the ordered short-write observations required by runtime evaluation
/// (`spec/40-runtime/42-evaluation.md` section 6.2 and
/// `spec/40-runtime/45-native-backend.md` section 4).
#[test]
#[ignore = "RT-RESULT-CONTINUATION-BINDING-PROVENANCE: retained result-closure representation succeeds; the existing D3 frontier next returns a CheckedIhCapturedEnvironment where fresh R2 belongs"]
fn linked_checked_write_all_observes_short_progress_and_matches_interpreter() {
    std::thread::Builder::new()
        .name("px8f-write-all".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(run_linked_checked_write_all)
        .expect("spawn large-stack PX8-F fixture")
        .join()
        .expect("PX8-F fixture thread");
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. A specialization-owned retained call is
/// admitted only from its checked raw owner's exact graph subtree; an unrelated
/// root, missing claim, wrong target, or ambiguous target reaches a production
/// refusal.
///
/// MEASURED: each child moves the traversal root or mutates resolved graph
/// claims before function-local declaration and asserts the exact downstream
/// refusal family.
/// CLAIMED: an unrelated owner, lookup failure, target disagreement, and
/// candidate ambiguity cannot synthesize or select a retained-body target.
/// THE GAP: the child reuses the real checked `writeAll` compile, so a green row
/// depends on the production context-definition and call-emission path.
#[test]
fn retained_unit_call_target_controls_reject_malformed_derivations() {
    let cases = [
        (
            "unrelated-owner-root",
            "retained body StaticOriginId(1236) has no graph-derived call target in this unit",
        ),
        (
            "suppress-graph-claims",
            "retained body StaticOriginId(1236) has no graph-derived call target in this unit",
        ),
        ("wrong-target", "a retained-body graph claim for"),
        (
            "ambiguous-target",
            "has more than one graph-derived call target",
        ),
    ];
    for (mode, expected) in cases {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "linked_checked_write_all_observes_short_progress_and_matches_interpreter",
                "--ignored",
                "--nocapture",
            ])
            .env(RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD, mode)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn isolated retained-unit call-target mutation child");
        assert!(
            output.status.success(),
            "{mode}: mutation child failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected),
            "{mode}: child did not publish intended refusal; stderr:\n{stderr}"
        );
        if mode == "unrelated-owner-root" {
            assert!(
                stderr.contains("retained-unit root control replaced checked root"),
                "{mode}: child did not traverse from a real unrelated owner; stderr:\n{stderr}"
            );
        }
    }
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. The admitted proof population is exactly
/// the planner's typed continuation-result relation for this occurrence, and
/// its capture run is positional (`spec/40-runtime/45-native-backend.md`
/// sections 3.2 and 7).
///
/// MEASURED: the same real px8f compile accepts the exact population, while
/// dropping, duplicating, substituting real neighboring owner/body/field/target
/// rows, permuting the real capture run, widening to every other captured
/// lexical occurrence, dropping the exact downstream call edge, or suppressing
/// the caller's exact result arm reaches a distinct production refusal.
/// CLAIMED: only the exact result/constructor/field/closure/body/capture/target
/// tuple and its joined static call edge may acquire the existing M4 environment
/// representation, and the result arm is causally necessary: its result-derived
/// record cannot borrow either weaker M4 authorization.
/// THE GAP: numeric coordinates below establish that each population mutation
/// reached D0's exact fixture row; they select no authority and are never inputs
/// to production derivation.
#[test]
fn retained_result_closure_proof_controls_are_exact_and_positional() {
    let cases = [
        ("exact", None),
        (
            "missing",
            Some("proof population omits an exact typed occurrence"),
        ),
        (
            "duplicate",
            Some("proof population duplicates an exact typed occurrence"),
        ),
        (
            "wrong-owner",
            Some("proof changes the exact emission owner"),
        ),
        ("wrong-body", Some("proof changes the exact closure body")),
        (
            "wrong-field",
            Some("proof changes the exact result constructor field"),
        ),
        (
            "wrong-generated-target",
            Some("proof changes the exact generated continuation target"),
        ),
        (
            "permuted-captures",
            Some("proof changes the exact positional capture run"),
        ),
        (
            "widened-population",
            Some("proof population widens beyond an exact typed occurrence"),
        ),
        (
            "missing-static-body-call-edge",
            Some("proof has no unique exact static-body call edge"),
        ),
        (
            "suppressed-result-authorization",
            Some("a closure cannot cross the boundary"),
        ),
    ];
    for (mode, expected) in cases {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "linked_checked_write_all_observes_short_progress_and_matches_interpreter",
                "--ignored",
                "--nocapture",
            ])
            .env(RETAINED_RESULT_CLOSURE_PROOF_MUTATION_CHILD, mode)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn isolated retained result-closure proof mutation child");
        assert!(
            output.status.success(),
            "{mode}: mutation child failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        if let Some(expected) = expected {
            assert!(
                stderr.contains(expected),
                "{mode}: child did not publish intended refusal; stderr:\n{stderr}"
            );
            for coordinate in [
                "construct=StaticOriginId(815)",
                "field=1",
                "seat=StaticOriginId(810)",
                "body=StaticOriginId(800)",
                "StaticOriginId(809)",
                "StaticOriginId(801)",
                "target=ContinuationSpecializationId(3)",
            ] {
                assert!(
                    stderr.contains(coordinate),
                    "{mode}: mutation did not report D0's exact typed row coordinate {coordinate}; stderr:\n{stderr}"
                );
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn run_linked_checked_write_all() {
    use std::os::unix::ffi::OsStrExt as _;

    if std::env::var_os(RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD).is_some() {
        assert_retained_unit_call_target_mutation_child();
        return;
    }
    if std::env::var_os(RETAINED_RESULT_CLOSURE_PROOF_MUTATION_CHILD).is_some() {
        assert_retained_result_closure_proof_mutation_child();
        return;
    }

    let dir = tempfile::Builder::new()
        .prefix("ken-px8f-write-all-")
        .tempdir()
        .unwrap();
    std::fs::write(dir.path().join("input.bin"), b"abcdef").unwrap();
    let preload = build_short_pwrite_preload(dir.path());

    eprintln!("PX8-F: compiling checked writeAll fixture");
    let output = ken_cli::build_native_program(
        WRITE_ALL,
        ken_cli::SourceFormat::Ken,
        "px8f_write_all_native",
        dir.path(),
    )
    .expect("checked writeAll reaches linked native lowering");
    eprintln!("PX8-F: running linked fixture");
    let observation = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: vec![("LD_PRELOAD".into(), preload.into_os_string())],
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked checked writeAll runs");
    eprintln!("PX8-F: running interpreter fixture");

    if observation.exit_status != 0 {
        let frontier = format!(
            "{:?}",
            observation
                .terminal_error
                .as_ref()
                .expect("a nonzero linked run must report its terminal error")
        );
        assert!(
            frontier.contains("PatternMatchFailure") && frontier.contains("ResourceBodyResult"),
            "the represented call stopped at an unnamed frontier: {frontier}"
        );
        eprintln!(
            "PX8-F: retained result-closure representation advanced to the named D3 frontier: \
             {frontier}"
        );
    }
    assert_eq!(observation.exit_status, 0);
    assert_eq!(observation.terminal_error, None);
    assert_eq!(
        std::fs::read(dir.path().join("output.bin")).unwrap(),
        b"abcdef"
    );
    let writes: Vec<_> = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::FsWriteAt)
        .collect();
    assert_eq!(writes.len(), 3, "short progress must recurse twice");
    for (event, expected) in writes.iter().zip([(0, 0, 6), (2, 2, 4), (4, 4, 2)]) {
        assert!(matches!(
            (&event.request, &event.outcome),
            (
                ken_runtime::CanonicalRequestV1::FsWriteAt {
                    file_offset,
                    buffer_start,
                    length,
                },
                ken_runtime::CanonicalOutcomeV1::Success(
                    ken_runtime::CanonicalReplyV1::WriteProgress(_)
                )
            ) if (*file_offset, *buffer_start, *length) == expected
        ));
    }

    let mut unsupported_virtual = ken_interp::CaptureHost::new(Vec::new());
    unsupported_virtual.insert_file(b"input.bin".to_vec(), b"abcdef".to_vec());
    let virtual_observation = ken_cli::run_program_effect_observation(
        WRITE_ALL,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        b".",
        &mut unsupported_virtual,
    )
    .expect("the virtual-root control reaches the resource-open boundary");
    assert_eq!(virtual_observation.exit_status, 81);
    let denied_open = virtual_observation
        .effect_trace
        .first()
        .expect("virtual-root control records its denied resource open");
    assert_eq!(denied_open.operation, ken_runtime::HostOpV1::FsOpen);
    let ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::File(error)) =
        &denied_open.outcome
    else {
        panic!("virtual-root resource open did not return a file error");
    };
    assert_eq!(format!("{:?}", error.cause), "Capability(ScopeEscape)");

    let mut interpreter = ken_interp::PosixHost::new_at(dir.path());
    let interpreted = ken_cli::run_program_effect_observation(
        WRITE_ALL,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        dir.path().as_os_str().as_bytes(),
        &mut interpreter,
    )
    .expect("the same checked writeAll runs in the interpreter");
    eprintln!("PX8-F: comparing observations");
    assert_eq!(interpreted.exit_status, observation.exit_status);
    assert_eq!(interpreted.terminal_error, observation.terminal_error);
    assert_eq!(
        std::fs::read(dir.path().join("output.bin")).unwrap(),
        b"abcdef"
    );
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. Recut AC-1 (congruence) + AC-4 (mixed
/// both-colors) + the AC-3 compile half. The checked `writeAll` fixture carries
/// a Specialized ordinary response, a Deferred P1 response with no continuation
/// unit, and Deferred P2 transport sources. P1 keeps the response plane open, so
/// execute-then-resume conservatively retains every transport source as P2 rather
/// than partially specializing a plane with no owner target for every response.
#[test]
fn write_all_classifies_mixed_specialized_and_deferred_responses() {
    std::thread::Builder::new()
        .name("px8f-classify-mixed".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let dir = tempfile::Builder::new()
                .prefix("ken-px8f-classify-mixed-")
                .tempdir()
                .unwrap();
            let (result, diagnostics) =
                ken_runtime::with_static_response_feasibility_diagnostics(|| {
                    ken_cli::build_native_program(
                        WRITE_ALL,
                        ken_cli::SourceFormat::Ken,
                        "px8f_write_all_classify_mixed",
                        dir.path(),
                    )
                });
            // AC-3 (compile half): the deferred-frontier writeAll program compiles;
            // the Deferred residual is routed, not aborted.
            result.expect("the checked writeAll fixture compiles under the recut");
            assert_eq!(diagnostics.len(), 1, "one compile publishes one plan");
            let diagnostic = diagnostics.into_iter().next().unwrap();
            assert_eq!(diagnostic.static_response_infeasible, None);
            assert_eq!(diagnostic.all_static_response_infeasible, None);

            let specialized_vis: std::collections::BTreeSet<u32> = diagnostic
                .all_static_response_rows
                .iter()
                .map(|row| row.vis_origin)
                .collect();
            let deferred_vis: std::collections::BTreeSet<u32> = diagnostic
                .static_response_deferred
                .iter()
                .map(|row| row.vis_origin)
                .collect();

            // AC-1 congruence is enforced STRUCTURALLY, not by a vis-level
            // disjointness assertion here: classify (static_response_context_demands
            // _filtered) puts every response Vis-with-host-route into a demand
            // (Specialized) or a deferred P1/P2 row
            // or a whole-plan SsaInfeasible
            // -- no fourth "unclassified" path -- and the §7 total match over
            // Option<ResponseDisposition> at each production seat compiles only if
            // every variant is handled (AC-2). The None case is verified to mean
            // exclusively "not a static-response Vis" (Architect ruling
            // evt_37dx1wqamabg). Vis-level disjointness is deliberately not
            // asserted: a single multi-K producer may carry a Specialized K and
            // a P1 unit-less response at one Vis origin. The partition is per
            // (Vis,K), not per Vis.

            // AC-4 mixed: both colours coexist in this one unit -- a genuine
            // polyvariant witness. (Discriminating: a single-colour program cannot
            // tell real threading from a flag.)
            assert!(
                !specialized_vis.is_empty(),
                "the mixed fixture must carry at least one Specialized response; \
                 rows={:?}",
                diagnostic.all_static_response_rows
            );
            assert!(
                !deferred_vis.is_empty(),
                "the mixed fixture must carry at least one Deferred response; \
                 deferred={:?}",
                diagnostic.static_response_deferred
            );
            let deferred_kinds = diagnostic
                .static_response_deferred
                .iter()
                .map(|row| row.sub_case.as_str())
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                deferred_kinds,
                std::collections::BTreeSet::from([
                    "NoContinuationUnit",
                    "UnconsumedTransportCaller",
                ]),
                "the open-chain control must contain both P1 and its retained P2 siblings"
            );
        })
        .expect("spawn large-stack classify-mixed probe")
        .join()
        .expect("classify-mixed probe thread");
}
