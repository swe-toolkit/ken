---
id: RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL
title: "The result-closure lifetime-containment check has zero negative coverage — aggregates.rs:6913 refuses an environment whose meet outlives its paired constructor field, but no control ever constructs that shape, so deleting the check leaves the whole suite green. Latent today; it guards an escape invariant that goes live when D3 lifts."
status: draft
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]
blocks: []
github: null
origin: "Adversary M8 hunt evt_2kdx72vs884zp (thr_3q2mw0qb0xcq8), 2026-08-28, on squash bd4ddf2138362bd1ac7066c39161602fdc9dddc2 (range 4d3d4d848..6ce003a26, six paths). The hunt's verdict on that candidate was CLEAN core, strongly fail-closed, controls non-vacuous; this is its ONE grounded finding, classed leak/gap and explicitly LATENT. Routed to the Steward for disposition rather than reopening the merged node. Steward framing per COORDINATION section 2."
---

> # NOT AN AMENDMENT TO THE MERGED NODE, AND NOT WORK FOR THE IN-FLIGHT NODE
>
> [[RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]] merged at `bd4ddf213`.
> **This finding does NOT reopen it.** The check is CORRECT and it PASSES; what
> is missing is any control proving it load-bearing. The landing stands and its
> gates stand.
>
> **The runtime ring is on [[RT-CHECKED-IH-FRESH-RESULT-ROUTE]] and must not
> pick this up.** That node is a typed forward-route derivation under an HS10
> stop rule; this is control authoring over an already-landed guard. Different
> object, different turn.

## Objective

Prove the lifetime-containment refusal at `aggregates.rs:6913` is load-bearing,
by constructing a control that reaches it with a REAL over-living occurrence —
or establish, as a measured result, that no such occurrence is constructible
until D3 lifts.

**Both outcomes are deliverable.** This node exists because nobody has asked the
question, not because the answer is known.

## Fixed inputs (Adversary `evt_2kdx72vs884zp`, measured at `bd4ddf213`)

**These are the Adversary's measurements except where marked. D0 reproduces or
corrects them.** Path, line numbers and text below were re-verified by the
Steward against `origin/main` `572ca1e17` and are exact.

- The check is at
  `crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs:6913`,
  inside `boundary_continuation_result_authorization` (opens `:6847`):
  `if environment_record.meet > paired_field.lifetime` raising
  *"a retained result-closure environment outlives its exact constructor field"*.
- The property it guards is an **escape / use-after-scope invariant**: the
  crossed result-closure environment must not outlive the constructor field
  backing it. It is INDEPENDENT of the tuple-identity joins, which pin identity
  and not lifetime, so nothing else in the proof implies it.
- **Why nothing covers it.** `boundary_continuation_result_proofs(plan)?` runs
  early in `build_aggregate_ownership_plan`, so all NINE population mutations
  (drop / duplicate / substitute-owner / body / field / target,
  permute-captures, widen) abort at population-validate BEFORE any boundary
  crossing runs. Only `exact`, `suppressed-result-authorization` (which skips
  the arm via the `:3109` exclusivity guard) and `missing-static-body-call-edge`
  reach authorization at all. Under `exact` and `drop-call-edge` the check
  EXECUTES but PASSES — `WRITE_ALL` satisfies it with slack.
- **Repro of the gap:** replace `:6913`'s body with `Ok(())`. `exact` still
  compiles, every malformed control still fails at its own upstream point, and
  `retained_result_closure_proof_controls_are_exact_and_positional` stays green.

> ### STEWARD CORRECTION TO THE CITED COORDINATE — carry this, it is the lever
>
> The hunt places the proofs call at *"the TOP of build_aggregate (~3957)"*.
> Measured: the call is at **`:3920`**, inside `build_aggregate_ownership_plan`
> (opens `:3731`). The ordering claim is unaffected and holds.
>
> **But `~3957` is not the call site — it is where `meet` is DERIVED**, from the
> escape analysis: `meet` becomes `PlannedReferentLifetime::ActivationOwned`
> when any child owner is `BoundaryReferentOwner::InvocationArena`, and
> `Persistent` otherwise. That is the natural producer of the very value
> `:6913` reads, and it is where a real over-living occurrence has to come
> from. D0 starts there.

## Why this is latent and NOT a live regression

`WRITE_ALL` satisfies the invariant, and the end-to-end path is `#[ignore]`d at
the D3 frontier with D3A+D3B frozen. Nothing miscompiles today. **It bites when
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] lifts D3 and real programs cross
result-closure environments** — at which point a memory-safety-relevant check
would go live having never once been shown to fire.

That is why this node is `draft` and queued: an unpinned guard on a staged path,
not a blocker on any lane.

## Deliverables

- **D0 — constructibility, before writing any control.** Determine whether an
  environment with `meet > paired_field.lifetime` is reachable at authorization
  in today's tree. Start at the `meet` derivation (~`:3957`) and work forward:
  what child-owner shape yields `ActivationOwned` against a `Persistent` paired
  field, and does such a population survive population-validate to reach
  `:6847`? **Report the answer either way.** A measured "not constructible until
  D3 lifts, and here is the exact blocking step" is a complete D0 and closes
  this node into a carry-forward on the D3 node.
- **D1 — the control**, only if D0 says yes. One negative control producing a
  real over-living occurrence, asserting the exact `:6913` refusal text.
- **D2 — the two-sided proof.** D1 must FAIL against a tree with `:6913` neutered
  to `Ok(())`, and PASS against the tree as it stands.

## Acceptance criteria, each with its control

- **AC-REAL-OCCURRENCE.** The control must produce the over-living shape by
  moving the natural producer of `meet` — the escape/ownership analysis — so
  that a genuine occurrence arrives at authorization. Control: the mutation is
  named by its INJECTION POINT, and that point is upstream of `:6847`.
- **AC-NOT-MANUFACTURED — the hard one, and the reason this node is T1.** A
  control that injects an error INSIDE `boundary_continuation_result_authorization`,
  or that writes `meet` directly at the check, is NOT evidence the check is
  load-bearing — it proves only that a refusal placed there refuses. **This is
  the exact defect that cost the parent node two gate rejects** (the inert
  `SuppressResultAuthorizationArm`, `evt_63cdjgpfzzp1e`), and it is
  self-evaluatable: if the mutation would still red with `:6913` deleted, it is
  measuring the mutation and not the guard. Control: run D1 against the
  `:6913`-neutered tree; it MUST go green there.
- **AC-GUARD-NOT-SHADOWED.** Prove no upstream invariant already excludes the
  over-living shape. If population-validate or the escape analysis itself
  refuses it first, then `:6913` is unreachable-by-construction and pinning it is
  impossible — **that is a finding, not a failure**, and it belongs in the
  report.
- **AC-SUITE-INTACT.** Every existing control in
  `retained_result_closure_proof_controls_are_exact_and_positional` still passes,
  unedited. Control: it is the pre-existing suite; run it, do not touch it.
- **AC-AFFECTED-CLOSURE.** Cover every target that loads any module whose
  closure this increment changes, whether or not the increment touches that
  target's file. **Scope by which PATHS changed, never by which VALUES changed**
  — the parent node's CI red had every production value byte-identical on both
  sides and still broke an untouched consumer. Targeted via `scripts/ken-cargo`
  only, never `--workspace`.

## HARD STOP

**If the only achievable control is a manufactured one, STOP and return that.**
Do not ship a control that cannot distinguish the guard from itself. A green
suite containing a vacuous control is strictly worse than a known-uncovered
check, because it retires the question.

Likewise, if D0 shows the shape needs D3 lifted, **stop cleanly** — do not
construct a staged or `#[ignore]`d approximation to have something to land.

## FORBIDDEN

Do NOT delete, widen, or weaken the `:6913` check. Do NOT alter the escape
analysis or `meet` derivation as a production change — the mutation is a
control-side injection, not a semantics edit. Do NOT touch the D3 frontier, the
frozen D3A/D3B contract, or `boundary_transfer_admissibility`.

## Not a defect — do not act on it

The hunt also recorded two observations that are NOT findings, kept here so a
later reader does not re-derive them:

- The `:3109` exclusivity guard `if result_proof.is_some() { return Ok(None); }`
  is production-dead — authorization never returns `NotApplicable` when a proof
  is present, so it is reachable in production only under the suppress control.
  **Belt-and-suspenders, disclosed, non-vacuous under its own control.** This is
  the same shape as the `units.rs:676` body-mismatch guard recorded at
  [[RT-RETAINED-TARGET-IDEMPOTENT-REDERIVATION]]. No action.
- `boundary_continuation_result_proofs` is rebuilt in full on every environment
  query (once in `build_aggregate_ownership_plan`, then per-environment). A
  performance smell with correctness unaffected. No action.

## Contention check

Touches
`crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`.
[[RT-CHECKED-IH-FRESH-RESULT-ROUTE]] is in flight over the fresh-result route
and its own surface is the producer enum plus `lowering/source.rs`, so they are
expected disjoint — **but `aggregates.rs` is large and shared. Re-check at
release**, and do not release while that ring holds an open candidate over this
file.

## Sequencing

`draft`, QUEUED behind [[RT-CHECKED-IH-FRESH-RESULT-ROUTE]]. It does not block
the lane. [[RT-RETAINED-TARGET-IDEMPOTENT-REDERIVATION]] is also queued; both
are Adversary M8 findings on landed nodes and neither is urgent.

**Tier T1, though the node is small.** Size and capability are independent axes
(`steward.md` §4h). Nothing here is mechanical: the whole deliverable is an
argument about whether a control is causally honest, and the parent node proved
twice that this specific judgment is where competent work goes wrong. A seat
that writes a plausible mutation without asking whether it discriminates will
produce a green suite and a worthless control.
