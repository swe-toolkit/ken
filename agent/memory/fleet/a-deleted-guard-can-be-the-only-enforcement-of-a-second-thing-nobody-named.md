---
scope: fleet
audience: (see scope README)
source: private memory
  `a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named`,
  `a-resource-exhaustion-failure-may-be-a-deleted-guard-not-new-code` (R4
  triage, 2026-09-26)
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

    git log --oneline -S'stack_size' -- crates/ken-cli/tests/abi_s6_mapping_surface_native.rs
    1c48b6c5c  fix stack-overflow regression      (adds .stack_size(...) at two sites)
    d8bbef963  ABI-S6 D5a-surface Path-B: ...      (removes both, subject unrelated)
    5fe2b9bd4  restore deleted 32MiB stack guard   (later restoration)

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
