---
id: LANG-EXPRESSION-SIGMA
title: "Add expression-position dependent Σ, mirroring the Π production, so a Σ at Ω such as And's body can be written in catalog Ken; spec grammar and parser/elaborator land together, with no kernel or trusted-base change; unblocks moving the And family out of the prelude"
status: active
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
`fn And (a : Omega) (b : Omega) : Omega = (x : a) × b` checks in a catalog
module. `And` is not a prelude floor member (`30-taxonomy.md §4`, `§6`), so
the spec and conformance make no claim about a prelude `And`.

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

- **AC-1 (positive, spec and conformance).** `fn And (a : Omega) (b :
  Omega) : Omega = (x : a) × b` declared in a module checks, and a dependent
  pair checks against `And a b`. The `32 §3` prose and the conformance pair
  use only identities available under `33 §3.3`: the seed declares its own
  `And` or imports one. Neither calls it a prelude conjunction.
- **AC-1a (migration evidence, Rust suite only; Spec block
  `evt_45yz6089crevc`, ruling `evt_nh39zr86trxt`).** The catalog definition
  is convertible, both ways and in an open context, with the implementation's
  current legacy prelude `And` term. The L3 move of the `And` family relies on
  this. It is a property of the implementation during the migration, not a
  spec claim, and it lives only in the Rust test.
- **AC-2 (negative).** A Σ whose first component is relevant still sorts at
  `Type`, per `sort_sigma`, so ascribing it `Omega` is rejected. Existing
  type-position Σ suites stay green.
- **AC-3 (controls).** AC-1 and AC-1a are red on base. Reverting the parser
  production reddens it. Targeted builds only, through `scripts/ken-cargo`;
  no-regression means green in CI.

## Stop conditions

- Any kernel, reduction or `trusted_base()` change.
- The production makes an existing expression parse differently.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Gate

Spec text rides this branch, so it needs the Spec-domain vote as well as the
Architect's, and lands atomically. It must land before the L3 collections
slice, which moves the `And` family with `is_sorted`.
