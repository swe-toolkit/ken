---
name: a-uniqueness-check-that-rejects-on-key-collision-without-comparing-values-over-rejects-a-graph-diamonds-idempotent-re-derivation
description: A dedup/uniqueness gate that rejects on the second key collision (`map.insert(k, v).is_some()` -> Err) without comparing the two values over-rejects idempotent re-derivation -- a call-graph diamond re-derives the identical (key -> target) pair and is refused as "ambiguous" though there is nothing to choose. Tells - a walk that dedups nodes but collects claims per edge, and a mutation control that manufactures only the disagreement case.
metadata:
  type: feedback
---

# A key-collision uniqueness check without value comparison over-rejects a diamond's idempotent re-derivation

**Measured 2026-08-28 on RT-RETAINED-UNIT-CALL-TARGET-DERIVATION (`e03b4d500`),
`crates/ken-runtime/src/cranelift_backend/lowering/units.rs`.**
`declare_retained_body_targets_in_func` walks StaticBody graph edges
transitively reachable from a generated context's raw owner. The walk dedups
**owners** via a `visited` set but pushes one **claim per (owner, edge)** with a
per-body key `body: target.call_site_origin`. The dedup gate is
`if unique.insert(claim.body, claim.target).is_some() { Err("retained body ...
has more than one graph-derived call target; choosing one by preference or
iteration order is forbidden") }` -- it rejects on the **second occurrence of
the key** and never compares the two target **values**. (Line numbers at that
SHA were 636, 640-644 and 683.)

A call-graph **diamond** -- one shared static body B reached via a StaticBody
edge from two distinct reachable owners X and Y -- yields two claims with the
**same key and the same resolved target**. The gate refuses it as ambiguity,
though there is nothing to choose: idempotent re-derivation of the identical
`body -> target` pair from two graph paths is well-formed. The gate's own
comment justifies the reject by "choosing one by preference or iteration order
is forbidden", which is the reasoning for **disagreeing** targets only; the
implementation is stricter than its stated rationale.

**The tell that the agreement case is unhandled:** the mutation control that
exercises this arm (`DuplicateTargetClaim`) manufactures ONLY the
**disagreement** case -- it relabels `claims[1]`'s target onto `claims[0].body`
(a different target under the same key). The identical-duplicate case is never
constructed, so the reject-on-agreement behavior has zero coverage. A control
that only injects the disagreeing collision cannot show the gate tolerates the
agreeing one.

## How to apply

When a derivation collects claims or edges into a `BTreeMap`/`HashMap` and
gates uniqueness with `insert(k, v).is_some() -> Err`:

1. Check whether the producing walk can reach one key via **two paths** (a
   diamond). A node-dedup `visited` set does NOT dedup per-edge claims.
2. If so, the correct gate compares values and rejects only on
   **disagreement** (`Vacant -> insert; Occupied && differs -> Err; Occupied &&
   equal -> benign skip`). Ask whether the identical-duplicate path is refused.
3. Confirm the control suite actually constructs the **agreement** collision,
   not only a relabeled disagreement. If it only tests disagreement, the
   over-rejection is uncovered. The fix is cheap and standard: compare on
   collision.

**Severity calibration (this instance was LATENT, not live):** the gate sat on a
staged path (`define_continuation_context_bodies`) no then-green program
completed, so the over-rejection could not regress a green program at the time;
it bites when a successor completes the path. Rank as a leak/gap and recommend
a positive diamond fixture (one body, two reachable call sites, asserts it
compiles).

Kin of [[a-population-held-at-a-degenerate-value-cannot-see-that-axis]],
[[a-candidate-collector-can-over-collect-and-the-design-note-only-reasons-about-missing]],
and [[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]].
