---
id: LANG-INLINE-MODULE-ENCLOSING-SCOPE
title: "Under Strict an inline child module builds its scope fresh and sees none of its enclosing module's declarations, so a root-local class used inside module N { .. } resolves only through Legacy's flat-global passthrough. Spec 33 now rules lexical visibility (Spec evt_318gqrbkefpmt). Make an inline child resolve its enclosing module's own declarations to their checked identities as they stand at the child's position, without inheriting imports, so LANG-DELETE I4 can migrate the held rows; forward references across boundaries are LANG-SCOPE-BOUNDARY-DEPENDENCY-ORDER"
status: merged
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-DELETE-LEGACY-RESOLUTION]
github: null
origin: "Architect evt_4em1d5vqwab2p and design note evt_13ms2ywx4pta0 on LANG-DELETE I1b's P1 Ord stop; Spec contract evt_318gqrbkefpmt (option (i), lexical). Steward-filed per COORDINATION section 2."
---

# An inline child sees its enclosing module

## Objective

Under Strict, the body of an inline `module N { .. }` resolves the bare
names of its enclosing module's own declarations to the enclosing unit's
checked identities, for every enclosing declaration checked before the
child's position. It does not inherit the enclosing module's imports.
Forward references across module, class and import boundaries are
`LANG-SCOPE-BOUNDARY-DEPENDENCY-ORDER`, not this node.

## Settled inputs (main `8cf27bd2c`)

- **The contract.** Spec evt_318gqrbkefpmt: visibility is lexical;
  forward references follow §8.4; scopes compose outward; enclosing
  imports are not inherited; resolution goes to the enclosing unit's
  checked identity. Its conformance case is an input to this node.
- **The defect.** `expand_scope`'s ModuleDecl arm (`modules.rs` :5848-5857)
  builds the child with `Scope::with_mode(scope.mode,
  scope.kernel_names.clone())` and copies only `private_ids`.
- **What to inherit.** Only the enclosing module's own declarations:
  Scope's `locals` and `checked_local_ids` (`modules.rs` :1002-1065).
  `bindings`, `prefixes`, `qualified_ids` and `binding_ids` also hold
  imports and aliases, so they are not inherited.
- **The held rows.** The 16 `lang_standard_infix_call_completion`
  first-hit identities and `lang_membership_operator_surface`, in
  LANG-DELETE's held ledger.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 and Spec answers (2026-11-18)

Architect D0 `evt_6h2cebbwrsmbs` rules an enclosing layer, snapshotted at
the `ModuleDecl` position and chained outward, never merged into the
child's maps; a checked layer hit resolves to its checked id and an
unchecked hit refuses at the resolver. Spec Leader `evt_5bnakm62drgdt`
answers Q1 with S-b: forward references across boundaries are the
contract, and that repair is recut as `LANG-SCOPE-BOUNDARY-DEPENDENCY-ORDER`.
Q2 is SH-shadow for both a child local and a child import. So:

- **AC-2** pins backward references only (p0, p2 accept with the
  enclosing checked ids). The forward rows p1 and p3 are not pinned as
  refusals; they carry to the recut node.
- **AC-3**'s second sentence is pinned as framed, for both cases. Spec
  33 §3.3 gains one sentence saying a nearer-scope binding shadows an
  inherited enclosing local, and a clash within one scope stays
  `AmbiguousReference` (Spec enclave half, with CV review).

## Scope

`expand_scope`'s child-scope construction (the enclosing layer of the
amendment above), in `crates/ken-elaborator`; the pins; the §3.3 sentence
and the conformance case from Spec. No pre-pass and no change to the walk
order. No catalog change. The spelling-keyed
`class_env` lookups in the held rows (:728, :749) are not in scope.

## Acceptance

- **AC-0 (D0, Architect, before product code).** Measure, on main, a
  child that references an enclosing class declared after the child, and
  a second case with a function: does the current prebind give their
  checked identities when the child's body elaborates
  (`pending_local_classes` refuses an unchecked local class)? Rule
  whether the pre-pass must cover inline children, or whether child
  bodies elaborate after the enclosing signatures. Rule whether a child
  import with the spelling of an enclosing local shadows it or is an
  AmbiguousReference, routing that to Spec if spec 33 does not settle
  it.
- **AC-1.** A root-local class used bare inside an inline child resolves
  to the root's checked id, beside a same-spelled flat-global decoy
  where the harness allows one.
- **AC-2.** p0 and p2 (enclosing class and fn declared before the
  child) resolve to the enclosing checked ids. p1 and p3 are not pinned.
- **AC-3.** An import that the enclosing module has and the child does
  not refuses UnboundName in the child. The child's own locals and
  imports shadow inherited enclosing locals, and the floor stays
  unshadowable.
- **AC-4.** A grandchild sees child locals, then enclosing locals, and
  never the enclosing imports.
- **AC-5.** Under Strict, the held rows elaborate with no fixture or
  assertion edit. Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs to resolve an enclosing name by spelling through
  globals.
- The repair changes the walk order or adds a pre-pass (that is the
  recut node's work).

Checks: 5 fired: the inheritance rule is keyed on "declared by the
enclosing module", which Scope carries in `locals` and
`checked_local_ids` apart from imports. 4 fired: the forward-reference
obligation is written against `expand_scope`'s textual-position
expansion and `pending_local_classes`; AC-0 measures it before design.
8 fired: AC-3's imported-only name refuses under the repair and would be
accepted by a repair that copied `bindings`.
