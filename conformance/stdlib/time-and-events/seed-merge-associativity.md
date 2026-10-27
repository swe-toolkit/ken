# Left-biased trace merge associativity

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §6.
Frame: `docs/program/issues/SPEC-TIME-MERGE-ASSOCIATIVITY.md`
(Steward catalog-scope amendment `evt_5xar42txx6wnr`).

Each case promises a durable invariant. Three two-event traces
interleave, and the `xs` event `a1` and `zs` event `c0` tie at
time 4. Both groupings yield the exact **stored-event order**
`[a0,b0,a1,c0,b1,c1]`; each side is independently checked
with `Refl` against this explicit list. The public law is applied
separately. Closed computations do not prove a theorem for all
payload types, lists, or unordered inputs.

## Accepted client

The entire accepted client must check before appending the one
refusal. The tie controls check both order comparisons at time 4
without conflating either one with a distinct payload.

```ken
import Data.Time.EventTrace (Event, MkEvent, event_before, trace_merge, trace_merge_assoc)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const a0 : Event Bool = MkEvent Bool one True

const a1 : Event Bool = MkEvent Bool four True

const b0 : Event Bool = MkEvent Bool two False

const b1 : Event Bool = MkEvent Bool five False

const c0 : Event Bool = MkEvent Bool four False

const c1 : Event Bool = MkEvent Bool six False

const xs : List (Event Bool) = Cons (Event Bool) a0 (Cons (Event Bool) a1 (Nil (Event Bool)))

const ys : List (Event Bool) = Cons (Event Bool) b0 (Cons (Event Bool) b1 (Nil (Event Bool)))

const zs : List (Event Bool) = Cons (Event Bool) c0 (Cons (Event Bool) c1 (Nil (Event Bool)))

const regrouped : List (Event Bool) =
  Cons
    (Event Bool)
    a0
    (Cons
      (Event Bool)
      b0
      (Cons
        (Event Bool)
        a1
        (Cons
          (Event Bool)
          c0
          (Cons (Event Bool) b1 (Cons (Event Bool) c1 (Nil (Event Bool)))))))

theorem left_head_ties_outer_right : Equal Bool (event_before Bool a1 c0) True = Proved

theorem outer_right_ties_left_head : Equal Bool (event_before Bool c0 a1) True = Proved

theorem left_grouping_exact
    : Equal (List (Event Bool)) (trace_merge Bool (trace_merge Bool xs ys) zs) regrouped =
  Refl

theorem right_grouping_exact
    : Equal (List (Event Bool)) (trace_merge Bool xs (trace_merge Bool ys zs)) regrouped =
  Refl

theorem merge_regroups
    : Equal
        (List (Event Bool))
        (trace_merge Bool (trace_merge Bool xs ys) zs)
        (trace_merge Bool xs (trace_merge Bool ys zs)) =
  trace_merge_assoc Bool xs ys zs
```

### stdlib/time-and-events/merge-assoc-cross-input-tie

- spec: §6, `trace_merge_assoc`, and §1.2 left-biased ties.
- promise class: durable invariant.
- given: three interleaved traces with distinct payloads at the
  shared time 4 (`a1` and `c0`).
- expect: each grouping reduces to the explicit six-event list
  `regrouped`, and the public law proves their structural equality.
- measured: two independent `Refl` list computations, both
  directions of the time-4 `event_before` check, and one public
  law application.
- claimed: structural associativity holds for all `p`, `xs`, `ys`,
  and `zs`, without an ordering premise.
- gap: these ordered, closed Bool-payload traces neither prove the
  generic theorem nor exercise arbitrary unordered inputs; that
  obligation is discharged by the catalog's generic checked proof.
- why: the separate expected list prevents the law application from
  serving as its own result oracle, and the cross-input tie fixes
  the only axis on which the isolated false equality differs.

## Isolated refused outer-operand swap

On a **fresh complete copy** of the accepted client, append only
the following false theorem. The true `left_grouping_exact` and
`right_grouping_exact` controls use the same `Equal (List (Event
Bool)) ... = Refl` form and the same inputs. The false side instead
swaps the outer merge operands, causing `c0` from `zs` to precede
`a1` at their time-4 tie: the swapped result is
`[a0,b0,c0,a1,b1,c1]`. Refusal must be at the appended `Refl`,
not at an import or earlier control.

### stdlib/time-and-events/reject-swapped-outer-operands

- spec: §6's orientation and §1.2's left-biased tie.
- promise class: durable invariant.
- given: the same three traces, with `a1` and `c0` at time 4.
- expect: reject structural equality after swapping the operands
  of the outer call.
- measured: the complete accepted prefix checks; the appended
  false `Refl` is kernel-refused at its own goal. Architect D0 also
  changed **only** `c0`'s time to 3 in a separate no-tie control,
  and then the swapped equality accepted.
- claimed: associativity must not be misread as commutativity;
  left-biased order at a cross-input tie is observable.
- gap: a single swapped counterexample does not establish general
  noncommutativity, and generic `Refl` diagnostic text alone does
  not locate the cause; the no-tie acceptance isolates it here.
- why: both the proof form and the outer operands are usable,
  while the tie gives the two merged lists distinct event order.

```ken
theorem refuse_swapped_outer_operands :
  Equal
    (List (Event Bool))
    (trace_merge Bool (trace_merge Bool xs ys) zs)
    (trace_merge Bool zs (trace_merge Bool xs ys)) =
  Refl
```

## Evidence boundary

Architect D0 on `df0b57c98` checked the generic law and its seven
private helpers with `ken check` and `ken fmt --check` rc 0,
zero new `Axiom`, two scoped tests passed, and exact scratch
trusted GlobalIds unchanged (114 → 114). Catalog patch SHA-256:
`43db06b11a2d63db19dfa6cddc28217a3ab278662ccf9a7bf2ed78e97c991590`.
Architect's full accepted seed checked; appending only this false
claim kernel-refused, while removing the tie by moving `c0` to
time 3 admitted the swapped claim.

The pre-merge author check on `a4d1662df` accepted the complete
positive prefix and refused the sole appended false `Refl` at bytes
1938–1942 (rc 1/E0038 `TypeMismatch`). Its formatted positive
client passed `ken fmt --check`. A separate pre-merge no-tie
control moved `c0` alone to time 3, updated the expected literal
list and comparison, and admitted the swap (rc 0). On untouched
`df0b57c98`, independent value controls passed without the absent
law import/application; the full client failed rc 1/E0012
`UnboundName` at the missing `trace_merge_assoc` import. These
historical logs under `/workspaces/ken/local/` are
`spec-author-time-assoc-seed-positive.log`,
`spec-author-time-assoc-seed-negative.log`,
`spec-author-time-assoc-SpecTimeAssocNoTie.log`,
`spec-author-time-assoc-base-values.log` and
`spec-author-time-assoc-base-laws.log`. The preexisting CLI used
there was not independently built on the detached base.

After CAT-TIME landed, the author rebuilt `ken-cli` on the merged
candidate through `scripts/ken-cargo build -p ken-cli --bin ken`
(rc 0). The merged EventTrace package passed `ken check` and
`ken fmt --check` (both rc 0). A temporary extraction of the
seed's complete accepted Ken fence was formatted mechanically;
its check and `ken fmt --check` both passed (rc 0). A fresh
complete positive prefix plus only the refused fence failed
rc 1/E0038 `TypeMismatch` at the appended `Refl`, now bytes
1944–1948. Temporary clients were removed. The refreshed logs
under `/workspaces/ken/local/` are
`spec-author-time-assoc-refresh-cli-build.log`,
`spec-author-time-assoc-refresh-catalog-check.log`,
`spec-author-time-assoc-refresh-catalog-fmt.log`,
`spec-author-time-assoc-refresh-seed-format.log`,
`spec-author-time-assoc-refresh-seed-positive.log`,
`spec-author-time-assoc-refresh-seed-fmt.log` and
`spec-author-time-assoc-refresh-seed-negative.log`.
These candidate-local checks do not prove the generic law from closed
instances or establish official conformance-runner execution, CI,
publication, landing or landed trust.
