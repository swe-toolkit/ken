# Finite least-cost candidate selection

> **Status: DRAFT v0 (SPEC-OPTIMIZATION-SEARCH-CONTRACT §1;
> SPEC-OPTIMIZATION-ARGMIN-OPTION §1.4;
> SPEC-OPTIMIZATION-BOUNDED-SEARCH §2).** Section 1 specifies the
> non-empty selector and its seven laws; its catalog module and seed
> are present on main. Section 1.4 specifies the Option selector and
> four additional public laws from Architect-checked scratch D0; its
> catalog extension and seed are not yet delivered. Section 2's catalog
> module and seed are also present on main. Scratch checks are not
> shipped-seed execution, CI or a landed `trusted_base()` measurement.

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

Real-valued and continuous optimization, infinite or implicit search
spaces, heuristics and approximation guarantees, maximization
(`argmax`), costs into an arbitrary `Ord`, multi-objective and
lexicographic costs, and top-k and stable multi-result selection remain
deferred. The Option-returning selector for a possibly empty `List` is
specified in §1.4, not deferred. The right-fold description is not a
machine-checked complexity result; all asymptotic and resource claims
remain deferred.

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
These are scratch observations, not the §1.4 catalog extension or
shipped-seed execution; no landed trust-identity census or CI result
is established here. The catalog extension must keep the helpers
private and add no ambient name outside the existing module imports.

## 2. Least witness below a finite bound

> **Section 2 status: DRAFT v0 (SPEC-OPTIMIZATION-BOUNDED-SEARCH).**
> Three general laws were checked in Architect D0; the catalog module
> and this section's conformance seed are now present on main. Scratch
> checks alone establish neither their shipped execution nor CI.

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
goals, for exactly these three generic theorem interfaces. The
result-equality premises connect each claim to the actual `Option`
result, and `Data.Numeric.Nat.Order.lt_nat` supplies strict order:

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
private helper is a fourth public law or a mandated implementation
strategy.

Section 1.4 now specifies the `Option`-returning selector over a
possibly empty `List`: it wraps `argmin` after distinguishing `Nil`
from `Cons`, not a search over a numeric range. It adds no §2 law.
General implicit or infinite search, heuristics and approximations,
arbitrary candidate orders, and machine-checked performance guarantees
remain outside this bounded contract.
