---
id: DOC-COMPILER-GUIDE-STYLE-PASS
title: "First test of the new library-style AI-pattern guidance: review and revise the compiler guide (library/guide/compiler/, 8 files) applying the DOC-LIBRARY-STYLE-AI-PATTERNS heuristics, and report whether the writing improved. Standard doc-ring workflow (doc-author revises, Librarian QA)."
status: draft
owner: doc
size: M
gate: none
depends_on: [DOC-LIBRARY-STYLE-AI-PATTERNS]
blocks: []
github: null
tier: T2
origin: "Operator ruling (Pat, 2026-09-13): 'Then kick off the doc ring on a program to review the compiler guide as a first test of the new guidance to see if it improves the writing. Use the standard doc ring workflow for the revisions.' Sequenced AFTER DOC-LIBRARY-STYLE-AI-PATTERNS (the guidance is the input). The compiler guide is chosen as the first test case; the outcome informs whether to run the same pass over the rest of library/guide/."
---

# Objective

Apply the AI-pattern prose-quality guidance from
`DOC-LIBRARY-STYLE-AI-PATTERNS` to the compiler guide as its **first test**,
and report whether it improves the writing. The compiler guide is the eight
files under `library/guide/compiler/`:

- `README.md`, `reading-workflow.md`, `front-end.md`, `kernel.md`,
  `interpreter-and-values.md`, `native-backend.md`, `artifacts-and-erasure.md`,
  `validation-and-limits.md` (678 lines total, measured at `origin/main`
  `aa1f3267d`; re-measure at the cut).

# How

Standard doc-ring workflow: doc-leader frames the team WP, doc-author revises
under the new guidance, Librarian QA (the doc team's QA and as-built authority),
Steward routes to the lieutenant. This is a `library/` change, so the Librarian
is the reviewer (AGENTS.md / COORDINATION 8a); the doc track runs concurrently
with the build lanes.

Revise for the heuristics, not by mechanical substitution. The **counterweight
governs**: the compiler guide legitimately describes refusals, trust boundaries,
and distinct pipeline stages -- keep every negative that names a specific
operation/condition/consequence; only cut scaffolding that substitutes for
explanation. Preserve every technical fact, cross-reference, and the guide's
`reading-workflow` navigation structure; do not vary precise spec/impl terms for
style.

# Deliverables

- Revised `library/guide/compiler/*.md` (the eight files), edited only where the
  guidance genuinely improves clarity; unchanged where the prose is already
  precise.
- A short outcome note (in the WP thread or a brief report) stating whether the
  guidance improved the writing and where it did not apply cleanly -- this is the
  test result that decides whether to run the same pass over the rest of
  `library/guide/`.

# Acceptance criteria

- Every revised passage still states the same technical content; no fact,
  refusal, boundary, or cross-reference is lost or weakened.
- Changes trace to a specific guidance heuristic; no change is stylistic drift
  against the guidance (e.g. no term-variation introduced).
- The guide's structure and navigation (`reading-workflow.md`) remain coherent.
- The outcome note answers the test question: did the guidance improve the
  compiler guide's writing, and is it ready to apply corpus-wide?

# Not this node

- Revising `library/guide/` beyond `compiler/` -- gated on this test's outcome.
- Amending the guidance itself (that is `DOC-LIBRARY-STYLE-AI-PATTERNS`).
- Any `catalog/`, `spec/`, or `crates/` change.

# Sizing / tier

Size M, tier T2. A bounded, judgment-guided revision of eight short guide files
under an established workflow; the reasoning is in the guidance node, the
application is careful editing.
