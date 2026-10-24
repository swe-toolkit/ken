# Greatest-cost candidate selection

Spec: `spec/50-stdlib/72-optimization.md` §3.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMAX.md`.

These are source-shaped conformance cases for the §3 earliest-tie rule.
The base is
`218387582c9cda217ac33f5513d8d1eecc3a8c82`; the §3 catalog export is
absent there, so this seed has not been executed against the package.
Architect D0 checked the same closed fixture in a separate scratch
client. That is not execution of this seed or an official conformance
run. No CI, publication, or landing is claimed.

## Closed earliest-tie case

`seed_cost` assigns nine at zero, one at one and two, and five at
three and above. `tie_max` is the ordered nonempty input
`[one, three, two, four]`; its costs are `1, 5, 1, 5`. Zero is not
in that input. Thus `three` is the first of the two greatest-cost
members. The same-shape refusal differs only by claiming `four`. A
last-tie replacement would return `four` and make that claim true.

```ken
import Algorithm.Optimization.Argmin (argmax)
import Data.Collections.NonEmpty (NonEmpty, nonempty_cons)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc (Suc (Suc (Suc (Suc Zero))))
const nine : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))))

fn seed_cost (n : Nat) : Nat =
  match n {
    Zero ↦ nine;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc k ↦
          match k {
            Zero ↦ one;
            Suc j ↦ five
          }
      }
  }

const tie_max : NonEmpty Nat =
  nonempty_cons Nat one
    (Cons Nat three (Cons Nat two (Cons Nat four (Nil Nat))))

theorem argmax_earliest_tie
    : Equal Nat (argmax Nat seed_cost tie_max) three =
  Proved
```

### stdlib/optimization/argmax-earliest-tie

- spec: §3.1–§3.2.
- promise class: durable invariant.
- given: `[one, three, two, four]` under `seed_cost`, with costs
  `1, 5, 1, 5`.
- expect: `argmax` returns `three`, the earlier of the two
  greatest-cost members.
- why: equal-cost candidates do not replace the earlier occurrence.

## Refuse the later tied maximum

Runner assembly: first build and accept a client containing the entire
positive Ken fence above. Copy that accepted client fresh and append
only the refusal fence below. Never check the refusal alone. The
positive `argmax_earliest_tie` is the same-shape control.

### stdlib/optimization/argmax-rejects-later-tie

- spec: §3.1–§3.2.
- promise class: durable invariant.
- given: the accepted positive client followed by a claim that
  `argmax` returns `four` on `tie_max`.
- expect: reject at this equality; no missing declaration or earlier
  positive failure is the expected result.
- why: the correct result is the earlier tied maximum, `three`; a
  last-tie selector would produce `four` and accept this claim.

```ken
theorem seed_neg
    : Equal Nat (argmax Nat seed_cost tie_max) four =
  Proved
```

## Coverage boundary

This seed discriminates the earliest-tie rule on one closed input. It
does not replace the seven generic law proofs or establish behavior on
all candidate types and inputs. The source-shaped cases have not run
against §3 package exports; no official conformance execution, CI,
publication, or landing is claimed.
