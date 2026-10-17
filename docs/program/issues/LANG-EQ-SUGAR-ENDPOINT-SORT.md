---
id: LANG-EQ-SUGAR-ENDPOINT-SORT
title: "A declaration with an Eq type-position binder over an unannotated Type, such as (a : Type) (h : Eq Type a a), is refused at its obligation with Type u1 vs Type u0, and a Type 1 carrier is refused outright: elab_type's SUGAR_EQ arm builds Term::Eq unchecked, so the endpoints' sort is never recorded against the carrier. Record it, zonking the carrier without defaulting"
status: active
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-TYPE-POSITION-ARGUMENT-SORT]
blocks: []
github: null
origin: "Adversary M8 finding F1 on 1bf4bc0de (evt_3fhse1gr94f04); Architect ruling evt_13wfdyg5vj9vz: real and wider, S, one branch plus one helper, queued behind LANG-SOLVED-INDEX-SUBSTITUTION. Steward-filed per COORDINATION section 2."
---

# An Eq type's endpoints have the carrier's sort

## Objective

A declaration whose `Eq` binder relates unannotated `Type` endpoints
checks and reports its obligation, as its `Type 0`-annotated twin does.

## Settled inputs (Architect `evt_13wfdyg5vj9vz`, on `40fdd16f8`)

- **The refusals.** These nine valid rows are KernelRejected:
  - the Adversary's five: `Eq (Type 0) a a` and `Eq Type a a` with `use5`,
    `need` and `n / d`, and `Eq Type a b`;
  - `Eq Type a a` beside a free `b : Type`;
  - `Eq (Type -> Type) f f`;
  - `Eq Type a Int`;
  - `Type 0` and `Type 1` carriers in one declaration, which is refused
    over the whole declaration because the final zonk defaults the carrier
    meta to Zero.
- **Mechanism.** The SUGAR_EQ arm of `elab_type` (elab.rs:1534-1540)
  builds `Term::Eq` without relating the endpoints' sort to the carrier.
  It is the only raw `elab_type` builder that relates one written type's
  sort to another's and does not record it, per the Architect's sweep of
  every arm.
- **Ruled code.** The ruling's text is the code:
  - after `let b_k = elab_type(cx, args[2])?;`, call
    `unify_eq_endpoint_sort(cx, &a_ty_k, &a_k)` and the same with `&b_k`;
  - add the new helper `unify_eq_endpoint_sort` after
    `unify_type_position_argument_sort`. It reads the endpoint's type with
    `kernel_infer_in_context_open`, returns early on failure, zonks the
    carrier with `zonk_term_with(_, false)`, and calls `unify_types`.
  - The zonk is load-bearing. Without it, `Eq Type a Int` records
    carrier := u0 and drops carrier = 0.
- **Consumer.** The catalog `Core/Logic/Transport` `cast` elaborates
  byte-identically with and without the repair.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ruled code in `crates/ken-elaborator/src/elab.rs`, with the ruling's
rows in the `type_position_argument_sort_tests` shape. No kernel, trust
or spec change.

## Acceptance

- **AC-1.** The ruling's 23-row table as exact rows, each with its hole
  count, parameter count and first goal head. This includes
  `Eq Type a Int` with 0 params and the `Type 0` plus `Type 1` row.
- **AC-2.** Three negatives stay refused as `Type 0` vs `Type suc 0`:
  - `(a : Type 1) (h : Eq (Type 0) a a)`;
  - `(a : Type 1) (h : Eq Type a Int)`;
  - `(F : Type -> Type) (a : Type 1) (o : F a) (q : F Int)`.

  The Adversary's controls are unchanged, and
  `type_position_argument_sort_tests` stays 17/17. QA runs the catalog and
  `conformance/` `Eq` consumers that the diff touches.
- **AC-3 (mutation, QA).**
  - Removing both calls returns the nine rows to KernelRejected.
  - Removing only the `zonk_term_with` line turns `Eq Type a Int` red.

## Stop conditions

- A previously green elaborator, catalog or conformance row changes
  outcome: stop to the Architect.
- Any kernel, conversion or `trusted_base()` change.
