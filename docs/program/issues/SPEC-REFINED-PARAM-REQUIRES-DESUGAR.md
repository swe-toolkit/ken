---
id: SPEC-REFINED-PARAM-REQUIRES-DESUGAR
title: "Spec 21 and 22 assume a refined parameter's predicate in the body while the parameter lowers to its bare carrier, so a higher-order caller passes an unproved value and the body relies on it. Make a refined parameter a desugar to the carrier parameter plus requires, and state contract examples as propositions, not Bool expressions"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: [LANG-REFINED-PARAM-REQUIRES-DESUGAR]
github: null
origin: "Operator 2026-10-03: 'approve 3(a) as a desugar to requires' (evt_2903kfxhh96c) and 'concur with 2' (spec 21 Bool examples to Not (Equal ..), evt_6d6fzbqt2sa8r). Architect higher-order bypass evt_1ytv0fc4j1c1j; KERNEL-INT-DIV-MOD-NATIVE audit redirect evt_27wngyard37xa. Steward-filed per COORDINATION section 2."
---

# A refined parameter is a parameter plus requires

## Objective

The spec states one mechanism for a premise a function body may assume:
`requires`. A refined parameter `(x : {y:A|φ})` is defined as `(x : A)`
with `requires φ[x]`, and every contract example in spec 21 states its
premise as a proposition.

## Settled inputs

- Spec 21 §6.3 lowers a refined parameter to its carrier and adds `φ[x]` to
  Γ; 22 §3 repeats it and cites a §2.5.1 that has no heading. The core type
  is then `A → B`, so a function passed where `A → B` is expected is
  applied to an unproved value while its body assumed `φ` (Architect
  `evt_1ytv0fc4j1c1j`). This is the defect.
- `requires` already lowers soundly: a Π proof-argument at Ω, assumed in the
  body, discharged at the call (21 §6.3 `elabView`, 22 §2.3).
- `≠`, `==`, `>` and the other comparison operators are Bool-valued (33
  §6.1), so in a `requires`, `ensures` or refinement position they are not
  propositions. Spec 21 uses them at `:50`, `:79` and `:98` at least; the
  author counts every contract and refinement example in 21 and 22 (check 6:
  the population is every code block in those two files).
- As built, KERNEL-INT-DIV-MOD-NATIVE gives a refined divisor 1 obligation
  and a `requires` divisor 0. Under this spec the refined case becomes 0
  through the desugar; `LANG-REFINED-PARAM-REQUIRES-DESUGAR` builds it.

## Deliverable

1. Spec 21 §6.3: refined parameters desugar before `elabView` to the carrier
   parameter plus a `requires` clause, with the clause order stated. A
   refined domain in a written function type `(x : {y:A|φ}) → B` desugars
   the same way, to `(x : A) → (_ : φ[x]) → B`, so a higher-order position
   carries the premise. The forgetful direction `{x:A|φ} ≤ A` (§2, 22 §2.1)
   is unchanged for values.
2. Spec 22 §3: the refined-parameter bullet becomes a pointer to the
   desugar and the Preconditions bullet; the dangling §2.5.1 citation goes.
3. Spec 21, 22, 33, 34 and 38 examples (the last three added on the
   spec-leader's residual, `evt_2czwycgn39psm`): every Bool-valued operator in a premise,
   postcondition or refinement position becomes a proposition (`Not (Equal
   Int d 0)` for `d ≠ 0`, `Equal`, the ordering propositions).
4. Retired surface in the same five files (CV via spec-leader
   `evt_69gwr5whd0n39`): no example defines a protected built-in name (34
   §5's `def Nat` is renamed, 33 §3.3), and the retired `view` definition
   keyword (33 §1) is replaced by the current one in examples and in 21
   §6.1's grammar; the elaboration pseudocode follows. No other chapter.
5. Conformance: `seed-obligations.md` (`:138` and the refined-parameter
   rows) and `seed-spec-syntax.md:305` follow, with a row pinning that a
   function with a refined parameter is refused where a plain `A → B` is
   expected. A conformance path this WP already changes states its
   propositions in Ω form too, including `conformance/README.md`'s bytes
   round-trip law in 38 §1.5's `Equal` form (spec-leader
   `evt_6rhcyc8n3r8de`).
6. A test that claims a heading this WP renames follows the rename. Only
   the claim comment changes, never an assertion. M5 red on `ecfb85936`:
   `crates/ken-elaborator/tests/v2_acceptance.rs:254` claims
   `refined-param-is-hypothesis-not-obligation`, renamed
   `refined-param-desugars-to-requires`. The same sweep finds two more:
   `src/r_layer_tests/effects.rs:490` (`pure-view-usable-in-pure-context`,
   now `pure-fn-…`) and `src/r_layer_tests/acceptance.rs:66`
   (`const-elaborates-checks`, now `konst-…`). Re-run the sweep at the
   respin tip: every heading removed under `conformance/` against every
   claim in `crates/`, `scripts/` and `.github/`.

## Acceptance

- **AC-1.** The CV finds no remaining spec 21 or 22 text that assumes a
  refined parameter's predicate other than through the desugar, and no Bool
  expression in a proposition position in the examples of the five files.
- **AC-2.** The higher-order row's verdict is refusal, and its `requires`
  twin (same function written with an explicit `requires`) has the same
  verdict.

## Stop conditions

- The desugar needs a kernel change, or a refinement position other than a
  parameter or a written function domain needs the same treatment: stop to
  the Architect.

## Closeout

Merged `80afb1b10` from exact `b41dd8847` (FULL CI green, run
`37118271593`); the main tree equals the routed merge tree. Gates:
Architect `evt_5mfyzfy8jbnfw` and `evt_mwgrccpy7thm` (D6), CV
`evt_23p1t5tv91t1w`, Decision `dec_4efznr0k3pf6`.
`LANG-REFINED-PARAM-REQUIRES-DESUGAR` now waits only on the two Verify
repairs. Architect carry: the spec 21
`:504` `recordRefinement` comment goes to the next spec 21 touch.
