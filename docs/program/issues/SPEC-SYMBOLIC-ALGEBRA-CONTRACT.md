---
id: SPEC-SYMBOLIC-ALGEBRA-CONTRACT
title: "No symbolic-algebra contract exists in spec/50-stdlib, so the catalog's next roadmap layer after linear algebra has nothing to build against. Author its chapter with a first section: a polynomial expression type over natural coefficients, its evaluation, and a normal form whose evaluation is proved equal to the expression's, at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [CAT-LINEAR-ALGEBRA-PRODUCT-LAWS]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:375-376 places symbolic algebra after linear algebra (dimension-safe). Linear algebra's deferred items (signed scalars, determinants, inverses, solving) need a signed field, which the catalog does not have: spec 64's Rational is non-negative with no subtraction. Same shape as SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT. Steward-filed per COORDINATION section 2."
---

# A symbolic algebra contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for symbolic polynomial expressions. The first
symbolic-algebra catalog node builds against it.

## Settled inputs (on `033c19fa7`)

- **Natural arithmetic is proved.** `Data.Numeric.Nat.Arithmetic` exports
  `add` and `mul`. For `add` it has `zero_l`, `suc_l`, `assoc` and `comm`
  as `proof … for` laws. For `mul` it has `zero_l`, `one_l`, `one_r`,
  `comm`, `assoc` and `cancel_r`. It also has `mul_add_distrib_l` and
  `mul_add_distrib_r`.
- **Collections are landed.** `Data.Collections` has `List`, `Map`,
  `NonEmpty`, `Deque`, `PriorityQueue` and `Derived`. `Data.Vector.Vector`
  exports `Vec`, `map`, `zip_with` and `vec_zero_vnil`.
- **No symbolic declaration exists.** Nothing in `catalog/` or
  `spec/50-stdlib` declares a polynomial, a symbolic expression or a
  normal form.
- **Known elaborator limits.** A λ passed to `map` hits
  `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`; use a named helper,
  as spec 65 does. A dependent `match` refines only its scrutinee as
  written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the expression type: for example `Expr` with variables, natural
     constants, `+` and `*`, and how variables are indexed;
   - evaluation under an environment;
   - the normal form, for example a sorted coefficient list or `Map`
     from monomial to coefficient, and `normalize`;
   - the laws: at least that `eval (normalize e) = eval e` for every
     environment, and that `normalize` is idempotent. Whether equal
     normal forms decide semantic equality is in scope only if the D0
     measures it;
   - what stays deferred: subtraction and signed coefficients, division,
     factoring, and anything that needs a field.
2. **The chapter, section 1**, stating the D0's operations and proved
   laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one case per operation and one per law,
  and a case that refuses a normalizer which drops a monomial.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  chapter.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot check normalization's soundness generically. Report the
  measured blocker, and do not state that law as proved.
