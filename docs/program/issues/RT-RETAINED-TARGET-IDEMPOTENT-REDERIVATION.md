---
id: RT-RETAINED-TARGET-IDEMPOTENT-REDERIVATION
title: "Retained-body uniqueness check rejects idempotent re-derivation, not just ambiguity — units.rs:683 collides on the body key without comparing target values, so a call-graph diamond (one static body reached from two reachable owners, resolving to the SAME target) is refused as ambiguous. Latent on the staged retained-unit path today; it bites when the closure-representation successor completes the path."
status: draft
owner: runtime
size: S
gate: none
tier: T2
depends_on: [RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]
blocks: []
github: null
origin: "Adversary M8 hunt evt_4qegxp4q31ytc (thr_3q2mw0qb0xcq8), 2026-08-28, on squash e03b4d500df422ed2fd7a14569279f1a48be64cd (range 35b9d3fa1..a5d21de81, three paths, +363/-12). The hunt's verdict on that candidate was CLEAN core with non-vacuous controls; this is its ONE grounded finding, classed leak/gap and explicitly LATENT — not a live regression. Routed to the Steward for successor-WP disposition rather than reopening the merged node, which landed as accepted partial work. Steward framing per COORDINATION section 2."
---

> # NOT AN AMENDMENT TO THE MERGED NODE, AND NOT WORK FOR THE IN-FLIGHT SUCCESSOR
>
> [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] merged as accepted partial work at
> `e03b4d500`. **This finding does NOT reopen it.** The defect is latent — no
> currently-green program completes through the affected path — so the landing
> stands and its gates stand.
>
> **The Adversary recommended the successor WP absorb the positive fixture. The
> Steward declines that placement.**
> [[RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]] is a DIFFERENT component
> object — post-call result composition and runtime-value representation, ruled
> distinct by the Architect at `evt_2jdfsv6w8nh19`. This finding is
> call-graph-derivation authority, the SAME object as the merged predecessor.
> Folding it in would bundle two component objects into one acceptance criterion,
> which is the exact defect the predecessor's `AC-DERIVE` was recut to remove.
> **The successor's ring is in D0 and must not pick this up.**

## Objective

On a body-key collision in the retained-body target map, compare the resolved
TARGET and reject only on disagreement. An identical re-derivation of the same
`(body -> target)` pair is benign and must be ADMITTED.

## Fixed inputs (Adversary `evt_4qegxp4q31ytc`, measured at `e03b4d500`)

**These are the Adversary's measurements, not the Steward's.** D0 reproduces or
corrects them.

- The reject is at
  `crates/ken-runtime/src/cranelift_backend/lowering/units.rs:683` —
  `if unique.insert(claim.body, claim.target).is_some()` raising "retained body
  ... has more than one graph-derived call target".
- It is fed by the walk at `units.rs:636-644`, which pushes one claim per
  `(owner, StaticBody edge)` with key `body: target.call_site_origin`
  (`units.rs:641`).
- **The walk dedups OWNERS via `visited` but does NOT dedup CLAIMS.** A body `B`
  reached by a `StaticBody` edge from two distinct reachable owners `X` and `Y`
  yields two claims with the same `body` key and the same resolved target.
- The consumer is body-keyed: `call_declared_unit(body_origin)` at
  `calls.rs:1638` looks the target up by body. One target per retained body is
  the map's contract, and a diamond satisfies it.
- The affected path is `define_continuation_context_bodies`, the staged
  retained-unit / continuation-context path.

## Why this is latent and NOT a live regression

The flagship `WRITE_ALL` fixture still refuses downstream — its `#[ignore]` at
`px8f_buffer_native.rs:250` is re-pointed to
[[RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]] — and D3A+D3B are frozen. No
currently-green program completes through this path, so the over-rejection
cannot regress anything today. **It bites when the successor completes the path
and a valid continuation-context graph shares a static body across two edges.**

That is why this node is `draft` and queued rather than released: it is a
correctness gap on a staged path, not a blocker on any lane.

## The stated rationale is narrower than the implementation

The check's own comment justifies rejecting by "choosing one by preference or
iteration order is forbidden". **That rationale covers DISAGREEING targets
only.** For identical targets there is nothing to choose and no preference is
being exercised. The implementation is stricter than the reason given for it,
and the gap between the two is the whole defect.

**This does not license widening the check.** Wrong-target and ambiguous-target
outcomes must still be REJECTED, never resolved by preference or first-match —
that is the merged node's binding design judgment and it is unchanged here.

## Deliverables

- **D0 — measure before changing anything.** Reproduce the collision-on-agreement
  behaviour, and **verify the one link the Adversary could not build here**: that
  `source_bindings` collapses `(X, B_origin)` and `(Y, B_origin)` to a single
  source-body binding value (`units.rs:1093-1102`). If it does NOT, the diamond
  produces claims that differ in more than multiplicity and this frame's premise
  is wrong — **stop and return that**, do not repair around it.
- **D1 — the fix.** On collision: vacant inserts; occupied with a DIFFERING
  target errs as today; occupied with an EQUAL target is a benign skip.
- **D2 — controls**, below.

## Acceptance criteria, each with its control

- **AC-ADMIT-IDENTICAL.** Two claims sharing a `body` key and resolving to the
  SAME target compile without refusal. Control: the identical-duplicate mirror of
  the existing `DuplicateTargetClaim` control — that control relabels `claims[1]`
  onto `claims[0].body` with a DIFFERENT target, so the identical case is
  currently untested and the reject-on-agreement behaviour has NO coverage.
  Construct the same-target relabel; it must compile before D1 is claimed to work
  and it must FAIL against the pre-D1 code.
- **AC-REJECT-DISAGREEING.** The existing `DuplicateTargetClaim` control still
  reddens, unchanged and unwidened. Control: it is the pre-existing control; run
  it, do not edit it.
- **AC-NO-PREFERENCE.** The benign-skip arm may not select between two targets
  under any circumstance. Control: a mutation that makes the two targets differ
  by one field must reach the refusal, not the skip. **Equality must be over the
  whole target value, not a discriminant or a single field** — a partial
  comparison silently reinstates preference under a new name.
- **AC-FAIL-CLOSED-INTACT.** The `calls.rs:1638` fail-closed lookup still fires,
  unwidened, for a body whose target genuinely is not derivable. Control: the
  pre-existing witness that still reaches that refusal.

## FORBIDDEN

Do NOT widen or delete the `units.rs:683` refusal to make the collision stop
occurring — that removes the instrument rather than repairing it. Do NOT dedup
claims by discarding one before the check (that hides disagreement as well as
agreement). Do NOT touch `boundary_transfer_admissibility` or anything on the
successor's post-call result path.

## Not a defect — do not act on it

The Adversary also observed that the body-mismatch guard at `units.rs:676`
(`claim.body != claim.target.call_site_origin`) is a theorem on the exact path:
`claim.body` is assigned FROM `target.call_site_origin` at `units.rs:641`, so it
cannot fire in production and is non-vacuous only under the
`SubstituteWrongTarget` control — its intended and disclosed use. **No action.**
It is recorded here so a later reader does not re-derive it as a finding.

## Contention check

Touches `crates/ken-runtime/src/cranelift_backend/lowering/units.rs`, which
[[RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]] may also touch — that node's
own contention surface is `core.rs` (constructor composition and
`boundary_transfer_admissibility`), so they are expected disjoint, **but this is
the one file where they could meet. Re-check at release**, and release this node
only when the successor's ring is not holding an open candidate over `units.rs`.

## Sequencing

`draft`, QUEUED. Not released, and it does not block the lane. The runtime ring
is single-threaded on
[[RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION]]; this waits behind it.

**Release it when the runtime seat frees.** The fix and every control above are
constructible TODAY — they are claim-level, not end-to-end — so this node does
not need the successor to land first. A stronger end-to-end witness (a real
generated continuation-context graph sharing a static body across two call
sites, asserted to compile) only becomes constructible once the successor
completes the path; **that is a bonus, not a deliverable, and its absence is not
a reason to hold this node.**

Tier T2: the repair is mechanical and its review is differential. It is NOT T1 —
the design judgment it rests on (reject disagreement, admit agreement, never
prefer) is settled above and in the merged predecessor, so nothing here turns on
an argument.
