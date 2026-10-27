---
id: RT-NATIVE-ERASED-OMEGA-CALL-ARGUMENT
title: "A native build refuses a recursive callee that takes an erased proof argument: fn fw (n : Nat) : Nat requires Top = match n { Zero ↦ Zero; Suc m ↦ Suc (Suc (fw m)) } fails with E0074 at the computational-IH collector, though the reference runtime runs it. Lower calls that carry an erased Ω argument natively"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_5nt4mcwbc2e7k on LANG-REFINED-PARAMETER-STRUCTURAL-RECURSION: the native failure exists at base 1d82fd8ec through a requires precondition, and the refined-parameter worker shares its call shape. Fails closed: a valid program is refused on the native route. Steward-filed per COORDINATION section 2."
---

# Native calls carry erased proof arguments

## Objective

A recursive call whose callee takes an erased Ω argument builds
natively and computes the same result as the reference runtime, for
both a `requires` precondition and a refined parameter.

## Settled inputs (measured on `main` at `1d82fd8ec`, Architect `evt_5nt4mcwbc2e7k`)

- **The refusal.** Native normalization leaves `main` calling the
  callee as `DirectDeclarationCall` applied to the carrier and then to
  `ErasedOmegaSubterm`. `ComputationalIHTemplateCollector::visit`
  (`crates/ken-elaborator/src/compiler_driver.rs`, struct at ~:1639,
  visit ~:2118) has no arm for `(_, ErasedOmegaSubterm)` and falls
  through to E0074, with owner `main`.
- **A leaf arm alone does not fix it.** Adding that arm moves the
  failure to E0079 `erased_omega_subterm_reached_computation`
  (`crates/ken-elaborator/src/erasure.rs` ~:5174), because checked-core
  lowering cannot lower an Ω subterm as a runtime expression.
- **The four probes.** Each uses the exit-code shape of
  `crates/ken-cli/tests/lang_checked_ih_body_view_cause.rs` with a
  doubled body, so correct recursion exits 4 and dropped recursion
  exits 2. The sources are `x_plain.ken`, `x_requires.ken`,
  `x_refined.ken` and `x_refnonrec.ken` in
  `/workspaces/ken/local/arch-rpsr-native/`. At base, `plain` exits 4
  natively, `requires` fails E0074, and `refnonrec` exits 3.
- **Refined workers.** Once LANG-REFINED-PARAMETER-STRUCTURAL-RECURSION
  lands, `refined` reaches the same E0074 through its worker. Until
  then it is refused earlier, with K0014.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Native lowering that erases an Ω call argument before computation, and
a committed native test of the probe sources.

## Acceptance

- **AC-0 (D0, Architect).** Rule the seat: erase the argument at the
  call site in the runtime view, or drop the parameter from the native
  signature, with the Ω-binder evidence it relies on.
- **AC-1.** `requires` builds natively and exits 4. If the language WP
  has landed, `refined` does too. `plain` exits 4 and `refnonrec`
  exits 3, as on base.
- **AC-2 (mutation).** Dropping the recursive call in `requires` makes
  the native build exit 2, so the test cannot pass on a non-recursive
  lowering.

## Stop conditions

- A fix that lowers a proof term as a runtime value, or any kernel or
  `trusted_base()` change.
- Any change to reference-runtime results, or to native parity on the
  existing corpus.

Checks: 4 fired: the collector, the E0079 site and the four probe
sources exist at `8546cb58c`/`local/arch-rpsr-native/`. 8 fired: the
doubled body tells recursion from no recursion (4 against 2).
