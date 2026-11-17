---
id: LANG-INLINE-MODULE-IMPORT-SHADOW-POSITION
title: "Spec 33 §3.3 now says a selective import shadows a farther enclosing local throughout its inline scope and that scope's descendants, but a child module's enclosing layer is snapshotted at the child's textual position, before any import written after it is applied, so moving the import line below the child flips the child from the required UnboundName to silently binding the root's same-spelled declaration. Make the shadow position-independent, or refuse the ordering that defeats it, so one spelling has one meaning across the scope"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary evt_23grgjqw0zsnp on LANG-INLINE-MODULE-IMPORT-SHADOWS-LAYER squash b3aa83d7d, mechanism re-read by the Steward at b3aa83d7d. A silent wrong binding in Strict, the one mode after LANG-DELETE-LEGACY-RESOLUTION. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# An import shadows its whole scope, wherever it is written

## Objective

In Strict, whether an inline module's selective import shadows an
enclosing local for a descendant does not depend on where the import line
sits among the module's declarations and children. Either the shadow
holds for every descendant, or the ordering that would defeat it is
refused with a diagnostic. No source accepts with a descendant bound to
the farther local.

## Settled inputs (main `b3aa83d7d`)

- **The contract.** Spec 33 §3.3 (`33-declarations.md:323`-`:326`): a
  selective import in an inline scope shadows any farther enclosing local
  of that spelling throughout the scope and its inline descendants.
- **The mechanism.** A child's layer is taken by `snapshot_enclosing()`
  at the child's position (`modules.rs:6005`). Its shadow entries come
  from `self.bindings` at that moment (`:1159`). Imports are applied in
  textual order (`:5967`-`:5973`), on purpose, because `import M` needs
  sibling `M` already expanded. So an import written after a child is
  absent from that child's layer.
- **The defect.** With the merged rows' provider and root `helper`,
  `module Mid { module G { ... helper b } import Provider (helper) ... }`
  accepts, binding `G`'s `helper` to the root declaration. The same source
  with the import first refuses UnboundName, which is the merged row
  `grandchild_stops_at_parent_import_shadow_and_parent_keeps_provider_id`.
  A sibling child after the import refuses, so two siblings give one
  spelling two meanings (Adversary `evt_23grgjqw0zsnp` a1, a2; renamed
  import b2).
- **The same-scope arm.** Within `Mid` itself, a reference before the
  import binds the root `helper` and one after it binds
  `Provider.helper` (b1). This predates the shadow WP.
- **Why the rows missed it.** All six merged rows write the import first.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 ruling (2026-11-18)

Architect D0 `evt_6at1av8196w20`: pre-collect each scope's selective-import
spellings (`rename`, else `name`) into a scope set before its ordered pass.
`snapshot_enclosing` adds them as Shadow entries, and `checked_enclosing`
returns early on them. No import position is refused, and imports still
apply in textual order. The same-scope arm (b1) is in scope and closed by
the same set: a use before the import refuses UnboundName. The patch is
`/workspaces/ken/tmp/arch-shadowpos.patch` (sha256 `215f8d4dcf914841...`,
`modules.rs` only, +20 -1, applies 3-way onto the ATTACHED child). The D0
replaces AC-1 and AC-2 with its rows R1 to R6, mutations M1 and M2, and
arms H1 to H4. The census finds no verdict change outside the new rows.
One §3.3 sentence is owed: position-independence, worded by the Spec
enclave, joining this candidate as a Spec half. Whether an import's
binding is visible before its line is a Spec Leader question, not owed
here.

## Scope

The shadow population or an ordering refusal in `modules.rs`, rows in
`lang_inline_module_enclosing_scope.rs`, and a spec 33 §3.3 sentence if
AC-0 rules one owed. No change to import resolution order for module
availability, to what a child inherits, or to kernel or trust.

## Deliverable

One candidate on the Language ring, after
`LANG-ATTACHED-PROOF-CHECKED-SELECTION`, with a Spec enclave half if
AC-0 rules spec text owed.

## Acceptance

- **AC-0 (Architect, before code).** Rule the repair: pre-collect each
  scope's selective-import spellings as shadows before expanding its
  children, or refuse an import that follows a declaration or child in an
  inline scope. Rule whether the same-scope arm (b1) is in scope, and
  census the rows and catalog files whose verdict changes.
- **AC-1.** The a1 and b2 sources, the import after the child, give `G`
  the same verdict as the import-first row. The a2 source gives both
  siblings the same verdict.
- **AC-2.** The merged six rows keep their verdicts, and an import-first
  source with no shadowed spelling still binds the root local.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs a change to module-availability ordering, to what a
  child inherits, or to the kernel or trust surface.
- A catalog or conformance row changes verdict outside AC-0's census.

Checks: 2 fired. I read `modules.rs` at `b3aa83d7d`: the snapshot at
`:6005` is the only shadow population site for a child, and imports apply
in textual order at `:5967`-`:5973`, so an import after the child cannot
reach it. 8 fired. AC-1's after-the-child source accepts with the root
`helper` on the unrepaired code and must not on the repair; the merged
import-first rows cannot see this axis.
