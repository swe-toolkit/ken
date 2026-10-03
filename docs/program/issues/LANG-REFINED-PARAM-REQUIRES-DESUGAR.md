---
id: LANG-REFINED-PARAM-REQUIRES-DESUGAR
title: "A refined parameter lowers to its bare carrier, so its predicate is neither available in the body nor carried in the function's type. Desugar a refined parameter, and a refined domain in a written function type, to the carrier plus a requires premise, as spec 21 now defines"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [SPEC-REFINED-PARAM-REQUIRES-DESUGAR, KERNEL-INT-DIV-MOD-NATIVE]
blocks: []
github: null
origin: "Operator 2026-10-03: 'approve 3(a) as a desugar to requires' (evt_2903kfxhh96c). Spec contract SPEC-REFINED-PARAM-REQUIRES-DESUGAR. Steward-filed per COORDINATION section 2."
---

# Refined parameters desugar to requires

## Objective

`fn f (n : Int) (d : {x : Int | Not (Equal Int x 0)}) : Int = n / d`
elaborates exactly as the same function written with `(d : Int)` and
`requires Not (Equal Int d 0)`: the body discharges the divisor obligation
from the premise, callers owe it, and `f` is refused where a plain
`Int → Int → Int` is expected.

## Settled inputs

- The contract is the landed `SPEC-REFINED-PARAM-REQUIRES-DESUGAR` (spec 21
  §6.3, 22 §3). The `requires` path is landed: KERNEL-INT-DIV-MOD-NATIVE's
  assumption recognition discharges a direct `requires` premise by
  conversion, so the desugar needs no new recognition code.
- As built, the refined divisor gives 1 obligation (KERNEL-INT-DIV-MOD-NATIVE
  AC-1b). That row flips to 0 here.
- Consumers (check 3, perishable): `Derived.ken.md:1924`
  `true_refinement_project`, used as an instance field at `:1927`;
  `library/guide/surface-reference.ken.md:217`; `v2_acceptance.rs`;
  `cat3_collections_package.rs:283`; `seed-obligations.md` refined rows.

## Deliverable

1. The elaborator desugars refined parameters and written refined function
   domains to the carrier plus a Π premise, in the spec's clause order.
2. KERNEL-INT-DIV-MOD-NATIVE's refined-divisor rows (`/` and `%`) assert 0
   obligations; the caller pair (`g = f 1 0`) keeps 1 open obligation in
   `g`.
3. Consumers migrate; the refined-parameter conformance rows pass.

## Acceptance

- **AC-1.** Each refined-parameter function and its `requires` twin give
  the same obligation counts and the same core type. The higher-order row
  is refused. Catalog card verdict parity holds.
- **AC-2 (falsifiers).** M1: lower the refined parameter to its bare
  carrier (the old path); the twin-equality and higher-order pins redden.

## Stop conditions

- `true_refinement_project`'s instance field, or any other consumer, needs
  more than its type to follow the desugar: stop to the Architect.
