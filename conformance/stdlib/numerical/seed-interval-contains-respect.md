# Interval membership respects rational value equality

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §12.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-CONTAINS-RESPECT.md`.

Every case promises a durable invariant. `MkRational n k` denotes
`n/(Suc k)`. The accepted client establishes membership of `1/2` in
`[1/2,1]`, transports it to the equivalent spelling `2/4` with the public
respect law, and checks that membership directly. The two spellings are
value-equivalent under `rational_equiv`, not structurally equal. `measured`
describes the intended oracle, not a report that this seed was executed.

Each refused claim is appended alone to a fresh complete copy of the accepted
client. One claims that `2/6` is a member of the same interval; the other tries
to transport membership from `1/2` to `2/6`. The direct same-form control
checks `2/4` as a member; the accepted transport checks that the law applies
with the same interval and source member. The Architect's AC-0 corrected its
own D0 source: both transport rows use `Proved` for the `rational_equiv`
premise, not `Refl`. The candidate must accept the corrected positive row and
refuse the invalid premise row.

## Accepted client

Check this entire client before appending either isolated refusal below.

```ken
import Data.Numeric.Rational (Rational, MkRational)

import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_contains_respects_equiv)

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const two_quarters : Rational = MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const two_sixths : Rational = MkRational (Suc (Suc Zero)) (Suc (Suc (Suc (Suc (Suc Zero)))))

const one : Rational = MkRational (Suc Zero) Zero

const i : Interval = MkInterval half one

theorem half_member : IsTrue (interval_contains i half) = Proved

theorem moved_by_equivalence : IsTrue (interval_contains i two_quarters) =
  interval_contains_respects_equiv i half two_quarters Proved half_member

theorem two_quarters_member_control : IsTrue (interval_contains i two_quarters) = Proved
```

### stdlib/numerical/interval-contains-respects-equivalence

- spec: §12, `interval_contains_respects_equiv`.
- promise class: durable invariant.
- given: `half = 1/2`, `two_quarters = 2/4`, and
  `i = [1/2,1]`.
- expect: membership of `half` transports to `two_quarters`, and the
  direct same-form membership control accepts.
- measured: the client checks source membership, applies the public
  law with `Proved` for `rational_equiv`, and checks target membership.
- claimed: equivalent rational spellings have the same membership in
  an interval under the checked public law.
- gap: these closed applications do not establish the generic law.
- why: the transport has the same interval, operation, source value
  and claim form as the direct control.

## Isolated refused membership claim

On a fresh complete copy of the accepted client, append only
`reject_two_sixths_member` from the block below. The accepted direct
`two_quarters_member_control` uses the same interval and claim form; only the
member value changes. `two_sixths` denotes `2/6`, the value `1/3`, below the
lower endpoint `1/2`.

### stdlib/numerical/reject-two-sixths-interval-membership

- spec: §1.1 and §12.
- promise class: durable invariant.
- given: `i = [1/2,1]`, with the accepted same-form control for
  `2/4`.
- expect: claiming that `2/6` belongs to `i` is refused.
- measured: the intended candidate package-test pin is this false
  membership claim; this seed remains unexecuted.
- claimed: membership uses rational values, and `1/3` is below the
  interval's lower endpoint.
- gap: this concrete refusal does not establish the generic law.
- why: the positive control differs only in the claimed member.

```ken
-- Reject: 2/6 denotes 1/3, below the lower endpoint 1/2.
theorem reject_two_sixths_member : IsTrue (interval_contains i two_sixths) = Proved
```

## Isolated refused equivalence transport

On another fresh complete copy of the accepted client, append only
`reject_transport_to_two_sixths` from the block below. The accepted
`moved_by_equivalence` uses the same law, interval and source membership;
the claimed target differs. Here `Proved` cannot supply the false premise
`rational_equiv half two_sixths`.

### stdlib/numerical/reject-transport-to-two-sixths

- spec: §12, `interval_contains_respects_equiv`.
- promise class: durable invariant.
- given: `i = [1/2,1]`, source `half = 1/2`, and the accepted
  transport to `2/4`.
- expect: the transport claim to `2/6` is refused at its false
  equivalence premise.
- measured: the intended candidate package-test pin is this invalid
  equivalence premise; this seed remains unexecuted.
- claimed: the public law transports membership only when its
  rational-equivalence premise holds.
- gap: this one false premise does not establish the law's generic
  behavior.
- why: the accepted and refused transport use the same law and source
  membership, varying only the target value.

```ken
-- Reject: 1/2 is not rational_equiv to 2/6.
theorem reject_transport_to_two_sixths : IsTrue (interval_contains i two_sixths) =
  interval_contains_respects_equiv i half two_sixths Proved half_member
```

## Evidence boundary

The Architect's AC-0 ruling is recorded in `evt_2crpyyx0fafnw`. Its D0
source incorrectly used `Refl` for `rational_equiv`; the correction rules
`Proved` for the equivalence premise in both transport rows. The five
candidate pins are `half_member`, `moved_by_equivalence`,
`two_quarters_member_control`, `reject_two_sixths_member`, and
`reject_transport_to_two_sixths`. The optional point-sound converse
composition is omitted; the five required AC-2 pins are unchanged.

This CV turn performed static source and structure review only. No Ken check
or official conformance-runner execution of this seed, package test, CI,
publication, landing, or landed-trust result is claimed. No external or
`local/refs` material was consulted.
