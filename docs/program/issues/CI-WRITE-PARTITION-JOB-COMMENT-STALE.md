---
id: CI-WRITE-PARTITION-JOB-COMMENT-STALE
title: "ci.yml tells readers that native-slow (px8f_write_partition) is green because it selects zero tests. It has selected and run a real test since 2026-09-05. The comment names a line and an #[ignore] that no longer exist, so it instructs a reader to discount a green that is carrying signal -- and, worse, to discount the job's red."
status: ready
owner: runtime
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Measured by the Steward at 2026-09-15 while routing the lieutenant's M5a escalation on D5b candidate 0f71ab5b9 (evt_7dztnr256few8), whose red was this exact job. Steward-filed per COORDINATION section 2."
---

## The defect

`.github/workflows/ci.yml:287-303`, on the `native-write-partition` job, says:

> THIS JOB CURRENTLY SELECTS ZERO TESTS, and `--no-tests=pass` is what keeps
> that from being an error.
>
> `px8f_write_partition.rs` has exactly one `#[test]`, at `:354`,
> `checked_write_all_reaches_full_short_zero_progress_flip_and_error_prefixes`,
> and it is `#[ignore]`d pending RT-CARRIED-RESOURCE-SCALAR.
>
> Read the green here correctly: this job is green because it is EMPTY, not
> because anything passed. It carries no signal about the write partition
> while that row is ignored.

Measured on `origin/main` at `1dec48f33cc0697569e9e8765874fe61269332f4`:

    only #[test] in the file        :396, not :354
    #[ignore] anywhere in the file  none
    the Test step                   cargo nextest run --locked -p ken-verify \
                                      --test px8f_write_partition --no-tests=pass

The step carries no filter, so it selects that test. The comment's own undo
condition — "RT-CARRIED-RESOURCE-SCALAR re-arms it: when that node un-ignores
`:354` the job selects a real test again and **this comment stops being true**"
— has already fired. The row was armed by `4b1a0a590` (2026-09-05) at the
latest; `git log -S'ignore'` on the file attributes the `#[ignore]` to the
RT-SRCBODY-BIND-ORDER base-debt quarantine, and nothing in the file carries it
now.

## Why this is worth a node rather than a drive-by edit

The comment does not merely describe the job. **It instructs the reader how to
read the job's result**, and the instruction is now backwards in both
directions:

- a **green** is described as empty and signal-free, when it is a real pass of
  a native write-partition differential;
- a reader who has internalised "this job carries no signal" is primed to
  discount a **red**, which is the expensive direction and is exactly the
  reading that was available during the D5b escalation.

The same file argues this principle against itself, twenty lines up, about the
nextest action pin: *"Keep the version comment in sync with the SHA — a stale
version comment is a false provenance claim."* This is that, about a test
population instead of a dependency.

## Note the second, quieter hazard

`--no-tests=pass` was added to suppress the empty-selection error. It is still
correct to keep — it suppresses only an empty selection, never a failing test,
and the comment says so. But with the row armed it now also means **a future
change that silently stops selecting this test produces a green job**, with
nothing to say the population went to zero. That is the same shape as the
`#[ignore]` quarantine that produced this node, one level up.

## Deliverables

- **D0.** Correct the comment to the tree: the row is live, at its real line,
  and the job's result carries signal in both directions. Delete the
  re-arming paragraph, which has already happened.
- **D1.** State what a green and a red each mean now, in one line each, so the
  next reader gets the instruction rather than the history.
- **D2.** Decide and record whether `--no-tests=pass` should stay. If it stays,
  say in the comment that an empty selection is now a silent green and what
  would cause one.

## Acceptance criteria

- **AC-1.** No claim in the comment is false against the tree at the SHA the
  fix lands on. Check the line number, the `#[ignore]`, and the selection
  behaviour, each against the file rather than against this node.
- **AC-2.** The sibling `native-slow (px8f_buffer_native)` job carries a
  near-identical comment block. **Check it the same way and fix it if it is
  also stale** — do not fix only the one this node names. A census of both is
  the deliverable, even if the answer for the sibling is "accurate, unchanged".
- **AC-3.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Sequencing

**Queued behind the active lane.** Do not let this compete with the D5b repair
it was found next to. It is a comment fix; it can wait for a quiet moment, and
it should not be handed to the seat working the escalation.

## Why T2

The measurement is done and recorded above. The remaining work is reading two
comment blocks against the tree and rewriting them. No design judgment is left.
