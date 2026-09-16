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

## `px8ds`: a RULED disposition that was never executed

**This row is NOT a member of this node's ledger and must not be worked as
one.** It is recorded here because this is where someone comparing rosters will
come looking for it, and because the reason it is loose is worth more than the
row is.

    row      px8ds_real_same_depth_path_runs_exact_edges
    file     crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:389
    label    "focused native resource-cost row; run outside default suite"

**Its disposition is already ruled.**
`docs/program/wp/RT-IGNORED-PASSING-ROWS-DISPOSITION.md` classifies it under
**`D-REGISTER` — keep ignored, add to `.github/ignored-test-exemptions.toml`**,
class **`policy-cost`**, *"exactly like the registered
`b2v_ac10_..._at_thirty_thousand`"*, with the instruction that the implementer
**confirms rather than re-decides**. That is an operative section of that frame,
not a stale roster.

**What is MEASURED**, at `origin/main`
`89d33f74cbe8227580acc486df3f5879f40e31b3` unless stated:

1. `#[ignore]`d on `main` (`:388`), with the label above.
2. **Not** in `.github/ignored-test-exemptions.toml` — the registry names six
   rows and none is in `px8ta_oriented_subcontinuation`. **The ruled
   registration has not happened.**
3. Pre-classified `D-REGISTER`/`policy-cost` in the disposition frame, which
   derived that population from the **twelve at `0f71ab5b9`**.
4. Absent from the eleven passing on run `34753101365` (2026-09-13) and from
   the eleven the disposition node actually executed against.
5. That node completed **eleven of eleven — nine readmitted, two held.**
   `px8ds` is in none of the three outcomes.

⇒ **The row fell out of the population between the frame being written and the
frame being executed.** The disposition was ruled against the twelve, the
executing node correctly worked the eleven it measured at its own base, and
**the ruling for the row that is in the first set and not the second was
silently not carried out.** Nobody erred: the frame's pre-classification was
sound, and the executing node was explicitly told to work its measured
population and not manufacture a twelfth.

**This node's §2 listing does not name it either** — §2 names
`ken-cli::px8ta_oriented_subcontinuation` with
`public_two_three_level_brackets_finish_and_release_lifo`, a different row in
the same binary. **That binary holds three ignored rows** (`:256`, `:283`,
`:389`) and the two frames between them name two.

## What to do with it, and what not to

**Do not produce a failure signature for it and do not add it to the ledger.**
Its `#[ignore]` reason is a standing policy cost, not a defect — the sweep
should never have been reporting it, which is precisely what `D-REGISTER`
exists to fix. Running it to get a signature answers a question nobody asked.

**The action is registration, and it belongs to
`RT-IGNORED-PASSING-ROWS-DISPOSITION`, not here.** One registry row,
class `policy-cost`. Whoever picks that up should also re-check
`d0_distinct_recursive_map_child`, the other row pre-classified `D-REGISTER` in
the same passage, against the executed eleven.

**Do NOT write it up as a regression.** `0f71ab5b9` is **not an ancestor of
`main`**, so there is no shared history in which a status changed and "moved
from passing to failing" has no referent. **One row, two disjoint trees, two
results** — and since the ruled disposition is to keep it ignored, its colour on
either tree is not the question.

## The generalizable defect, which is the reason this section exists

**A per-row ruling made against one measured population is silently dropped for
any row that leaves the population before execution.** The executing node's
completion report is honest — eleven of eleven — and the dropped row appears in
no outcome, no diff, and no red. **There is no artifact in which its absence
shows up**, which is why it took a set comparison between two rosters to find.

**The cheap detector is a per-binary census rather than a better roster:** for
each file containing `#[ignore]`, does every ignored row appear in exactly one
operative disposition? `px8ta_oriented_subcontinuation.rs` returns **3 rows, 2
claimed** on that query, with no hand comparison of sets.

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
