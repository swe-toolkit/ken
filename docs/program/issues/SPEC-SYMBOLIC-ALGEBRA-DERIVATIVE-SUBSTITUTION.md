---
id: SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-SUBSTITUTION
title: "Spec 66 defers derivative/substitution commutation. Rule from a checked D0 at zero trust that the derivative in v of a substituted expression evaluates to the substituted derivative when the substitution's images have derivative one at v and zero elsewhere, and state it with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra is next in the layer rotation after Time. Same shape as SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-INDEPENDENT-VARIABLE. Steward-filed per COORDINATION section 2."
---

# The derivative commutes with a v-preserving substitution

## Objective

Spec 66 states, as a public checked law over
`Algorithm.SymbolicAlgebra.NatPolynomial`, that at an environment
`env`, `eval env (deriv v (substitute s e))` equals
`eval (compose_env env s) (deriv v e)` whenever, at `env`, `s v`
has derivative one in `v` and every other `s w` has derivative
zero in `v`. This is the chain rule in the one case where it needs
no sum over variables.

## Settled inputs (spec 66 §1 to §7, main `329a11343`)

- **The operations.** `substitute` replaces each `EVar w` by `s w`
  structurally, and `eval_substitute` already equates
  `eval env (substitute s e)` with `eval (compose_env env s) e`
  (§3). `deriv` is structural with the left-then-right product rule
  (§5). `var_deriv` and `nat_same` are private.
- **Deferred, not stated.** §5.3 and §6 both say derivative and
  substitution commutation does not follow from them
  (`66-symbolic-algebra.md:570`, `:652`). No law combines `deriv`
  and `substitute`.
- **Why it should hold.** By induction on `e`, at the one `env`:
  - `EVar w` reduces to the two premises;
  - `EAdd` is linear;
  - `EMul` uses the induction hypotheses for the two derivative
    factors and `eval_substitute` for the two plain factors.
- **The premises are expected to be necessary.** Each fails at
  `v = Zero` on a one-variable expression:
  - **derivative one at `v`:** `s w = EConst Zero` for every `w`,
    with `e = EVar Zero`. The left side is `0`, the right `1`. The
    other premise holds;
  - **derivative zero elsewhere:** `s Zero = EVar Zero`,
    `s (Suc Zero) = EVar Zero`, otherwise `EVar w`, with
    `e = EVar (Suc Zero)`. The left side is `1`, the right `0`. The
    first premise holds.

  The D0 measures each premise and rules how it is spelled: pointwise
  at `env` as `Equal Nat` values, through `Equal Nat v w → Bottom`
  as §7 does, or otherwise. It keeps a premise only with its
  counterexample.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-derivative-independent-variable.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §8 in spec 66 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/symbolic-algebra/`, with the
status lines changed in the same candidate. §5.3's and §6's deferral
sentences are corrected to point at §8. The catalog half travels in
this candidate if the Architect rules so, as for
SPEC-TIME-WINDOW-ADJACENT-SPLIT. If it does, it comes with a package
test that runs the seed. Otherwise it is a separate `CAT-SYMBOLIC-ALGEBRA-`
node framed from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type and argument order, the premise spelling, and
  which helpers stay private. Each premise it keeps comes with a closed
  counterexample to the law without it.
- **AC-1.** The new section states the ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law at a closed
  environment. The expression contains an `EMul`, and the substitution
  maps some `w ≠ v` to a non-variable expression free of `v`. Each side
  is also computed independently. Its refused block is pinned to one
  false proposition of the same shape: the conclusion for a closed
  substitution that sends some `w ≠ v` to `EVar v`. It also carries an
  accepted same-shape control and an independent `False` value.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, a new public
  operation such as an environment update, or a premise beyond the two
  derivative conditions on `s`.
- A change to an existing §1 to §7 law or operation.

Checks: 3 fired: grepped spec 66 and `NatPolynomial.ken.md` on
`329a11343` for `deriv` and `substitut` on one line. The only hits are
the two deferral sentences (`:570`, `:652`). The package's 25 public
declarations were listed, and no theorem takes both. 8 fired: both
counterexamples were evaluated by hand from the defining equations
above, and each violates exactly one premise. AC-2's refused claim
violates the zero-elsewhere premise, so a seed whose substitution
already met both premises would not separate it.
