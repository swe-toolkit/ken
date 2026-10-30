# Finite candidate selection and bounded search

> **Status: DRAFT v0 (SPEC-OPTIMIZATION-SEARCH-CONTRACT §1;
> SPEC-OPTIMIZATION-ARGMIN-OPTION §1.4;
> SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE §1.5;
> SPEC-OPTIMIZATION-ARGMIN-APPEND-SPLIT §1.6;
> SPEC-OPTIMIZATION-BOUNDED-SEARCH §2;
> SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY §2.4;
> SPEC-OPTIMIZATION-ARGMAX §3;
> SPEC-OPTIMIZATION-ARGMAX-OPTION §3.4;
> SPEC-OPTIMIZATION-ARGMAX-COST-ORDER-INVARIANCE §3.5;
> SPEC-OPTIMIZATION-ARGMAX-APPEND-SPLIT §3.6).** Section 1 specifies
> the non-empty least-cost selector and seven initial laws; its catalog
> module and seed are present on main. Section 1.4's Option selector,
> four laws and seed are also present on main. Section 1.5's checked
> cost-order-invariance export and package-owned seed test are on main;
> prior candidate-local execution is not official conformance or a
> landed trust measurement. Section 1.6's two append-selection laws,
> catalog exports, CV seed and package-owned test are on this base;
> Architect's isolated D0 on `c99b3e998` checked the generic proofs
> with no new `Axiom` and equal scratch trusted identities 114 → 114.
> Earlier candidate-local checks are not official conformance or
> landed trust. Section 2's catalog module and seed are present on
> main. Section 3's greatest-cost selector and seven laws are landed;
> §3.4's Option extension and four laws, and §3.5's conditional
> cost-order-invariance law, have checked catalog exports, seeds and
> package tests on this base. Their earlier checks are not official
> conformance or landed trust. Section 3.6 specifies two public argmax
> append laws: False keeps left, including ties, and True takes right.
> Architect's isolated AC-0 on `1a3b2aa9f` checked both generic
> proofs with no `Axiom` and equal scratch trusted identities
> 114 → 114. This assembled candidate has both §3.6 catalog exports,
> a package-owned test and CV's authored seed. Foundation's two named
> targets passed 2/2 each on implementation tip `886f3557`, with equal
> trusted identity sets 114 → 114; two fixture controls reddened and
> were restored, while the raised keep-left mutation has no verdict
> after a build-lock timeout. The seed is unexecuted, and neither AC-0
> scratch nor parent tests are current-tip execution. No official
> conformance, CI, publication, landing or landed trust is claimed.
> The §2.4 law and package-owned seed test are on main; their local
> execution is not official conformance or landed trust.

## 1. Least-cost candidate

### 1.1 Carrier, operation and tie rule

The catalog Section is `Algorithm.Optimization`. Its landed module
`Algorithm.Optimization.Argmin`, at
`catalog/packages/Algorithm/Optimization/Argmin.ken.md`, exports:

```ken
pub fn argmin (a : Type) (cost : a → Nat) (xs : NonEmpty a) : a
```

`xs` is the **existing** `Data.Collections.NonEmpty.NonEmpty a`. The
operation selects one of its values, with the **least `Nat` cost** among
all members; when several candidates share that cost, it returns the
**earliest occurrence** in the input. It is defined for every `a` and
`cost : a → Nat`: no equality decision or order on `a` is required.
This is an exact finite-list selection, not a general search engine.
There is no empty-input case because `NonEmpty a` cannot be empty.

The selection is equivalent to a right fold over the head and tail:
compare the best value of the tail against the head, retaining the
head unless the tail's cost is **strictly less** according to
`Data.Numeric.Nat.Order.lt_nat`. The strict comparison, not a
non-strict replacement, ensures that an earlier tied candidate wins.
The `Nat` order's `leq_nat` states the optimality law. This describes
functional behavior, not a proven time or space bound.

The package reuses `NonEmpty`, its public smart constructors and
projections, `Data.Collections.Derived.list_elem`, and the existing
`Nat` order. It introduces no replacement list, membership predicate,
order instance, or kernel feature. The `NonEmptyCons` constructor is
private: clients form values with `nonempty_cons` or
`nonempty_singleton`, use `nonempty_head` and `nonempty_tail`, and
reason about the full ordered input through `nonempty_to_list` and
`nonempty_to_list_shape`. The shape equality transports a proof about
the private head/tail representation to the exported list view.

`list_elem a y (nonempty_to_list a xs)` is the established
**Ω-valued, propositionally truncated membership** of `y` in that
list. It is neither Boolean equality testing on `a` nor a position
witness. Thus the membership and optimality laws do not assume
`DecEq a` and do not assert which index was selected. The earliest
occurrence is specified separately by the tie rule and the checked
recursion equations below, which distinguish equal-cost values even
when they are not equal as values of `a`.

### 1.2 Seven checked laws

The module must export checked proof bodies, not postulates or
unproved goals, for **exactly** the seven interfaces below. In the
signatures, `list_elem` is the existing membership proposition;
`IsTrue (leq_nat …)` uses the proved natural order. The three
selection equations state the exact behavior, including both results
of the strict comparison:

```ken
pub theorem argmin_member
    (a : Type) (cost : a → Nat) (xs : NonEmpty a) :
  list_elem a (argmin a cost xs) (nonempty_to_list a xs)

pub theorem argmin_least
    (a : Type) (cost : a → Nat) (xs : NonEmpty a) :
  (y : a) → list_elem a y (nonempty_to_list a xs)
    → IsTrue (leq_nat (cost (argmin a cost xs)) (cost y))

pub theorem argmin_no_cheaper
    (a : Type) (cost : a → Nat) (xs : NonEmpty a) (y : a) :
  list_elem a y (nonempty_to_list a xs)
    → Equal Bool (lt_nat (cost y) (cost (argmin a cost xs))) False

pub theorem argmin_head_on_tie
    (a : Type) (cost : a → Nat) (x : a) (rest : List a) :
  ((y : a) → list_elem a y rest
    → IsTrue (leq_nat (cost x) (cost y)))
    → Equal a (argmin a cost (nonempty_cons a x rest)) x

pub theorem argmin_singleton (a : Type) (cost : a → Nat) (x : a) :
  Equal a (argmin a cost (nonempty_singleton a x)) x

pub theorem argmin_keeps_head
    (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a) :
  Equal Bool
    (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost x))
    False
    → Equal a (argmin a cost (nonempty_cons a x (Cons a y ys))) x

pub theorem argmin_takes_tail
    (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a) :
  Equal Bool
    (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost x))
    True
    → Equal a
        (argmin a cost (nonempty_cons a x (Cons a y ys)))
        (argmin a cost (nonempty_cons a y ys))
```

`argmin_member` says the result came from the given input rather than
being a fabricated cheaper value. `argmin_least` compares it against
**every** member, conditional on that member's membership proof.
`argmin_no_cheaper` rules out each purported strict improvement by a
member. These cost laws alone do **not** choose among tied candidates:
a selector returning the latest minimum could satisfy all three.
`argmin_head_on_tie` preserves the head if its cost is no greater than
that of any member of `rest`, even if a later value has equal cost.
`argmin_singleton`, `argmin_keeps_head`, and `argmin_takes_tail` fix the
singleton and both branches of every non-singleton selection. A tie
makes the strict comparison `False`, so it takes the keep-head branch.
No extra public law is inferred from these seven interfaces.

For a discriminating finite instance, assign distinct candidates
`3, 2, 0, 1` costs `5, 1, 9, 1`. The output is `2`, **not** the later
minimum `1`. For `3, 0, 1` with those costs, the unique minimum is
`1`, **not** `3` or `0`. A claimed result of `3` in the first case is
beaten by member `2`; a claimed result of `1` in the first case gets
the minimum cost but violates the earliest-tie rule. These two
refusals have different grounds, so neither a cost-only test nor a
tie-only test replaces the generic proof bodies. The separately
owned conformance seed supplies the executable closed instances and
negative controls.

### 1.3 Proof boundary, dependencies and deferred work

Architect D0 on base `17bf5738e` reported a `ken check`-accepted
216-line development of the operation and all seven general laws,
with zero `Axiom`, `postulate`, or `primitive` declarations. The D0
positive seeds checked, four individually enabled false claims were
rejected, and three implementation mutations failed: selecting a
later tie, reversing the cost order, or always retaining the head.
The tie mutation reddened the earliest-tie seed; the order-reversing
and always-head mutations reddened the strict-minimum seed. This is
**Architect-reported scratch evidence**. The §1 catalog module and its
seed are now present on main; that fact does not turn D0's scratch
runs into shipped-seed execution, CI, or a measured landed
`trusted_base()` delta. The landed module carries checked proof bodies
without introducing a new package assumption.

D0's membership proof uses the canonical `NonEmpty` list-view shape
equality, rather than destructuring its inaccessible constructor in a
client. The least proof eliminates truncated `list_elem` only into an
Ω-valued proposition and uses the proved `Nat` order, not an equality
decision on candidates. The Boolean comparison branch is handled with
a bound Boolean and its defining equality in the goal; the branch
proof cannot assume that a match on a computed term refines an
indirectly convertible expression. These are proof-strategy constraints,
not extra public interfaces.

The landed `Data.Numeric.Nat.Order` supplies two generic attached
`lt_nat` bridges: `lt_nat m n = True` implies `leq_nat m n`, and
`leq_nat m n` implies `lt_nat n m = False`. Earlier D0 proved both as
private helpers before that package exported them; the landed
`Argmin` imports their public proofs instead of maintaining duplicate
local order facts. They remain Nat-order laws, not eighth or ninth
optimization laws.

Greatest-cost selection (`argmax`) on `NonEmpty a` under a `Nat` cost
is specified in §3, not deferred. Real-valued and continuous
optimization, infinite or implicit search spaces, heuristics and
approximation guarantees, costs into an arbitrary `Ord`,
multi-objective and lexicographic costs, and top-k and stable
multi-result selection remain deferred. The Option-returning selector
for a possibly empty `List` is specified in §1.4, not deferred. The
right-fold description is not a machine-checked complexity result;
all asymptotic and resource claims remain deferred.

### 1.4 Option selector over a possibly empty list

The same `Algorithm.Optimization.Argmin` module extends §1 with a public
selector on the existing `List a`. It branches on the list's shape:
`Nil` returns `None a`; `Cons x rest` constructs the existing
`NonEmpty a` with `nonempty_cons` and returns `Some a` of `argmin` on
that input. No second selection algorithm, equality decision on `a`,
or order on `a` is needed. Its public definition and two checked
selection equations are:

```ken
pub fn argmin_option (a : Type) (cost : a → Nat) (xs : List a) : Option a =
  match xs {
    Nil ↦ None a;
    Cons x rest ↦ Some a (argmin a cost (nonempty_cons a x rest))
  }

pub theorem argmin_option_nil
  (a : Type) (cost : a → Nat) :
  Equal (Option a) (argmin_option a cost (Nil a)) (None a)

pub theorem argmin_option_cons
  (a : Type) (cost : a → Nat) (x : a) (rest : List a) :
  Equal (Option a)
    (argmin_option a cost (Cons a x rest))
    (Some a (argmin a cost (nonempty_cons a x rest)))
```

The `Cons` equality names the existing `argmin` result exactly, so it
inherits the earlier §1.2 earliest-tie behavior without restating its
head, singleton, or branch laws for `List`. `Nil` cannot yield a
`Some` result, and the `Cons` branch cannot yield `None`; this is a
constructor distinction, not a premise about the candidate type.

For a client holding only a list and an observed `Some y`, the module
also exports direct-to-`List` corollaries with checked bodies. The
first recovers truncated membership in the original input:

```ken
pub theorem argmin_option_member
  (a : Type) (cost : a → Nat) (xs : List a) (y : a) :
  Equal (Option a) (argmin_option a cost xs) (Some a y)
    → list_elem a y xs
```

Its private `option_member` predicate selects `Top` for `None` and
`list_elem` for `Some`. A private `argmin_option_member_base` splits
`xs` and uses the existing `argmin_member` in the `Cons` case; `J`
transports that proof along the observed result equality. The second
corollary bounds the selected cost against every input member:

```ken
pub theorem argmin_option_least
  (a : Type) (cost : a → Nat) (xs : List a) (y : a) :
  Equal (Option a) (argmin_option a cost xs) (Some a y)
    → (z : a) → list_elem a z xs
    → IsTrue (leq_nat (cost y) (cost z))
```

Its private `option_least` predicate likewise selects `Top` for
`None`; `argmin_option_least_base` uses the existing `argmin_least` in
the `Cons` case and `J` transports the result. These four helpers are
private proof steps, not additional public laws or trusted premises.
Membership is Ω-valued, not a Boolean equality test on `a`, and the
least-cost law quantifies only over members of the input list. The
other five §1.2 laws remain attached to `argmin`; the `Cons` equation
makes their behavior available without duplicating their interfaces.

Architect AC-0 checked this general development on base `1d82fd8ec`
at exact scratch tip `04f60997a494b12ca04ff91d3208f73a68e33fd5`:
`ken check` rc 0, with no `Axiom`, `Opaque` or `postulate` in the new
source. A scratch false claim of `Some` on `Nil` was refused with E0038.
These are scratch observations, separate from the §1.4 catalog extension.
The package test root-loads that extension, checks its five exports and
private helper boundary, and executes the positive and isolated refused
seed fences. It measures unchanged trusted-base identities on the
checked package; this source and test status claims neither official
conformance-runner execution nor CI, publication or landing. The extension
keeps the helpers private and adds no ambient name outside the existing
module imports.

### 1.5 Invariance under a strict-order-preserving cost transform

`Algorithm.Optimization.Argmin` must export one further public
Transparent theorem with a checked body, not a postulate, with the
following exact type and argument order:

```ken
pub theorem argmin_cost_order_invariant
      (a : Type)
      (cost : a → Nat)
      (f : Nat → Nat)
      (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
      (xs : NonEmpty a)
    : Equal a (argmin a (comp a Nat Nat f cost) xs) (argmin a cost xs)
```

The premise says that `f` preserves **and reflects** every strict
comparison of natural costs. It is in public names only: `lt_nat`,
`Equal Bool` and the two natural inputs. In particular, when two
original costs tie, neither strict comparison becomes true after
`f`, so §1.1's earlier tied candidate remains selected. A strictly
increasing transformation may change the numeric costs and need
not be surjective; it cannot change the chosen input value. The
conclusion is structural `Equal a` on the selected candidate,
not equality of costs, a new optimization operation, or a claim
about runtime complexity. It applies to every candidate type,
nonempty input and total cost function without `DecEq a`.

`comp` is the existing public `Core.Function.Combinators.comp`;
`comp a Nat Nat f cost x` computes `f (cost x)`. A lambda used as
an argument inside this public theorem's type is not accepted by
Ken's type-position syntax. A public client may use `f` directly
and discharge `same_order` by any checked proof. For
`f c = add c three`, both sides of the order equality reduce to
the same Boolean, so `Refl` checks on open `i` and `j`. This is
an instance, not a second generic law.

Order equality fixes every Boolean comparison the existing
right-fold selector makes on a tail result and its current head.
The private `select_from_cost_order` inducts over the tail and
transports the induction hypothesis through the head choice;
`choose_cost_order` transports the Boolean comparison with `J`.
Both helpers remain private, and the seven §1.2 laws and the
§1.4 selector and laws retain their existing types and bodies.
This explains Architect's checked D0 route, not a prescribed
implementation strategy.

The premise cannot be omitted. On labelled candidates
`[0, 1, 2, 3]` with respective costs `[5, 2, 7, 2]`, both
`cost` and `add three ∘ cost` select label `1`, the earliest of
two minima away from the head. The constant transform
`f c = Zero` instead ties every candidate and selects head `0`;
claiming it selects the original minimum `1` is false. The
closed seed independently checks each accepted selection and
the flattened transform's actual head result before refusing
the false equality on the **same input**. These instances do
not prove the universally quantified theorem.

Architect's isolated AC-0 on `12e91c41b` checked the generic
proof with `ken check` rc 0, `ken fmt --check` rc 0, no new
`Axiom`, owner-local Transparent status, private helper import
refusals, and unchanged exact scratch `trusted_base()` identities
114 → 114. It also checked the four-label client and its false
flattened-cost claim. That historical D0 and the earlier
spec-and-seed candidate did not deliver the catalog export. The
checked public law, private helpers and package-owned test are now
present on main. The test root-loads
`Argmin`, consumes the full generic public type, checks the accepted
§1.5 client including both independently reduced selection values
and the constant-cost head result, then rejects the isolated false
flattened-cost claim. Its candidate-local trust comparison and seed
execution are not official conformance-runner execution or a landed
trust measurement. The public §1.5 type and its proof boundary remain
unchanged by §3.5 or the §1.6 extension below.

### 1.6 Selection over two appended non-empty parts

`Algorithm.Optimization.Argmin` must export checked, public Transparent
proof bodies for the following two laws in this order, after
`argmin_cost_order_invariant` and before `argmin_option`. Both
`xs` and `ys` use the existing `NonEmpty a`; their concatenation is
`Data.Collections.NonEmpty.nonempty_append`. The comparison asks
whether the best value of the **later** `ys` is strictly cheaper than
the best value of the **earlier** `xs`:

```ken
pub theorem argmin_append_keeps_left
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) False
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost xs)

pub theorem argmin_append_takes_right
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) True
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost ys)
```

On `False`, including an equal-cost tie between the two parts, the
append result is the left part's choice. On `True`, the right part is
strictly cheaper and its choice wins. Each part retains §1.1's own
earliest-tie rule, so a tied choice spanning the split is the
**earlier left** choice, even if the left and right values differ.
These are structural `Equal a` results about the selected candidate,
not mere equality of minimum costs. They quantify over every candidate
type and total natural cost function, without `DecEq a`, candidate
ordering, a non-strict cost premise, or exposure of a private helper.
Neither law asserts a function equality or a machine complexity bound.

The generic proof route composes the existing strict selector with
the public `nonempty_append` list-view equality and
`nonempty_to_list_shape`. The private list-fold combination uses
strict-order transitivity and the false-comparison order facts to
preserve the earliest tied result. `choose` remains private, as do
the shared append helpers `select`, `select_view`,
`select_via_view`, `select_append_choice`, `select_from_append` and
`choose_better_assoc`, their order-fact aliases
`holds_at` and `transitive_at`, and the `lt_nat_true_trans` and
`lt_nat_false_trans` facts that instantiate them for `argmin`; none is
an extra public law, trust assumption, or permitted premise in these
types. The proof does not require access to `NonEmptyCons`. Section
1.2's seven interfaces, §1.4's selector and four interfaces, §1.5's
law, and §§2–3 retain their existing operation and law types. The
argmax mirror is §3.6, not part of §1.6.

The assigned closed seed splits labels `0, 1 | 2, 3` with costs
`5, 2 | 2, 7`. Both parts have cost `2` as their minimum, but the
append selects left label `1`; it applies `argmin_append_keeps_left`
and checks the selected label independently with `Proved`. Lowering
label `2`'s cost to `1` instead exercises
`argmin_append_takes_right` and independently selects right label
`2`. A false claim that the original tied append selects the right
part's label `2` is refused at its **appended theorem** after a fresh,
fully accepted prefix. An independent accepted left result and
right result on the same split isolate the difference; these closed
instances do not prove either generic law. The seed and its
package-owned test are CV- and Foundation-owned turns, respectively.

Architect's isolated AC-0 on exact base
`c99b3e998b5993f87acdda57afeaf19a2d72ca8b` checked both generic
bodies, a generic public client, the closed tie and strict-right
instances, and isolated false claims with `ken check` rc 0 for the
accepted fixtures, no `Axiom`, and unchanged scratch `trusted_base()`
GlobalId sets 114 → 114. The private helpers refused import. This is
scratch design evidence, not itself catalog delivery. This assembled
candidate includes the two checked §1.6 catalog exports, the CV seed,
and a package-owned test of the complete accepted client and isolated
later-tie refusal. Candidate-local checks pass 2/2, and a true-neighbor
mutation reddens the isolated refusal at its behavioral assertion.
Neither the scratch D0 nor these candidate-local checks are official
conformance-runner execution, CI, publication, landing or landed trust.
No new primitive, kernel or surface feature is introduced.

## 2. Least witness below a finite bound

> **Section 2 status: DRAFT v0 (SPEC-OPTIMIZATION-BOUNDED-SEARCH;
> SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY §2.4).** The
> original three general laws, catalog module and seed are present on
> main. Section 2.4's fourth law was checked in Architect D0; its
> catalog export and package-owned test are on main. The roots-loaded
> test checks the authored seed, the complete public type, and the
> rejected `None` counterpart. These local tests do not establish
> official conformance-runner execution, CI or landed trust.

### 2.1 Carrier and result

The catalog Section remains `Algorithm.Optimization`. Its landed
module `Algorithm.Optimization.BoundedSearch`, at
`catalog/packages/Algorithm/Optimization/BoundedSearch.ken.md`, exports:

```ken
pub fn find_below (p : Nat → Bool) (bound : Nat) : Option Nat
```

The search domain is the **finite, explicitly bounded** range of natural
numbers `0 ≤ n < bound`. For a Boolean predicate `p`, `find_below`
returns `Some Nat n` for the **least** `n` in that range with
`p n = True`; if no such `n` exists, it returns `None Nat`. At bound
`Zero`, the range is empty and the result is `None Nat`. The carrier
`Option Nat` is the existing prelude sum with `None` and `Some`:
neither a new container nor an equality decision on predicates is
introduced. Unlike §1's `argmin`, the operation receives a finite
numeric bound and a predicate, not an already populated `NonEmpty a`.
This does not specify infinite or unconstrained implicit search.

The contract is **extensional**. In particular, it does not prescribe
search direction, recursion shape, early exit, a predicate-call count,
or an asymptotic or resource bound. Implementations satisfying the
three laws below may use different evaluation strategies.

### 2.2 Three checked laws

The module must export checked proof bodies, not postulates or unproved
goals, for the three generic theorem interfaces below, plus the
bound-stability law of §2.4. The result-equality premises connect each
claim to the actual `Option` result, and
`Data.Numeric.Nat.Order.lt_nat` supplies strict order:

```ken
pub theorem find_below_sound
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : Equal (Option Nat) (find_below p bound) (Some Nat n)
      → And (Equal Bool (lt_nat n bound) True) (Equal Bool (p n) True)

pub theorem find_below_least
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : Equal (Option Nat) (find_below p bound) (Some Nat n)
      → (m : Nat) → Equal Bool (lt_nat m n) True → Equal Bool (p m) False

pub theorem find_below_complete
      (p : Nat → Bool) (bound : Nat)
    : Equal (Option Nat) (find_below p bound) (None Nat)
      → (m : Nat) → Equal Bool (lt_nat m bound) True → Equal Bool (p m) False
```

`find_below_sound` establishes both the strict upper bound and the
predicate hit whenever a result is `Some n`. `find_below_least` rules
out any earlier hit: **every** `m < n` has `p m = False`.
`find_below_complete` rules out **every** hit within the bound when
the result is `None`. Together, these laws determine the returned
value or its absence; they do not add a witness for a result reported
as `None`, an unconstrained search domain, or a cost guarantee.

For a discriminating finite instance, take `p n` to mean `2 < n`.
Bound `10` returns `Some 3`, **not** `Some 5` even though `5` also
satisfies `p`: the smaller witness `3` beats it. `None` at bound `10`
is false because `3` is a hit. Bound `3` returns `None` because none
of `0, 1, 2` satisfies `p`, and bound `Zero` also returns `None`.
A claimed `Some 2` at bound `3` is false for a different reason:
`p 2 = False`. These are behavioral illustrations, not a claim that
the separately owned conformance seed has been committed or run.

### 2.3 Proof evidence and boundary

Architect D0 on landed base `0cd98b0cd` reported `ken check` rc 0 for
a 167-line development of the operation and all three general laws,
with zero `Axiom`, `postulate`, `primitive`, or opaque declarations. The
D0 positive client seeds checked; three individually appended false
claims were refused at their own declarations. Mutating the
implementation to prefer a later witness, never report a witness, or
include the bound made the proof development fail, and restoring each
mutation restored rc 0. These are **Architect-reported scratch
results**. The §2 catalog module and seed are now present on main;
these earlier D0 results do not establish shipped-seed execution, CI,
or a landed trust-identity measurement.

The D0 proof route inducts over the finite bound with a private
outcome invariant: a missing result means no hit below the bound; a
found result carries proofs that its witness is below the bound,
satisfies the predicate, and has no earlier hit.
Transport along the result equality yields the three public laws
without requiring a new `Option` injectivity or discrimination law.
The proof of the missing case also uses the private Nat fact that
`m < Suc b` but not `m < b` forces `m = b`. Splitting on a
**separately bound `Bool` variable** rather than matching dependently
on an indirectly computed `p b` avoids assuming a refinement the
elaborator does not provide. These explain the D0 evidence; no
private helper is an additional public law or a mandated
implementation strategy. Section 2.4 introduces a fourth public law
with its own private `leq_suc_not_leq_same` proof helper.

Section 1.4 now specifies the `Option`-returning selector over a
possibly empty `List`: it wraps `argmin` after distinguishing `Nil`
from `Cons`, not a search over a numeric range. It adds no §2 law.
General implicit or infinite search, heuristics and approximations,
arbitrary candidate orders, and machine-checked performance guarantees
remain outside this bounded contract.

### 2.4 Bound stability for an observed witness

The existing `Algorithm.Optimization.BoundedSearch` module must export
one further **public Transparent theorem** with a checked proof body,
not a postulate. Its name, argument order and exact public type are:

```ken
pub theorem find_below_stable
      (p : Nat → Bool) (bound : Nat) (larger : Nat) (n : Nat)
    : Equal Bool (leq_nat bound larger) True
      → Equal (Option Nat) (find_below p bound) (Some Nat n)
      → Equal (Option Nat) (find_below p larger) (Some Nat n)
```

For every Boolean predicate and natural bounds with
`leq_nat bound larger = True`, an observed `Some n` below `bound`
remains **the same** `Some n` at `larger`. The observed result is the
second premise, as in the existing soundness and leastness laws. The
non-strict `leq_nat` premise covers an unchanged bound as well as a
strictly greater one; no distinctness or positive-extension premise
is required. The statement is not a promise about predicate calls,
search direction or computational cost.

The premise directly states the order relation a client ordinarily
knows. A client with an extension written `add bound extra` can apply
this same law using the existing public Nat-order fact
`leq_nat_add_left_bound bound extra`. The reverse choice—requiring an
additive decomposition rather than `leq_nat bound larger = True`—would
oblige such an order-only client to derive an additional decomposition
lemma that the current Nat-order interface does not export. No second
public bound-stability law is introduced for the additive spelling.

The Architect's checked proof inducts on `larger`. With `larger = Zero`,
the premises contradict either a nonzero `bound` or a `Some` result at
`bound = Zero`. At `Suc below`, it splits on the computed Boolean
`leq_nat bound below` **with its equation**. If true, the inductive
hypothesis finds the same `Some n` below `below`, which `find_next`
keeps at `Suc below`. Otherwise a new private Nat-order helper
`leq_suc_not_leq_same` shows `bound = Suc below`, and equality
transport reuses the observed result. The helper belongs directly
after the existing private `lt_suc_not_lt_same`; it is not a public
fifth law or a new assumption. The proof changes no §1–3 operation
or existing theorem, and uses only the existing private `find_next`,
congruence, symmetry and transitivity. Matching on a computed order
comparison supplies an explicit equation, not a refinement inferred
from an indirectly convertible expression.

There is **no corresponding `None`-stability law**. For
`above_two n = lt_nat two n`, `find_below above_two three` is `None`,
while `find_below above_two ten` is `Some three`. Growing a bound can
introduce a first hit; the accepted same-premise `Some` result and the
refused false `None` conclusion are paired in the closed seed at
`conformance/stdlib/optimization/seed-bounded-search-bound-stability.md`.
The finite instance checks the boundary but does not prove the
generic theorem. Architect's isolated D0 at landed base `0bb872591`
checked the generic body (`ken check` and `ken fmt --check` rc 0), two
probe tests, no new `Axiom`, and identical scratch `trusted_base()`
GlobalId sets 114 → 114. The §2.4 catalog export and a roots-loaded
package test are now on main; that test checks its full generic
interface, the seed's accepted client, and refusal of the same-premise
false `None` claim. The test measures identical exact `trusted_base()`
GlobalId sets before and after loading (114 → 114), and refuses selective
import of the private helper. These local checks are **not** official
conformance-runner execution, CI, publication, landing or a landed trust
measurement.

## 3. Greatest-cost candidate

> **Section 3 status: DRAFT v0 (SPEC-OPTIMIZATION-ARGMAX;
> SPEC-OPTIMIZATION-ARGMAX-OPTION §3.4;
> SPEC-OPTIMIZATION-ARGMAX-COST-ORDER-INVARIANCE §3.5;
> SPEC-OPTIMIZATION-ARGMAX-APPEND-SPLIT §3.6).** The §3.1–§3.2
> operation and seven laws, plus their seed, are present on main.
> Architect checked §3.4 on an isolated D0 from `914cfbda0`; the
> §3.4 exports, seed and package-owned test are now on main. Those
> package checks are not official conformance or landed trust.
> Section 3.5's catalog law, seed and package-owned test are on this
> base; its earlier isolated D0 on `6e778c2d5` does not establish
> official conformance or landed trust. Section 3.6 specifies two
> public argmax append laws; Architect's isolated AC-0 on `1a3b2aa9f`
> checked them with no new `Axiom` and equal scratch trusted identity
> sets 114 → 114. This assembled candidate includes both checked
> §3.6 catalog exports, the package-owned test and CV's authored seed.
> Foundation's named argmax and argmin targets each passed 2/2 with
> equal trusted identity sets 114 → 114 on implementation tip
> `886f3557`; two fixture-only controls reddened and were restored.
> The raised keep-left mutation has no verdict after a build-lock
> timeout. The seed has not been Ken-checked or officially run, and
> neither scratch AC-0 nor parent tests are current-tip execution.
> No CI, publication, landing or landed trust is claimed.

### 3.1 Carrier, operation and tie rule

The existing `Algorithm.Optimization.Argmin` module extends its
`NonEmpty a` selectors with:

```ken
pub fn argmax (a : Type) (cost : a → Nat) (xs : NonEmpty a) : a
```

For every candidate type `a` and total `cost : a → Nat`, `argmax`
returns a member of `xs` with the **greatest natural cost**. Among
candidates tied for that cost, it returns the **earliest occurrence**.
No `DecEq a` or order on candidates is required. An empty-input
case is excluded by `NonEmpty`; this operation neither returns an
`Option` nor changes `argmin_option`'s §1.4 contract.

At each non-empty head/tail decomposition, a candidate selected from
the tail replaces the head only if the head's cost is **strictly less**
than the tail candidate's cost under `lt_nat`. An equal-cost tail
candidate does not replace the earlier head. This is the observable
choice fixed by the branch laws below, not a requirement to expose a
particular fold or helper. Like §1, membership uses the existing
Ω-valued, propositionally truncated `list_elem` over
`nonempty_to_list`, and optimality uses the established `Nat` order.
No position witness, equality decision on `a`, new built-in, or
complexity guarantee is introduced.

### 3.2 Seven checked laws

The same module must export proof bodies for **exactly** these seven
new public theorem interfaces. The order predicates refer to the
actual cost of the selected input member; all quantification over
`y` in the greatest-cost law is conditioned on input membership:

```ken
pub theorem argmax_member (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : list_elem a (argmax a cost xs) (nonempty_to_list a xs)

pub theorem argmax_greatest (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : (y : a) → list_elem a y (nonempty_to_list a xs)
      → IsTrue (leq_nat (cost y) (cost (argmax a cost xs)))

pub theorem argmax_no_costlier (a : Type) (cost : a → Nat) (xs : NonEmpty a) (y : a)
    : list_elem a y (nonempty_to_list a xs)
      → Equal Bool (lt_nat (cost (argmax a cost xs)) (cost y)) False

pub theorem argmax_head_on_tie (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : ((y : a) → list_elem a y rest → IsTrue (leq_nat (cost y) (cost x)))
      → Equal a (argmax a cost (nonempty_cons a x rest)) x

pub theorem argmax_singleton (a : Type) (cost : a → Nat) (x : a)
    : Equal a (argmax a cost (nonempty_singleton a x)) x

pub theorem argmax_keeps_head (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a)
    : Equal Bool (lt_nat (cost x) (cost (argmax a cost (nonempty_cons a y ys)))) False
      → Equal a (argmax a cost (nonempty_cons a x (Cons a y ys))) x

pub theorem argmax_takes_tail (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a)
    : Equal Bool (lt_nat (cost x) (cost (argmax a cost (nonempty_cons a y ys)))) True
      → Equal a (argmax a cost (nonempty_cons a x (Cons a y ys)))
          (argmax a cost (nonempty_cons a y ys))
```

`argmax_member` excludes an invented greater-cost value;
`argmax_greatest` bounds every member by the selected cost; and
`argmax_no_costlier` excludes a strictly more expensive input member.
Those three laws alone do not choose among tied maxima. When every
member of `rest` costs at most the head, `argmax_head_on_tie` returns
the head even if another member costs the same. The singleton and
strict-comparison branch laws determine both outcomes for every
non-singleton input. A tie makes the comparison `False` and keeps
the earlier head. These seven are additional to, and do not change,
§1.2's seven `argmin` laws or §1.4's four Option laws.

For a finite discriminator, set costs `0 ↦ 9`, `1 ↦ 1`, `2 ↦ 1`,
and every value at least `3 ↦ 5`. On `[1, 3, 2, 4]`, `3` and `4`
tie for the greatest cost, so the result is `3`, **not** later `4`.
Replacing on a non-strict comparison would instead return `4` and
violate the earliest-tie rule while still meeting greatest-cost
optimality. On `[3, 0, 1]`, the strictly greater-cost `0` wins.
These are illustrative instances of the general laws, not shipped
conformance results; the closed seed is separately owned.

### 3.3 Proof boundary and deferred work

Architect's isolated D0 on `136f959ad` reported package `ken check`
and `ken fmt --check` rc 0, 114→114 identical `trusted_base()` GlobalId
sets, and no `Axiom`, `Opaque`, or `Primitive` among 41 package-owned
declarations. A generic client restating all eight `argmax` statements
checked. The closed earliest-tie control selected `3` and its
later-tie refusal `4` reached the kernel's `TypeMismatch`; a local
non-strict selector reversed those two observations. These are
Architect-reported D0 measurements for §3.1–§3.2; the separately
delivered §3.4 package test executes its authored seed.

The checked D0 shares a private selection fold and membership proof
between `argmin` and `argmax`, with a private greatest-cost proof and
order helpers for the new direction. The public declarations of
`argmin`, its seven laws, and `argmin_option` and its four laws stay
unchanged. Fold sharing is a package design choice, not another public
selector, law, dependency, or required execution strategy.

Option-returning maximization on a possibly empty input is specified
in §3.4, and append-split selection in §3.6; neither is deferred.
Arbitrary candidate orders or equality decisions, multiple sorted
results, continuous optimization, approximation guarantees and
performance bounds remain outside §3.
In particular, §3 supplies no top-k selection.

### 3.4 Greatest-cost Option selector over a possibly empty list

For a possibly empty `List a`, the same
`Algorithm.Optimization.Argmin` module extends §3 with:

```ken
pub fn argmax_option (a : Type) (cost : a → Nat) (xs : List a) : Option a =
  match xs {
    Nil ↦ None a;
    Cons x rest ↦ Some a (argmax a cost (nonempty_cons a x rest))
  }
```

The selected member has the greatest `Nat` cost and retains §3.1's
earliest-tie rule. This constructor split does not grant a public
position witness or require a new selector primitive. The four public
checked interfaces are exactly:

```ken
pub theorem argmax_option_nil
      (a : Type) (cost : a → Nat)
    : Equal (Option a) (argmax_option a cost (Nil a)) (None a)

pub theorem argmax_option_cons
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : Equal
        (Option a)
        (argmax_option a cost (Cons a x rest))
        (Some a (argmax a cost (nonempty_cons a x rest)))

pub theorem argmax_option_member
      (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    : Equal (Option a) (argmax_option a cost xs) (Some a y) → list_elem a y xs

pub theorem argmax_option_greatest
      (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    : Equal (Option a) (argmax_option a cost xs) (Some a y)
      → (z : a)
      → list_elem a z xs
      → IsTrue (leq_nat (cost z) (cost y))
```

The first law holds on `Nil`; the second exposes the `Cons` case
through public `nonempty_cons` and `argmax`. The remaining two are
direct-to-`List` corollaries conditioned on an observed `Some y`.
They do not assert that a result exists on `Nil` or require `DecEq a`,
an order on `a`, or function extensionality. The checked development
proves the `Nil` law with `Proved` and the `Cons` law with `Refl`.
Private `option_greatest`, `argmax_option_member_base`, and
`argmax_option_greatest_base` use the existing private
`option_member` predicate without changing it. The `Nil`/`Cons`
proof split and `J` follow §1.4, but the `_base` proofs remain
selector-specific: none of these helpers is a fifth public law.
The §1.2, §1.4 and §3.2 public types and proof bodies stay unchanged.

Architect's checked §3.4 D0 from `914cfbda0` reported `ken check` and
`ken fmt --check` rc 0 on the extended package, no new `Axiom`, and
114→114 identical `trusted_base()` identities over its eight
providers. The four new generic laws were consumed by a scratch
client; the `Nil`, singleton and tied-maximum controls and five
false-claim probes were checked separately in scratch. Those are
Architect-reported measurements. The landed §3.4 package test instead
loads its catalog exports and checks the seed's accepted client and
each isolated refused theorem. It checks the exact trusted-base
identities relative to the provider closure, without claiming a
landed trust measurement. No ambient name beyond the
package's current census row or change to existing selectors is
required. Option minimization remains §1.4; `argmax_option` adds no
`argmax` law, generic Option search, top-k selection or runtime-cost
promise.

### 3.5 Invariance under a strict-order-preserving cost transform

`Algorithm.Optimization.Argmin` must export one more public Transparent
checked theorem immediately after `argmax_takes_tail` and before
`argmax_option`. Its exact name, binder order and type are:

```ken
pub theorem argmax_cost_order_invariant
      (a : Type)
      (cost : a → Nat)
      (f : Nat → Nat)
      (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
      (xs : NonEmpty a)
    : Equal a (argmax a (comp a Nat Nat f cost) xs) (argmax a cost xs)
```

As in §1.5, `same_order` states equality of each strict Nat-order
comparison before and after the transform: it both preserves and
reflects strict order. The premise names only public `lt_nat`,
`Equal Bool`, and Nat indices. The result is structural `Equal a`
of the two selected **input values** for any type `a`, total cost
function and nonempty input. It is not equality of transformed
numeric costs, an operation on arbitrary orders, or a runtime-cost
claim. Equal original costs cannot become strictly ordered under
`f`; unchanged strict comparisons also preserve the earlier winner
when the maximum is tied away from the head. No `DecEq a` is needed.
`comp` is the existing public `Core.Function.Combinators.comp`,
not a new selector or a lambda in a public type position.

The §1.5 `argmin_cost_order_invariant` theorem keeps its exact public
type, including its sole `same_order` premise and `NonEmpty a` input.
No §1.2, §1.4, §3.2 or §3.4 public operation or law changes. The
existing private `select_from_cost_order` and `choose_cost_order`
can serve both laws by taking a private `better : Nat → Nat → Bool`
and a private equality premise for that comparison. For argmin,
`better` is `lt_nat` and its existing public `same_order` is used
unchanged. For argmax, `better` is the existing private `costlier`:
`costlier challenger incumbent` reduces to
`lt_nat incumbent challenger`, so `same_order j i` supplies the
reversed comparison without a second public premise. The old argmin
proof boundary remains one private selection lemma (whose private
choice step is likewise shared): its body passes `lt_nat` to the
generalized lemma, while its public type is byte-identical.
Generalizing those helpers does not export them or change the
theorem clients import. The helpers
and `costlier` remain private. This states the checked D0 route;
Foundation owns its catalog implementation, with no new import,
assumption, built-in, or trusted-base entry.

The premise is necessary. Label the four candidates `0, 1, 2, 3`,
with respective costs `5, 7, 2, 7`. For `f c = add c three`, the
strict-order comparisons agree on open costs and both selectors
independently return label `1`, the **earlier of the two tied maxima**
and not the head. A constant transform `flattened c = Zero` instead
turns every cost into a tie and returns head `0`; the original
selector still returns `1`. A refused claim equating those two
selections on the same input must be isolated after a fresh accepted
prefix. The seed checks the original result, shifted result and
flattened head independently before refusing the false equality at
its own proof term. Closed examples discriminate the premise; they
do not establish the generic checked law.

Architect's isolated AC-0 on exact landed base
`6e778c2d5288219287884de49eb53fb278ba98ea` checked the
generic proof and both public argmin/argmax types, `ken check` and
`ken fmt --check` rc 0, no new `Axiom`, package-owned Transparent
status, three private import refusals, and equal scratch
`trusted_base()` identity sets 114 → 114. Its accepted seed client
and separately refused flattened-cost claim also checked as expected;
the existing five named optimization package-test binaries passed
in that scratch overlay. Those are Architect D0 observations, not
shipped §3.5 execution. The catalog export was absent on the spec
author's base; this assembled candidate adds its checked proof and
a package-owned test that root-loads the complete accepted seed and
refuses the isolated false claim. The generalized helpers remain
private, and §1.5's public type is unchanged. Neither scratch D0 nor
these candidate-local checks establish official conformance-runner
execution, CI, publication, landing or a landed trust measurement.

### 3.6 Selection over two appended non-empty parts

> **Section 3.6 status: DRAFT v0
> (SPEC-OPTIMIZATION-ARGMAX-APPEND-SPLIT).** Architect's isolated
> AC-0 on `1a3b2aa9f` checked both generic proofs with no new
> `Axiom`, equal scratch trusted GlobalId identity sets 114 → 114,
> and a tie/strict-right client. This assembled candidate has both
> checked public exports, a package-owned test and CV's closed seed.
> On implementation tip `886f3557`, the named argmax and argmin
> targets each passed 2/2 and compared trusted GlobalId identity sets
> equal at 114 → 114. Two fixture-only refusal controls compiled, went
> red and were restored; `reject_keep_left_on_raised` has no verdict
> after a build-lock timeout. The seed is authored but has not been
> Ken-checked or officially conformance-run. AC-0 is isolated scratch
> evidence; the parent checks did not rerun on the current tip. No CI,
> publication, landing or landed trust is claimed.

`Algorithm.Optimization.Argmin` must export checked public Transparent
proof bodies for two laws, in the following order, immediately after
`argmax_cost_order_invariant` and before `argmax_option`. Both parts
are `NonEmpty a`, with `nonempty_append a xs ys` the existing append.
The public comparison asks whether the left part's selected cost is
strictly lower than the right part's selected cost:

```ken
pub theorem argmax_append_keeps_left
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmax a cost xs)) (cost (argmax a cost ys))) False
      → Equal a (argmax a cost (nonempty_append a xs ys)) (argmax a cost xs)

pub theorem argmax_append_takes_right
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmax a cost xs)) (cost (argmax a cost ys))) True
      → Equal a (argmax a cost (nonempty_append a xs ys)) (argmax a cost ys)
```

When the comparison is `False`, including a tied maximum across the
split, the appended selection equals the left part's choice. When it
is `True`, the right part is strictly costlier and its choice wins.
Each part retains §3.1's earliest-tie rule internally, so even when
the two parts select different equal-cost values, the combined choice
is the earlier left one. These are structural `Equal a` results about
selected input values, not merely equality of maximum costs. They
hold for every candidate type and total natural cost function,
without `DecEq a`, a candidate order, a non-strict cost premise, or a
private name in either public type. No function equality or runtime
complexity bound is asserted.

The proofs share §1.6's private append helpers generalized over the
strict comparator `better`, rather than duplicating the induction
for `argmax`. The `argmin` proof passes `lt_nat`; the `argmax` proof
passes the private `costlier`, which reduces to
`lt_nat incumbent challenger`. This makes each private choice
Boolean definitionally
equal to its public `lt_nat` premise. The generic induction uses the
True and False transitivity facts for the chosen comparator, but
neither is an additional public premise. `choose`, `costlier`,
`select`, `select_view`, `select_via_view`, `holds_at`,
`transitive_at`, `select_append_choice`, `select_from_append`,
`choose_better_assoc`, `lt_nat_true_trans`, `lt_nat_false_trans`,
`costlier_true_trans` and `costlier_false_trans` all stay private.
The two existing §1.6 public types and all other §1–§3.5 operation
and law types remain unchanged; no new `Axiom`, public Nat law,
primitive, kernel or surface feature is required.

The assigned closed seed uses labels `0, 1 | 2, 3` with costs
`2, 7 | 7, 1`: the maximum ties across parts, so the appended
selector returns left label `1`, independently checked by `Proved`
in addition to applying `argmax_append_keeps_left`. Raising label
`2`'s cost to `8` makes the comparison `True` and selects right
label `2` through `argmax_append_takes_right`, again checked
independently. A claim that the tied original append selects the
right part's label `2` must refuse after a fresh accepted prefix.
Separate positive selections of both parts isolate the tie: a
selector that takes right on ties would produce a different value.
These closed cases discriminate the branch rule; the generic laws
rest on checked proof bodies, not the closed instances.
