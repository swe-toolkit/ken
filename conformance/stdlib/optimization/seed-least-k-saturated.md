# Least-k keeps the sorted list when its bound covers the input

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §4.6, reviewed Spec child
`a4693101a6acc791d44d9f34a59e125a059fbc25` (not in this seed's base).
Base: `8cf27bd2cb7a05a674fdf561e4b013193542cda5`.

The public `least_k_saturated` law requires the original input length to
be at most `k`. Since `least_k` takes from the stable cost sort, a bound
covering the input returns the entire sorted list, not the input order.
The fixture and costs are those of §4.3: `[A, B, C, D]` with costs
`2, 1, 2, 3`. The equal-cost candidates `A` and `C` retain their input
relative order.

Every case has promise class **durable invariant**.

## Open public applications

The open clients apply both public laws without reducing their arguments.
The Argmin law's only premise is the original-input length bound; its
conclusion is structural list equality.

```ken
import Algorithm.Optimization.Argmin (least_k, cost_le, least_k_saturated)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (take, sort)
import Data.Collections.List (length)

theorem least_k_saturated_generic
      (a : Type) (cost : a → Nat) (k : Nat) (xs : List a)
    : Equal Bool (leq_nat (length a xs) k) True
      → Equal (List a) (least_k a cost k xs) (sort a (cost_le a cost) xs) =
  least_k_saturated a cost k xs

theorem take_saturated_generic
      (a : Type) (n : Nat) (xs : List a)
    : Equal Bool (leq_nat (length a xs) n) True
      → Equal (List a) (take a n xs) xs =
  (proof saturated for take) a n xs
```

### `stdlib/optimization/least-k-saturated-open`

- spec: §4.6, `least_k_saturated` and `take::saturated`.
- promise class: durable invariant.
- given: arbitrary `a`, `cost`, `k`, and `xs`, with the public bound
  premise.
- expect: both generic law applications check with their arguments open.
- measured: the clients apply the public Argmin and Derived proofs at
  their stated types.
- claimed: clients can use the exact public law routes without relying
  only on closed reduction.
- gap: applying a public proof does not check its generic proof body.
- why: this pins the binder order, single premise and public `take`
  route independently of the closed fixture.

## Closed accepted fixture

The same fixed input and cost function are used at both bounds. `v_sort`
fixes the sorted control; `v_least_four` and `v_least_nine` fix the
selected results; `c_law_four` and `c_law_nine` apply the public law.

```ken
import Algorithm.Optimization.Argmin (least_k, cost_le, least_k_saturated)
import Data.Collections.Derived (take, sort)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const nine : Nat = Suc (Suc (Suc (Suc (Suc four))))

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

const sorted_input : List Candidate =
  Cons Candidate B (Cons Candidate A (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem v_sort : Equal (List Candidate) (sort Candidate (cost_le Candidate seed_cost) input) sorted_input =
  Refl

theorem v_least_four : Equal (List Candidate) (least_k Candidate seed_cost four input) sorted_input =
  Refl

theorem v_least_nine : Equal (List Candidate) (least_k Candidate seed_cost nine input) sorted_input =
  Refl

theorem c_law_four
    : Equal (List Candidate) (least_k Candidate seed_cost four input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost four input Proved

theorem c_law_nine
    : Equal (List Candidate) (least_k Candidate seed_cost nine input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost nine input Proved
```

### `stdlib/optimization/least-k-saturated-at-input-length`

- spec: §4.6, with `length input = 4` and `k = 4`.
- promise class: durable invariant.
- given: four candidates with costs `2, 1, 2, 3`.
- expect: `least_k` and `sort` both yield `[B, A, C, D]`; the bound
  premise is true.
- measured: `v_sort`, `v_least_four`, and `c_law_four` use `Refl` or
  the public law application at the closed fixture.
- claimed: the saturated result retains every item in sorted order
  when the bound equals the input length.
- gap: one closed instance does not establish the generic law.
- why: `A` precedes the equal-cost `C`, so the output distinguishes
  stable sorted order from the original input order.

### `stdlib/optimization/least-k-saturated-past-input-length`

- spec: §4.6, with `length input = 4` and `k = 9`.
- promise class: durable invariant.
- given: the same four candidates, costs and input as the `k = 4`
  case.
- expect: `least_k` and `sort` both yield `[B, A, C, D]`; the bound
  is true beyond the input length.
- measured: `v_least_nine` and `c_law_nine` use the fixed result and
  public law application.
- claimed: any bound past the length still returns the full sorted list.
- gap: this closed instance does not establish the generic theorem.
- why: comparing `k = 4` and `k = 9` changes only the bound while
  preserving the input and cost function.

## Refuse input order as the saturated result

Start from a fresh complete copy of the accepted closed client above
and append only `r_input_unchanged`. The accepted prefix must check
before the new false claim is considered.

```ken
import Algorithm.Optimization.Argmin (least_k, cost_le, least_k_saturated)
import Data.Collections.Derived (take, sort)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const nine : Nat = Suc (Suc (Suc (Suc (Suc four))))

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

const sorted_input : List Candidate =
  Cons Candidate B (Cons Candidate A (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem v_sort : Equal (List Candidate) (sort Candidate (cost_le Candidate seed_cost) input) sorted_input =
  Refl

theorem v_least_four : Equal (List Candidate) (least_k Candidate seed_cost four input) sorted_input =
  Refl

theorem v_least_nine : Equal (List Candidate) (least_k Candidate seed_cost nine input) sorted_input =
  Refl

theorem c_law_four
    : Equal (List Candidate) (least_k Candidate seed_cost four input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost four input Proved

theorem c_law_nine
    : Equal (List Candidate) (least_k Candidate seed_cost nine input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost nine input Proved

theorem r_input_unchanged : Equal (List Candidate) (least_k Candidate seed_cost four input) input =
  Refl
```

### `stdlib/optimization/reject-least-k-saturated-as-input-order`

- spec: §4.6, structural result of `least_k_saturated` at `k = 4`.
- promise class: durable invariant.
- given: the complete accepted closed client followed by
  `r_input_unchanged` on the same input and cost function.
- expect: the prefix accepts and the appended false equality refuses
  at its own `Refl`.
- measured: D0 reports `TypeMismatch` at the appended `Refl`; the
  accepted `v_least_four` row has the same input, bound and proof form.
- claimed: saturated `least_k` returns the sorted list, not the
  original input order.
- gap: this one fixture does not characterize every ordering or cost.
- why: `[B, A, C, D]` differs from `[A, B, C, D]` at the first
  element, while all other refusal dimensions are held fixed.

The `k = 3` bound is false for this four-element input. The existing
`conformance/stdlib/optimization/seed-least-k-prefix.md` case
`v_least_three` checks that `least_k` returns the partial
`[B, A, C]` result. This seed does not apply `least_k_saturated` at
that invalid bound.

## Companion package-test boundary

Architect's D0 did not measure exact `trusted_base()` identity sets.
Foundation's package test owns that gate and must compare exact
`GlobalId` sets before and after loading `Argmin` over its providers.
It also checks both law owners and transparency, the open-client owner
`Const`s, the accepted closed client, and a fresh-prefix refusal whose
span is inside `r_input_unchanged` and covers its `Refl`. This seed
makes no trust-set or package-test claim.

## Evidence boundary

Architect's AC-0 D0 on base `8cf27bd2cb7a05a674fdf561e4b013193542cda5`
reports that the generic laws, open clients, closed `k = 4` and `k = 9`
clients, and fresh-prefix refusal checked in scratch. The scratch binary's
source commit was not identified, and D0 did not measure exact trusted
`GlobalId` identity sets. Those are D0 reports, not execution of this
seed. The laws are absent from this seed's direct base; CV ran no Ken
check or formatter check, package test, or official conformance runner.
No CI, publication, landing, or measured landed-trust result is claimed.
