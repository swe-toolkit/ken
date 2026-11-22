# Least-k selection after merging selected parts

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §4.7, Spec child
`d92453faef8c49e9d4a8010e5074e77667ad4469`.
Architect AC-0: `evt_4x3a0wnmhs12m`.

`least_k_append_merge` compares selecting from an append with re-selecting
from the concatenated `k`-prefixes of its parts. The final `least_k` on the
right side is required: the two selected part results are not already a
selected prefix of their append. The public Argmin law has no premise. Its
supporting `sort::take_append` law is generic and requires totality and
transitivity of its comparator.

## Open public clients

Apply both public laws with their arguments left open. The first application
pins the Argmin binder order and its premise-free type. The second pins the
generic sort law's comparator premises and binder order.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_append_merge)
import Data.Collections.Derived (list_append)

theorem least_k_append_merge_generic
      (a : Type) (cost : a → Nat) (k : Nat)
      (xs : List a) (ys : List a)
    : Equal
        (List a)
        (least_k a cost k (list_append a xs ys))
        (least_k a cost k
          (list_append a
            (least_k a cost k xs)
            (least_k a cost k ys))) =
  least_k_append_merge a cost k xs ys
```

```ken
import Core.Classes.LawfulClasses (IsTrue, bool_or)
import Data.Collections.Derived (list_append, sort, take)

theorem sort_take_append_generic
      (a : Type) (le : a → a → Bool)
      (total : (x : a) → (y : a) →
        IsTrue (bool_or (le x y) (le y x)))
      (transitive : (x y z : a) →
        Equal Bool (le x y) True →
        Equal Bool (le y z) True →
        Equal Bool (le x z) True)
      (k : Nat) (xs : List a) (ys : List a)
    : Equal (List a)
        (take a k (sort a le (list_append a xs ys)))
        (take a k
          (sort a le
            (list_append a
              (take a k (sort a le xs))
              (take a k (sort a le ys))))) =
  sort::take_append a le total transitive k xs ys
```

### stdlib/optimization/least-k-append-merge-open

- spec: §4.7, both public theorem types
- promise class: normative compatibility vector
- given: arbitrary carrier, natural cost function, bound, and two lists;
  the `sort::take_append` client also receives its stated total and
  transitivity proofs
- expect: both clients accept with the listed binder order. The Argmin
  application needs no additional premise; the Derived application has exactly
  the two comparator premises
- measured: the open applications leave the carrier, cost/comparator, bound,
  and input lists neutral
- claimed: both public laws are usable at open arguments, with no extra
  hypothesis on the Argmin law
- gap: these applications check public availability and type, not the proof
  bodies for all inputs
- why: closing the inputs could make either result equality hold by
  computation without checking the corresponding general law

## Accepted closed clients

The main fixture has costs `A ↦ 2`, `B ↦ 1`, `C ↦ 1`, and `D ↦ 0`, with
`xs = [A, B]`, `ys = [C, D]`, and `k = 2`. The part selections are `[B, A]`
and `[D, C]`. Both sides of the law independently reduce to `[D, B]`; the tie
between `B` and `C` retains the earlier occurrence from `xs`.

The second fixture has constant cost, `k = 1`, `xs = [A]`, and `ys = [C]`.
The combined selection is `[A]`, preserving the earlier occurrence.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_append_merge)
import Data.Collections.Derived (list_append, take)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ one;
    D ↦ Zero
  }

fn tie_cost (x : Candidate) : Nat = one

const left : List Candidate =
  Cons Candidate A (Cons Candidate B (Nil Candidate))
const right : List Candidate =
  Cons Candidate C (Cons Candidate D (Nil Candidate))
const whole : List Candidate = list_append Candidate left right

const tie_left : List Candidate = Cons Candidate A (Nil Candidate)
const tie_right : List Candidate = Cons Candidate C (Nil Candidate)
const tie_whole : List Candidate =
  list_append Candidate tie_left tie_right

theorem cost_a : Equal Nat (seed_cost A) two = Refl
theorem cost_b : Equal Nat (seed_cost B) one = Refl
theorem cost_c : Equal Nat (seed_cost C) one = Refl
theorem cost_d : Equal Nat (seed_cost D) Zero = Refl

theorem left_selects_ba
    : Equal (List Candidate)
        (least_k Candidate seed_cost two left)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem right_selects_dc
    : Equal (List Candidate)
        (least_k Candidate seed_cost two right)
        (Cons Candidate D (Cons Candidate C (Nil Candidate))) =
  Refl

theorem whole_selects_db
    : Equal (List Candidate)
        (least_k Candidate seed_cost two whole)
        (Cons Candidate D (Cons Candidate B (Nil Candidate))) =
  Refl

theorem reselected_parts_select_db
    : Equal (List Candidate)
        (least_k Candidate seed_cost two
          (list_append Candidate
            (least_k Candidate seed_cost two left)
            (least_k Candidate seed_cost two right)))
        (Cons Candidate D (Cons Candidate B (Nil Candidate))) =
  Refl

theorem append_merge_law
    : Equal (List Candidate)
        (least_k Candidate seed_cost two whole)
        (least_k Candidate seed_cost two
          (list_append Candidate
            (least_k Candidate seed_cost two left)
            (least_k Candidate seed_cost two right))) =
  least_k_append_merge Candidate seed_cost two left right

theorem tie_left_selects_a
    : Equal (List Candidate)
        (least_k Candidate tie_cost one tie_left)
        (Cons Candidate A (Nil Candidate)) =
  Refl

theorem tie_right_selects_c
    : Equal (List Candidate)
        (least_k Candidate tie_cost one tie_right)
        (Cons Candidate C (Nil Candidate)) =
  Refl

theorem tie_whole_selects_a
    : Equal (List Candidate)
        (least_k Candidate tie_cost one tie_whole)
        (Cons Candidate A (Nil Candidate)) =
  Refl

theorem tie_reselected_parts_select_a
    : Equal (List Candidate)
        (least_k Candidate tie_cost one
          (list_append Candidate
            (least_k Candidate tie_cost one tie_left)
            (least_k Candidate tie_cost one tie_right)))
        (Cons Candidate A (Nil Candidate)) =
  Refl

theorem tie_append_merge_law
    : Equal (List Candidate)
        (least_k Candidate tie_cost one tie_whole)
        (least_k Candidate tie_cost one
          (list_append Candidate
            (least_k Candidate tie_cost one tie_left)
            (least_k Candidate tie_cost one tie_right))) =
  least_k_append_merge Candidate tie_cost one tie_left tie_right
```

### stdlib/optimization/least-k-append-merge-values

- spec: §4.7, stable selection after appending the selected parts
- promise class: normative compatibility vector
- given: four distinct candidates with costs `2, 1, 1, 0`, the split
  `[A, B] ++ [C, D]`, and `k = 2`; separately, constant cost, the split
  `[A] ++ [C]`, and `k = 1`
- expect: the whole-input and re-selected-parts results are both `[D, B]`;
  the tied singleton split selects `[A]` on both law sides
- measured: the fixture pins both part results, both `[D, B]` sides, four
  cost values, both `[A]` tie sides, and applications of the public law
- claimed: re-selection after appending the part prefixes preserves the
  whole-input result and its stable tie order
- gap: these finite values do not establish the law for arbitrary carriers,
  costs, bounds, or lists
- controls: the separate part selections expose the left/right order and
  the tied `A`/`C` selections before the combined result
- why: costs are ordered `D < B = C < A`; the stable sort retains `B` before
  `C` at the `k = 2` cut. At constant cost, the original `A` precedes `C`

## Refuse omission of the final re-selection

Use a fresh copy of the complete accepted closed client above, including its
part, whole-input, re-selected-parts, and tie controls. Append only the false
claim below. Its right side takes `k` directly from the raw append of the two
part results, which is `[B, A]`. The public law instead re-applies `least_k`
to that append, yielding `[D, B]`.

```ken
theorem no_reselection_claim
    : Equal (List Candidate)
        (least_k Candidate seed_cost two whole)
        (take Candidate two
          (list_append Candidate
            (least_k Candidate seed_cost two left)
            (least_k Candidate seed_cost two right))) =
  least_k_append_merge Candidate seed_cost two left right
```

### stdlib/optimization/reject-least-k-append-merge-without-reselection

- spec: §4.7, the right side must re-select from the appended part results
- promise class: durable invariant
- given: a fresh accepted prefix from `least-k-append-merge-values`, followed
  only by `no_reselection_claim`
- expect: the accepted prefix succeeds; the false theorem is refused with
  `KernelRejected(TypeMismatch)` at theorem-declaration granularity. Do not
  pin a finer diagnostic span
- measured: the accepted prefix fixes the law's whole-input and re-selected
  sides as `[D, B]`, and the raw `take` side as `[B, A]`
- claimed: the concatenated part prefixes are not already the combined
  least-`k` result
- gap: this one input does not characterize every invalid replacement for
  the final re-selection
- controls: `append_merge_law`, `whole_selects_db`,
  `reselected_parts_select_db`, and the raw-append value are accepted before
  the false theorem
- why: the law application has the correct whole-input equality but its
  right endpoint differs from the claim's raw `take` endpoint

## Evidence boundary

Architect AC-0 reports the closed values and law applications measured on a
scratch candidate at base `d7d1e430efa4a5ce53d51df9370d8a60fb4c384f`. This
seed has not been executed. Its expected behavior comes from the exact
§4.7 contract and those scratch client observations. No official conformance
execution, package test, CI, publication, landing, or measured landed trust
is claimed.
