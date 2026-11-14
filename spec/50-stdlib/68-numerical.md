# Exact numerical intervals

> **Status: DRAFT v0 (SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT §1;
> SPEC-NUMERICAL-INTERVAL-HULL §2;
> SPEC-NUMERICAL-INCLUSION-ISOTONICITY §3;
> SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY §4;
> SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER §5;
> SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE §6;
> SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-BOUNDS §7;
> SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ABSORPTION §8;
> SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ASSOCIATIVITY §9;
> SPEC-NUMERICAL-INTERVAL-POINT-UNITS §10;
> SPEC-NUMERICAL-INTERVAL-POINT-ZERO-ABSORPTION §11;
> SPEC-NUMERICAL-INTERVAL-CONTAINS-RESPECT §12;
> SPEC-NUMERICAL-INTERVAL-INVERTED-EMPTY §13;
> SPEC-NUMERICAL-INTERVAL-INSIDE-ANTISYMMETRY §14;
> SPEC-NUMERICAL-INTERVAL-WIDTH §15;
> SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO §16).** The §1–2
> operations,
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
> trusted GlobalIds (114 → 114). Both §7 checked exports and a package-owned
> test are present on this base; the test checks the complete accepted seed
> and isolated false hull claim. These package checks are not official
> conformance-runner execution, CI, publication, landing or landed trust.
> Section 8's two checked absorption laws, corrected CV seed and
> package-owned test are now on main. Architect's isolated D0 had equal
> scratch trusted identities 114 → 114; earlier candidate-local checks
> do not establish official conformance or measured landed trust.
> Section 9's four checked regrouping laws, authored seed and
> package-owned test are now on main. Architect AC-0 checked the generic
> proofs in scratch without a new `Axiom` and with equal trusted
> identities 114 → 114. The named package target previously passed 2/2
> with candidate-local trusted identities 114 → 114; CV statically
> checked the seed but did not run it as Ken or through the official
> conformance runner. Neither result establishes an official runner
> execution or measured landed trust. Section 10's four public
> point-unit inclusions, authored CV seed and package-owned test are
> present on this base. Architect AC-0 checked them earlier in scratch
> with equal trusted identities 114 → 114; neither that result nor the
> package test establishes official conformance-runner execution or
> measured landed trust. Section 11 states two further left-point-zero
> multiplication laws. Architect AC-0 checked their proposed bodies
> on `70dc30f97` in scratch without `Axiom` and with equal trusted
> identities 114 → 114. The historical Spec-only child `bdc3e5b47`
> stated the types without catalog exports, a package test or a CV seed.
> Foundation reports that its assembled working tree contains the two
> checked laws, package-owned test and corrected CV seed; the named
> package test passed 3/3 using the actual seed fence with trusted
> GlobalId identities 114 → 114. That working-tree report is not an
> exact committed-tip gate and does not establish official conformance-
> runner execution, CI, publication, landing or measured landed trust.
> Section 16 specifies width under inclusion and its Nat and Rational
> prerequisites. Architect's isolated AC-0 on `750f1622b` checked all
> seven public laws with equal scratch trusted-base identity sets
> 114 → 114. This does not establish an assembled-tip check, official
> conformance, CI or measured landed trust for §16.

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

Signed values, signed subtraction and interval subtraction remain
deferred; §15 separately specifies truncated subtraction on
non-negative `Rational`. Signed multiplication would require
comparing four endpoint products. Division and reciprocal intervals,
particularly those spanning zero, are deferred. Floating point,
rounding modes and outward rounding, width (specified in §15) and
diameter, normalized endpoint spellings, and sub-distributivity
remain outside §§1–3. Section 2 separately specifies hull and
intersection; §3 specifies inclusion isotonicity.
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
rounding, diameter and normalized endpoint spellings remain deferred.
Width is specified separately in §15. Sub-distributivity is specified
separately in §4. Arbitrary-pair
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
> `trusted_base()` GlobalId sets (114 → 114). Both §7 catalog exports
> and a package-owned test are present on this base. The test root-loads
> the laws, checks the complete accepted conformance seed and refuses
> the isolated false hull claim. These package checks are not official
> conformance-runner execution, CI, publication, landing or landed trust.

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
package-owned test are present on this base, having been added in the
separate Foundation candidate, not the spec author's original turn.
No new Rational export, axiom, postulate, kernel rule, surface feature,
or trust entry is specified.

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
package-owned test checks the authored seed against the §7 exports,
not as an official conformance-runner result.

## 8. Hull and intersection absorption by endpoint-value inclusion

> **Section 8 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ABSORPTION).** Architect's
> isolated D0 on `fd2f575576aa2f5893054dcaf89e38243b739edc` checked
> both generic proof bodies and a public client (`ken check` and
> `ken fmt --check` rc 0), with no new `Axiom` and equal scratch
> `trusted_base()` GlobalId sets (114 → 114). This assembled candidate
> adds both §8 checked catalog exports, the corrected CV seed and a
> package-owned test. D0 remains scratch evidence; reported candidate-local
> tests and mutations are not official conformance-runner execution, CI,
> publication, landing or landed trust.

For any two intervals `i` and `j`, including inverted ones, each of
these absorption pairs is **mutually included by endpoint value**:
`interval_intersect i (interval_hull i j)` and `i`; and
`interval_hull i (interval_intersect i j)` and `i`. The comparison is
§3.1's `interval_inside`: it compares exact rational endpoint values,
not stored endpoint spellings or member sets. No validity, nonempty
membership, ordering, or distinctness premise is needed. In particular,
these are not structural `Equal Interval` laws; equivalent rational
endpoints can retain distinct spellings.

The two compositional directions require exactly two new public
Transparent laws, with the shared interval `i` first:

```ken
pub theorem interval_inside_intersect_hull_left
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside i (interval_intersect i (interval_hull i j)))

pub theorem interval_hull_intersect_inside_left
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_hull i (interval_intersect i j)) i)
```

The other two directions are already single applications of §3's
public bounds, not new theorem names. For the same `i` and `j`, their
public applications and result types are:

```ken
interval_intersect_inside_left i (interval_hull i j)
  : IsTrue (interval_inside (interval_intersect i (interval_hull i j)) i)

interval_inside_hull_left i (interval_intersect i j)
  : IsTrue (interval_inside i (interval_hull i (interval_intersect i j)))
```

Thus each absorption is mutual inclusion, with one new compositional
law and one existing-law instance. The first new law composes §7's
`interval_intersect_greatest i i (interval_hull i j)` with
`interval_inside_refl i` and `interval_inside_hull_left i j`. The
second composes §7's `interval_hull_least i (interval_intersect i j) i`
with `interval_inside_refl i` and `interval_intersect_inside_left i j`.
Neither proof requires a helper, an `Axiom`, or a Rational export;
private `equiv_leq` and `rational_distrib_r` remain private and unused.
The new laws sit immediately after `interval_intersect_greatest`
and before private `equiv_leq` in the catalog.
All §1–§7 operations and law types remain unchanged.

The assigned closed seed is
`conformance/stdlib/numerical/seed-interval-hull-intersect-absorption.md`.
Architect's D0 used overlapping `[0,2]` and `[1,3]`, with distinct
endpoints and neither interval inside the other. The accepted client
applies each of the four public directions and separately checks each
closed inclusion value by `Proved`. Moving the shared argument in
`interval_hull (interval_intersect i j) j` breaks absorption: for those
inputs it is `[1,3]`, which is not inside `[0,2]`. That false claim
rejects both a `Proved` body and a body using
`interval_hull_intersect_inside_left i j`; a same-shape true control
and an independently computed `False`
isolate the false endpoint comparison rather than a malformed proof
form. The closed checks illustrate the four directions; they do not
prove the generic laws. The package-owned
`cat_numeric_interval_hull_intersect_absorption` test checks the seed's
accepted client and isolated refusal against the two §8 catalog laws; it
is not official conformance-runner execution.

## 9. Hull and intersection regroup by endpoint-value inclusion

> **Section 9 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ASSOCIATIVITY).** Architect's
> isolated AC-0 on `903b06068d7c67c74ab9d979513c31c0c7572b31`
> checked all four generic obligations and closed controls with rc 0,
> no `Axiom`, and equal scratch `trusted_base()` GlobalId sets
> (114 → 114). This assembled candidate has the four checked catalog
> exports, a package-owned test and the authored
> `conformance/stdlib/numerical/seed-interval-hull-intersect-associativity.md`.
> The named `cat_numeric_interval_hull_intersect_associativity` test
> passed 2/2 with reported candidate-local trusted identity sets
> 114 → 114. CV checked the new seed statically, not with Ken or the
> official conformance runner. D0 remains isolated scratch evidence;
> these results establish neither official conformance, CI,
> publication, landing nor landed trust.

For every three intervals `i`, `j`, `k`, including inverted intervals,
`Algorithm.Numeric.RationalInterval` must export the following four
public Transparent laws in the displayed order, immediately after
`interval_hull_intersect_inside_left` and before private `equiv_leq`.
Arguments are `i`, `j`, `k`; there are no premises:

```ken
pub theorem interval_hull_assoc_inside (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside (interval_hull i (interval_hull j k)) (interval_hull (interval_hull i j) k))

pub theorem interval_hull_assoc_rev_inside (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside (interval_hull (interval_hull i j) k) (interval_hull i (interval_hull j k)))

pub theorem interval_intersect_assoc_inside (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside (interval_intersect i (interval_intersect j k)) (interval_intersect (interval_intersect i j) k))

pub theorem interval_intersect_assoc_rev_inside (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside (interval_intersect (interval_intersect i j) k) (interval_intersect i (interval_intersect j k)))
```

As in §6, plain `_assoc_inside` puts the **right-nested** result inside
the left-nested one; `_assoc_rev_inside` states the reverse. Two laws
per operation are public, rather than expecting clients to derive a
missing direction from a single export. §3.1's `interval_inside`
compares the corresponding exact rational endpoint **values**, not
interval spellings or member sets. The results give mutual
endpoint-value inclusion for both hull and intersection regroupings,
even when
an input is inverted. They do not assert structural `Equal Interval`
between the regroupings or require ordered, nonempty or otherwise
valid inputs. The §2.2 warning against structural hull/intersection
associativity remains in force.

The proofs compose only existing public interval facts: §7's
`interval_hull_least` and `interval_intersect_greatest`, §3's left and
right hull/intersection inclusions, and `interval_inside_trans`.
Hull least collects the two nested operands after transitive inclusion;
intersection greatest bounds the nested result by both operands.
Reversing the grouping reverses those compositions. No new helper,
Rational law or endpoint case split is required; private `equiv_leq` and
`rational_distrib_r` remain private and unused. Sections 1–8 retain all
operation and law types. There is no new axiom, kernel or surface rule.

The authored CV seed uses `i=[1/2,1]`, `j=[1/3,2]` and
`k=[3/2,3]` to apply each public law. Its isolated refusal drops `k`
from the asserted containing hull: `interval_hull i (interval_hull j k)` is
`[1/3,3]`, whereas `interval_hull i j` is `[1/3,2]`. The former is
not inside the latter by endpoint value. Architect's D0 separately
accepted the `False` value for that inclusion and refused its `IsTrue`
claim at the theorem, while also checking an inverted interval in each
operand position. Those D0 observations remain isolated scratch
results. CV statically checked the authored seed but did not run it as
Ken or through the official conformance runner; the package-owned
test is candidate-local evidence, not execution of that seed.

## 10. Point zero and point one as interval units

> **Section 10 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-POINT-UNITS).** Architect AC-0 on exact
> base `86b7cac5e` checked four generic laws with `ken check` and
> `ken fmt --check` rc 0, no added `Axiom`, and equal scratch
> `trusted_base()` identity sets 114 → 114. The four checked catalog
> exports, CV-authored seed and package-owned test are now on this
> base. The package test checks its own generic and closed clients,
> not execution of the separate seed through the official conformance
> runner. Neither scratch evidence nor package checks establish an
> official conformance-runner result or measured landed trust.

`Algorithm.Numeric.RationalInterval` must export four public
Transparent theorems in the order below, immediately after
`interval_intersect_assoc_rev_inside` and before the existing private
`equiv_leq`. Each takes **one** interval and has **no premise**:

```ken
pub theorem interval_add_zero_l_inside
    (i : Interval)
  : IsTrue
      (interval_inside (interval_add (interval_point rational_zero) i) i)

pub theorem interval_add_zero_l_rev_inside
    (i : Interval)
  : IsTrue
      (interval_inside i (interval_add (interval_point rational_zero) i))

pub theorem interval_mul_one_l_inside
    (i : Interval)
  : IsTrue
      (interval_inside (interval_mul (interval_point rational_one) i) i)

pub theorem interval_mul_one_l_rev_inside
    (i : Interval)
  : IsTrue
      (interval_inside i (interval_mul (interval_point rational_one) i))
```

The first and second laws give both directions of endpoint-value
inclusion between `i` and its sum with the **left** point zero; the
third and fourth do likewise for its product with the **left** point
one. `interval_point` uses equal endpoints, while `interval_add` and
`interval_mul` act componentwise. Section 3's `interval_inside`
compares stored rational endpoint values, even for an interval with
inverted endpoints. These laws therefore hold for **every** `i`,
without an ordered-endpoints, membership or nonemptiness premise.
Neither mutual inclusion nor a closed fixture's reduction asserts
structural `Equal Interval` or canonical rational spelling.

A checked proof can match the written `i : Interval` as
`MkInterval a b`, introduce the two Boolean endpoint bounds with
the existing conjunction law, and transport §64's
`zero_l for rational_add` or `one_l for rational_mul` through the
existing private `equiv_leq`; the opposite bound uses symmetry of the
same rational equivalence. `equiv_leq` and `rational_distrib_r` remain
private.
There is no new helper, axiom, kernel or surface rule, and all §1–§9
operations, law types and proof-route contracts remain unchanged.

**Composition note, not another public law.** Right-point-unit forms
need no separate exports. For addition, the public
`interval_add_comm_inside i (interval_point rational_zero)` followed
by `interval_add_zero_l_inside i` feeds `interval_inside_trans` to
put `interval_add i (interval_point rational_zero)` inside `i`.
The converse uses `interval_add_zero_l_rev_inside i` followed by
`interval_add_comm_inside (interval_point rational_zero) i`. Replace
`add`/`zero` with `mul`/`one` and use the corresponding §5
commutativity law for multiplication. These are compositions of the
existing public results, not new declarations or uses of Rational's
right-unit proofs.

A later CV seed can apply all four laws to `i=[1/2,1]` and exercise
an inverted interval such as `[1,1/2]`. Its wrong-unit refusal
compares `interval_mul (interval_point rational_zero) i = [0,0]`
with `i`, whose lower bound is `1/2`; the accepted same-form control
uses `rational_one` instead. Both `IsTrue (interval_inside …)` claims
use `Proved`: `Refl` is refused even for the true control on this
goal shape, so it cannot discriminate the wrong unit. A closed
`Equal Interval` claim can happen to accept `Refl` on the ordinary
fixture but does not supply a structural unit law. These cases
illustrate the generic obligation; no §10 seed is authored or run
by this spec-only child.

## 11. Point zero absorbs interval multiplication by inclusion

> **Section 11 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-POINT-ZERO-ABSORPTION).** Architect AC-0
> on exact landed base `70dc30f97` checked both generic proposals in
> scratch with rc 0, no `Axiom`, and equal `trusted_base()` identity
> sets 114 → 114. The historical Spec-only child `bdc3e5b47`
> stated the law types but did not add the catalog exports, package
> test or CV seed. Foundation reports its assembled working tree now
> includes both checked exports, a package-owned test and the corrected
> CV-authored seed. The named package test passed 3/3 on that working
> tree, exercising the actual seed's accepted block and isolated own-
> span refusal, with trusted GlobalId identities 114 → 114. This is
> Foundation's working-tree package-test evidence, not an exact
> committed-tip gate or official conformance-runner execution; neither
> scratch nor that report establishes CI, publication, landing or
> measured landed trust.

`Algorithm.Numeric.RationalInterval` must export two public Transparent
laws in the following order, immediately after
`interval_mul_one_l_rev_inside` and before the private `equiv_leq`:

```ken
pub theorem interval_mul_zero_l_inside
      (i : Interval)
    : IsTrue
        (interval_inside
          (interval_mul (interval_point rational_zero) i)
          (interval_point rational_zero))

pub theorem interval_mul_zero_l_rev_inside
      (i : Interval)
    : IsTrue
        (interval_inside
          (interval_point rational_zero)
          (interval_mul (interval_point rational_zero) i))
```

Both laws quantify over **every** `i : Interval`, including inverted
endpoints, with no side condition. They assert mutual **endpoint-value
inclusion** between the product of the left point zero and point zero
itself. As in §§3 and 10, `IsTrue (interval_inside …)` observes the
stored lower and upper rational values; it does not assert structural
`Equal Interval` or collapse every empty interval to a canonical value.
Multiplication is componentwise: for `i = MkInterval a b`, the two
product endpoints are `rational_mul rational_zero a` and
`rational_mul rational_zero b`. Rational's public
`(proof zero_l for rational_mul)` gives equivalence of each with
`rational_zero`; the existing private `equiv_leq` transports each
required direction to `rational_leq`, and the existing conjunction
introduction combines the endpoint comparisons. Both checked proofs
match the **written** `i`. `equiv_leq` and `rational_distrib_r` stay
private; no new import, premise, axiom, kernel or surface rule is needed.
No §1–§10 operation or law type changes.

**Right-point-zero is a composition, not another export.** For the
inclusion of `interval_mul i (interval_point rational_zero)` inside
point zero, compose §5's
`interval_mul_comm_inside i (interval_point rational_zero)` with
`interval_mul_zero_l_inside i` using `interval_inside_trans`. In the
opposite direction, compose `interval_mul_zero_l_rev_inside i` with
`interval_mul_comm_inside (interval_point rational_zero) i`. These
produce the right-zero inclusions from public laws only, without
`zero_r for rational_mul` or a new named theorem.

The CV-owned seed may apply both left-zero laws to the ordinary
`i = [1/2,1]` and inverted `[1,1/2]` intervals. Its same-form
closed discriminator keeps the product fixed: the true claim places
`interval_mul (interval_point rational_zero) i` inside
`interval_point rational_zero`, while the false claim substitutes
`interval_point rational_one` as its target. Both use `Proved`:
Architect AC-0 observed acceptance for the point-zero control and
`KernelRejected TypeMismatch` at the false claim's own span. `Refl`
refuses **both** claims and cannot separate them. A closed case is a
behavioral discriminator, not a replacement for the two generic
checked proofs. Seed authorship and official conformance execution
belong to separate evidence stages; neither is claimed by this
Spec-only child.

## 12. Membership respects rational value equality

> **Section 12 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-CONTAINS-RESPECT).** Architect AC-0
> `evt_2crpyyx0fafnw` ruled from Foundation's detached scratch D0 on
> `e6e13def6b1877bb7b8e33aee64f2e3d5533788d`. The generic law
> checked in the package's named test (3/3) with no added `Axiom` and
> unchanged `trusted_base()` identity sets 114 → 114; its package diff
> was insertion-only. The first seed probe used `Refl` on a
> `rational_equiv` proposition and therefore did not exercise the
> law's equivalence premise. AC-0 corrected those two probe terminals
> to `Proved`; the corrected rows are candidate pins, not D0 results.
> Neither scratch result checks this Spec-only child or an assembled
> exact tip, nor establishes official conformance-runner execution, CI,
> publication, landing or measured landed trust.

`Algorithm.Numeric.RationalInterval` must export one public Transparent
law directly after `interval_point_sound` and before
`interval_add_point`, with this exact public type:

```ken
pub theorem interval_contains_respects_equiv
      (i : Interval) (x : Rational) (y : Rational)
    : rational_equiv x y
      → IsTrue (interval_contains i x)
      → IsTrue (interval_contains i y)
```

The two premises say that `x` and `y` denote the same rational value
and that `x` belongs to the **written** interval `i`; the conclusion
moves membership to `y` without requiring normalized representations,
non-inverted bounds or another premise. The law must match on the
written interval, using Rational's public
`(proof respect for rational_leq)` to transport both endpoint
comparisons, then `(proof intro for bool_and)` to rebuild membership.
For `MkInterval lo hi`, the lower comparison uses `lo`'s reflexive
rational equivalence and `rational_equiv x y`; the upper comparison
uses `rational_equiv x y` and `hi`'s reflexive equivalence. The existing
private `equiv_leq` stays private and unchanged: it establishes a bare
`rational_leq` fact, whereas this law transports membership in a given
interval. No new import, helper, axiom, kernel or surface rule is needed;
no §1–§11 operation or law type changes.

The converse direction of `interval_point_sound` needs no second public
export. For any `x` and `y`, the following composition derives
`rational_equiv x y → IsTrue (interval_contains (interval_point x) y)`
from the new law and the existing public `interval_point_contains`:

```ken
λh. interval_contains_respects_equiv (interval_point x) x y h
      (interval_point_contains x)
```

The separate CV-owned seed should move membership of `1/2` in
`[1/2,1]` to `2/4` through the law, with a same-form `Proved` control
for membership of `2/4`. On that same interval, a `Proved` claim that
`2/6` is a member must refuse: `2/6` denotes `1/3`, below `1/2`.
A second refused application tries to transport membership from
`1/2` to `2/6` but lacks `rational_equiv 1/2 2/6`; its refusal
specifically pins the equivalence premise only if the corresponding
`2/4` application accepts. The corrected applications were not
measured in D0; the candidate must check the accepted route and
separate the two refusals. `Refl` is not a proof of
`rational_equiv 1/2 2/4`: that predicate is a `Prop`, not an
`Equal` goal. The seed, its package-owned execution and official
conformance-runner execution are separate from this Spec-only child.

## 13. Inverted intervals have no members; ordered endpoints are members

> **Section 13 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-INVERTED-EMPTY).** Architect AC-0
> `evt_7p5m1mn4fjmha` ruled D0-CLEAN on isolated scratch at
> `c66722ce385e8759f10d5ac2ffe1d4e900a599f2`. Its harness
> passed 2/2, twelve named regressions passed 36/36, no `Axiom`
> declaration was added, and the 114-element `trusted_base()`
> identity sets were equal before and after. The scratch checked a
> fourth, `_eq`-named law only as a probe; it is not exported here.
> These measurements do not check this Spec-only child or an
> assembled candidate. They establish no official conformance-runner
> execution, CI, publication, landing or measured landed trust. CV
> owns the separate closed seed; Foundation owns the catalog and
> package-owned test.

`Algorithm.Numeric.RationalInterval` must export exactly these three
new public Transparent checked laws, in order immediately after
`interval_contains_respects_equiv` (§12) and before
`interval_add_point`:

```ken
pub theorem interval_inverted_empty
      (lo : Rational) (hi : Rational) (x : Rational)
    : Equal Bool (rational_leq lo hi) False
      → IsTrue (interval_contains (MkInterval lo hi) x)
      → Bottom

pub theorem interval_ordered_contains_lo
      (lo : Rational) (hi : Rational)
    : IsTrue (rational_leq lo hi)
      → IsTrue (interval_contains (MkInterval lo hi) lo)

pub theorem interval_ordered_contains_hi
      (lo : Rational) (hi : Rational)
    : IsTrue (rational_leq lo hi)
      → IsTrue (interval_contains (MkInterval lo hi) hi)
```

The first law takes an inverted-endpoint decision and purported
membership at **any** `x`, and derives `Bottom`. Membership supplies
both `rational_leq lo x` and `rational_leq x hi`; transitivity would
make `rational_leq lo hi` true, contradicting the first premise.
The order premise alone proves each endpoint is a member: combine
reflexivity of `rational_leq` at that endpoint with the supplied
comparison using `bool_and` introduction. Each statement concerns
rational **values** under §64's order, not syntactic equality of
endpoint representations. In particular, §1.1's claim that an
inverted interval has no member now has a checked law; when the
endpoints are ordered, `lo` (and `hi`) supplies a member. The two
Boolean outcomes make emptiness equivalent to inverted endpoint
order for this representation, without changing `MkInterval` or
requiring a proof field at construction.

The contradiction is `Bottom`, not a second exported
`interval_inverted_empty_eq` law. The high-endpoint claim is its
own theorem, not a conjunction or a private corollary. The checked
bodies use the existing `(proof trans for rational_leq)`,
`(proof refl for rational_leq)` and `bool_and` projection and
introduction proofs; no new axiom, helper, kernel form or surface
rule is part of the contract. No §1–§12 operation or law type changes.

The CV-owned closed seed must use `interval_inverted_empty` to refute
membership of `rational_zero` in
`MkInterval rational_one rational_zero`, supplying `Proved` for its
inverted-order premise. Both endpoint laws apply to
`MkInterval rational_zero rational_one`. A direct `Proved` claim that
`rational_zero` belongs to the inverted interval refuses; the same
form of claim on the ordered interval accepts. The scratch D0
observed the refused row as `KernelRejected TypeMismatch` and the
control as accepted; a refused claim without the accepted same-form
control cannot pin inversion as the cause. These are scratch
measurements, not execution or acceptance of the separate CV seed.

## 14. Mutual inclusion fixes both endpoint values

> **Section 14 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-INSIDE-ANTISYMMETRY).** Architect AC-0
> `evt_4w1k09rrv4f8z` ruled the two laws from disposable scratch at
> `bc1d29fa2501bd9b946d2e2c051417074687a6b7`. The two checked
> proofs and an accepted closed client passed; two false clients refused
> with `KernelRejected TypeMismatch`. The measured 114-element
> `trusted_base()` identity sets were equal, and no `Axiom` was added.
> Thirteen named scratch regressions passed 39/39. The first
> independently loaded generic client refused `AmbiguousReference`:
> its harness duplicated an `IsTrue` import after the law checks.
> A corrected harness-only scratch rerun (`evt_yez73k5m74zs`) passed
> 2/2, including `D0 generic client ok`, with the same 114-element
> trusted identity sets. Foundation's package test separately checks
> the generic client on the candidate, not on this Spec child.
> No scratch result checks this Spec child or an assembled candidate;
> none establishes official conformance, CI, publication, landing or
> measured landed trust. CV owns the separate closed seed.

`Algorithm.Numeric.RationalInterval` must export exactly these two
public Transparent checked theorems immediately after
`interval_inside_trans` (§5) and before `interval_add_inside`:

```ken
pub theorem interval_inside_antisym_lo
      (a : Rational) (b : Rational) (c : Rational) (d : Rational)
    : IsTrue (interval_inside (MkInterval a b) (MkInterval c d))
      → IsTrue (interval_inside (MkInterval c d) (MkInterval a b))
      → rational_equiv a c

pub theorem interval_inside_antisym_hi
      (a : Rational) (b : Rational) (c : Rational) (d : Rational)
    : IsTrue (interval_inside (MkInterval a b) (MkInterval c d))
      → IsTrue (interval_inside (MkInterval c d) (MkInterval a b))
      → rational_equiv b d
```

The premises give both directions of the §3.1 endpoint comparison:
`c ≤ a`, `b ≤ d`, `a ≤ c`, and `d ≤ b`, with `≤` read as
`rational_leq`. Applying `(proof antisym for rational_leq)` to the
corresponding lower and upper pairs yields `rational_equiv a c` and
`rational_equiv b d`, respectively. The checked proofs project the
left and right conjuncts using the existing `bool_and` laws. Neither
conclusion asserts `Equal Rational` or structural `Equal Interval`:
different stored fractions can denote the same value. Inverted
intervals need no exception because `interval_inside` compares stored
endpoints rather than membership. Each endpoint has its own theorem;
no public conjunction, helper, axiom, kernel form, surface rule or
change to any §1–§13 operation or law type is part of this section.

The separate CV-owned closed seed compares `[1/2, 2]` with
`[2/4, 4/2]`: the spellings differ, while the endpoints are
rationally equivalent. With both inclusion premises it obtains the
lower and upper conclusions by applying the corresponding theorems.
An isolated refused application uses `[0, 1]` and `[0, 2]`;
its reverse-inclusion premise fails even though the lower-endpoint
conclusion is true. The accepted equal-value application is the
same-form control, so refusal tests the missing premise rather than
rejecting the theorem form. The scratch refusal does not establish
execution of the separate seed as Ken or by the official runner.

## 15. Truncated rational subtraction and interval width

> **Section 15 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-WIDTH).** Architect's AC-0 on base
> `22876f808` checked the generic Nat visibility promotion, Rational
> subtraction and Interval proofs in scratch: no added `Axiom`,
> unchanged 114-element trusted `GlobalId` identity sets, and public
> export deltas Nat +1, Rational +4, Interval +3. Accepted and
> same-form refused value clients ran only in scratch. The direct
> `interval_width := hi` mutation rejects the provider's point-law
> proof before a client can load; a separate diagnostic overlay
> removed the invalid generic gates and showed the closed nonzero
> value rows refuse on their own. Neither result is a test of this
> Spec child, official conformance, CI, publication, landing or a
> measured landed-trust delta.

Rational subtraction here is **truncated at zero**: the existing
`Rational` carrier represents non-negative values and has no signed
result. `Data.Numeric.Rational` must export this public operation and
three public checked laws after `rational_add_mono`:

```ken
pub fn rational_sub (x : Rational) (y : Rational) : Rational

pub proof self_is_zero for rational_sub (x : Rational)
    : rational_equiv (rational_sub x x) rational_zero

pub proof add_cancel for rational_sub
      (x : Rational) (y : Rational)
    : IsTrue (rational_leq y x) → rational_equiv (rational_add (rational_sub x y) y) x

pub proof add_distrib for rational_sub
      (x1 : Rational) (y1 : Rational) (x2 : Rational) (y2 : Rational)
    : IsTrue (rational_leq y1 x1)
      → IsTrue (rational_leq y2 x2)
      → rational_equiv
          (rational_sub (rational_add x1 x2) (rational_add y1 y2))
          (rational_add (rational_sub x1 y1) (rational_sub x2 y2))
```

For `x = n/d` and `y = m/e`, with both denominators positive,
`rational_sub x y` represents `sub (mul n e) (mul m d)` over the
positive denominator `d·e`. Thus it represents `max(x − y, 0)` by
value, without a signed carrier or reduction to a canonical stored
fraction. `self_is_zero` relates the result of subtracting a value
from itself to `rational_zero` by `rational_equiv`, not structural
`Equal Rational`. `add_cancel` states the right-inverse property
only when `y ≤ x`; `add_distrib` needs each corresponding pair
ordered. The private cancellation and reshuffle stages of the
checked proofs introduce no further public law. The existing
Nat.Arithmetic `add_cancel_r` becomes public under §64 §4 with its
original checked type and body, as a visibility-only prerequisite.

`Algorithm.Numeric.RationalInterval` must export the following
public checked operation after `interval_point`, and two public
Transparent checked laws after `interval_mul_point`:

```ken
pub fn interval_width (i : Interval) : Rational

pub theorem interval_width_point (x : Rational)
    : rational_equiv (interval_width (interval_point x)) rational_zero

pub theorem interval_width_add
      (lo1 : Rational) (hi1 : Rational) (lo2 : Rational) (hi2 : Rational)
    : IsTrue (rational_leq lo1 hi1)
      → IsTrue (rational_leq lo2 hi2)
      → rational_equiv
          (interval_width (interval_add (MkInterval lo1 hi1) (MkInterval lo2 hi2)))
          (rational_add (interval_width (MkInterval lo1 hi1)) (interval_width (MkInterval lo2 hi2)))
```

For `i = MkInterval lo hi`, the operation is **defined** as
`interval_width i = rational_sub hi lo`, written `hi ∸ lo` here:
its value is the truncated upper-minus-lower difference. The
returned `Rational` need not have a canonical stored spelling.
The point law needs no order premise and gives zero by value. The
sum law needs **both** displayed ordered-endpoint premises; then
the width of the pointwise interval sum is equivalent to the sum of
the widths. It is not a law for arbitrary inverted operands: for
`[2,1] + [0,5]`, the combined width is 4 while the two widths sum
to 5. The two theorem bodies reuse Rational's `self_is_zero` and
`add_distrib` proofs by conversion. They do not assert structural
`Equal Rational`, require a new Interval helper, or alter any
§1–§14 operation or law type.

These two laws are **properties** of the stated width definition,
not a characterization of it. A constant-zero operation also
satisfies both laws, so a separate closed value check must observe
a nonzero width with a nonzero lower endpoint, such as `[1,3]`
giving width 2. A CV-owned seed separately checks the point and
ordered-sum laws and gives same-form accepting controls for its
inverted-operand and wrong-width refusals; Foundation's package
test owns the nonzero value discriminator. Scratch results do not
establish execution of either committed artifact.

## 16. Interval width under inclusion

> **Section 16 status: DRAFT v0
> (SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO).** Architect's AC-0 on
> exact base `750f1622b` checked the seven generic public law types
> below in five isolated catalog modules, without a new `Axiom` and
> with equal trusted-base source-name/Decl-kind sets (114 → 114).
> The accepted applications, their independent closed conclusions,
> three same-form refused twins and private-import refusals were
> scratch measurements. They are not this Spec child's tests, an
> assembled-tip result, official conformance, CI, publication,
> landing or a measured landed-trust delta.

### 16.1 Seven public laws and their placement

`Data.Numeric.Nat.Order` exports three additional checked proofs
attached to `sub`, immediately after `sub::add_cancel`. They give
truncated subtraction a bound, left monotonicity and right
antitonicity. Two public theorems follow them: multiplication
preserves truncated subtraction with the factor on either side.
Their exact public types are:

```ken
pub proof leq_left for sub (a : Nat)
    : (k : Nat) → IsTrue (leq_nat (sub a k) a)

pub proof left_mono for sub (a : Nat)
    : (b : Nat) → (k : Nat) → IsTrue (leq_nat a b)
      → IsTrue (leq_nat (sub a k) (sub b k))

pub proof right_antitone for sub (a : Nat)
    : (k : Nat) → (l : Nat) → IsTrue (leq_nat k l)
      → IsTrue (leq_nat (sub a l) (sub a k))

pub theorem mul_sub_distrib_r (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul a (sub b c))
        (sub (mul a b) (mul a c))

pub theorem mul_sub_distrib_l (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (sub a b) c)
        (sub (mul a c) (mul b c))
```

The auxiliary `sub_add_same` follows the two distribution laws in
Order, but is **private**. The attached proofs belong to `sub`, not
to three competing top-level theorems. `Data.Numeric.Rational`
exports one theorem after `rational_sub::add_distrib` and before
`rational_mul_mono`:

```ken
pub theorem rational_sub_mono
      (x : Rational) (y : Rational) (z : Rational) (w : Rational)
    : Equal Bool (rational_leq x y) True
      → Equal Bool (rational_leq w z) True
      → Equal Bool (rational_leq (rational_sub x z) (rational_sub y w)) True
```

Its cross-multiplied proof stage `rational_sub_cross_mono` follows
`rational_mul_cross_mono` in Rational and remains **private**.
`Algorithm.Numeric.RationalInterval` exports the final theorem
immediately after `interval_inside_antisym_hi`, within the fence
that defines `interval_inside`:

```ken
pub theorem interval_width_inside (i : Interval) (k : Interval)
    : IsTrue (interval_inside i k)
      → IsTrue (rational_leq (interval_width i) (interval_width k))
```

No §1–§15 operation or law type changes. These are five new public
Nat.Order laws, one Rational law and one RationalInterval law;
NatBox and Gcd gain no new public declaration.

### 16.2 Subtraction, cross multiplication and the proof boundary

`rational_sub_mono` is monotone in its **first** argument and
antitone in its **second**: `x ≤ y` and `w ≤ z` imply
`x ∸ z ≤ y ∸ w`. In the existing non-negative `Rational` carrier,
`∸` truncates at zero (§15); the result is a value inequality,
not an assertion of structural `Equal Rational`.

For `x = a/aa`, `y = b/bb`, `z = c/cc`, `w = d/dd` with positive
natural denominators, the hypotheses cross-multiply to
`a·bb ≤ b·aa` and `d·cc ≤ c·dd`. The conclusion compares the
truncated numerators after multiplication by the other result's
denominator:

```text
(sub (mul a cc) (mul c aa)) · (bb · dd)
  ≤ (sub (mul b dd) (mul d bb)) · (aa · cc)
```

`mul_sub_distrib_l` distributes those factors over truncated Nat
`sub`; the two existing order-scaling stages transport the original
hypotheses. `sub::left_mono` carries the first numerator bound, and
`sub::right_antitone` carries the reversed second bound. The latter
uses `sub::leq_left`, which bounds any truncated subtraction by its
minuend. `mul_sub_distrib_r` supplies the general Nat provider law;
its left-facing counterpart uses multiplication commutativity.
`sub_add_same` and `rational_sub_cross_mono` are internal stages,
not new imports available to clients.

For `i = [a,b]` inside `k = [c,d]`, the existing `interval_inside`
means `c ≤ a` and `b ≤ d`. Applying `rational_sub_mono` with
`x=b`, `y=d`, `z=a`, `w=c` yields `b ∸ a ≤ d ∸ c`, exactly the
width bound. **Inclusion is the only premise** of
`interval_width_inside`: no separate ordered-endpoint or validity
hypothesis is necessary. In particular, an inverted inner interval
has width zero by truncation; if it is inside an outer interval, the
same bound still holds.

The three attached sub proofs move from NatBox's private scaffolding
to the generic Nat.Order provider; NatBox consumes those exports
rather than retaining private copies. Gcd consumes Order's
`mul_sub_distrib_r` and removes its local multiplication/subtraction
proof and local `sub_add_same`. No new import beyond the named Nat
and Rational laws is needed for the owning catalog modules. There
is no new axiom, primitive, trusted assumption, kernel or surface
rule; the measured **114 → 114** equality applies only to the
isolated AC-0 roots, not an unmeasured assembled candidate.

### 16.3 Closed discriminators and evidence limit

The correct width of `[0,4]` is four and that of `[1,2]` is one,
so T1's reversed claim that the former has width at most the latter
is false. Its accepted control reverses the two intervals. By
calculation, a mistaken width defined as the lower endpoint would
accept T1 (`0 ≤ 1`), as would constant zero (`0 ≤ 0`): the
inclusion law alone does **not** characterize the width operation.
Using the upper endpoint instead also refuses T1 (`4 ≤ 2` is false),
but §15's point law excludes that alternative. These mutant
outcomes are **derived**, not measured mutant runs.

Two more false claims fix the direction of each Rational argument.
T2 claims `3 ∸ 1 ≤ 2 ∸ 1`, false because `2 ≤ 1` fails; its
same-form control reverses the bound. T3 claims
`3 ∸ 0 ≤ 3 ∸ 1`, false because `3 ≤ 2` fails; its same-form
control likewise reverses the bound. T2 detects reversal of the
first-argument direction; T3 detects reversal of the second-argument
antitonicity. The D0 measured acceptance of all three controls,
then each twin's own-span `KernelRejected(TypeMismatch)` on a fresh
prefix. Its accepted closed interval clients also include an
inverted inner interval and an intersection bound, each with an
independent closed conclusion. These are **scratch** checks, not
execution of the separate CV seed, official conformance, an
assembled-tip gate, CI or measured landed trust.
