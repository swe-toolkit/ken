---
id: VERIFY-REUSED-ENV-TRUST-RESIDUE
title: "A reusable elaboration environment keeps trusted-base entries nobody reports: a declaration that fails after minting a premise hole leaves an orphan postulate, and an Axiom in elaborate_expr or the REPL persists from a context with no obligation channel. Make a failed declaration and a reporting-free expression leave no trust residue"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-04 (concur): queue the two reused-environment residues as one WP on the verify ring. Sources: the orphan hole, Architect evt_7x9fznedwjygv (recorded in VERIFY-CALL-SITE-OBLIGATION-CHANNEL's Not-this-WP), and the Axiom carry, Architect evt_3m2d23whyhqk1 with research advisory evt_6wfy5c9wpn849. Steward-filed per COORDINATION section 2. Measured at origin/main 18543f8e8."
---

# No unreported trust residue in a reused environment

## Objective

In every environment that outlives one declaration or expression (the REPL
`Session`, `modules::expand_and_elaborate`, `load_unit`, and
`elaborate_expr`), `trusted_base()` grows only by entries that a successful
declaration reports, or by an explicit user `axiom` declaration.

## Settled inputs (measured at `18543f8e8`)

- **Orphan hole** (Architect `evt_7x9fznedwjygv`).
  `precondition_proof` (`elab.rs:4522`) declares a premise hole before the
  declaration finishes. If the declaration then fails, the hole stays in the
  environment.
  - This matters wherever the environment is reused: the REPL `Session`
    (`crates/ken-cli/src/repl.rs:27`), `expand_and_elaborate`
    (`modules.rs:4589`) and `load_unit` (`modules.rs:1753`).
  - It over-reports in `trusted_base()`, and no source can reference it.
- **Axiom in a reporting-free context** (Architect `evt_3m2d23whyhqk1`).
  `(Axiom : P)` in `elaborate_expr` (`lib.rs:572`) or a REPL expression
  persists a trusted-base entry. The context has no obligation channel
  (`PremiseHoles::Refused`). The entry is explicit and visible, but nothing
  reports it.
- **Prior art** (research `evt_6wfy5c9wpn849`). Lean refuses to evaluate
  terms that depend on `sorry`, and elaborates commands under
  `withoutModifyingEnv`.
- `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` is changing the `Refused` gate
  in `elab.rs`. Implementation rebases onto main after it lands.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0, at the start of the repair.**
   - For each reusable environment above, measure the `trusted_base()`
     delta for two rows:
     - a declaration that mints a premise hole and then fails;
     - `(Axiom : P)` evaluated as an expression.
   - The Architect then rules the mechanism for each row. For a failed
     declaration: transactional rollback of the environment, or deferring
     the hole's declaration until success. For a reporting-free expression:
     refuse an `Axiom`, or elaborate without persisting.
2. **The ruled repair**, applied to every reusable environment in the list,
   not only the REPL (CHECKS 2).

## Acceptance

- **AC-1.** In each reusable environment:
  - a failed declaration leaves the `trusted_base()` delta at 0;
  - a reporting-free `(Axiom : P)` gets the ruled behaviour: refused, or
    delta 0.

  Each row asserts the delta before it asserts anything else.
- **AC-2 (controls).**
  - A successful declaration with reported holes keeps its delta, which
    equals its reported `hole_id`s.
  - An explicit `axiom` declaration is still recorded.
  - The call-site discharge and opaque-hole suites stay green.
- **AC-3 (falsifier).** Reverting the repair at one environment turns its
  row red.

## Stop conditions

- The ruled mechanism needs a kernel or spec change.
- A catalog or example consumer relies on the residue: stop to the
  Architect with the consumer (CHECKS 3).
