---
name: a-full-disk-presents-as-a-test-regression-and-df-slash-cannot-see-it
description: "A full `/workspaces/ken` surfaces as a wave of test failures that reads as a regression from your own change; `df /` measures a DIFFERENT filesystem and reports steady free space throughout, so the one instinctive check confirms the wrong conclusion"
metadata:
  type: fleet
  scope: fleet
---

# A full disk presents as a test regression, and `df /` cannot see it

**Measured 2026-08-10 by `runtime-implementer`, mid-c1.** It hit **23 test
failures** and nearly reported them as a regression from its own rewire. The
real cause was **`ld: No space left on device`** — `/workspaces/ken` was at
roughly **1G free**.

## The trap is the instrument, not the symptom

The symptom is survivable on its own; the reason a seat loses most of a turn is
that **the obvious check answers confidently and wrongly.**

```
df -h /                 → 8.0G avail   (an unrelated overlay; NEVER moves)
df -h /workspaces/ken   → /dev/nvme0n1p6, the real device
```

`/` is the container overlay (`nvme0n1p7`). **`/workspaces/ken` is a separate
device, `/dev/nvme0n1p6`.** The implementer watched `df /` report a steady
"8.0G avail" *the entire time it freed 37G*, because that number describes a
partition its build never writes to.

⇒ **A disk check against `/` does not return "unknown" — it returns a confident
"there is space", which is exactly the answer that sends you back to blaming
your own diff.** Always name the path:

```sh
df -h /workspaces/ken        # the only one that answers the question
```

## The linker error is the tell, and it does not look like disk

`ld: No space left on device` and a linker **`SIGBUS`** both read as toolchain
or memory faults, not as a full volume — so they cost a whole cycle before
anyone checks `df`.

⇒ **Add `df -h /workspaces/ken` to the triage of any build or test failure that
is not a compile error.** Never re-derive a candidate verdict, a QA approval, or
a regression claim from a run that died this way.

## Reclaim: your own `target/` is yours, everyone else's is the Steward's

The implementer's own `target/` was **38G**, the largest on the volume;
clearing its stale `target/debug` took it to **264M** and restored p6 to 38G
free. It touched **no other seat's worktree**, and that was the correct call.

**Why the boundary is not politeness.** `target/` looks free to delete because
it is derived data — but the rebuild runs under the **single machine-wide
`ken-cargo` flock**, already the fleet's tightest resource. **Deleting another
seat's cache converts disk pressure into lock-hold time**, and it does so at
whatever moment that seat next needs a verdict. The axis is **imminent work,
never size**.

⇒ Clear your own and pay your own cold rebuild. If that is not enough, **report
the number and stop** — reclaiming across seats is a Steward call, because only
the Steward knows which rings are about to need the flock.

## Before reaching for any `target/`, spend the free levers

In reclaim order, because the first two cost nothing:

1. `bash /workspaces/ken/.moot/truncate-logs.sh` — uses `truncate -s 0`, so
   live appenders keep writing. Do not `rm` these.
2. `find /workspaces/ken/tmp -maxdepth 1 -mindepth 1 -name 'ken-*' -type d
   -mmin +30 -exec rm -rf {} +` — test fixtures mint a timestamped scratch dir
   per run and never clean up, ~2G/hour under load. `-type d` spares agent
   artifacts (they are files); the age guard spares a live suite.
3. `target/` dirs — **costs a cold rebuild under the flock.**

**If a reclaim seems not to move `df`, measure the refill rate before concluding
it failed:** `find /workspaces/ken/tmp -maxdepth 1 -type d -mmin -15 | wc -l`.
Roughly 400 dirs/hour under load will outrun a small reclaim and make a real
one look inert — which is how a seat talks itself into the expensive lever.

## Do not investigate the `du`/`df` gap

`du` and `df` disagree on this volume because the mount is a **subtree bind**
(`/dev/nvme0n1p6[/pat/src/ken]`). The ~108G you cannot see is **the operator's
own source and data** — not waste, not reclaimable, not growing, and **not why
the volume fills** (operator, 2026-08-04). Run `findmnt -R /workspaces/ken` and
move on. It has been escalated four times for a non-condition.

## Why this is filed at `fleet` scope

It was already written down — in the **Steward's private memory**, which no
build seat reads. It has now recurred at least seven times, and this instance
cost an implementer most of a turn mid-WP. **A lesson filed where its audience
does not read is indistinguishable from one nobody wrote.** Every role here
builds and tests, so every role needs it.

Related: [[no-full-local-cargo-builds-targeted-only]],
[[a-p-scoped-run-and-cis-workspace-run-compile-different-feature-sets]].
