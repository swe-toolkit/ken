---
id: LANG-ATTACHED-PROOF-OWN-SCOPE-SELECTION
title: "In Strict, a scope that owns a local subject accepts any attached-proof name on it, whether or not that scope declares the proof, and leaves the reference to the flat global spelling, so a parent reaches an inline child's private `proof p for f` as `f::p` while a sibling module refuses it. Select own-scope attached proofs only from the scope's declared proofs, so a child's private proof is invisible to its parent as spec 33 §8.2 requires"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary evt_4848abp5ht6j7 in the blast radius of LANG-ATTACHED-PROOF-CHECKED-SELECTION-recut (squash 93b3d40b9); the mechanism re-read by the Steward on main 8da54fbc1. A visibility violation in Strict, the one mode after LANG-DELETE-LEGACY-RESOLUTION (operator 2026-09-25). Steward-filed per COORDINATION section 2."
---

# A child's private attached proof is invisible to its parent

## Objective

In Strict, a reference `s::p` or `(proof p for s)` selects an attached
proof the referring scope may see: its own declared proof, an exported
proof through its subject's provider, or an enclosing proof by checked
identity. A proof declared only in an inline child is not one of these,
and the parent's reference refuses as the sibling's does.

## Settled inputs (main `8da54fbc1`)

- **The contract.** Spec 33 §8.2 (`33-declarations.md:1418`): attached
  proofs are private by default, like other declarations. §4.1: a private
  name is invisible outside its module.
- **The mechanism.** `resolve_attached_ref` (`modules.rs:1755`) returns
  `(canonical_subject::proof_name, None)` when
  `subject_is_local || scope.local_attached_proofs.contains(&selected)`
  (`:1776`). The first disjunct admits any proof name on a local subject.
  With no checked ID, the reference is resolved by its flat spelling, and
  a child's proof on the parent's `f` was minted at that same spelling.
- **The population is pre-collected.** `local_attached_proofs` is filled
  for the whole scope before imports and bodies (`:5625`), so a
  same-scope proof declared later is in the set without the disjunct.
- **The defect.** With `fn f` at the root and `module Child { proof p for
  f ... = Refl }`, a root `theorem after ... = f::p x` accepts and mentions
  Child's proof; the `(proof p for f) x` form does too. The same reference
  from a sibling module refuses `UnboundName "f::p"`. It predates
  `93b3d40b9`.
- **The pin that fixes the leak's shape.**
  `lang_attached_proof_checked_selection::child_local_subject_without_proof_does_not_borrow_enclosing_proof`
  asserts `UnresolvedCon "Child.f::p"`, the diagnostic the same spelling
  fallthrough produces. The Adversary measured it red when only the
  disjunct is deleted.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The own-scope arm of `resolve_attached_ref` and, if AC-0 rules it, the
matching arm of the qualified-path route (`:1710`), with rows in
`lang_attached_proof_checked_selection.rs`. No change to the enclosing or
imported selection routes, to attached-proof export, or to kernel or
trust.

## Deliverable

One candidate on the Language ring.

## Acceptance

- **AC-0 (Architect, before code).** Rule the repair: delete the
  `subject_is_local` disjunct, or state the reachable case it serves and
  the narrower predicate. Rule the diagnostic both forms give under §4.1
  (`UnboundName`, the sibling's class, unless the spec names another), and
  the re-key of the existing pin to it.
- **AC-1.** The root reference to a child's private proof refuses in both
  forms with the sibling's diagnostic. The sibling control still refuses.
  A `pub proof` on an exported subject, a same-scope proof declared after
  its use, and the merged enclosing rows keep their verdicts.
- **AC-2 (QA mutation).** Restoring the disjunct reddens the AC-1 parent
  row and no other row passes in its place. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair changes a verdict in a catalog file or conformance row.
- It needs a change to attached-proof export or to the enclosing route.

Checks: 2 fired. I read `resolve_attached_ref` on `8da54fbc1`: the own-scope
arm at `:1776` is the only arm that returns no checked ID for a non-declared
proof; the enclosing arm selects by checked identity. 8 fired. AC-2's
mutant is the reported mechanism; the merged rows have no
parent-reads-child source, which is why the leak passed them. 9 fired. Every anchor was read on current main,
not the Adversary's worktree.
