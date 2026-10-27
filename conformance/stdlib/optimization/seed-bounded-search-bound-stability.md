# A found witness persists when a finite bound grows

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.4.

This seed targets the public `find_below_stable` interface. Architect
checked the general proof in isolated D0; the catalog candidate now
roots-loads the checked law and exercises these clients in a package
test. The closed examples do not prove its quantifiers. There is one
refused proposition, appended to a fresh full positive prefix. Package
tests are not official conformance-runner execution or a landing claim.

## Accepted client

`above_two n` holds exactly when `2 < n`. Its first hit is `3`.
The search below bound `5` finds that witness, and the public
bound-stability law transports the same result to bound `10` using
the closed `leq_nat five ten = True` premise. Bound `3`, by contrast,
excludes the first hit and returns `None`. The final accepted theorem
has the **same premise and `λmissing. Proved` proof form** as the
negative theorem, but its conclusion is the true `Some 3` result at
bound `10`.

```ken
import Algorithm.Optimization.BoundedSearch (find_below, find_below_stable)

import Data.Numeric.Nat.Order (lt_nat)

const two : Nat = Suc (Suc Zero)
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const ten : Nat = Suc (Suc (Suc (Suc (Suc five))))

fn above_two (n : Nat) : Bool = lt_nat two n

theorem found_below_five
    : Equal (Option Nat) (find_below above_two five) (Some Nat three) =
  Proved

theorem stable_five_to_ten
    : Equal (Option Nat) (find_below above_two ten) (Some Nat three) =
  find_below_stable above_two five ten three Proved found_below_five

theorem no_witness_below_three
    : Equal (Option Nat) (find_below above_two three) (None Nat) =
  Proved

theorem none_then_found_three_to_ten
    : Equal (Option Nat) (find_below above_two three) (None Nat)
      → Equal (Option Nat) (find_below above_two ten) (Some Nat three) =
  λmissing. Proved
```

### `stdlib/optimization/bounded-search-found-stable`

- spec: `spec/50-stdlib/72-optimization.md` §2.4.
- promise class: durable invariant.
- given: the closed predicate `above_two`, observed `Some 3` below
  bound `5`, and a non-decreasing bound `10`.
- expect: the public law gives the **same** `Some 3` below `10`.
- measured: one closed use of `find_below_stable`, with independent
  exact-value controls `found_below_five` and
  `no_witness_below_three`.
- claimed: the public law is generic over `p`, `bound`, `larger` and
  `n`; the seed checks only its displayed closed instance.
- gap: the closed instance does not prove generic stability or a
  trust-base delta and does not specify search strategy or cost.
- why: the law consumes the earlier `Some` equation and the true
  order premise rather than merely recomputing the result at `10`.

### `stdlib/optimization/bounded-search-none-to-some-control`

- spec: `spec/50-stdlib/72-optimization.md` §2.4.
- promise class: durable invariant.
- given: the same `above_two`, a true `None` result below `3`, and
  the larger bound `10`.
- expect: the premise is accepted independently, and the implication
  with the true `Some 3` conclusion accepts with `λmissing. Proved`.
- measured: `no_witness_below_three` accepts with the **exact**
  negative case's antecedent; `none_then_found_three_to_ten` accepts
  with its proof form and the positive conclusion.
- claimed: growing the range can introduce a first satisfying index.
- gap: this instance does not classify all missing-result cases.
- why: the two controls show that rejection below must concern the
  false `None` conclusion, not a bad premise or invalid proof form.

## Refused None-stability claim

Runner assembly: take a **fresh copy of the entire accepted client**,
including both same-premise controls, then append only this theorem.
Reject it at the new theorem, not at an import or earlier fixture.
The conclusion `None` below `10` is false because `3` is a hit there.
Nothing in §2.4 promises that a missing result persists when the range
grows.

### `stdlib/optimization/reject-bounded-search-none-stability`

- spec: `spec/50-stdlib/72-optimization.md` §2.4.
- promise class: durable invariant.
- given: a true `None` result below `3`, followed by a claim that
  the result is still `None` below `10` for the same predicate.
- expect: refusal inside `seed_neg` at the false conclusion; the
  accepted prefix remains accepted unchanged.
- measured: the independently accepted premise and accepted
  same-`λmissing. Proved` implication with true `Some 3` conclusion
  isolate the changed conclusion.
- claimed: the result-stability theorem is **Some-only**, not a
  monotone-`None` law.
- gap: this single refused instance is not a generic theorem about
  every `None` result or all larger bounds.
- why: an invalid None-direction extension would accept this false
  claim on precisely the input whose positive control finds `3`.

```ken
theorem seed_neg
    : Equal (Option Nat) (find_below above_two three) (None Nat)
      → Equal (Option Nat) (find_below above_two ten) (None Nat) =
  λmissing. Proved
```

## Evidence boundary

Architect's isolated D0 on base `0bb872591` checked the generic
`find_below_stable` proof: package `ken check` and `ken fmt --check`
rc 0, two probe tests passed, no `Axiom` added, and equal scratch
`trusted_base()` GlobalId sets 114 → 114. In a separately assembled
scratch client after root-loading the D0 package, the complete
positive block accepted; the one false claim appended to it was
refused with `KernelRejected TypeMismatch`. These were Architect
scratch measurements, not checks against shipped §2.4 exports.

The author also checked these exact fences in an isolated worktree at
`0bb872591` with only the catalog path of Architect's D0 patch applied
(patch SHA-256
`695150c5f9b5b1a8e45df048fc04f1e273ea866ceceba3f98cac6e5f72e776ca`).
The locally built scratch CLI at this check had SHA-256
`645e6ddf67dbb45697ad1ba49db8b975a745d94520d2d1c1c158b7edd673b256`.
The accepted client returned rc 0. A fresh entire-positive-prefix
client plus only `seed_neg` returned rc 1/K0004 `KernelRejected`
`TypeMismatch`, span 938–1108 wholly inside the appended theorem
starting at byte 938. Logs:
`/tmp/spec-author-bound-stability-{Positive,NoneRefusal}.log`.
These are historical D0-overlay checks, not tests of the catalog
candidate. The candidate's package test roots-loads the checked §2.4
law and accepts the full positive client, including the observed
`Some three` results, the true `None` premise and the same-premise,
same-proof-form `Some three` control. It checks that the refused
proposition claims `None` at ten for that same predicate and premise,
then observes `KernelRejected TypeMismatch` within its own theorem.
A separate arbitrary-argument client consumes the full public type;
exact `trusted_base()` GlobalId sets agree before and after loading
(114 → 114), and the private helper refuses selective import. This
is local package-test evidence, **not** official conformance-runner
execution, CI, publication, landing or landed trust.
