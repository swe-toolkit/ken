---
name: a-negative-control-pins-only-the-check-that-alone-rejects-it
description: A negative control pins a check only if the control reaches that check and that check is the sole reason it rejects. Controls that all abort at an earlier stage, a new AND-wise layer that also rejects the injected fault, a rewritten fixture now stopped by a new early error, and one nulling arm under a many-leg conjunction all leave a named check deletable with the suite green. Map every check to a control that fails only because of it.
metadata:
  type: feedback
---

# A negative control pins only the check that alone rejects it

A check is pinned when deleting it (or replacing its body with `Ok(())`, or a
conjunct with `true`) reddens some control. That requires a control that
**reaches** the check and **fails only because of it**. Four Adversary findings,
2026-08-28 to 2026-09-25, each broke one half of that, merged here as one
mechanism.

| shape | which half fails | instance |
|---|---|---|
| all controls abort at an earlier stage | not reached | RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION |
| a rewritten fixture now stops at a new early error | not reached | LANG-FACADE-EXPORT-LOAD-ORDER |
| a new AND-wise layer also rejects the injected fault | reached, not sole | CORE-FO-CHECK-TREE-SORT-VALIDATION |
| one nulling arm under a conjunction of identity legs | reached, not sole, for every other leg | RT-CHECKED-IH-FRESH-RESULT-ROUTE |

## How to apply

1. **Build the matrix: checks by controls.** For each control, record the stage
   and the check where it actually rejects, by the exact error, not `is_err`.
   Do not credit a later check because "there are 11 controls".
2. **List the checks reached only by the exact (passing) case.** Those are
   executed but never refuted. For each, ask whether any control constructs the
   value that makes its branch fire; if not, mutate it and predict green.
3. **For each check a control names, evaluate every other layer on that
   control's fixture.** If another layer rejects it alone, the control is
   over-determined and its named check is masked.
4. **Decompose conjunctive predicates into legs** and map each nulling or
   mutation arm to the one leg whose equality its suppressed endpoint
   participates in. Every other leg is asserted in the positive alone.
5. **Before asking for the missing control, check the violating input is
   constructible.** If no well-formed input reaches the branch, it is dead
   code, not a coverage gap
   ([[an-ordered-validators-first-stage-subsumes-most-of-its-later-laws]]); see
   the correction under instance 1.
6. **Fix by re-injecting the fault on a carrier that is well-formed for every
   other layer and leg**, so only the named check rejects.
7. **Severity:** if a sibling control still pins the property, it is an
   isolation weakness; if every control for a property is masked, the property
   is unpinned. Check whether the path is live.

## 1. Every malformed control aborts at stage one

**Measured 2026-08-28 on RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION
(`bd4ddf213`),
`crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`.**
Two fail-closed stages: `boundary_continuation_result_proofs` builds and
validates a proof population against a pre-mutation snapshot at the top of
`build_aggregate`; then `boundary_continuation_result_authorization`, reached
during boundary crossing, does per-instance checks including lifetime
containment (`environment_record.meet > paired_field.lifetime -> Err`). The
mutation suite (`RetainedResultClosureProofMutation`, 11 arms) perturbed only
stage-one inputs except two (suppress-arm, which skips authorization, and
drop-call-edge, which passes the lifetime check and fails later). No control
constructed `meet > lifetime`, so the check could be replaced with `Ok(())` and
the suite stayed green. Prioritize safety-critical checks (lifetime, escape,
containment, bounds) hiding behind identity joins: proving identity-exactness
does not imply the safety property.

**Correction (2026-09-26).** The repair node's own D0 found the check
**unreachable by construction**, not merely uncovered: the exhaustive
`LexicalClosure` occurrence-lifetime arm always yields `ActivationOwned` and
aggregate construction copies that to the paired field, so the refused ordering
cannot hold for any well-formed proof row (Architect acceptance
`evt_bm4trnrjpymy`; node `RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL`,
closed). Zero negative coverage was a consequence, not a gap. That is step 5
above: establish constructibility before filing the missing control.

## 2. A fixture rewritten for a new rule takes a second guard's only pin

**Measured 2026-09-25 on LANG-FACADE-EXPORT-LOAD-ORDER**, squash
`2c6f8b204958a8ce450cc228efe9d560eee45dae`, reported at `evt_48nh6tx24yedt`
(thread `thr_8345f1m812q9`). Severity low; behaviour intact, coverage lost. The
new rule made a same-unit self facade `UnboundName`, and two fixtures that used
one as their collision source were rewritten to expect the new error. One,
`forward_local_and_file_facade_collide_when_ids_differ`, had existed to reach
the delayed forward-export reconciliation collision in `checked_export_ids`
(`crates/ken-elaborator/src/modules.rs`); the new rule now stopped it first. Its
rewritten doc said *"a separate pin checks forward local ID reconciliation"*;
that pin is a positive case that never reaches the collision arm, and the new
replacement uses an eager `pub const`, which the old comment had warned
exercises only the eager check. Mutating the guard on the merged tree left the
elaborator lib green (554 tests); on the parent the same mutation reddened the
old fixture.

- In a diff, list every test whose expected outcome changed (error to
  different error, or success to error). The new rule drove those edits, not
  the test's purpose.
- Read the old test's doc for the guard it was written to reach, and check
  whether the new early error fires first.
- Mutate that guard on the merged tree and on the parent. Treat a "covered by a
  separate pin" claim in the rewritten comment as a prediction and read the
  named pin's path to the guarded arm. Related:
  [[a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named]].

## 3. A new rejection layer over-determines an old negative control

**Measured 2026-08-28 on the CORE-FO-CHECK-TREE-SORT-VALIDATION respin
(`99a0b548`, repairing the D1a fixture after `57d209fca` went CI-red),
`crates/ken-elaborator/tests/v3_fo_checker_soundness_d1a_rule_correspondence.rs`.**
The recut added `validate_certificate` as an AND-wise front: `check_tree =
validate && structural`. D1a's two negative controls assert `fok_check_tree
sequent stale_eigen_cert = False` with a non-fresh eigenparameter, meant to pin
the structural freshness check.

- **World case, clean:** body `FokAccess(Param0, Bound0)`, stale eigen `Param0`
  reused as a `ForallWorld` eigen. Validation passes (Param0 is consistently
  World), so freshness alone produces `False`.
- **Obj case, masked:** body `FokForcingP(Param0, Bound0)` fixes `Param0` to
  World, and `ForallObj` expects an Object eigen, so the shared parameter
  environment rejects on a sort conflict independent of freshness. Break
  freshness and the test stays green.

The tell is one injected term that is both the intended fault and a fault of
the new layer (a non-fresh parameter that is also wrong-sort; an out-of-range
index that is also a type error). Reused parameters are the usual culprit: the
reuse that makes it non-fresh also fixes its sort. Freshness stayed pinned by
the World sibling, so this was an isolation weakness. The clean Obj fixture
injects a well-sorted (Object) non-fresh eigen.

## 4. One co-emission nulling arm under a conjunction of identity legs

**Measured 2026-08-28 on RT-CHECKED-IH-FRESH-RESULT-ROUTE (`7d36d24f0`),
`crates/ken-cli/tests/rt_parity_native.rs` and
`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs`.** The headline
claim: the certified tail route is *"a directed value-flow edge rather than four
co-emitted endpoints."* The dynamic proof built a `paired` predicate as a
conjunction of `source_result_value == active_edge_value` (leg 1),
`header_input_value == ret_input_value` (leg 2), plus answer-route, ret-body and
forward-order legs. The only negative control,
`CheckedIhFreshResultRouteObservationMutation`, then had exactly `Exact` and
`CoEmissionOnly`, and its recorder suppressed only `source_result_value`; its
doc scoped it to leg 1. Replace leg 2's line with `&& true` and the suite stays
green: `Exact`'s `all(paired)` only weakens, and `CoEmissionOnly`'s
`all(!paired)` still fails at leg 1. Leg 2 was the sink half of the
directed-edge claim.

Not a live miscompile: leg 2's positive reads independent SSA values, so a real
sink bug fails `Exact`. The gap is the symmetric control, and a later refactor
deriving `ret_input_value` from `header_input_value` would go unnoticed. The
retro credited leg 2 to *"body-merge substitution"*, which is
`RouteBodyMergeOutput`, a **static** certificate-field rejection in a different
test. **Do not accept a static certificate-field rejection as coverage of a
dynamic emission-identity leg**; they are claims about different objects.
The cheap fix is one co-emission arm per leg that suppresses that leg's
identity while preserving both seats' emission. (Repaired by
`RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS`: the enum now has a
`PairingLegOnly(..)` arm and the control is
`fresh_result_route_pairing_control`.)

## Related

[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]] (its
7th axis: negatives short-circuit before the fixture-validity check),
[[a-mutation-that-reddens-does-not-confirm-which-detector-caught-it]] (the
two-factor mutation when an outer layer rejects first),
[[an-ordered-validators-first-stage-subsumes-most-of-its-later-laws]] (the dead
law, as opposed to the unreached one), and
[[attribute-a-suite-arm-reject-before-calling-it-a-gap]] (a coincidental reject
for the wrong reason).
