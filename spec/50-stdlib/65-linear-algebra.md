# Dimension-safe exact linear algebra

> **Status: DRAFT v1 (SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT;
> SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS;
> SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY §3;
> SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY §4;
> SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC §5;
> SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY §6;
> SPEC-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT §7;
> SPEC-LINEAR-ALGEBRA-IDENTITY-SYMMETRIC §8;
> SPEC-LINEAR-ALGEBRA-VECTOR-SCALE-ACTION §9;
> SPEC-LINEAR-ALGEBRA-VECTOR-ZERO-UNIT §10;
> SPEC-LINEAR-ALGEBRA-ZERO-ANNIHILATION §11).** Sections 1–6 and their
> twenty public checked laws are present in the catalog. Package-owned
> tests check the authored §4–6 seeds against root-loaded exports; they
> are not official conformance-runner runs or landed trust measurements.
> Section 7's generic law, closed seed and package-owned test are now
> on main; earlier D0 and local test execution are not official
> conformance or landed trust. Section 8 promotes the existing checked
> transpose-of-identity helper to a public law without a wrapper or
> new proof body. Architect's isolated D0 on `18e269f32` checked its
> public visibility and equal scratch trusted identities 109 → 109.
> The §8 checked export, seed and package-owned test are on main. Neither
> scratch D0 nor that test establishes official conformance-runner
> execution or landed trust. Section 9's four checked scaling laws,
> seed and package-owned test are now on main. Its isolated D0 had equal
> scratch trusted identities 114 → 114; neither that scratch work nor
> candidate-local tests establish official conformance-runner execution
> or a measured landed trust result. Section 10's zero-vector operation,
> four checked laws, authored seed and package-owned test are on main:
> nine operations and thirty laws, thirty-nine public names. Architect's
> §10 AC-0 checked generic proofs in scratch with equal trusted identity
> sets 109 → 109. This assembled candidate contains §11's three
> checked catalog laws, CV's authored seed and a package-owned test.
> Foundation reports the named `cat_linear_algebra_nat_matrix` target
> passed 7/7 on the assembled content. Separately, Architect AC-0 on
> `a39d385c8` checked proposed bodies in scratch with no new `Axiom`
> and equal trusted identities 109 → 109. These candidate-local and
> scratch checks are not official §11 conformance-runner execution,
> CI, publication, landing or a measured landed trust result.

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

### 1.2 Eight exact operations

All eight functions belong to `Algorithm.LinearAlgebra.NatMatrix`.
Their result indices are part of their checked types. `vector_add`
adds aligned components, `vector_scale` multiplies every component
by the natural factor, and `vector_zero n` contains exactly `n`
copies of natural `Zero`. At `n = Zero`, all three return an empty
vector. No zip discards a component.

```ken
pub fn vector_add
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n

pub fn vector_scale
  (n : Nat) (factor : Nat) (xs : Vec Nat n) : Vec Nat n

pub fn vector_zero (n : Nat) : Vec Nat n

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
existing Nat laws account for §1's specified work. The seven §1
operations landed before §10, and the five generic §1 laws, together
with §2's left identity and product associativity and §3's two
compatibility laws and
`dot_scale`, are landed in `Algorithm.LinearAlgebra.NatMatrix`.
The checked catalog module introduces no `Axiom`. The historical §3
D0 measured `trusted_base()` as 109 before and after its scratch
package; that measurement does not establish a landed §3 trust delta.
The historical §4 D0 is separately measured in §4.3. Section 5's
four sum laws now have checked catalog bodies and a package-owned seed
test. Architect's isolated §5 D0 measured unchanged `trusted_base()`
identities (114 → 114); that scratch measurement is not landed trust.
No statement in this chapter adds a kernel form, syntax, primitive,
or `Float` support. A package test is
not official conformance-runner execution.

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
that is a different orientation, specified separately in §6.

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
> new `Axiom` and identical `trusted_base()` sets (114 → 114). The four
> §5 catalog exports and a roots-loaded package test of the authored
> seed's positive client and isolated refusals are on main. That test
> does not establish official conformance-runner execution or landed
> trust.

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
`matrix_add` and no new Nat lemma. Section 10 subsequently adds a
zero vector and its four laws; it introduces no commutative-monoid
instance.

The proof must not treat both nested additions as definitionally
convertible to a `VCons` result. `zip_with` matches its **first**
operand, then its second within the successor arm. In Architect's
§5 D0, the left-nested successor form reduced, but the right-nested
`vector_add` stayed stuck on an inner `zip_with` result in that
**second** position. Both vector and matrix associativity therefore
pass through one-level cons equations, `cong` and `trans`; the head
step uses the checked Nat law in the required direction. This
records that proof route, not a general second-operand match rule or
a new computation rule.

Architect's isolated D0 inserted only `NatMatrix` proof bodies and
added four names to the base's twenty-two-name allowed-inventory test.
The actual scratch set had twenty-six names; its assertion message's
"twenty-one" numeral was stale. Its generic client applied all four
laws; checked
closed controls included an empty vector, zero rows, zero columns
and changed-component refusals. `ken check` and `ken fmt --check`
returned rc 0; after loading its four providers and again after
loading NatMatrix, `trusted_base()` contained the same 114 GlobalIds.
Those measurements belong to the D0 scratch package, not this candidate.
The package test checks the authored §5 seed against root-loaded
exports and separately measures its trusted-identity set. Its result
is not official conformance-runner execution or a landed trust delta.

## 6. Linearity of matrix-vector application in its vector

> **Section 6 status: DRAFT v0
> (SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY).** Architect's
> isolated D0 on `aa6370449` checked both generic proofs with no new
> `Axiom` and identical scratch `trusted_base()` GlobalIds (109 → 109).
> The two catalog exports have checked bodies on main, with a package
> test of the authored seed against root-loaded exports. The isolated
> D0 and that test do not establish official conformance-runner execution
> or landed trust.

### 6.1 Two public checked-law obligations

`Algorithm.LinearAlgebra.NatMatrix` must export both Transparent
checked theorems. The `m × n` matrix acts on the **vector** operand:
additivity compares its image of `vector_add n x y` with addition of
its two images, while homogeneity compares its image of a scaled
vector with scaling its image by the same natural `factor`. These
are structural `Equal (Vec Nat m)` claims, not a numerical
approximation or a new vector equality relation.

```ken
pub theorem matrix_vector_additive
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n) (y : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector m n a (vector_add n x y))
        (vector_add m (matrix_vector m n a x) (matrix_vector m n a y))

pub theorem matrix_vector_homogeneous
      (m : Nat) (n : Nat) (factor : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector m n a (vector_scale n factor x))
        (vector_scale m factor (matrix_vector m n a x))
```

Both laws have no side premise and apply to every natural `m`, `n`
and `factor`, including zero dimensions. The dimensions and operand
order are explicit: a length-`n` vector is mapped to length `m`;
there is no implicit resizing. This is not §4.2's public
`matrix_vector_distrib`, which adds the **matrices** at fixed `x`.
Neither a finite closed equality nor only checking the displayed
types supplies the required generic proof.

### 6.2 Private proof boundary and evidence

Additivity reuses §4.3's private `dot_distrib_right`, obtained from
public `dot_comm` and `dot_distrib`. For homogeneity the private
second-position sibling `dot_scale_right` follows from public
`dot_comm` and `dot_scale`; it is not another public law:

```ken
theorem dot_scale_right
      (n : Nat) (factor : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    : Equal Nat (dot n xs (vector_scale n factor ys)) (mul factor (dot n xs ys))
```

Both matrix-vector proofs descend over the matrix rows. A row step
transports the dot-product head with its corresponding private law,
then transports the ordered tail with the induction hypothesis using
`cong` and `trans`. The existing vector and matrix operations and all
§1–5 public laws are unchanged. No axiom, new Nat law, kernel rule,
surface form, or additional import is required. The helper remains
private even though its first-position sibling `dot_scale` is public.

The Architect's detached D0 checked the two generic proof bodies and
a client on `aa6370449`: `ken check` and `ken fmt --check` returned
rc 0, two probe tests passed, no `Axiom` was added, and the scratch
trusted GlobalId set stayed 109 → 109 after loading the existing
providers and NatMatrix. Selective imports of both private dot
helpers were refused. The separately authored seed has independent
closed `Refl` value oracles for a non-square `2 × 3` matrix, direct
applications of both new public laws, and one false equation after
a fresh accepted prefix; its closed instances do not prove generic
linearity. The D0 checked proposed proofs in isolation; both checked exports and a
root-loaded package test of the seed have since landed on main. The D0
and package test do not establish official conformance-runner execution
or landed trust.

## 7. Transpose is adjoint to matrix-vector application

> **Section 7 status: DRAFT v0
> (SPEC-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT).** Architect checked the
> generic proof in an isolated D0 on `54f68d3cc` with no new `Axiom`,
> `ken check` and `ken fmt --check` rc 0, and identical scratch
> `trusted_base()` GlobalId sets (109 → 109). The earlier spec-and-seed
> candidate did not deliver the §7 catalog export. The checked law,
> seed and package-owned test are now on main. Its earlier candidate-local
> checks are not official conformance-runner execution or landed trust.

### 7.1 One premise-free public law

`Algorithm.LinearAlgebra.NatMatrix` must export the following Transparent
checked theorem. An `m × n` matrix acts on a length-`n` vector `x`;
its length-`m` image is dotted with `y`. Transposing the matrix
instead lets `y` act through an `n × m` matrix, and its length-`n`
image is dotted with `x`. Both results are exact naturals.

```ken
pub theorem transpose_adjoint
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n) (y : Vec Nat m)
    : Equal Nat
        (dot m (matrix_vector m n a x) y)
        (dot n x (matrix_vector n m (transpose m n a) y))
```

There is no premise or nonzero-dimension condition. The law holds for
all `m` and `n`, including zero rows or zero columns, without reversing
row order or exchanging `x` and `y`. It is an equality of the two
natural dot results, not an equality of vectors or matrices. Its
argument order is dimensions first, then `a`, `x`, `y`. A closed
example cannot prove the universal theorem.

### 7.2 Private proof boundary and evidence

The new `dot_zero_columns` handles the zero-row base. The new
`transpose_cons_image` identifies the transposed image after adding a
row: the leading component of `y` scales that row and the tail acts
through the remaining columns. Both are **private** proof helpers,
not additional public laws. The proof reuses public `dot_comm` and
Nat multiplication commutativity, plus existing private
`dot_scale_right` and `dot_distrib_right` and Nat addition's zero law;
`cong`, `sym`, and `trans` transport the equalities. Its public type
has no added premise, import, operation, kernel rule, or scalar tier.
The §1–6 operations, laws, and private helper visibility remain
unchanged. The §7 checked export and updated package law count are now on main.

The closed seed uses the non-square `2 × 3` matrix with rows
`[1,2,3]` and `[4,0,1]`, `x = [2,1,3]`, and `y = [1,2]`. Separate
closed calculations give both sides as 35; the theorem is also applied
independently of those value controls. With only `y` replaced by
`[2,1]` on the right, that side becomes 37 while the left stays 35.
An isolated refused claim follows a fresh fully accepted prefix and
uses the same `Proved` form as the accepted true `Equal Nat` value
controls. `Refl` is not a proof of `Equal Nat` here, even for two
closed equal naturals; it is appropriate for a structural `Vec`
equality such as the closed transpose oracle. The refusal tests this
false swapped-`y` equality, not a generic limit on the theorem.

Architect's D0 established proof feasibility and isolated client
behavior on a scratch catalog overlay; that historical check did not
establish a shipped §7 export or official seed execution. The checked
law, authored seed and package test are now on main. Neither earlier
candidate-local checking nor the historical D0 establishes official
conformance-runner execution or landed trust. No rational, signed,
semiring-general, or approximate matrix law is asserted.

## 8. Transposing the square identity preserves its entries

> **Section 8 status: DRAFT v0
> (SPEC-LINEAR-ALGEBRA-IDENTITY-SYMMETRIC).** Architect's isolated
> D0 on exact landed base `18e269f3205be0d6c1d0ab3e776960a824ae2646`
> checked the existing theorem with public visibility, `ken check` and
> `ken fmt --check` rc 0, and equal scratch `trusted_base()` identity
> sets 109 → 109. Its public catalog export, closed seed and package-owned
> test are on main. Neither D0 nor the package test is official
> conformance-runner execution or a landed trust measurement.

### 8.1 One checked law, promoted in place

`Algorithm.LinearAlgebra.NatMatrix` must expose its already checked
private `transpose_identity` proof as a **public Transparent theorem**.
Its name, type and body remain unchanged; the catalog moves the whole
declaration out of the private helper region and places it directly
after `transpose_adjoint`, before `matrix_add_assoc_cons`, with only
the `pub` visibility prefix added. It must not publish a wrapper or
second spelling. Its exact public type is:

```ken
pub theorem transpose_identity
      (n : Nat)
    : Equal (Vec (Vec Nat n) n) (transpose n n (identity n)) (identity n)
```

For every `n`, transposing the `n × n` matrix built by `identity n`
yields exactly that indexed matrix, including `n=Zero`. The equality
is **structural on `Vec` entries**, not equality modulo permutation
or an assumption that every square matrix is symmetric. The theorem
needs no index positivity premise or new operation. It is distinct
from §1.3's `transpose_invol`, which transposes *twice*, and from
§7's `transpose_adjoint`, which equates natural dot results. This
visibility promotion adds one public law, taking the package from
eight operations and twenty-one laws (twenty-nine public names) to
eight operations and twenty-two laws (thirty public names). All
§1–§7 public operation and law types remain byte-identical.

The original proof inducts on `n`, using a zero row over the smaller
identity and the induction hypothesis to recover the trailing rows
when the successor identity is transposed. Existing private
`transpose_zero_prefix_rows`, `zip_zero_map`, `prepend_zero`, and
`repeat` remain private. The existing recursive reference and the
private consumer `row_right_identity` still call the **same**
`transpose_identity` name and type; moving it to the public law
block does not replace or duplicate its body. No new helper,
postulate, kernel rule or surface feature is introduced. Its type
mentions only the already public `transpose`, `identity`, `Vec` and
`Nat`.

### 8.2 Closed discriminator and evidence boundary

The closed seed applies the law at `n=3`. It independently computes
`identity three` and `transpose three three (identity three)` as
the same explicitly constructed `3 × 3` identity, each with `Refl`.
Closed `Equal (Vec (Vec Nat three) three)` goals use `Refl`, **not**
`Proved`: Architect's D0 refused `Proved` even for a true closed
matrix equality. This distinguishes the proof-form boundary from
the law's actual content.

A separate `2 × 2` upper-triangular matrix with rows `[1,2]` and
`[0,1]` transposes to rows `[1,0]` and `[2,1]`. The accepted prefix
computes that transpose with `Refl` and includes the same-form true
control `upper = upper` with `Refl`. An isolated claim that
`transpose two two upper = upper` must then be refused at **its own**
`Refl`, after a fresh fully accepted prefix. The false matrix claim
is not a consequence of the identity-specific law; the true control
with the same proof form and the independently computed transpose
isolate its false content. Closed instances do not prove the generic
`n` theorem.

Architect D0 checked the public theorem's module ownership and
Transparent status, a generic public client, private-import refusals
for all four direct helpers, and one D0 test. The existing
`cat_linear_algebra_nat_matrix` visible-name inventory receives a
matching one-name update: twenty-nine to thirty public names with
`transpose_identity` as the additional visible name. The package-owned
§8 test checks the complete accepted seed and its isolated false claim. The D0 patch
SHA-256 is
`74cff3e2237167533ec8324c0193ab140b871662e3cd7cf394635f46f0a1db4a`.
These scratch checks are not package-test execution on the landed
§8 source or official conformance. The §8 checked export, seed and
package-owned test are on main; the historical D0 does not establish
a landed trust measurement. Neither this visibility promotion nor a
closed symmetric example grants symmetry to an arbitrary matrix.

## 9. Natural scaling acts on vector sums and scalar products

> **Section 9 status: DRAFT v0
> (SPEC-LINEAR-ALGEBRA-VECTOR-SCALE-ACTION).** Architect's isolated
> AC-0 on `524ea2cfd` checked four generic proofs with rc 0, no new
> `Axiom` or premise, and equal scratch `trusted_base()` identity sets
> 114 → 114. The four §9 exports, closed seed and package-owned test
> are now on main. Isolated D0 and earlier candidate-local tests do
> not establish official conformance-runner execution or a measured
> landed trust result.

### 9.1 Four public checked-law obligations

`Algorithm.LinearAlgebra.NatMatrix` must export exactly these four
public Transparent theorems. The argument order follows
`vector_scale n factor xs`: the dimension comes first, then natural
factors and then the length-indexed vectors. They belong together
after `vector_add_assoc` and before `matrix_vector_additive`, without
moving any §1–§8 declaration or changing its type.

```ken
pub theorem vector_scale_distrib
    (n : Nat) (x : Nat) (u : Vec Nat n) (v : Vec Nat n)
  : Equal
      (Vec Nat n)
      (vector_scale n x (vector_add n u v))
      (vector_add n (vector_scale n x u) (vector_scale n x v))

pub theorem vector_scale_factor_distrib
    (n : Nat) (x : Nat) (y : Nat) (u : Vec Nat n)
  : Equal
      (Vec Nat n)
      (vector_scale n (add x y) u)
      (vector_add n (vector_scale n x u) (vector_scale n y u))

pub theorem vector_scale_compose
    (n : Nat) (x : Nat) (y : Nat) (u : Vec Nat n)
  : Equal
      (Vec Nat n)
      (vector_scale n (mul x y) u)
      (vector_scale n x (vector_scale n y u))

pub theorem vector_scale_one
    (n : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n (Suc Zero) u) u
```

The first law scales each component of `vector_add n u v` and equals
adding the separately scaled vectors. The second distributes a sum of
natural factors over one vector; it is not a second vector-argument
law. The third composes two scalar actions in the written `x` then `y`
order, matching `mul x y` on the left. The last identifies the
`Suc Zero` action. Every law is structural `Equal (Vec Nat n)` for
all `n`, including zero, without a nonempty-vector premise. These
are not additional operations or runtime-cost claims, and a closed
example cannot establish their generic proof bodies.

### 9.2 Proof boundary and closed discriminator

`vector_scale` is the existing map of `mul factor`, while
`vector_add` zips componentwise `add`. The checked Nat laws
`mul_add_distrib_r`, `mul_add_distrib_l`, `assoc for mul` (in the
symmetric direction), and `one_l for mul` supply the four head
steps respectively. The single new helper `vector_cons_eq` transports
equal heads and tails through `VCons` and remains private in the
module's helper tail, immediately before `fn prepend` after
`vector_add_cons`. A client cannot import it. The proof needs no new
import, axiom, primitive, kernel or surface rule; all §1–§8
operations and public law types remain unchanged. Adding the four
public laws takes the package from eight operations and twenty-two
laws (thirty names) to eight operations and twenty-six laws (thirty-four
names); the package's public-inventory test must be updated when the
catalog half is assembled.

The closed seed uses `n=2`, factors `x=2` and `y=3`, and vectors
`u=[1,2]`, `v=[3,0]`. Applying each public law and independently
computing both sides with `Refl` gives `[8,4]`, `[5,10]`, `[6,12]`,
and `[1,2]` in order. On the same inputs,
`vector_add two (vector_scale two two u) u` computes to `[3,6]`,
not `[5,10]`. The isolated dropped-scale claim about
`vector_scale two (add two three) u` must be refused at its own
`Refl` after a fresh accepted prefix; a same-scope true claim with
the second scaled operand restored must be accepted. This separates
false content from a proof-shape or scope failure. The finite
calculations delimit the law but do not replace its generic proof.

Architect's isolated D0 on `524ea2cfd` checked the four generic
bodies and public clients, confirmed Transparent module ownership,
no new `Axiom` or premise, equal scratch trusted identity sets
114 → 114, and selective-import refusal for `vector_cons_eq`. The
scratch accepted client and false-claim refusal are AC-0 evidence,
not a shipped §9 conformance run. The §9 catalog code, inventory
update, closed seed and package test are now on main. The historical
§9 spec-only turn ran no Ken test;
its isolated D0 and candidate-local checks did not establish official
conformance-runner execution or a measured landed trust result.

## 10. Zero vectors and four unit laws

> **Section 10 status: DRAFT v0
> (SPEC-LINEAR-ALGEBRA-VECTOR-ZERO-UNIT).** Architect AC-0 on exact
> base `42ba094f3` checked generic proofs with `ken check` and
> `ken fmt --check` rc 0, no new `Axiom`, and equal scratch
> `trusted_base()` identity sets 109 → 109. That is isolated design
> evidence, not a measurement of landed trust. The §10 operation and
> four checked laws, authored seed and package-owned test are on main.
> Neither scratch D0 nor that test establishes official conformance-
> runner execution, fresh CI or a measured landed trust result.

### 10.1 Public definition and premise-free checked laws

The new public Transparent `vector_zero (n : Nat) : Vec Nat n`
belongs to `Algorithm.LinearAlgebra.NatMatrix` immediately after
`vector_scale`
(§1.2). It returns `repeat Nat n Zero`, using the module's existing
**private** generic `repeat`; it does not publish `repeat` or move it
to `Data.Vector.Vector`. At zero length the result is `VNil Nat`.

The module must export four public Transparent theorems in the order
below, immediately after `vector_scale_one` and before
`matrix_vector_additive`. The existing private `dot_zero_left` is
**promoted**, not duplicated as an alias: its private block is removed
and its public declaration occupies the fourth slot above. Its
conclusion now names `vector_zero n`, which unfolds to its previous
`repeat` form. Its proof body and private callers remain unchanged.

```ken
pub theorem vector_add_zero_left
    (n : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_add n (vector_zero n) u) u

pub theorem vector_add_zero_right
    (n : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_add n u (vector_zero n)) u

pub theorem vector_scale_zero
    (n : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n Zero u) (vector_zero n)

pub theorem dot_zero_left
    (n : Nat) (xs : Vec Nat n)
  : Equal Nat (dot n (vector_zero n) xs) Zero
```

Every statement is structural `Equal` for **all** natural `n`,
including zero, and has no validity, nonempty-vector or other
premise. The first two laws give `vector_zero n` as the left and
right unit of `vector_add`; scaling any vector by `Zero` yields that
zero vector; its dot product with any aligned vector is natural
`Zero`. They assert no commutative-monoid instance and change no
§1–§9 public operation or law type.

### 10.2 Proof route, private boundary and closed discriminator

Both unit proofs match on the written vector `u`, refining its
index so the private `repeat Nat n Zero` unfolds in each successor
case. On the left, the head is `add Zero a` and uses the existing
checked `(proof zero_l for add) a`; on the right, `add a Zero`
reduces definitionally, so `cong` transports the tail equality.
The scaling proof uses `(proof zero_l for mul) a` at each head.
The promoted dot-zero proof keeps its existing body: `vector_zero n`
is definitionally the former `repeat Nat n Zero`, so the four private
callers of `dot_zero_left` remain valid. `repeat`, `vector_cons_eq`,
`vector_add_cons` and the other helper declarations stay private.
These proof routes add no axiom, kernel or surface requirement.

A later CV-authored seed can take `n = 2`, `u = [1,2]` and check both
unit results as `[1,2]`, the scale-zero result and `vector_zero two`
as `[0,0]`, and the dot-zero result as `Zero`. At that closed length,
the Vec equalities accept `Refl`; the true `Equal Nat` dot-zero
claim uses `Proved`, not `Refl` (the latter is refused for this goal
shape). An empty-vector `Equal (Vec Nat Zero)` control also accepts
`Proved`. A refused same-scope `Equal (Vec Nat two)` claim that
`vector_scale two Zero u = u` must be paired with the accepted
same-form `vector_scale two (Suc Zero) u = u` control. The former
has `[0,0]` against `[1,2]`; the latter holds. An optional false
natural dot-zero claim must use `Proved` so its refusal tests the
value rather than an invalid `Refl` proof shape. Finite cases do not
replace the generic checked laws, and no seed is authored or run by
this spec-only turn.

The §9 public inventory on this base is eight operations and
twenty-six laws (thirty-four names). Adding `vector_zero` and
exporting the four theorems above yields **nine operations and thirty
laws, thirty-nine public names** when the catalog half is assembled.
The new names are owned by `NatMatrix`, not by `Data.Vector.Vector`;
`repeat` remains private. The exact-public-inventory package test
must advance to thirty-nine names with the catalog child. Architect
AC-0 measured this roster and equal scratch trust identities, not
candidate-local package tests, official conformance, CI or landing.
§11 advances the roster to nine operations and thirty-three laws,
forty-two public names.

## 11. Zero annihilation for dot, scaling and matrix application

> **Section 11 status: DRAFT v0
> (SPEC-LINEAR-ALGEBRA-ZERO-ANNIHILATION).** Architect AC-0 on exact
> base `a39d385c8` checked the three proposed generic proof bodies in
> scratch with no new `Axiom` and equal `trusted_base()` identity sets
> 109 → 109. The assembled candidate contains all three checked §11
> catalog laws, CV's authored seed and the package-owned test.
> Foundation's named package test reported 7/7 on this content; the
> seed was package-tested. That result does not establish official
> conformance-runner execution, and none is claimed here. Neither that
> candidate-local test nor scratch AC-0 establishes CI,
> publication, landing or a measured landed trust result.

### 11.1 Three premise-free public checked laws

`Algorithm.LinearAlgebra.NatMatrix` must export these three Transparent
checked theorems, in this order, immediately after `dot_zero_left`
and before `matrix_vector_additive`. Each parameter follows its
operation's argument order. The laws hold for every natural dimension,
including zero, with no premise or change to any §1–§10 operation or
public law type.

```ken
pub theorem dot_zero_right
      (n : Nat) (xs : Vec Nat n)
    : Equal Nat (dot n xs (vector_zero n)) Zero

pub theorem vector_scale_zero_vector
      (n : Nat) (factor : Nat)
    : Equal (Vec Nat n) (vector_scale n factor (vector_zero n)) (vector_zero n)

pub theorem matrix_vector_zero
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m)
    : Equal (Vec Nat m) (matrix_vector m n a (vector_zero n)) (vector_zero m)
```

`dot_zero_right` places the zero vector on the **right** of `dot`;
it is not a duplicate spelling of §10's left-zero law.
`vector_scale_zero_vector` scales the zero vector by **any** natural
factor, distinct from §10's `vector_scale_zero`, which scales any
vector by the particular factor `Zero`. `matrix_vector_zero`
sends the zero vector of length `n` to the zero vector of length `m`
for any indexed `m × n` matrix, not merely a square or zero matrix.
All three conclusions are structural `Equal` over their indicated
carriers; there is no nonzero-dimension exception, new scalar tier,
semiring abstraction, kernel rule, surface form or postulate.

### 11.2 Proof boundary and closed discriminator

The right-zero dot law is derived directly by `dot_comm n xs
(vector_zero n)` followed by §10's `dot_zero_left n xs`.
`vector_scale_zero_vector` matches on the **written `n`**, which
occurs in both sides of its indexed goal; the base case is empty,
and the successor transports its tail induction hypothesis under
`VCons Nat k Zero` using `cong`. Its head is `mul factor Zero`,
which reduces to `Zero` because `mul` recurses on its second argument.
No `mul::zero_r` is available to this module or required by the
proof; the law does not borrow a new public multiplication lemma.
`matrix_vector_zero` uses the symmetry of
`vector_scale_zero n (vector_zero n)` to express its input as the
zero scaling of itself, then `matrix_vector_homogeneous` at factor
`Zero`, followed by `vector_scale_zero` at length `m` on the image.
It has no match of its own. The existing `repeat` and
`vector_cons_eq` remain private; none of these laws exports a helper
or changes the earlier private proof boundary.

For a closed discriminator take `n = m = two`,
`u = [1,2]`, `zero2 = [0,0]`, and the nonzero matrix with rows
`[1,2]` and `[2,1]`. Its image of `u` is `[5,4]`, not `zero2`;
its image of `zero2` is `zero2`. After a fresh accepted prefix
that applies all three public laws, append alone the false claim
`matrix_vector two two a u = zero2` using `Refl`; it must refuse at
its own proof span. The otherwise identical claim with `zero2`
as the input, also using `Refl`, is an accepted same-form control.
Architect's D0 measured this proof-form distinction: `Proved`
refuses even the true vector-equality control, so it cannot serve
as the negative's discriminating proof form. A closed true
`Equal Nat` dot-zero claim likewise does not accept `Refl`; apply
`dot_zero_right` to establish the generic dot equality instead.
These finite rows separate values and proof forms; they do not
replace the generic checked theorems or claim a seed was run.
