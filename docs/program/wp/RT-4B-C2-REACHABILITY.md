# RT-4B-C2-REACHABILITY — can C2 reach the D2f observation at all, and at what cost

**Owner: runtime. Size: S. Gate: none.**
**This node REPORTS. It builds nothing.**

**Base: re-derive `origin/main` at cut time**, after `81f46822`
(`RT-4B-ENUMERATION-INPUT-SIZE`) lands.

## Fixed inputs

| fact | site |
|---|---|
| the observation, and its `#[cfg(test)]` gate | `lowering/core.rs:540-560`, filled at `:2182-2194` |
| the real witness, and the crate it drives through | `crates/ken-elaborator/tests/r3_c2_source_mixed_branch.rs:84`, via `compile_native_program_sources` |
| the dependency direction | `ken-elaborator` depends on `ken-runtime`; **not** the reverse |
| what the last increment actually measured | D2j fixtures — `arrived_empty` iterates `[D2jCause::ExactSuffix, D2jCause::CallIdentity]`, both **perturbed so fusion cannot form**; the artifact-identity control drives `d2j_checked_fixture_under(D2jCause::Exact)` |
| the unperturbed behaviour, which is not in doubt | the same assertion: three positive rows each resolve **one key and one descriptor** |

## D1 — answer the question

**Can a witness driving through `ken-elaborator` reach the `D2f` observation?**

**Answer with exactly one of the four rows in the licensing table**, naming the
mechanism where there is one. **Not "yes" or "no" — the truth here has four
shapes and two of them are "reachable".** If several mechanisms work, **name and
rank them**; the choice is a Steward/Architect call, not the implementer's.

## D2 — cost it in the terms that decide it

For each viable mechanism:

- **Production footprint**, if any. Zero is the expected answer and anything
  else is a finding.
- **Whether it survives the 4b envelope** — no second observer, no production
  API. **If the only achievable mechanism breaks one of those, that is the
  finding**, and it goes to the Architect rather than being absorbed.
- **Whether any produced artifact changes.** It must not.

## Acceptance criteria

- **AC-1 — the answer names exactly one of the FOUR rows in the licensing
  table below**, not a plan and not a binary. **"Reachable" alone is not an
  answer** — it does not distinguish same-build from differs-from-production,
  and those license different next steps.
- **AC-2 — NOTHING IS BUILT.** No feature gate, no sink, no re-gating, no
  counter. **A candidate that lands a mechanism has answered a question nobody
  has asked yet**, and the ranking call was not the implementer's to make.
- **AC-3 — if the answer is no, the reason is stated at the level of the
  obstacle**, not as "I could not find a way." `#[cfg(test)]` not crossing a
  crate boundary is a fact about the language; exhausting the options is a fact
  about the search. **Say which one you are reporting.**
- **AC-4 — the report names what it did NOT try**, so the boundary is honest
  and the next reader does not re-run it.

## Pre-stated licensing — read BEFORE reporting

**There are FOUR answers, not two. Reporting a binary here would report a
shape the truth does not have.**

| answer | what it licenses |
|---|---|
| **reachable, SAME build** | `RT-4B-UNIQUENESS-GATE-REACH` re-points at C2 and downstream follows. **Nothing about the planner** — a measurement becomes possible, not any result from it. |
| **reachable, but the REACHING BUILD DIFFERS FROM PRODUCTION** | **Re-pointing is CONDITIONAL on an enabled-vs-disabled artifact-identity proof, and that proof IS the increment — not a footnote to it.** |
| **not reachable — LANGUAGE** | **4b's status becomes BLOCKED ON CROSS-CRATE GATE EXPRESSIBILITY, in those words.** Not "awaiting a measurement". No count is available to run. |
| **not reachable — SEARCH EXHAUSTED** | A search result, **not a wall.** Say what was searched, so the next reader does not re-run it. |

> ### THE SECOND ROW IS THE ONE THIS NODE EXISTS TO CATCH
>
> **A Cargo feature gate is not blocked by the language. It is the standard
> mechanism and you will find it quickly.** But a control that enables it
> observes **a compilation production never performs.**
>
> That is neither "not reachable" nor "reachable at acceptable cost" — it is
> reachable **conditional on proving the enabled and disabled builds produce
> identical artifacts**, which is the constraint on the original 4b envelope and
> **the one most likely to be quietly skipped when the answer feels like a clean
> yes.**
>
> ⇒ **This is the answer where an honest report and an over-claim look
> identical.** *"Yes, a feature works"* is true, and it hides that the thing
> which would then be measured is not the thing production compiles. **If your
> answer is the second row, say so in those words** — do not report it as the
> first.

> **NO row says anything about whether the planner fuses for C2.** That
> is the question this makes askable; it is not this node's output. **The
> unperturbed D2j rows already show the planner fusing** — so "does it fuse at
> all" was never the open question, and this node must not be reported as
> settling it.

## Banned scope

- Building any mechanism (see AC-2).
- Counting, attributing, or measuring the planner's behaviour on any witness.
- Relaxing the 4b envelope. Gates 5 and 6 held; production unarmed.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair.

## Hard stops — return to the Steward

- **The only achievable mechanism requires a production API or a second
  observer.**
- **A mechanism would change a produced artifact.**

## Sequencing and contention

Runtime, one lane, after `81f46822` lands. Reads `lowering/core.rs`,
`planning/static_transition.rs` and `crates/ken-elaborator/tests/`. **It should
not need to modify any of them** — if it does, that is AC-2's hard stop.

## Why this node exists at all, stated for a cold reader

The 4b arc has now produced **six claims wider than their instruments**, and the
most recent was the Steward's own: a measurement taken on comparators
**perturbed so fusion cannot form** was published as evidence that the planner
formed nothing. **The instrument was honest; it was pointed at a negative
control.**

The reason it was pointed there is this node's subject. **Answer the
reachability question before pointing anything else.**
