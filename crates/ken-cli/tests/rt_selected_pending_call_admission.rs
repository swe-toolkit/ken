// Checked-source admission and native artifact emission. The planner rows
// below are not a proxy for execution or for consumption at the D2 gate.

const PX7L: &str = r#"program capabilities FS APartial
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

#[test]
fn selected_pending_call_admits_validated_owner_without_a_package() {
    let dir = tempfile::tempdir().unwrap();
    let (outcome, rows) = ken_runtime::with_selected_pending_call_admissions(|| {
        ken_cli::build_native_program(
            PX7L,
            ken_cli::SourceFormat::Ken,
            "rt-pending-admission",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    outcome.expect("the validated owner must emit a native artifact");
    let admitted: Vec<_> = rows.iter().filter_map(|row| match &row.outcome {
        ken_runtime::SelectedPendingCallOutcomeObservation::ValidatedResponseOwner {
            candidates, defining_function, visited, traversed_families,
            gates, gate_binder_pairs,
        } => Some((candidates, defining_function, visited,
                   traversed_families, gates, gate_binder_pairs)),
        _ => None,
    }).collect();
    assert_eq!(admitted.len(), 1, "one differing-unit Match is owner-validated: {rows:#?}");
    let (candidates, owner, visited, traversed_families, gates, gate_binder_pairs) = admitted[0];
    assert_eq!(*owner, 3);
    assert_eq!(candidates.iter().map(|candidate| (candidate.arm, candidate.body))
        .collect::<Vec<_>>(), vec![(0, 343), (1, 322)]);
    assert!(candidates.iter().all(|candidate| matches!(candidate.callee,
        ken_runtime::SelectedPendingCalleeObservation::StaticResponseOwner(_))),
        "every admitted leaf's direct callee must be validated by its owner");
    for expected in [(311, "F1"), (96, "F1"), (55, "F1"), (49, "F3"), (47, "F4")] {
        assert!(visited.contains(&expected), "missing route visit {expected:?}: {visited:?}");
    }
    let derived = visited.iter()
        .filter_map(|(_, kind)| (*kind != "Local").then_some(*kind))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter().collect::<Vec<_>>();
    assert_eq!(traversed_families, &derived);
    assert!(!gates.is_empty(), "the checked call gate must remain reachable");
    assert!(gate_binder_pairs.contains(&(47, 4, 4)),
        "checked template identity, not raw index equality, authorizes this gate");
    for &(origin, ..) in gate_binder_pairs {
        assert!(visited.contains(&(origin, "F4")));
    }
}

// P2: mutating only the owner-fed emission's answer back to ordinary must
// expose its undisposed Vis-subtree join. This is an object-emission refusal,
// not a native result; the positive artifact is built in the test above.
#[test]
fn owner_scoped_false_ordinary_answer_restores_join_refusal() {
    let dir = tempfile::tempdir().unwrap();
    let (outcome, applied) = ken_runtime::with_owner_fed_join_forced_ordinary(|| {
        ken_cli::build_native_program(
            PX7L, ken_cli::SourceFormat::Ken, "rt-pending-false-ordinary",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    assert!(applied > 0, "the owner-fed emission must be reached");
    let error = outcome.expect_err("undisposed owner Vis join must refuse object emission");
    assert!(format!("{error:?}").contains(
        "function left planned source join StaticOriginId(19) neither emitted nor statically unselected"
    ), "the precise Vis-subtree join must refuse: {error:?}");
}

// P3: a test-only Vis-subtree ledger consumption in the owner function must
// be rejected as an already-emitted non-Ret case, before S2 can hide it.
// The normal emission above supplies the positive, unmutated control.
#[test]
fn owner_vis_join_consumption_refuses_before_dead_subtree_disposition() {
    let dir = tempfile::tempdir().unwrap();
    let (outcome, applied) = ken_runtime::with_owner_vis_join_consumed(|| {
        ken_cli::build_native_program(
            PX7L, ken_cli::SourceFormat::Ken, "rt-pending-vis-consumed",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    assert_eq!(applied, 1, "the owner-fed non-Ret subtree must supply one Vis join");
    let error = outcome.expect_err("a consumed dead Vis join must refuse object emission");
    assert!(format!("{error:?}").contains("an owner-fed Match emitted a non-Ret case body"),
        "the emitted-case check, not a later join error, must refuse: {error:?}");
}

/// Refused upstream by the response planner at 6bdd75394; not an admission
/// witness. This checked double-bind source cannot reach the linearity guard:
/// the response planner rejects its duplicate host response case first. No
/// native artifact is executed in this test. The planner-level unit fixture in
/// ken-runtime observes the shadowed admission guard separately.
#[test]
fn checked_double_bind_is_refused_upstream_not_admitted() {
    const ORIGINAL: &str = "  bind (Coproduct (FSOp APartial) AmbientOp)\n    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)\n    Unit ExitCode\n    (body MkUnit)\n    (\\_. bind (Coproduct (FSOp APartial) AmbientOp)\n      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)\n      (Result IOError Unit) ExitCode\n      (host_console APartial (Result IOError Unit) (flush Stdout))\n      (\\_. host_exit APartial Success))";
    const DOUBLE_BIND: &str = "  let p : HostIO APartial Unit = body MkUnit in\n  bind (Coproduct (FSOp APartial) AmbientOp)\n    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)\n    Unit ExitCode\n    p\n    (\\_. bind (Coproduct (FSOp APartial) AmbientOp)\n      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)\n      Unit ExitCode\n      p\n      (\\_. host_exit APartial Success))";
    assert_eq!(PX7L.matches(ORIGINAL).count(), 1);
    let source = PX7L.replacen(ORIGINAL, DOUBLE_BIND, 1);
    let dir = tempfile::tempdir().unwrap();
    let (outcome, rows) = ken_runtime::with_selected_pending_call_admissions(|| {
        ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            "rt-pending-duplicate",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    assert!(
        rows.is_empty(),
        "response planning must refuse before admission: {rows:#?}"
    );
    let error = outcome.expect_err("double bind cannot produce a native artifact");
    assert!(
        format!("{error:?}").contains(
            "native static transition planner invariant failed; please report this compiler bug: two host response cases claim one operation constructor"
        ),
        "the direct double bind must remain an upstream refusal, not a surrogate admission witness: {error:?}"
    );
}

// F1 and its controls change only the selected_body of the landed PX7L
// source. This is a fixture constructor, not an admission/execution oracle.
fn with_selected_body(body: &str) -> String {
    let start = PX7L.find("  \\_. match terminal {").unwrap();
    let end = PX7L.find("\n\nproc delayed").unwrap();
    assert!(start < end);
    format!("{}{}{}", &PX7L[..start], body, &PX7L[end..])
}

const F1_BODY: &str = r#"  \_. match terminal {
    False |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. host_console APartial Unit (print_line "after-flush")) ;
    True |-> host_console APartial Unit (print_line message)
  }"#;

fn assert_native_interpreted_route(
    source: &str, name: &str, stdout: &[u8], ops: &[ken_runtime::HostOpV1],
) {
    let dir = tempfile::tempdir().unwrap();
    let output = ken_cli::build_native_program(
        source, ken_cli::SourceFormat::Ken, name, dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    ).expect("a selected route must emit an artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(), environment: Vec::new(),
            cwd: dir.path().to_owned(), plan_hash: output.plan_transport_hash,
        },
    ).expect("native selected route must return a complete observation");
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        source, ken_cli::SourceFormat::Ken, &[b"ken".to_vec()], &[], b"/", &mut host,
    ).expect("the same checked source must run through the interpreter");
    assert_eq!(native, interpreted, "native cannot drop a selected continuation");
    assert_eq!(native.exit_status, 0);
    assert_eq!(native.stdout, stdout);
    assert_eq!(native.effect_trace.iter().map(|event| event.operation)
        .collect::<Vec<_>>(), ops);
}

// Durable parity invariant: a shared handler Effect row belonging to another
// Vis cannot let the reached Write case resume without dispatch. Under the
// old row-only placeholder this source exited zero with empty native stdout.
#[test]
fn selected_pending_write_arm_runs_its_bind_continuation_natively() {
    assert_native_interpreted_route(
        &with_selected_body(F1_BODY), "rt-pending-selected-write",
        b"after-flush\n",
        &[ken_runtime::HostOpV1::ConsoleIsTerminal,
          ken_runtime::HostOpV1::ConsoleFlush,
          ken_runtime::HostOpV1::ConsoleWrite,
          ken_runtime::HostOpV1::ConsoleFlush],
    );
}

// Positive controls: the same leaf without a runtime Match or in the
// non-selected arm follows the already-lawful ordinary execution path.
#[test]
fn selected_pending_write_arm_controls_agree_across_executors() {
    const NO_MATCH: &str = r#"  \_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. host_console APartial Unit (print_line "after-flush"))"#;
    const UNSELECTED: &str = r#"  \_. match terminal {
    False |-> host_console APartial Unit (print_line message) ;
    True |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. host_console APartial Unit (print_line "after-flush"))
  }"#;
    assert_native_interpreted_route(
        &with_selected_body(NO_MATCH), "rt-pending-no-match", b"after-flush\n",
        &[ken_runtime::HostOpV1::ConsoleIsTerminal,
          ken_runtime::HostOpV1::ConsoleFlush,
          ken_runtime::HostOpV1::ConsoleWrite,
          ken_runtime::HostOpV1::ConsoleFlush],
    );
    assert_native_interpreted_route(
        &with_selected_body(UNSELECTED), "rt-pending-unselected", b"captured\n",
        &[ken_runtime::HostOpV1::ConsoleIsTerminal,
          ken_runtime::HostOpV1::ConsoleWrite,
          ken_runtime::HostOpV1::ConsoleFlush],
    );
}

// Transition sentinel: two prints in the selected arm still lack a binding
// for deferred relocated work. Refusal must be classified to the user; a
// planner invariant is a backend defect, not the admitted-route boundary.
#[test]
fn selected_pending_two_print_route_surfaces_its_admission_refusal() {
    const OLD_ARM: &str = "False |-> host_console APartial Unit (print_line message)";
    const TWO_PRINTS: &str = r#"False |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      Unit Unit
      (host_console APartial Unit (print_line message))
      (\_. host_console APartial Unit (print_line "second"))"#;
    assert_eq!(PX7L.matches(OLD_ARM).count(), 1);
    let source = PX7L.replacen(OLD_ARM, TWO_PRINTS, 1);
    let dir = tempfile::tempdir().unwrap();
    let (outcome, rows) = ken_runtime::with_selected_pending_call_admissions(|| {
        ken_cli::build_native_program(
            &source, ken_cli::SourceFormat::Ken, "rt-pending-two-print",
            dir.path(), ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    let refused = rows.iter().filter_map(|row| match row.outcome {
        ken_runtime::SelectedPendingCallOutcomeObservation::Refused(reason) => Some(reason),
        _ => None,
    }).collect::<Vec<_>>();
    assert_eq!(refused, [ken_runtime::PendingRefusal::RelocatedWorkMissingLoweringBinding],
        "the source must reach this exact admission refusal: {rows:#?}");
    let error = outcome.expect_err("a refused route cannot emit an artifact");
    let text = error.to_string();
    assert!(text.contains("unsupported runtime-IR lowering: PendingCallAdmission: refused pending call: RelocatedWorkMissingLoweringBinding"),
        "the user must see the classified admission reason: {text}");
    assert!(!text.contains("planner invariant") && !text.contains("compiler bug"),
        "the admission refusal is not a compiler ICE: {text}");
}
