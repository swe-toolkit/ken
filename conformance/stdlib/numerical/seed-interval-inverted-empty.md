# Inverted rational intervals have no members

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §13.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-INVERTED-EMPTY.md`.

These cases promise durable invariants from the public RationalInterval
laws. The accepted client applies the emptiness law to show that membership
of `rational_zero` in `[rational_one, rational_zero]` leads to `Bottom`,
and applies both endpoint laws to `[rational_zero, rational_one]`. A
refused direct membership claim and its accepted same-form control differ
only in endpoint order. These closed applications do not establish the
generic laws.

`measured` below refers to Architect AC-0's D0 report and ruling, not to a
CV execution of this seed. The seed is for the assembled candidate, where
the three public laws are available.

## Accepted client

Check the entire client before appending the isolated refusal below.

```ken
import Data.Numeric.Rational (Rational, rational_zero, rational_one)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_contains,
    interval_inverted_empty,
    interval_ordered_contains_lo,
    interval_ordered_contains_hi)

const inverted : Interval = MkInterval rational_one rational_zero

const ordered : Interval = MkInterval rational_zero rational_one

theorem inverted_zero_membership_eliminates
    : IsTrue (interval_contains inverted rational_zero) → Bottom =
  λmember. interval_inverted_empty rational_one rational_zero rational_zero Proved member

theorem ordered_low_endpoint_is_a_member : IsTrue (interval_contains ordered rational_zero) =
  interval_ordered_contains_lo rational_zero rational_one Proved

theorem ordered_high_endpoint_is_a_member : IsTrue (interval_contains ordered rational_one) =
  interval_ordered_contains_hi rational_zero rational_one Proved

theorem ordered_zero_membership_control : IsTrue (interval_contains ordered rational_zero) =
  Proved
```

### stdlib/numerical/inverted-interval-empty

- spec: §1.1 and §13; `interval_inverted_empty`.
- promise class: durable invariant.
- given: inverted `[rational_one, rational_zero]` and the public
  emptiness law.
- expect: the law turns a hypothetical membership of `rational_zero`
  into `Bottom`; both endpoint laws establish membership of the ordered
  interval's endpoints.
- measured: Architect AC-0 reports its `accept_client` row accepted
  with the exact public law forms. CV has not executed this seed.
- claimed: an interval with inverted endpoints has no members, while
  each endpoint of an ordered interval is a member.
- gap: these closed instances do not establish the laws for arbitrary
  rational endpoints or members.
- why: the accepted client applies the public emptiness law and both
  endpoint laws to closed values, with direct endpoint memberships as
  controls.

## Isolated refused inverted membership claim

On a fresh complete copy of the accepted client, append only the refused
claim below. The accepted `ordered_zero_membership_control` has the same
membership claim form and member; only the interval's endpoint order
changes.

### stdlib/numerical/reject-member-of-inverted-interval

- spec: §1.1 and §13; `interval_inverted_empty`.
- promise class: durable invariant.
- given: the inverted interval `[rational_one, rational_zero]` and the
  accepted same-form control for the ordered interval.
- expect: directly claiming that `rational_zero` belongs to the
  inverted interval is refused.
- measured: Architect AC-0 reports the `refuse_inverted` row refused
  as `KernelRejected`; this seed remains unexecuted by CV.
- claimed: membership is false for this inverted interval.
- gap: this single closed refusal does not establish the generic
  emptiness law.
- why: the accepted control has the same form and member, varying only
  the endpoints' order.

```ken
-- Reject: the inverted interval has no members.
theorem reject_inverted_zero_membership : IsTrue (interval_contains inverted rational_zero) =
  Proved
```

## Evidence boundary

Architect AC-0 ruling `evt_7p5m1mn4fjmha` fixes the three public law
names and types and reports the corresponding D0 client accepted, the
direct inverted-membership claim refused, and the ordered control
accepted. Those are Architect-reported D0 results, not execution of this
seed. CV performed source review and formatting checks only; it did not
run `ken check`, package tests, or the official conformance runner. No
CI, publication, landing, or landed-trust result is claimed. CV consulted
no `local/refs` or external reference source.
