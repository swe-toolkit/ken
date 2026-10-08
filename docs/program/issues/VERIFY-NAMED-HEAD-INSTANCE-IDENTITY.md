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

The Architect rules at D0:
- the identity of a named-head instance, injective over class and head and
  disjoint from user-spellable names;
- what happens to the importable dictionary binding when its spelling
  collides with an existing class, type or declaration name (refuse, or
  separate the binding from the identity).

The repair applies it to the globals key, owner and `result.name`.

## Acceptance

- **AC-1.** R1 and R2, each in one file and across two files in both
  orders, give two obligations, two declaration symbols without a `#n`
  fallback, and an order-independent hash. If the ruling refuses the
  collision, each row is refused with a typed error naming both
  declarations instead.
- **AC-2 (control).** The predecessor's named-head and `derive` rows and
  the structural-head rows keep distinct owners and obligation ids. List
  any symbol or hash that moves, with its cause.
- **AC-3 (mutation, QA).** Restoring the bare `_instance_` concatenation as
  the owner reddens AC-1.

## Stop conditions

- A catalog or corpus package is newly refused or changes hash: stop with
  the list.
- Any kernel, `trusted_base()` or spec change.
