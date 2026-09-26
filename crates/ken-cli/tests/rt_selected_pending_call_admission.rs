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
