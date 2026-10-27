---
id: CAT-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION
title: "Algorithm.SymbolicAlgebra.NatPolynomial exports spec 66 section 6: deriv_commute, proved at zero trust from the Architect's D0, with the section 6 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION]
blocks: []
github: null
origin: "Catalog half of SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION (spec candidate 0b93728b0, Decision dec_5bzwxz9nhq1cf). Same shape as CAT-TIME-WINDOW-COMPOSITION. Steward-filed per COORDINATION section 2."
---

# NatPolynomial exports derivative commutation

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports `deriv_commute` with
exactly spec 66 §6's public type, as a checked proof.

## Settled inputs (spec 66 §6 at `0b93728b0`, D0 on `2f78d45b8`)

- **The surface.** §6 states one law,
  `deriv_commute (v : Nat) (w : Nat) (e : Expr) (env : Nat → Nat)`,
  as pointwise `Equal Nat` of `eval` over the two derivative orders.
  No `poly_eq` corollary is exported.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/deriv-commute-d0/d0.patch`
  (sha256 `9be67780…`; ruling `ruling.md` beside it): rc 0, fmt rc 0,
  `trusted_base()` 114 before and after, with no `Axiom`. It has two
  hunks in `NatPolynomial.ken.md`: `deriv_commute` after
  `poly_eq_deriv`, and the private `deriv_var_deriv_eval` after
  `var_deriv`. Its third file, `arch_deriv_commute_d0.rs`, is scratch
  and does not ship.
- **One private helper.** `deriv_var_deriv_eval` stays private. The
  proof otherwise uses existing private `add_cong` and
  `add_interchange` and `Core.Logic.Transport`.
- **The consumers.** The package prose counts the derivative laws
  ("Its two checked laws", `NatPolynomial.ken.md:53-56`), and the D0
  does not edit it. `cat_symbolic_algebra_derivative.rs` is the nearest
  export pin. The implementer sweeps for any other export-set pin
  before editing.
- **Seed.** `conformance/stdlib/symbolic-algebra/seed-derivative-commutation.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The export and helper in `NatPolynomial.ken.md` with its law prose
corrected, and a new
`crates/ken-elaborator/tests/cat_symbolic_algebra_derivative_commutation.rs`
that root-loads `deriv_commute` and runs the seed. Once the spec
candidate has landed, update spec 66 §6's status lines and the seed's
execution-status statements in the same candidate to say the package
test checks them. Make no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  `deriv_commute` as package-owned and transparent, refuses importing
  `deriv_var_deriv_eval`, and runs the seed's accepted client. Its
  refused block is pinned to the seed's structural `Equal Expr`
  proposition, and the same-input control checks. The existing
  `cat_symbolic_algebra_*` binaries still pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.SymbolicAlgebra.NatPolynomial`
  is unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `deriv_commute`'s `EMul` arm with `add_interchange` dropped, so the
    middle two summands are not swapped, fails check;
  - the refused block's `Equal Expr` replaced by `Equal Nat` of `eval`
    on the same expression fails the pin.

## Stop conditions

- The law needs an Axiom, a new public helper, a kernel or surface
  change, or a public type other than §6's.
- A change to an existing §1 to §5 law or operation, or a census row
  change.

Checks: 4 fired: the §6 type, the D0's base, trust count and hunk
placement are read from `ruling.md`, the patch's hunk headers and its
`rc.txt`/`test-rc.txt` (all 0, test 2 passed). 8 fired: the second AC-3
mutant turns the refusal into the true law, so a pin that only checks
that some block was refused stays green on it.
