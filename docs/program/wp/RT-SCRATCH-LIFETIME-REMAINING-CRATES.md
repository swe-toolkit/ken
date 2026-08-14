# RT-SCRATCH-LIFETIME-REMAINING-CRATES

Node: `docs/program/issues/RT-SCRATCH-LIFETIME-REMAINING-CRATES.md`. Read it
first — it carries what the predecessor established, why its scope is the
finding, and the compensating control that keeps the residue invisible.

**Treat every anchor in this frame as perishable. If a fixed input turns out
false against the landed code, say so and escalate — do not quietly build
around it.** Sites below are cited by grep-able phrase rather than line number,
because this node's own deliverables move those lines.

## Fixed inputs, measured by the Steward at `origin/main = 2ca91a3a`

1. **`tempfile` is a dependency of exactly three crates** — `ken-cli`,
   `ken-elaborator`, `ken-runtime` (`grep -l tempfile crates/*/Cargo.toml`).
2. **`crates/ken-interp/src/eval.rs`'s `rt_parity_root` sits inside a
   `#[cfg(test)]` module.** It is a `src/` path but a test-only site, so a
   **dev-dependency** is sufficient for it. Confirm this for every site you
   migrate before choosing `dependencies` over `dev-dependencies`; a
   production-reachable `temp_dir()` site is a different problem and is
   **not** this node — route it back to the Steward rather than folding it in.
3. **`scripts/ken-cargo`'s reaper names the prefixes it sweeps**, including
   `ken-rt-parity-*`, and carries its own 2026-08-05 measurement. Read that
   comment; it is the closest thing to a ledger of which prefixes exist.

## The one thing this frame does NOT hand you

**The population.** An external report puts roughly 73 `temp_dir()`
occurrences across ~35 files at the pre-merge base, with about half outside the
predecessor's two directories and about 28 of those unguarded, and names
`ken-host`, `ken-interp`, `ken-verify` and part of `ken-elaborator` as the
crates involved.

**Those figures are an external report, not a fixed input, and D1 exists to
replace them.** Do not carry them into the handback as measurements; if your
census disagrees with them, your census is the answer and the disagreement is
worth one sentence. The reason this node exists is a population stated more
confidently than its scope supported — inheriting someone else's numbers would
reproduce that exactly.

## The design call, front-loaded

**Define the population by the hazard property, not by a directory list.**

A site belongs to this node's population when it **creates a filesystem
directory whose removal is not tied to a scope exit** — regardless of which
crate it lives in, which environment variable names its parent, or whether the
path carries a timestamp. Both spellings the predecessor found
(`std::env::temp_dir()` and `CARGO_TARGET_TMPDIR`) are instances; enumerate by
the property and let the spellings fall out.

**Where you must bound the scope, the bound is a deliverable of the census, not
a premise of it** — state it, and state what is outside it, so the next reader
inherits a boundary rather than a reassurance.

**Reuse the predecessor's lifetime policy unchanged.** It is landed and
reviewed: migrated system-temporary sites clean unconditionally on success and
on unwind; the one preservation exception is `CARGO_TARGET_TMPDIR`-based and
justified by the location being `cargo clean`-reclaimable, not by evidentiary
value. Do not re-litigate it, and do not invent a second policy for the new
crates.

## Deliverables

**D1 — the census, by property and across every crate.** Enumerate the
population defined above over all of `crates/`. Report **per crate**: total
sites, sites with a drop guard, sites without one, and — for each unguarded
site — whether it is `cfg(test)`-only or production-reachable. Report the two
spellings separately, as the predecessor's `AC-3` requires, **and** report the
crate breakdown, which is the axis it does not range over.

**D2 — state the boundary you are working to, and what is outside it.** One
short section naming any scope bound D1 took and what falls outside it. If the
census is genuinely exhaustive over `crates/`, say that, and say what is
outside `crates/` that a reader might wrongly assume is covered.

**D3 — migrate the unguarded `cfg(test)` sites onto the predecessor's policy**,
adding `tempfile` as a **dev-dependency** to each crate that needs one. Sites
that already carry a real `impl Drop` guard are not migrated and not touched.

**D4 — the second `ken-rt-parity-*` producer, named explicitly.** Migrate
`ken-interp/src/eval.rs`'s `rt_parity_root` and its nine call sites. Called out
as its own deliverable because it is the site the predecessor's own defect
statement describes and the one `scripts/ken-cargo` already sweeps by prefix —
a partial migration that left it would be the same defect a third time.

**D5 — correct the false guard comment on the fixed-name residual.** In
`crates/ken-cli/tests/fs_read_file_lines_flip_e2e.rs`, the comment above the
scratch-dir setup says the source is written to *a fresh file under a per-test
tmp dir*. The directory is **per-file and reused across every test in the
binary**, and the file is overwritten rather than fresh. That sentence is what
the next author reads immediately before choosing a scratch name, and it
currently reassures where the recorded residual says to be careful. Correct it
to describe what the code does and name the residual it belongs to
(`RT-TEST-SCRATCH-RAII`'s fixed-name section). **Comment only — change no
assertion and no path.**

## Acceptance criteria

**AC-1 — D1's report distinguishes the two axes and the crate breakdown.**
Spelling (`temp_dir` vs `CARGO_TARGET_TMPDIR`) reported separately, and a
per-crate row for each. A single total is not the deliverable; the whole point
is that a total hid a boundary.

**AC-2 — every unguarded site is dispositioned, one row each.** For each site
D1 finds unguarded: migrated, already-guarded-on-closer-reading, or
deliberately left with the reason. **No site may be absent from the table**,
and "the rest are fine" is not a row. Cite each by grep-able phrase.

**AC-3 — the migration is proved by drop, not by a passing suite.** For at
least one migrated site per crate touched, demonstrate the directory is removed
**on the unwind path**: force the test body to panic after the scratch dir
exists, confirm the directory is gone afterward, restore. Report one row per
crate — crate, site, directory-gone yes or no. A suite that passes proves the
success path only, which is the path that was never the problem.

**AC-4 — `ken-rt-parity-*` has no unmigrated producer.** Grep the tree for that
prefix's format string and report every producer with its disposition. The
positive control is that the count of unmigrated producers is zero and you
name the ones you found; reporting only the one you fixed does not discharge
this.

**AC-5 — no production-reachable site was migrated silently.** If D1 finds a
`temp_dir()` site that is not `cfg(test)`-only, it is reported and **left
alone**, and the handback names it for the Steward. Migrating one is out of
scope; a `dependencies` entry appearing in any `Cargo.toml` under this node is
the tell that this AC was missed.

**AC-6 — D5 changes no assertion.** `git diff` on
`fs_read_file_lines_flip_e2e.rs` shows comment lines only.

**AC-7 — no-regression, in CI.** Green in CI on the candidate. Locally, build
and test **only the crates you touched**, `-p <crate>` — do **not** run a
`--workspace` build (`COORDINATION §12`).

## Contention

**`crates/ken-interp/src/eval.rs` is the file to check before starting.**
Runtime's in-flight `RT-LEXICAL-R3-FUSION-EMITTER` works in the lexical/fusion
lane; this node touches a `#[cfg(test)]` module far from it, but they are the
same file in at least one crate's case. **Re-check at pickup rather than
trusting this line** — if R3's held object has moved into `eval.rs` since
`2ca91a3a`, say so and the Steward sequences rather than the ring absorbing a
conflict.

No contention with Language: disjoint crates.

## Not this node

No change to what any fixture asserts. No change to runtime, interpreter, host
or verifier behavior. No reclaim-tooling or `scripts/ken-cargo` change — the
reaper is the Steward's operational record. No recount of the predecessor's 40
or 14. No migration of a production-reachable site (AC-5).
