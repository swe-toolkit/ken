# WP frame — `RT-HOST-RESPONSE-DUPLICATE-PRELUDE-BLOCK`

> # VOID — THIS FRAME'S NODE IS SUPERSEDED. DO NOT RELEASE, DO NOT KICK.
>
> **Steward, 2026-09-18.** The node this frames is a duplicate filing of
> `[[RT-DUPLICATED-RESPONSE-BLOCK]]` — same defect, same four rows, same
> census, filed ten minutes apart by the same author, and **both were framed.**
> The surviving id is `RT-DUPLICATED-RESPONSE-BLOCK`, which is in flight.
>
> **The live frame is `docs/program/wp/RT-DUPLICATED-RESPONSE-BLOCK.md`.**
> Release that one. The node's tombstone explains the fold and what moved.
>
> This file is kept, not deleted, so existing citations resolve to an
> explanation rather than to a 404 — **and because a deleted frame cannot tell
> you it was ever a duplicate.**

    owner    runtime
    size     M
    tier     T1   (a design fork routes out of this node; the D0 is a hunt,
                   not a ledger execution)
    base     origin/main e75f1fe27 at framing time -- cut from whatever main
             is when you start, and RE-MEASURE section 2 there
    node     docs/program/issues/RT-HOST-RESPONSE-DUPLICATE-PRELUDE-BLOCK.md

**Read the node first.** It carries the four rows, the measured mechanism, the
two already-refuted repairs, and the conclusion-without-a-premise that this
frame's `AC-2` exists to settle. This frame does not repeat them.

## 1. Objective

**Resolve the duplication that makes `host_response_routes` see one prelude
block twice, and re-disposition the four rows by file and line.**

The readmission condition in the landed labels — *"readmits when the duplicate
prelude block is resolved"* — is the deliverable. It is the common prerequisite
for all four rows, and it is the only part of the condition this node owns.

## 2. Fixed inputs — RE-MEASURE THESE AT YOUR BASE, do not inherit them

Everything in this section was measured by the Steward at `e75f1fe27` and is a
pointer to check, not a fact to carry.

1. **The four rows** — `px7n_nested_computational_eliminator.rs:149,:170` and
   `rt_escape_second_resource_native.rs:653,:713`. Line numbers move; **key on
   the label text and the function name**, then report the lines you found.
2. **The invariant** — `responses.rs:1279-1283`, inside `host_response_routes`
   at `:1246`. Its consumer is `selected_host_response_route` at `:1289`.
3. **The `ken-runtime` lib-suite baseline.** The diagnosis node used
   `1035 / 0 / 2`. **That was measured at a different base. Re-measure yours
   and state the SHA you measured it at.** A baseline inherited across a moved
   `main` is not a baseline.
4. **None of the four rows is in `.github/ignored-test-exemptions.toml`** (8
   rows there, none in these two files). So each of the four is a *selected*
   row, and readmitting one with an accepted red requires **adding** a registry
   row (`class` + `readmission` + `test_path`), not editing one.

## 3. D0 — where does the second materialization come from?

**One question, and it is upstream of the planner:** what puts the prelude
block into `plan.source_occurrences` twice?

The evidence you are explaining is in the node: 29 constructors, all agreeing
on `operation`, effect-origin deltas collapsing to **one** distinct value per
program (`365` for `px7n`, `317` for both `rt_escape` programs). **Explain the
constant, not just the duplicate.** A cause that predicts a duplicate but not a
*uniform offset* has not explained what was measured.

**The delta differs between programs (`365` vs `317`) while being constant
within each.** That is a live constraint on the cause: whatever duplicates the
block does so at a point whose origin-id distance is program-relative. A cause
that predicts a fixed constant across programs is refuted by this before you
write it down.

**Report the D0 answer before building anything on it**, and say plainly if it
comes back "the duplication is intended" — that is a real answer and it changes
which arm `AC-2` takes.

## 4. The design fork — `AC-2` settles it, YOU do not

**Is the invariant `one host-operation constructor ⇒ one response-handling
site in the whole program` too strong?**

- **Arm A — NO, the invariant is right.** Every second entry is a
  materialization artefact; no admissible Ken program has two distinct
  response-handling sites for one constructor. Then the repair is the
  de-duplication found in D0, the invariant is untouched, and this node closes
  here.
- **Arm B — YES, it is too strong.** Some admissible program legitimately has N
  handling sites. Then the repair is to make routing occurrence-aware — pairing
  N `Vis` sites to N handlers — **which is planner work and is a SUCCESSOR
  node, not this one.** This node closes by delivering the ruling and the D0,
  and the Steward cuts the successor.

**Do not pick an arm to get started.** The two have disjoint deliverables. The
diagnosis's surviving *"too strong"* verdict is not a ruling — its only
supporting case was measured false and no replacement was offered (node,
"THE CONCLUSION THAT OUTLIVED ITS PREMISE").

## 5. Deliverables

1. **The D0 answer** (section 3), with the mechanism named at file and line and
   the constant offset explained.
2. **The ruling on section 4's fork**, cited by `evt_`/`dec_` id, with the
   node recording which arm was taken.
3. **The de-duplication repair**, if arm A.
4. **The four rows dispositioned by file and line** — readmitted and passing,
   or still ignored with a **measured** next blocker. A count is not a
   disposition, and a *stated* blocker is not a measured one.
5. **Every label you leave behind is correct at your base.** A row that stays
   ignored gets a label naming what the run actually does, measured, with the
   SHA. **Leaving a wrong label on a row re-seeds the defect this whole program
   exists to clear** — and this node's own predecessor exists because two labels
   named mechanisms the runs did not exhibit.
6. **A registry row** in `.github/ignored-test-exemptions.toml` for any row
   readmitted with an accepted red.

## 6. Acceptance criteria, with their controls

**AC-1 — four rows, four named dispositions**, resolved by file and line. Say
which of the four the repair reached and which it did not. **Two of four is a
legitimate outcome** (node, "Deferring to point-of-use splits the four rows in
two") and must be reported as that, not as a partial failure.

**AC-2 — the section 4 fork is answered by a cited ruling BEFORE any planner
edit.** Record the `evt_`/`dec_` id and the arm. An edit to
`host_response_routes`' key or loop shape that lands without this citation is
out of scope by construction.

**AC-3 (control, REQUIRED) — the repair is shown to be reaching the site.**
Revert the D0 repair and confirm the rows you readmitted go RED again with the
collision message. **If they stay green without your change, the rows were not
gated on what you fixed** — and that is a finding to report, not a quiet pass.

**AC-4 (control, REQUIRED) — the invariant still refuses something.** If the
D0 repair removes the duplication, the guard at `:1279` now fires on nothing in
these programs. **Name one case it must still refuse and show it still
refuses** — a construction, a targeted unit test, or a stated argument from the
plan shape. **A check that refuses nothing has been deleted rather than
satisfied**, and a silent deletion is exactly how this cluster's labels went
wrong the first time.

**AC-5 — no regression on the `ken-runtime` lib suite** against YOUR
re-measured baseline (section 2.3). **Report the delta in three buckets, never
as a total:** red-on-both (pre-existing), green-on-main-red-here (yours), and
candidate-only with no `main` counterpart. **Publish the third bucket even at
zero** — no subtraction reveals it, so an unstated zero is indistinguishable
from an unrun check.

**AC-6 — the crate set is DERIVED, not named.** Compute the reverse-dependency
closure over your touched set to a fixpoint and test that set. **Never
`--workspace`** (`COORDINATION §12`); the workspace build and the conformance
suite run in CI. **State your target selection beside the claim** — `cargo
check` does not compile `#[cfg(test)]`, so a green `check` is not evidence that
any test built.

**AC-7 — the second blockers are checked and NOT pulled in.** For each row that
does not readmit, say which blocker it now stops at, measured. Specifically:
does the `px7n` pair reach `RT-FRAME-MARKER-ONCE`'s frame-marker message, and
does anything under the `rt_escape` pair confirm or refute
`RT-CLOSURE-BOUNDARY-LANE`? **Both answers are findings for the Steward to
re-cut on. Do not absorb either into this node**, and do not record
"undetermined" as "ruled out".

## 7. Contention

- **`responses.rs`** — `RT-CARRIER-PRODUCER-OCCURRENCE` is `ready` and is the
  runtime ring's current work. **Check its candidate's touched paths against
  yours before you cut**, and if they intersect, sequence rather than run
  concurrently. The Steward routes; raise the intersection rather than
  resolving it yourself.
- **The two test files** carry two ignored rows each that are **not** in scope
  (`rt_escape:684` `RT-SITEOP-CARRIED-WITNESS D2`, `rt_escape:776`
  `RT-PROCESS-EXIT-STATUS`). Do not touch their labels. A crate-wide `fmt` plus
  `git add -A` is how unrelated files get smuggled into a scoped diff — check
  your diff with `git diff --stat <base> HEAD` before handoff and state the
  file and line counts.

## 8. Sizing

One hour to a releasable increment or a genuine hard stop. **The natural split
is D0 first**: the section 3 answer plus the section 4 ruling request is an
increment by itself, and it is the increment that unblocks the rest. If D0 runs
long, stop and post it — a measured cause with no repair is worth more here
than a repair built on an unmeasured one.
