fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px7l-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const PROGRAM: &str = r#"program capabilities FS APartial
proc selected_body (terminal : Bool) (message : String)
  : Unit -> HostIO APartial Unit visits [Console] =
  \_. match terminal {
    False |-> host_console APartial Unit (print_line message) ;
    True |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit)
  }

proc delayed (body : Unit -> HostIO APartial Unit)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit ExitCode
    (body MkUnit)
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) ExitCode
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. host_exit APartial Success))

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Bool ExitCode
    (host_console APartial Bool (is_terminal Stdout))
    (\terminal. delayed (selected_body terminal "captured"))
"#;

const STATIC_DIRECT_VIS: &str = r#"program capabilities FS APartial
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "static")
"#;

const CONSUMED_RUNTIME_RESPONSE: &str = r#"program capabilities FS APartial
proc selected_result (terminal : Bool) (message : String)
  : Unit -> HostIO APartial (Result IOError Unit) visits [Console] =
  \_. match terminal {
    False |-> host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode message)) ;
    True |-> host_console APartial (Result IOError Unit) (flush Stdout)
  }

proc delayed_result (body : Unit -> HostIO APartial (Result IOError Unit))
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) ExitCode
    (body MkUnit)
    (\written. match written {
      Err _ |-> host_program APartial (print_line "write-error") ;
      Ok _ |-> host_program APartial (print_line "write-ok")
    })

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Bool ExitCode
    (host_console APartial Bool (is_terminal Stdout))
    (\terminal. delayed_result (selected_result terminal "probe"))
"#;

fn contains_recursive_bind_ir(expr: &ken_runtime::RuntimeExpr) -> bool {
    use ken_runtime::RuntimeExpr;
    match expr {
        RuntimeExpr::ComputationalMatch { .. } | RuntimeExpr::LexicalClosure { .. } => true,
        RuntimeExpr::Let { value, body } => {
            contains_recursive_bind_ir(value) || contains_recursive_bind_ir(body)
        }
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            contains_recursive_bind_ir(scrutinee)
                || contains_recursive_bind_ir(then_expr)
                || contains_recursive_bind_ir(else_expr)
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            args.iter().any(contains_recursive_bind_ir)
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            contains_recursive_bind_ir(scrutinee)
                || cases
                    .iter()
                    .any(|case| contains_recursive_bind_ir(&case.body))
        }
        RuntimeExpr::Record { fields } => fields
            .iter()
            .any(|(_, value)| contains_recursive_bind_ir(value)),
        RuntimeExpr::Project { record, .. } => contains_recursive_bind_ir(record),
        RuntimeExpr::Closure { body, .. } => contains_recursive_bind_ir(body),
        RuntimeExpr::Call { callee, args } => {
            contains_recursive_bind_ir(callee) || args.iter().any(contains_recursive_bind_ir)
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            capability
                .as_ref()
                .is_some_and(|capability| contains_recursive_bind_ir(&capability.value))
                || args.iter().any(contains_recursive_bind_ir)
        }
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => {
            contains_recursive_bind_ir(body)
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Var(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => false,
    }
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
#[ignore = "RT-CONTEXT-FRAME-LABEL-CORRECTION, measured at b0421afd0; supersedes the RT-CARRIED-RESIDUAL-IH-ARITY label measured at 3f4ae2d83, two of whose clauses are refuted below. STILL TRUE, re-measured here: the BoundaryCarrier arity text is a FALLBACK SYMPTOM, not the mechanism, and the arity refusal itself is correct and must not be relaxed. The admission query at core.rs:13578-80 applies THREE tests, and its two cardinality equalities BOTH hold here: the frame's 3 worker captures equal the queried capture count, and its 3 context captures equal the queried claim count. Those two runs are different populations by design and are never compared to each other; each is compared to the query. The third test is what fails -- the frame is keyed to a sibling body (stored worker_body_origin 343), so the admission query for the sibling body 322 falls through while the query for 343 is admitted. REFUTED, both measured FALSE at b0421afd0: (a) the prior claim that this is a single Option slot overwritten per construction, so the slot holds only the last -- constructed_context_frame is written EXACTLY ONCE in this compile (continuation_origin 11, recursive_position 1, worker_body_origin 343), never overwritten, so no frame is lost to a second write; (b) the prior claim that it readmits when the slot is keyed by worker_body_origin -- keying it readmits NOTHING. The admission axis is red at BOTH ends: admit fewer, which is today's setting, yields Ok(None) and control falls to the zero-argument route that reports the arity; admit more, with the frame arm forced to return Ok(true) unconditionally and so strictly more permissive than any key, still fails at agreeing_recursive_body_unit (core.rs:1230) with: plain Match branches declare different recursive body units: 343 versus 322. The resolved body origins come from the closure structure rather than from the frame, so no setting of this gate passes this row. That refusal is deliberate and carries its own two-direction unit test; it is not a fallthrough. READMISSION CONDITION UNKNOWN: the stack behind this gate was measured only in the forced configuration and only to its first stop, so whether core.rs:1230 is the last layer or the next in a queue is NOT established -- do not read either into this label. INHERITED from 3f4ae2d83 and NOT re-measured here: RT-SITEOP-CARRIED-WITNESS D2 still succeeds; the recursive-position argument is a LexicalClosure with a callable body; the fallthrough refuses on claim 0 having no availability.context_capture."]
fn delayed_capturing_generic_bind_agrees_across_real_executors() {
    let dir = output_dir("agreement");
    let output = ken_cli::build_native_program(
        PROGRAM,
        ken_cli::SourceFormat::Ken,
        "px7l-recursive-bind",
        dir.path(),
    )
    .expect("generic checked HostIO bind reaches the linked artifact");
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
        PROGRAM,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("same checked source runs through interpreter");
    assert_eq!(native, interpreted);
    assert_eq!(native.exit_status, 0);
    assert_eq!(native.stdout, b"captured\n");
    assert_eq!(native.effect_trace.len(), 3);
    assert_eq!(
        native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::ConsoleIsTerminal,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleFlush,
        ]
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
#[ignore = "RT-CONTEXT-FRAME-LABEL-CORRECTION, measured at b0421afd0; supersedes the RT-CARRIED-RESIDUAL-IH-ARITY label measured at 3f4ae2d83, two of whose clauses are refuted below. STILL TRUE, re-measured here: the BoundaryCarrier arity text is a FALLBACK SYMPTOM, not the mechanism, and the arity refusal itself is correct and must not be relaxed. The admission query at core.rs:13578-80 applies THREE tests, and its two cardinality equalities BOTH hold here: the frame's 3 worker captures equal the queried capture count, and its 3 context captures equal the queried claim count. Those two runs are different populations by design and are never compared to each other; each is compared to the query. The third test is what fails -- the frame is keyed to a sibling body (stored worker_body_origin 362), so the admission query for the sibling body 347 falls through while the query for 362 is admitted. REFUTED, both measured FALSE at b0421afd0: (a) the prior claim that this is a single Option slot overwritten per construction, so the slot holds only the last -- constructed_context_frame is written EXACTLY ONCE in this compile (continuation_origin 11, recursive_position 1, worker_body_origin 362), never overwritten, so no frame is lost to a second write; (b) the prior claim that it readmits when the slot is keyed by worker_body_origin -- keying it readmits NOTHING. The admission axis is red at BOTH ends: admit fewer, which is today's setting, yields Ok(None) and control falls to the zero-argument route that reports the arity; admit more, with the frame arm forced to return Ok(true) unconditionally and so strictly more permissive than any key, still fails at agreeing_recursive_body_unit (core.rs:1230) with: plain Match branches declare different recursive body units: 362 versus 347. The resolved body origins come from the closure structure rather than from the frame, so no setting of this gate passes this row. That refusal is deliberate and carries its own two-direction unit test; it is not a fallthrough. READMISSION CONDITION UNKNOWN: the stack behind this gate was measured only in the forced configuration and only to its first stop, so whether core.rs:1230 is the last layer or the next in a queue is NOT established -- do not read either into this label. INHERITED from 3f4ae2d83 and NOT re-measured here: RT-SITEOP-CARRIED-WITNESS D2 still succeeds; the recursive-position argument is a LexicalClosure with a callable body; the fallthrough refuses on claim 0 having no availability.context_capture."]
fn runtime_selected_non_unit_response_is_consumed_across_real_executors() {
    let dir = output_dir("consumed-response");
    let output = ken_cli::build_native_program(
        CONSUMED_RUNTIME_RESPONSE,
        ken_cli::SourceFormat::Ken,
        "px7l-consumed-runtime-response",
        dir.path(),
    )
    .expect("runtime-selected Result response reaches the linked artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked artifact consumes its runtime-selected response");

    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        CONSUMED_RUNTIME_RESPONSE,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("interpreter consumes the same runtime-selected response");
    assert_eq!(native, interpreted);
    assert_eq!(native.exit_status, 0);
    assert_eq!(native.stdout, b"probewrite-ok\n");
    assert_eq!(
        native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::ConsoleIsTerminal,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
        ]
    );
}

#[test]
fn static_direct_vis_retains_the_existing_lowering_path() {
    let dir = output_dir("static-direct-vis");
    let output = ken_cli::build_native_program(
        STATIC_DIRECT_VIS,
        ken_cli::SourceFormat::Ken,
        "px7l-static-direct-vis",
        dir.path(),
    )
    .expect("static direct Vis remains supported");
    let main = output
        .runtime_program
        .declarations
        .iter()
        .find(|declaration| declaration.symbol == output.plan.main().to_string())
        .expect("runtime program contains checked main");
    let ken_runtime::RuntimeDeclarationKind::Transparent { body } = &main.kind else {
        panic!("checked main remains transparent")
    };
    assert!(
        !contains_recursive_bind_ir(body),
        "static direct Vis must not be rerouted through PX7-L dynamic machinery"
    );
    let ran = std::process::Command::new(&output.artifact.executable_path)
        .output()
        .expect("static artifact runs");
    assert_eq!(ran.status.code(), Some(0));
    assert_eq!(ran.stdout, b"static\n");
}
