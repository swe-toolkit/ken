# Dimension-safe exact linear algebra

> **Status: DRAFT v1 (SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT;
> SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS;
> SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY §3;
> SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY §4;
> SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC §5).** Sections 1–4 and their
> fourteen public checked laws are present in the catalog. The §4
> package-owned test checks its sum seed and measures unchanged
> trusted identities (109 → 109), not official conformance-runner
> execution. Section 5 specifies four further generic sum laws; its
> catalog exports are absent at this base. Architect D0 and an authored
> §5 seed do not establish shipped-seed execution, CI, publication,
> landing or a landed §5 trust measurement.

## 1. Vectors and matrices over natural scalars

### 1.1 Carrier, dimensions, and imports

`Algorithm.LinearAlgebra.NatMatrix` operates on the **existing**
`Data.Vector.Vector` carrier. A length-`n` vector of scalars is
`Vec Nat n`; an `m`-row, `n`-column matrix is `Vec (Vec Nat n) m`,
with each inner vector a row in order. A matrix has no second indexed
carrier, unchecked shape field, truncating zip, or implicit resizing.
A vector of length zero and a matrix with either dimension zero are
valid. `Vec`, `VNil`, `VCons`, `map`, `zip_with`, and the checked
`vec_zero_vnil` are published by `Data.Vector.Vector`; the latter
supplies the empty-index transport needed by generic proofs. The
chapter neither duplicates the family nor changes its constructors
or law bodies.

`Nat` is the only scalar in this section. Its existing checked `add`
and `mul`, identities, commutativity, associativity, and distributivity
supply the arithmetic. A new semiring class and a `Rational` instance
are not implied: the landed Rational carrier identifies values under
`rational_equiv`, not structural `Equal Rational`, so substituting it
would change the very equality in the laws below. No floating-point
or signed calculation is silently represented by a natural.

### 1.2 Seven exact operations

All seven functions belong to `Algorithm.LinearAlgebra.NatMatrix`.
Their result indices are part of their checked types. `vector_add`
adds aligned components, and `vector_scale` multiplies every
component by the natural factor. Their empty inputs return an empty
vector. No zip discards a component.

```ken
pub fn vector_add
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n

pub fn vector_scale
  (n : Nat) (factor : Nat) (xs : Vec Nat n) : Vec Nat n

pub fn dot
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Nat
```

`dot Zero (VNil Nat) (VNil Nat)` is zero. On aligned successors
`VCons Nat k x xs` and `VCons Nat k y ys`, it is
`add (mul x y) (dot k xs ys)`. Thus a mismatch is refused by `Vec`'s
index before the product is evaluated, and every component
contributes exactly once. A natural result has no rounding or
subtraction. The arithmetic's checked laws, not a numeric primitive,
justify the generic equations in §1.3.

```ken
pub fn matrix_vector
  (m : Nat) (n : Nat)
  (rows : Vec (Vec Nat n) m) (vector : Vec Nat n) : Vec Nat m

pub fn transpose
  (m : Nat) (n : Nat)
  (rows : Vec (Vec Nat n) m) : Vec (Vec Nat m) n

pub fn matrix_product
  (m : Nat) (n : Nat) (p : Nat)
  (left : Vec (Vec Nat n) m) (right : Vec (Vec Nat p) n)
  : Vec (Vec Nat p) m

pub fn identity (n : Nat) : Vec (Vec Nat n) n
```

`matrix_vector` takes each row's dot product with the vector, in
row order. `transpose` takes the ordered columns as its rows: entry
at row `i`, column `j` becomes entry at row `j`, column `i`; it does
not reverse either order. `matrix_product` takes each row of the
`m × n` left input and dots it with every ordered column of the
`n × p` right input, producing an `m × p` matrix. The two inner
indices must be the **same `n`**; `m × n` by `p × q` with `n ≠ p`
does not type-check. The zero-dimensional result follows from those
same indexed operations, without an exceptional arithmetic value.

`identity n` has one on its diagonal and zero elsewhere, using
`Suc Zero` and `Zero` as the scalar values. `identity Zero` has zero
rows. At `Suc k`, its first row is one followed by `k` zeros, and
each remaining row prefixes zero to the corresponding row of
`identity k`. This construction returns exactly `Vec (Vec Nat n) n`;
there is no separately stored dimension witness. Private local
helpers for repeating zeros, prepending elements and computing row
products are implementation choices, not additional public
operations.

### 1.3 Five checked-law obligations

The landed catalog module provides a **checked proof body** for each
public theorem below, with no `Axiom`, postulate, assumed lemma, or
open obligation. The statements quantify over every length and
well-typed operand; a finite fixture is not a substitute for any
of them. `Equal Nat` and `Equal (Vec …)` are ordinary checked
propositions here, not floating comparisons or a new matrix
equality relation.

```ken
pub theorem dot_comm
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) :
  Equal Nat (dot n xs ys) (dot n ys xs)

pub theorem dot_distrib
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n) :
  Equal Nat
    (dot n (vector_add n xs ys) zs)
    (add (dot n xs zs) (dot n ys zs))

pub theorem transpose_invol
  (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m)
    (transpose n m (transpose m n rows)) rows

pub theorem vector_identity
  (n : Nat) (v : Vec Nat n) :
  Equal (Vec Nat n) (matrix_vector n n (identity n) v) v

pub theorem matrix_identity_right
  (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m)
    (matrix_product m n n a (identity n)) a
```

The dot laws are over natural sums: commutativity rearranges each
paired factor, and left distributivity expands a componentwise sum
before adding the resulting dot products. The transpose theorem
requires the original **row and column order**, not merely the right
shape. The vector identity law applies to any `Vec Nat n` including
an empty vector. The right matrix identity law composes that
behavior over arbitrary rows and the columns of `identity n`; it
is not a left-identity claim in disguise. The `vec_zero_vnil`
transport used in the checked proof of the dot and transpose empty
cases is a **proved Vec law**, not a fresh axiom.

### 1.4 Trust boundary and evidence limits

The ordinary `Vec` eliminator, natural recursion, `Equal`, and the
existing Nat laws account for §1's specified work. Its seven
operations and five generic laws, together with §2's left identity
and product associativity and §3's two compatibility laws and
`dot_scale`, are landed in `Algorithm.LinearAlgebra.NatMatrix`.
The checked catalog module introduces no `Axiom`. The historical §3
D0 measured `trusted_base()` as 109 before and after its scratch
package; that measurement does not establish a landed §3 trust delta.
The historical §4 D0 is separately measured in §4.3. Section 5's
four sum laws have checked generic bodies only in Architect's isolated
D0, with unchanged `trusted_base()` identities (114 → 114). Its
catalog exports are not yet present; that scratch measurement is not
landed trust. No statement in this chapter adds a kernel form,
syntax, primitive, or `Float` support. An authored seed or scratch
check alone is not execution of a shipped seed harness.

Signed scalars, a relation-parametric semiring abstraction and a
Rational-valued matrix tier, determinants, inverses, solving and
decompositions, and every `Float` or approximate/error-bound result
are outside this contract. No requirement is discharged by a comment,
an unchecked assumption, or a numerical example alone.

## 2. Left identity and associativity of matrix products

The following two **generic laws have checked proof bodies** in the
landed catalog module. Its §1–3 public laws are all exported;
§3 states the separate compatibility contract. Neither §2 law
is gated on `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`; both use the
existing `Nat`, `Vec`, and matrix operations, without a new carrier or scalar
tier.

```ken
pub theorem matrix_identity_left
  (m : Nat) (n : Nat) (a : Vec (Vec Nat m) n) :
  Equal (Vec (Vec Nat m) n)
    (matrix_product n n m (identity n) a) a

pub theorem matrix_product_assoc
  (m : Nat) (n : Nat) (p : Nat) (q : Nat)
  (a : Vec (Vec Nat n) m)
  (b : Vec (Vec Nat p) n)
  (c : Vec (Vec Nat q) p) :
  Equal (Vec (Vec Nat q) m)
    (matrix_product m p q (matrix_product m n p a b) c)
    (matrix_product m n q a (matrix_product n p q b c))
```

Left identity picks each row of `a` with the corresponding unit row
and preserves its order, including when either dimension is zero.
This is not a rephrasing of §1's right identity: the unit matrix is
on the other side and its size is the **row** count `n`, rather than
the column count `m`. Product associativity compares two ways to
compose `m × n`, `n × p`, and `p × q` matrices; both sides are `m × q`
and the shared indices are statically equal. Its proof distributes
column products across vector addition and scaling, using the
existing checked Nat distributivity and associativity laws. Neither
a finite equality of products nor the type formation of these
propositions is the generic proof.

The historical §2 D0 check on base `e940e481c` closed each theorem
and both together by structural recursion, using existing Nat proofs and
`Refl`, `Proved`, `J`, `cong`, `trans`, and `sym`. The added proof text
has **no `Axiom`**; the trusted-base delta was **not measured**, so
that observation is not a quantified zero-delta assertion. The
checked D0 uses a private named helper `dot_with n row` where the
earlier §1 package wrote the argument to `map` as
`λcolumn. dot n row column`. With the λ spelling, the proof checks
failed during elaboration at `left_head` and `columns_cons` with an
ill-typed ambient-method error; the named spelling checked both
laws without changing any public type or operation. The λ failure
belongs to `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`; fixing
that elaborator defect is **not a prerequisite** for these theorems.
The landed catalog uses the named private helper while retaining
the exact public interface above. The historical D0 established
feasibility only; the two §2 catalog proofs have since landed.
Neither that D0 nor their presence proves shipped-seed execution or CI.

## 3. Products acting on vectors and reversing under transpose

Two further generic laws relate the §1 operations over the same `Nat`
scalars and indexed `Vec` carrier. These are required public theorem
types for `Algorithm.LinearAlgebra.NatMatrix`, not consequences of
checking a finite fixture. The first states that applying an
`m × p` product to a length-`p` vector is the same as applying its
`n × p` right factor and then its `m × n` left factor:

```ken
pub theorem matrix_vector_product
  (m : Nat) (n : Nat) (p : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) (x : Vec Nat p) :
  Equal (Vec Nat m)
    (matrix_vector m p (matrix_product m n p a b) x)
    (matrix_vector m n a (matrix_vector n p b x))
```

The second states that transpose reverses the factors of a matrix
product. Transposing the `m × p` product yields a `p × m` matrix;
the reversed product multiplies `transpose n p b : p × n` by
`transpose m n a : n × m`. The reversed order is required, not a
notational convenience:

```ken
pub theorem transpose_product
  (m : Nat) (n : Nat) (p : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat m) p)
    (transpose m p (matrix_product m n p a b))
    (matrix_product p n m (transpose n p b) (transpose m n a))
```

The supporting scaled-dot law is also required to be public. It is
the existing checked private `dot_scale`, promoted without a second
spelling or proof body in the follow-on catalog work. It complements
§1.3's already-public `dot_distrib`, which handles the summed
first argument:

```ken
pub theorem dot_scale
  (n : Nat) (x : Nat) (u : Vec Nat n) (c : Vec Nat n) :
  Equal Nat (dot n (vector_scale n x u) c) (mul x (dot n u c))
```

The four proof-stage helpers `dot_row_product`,
`dot_row_product_empty`, `zero_width_repeat`, and
`zip_prepend_map` stay private: their types refer to private row,
repeat, or prepend stages rather than only the public operations.
Neither compatibility law needs subtraction, signed scalars, a
semiring abstraction, a premise beyond its displayed operands,
or a new primitive, kernel rule, or surface feature. Both apply
at zero dimensions as well as to non-square matrices. An
unreversed product law is not specified: on the D0's closed `2 × 2`
example it equates different matrices. The Architect's development
on base `21c53baae` checked both generic law bodies, their generic
clients and a non-square closed example, with no `Axiom` and an
unchanged `trusted_base()` set of 109 entries. That D0 is historical
scratch evidence; the two laws and `dot_scale` have since landed as
catalog exports. The existing §1–3 seed and its source acceptance
target include closed §3 clients, but this statement is not a fresh
run of that target, CI result, or landed trust measurement. The
catalog places the two compatibility laws after
`matrix_product_assoc` and keeps the four stage helpers private.

## 4. Row-wise matrix sums and distributivity

> **Section 4 status: DRAFT v1
> (SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY).** The Architect
> checked the generic definitions and proofs in a scratch package on
> the landed §1–3 base. The five §4 exports now have checked catalog
> bodies; a package-owned roots-loader test checks all five seed source
> fences, including the isolated refusal, and measures unchanged
> trusted identities (109 → 109); the unchanged NatMatrix census row
> passes in the targeted strict-resolution suite (5/5). This test is
> not an official conformance-runner result or a fresh CI run.

### 4.1 Exact sum on the existing indexed carrier

`Algorithm.LinearAlgebra.NatMatrix` adds one public operation. Two
matrices of the same `m × n` dimensions sum row by row by applying
§1.2's `vector_add n` to the corresponding rows. The existing indexed
`zip_with` cannot truncate either operand; no new matrix carrier,
scalar class or implicit broadcasting is introduced. The definition
is exact, including `m = Zero` and `n = Zero`:

```ken
pub fn matrix_add
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    : Vec (Vec Nat n) m =
  zip_with (Vec Nat n) (Vec Nat n) (Vec Nat n) m (vector_add n) a b
```

### 4.2 Four public checked-law obligations

The first two laws hold over arbitrary `m`, `n`, `p` and typed
operands. The `left` suffix means that the fixed left factor
multiplies a **sum on the right**: `a(b+c) = ab+ac`.
The `right` suffix means a **sum on the left** of a fixed right
factor: `(a+b)c = ac+bc`. These names refer to the product side
held fixed, not to an assumed commutative matrix product. In each
case the two terms have exactly `m × p` shape; `matrix_add n p`
requires both right summands to be `n × p`, whereas
`matrix_add m n` requires both left summands to be `m × n`.

```ken
pub theorem matrix_product_distrib_left
      (m : Nat) (n : Nat) (p : Nat)
      (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) (c : Vec (Vec Nat p) n)
    : Equal (Vec (Vec Nat p) m)
        (matrix_product m n p a (matrix_add n p b c))
        (matrix_add m p (matrix_product m n p a b) (matrix_product m n p a c))

pub theorem matrix_product_distrib_right
      (m : Nat) (n : Nat) (p : Nat)
      (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) (c : Vec (Vec Nat p) n)
    : Equal (Vec (Vec Nat p) m)
        (matrix_product m n p (matrix_add m n a b) c)
        (matrix_add m p (matrix_product m n p a c) (matrix_product m n p b c))
```

Two further public laws give compatibility with §1.2's
`matrix_vector` and `transpose`. In the first, the same length-`n`
vector acts on both `m × n` summands and the results add at length
`m`. In the second, the two `m × n` inputs transpose separately to
`n × m` before addition. Neither law is already supplied by §1–3;
`transpose_add` is also a proof ingredient for the left product law.

```ken
pub theorem matrix_vector_distrib
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) (x : Vec Nat n)
    : Equal (Vec Nat m)
        (matrix_vector m n (matrix_add m n a b) x)
        (vector_add m (matrix_vector m n a x) (matrix_vector m n b x))

pub theorem transpose_add
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    : Equal (Vec (Vec Nat m) n)
        (transpose m n (matrix_add m n a b))
        (matrix_add n m (transpose m n a) (transpose m n b))
```

Every law is a generic equality of indexed values, not a comparison
of unchecked shapes or of only closed numerals. Zero dimensions and
non-square matrices are included. The section does not specify
`matrix_vector` distributivity over a sum in its **vector** argument:
that is a different orientation, neither requested nor measured.

### 4.3 Private proof boundary and evidence

The ruled proof stages `dot_distrib_right`, `columns_add_right`,
`row_product_add`, and `zip_prepend_add` remain **private**. Right-
argument dot distributivity is obtained from the existing public
`dot_comm` and `dot_distrib`; it is not a fifth exported law. The
private `columns_add_right` carries that dot step over columns;
`row_product_add` uses `transpose_add` and the column step. The
previously private `columns_add` and `zero_width_repeat` are reused.
The right product law reduces row-by-row through `columns_add`,
`matrix_vector_distrib` through `dot_distrib`, and `transpose_add`
through the empty-width case and `zip_prepend_add`. None promotes a
private helper or changes any §1–3 operation or law.

The Architect's D0 on base `b5b423cad974d803da3286b82156bfcc92d515bb`
added only the scratch `NatMatrix` declarations: `ken check` returned
rc 0 with no new `Axiom`, postulate or opaque declaration. Its scratch
trust test passed 1/1; the exact `trusted_base()` set remained
**109 → 109**, with 61 owned package declarations and five new public
names transparent and owned. A client import of private
`dot_distrib_right` was refused with E0012. The ambient-passthrough
census test **was not run to a verdict**: the Architect stopped it to
release the shared build lock. That scratch result claims no census
success; the package candidate separately passes the strict-resolution
suite (5/5) with the NatMatrix row unchanged. Closed scratch values used
`Refl` and covered a `2 × 2` sum and non-square distributivity. The
separate, square-typed false claim `a(b+c) = ab+c` refused with E0038
while its correct-RHS control checked; that negative does not itself
prove any generic law. These are scratch D0 observations, **not** a
§4 catalog or seed delivery, shipped seed execution, CI, publication,
landing or a landed trust measurement.

The §4 seed must include a **direct closed `matrix_add` oracle**:
`[[1,0],[2,1]] + [[0,3],[1,1]] = [[1,3],[3,2]]` as a `2 × 2`
indexed value, comparing `matrix_add` to an independently constructed
expected `Vec`. It must also distinguish the side on which the sum
occurs, with closed non-square products for both distributivity
orientations. The false law `a(b+c) = ab+c` needs a square left
factor so the incorrect `+c` term type-checks. The accepting
positive control must use **the same `a`, `b`, and `c` operands** as
the isolated refusal and assert the correct `a(b+c) = ab+ac` result.
Append only the false-RHS claim to a fresh accepting client: an
unrelated rejection cannot satisfy that negative. The package-owned
test now runs the exact seed fences, but official conformance-runner
execution, CI and landing remain unmeasured. Signed,
Rational-valued, semiring-general and approximate matrices, and
subtraction, are not implied by these natural-number laws.

## 5. Commutativity and associativity of indexed sums

> **Section 5 status: DRAFT v1
> (SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC).** Architect checked four
> generic indexed proofs in an isolated D0 on `4e7cf78a6`, with no
> new `Axiom` and identical `trusted_base()` sets (114 → 114). This
> section and its closed seed do not deliver §5 catalog exports,
> shipped-seed execution, CI, publication, landing or landed trust.

### 5.1 Four public checked-law obligations

`Algorithm.LinearAlgebra.NatMatrix` must export the following four
checked theorems for every natural length and matrix dimension. Their
`Equal` claims are structural equalities of indexed `Vec` values;
there is no pointwise equivalence relation, nonempty premise, or new
matrix carrier. Each law includes the zero-index case.

```ken
pub theorem vector_add_comm
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    : Equal (Vec Nat n) (vector_add n xs ys) (vector_add n ys xs)

pub theorem vector_add_assoc
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n)
    : Equal
        (Vec Nat n)
        (vector_add n (vector_add n xs ys) zs)
        (vector_add n xs (vector_add n ys zs))

pub theorem matrix_add_comm
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    : Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_add m n b a)

pub theorem matrix_add_assoc
      (m : Nat)
      (n : Nat)
      (a : Vec (Vec Nat n) m)
      (b : Vec (Vec Nat n) m)
      (c : Vec (Vec Nat n) m)
    : Equal
        (Vec (Vec Nat n) m)
        (matrix_add m n (matrix_add m n a b) c)
        (matrix_add m n a (matrix_add m n b c))
```

In both associativity statements the left endpoint groups the first
two operands; the right groups the last two. The existing Nat class
law `(proof assoc for add) x y z` points from `x + (y + z)` to
`(x + y) + z`, so the vector proof uses `sym` for this displayed
direction. `matrix_add_comm` and `matrix_add_assoc` follow row by row
from the corresponding vector laws; neither relies on matrix-product
commutativity. An empty vector, zero-row matrix and positive-row
zero-column matrix remain in the quantified domain. A sum is not
thereby equal to either operand when the other is nonzero, and
regrouping cannot change a component or row.

### 5.2 Proof and trust boundary

The four new per-step helpers `vector_add_cons`,
`vector_add_assoc_cons`, `matrix_add_cons` and
`matrix_add_assoc_cons` remain **private**. `vector_add_cons` is local
because the underlying `zip_with_vcons` is not public in
`Data.Vector.Vector`; the local theorem is not a second public
vector API. Matrix proofs lift the vector equalities through the
ordered rows, with no operation change to `vector_add` or
`matrix_add` and no new Nat lemma. Neither a zero vector nor a
commutative-monoid instance is introduced.

The proof must not treat both nested additions as definitionally
convertible to a `VCons` result. In Architect's D0, the left-nested
successor form reduced, but right-nested `vector_add` did **not**:
`zip_with` matches on its second operand and remains stuck on an
inner `zip_with` result. Both vector and matrix associativity
therefore pass through one-level cons equations, `cong` and `trans`;
the head step uses the checked Nat law in the required direction.
This records a proof route, not a new computation rule.

Architect's isolated D0 inserted only `NatMatrix` proof bodies and
migrated an allowed-inventory test from seventeen to twenty-one
public names. Its generic client applied all four laws; checked
closed controls included an empty vector, zero rows, zero columns
and changed-component refusals. `ken check` and `ken fmt --check`
returned rc 0; before and after loading the four named providers,
`trusted_base()` contained the same 114 GlobalIds. Those
measurements belong to the D0 scratch package, not this spec-only
candidate. The authored §5 seed separately states its own checking
boundary. Neither record alone establishes a shipped seed harness,
CI, publication, landing or a landed trust delta.
