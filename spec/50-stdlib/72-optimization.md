# Finite least-cost candidate selection

> **Status: DRAFT v0 (SPEC-OPTIMIZATION-SEARCH-CONTRACT).** Section 1
> normatively specifies a least-cost selector over an existing non-empty
> list with seven laws proved in Architect D0. The catalog module and
> conformance seed are prospective; neither shipped checks, CI, nor a
> measured `trusted_base()` delta is claimed.

## 1. Least-cost candidate

### 1.1 Carrier, operation and tie rule

The catalog Section is `Algorithm.Optimization`. Its prospective module
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
**Architect-reported scratch evidence**, not a delivered catalog
module, a committed or executed conformance seed, CI, or a measured
`trusted_base()` delta. A future catalog module must carry the actual
checked proof bodies and preserve the zero-new-assumption construction.

D0's membership proof uses the canonical `NonEmpty` list-view shape
equality, rather than destructuring its inaccessible constructor in a
client. The least proof eliminates truncated `list_elem` only into an
Ω-valued proposition and uses the proved `Nat` order, not an equality
decision on candidates. The Boolean comparison branch is handled with
a bound Boolean and its defining equality in the goal; the branch
proof cannot assume that a match on a computed term refines an
indirectly convertible expression. These are proof-strategy constraints,
not extra public interfaces.

A catalog follow-on should place two generic Nat-order bridges at their
natural home, `Data.Numeric.Nat.Order`: `lt_nat m n = True` implies
`leq_nat m n`, and `leq_nat m n` implies `lt_nat n m = False`. D0
proved both as private helpers because that package does not yet
export them. Publishing them as attached `lt_nat` proofs and importing
them into `Argmin` avoids maintaining duplicate private order facts;
this is a **separately authorized catalog obligation**, not part of
the §1 author-slice paths and not an eighth or ninth optimization law.

Real-valued and continuous optimization, infinite or implicit search
spaces, heuristics and approximation guarantees, maximization
(`argmax`), costs into an arbitrary `Ord`, multi-objective and
lexicographic costs, top-k and stable multi-result selection, and an
`Option`-returning selector for a possibly empty `List` are deferred.
The right-fold description is not a machine-checked complexity result;
all asymptotic and resource claims remain deferred.
