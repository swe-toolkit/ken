---
id: RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL
title: "CLOSED by its own D0: the result-closure lifetime-containment check at aggregates.rs:7253 is UNREACHABLE BY CONSTRUCTION, not merely uncovered. The exhaustive LexicalClosure occurrence-lifetime arm always yields ActivationOwned and aggregate construction copies that to the paired field, so with Persistent < ActivationOwned the refused ordering meet > paired_field.lifetime cannot hold for any well-formed proof row. Zero negative coverage is a consequence, not a gap."
status: closed
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]
blocks: []
github: null
origin: "Adversary M8 hunt evt_2kdx72vs884zp (thr_3q2mw0qb0xcq8), 2026-08-28, on squash bd4ddf2138362bd1ac7066c39161602fdc9dddc2 (range 4d3d4d848..6ce003a26, six paths). The hunt's verdict on that candidate was CLEAN core, strongly fail-closed, controls non-vacuous; this is its ONE grounded finding, classed leak/gap and explicitly LATENT. Routed to the Steward for disposition rather than reopening the merged node. Steward framing per COORDINATION section 2."
---

> # CLOSED 2026-08-29 — D0 RETURNED "NOT CONSTRUCTIBLE", WHICH THIS FRAME
> # DEFINED AS A COMPLETE DELIVERABLE. `AC-GUARD-NOT-SHADOWED` IS DISCHARGED.
>
> D0 at `origin/main` `041bdd637`, tree `da1c15001209dc4283ac18eb9ac3b196282046ba`
> before and after, report SHA-256
> `01be9e079ee2d6b4805c0b5b60dd4bb355a51452c5ffefa9622885f1bb1b724d`. No control,
> no production change, no commit, no candidate, no QA route — as scoped.
>
> **The guard is unreachable BY CONSTRUCTION, and the block precedes population
> validation.** `boundary_continuation_result_proofs` admits only a
> lexical-closure proof seat and rechecks paired-field origin; aggregate
> construction copies the field lifetime from occurrence authority; the
> exhaustive `LexicalClosure` occurrence-lifetime arm always yields
> `ActivationOwned`. With `Persistent < ActivationOwned` the paired field is
> forced to the maximum while the environment meet is at most that maximum, so
> **`meet > paired_field.lifetime` is impossible for every well-formed proof
> row.** Measured, not only argued: 79 authorization visits across three exact
> result rows, all `ActivationOwned`/`ActivationOwned`, **zero** `Persistent`
> fields and zero mismatches.
>
> **Steward verified the two load-bearing facts independently** rather than
> repeating the ring's verification: `PlannedReferentLifetime` has exactly two
> variants declared `Persistent` then `ActivationOwned`, and the comparison sits
> at `:7253` inside `boundary_continuation_result_authorization` opening at
> `:7187`. The "always `ActivationOwned`" arm is taken on the ring's 79-visit
> measurement, not on a Steward reading.
>
> **This is a RESOLUTION of the Adversary finding, not a deferral.** "Zero
> negative coverage" is a CONSEQUENCE of unreachability, not a gap in the suite.
> The check is belt-and-suspenders — the same disposition already recorded below
> for the `:3109` exclusivity guard. **Rewriting the field or the lifetime arm to
> force the shape manufactures a post-construction state; it does not produce a
> real occurrence**, and doing so would have been precisely the manufactured
> control `AC-NOT-MANUFACTURED` forbids.
>
> ### CARRY-FORWARD IS A PREDICATE, AND IT IS **NOT** KEYED ON D3
>
> **Rerun only if a later authorized design changes (a) the admitted proof-seat
> shape, (b) the lexical-closure field-lifetime classification, or (c) the
> lifetime order.** Nothing else re-opens it.
>
> **THIS CORRECTS THIS FRAME'S OWN PREMISE, WHICH WAS WRONG.** The text below
> said the finding *"bites when [[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]]
> lifts D3 and real programs cross result-closure environments."* **D0 refuted
> that: D3 execution alone cannot create the ordering.** So the carry-forward was
> NOT re-pointed to that node's successors when it closed — re-pointing a
> sentinel at whatever replaced its old target would have carried a FALSE trigger
> forward and guaranteed a wasted rerun. **A carry-forward names the condition
> that revives it, never the node that happened to be next.**
>
> **Stale coordinate corrected:** the frame carried `aggregates.rs:6913` from the
> Adversary hunt; in the released tree the comparison is `:7253`. Same class as
> the `~3957`-vs-`:3920` correction already recorded below — **a line number in a
> hunt describes the tree when it was written.**

> # HISTORICAL — NOT AN AMENDMENT TO THE MERGED NODE, AND NOT D3 WORK
>
> [[RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]] merged at `bd4ddf213`.
> **This finding does NOT reopen it.** The check is CORRECT and it PASSES; what
> is missing is any control proving it load-bearing. The landing stands and its
> gates stand.
>
> **STALE-BANNER CORRECTION, 2026-08-28, applied at release review.** This block
> previously read *"the runtime ring is on [[RT-CHECKED-IH-FRESH-RESULT-ROUTE]]
> and must not pick this up."* **That node MERGED at `7d36d24f0`** and the
> prohibition is void. It is corrected here rather than waived in a convo post,
> because a frame passage that outlives the condition it was written under does
> not read as stale — it reads as authoritative, and a ring reading this node
> cold would have refused a node the Steward had just released.
>
> **RELEASED 2026-08-29 — this node is `active` and IS the runtime ring's
> current work.** The queue condition it waited on is discharged:
> [[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] took **HARD STOP 14** when its
> corrected D0 returned **NO** (Architect acceptance `evt_bm4trnrjpymy`, exact
> `47b55bd5a`), so D3 is frozen pending a Steward-routed design ruling and is
> not holding the seat. `aggregates.rs` contention was re-measured at release,
> not inherited: no branch ahead of `origin/main` carries any commit over that
> file, and D3 has no remote branch at all.
>
> **The standing yield rule still binds.** If D3 is re-released over
> `aggregates.rs`, that is live contention and this node yields — see the
> Contention check, which is a criterion and not a historical note.
>
> **Prior history, kept because it explains the banner above it.** This node was
> briefly flipped `active` on 2026-08-28 while D3 sat blocked at HS10 with the
> runtime seat idle; the Architect then ruled that stop (`evt_1ckwtvwe23e3e`)
> and D3 resumed, so the flip was reverted before any release was posted and
> nothing was handed to the ring. **That is the opposite of what happened here:
> this release follows a D0 that returned NO, not a stop that was cleared.**

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

That is what this node is: an unpinned guard on a staged path, not a blocker on
any lane. It is `draft` and queued for that reason. **Nothing here is a
regression, nothing is on fire, and the finding has not become more urgent since
it was filed.** When it is released it will be because a seat came free, never
because the risk changed.

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
[[RT-CHECKED-IH-FRESH-RESULT-ROUTE]] MERGED at `7d36d24f0`, so the in-flight
concern this section was written against is discharged. **Re-checked at release,
2026-08-28, and this is a measurement rather than an inheritance:** no runtime
node holds an open candidate over `aggregates.rs`.
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] stopped cleanly at HS10 with no
commit and no candidate, and freed its branch at `da95daadf`.

**The standing criterion is unchanged and still binds: do not release, or
continue, while another ring holds an open candidate over this file.** The route
node's own landing touched `aggregates.rs`, so the disjointness once expected
here was not real — it was the merge that removed the conflict, not the
separation of surfaces. Treat a re-released D3 over this file as live
contention.

## Sequencing

**`active`, RELEASED 2026-08-29. Dependencies were always clear; the queue was a
PRIORITY call and that priority has moved.** Both
`depends_on` are `merged` (re-measured 2026-08-28), and
[[RT-CHECKED-IH-FRESH-RESULT-ROUTE]] — which this node was originally queued
behind — has landed. **Nothing blocks it technically.** It waits because
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] is lane 1's main line and the
runtime seat holds one WP at a time.

**When it does go, it goes ahead of the two S/T2 siblings**
([[RT-RETAINED-TARGET-IDEMPOTENT-REDERIVATION]],
[[RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS]]) on a capability match, not on
urgency: all three are Adversary M8 findings on landed nodes and none is a live
defect, but this one is T1 and the runtime implementer seat is T1-provisioned,
while putting a T2 node on that seat spends a reasoning tier the work would
never exercise (`steward.md` §4h).

**Release trigger — FIRED 2026-08-29.** The condition was "the next time the
runtime seat frees with D3 not holding it, re-checking `aggregates.rs`
contention at that moment rather than trusting this paragraph." Both halves were
measured at release: D3 froze at HS14 on a D0 that returned NO, and no branch
ahead of `origin/main` carries a commit over `aggregates.rs`.

**Tier T1, though the node is small.** Size and capability are independent axes
(`steward.md` §4h). Nothing here is mechanical: the whole deliverable is an
argument about whether a control is causally honest, and the parent node proved
twice that this specific judgment is where competent work goes wrong. A seat
that writes a plausible mutation without asking whether it discriminates will
produce a green suite and a worthless control.
