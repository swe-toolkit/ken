//! PX7-F public checked-Ken linked-native discriminators.

fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px7f-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

fn run(name: &str, source: &str) -> ken_runtime::EffectObservation {
    let dir = output_dir(name);
    std::fs::write(dir.path().join("held.bin"), b"held resource").unwrap();
    let output = ken_cli::build_native_program(source, ken_cli::SourceFormat::Ken, name, dir.path())
        .expect("PX7-F checked program reaches the native resource lane");
    let oriented = output
        .runtime_program
        .erased_core
        .metadata
        .checked_core
        .metadata
        .values()
        .find(|bytes| bytes.starts_with(ken_runtime::ORIENTED_SUBCONTINUATION_PLAN_V1_HEADER))
        .and_then(|bytes| ken_runtime::OrientedSubcontinuationPlanV1::decode(bytes).ok())
        .expect("resource bracket carries its checked oriented plan");
    assert!(
        !oriented.frames.is_empty(),
        "the reaching resource bracket must retain checked answer interfaces"
    );
    let observation = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked PX7-F child emits its canonical observation");
    observation
}

const ESCAPE_CLOSED: &str = r#"program capabilities FS AFull
fn escape_body (resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit (Resource FsHandle)) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit (Resource FsHandle))
    (ResourceBodyOk Unit (Resource FsHandle) resource)

proc after_escape (bracket : ResourceBracketResult Unit (Resource FsHandle))
  : HostIO AFull ExitCode visits [FS] =
  match bracket {
    ResourceBracketOk resource |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result ResourceError FileMetadata) ExitCode
        (resourceMetadata AFull resource)
        (\used. match used {
          Err Closed |-> host_exit AFull Success;
          Err error |-> host_exit AFull (Failure 91);
          Ok metadata |-> host_exit AFull (Failure 92)
        });
    ResourceBracketBodyError error |-> host_exit AFull (Failure 93);
    ResourceBracketReleaseError error |-> host_exit AFull (Failure 94);
    ResourceBracketBodyAndReleaseError body_error release_error |->
      host_exit AFull (Failure 95)
  }

proc after_outer
  (outcome : Result FileError (ResourceBracketResult Unit (Resource FsHandle)))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err open_error |-> host_exit AFull (Failure 96);
    Ok bracket |-> after_escape bracket
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit (Resource FsHandle))) ExitCode
        (withResource AFull Unit (Resource FsHandle)
          cap (bytes_encode "held.bin") ResourceMetadata
          (\resource. Ret (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (ResourceBodyResult Unit (Resource FsHandle))
            (ResourceBodyOk Unit (Resource FsHandle) resource)))
        (\outcome. after_outer outcome)
  }
"#;

const RIGHT_NOT_HELD: &str = r#"program capabilities FS AFull
fn metadata_after (outcome : Result ResourceError FileMetadata)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit error);
    Ok metadata |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyOk ResourceError Unit MkUnit)
  }

proc metadata_body (resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError FileMetadata) (ResourceBodyResult ResourceError Unit)
    (resourceMetadata AFull resource) (\outcome. metadata_after outcome)

fn right_masks (error : ResourceError) : Bool =
  match error {
    ResourceHostIO io |-> False;
    Closed |-> False;
    MalformedResource |-> False;
    RightNotHeld required held |->
      match eq_int required 32 {
        True |-> eq_int held 1;
        False |-> False
      };
    ReleaseFailed kind identity io |-> False;
    ResourceKindMismatch expected actual |-> False;
    BufferLimit |-> False;
    AllocationFailed |-> False;
    InvalidOffset |-> False;
    InvalidBounds |-> False;
    NoProgress |-> False;
    MappingLimit |-> False
  }

fn bracket_has_right_denial (bracket : ResourceBracketResult ResourceError Unit) : Bool =
  match bracket {
    ResourceBracketOk unit |-> False;
    ResourceBracketBodyError error |-> right_masks error;
    ResourceBracketReleaseError error |-> False;
    ResourceBracketBodyAndReleaseError body_error release_error |-> False
  }

fn after_right_outer
  (outcome : Result FileError (ResourceBracketResult ResourceError Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err open_error |-> host_exit AFull (Failure 81);
    Ok bracket |-> match bracket_has_right_denial bracket {
      True |-> host_exit AFull Success;
      False |-> host_exit AFull (Failure 82)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult ResourceError Unit)) ExitCode
        (withResource AFull ResourceError Unit
          cap (bytes_encode "held.bin") ResourceRead metadata_body)
        (\outcome. after_right_outer outcome)
  }
"#;

const DOUBLE_RELEASE: &str = r#"program capabilities FS AFull
fn double_release_unexpected (error : ResourceError)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult ResourceError Unit)
    (ResourceBodyErr ResourceError Unit error)

fn double_release_second_error (error : ResourceError)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  match error {
    Closed |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyOk ResourceError Unit MkUnit);
    ResourceHostIO io |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit (ResourceHostIO io));
    MalformedResource |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit MalformedResource);
    RightNotHeld required held |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit (RightNotHeld required held));
    ReleaseFailed kind identity io |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit (ReleaseFailed kind identity io));
    ResourceKindMismatch expected actual |->
      double_release_unexpected (ResourceKindMismatch expected actual);
    BufferLimit |-> double_release_unexpected BufferLimit;
    AllocationFailed |-> double_release_unexpected AllocationFailed;
    InvalidOffset |-> double_release_unexpected InvalidOffset;
    InvalidBounds |-> double_release_unexpected InvalidBounds;
    NoProgress |-> double_release_unexpected NoProgress;
    MappingLimit |-> double_release_unexpected MappingLimit
  }

fn double_release_after_second (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  match outcome {
    Err error |-> double_release_second_error error;
    Ok unit |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit MalformedResource)
  }

proc double_release_after_first
  (resource : Resource FsHandle) (first : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult ResourceError Unit)
    (release AFull resource) (\second. double_release_after_second second)

proc double_release_body (resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult ResourceError Unit)
    (release AFull resource) (\first. double_release_after_first resource first)

fn double_release_done
  (outcome : Result FileError (ResourceBracketResult ResourceError Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Ok (ResourceBracketOk unit) |-> host_exit AFull Success;
    Ok bracket |-> host_exit AFull (Failure 74);
    Err error |-> host_exit AFull (Failure 75)
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult ResourceError Unit)) ExitCode
        (withResource AFull ResourceError Unit cap (bytes_encode "held.bin")
          ResourceMetadata double_release_body)
        (\outcome. double_release_done outcome)
  }
"#;

#[cfg(target_os = "linux")]
// Readmitted under RT-IGNORED-PASSING-ROWS, row 1 of 11.
//
// The prior label read: "RT-CARRIED-RESOURCE-SCALAR: the FsHandleMetadata seat
// cannot observe a carried word as a resource scalar; fails at base 21fd46dc",
// on the signature "Effect: seat Argument(0) of FsHandleMetadata needs
// ResourceScalar, which it cannot observe in CarriedWord", and asserted that the
// row "refuses at object emission, so the program never executes". None of that
// still holds: the program emits, executes, and the row passes un-ignored.
//
// The readmission does NOT rest on that green. Mutating the stale-generation arm
// of ken-host effect_v1.rs fn lookup (Err(Closed) -> Err(RightNotHeld{0,0})) reds
// the row with the perturbation visible AT the assertion -- event 2 outcome
// Error(Resource(RightNotHeld{0,0})) where Closed is asserted, exit_status 91
// rather than 0. Two earlier mutations of resolve_fs_handle's state-match arms
// left it green because lookup's generation check returns before that match is
// reached; a green under mutation is "vacuous row" OR "mutation did not reach",
// and only the third mutation separated them.
#[test]
fn linked_public_escape_is_exact_closed() {
    let observation = run("escape-closed", ESCAPE_CLOSED);
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::FsOpen,
            ken_runtime::HostOpV1::ResourceRelease,
            ken_runtime::HostOpV1::FsHandleMetadata,
        ]
    );
    assert!(matches!(
        observation.effect_trace[2].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::Closed
        ))
    ));
}

#[cfg(target_os = "linux")]
// Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
//
// Observed signature, exactly:
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it cannot observe in CarriedWord
//
// Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// The four px4b rows carry this same owner with the OPPOSITE provenance:
// those were branch-introduced, this one predates the branch.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-PLANNER-KRET-GRAFTED-SPINE: the planner derives k_ret_identity from a continuation the spec does not let the runtime invoke. This row refuses at the require_i64 on k_ret_identity in define_static_response_owner_bodies (at 1f0402ba4, around units.rs:3430 -- the SYMBOL is the citation, the line is a hint anchored to that SHA), which compares the planner expected Ret-case identity against the runtime carrier tag. exact_response_ret_identity reads ONE syntactic occurrence and composes nothing, while 42 s6.4 requires the grafted spine -- bind (Vis e f) k = Vis e (lambda r. bind (f r) k), per 36 s2.2 -- so after grafting the immediate syntactic continuation is not an object in the tree and the node only continuation is the composed one. Established by RT-PX7F-LINKED-PUBLIC-ROWS, which measured expected ctor:right-denial::ITree::Ret against carried ::ITree::Vis, then decoded that Vis operation to PrivateResourceRelease (FSOp ctor_543 -- the arena spells these operations ctor_NNN because the prelude strips the eleven Private FS-op spellings from globals once the checked public bracket is installed, per the private-name strip loop in ken-elaborator prelude and the prelude-record doc above it, leaving the eleven name-stripped ids as the last eleven FSOp constructors in declaration order; the artifact own FSOp roster returns that 10-named/11-unnamed partition at 29e6b21e1f8c5b80557428693a66eb61c19ea4c4) while this row own origin is PrivateFsHandleMetadata (542): the K departs onto the BRACKET OWN RELEASE, which composition predicts specifically and a selection defect gives no reason for. THIS ROW IS THE DISCRIMINATING WITNESS. Architect ruled fork (A) normative; the emission-side alternative is foreclosed because it would make a conformant runtime non-conformant."]
fn linked_public_right_denial_preserves_exact_masks() {
    let observation = run("right-denial", RIGHT_NOT_HELD);
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert!(observation.effect_trace.iter().any(|event| matches!(
        event.outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::RightNotHeld {
                required: 32,
                held: 1
            }
        ))
    )));
}

#[cfg(target_os = "linux")]
// Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
//
// Observed signature, exactly:
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it cannot observe in CarriedWord
//
// Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// The four px4b rows carry this same owner with the OPPOSITE provenance:
// those were branch-introduced, this one predates the branch.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-PLANNER-KRET-GRAFTED-SPINE: THIS ROW IS AN INSTANCE OF THE DEFECT. It fails the same check by the same mechanism as the right-denial row above -- the require_i64 on k_ret_identity in define_static_response_owner_bodies -- because the planner derives k_ret_identity from a continuation the spec does not let the runtime invoke. Established by RT-PX7F-LINKED-PUBLIC-ROWS. WHAT THIS ROW LACKS IS DISCRIMINATING POWER, NOT MEMBERSHIP: it is not evidence FOR that mechanism and must not be read as a second confirmation of it, because its K operation equals its OWN origin -- both PrivateResourceRelease, FSOp ctor_543 -- since in this program the composed continuation after the inner release also reaches the bracket release. Immediate and composed agree here by coincidence of constructor, which is VALUE EQUALITY AND NOT IDENTITY. The discrimination between the immediate and composed readings rests entirely on the right-denial row, whose origin is PrivateFsHandleMetadata (542) and whose K departs onto 543. Repairing this row means repairing that mechanism; measuring it does not test that mechanism."]
fn linked_public_second_release_is_closed_and_the_handle_closes_once() {
    let observation = run("double-release", DOUBLE_RELEASE);
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(observation.terminal_error, None);
    let releases = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .collect::<Vec<_>>();
    assert_eq!(
        releases.len(),
        3,
        "two public calls plus bracket settlement"
    );
    assert!(matches!(
        releases[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(
            ken_runtime::CanonicalReplyV1::ResourceSettlement(_)
        )
    ));
    assert!(matches!(
        releases[1].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::Closed
        ))
    ));
    assert_eq!(
        releases
            .iter()
            .filter(|event| matches!(
                event.outcome,
                ken_runtime::CanonicalOutcomeV1::Success(
                    ken_runtime::CanonicalReplyV1::ResourceSettlement(_)
                )
            ))
            .count(),
        1,
        "the owned descriptor is actually closed exactly once"
    );
}
