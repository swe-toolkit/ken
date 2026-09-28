---
id: RT-RESPONSE-DESCENT-NESTED-COMPLETENESS
title: "Host-response descent applies its completeness rule at every dispatch level: a nested dispatch case whose branch has an unrelated exit refuses, as the top level already does, instead of selecting the one leaf it reached"
status: ready
owner: runtime
size: S
gate: architect
tier: T1
depends_on: [RT-IGNORED-ROWS-NEXT-GROUP]
blocks: []
github: null
origin: "Adversary finding 2026-09-28 on M8 68be6686d (evt_5xwyfjkdxzf7s): a latent over-admission in RT-IGNORED-ROWS-NEXT-GROUP's ruled descent (Architect evt_1y9rpazn2sj6j). Steward-filed per COORDINATION section 2."
---

# Nested completeness in the response descent

## Objective

The multi-candidate host-response descent admits a route only when every
reached tail exit, at every dispatch level, finds a dispatch entry.

## Settled inputs (Adversary `evt_5xwyfjkdxzf7s`, at `68be6686d`)

- **The rule** is stated at
  `crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs:1421`:
  every reached tail exit must find an entry. The top level applies it
  (`:1533`, `if !complete || entries.len() != 1` refuses).
- **The nested level drops it.** `response_dispatch_leaves` calls
  `response_dispatch_entries(plan, body, 0, &mut next)?` at `:1494` and
  discards the returned `complete`. The Steward read this at `68be6686d`.
- **Repro** (planner-level unit rows in the style of `responses.rs`
  `mod tests`, reverted by the Adversary). The CM Vis case has two argument
  binders and recursive positions `[1]`, and there are two `Allocate`
  leaves.
  - Top level: `If c (Match Var(1) {..}) (Match Var(2) {..})` refuses with
    the "no structural path to exactly one of them" message.
  - The same `If` one level down, under `Match Var(1) { InL p => .. }`, is
    admitted. It selects the leaf at call origin 11, while the else leaf, at
    origin 22 with scrutinee `Var(3)`, is never paired.
- **Unmeasured:** whether any source or generated program reaches the nested
  shape. No native divergence is claimed.

## Deliverable

The nested level refuses whenever the top level would, on the same shape.
The Adversary's proposed shape, for the Architect to rule: propagate
`complete` out of `response_dispatch_leaves`, and return `None` when any
level is incomplete.

## Acceptance

- **AC-0 (ruling).** The Architect rules the repair shape before any edit.
  The census of every caller of `response_dispatch_entries` and
  `response_dispatch_leaves` goes into the ruling request (Check 7).
- **AC-1.** The Adversary's nested row refuses with the top-level message,
  and its top-level row still refuses. Both are committed as planner unit
  rows.
- **AC-2 (control).** Reverting the repair re-admits the nested row. The
  landed `ambiguous_vis_in_call_argument_has_no_structural_dispatch_path`,
  `source_match_on_op_binder_selects_its_effect_leaf`,
  `rt_ignored_response_occurrence_native` and the double-bind sentinel stay
  green and unedited.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A landed native row that turns red under the repair is a stop to the
  Architect with the row and its verbatim refusal.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
