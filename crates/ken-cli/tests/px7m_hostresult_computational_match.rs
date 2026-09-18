fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px7m-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const OK_PROGRAM: &str = r#"program capabilities FS APartial
proc two_step (label : String) : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit Unit
    (host_console APartial Unit (print_line label))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit))

proc after_write (written : Result IOError Unit)
  : HostIO APartial Unit visits [Console] =
  match written {
    Err _ |-> two_step "unexpected-error" ;
    Ok unit |-> match unit { MkUnit |-> two_step "ok-payload" }
  }

proc inner : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "probe:")))
    after_write

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit ExitCode inner (\_. host_exit APartial Success)
"#;

const ERR_PROGRAM: &str = r#"program capabilities FS APartial
proc write_bytes_then_line (bytes : Bytes) (label : String)
  : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit) (write Stdout bytes))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      Unit Unit
      (host_console APartial Unit (print_line label))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit))

fn failed_path (error : FileError) : Bytes =
  match error {
    MkFileError _operation path _kind |-> match path {
      None |-> bytes_encode "no-path" ;
      Some bytes |-> bytes
    }
  }

proc after_read (read : Result FileError Bytes)
  : HostIO APartial Unit visits [Console] =
  match read {
    Err error |-> write_bytes_then_line (failed_path error) "not-found" ;
    Ok bytes |-> write_bytes_then_line bytes "unexpected-ok"
  }

proc inner (cap : Cap APartial) : HostIO APartial Unit visits [FS, Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result FileError Bytes) Unit
    (inject_l (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp
      (Result FileError Bytes)
      (readFile APartial cap (bytes_encode "missing.bin")))
    after_read

proc main (_input : ProcessInput) (caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS, Console] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode (inner cap) (\_. host_exit APartial Success)
  }
"#;

fn assert_agreement(
    source: &str,
    name: &str,
    expected_stdout: &[u8],
    expected_operations: &[ken_runtime::HostOpV1],
) {
    let dir = output_dir(name);
    let output = ken_cli::build_native_program(source, ken_cli::SourceFormat::Ken, name, dir.path())
        .expect("dynamic HostResult producer reaches the linked artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked artifact returns its complete observation");

    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("same checked source runs through the interpreter");
    assert_eq!(native, interpreted);
    assert_eq!(native.exit_status, 0);
    assert_eq!(native.stdout, expected_stdout);
    assert_eq!(
        native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        expected_operations
    );
}

// Owner node: RT-CARRIED-RESIDUAL-IH-ARITY.
//
// Observed signature, exactly, re-measured 2026-09-17 at origin/main
// 3f4ae2d83 (NOT carried from the ledger):
//   unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
//   hypothesis is an eliminated value, not a callable, so it takes no
//   arguments, but the call provides 1
//
// SUPERSEDED OWNER, recorded because it is what this row was filed under and
// the old text asserted a signature this row no longer produces:
// RT-CARRIER-BYTESPAN-OBSERVE, whose signature was
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it
//   cannot observe in CarriedWord
// That refusal no longer reaches this row -- RT-SITEOP-CARRIED-WITNESS D2
// landed the carried SiteOperand port and the labels record it succeeding.
// The four px4b rows still carry RT-CARRIER-BYTESPAN-OBSERVE, with the
// OPPOSITE provenance: those were branch-introduced, this one predates the
// branch.
//
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-CONTEXT-FRAME-LABEL-CORRECTION, measured at b0421afd0; supersedes the RT-CARRIED-RESIDUAL-IH-ARITY label measured at 3f4ae2d83, two of whose clauses are refuted below. STILL TRUE, re-measured here: the BoundaryCarrier arity text is a FALLBACK SYMPTOM, not the mechanism, and the arity refusal itself is correct and must not be relaxed. The admission query at core.rs:13578-80 applies THREE tests, and its two cardinality equalities BOTH hold here: the frame's 4 worker captures equal the queried capture count, and its 3 context captures equal the queried claim count. Those two runs are different populations by design and are never compared to each other; each is compared to the query. The third test is what fails -- the frame is keyed to a sibling body (stored worker_body_origin 332), so the sole admission query, for body 369, falls through. REFUTED, both measured FALSE at b0421afd0: (a) the prior claim that this is a single Option slot overwritten per construction, so the slot holds only the last -- constructed_context_frame is written EXACTLY ONCE in this compile (continuation_origin 11, recursive_position 1, worker_body_origin 332), never overwritten, so no frame is lost to a second write; (b) the prior claim that it readmits when the slot is keyed by worker_body_origin -- keying it readmits NOTHING. The admission axis is red at BOTH ends: admit fewer, which is today's setting, yields Ok(None) and control falls to the zero-argument route that reports the arity; admit more, with the frame arm forced to return Ok(true) unconditionally and so strictly more permissive than any key, still fails at agreeing_recursive_body_unit (core.rs:1230) with: plain Match branches declare different recursive body units: 369 versus 332. The resolved body origins come from the closure structure rather than from the frame, so no setting of this gate passes this row. That refusal is deliberate and carries its own two-direction unit test; it is not a fallthrough. READMISSION CONDITION MEASURED at 7cb535be5, with crates/ byte-identical through 9dfa6978e, and core.rs:1230 is NOT the last layer. Forced past it in BOTH directions -- taking the first declared unit and taking the last -- this row reaches exactly one further refusal, BYTE-IDENTICAL between the two directions: ContinuationSpecialization at core.rs:9558, a generated context capture carries no context-capture availability claim, so nothing says where this frame holds ProducerLocal { ... }; RT-CONTSRC-PRODUCER-LOCAL D3b refuses rather than reading the direct-emission claim. On this row its operands are owner PredeclaredFunctionId(5), binding origin 395, environment origin 391. THREE distinct stops were seen under TWO stacked forcings -- the BoundaryCarrier arity report unforced, 1230 under forced admission, D3b under both -- and THE STACK BEHIND D3b WAS DELIBERATELY NOT FORCED AND IS NOT BOUNDED; do not read this label as naming the last blocker. The two-direction agreement is NOT VACUOUS: the chosen unit is consumed and changes control flow, the disagreement being reached 1 time(s) taking the first unit and 2 taking the last. WHAT THE TWO ORIGINS DENOTE, measured by REFERENT and not by ordinal -- PlannedOccurrence holds a reference into the source IR, so the bodies themselves are comparable: the bodies at StaticOriginId(369) and StaticOriginId(332) sit at distinct addresses and are BYTE-IDENTICAL as rendered RuntimeExpr, 558 bytes, so this row's arms declare EQUAL bodies and the refusal is testing the wrong relation. REFUTED by that same measurement, in all four rows of this node: the reading that resolution produces two origins for ONE body. The two origins name two genuinely distinct nodes at distinct addresses; there is no aliasing anywhere and resolution is faithful. THE MECHANISM IS THAT THE COMPARISON TESTS NODE IDENTITY WHERE THE PROPERTY IT NEEDS IS BODY EQUALITY. NO REPAIR AT 1230 PASSES THIS ROW: D3b sits immediately behind it on all four rows, so closing 1230 readmits three of the four past 1230 and changes nothing observable about any of them. INHERITED from 3f4ae2d83 and NOT re-measured here: RT-SITEOP-CARRIED-WITNESS D2 still succeeds; the recursive-position argument is a LexicalClosure with a callable body; the fallthrough refuses on the D3 equality claims.len() == captures, which compares the two populations the D2 route documents as different."]
fn dynamic_ok_payload_selects_a_multistep_tree_across_real_executors() {
    assert_agreement(
        OK_PROGRAM,
        "px7m-ok",
        b"probe:ok-payload\n",
        &[
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleFlush,
        ],
    );
}

// Owner node: RT-CARRIED-RESIDUAL-IH-ARITY.
//
// Observed signature, exactly, re-measured 2026-09-17 at origin/main
// 3f4ae2d83 (NOT carried from the ledger):
//   unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
//   hypothesis is an eliminated value, not a callable, so it takes no
//   arguments, but the call provides 1
//
// SUPERSEDED OWNER, recorded because it is what this row was filed under and
// the old text asserted a signature this row no longer produces:
// RT-CARRIER-BYTESPAN-OBSERVE, whose signature was
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it
//   cannot observe in CarriedWord
// That refusal no longer reaches this row -- RT-SITEOP-CARRIED-WITNESS D2
// landed the carried SiteOperand port and the labels record it succeeding.
// The four px4b rows still carry RT-CARRIER-BYTESPAN-OBSERVE, with the
// OPPOSITE provenance: those were branch-introduced, this one predates the
// branch.
//
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-CONTEXT-FRAME-LABEL-CORRECTION, measured at b0421afd0; supersedes the RT-CARRIED-RESIDUAL-IH-ARITY label measured at 3f4ae2d83, two of whose clauses are refuted below. STILL TRUE, re-measured here: the BoundaryCarrier arity text is a FALLBACK SYMPTOM, not the mechanism, and the arity refusal itself is correct and must not be relaxed. The admission query at core.rs:13578-80 applies THREE tests, and its two cardinality equalities BOTH hold here: the frame's 5 worker captures equal the queried capture count, and its 4 context captures equal the queried claim count. Those two runs are different populations by design and are never compared to each other; each is compared to the query. The third test is what fails -- the frame is keyed to a sibling body (stored worker_body_origin 341), so the sole admission query, for body 378, falls through. REFUTED, both measured FALSE at b0421afd0: (a) the prior claim that this is a single Option slot overwritten per construction, so the slot holds only the last -- constructed_context_frame is written EXACTLY ONCE in this compile (continuation_origin 12, recursive_position 1, worker_body_origin 341), never overwritten, so no frame is lost to a second write; (b) the prior claim that it readmits when the slot is keyed by worker_body_origin -- keying it readmits NOTHING. The admission axis is red at BOTH ends: admit fewer, which is today's setting, yields Ok(None) and control falls to the zero-argument route that reports the arity; admit more, with the frame arm forced to return Ok(true) unconditionally and so strictly more permissive than any key, still fails at agreeing_recursive_body_unit (core.rs:1230) with: plain Match branches declare different recursive body units: 378 versus 341. The resolved body origins come from the closure structure rather than from the frame, so no setting of this gate passes this row. That refusal is deliberate and carries its own two-direction unit test; it is not a fallthrough. READMISSION CONDITION MEASURED at 7cb535be5, with crates/ byte-identical through 9dfa6978e, and core.rs:1230 is NOT the last layer. Forced past it in BOTH directions -- taking the first declared unit and taking the last -- this row reaches exactly one further refusal, BYTE-IDENTICAL between the two directions: ContinuationSpecialization at core.rs:9558, a generated context capture carries no context-capture availability claim, so nothing says where this frame holds ProducerLocal { ... }; RT-CONTSRC-PRODUCER-LOCAL D3b refuses rather than reading the direct-emission claim. On this row its operands are owner PredeclaredFunctionId(5), binding origin 409, environment origin 405. THREE distinct stops were seen under TWO stacked forcings -- the BoundaryCarrier arity report unforced, 1230 under forced admission, D3b under both -- and THE STACK BEHIND D3b WAS DELIBERATELY NOT FORCED AND IS NOT BOUNDED; do not read this label as naming the last blocker. The two-direction agreement is NOT VACUOUS: the chosen unit is consumed and changes control flow, the disagreement being reached 1 time(s) taking the first unit and 2 taking the last. WHAT THE TWO ORIGINS DENOTE, measured by REFERENT and not by ordinal -- PlannedOccurrence holds a reference into the source IR, so the bodies themselves are comparable: the bodies at StaticOriginId(378) and StaticOriginId(341) sit at distinct addresses and DIFFER, 1164 versus 1168 bytes, in exactly one leaf -- Value(String(\"not-found\")) versus Value(String(\"unexpected-ok\")) -- so THIS ROW's arms genuinely declare DIFFERENT bodies and the refusal is CORRECT here. This row is the 3-1 split of this node's population, and the split is INSIDE px7m rather than between px7m and px7l. REFUTED by that same measurement, in all four rows of this node: the reading that resolution produces two origins for ONE body. The two origins name two genuinely distinct nodes at distinct addresses; there is no aliasing anywhere and resolution is faithful. THE MECHANISM IS THAT THE COMPARISON TESTS NODE IDENTITY WHERE THE PROPERTY IT NEEDS IS BODY EQUALITY. NO REPAIR AT 1230 PASSES THIS ROW: D3b sits immediately behind it on all four rows, so closing 1230 readmits three of the four past 1230 and changes nothing observable about any of them. INHERITED from 3f4ae2d83 and NOT re-measured here: RT-SITEOP-CARRIED-WITNESS D2 still succeeds; the recursive-position argument is a LexicalClosure with a callable body; the fallthrough refuses on the D3 equality claims.len() == captures, which compares the two populations the D2 route documents as different."]
fn dynamic_err_payload_selects_a_multistep_tree_across_real_executors() {
    assert_agreement(
        ERR_PROGRAM,
        "px7m-err",
        b"missing.binnot-found\n",
        &[
            ken_runtime::HostOpV1::FsReadFile,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
        ],
    );
}
