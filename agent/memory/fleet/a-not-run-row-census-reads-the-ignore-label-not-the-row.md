---
name: a-not-run-row-census-reads-the-ignore-label-not-the-row
description: The fleet's not-run-row census enumerates `#[ignore]` attributes and reads their reason text. So a runtime self-skip is invisible to it, a reason relabeled from a disclosed defect to "cost only" launders a known-broken row past it, and a truthful relabel can orphan a second defect the old label was the last live carrier of. Diff the behaviour, the reason clause by clause, and the ancestry.
metadata:
  type: feedback
---

# A not-run-row census reads the ignore label, not the row

The fleet tracks not-run rows and deferred debt with a census keyed on the
`#[ignore = "..."]` attribute: a source scan cross-checked against
`cargo nextest list --run-ignored=only`, every row enumerated and claimed
(today `.github/ignored-test-exemptions.toml` plus the CI job that checks it).
**A "every skipped row is claimed" gate constrains the annotation, never the
behaviour, and it trusts the reason text.** Three distinct ways a not-run row
escapes it follow. Before crediting such a census, ask which mechanism it
enumerates and what other mechanism produces the same outcome.

## 1. A runtime self-skip is a not-run row the census cannot see

**Measured 2026-08-17 on `4011e58be`** (`V3-Z3-EMISSION-CONTROL` D2b). A test
gained a probe: spawn the external binary, and on `ErrorKind::NotFound`
`eprintln!` a skip note and `return`.

Run the landed binary once with the tool on `PATH` and once with it masked:

| environment | harness output |
|---|---|
| tool present, body ran | `test <name> ... ok` |
| tool absent, body skipped | `test <name> ... ok`, `0 ignored` |

**Byte-identical.** The `eprintln!` appears only under `--nocapture`; libtest
discards captured output for passing rows, so it is absent from exactly the
artifact CI keeps. The only difference is a per-test duration the harness does
not print. So `... ok` is not evidence a body executed, and "the job log names
the test" discharges nothing. A federation that adopts *read the log, confirm it
names X* as its verification instrument is already blind to this.

A runtime `return` carries no attribute, so it is outside the census population
by construction; measured, the ignored count did not move. Same family as
[[suppressing-a-row-can-empty-a-whole-named-gate]], one level worse: there the
suppression was an `#[ignore]` the census could see and the blindness was in the
aggregate; here nothing anywhere can see it.

**Measure it on the row that matters; do not argue it.** The question was "is
there a route by which a required job stops selecting D1 without any job
disappearing?" Paste the idiom the candidate introduced at the head of D1's
body, re-run the required job's exact command, and diff both axes:

| | the row's log line | totals |
|---|---|---|
| landed | `... ok` | `137 passed; 0 failed; 1 ignored` |
| body never executed | `... ok` | `137 passed; 0 failed; 1 ignored` |

Identical. The job's command carried no `--exact`, no count assertion and no
membership assertion, so it stays green. The route is the idiom, and the proof
is one edit and one targeted run; revert and re-confirm byte-identity after.
Same move as
[[a-test-pinning-a-missing-port-refusal-inverts-its-signal-when-the-port-lands]]
and [[disable-the-mutation-mechanism-to-find-vacuous-controls]].

Check whether the idiom is the first of its kind (one grep over the tree; here a
single hit). A first instance is precedent, and precedent is the part worth
filing, because the row it lands on today may be one nobody gates.

## 2. A disclosed-defect reason relabeled as cost-only launders a broken row

**Measured 2026-08-25 on the landed squash `d9bc68db0`**
(RT-RESOURCE-SEAT-TAG-DOMAIN), one of a lieutenant-dispatched M8 batch of five
independent squashes (RT-DYNAMIC `c7541df21`, RT-gencont-pairing `e10dabf8e`,
Nat-floor `d5c41ec1e`, RT-carried-bool `d82ea01e7`, RT-resource-seat
`d9bc68db0`); four were clean. Reported `evt_44w0ddhjv39t4` (thread
`thr_6fe6wp65996a8`).

The production change was a genuine, well-tested fix: `lower_resource_token_seat`
now reads the transport tag via `band_imm(word, BOUNDARY_TAG_MASK)`, matching
`BoundaryWord::tag()`, instead of `emit_carrier_tag`/`NODE_TAG_ID`, which lived
in an unrelated domain and would have trapped nearly every legitimate carried
resource token. The same squash also rewrote the reason on an unrelated,
already-ignored row, `px8ds_real_same_depth_path_runs_exact_edges`:

- before: `#[ignore = "post-D1 px8ds residual: generated context receives Bool
  payload 1 after host false"]`, with a comment naming the wrong-branch bug (host
  observes `ConsoleIsTerminal(false)`, the generated context reads
  `ImmediateBool` payload 1, selects True, and the continuation returns `-1`
  before a second Console effect).
- after: `#[ignore = "focused native resource-cost row; run outside default
  suite"]`, plus "the assertions pin a successful false branch"; and the
  neighbouring hedge "or a claim that the ignored post-D1 operand residual is
  green" was deleted.

The test body was unchanged and asserts `exit_status == 0` and an exact
effect-trace, which the disclosed defect violates (exit `-1`, early return). So
the row is known-broken if un-ignored, not merely expensive.

**The verification that promotes suspicion to finding.** A relabel from "known
defect" to "cost only" is justified only by a fix, so prove there was none:

```
git merge-base --is-ancestor <disclosure> <squash-base>        # confirm linear
git log <disclosure>..<squash-base> -- crates/<defect-path>/   # must be EMPTY
```

Here `git log d82ea01e7..8bf17eb98 -- crates/ken-runtime/src/` was empty (only
docs commits), and the sibling generated-context fix `e10dabf8e` predates the
disclosure, so the residual was still live. Had a fix landed on that path, the
cost-only reason would be honest and there would be no finding; that is why the
ancestry check, not the diff alone, decides it.

Nothing reads green in CI (the row stays ignored), so the finding is
non-blocking, leak-or-gap. The harm is to the reason census: a reader now sees
"expensive row" where the only static evidence says "known-broken,
unaddressed". The repair is a truthful reason naming the open residual and its
owner, or citing the fix.

Siblings in the construction/deferral honesty family:
[[a-completion-census-that-asserts-construction-reads-as-success-while-the-behavioral-oracle-is-a-dormant-ignore-row]],
[[a-completion-census-that-asserts-construction-reads-as-success-while-the-behavioral-oracle-is-a-dormant-ignore-row]],
and [[an-alarm-that-does-not-name-its-obligation-is-resolved-by-silencing-it]]
(a disclosure quietly rewritten into nothing).

## 3. A truthful relabel can orphan a second defect

**Measured 2026-09-23 on RT-CONTEXT-CAPTURE-CLAIM-ABSENCE**, squash
`a76aed9fc14253fb464c1447812b87ea2d05cabc`. Filed LEAK/GAP at
`evt_132c307qm7jeg` (thread `thr_5zk0nhr4cg8mm`).

Four ignored rows were relabeled to record outcome B, a terminal claim-absence
at L3. The new labels are **true about the rows**. The old labels also carried a
finding about **L2** (`agreeing_recursive_body_unit`, then `core.rs:1230`): the
check tests node identity where the property it needs is body equality. That
finding came from the merged LABEL-CORRECTION node, which called it "real and
should be repaired" but said "no node owns this, and I am not filing one".
Nobody filed the successor, and the live node says "L2 is CORRECT". Once the
labels were rewritten, nothing live carried the defect.

This is not shape 2: no row is misdescribed. The loss is a defect that sits
beside the rows. The landing itself was inert (4 attribute lines; ignore and
test counts unchanged; see
[[a-guard-added-to-a-pattern-can-consume-the-token-the-pattern-searches-for]] on
counting ignore attributes rather than their text).

## How to apply

For any candidate that touches a test body's early exits or an `#[ignore]`
reason:

1. **Self-skip.** Grep the diff for a runtime `return` guarded by an
   environment probe. If present, apply it to the row you care about and diff
   the log line and the totals under the gating job's exact command.
2. **Relabel, row level.** Diff the reason and the neighbouring comment. Watch
   for a move from a specific defect or owner to a benign, cost-only or
   "successful" framing, and for a deleted "not green" hedge. Read the unchanged
   body: if it asserts the success the old reason said was broken, the row is
   known-broken. Run the ancestry check before filing.
3. **Relabel, guard level.** Diff old against new clause by clause. For each
   dropped clause about a guard the row passes through, grep issues and WPs for
   the guard's symbol and file:line and read each hit's `status:`. If only
   merged or closed nodes say it and the successor was deferred, the relabel
   orphaned it. Then check whether the live "this guard is correct" rests on a
   control that can tell the two readings apart (here the unit test compared
   unit ids, not bodies, so it passes under both). For each dropped clause, find
   a live owner or an explicit ruling.

Batch note: a finder fan-out is the right breadth tool for a batch of
independent squashes, but a finder verdict is a candidate, not a filing. Reserve
the top tier to corroborate the highest-risk node independently and to verify
any finding at the diff before filing.
