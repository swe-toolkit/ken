//! ABI-S6 D5b: checked file-backed MAP_PRIVATE mapping differential.
//!
//! Promise class: normative compatibility vector from `38 §1.9` and
//! `conformance/surface/ffi-io/seed-mapping.md` case 1. The same checked program
//! runs through the native mmap-of-fd path and the interpreter's in-process
//! model. Both must observe a private write through `mapBytes`, while an ordinary
//! file read and the backing file itself retain the original bytes.

#![cfg(target_os = "linux")]

const SOURCE: &str = r#"program capabilities FS AFull
const body_ok_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyOk Unit Unit MkUnit)

const body_error_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyErr Unit Unit MkUnit)

proc after_mapping_write (mapping : MappingHandle)
  (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (2 : Int) (3 : Int)))
      (\read. match read {
        Err error |-> body_error_io;
        Ok bytes |-> body_ok_io
      })
  }

proc mapping_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (2 : Int) (bytes_encode "NEW"))
    (\outcome. after_mapping_write mapping outcome)

fn mapping_bracket_body
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err error |-> body_error_io;
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> body_ok_io;
      ResourceBracketBodyError error |-> body_error_io;
      ResourceBracketReleaseError error |-> body_error_io;
      ResourceBracketBodyAndReleaseError body_error release_error |-> body_error_io
    }
  }

proc file_body (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withMapping AFull Unit Unit
      (FileBacked file (8 : Int)) ReadWrite mapping_body)
    (\outcome. mapping_bracket_body outcome)

fn finish_file_read (outcome : Result FileError Bytes) : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 95);
    Ok bytes |-> host_exit AFull Success
  }

proc after_file_bracket (cap : Cap AFull)
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> host_exit AFull (Failure 91);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError Bytes) ExitCode
        (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
          (Result FileError Bytes)
          (read_bytes AFull cap (bytes_encode "mapped.bin")))
        (\read. finish_file_read read);
      ResourceBracketBodyError error |-> host_exit AFull (Failure 92);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 93);
      ResourceBracketBodyAndReleaseError body_error release_error |->
        host_exit AFull (Failure 94)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
      (withResource AFull Unit Unit cap (bytes_encode "mapped.bin")
        (ResourceWriteCreate CreateOrKeep) file_body)
      (\outcome. after_file_bracket cap outcome)
  }
"#;

const ORIGINAL: &[u8] = b"abcdefgh";
const PRIVATE_WRITE: &[u8] = b"NEW";

struct Differential {
    interpreted: ken_runtime::EffectObservation,
    native: ken_runtime::EffectObservation,
    native_backing: Vec<u8>,
    interpreted_backing: Vec<u8>,
}

fn differential() -> Differential {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-file-cow-")
        .tempdir()
        .expect("creates temporary root");
    let path = root.path().join("mapped.bin");
    std::fs::write(&path, ORIGINAL).expect("writes native backing fixture");
    let output = ken_cli::build_native_program(
        SOURCE,
        ken_cli::SourceFormat::Ken,
        "abi_s6_d5b_file_cow",
        root.path(),
    )
    .expect("D5b checked source lowers natively");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("D5b native file mapping executes");
    let native_backing = std::fs::read(&path).expect("reads native backing file");

    std::fs::write(&path, ORIGINAL).expect("resets interpreter backing fixture");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        SOURCE,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("D5b interpreted file mapping executes");
    let interpreted_backing = std::fs::read(&path).expect("reads interpreted backing file");

    Differential {
        interpreted,
        native,
        native_backing,
        interpreted_backing,
    }
}

fn operation_events(
    observation: &ken_runtime::EffectObservation,
    operation: ken_runtime::HostOpV1,
) -> Vec<&ken_runtime::EffectEvent> {
    observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == operation)
        .collect()
}

/// MEASURED: both engines execute the same checked nested resource brackets;
/// MappingAcquireFile uses the held FsHandle, mapWrite changes [2,5), mapBytes
/// returns `NEW`, both terminal releases succeed, and the later FsReadFile plus
/// the on-disk file still carry `abcdefgh`. CLAIMED: D5b is real file-backed
/// MAP_PRIVATE COW with copied views and no write-through. THE GAP: external
/// concurrent truncation is outside this single-invocation fixture; short-source
/// refusal and source-lineage revocation remain pinned at the shared dispatcher.
///
/// Mutation control: replace `file_map_flags_v1()`'s `MAP_PRIVATE` with
/// `MAP_SHARED` and run this unchanged test. The native backing and FsReadFile
/// assertions redden while the in-mapping read stays green, distinguishing COW
/// from the tempting write-through implementation.
#[test]
fn file_backed_mapping_is_private_in_both_engines_and_preserves_the_file() {
    use ken_runtime::HostOpV1::{
        FsOpen, FsReadFile, MappingAcquireFile, MappingReadView, MappingWriteView, ResourceRelease,
    };

    let ((result, calls, hs3_applications), edges, compositions, hs5_applications) =
        ken_runtime::with_d5b_hs5_source_parent_mutation(
            ken_runtime::D5bHs5SourceParentMutation::Exact,
            || {
                ken_runtime::with_d5b_hs3_call_mutation(
                    ken_runtime::D5bHs3CallMutation::Exact,
                    differential,
                )
            },
        );
    assert_eq!(hs3_applications, 0, "the exact HS3 control applies no mutation");
    assert_eq!(hs5_applications, 0, "the exact HS5 control applies no mutation");
    let [first, second] = edges.as_slice() else {
        panic!("the nested COW path must mint exactly two checked-IH edges: {edges:?}");
    };
    assert!(
        first.parent_is_distinguished_root(),
        "the first checked-IH edge begins at the distinguished root"
    );
    assert_eq!(
        second.parent_invocation_instance_id,
        first.child_invocation_instance_id,
        "the second checked-IH edge's parent is the first edge's child"
    );
    assert_ne!(
        second.child_invocation_instance_id,
        first.child_invocation_instance_id,
        "the nested checked-IH edge mints a distinct child invocation"
    );
    assert_eq!(
        (
            second.checked_call_template_id,
            second.parent_frame_template_id,
            second.segment_site_id,
        ),
        (
            first.checked_call_template_id,
            first.parent_frame_template_id,
            first.segment_site_id,
        ),
        "both dynamic edges retain the same plan-named call, parent frame, and segment"
    );
    let [composition] = compositions.as_slice() else {
        panic!(
            "the nested child must consume exactly one transient source parent: {compositions:?}"
        );
    };
    assert_eq!(
        (
            composition.external_parent_invocation_instance_id,
            composition.external_parent_frame_template_id,
        ),
        (
            second.parent_invocation_instance_id,
            second.parent_frame_template_id,
        ),
        "composition admits the exact invocation/frame parent key minted on edge 2"
    );
    assert_eq!(
        composition.matching_edge_count, 1,
        "the transient source parent must authorize exactly one incoming edge"
    );
    assert_eq!(
        (
            composition.child_invocation_instance_id,
            composition.child_frame_template_id,
        ),
        (
            second.child_invocation_instance_id,
            second.parent_frame_template_id,
        ),
        "frame 0 is instantiated under edge 2's newly minted child"
    );
    assert!(
        composition.child_key_present,
        "the child invocation/frame key must exist after instantiation"
    );
    assert_eq!(
        calls,
        vec![ken_runtime::D5bHs3CallObservation {
            application_arguments: 1,
            caller_worker_captures: 5,
            frame_worker_captures_available: 5,
            frame_worker_captures_emitted: 0,
            frame_context_captures_available: 4,
            frame_context_captures_emitted: 4,
        }],
        "the statically selected direct worker owns its complete Parameter run; \
         the exact-key frame contributes only the context Capture run"
    );
    assert_eq!(
        (result.native.exit_status, result.interpreted.exit_status),
        (0, 0),
        "both engines take the checked program's success branch"
    );
    assert_eq!(result.native.terminal_error, None);
    assert_eq!(result.interpreted.terminal_error, None);
    assert_eq!(result.native_backing, ORIGINAL);
    assert_eq!(result.interpreted_backing, ORIGINAL);

    for observation in [&result.native, &result.interpreted] {
        assert_eq!(
            observation
                .effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![
                FsOpen,
                MappingAcquireFile,
                MappingWriteView,
                MappingReadView,
                ResourceRelease,
                ResourceRelease,
                FsReadFile,
            ],
            "nested mapping and file brackets settle before ordinary file read"
        );
        let acquisitions = operation_events(observation, MappingAcquireFile);
        assert_eq!(acquisitions.len(), 1);
        assert!(matches!(
            acquisitions[0].request,
            ken_runtime::CanonicalRequestV1::MappingAcquireFile {
                length: 8,
                protection: _,
            }
        ));
        let mapped_reads = operation_events(observation, MappingReadView);
        assert_eq!(mapped_reads.len(), 1);
        assert!(matches!(
            &mapped_reads[0].outcome,
            ken_runtime::CanonicalOutcomeV1::Success(
                ken_runtime::CanonicalReplyV1::Bytes(bytes)
            ) if bytes.as_slice() == PRIVATE_WRITE
        ));
        let file_reads = operation_events(observation, FsReadFile);
        assert_eq!(file_reads.len(), 1);
        assert!(matches!(
            &file_reads[0].outcome,
            ken_runtime::CanonicalOutcomeV1::Success(
                ken_runtime::CanonicalReplyV1::Bytes(bytes)
            ) if bytes.as_slice() == ORIGINAL
        ));
        assert_eq!(operation_events(observation, ResourceRelease).len(), 2);
    }

    assert_eq!(
        result.native.effect_trace, result.interpreted.effect_trace,
        "native mmap and interpreter model preserve exact opaque trace semantics"
    );
    assert_eq!(
        result.native.terminal_exit,
        result.interpreted.terminal_exit
    );
}

/// Promise class: durable invariant. An explicit canonical source parent with
/// one exact `(0, frame)` edge is admitted without becoming the child or being
/// inferred from absence. Rejecting only that exact root must restore HS9 once;
/// RAII restoration then lets the unchanged COW path complete.
#[test]
fn exact_external_root_is_admitted_by_its_zero_frame_edge() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-hs9-external-root-")
        .tempdir()
        .expect("creates temporary root");
    let (mutated, applications) = ken_runtime::with_d5b_hs9_external_root_mutation(
        ken_runtime::D5bHs9ExternalRootMutation::RejectExactRoot,
        || {
            ken_cli::build_native_program(
                SOURCE,
                ken_cli::SourceFormat::Ken,
                "abi_s6_d5b_hs9_reject_external_root",
                root.path(),
            )
        },
    );
    assert_eq!(
        applications, 1,
        "the mutation must reject exactly one canonical root after its exact edge match"
    );
    let refusal = match mutated {
        Ok(_) => panic!("rejecting the exact canonical root must restore HS9"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains("an external source parent is not a non-root checked invocation"),
        "the mutation must restore the exact HS9 refusal: {refusal}"
    );

    let restored = differential();
    assert_eq!(
        (restored.native.exit_status, restored.interpreted.exit_status),
        (0, 0),
        "RAII restoration to Exact must let the unchanged COW path complete"
    );
    assert_eq!(restored.native_backing, ORIGINAL);
    assert_eq!(restored.interpreted_backing, ORIGINAL);
    assert_eq!(restored.native.effect_trace, restored.interpreted.effect_trace);
}

/// Promise class: durable invariant. An exact per-owner/per-Construct checked-IH
/// transport destination enters the transport-aware producer dispatcher even
/// when it first reaches ordinary `lower_expr` ingress. Bypassing only after
/// that exact lookup succeeds must restore HS8 once; RAII restoration then lets
/// the unchanged COW path complete.
#[test]
fn exact_transport_destination_does_not_bypass_producer_dispatch() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-hs8-transport-ingress-")
        .tempdir()
        .expect("creates temporary root");
    let (mutated, applications) = ken_runtime::with_d5b_hs8_transport_ingress_mutation(
        ken_runtime::D5bHs8TransportIngressMutation::BypassExactTransport,
        || {
            ken_cli::build_native_program(
                SOURCE,
                ken_cli::SourceFormat::Ken,
                "abi_s6_d5b_hs8_bypass_transport_ingress",
                root.path(),
            )
        },
    );
    assert_eq!(
        applications, 1,
        "the mutation must bypass exactly one confirmed transport destination"
    );
    let refusal = match mutated {
        Ok(_) => panic!("bypassing the exact transport destination must restore HS8"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains(
            "a deferred host response is compiler control and can only enter its exact response owner"
        ),
        "the mutation must restore the exact HS8 boundary refusal: {refusal}"
    );

    let restored = differential();
    assert_eq!(
        (restored.native.exit_status, restored.interpreted.exit_status),
        (0, 0),
        "RAII restoration to Exact must let the unchanged COW path complete"
    );
    assert_eq!(restored.native_backing, ORIGINAL);
    assert_eq!(restored.interpreted_backing, ORIGINAL);
    assert_eq!(restored.native.effect_trace, restored.interpreted.effect_trace);
}

/// Promise class: durable invariant. An edge settled by its full identity as
/// `InlineNoCall` is excluded from the detached constructor-required residual.
/// Ignoring only that exact disposition must reach once and restore the HS7
/// non-constructor refusal; RAII restoration then lets the same COW path finish.
#[test]
fn inline_no_call_disposition_clears_only_its_exact_detached_residual() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-hs7-inline-no-call-")
        .tempdir()
        .expect("creates temporary root");
    let (mutated, applications) =
        ken_runtime::with_d5b_hs7_detached_disposition_mutation(
            ken_runtime::D5bHs7DetachedDispositionMutation::IgnoreInlineNoCall,
            || {
                ken_cli::build_native_program(
                    SOURCE,
                    ken_cli::SourceFormat::Ken,
                    "abi_s6_d5b_hs7_ignore_inline_no_call",
                    root.path(),
                )
            },
        );
    assert_eq!(
        applications, 1,
        "the mutation must act on exactly one full-identity InlineNoCall edge"
    );
    let refusal = match mutated {
        Ok(_) => panic!("ignoring the InlineNoCall disposition must restore HS7"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains(
            "a projected causal call reached the detached-result seat with a unit result that \
             is not a specialized constructor"
        ),
        "the mutation must restore the exact HS7 non-constructor refusal: {refusal}"
    );

    let restored = differential();
    assert_eq!(
        (restored.native.exit_status, restored.interpreted.exit_status),
        (0, 0),
        "RAII restoration to Exact must let the unchanged COW path complete"
    );
    assert_eq!(restored.native_backing, ORIGINAL);
    assert_eq!(restored.interpreted_backing, ORIGINAL);
    assert_eq!(restored.native.effect_trace, restored.interpreted.effect_trace);
}

/// Promise class: durable invariant. Retaining the source parent's invocation
/// on the layer that must become the child reaches one transfer seam and
/// restores the exact HS5 frame-sequence refusal. The positive COW test above
/// is the paired exact control.
#[test]
fn source_parent_is_not_retained_in_the_child_layer() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-retain-parent-")
        .tempdir()
        .expect("creates temporary root");
    let (result, edges, compositions, applications) =
        ken_runtime::with_d5b_hs5_source_parent_mutation(
            ken_runtime::D5bHs5SourceParentMutation::RetainParentInChildLayer,
            || {
                ken_cli::build_native_program(
                    SOURCE,
                    ken_cli::SourceFormat::Ken,
                    "abi_s6_d5b_retain_parent",
                    root.path(),
                )
            },
        );
    assert_eq!(applications, 1, "the retain-parent mutation applies once");
    assert_eq!(edges.len(), 2, "the second edge is minted before instantiation");
    assert!(compositions.is_empty(), "instantiation refuses before composition");
    let refusal = match result {
        Ok(_) => panic!("retaining the parent in the child layer must refuse"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains("expected={0} instantiated={}")
            && refusal.contains("actual=[(Some(0), Some(1))]"),
        "the mutation must restore the exact HS5 refusal: {refusal}"
    );
}

/// Promise class: durable invariant. Unqualifying the child without supplying
/// the captured source parent to mint reaches one transfer seam and restores
/// the source-open-parent cross-check rather than inferring a parent.
#[test]
fn source_parent_is_required_at_mint() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-drop-mint-parent-")
        .tempdir()
        .expect("creates temporary root");
    let (result, edges, compositions, applications) =
        ken_runtime::with_d5b_hs5_source_parent_mutation(
            ken_runtime::D5bHs5SourceParentMutation::DropSourceParentAtMint,
            || {
                ken_cli::build_native_program(
                    SOURCE,
                    ken_cli::SourceFormat::Ken,
                    "abi_s6_d5b_drop_mint_parent",
                    root.path(),
                )
            },
        );
    assert_eq!(applications, 1, "the drop-at-mint mutation applies once");
    assert_eq!(edges.len(), 2, "the malformed second edge is observed at mint");
    assert!(compositions.is_empty(), "source validation refuses before composition");
    let refusal = match result {
        Ok(_) => panic!("withholding the source parent at mint must refuse"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains(
            "source open occurrence disagrees with the closure-selected dynamic parent"
        ),
        "the mutation must restore the exact HS4 parent refusal: {refusal}"
    );
}

/// Promise class: durable invariant. Withholding only the already-validated
/// transient parent from composition reaches one source seam and leaves the
/// child instantiated, then restores the stale-parent tree refusal.
#[test]
fn source_parent_is_required_at_compose() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-drop-compose-parent-")
        .tempdir()
        .expect("creates temporary root");
    let (result, edges, compositions, applications) =
        ken_runtime::with_d5b_hs5_source_parent_mutation(
            ken_runtime::D5bHs5SourceParentMutation::DropSourceParentAtCompose,
            || {
                ken_cli::build_native_program(
                    SOURCE,
                    ken_cli::SourceFormat::Ken,
                    "abi_s6_d5b_drop_compose_parent",
                    root.path(),
                )
            },
        );
    assert_eq!(applications, 1, "the drop-at-compose mutation applies once");
    assert_eq!(edges.len(), 2, "mint and source-parent validation both complete");
    assert!(compositions.is_empty(), "the external tuple is withheld at compose");
    let refusal = match result {
        Ok(_) => panic!("withholding the source parent at compose must refuse"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains("dynamic splice edge names a stale parent invocation"),
        "the mutation must refuse after child instantiation at the tree check: {refusal}"
    );
}

/// Promise class: durable invariant. Re-appending the exact-key frame's worker
/// vector to a complete direct-worker call must reach the production partition
/// and reproduce the 11-for-6 refusal. The positive test above distinguishes
/// this from an unrelated compile failure and proves the unmutated source.
#[test]
fn complete_direct_worker_refuses_duplicate_frame_worker_captures() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-duplicate-workers-")
        .tempdir()
        .expect("creates temporary root");
    let (result, observations, applications) = ken_runtime::with_d5b_hs3_call_mutation(
        ken_runtime::D5bHs3CallMutation::ReappendFrameWorkerCaptures,
        || {
            ken_cli::build_native_program(
                SOURCE,
                ken_cli::SourceFormat::Ken,
                "abi_s6_d5b_duplicate_workers",
                root.path(),
            )
        },
    );
    assert_eq!(applications, 1, "the duplicate-worker mutation reaches D5b");
    assert!(
        observations.is_empty(),
        "the malformed Parameter run must refuse before emission"
    );
    let refusal = match result {
        Ok(_) => panic!("duplicating frame worker captures must refuse"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains("supplies 11 Parameter operands")
            && refusal.contains("declares 6"),
        "the mutation must reproduce the exact double-append cardinality failure: {refusal}"
    );
}

/// Promise class: durable invariant. Producer-local context captures are
/// sourced only from the exact-key creation frame. Withholding that run must
/// refuse rather than reading the specialization's entry ABI operands.
#[test]
fn complete_direct_worker_refuses_missing_frame_context_captures() {
    let root = tempfile::Builder::new()
        .prefix("ken-abi-s6-d5b-missing-contexts-")
        .tempdir()
        .expect("creates temporary root");
    let (result, observations, applications) = ken_runtime::with_d5b_hs3_call_mutation(
        ken_runtime::D5bHs3CallMutation::SuppressFrameContextCaptures,
        || {
            ken_cli::build_native_program(
                SOURCE,
                ken_cli::SourceFormat::Ken,
                "abi_s6_d5b_missing_contexts",
                root.path(),
            )
        },
    );
    assert_eq!(applications, 1, "the missing-context mutation reaches D5b");
    assert!(
        observations.is_empty(),
        "the incomplete Capture run must refuse before emission"
    );
    let refusal = match result {
        Ok(_) => panic!("withholding frame context captures must refuse"),
        Err(error) => format!("{error:?}"),
    };
    assert!(
        refusal.contains("supplies 0 context captures") && refusal.contains("projects 4"),
        "producer-local captures must not fall back to entry ABI operands: {refusal}"
    );
}
