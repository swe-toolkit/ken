# Exact numerical intervals

> **Status: DRAFT v0 (SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT §1;
> SPEC-NUMERICAL-INTERVAL-HULL §2;
> SPEC-NUMERICAL-INCLUSION-ISOTONICITY §3;
> SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY §4;
> SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER §5;
> SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE §6;
> SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-BOUNDS §7).** The §1–2 operations,
> ten checked interval laws, required Rational exports and seeds are
> present on main. Section 3 specifies endpoint-wise inclusion and nine
> checked laws, now exported by `Algorithm.Numeric.RationalInterval` with
> kernel-checked bodies. Its conformance seed is present at
> `conformance/stdlib/numerical/seed-interval-inclusion.md` and the targeted
> package test runs its positive and two refused clients. Exact trusted-base
> identities are unchanged in the candidate's targeted test; this is not a
> claim of CI or a landed trust measurement. Section 4's four checked
> distributivity/factoring inclusions are now catalog exports, and a targeted
> package test exercises the authored seed's positive and refused clients.
> Package tests do not establish official conformance-runner execution, CI,
> publication, landing or a landed trust measurement. Section 5 specifies
> five new generic laws for transitivity and swapped-operand endpoint
> inclusion. Architect checked their bodies in isolated D0 with no new
> `Axiom`, Transparent ownership and unchanged scratch trusted identities
> (114 → 114). The §5 catalog exports and package test now check the generic
> types, shipped seed's accepted client and two refused clients, with an exact
> package-local trusted-identity delta of 114 → 114. This does not establish
> official conformance-runner execution, CI, landing or a landed trust delta.
> Section 6's four generic mutual-inclusion laws for addition and
> multiplication regrouping have checked catalog exports and a
> package-owned seed test on main. Architect's earlier isolated D0 on
> `edc1da359` had no new `Axiom` and unchanged scratch trusted GlobalIds
> (114 → 114); neither it nor the package test establishes official
> conformance or landed trust. Section 7 specifies two endpoint-order
> hull/intersection bounds laws. Architect's isolated D0 on `1363b09b4`
> checked both generic proofs with no new `Axiom` and unchanged scratch
> trusted GlobalIds (114 → 114). The §7 catalog exports were absent on
> the spec author's base; this separate Foundation candidate adds both
> checked laws and a package-owned test of the complete accepted seed
> and isolated refusal. Candidate-local checks are not official
> conformance-runner execution, CI, publication, landing or landed trust.

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

The landed `Algorithm.Numeric.RationalInterval` exports checked
proof bodies for these six §1 interval laws, generic in every
displayed interval or rational argument. `IsTrue` turns Boolean membership
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
quantifiers. No §1 law equates extensionally equal intervals or
states inclusion isotonicity for arbitrary interval pairs; §3
specifies the latter separately.

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

A scratch copy of the Rational package with these two checked
public proofs and their private Nat-derived helpers made the six-law
D0 check rc=0. On the original D0 base, before the exports landed,
the same development rejected `UnboundName rational_add_mono`.
The Rational exports and §1 interval catalog module have since
landed. Neither that historical failure nor the scratch proof alone
established a shipped package or its trust delta.

The historical D0 interval source and Rational appendix declared
no `Axiom`, postulate, or primitive, and needed no new kernel or
surface feature. Their scratch checks did **not** measure
`trusted_base()` before and after loading, including inherited
assumptions from imports; they are not a landed trust measurement.
The §1 module and seed are now present on main, but that D0 did not
execute a shipped seed or establish CI.

Signed values and subtraction are deferred: signed multiplication
would require comparing four endpoint products. Division and
reciprocal intervals, particularly those spanning zero, are
deferred. Floating point, rounding modes and outward rounding,
width and diameter, normalized endpoint spellings, and
sub-distributivity remain outside §§1–3. Section 2 separately
specifies hull and intersection; §3 specifies inclusion isotonicity.
Neither follows from the six §1 laws alone.

## 2. Hull and intersection by containment

> **Section 2 status: DRAFT v0 (SPEC-NUMERICAL-INTERVAL-HULL).**
> Architect D0 checked the two operations and four generic laws below
> on the then-landed §1 base. The §2 Rational additions, interval
> proofs and seed have since landed; the historical D0 alone does not
> establish shipped seed-harness execution, CI or a landed trust delta.

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

The landed §2 extension provides checked generic proof bodies for
these four **public interval laws**. `IsTrue` turns
Boolean membership into a proposition. `And` here is the ambient
Ω-valued conjunction with `and_intro` and its two projections,
not the distinct `Type`-valued `Core.Logic.And` carrier. No premise
assumes ordered endpoints.

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
The §5 endpoint-value swap inclusions do not strengthen this to
structural equality; §6 separately states arithmetic associativity
by endpoint-value inclusion, not structural interval equality. In particular,
the hull of the points at `half` and `two_quarters` stores
`half` as its lower bound and `two_quarters` as its upper bound;
swapping the arguments stores the reverse. Here `half = 1/2`
and `two_quarters = 2/4` are `rational_equiv` but not
structurally `Equal Rational`. The two hulls' structural equality
was **refused** by D0 with both `Refl` and `Proved`. Extensional
claims beyond the four displayed membership laws are not added.

### 2.3 Rational catalog prerequisites and evidence

`Data.Numeric.Rational` separately exports the following
operations and **six order proofs** on main. They were Rational
catalog obligations, not additional interval laws. Their
comparison-based selection behavior is stated in §2.1; the proofs
have the Architect-checked public types:

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
check, executed shipped §2 seed, CI result or landed trust measurement.

The §1–2 Rational exports, interval catalog module and two seeds are
present on main. Signed interval arithmetic, division, floating-point
rounding, width and normalized endpoint spellings remain deferred.
Sub-distributivity is specified separately in §4. Arbitrary-pair
inclusion isotonicity is specified in §3; §2's four laws alone do not
supply it.

## 3. Interval inclusion and endpoint isotonicity

> **Section 3 status: DRAFT v0 (SPEC-NUMERICAL-INCLUSION-ISOTONICITY).**
> Architect D0 checked the following public operation and nine generic
> laws on the landed §1–2 base. The §3 catalog exports are present and
> their checked bodies, public types and positive/refused seed clients pass
> targeted package tests. The seed is present at
> `conformance/stdlib/numerical/seed-interval-inclusion.md`. These local
> results do not claim CI, publication or a landed trust measurement.

### 3.1 Inclusion by endpoint comparisons

`interval_inside i k` means the endpoints of `i` lie inside the
endpoints of `k`, in the order appropriate to a containing interval.
Its complete public Boolean definition uses the existing checked
`rational_leq` and conjunction on the **values** of the endpoints,
not structural `Equal Rational` or a normalized spelling:

```ken
pub fn interval_inside (i : Interval) (k : Interval) : Bool =
  match i {
    MkInterval a b ↦
      match k {
        MkInterval c d ↦ bool_and (rational_leq c a) (rational_leq b d)
      }
  }
```

Thus `i=[a,b]` lies inside `k=[c,d]` exactly when `c ≤ a` and
`b ≤ d`. The operation **does not inspect membership or validity**
and does not special-case an inverted `i`. For example, `[2,1]`
lies inside `[0,4]` by its endpoints, but the empty `[1/2,0]` is
not inside `[1,3]`: its lower comparison `1 ≤ 1/2` is false.
Both have no members. The test is sufficient for inclusion of
member sets, but it is not necessary without a point in `i`.
The point interval `[1/2,1/2]` lies inside `[2/4,2/4]` by rational
value order, even though the endpoint spellings differ structurally.

### 3.2 Nine checked-law obligations

The module must export checked proof bodies for exactly these nine
new public theorem types. `IsTrue` turns each Boolean result into an
Ω proposition. Soundness carries every member of an included inner
interval into the outer interval; reflexivity includes inverted
intervals without identifying different representations of emptiness:

```ken
pub theorem interval_inside_sound
  (i : Interval) (k : Interval) (x : Rational) :
  IsTrue (interval_inside i k) →
  IsTrue (interval_contains i x) →
  IsTrue (interval_contains k x)

pub theorem interval_inside_refl (i : Interval) :
  IsTrue (interval_inside i i)

pub theorem interval_add_inside
  (i : Interval) (i2 : Interval) (j : Interval) (j2 : Interval) :
  IsTrue (interval_inside i i2) →
  IsTrue (interval_inside j j2) →
  IsTrue (interval_inside (interval_add i j) (interval_add i2 j2))

pub theorem interval_mul_inside
  (i : Interval) (i2 : Interval) (j : Interval) (j2 : Interval) :
  IsTrue (interval_inside i i2) →
  IsTrue (interval_inside j j2) →
  IsTrue (interval_inside (interval_mul i j) (interval_mul i2 j2))

pub theorem interval_inside_hull_left (i : Interval) (j : Interval) :
  IsTrue (interval_inside i (interval_hull i j))

pub theorem interval_inside_hull_right (i : Interval) (j : Interval) :
  IsTrue (interval_inside j (interval_hull i j))

pub theorem interval_intersect_inside_left (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_intersect i j) i)

pub theorem interval_intersect_inside_right (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_intersect i j) j)

pub theorem interval_inside_complete
  (i : Interval) (k : Interval) (q : Rational) :
  IsTrue (interval_contains i q) →
  ((x : Rational) → IsTrue (interval_contains i x) →
    IsTrue (interval_contains k x)) →
  IsTrue (interval_inside i k)
```

The addition and multiplication laws are **isotonic in both interval
arguments**: when each input lies inside its corresponding larger
input, endpoint-wise arithmetic preserves both comparisons. They
use the `rational_add_mono` and `rational_mul_mono` laws specified
in §68 §1.4 and exported by `Data.Numeric.Rational` for exact
**non-negative** values. `rational_mul_mono` needs no separate sign
premise on this carrier. Neither law assumes ordered endpoints or
changes §1's total, corresponding-endpoint formulas. No signed arithmetic,
subtraction, four-cross-product multiplication, or rounding follows.

The hull and intersection inclusions are **unconditional corner
facts**, even for inverted inputs or a disjoint intersection. The
§2 min/max endpoint formulas give each input bounds inside the hull
and the intersection's bounds inside each input. These are Boolean
inclusion statements, not structural `Equal Interval`, a member-set
quotient, a least hull for an empty operand, or a canonical empty
interval. Equivalent rational values may still have different
stored spellings.

The final theorem is a **witnessed converse** of soundness. A member
`q` of `i=[a,b]` supplies `a ≤ q ≤ b`; hence both `a` and `b` are
members of `i`. Pointwise member inclusion places both endpoints in
`k=[c,d]`, giving `c ≤ a` and `b ≤ d`. Without `q`, pointwise
inclusion is vacuous for inverted `[1/2,0]`, even though
`interval_inside [1/2,0] [1,3]` is `False`. No unwitnessed converse,
equivalence of member sets, or structural equality of intervals is
promised. Transitivity is not among these nine §3 laws; §5 states
it separately, without a validity or membership premise.

### 3.3 Evidence and boundary

On the landed base `2f3977991d7f678020f69bb93835472985a20db4`,
Architect D0 checked the complete §3 development in a scratch copy
of `Algorithm.Numeric.RationalInterval`: package elaboration rc 0,
no `Axiom` or `Opaque`, ten new owned Transparent declarations,
and matching `trusted_base()` identity sets of 114 entries before
and after loading it. A public-only scratch client checked nested
and self inclusion, soundness, both isotonicity instances, hull and
intersection cases, an inverted intersection, equivalent rational
endpoint spellings, and the witnessed converse. The wrong claim
that `[1,3]` lies inside `[2,2]` was kernel-refused. Two inverted
closed examples distinguish the corner test from vacuous member
inclusion. Existing §1 and §2 targeted Rust suites each passed 3/3
on the scratch package; the ambient-name census row remained
`[And, and_fst, and_intro, and_snd]` with no new ambient name.

The Architect's D0 results alone did not establish catalog delivery,
seed execution or a landed trust measurement. The §3 package and its targeted
acceptance now check the public exports, the authored seed and exact
trusted-base identity equality on the candidate; no CI or landed trust
delta is claimed. This contract adds no primitive,
kernel/surface feature, signed carrier, subtraction,
`rational_equiv`-as-`Equal`, canonical empty interval, or member-set
equality.

## 4. Distributivity as endpoint-value inclusion

The §1.2 total endpoint operations and §3.1 `interval_inside` now
support distributivity in both operand positions. Let `i`, `j`, and
`k` be any intervals, including intervals with inverted bounds.
Write `L` for `interval_mul i (interval_add j k)` and `R` for
`interval_add (interval_mul i j) (interval_mul i k)`. The public
checked left-hand laws state **both** endpoint inclusions:

```ken
pub theorem interval_distrib_l_inside
  (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_mul i (interval_add j k))
    (interval_add (interval_mul i j) (interval_mul i k)))

pub theorem interval_factor_l_inside
  (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_add (interval_mul i j) (interval_mul i k))
    (interval_mul i (interval_add j k)))
```

For the right-hand orientation, let `Lr` be
`interval_mul (interval_add j k) i` and `Rr` be
`interval_add (interval_mul j i) (interval_mul k i)`.
The module exports a **separate pair** of checked laws:

```ken
pub theorem interval_distrib_r_inside
  (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_mul (interval_add j k) i)
    (interval_add (interval_mul j i) (interval_mul k i)))

pub theorem interval_factor_r_inside
  (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_add (interval_mul j i) (interval_mul k i))
    (interval_mul (interval_add j k) i))
```

Each inclusion is the §3 endpoint comparison, not a statement that
arithmetic with an empty interval preserves emptiness. For each
orientation, the two directions force mutual `rational_leq` between
corresponding lower endpoints and between corresponding upper
endpoints. By §64 §2.2 antisymmetry, each pair of endpoint **values**
is `rational_equiv`. Thus these laws give equality of endpoint values
without asserting `Equal Interval`: §64 operations can retain
distinct spellings for equivalent rational values. No normalization, quotient
carrier, structural distributivity theorem, or interval commutativity
law is added **in §4**. Section 5 adds swapped-operand inclusion for
endpoint values, not structural `Equal Interval`. In particular, the
**right** laws here are not obtained by swapping interval factors
through a §4 interval commutativity law; §64's `comm for rational_mul`
supports a private right-hand Rational derivation from `distrib_l for
rational_mul`.

The checked development uses two private proof stages: `equiv_leq`
carries `rational_equiv x y` into `IsTrue (rational_leq x y)` through
§64's respect and reflexivity, while `rational_distrib_r` derives
the right-hand Rational value equality using multiplication
commutativity, left distributivity, equivalence transitivity and
addition respect. Neither is a public helper or a restatement of
§64's public distributivity law. The four interval laws require no
validity or membership premise and add no primitive, Rational
public declaration, signed endpoint, subtraction or division.

On base `826a4fed0`, the Architect's AC-0 scratch development
checked all four generic theorem bodies with rc 0, no `Axiom`, and
no open obligation. The generic `Refl` claim for structural
`Equal Interval` was refused; a closed point-`1/2` structural
equality claim was also refused. This is evidence for the chosen
endpoint-value boundary, not delivery of §4 catalog exports, seed
execution, CI, publication, landing, or a landed `trusted_base()`
measurement. The separate AC-2 seed uses distinct-endpoint
intervals and a same-input accepting control against a dropped-product
refusal. The checked §4 exports are now in the catalog. A package-owned test
runs the unchanged seed Ken fences, confirms that the refused claim is the
paired dropped-product proposition, and measures exact trusted-base identity
sets before and after package loading. The unchanged RationalInterval census
row passes the targeted strict-resolution suite with no D0 residuals. These
are package-test measurements, not official conformance-runner execution,
CI, publication, landing or a landed trust measurement. The former §1–3
sub-distributivity deferral is settled **here**, not by revising any §1–3
operation or law.

## 5. Endpoint inclusion order and operand swaps

`Algorithm.Numeric.RationalInterval` extends the §3 endpoint test
with one transitivity theorem and four operand-swap theorems. These
are five **public checked-law obligations**, with exactly the
following types:

```ken
pub theorem interval_inside_trans
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside i j)
      → IsTrue (interval_inside j k)
      → IsTrue (interval_inside i k)

pub theorem interval_add_comm_inside (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_add i j) (interval_add j i))

pub theorem interval_mul_comm_inside (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_mul i j) (interval_mul j i))

pub theorem interval_hull_comm_inside (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_hull i j) (interval_hull j i))

pub theorem interval_intersect_comm_inside (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_intersect i j) (interval_intersect j i))
```

Transitivity composes both §3.1 endpoint comparisons: if
`i=[a,b]` lies inside `j=[c,d]` and `j` lies inside `k=[e,f]`,
then `e ≤ c ≤ a` and `b ≤ d ≤ f`. It applies also to inverted
intervals, because the test compares stored endpoint **values**,
not the existence of a member. The proof uses §64's checked
transitivity of `rational_leq` independently on the lower and
upper endpoints. No interval validity or membership premise is
added, and the witnessed converse in §3.2 remains witnessed.

For every operation, the public swap law compares **the result
of the operands in the written order** with **the result in reverse
order** by `interval_inside`; its endpoints need not be stored
identically. Applying the same public theorem at `(j, i)` gives
the opposite inclusion. These two inclusions establish mutual
endpoint-value order, not `Equal Interval` between the results.
The addition and multiplication proofs transport §64's
`rational_equiv` commutativity of the endpoint operations into
`rational_leq` using the existing private `equiv_leq`. The hull
and intersection proofs use §2.3's public min/max order bounds,
not structural commutativity of those selectors. The private
`equiv_leq` and `rational_distrib_r` remain unexported and
unchanged; there is no new Rational declaration or proof helper.

These four swaps are unconditional, including when either operand
or an intersection has inverted endpoints. In particular, a
reversed `interval_intersect` on disjoint inputs can be empty
while still satisfying the endpoint comparisons. This does not
assert equality of member sets as the definition of inclusion,
canonical emptiness, structural commutativity, or any law for
signed, approximate or divided intervals. It changes none of
the §1–4 operations or laws, and adds no primitive or kernel rule.

On base `d6c05223e`, Architect's isolated AC-0 patch checked all
five generic proof bodies, their ownership and Transparent status,
and a public-only client at `trusted_base()` identities 114 → 114;
no new `Axiom` was present. The separate closed chain and operand
pair tested in that D0 illustrate the obligations but do not prove
the generic quantifiers. The §5 catalog exports and a targeted package test
now check the
five generic public types and run the seed's accepted and two isolated
refused Ken clients against the roots-loaded package. The package test
observes unchanged exact trusted-base identities (114 → 114), not a
landed trust measurement. Neither the historical D0 nor the package
test is official conformance-runner execution, CI or landing.

## 6. Associativity by endpoint-value inclusion

> **Section 6 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE).** Architect's isolated
> D0 checked four generic proof bodies on `edc1da359` with no new
> `Axiom` or trusted GlobalId. All four Transparent laws and the
> targeted package test are on main: the test root-loads the generic
> client and accepted §6 seed, rejects the isolated false inclusion,
> and measures no local trust delta. This package-test evidence is not
> official conformance or landed trust.

For every three intervals `i`, `j`, `k`, including inverted ones,
`Algorithm.Numeric.RationalInterval` must export these **four**
checked laws, with exactly these public names, argument orders, and
result types:

```ken
pub theorem interval_add_assoc_inside (i : Interval) (j : Interval) (k : Interval)
  : IsTrue (interval_inside (interval_add i (interval_add j k)) (interval_add (interval_add i j) k))

pub theorem interval_add_assoc_rev_inside (i : Interval) (j : Interval) (k : Interval)
  : IsTrue (interval_inside (interval_add (interval_add i j) k) (interval_add i (interval_add j k)))

pub theorem interval_mul_assoc_inside (i : Interval) (j : Interval) (k : Interval)
  : IsTrue (interval_inside (interval_mul i (interval_mul j k)) (interval_mul (interval_mul i j) k))

pub theorem interval_mul_assoc_rev_inside (i : Interval) (j : Interval) (k : Interval)
  : IsTrue (interval_inside (interval_mul (interval_mul i j) k) (interval_mul i (interval_mul j k)))
```

The plain `_assoc_inside` names follow §64's public `assoc for
rational_add` and `assoc for rational_mul`: **right-nested inside
left-nested**. The `_assoc_rev_inside` names give the reverse
inclusion. Two laws per operation are required. Unlike §5's
swapped-operand law, changing the three arguments of the forward
law cannot reverse its nesting. Neither direction is merely an
instance of the other; a client must not have to derive the missing
public library law in its seed. Each theorem is premise-free and
compares endpoint **values** using §3.1's `interval_inside`; it
requires neither ordered endpoints nor a nonempty interval.

Mutual inclusion means both corresponding lower endpoint values and
both upper endpoint values compare in both directions. By §64's
antisymmetry these values are `rational_equiv`, but this contract
does not assert generic structural `Equal Interval`. The package's
arithmetic-law vocabulary uses endpoint-value inclusion. A generic
structural associativity theorem would need a new `Data.Numeric.Rational`
export, excluded by this WP's stop condition. Closed regroupings
**do** coincide structurally: their stored rational components are
closed Nat arithmetic expressions and normalize to the same numerals,
so the seed's `Equal Interval` claims accept by `Refl` for both
operations. For open operands the regroupings are **not**
definitionally convertible (`Refl` was refused in D0 for both);
a closed computation therefore cannot stand in for these generic
checked laws. Do not infer from §1.1's different-spelling examples
that these particular **closed** regroupings have different spellings.

The proofs destructure the three interval arguments and reuse the
existing private `equiv_leq`, §64's public rational associativity
proofs, their equivalence symmetry, and Boolean conjunction
introduction. The lower and upper comparisons require opposite
orientations of rational associativity; the reverse laws exchange
those orientations. No new helper is added. The four public
obligations belong directly after `interval_intersect_comm_inside`
and before private `equiv_leq` in the catalog package; that placement
is an obligation for the separate catalog half, not a catalog edit
in this spec candidate. Sections 1–5's operations and laws are
unchanged. There is no new Rational export, axiom, primitive,
kernel or surface change, signed arithmetic or rounding.

The authored closed seed applies all four laws to `[1/2,1]`,
`[1/3,2]` and `[3/2,3]`, independently checks two direct inclusion
values and both structural coincidences, and isolates the false
claim that their three-operand sum lies inside the first two-operand
sum. Its true `IsTrue (interval_inside _ (interval_add i j)) = Proved`
control and independent `False` value establish that the refusal
comes from the false endpoint comparison rather than the proof form.
These closed checks are examples, not proofs of the generic laws.
The Architect's earlier D0 checked a proposed implementation. The
landed catalog's package test checks the §6 seed against its exports;
neither observation is official conformance or landed trust.

## 7. Hull and intersection bound endpoint inclusion

> **Section 7 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-BOUNDS).** Architect's
> isolated D0 on `1363b09b4` checked both generic Transparent proofs,
> their public clients and closed controls, with no new `Axiom`,
> `ken check` and `ken fmt --check` rc 0, and equal scratch
> `trusted_base()` GlobalId sets (114 → 114). Neither §7 catalog
> export was present on the spec author's base. The separate Foundation
> candidate now exports both checked laws, root-loads them and tests the
> complete accepted conformance seed and isolated false hull claim.
> These package checks are not official conformance-runner execution,
> CI, publication, landing or landed trust.

### 7.1 Two premise-bearing public laws

`Algorithm.Numeric.RationalInterval` must export the following public
Transparent theorems immediately after §6's
`interval_mul_assoc_rev_inside`. They use §3.1's endpoint-value
`interval_inside` and no new membership or equality relation. Each
premise is an arrow from `IsTrue` of an endpoint-inclusion test; the
premises are supplied in the same order as their corresponding
operands.

```ken
pub theorem interval_hull_least
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside i k)
      → IsTrue (interval_inside j k)
      → IsTrue (interval_inside (interval_hull i j) k)

pub theorem interval_intersect_greatest
      (k : Interval) (i : Interval) (j : Interval)
    : IsTrue (interval_inside k i)
      → IsTrue (interval_inside k j)
      → IsTrue (interval_inside k (interval_intersect i j))
```

The hull law receives its two operands first, then their common outer
bound `k`: if each lies inside `k`, their hull does too. The
intersection law receives the common inner bound `k` first, then
the two intervals: if `k` lies inside each, it lies inside their
intersection. Neither conclusion alone supplies either premise, and
neither orientation is a claim that a hull lies inside either of its
operands or that an intersection lies inside every unrelated
interval. The laws include inverted intervals, since the endpoint
test compares stored values even when the represented member set is
empty. There is no validity, nonempty-membership, distinctness, or
structural `Equal Interval` premise.

### 7.2 Proof and evidence boundary

The proofs destructure the three intervals and combine their two
endpoint comparisons with the existing public
`(proof greatest for rational_min)` and
`(proof least for rational_max)` in opposite positions for hull and
intersection. They introduce no helper. The existing private
`equiv_leq` and `rational_distrib_r` remain private, and all §1–§6
operations and laws remain unchanged. The two checked bodies and
package-owned test are now in the separate Foundation catalog candidate,
not the spec author's original status candidate. No new Rational export,
axiom, postulate, kernel rule, surface feature, or trust entry is specified.

In Architect's D0 closed fixture, `low = [1/2,1]` and
`high = [2,3]` both lie inside `outer = [0,4]`, and their hull
`[1/2,3]` lies inside that strictly wider outer bound. The interval
`narrow = [2,3]` lies inside both `[0,3]` and `[1,4]`, and thus
inside their intersection `[1,3]`, which is strictly wider than
`narrow`. An inverted `[4,0]` exercises both laws under the same
endpoint-value relation. Independent closed `Proved` controls
compute the premise and conclusion values; an accepted same-form
control and a separately computed `False` isolate the false claim
that the hull of `low` and `high` lies inside `low`. The refusal
cannot supply the missing inclusion premise. These finite checks
illustrate the orientations, not proofs of the generic laws. The
D0's scratch package and client were not shipped §7 execution. The
separate Foundation candidate now checks the authored seed against its
exports in a package-owned test, not an official conformance-runner result.
