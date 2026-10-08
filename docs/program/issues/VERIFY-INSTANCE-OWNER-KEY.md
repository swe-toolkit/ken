---
id: VERIFY-INSTANCE-OWNER-KEY
title: "Since VERIFY-OBLIGATION-STABLE-IDENTITY an instance's unnamed declarations are owned by the class name, which every instance of the class shares, so instance literal and hole symbols {class}#{n} and the package hash depend on source order. Key each instance's owner on its own stable identity"
status: active
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-OBLIGATION-STABLE-IDENTITY]
blocks: []
github: null
origin: "Adversary finding evt_3abb6y463ghsh on fe21d61c4: AC-1(i) and AC-1(ii) of VERIFY-OBLIGATION-STABLE-IDENTITY fail for a class with two or more instances. Steward-filed per COORDINATION section 2."
---

# An instance owns its own allocations

## Objective

Reordering sources, or adding an unrelated instance of the same class,
leaves every instance's symbols, semantic entries and the package hash
unchanged (spec 46 §3.2).

## Settled inputs (Adversary, measured at `fe21d61c4`)

- The unnamed-declaration owner is `rdecl.name` (`modules.rs:3102`). For
  an instance it is the class name (`resolve.rs:1751`,
  `name: class_name.clone()`).
- Ordinals are counted per owner spelling (`lib.rs:287`,
  `owner_ordinals: HashMap<String, u32>`), so a class and all its instances
  share one counter. The symbol is `{class}#{n}` (`compiler_driver.rs:4180`).
- **Repro.** Package `zz_adv_ownerkey`, target `k`, NonRuntime. `src/c.ken`
  has `class Lbl carrier { label : String }`, `data A`, `data B` and
  `const k : Nat = Zero`. `src/a.ken` has `instance Lbl A { label = "aaa" }`
  and `src/b.ken` has `instance Lbl B { label = "bbb" }`. Orders c,a,b and
  c,b,a give different hashes. `Lbl_instance_B` references `Lbl#1` alone
  and `Lbl#3` after instance A.
- Named const, fn, proof and data owners, mutual members and prelude
  stages are already distinct. Only the instance arm shares an owner.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

An instance's owner is its own qualified stable identity, the same string
its declaration symbol is built from. The class name is not the owner.
That covers literals and `requires` holes opened inside instance methods.

## Acceptance

- **AC-1.** The repro in both orders gives identical package hash,
  semantic entries and symbols. `c,b` alone versus `c,a,b` gives an
  identical `Lbl_instance_B` entry.
- **AC-2.** A `requires` hole inside an instance method has an
  order-independent symbol and trust key under the same two reorders.
- **AC-3 (controls).** The landed `verify_obligation_stable_identity`
  rows and the mutual-member row stay green.
- **AC-4 (mutation).** Restoring the class name as the instance owner
  reddens AC-1.
- **AC-5.** Sweep every `with_owner` entry and every declaration kind's
  owner source. Name any other kind whose owner spelling is shared, and
  fix it in this WP (CHECK 7).

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A pinned hash or id that moves for any reason other than the instance
  owner key: stop with the consumers listed.
