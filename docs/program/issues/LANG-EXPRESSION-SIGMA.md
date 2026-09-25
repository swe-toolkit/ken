---
id: LANG-EXPRESSION-SIGMA
title: "Add expression-position dependent Σ, mirroring the Π production, so a Σ at Ω such as And's body can be written in catalog Ken; spec grammar and parser/elaborator land together, with no kernel or trusted-base change; unblocks moving the And family out of the prelude"
status: ready
owner: language
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect evt_16ve7jwxbzax3: CAT-LOGIC-PRELUDE-MOVE hard stop 1, And's body has no catalog spelling because Σ exists only in type position. Serves the operator's 2026-09-25 minimal-prelude ruling: the L2 flip deletes And, whose consumers then need a catalog And. Steward-filed per COORDINATION section 2."
---

# Expression-position Σ

## Objective

A dependent pair type can be written in expression position, so
`fn And (a : Omega) (b : Omega) : Omega = (x : a) × b` checks and is
convertible with the prelude `And`.

## Settled inputs -- Architect, measured at `ae5cce97f`

- `spec/30-surface/32-grammar.md §2` has dependent Σ
  `"(" ident ":" type ")" "×" type` in type position only. `§3` gives Π both an
  expression and a type spelling; Σ has only the type one.
- The parser's `parse_dependent_binder_type` accepts `×` in types only.
- `def T = A` (`parser.rs:3748`) takes no parameters, so it cannot abstract
  over `a b`. `Pair a b` fails at Ω with `TypeMismatch { expected: Type 0,
  found: Ω0 }`. A record or inductive `And` would be a new, non-convertible
  identity. So no existing form spells `And`.
- The kernel Σ former and `sort_sigma` are landed. No kernel feature is
  needed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- The `32 §3` expression production for Σ, mirroring the Π row. Spec owns the
  exact production and authors it on this branch.
- Parser and elaborator support, elaborating to the landed kernel Σ former.
- A conformance pair.

## Acceptance

- **AC-1 (positive).** `fn And (a : Omega) (b : Omega) : Omega = (x : a) × b`
  in a catalog module checks, and a term of the prelude `And a b` checks
  against it and back (convertible).
- **AC-2 (negative).** A Σ whose first component is relevant still sorts at
  `Type`, per `sort_sigma`, so ascribing it `Omega` is rejected. Existing
  type-position Σ suites stay green.
- **AC-3 (controls).** AC-1 is red on base. Reverting the parser production
  reddens it. Targeted builds only, through `scripts/ken-cargo`; no-regression
  means green in CI.

## Stop conditions

- Any kernel, reduction or `trusted_base()` change.
- The production makes an existing expression parse differently.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Gate

Spec text rides this branch, so it needs the Spec-domain vote as well as the
Architect's, and lands atomically. It must land before the L3 collections
slice, which moves the `And` family with `is_sorted`.
