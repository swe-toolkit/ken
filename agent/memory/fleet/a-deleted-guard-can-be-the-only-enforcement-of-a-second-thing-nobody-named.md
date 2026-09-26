---
scope: fleet
audience: (see scope README)
source: private memory
  `a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named`,
  `a-resource-exhaustion-failure-may-be-a-deleted-guard-not-new-code` (R4
  triage, 2026-09-26); Adversary lesson
  `dropping-a-conjunct-from-a-gate-un-gates-a-downstream-assertion-whose-comment-names-the-dropped-precondition`
  (2026-08-26, curated 2026-09-26)
---

# A deleted guard can be the only enforcement of a second thing nobody named

A refusal, guard, stub, or fallback arm deleted for a correct, well-justified
reason can also be the sole enforcement point of a second invariant nobody
wrote down anywhere — not in its name, not in a comment, not in an
acceptance criterion. The deletion is reviewed against the reason given for
it, and against that reason it looks ideal, so nobody asks what else the same
code was doing. The second duty fails silently, on a path the deletion never
mentions.

## The deletion case

A prelude stub that refused a non-zero file offset was removed once the spec
made an offset-less `FileBacked` mandatory — correct, spec-required, and the
best possible way to retire a check: the state it guarded became
unrepresentable. But the stub sat **above both executors** in the prelude, and
it was also the only thing enforcing "a `RepresentedUnavailable` operation is
refused" for the interpreter path (the native path had its own separate gate,
`require_native_operation_v1`). Once the stub went, the same Ken source
executed under the interpreter and was refused natively — one program, two
meanings, from a deletion whose stated rationale was airtight and entirely
about something else.

The remedy that generalizes: put an invariant at the single point every
consumer traverses (in that codebase, `dispatch_host_op_v1`, called by both
the interpreter and the native backend), gated on the property itself rather
than on the one operation that exposed the gap — one line then covers every
path, and any replacement must land at the same choke point or higher.

## The correction and retraction variants

The same shape recurs without a deletion. Rewriting a multi-claim sentence to
fix one wrong claim can delete an adjacent, correct, and independent claim
that happened to share the sentence — a correction inherits the scope of the
sentence it lands in, not the scope of the error it targets. Retracting a
multi-sentence claim by quoting only its first sentence leaves the remaining
sentences asserting the withdrawn claim in full, present tense, because a
retraction's scope is a textual boundary that can fall inside a claim instead
of around it. Both fail the same review: enumerate every claim a unit
carries, mark each refuted or untouched, and carry the untouched ones forward
explicitly — if the unit is too big to do that, split it before correcting.

## The partial-deletion variant: dropping one conjunct from a gate

A gate widened by removing one conjunct (`A && B` becomes `A`) deletes part of
a guard. Downstream of it, an `expect`, `unwrap`, `panic!` or `Some`-access
whose safety `B` guaranteed is now reachable on the `!B` values the widening
admits.

**Measured 2026-08-26 on the landed squash `21d621303`** (RT-ITREE D1,
Adversary post-merge hunt; cranelift carried-computational-match lowering, own
delta 6 files +458/-63; verdict: findings around a sound core; reported
evt_wxfvb470pxh1, lieutenant M8 thread thr_6fe6wp65996a8). The checked-answer
fallback selector went from
`answer_route == CheckedSelectedRecursor && px8tr_deforested_answer_route_enabled()`
to just `px8tr_deforested_answer_route_enabled()` (`core.rs:12546` at that SHA).
The dropped conjunct guaranteed `checked_frame_id` is `Some`; downstream,
`eliminator.checked_frame_id.expect("checked answer routes carry exact frame ids")`
(`core.rs:12599`, `#[cfg(test)]` only; on current `main` the expect lives in
`lowering/source.rs`). An ordinary `DirectScrutinee` match with
`checked_frame_id: None` (built by non-bridge `ComputationalMatch` lowering at
`core.rs:3049-3082` and 3334/3572/3877, where `checked.id` reads `None` with no
active subcontinuation frame, `mod.rs:11150-11157`) and ITree `Ret`/`Vis`
topology now reaches the `expect` and panics in test builds.

**The tell: the assertion's justification comment names the dropped conjunct.**
*"checked answer routes carry exact frame ids"* is exactly the precondition the
widened gate removed. When a gate loses a term, read every downstream
`expect`/`unwrap` message that asserts a property and ask whether the removed
term established it.

**Bound the impact by venue and by direction, and keep unproven legs
unproven.** Venue: the `expect` is `#[cfg(test)]`, so the confirmed impact is a
test-build panic, not a shipped crash. Direction: at runtime the widened path is
fail-closed and exit-preserving (for a Direct frame `route_control` is the
compile-time constant `0`, so the program still takes the default trap, exit
1): no miscompile, rank it leak-or-gap. The unproven leg: the return-case body
is lowered a second time unconditionally in production too
(`core.rs:12601-12645`) under `case_env = [Carried(scrutinee)] ++ env`; if that
can `Err`, an ITree interpreter that compiled before now fails to compile. It
was reported as a real unconditional path whose failure is unproven, naming the
fixture that would settle it.

**A witness must clear the upstream guards first.** The first-pass witnesses,
the D8m bridge arms (`core.rs:6069`/`6170`), genuinely set
`checked_frame_id: None` and `answer_route: DirectScrutinee` — and are refuted:
both set `deferred_constructor_case: Some(&deferred)`, and `_inner` refuses
unconditionally on that field at `core.rs:12293-12299`, a pre-existing guard,
before control reaches the widened gate. The adversarial refute pass caught the
over-attribution and the reachable witness was the adjacent ordinary-match
family (`deferred_constructor_case: None`). ⇒ **A construction site that sets
the field your target reads is not a witness until you chain the whole path and
confirm no upstream guard refuses it first.** A refutable witness sinks an
otherwise-real finding. Siblings:
[[a-narrowed-check-lands-on-the-node-not-on-the-value-that-reaches-it]],
[[an-unreachability-argument-covers-one-route-and-the-catch-all-covers-another]].

**Anchoring detail from the same hunt.** The dispatch range base `5272a68d4` was
stale by one intervening doc commit (the squash's real parent is `e3a31614f`),
so the dispatched range unioned an extra file. The hunt target is the squash's
own delta `21d621303^..21d621303`, and `git diff 7b1820194 21d621303 -- <the six
files>` came back empty, proving landed equals reviewed; per-file delta sizes
differed only because the two candidates had different parents.

**Secondary: a missing discriminating pair.** The one fixture built for opposite
per-edge routes (`d6a_mixed_route_predecessors_at_one_origin_stay_separate`,
`specialization_binding.rs`) never asserts `header_controls()`, and the three
tests that pin per-edge control words are all uniform-route, so a swap between
the two `carried_computational_loop_control_word` call sites (`core.rs:12216` vs
`12232`) is invisible
([[a-non-degenerate-pair-fails-to-fail-if-the-assertion-cannot-tell-the-halves-apart]]).
Widening leaves old checks green over the current population in general: see
[[a-filter-or-list-keyed-on-todays-members-expires-when-the-kind-widens]],
[[a-filter-or-list-keyed-on-todays-members-expires-when-the-kind-widens]] and
[[a-green-census-proves-its-classifier-total-over-the-current-population-not-total]].

## The resource-exhaustion diagnostic

CI can fail with a resource signature — stack overflow, OOM, timeout, fd
exhaustion — on a candidate that never touches the failing file. The instinct
is to reason about the candidate's own diff or bisect it; that is not the
cheapest instrument. Ask first whether a guard for this **exact** failure, on
this **exact** file, existed and was deleted:

    git log -S'<guard token>' -- <failing file>

Guard tokens worth trying: `stack_size`, `RUST_MIN_STACK`, `timeout`, `limit`,
`reserve`, `with_capacity`. This command still runs against a real incident in
this repo:

    git log --oneline -S'stack_size' --
    crates/ken-cli/tests/abi_s6_mapping_surface_native.rs 1c48b6c5c fix
    stack-overflow regression (adds .stack_size(...) at two sites) d8bbef963
    ABI-S6 D5a-surface Path-B: ... (removes both, subject unrelated) 5fe2b9bd4
    restore deleted 32MiB stack guard (later restoration)

The removing commit's subject was about mapping-surface alignment, not stack
size — that is *why* the revert was invisible, not a reason to doubt the
`-S` result. Read the removing commit's message and expect it to be about
something else.

**The candidate is then correctly described as the trigger, not the cause,**
and the remedy is main-side. Guard against the trap this creates: a margin
ruling made during review (e.g. "a ~1% per-frame growth is proportionate, add
N MiB of headroom") can be true and still meet a module whose margin was
separately deleted — both facts real, only one the defect. Close the
question by mapping every failing test to its shared helper and checking
which siblings in the same file are **not** failing: a clean partition
(failing set == the file's whole test set == the set routed through the
de-guarded helper) is the closure; "all the failures fit my story" alone is
satisfied by any story that covers the observed set.

## How to apply

- When a diff deletes a refusal, guard, stub, or fallback arm, ask what else
  it was the **only** enforcement of — not merely whether its stated purpose
  is now satisfied. Trace every path that reached it. Look hardest when it sat
  above a fork: a guard in shared code upstream of two backends is, by
  position, the only thing enforcing anything uniformly.
- When a diff widens a gate by dropping a conjunct, name the dropped conjunct,
  find every assertion or `unwrap` downstream that it silently protected (the
  assertion's own message often names it), chain a witness past every upstream
  guard, and bound the result by venue (`cfg(test)` or production) and
  direction (fail-closed or miscompile).
- Before rewriting or retracting a multi-claim sentence or paragraph,
  enumerate its claims and carry every untouched one forward explicitly.
- On a resource-signature CI failure, run `git log -S'<guard token>' --
  <failing file>` before reasoning about the candidate's own diff — it
  answers "was a guard here deleted?" in one command. Never measure stack
  headroom with `RUST_MIN_STACK`, since it throttles `rustc` too, so "no
  overflow" can be the compiler segfaulting rather than the test passing;
  build with `--no-run` at the default stack and invoke the test binary
  directly.
- For a sharded CI failure, a local `-p <crate> --test <name>` run (the only
  local option under this repo's no-`--workspace` policy) is a different
  binary under different feature unification, not a slower copy of the CI
  shard — a local pass is consistent with both "the candidate is innocent"
  and "this configuration never had the problem." Prefer a CI run on an
  ancestor where the suspected file is byte-identical as the control, and
  check each shard's conclusion string, not the workflow's color — a
  doc-only `skipped` or concurrency-cancelled shard is not a pass.
