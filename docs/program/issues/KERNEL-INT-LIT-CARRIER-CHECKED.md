---
id: KERNEL-INT-LIT-CARRIER-CHECKED
title: "Code outside the kernel can make every IntLit check at any type, because GlobalEnv::register_int_lit_type is pub and unchecked. Register the Int literal carrier through a checked kernel entry point, as the String and Char carriers already are, and make the raw setter crate-private"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-ENV-RAW-INSTALL-CRATE-PRIVATE]
blocks: []
github: null
origin: "Architect carry evt_3dj22arjesbg4 on KERNEL-ENV-RAW-INSTALL-CRATE-PRIVATE (the same latent class: a public kernel mutator that installs trusted state without a check). Placed after KERNEL-OBS-NESTED-CAST-LINEAR on the kernel ring. Steward-filed per COORDINATION section 2."
---

# A checked Int literal carrier

## Objective

Only a checked kernel entry point can set the type that `Term::IntLit`
inhabits. It refuses a registrant that is not a live opaque primitive type,
and it refuses a second registration.

## Settled inputs (read at `f76381399`)

- **The hole.** `GlobalEnv::register_int_lit_type` (`env.rs:763`) is `pub`
  and sets `int_lit_ty` with no check. `infer` types every `IntLit` at the
  registered id (`check.rs:234`). An external caller could register an empty
  type's id, and every Int literal would then check at it.
- **Latent, like `RAW-INSTALL`'s.** Production registers correctly:
  `numbers.rs:414` passes `Int`, declared by `reg_ty!` as a `Primitive`
  with `PrimReduction::OpaqueType` at `Type 0` (`numbers.rs:280-290`).
- **The precedent.** `register_checked_string_carrier` (`check.rs:1324`)
  refuses unless the id is a live `OpaqueType` primitive and no carrier is
  registered yet. `register_checked_char_carrier` (`:1345`) reads the Int
  carrier as its own premise.
- **Callers of the raw setter:**
  - production: `numbers.rs:414`;
  - kernel-internal test: `conv.rs:3540`;
  - integration tests: `ken-kernel/tests/k3_literal_char_registration.rs:40`,
    `ds6b_intlit_eq_reduction.rs:66`, and
    `ken-interp/tests/ds6b_intlit_eq_reduction_cross_layer.rs:46`.
  - `ds6b_intlit_eq_reduction.rs:90` pins that `infer` fails closed with no
    registration, and it stays.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- A checked `check::` entry point registers the Int literal carrier on the
  String-carrier precedent.
- `register_int_lit_type` becomes `pub(crate)`. Every external caller
  migrates to the checked entry point.

## Acceptance

- **AC-1.**
  - A compile_fail doctest pins that external code cannot name the raw
    setter.
  - All callers migrate, and the kernel and elaborator all-targets checks
    pass.
  - `trusted_base()` is unchanged.
- **AC-2 (controls).**
  - Each of the following refuses and leaves `int_lit_type()` unchanged:
    - a transparent definition's id;
    - an inductive former's id;
    - a primitive whose reduction is not `OpaqueType`;
    - a second registration after a valid one.
  - Removing each check in turn reddens its own row.
  - The valid `Int` registration succeeds, and the `IntLit` suites stay
    green (`ds6b_intlit_eq_reduction`, the cross-layer ds6b test,
    `k3_literal_char_registration`).

## Stop conditions

- A production registrant that the checked entry point would refuse: stop
  to the Architect with the registrant as found.
- Any change to what the kernel accepts for a correctly registered carrier,
  or any spec change: stop to the Architect.
