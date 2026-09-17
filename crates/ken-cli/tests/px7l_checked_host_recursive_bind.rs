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
#[ignore = "RT-CARRIED-RESIDUAL-IH-ARITY, measured at 3f4ae2d83: RT-SITEOP-CARRIED-WITNESS D2 still succeeds and the BoundaryCarrier arity text is a FALLBACK SYMPTOM, not the mechanism. The recursive-position argument IS a LexicalClosure with a callable body; resolve_recursive_unit_body declines the callable route, and control then falls to the zero-argument route, which reports the arity. The decline is NOT a capture shortfall: function_local.constructed_context_frame is a single Option slot overwritten per construction, so when one function constructs frames for two worker bodies the slot holds only the last, and the admission query for the other body sees frame.worker_body_origin != body_origin and falls through. MEASURED HERE: the live frame matches on both cardinalities (3 worker captures, 3 context captures) but is keyed to a sibling body, so the fallthrough then refuses on claim 0 having no availability.context_capture. Readmits when that slot is keyed by worker_body_origin rather than holding one frame per function. The arity refusal itself is correct and must not be relaxed."]
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
#[ignore = "RT-CARRIED-RESIDUAL-IH-ARITY, measured at 3f4ae2d83: RT-SITEOP-CARRIED-WITNESS D2 still succeeds and the BoundaryCarrier arity text is a FALLBACK SYMPTOM, not the mechanism. The recursive-position argument IS a LexicalClosure with a callable body; resolve_recursive_unit_body declines the callable route, and control then falls to the zero-argument route, which reports the arity. The decline is NOT a capture shortfall: function_local.constructed_context_frame is a single Option slot overwritten per construction, so when one function constructs frames for two worker bodies the slot holds only the last, and the admission query for the other body sees frame.worker_body_origin != body_origin and falls through. MEASURED HERE: the live frame matches on both cardinalities (3 worker captures, 3 context captures) but is keyed to a sibling body, so the fallthrough then refuses on claim 0 having no availability.context_capture. Readmits when that slot is keyed by worker_body_origin rather than holding one frame per function. The arity refusal itself is correct and must not be relaxed."]
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
