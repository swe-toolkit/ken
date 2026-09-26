---
name: a-preventive-fail-safe-guard-that-and-narrows-an-existing-collapse-condition-flips-no-arm-today-so-verify-monotone-no-new-risk-and-pin-the-guards-own-determination-with-a-non-degenerate-pair-carrying-a-forward-gate
description: A "preventive collapsibility guard" landing — a new structural guard that gates a codegen optimization (a forward-edge return-collapse) but is declared to change no emitted bytes today ("flips no arm on today's programs") — is cleared by four moves, none from the commit prose. (1) MONOTONE-NARROWING = no new risk. Confirm the wiring ANDs the new guard INTO the existing gate: both consumption seams went from `if !existing_gate` to `if !existing_gate || !new_guard`, so collapse now needs BOTH. That can only REMOVE collapses (→ the base path, which the design establishes as always parity-correct), never introduce one — so no parity risk is introduced regardless of whether the guard is perfectly right; worst case is a lost optimization. Verify the direction: a guard ANDed into the fall-to-base test is safe; a guard ORed into the DO-collapse test would be the dangerous inverse. (2) FAIL-SAFE POSITIVE determination. The guard returns collapse-true ONLY on a positively-proven shape; EVERY ambiguity (no matching inheritance, no/2+ producer steps, None/Err resolution, any intervening control) returns false→base, and its error is `?`-propagated as a hard backend error, never a silent collapse. (3) LAYERING-DEFECT fix = canonical re-source. The guard reads the CANONICAL derivation built from the inert plan (build_X(self)?), NOT the stored suppressible field, because a SuppressForInertness pass clears the stored field and would flip the arm and diverge the final-codegen hash (the prior byte-inert-certificate defect); validate asserts stored==canonical so the determination is unchanged on the normal path. (4) The killer: a guard that flips no arm is UNPINNED by behavior fixtures — a hard-coded `false` passes every read-collapse and no-regression fixture identically, because the existing gate short-circuits the new guard away (`||` never evaluates it) on all current programs. So the control MUST observe the guard's OWN determination directly (recorded independent of the short-circuit) and assert a NON-DEGENERATE PAIR (a collapsible route AND a non-collapsible route in one program, classified oppositely), fail-closed both ways (stuck-false breaks the all-collapsible arm; stuck-true breaks the exactly-one-non-collapsible arm), keyed on the determination relation not origin ids, with a not-empty guard against the recorder silently never firing. FORWARD GATE: the property "collapse-true ⟹ genuinely no intervening control" is inert while the guard flips no arm, but becomes load-bearing at the NEXT increment that lets the guard actually keep the edge on a real (effect) tail — that landing must re-attack the reject-set completeness of the pure-narrowing walk (does any node in its descend arm carry an OUTWARD control transfer not exposed as a child?). The standing determination-pin is the tripwire for that drift. Measured 2026-09-04 on RT-COMPOSED-RETURN-FORWARD-RET-EDGE inc2 (A) ea0c04eec (PR #3313, base 87f650783), verdict evt_7psp39360rnt9. NO OBJECTION.
metadata:
  type: feedback
---

# A preventive fail-safe guard that AND-narrows an existing collapse condition changes no bytes today; verify it is monotone (no new risk) and PIN the guard's own determination, because behavior fixtures cannot

**Measured 2026-09-04 on `RT-COMPOSED-RETURN-FORWARD-RET-EDGE` inc2 (A)
`ea0c04eec38a26c2a957de2dfd32d86080a07661` (PR #3313, base `87f650783`),
verdict `evt_7psp39360rnt9` (thread `thr_7vb4452cn7m1y`).** The lieutenant flagged
it as a "preventive collapsibility guard": a structural guard
(`checked_ih_forward_edge_route_collapsible` in aggregates.rs) that gates a
Cranelift-backend forward-Ret return-collapse optimization, declared `gate=none`,
zero TCB, and "flips no arm on today's programs." Provenance clean, all 9 blobs
byte-identical to reviewed `ad96e1532` (whose own parent is the withdrawn
`8639b28ab` — consistent with a one-line canonical re-source superseding it).

## Why this shape (`Route::FO`-style verdict flips are NOT the only load-bearing
landings)

A codegen guard that "changes no bytes" reads as safe and boring. It is exactly
where a false sense of safety hides, because the property the guard establishes
is dormant now and consumed by a LATER increment. Clear it on structure, in four
moves — none from the commit's word.

## Move 1 — MONOTONE-NARROWING means no new parity risk, independent of guard
correctness

Read the WIRING, not the guard. Here both consumption seams changed from `if
!tail_worker_body_is_ret_kmatch(&transport)?` to `if
!tail_worker_body_is_ret_kmatch(&transport)? ||
!tail_route_is_forward_edge_collapsible(&transport)?`. That is the fall-to-BASE
test, so collapse now requires the existing gate AND the new guard. ANDing a
condition into the collapse set can only SHRINK it — every removed collapse
falls to the base path, which the node/spec establish as always parity-correct
(the unoptimized path). **No new collapse is ever introduced, so no parity
regression is possible even if the guard is wrong** — the worst case is a missed
optimization. The direction is the whole argument: a guard ANDed into the
fall-to-base test is monotone-safe; the dangerous inverse is a guard ORed into a
DO-collapse test (that ADDS collapses). Confirm which one you are looking at
before anything else.

## Move 2 — FAIL-SAFE positive determination

The guard returns collapse-`true` ONLY on a positively-proven pure-narrowing
tail; every ambiguity returns `Ok(false)` → base: no matching canonical
inheritance, no producer step, more than one producer step, a `None`/`Err`
recursive-unit resolution (the `Err` is the "units disagree" divergent recursor —
the known-unsound read-then-write tail is caught HERE, before the walk), or any
intervening control in the body. Its `build_X(self)?` error is `?`-propagated as a
hard `CraneliftBackendError`, never a silent collapse or silent base. Verify the
default of every arm is base.

## Move 3 — the layering-defect fix is the canonical re-source

The guard's result gates a codegen arm, so it must be a pure function of the
inert plan: it reads the canonical
`build_checked_ih_continuation_inheritances(self)?`, not the stored,
suppressible `checked_ih_continuation_inheritances` field, and `validate`
asserts stored == canonical. Reading the stored field was the
byte-inert-certificate defect on the withdrawn candidate `8639b28ab`. The
mechanism and why this is the ruled fix are in
[[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]].

## Move 4 — the killer: a no-arm-flip guard is UNPINNED by behavior fixtures

Because the guard is ANDed after an existing gate that short-circuits it, on
EVERY current program the `||` never evaluates the guard on the live seam, and no
read-collapse or no-regression fixture can observe it — **a hard-coded `false`
would pass every one of them identically.** The "the guard discriminates
correctly" acceptance is therefore vacuous unless the control observes the guard's
OWN determination. The correct pin (rt_parity_native.rs here):

- records the determination DIRECTLY, independent of the short-circuit (a
  test-support recorder fired for every formed authority, not just where the
  live seam consults the guard);
- asserts a NON-DEGENERATE PAIR — one program (read) all-collapsible, one program
  (write) with at least one collapsible AND exactly one non-collapsible — so the
  two classes appear and are opposite;
- is fail-closed both ways: stuck-`false` breaks `all(collapsible)` and
  `any(collapsible)`; stuck-`true` breaks `filter(!collapsible).count()==1`;
- is keyed on the determination RELATION, not on origin ids (ids appear only in
  comments), so it is a durable invariant not a snapshot;
- guards `!observations.is_empty()` so a recorder that silently never fires is
  caught (silence from an instrument means the wrong instrument).

If the pin only observed seat behavior, or only asserted one arm, it would be
unfalsifiable — the exact `Freeze PRED not ROSTER` / vacuous-control failure.

## FORWARD GATE — inc3 (the increment that flips the arm)

The property `collapse-true ⟹ genuinely no intervening pending control` is inert
this landing (ANDed with the worker-body gate; the known-unsound tail is caught at
the `Err` divergent-recursor check). It becomes LOAD-BEARING when inc3's
effect-continuation mechanism lets the guard actually keep the forward edge on a
real effect tail. At that landing, re-attack the reject-set completeness of
`checked_ih_body_is_pure_narrowing`: it rejects `Effect` + the four checked-IH
continuation nodes and skips closures, but treats ordinary `Call`/`PrimitiveCall`
as value-producing (descending only their arg subtrees, not callee bodies). Prove
no tail shape reaches the strict-Ret sink through a node in the descend arm that
carries an OUTWARD control transfer the walk does not expose as a child. The
standing determination-pin is the tripwire for that drift — it reddens if the
effect tail is ever mis-classified collapsible.

## FORWARD GATE — inc3 (i) PARTIAL DISCHARGE (2026-09-04, `553d99a42`, PR #3315)

The forward gate above fired as predicted. inc3 (i) "fail-closed reject-set
repair" landed (squash `553d99a42`, base `1b10b86ee`, verdict
`evt_3stvqh4prxq20`; reviewed `ac6874e0f`, rebase-forward over the cleared Map
inc5 — per-path blob-identity, whole-tree diff does NOT apply). It HARDENED
`checked_ih_body_is_pure_narrowing` to close the HS5 hole the gate named
(a value-returning `Call` whose static callee body returns a suspended
`ITree::Vis` — the "OUTWARD control transfer not exposed as a child"). Two
closures: the `Call` arm now resolves and traverses the exact local `StaticBody`
callee body (closure / lexical-closure / `DeclarationRef` via
`DeclarationCallTargetClass`; dynamic/imported/unresolved callee -> reject), and
a `Construct{..::ITree::Vis}` is explicitly rejected.

Two clearance facts worth reusing:
- **`{Ret, Vis}` completeness is PROVABLE, not asserted.** `data ITree R where
  Ret : R -> ITree R ; Vis : (Nat -> ITree R) -> ITree R` (effects/itree.rs:7,38)
  — exactly two constructors, so no third effectful ITree constructor escapes to
  the permissive `_` descend arm. That is why the Architect's constructor-set
  argument holds.
- **The `ends_with("::ITree::Vis")` predicate is NOT green-on-fixture-only.** It
  is the established backend-wide convention: the forward-Ret optimization this
  guard protects already keys on `ends_with("::ITree::Ret")` /`::ITree::Vis`
  (core.rs:8451/13441, source.rs:2201/2207, responses.rs:1327) for its LIVE,
  e2e-tested effect lowering (the P1 pair, effect_composition_state_console_e2e).
  So the real Vis id ends with the suffix (package-qualified too, e.g.
  `ctor:fixture::PX8TR::ITree::Vis`) and the guard fires on production Vis. A
  suffix false-positive only adds rejection (monotone-safe); only a false-negative
  is dangerous, and the live-elsewhere convention refutes it. GENERAL LESSON: a
  string-suffix constructor predicate in a new guard is cleared (not smelled) when
  the SAME predicate already gates a live, independently-tested path — verify that
  before flagging it as brittle.

Still MONOTONE this landing (source.rs gate wiring unchanged; the new
`candidate_body_purities` feeds only the `#[cfg(px8-ds-test-support)]` recorder),
and every walk change is a tightening (new `return Ok(false)` arms, deeper
descent, `?`-propagated `child_origins` error) — no new wrong-accept, so no parity
risk regardless of residual completeness. The Move-4 pin was strengthened
correctly: the determination-pin now records `(collapsible,
candidate_body_purities)` and adds `any(!collapsible &&
body_purities.any(!pure))`, so the effect tail's non-collapsibility is pinned to
the walk's OWN reject determination, not to multiplicity — the reject-set repair
is now its own tripwire.

RESIDUAL gate carried to inc3 (ii)+ (the arm-flip, where the walk finally becomes
load-bearing): three nodes remain in the permissive `_` descend arm whose "every
outward control transfer is exposed as a child origin" property must be confirmed
freshly — (1) `PrimitiveCall` (descends args only; confirm no `RuntimePrimitive`
suspends), (2) `CheckedComputationalIHSlots` (sits OUTSIDE the 5-marker reject set
that holds its four checked-IH siblings + `Effect`; confirm it is a pure
slot-binding structural node), (3) `Call -> SchedulingEntry` (traversed inline;
confirm scheduling a declaration is in-frame, not an outward deferral to a
scheduler). None reproduce today (monotone + arm-not-flipped).

## Relation to siblings

Same RT node family as
[[a-carrier-promoted-to-a-live-ssa-edge-clears-when-the-ordinal-is-derived-and-boundary-trapped-and-the-diff-already-fixed-the-byte-inertness-your-prior-lesson-predicts]]
(carrier→SSA edge, same byte-inertness discipline) and
[[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]] (a
`build_X(self)?` that can `Err` is not unconditionally inert — here the `?`
propagates as a hard failure, which is the safe direction). The forward-gate
discipline mirrors
[[a-kernel-guarded-encoder-is-non-trusted-iff-the-composite-is-kernel-rechecked-against-an-independent-obligation-and-the-apparatus-lands-inert-with-zero-verdict-path-callers]]:
an inert apparatus is cleared now, and the increment that WIRES it is where the
soundness property becomes load-bearing and must be attacked freshly. The Move 4
pin is an instance of
[[a-non-degenerate-pair-fails-to-fail-if-the-assertion-cannot-tell-the-halves-apart]].
