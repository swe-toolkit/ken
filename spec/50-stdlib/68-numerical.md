# Exact numerical intervals

> **Status: DRAFT v0 (SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT).**
> Section 1 specifies exact closed intervals over non-negative
> `Rational`, three total operations, and six generic interval laws
> checked in Architect D0. The D0 used two additional checked Rational
> monotonicity proofs in scratch; neither their catalog export nor the
> interval module has landed as part of this contract. No shipped-seed
> execution, CI result, or trusted-base delta is claimed.

## 1. Exact intervals over non-negative rationals

### 1.1 Carrier, membership, and equality

`Algorithm.Numeric.RationalInterval`, at
`catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`, reuses
`Data.Numeric.Rational.Rational` (§64). A rational stores a `Nat`
numerator and the predecessor of a positive denominator; it is
therefore non-negative, exact, and not necessarily normalized. The
interval has two endpoint values and **no proof field** requiring
`lo ≤ hi`:

```ken
pub data Interval : Type where {
  MkInterval : (lo : Rational) → (hi : Rational) → Interval
}
export MkInterval

pub fn interval_contains (i : Interval) (x : Rational) : Bool
```

For `MkInterval lo hi`, membership is
`bool_and (rational_leq lo x) (rational_leq x hi)`. Both bounds
are **inclusive**, and the comparisons are §64's exact Boolean
order on rational values, not comparisons of numerators or raw
constructor fields. If `lo` is greater than `hi`, no value is a
member: inverted bounds are the ordinary representation of an empty
interval, not an error, an `Option`, or a separate constructor.
There is no requirement that an operation preserve the emptiness of
an operand; its endpoint formula still defines a total result.

`Equal Interval` compares the **stored endpoint spellings**, not
sets of members. For example, `interval_point (1/2)` and
`interval_point (2/4)` contain the same rational values but have
different endpoint terms. The correct equality of those values is
`rational_equiv` (§64), not `Equal Rational` between different
spellings or `Equal Interval` between those two point intervals.
Differently represented empty intervals need not be equal either.
The explicit `export MkInterval` is necessary for a client to
construct an interval: `pub data` alone exports only its type
(`../30-surface/33 §4.1`).

### 1.2 Endpoint operations

```ken
pub fn interval_add (i : Interval) (j : Interval) : Interval
pub fn interval_mul (i : Interval) (j : Interval) : Interval
pub fn interval_point (x : Rational) : Interval
```

For `i = MkInterval a b` and `j = MkInterval c d`, the results are
exactly:

- `interval_add i j = MkInterval (rational_add a c)
  (rational_add b d)`.
- `interval_mul i j = MkInterval (rational_mul a c)
  (rational_mul b d)`.
- `interval_point x = MkInterval x x`.

Multiplication uses **only corresponding endpoints**, not the
minimum and maximum of four cross-products. This formula encloses
products of members because §64's `Rational` is **non-negative** and
multiplication is monotone in both inputs on that domain. It must
not be applied unchanged to signed rationals: negative factors
would invalidate the enclosure proof. All three operations are
total on inverted inputs. In particular, the endpoints of
`[1,1/2] + [1/2,1]` both denote `3/2`, despite the first input
being empty; no law says that arithmetic with an empty interval
always produces an empty one. The six laws below assert enclosure
**only when their membership premises hold**.

### 1.3 Six checked interval laws

The prospective module must export checked proof bodies for
**exactly these six interval laws**, generic in every displayed
interval or rational argument. `IsTrue` turns Boolean membership
into a proposition. The enclosure laws require **both** membership
premises and compare result values using the checked rational
operations. `rational_equiv` in `interval_point_sound` preserves
§64's value equality even when equivalent rationals have distinct
pair spellings. The final two equations use structural
`Equal Interval`: they follow from constructing exactly the same
endpoint terms on either side, not from quotienting rational
spellings.

```ken
pub theorem interval_add_encloses
  (i : Interval) (j : Interval) (x : Rational) (y : Rational) :
  IsTrue (interval_contains i x) →
  IsTrue (interval_contains j y) →
  IsTrue (interval_contains (interval_add i j) (rational_add x y))

pub theorem interval_mul_encloses
  (i : Interval) (j : Interval) (x : Rational) (y : Rational) :
  IsTrue (interval_contains i x) →
  IsTrue (interval_contains j y) →
  IsTrue (interval_contains (interval_mul i j) (rational_mul x y))

pub theorem interval_point_contains (x : Rational) :
  IsTrue (interval_contains (interval_point x) x)

pub theorem interval_point_sound (x : Rational) (y : Rational) :
  IsTrue (interval_contains (interval_point x) y) → rational_equiv x y

pub theorem interval_add_point (x : Rational) (y : Rational) :
  Equal Interval
    (interval_add (interval_point x) (interval_point y))
    (interval_point (rational_add x y))

pub theorem interval_mul_point (x : Rational) (y : Rational) :
  Equal Interval
    (interval_mul (interval_point x) (interval_point y))
    (interval_point (rational_mul x y))
```

The Architect's scratch development checked both generic enclosure
proofs using each side of each `bool_and` membership hypothesis and
the two Rational monotonicity proofs in §1.4. Point containment
uses reflexivity of rational order; point soundness uses
antisymmetry **up to `rational_equiv`**. Both point-operation laws
check by structural equality of their constructed intervals. Closed
seed instances exercise each law but do not establish its generic
quantifiers. No law here equates extensionally equal intervals or
states inclusion isotonicity for arbitrary interval pairs.

### 1.4 Rational dependency, trust, and deferrals

On the D0 base `8e0fc5911`, `Data.Numeric.Rational` exports
`rational_add`, `rational_mul`, `rational_leq` and the existing order
proofs, but **not** the two monotonicity theorems below. They are
new **Rational catalog obligations**, not seventh and eighth
interval laws. Each carries two independently required Boolean
inequality premises:

```ken
pub theorem rational_add_mono
  (x : Rational) (y : Rational) (z : Rational) (w : Rational) :
  Equal Bool (rational_leq x y) True →
  Equal Bool (rational_leq z w) True →
  Equal Bool (rational_leq (rational_add x z) (rational_add y w)) True

pub theorem rational_mul_mono
  (x : Rational) (y : Rational) (z : Rational) (w : Rational) :
  Equal Bool (rational_leq x y) True →
  Equal Bool (rational_leq z w) True →
  Equal Bool (rational_leq (rational_mul x z) (rational_mul y w)) True
```

A scratch copy of the existing Rational package with these two
checked public proofs and their private Nat-derived helpers makes
the six-law D0 check rc=0. With the landed Rational package, the
same D0 instead rejects `UnboundName rational_add_mono`. The
Rational exports, proof helpers, interval catalog publication, and
any coupled Rust acceptance or catalog-census updates belong to a
**separate catalog follow-on**; this spec WP edits none of them.
Neither a missing export nor a scratch proof is a shipped package.

The D0 interval source and Rational monotonicity appendix declare
no `Axiom`, postulate, or primitive, and need no new kernel or
surface feature. Their successful checks have **not** measured
`trusted_base()` before and after loading, including inherited
assumptions from imports; zero trusted-base delta is a catalog
validation obligation, not a result of this spec WP. Likewise,
the source-shaped conformance seed is not execution of a shipped
suite, Rust acceptance, catalog delivery, or CI.

Signed values and subtraction are deferred: signed multiplication
would require comparing four endpoint products. Division and
reciprocal intervals, particularly those spanning zero, are
deferred. Floating point, rounding modes and outward rounding,
width and diameter, interval intersection and hull, normalized
endpoint spellings, inclusion isotonicity, and sub-distributivity
are also outside §1. None is established by the six laws above.
