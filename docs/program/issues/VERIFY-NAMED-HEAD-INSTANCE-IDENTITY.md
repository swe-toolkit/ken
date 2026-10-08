---
id: VERIFY-NAMED-HEAD-INSTANCE-IDENTITY
title: "Since VERIFY-INSTANCE-OWNER-KEY a named-head instance's owner is its synthesized dictionary name {class}_instance_{head}, which is not injective and which a user declaration can spell, so two distinct instances (or an instance and a const) share one owner: one requires premise drops out of the package's obligations, the second falls to a #n fallback, and the hash depends on order. Give named-head instances an identity no other instance or declaration can take"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY]
blocks: []
github: null
origin: "Adversary finding evt_4n1d4dwq571kx on 06c037f92: a reporting regression of VERIFY-INSTANCE-OWNER-KEY (soundness-adjacent: a live premise is unreported). Separate from the structural-head WP, whose D0 ruling evt_407cerfqyf2yn covers structural heads only. Steward-filed per COORDINATION section 2."
---

# A named-head instance has its own identity

## Objective

Distinct named-head instances, and an instance and any user declaration,
never share a declaration symbol, owner or obligation id, in any
declaration order.

## Settled inputs (Adversary `evt_4n1d4dwq571kx`, at `06c037f92`)

- **The mechanism.** The owner is `synthesized_dictionary_name(..).canonical`
  (`modules.rs:4633-4646`), built as
  `format!("{resolved_class}_instance_{resolved_head}")` (`:3308`).
  `Scope::bind_local` accepts a re-bind of the same surface to the same
  canonical name (`:639-640`).
- **Shared prelude.** `class A carrier { la : carrier -> Int }`,
  `class A_instance_B carrier { lb : carrier -> Int }`,
  `data B_instance_C : Type where { MkBC : B_instance_C }`,
  `data C : Type where { MkC : C }`,
  `const need : Int requires Equal Int 0 1 = 0`, `const k : Nat = Zero`.
  Package `zz_adv_ownerkey`, target `k`, NonRuntime.
- **R1.** `instance A B_instance_C { la = \x. need }` and
  `instance A_instance_B C { lb = \x. need }` both give
  `A_instance_B_instance_C`. Result: one obligation
  (`obl:A_instance_B_instance_C.requires.0`) where there should be two,
  declarations `[A_instance_B_instance_C, A_instance_B_instance_C#1]`, and
  an order-dependent hash (`0x946d9d38a813e883` vs `0xf4bb8d2a1b7a530`).
  Same in one file and across two files in either order.
- **R2.** `class Lbl carrier { mark : carrier -> Int }`,
  `instance Lbl A { mark = \x. need }` and
  `const Lbl_instance_A : Int = need` give one obligation and an
  order-dependent hash.
- **Pre-WP control.** With `modules.rs` from `e5ec530dc`, every row gives
  two obligations.
- The dictionary name is also an importable surface binding, so the
  repair must keep or deliberately replace that binding.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

**D0 ruling (Architect `evt_2wbp7vr08k3pf`, at `b5ba6619c`): refuse the
collision.** The identity stays `{resolved_class}_instance_{resolved_head}`,
which is also the importable binding (10 catalog files depend on it). A
declaration whose identity key is already bound in `elab.globals` to a
different checked GlobalId is refused with a new typed
`DeclarationIdentityCollision`. The error names both declarations, sorted,
so the message does not depend on declaration order. The guard sits at the
module-path choke point `elaborate_checked_as`. The structural WP's
same-instance-key predicate is the only exemption.

1. **AC-0 (measure and report before building).**
   - For named `instance`, `derive` and `const`, the key each inserts into
     `elab.globals` equals the `owner` passed to `elaborate_checked_as`.
   - R1 and R2 refuse in all 12 layouts and orders with the guard in
     place.
   - A data constructor spelled like a dictionary
     (`data D : Type where { Lbl_instance_A : D }`), in both orders.
     Constructors enter globals by another path. Report the result; if
     it collapses too, it goes to the Steward as a residual and is not
     folded in.
2. **The guard and its typed error.**

## Acceptance

- **AC-1.** Each R1 and R2 row (one-, two- and three-file layouts, both
  orders) is refused with the typed error naming both declarations. The
  non-colliding controls give one obligation per live premise, no `#n`
  fallback and an order-independent hash.
- **AC-2 (control).** The predecessor's named-head and `derive` rows and
  the structural-head rows keep their owners and obligation ids, and no
  catalog package is refused or changes hash.
- **AC-3 (mutation, QA).** Deleting the guard reddens AC-1: R1 and R2
  admit again with one obligation, a `#1` fallback and order-dependent
  hashes.

## Stop conditions

- A catalog or corpus package is newly refused or changes hash: stop with
  the list.
- Any kernel, `trusted_base()` or spec change.
