# Exact geometry

> **Status: DRAFT v0 (SPEC-GEOMETRY-EXACT-CONTRACT §1;
> SPEC-GEOMETRY-MANHATTAN-METRIC §2;
> SPEC-GEOMETRY-BOX-CONTAINMENT §3;
> SPEC-GEOMETRY-BOX-AREA §4).** Sections 1–3, their catalog operations
> and checked laws, and their seeds are present on main. Section 4
> specifies natural-valued area, containment monotonicity, and both
> generic inverted-axis zero laws. Architect checked the §4 generic
> proofs in scratch, but §4 catalog exports and conformance seed have
> not been delivered. Neither that scratch check nor the earlier
> seeds establish §4 shipped-seed execution, CI or a landed trust
> measurement.

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
of an intersection. The hull laws each preserve membership from one
input; neither asserts structural idempotence, a least hull for empty
inputs, or an equality between hull and union. The distance laws
establish symmetry and zero at identical points, not triangle
inequality. Architect D0 checked all six laws generically with
private per-axis lemmas and existing checked `Nat`/Boolean laws; no
closed seed replaces these proofs.

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
measurements**, not §3 catalog publication, an authored or executed
§3 seed, CI, or a measured landed trust delta.

The approved contract adds no signed coordinates, dimensions beyond
two, new primitive or surface feature. It does not collapse distinct
empty boxes or change §1's `contains`, `intersect` or `hull`.

## 4. Natural area of a closed box

> **Section 4 status: DRAFT v0 (SPEC-GEOMETRY-BOX-AREA).** Architect
> checked the generic area laws on the landed §1–3 base in scratch.
> The §4 catalog extension and seed have not been delivered. Scratch
> checking does not establish §4 shipped-seed execution, CI,
> publication, landing or a landed trust measurement.

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
Section 3.2 already exports both corner-inclusion premises. No structural
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
