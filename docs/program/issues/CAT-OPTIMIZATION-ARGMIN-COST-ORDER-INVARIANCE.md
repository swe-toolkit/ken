---
id: CAT-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE
title: "Algorithm.Optimization.Argmin exports argmin_cost_order_invariant, proved at zero trust from the Architect's D0, with the spec 72 seed run by a package test"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE]
blocks: []
github: null
origin: "Catalog half of SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE (Architect AC-0 evt_64h89e05agzat). Same shape as CAT-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY. Steward-filed per COORDINATION section 2."
---

# Argmin exports cost-order invariance

## Objective

`Algorithm.Optimization.Argmin` exports `argmin_cost_order_invariant`
with exactly the AC-0 ruled public type, as a checked proof.

## Settled inputs (AC-0 `evt_64h89e05agzat`, D0 on `12e91c41b`)

- **The surface.** One law, `argmin_cost_order_invariant (a : Type)
  (cost : a → Nat) (f : Nat → Nat) (same_order : (i : Nat) → (j : Nat)
  → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j)) (xs : NonEmpty a)`,
  concluding `Equal a (argmin a (comp a Nat Nat f cost) xs) (argmin a
  cost xs)`. It goes after `argmin_takes_tail` and before
  `argmin_option`.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/argmin-cost-order-d0/d0.patch`
  (sha256 `4be53321…`; `SUMMARY.txt` and `rc-final.txt` beside it):
  check, fmt and rustfmt rc 0, `trusted_base()` 114 before and after,
  no `Axiom`, existing argmin and argmax binaries 12/12. Its catalog
  hunks add `import Core.Function.Combinators (comp)`, the law, the two
  private helpers after `choose_yes`, and the prose count and API
  changes. Its `arch_argmin_cost_order_d0.rs` is scratch and does not
  ship. One Derivation prose line runs past 85 columns; wrap it.
- **Two private helpers.** `select_from_cost_order` and
  `choose_cost_order` stay private.
- **The consumers.** `cat_optimization_argmin.rs` pins the seven §1.2
  laws by name (`:108`) and its generic client says "all seven"
  (`:121`). `cat_optimization_argmin_option.rs` keeps a private-name
  list (`:170`). `lang_mod_strict_resolution_d0.rs` has an Argmin row
  (`:535`). The implementer sweeps for any other export-set pin before
  editing.
- **Seed.** The seed under `conformance/stdlib/optimization/` arrives
  with the spec candidate. Until it lands, build on main against the
  queued spec candidate's seed and checkpoint as NOT REVIEWABLE; after
  it lands, merge main (no rebase) and hand off one exact tip.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The export, helpers and import in `Argmin.ken.md` with its prose
counts corrected, the consumer pins updated, and a new
`crates/ken-elaborator/tests/cat_optimization_argmin_cost_order.rs`
that root-loads `argmin_cost_order_invariant` and runs the seed. Once
the spec candidate has landed, update spec 72's new subsection status
lines and the seed's execution-status statements in the same candidate
to say the package test checks them. Make no conformance-runner, CI or
landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the law as package-owned and transparent, refuses importing either
  helper, and runs the seed's accepted client. Seed cases are selected
  by theorem name, not by fence count. Its refused block is pinned to
  the seed's flattened-cost proposition. The existing
  `cat_optimization_*` binaries still pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Optimization.Argmin` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `choose_cost_order`'s transport with `same_order`'s arguments
    swapped fails check;
  - the refused block's conclusion replaced by the true flattened
    selection `Zero` fails the pin.

## Stop conditions

- The law needs an Axiom, a new public helper, a kernel or surface
  change, or a public type other than the ruled one.
- A change to an existing §1 to §3 law or operation, or a census row
  change.

Checks: 4 fired: the type, base, trust count and placement are read
from the AC-0 post and the patch's hunk headers on `12e91c41b`;
`comp` replaces the frame's lambda spelling, which the D0 measured as
refused in type position. 8 fired: the second AC-3 mutant turns the
refusal into an accepted true claim, so a pin that only checks that
some block was refused stays green on it.
