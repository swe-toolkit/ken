// AC-1 increment 1: checked-source planner admission only. Nothing in this
// test expects a package to have been emitted or a ticket to have been issued.

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

// Source declaration, not the planner coordinate, names the expected slot.
// Parsing the fixture's Ken declaration keeps `_caps`; erasure canonicalizes
// the runtime binder name to `program_caps`.
fn source_main_parameters(source: &str) -> Vec<String> {
    ken_elaborator::parser::parse_decls(source)
        .expect("checked source parses")
        .into_iter()
        .find_map(|decl| match decl {
            ken_elaborator::Decl::ViewDecl { name, params, .. } if name == "main" => {
                Some(params.into_iter().flat_map(|binder| binder.names).collect())
            }
            _ => None,
        })
        .expect("source declares main")
}

#[test]
fn selected_pending_call_planner_checked_source_baseline_probe() {
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
    let error = outcome.expect_err("increment 1 does not change emission's first refusal");
    assert!(format!("{error:?}").contains("BoundaryCarrier: a carried recursive hypothesis is an eliminated value, not a callable, so it takes no arguments, but the call provides 1"), "{error:?}");
    let planned: Vec<_> = rows
        .iter()
        .filter_map(|row| match &row.outcome {
            ken_runtime::SelectedPendingCallOutcomeObservation::Planned {
                candidates,
                width,
                defining_function,
                visited,
                traversed_families,
                gates,
                gate_binder_pairs,
            } => Some((
                candidates,
                width,
                defining_function,
                visited,
                traversed_families,
                gates,
                gate_binder_pairs,
            )),
            _ => None,
        })
        .collect();
    assert_eq!(
        planned.len(),
        1,
        "one differing-unit Match must be planned: {rows:#?}"
    );
    let (candidates, width, owner, visited, traversed_families, gates, gate_binder_pairs) =
        planned[0];
    assert_eq!((*width, *owner), (6, 3));
    assert_eq!(
        candidates
            .iter()
            .map(|candidate| (candidate.arm, candidate.body))
            .collect::<Vec<_>>(),
        vec![(0, 343), (1, 322)]
    );
    assert_eq!(
        candidates
            .iter()
            .map(|candidate| candidate.worker_captures)
            .collect::<Vec<_>>(),
        vec![3, 3]
    );
    // Transition sentinel for this checked source and planner base: each
    // origin below is an actual route step, not an allowed-family inventory.
    for expected in [(311, "F1"), (96, "F1"), (55, "F1"), (49, "F3"), (47, "F4")] {
        assert!(
            visited.contains(&expected),
            "missing route visit {expected:?}: {visited:?}"
        );
    }
    let derived_families = visited
        .iter()
        .filter_map(|(_, kind)| (*kind != "Local").then_some(*kind))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(traversed_families, &derived_families);
    assert!(
        !gates.is_empty(),
        "the planner must locate the carried-call path"
    );
    assert!(
        gate_binder_pairs.contains(&(47, 4, 4)),
        "erasure AC-2 makes the raw IH Var and binder morphism agree at marker 47; checked template identity, not index agreement, authorizes the gate: {gate_binder_pairs:?}"
    );
    for &(origin, ..) in gate_binder_pairs {
        assert!(
            visited.contains(&(origin, "F4")),
            "a binder pair must be attributed to a visited marker: {origin}"
        );
    }
    let source_main = source_main_parameters(PX7L);
    assert_eq!(source_main, ["_input", "_caps"]);
    for candidate in candidates {
        use ken_runtime::SelectedPendingCallCaptureObservation as Capture;
        assert!(matches!(
            candidate.context_sources.as_slice(),
            [
                Capture::ProducerLocal { ordinal: 0 },
                Capture::EntryAbi {
                    ordinal: 1,
                    slot: 1
                },
                Capture::EntryAbi {
                    ordinal: 2,
                    slot: 0
                }
            ]
        ));
        for (index, expected) in [(1, "_caps"), (2, "_input")] {
            let Capture::EntryAbi { slot, .. } = candidate.context_sources[index] else {
                panic!("a named source parameter must back context member {index}")
            };
            assert_eq!(
                &source_main[slot as usize], expected,
                "C{index} source name must load from its own declaration's ABI slot"
            );
        }
    }
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
