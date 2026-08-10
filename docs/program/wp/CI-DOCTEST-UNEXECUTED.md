# CI-DOCTEST-UNEXECUTED

Owner: **verify**. Size **S**. Gate: none. Depends on: nothing.

## 1. Objective

CI runs no `cargo test --doc` step, on a written premise that the workspace has
zero collectible doctests. The premise is false: `ken-runtime` alone collects
**14**. Establish what actually exists, make the false claims true or retire
them, and put the gate in place so the answer stays true.

## 2. Fixed inputs, measured at `origin/main = 376d495d`

Documentary facts only. **The census is deliberately not here — it is `D1`.**

**2a. The CI premise, `.github/workflows/ci.yml:124-130`**, verbatim:

> `# No `cargo test --workspace --doc --locked` step: nextest does not run`
> `# doctests, but this workspace has none to lose. Every ``` fence inside`
> `# a `///`/`//!` doc comment under crates/ is opened ```text (rustdoc's`
> `# explicit non-Rust marker), so `cargo test --doc` collects zero tests`
> `# today (checked via `grep -rn '```' crates/ --include=*.rs` — every`
> `# opening fence is `text`). If a real ```rust doctest is ever added,`
> `# add a `cargo test --workspace --doc --locked` step back here.`

**2b. `grep -n -- "--doc" .github/workflows/ci.yml` returns three hits, all
inside that comment.** There is no `--doc` step anywhere in the workflow.

**2c. The false claim, `crates/ken-runtime/src/values.rs:126-129`:** *"It also
**runs**, so the capability is shown to be genuinely available rather than
merely well-typed."* Its own preceding sentences state why it matters — a
`compile_fail` block passes for **any** compilation error, so a negative-only
set establishes nothing. `ken-runtime` carries 20 `compile_fail` occurrences.

**2d. The mechanism.** `values.rs:130` opens a **bare** ` ``` `. Rustdoc treats
a marker-less fence as Rust. The premise in 2a was derived by grepping for
markers, and a bare fence has none.

**2e. The Adversary's reported number is a floor, not a count.** It measured
`ken-runtime` only and said so. Its own syntactic probe found 10 collectible
opening fences where the collector reported 14 — **a 40% under-count by
grep**, which is why `D1` is a collector run and not a search.

## 3. Deliverables

**D1 — The census, from the collector.** For each workspace crate, run
`scripts/ken-cargo test --doc -p <crate> -- --list` and report the count.
**Targeted per-crate, never `--workspace`** (`COORDINATION §12`). Then run them
and report **pass/fail per crate**. The Adversary explicitly did not run them,
so their status is unknown and establishing it is this deliverable.

**D2 — Dispose of what `D1` finds.** A failing doctest is a **finding**: report
it with `file:line` and the failure. **Do not delete it and do not mark it
`ignore` to get green** — that reproduces this defect in a quieter form. If a
failure needs a production change, **hold and report**; that is a different
node and it is not this one.

**D3 — Make the `values.rs:129` claim true, or retire it.** Preference is
strongly for making it true, since the whole point of the block is to be the
executed positive control for 20 `compile_fail` siblings. Retirement is
acceptable **only** if `D1`/`D2` show it cannot run, and then the sentence must
say what is actually established rather than being deleted silently.

**D4 — Replace the false CI comment and add the step.** The comment at
`ci.yml:124-130` is retired and replaced by the real state. Add the
`--doc` step. **Sequence this last**, after `D1`/`D2` are green.

## 4. Acceptance criteria

**AC-1 — The gate actually collects and executes.** Add a deliberately failing
doctest in a crate the step covers; the CI step must **red**, naming the file
and the test. Restore byte-identically and re-run green. **A step that is merely
present proves nothing** — this is the same population-side control that
`CI-ROW-CLAIM-COMMENT-FORM` used, and it is required for the same reason.

**AC-2 — The positive control is live.** After `D3`, the `values.rs` block that
claims to run is **collected by `--list` and passes**. State its test name.
Without this the 20 `compile_fail` siblings remain unanchored.

**AC-3 — The census is from the collector, not a search.** `D1`'s numbers must
come from `--list`. **If a grep and the collector disagree, the collector is
right and the disagreement is worth one line in the report** — that gap is the
defect's whole mechanism and the third instance of it in this node's history.

**AC-4 — No count is carried from this frame.** The only number here is 14, for
one crate, from the Adversary. Re-measure everything and report what you get.

**AC-5 — Green before the gate.** `D4`'s step lands only once `D1`/`D2` show the
covered doctests pass. If they do not, **stop and report** — do not narrow the
step's scope to make it green. Narrowing the population to pass is precisely
what produced `CI-ROW-CLAIM-COMMENT-FORM`.

## 5. Scope

**In:** `.github/workflows/ci.yml` (the comment and the step),
`crates/ken-runtime/src/values.rs` (the claim, and its doc block if `D3`
requires it), plus any doc comment whose doctest `D2` must fix.

**Out, and these are bans:**

- **No production behaviour changes.** Doc comments and CI only. If a doctest
  fails because production code is wrong, that is a finding — hold and report.
- **No `#[ignore]` and no deletion** as a route to green.
- **No `--workspace` locally.** Per-crate `-p` only; the workspace `--doc` run
  is CI's.
- **No widening into general CI restructuring.** One step, one comment.

## 6. Contention

`.github/workflows/ci.yml` — **check this at pickup.** No open node writes it
today, but it is the file most likely to collide with an unrelated infra change,
and the publisher App's `Workflows` permission has been a live question before.
**Test the push capability rather than citing a note about it** (`COORDINATION
§7a`): if a `.github/` write is refused, that is a fact to establish by
attempting it, not to escalate from memory.

`crates/ken-runtime/src/values.rs` — the Runtime `c1` recut is active in
`ken-runtime`, but in `src/ir.rs`, the native lowering path, and cross-crate
test consumers. **Confirm the intersection is empty at pickup**; if the recut
has moved into `values.rs`, sequence behind it and re-derive.

> **Framing note.** The Adversary's report and this frame each reproduced the
> defect's own error class before catching it — its fence probe under-counted
> 10 against the collector's 14, and the Steward's per-crate fence count could
> not separate opening from closing fences and was struck from the node. **A
> corpus property that a line-local grep appears to answer is the exact shape
> that has now fooled three readers in a row.** Ask the tool that owns the
> question.
