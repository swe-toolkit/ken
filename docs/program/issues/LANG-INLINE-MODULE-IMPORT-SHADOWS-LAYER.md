---
id: LANG-INLINE-MODULE-IMPORT-SHADOWS-LAYER
title: "Spec 33 §3.3 says a child-scope import shadows an inherited enclosing local within that child, but the enclosing layer a scope hands its children records only its locals, so a grandchild's lookup walks past its parent's import and silently binds the root's same-spelled local: the same text `helper b` binds to two declarations one layer apart, with no diagnostic. Record every name a scope binds in the layer, so an import blocks the walk and the grandchild refuses UnboundName, because imports are not inherited"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary evt_7zpbe0r9dmqz9 F1 on LANG-INLINE-MODULE-ENCLOSING-SCOPE squash 51bc0bf94, mechanism re-read by the Steward at 51bc0bf94. A silent wrong binding in Strict, the one mode after LANG-DELETE-LEGACY-RESOLUTION. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# An import blocks the enclosing-layer walk

## Objective

In Strict, a name that an inline module binds by import shadows every
enclosing local of that spelling for the module's whole text, its own
children included. A grandchild that spells the name refuses
`UnboundName`, because it inherits neither the import nor the shadowed
local.

## Settled inputs (main `51bc0bf94`)

- **The defect.** With provider `Provider.ken` = `pub fn helper (b :
  Bool) : Bool = False`, the root declares `fn helper (b : Bool) : Bool =
  b` and `module Mid { import Provider (helper); pub fn m ... = helper b;
  module G { pub fn g ... = helper b } }`. It is accepted: `Mid.m` binds
  `Provider.helper` and `Mid.G.g` binds the root `helper` (Adversary
  `evt_7zpbe0r9dmqz9` F1, `elaborate_isolated_file_strict_v1`).
- **The mechanism.** `snapshot_enclosing`
  (`crates/ken-elaborator/src/modules.rs:1126`) builds the layer from
  `self.locals` only, so `Mid`'s import binding never enters it.
  `EnclosingLayer::find` (`:1020`) walks to the first layer holding the
  bare name, and `checked_enclosing` (`:1154`) checks shadowing only
  against the current scope's `bindings`. The `EnclosingLocal` comment
  (`:1005`) says imports are deliberately absent: absent as a grant, which
  is correct, but also absent as a shadow.
- **The contract.** Spec 33 §3.3 (added by `51bc0bf94`): a child-scope
  local or import shadows an inherited enclosing local within that child.
  Spec 32 §1: a child does not inherit its enclosing module's imports.
- **The controls already true.** If `Mid` declares a local `helper`, `G`
  binds `Mid`'s. With no `Mid` binding, `G` binds the root's. The landed
  row `grandchild_does_not_inherit_immediate_parent_import`
  (`lang_inline_module_enclosing_scope.rs:228`) has no root local, so it
  cannot see this.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 ruling (2026-11-18)

Architect D0 `evt_5zz90zrhkemdg`: each layer records every name its scope
binds, in one map with a tagged entry (a local as a grant, any other
binding as a shadow that stops the walk). The patch is
`/workspaces/ken/tmp/arch-shadow-d0.patch` (four hunks, `modules.rs`
only). Not folded into `LANG-ATTACHED-PROOF-CHECKED-SELECTION`; this node
lands first. No Spec co-turn. The D0 replaces AC-1 and AC-2 below with
its rows AC-1, AC-1b (renamed import), AC-1c (depth), the three AC-2
controls and the AC-3 mutant (the hunk-4 loop deleted), and its arms C1
to C4. The census finds 0 existing rows that change verdict.

## Amendment: Spec co-turn (2026-11-18)

Spec Leader `evt_276j5m0wmkf13`, on Language QA `evt_hcwr55hcj14p`: spec
33 §3.2 to §3.3 and 32 §1 do not state the barrier a non-inherited import
creates for a deeper inline descendant, so the behaviour is a Spec gap.
Steward ruling: a one-sentence §3.3 co-turn joins this candidate, from the
D0's suggested text ("within that child's text, its inline descendants
included, which inherit neither the import nor the local it shadows"),
worded by the Spec enclave. It adds no diagnostic kind. The Spec Leader
and CV review `/spec`. No seed is owed: the crate rows carry the
observation.

## Scope

The enclosing layer's record and walk in `modules.rs`, rows in
`lang_inline_module_enclosing_scope.rs`, and the one spec 33 §3.3
sentence. No change to what a child
inherits, to import resolution, or to any kernel or trust surface.

## Deliverable

One candidate on the Language ring, with a Spec enclave half for the
§3.3 sentence.

## Acceptance

- **AC-0 (Architect, before code).** Rule the layer's shadow record (for
  example a blocking entry per non-local binding) and whether it shares
  the record change that `LANG-ATTACHED-PROOF-CHECKED-SELECTION` makes to
  `EnclosingLocal`, or folds into it.
- **AC-1.** The F1 source refuses `UnboundName` at `G`'s `helper`, while
  `Mid.m` still binds `Provider.helper` by checked identity.
- **AC-2.** The two controls above pass unchanged: `Mid`'s local `helper`
  reaches `G`, and with no `Mid` binding the root `helper` reaches `G`.
  So the repair blocks only on a binding the walk passes, not on the
  layer's presence.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair changes what a child inherits, or needs import resolution,
  kernel or trust changes.
- A catalog, conformance or test row changes verdict, other than the new
  rows. That census goes to the Architect.

Checks: 2 fired. I read `snapshot_enclosing`, `EnclosingLayer::find` and
`checked_enclosing` at `51bc0bf94`: only `self.locals` populates the
layer, and the walk stops at the first layer holding the spelling, so an
import in an intermediate scope is invisible to it. 8 fired. AC-1's
refusal is produced only when the walk stops at `Mid`; the unrepaired
walk binds the root `helper` and is accepted. AC-2's controls fail if the
repair blocks on every layer.
