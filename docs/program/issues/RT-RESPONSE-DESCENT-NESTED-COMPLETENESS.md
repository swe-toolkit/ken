---
id: RT-RESPONSE-DESCENT-NESTED-COMPLETENESS
title: "Host-response descent admits a multi-candidate route only when it reaches exactly one leaf and that leaf is the only candidate its Vis case owns, so a nested dispatch whose sibling branch leaves an owned candidate unpaired refuses, as the top level already does"
status: active
owner: runtime
size: S
gate: architect
tier: T1
depends_on: [RT-IGNORED-ROWS-NEXT-GROUP]
blocks: []
github: null
origin: "Adversary finding 2026-09-28 on M8 68be6686d (evt_5xwyfjkdxzf7s): a latent over-admission in RT-IGNORED-ROWS-NEXT-GROUP's ruled descent (Architect evt_1y9rpazn2sj6j). Steward-filed per COORDINATION section 2."
---

# Candidate ownership in the response descent

## Objective

The multi-candidate host-response descent admits a route only when it
reaches exactly one leaf, and that leaf is the only candidate owned by this
Vis's case. A candidate is owned by the nearest enclosing
`ComputationalMatch` whose case at its position is `ITree::Vis`.

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
- **Source-reachable** (runtime leader `evt_2fqx849dqy550`): the checked
  two-bracket `withBuffer` witness reaches nested descent. No native
  divergence is claimed.

## Deliverable

The ownership rule above, ruled by the Architect (`evt_g30yv6w4a4zf`). A
new `response_leaf_owner` walks `response_parents`; the `:1541` return
requires `reached.len() == 1 && owned == reached`. The descent functions and
the top-level `:1533` gate are unchanged.

- **Withdrawn shapes.** The Adversary's "propagate `complete` from every
  level" and the Architect's leaf-boundary completeness
  (`evt_5d8ea6zyw9dve`). AC-0b falsified the latter: the two-bracket source
  witness has incomplete cases at non-leaf levels, with `Let` dead ends, so
  tail completeness is the wrong predicate.
- **AC-0c** measured ownership before the edit: each witness Vis has
  owned == reached of size 1, and the nested row owns {18, 29} but reaches
  {18}.

## Acceptance

- **AC-0 (reachability, then ruling).**
  - First measure whether a well-formed Ken source program reaches the
    nested shape. If only planner fixtures or hand-built IR reach it, stop:
    the Steward closes the node (operator 2026-08-29, fund only reachable
    shapes).
  - If a source program reaches it, the Architect rules the repair shape
    before any edit. The census of every caller of
    `response_dispatch_entries` and `response_dispatch_leaves` goes into the
    ruling request (Check 7).
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
