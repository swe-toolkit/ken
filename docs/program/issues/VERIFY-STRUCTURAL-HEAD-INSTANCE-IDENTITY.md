---
id: VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY
title: "An instance whose head is structural (arrow, Pi, Sigma, Univ, Trunc, Proj) gets no synthesized dictionary identity, so two such instances of one class share the declaration symbol {class}_instance_->: the second overwrites globals, the first falls back to {owner}#{n}, and their owner and obligation ids stay order-dependent. Give a structural-head instance a canonical, injective identity"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-INSTANCE-OWNER-KEY]
blocks: []
github: null
origin: "Architect ruling evt_7ndy6vqxw51w5 on VERIFY-INSTANCE-OWNER-KEY: the structural-head population is reachable and out of that WP's scope, because fixing it moves instance declaration symbols. Steward chose a successor WP over an amendment. D0 ruled evt_407cerfqyf2yn. Steward-filed per COORDINATION section 2."
---

# A structural-head instance has its own identity

## Objective

Two instances of one class at distinct structural heads have distinct
declaration symbols and distinct owners, and their obligation ids do not
depend on declaration order.

## Settled inputs (Architect `evt_7ndy6vqxw51w5`, at `e1b609597`)

- `named_type_head` (`modules.rs:3258-3270`) returns `None` for arrow, Pi,
  Sigma, Univ, Trunc and Proj heads, so `synthesized_dictionary_name`
  (`:3284`) gives no canonical identity.
- **Repro.** `class Lbl carrier { label : String }`,
  `instance Lbl (Nat -> Nat) { label = "x" }` and
  `instance Lbl (Bool -> Bool) { label = "y" }` check at rc=0. Both
  declaration symbols are `Lbl_instance_->`: the second overwrites
  `globals` and the first falls back to `{owner}#{n}`.
- Their core keys are distinct `Structural` keys (`instance_head_key`,
  `elab.rs:15318`).
- After `VERIFY-INSTANCE-OWNER-KEY`, named-head instances and `derive` take
  the owner from `synthesized_dictionary_name`. Structural heads keep the
  class owner.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

As ruled at D0 (Architect `evt_407cerfqyf2yn`, exact code there): one
function in `elab.rs`, `structural_instance_symbol(class, head)`, renders
`{class}_instance_` plus the resolved head as a tagged prefix tree. It is
`None` for named heads and for refused forms. Both consumers call it on
the same `rdecl`: the elab globals key, with a guard that refuses a reused
symbol for a different head, and the owner and `result.name` in the
`modules.rs` else-arm. Structural heads keep
`DeclNamespaceEffect::ReferenceOnly`, so no importable binding is minted
and `synthesized_dictionary_name` is unchanged. List every pinned hash or
symbol that moves, with the reason for each.

## Acceptance

- **AC-1.** The repro gives two distinct declaration symbols, neither a
  `#n` fallback, in both declaration orders.
- **AC-2.** The same pair with a `requires` hole in each method gives two
  distinct obligation ids and two metadata entries, in both orders.
- **AC-3 (control).** Named-head and `derive` rows of
  `verify_instance_owner_key` keep their results. The retired sentinel
  `structural_head_instances_keep_their_existing_symbols` is replaced by
  the AC-1 and AC-2 rows. Named-head collisions are
  `VERIFY-NAMED-HEAD-INSTANCE-IDENTITY`, not this WP.
- **AC-4 (mutation, QA).** Removing `RType::RArr(..)` from the top-level
  match reddens AC-1 and AC-2.

## Stop conditions

- A pinned hash or symbol moves for a reason other than structural-head
  identity.
- Any kernel, `trusted_base()` or spec change.

## Closeout

Merged `b5ba6619c` from exact `61fe43409` (PR #4603). Verify QA
`evt_5nx7hkwq346yb`, Architect `evt_7yz2sn8xbbd89`, Decision
`dec_4g2btvmwaxcg`. A structural-head instance with no synthesized
dictionary name gets a tagged canonical owner from
`structural_instance_symbol`, applied to its owner and `result.name` in
`expand_scope` and `elab_instance_decl`. The rows in
`verify_instance_owner_key.rs` pin distinct symbols, owners and obligation
ids in both declaration orders.
