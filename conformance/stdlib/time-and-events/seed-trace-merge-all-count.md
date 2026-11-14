# Finite-trace merge count preservation

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §16.
Frame: `docs/program/issues/SPEC-TIME-TRACE-MERGE-ALL-COUNT.md`.

The client has three input traces. The queried event occurs once in each
of two traces; a third event at a different timestamp keeps the classifier
from counting every event. The Boolean classifier recognizes equal event
timestamps. In this closed input, only the two copies of the queried event
have its timestamp. The accepted client applies the public generic law and
independently checks the merged count as two. The refused client is a fresh
copy of the accepted prefix with only the closed count claim changed to one.

## Accepted three-trace client

```ken
import Core.Classes.LawfulClasses (bool_and, leq_nat)
import Data.Time.EventTrace
  (Event, MkEvent, event_time, trace_merge_all, trace_merge_all_count)
import Data.Collections.Derived (count, list_concat)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two

const early : Event Bool = MkEvent Bool zero False
const repeated : Event Bool = MkEvent Bool three True

fn same_time (x : Event Bool) (y : Event Bool) : Bool =
  bool_and
    (leq_nat (event_time Bool x) (event_time Bool y))
    (leq_nat (event_time Bool y) (event_time Bool x))

const first : List (Event Bool) = Cons (Event Bool) early (Nil (Event Bool))
const second : List (Event Bool) =
  Cons (Event Bool) repeated (Nil (Event Bool))
const third : List (Event Bool) =
  Cons (Event Bool) repeated (Nil (Event Bool))

const three_traces : List (List (Event Bool)) =
  Cons (List (Event Bool)) first
    (Cons (List (Event Bool)) second
      (Cons (List (Event Bool)) third (Nil (List (Event Bool)))))

theorem count_preserved
    : Equal Nat
        (count (Event Bool) same_time repeated
          (trace_merge_all Bool three_traces))
        (count (Event Bool) same_time repeated
          (list_concat (Event Bool) three_traces)) =
  trace_merge_all_count Bool same_time repeated three_traces

theorem merged_count_is_two
    : Equal Nat
        (count (Event Bool) same_time repeated
          (trace_merge_all Bool three_traces))
        two = Proved
```

### stdlib/time-and-events/trace-merge-all-count-three

- spec: §16.1 and §16.3, `trace_merge_all_count`.
- given: the closed three-trace input and its timestamp classifier.
- expect: `count_preserved` applies the public law; the independent
  `merged_count_is_two` claim checks by reduction.
- measured: the two repeated occurrences, the distinct-timestamp control,
  one generic law application, and the closed count result.
- claimed: the count of the queried event is preserved at two.
- gap: this finite case does not establish the generic law for every
  classifier, query, and finite trace list.
- why: the traces contain exactly two occurrences with the queried timestamp;
  the third trace is present and contributes no such occurrence.

## Refused count-one claim

On a fresh complete copy of the accepted client above, append only the
following theorem. The accepted prefix, including its two-count control and
generic law application, must check before this false closed claim is
considered. The expected refusal is at the `Proved` claim in
`merged_count_is_one`, not at an import or unrelated declaration.

```ken

theorem merged_count_is_one
    : Equal Nat
        (count (Event Bool) same_time repeated
          (trace_merge_all Bool three_traces))
        one = Proved
```

### stdlib/time-and-events/reject-trace-merge-all-count-one

- spec: §16.3, which pins the merged count at two for this input.
- given: the accepted client and its complete prefix, with only the false
  count-one claim appended.
- expect: the prefix accepts; `merged_count_is_one` is refused at its
  `Proved` claim.
- measured: the accepting count-two claim uses the same input, classifier,
  merged expression, and equality form.
- claimed: one occurrence is not an acceptable result for these two copies.
- gap: this closed refusal alone does not establish generic preservation.
- why: the operation and imports are unchanged; only the claimed count
  differs from the accepted control.

## Evidence boundary

The accepted and refused clients are authored from §16. The Foundation
catalog child and its named package tests are separate evidence; neither is
an assembled-seed run. Local scratch checks, if performed, are not official
conformance execution, CI, publication, landing, or landed-trust evidence.
