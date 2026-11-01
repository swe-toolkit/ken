# A refuted finite range returns no witness

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.5.

This authored behavioral oracle targets the public checked converse
`find_below_refuted`. The finite predicate is true exactly above two. The
accepted bound-three control and law client establish that the premise is
usable; the same-form bound-ten claim is refused after a fresh copy of the
entire accepted client. The seed states no search strategy, predicate-call
count, or performance guarantee. Every case has promise class **durable
invariant**.

The expected finite results follow from §2.1: below three, the range is
`0, 1, 2`, none of which satisfies `2 < n`; below ten, `3` is the least
satisfying index. The Architect's AC-2 scratch measurement reports the
bound-three `Proved` control accepted and the isolated bound-ten `Proved`
claim refused with `KernelRejected TypeMismatch`. This seed has not been
Ken-checked or officially executed.

## Accepted client and controls

The accepted client uses the same `two`, `three`, `five`, `ten`, and
`above_two` fixtures as the bounded-search bound-stability seed. Its
refutation proof covers each of `0`, `1`, and `2`; for any larger `m`, the
bound premise is contradictory. The law client applies the public
`find_below_refuted` theorem to that proof.

```ken
import Algorithm.Optimization.BoundedSearch (find_below, find_below_refuted)
import Data.Numeric.Nat.Order (lt_nat)

const two : Nat = Suc (Suc Zero)
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const ten : Nat = Suc (Suc (Suc (Suc (Suc five))))

fn above_two (n : Nat) : Bool = lt_nat two n

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

theorem none_below_three_by_proved
    : Equal (Option Nat) (find_below above_two three) (None Nat) =
  Proved

theorem none_below_three_by_law
    : Equal (Option Nat) (find_below above_two three) (None Nat) =
  find_below_refuted above_two three above_two_refuted_below_three

theorem found_below_ten_by_proved
    : Equal (Option Nat) (find_below above_two ten) (Some Nat three) =
  Proved

theorem five_is_a_later_hit
    : Equal Bool (above_two five) True =
  Proved
```

### `stdlib/optimization/bounded-search-refuted-range-control`

- spec: §2.1 and §2.5.
- promise class: durable invariant.
- given: `above_two` below bound `3`.
- expect: the result is `None`; `none_below_three_by_proved` accepts.
- measured: the proposition is accepted with `Proved`, and the range
  contains only `0`, `1`, and `2`.
- claimed: the finite range has no satisfying index.
- gap: this closed instance does not establish the generic converse.
- why: this is the positive control for the same-form bound-ten refusal.

### `stdlib/optimization/bounded-search-refuted-range-law-client`

- spec: §2.5.
- promise class: durable invariant.
- given: the refutation of every `m < 3` for `above_two`.
- expect: `none_below_three_by_law` accepts by applying
  `find_below_refuted`.
- measured: the fixture supplies the public law's full refuted-range
  premise, and the law returns the expected `None` equation.
- claimed: a client can use the public converse with its stated premise.
- gap: this single closed client does not prove the generic law.
- why: it exercises the law rather than only recomputing the result.

### `stdlib/optimization/bounded-search-refuted-range-some-control`

- spec: §2.1 and §2.5.
- promise class: durable invariant.
- given: `above_two` below bound `10`.
- expect: `Some 3` is accepted by `found_below_ten_by_proved`; the later
  point `5` also satisfies the predicate.
- measured: both controls concern the same predicate and bound used by
  the refused `None` claim.
- claimed: the refused conclusion is false because the search has a
  satisfying in-range witness.
- gap: these closed observations are controls, not a proof of a
  universal search property.
- why: the control exhibits the actual result on the negative case's
  input and identifies the false conclusion.

## Refused claim

For the refusal, use a fresh copy of the entire accepted client above and
append only the declaration below. Refuse it at that appended theorem,
with `KernelRejected TypeMismatch`; do not count a refusal in an import or
earlier declaration as a pass.

### `stdlib/optimization/bounded-search-refuted-range-false-none`

- spec: §2.1 and §2.5.
- promise class: durable invariant.
- given: the same predicate and bound-ten input as
  `found_below_ten_by_proved`.
- expect: refuse the claim that the result is `None`.
- measured: the identical input has an accepted `Some 3` control, and
  the false claim is isolated in its own appended declaration.
- claimed: `find_below_refuted` is not an invalid converse that accepts
  a false `None` claim when an in-range hit exists.
- gap: this finite refusal does not prove the universal theorem or
  characterize every possible predicate.
- why: `3 < 10` and `above_two 3 = True`; the accepted control confirms
  the actual result is `Some 3`, so the negative targets the false
  conclusion rather than a malformed fixture.

```ken
-- Refused: the in-range witness three makes this None claim false.
theorem none_below_ten_by_proved
    : Equal (Option Nat) (find_below above_two ten) (None Nat) =
  Proved
```

## Coverage boundary

The seed covers the public converse's finite premise, an accepted law
client, the true `None` control at bound three, and the false `None`
claim at bound ten. The accepted bound-ten result and later-hit control
make the refusal's cause explicit. These closed cases do not prove the
generic law, specify a search strategy, or claim official
conformance-runner execution, CI, publication, or landing.
