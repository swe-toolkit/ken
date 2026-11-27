# A missing result below a larger bound persists below a smaller bound

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.9.

This seed targets `find_below_none_shrink`: from `None` below a larger
bound and an order premise, the law concludes `None` below a smaller
bound. The accepted client leaves the predicate open and cites the law.
It is not a closed search example: the existing §2.4 seed covers the
concrete `None` behavior. The authored seed has not been executed by the
official conformance runner. This seed does not claim package-test execution,
official conformance-runner execution, CI, publication, landing, or a landed-trust
measurement. Architect AC-0 separately reports a scratch D0 trusted-GlobalId
measurement (114 → 114).

## Accepted open application

The law's premises are `leq_nat three five = True` and an observed
`None` below five. Its conclusion is `None` below three. The predicate
`p` is arbitrary, so this application does not close by recomputing a
particular search result.

```ken
import Algorithm.Optimization.BoundedSearch
  (find_below, find_below_none_shrink)

const three : Nat = Suc (Suc (Suc Zero))

const five : Nat = Suc (Suc three)

theorem none_shrink_five_to_three (p : Nat → Bool)
    : Equal (Option Nat) (find_below p five) (None Nat)
      → Equal (Option Nat) (find_below p three) (None Nat) =
  find_below_none_shrink p three five Proved
```

### `stdlib/optimization/bounded-search-none-shrink-open`

- spec: `spec/50-stdlib/72-optimization.md` §2.9.
- promise class: durable invariant.
- given: an arbitrary predicate `p` and the premise that its search
  below five is `None`.
- expect: applying `find_below_none_shrink` with the true order premise
  yields `None` below three.
- measured: the AC-0 scratch client checks with the law application.
  The same result type with body `λmissing. Proved` or
  `λmissing. missing` refuses with `KernelRejected(TypeMismatch)`;
  body `λmissing. Refl` refuses because the two sides are not
  convertible.
- claimed: `None` below a larger bound entails `None` below any smaller
  bound it bounds.
- gap: this open client does not prove the generic law body or measure
  trusted-base identities.
- why: `p` remains open, and the accepted proof explicitly applies the
  public law rather than relying on a closed result.

## The converse does not hold

Start from a fresh copy of the accepted prefix above, then append the
following theorem. The attempted law application reverses the bounds:
it needs `five ≤ three`, which is false. Require refusal at this
converse theorem with `KernelRejected(TypeMismatch)`; do not count an
earlier import or declaration failure. No absolute span or diagnostic
wording is pinned.

```ken
-- Append to a fresh accepted prefix from the preceding block.
theorem none_grow_three_to_five (p : Nat → Bool)
    : Equal (Option Nat) (find_below p three) (None Nat)
      → Equal (Option Nat) (find_below p five) (None Nat) =
  find_below_none_shrink p five three Proved
```

### `stdlib/optimization/bounded-search-none-grow-refused`

- spec: `spec/50-stdlib/72-optimization.md` §2.4 and §2.9.
- promise class: durable invariant.
- given: the accepted open-law prefix, followed by the converse
  implication from `None` below three to `None` below five.
- expect: `none_grow_three_to_five` refuses at its own theorem with
  `KernelRejected(TypeMismatch)`.
- measured: the law application requires
  `Equal Bool (leq_nat five three) True`; that order proposition is
  false, while the accepted prefix and imports are valid.
- claimed: absence persists downward only when the order premise points
  from the smaller bound to the larger bound; the converse is not a law.
- gap: this refusal does not classify every predicate whose search
  results happen to be `None` at both bounds.
- why: the isolated change reverses the bound direction, making the
  law's order premise fail instead of relying on an unrelated refusal.

## Evidence boundary

Architect AC-0 `evt_3rzm0b1aw8yr0` checked the generic law and open seed
clients in scratch at base `2d4cdc1b4a89b068ec2f2e213f3d4bc1a8b91a3f`.
It reports no added `Axiom` or postulate, equal exact scratch trusted
`GlobalId` sets (114 → 114), and successful checks of the law and
clients. The three same-type computation-only proof bodies listed above
were measured in that scratch. These are Architect scratch findings,
not execution of this seed by the official conformance runner, assembled
AC-T, CI, publication, landing, or a measured landed trust result.
