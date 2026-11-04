# The least in-range hit is the search result

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.6.

This seed targets the public `find_below_least_hit` law. Its accepted
client applies the law to `above_two` at bound ten and witness three.
The false `Some four` claim uses the same predicate and bound; a
same-form `Some four` control for `above_three` isolates the earlier-hit
reason for refusal. These closed cases do not prove the generic law.
The seed has not been run by the official conformance runner. No CI,
publication, landing, or trust measurement is claimed.

## Accepted client and controls

`above_two n` holds exactly when `2 < n`, so three is its least hit.
The explicit prefix proof refutes each smaller natural. The law client
uses the exact premise order from the public type: bound, hit, then
refuted prefix. For the same bound, `above_three n` holds exactly when
`3 < n`, so its first hit is four.

```ken
import Algorithm.Optimization.BoundedSearch (find_below, find_below_least_hit)

import Data.Numeric.Nat.Order (lt_nat)

const two : Nat = Suc (Suc Zero)

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const ten : Nat = Suc (Suc (Suc (Suc (Suc five))))

fn above_two (n : Nat) : Bool = lt_nat two n

fn above_three (n : Nat) : Bool = lt_nat three n

theorem above_two_refuted_below_three
    : (m : Nat) → Equal Bool (lt_nat m three) True → Equal Bool (above_two m) False =
  λm.
    match m {
      Zero ↦ λwithin. Proved;
      Suc m1 ↦
        match m1 {
          Zero ↦ λwithin. Proved;
          Suc m2 ↦
            match m2 {
              Zero ↦ λwithin. Proved;
              Suc m3 ↦ λwithin. absurd within
            }
        }
    }

theorem least_hit_by_law : Equal (Option Nat) (find_below above_two ten) (Some Nat three) =
  find_below_least_hit above_two ten three Proved Proved above_two_refuted_below_three

theorem least_hit_by_proved : Equal (Option Nat) (find_below above_two ten) (Some Nat three) =
  Proved

theorem above_three_hits_at_four
    : Equal (Option Nat) (find_below above_three ten) (Some Nat four) =
  Proved
```

### `stdlib/optimization/bounded-search-least-hit-law-client`

- spec: §2.6.
- promise class: durable invariant.
- given: `above_two`, bound ten, witness three, and a proof that every
  smaller index is refuted.
- expect: the public law accepts with result `Some three`.
- measured: the client supplies all three premises in their public
  order; the direct `Proved` control independently checks the result.
- claimed: an in-range hit with a refuted prefix is the least search
  result.
- gap: this closed instance does not establish the generic law.
- why: the client consumes the public theorem rather than recomputing
  the result as its proof.

### `stdlib/optimization/bounded-search-least-hit-same-form-control`

- spec: §2.1 and §2.6.
- promise class: durable invariant.
- given: `above_three` at bound ten and the claim that the result is
  `Some four`, proved by `Proved`.
- expect: `above_three_hits_at_four` accepts.
- measured: this is the exact result and proof form used by the refused
  claim, with the predicate threshold changed from two to three.
- claimed: the `Some four` proposition and proof form are accepted for
  a predicate whose first hit is four.
- gap: the closed instance does not establish the generic law.
- why: it controls for a malformed proposition or proof form as the
  reason for refusing the `above_two` claim.

### `stdlib/optimization/bounded-search-least-hit-exact-result-control`

- spec: §2.1 and §2.6.
- promise class: durable invariant.
- given: `above_two` at bound ten and the claim that the result is
  `Some three`.
- expect: `least_hit_by_proved` accepts.
- measured: the direct computation agrees with the public-law client
  on the same predicate and bound.
- claimed: the result asserted by the law client is the finite result.
- gap: this finite result is not a proof of the universal theorem.
- why: it supplies an independent exact-result control for the input
  used by the refused claim.

## Refused claim

For the refusal, use a fresh copy of the entire accepted client above
and append only the declaration below. Require refusal at this appended
theorem with `KernelRejected TypeMismatch`; do not count a refusal in
an import or earlier declaration as a pass. The accepted same-form
control establishes that `Some four` by `Proved` is not itself the
problem. The earlier hit at three makes that result false for
`above_two`.

### `stdlib/optimization/bounded-search-least-hit-later-result-refusal`

- spec: §2.1 and §2.6.
- promise class: durable invariant.
- given: the same `above_two`, bound ten, and result form as the
  accepted `Some three` controls.
- expect: refuse the claim that the result is `Some four`.
- measured: the accepted control gives `Some three` on this same input;
  the same-form control accepts `Some four` for `above_three`.
- claimed: the public law does not accept a later hit as the least
  result when an earlier hit exists.
- gap: this finite refusal does not prove the generic law.
- why: three is the first hit; a search returning four here would
  accept this claim. The same-input `Some three` control detects that
  wrong result, and the `above_three` control isolates the proof form.

```ken
-- Refused: three is an earlier hit for this predicate.
theorem reject_later_hit_by_proved
    : Equal (Option Nat) (find_below above_two ten) (Some Nat four) =
  Proved
```

## Coverage boundary

The seed covers one closed application of the public law, its explicit
refuted-prefix premise, a direct exact-result control, a same-form
accepted control, and an isolated false later-result claim. The finite
instances do not prove the universally quantified law or specify a
search strategy, predicate-call count, or performance guarantee. No
package test or official conformance execution is claimed.
