---
id: CI-ROW-CLAIM-COMMENT-FORM
title: "verify-row-claims extracts only from /// doc comments, so a row claim written with // is invisible to it -- two false soundness certificates survive on main in exactly that form"
status: merged
owner: verify
size: S
gate: none
depends_on: [CI-L1-EXECUTING-COVER]
blocks: []
github: null
origin: "Adversary finding evt_3pwk4j7nspvb6 on origin/main 53c09f9b, hunting the CI-L1-EXECUTING-COVER merge against a lead the Steward supplied in its merge notification (evt_51ethef036tv6): that the claim extractor is a parser over comments, so a claim it cannot parse is indistinguishable from a claim that does not exist. Both phantom ids independently re-measured by the Steward at 53c09f9b. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> ## Frame: `docs/program/wp/CI-ROW-CLAIM-COMMENT-FORM.md`. `ready`, shovel-ready.
>
> Its dependency `CI-L1-EXECUTING-COVER` is **merged** (`bfac3f6f`, PR #1776).
> The census this node needs is **already in the frame's §2** rather than owed
> as a first deliverable.

## The defect in one line

`scripts/ci-ignored-sweep.py verify-row-claims` governs claims attached to Rust
`#[test]` functions, but extracts only from `///` doc comments. **Three claims
are written with `//` and two of them resolve to zero headings anywhere under
`conformance/`.** Both carry `(soundness)`, the strongest marker in that
vocabulary.

| line in `crates/ken-interp/src/eval.rs` | id | resolves |
|---|---|---|
| `:8451` | `surface/numbers/f1-store-roundtrip-above-i128-byte-identical` | 0 |
| `:8497` | `surface/numbers/f1-dedup-content-address-stable-across-paths` | 0 |
| `:8519` | `surface/numbers/f1-zero-and-sign-canonical` | 1 — valid |

## Why this is not a regression in the merge that landed the checker

**The `///` path is complete.** Measured on the claim form: `///` = 29 —
exactly the count `CI-L1-EXECUTING-COVER` delivered and reported — `//!` = 0,
`//` = 3. **This is a population gap, not a parser bug**, and the checker did
its job on the population it was given. The gate is one merge old and is
already finding real defects at a rate of five, then two.

**The lead was supplied and answered.** The Steward's merge notification told
the Adversary that an unparsed claim is indistinguishable from an absent one.
That is exactly what it found, which is the notification working as designed
rather than a lucky hit.

## Two traps recorded so nobody re-derives them

**The `//!` hypothesis is dead.** A census matching `surface/[a-z0-9]` returned
157 mentions of which 29 were `//!`, which reads as an uncounted module-level
claim population — the file-header certifying form that got
`CI-ASSERTIONLESS-L1` rejected three times. **Those 29 are file-path
citations, not row ids.** The Adversary caught and corrected this in its own
first pass and reported the correction rather than the finding. A count over a
mixed population is precisely the shape that survived five reviews on
`CI-IGNORED-SWEEP`.

**Heading resolution must be by prefix**, because real headings carry trailing
markers such as `(soundness)` and `[NODE]`. An exact-match resolver reports
false negatives on every tagged row. The landed checker already does this
correctly; the node must not regress it.
