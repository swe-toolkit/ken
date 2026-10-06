---
id: LANG-NAMED-REFINEMENT-TYPE-ARGUMENT
title: "A named refinement used as a type argument is δ-transparent: def Five = { x : Int | Equal Int x 5 } and const ys : List Five = Cons Int six (Nil Int) check with zero obligations, so List Five admits 6. The literal form is refused by LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION's slot guard, which sees only source RRefine. Introduce or refuse the named form"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION]
blocks: []
github: null
origin: "F-E: separate finding the Architect's TYPE-POSITION D0 ruling evt_5am0p8wy7vc9j asked to be measured; measured on c49297983 by the language implementer (evt_2tgrvgpr76kwm), boundary confirmed by the Architect (evt_19x6v93jytdy4). Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# A named refinement in a type argument is introduced or refused

## Objective

A named refinement written as a type-constructor argument either
introduces its predicate on every value that inhabits that position (spec
34 §5) or is refused, exactly as its literal spelling is.

## Settled inputs (`evt_2tgrvgpr76kwm`, Architect `evt_19x6v93jytdy4`, on `c49297983`)

- **The row.** `const six : Int = 6; def Five = { x : Int | Equal Int x 5 };
  const ys : List Five = Cons Int six (Nil Int)` checks, exit 0, with zero
  obligations. It gives 0 on the TYPE-POSITION candidate too, so that WP
  does not close it.
- **Mechanism.** `emit_refinement_introduction` (`elab.rs:11538`) finds a
  named predicate only when `expected` zonks to a bare `Term::Const`
  (`:11546`). `List Five` is an `App`, so none is found, and conversion
  accepts `List Int` as `List Five` because `Five` δ-unfolds.
- **The leak spreads.** The same-root reuse exemption (`:11549-11553`)
  assumes every value typed `Five` was introduced with an obligation. An
  element taken out of `ys` has type `Five` without one, so passing it to
  any `x : Five` parameter is also accepted silently.
- **Two routes to a nested `Five`.** (a) Written in source: `List Five`, an
  alias whose body is `List Five`, or a type-level function applied to
  `Five`. TYPE-POSITION's `RefinementSlot` would cover these if the
  named-reference arm refused a `Const` with a `refinement_root` in slot
  `None`. (b) A type argument solved to `Five` by unification, for example
  `Cons _ x (Cons _ six (Nil _))` with `x : Five`; unmeasured, and whether
  it leaks depends on check and solve order.
- Kernel soundness is intact: the kernel sees `Int`, and named predicates
  are never assumed as hypotheses.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Route (b) measured: whether a meta solved to a
   named refinement reaches an element unintroduced. A census of named
   refinements under type constructors (`List`, `Option`, records) across
   `catalog/`, `library/`, `crates/*/tests`, `examples/` and
   `conformance/`. The Architect rules introduce or refuse per route.
2. **The ruled closure.**

## Acceptance

- **AC-1.** The `List Five` row with `six` emits its obligation, or is
  refused with a message naming the type argument, per the ruling. Its
  `5` twin checks. An element of `ys` passed to an `x : Five` parameter is
  no longer accepted without an obligation.
- **AC-2 (controls).** Named refinements at binder, field, result and
  value-call positions keep their current obligations; the 81-file corpus
  census shows no newly refused or undischargeable program beyond what the
  ruling names.
- **AC-3 (mutation, QA).** Removing the closure returns the row to zero
  obligations.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A catalog, library or example program that relies on the transparent
  behaviour.
