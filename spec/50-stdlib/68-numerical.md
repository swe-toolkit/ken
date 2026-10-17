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

## 2. Hull and intersection by containment

> **Section 2 status: DRAFT v0 (SPEC-NUMERICAL-INTERVAL-HULL).**
> Architect D0 checked the two operations and four generic laws below
> on the landed §1 base. Its Rational additions and interval proofs
> were in scratch copies, not published §2 catalog packages.

### 2.1 Endpoint formulas and empty results

`Algorithm.Numeric.RationalInterval` adds two total operations:

```ken
pub fn interval_hull (i : Interval) (j : Interval) : Interval
pub fn interval_intersect (i : Interval) (j : Interval) : Interval
```

For `i = MkInterval a b` and `j = MkInterval c d`, their exact
results are:

- `interval_hull i j = MkInterval (rational_min a c)
  (rational_max b d)`. The chosen lower bound is a minimum of the
  two lowers, and the chosen upper bound is a maximum of the uppers.
- `interval_intersect i j = MkInterval (rational_max a c)
  (rational_min b d)`. The chosen lower bound is a maximum of the
  two lowers, and the chosen upper bound is a minimum of the uppers.

Each operation uses the existing exact Boolean `rational_leq` order,
not a comparison of stored numerator and denominator fields. The
min/max operations select **stored Rational values** rather than
normalizing them. If `rational_leq x y` is `True`, the minimum
selects `x` and the maximum selects `y`; otherwise they select
`y` and `x`, respectively. Equivalent values with different
spellings need not produce structurally equal endpoints after
swapping the arguments. These scalar operations and their
order proofs are the separate Rational obligations in §2.3.

Intersection returns an `Interval`, **not** an `Option Interval`.
Disjoint inputs can give inverted bounds, already the ordinary
empty-interval representation in §1.1. For example, `[0,1/2]`
intersected with `[2,3]` produces `[2,1/2]`, with no members.
Their hull is `[0,3]`, containing endpoints from each interval as
well as gap points such as `1`; it excludes `4`. The operations
remain total for inverted inputs. Hull encloses any existing
members, but with an empty operand its endpoint formula need not
produce the least *member set* enclosing the operands: the empty
operand can still influence a stored bound. For example, the hull
of empty `[4,3]` and nonempty `[0,1]` is `[0,3]`, not `[0,1]`.
No minimality or empty-operand preservation law is claimed.

### 2.2 Four checked containment laws

The prospective extension must provide checked generic proof bodies
for exactly these four **public interval laws**. `IsTrue` turns
Boolean membership into a proposition; `And` combines the two
intersection conclusions. No premise assumes ordered endpoints.

```ken
pub theorem interval_hull_contains_left (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains i x) → IsTrue (interval_contains (interval_hull i j) x)

pub theorem interval_hull_contains_right (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains j x) → IsTrue (interval_contains (interval_hull i j) x)

pub theorem interval_intersect_sound (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains (interval_intersect i j) x)
  → And (IsTrue (interval_contains i x)) (IsTrue (interval_contains j x))

pub theorem interval_intersect_complete (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains i x) → IsTrue (interval_contains j x)
  → IsTrue (interval_contains (interval_intersect i j) x)
```

The hull laws carry each input's membership into the envelope.
Intersection soundness projects any result member into **both**
inputs; completeness admits any common member to the result. These
are membership implications, not equations between stored
intervals. The D0 checked the proofs by splitting the endpoints
and the two sides of Boolean membership, using existing rational
order transitivity with the §2.3 bounds. In particular, soundness
and completeness still apply when an input or result is empty:
without a member their corresponding implications have no false
witness to assert. Overlapping `[1/2,2]` and `[1,3]` intersect to
`[1,2]`; `1` and `3/2` are common members, but `1/2` is not in
their intersection.

These laws do **not** assert structural `Equal Interval` for
commutativity, idempotence or associativity of either operation.
No such law was proved by containment in D0 either. In particular,
the hull of the points at `half` and `two_quarters` stores
`half` as its lower bound and `two_quarters` as its upper bound;
swapping the arguments stores the reverse. Here `half = 1/2`
and `two_quarters = 2/4` are `rational_equiv` but not
structurally `Equal Rational`. The two hulls' structural equality
was **refused** by D0 with both `Refl` and `Proved`. Extensional
claims beyond the four displayed membership laws are not added.

### 2.3 Rational catalog prerequisites and evidence

`Data.Numeric.Rational` must separately export the following
operations and **six order proofs**. They are Rational catalog
obligations, not additional interval laws and not delivered in
this spec WP. Their comparison-based selection behavior is stated
in §2.1; the proofs have the Architect-checked public types:

```ken
pub fn rational_min (x : Rational) (y : Rational) : Rational
pub fn rational_max (x : Rational) (y : Rational) : Rational

pub proof leq_left for rational_min (x : Rational) (y : Rational) :
  IsTrue (rational_leq (rational_min x y) x)
pub proof leq_right for rational_min (x : Rational) (y : Rational) :
  IsTrue (rational_leq (rational_min x y) y)
pub proof greatest for rational_min (x : Rational) (y : Rational) (k : Rational) :
  IsTrue (rational_leq k x) → IsTrue (rational_leq k y)
  → IsTrue (rational_leq k (rational_min x y))

pub proof left_leq for rational_max (x : Rational) (y : Rational) :
  IsTrue (rational_leq x (rational_max x y))
pub proof right_leq for rational_max (x : Rational) (y : Rational) :
  IsTrue (rational_leq y (rational_max x y))
pub proof least for rational_max (x : Rational) (y : Rational) (k : Rational) :
  IsTrue (rational_leq x k) → IsTrue (rational_leq y k)
  → IsTrue (rational_leq (rational_max x y) k)
```

The existing public `refl`, `trans` and `total for rational_leq`
remain unchanged. The D0 checked the six new proofs in a scratch
Rational copy without an import change there; its private
`rational_pick` and `pick_*` helpers are proof structure, **not**
public APIs. A dependent match on a computed rational comparison
does not itself refine the needed occurrence, so the D0 helpers
bind the Boolean result explicitly with an equality premise. The
interval module adds only the two min/max names to its existing
§1 Rational import list, then uses their six public proofs. It must
not clone the Rational ordering functions or proofs locally.

On base `e07a73535`, Architect reported `ken check` rc 0 for both
scratch modules, four generic interval laws, and a separate
16-assertion closed positive client. Four independently appended
negative clients each rejected: false outside-hull membership,
disjoint/overlap false intersection membership, and structural
equality of the swapped hulls. Positive structural controls checked
for the two differing hull spellings. The scratch source had no
new `Axiom`, kernel/surface change, primitive or signed carrier.
A mutated `leq_left for rational_min` also failed to check. These
are **Architect-reported scratch results**, not an author-run Ken
check, shipped §2 catalog module or seed, Cargo test, CI result
or `trusted_base()` before/after measurement.

The §1 Rational monotonicity exports, interval package and §1 seed
are present on this landed base; §1.4's follow-on wording records
its earlier D0 state. The §2 Rational min/max exports and interval
extension remain catalog follow-ons. Signed interval arithmetic,
division, floating-point rounding, width, normalized endpoint
spellings, arbitrary-pair inclusion isotonicity and
sub-distributivity remain deferred.
