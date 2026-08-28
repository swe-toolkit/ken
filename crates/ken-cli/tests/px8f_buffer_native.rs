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
/// Promise class: durable invariant. The native run and interpreter must agree
/// on the ordered short-write observations required by runtime evaluation
/// (`spec/40-runtime/42-evaluation.md` section 6.2 and
/// `spec/40-runtime/45-native-backend.md` section 4).
#[test]
#[ignore = "RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION: retained-unit target derivation succeeds; post-call constructor composition next refuses the result closure representation"]
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
fn run_linked_checked_write_all() {
    use std::os::unix::ffi::OsStrExt as _;

    if std::env::var_os(RETAINED_UNIT_CALL_TARGET_MUTATION_CHILD).is_some() {
        assert_retained_unit_call_target_mutation_child();
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
