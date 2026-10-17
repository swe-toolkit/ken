# Exact geometry

> **Status: DRAFT v0 (SPEC-GEOMETRY-EXACT-CONTRACT).** Section 1
> specifies two-dimensional natural points, closed axis-aligned boxes,
> five operations and six generically checked D0 laws. The development
> is a scratch check, not a landed catalog module or a measurement of
> the trusted-base delta.

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

The prospective module must export checked proof bodies for exactly
these six laws. The box-membership statements are generic in `b`,
`c` and `p`; the distance statements are generic in `p` and `q`.
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

The existing `Data.Numeric.Nat.Order` defines `greatest for min` and
`least for max` but keeps **both private** on the D0 base. The proof
of `intersect_complete` needs their public statements to combine the
two lower and two upper inequalities. With the private declarations,
the scratch D0 fails `UnboundName` at `max::least`; adding only `pub`
to both declarations makes the checked development succeed. The
catalog implementation must publish those existing checked bounds
as a separate prerequisite follow-on rather than clone their proofs.
That follow-on also owns the existing Rust test that pins their
privacy. **Neither the bound publication nor the Rust test update
has landed as part of this spec WP.** This chapter does not edit
`catalog/` or `crates/` to meet the dependency.

The Architect's 324-line development at base `813346f95` checked all
six generic law bodies with the two visibility changes and no
`Axiom` declared in the geometry source. An import of
`Core.Classes.LawfulClasses` transitively loads unrelated existing
assumptions (including Int and String), although no geometry proof
body names them. Neither an absence of geometry-local axioms nor a
successful scratch check is a measured `trusted_base()` delta.
Constructor exports were independently added and checked against a
client. Catalog publication, Rust acceptance, execution of a shipped
conformance seed and CI require separate evidence.

Signed coordinates and signed-dependent orientation, cross products
and rotation are deferred. Euclidean and floating-point distances,
the Manhattan triangle inequality, box area, dimensions other than
two, a canonical empty-box representation and point-set equality of
boxes are also deferred. None follows from the six laws above.

## 2. Manhattan metric laws on natural points

> **Section 2 status: DRAFT v0 (SPEC-GEOMETRY-MANHATTAN-METRIC).**
> Architect D0 checked the two new generic laws below on the landed
> §1 base. This is a contract for a future catalog extension, not a
> claim that its §2 source or conformance seed has shipped.

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
**Architect-reported scratch results**, not an author-run Ken check,
shipped §2 catalog proof, executed §2 conformance seed, CI result,
or trust measurement on a landed §2 module.

The §1 Nat Order `greatest for min` and `least for max` exports and
§1 `NatBox` package are present on this landed base; the older §1
follow-on wording records its prior D0 situation, not the current
state. Section 2 adds no new prerequisite publication or change to
§1's public helper boundary. Signed coordinates and orientation,
Euclidean or floating-point distance, box area, dimensions beyond
two, canonical empty boxes and point-set equality of boxes remain
outside this metric extension.
