---
id: VERIFY-REUSED-ENV-TRUST-RESIDUE
title: "A reusable elaboration environment keeps a trusted-base entry nobody reports: a declaration that fails after minting a premise hole leaves an orphan postulate in the REPL, expand_and_elaborate, load_unit and a reused ElabEnv. Roll the environment back to a mark taken before each declaration"
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
declaration reports, or by an explicit user `axiom` declaration or `Axiom`
term, recorded under its caller's owner label (AX-2).

## Settled inputs (measured at `18543f8e8`)

- **Orphan hole** (Architect `evt_7x9fznedwjygv`).
  `precondition_proof` (`elab.rs:4522`) declares a premise hole before the
  declaration finishes. If the declaration then fails, the hole stays in the
  environment.
  - This matters wherever the environment is reused: the REPL `Session`
    (`crates/ken-cli/src/repl.rs:27`), `expand_and_elaborate`
    (`modules.rs:4589`) and `load_unit` (`modules.rs:1753`).
  - It over-reports in `trusted_base()`, and no source can reference it.
- **Prior art** (research `evt_6wfy5c9wpn849`). Lean refuses to evaluate
  terms that depend on `sorry`, and elaborates commands under
  `withoutModifyingEnv`.
- `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` is changing the `Refused` gate
  in `elab.rs`. Implementation rebases onto main after it lands.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## AC-0 result (verify `evt_czej6qsb7131`, ruled `evt_2sb1ehveqk5m0`)

- **Failed declaration.** In all four environments the delta is one orphan
  hole: the hole is minted, then the kernel rejects the body that references
  it (`KernelRejected(TypeMismatch)`). The staged paths (`elab.rs:15462`,
  `:15588`, `:15929`) already roll back; the orphan comes from the unstaged
  declaration paths.
- **`(Axiom : P)` is not a residue.** It is the AX-2 contract, pinned by
  `crates/ken-elaborator/tests/ax2_axiom_named_postulates.rs:86-110`: an
  explicit postulate whose report is the `Opaque` under its owner label.
  No change.

## Mechanism (kernel API operator-approved 2026-10-05)

The ruled mechanism is **rollback, not deferral**: the hole must be in the
environment when the kernel checks the body.

- **Kernel.** A public `EnvMark` with `env_mark` and `rollback_to_mark`,
  generalized from `rollback_pending` (`crates/ken-kernel/src/check.rs:1349`)
  without the staged-tail requirement. `rollback_pending` is rewritten in
  terms of it. It only removes declarations, never admits one.
  `GlobalEnv::remove_last` (`env.rs:691`) is `pub(crate)` today.
- **Elaborator.** Mark before each declaration at each reuse seam (REPL
  `do_def`, the per-declaration loop of `expand_and_elaborate`, `load_unit`,
  `elaborate_decl_v1`). On `Err`, roll back and scrub every `GlobalId`-keyed
  table in `ElabEnv`, enforced by an exhaustive destructuring with no `..`.
  Rollback resets the next id, so a stale entry would alias a later
  declaration (CHECKS 10).

The operator approved the removal-only `EnvMark` API on 2026-10-05. The
verify ring writes both halves in this WP, because the API has no use
without its elaborator consumer. **Kernel QA reviews the `crates/ken-kernel`
diff** alongside verify QA, and the Architect gate stands.

## Acceptance

- **AC-1.** In each reusable environment, a failed declaration leaves the
  `trusted_base()` delta at 0. Each row asserts the delta first.
- **AC-2 (controls).**
  - A successful declaration with reported holes keeps its delta, which
    equals its reported `hole_id`s.
  - An explicit `axiom` declaration and a standalone `Axiom` term are still
    recorded (AX-2).
  - After a rolled-back failure, the next successful declaration's
    `GlobalId` resolves only to itself in every scrubbed table.
  - The call-site discharge and opaque-hole suites stay green.
- **AC-3 (falsifier).** Removing the rollback at one seam turns its row red.

## Stop conditions

- Any kernel change beyond the removal-only `EnvMark` API.
- A catalog, conformance or test consumer relies on the residue: stop to the
  Architect with the consumer (CHECKS 3).

## Symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. an inner clone-restore replaced the environment under an outer mark, keyed on environment instance identity
```
