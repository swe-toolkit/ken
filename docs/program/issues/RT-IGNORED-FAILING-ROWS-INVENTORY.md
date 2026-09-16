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

## One measured instance of the banner's own warning: `px8ds`

**The banner above says the sixteen are a hypothesis about the population, not
the population. Here is a row that makes that concrete, and it is an ORDINARY
member of the ledger — not an escalation and not a regression.**

    row      px8ds_real_same_depth_path_runs_exact_edges
    file     crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:389
    label    "focused native resource-cost row; run outside default suite"

**What is MEASURED**, all at `origin/main`
`89d33f74cbe8227580acc486df3f5879f40e31b3` unless stated:

1. The row is `#[ignore]`d on `main` (`:388`).
2. It is **not** in `.github/ignored-test-exemptions.toml` — that registry names
   six rows and none of them is in `px8ta_oriented_subcontinuation`.
3. The frame's §2 listing names `ken-cli::px8ta_oriented_subcontinuation` with
   **one** row, `public_two_three_level_brackets_finish_and_release_lifo`.
   **`px8ds` is a different row in the same binary and is not listed.**
4. At `0f71ab5b9267781ae1d91bc654011cad42b926af` the row **passed** — it was a
   member of that tree's twelve.
5. It is absent from the eleven passing on run `34753101365` (2026-09-13) and
   absent from today's eleven.

**What is INFERRED, and labelled as such:** ignored plus un-exempted means the
sweep selects it, and absent-from-the-passing-set then places it in the failing
set. **That is an argument from two measurements, not a third measurement.**
`D0` confirms or kills it, and if the row turns out not to be selected at all
**that is the more interesting answer** — say so rather than forcing it into the
ledger.

**Do NOT write this up as a regression.** `0f71ab5b9` is **not an ancestor of
`main`**. There is no shared history in which a status changed, so "moved from
passing to failing" has no referent. The honest statement is **one row, two
disjoint trees, two results** — and producing the signature on the tree that
matters is exactly this node's job.

**First instruction for whoever takes this row: get its own failure output.**
Every hypothesis about why it is not passing — unlanded work, a correctly
failing assertion, the runner's thread/stack provisioning — is idle until the
row has been run and read. Its label explicitly says it runs on a separately
provisioned thread outside the default suite, so **the run conditions are part
of the signature**, not background.

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
