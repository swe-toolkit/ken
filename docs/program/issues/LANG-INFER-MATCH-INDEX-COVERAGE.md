---
id: LANG-INFER-MATCH-INDEX-COVERAGE
title: "An unannotated match on an indexed family never discharges an index-impossible constructor, so an empty VNil bucket fails coverage; the matrix path must use the dependent path's one authority for index impossibility"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-GENERATED-J-PROOF-ASCRIPTION]
blocks: []
github: null
origin: "Architect AC-0 disposition evt_6e58trrr1bnfx on LANG-REFINED-SIBLING-MATCH-TAIL (component 2, mechanism measured). Inherits the operator's 2026-09-30 Language scheduling ('concur with recs'). Steward-filed per COORDINATION section 2."
---

# One authority for index impossibility

## Objective

An inferred `match` on an indexed family discharges an index-impossible
constructor exactly as the dependent path does.

## Settled inputs (Architect `evt_6e58trrr1bnfx`)

- **The failing path.** An unannotated `match` goes through `infer_match`
  (`elab.rs:18028`) to `build_ctor_buckets` (`:17500`). That path never
  discharges an impossible constructor, and fails on the empty VNil bucket
  even with no sibling (the direct control: depth 3, 0 frames).
- **The passing path.** With an expected type, the same source takes the
  dependent path. There `synthesize_omitted_index_method` (`:8177`) proves
  Bottom from the premise, and the match checks.

## Deliverable

The matrix path discharges an empty bucket with
`synthesize_omitted_index_method`'s proof, with no second disjointness rule.
Its motive carries the index premise domains.

## Acceptance

- **AC-0 (check 4; no build).** Write the obligation in the code's own
  vocabulary: the `premise_domains` the matrix path would pass, and the
  motive shape that carries them. If that needs a new motive plane rather
  than the dependent path's existing premise construction, stop to the
  Architect.
- **AC-1.**
  - The direct inferred control checks.
  - `lookup_zip_with` either checks or fails only at a
    `LANG-SIBLING-GOAL-REFINEMENT` site. Name which.
- **AC-2 (negative pair, one input).** A genuinely reachable missing
  constructor still raises `ExhaustivenessError`.

## Stop conditions

- Any kernel or trust change.
- A second index-impossibility rule beside the dependent path's.
