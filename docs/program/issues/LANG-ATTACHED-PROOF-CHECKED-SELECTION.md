---
id: LANG-ATTACHED-PROOF-CHECKED-SELECTION
title: "Spec 33 §8.2 resolves an attached proof's subject first and then selects the proof on it, but the elaborator selects a local subject's proof by canonical spelling, with no checked identity, so a Strict inline child cannot select an enclosing subject's attached proof and refuses UnboundName. Select attached proofs by checked identity, keyed on the resolved subject's identity and the proof name, for root-local, inherited and imported subjects alike, so a child sees an enclosing proof checked before its position, as flat elaboration would"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-INLINE-MODULE-ENCLOSING-SCOPE]
blocks: [LANG-DELETE-LEGACY-RESOLUTION]
github: null
origin: "Architect evt_2qff80j4mc920 on Language QA's D0-boundary question evt_4336pz4yymg94, LANG-INLINE-MODULE-ENCLOSING-SCOPE candidate 90ab2baaf: the inherited attached-proof refusal is a residual against spec 33 §3.1, closed by one successor node. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# Attached proofs are selected by checked identity

## Objective

In Strict, `s::p` and `proof p for s` select the attached proof `p` of
the resolved subject `s` by checked identity, keyed on the pair (the
subject's checked identity, `p`). One mechanism serves a root-local
subject, a subject inherited from an enclosing layer, and an imported
subject. A child selects an enclosing subject's attached proof only if
that proof is checked before the child's position. No layer hit is
resolved by spelling.

## Settled inputs (main `51bc0bf94`, the INLINE-MODULE squash)

Re-anchored at release on the landed squash: each `modules.rs` line moved
by +2 from the candidate, and no other anchor moved.

- **The selection today.** `resolve_attached_ref` (`modules.rs:1588`
  to `:1627`) admits a selector only through the child's own
  `qualified_ids` or `local_attached_proofs` (`:1091`, a set of
  spellings). For a local subject it returns `(canonical, None)`
  (`:1609`-`:1610`), and the rewrite turns `None` into `RCon(name)`
  (`:3994`-`:3997`). That is a lookup by canonical spelling, with no
  checked identity.
- **The second route.** `resolve_ref`'s `::` branch (`:1335`-`:1352`)
  falls through to Strict UnboundName, because `checked_enclosing`
  (`:1154`) returns `None` for any spelling containing `::`
  (`:1161`-`:1163`).
- **Why the layer cannot inherit the root path.** Inheriting the
  spelling path would make a layer hit reach `globals[canonical]`, which
  the INLINE-MODULE D0 Ruling 2 forbids (Architect `evt_6h2cebbwrsmbs`).
- **The contract.** Spec 33 §8.2 (`spec/30-surface/33-declarations.md`
  `:1365`): the subject resolves first, attached lookup runs after, and a
  bare `p` never resolves to an attached proof. Duplicate proof names on
  the same resolved subject reject. §3.1's flat equivalence makes an
  enclosing proof checked before the child's position visible to it.
- **The consumer.** `es3_modules_acceptance.rs:344`
  (`parameterized_abstract_export_preserves_kind_and_owner_proof`): child
  `M` uses the enclosing root's `list_append::assoc`. It runs Legacy
  today (`ElabEnv::new`) and refuses once LANG-DELETE I4 makes Strict the
  only mode. That is why this node blocks LANG-DELETE.
- **The declaration path.** `resolve_raw_scoped_decl` (`:5036`, around
  `:5043`-`:5055`) names a child's `proof q for f`, with `f` inherited,
  `"<f's canonical>::q"`. Language QA's D/C measurement on `90ab2baaf`
  (Architect `evt_2qff80j4mc920`, arms A1 to A3) records whether that
  path rejects a duplicate across the child boundary. Under arm A3 the
  child-declared path joins this node's scope.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 ruling (2026-11-18)

Architect D0 `evt_6vtz5z879681w`: a scope field keyed on (subject
GlobalId, proof name), snapshotted into the enclosing layer with its own
`find_attached`, and read by one helper from both reference forms. The
spelling-keyed `local_attached_proofs` route is kept for own-scope
references only. The D0's exact AC-1, AC-1b, AC-2, AC-3 (a Strict twin of
`es3:344`) and the mutant replace those below, under arms B1 to B5. The
Spec half is one §8.2 sentence, child `ea85625d4`, approved by the Spec
Leader and CV. The census finds es3:344 the only changed verdict.

## Amendment: conformance half (2026-11-18)

CV `/spec` HOLD `evt_519ta851cf9jr` on candidate `4240d2e18`: the §8.2
sentence has no conformance discriminator for inline-child selection by
checked subject identity or for the enclosing-proof position boundary.
Steward ruling: the discriminator joins this candidate as a CV-authored
conformance child, not a successor, because the sentence and the case
that makes it testable land together. The CV authors it on the
candidate's base, from the D0's AC-1, AC-1b and AC-X shapes, and the
Spec Leader reviews it. The Language Leader assembles it, and every
assembled-tip vote is fresh. No crates or spec text changes.

## Scope

The attached-proof selection record and its lookup, for the reference
path and, under arm A3, the child-declared path. The §3.3 or §8.2
sentence in a Spec enclave co-turn if the D0 finds one owed. No change
to subject resolution, to the walk order, or to any kernel or trust
surface.

## Deliverable

One candidate on the Language ring, with a Spec enclave half only if
AC-0 rules spec text owed.

## Acceptance

- **AC-0 (D0, Architect, before code).** On the landed INLINE-MODULE
  base, rule the record's key and where it is populated, how the root,
  layer and import routes read it, and whether the root's
  spelling-keyed `local_attached_proofs` path is replaced or kept as a
  fallback. Census the `crates/*/tests` and catalog rows whose verdict
  changes.
- **AC-1.** In a Strict child, `f::p` and `proof p for f`, with `f` and
  `p` enclosing and `p` checked before the child, resolve to `p`'s
  checked identity. The same reference to a proof declared after the
  child refuses.
- **AC-2.** A child that declares its own `f`, shadowing the enclosing
  one and with no proof `p`, refuses `f::p` instead of selecting the
  enclosing `f`'s `p`. The spelling `f::p` is the same in both, so this
  shows the lookup is keyed on the resolved subject's identity, not on
  spelling.
- **AC-3.** `es3_modules_acceptance.rs:344` passes under Strict with no
  assertion edit.
- **AC-X.** This node lands after
  `LANG-INLINE-MODULE-IMPORT-SHADOWS-LAYER` and carries its interaction
  row (Architect `evt_5zz90zrhkemdg`): the root's own `f` with `proof idp
  for f`, and `module Mid { import Provider (f); module G { ... (proof idp
  for f) ... } }`, refuses UnboundName under Strict, because the subject is
  shadowed at `Mid` and resolves first. The row asserts the variant only.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs a kernel or trust change, or a change to subject
  resolution.
- A child-declared proof on an inherited subject takes the same flat
  canonical key as an enclosing one (arm A2). That stop goes to the
  Architect.

Checks: 3 fired. The Architect's grep of `crates/*/tests` and catalog at
`origin/main` found one inline-module consumer of an enclosing attached
proof, `es3:344`, and I read it at `origin/main`: child `M` applies the
root-local `list_append::assoc`. 8 fired. AC-2's shadowing child separates
selection by identity from selection by spelling, and AC-1's later-proof
row separates position from mere presence. 10 fired. The root path's
`RCon` rewrite is why this node keys on checked identity, not spelling.
