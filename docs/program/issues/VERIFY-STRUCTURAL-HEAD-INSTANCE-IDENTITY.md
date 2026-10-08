---
id: VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY
title: "An instance's dictionary identity {class}_instance_{head} is not injective: a structural head (arrow, Pi, Sigma, Univ, Trunc, Proj) gets no identity at all, and since VERIFY-INSTANCE-OWNER-KEY two distinct named-head instances, or an instance and a user declaration, can share one spelling, so one requires premise drops out of the package's obligations and the hash depends on order. Give every admitted instance a canonical, injective identity that no user declaration can spell"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-INSTANCE-OWNER-KEY]
blocks: []
github: null
origin: "Architect ruling evt_7ndy6vqxw51w5 on VERIFY-INSTANCE-OWNER-KEY (structural heads, reachable, out of that WP's scope). Widened to every head by Adversary finding evt_4n1d4dwq571kx on 06c037f92 (named-head collision, a regression of that WP). Steward-filed per COORDINATION section 2."
---

# Every instance has its own identity

## Objective

Distinct instances have distinct declaration symbols, owners and
obligation ids, independent of declaration order. No user declaration can
take an instance's identity.

## Settled inputs (re-measure on the base)

- **Structural heads** (Architect `evt_7ndy6vqxw51w5`, at `e1b609597`).
  `named_type_head` (`modules.rs:3258-3270`) returns `None` for arrow, Pi,
  Sigma, Univ, Trunc and Proj heads. `instance Lbl (Nat -> Nat)` and
  `instance Lbl (Bool -> Bool)` both admit as `Lbl_instance_->`: the second
  overwrites `globals` and the first falls back to `{owner}#{n}`. Their core
  keys are distinct `Structural` keys (`instance_head_key`,
  `elab.rs:15318`).
- **Named heads collide** (Adversary `evt_4n1d4dwq571kx`, at `06c037f92`).
  The owner is `synthesized_dictionary_name(..).canonical`
  (`modules.rs:4633-4646`), built as
  `format!("{resolved_class}_instance_{resolved_head}")` (`:3308`).
  `Scope::bind_local` accepts a re-bind of the same surface to the same
  canonical name (`:639-640`). Shared prelude: `class A`, `class
  A_instance_B`, `data B_instance_C`, `data C`, and `const need : Int
  requires Equal Int 0 1 = 0`.
  - **R1.** `instance A B_instance_C { la = \x. need }` and
    `instance A_instance_B C { lb = \x. need }` both canonicalize to
    `A_instance_B_instance_C`. One obligation results where there should be
    two, declarations are `[A_instance_B_instance_C,
    A_instance_B_instance_C#1]`, and the hash depends on order.
  - **R2.** `instance Lbl A { mark = \x. need }` with a user
    `const Lbl_instance_A : Int = need` gives one obligation and an
    order-dependent hash.
  - Pre-WP (`e5ec530dc`), every row gave two obligations.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect rules at D0 a canonical rendering of an instance's class and
head that is injective over every admitted head, named and structural, and
disjoint from every user-spellable declaration name.
`synthesized_dictionary_name` returns it, and the owner and `result.name`
use it. List every pinned hash or symbol that moves, with the reason for
each.

## Acceptance

- **AC-1.** The structural pair gives two distinct declaration symbols,
  neither a `#n` fallback, in both orders.
- **AC-2.** R1 and R2, each in one file and across two files in both
  orders, give two obligations, two declaration symbols without a `#n`
  fallback, and an order-independent hash.
- **AC-3 (control).** The predecessor's named-head and `derive` rows keep
  distinct owners and obligation ids. Any symbol or hash that moves is
  listed with its cause.
- **AC-4 (mutation, QA).** Restoring the `_instance_` concatenation
  reddens AC-2. Returning `None` for the arrow head reddens AC-1.

## Stop conditions

- A pinned hash or symbol moves for a reason other than instance identity.
- Any kernel, `trusted_base()` or spec change.
