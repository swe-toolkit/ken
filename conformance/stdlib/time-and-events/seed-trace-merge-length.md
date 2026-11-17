# Trace merging preserves event multiplicity

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §19.
Frame: Steward's released SPEC-TIME-MERGE-LENGTH frame (not on this base).

The client applies both premise-free public length laws and separately
checks three closed results. The first pair of singleton traces is
unordered when appended: `[5T,3F]`. The second pair contains **two
occurrences** of the identical event `3T`, one in each input. The
three-trace input has a nonempty first trace, an empty middle trace,
and a nonempty last trace. Every merged length is two. These finite
cases exercise the laws but do not prove their open quantifiers.

## Accepted client

```ken
import Data.Time.EventTrace
  (Event, MkEvent, trace_merge, trace_merge_all,
    trace_merge_length, trace_merge_all_length)
import Data.Collections.Derived (length, list_concat)
import Data.Numeric.Nat.Arithmetic (add)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)

const late : Event Bool = MkEvent Bool five True
const early : Event Bool = MkEvent Bool three False
const same : Event Bool = MkEvent Bool three True

const late_first : List (Event Bool) =
  Cons (Event Bool) late (Nil (Event Bool))
const early_second : List (Event Bool) =
  Cons (Event Bool) early (Nil (Event Bool))
const same_left : List (Event Bool) =
  Cons (Event Bool) same (Nil (Event Bool))
const same_right : List (Event Bool) =
  Cons (Event Bool) same (Nil (Event Bool))

theorem unordered_law
    : Equal Nat
        (length (Event Bool) (trace_merge Bool late_first early_second))
        (add (length (Event Bool) late_first)
          (length (Event Bool) early_second)) =
  trace_merge_length Bool late_first early_second

theorem unordered_two
    : Equal Nat
        (length (Event Bool) (trace_merge Bool late_first early_second))
        two = Proved

theorem duplicate_law
    : Equal Nat
        (length (Event Bool) (trace_merge Bool same_left same_right))
        (add (length (Event Bool) same_left)
          (length (Event Bool) same_right)) =
  trace_merge_length Bool same_left same_right

theorem duplicate_two
    : Equal Nat
        (length (Event Bool) (trace_merge Bool same_left same_right))
        two = Proved

const three_traces : List (List (Event Bool)) =
  Cons (List (Event Bool)) late_first
    (Cons (List (Event Bool)) (Nil (Event Bool))
      (Cons (List (Event Bool)) same_right (Nil (List (Event Bool)))))

theorem all_law
    : Equal Nat
        (length (Event Bool) (trace_merge_all Bool three_traces))
        (length (Event Bool) (list_concat (Event Bool) three_traces)) =
  trace_merge_all_length Bool three_traces

theorem all_two
    : Equal Nat
        (length (Event Bool) (trace_merge_all Bool three_traces))
        two = Proved
```

### stdlib/time-and-events/trace-merge-length-unordered

- spec: §19, `trace_merge_length` without an order premise.
- given: `[5T]` and `[3F]`, whose append `[5T,3F]` is not ordered.
- expect: `unordered_law` applies directly and `unordered_two`
  checks the closed length independently by reduction.
- measured: a public law application with no ordering proof and an
  independently checked two-event result on the unordered pair.
- claimed: each input event contributes one occurrence to the merge
  even when the input order would not make their append sorted.
- gap: this finite case does not establish the open law for all `p`,
  `xs`, and `ys`; the provider's generic checked body does.
- why: a hidden `time_ordered` precondition would prevent this
  unqualified public application, while the closed result checks
  the actual merged cardinality without relying on the law.

### stdlib/time-and-events/trace-merge-length-duplicates

- spec: §19, `trace_merge_length` and duplicate preservation.
- given: the identical `MkEvent Bool three True` occurs in each of
  two one-event input traces.
- expect: `duplicate_law` applies and independent `duplicate_two`
  checks that the merged length is two, not one.
- measured: a same-event pair with one occurrence in each input,
  an application of the public theorem and a closed result check.
- claimed: merging is multiplicity-preserving, not deduplicating.
- gap: the closed two-event case cannot prove generic preservation;
  the public theorem's checked proof is a separate obligation.
- why: distinct payloads or timestamps would leave a set-union
  implementation green; identical occurrences force the distinction.

### stdlib/time-and-events/trace-merge-all-length-empty-middle

- spec: §19, `trace_merge_all_length`.
- given: the list of traces `[late_first, [], same_right]`.
- expect: `all_law` applies the public theorem and independent
  `all_two` checks the length of the right-fold merge as two.
- measured: a three-element outer list with a nonempty tail after
  the empty middle, a public law application and the closed result.
- claimed: all nonempty input contributions survive k-way merge.
- gap: one three-trace instance does not prove the law for every
  finite list; it also does not specify the event order.
- why: a fold that stops at an empty middle trace loses the last
  event and makes the independent two-event result fail.

## Isolated refused duplicate-length-one claim

Append **only** this theorem to a fresh complete copy of the accepted
client above. The accepted prefix, including `duplicate_law` and
`duplicate_two`, must check first. The false claim uses the same
merge expression and `Equal Nat`/`Proved` form as `duplicate_two`;
only the proposed right-hand length changes. Expect a
`KernelRejected(TypeMismatch)` at `duplicate_one` itself, not an
import failure or a refusal of the public law application.

```ken
theorem duplicate_one
    : Equal Nat
        (length (Event Bool) (trace_merge Bool same_left same_right))
        one = Proved
```

### stdlib/time-and-events/reject-trace-merge-length-deduplication

- spec: §19, the duplicate pair's length is two.
- given: the complete accepted prefix plus only `duplicate_one`.
- expect: the prefix accepts and `duplicate_one` refuses at its own
  false `Proved` conclusion.
- measured: the same exact duplicate input and merged expression
  accept with `two` and refuse with `one` on separate checks.
- claimed: interpreting `trace_merge` as deduplicating equal events
  conflicts with the length-preservation contract.
- gap: a single false closed claim does not prove the generic law;
  the accepted law application and provider proof are separate.
- why: changing only the expected length isolates the multiplicity
  boundary, rather than testing an unavailable name or proof form.

## Evidence boundary

Architect's D0 on a scratch copy of `a0a4df8d3` checked the two
generic proofs and an equivalent six-theorem accepted client, then
refused the appended `duplicate_one` with `KernelRejected(TypeMismatch)`;
its binary source was not identified. This seed is newly authored
from the §19 contract and has **not** been run through the official
conformance runner or checked as part of an assembled candidate.
The D0 did not measure trusted `GlobalId` identity sets. The separate
package test owes that measurement, ownership and transparency checks,
generic-client checks and the doubled-`xs` proof mutation. Neither
scratch D0 nor this seed establishes official conformance, CI,
publication, landing or measured landed trust.
