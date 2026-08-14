---
id: RT-SCRATCH-LIFETIME-REMAINING-CRATES
title: "`RT-TEST-SCRATCH-RAII` fixed the scratch-directory leak in the two directories its census declared, and the defect is not confined to them -- unguarded `temp_dir()` sites remain in `ken-interp`, `ken-host` and `ken-verify`, including one that reproduces the original node's defect statement verbatim and generates the second half of a prefix `scripts/ken-cargo`'s reaper already names"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-TEST-SCRATCH-RAII]
blocks: []
github: null
origin: Adversary hunt on the landed squash `b8fe2d90`, evt_7trpxj2rwra31, target 3. Its verdict on the predecessor is that the stated counts are exact and the classification of them is correct -- the finding is the scope the census declared, not an error inside it. Triaged by the Steward 2026-08-14. The `tempfile` dependency boundary and the `cfg(test)` siting of the strongest site are the Steward's own measurements at `2ca91a3a`.
---

## What the predecessor established, and where it stopped

`RT-TEST-SCRATCH-RAII` (squash `b8fe2d90`) declared its population as *"40
`std::env::temp_dir()` call sites across `crates/ken-runtime/src/` and
`crates/ken-cli/tests/`"*, classified all 40, migrated the leaking ones onto
RAII, and separately censused 14 `CARGO_TARGET_TMPDIR` sites. **An independent
recount agreed with both numbers exactly, and found the unclassified population
within that declared scope to be genuinely zero.** Nothing in this node
disputes what landed; the 24-path migration does what it says.

**The declared scope is two directories. The defect is the repo volume
filling, and the volume is not confined to two directories.**

## Why this is a node and not a note

The constraint is a **measured capability gap**, the same one the predecessor
was created for: `/workspaces/ken` has reached 100% seven times, the reclaim is
a Steward action on someone else's derived data, and each recurrence costs
hours across the fleet in failures that present as linker regressions rather
than as a disk condition. That grounding is unchanged; only the population
changes.

**The strongest single site is the predecessor's own defect statement, on a
file the census never saw.** `crates/ken-interp/src/eval.rs`, grep
`fn rt_parity_root` — it joins `temp_dir()` with a pid and a nanosecond
timestamp, `create_dir_all`s it, returns a bare `PathBuf`, and is called by
nine tests each cleaned only by a trailing statement on the success path. That
is, clause for clause, what the predecessor wrote about
`native_execution_differential.rs`.

**And the tooling written for this defect already names it.** The reaper
comment in `scripts/ken-cargo` names the prefixes it sweeps, `ken-rt-parity-*`
among them, and records 49G across 1901 entries measured 2026-08-05. There are
**two** producers of that prefix: one in `ken-cli/tests/`, in scope and
migrated, and the one above, out of scope and untouched. The infrastructure
built for this node's predecessor names a prefix whose second producer the
census could not see.

**The mechanism is the boundary, not availability.** `tempfile` was added as a
dependency of `ken-elaborator`, `ken-cli` and `ken-runtime` only — verified by
the Steward at `2ca91a3a`, `grep -l tempfile crates/*/Cargo.toml`. It is not a
dependency of `ken-interp`, `ken-host` or `ken-verify`. **The fix's reach and
the census's scope stop at the same line**, which is why neither shows the
other's edge.

## The general form, which is the part worth keeping

The predecessor already took this correction once, on a different axis: its
*"THAT COUNT IS ONE AXIS OF TWO"* note corrected for a leaker under a different
**environment-variable spelling** (`CARGO_TARGET_TMPDIR`, invisible to a
`temp_dir` grep), and its `AC-3` requires both spellings reported separately.

**This is the same failure under a different axis: directory.** Nothing in that
AC ranges over path. A census that declares its own scope and then reports an
unclassified population of zero is stating a true fact about a set it chose —
and a reader takes it as a fact about the hazard.

⇒ **Define a census population by the property that makes a site a hazard, not
by the directories you looked in.** Where a scope bound is genuinely necessary,
the boundary is a deliverable of the census, not a premise of it.

## What keeps this invisible

`scripts/ken-cargo`'s reaper has a 120-minute age gate. It is the compensating
control, and it is why the residue does not surface as a recurrence until the
rate exceeds what the gate reclaims. **Do not read the absence of a recent
recurrence as evidence the population is small.**

## Not this node

No change to what any fixture asserts, no change to runtime, interpreter, host
or verifier behavior, and no reclaim-tooling change. No revisiting of the
predecessor's 40 or 14 — both are correct and this node does not recount them.
