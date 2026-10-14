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
