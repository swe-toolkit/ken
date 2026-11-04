---
id: SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-POINTWISE-CONGRUENCE
title: "NatPolynomial's poly_eq_substitute says one substitution preserves canonical equality of the expression it is applied to, but no law covers the substitution itself: two substitutions whose images are canonically equal at every variable are not known to give canonically equal results. A client that rewrites `x := (a + b)` to `x := (b + a)` inside a substituted expression must re-prove the whole result from evaluation. Rule from a checked D0 at zero trust a public pointwise congruence law for substitute, with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-TIME-WINDOW-LENGTH-BOUND]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra follows Time in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Canonically equal substitutions give canonically equal results

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law
that, for every `s`, `t : Nat → Expr` and `e : Expr`:

```ken
((v : Nat) → IsTrue (poly_eq (s v) (t v)))
  → IsTrue (poly_eq (substitute s e) (substitute t e))
```

Spec 66 states it by its public type.

## Settled inputs (main `c9415785b`)

- **The operations.** In `NatPolynomial.ken.md`, `substitute` (`:834`)
  replaces each `EVar v` by `s v` in one pass, and `compose_env`
  (`:832`) is `eval env (s v)`.
- **The neighbours.** `poly_eq_substitute` (`:866`) is congruence in the
  expression for a fixed substitution. `eval_substitute` (`:842`) turns
  `eval env (substitute s e)` into `eval (compose_env env s) e`.
  `poly_eq_sound` (`:721`) gives `eval env (s v) = eval env (t v)` from
  each premise, so `compose_env env s` and `compose_env env t` agree at
  every variable. `poly_eq_complete` (`:744`) returns the `IsTrue`
  decision from evaluation equality at every environment.
- **The missing step.** The proof needs `eval` to agree under two
  environments that agree at every variable. That is an induction on
  `Expr` with no function extensionality. No public or private theorem
  of that shape was found; the private `eval` helpers are
  `eval_monomials_append` (`:422`), `shift_cons_eval` (`:2017`) and
  `deriv_var_deriv_eval` (`:2530`).
- **Not yet stated.** Spec 66 (12 sections, file end `:1131`) and the
  package state substitution congruence only in the expression
  (§3, `poly_eq_substitute`). `conformance/stdlib/symbolic-algebra/`
  has no seed for congruence in the substitution.
- **Seeds.** Next to `seed-nat-polynomial-substitution.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in NatPolynomial, a spec 66 section stating it by its
public type, a closed conformance seed under
`conformance/stdlib/symbolic-algebra/`, a named package test, and spec
66's own status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the law's name, argument order and placement;
  - the section number;
  - whether the environment-agreement step is public or private.

  It confirms that no §1 to §12 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client substitutes `EAdd (EVar Zero)
  (EVar (Suc Zero))` and `EAdd (EVar (Suc Zero)) (EVar Zero)` for
  variable 0 into `EMul (EVar Zero) (EVar Zero)`, and concludes the two
  results are canonically equal through the law.
  - Its refused block claims the same canonical equality when the
    second substitution maps variable 0 to `EVar (Suc Zero)` alone. That
    is false, because the two images differ by `EVar Zero`.
  - Its accepted same-form control states the true claim of the
    accepted client directly, so the refusal is the image, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- A law needs an Axiom, function extensionality, a kernel or surface
  change, or any premise beyond the pointwise canonical equality.
- A change to an existing §1 to §12 operation or law type.

Checks: 4 fired: the law is written from public `substitute`,
`poly_eq`, `eval_substitute`, `poly_eq_sound` and `poly_eq_complete`,
each read at its line, and the one missing step, environment agreement
by induction, is named rather than assumed. 3 fired: the spec, the
package and the seed directory were searched for congruence in the
substitution, and only the expression-side law exists. 8 fired: the
refused row and its control share the expression, the first
substitution and the claim form, and differ only in the second
substitution's image of variable 0.
