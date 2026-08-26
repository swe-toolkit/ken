---
id: CAT-REUSE-CENSUS
title: "Catalog-reuse modernization census — a catalog-wide, source-editing-free inventory that sizes the rework campaign: per catalog package, enumerate (a) definitions now redundant with the expanded prelude, (b) tools reimplemented locally that a sibling module now pub-exports, (c) bottom-up file arrangement, each item risk-tagged (low computational-dup / higher proof-scaffolding). The census that scopes the per-package rework WPs; makes no source edits and closes nothing."
status: merged
owner: foundation
size: M
gate: none
depends_on: [LANG-MOD-PUB-ELIGIBILITY, LANG-MOD-CATALOG-REALIZATION]
blocks: []
github: null
origin: "Steward, 2026-08-26, launching the operator-concurred catalog-reuse modernization campaign (charter docs/program/wp/catalog-reuse-modernization.md). Census-first shape: this inventory sizes the campaign so per-package rework WPs are framed from evidence, not blind. Runs on the foundation ring beside/after the CAT-ORDER-PUB-EXPORT + CAT-GCD-REFACTOR pilot. Steward-filed per COORDINATION section 2."
---

> # MERGED 2026-08-26 — census document landed at origin/main `6f00843de`
>
> The doc-only review-evidence candidate `0479ce611` merged (census document
> `docs/program/cat-reuse-census.md`, +379/-0, no checked-in oracle per the
> ruling below). Gate: foundation-qa APPROVE `evt_11dbp1rywhjeq` + CV APPROVE
> `evt_b8tntz94n5c8` on the exact SHA (independent reproduction), resolved merge
> Decision `dec_1sf7k968ssr6e`. Ring retro boundary closed: QA `evt_3qn0br99s5jb4`,
> implementer `evt_63wnfg9tgh3vd`, foundation coordination retro — carry is
> node-internal (classify a pin's subject before hardening a review inventory);
> no follow-up WP triggered. Its D1 rollup is the input for the per-package
> rework WPs the campaign will frame next.
>
> # SCOPE/AUTHORITY RULING 2026-08-26 — census is REVIEW EVIDENCE, no checked-in oracle (Steward)
>
> The recut cycle hard-stopped on a real authority fork (foundation-leader
> evt_5a861a3wae10r): the CV review standard demanded a fail-closed checker
> (`scripts/check-cat-reuse-census.py`) that rejects document-side mutations,
> while Foundation QA — correctly applying the operator Build-QA prohibited-
> subject rule (`agent/playbooks/build/qa-test-design.md` §"never assert facts
> about repository TEXT": occurrence counts, heading inventories, section
> presence, or a hardcoded census of where words appear in `catalog/`/`docs/`
> are BLOCKED, "not weighed against usefulness") — retracted approval because
> that checker IS the prohibited corpus-text oracle (its own motivating example
> is "a milestone census frozen as a permanent test").
>
> Steward ruling, option (a), grounded in that operator rule + PRINCIPLES (small
> auditable TCB, honesty about the boundary, subsume-don't-proliferate): the
> census is a ONE-TIME campaign-sizing artifact and ships as REVIEW EVIDENCE —
> the census DOCUMENT alone, with NO checked-in test oracle. `check-cat-reuse-
> census.py` is NOT merged (it was a review aid; it may be used at review time to
> reproduce, never checked in as a test). The CV's legitimate "support-not-self-
> assert" concern is met by INDEPENDENT REPRODUCTION at review time — exactly the
> enumeration/source-grounding/mutation work QA and CV have already done — not by
> a merged oracle. Any claim the reviewers cannot mechanically ground at review
> time is DISCLOSED in the document as review-only, not printed as checked. This
> also makes the next candidate DOC-ONLY (lieutenant squash, no PR CI). The frame
> never required a checker; its ACs (below) are satisfied by the document. This
> was my framing gap to own; flagged to the operator on return — they may elect
> option (b) a generated living census from a non-document data model if a
> standing artifact is wanted, but (a) fits the sizing purpose.
>
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
     exporting module, and record the prerequisite at BOTH depths — the pilot
     [[CAT-ORDER-PUB-EXPORT]] showed "not `pub` yet" is only the shallow one:
     (i) whether that module currently `pub`-exports the tool (if NOT, flag the
     missing-export prerequisite); and (ii) whether that module ELABORATES
     STANDALONE at Omega with its attached proofs owned locally, or hits a
     standalone-load / attached-proof ownership gap (as `Order` did — it fails at
     `leq_nat::antisym` with a nonlocal conflict at `bool_or::eq_true_of_or`, a
     design-ruling prerequisite the pub-only step cannot reach). Depth (ii) is a
     `higher`-risk prerequisite (routes to spec/Architect), depth (i) is `low`.
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
- AC-4 (prerequisites surfaced, both depths) — every sibling-reuse item whose
  exporting module does NOT yet `pub`-export the tool is flagged as a missing-export
  prerequisite (the [[CAT-ORDER-PUB-EXPORT]] pattern), AND every exporting module
  that does NOT elaborate standalone or carries an attached-proof ownership gap is
  flagged as a `higher`-risk design-ruling prerequisite (the `Order` wall), so the
  Steward sequences each prerequisite — and routes the design-ruling ones to
  spec/Architect — before the consumer.
- AC-5 (census-only) — NO file under `catalog/packages/` is edited; no package is
  claimed reworked; nothing is closed. The deliverable is the inventory + rollup.
- AC-6 (no checked-in oracle — per the ruling above) — the merged deliverable is
  the census DOCUMENT only. NO test/script asserting facts about repository text
  (`scripts/check-cat-reuse-census.py` or any successor) is checked in — that is
  the prohibited corpus-text-oracle subject. The document's claims are verified by
  the reviewers via INDEPENDENT REPRODUCTION at review time; any claim not
  mechanically groundable then is DISCLOSED in the document as review-only rather
  than asserted as checked.
- AC-NO-REGRESSION — none applicable (no source change); the census document
  builds/renders.

## Reviewers

foundation-qa (coverage is complete and counted; items are identity-grounded not
name-matched; risk tags and prerequisites are correct; AND no checked-in
corpus-text oracle ships — the merged artifact is the document, per AC-6) +
conformance-validator (independently REPRODUCES the census's claims at review
time — enumeration, canonical-identity grounding, prerequisite depths — and
confirms the disclosed review-only residual is honest; the CV's verification is
its own reproduction, NOT a demand for a merged fail-closed checker). No
Architect review unless the census itself surfaces a design gap.

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
