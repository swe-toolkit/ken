---
id: CAT-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY
title: "Algorithm.Optimization.BoundedSearch exports spec 72 section 2.4: find_below_stable, proved at zero trust from the Architect's D0, with the section 2.4 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY]
blocks: []
github: null
origin: "Catalog half of SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY (spec candidate ab213eeee, Decision dec_19aewt4ya1kqp). Same shape as CAT-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION. Steward-filed per COORDINATION section 2."
---

# BoundedSearch exports bound stability

## Objective

`Algorithm.Optimization.BoundedSearch` exports `find_below_stable` with
exactly spec 72 §2.4's public type, as a checked proof.

## Settled inputs (spec 72 §2.4 at `ab213eeee`, D0 on `0bb872591`)

- **The surface.** §2.4 states one law, `find_below_stable (p : Nat →
  Bool) (bound : Nat) (larger : Nat) (n : Nat)`, with the premises
  `leq_nat bound larger = True` and `find_below p bound = Some n`, in
  that order. No `None` counterpart is stated.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/bound-stability-d0/d0.patch`
  (sha256 `695150c5…`; ruling `ruling.md` beside it): check rc 0,
  `trusted_base()` 114 before and after, with no `Axiom`. The saved
  `rc.txt` records `fmt=1` from a run before the final save; `ken fmt
  --check` on the saved patch applied to `0bb872591` is rc 0. It has
  three hunks in `BoundedSearch.ken.md`: the imports gain
  `Core.Logic.Transport (cong, sym, trans)` and
  `Data.Numeric.Nat.Order (leq_nat, lt_nat)`, `find_below_stable` goes
  after `find_below_complete`, and the private `leq_suc_not_leq_same`
  goes after `lt_suc_not_lt_same`. Its third file,
  `arch_bound_stability_d0.rs`, is scratch and does not ship.
- **One private helper.** `leq_suc_not_leq_same` stays private.
- **The consumers.** The package prose counts "three laws"
  (`BoundedSearch.ken.md:26`, `:229`, `:245`, `:251`, `:261`), and the
  D0 does not edit it. `cat_optimization_bounded_search.rs` keeps exact
  public and private name lists (`:43`, `:80`):
  `find_below_stable` joins the public list and `leq_suc_not_leq_same`
  the private list. The `lang_mod_strict_resolution_d0` BoundedSearch
  row lists ambient names only, so explicit imports leave it unchanged.
  The implementer sweeps for any other export-set pin before editing.
- **Seed.**
  `conformance/stdlib/optimization/seed-bounded-search-bound-stability.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The export, helper and imports in `BoundedSearch.ken.md` with its law
prose corrected, the two name lists in
`cat_optimization_bounded_search.rs` updated, and a new
`crates/ken-elaborator/tests/cat_optimization_bounded_search_bound_stability.rs`
that root-loads `find_below_stable` and runs the seed. Once the spec
candidate has landed, update spec 72 §2.4's status lines and the seed's
execution-status statements in the same candidate to say the package
test checks them. Make no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  `find_below_stable` as package-owned and transparent, refuses
  importing `leq_suc_not_leq_same`, and runs the seed's accepted
  client. Seed cases are selected by theorem name, not by fence count.
  Its refused block is pinned to the seed's `None`-stability
  proposition, and the same-premise controls check. The existing
  `cat_optimization_*` binaries still pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Optimization.BoundedSearch`
  is unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `find_below_stable`'s `False` arm transporting `found` without the
    `leq_suc_not_leq_same` step fails check;
  - the refused block's conclusion replaced by the true `Some three`
    at bound ten fails the pin.

## Stop conditions

- The law needs an Axiom, a new public helper, a kernel or surface
  change, or a public type other than §2.4's.
- A change to an existing §1 to §3 law or operation, or a census row
  change.

Checks: 4 fired: the §2.4 type, base, trust count and hunk placement
are read from `ruling.md` and the patch's hunk headers. The saved
`rc.txt` says `fmt=1`, so the patch was applied to `0bb872591` and
`ken fmt --check` rerun: rc 0. 8 fired: the second AC-3 mutant turns
the refusal into an accepted true claim, so a pin that only checks
that some block was refused stays green on it.
