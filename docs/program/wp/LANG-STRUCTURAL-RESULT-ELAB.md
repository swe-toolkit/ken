# LANG-STRUCTURAL-RESULT-ELAB — implement the structural-result selector

**Owner:** `language`. **Size:** L. **Deliverables:** `D0`, `D1`, `D2`.

Tracker node: `docs/program/issues/LANG-STRUCTURAL-RESULT-ELAB.md`. **Read it,
and read its predecessor `KERNEL-RECURSIVE-RESULT-SURFACE.md`** — the
predecessor carries the measured obstruction, the Architect's approved semantic
shape, and the two prohibitions. Neither this frame nor the node restates them.

**The contract is landed normative spec.** `spec/30-surface/34-data-match.md
§3.1.1` and `39-elaboration.md §2.3`/`§4`. You implement it; you do not
re-derive it and you do not amend it here.

## Fixed inputs, measured at `f9572c27` (2026-08-10)

Perishable-frame discipline: every figure is measured at that SHA. **If `main`
has moved when you pick this up, re-measure and say so in your first
checkpoint.** A number here is a fixed input, never an acceptance criterion.

| input | value at `f9572c27` |
|---|---|
| the association struct | `crates/ken-elaborator/src/elab.rs:421` `struct LiftBinding { evidence_position: usize, support: Option<GlobalId> }` |
| its validator | `elab.rs:426` `validate_lift_associations`, two production callers at `:1165` and `:1285` |
| the association store | `cx.lift_bindings: HashMap<usize, LiftBinding>`, keyed by source field |
| hidden-binder gate | `elab.rs:349` `hidden_positions`, consulted by `surface_var` at `:385-388`; pushed at `:1145`, truncated at `:1193` |
| lift entry point | `elab.rs:1002` `check_match_with_lift` |
| telescope helper | `ken-kernel/src/inductive.rs:1095` `all_support_evidence_positions`, already imported by `elab.rs:14` |
| `elab.rs` size | 7645 lines |
| existing validator unit tests | `elab.rs:7607-7645` |

## Design judgment, front-loaded: `D0` extends, and the gap is one direction

**Do not write a new association mechanism.** `LiftBinding` already carries the
evidence position and the support provenance, and
`validate_lift_associations` already implements three of the spec's four
failure modes — its error strings are literally *"missing generated lift
association"*, *"swapped …"*, and *"foreign …"*. `all_support_evidence_positions`
is already imported. The recursive-result position is a **third field on an
existing struct**, validated by an **existing** function at **two existing**
call sites.

**The one thing genuinely absent is the duplicate direction**, and it is worth
knowing before you start rather than discovering it in review:

`validate_lift_associations` iterates `expected` and looks each entry up in
`installed`. That checks every expected association is present and correct. It
**never checks the other direction** — that no evidence or result term serves
two fields. The store being a `HashMap` keyed by source field makes *one field
to two results* unrepresentable, so the surviving hazard is precisely
**injectivity of evidence and result over fields**, which nothing today tests.

The spec closes exactly this: *"one-to-one in both directions. A result or
evidence term cannot serve two fields, and a field cannot select two
results."* ⇒ `StructuralResultAssociationDuplicate` is not a fourth variant of
the existing check; it is **the missing direction of the existing check**, and
implementing it as a parallel scan over `installed` is the natural shape.

⚠ **The existing error strings are `&'static str`, not the spec's named
diagnostics.** `D0` owes the five named diagnostics of `39 §4` with the spans
that table specifies. Renaming the three that exist is in scope; inventing a
sixth is not.

## `D0` — the association

Extend `LiftBinding` with an optional recursive-result position, derived
**only** from the kernel method telescope, per the correlation the predecessor
node states: for each `support_shapes[shape_ordinal]`, correlate
`shape.position` with the aligned support-evidence argument
`host_ctor.args.len() + evidence_ordinal`; map that evidence ordinal back
through `all_support_evidence_positions` to the exact source field; the result
is the method binder at `support_ctor.args.len() + shape_ordinal`.

Validation runs **before** branch-body elaboration and fails closed. No
guessed, positional, or name-derived fallback.

**Scope: `crates/ken-elaborator/` only. No surface form in `D0`.** Diagnostics
owed: `StructuralResultAssociationMissing`, `...Duplicate`, `...Swapped`,
`...Foreign`.

## `D1` — the selector

The `structural_result` production of `32 §3`; name resolution of the operand
to exactly one surface binding; elaboration to the associated hidden result
term, typed as the method telescope assigns it, including a `Type`- or
`Omega`-valued motive. Diagnostic owed: `StructuralResultOutOfScope`.

⛔ **The two prohibitions are where this deliverable fails.** The hidden term's
name is never published, no constructor-pattern field is added, `x` keeps its
declared source type, and no ordinary call is rewritten to obtain the result.
An implementation that reaches the result through `surface_var` satisfies every
functional test and is rejected.

## `D2` — the discriminators

`AC-1` … `AC-6` from the node. Split out deliberately: the Architect's
*"depth-three alone is not the closure argument"* is the standard on which
three `D6` candidates were already rejected, and a control set appended to an
implementation deliverable is the shape that produced those rejections.

**Controls, stated as the discriminating pair rather than the assertion:**

- **`AC-2`** — the deletion must red a control **arbitrarily deeper than any
  depth exercised elsewhere in the suite**, with the undeleted run as positive
  control. A control that reds at the same depth the implementation was written
  against proves the association is *present*, not that it is *load-bearing*.
- **`AC-4`** — a source program naming a generated support identifier must
  **fail to resolve**. This one passes vacuously if the identifier is spelled
  wrongly, so it needs a positive control that the same program resolves a real
  binding in the same position.
- **`AC-5`** — byte-for-behaviour, and the population must include a program
  where a direct, a W-style, and a nested structural field occur **together**.
  Separate programs do not exercise the interaction the spec normatively
  preserves.
- **`AC-6`** — construct the spec's own `u -> r` case: an arbitrary branch
  binding with a validated association under no named carrier, constructor,
  field, or motive. It must accept, and removing the association must reject,
  **with no new case added to spec or implementation**. An implementation that
  matches on a name passes `AC-1` … `AC-5` and fails only this row.

## What is NOT in scope

⛔ **Binding the two `seed-nested.md` rows.** That is a `KERNEL-NESTED-IND`
`D6` successor with fresh QA, Architect, and frontier-class
conformance-validator review. **No verdict from the four spent `D6` candidates
transfers to anything.**

⛔ Kernel changes. The predecessor establishes that the kernel and the
generated `All` representation need none — this is a surface and elaborator
capability. A deliverable that edits `crates/ken-kernel/` is a signal the
association derivation went wrong, not that the kernel was insufficient.

⛔ DS-9 `D3`+ itself. This node unblocks it; Foundation owns it.

## Contention check

**No file contention with the in-flight lane.** This node is
`crates/ken-elaborator/` and the surface parser; Runtime's RecursiveDescent
campaign is `crates/ken-runtime/src/cranelift_backend/`. The predecessor's
`spec/30-surface/` paths are landed and are read-only here.

## Sizing

Three deliverables against the one-hour turn (`steward.md §4b`). `D0` is an
extension of an existing struct and an existing validator at two existing call
sites; `D1` is a production plus a resolution rule; `D2` is controls only. If
`D0` runs long, the natural cut is the duplicate direction as its own turn —
it is the one piece with no existing code to extend.
