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
- Mechanism and shape (Architect ruling `evt_3zjpbz03wm5b9`, measured with
  env-gated fixes; it corrects the Adversary's second source). Three
  defects, each a hand-written second derivation of a coordinate change:
  - (a) E1, D1, D7: discovery takes the `motive.is_none()` fallback, which
    builds fields with `weaken(ty, 1)` and shifts the earlier field `m`
    too (`@2` against `method_type`'s `@1`).
  - (b) D3, D4: the discovery IH branch (`:18058-18068`) does not push the
    IH but passes the telescope `col_types[1..]` on unlowered.
  - (c) `lower_by_inner` is not total (catch-all arm): THREE still fails
    after (a)+(b), and the R projection (`:17922`) rejects an inferred Σ
    result mentioning an outer variable (SIGR, and SIGRtree, which the
    parent also refused: pre-existing, fail-closed, ruled into scope
    because (b) needs the total primitive).
- The shape is the ruling's §3: item 1, a `tail_under_split` bucket always
  takes fields, IHs and tail from `method_type` (domain-only motive while R
  is unknown), the `weaken` fallback arm and `dependent_tail` go, and the
  impossible case is a typed `Internal`; item 2, one `lower_binders` built
  on the kernel's total `shift` replaces `lower_by`/`lower_by_inner`; item
  3, the IH skip lowers the tail through it.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. Items 1-3 of the ruling, one increment. The replay guard, literal memo,
   root-frame ownership and rerun path are unchanged.
2. Committed value pins, infer mode plus check forms, for every row of the
   ruling's table (E1, D1, D3, D7c, THREE, SIGR, SIGRtree, TREE control)
   and up to three reconstructed `(CVec, Vec)` variants labelled as such
   (the Adversary's D2, D5 and D8 sources were never posted).

## Acceptance

- **AC-1.** Each row gives the ruling's A+B+C value; trust unchanged on
  every Ok row. The TELESCOPE suites (`lang_nested_matrix_*`,
  `lang_nested_split_field_dependence`) and catalog `ken check` outcomes are
  unchanged. DEP stays refused (out of scope).
- **AC-2 (falsifiers; each gives its named red).** Restore the `weaken`
  route: E1 `Internal` replay. Pass `col_types[1..]` unlowered: D3 the same
  `Internal`. Old traversal at the IH skip: THREE `Internal`. Old traversal
  at `:17922`: SIGRtree `KernelRejected TypeMismatch`.

## Stop conditions

- The repair needs the replay guard weakened or removed: stop to the
  Architect.
- A committed `InferredMatchResultEscapesPattern` binder-name pin changes
  (the escaping index is now the smallest mentioned): stop and report; do
  not re-pin.
