# Least-k selection under an order-preserving cost transform

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §4.4.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-LEAST-K-COST-ORDER-INVARIANCE.md`.
Spec child: `19e26b24cd23b0baa1e7b72a7005fef7dc376bd7`.

The input is `[A, B, C, D]` with costs `2, 1, 2, 3`. Adding three
to every cost preserves and reflects each strict cost comparison, so
`least_k` at `k = 2` still selects `[B, A]`. Flattening every cost to
zero makes all four candidates tied; stable selection then returns
`[A, B]`, not `[B, A]`.

## Accepted client

The public-law application and its closed value are separate rows: the
first applies the generic law, while the second independently checks
that the shifted selector returns `[B, A]`. The accepted constant-cost
value pins the stable result used by the refusal.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_cost_order_invariant)

import Core.Function.Combinators (comp)

import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

fn shift (c : Nat) : Nat = add c three

fn flatten (c : Nat) : Nat = Zero

const seed_input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem cost_a_is_two : Equal Nat (seed_cost A) two = Proved

theorem cost_b_is_one : Equal Nat (seed_cost B) one = Proved

theorem cost_c_is_two : Equal Nat (seed_cost C) two = Proved

theorem cost_d_is_three : Equal Nat (seed_cost D) three = Proved

theorem shift_law
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat shift seed_cost) two seed_input)
        (least_k Candidate seed_cost two seed_input) =
  least_k_cost_order_invariant Candidate seed_cost shift (λi. λj. Refl) two seed_input

theorem shift_value
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat shift seed_cost) two seed_input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem flatten_value
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat flatten seed_cost) two seed_input)
        (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl
```

### `stdlib/optimization/least-k-cost-order-invariance-shift`

- spec: §4.3 finite input and §4.4 public law.
- promise class: durable invariant.
- given: labels `A`, `B`, `C`, and `D` with costs `2`, `1`, `2`, and
  `3`; `k = 2`; and `shift c = add c three`.
- expect: the public law equates the transformed and original selections;
  both independently checked values are `[B, A]`.
- measured: the source applies `least_k_cost_order_invariant` with
  `same_order` discharged by the closed shift proof, checks the four
  fixture costs, and checks both selection values.
- claimed: this closed instance of a strict-order-preserving transform
  leaves the selected prefix unchanged.
- gap: this one input and transform do not prove the generic law for all
  candidate types, costs, counts, lists, and transforms.
- why: `B` has the unique least cost; `A` is the earliest of the two
  candidates tied at cost `2`.

## Refuse a false constant-transform result

Use a fresh copy of the complete accepted client and append only the
false claim below. The accepted `flatten_value` control checks the actual
constant-cost result on the same input and by the same `Refl` form. The
refusal must occur at the appended theorem, not at an import, fixture, or
accepted control.

### `stdlib/optimization/reject-constant-least-k-cost-order-invariance`

- spec: §4.3 stable selection and §4.4 premise boundary.
- promise class: durable invariant.
- given: the accepted fixture and `flatten c = Zero`.
- expect: reject the claim that the constant transform selects `[B, A]`.
- measured: the fresh input retains all accepted controls before the
  false selection equality.
- claimed: a transform that merges distinct costs into one tie need not
  preserve the selected prefix.
- gap: this constant transform is one counterexample, not a
  characterization of every transform that fails `same_order`.
- why: stable selection under the constant cost returns `[A, B]`, as the
  accepted `flatten_value` control states.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_cost_order_invariant)

import Core.Function.Combinators (comp)

import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

fn shift (c : Nat) : Nat = add c three

fn flatten (c : Nat) : Nat = Zero

const seed_input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem cost_a_is_two : Equal Nat (seed_cost A) two = Proved

theorem cost_b_is_one : Equal Nat (seed_cost B) one = Proved

theorem cost_c_is_two : Equal Nat (seed_cost C) two = Proved

theorem cost_d_is_three : Equal Nat (seed_cost D) three = Proved

theorem shift_law
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat shift seed_cost) two seed_input)
        (least_k Candidate seed_cost two seed_input) =
  least_k_cost_order_invariant Candidate seed_cost shift (λi. λj. Refl) two seed_input

theorem shift_value
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat shift seed_cost) two seed_input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem flatten_value
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat flatten seed_cost) two seed_input)
        (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl

theorem constant_transform_claims_shifted_result
    : Equal
        (List Candidate)
        (least_k Candidate (comp Candidate Nat Nat flatten seed_cost) two seed_input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl
```

## Direct clients of the public comparator-congruence proofs

This client imports the operation subjects and applies each public attached
proof through its canonical `subject::proof` path. The comparator bodies differ:
one always returns `False`, and the other returns `False` by matching on its
first argument. `same_candidate_le` proves their pointwise equality by splitting
that argument. The client leaves `x` and `xs` neutral so neither result equality
collapses before the public proof application.

```ken
import Data.Collections.Derived (insert, sort)

data Candidate : Type where {
  A : Candidate;
  B : Candidate
}

fn candidate_le_false (x : Candidate) (y : Candidate) : Bool = False

fn candidate_le_case_false (x : Candidate) (y : Candidate) : Bool =
  match x {
    A ↦ False;
    B ↦ False
  }

theorem same_candidate_le
    : (x : Candidate)
      → (y : Candidate)
      → Equal Bool (candidate_le_false x y) (candidate_le_case_false x y) =
  λx.
    match x {
      A ↦ λy. Proved;
      B ↦ λy. Proved
    }

theorem insert_comparator_cong_client
      (x : Candidate) (xs : List Candidate)
    : Equal
        (List Candidate)
        (insert Candidate candidate_le_false x xs)
        (insert Candidate candidate_le_case_false x xs) =
  insert::comparator_cong
    Candidate
    candidate_le_false
    candidate_le_case_false
    same_candidate_le
    x
    xs

theorem sort_comparator_cong_client
      (xs : List Candidate)
    : Equal
        (List Candidate)
        (sort Candidate candidate_le_false xs)
        (sort Candidate candidate_le_case_false xs) =
  sort::comparator_cong
    Candidate
    candidate_le_false
    candidate_le_case_false
    same_candidate_le
    xs
```

### `stdlib/optimization/public-insert-comparator-congruence`

- spec: §4.3 public attached proof signature.
- promise class: normative compatibility vector.
- given: neutral `x` and `xs`, with two pointwise-equal comparator bodies.
- expect: the client application of `insert::comparator_cong` is accepted.
- measured: the source imports `insert` and applies its attached proof with
  `same_candidate_le`.
- claimed: the public proof relates insertion under pointwise-equal
  comparators.
- gap: this checks one carrier and a valid comparator pair, not every carrier,
  comparator or list.
- why: `same_candidate_le` proves the premise by case analysis; the result
  remains open over client-supplied `x` and `xs`.

### `stdlib/optimization/public-sort-comparator-congruence`

- spec: §4.3 public attached proof signature.
- promise class: normative compatibility vector.
- given: neutral `xs`, with the same pointwise-equal comparator pair.
- expect: the client application of `sort::comparator_cong` is accepted.
- measured: the source imports `sort` and applies its attached proof with
  `same_candidate_le`.
- claimed: the public proof relates sorting under pointwise-equal
  comparators.
- gap: this checks one carrier and a valid comparator pair, not every carrier,
  comparator or list.
- why: `xs` remains neutral, so the result equality does not compute to a
  closed reflexive case before the public proof is used.

## Evidence boundary

A local `ken` binary was built with
`scripts/ken-cargo build -p ken-cli --bin ken` from base
`273981a28868648994e656873dac25180a524d0b`. The scratch catalog began with
that base's `catalog/packages` and overlaid the exact candidate
`a3305f0aa5fbc6443346d5e7b977930e11d18d1d` blobs for `Argmin.ken.md`
(`ff8f0cce69d7f3c556f3f46ba7e86ffc98a9610a`) and `Derived.ken.md`
(`94ae51ef9296e935559290eb7e3f70dde1daefef`). The three fences were
extracted under that overlay. `ken fmt --check` accepted all three;
`ken check` returned 0 for the least-k accepted client and the two public
comparator applications, and returned 1 for the least-k false claim with
E0038 TypeMismatch at its final `Refl`, span 1868..1872.

In separate scratch mutations, changing each `pub proof comparator_cong`
declaration to private caused `ken check` to return 1 with E0012 UnboundName
for the corresponding attached path: `insert::comparator_cong` at span
761..784 and `sort::comparator_cong` at span 1090..1111. Each mutation
changed one declaration (anchor count 1 to 0), and each was restored to exact
candidate `Derived.ken.md` blob `94ae51ef9296e935559290eb7e3f70dde1daefef`;
the client returned rc 0 after each restore. These are scratch visibility
mutations, not added conformance refusal cases. No separate false-premise
comparator control was added; the least-k false claim remains the refusal
case. This is scratch validation against an exact-candidate catalog overlay,
not official conformance execution or an assembled-tip run. No CI,
publication, landing or trust-measurement claim is made.

## Coverage boundary

The seed covers the shifted-cost law application, independent shifted and
constant-cost values, the two public comparator-congruence applications, and
one same-form false constant-transform claim. The comparator clients use one
carrier and one valid pointwise-equal comparator pair; they do not establish
behavior for every carrier, comparator or list. The least-k closed cases do
not prove the generic law for every transform and input.
