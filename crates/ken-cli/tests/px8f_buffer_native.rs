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
const HANDLER_OWNED_DEFERRED_RESPONSE_MUTATION_CHILD: &str =
    "KEN_RT_HANDLER_OWNED_DEFERRED_RESPONSE_MUTATION_CHILD";
#[cfg(target_os = "linux")]
const HS17_STATIC_RESPONSE_RETURN_MUTATION_CHILD: &str =
    "KEN_RT_HS17_STATIC_RESPONSE_RETURN_MUTATION_CHILD";

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RetainedResultClosureOwner {
    Predeclared(u64),
    Specialization(u64),
}

#[cfg(target_os = "linux")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RetainedResultClosureRow {
    owner: RetainedResultClosureOwner,
    result: u64,
    construct: u64,
    field: u64,
    seat: u64,
    body: u64,
    captures: Vec<u64>,
    target: u64,
}

#[cfg(target_os = "linux")]
fn parse_decimal_after(text: &str, prefix: &str) -> u64 {
    let tail = text
        .split_once(prefix)
        .unwrap_or_else(|| panic!("retained-result row has no {prefix}: {text}"))
        .1;
    let digits = tail
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>();
    assert!(
        !digits.is_empty(),
        "retained-result row has no decimal after {prefix}: {text}"
    );
    digits
        .parse()
        .unwrap_or_else(|_| panic!("retained-result row has invalid {prefix}: {text}"))
}

#[cfg(target_os = "linux")]
fn parse_retained_result_closure_row(line: &str) -> Option<RetainedResultClosureRow> {
    if !line.starts_with("retained-result-closure-control mutation=") {
        return None;
    }
    let owner = if line.contains(" owner=Predeclared(PredeclaredFunctionId(") {
        RetainedResultClosureOwner::Predeclared(parse_decimal_after(
            line,
            " owner=Predeclared(PredeclaredFunctionId(",
        ))
    } else if line.contains(" owner=Specialization(ContinuationSpecializationId(") {
        RetainedResultClosureOwner::Specialization(parse_decimal_after(
            line,
            " owner=Specialization(ContinuationSpecializationId(",
        ))
    } else {
        panic!("retained-result row has an unknown owner: {line}");
    };
    let capture_text = line
        .split_once(" captures=[")
        .unwrap_or_else(|| panic!("retained-result row has no captures: {line}"))
        .1
        .split_once("] target=")
        .unwrap_or_else(|| panic!("retained-result row has no target after captures: {line}"))
        .0;
    let captures = if capture_text.is_empty() {
        Vec::new()
    } else {
        capture_text
            .split(", ")
            .map(|capture| parse_decimal_after(capture, "StaticOriginId("))
            .collect()
    };
    Some(RetainedResultClosureRow {
        owner,
        result: parse_decimal_after(line, " result=StaticOriginId("),
        construct: parse_decimal_after(line, " construct=StaticOriginId("),
        field: parse_decimal_after(line, " field="),
        seat: parse_decimal_after(line, " seat=StaticOriginId("),
        body: parse_decimal_after(line, " body=StaticOriginId("),
        captures,
        target: parse_decimal_after(line, " target=ContinuationSpecializationId("),
    })
}

#[cfg(target_os = "linux")]
fn assert_retained_result_closure_population(stderr: &str, mode: &str) {
    let reported = stderr
        .lines()
        .filter_map(parse_retained_result_closure_row)
        .collect::<Vec<_>>();
    assert!(
        !reported.is_empty() && reported.len() % 6 == 0,
        "{mode}: every proof projection must report the fixed fixture's six retained result-closure rows; stderr:\n{stderr}"
    );
    let expected = reported[..6]
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        expected.len(),
        6,
        "{mode}: one retained result-closure row is duplicated in a projection; stderr:\n{stderr}"
    );
    for projection in reported.chunks_exact(6) {
        assert_eq!(
            projection
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>(),
            expected,
            "{mode}: repeated proof projection changed the typed population; stderr:\n{stderr}"
        );
    }
    let rows = expected.into_iter().collect::<Vec<_>>();

    let projections: [fn(&RetainedResultClosureRow) -> u64; 5] = [
        |row| row.result,
        |row| row.construct,
        |row| row.seat,
        |row| row.body,
        |row| row.target,
    ];
    for project in projections {
        assert_eq!(
            rows.iter()
                .map(project)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            rows.len(),
            "{mode}: one typed retained-result coordinate is duplicated; stderr:\n{stderr}"
        );
    }

    for row in &rows {
        assert_eq!(
            row.field, 1,
            "{mode}: retained result closure is not the recursive Result field: {row:?}"
        );
        assert!(
            row.body < row.seat,
            "{mode}: retained closure body does not precede its seat: {row:?}"
        );
        assert_eq!(
            row.captures,
            ((row.body + 1)..row.seat).rev().collect::<Vec<_>>(),
            "{mode}: retained closure captures are not the exact positional child run: {row:?}"
        );
    }

    let predeclared = rows
        .iter()
        .filter(|row| matches!(row.owner, RetainedResultClosureOwner::Predeclared(_)))
        .count();
    let specialized = rows
        .iter()
        .filter_map(|row| match row.owner {
            RetainedResultClosureOwner::Predeclared(_) => None,
            RetainedResultClosureOwner::Specialization(owner) => Some((owner, row)),
        })
        .collect::<Vec<_>>();
    assert_eq!(predeclared, 4, "{mode}: fixed fixture predeclared rows");
    assert_eq!(specialized.len(), 2, "{mode}: fixed fixture nested rows");
    for (source_target, child) in specialized {
        let parents = rows
            .iter()
            .filter(|parent| {
                matches!(parent.owner, RetainedResultClosureOwner::Predeclared(_))
                    && parent.target == source_target
                    && parent.body == child.result
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parents.len(),
            1,
            "{mode}: a specialization-owned retained row lacks its unique parent target/body relation: {child:?}"
        );
    }
}

#[cfg(target_os = "linux")]
fn assert_retained_unit_call_target_mutation_child() {
    use ken_runtime::RetainedUnitCallTargetMutation as Mutation;

    let mode = std::env::var(RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD)
        .expect("retained-unit call-target mutation child mode");
    let (mutation, expected) = match mode.as_str() {
        "unrelated-owner-root" => (
            Mutation::SubstituteUnrelatedOwnerRoot,
            "has no graph-derived call target in this unit",
        ),
        "suppress-graph-claims" => (
            Mutation::SuppressGraphClaims,
            "has no graph-derived call target in this unit",
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
fn assert_handler_owned_deferred_response_mutation_child() {
    use ken_runtime::HandlerOwnedDeferredResponseMutation as Mutation;

    let mode = std::env::var(HANDLER_OWNED_DEFERRED_RESPONSE_MUTATION_CHILD)
        .expect("handler-owned Deferred-response mutation child mode");
    let mutation = match mode.as_str() {
        "suppress-unitless-drive" => Mutation::SuppressUnitlessDrive,
        "suppress-local-continuation" => Mutation::SuppressLocalContinuationDrive,
        other => panic!("unknown handler-owned Deferred-response mutation: {other}"),
    };
    let dir = tempfile::Builder::new()
        .prefix("ken-px8f-handler-owned-response-control-")
        .tempdir()
        .unwrap();
    std::fs::write(dir.path().join("input.bin"), b"abcdef").unwrap();
    let preload = build_short_pwrite_preload(dir.path());
    let (compiled, applications) =
        ken_runtime::with_handler_owned_deferred_response_mutation(mutation, || {
            ken_cli::build_native_program(
                WRITE_ALL,
                ken_cli::SourceFormat::Ken,
                "px8f_write_all_handler_owned_response_control",
                dir.path(),
            )
        });
    assert_eq!(
        applications, 1,
        "{mode}: mutation must reach exactly one natural production consumer"
    );
    match mode.as_str() {
        "suppress-unitless-drive" => {
            let output = compiled.expect("suppressing P1 execution still emits the old artifact");
            let run = ken_runtime::run_bound_process_effect_observation(
                &output.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: vec![("LD_PRELOAD".into(), preload.into_os_string())],
                    cwd: dir.path().to_owned(),
                    plan_hash: output.plan_transport_hash,
                },
            );
            let observation = run.expect("the inert P1 artifact reports its source trap");
            assert_ne!(
                observation.exit_status, 0,
                "suppressing P1 execution preserved the successful program"
            );
            let Some(ken_runtime::TerminalErrorV1::RuntimeTrap(provenance)) =
                observation.terminal_error
            else {
                panic!("suppressing P1 execution did not restore the source Result trap");
            };
            assert_eq!(
                provenance.trap.code,
                ken_runtime::RuntimeTrapCode::PatternMatchFailure
            );
            assert!(
                provenance.trap.message.ends_with("::Result"),
                "the P1 suppression reached the wrong source trap: {:?}",
                provenance.trap
            );
            assert!(
                observation
                    .effect_trace
                    .iter()
                    .all(|event| event.operation != ken_runtime::HostOpV1::FsWriteAt),
                "the inert P1 path unexpectedly dispatched a write"
            );
            assert_eq!(
                std::fs::read(dir.path().join("output.bin")).unwrap_or_default(),
                b"",
                "suppressing P1 execution still wrote the output"
            );
        }
        "suppress-local-continuation" => {
            let error = compiled
                .expect_err("suppressing the handler-local continuation drive crossed its closure");
            assert!(
                format!("{error:?}").contains("a closure cannot cross the boundary"),
                "the local-continuation mutation reached the wrong refusal: {error:?}"
            );
        }
        _ => unreachable!("the mode was validated above"),
    }
    assert!(
        ken_runtime::handler_owned_deferred_response_mutation_is_exact(),
        "{mode}: scoped handler-owned response mutation did not restore"
    );
}

#[cfg(target_os = "linux")]
fn assert_hs17_static_response_return_mutation_child() {
    let mode = std::env::var(HS17_STATIC_RESPONSE_RETURN_MUTATION_CHILD)
        .expect("HS17 mutation child mode");
    let mutation = match mode.as_str() {
        "delete-boundary" => {
            ken_runtime::D5bHs17PostCallConsumerMutation::DeleteStaticResponseBoundary
        }
        "transplant-boundary" => {
            ken_runtime::D5bHs17PostCallConsumerMutation::TransplantStaticResponseBoundary
        }
        "wrong-forwarded-word" => {
            ken_runtime::D5bHs17PostCallConsumerMutation::SubstituteForwardedResultWord
        }
        "replay-selected-exit" => {
            ken_runtime::D5bHs17PostCallConsumerMutation::ReplayCompletedSelectedExit
        }
        "drop-residual-suffix" => ken_runtime::D5bHs17PostCallConsumerMutation::DropResidualSuffix,
        "mint-at-tail" => {
            ken_runtime::D5bHs17PostCallConsumerMutation::MintReceiptAtNonEmittingTail
        }
        other => panic!("unknown HS17 mutation child mode {other}"),
    };
    let dir = tempfile::Builder::new()
        .prefix("ken-px8f-hs17-control-")
        .tempdir()
        .unwrap();
    std::fs::write(dir.path().join("input.bin"), b"abcdef").unwrap();
    let preload = build_short_pwrite_preload(dir.path());
    let (built, applications) =
        ken_runtime::with_d5b_hs17_post_call_consumer_mutation(mutation, || {
            ken_cli::build_native_program(
                WRITE_ALL,
                ken_cli::SourceFormat::Ken,
                "px8f_write_all_hs17_control",
                dir.path(),
            )
        });
    assert!(
        applications > 0,
        "{mode}: mutation did not reach production"
    );
    match mode.as_str() {
        "transplant-boundary"
        | "wrong-forwarded-word"
        | "replay-selected-exit"
        | "mint-at-tail" => {
            let error = built.expect_err("malformed HS17 proof must refuse before an object");
            let text = format!("{error:?}");
            assert!(
                text.contains("static-response")
                    || text.contains("detached caller cut")
                    || text.contains("post-call consumer receipt")
                    || text.contains("non-emitting Tail"),
                "{mode}: wrong refusal: {text}"
            );
        }
        "delete-boundary" | "drop-residual-suffix" => {
            let built = built.expect("control keeps the outer ABI shape buildable");
            let observation = ken_runtime::run_bound_process_effect_observation(
                &built.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: vec![("LD_PRELOAD".into(), preload.into_os_string())],
                    cwd: dir.path().to_owned(),
                    plan_hash: built.plan_transport_hash,
                },
            )
            .expect("HS17 wrong-stage control executes to its fail-closed frontier");
            assert_ne!(
                observation.exit_status, 0,
                "{mode}: wrong cut reached success"
            );
            let frontier = format!("{:?}", observation.terminal_error);
            assert!(
                frontier.contains("PatternMatchFailure"),
                "{mode}: dropped proof obligation was not rejected: {frontier}"
            );
            if mode == "delete-boundary" {
                assert!(
                    frontier.contains("Result"),
                    "{mode}: outer-tag-equal wrong stage was not rejected: {frontier}"
                );
            }
        }
        _ => unreachable!(),
    }
    assert!(
        ken_runtime::d5b_hs17_post_call_consumer_mutation_is_exact(),
        "{mode}: scoped HS17 mutation did not restore"
    );
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. The native run and interpreter must agree
/// on the ordered short-write observations required by runtime evaluation
/// (`spec/40-runtime/42-evaluation.md` section 6.2 and
/// `spec/40-runtime/45-native-backend.md` section 4).
#[test]
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
/// Promise class: durable invariant. The static-response return receipt is
/// issued only by the exact selected-owner call and removes only its completed
/// source cut; equal outer shape never substitutes for that proof.
#[test]
fn static_response_return_boundary_controls_are_reaching() {
    for mode in [
        "delete-boundary",
        "transplant-boundary",
        "wrong-forwarded-word",
        "replay-selected-exit",
        "drop-residual-suffix",
        "mint-at-tail",
    ] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "linked_checked_write_all_observes_short_progress_and_matches_interpreter",
                "--nocapture",
            ])
            .env(HS17_STATIC_RESPONSE_RETURN_MUTATION_CHILD, mode)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn isolated HS17 static-response mutation child");
        assert!(
            output.status.success(),
            "{mode}: mutation child failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
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
            "has no graph-derived call target in this unit",
        ),
        (
            "suppress-graph-claims",
            "has no graph-derived call target in this unit",
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
/// lexical occurrence, or dropping the exact downstream call edge reaches a
/// distinct production refusal. Each malformed child also reports the fixed
/// fixture's six unique rows, exact descending capture runs, and both nested
/// specialization-to-parent target/body joins.
/// CLAIMED: only the exact result/constructor/field/closure/body/capture/target
/// tuple and its joined static call edge may acquire the existing M4 environment
/// representation. Handler-owned Deferred continuations no longer cross this
/// boundary; their replacement control is
/// `handler_owned_deferred_response_controls_are_load_bearing`.
/// THE GAP: the fixed fixture supplies four predeclared and two nested rows; the
/// assertions identify them by typed relations, never planner allocation ids.
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
    ];
    for (mode, expected) in cases {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "linked_checked_write_all_observes_short_progress_and_matches_interpreter",
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
            assert_retained_result_closure_population(&stderr, mode);
        }
    }
}

#[cfg(target_os = "linux")]
/// Promise class: durable mutation proof. Dropping the remaining handler-owned
/// P1 execution restores the inert no-write path on the real WRITE_ALL compile.
/// The former local-K control was subsumed when that route became the exact
/// static-response return boundary; its reaching controls are the HS17 test.
///
/// MEASURED: the isolated child mutates the natural P1 production consumer,
/// reports one application, and observes its pre-recut failure.
/// CLAIMED: executable P1 dispatch remains necessary for the carried success
/// value. THE GAP: the unchanged positive row proves the same fixture reaches
/// exit zero with the exact three writes; the child proves causality.
#[test]
fn handler_owned_deferred_response_controls_are_load_bearing() {
    for mode in ["suppress-unitless-drive"] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "linked_checked_write_all_observes_short_progress_and_matches_interpreter",
                "--nocapture",
            ])
            .env(HANDLER_OWNED_DEFERRED_RESPONSE_MUTATION_CHILD, mode)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn isolated handler-owned Deferred-response mutation child");
        assert!(
            output.status.success(),
            "{mode}: mutation child failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
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
    if std::env::var_os(HANDLER_OWNED_DEFERRED_RESPONSE_MUTATION_CHILD).is_some() {
        assert_handler_owned_deferred_response_mutation_child();
        return;
    }
    if std::env::var_os(HS17_STATIC_RESPONSE_RETURN_MUTATION_CHILD).is_some() {
        assert_hs17_static_response_return_mutation_child();
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
    let reads: Vec<_> = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::FsReadAt)
        .collect();
    assert_eq!(reads.len(), 1, "the source must be read exactly once");
    assert!(matches!(
        &reads[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(
            ken_runtime::CanonicalReplyV1::ReadProgress(
                ken_runtime::ReadProgressV1::ReadSome { span, transferred }
            )
        ) if span.start() == 0
            && span.length() == 6
            && transferred.get() == 6
            && transferred.effective_request() == 6
    ));

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
    let releases: Vec<_> = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .collect();
    assert_eq!(
        releases.len(),
        3,
        "every acquired resource must be released"
    );
    assert!(releases.iter().all(|event| matches!(
        &event.outcome,
        ken_runtime::CanonicalOutcomeV1::Success(
            ken_runtime::CanonicalReplyV1::ResourceSettlement(settlement)
        ) if format!("{:?}", settlement.outcome) == "Released"
    )));

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
// Baseline provisioning, not a depth claim. With ambient RUST_MIN_STACK absent,
// the exact three-arm compile aborted at 2 MiB twice and completed at 4 MiB
// twice. Treat 4 MiB as the conservative measured peak. Retaining this test's
// pre-existing 256 MiB provision adds exactly 252 MiB of headroom; the named
// local Builder stack, rather than an ambient harness default, is operative.
const WRITE_ALL_CLASSIFIER_STACK_MEASURED_PEAK_BYTES: usize = 4 * 1024 * 1024;
#[cfg(target_os = "linux")]
const WRITE_ALL_CLASSIFIER_STACK_HEADROOM_BYTES: usize = 252 * 1024 * 1024;
#[cfg(target_os = "linux")]
const WRITE_ALL_CLASSIFIER_STACK_BYTES: usize =
    WRITE_ALL_CLASSIFIER_STACK_MEASURED_PEAK_BYTES + WRITE_ALL_CLASSIFIER_STACK_HEADROOM_BYTES;

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. The checked `writeAll` response plane
/// promotes exactly its two exclusively-predeclared producer groups while the
/// unit-less P1 and mixed-owner group retain their existing lowering paths.
///
/// MEASURED: the exact FsReadAt and BufferAllocate rows acquire response owners
/// and compile beside the one FsWriteAt P1 row and three mixed-owner
/// ResourceRelease rows. Suppression restores those two rows to P2, while
/// deliberately over-promoting all three ResourceRelease rows reaches the
/// owner-escape refusal. CLAIMED: the Route-B ownership restriction is both
/// sufficient and necessary for sound partial specialization. THE GAP: the
/// successful `ReadSome` body still belongs to the held parent carry WP, so this
/// predecessor does not claim the runtime right-path witness.
#[test]
fn write_all_classifies_mixed_specialized_and_deferred_responses() {
    std::thread::Builder::new()
        .name("px8f-classify-mixed".to_string())
        .stack_size(WRITE_ALL_CLASSIFIER_STACK_BYTES)
        .spawn(|| {
            let dir = tempfile::Builder::new()
                .prefix("ken-px8f-classify-mixed-")
                .tempdir()
                .unwrap();
            let compile = |package: &str| {
                ken_runtime::with_static_response_feasibility_diagnostics(|| {
                    ken_cli::build_native_program(
                        WRITE_ALL,
                        ken_cli::SourceFormat::Ken,
                        package,
                        dir.path(),
                    )
                })
            };

            let ((result, diagnostics), hs11_observations, hs11_applications) =
                ken_runtime::with_d5b_hs11_materializer_mutation(
                    ken_runtime::D5bHs11MaterializerMutation::Exact,
                    || compile("px8f_write_all_plane_closed"),
                );
            result.expect("the sound mixed response plane compiles without owner escape");
            assert_eq!(hs11_applications, 0, "the exact HS11 arm mutates nothing");
            assert!(
                ken_runtime::d5b_hs11_materializer_mutation_is_exact(),
                "the exact HS11 scope must restore before assertions"
            );
            let carried = hs11_observations
                .iter()
                .filter(|row| row.contains_carried)
                .collect::<Vec<_>>();
            assert_eq!(
                carried,
                vec![
                    &ken_runtime::D5bHs11MaterializerObservation {
                        shell_origin: 1639,
                        selected_origin: 1638,
                        occurrence: 343,
                        whole_bound: true,
                        contains_carried: true,
                        completion: ken_runtime::D5bHs11MaterializerCompletion::WholeTransferred,
                    },
                    &ken_runtime::D5bHs11MaterializerObservation {
                        shell_origin: 1640,
                        selected_origin: 1639,
                        occurrence: 344,
                        whole_bound: false,
                        contains_carried: true,
                        completion: ken_runtime::D5bHs11MaterializerCompletion::ImmediateFields,
                    },
                ],
                "the carried source completes shell 1639 exactly once at occurrence 343, while \
                 immediate Vis shell 1640 installs fields without whole construction"
            );
            assert_eq!(diagnostics.len(), 1, "one compile publishes one plan");
            let diagnostic = diagnostics.into_iter().next().unwrap();
            assert_eq!(diagnostic.static_response_infeasible, None);
            assert_eq!(diagnostic.all_static_response_infeasible, None);

            let specialized = diagnostic
                .all_static_response_rows
                .iter()
                .map(|row| row.operation.as_str())
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(diagnostic.all_static_response_rows.len(), 2);
            assert_eq!(
                specialized,
                std::collections::BTreeSet::from(["FsReadAt", "BufferAllocate"]),
                "only the two exclusively-predeclared groups acquire response owners"
            );
            let deferred = diagnostic.static_response_deferred.iter().fold(
                std::collections::BTreeMap::new(),
                |mut counts, row| {
                    *counts
                        .entry((row.operation.as_str(), row.sub_case.as_str()))
                        .or_insert(0usize) += 1;
                    counts
                },
            );
            assert_eq!(
                deferred,
                std::collections::BTreeMap::from([
                    (("FsOpen", "InlineBridgeNoCall"), 1),
                    (("FsWriteAt", "NoContinuationUnit"), 1),
                    (("ResourceRelease", "UnconsumedTransportCaller"), 3),
                ]),
                "the immediate bridge stays owner-free, P1 stays main-lowered, and the mixed-owner group stays P2"
            );

            // Gate 0, on continuation structure rather than the RecursiveBackedge
            // marker. The maximum path use is one, every tail returns or makes one
            // static declared call, and no other exit can publish the continuation.
            // The P1 owner must be the exact promoted BufferAllocate handler.
            let buffer_handler = diagnostic
                .all_static_response_rows
                .iter()
                .find(|row| row.operation == "BufferAllocate")
                .expect("the promoted BufferAllocate response remains present")
                .base_owner
                .clone();
            let p1 = diagnostic
                .static_response_deferred
                .iter()
                .find(|row| row.operation == "FsWriteAt" && row.sub_case == "NoContinuationUnit")
                .expect("the P1 response remains in the Deferred population");
            assert_eq!(p1.handler_owner.as_deref(), Some(buffer_handler.as_str()));
            for row in &diagnostic.static_response_deferred {
                if row.sub_case == "InlineBridgeNoCall" {
                    assert_eq!(row.handler_owner, None, "an immediate bridge has no response owner");
                    continue;
                }
                if row.handler_owner.is_none() || row.k_body_origin.is_none() {
                    assert_eq!(
                        (row.operation.as_str(), row.sub_case.as_str()),
                        ("ResourceRelease", "UnconsumedTransportCaller"),
                        "only a mixed-owner release residual may lack one static handler"
                    );
                    continue;
                }
                assert_eq!(
                    row.response_uses,
                    Some(1),
                    "Deferred response {} can use one host response more than once on one path",
                    row.vis_origin
                );
                assert!(
                    row.tail_ret_exits.is_some_and(|exits| exits > 0),
                    "Deferred response {} has no tail Ret exit",
                    row.vis_origin
                );
                assert_eq!(
                    row.tail_other_exits,
                    Some(0),
                    "Deferred response {} has a non-local or non-tail exit",
                    row.vis_origin
                );
            }
            assert_eq!(
                p1.tail_static_calls,
                Some(1),
                "P1 short progress must recurse through one static tail call"
            );
            assert!(
                diagnostic
                    .static_response_deferred
                    .iter()
                    .filter(|row| row.operation == "ResourceRelease")
                    .all(|row| row.tail_static_calls == Some(0)),
                "resource-release continuations must return directly rather than recurse"
            );

            let (suppressed_result, suppressed_diagnostics) =
                ken_runtime::with_suppressed_execute_then_resume_response(|| {
                    compile("px8f_write_all_plane_suppressed")
                });
            suppressed_result.expect("the suppression control retains main lowering");
            assert_eq!(
                suppressed_diagnostics.len(),
                1,
                "one suppressed compile publishes one plan"
            );
            let suppressed = suppressed_diagnostics.into_iter().next().unwrap();
            let suppressed_specialized = suppressed
                .all_static_response_rows
                .iter()
                .map(|row| row.operation.as_str())
                .collect::<Vec<_>>();
            assert_eq!(
                suppressed_specialized,
                Vec::<&str>::new(),
                "suppression must remove both execute-then-resume owners while the immediate bridge remains owner-free"
            );
            let suppressed_p2 = suppressed
                .static_response_deferred
                .iter()
                .filter(|row| row.sub_case == "UnconsumedTransportCaller")
                .fold(std::collections::BTreeMap::new(), |mut counts, row| {
                    *counts.entry(row.operation.as_str()).or_insert(0usize) += 1;
                    counts
                });
            assert_eq!(
                suppressed_p2,
                std::collections::BTreeMap::from([
                    ("FsReadAt", 1),
                    ("BufferAllocate", 1),
                    ("ResourceRelease", 3),
                ]),
                "restoring the veto must reopen the ordinary groups as P2"
            );
            assert!(
                ken_runtime::suppressed_execute_then_resume_response_is_exact(),
                "the suppression hook did not restore"
            );

            let ((overpromoted_result, overpromoted_diagnostics), applications) =
                ken_runtime::with_mixed_owner_execute_then_resume_overpromotion(|| {
                    compile("px8f_write_all_plane_overpromoted")
                });
            let error =
                overpromoted_result.expect_err("over-promoting the mixed-owner group compiled");
            let rendered = format!("{error:?}");
            assert!(
                rendered.contains(
                    "a deferred host response is compiler control and can only enter its exact \
                     response owner"
                ) || rendered.contains(
                    "one generated-context Result word acquired terminal authority more than once"
                ),
                "over-promotion reached the wrong refusal: {error:?}"
            );
            assert_eq!(
                overpromoted_diagnostics.len(),
                1,
                "the over-promoted plan must reach the lowering refusal"
            );
            assert_eq!(
                applications, 6,
                "the mutation must over-promote all three mixed-owner responses in both the \
                 install and its closed re-derivation"
            );
            assert!(
                ken_runtime::mixed_owner_execute_then_resume_overpromotion_is_exact(),
                "the over-promotion hook did not restore"
            );
        })
        .expect("spawn large-stack classify-mixed probe")
        .join()
        .expect("classify-mixed probe thread");
}

#[cfg(target_os = "linux")]
/// Promise class: durable mutation proof. The phase-preserving materializer
/// completes only the real whole-bound shell, leaves the immediate Vis as
/// direct fields, and uses the deferred constructor's stored producer origin.
#[test]
fn deferred_constructor_materializer_completion_is_exact() {
    std::thread::Builder::new()
        .name("px8f-hs11-materializer".to_string())
        .stack_size(WRITE_ALL_CLASSIFIER_STACK_BYTES)
        .spawn(|| {
            let dir = tempfile::Builder::new()
                .prefix("ken-px8f-hs11-materializer-")
                .tempdir()
                .unwrap();
            let compile = |package: &str| {
                ken_cli::build_native_program(
                    WRITE_ALL,
                    ken_cli::SourceFormat::Ken,
                    package,
                    dir.path(),
                )
            };

            let (dropped, dropped_rows, dropped_applications) =
                ken_runtime::with_d5b_hs11_materializer_mutation(
                    ken_runtime::D5bHs11MaterializerMutation::DropWholeBoundCompletion,
                    || compile("px8f_hs11_drop_whole_bound"),
                );
            assert_eq!(dropped_applications, 1);
            let dropped_error = dropped.expect_err(
                "dropping the whole-bound Coproduct completion must redden the witness",
            );
            assert!(
                format!("{dropped_error:?}")
                    .contains("a deferred constructor field requires its selected operand"),
                "the missing whole shell must refuse at the next real deferred edge: \
                 {dropped_error:?}"
            );
            assert!(dropped_rows.iter().any(|row| {
                row.shell_origin == 1639
                    && row.selected_origin == 1638
                    && row.occurrence == 343
                    && row.completion == ken_runtime::D5bHs11MaterializerCompletion::WholeDropped
            }));
            assert!(ken_runtime::d5b_hs11_materializer_mutation_is_exact());

            let (overbuilt, overbuilt_rows, overbuilt_applications) =
                ken_runtime::with_d5b_hs11_materializer_mutation(
                    ken_runtime::D5bHs11MaterializerMutation::MaterializeImmediateShell,
                    || compile("px8f_hs11_materialize_immediate"),
                );
            let _ = overbuilt;
            assert_eq!(overbuilt_applications, 1);
            assert!(overbuilt_rows.iter().any(|row| {
                row.shell_origin == 1640
                    && row.selected_origin == 1639
                    && row.occurrence == 344
                    && !row.whole_bound
                    && row.completion
                        == ken_runtime::D5bHs11MaterializerCompletion::WholeTransferred
            }));
            assert!(ken_runtime::d5b_hs11_materializer_mutation_is_exact());

            let (wrong_origin, wrong_origin_rows, wrong_origin_applications) =
                ken_runtime::with_d5b_hs11_materializer_mutation(
                    ken_runtime::D5bHs11MaterializerMutation::UseCurrentFrameOrigin,
                    || compile("px8f_hs11_current_frame_origin"),
                );
            assert_eq!(wrong_origin_applications, 1);
            let wrong_origin_error = wrong_origin
                .expect_err("using the current frame instead of producer origin must refuse");
            assert!(
                format!("{wrong_origin_error:?}").contains("aggregate")
                    || format!("{wrong_origin_error:?}").contains("no ConstructorSymbol atom"),
                "the wrong origin must refuse at aggregate authority before allocation: \
                 {wrong_origin_error:?}"
            );
            assert!(
                wrong_origin_rows.is_empty(),
                "a failed authority lookup must publish no completed shell"
            );
            assert!(ken_runtime::d5b_hs11_materializer_mutation_is_exact());

            let (restored, restored_rows, restored_applications) =
                ken_runtime::with_d5b_hs11_materializer_mutation(
                    ken_runtime::D5bHs11MaterializerMutation::Exact,
                    || compile("px8f_hs11_restored"),
                );
            restored.expect("restoration to exact must compile the unchanged witness");
            assert_eq!(restored_applications, 0);
            let restored_carried = restored_rows
                .iter()
                .filter(|row| row.contains_carried)
                .collect::<Vec<_>>();
            assert_eq!(restored_carried.len(), 2);
            assert_eq!(restored_carried[0].shell_origin, 1639);
            assert_eq!(restored_carried[1].shell_origin, 1640);
        })
        .expect("spawn HS11 materializer proof thread")
        .join()
        .expect("HS11 materializer proof thread");
}

#[cfg(target_os = "linux")]
/// Promise class: durable mutation proof. MEASURED: the discharge ledger of
/// `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `D2` refuses a constructor word that
/// discharges a second Result obligation, on an arm-1/arm-2 collision built at
/// the production site that assembles `required` -- never at the ledger, which
/// would test the guard against itself. CLAIMED: consumption is a move, and the
/// missing case is a located planner error rather than a silent pass. THE GAP:
/// this pins the collision the ledger was widened to see; it does not claim the
/// collision is reachable from ordinary Ken source, which is unmeasured and
/// deliberately not read either way.
///
/// The two halves share ONE input and differ only in whether the ledger's
/// refusal is live, so a flipped guard fails both rather than neither. The
/// execution witness is the applications count plus the fail-closed refusal the
/// mutation raises when it finds no site: a control that silently declines to
/// fire is the dead instrument this node exists to rule out.
#[test]
#[ignore = "IGNORED BECAUSE ITS SHAPE IS ABSENT HERE, NOT BECAUSE IT IS UNFINISHED. \
            Measured at WRITE_ALL: staged=14, independent_and_published=2, \
            demanded_per_independent_body=[(1, true), (1, true)] -- both bodies carrying an \
            independent contract are demanded under exactly one identity, and it is their own \
            contract, so arm 2 never runs for either and the arm-1/arm-2 collision cannot be \
            built. Manufacturing it would mean synthesizing an identity, which \
            RT-CONSTRUCTOR-AUTHORITY-DISCHARGE D1a/D1b forbid in terms. \
            REIFY THIS TEST when a Ken source program is shown in which two callers demand \
            different constructor identities of one response-owner body -- that is the open \
            reachability node; drop this attribute and point the fixture at that source. \
            Until then the ledger is the SOLE net for this shape: the :4394 missing-contract \
            diagnostic is structurally unreachable in the collision case, because the collision \
            requires BOTH demands proven, which leaves `missing` empty."]
fn discharge_ledger_refuses_one_word_discharging_two_obligations() {
    std::thread::Builder::new()
        .name("px8f-d2-discharge-ledger".to_string())
        .stack_size(WRITE_ALL_CLASSIFIER_STACK_BYTES)
        .spawn(|| {
            use ken_runtime::GeneratedResultPathProofMutation::{
                DemandIndependentBodySecondIdentity, DemandIndependentBodySecondIdentityUnguarded,
                Exact,
            };

            let dir = tempfile::Builder::new()
                .prefix("ken-px8f-d2-discharge-ledger-")
                .tempdir()
                .unwrap();
            let compile = |package: &str| {
                ken_cli::build_native_program(
                    WRITE_ALL,
                    ken_cli::SourceFormat::Ken,
                    package,
                    dir.path(),
                )
            };

            const COULD_NOT_FIRE: &str = "so the mutation could not fire";
            const REFUSAL: &str =
                "one generated-Result constructor word discharges two Result obligations";

            // POSITIVE. The collision is built and the ledger must refuse it.
            let (guarded, guarded_applications) =
                ken_runtime::with_generated_result_path_proof_mutation(
                    DemandIndependentBodySecondIdentity,
                    || compile("px8f_d2_discharge_ledger_guarded"),
                );
            assert_eq!(
                guarded_applications, 1,
                "the discharge-ledger mutation must reach the required-assembly site exactly once"
            );
            let guarded = format!(
                "{:?}",
                guarded.expect_err("a word discharging two obligations must not compile")
            );
            assert!(
                !guarded.contains(COULD_NOT_FIRE),
                "the discharge-ledger control is a DEAD INSTRUMENT here: the mutation found no \
                 body whose published Result word is also an identity-bearing call obligation's \
                 result word, so it never built the collision it is meant to prove: {guarded}"
            );
            assert!(
                guarded.contains(REFUSAL),
                "the collision reached the wrong refusal, so this control does not pin the \
                 ledger: {guarded}"
            );

            // NEGATIVE, on the SAME input. With the ledger's refusal suppressed
            // -- the pre-repair path -- the identical double discharge compiles
            // and says nothing. Without this half, the positive cannot tell a
            // working ledger from a compile that was going to fail anyway.
            let (unguarded, unguarded_applications) =
                ken_runtime::with_generated_result_path_proof_mutation(
                    DemandIndependentBodySecondIdentityUnguarded,
                    || compile("px8f_d2_discharge_ledger_unguarded"),
                );
            assert_eq!(unguarded_applications, 1);
            unguarded.expect(
                "the pre-repair path must pass the identical double discharge SILENTLY -- if it \
                 also refuses, the positive half is not attributable to the ledger",
            );

            // The mutation is scoped and restores.
            let (exact, exact_applications) =
                ken_runtime::with_generated_result_path_proof_mutation(Exact, || {
                    compile("px8f_d2_discharge_ledger_exact")
                });
            assert_eq!(exact_applications, 0);
            exact.expect("the unmutated source must still compile");
        })
        .expect("spawn D2 discharge-ledger proof thread")
        .join()
        .expect("D2 discharge-ledger proof thread");
}
