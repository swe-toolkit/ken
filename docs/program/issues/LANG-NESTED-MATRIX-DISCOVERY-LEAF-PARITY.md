---
id: LANG-NESTED-MATRIX-DISCOVERY-LEAF-PARITY
title: "An inferred nested match whose first leaf sits under an indexed family's IH now fails with Internal, replayed leaf changed a non-IH context entry, because the discovery pass builds that leaf's context differently from the check-mode rerun. Make discovery and rerun build the same leaf context, so the programs db19a8d0c^ accepted elaborate again"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_4te2mfx5tsenj on db19a8d0c (LANG-NESTED-MATRIX-DERIVED-TELESCOPE increment 2): fail-closed regression, measured at db19a8d0c and its parent. Steward-filed per COORDINATION section 2; it runs ahead of LANG-SIBLING-GOAL-REFINEMENT increment 2 because it repairs landed behaviour."
---

# Discovery and rerun build the same first leaf

## Objective

Every inference-mode nested match that `db19a8d0c^` elaborated correctly
elaborates to the same value on main, and the replay guard
(`elab.rs:17800`) is reached by no well-typed surface program in the
Adversary's set.

## Settled inputs (Adversary `evt_4te2mfx5tsenj`, measured on `db19a8d0c`)

- E1: `fn f (n : Nat) (xs : CVec Nat n) (b : Bool) : Nat = let r = match (xs,
  b) { (CCons m e tl, True) ↦ e; (CCons m e tl, False) ↦ m; (CNil, _) ↦
  Zero } in r` gives `Internal("replayed leaf changed a non-IH context
  entry")`; on the parent `o1` is `Suc (Suc Zero)`. The check-mode form is
  correct at both SHAs.
- Siblings: D1, D2, D5, D8 (`(CVec, Vec)` tuples) and D7 (inferred inner
  match under a checked outer leaf, both forms) regress the same way. D3 and
  D4 (sibling field after the IH) were refused on the parent and are
  `Internal` now in infer mode, correct in check mode. E2, E3 and D6
  (non-indexed) stay correct.
- Mechanism: in D1 the leaf context differs at the `tl` field before the
  skipped IH: discovery holds `CVec Nat @2`, the rerun `CVec Nat @1` (the
  correct one). Two discovery-side sources: tail columns passed unshifted
  past the never-pushed IH (`elab.rs:18058-18068`; shifting them repairs D3
  and D4 only) and field domains read under the domain-only motive
  (`elab.rs:18505-18519`).
- The frame's R contract (Architect `evt_9s1tts2m0xjk`, input g
  `evt_216r68j77vfdx`) makes the mismatch an internal invariant. The
  Architect rules the fix shape before the implementation turn.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. Discovery builds the first leaf's context in the rerun's coordinate
   system, on the Architect's shape.
2. Committed pins for E1, D1, D2, D3, D4, D5, D7 and D8 in infer mode (and
   D7's check form), asserting values, plus the E2, E3 and D6 controls.

## Acceptance

- **AC-1.** Each regressed row gives its parent value; D3 and D4 give
  their check-mode values. The controls and the TELESCOPE suites are
  unchanged; catalog `ken check` outcomes are unchanged.
- **AC-2 (falsifier).** Reverting the discovery-side change reddens E1 with
  the `Internal` replay error.

## Stop conditions

- The repair needs the replay guard weakened or removed: stop to the
  Architect.
