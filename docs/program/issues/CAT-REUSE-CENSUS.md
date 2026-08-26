---
id: CAT-REUSE-CENSUS
title: "Catalog-reuse modernization census — a catalog-wide, source-editing-free inventory that sizes the rework campaign: per catalog package, enumerate (a) definitions now redundant with the expanded prelude, (b) tools reimplemented locally that a sibling module now pub-exports, (c) bottom-up file arrangement, each item risk-tagged (low computational-dup / higher proof-scaffolding). The census that scopes the per-package rework WPs; makes no source edits and closes nothing."
status: ready
owner: foundation
size: M
gate: none
depends_on: [LANG-MOD-PUB-ELIGIBILITY, LANG-MOD-CATALOG-REALIZATION]
blocks: []
github: null
origin: "Steward, 2026-08-26, launching the operator-concurred catalog-reuse modernization campaign (charter docs/program/wp/catalog-reuse-modernization.md). Census-first shape: this inventory sizes the campaign so per-package rework WPs are framed from evidence, not blind. Runs on the foundation ring beside/after the CAT-ORDER-PUB-EXPORT + CAT-GCD-REFACTOR pilot. Steward-filed per COORDINATION section 2."
---

> # CENSUS-ONLY 2026-08-26 — inventory that scopes the campaign; NO source edits, closes nothing

## Objective

Produce a catalog-wide, evidence-grounded inventory of catalog-reuse modernization
debt, so the Steward can frame a bounded, sequenced set of per-package rework WPs
from measured data rather than guesses. This node makes NO source edits, claims no
newly-reworked package, and closes nothing (not the campaign, not any package).

## Scope of the census

Every package under `catalog/packages/`. For EACH package, record the three
orthogonal observations below. The pilot package `Gcd` (and its `Order`
prerequisite) is included as a worked example / calibration row, not excluded.

## Deliverables

- D0 — the per-package census. For each package, a row (or small record) with:
  1. **Prelude redundancy**: local definitions that now duplicate a prelude member
     (the expanded `Nat` floor and friends). Name the local def and the prelude
     member it duplicates, grounded by the exact canonical identity — never by
     name-spelling coincidence.
  2. **Sibling reimplementation**: local tools that a sibling catalog module now
     `pub`-exports (e.g. `add`/`mul` from `Data.Numeric.Nat.Arithmetic`,
     `leq_nat`/`sub` from `Data.Numeric.Nat.Order`). Name the local def, the
     exporting module, and whether that module currently `pub`-exports the tool
     (if NOT, flag the missing-export prerequisite, as `Order` needed
     [[CAT-ORDER-PUB-EXPORT]]).
  3. **Arrangement**: whether the file is bottom-up (fundamentals first) vs
     top-down (headline result first), a boolean plus a one-line note.
  Each item in (1) and (2) carries a **risk tag**: `low` (computational-tool
  duplicate — replace with an import / prelude use, no proof change) or `higher`
  (removing the item touches proof scaffolding or changes proof obligations —
  soundness-sensitive, per-item decision required).
- D1 — a rollup: counts per axis, the set of missing-export prerequisites
  (sibling modules that must `pub`-export before a consumer can reuse), and a
  proposed grouping of the low-risk work into a small number of per-package rework
  WPs for the Steward to sequence. The `higher`-risk items are listed but NOT
  grouped into WPs — they await an explicit per-item operator/Steward decision.

## Acceptance criteria

- AC-1 (complete coverage) — every package under `catalog/packages/` appears in
  the census exactly once; the population is the enumerated directory, and the
  count is stated (no silent sampling or truncation).
- AC-2 (grounded items) — each prelude-redundancy and sibling-reimplementation
  item names the exact canonical identity / exporting module it duplicates, not a
  name-spelling match; a claimed duplicate is backed by identity evidence.
- AC-3 (risk-tagged) — every (1)/(2) item carries a `low`/`higher` risk tag per
  the charter's depth policy; `higher` items name why (which proof obligation).
- AC-4 (prerequisites surfaced) — every sibling-reuse item whose exporting module
  does NOT yet `pub`-export the tool is flagged as a missing-export prerequisite
  (the [[CAT-ORDER-PUB-EXPORT]] pattern), so the Steward sequences the prerequisite
  before the consumer.
- AC-5 (census-only) — NO file under `catalog/packages/` is edited; no package is
  claimed reworked; nothing is closed. The deliverable is the inventory + rollup.
- AC-NO-REGRESSION — none applicable (no source change); the census document
  builds/renders.

## Reviewers

foundation-qa (coverage is complete and counted; items are identity-grounded not
name-matched; risk tags and prerequisites are correct) + conformance-validator
(the census axes match the catalog implementation standard). No Architect review
unless the census itself surfaces a design gap.

## Capability tier

T1 — the census must ground each duplicate by canonical identity (not spelling)
and judge proof-scaffolding risk, which is reasoning over the catalog surface, not
mechanical enumeration. Size M.

## Sequencing

Lane-3 (foundation), the catalog-reuse modernization campaign's scoping step
(charter `docs/program/wp/catalog-reuse-modernization.md`). Runs on the single
foundation ring after (or interleaved with) the `CAT-ORDER-PUB-EXPORT` +
[[CAT-GCD-REFACTOR]] pilot — the pilot proves the per-package recipe, this census
sizes its application. Its D1 rollup is the input from which the Steward frames the
per-package rework WPs.
