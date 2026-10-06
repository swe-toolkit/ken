---
id: KERNEL-STRING-CARRIER-CHECKED-SORT
title: "register_checked_string_carrier checks only that the id is a Primitive with OpaqueType reduction, so it admits the two shapes KERNEL-INT-LIT-CARRIER-CHECKED's respin closed for Int: a value tagged OpaqueType, whose type is not a Type, and a polymorphic primitive. Require a Type-sorted monomorphic carrier the same way"
status: merged
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-INT-LIT-CARRIER-CHECKED]
blocks: []
github: null
origin: "Architect carry evt_7z7g8kgk04z84 in the KERNEL-INT-LIT-CARRIER-CHECKED approval: the precedent that WP followed has the same latent class. Steward-filed per COORDINATION section 2."
---

# The String carrier is a monomorphic type

## Objective

`register_checked_string_carrier` refuses every registrant that
`register_checked_int_lit_carrier` refuses: anything but a live
`OpaqueType` primitive with no level parameters whose declared type
whnfs to `Type ℓ`, and a second registration.

## Settled inputs (Architect `evt_7z7g8kgk04z84`, on `a1328e1cb`)

- **The gap.** `register_checked_string_carrier` (`check.rs:1814-1829` at
  `a1328e1cb`) checks only `Decl::Primitive { reduction: OpaqueType, .. }`
  and that no carrier is registered. A value tagged `OpaqueType` and a
  polymorphic primitive both register.
- **The model.** The INT-LIT guard (`check.rs:1764-1810`): `level_params`
  is empty and `whnf(ty)` is `Term::Type(_)`, all checked before the
  setter runs.
- **Latent.** Production registers correctly: `bytes.rs:63` passes the
  prelude `String`. Other callers are tests: `conv.rs:4841`,
  `env.rs:1233`, `k3_literal_char_registration.rs:27,55`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The two added conditions in `register_checked_string_carrier`, checked
before `install_checked_string_carrier`, and refusal rows for the two
shapes.

## Acceptance

- **AC-1.** A value tagged `OpaqueType` and a polymorphic `OpaqueType`
  primitive are each refused with the existing message. Each row asserts
  that `checked_string_type()` and `trusted_base()` are unchanged.
- **AC-2 (controls).** Production String registration through
  `ElabEnv::new()` still succeeds, and the existing String and Char
  literal rows (`k3_literal_char_registration`, the `conv.rs` and
  `env.rs` tests) keep their results.
- **AC-3 (mutation, QA).** Removing either added condition lets its row
  register.

## Stop conditions

- Any change to how String literals are typed or reduced, or to
  `trusted_base()`.
- Any spec change.

## Closeout

Merged `c7b5c2b38` from exact `d18477f8c` (PR #4542). Kernel QA
`evt_q7wpey1q9ywb`, Decision `dec_3j8wd57r8sxmv`.

- `register_checked_string_carrier` now also requires empty `level_params`
  and a declared type whose whnf is `Type ℓ`, both checked before the
  carrier is installed: the guard the Int carrier gained.
- String literal typing, reduction and `trusted_base()` are unchanged.
