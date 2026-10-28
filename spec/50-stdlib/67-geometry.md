# Exact geometry

> **Status: DRAFT v0 (SPEC-GEOMETRY-EXACT-CONTRACT §1;
> SPEC-GEOMETRY-MANHATTAN-METRIC §2;
> SPEC-GEOMETRY-BOX-CONTAINMENT §3;
> SPEC-GEOMETRY-BOX-AREA §4;
> SPEC-GEOMETRY-BOX-INSIDE-ORDER §5;
> SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY §6;
> SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY §7).** Sections 1–4,
> their catalog operations and checked laws, and their seeds are present
> on main.
> Section 4's package-owned roots-loader test executes its positive
> and refused seed fences and measures unchanged trusted identities
> (114 → 114); no official conformance-runner run is established.
> Section 5's three generic checked laws, seed and package-owned test
> are present on main. That test is not official conformance-runner
> execution or a landed trust measurement. Section 6 specifies two
> additional structural box equalities and two public Nat Order
> selector laws. Its catalog exports and package-owned seed test are on
> main; that test is not official conformance-runner execution or a
> landed trust measurement. Section 7 specifies four more laws of
> associativity. The separate Foundation catalog candidate exports the
> four checked laws and runs the §7 seed in a package-owned test. That
> test is not official conformance-runner execution, CI, publication,
> landing or a landed trust measurement.

## 1. Points and closed boxes over natural coordinates

### 1.1 Carriers, membership and empty boxes

`Algorithm.Geometry.NatBox` uses the existing `Nat` carrier on both
axes. It exports a point's two coordinates and a box's lower and upper
corners as ordinary inductive data:

```ken
pub data Point = MkPoint Nat Nat
export MkPoint

pub data Box = MkBox Point Point
export MkBox

pub fn axis_contains (lo : Nat) (hi : Nat) (x : Nat) : Bool
pub fn contains (b : Box) (p : Point) : Bool
```

The constructor exports are explicit: `pub data` publishes the type,
not its constructor (`../30-surface/33 §4.1`). A client must be able to
build points and boxes to use this interface. A separate checked
scratch client importing both constructors confirmed this correction
to D0's defining-module-only use; it did not change a proof body.

`axis_contains lo hi x` is `bool_and (leq_nat lo x)
(leq_nat x hi)` using the existing checked natural order. For
`MkBox (MkPoint lx ly) (MkPoint hx hy)`, `contains b
(MkPoint px py)` is `bool_and (axis_contains lx hx px)
(axis_contains ly hy py)`. Bounds are **inclusive**: an endpoint is
inside when it satisfies both inequalities. A box with lower bound
above upper bound on **either** axis contains no point. There is no
separate empty-box constructor or validity premise. `MkBox lo hi` is
still a valid `Box` in that case, so all five operations remain total.

`Eq Box` compares **the two corners structurally**, not the set of
points contained. Two differently represented empty boxes may have
unequal `Eq Box` values. No law identifies all empty boxes or equates
boxes solely because they contain the same points. The checked
intersection laws speak about point membership, not structural
identity of boxes.

### 1.2 Intersection, hull and exact distance

```ken
pub fn intersect (b : Box) (c : Box) : Box
pub fn hull (b : Box) (c : Box) : Box
pub fn manhattan (p : Point) (q : Point) : Nat
```

For box corners `blo=(blx,bly)`, `bhi=(bhx,bhy)`, `clo=(clx,cly)` and
`chi=(chx,chy)`, the operations compute:

- `intersect b c` has lower corner
  `(max blx clx, max bly cly)` and upper corner
  `(min bhx chx, min bhy chy)`. It does **not** return `Option` on a
  disjoint pair: inverted bounds represent their empty intersection.
- `hull b c` has lower corner `(min blx clx, min bly cly)` and upper
  corner `(max bhx chx, max bhy chy)`. It contains every point in
  either input. It is a coordinatewise corner hull, not a quotient
  by point-set membership; an empty input's corners still affect it.
- `manhattan (px,py) (qx,qy)` is
  `add (add (sub px qx) (sub qx px))
       (add (sub py qy) (sub qy py))`.
  `sub` is the existing saturating subtraction on `Nat`, so each
  parenthesized pair computes an absolute coordinate difference
  without signed coordinates. This is Manhattan distance, not
  Euclidean distance or box area.

These formulas specify results at **every** input, including inverted
boxes and coincident points. They introduce no new numeric carrier,
primitive, partial result or trusted arithmetic rule.

### 1.3 Six checked-law obligations

The landed `Algorithm.Geometry.NatBox` exports checked proof bodies
for these six §1 laws. The box-membership statements are generic in
`b`, `c` and `p`; the distance statements are generic in `p` and `q`.
`IsTrue` turns the Boolean result into a proposition. `And` here is
the **ambient Ω-valued conjunction** of the checked D0, with
`and_intro` and the two projections; it is not the distinct
`Type`-valued `Core.Logic.And` carrier. It packages the two separately
usable consequences of soundness. `Equal Nat` is
value equality, unlike the structural `Eq Box` of §1.1.

```ken
pub theorem intersect_sound (b : Box) (c : Box) (p : Point) :
  IsTrue (contains (intersect b c) p) →
  And (IsTrue (contains b p)) (IsTrue (contains c p))

pub theorem intersect_complete (b : Box) (c : Box) (p : Point) :
  IsTrue (contains b p) → IsTrue (contains c p) →
  IsTrue (contains (intersect b c) p)

pub theorem hull_contains_left (b : Box) (c : Box) (p : Point) :
  IsTrue (contains b p) → IsTrue (contains (hull b c) p)

pub theorem hull_contains_right (b : Box) (c : Box) (p : Point) :
  IsTrue (contains c p) → IsTrue (contains (hull b c) p)

pub theorem manhattan_symm (p : Point) (q : Point) :
  Equal Nat (manhattan p q) (manhattan q p)

pub theorem manhattan_self (p : Point) :
  Equal Nat (manhattan p p) Zero
```

Together the first two prove **exactly** that a point lies in the
intersection iff it lies in both boxes, including when either box is
empty. They do not assert an `Eq Box` equation for two representations
of an intersection. The §1 hull laws each preserve membership from
one input; they do not themselves prove a least-hull statement.
Section 5 states least-hull containment for every box, including
inverted boxes, without asserting structural idempotence or equality
between hull and union. The distance laws establish symmetry and
zero at identical points, not triangle inequality. Architect D0 checked
all six laws generically with private per-axis lemmas and existing
checked `Nat`/Boolean laws; no closed seed replaces these proofs.

### 1.4 Visibility dependency, trust and deferrals

The original §1 D0 base kept the `Data.Numeric.Nat.Order` bounds
`greatest for min` and `least for max` private. The proof of
`intersect_complete` needs their public statements to combine the
two lower and two upper inequalities. On that scratch base, the
private declarations caused `UnboundName` at `max::least`; publishing
the existing checked bounds and updating their privacy test removed
that obstacle. Both changes have since landed with the §1 catalog
module. This chapter does not edit `catalog/` or `crates/`.

The Architect's 324-line development at base `813346f95` checked all
six generic law bodies with the two visibility changes and no
`Axiom` declared in the geometry source. An import of
`Core.Classes.LawfulClasses` transitively loads unrelated existing
assumptions (including Int and String), although no geometry proof
body names them. Neither an absence of geometry-local axioms nor a
successful scratch check is a measured `trusted_base()` delta.
Constructor exports were independently added and checked against a
client. The §1 catalog and seed have since landed; that historical
scratch check alone does not establish shipped seed-harness execution,
CI, or a measured landed trust delta.

Signed coordinates and signed-dependent orientation, cross products
and rotation are deferred. Euclidean and floating-point distances,
dimensions other than two, a canonical empty-box representation and
point-set equality of boxes are also deferred. Box area is specified
in §4; the §1 distance operation is not an area calculation. Section
1's six laws alone do not establish the Manhattan triangle inequality;
§2 specifies and exports it separately.

## 2. Manhattan metric laws on natural points

> **Section 2 status: DRAFT v0 (SPEC-GEOMETRY-MANHATTAN-METRIC).**
> Architect D0 checked the two generic laws below on the then-landed
> §1 base. The §2 catalog extension and seed have since landed; the
> historical scratch check alone does not establish shipped seed-
> harness execution, CI, or a measured landed trust delta.

### 2.1 Triangle inequality and zero-distance identity

`Algorithm.Geometry.NatBox` extends §1's `Point`, `manhattan` and
six checked laws with exactly these two public, checked theorem
bodies. The signatures below are the D0-checked statements:

```ken
pub theorem manhattan_triangle (p : Point) (q : Point) (r : Point) :
  IsTrue (leq_nat (manhattan p r) (add (manhattan p q) (manhattan q r)))

pub theorem manhattan_zero_equal (p : Point) (q : Point) :
  Equal Nat (manhattan p q) Zero → Equal Point p q
```

The triangle law bounds the direct Manhattan distance from `p` to
`r` by the sum of distances along **any** detour through `q`. Its
comparison is the existing `leq_nat` converted to a proposition by
`IsTrue`; there is no new numeric carrier, order relation or
primitive. The bound includes both tight paths and strict slack.
For example, `p=(0,0)`, `q=(3,2)` and `r=(1,1)` yield direct
distance `2` and detour sum `5+3=8`. A reverse inequality is not
promised: `8 ≤ 2` is false in that example. A monotone detour can
instead be tight, for example `(0,0)` through `(1,1)` to `(2,3)`.

The identity law says a zero distance forces the two **points** to
be equal, including equality of both natural coordinates. Together
with §1's `manhattan_self`, it establishes zero distance exactly for
equal points, but does **not** add a separate converse theorem:
Architect D0 did not check such an additional law. For the distinct
points `(0,1)` and `(1,0)`, the distance is `2`, not `Zero`; an
attempt to assert zero for that pair must be refused. This is not
`Eq Box` or point-set equality of boxes, and it leaves the §1 box
operations and empty-box behavior unchanged.

### 2.2 Visibility, evidence and remaining scope

The per-axis triangle and zero-distance lemmas and their subtraction,
addition and point-equality helpers are **private** proof structure,
not further public laws. For any natural coordinates `a`, `b`, `c`,
the per-axis triangle bounds `axis_distance a c` by
`add (axis_distance a b) (axis_distance b c)`; the zero lemma turns
`Equal Nat (axis_distance a b) Zero` into `Equal Nat a b`. In
particular, §1's existing private `axis_distance` need not become
public. D0 checked a public-only client against the two theorems;
a client attempt to access `axis_distance` was refused as
`UnresolvedCon axis_distance`. The implementation can extend its
own **private imports** from the existing `Data.Numeric.Nat.Order`
and `Core.Logic.Transport` for proved order bounds and equality
transport. It must not clone the public Nat bounds or expose an
axis helper to make these statements usable. Both public signatures
refer only to the existing `Point`, `manhattan`, `add`, `leq_nat`,
`IsTrue` and `Equal` interface.

On base `3573ec369`, Architect D0 reported `ken check` rc 0 for the
generic in-module triangle and identity proofs in a **scratch copy**
of `NatBox`, with no new `Axiom`, primitive or postulate lines. The
existing `cat_geometry_nat_box` Rust test ran 3/3 against that
scratch module; its `trusted_base()` identity sets matched before
and after loading it, with counts **114 → 114**. A public-only
scratch client checked, including a strict-slack detour, a tight
path and both law instances. Separate negative clients rejected a
false reverse-detour bound and false zero distance or equality for
distinct points; four mutated proof steps also failed. These are
**Architect-reported scratch results**, not an author-run Ken check.
The §2 source and seed are now present on main; D0 by itself is not
shipped seed-harness execution, CI, or a measured landed trust delta.

The §1 Nat Order `greatest for min` and `least for max` exports and
§1 `NatBox` package are present on main. Section 2 adds no new
prerequisite publication or change to §1's public helper boundary.
Signed coordinates and orientation, Euclidean or floating-point
distance, dimensions beyond two, canonical empty boxes and point-set
equality of boxes remain outside this metric extension. Section 4
specifies box area separately from the §2 metric laws.

## 3. Corner-wise containment of boxes

> **Section 3 status: DRAFT v0 (SPEC-GEOMETRY-BOX-CONTAINMENT).**
> The following public operation and seven generic laws were checked
> in Architect D0 on the landed §1–2 base. The §3 conformance seed
> is authored and present at
> `conformance/stdlib/geometry/seed-box-containment.md`. The §3
> `inside` and seven law exports are present in
> `Algorithm.Geometry.NatBox` on main since `e3e0cda6c`. Exact-seed
> shipped-harness execution and CI are unmeasured. D0 scratch results
> do not establish a landed trust measurement.

### 3.1 Definition and inverted-box boundary

`inside b c` means that the **corners** of `b` fit inside the corners
of `c`. For `b = MkBox (MkPoint blx bly) (MkPoint bhx bhy)` and
`c = MkBox (MkPoint clx cly) (MkPoint chx chy)`, the public Boolean
operation has this exact definition, using the existing checked
natural order and conjunction:

```ken
pub fn inside (b : Box) (c : Box) : Bool =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          bool_and
                            (axis_within blx bhx clx chx)
                            (axis_within bly bhy cly chy)
                      }
                  }
              }
          }
      }
  }

fn axis_within (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) : Bool =
  bool_and (leq_nat l2 l1) (leq_nat h1 h2)
```

The matches destructure the corners, as in `contains` and
`intersect`. The per-axis `axis_within` is a private helper, **not**
an exported name or an additional public law. Both comparisons are
made even when `b` is inverted on one or both axes. There is no branch
for an empty inner box, validity
premise, canonical empty representation, or point-set quotient.

For instance, `(3,3)–(2,2)` is inside `(0,0)–(6,6)` by its corners;
`(6,6)–(5,5)` is **not** inside `(1,2)–(3,4)`, even though neither
inverted inner box contains a point. Thus the test is sufficient for
point-set inclusion but, without a witness in `b`, not necessary for
it. Unlike `Eq Box`, `inside` is a directional Boolean comparison of
bounds, not structural equality of two corner pairs.

### 3.2 Seven checked-law obligations

The module must export exactly these seven new public theorem types.
`IsTrue` turns each Boolean into an Ω proposition. Soundness applies
to **every** `b`, `c` and `p`: when the inner box is empty its second
premise is uninhabited, rather than adding a special case to the
operation.

```ken
pub theorem inside_sound (b : Box) (c : Box) (p : Point) :
  IsTrue (inside b c) → IsTrue (contains b p) → IsTrue (contains c p)

pub theorem inside_refl (b : Box) : IsTrue (inside b b)

pub theorem intersect_inside_left (b : Box) (c : Box) :
  IsTrue (inside (intersect b c) b)

pub theorem intersect_inside_right (b : Box) (c : Box) :
  IsTrue (inside (intersect b c) c)

pub theorem inside_hull_left (b : Box) (c : Box) :
  IsTrue (inside b (hull b c))

pub theorem inside_hull_right (b : Box) (c : Box) :
  IsTrue (inside c (hull b c))

pub theorem inside_complete (b : Box) (c : Box) (q : Point) :
  IsTrue (contains b q)
    → ((p : Point) → IsTrue (contains b p) → IsTrue (contains c p))
    → IsTrue (inside b c)
```

The intersection and hull laws hold **unconditionally**, including
for inverted input boxes and empty intersections: `max` of lower
bounds and `min` of upper bounds put an intersection inside each
operand; the reverse extrema put each operand inside the hull. These
are corner facts, not structural equations, and they require no
choice of a canonical empty box. `inside_refl` includes inverted
boxes because each bound compares with itself.

The final theorem is the **witnessed converse** of `inside_sound`.
A point `q` in `b` establishes a non-inverted bound on both axes;
therefore both corners of `b` are themselves points in `b`. The
pointwise inclusion premise puts both corners in `c`, supplying the
four comparisons of `inside b c`. Point-set inclusion **alone** is
insufficient: for inverted `b=(6,6)–(5,5)` and `c=(1,2)–(3,4)`,
`b` has no points, so every pointwise inclusion implication is
vacuously true while `inside b c` is `False`. No unwitnessed converse,
point-set equality, or equivalence of box representations follows.

### 3.3 Evidence and remaining scope

On base `ae42bb7fb56a7f18451578d3bd5cda4075fe7bc4`, Architect
D0 checked the package with the public `inside` and seven transparent
public proof bodies in a detached scratch worktree: rc 0, no `Axiom`,
53 owned declarations with zero `Opaque`, and identical
`trusted_base()` identity sets of 114 entries before and after
loading it. A scratch client checked nested, overlapping, self and
inverted pairs and an instance of `inside_complete`; a separate
client's claim of containment in a strictly smaller box was refused
with `TypeMismatch`. The ambient-passthrough census on that scratch
package passed 1/1 with no residuals. These are **D0 scratch
measurements**, not themselves §3 catalog publication, execution of
its authored seed, CI, or a measured landed trust delta.

Section 5 strengthens §3's one-sided intersection and hull bounds
with order laws; it does not alter `inside`, its seven laws, or the
witnessed premise of `inside_complete`. The approved §3 contract
adds no signed coordinates, dimensions beyond two, new primitive or
surface feature. It does not collapse distinct empty boxes or change
§1's `contains`, `intersect` or `hull`.

## 4. Natural area of a closed box

> **Section 4 status: DRAFT v0 (SPEC-GEOMETRY-BOX-AREA).** Architect
> checked the generic area laws on the landed §1–3 base in scratch.
> The four §4 exports and seed are present on main; the package-owned
> test runs the exact positive and refused seed fences and measures
> unchanged trusted `GlobalId` identities (114 → 114). No official
> conformance-runner execution is established by that package test.

### 4.1 Inclusive spans and inverted axes

`Algorithm.Geometry.NatBox` adds one public natural-valued operation.
For `MkBox (MkPoint lx ly) (MkPoint hx hy)`, `area` is the product of
two inclusive axis spans. The private `axis_span` is part of the
operation's definition, not a new export:

```ken
pub fn area (b : Box) : Nat =
  match b {
    MkBox lo hi ↦
      match lo {
        MkPoint lx ly ↦
          match hi {
            MkPoint hx hy ↦ mul (axis_span lx hx) (axis_span ly hy)
          }
      }
  }

fn axis_span (lo : Nat) (hi : Nat) : Nat = sub (Suc hi) lo
```

`sub` saturates at zero. Thus an axis with coincident endpoints has
span one; `Suc hi ≤ lo` makes that axis's span zero. The area is zero
if **either** axis is inverted, including when the other axis is
nonempty. A point box has area one, not zero: these are counts of
inclusive natural-coordinate positions, not a continuous Euclidean
measure. The area of a valid box with x span three and y span three
is nine. Different inverted boxes remain structurally distinct even
when both have area zero; no canonical empty-box or point-set quotient
is introduced.

### 4.2 Three public checked-law obligations

The module exports exactly these three new public theorem types. The
first reuses §3's corner-wise `inside` and the existing Boolean natural
order. The latter two state the inverted-axis rule for **all** natural
coordinates, not just the closed examples intended for the seed.
`Equal Nat` in these laws is value equality, not structural `Eq Box`.

```ken
pub theorem area_inside (b : Box) (c : Box) :
  IsTrue (inside b c) → IsTrue (leq_nat (area b) (area c))

pub theorem area_inverted_x
    (lx : Nat) (ly : Nat) (hx : Nat) (hy : Nat) :
  IsTrue (leq_nat (Suc hx) lx) →
  Equal Nat (area (MkBox (MkPoint lx ly) (MkPoint hx hy))) Zero

pub theorem area_inverted_y
    (lx : Nat) (ly : Nat) (hx : Nat) (hy : Nat) :
  IsTrue (leq_nat (Suc hy) ly) →
  Equal Nat (area (MkBox (MkPoint lx ly) (MkPoint hx hy))) Zero
```

`area_inside` has **only** its `inside b c` premise. It needs no
validity premise on the outer box `c`: if its x bounds are inverted,
`inside b c` requires `clx ≤ blx` and `bhx ≤ chx < clx`, forcing
`b` to be inverted on x as well. Both areas are then zero. The y-axis
case is symmetric. No special case for empty point sets is added to
`inside`; its corner comparisons already imply the required span
inequalities. The proof uses private monotonicity and antitonicity of
saturating `sub`, and multiplication monotonicity also when a factor
is zero.

Area of an intersection is no greater than either input, even for
inverted boxes. These are **corollaries**, not further public theorem
names: `IsTrue (leq_nat (area (intersect b c)) (area b))` follows by
`area_inside (intersect b c) b (intersect_inside_left b c)`; the
right-hand bound is
`area_inside (intersect b c) c (intersect_inside_right b c)`.
Section 3.2 already exports both corner-inclusion premises. These
are area corollaries, not additional area-intersection exports; §5's
separate greatest-intersection law is about `inside`. No structural
intersection equality or area-based equivalence of boxes follows.

### 4.3 Proof boundary and remaining scope

The per-axis span, generic `sub` and `mul` monotonicity helpers,
per-axis inversion and point-span helpers, and the two intersection
corollaries are **private**. Existing `Data.Numeric.Nat.Order` proves
saturation of `sub`, but does not export these generic monotonicity
helpers. Its public `leq_nat_mul_right_mono` requires a successor
factor, so the private proof also handles zero. No Nat library theorem
is promoted as part of this geometry contract. The exported operation
and three theorems require no new primitive, axiom or surface feature.

On landed base `0fd75f5515dded57eea1aeb47021573cfbc4a5e0`, the
checked scratch D0 `2645b9830d31e75e2d0974185ae3901402958ead`
returned `ken check` rc 0 with no local `Axiom` or postulate. Its
roots-loaded scratch trust test ran 1/1 and compared the exact
`trusted_base()` identity sets before and after the new module:
**114 → 114**. Architect independently repeated those checks on the
exact D0. The D0's inversion theorems were checked as private
proofs; Architect's ruling promotes their **unchanged theorem types**
to the §4 public interface. This is visibility, not an inference from
two closed examples. A separate scratch claim that a closed inner
box has strictly larger area than its enclosing box was refused with
rc 1/K0004. These measurements do **not** establish §4 catalog
exports, shipped-seed execution, CI, publication, landing or a landed
trust measurement.

Signed coordinates and orientations, continuous or approximate area,
higher-dimensional volume, canonical empty boxes and point-set
equality remain outside this extension. Neither area nor containment
supplies a runtime-cost guarantee.

## 5. The corner-wise inclusion order

> **Section 5 status: DRAFT v0 (SPEC-GEOMETRY-BOX-INSIDE-ORDER).**
> Architect checked three generic transparent laws in an isolated D0
> on `4f337ec26`. Those §5 `NatBox` exports, the authored seed and a
> package-owned test of its positive client and three isolated refusals
> are present on main. Neither D0 nor package tests establish official
> conformance-runner execution or a landed trusted-base measurement.

### 5.1 Three checked-law obligations

The module extends §3's corner-wise `inside` relation with exactly
three public theorem types. Every law holds for all `Box` values,
including inverted bounds and disjoint intersections. Each premise
and conclusion is an `IsTrue` proposition over `inside`; none is a
point-set inclusion or a structural `Equal Box` equation.

```ken
pub theorem inside_trans
      (a : Box) (b : Box) (c : Box)
    : IsTrue (inside a b) → IsTrue (inside b c) → IsTrue (inside a c)

pub theorem intersect_greatest
      (a : Box) (b : Box) (c : Box)
    : IsTrue (inside a b) → IsTrue (inside a c)
      → IsTrue (inside a (intersect b c))

pub theorem hull_least
      (b : Box) (c : Box) (d : Box)
    : IsTrue (inside b d) → IsTrue (inside c d)
      → IsTrue (inside (hull b c) d)
```

`inside_trans` composes comparisons of stored lower and upper
coordinates. `intersect_greatest` says any box inside both operands
is inside their coordinatewise intersection; together with §3.2's
`intersect_inside_left` and `intersect_inside_right`, this makes
`intersect b c` a greatest lower bound for **corner-wise** inclusion.
`hull_least` says a box containing both operands also contains their
coordinatewise hull; together with §3.2's `inside_hull_left` and
`inside_hull_right`, the hull is a least upper bound for that same
relation. These conclusions need **both** premises each; they do not
claim that a hull is inside either operand or that either operand is
inside its intersection.

No premise requires a box to contain a point. An inverted box still
has observable corners, so a chain may start with an inverted box and
`hull_least` may take an inverted operand. For example, the inverted
`(3,1)–(1,2)` fits in `(0,0)–(3,3)`, then in `(0,0)–(4,4)`.
The point-set interpretation would lose information about its corners
and cannot substitute for `inside`. The two existing §3.2 one-sided
laws and the three laws above describe bounds in this relation, not
an `Equal Box` identity, a canonical empty box, or an additional
public antisymmetry law.

### 5.2 Proof boundary and observed counterexamples

`axis_within` stays private. The D0's per-axis transitivity, meet and
join proofs (`axis_within_trans`, `axis_within_meet`, and
`axis_within_join`) are likewise **private**, not public Nat-order
laws or exported geometry interfaces. Their per-axis comparisons use the
already-public §1.4 bounds `greatest for min` and `least for max`, plus
`leq_nat` transitivity.
After destructuring the boxes, `bool_and` introductions combine the
two axes. This is a checked
proof route, not a new evaluation rule for `inside`, `intersect` or
`hull`. Neither those operations, §3's seven laws, nor §4's area
operation and laws change here. No validity premise, primitive,
axiom, surface feature or additional public helper is introduced.

The direction of each bound is observable on closed boxes. Let
`small=(1,1)–(2,2)` and `mid=(0,0)–(3,3)`. The hull of `small` and
`mid` fits in `mid` but **not** in `small`, because `mid` is not
inside `small`. Their intersection fits in `small`, but `mid` does
**not** fit in `intersect mid small`, since `mid` is not inside
`small`. The reverse chain `inside big inverted` also fails even
when `inverted` fits in `big`. The seed pairs each refusal with an
accepted observation on the same operands rather than inferring a
refusal from a missing export.

On the exact base `4f337ec26eaebf62d4951878e96809619aedc25f`,
Architect's isolated D0 reported `ken check` and `ken fmt --check`
rc 0 on its patched `NatBox`; four root-loaded providers retained
identical `trusted_base()` GlobalId sets, 114 → 114. The three
public laws were Transparent and owned by `NatBox`; a generic client
applied all three. Selective imports of `axis_within` and the three
new per-axis helpers were refused. The closed controls and isolated
reverse-inclusion claims were checked in that scratch development. The
package test now root-loads this candidate's exports and checks the seed's
accepted declarations and three isolated refused claims, including
same-operand positive controls.
The scratch results alone do not establish this catalog delivery; neither
the package test nor those results establish official conformance-runner
execution, CI, publication, landing or landed trust.

## 6. Intersection and hull commute as boxes

> **Section 6 status: DRAFT v0
> (SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY).** Architect's
> isolated D0 on `0bb872591` checked all four generic proofs, no new
> `Axiom`, and equal `trusted_base()` identities 114 → 114. The catalog
> catalog exports both Nat Order selectors and both NatBox laws on main;
> its package test checks the §6 seed against roots-loaded exports.
> That test is not official conformance-runner execution or a landed
> trust measurement.

### 6.1 Two public selector laws in Nat Order

`Data.Numeric.Nat.Order` must export exactly these two additional
**public Transparent checked proof selectors** for its existing `min`
and `max` functions. The declarations sit in their respective
operation families: `comm for min` after `greatest for min`, and
`comm for max` after `least for max`.

```ken
pub proof comm for min (m : Nat) (n : Nat) : Equal Nat (min m n) (min n m)

pub proof comm for max (m : Nat) (n : Nat) : Equal Nat (max m n) (max n m)
```

A client imports `min` or `max` and applies `(proof comm for min) m n`
or `(proof comm for max) m n`. These are facts about the public Nat
operations, not geometry-private copies. The existing §1.4 public
min/max bounds remain unchanged; no new Nat carrier, order operation,
axiom or primitive is needed. Their checked proof route splits each
pair of natural arguments and recurses on successor tails, using
congruence through `Suc`. The open successor cases require the
appropriate equality constructor (`Refl` in the two open max corners)
rather than replacing them all with `Proved`. No implementation strategy
is otherwise fixed by the public types.

### 6.2 Two structural box equalities

`Algorithm.Geometry.NatBox` must export exactly these two further
**public Transparent checked theorems**, in the existing package and
after `hull_contains_right` with the other §1 box-operation laws:

```ken
pub theorem intersect_comm (b : Box) (c : Box) : Equal Box (intersect b c) (intersect c b)

pub theorem hull_comm (b : Box) (c : Box) : Equal Box (hull b c) (hull c b)
```

The equalities hold for **every** pair of boxes, without a point
witness, nonemptiness or ordered-corner premise. In §1.2's formula,
intersection takes `max` of each lower coordinate and `min` of each
upper coordinate; hull takes the opposite selectors. Swapping the
two boxes swaps arguments to those Nat selectors, which are equal by
§6.1. Every stored corner coordinate is a `Nat`, so the four
coordinate equalities give equality of both `Point` corners and then
**structural `Equal Box`**. This includes inverted boxes: the
coordinate formulas remain total and the proof does not infer
identity from an empty set of members. It does not equate every
representation of an empty box, idempotence or a complement law.
Section 7 separately states associativity for each operation.

The private `point_equal` from the existing package stays private.
The new `box_with_lo`, `box_with_hi` and `box_equal` are private
congruence tools for rebuilding `MkBox` from equal corners, placed
directly after `point_equal`; no client may import them. The box
proofs destructure their two operands to the eight `Nat` coordinates,
apply the Order selectors to each, and assemble corners with
`point_equal` and `box_equal`. These are checked proofs in ordinary
Ken, not a new conversion rule or an `Eq Box` quotient. The existing
§1–§5 operations and laws are unchanged.

The distinction from `68-numerical.md` §5 matters: its rational
interval endpoints can have equivalent **values** but unequal stored
spellings. Swapping operands there guarantees mutual endpoint-value
inclusion, not structural `Equal Interval`. Here each corner is a
`Nat`, so the ruled min/max equalities give exactly the constructor
components needed for `Equal Box`, even if a box is inverted. The
closed seed at
`conformance/stdlib/geometry/seed-box-intersect-hull-commutativity.md`
uses overlapping but unequal boxes: both public laws apply; the
false claim that their hull equals their intersection is refused
with a same-proof-form equal-box control. Those examples delimit the
claim, not the generic proof.

Architect's isolated D0 at landed base `0bb872591` checked `ken check`
and `ken fmt --check` for both package files (rc 0), the four generic
bodies and two probe tests (2/2), with no new `Axiom` and unchanged
scratch `trusted_base()` GlobalId sets 114 → 114. The two Nat Order
selectors and two box laws were owned Transparent globals; selective
imports of the three new box helpers were refused. The new seed was
accepted with an overlay of only that checked package patch, and its
one false claim was refused there. The landed catalog exports all
four checked laws. A roots-loaded package test accepts the entire
seed client, including both direct Order selectors, independent value
checks and the same-form true `Equal Box` control; it pins the refused
`seed_neg` proposition and observes `TypeMismatch` at its own `Refl`.
The test separately checks the four generic public types and exact
before/after `trusted_base()` GlobalId sets (114 → 114), and refuses
selective imports of the three private helpers. The §6 exports and test
are on main. These local checks are not official conformance-runner
execution or a landed trust measurement.

## 7. Associativity of box intersection and hull

> **Section 7 status: DRAFT v0
> (SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY).** Architect's
> isolated D0 on `372a6a1a6` checked four generic proofs with no new
> `Axiom` and identical scratch `trusted_base()` identities 114 → 114.
> The earlier spec-and-seed candidate did not deliver the Nat Order or
> NatBox catalog exports. The separate Foundation candidate exports all
> four laws; its package test root-loads them and checks the complete
> accepted seed and the isolated false claim. This is candidate-local
> package-test evidence, not official conformance-runner execution, CI,
> publication, landing or a landed trust measurement.

### 7.1 Associativity selectors in Nat Order

`Data.Numeric.Nat.Order` must export these two further public Transparent
checked proof selectors for its existing natural extrema. Each belongs
directly after the corresponding §6.1 `comm for` selector, in that
operation's proof family:

```ken
pub proof assoc for min
    (k : Nat) (m : Nat) (n : Nat)
  : Equal Nat (min (min k m) n) (min k (min m n))

pub proof assoc for max
    (k : Nat) (m : Nat) (n : Nat)
  : Equal Nat (max (max k m) n) (max k (max m n))
```

Clients apply them as `(proof assoc for min) k m n` and
`(proof assoc for max) k m n`. They equate the two parenthesizations
for all natural arguments, including zero and repeated values, without
an order premise. The §6 commutativity selectors are distinct: swapping
two arguments or checking one closed value cannot replace a generic
regrouping law. The D0 proves these selectors by structural recursion
on the three arguments. The existing operations, Nat bounds and
commutativity laws remain unchanged; no new primitive or postulate is
needed.

### 7.2 Structural box equalities

`Algorithm.Geometry.NatBox` must export these two public Transparent
checked theorems alongside the §6 box laws:

```ken
pub theorem intersect_assoc
    (a : Box) (b : Box) (c : Box)
  : Equal Box (intersect (intersect a b) c) (intersect a (intersect b c))

pub theorem hull_assoc
    (a : Box) (b : Box) (c : Box)
  : Equal Box (hull (hull a b) c) (hull a (hull b c))
```

Both are unconditional for every three boxes, including inverted
bounds and disjoint intersections. By §1.2, each corner coordinate
of either grouping is a nested Nat extremum over the same three
coordinates. Intersection uses `max` on lower corners and `min` on
upper corners; hull reverses them. Section 7.1 equates each pair of
nested coordinates, yielding equality of both stored corners and
hence **structural `Equal Box`**. The argument does not identify
differently represented empty boxes by their point sets. It does
not imply that intersection distributes over hull, that hull
distributes over intersection, or that a mixed regrouping is equal.

The checked proof reuses existing private `point_equal` and
`box_equal` and, through them, private `box_with_lo` and
`box_with_hi`. There is **no new helper**, and none of these four
becomes public. Earlier §1–§6 operations and law types are unchanged.
The closed seed at
`conformance/stdlib/geometry/seed-box-intersect-hull-associativity.md`
uses three distinct pairwise overlapping boxes, independently
computes both sides of both equations, and applies the public laws.
A same-input mixed-operation equation is refused; these examples
delimit, but cannot prove, the generic laws.

On base `372a6a1a6`, Architect's D0 checked both package additions
with `ken check` and `ken fmt --check` rc 0, four owned Transparent
exports, no new `Axiom`, and unchanged scratch trusted GlobalId sets
114 → 114. Selective imports of all four existing private helpers
were refused with `UnboundName`. The D0 probe accepted closed
three-box controls and refused the mixed equation with `TypeMismatch`;
it did not run the CLI, official conformance runner or CI. Those
scratch measurements do **not** establish a shipped §7 export or
landed trust result. The separate Foundation catalog candidate exports
all four checked laws. Its `cat_geometry_box_intersect_hull_associativity`
package test root-loads them and checks the complete accepted seed and
its isolated refused claim. These candidate-local checks are not
an official conformance-runner run, CI, publication, landing or a
landed trust measurement. No new box representation, point-set
quotient, dimension, validity premise or runtime-cost guarantee is
introduced here.
