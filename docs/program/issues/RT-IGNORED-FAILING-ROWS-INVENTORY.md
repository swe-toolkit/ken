---
id: RT-IGNORED-FAILING-ROWS-INVENTORY
title: "Record the actual failure signature and owning node for every ignored row that the sweep runs and that FAILS at the implementation base, and produce a ledger -- repairing nothing. The frame's hypothesis that most of the failing set is one interp/native differential defect is a naming-pattern observation checked against zero signatures; the ledger confirms or kills it, and decides whether the repair program is two nodes or sixteen."
status: ready
owner: runtime
size: S
gate: none
tier: T2
depends_on: [CI-IGNORED-SWEEP]
blocks: []
github: null
origin: "Steward, 2026-09-16, on operator directive 2026-09-15: 'The other tests should be fixed.' Fixing the whole failing set is a program, not a work package, so this node produces the ledger that says how many programs it actually is -- the cut is then made on measured failure signatures instead of on a guess. Frame at docs/program/wp/RT-IGNORED-FAILING-ROWS-INVENTORY.md, landed 507bd4bd1, base-fixed at ac08fb581 on Architect evt_4hba5yyec810x. Steward-filed per COORDINATION section 2."
---

> ## RELEASED to Team Runtime 2026-09-16 — `ready`, size S, tier T2
>
> **Implementation base is `origin/main`** — same ruling as
> `RT-IGNORED-PASSING-ROWS-DISPOSITION`, same reason. A distinctly new effort
> is cut from `main` (operator, 2026-09-16). No dependency on PR #3676; do not
> stack on it.
>
> **The sixteen listed in the frame's §2 are the sixteen at `0f71ab5b9`.**
> Treat that listing as the starting hypothesis for the ledger's population,
> never as the population itself. `AC-1` is satisfied by whatever the sweep
> measures at the base — fifteen or seventeen satisfies it with that number.
> `px8f_buffer_native.rs` is the one named test file that differs between the
> two trees and it holds one of the listed rows.

Read the frame: `docs/program/wp/RT-IGNORED-FAILING-ROWS-INVENTORY.md`.

## This node READS. It repairs nothing.

`AC-4`: a diff touching `crates/**/src/` fails this WP. The deliverable is one
evidence file under `docs/program/evidence/`, one row per failing test, with
the observed signature **read from a run** and never copied from the
`#[ignore]` label.

That distinction is the whole point. The labels in this population cite base
`21fd46dce`, 2364 commits behind `origin/main` as measured at `507bd4bd1`, and
nine of the *passing* rows already carry labels asserting failures that no
longer happen. Sequencing repair work off those labels would cut nodes against
defects that may not exist.

## The hypothesis is the thing being tested, not an input

Frame §3 states a predicate — *the row's assertion is that native and
interpreted execution AGREE* — and counts ten of sixteen carrying it. **Both
the predicate and the count get re-derived at the base.** A shared name is
equally consistent with one shared cause and with ten unrelated defects in one
test family. An explanation that fits is exactly what stops the census being
run.

The frame's own correction box records why the count is stated this way: the
first version said twelve, and each sub-count was the **row count of the
containing binary** rather than the count matching the predicate. `AC-3` takes
the predicate, not the number.

## Sizing

S, and it should stay S. If the ledger's signatures do not fall out of one
sweep run plus reading the failures, that is itself the finding — report it
rather than growing the node.
