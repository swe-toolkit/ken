---
id: CAT-NUMERIC-INTERVAL-ASSOCIATIVITY-INSIDE
title: "Algorithm.Numeric.RationalInterval exports spec 68 section 6: both regroupings of interval_add and of interval_mul by mutual inclusion, proved at zero trust from the Architect's D0, with the section 6 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE]
blocks: []
github: null
origin: "Catalog half of SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE (spec candidate 46599f0b3, Decision dec_4759bbbb4awxs; AC-0 evt_1yd2w28b5exdv). Same shape as CAT-NUMERIC-INTERVAL-INSIDE-ORDER. Steward-filed per COORDINATION section 2."
---

# Interval regroupings export by mutual inclusion

## Objective

`Algorithm.Numeric.RationalInterval` exports
`interval_add_assoc_inside`, `interval_add_assoc_rev_inside`,
`interval_mul_assoc_inside` and `interval_mul_assoc_rev_inside`, each
with exactly spec 68 §6's public type, as checked Transparent proofs.

## Settled inputs (spec 68 §6 at `46599f0b3`, D0 on `edc1da359`)

- **The surface.** Four premise-free laws, each
  `(i : Interval) (j : Interval) (k : Interval) : IsTrue (interval_inside
  X Y)`, with X and Y the two regroupings in each order (§6, AC-0
  `evt_1yd2w28b5exdv` RULING 1).
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/interval-assoc-d0/d0.patch`
  (sha256 `81f9dfa9…`; `ruling.md` beside it): check and fmt rc 0,
  probe test 3/3, `trusted_base()` 114 before and after with equal
  GlobalId sets, no `Axiom`. The laws go directly after
  `interval_intersect_comm_inside` (`RationalInterval.ken.md:814`),
  ahead of the private `equiv_leq`. No import changes. Its test file
  `arch_interval_assoc_d0.rs` is scratch and does not ship.
- **No new helper.** The proofs use the private `equiv_leq` (`:839`),
  Rational's `proof assoc` and `proof sym for rational_equiv`. Keep the
  D0 bodies.
- **The prose.** The law count at `:38`, §4 Laws & proofs (`:911`) and
  the associativity non-claim near `:967` change with the exports.
- **The consumers.** `cat_numeric_interval_{distributivity,hull,
  inclusion,inside_order}.rs` and `cat_numeric_rational_interval.rs`
  assert trust sets and named rows. The Architect found no exhaustive
  export pin. Other importers run in CI; do not run them locally.
- **Seed.**
  `conformance/stdlib/numerical/seed-interval-associativity-inside.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four laws in `RationalInterval.ken.md` with its prose corrected, and
a new `crates/ken-elaborator/tests/cat_numeric_interval_associativity_inside.rs`
that root-loads the four names and runs the seed against the shipped
exports. Once the spec candidate has landed, update spec 68 §6's status
lines and the seed's execution-status statements in the same candidate
to say the package test checks them. Make no conformance-runner, CI or
landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass on the package. The
  test root-loads the four laws as RationalInterval-owned and
  Transparent, refuses importing `equiv_leq`, and runs the seed's whole
  accepted client, including the two `Refl` coincidence rows. Seed cases
  are selected by fence name, not by fence count. Its refused block is
  pinned to `seed_neg`'s false inclusion, and the seed's same-shape
  `Proved` row checks as the control.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The package's census row is unchanged, measured on the
  candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `interval_add_assoc_inside` with its low-endpoint `equiv_leq`
    argument taken from `proof assoc` directly, without `sym`, fails
    check;
  - `seed_neg`'s outer interval replaced by
    `interval_add (interval_add i j) k`, a true claim, fails the pin.

## Stop conditions

- A law needs an Axiom, a public helper, a kernel or surface change, or
  a public type other than §6's.
- A change to an existing RationalInterval law or operation, or a census
  row change.

Checks: 4 fired: the types, placement and rc are read from the AC-0
ruling, `rc.txt` and the `:814` and `:839` anchors on `edc1da359`. 3
fired: the Architect grepped catalog/, crates/, conformance/, spec/,
examples/ and docs/; the four names are free, and no exhaustive export
pin was found. 8 fired: the first AC-3 mutant orients the low endpoint
the wrong way, which a reflexive-looking proof would not catch; the
second turns the refusal into an accepted true claim.
