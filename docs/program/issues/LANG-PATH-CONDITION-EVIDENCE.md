---
id: LANG-PATH-CONDITION-EVIDENCE
title: "Path conditions are obligation-only (elab.rs: never unchecked evidence in an emitted program), so a proof stored in a refinement pair has no term for the conditions its obligation is closed over. Bind each pushed path condition as a context variable through a convoy, with the carrier encoding otherwise unchanged"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W2."
---

# Path conditions carry evidence terms

## Objective

Every pushed path condition is a term in Γ, so the subset-Σ introduction
(`LANG-REFINEMENT-SUBSET-SIGMA`) can apply the obligation hole to it.

## Settled inputs (Architect, measured at `1153a9fc6`)

- `elab.rs:11662`: path conditions are obligation-only, "never unchecked
  evidence in an emitted program".
- The obligation hole is declared at `Π Γ. Π conds. Π hyps. φ` and applied
  to Γ's variables (`close_refinement_goal_with`).
- Kernel `Let` is substitution, so a `let` equation's evidence is
  `refl`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- At every site that pushes a path condition, bind the equation as a
  context variable. For `if` and `match` this is a convoy: motive
  `λ y. Eq S s y → T`, applied to `refl s`. For a `let` equation the
  evidence is `refl`.
- `path_conditions` carries each condition's evidence term.
- The obligation hole is applied to that evidence instead of being left
  closed over an unwitnessed condition. Representation, obligation ids and
  discharge are otherwise unchanged.

## Acceptance

- **AC-0 (D0, measure only).** List every path-condition push site by
  mechanism, at the review SHA, with `file:line`.
- **AC-1.** Each listed site binds evidence. A test per site kind (`if`,
  `match`, `let`) decodes the emitted core and finds the condition as a
  bound variable that the hole application uses.
- **AC-2.** Population delta 0: per-package obligation counts and
  outcomes are unchanged across the catalog and the corpus.
- **AC-3.** Runtime parity is unchanged, which shows the convoy erases.
- **AC-4 (mutation).** Dropping the evidence argument at one site reddens
  that site's AC-1 row and is refused by the kernel.

## Stop conditions

- A push site whose condition has no term in Γ: stop to the Architect.
- Any kernel, `trusted_base()` or spec change.
