---
id: SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT
title: "No linear-algebra contract exists in spec/50-stdlib, so the catalog's dimension-safe linear algebra layer has nothing to build against. Author its chapter with a first section: vectors and matrices whose dimensions are in their types, dot product, matrix-vector and matrix-matrix product, transpose and identity, each law proved at zero trust"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". 06-catalog-campaign.md:375-376 places linear algebra (dimension-safe) after statistics/probability. Its exact and empirical tiers are delivered by CAT-STATISTICS-RATIONAL-VALUES. The approximate tier needs Float or error bounds, which the later numerical-computing layer supplies, so it is not next. Same shape as SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT. Steward-filed per COORDINATION section 2."
---

# A dimension-safe linear algebra contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for vectors and matrices whose dimensions are type
indices. The first linear-algebra catalog node builds against it.

## Settled inputs (on `b7b658e46`)

- **Length-indexed vectors are landed.** `Data.Vector.Vector`
  (`catalog/packages/Data/Vector/Vector.ken.md`, spec 60) declares
  `Vec a n` and `Fin n`. It has `map`, `zip_with`, `zip`, `unzip`,
  `lookup`, `to_list` and proved laws including `lookup_map`,
  `lookup_zip_with`, `vec_map_compose` and `zip_with_map`.
- **But only `Fin` is public.** `Fin`, `FZero` and `FSuc` are `pub` and
  exported, and two packages import them. `Vec`, its constructors, its
  operations and its laws are not `pub`, so an importing package cannot
  name them today. The D0 rules whether the CAT node publishes them as an
  existing-package change or the chapter declares its own carrier.
- **Natural arithmetic is proved.** `Data.Numeric.Nat.Arithmetic` exports
  `add` and `mul`, with `assoc`, `comm`, `zero_l` and `one_l`/`one_r` as
  `proof … for` laws, and `mul_add_distrib_l` and `mul_add_distrib_r`.
- **A rational carrier is landed** (spec 64). It is non-negative and has
  no subtraction. Its laws hold up to `rational_equiv`, not `Equal`.
- **No linear-algebra declaration exists.** Nothing in `catalog/` or
  `spec/50-stdlib` declares a matrix, a dot product or a transpose.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the scalar: `Nat` alone, or a lawful semiring class with `Nat` as
     its first instance. `Rational` is in this scope only if the
     Architect places it here;
   - the representation of an `m` by `n` matrix, for example
     `Vec (Vec s n) m`;
   - the operations and their exact laws: at least the dot product,
     vector addition and scaling, the matrix-vector and matrix-matrix
     products, transpose and the identity matrix. The laws are at least
     dot-product commutativity and distributivity over vector addition,
     involutive transpose, identity as a unit for both products, and
     associativity of matrix multiplication;
   - what stays deferred: signed scalars, determinants, inverses,
     solving, decompositions and any `Float` result.
2. **Section 1, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the trust boundary: no Axiom,
   primitive, `Float`, kernel form or `trusted_base()` entry. Index the
   chapter in `spec/50-stdlib/README.md` and `spec/SPEC-PROGRESS.md`.
3. **A conformance seed** under `conformance/stdlib/` in the existing seed
   shape:
   - a 2 by 3 times 3 by 2 product with its closed result;
   - an identity product;
   - a transpose round trip;
   - one dimension mismatch that must fail to check.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** The seed's mismatch row is refused by the checker, and the
  same product at matching dimensions checks.
- **AC-3.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed. Then Architect review.

## Stop conditions

- The contract needs a kernel, trust, `Float` or surface-syntax change.
- A law needs an elaboration capability the landed elaborator lacks.
  Stop with the failing development, since that is a Language
  prerequisite for the D0 to name.
