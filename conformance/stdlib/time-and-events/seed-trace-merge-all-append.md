# Finite-list trace merge distributes over batch append

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §18,
child `4b57a17ff7d56df2dbaa455366ed23853548ebc5`.

The two batches each contain one trace. The first trace is `[1T,3T]`;
the second is `[2F]`. Merging the batch lists gives `[1T,2F,3T]`.
Appending the two partial event-list merges instead gives
`[1T,3T,2F]`. This distinguishes append of trace batches from append
of the resulting event lists. Closed rows instantiate the public
contract; they do not establish its generic proof.

## Accepted client

The complete accepted client checks before adding the refusal fence.
It checks the public type at open batches, the exact batch-merge value,
and the law application. A same-form `Refl` control isolates the
false event-list-append claim.

```ken
import Data.Time.EventTrace
  (Event, MkEvent, trace_merge, trace_merge_all,
   trace_merge_all_append)
import Data.Collections.Derived (list_append)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two

const e1 : Event Bool = MkEvent Bool one True
const e2 : Event Bool = MkEvent Bool two False
const e3 : Event Bool = MkEvent Bool three True

const xs : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))
const ys : List (Event Bool) =
  Cons (Event Bool) e2 (Nil (Event Bool))

const xs_batch : List (List (Event Bool)) =
  Cons (List (Event Bool)) xs (Nil (List (Event Bool)))
const ys_batch : List (List (Event Bool)) =
  Cons (List (Event Bool)) ys (Nil (List (Event Bool)))
const both_batches : List (List (Event Bool)) =
  list_append (List (Event Bool)) xs_batch ys_batch

const expected : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e2 (Cons (Event Bool) e3 (Nil (Event Bool))))

theorem append_batches_open
      (p : Type) (first : List (List (Event p)))
      (second : List (List (Event p)))
    : Equal
        (List (Event p))
        (trace_merge_all p (list_append (List (Event p)) first second))
        (trace_merge p (trace_merge_all p first)
          (trace_merge_all p second)) =
  trace_merge_all_append p first second

theorem batch_merge_exact_value
    : Equal (List (Event Bool))
        (trace_merge_all Bool both_batches) expected =
  Refl

theorem batch_merge_same_form_control
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool both_batches)
        (trace_merge Bool
          (trace_merge_all Bool xs_batch)
          (trace_merge_all Bool ys_batch)) =
  Refl

theorem batch_append_law_application
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool both_batches)
        (trace_merge Bool
          (trace_merge_all Bool xs_batch)
          (trace_merge_all Bool ys_batch)) =
  trace_merge_all_append Bool xs_batch ys_batch
```

### stdlib/time-and-events/trace-merge-all-append-open

- spec: §18, `trace_merge_all_append`.
- promise class: durable invariant.
- given: arbitrary payload type and two lists of traces.
- expect: the exact public equality type checks with no ordering premise.
- measured: one open application at arbitrary `p`, `first`, and `second`.
- claimed: the public law applies to any two trace batches.
- gap: this application does not establish the generic proof body.
- why: the theorem call has no supplied order or separation witness.

### stdlib/time-and-events/trace-merge-all-append-interleaving

- spec: §18, `trace_merge_all_append` and §1.2 `trace_merge`.
- promise class: durable invariant.
- given: first batch `[[1T,3T]]` and second batch `[[2F]]`.
- expect: `trace_merge_all` of the concatenated batches is exactly
  `[1T,2F,3T]`.
- measured: a closed `Refl` equality against an independently written
  three-event list.
- claimed: batch merging preserves the merge's interleaving order.
- gap: this one closed instance does not establish the generic law.
- why: the expected list does not use either side of the new law.

### stdlib/time-and-events/trace-merge-all-append-law-application

- spec: §18, `trace_merge_all_append`.
- promise class: durable invariant.
- given: the same interleaving batches and their exact merged value.
- expect: the public law proves the equality between merging the
  concatenated batches and merging their partial results.
- measured: a closed application at the same batches as the value row.
- claimed: the new public theorem is available at its stated type.
- gap: the instance does not prove the generic theorem.
- why: the outer list append and event-list merge occur at their
  distinct specified types.

### stdlib/time-and-events/trace-merge-all-append-same-form-control

- spec: §18, `trace_merge_all_append`.
- promise class: durable invariant.
- given: the same interleaving batches as the refusal below.
- expect: the batch result equals the merge of the two partial results.
- measured: a direct `Refl` control with the same equality form as the
  refusal.
- claimed: the result and proposition form are checkable on this input.
- gap: this closed control does not establish the generic law.
- why: only the refusal replaces the final `trace_merge` with
  event-list append.

## Isolated refused event-list append claim

Append only the refusal below to a fresh complete copy of the accepted
client. The positive control has the same inputs and `Refl` equality
form. The proposed right side changes only from merging the partial
event lists to appending them.

```ken
-- Refused: appending the partial results gives [1T,3T,2F].
theorem refuse_append_of_partial_batch_merges
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool both_batches)
        (list_append (Event Bool)
          (trace_merge_all Bool xs_batch)
          (trace_merge_all Bool ys_batch)) =
  Refl
```

### stdlib/time-and-events/reject-append-of-partial-batch-merges

- spec: §18's orientation and §1.2's current-head merge order.
- promise class: durable invariant.
- given: the same batches as the accepted same-form control.
- expect: reject at the theorem's final `Refl` token with
  `ElabError::TypeMismatch`, not a kernel rejection.
- measured: the accepted control proves the corresponding merge claim
  with `Refl`; only the final operation changes. Architect AC-0
  reported the target TypeMismatch at the false `Refl` on scratch.
- claimed: concatenating the partial event results is not the
  batch-merge law.
- gap: one interleaving example does not characterize every input.
- controls: the accepted direct `Refl` control checks the true claim
  on identical inputs and in the same equality form.
- why: correct batch merge is `[1T,2F,3T]`, while event-list
  append is `[1T,3T,2F]`, so the false claim refuses. A mutant
  `trace_merge_all` that appends partial event results would accept
  this claim and refuse the same-form merge control.

## Evidence boundary

The case expectations follow §18 and §1.2. Architect AC-0 reported a
scratch accepted client and a same-form false append claim refused at
`Refl` with `ElabError::TypeMismatch`; this is not an execution of
this seed. The CV child is conformance material only. No Ken check of
the committed seed, official conformance-runner execution, package
test, CI, publication, landing, or measured landed-trust result is
claimed. No external or `local/refs` material was consulted.
