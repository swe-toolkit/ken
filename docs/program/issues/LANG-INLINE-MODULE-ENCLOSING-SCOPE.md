---
id: LANG-INLINE-MODULE-ENCLOSING-SCOPE
title: "Under Strict an inline child module builds its scope fresh and sees none of its enclosing module's declarations, so a root-local class used inside module N { .. } resolves only through Legacy's flat-global passthrough. Spec 33 now rules lexical visibility (Spec evt_318gqrbkefpmt). Make an inline child resolve its enclosing module's own declarations to their checked identities, forward references included, without inheriting imports, so LANG-DELETE I4 can migrate the held rows"
status: ready
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
checked identities, as spec 33 rules. It does not inherit the enclosing
module's imports.

## Settled inputs (main `d681db9d6`)

- **The contract.** Spec evt_318gqrbkefpmt: visibility is lexical;
  forward references follow §8.4; scopes compose outward; enclosing
  imports are not inherited; resolution goes to the enclosing unit's
  checked identity. Its conformance case is an input to this node.
- **The defect.** `expand_scope`'s ModuleDecl arm (`modules.rs` :5757-5766)
  builds the child with `Scope::with_mode(scope.mode,
  scope.kernel_names.clone())` and copies only `private_ids`.
- **What to inherit.** Only the enclosing module's own declarations:
  Scope's `locals` and `checked_local_ids` (`modules.rs` :933-1000).
  `bindings`, `prefixes`, `qualified_ids` and `binding_ids` also hold
  imports and aliases, so they are not inherited.
- **The held rows.** The 16 `lang_standard_infix_call_completion`
  first-hit identities and `lang_membership_operator_surface`, in
  LANG-DELETE's held ledger.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`expand_scope`'s child-scope construction and any signature pre-pass the
forward case needs, in `crates/ken-elaborator`; the pins; the
conformance case from Spec. No catalog change. The spelling-keyed
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
- **AC-2.** The forward cases from AC-0 resolve as ruled.
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
- The forward case needs a change to §8.4's pre-pass contract.

Checks: 5 fired: the inheritance rule is keyed on "declared by the
enclosing module", which Scope carries in `locals` and
`checked_local_ids` apart from imports. 4 fired: the forward-reference
obligation is written against `expand_scope`'s textual-position
expansion and `pending_local_classes`; AC-0 measures it before design.
8 fired: AC-3's imported-only name refuses under the repair and would be
accepted by a repair that copied `bindings`.
