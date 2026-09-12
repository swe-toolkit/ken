# WP frame — `DOC-COMPILER-DEEPEN-READING-WORKFLOW`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This final doc-only
child of `DOC-COMPILER-DEEPEN` re-indexes the completed compiler-guide chapters
by deepening the existing reading workflow and compiler-guide README in place.

## Objective

Expand `library/guide/compiler/reading-workflow.md` and
`library/guide/compiler/README.md` into a coherent explanatory entry and
navigation path for the now-deepened single-page compiler chapters. The pages
must help a reader choose a bounded reading path without manufacturing a new
architecture, authority, or validation claim.

**Exit property:** a reader can start from the README, select a stated question
or pipeline path, reach the responsible chapter and source/spec boundary, and
understand that the workflow is navigation rather than a proof of compiler
correctness or a replacement for any chapter's limits.

## Fixed inputs

Measured at `origin/main = 5582af54`:

- Front-end, kernel, artifacts-and-erasure, interpreter-and-values,
  native-backend, and validation-and-limits are each one deepened page.
- The final pair re-indexes those chapters and must remain consistent with their
  availability, authority, source, and limitation statements.
- This work does not create nested pages, revise compiler implementation, or
  turn explanatory navigation into normative specification.

## Deliverables

- Deepen the existing `reading-workflow.md` and `README.md` in place.
- Provide concise source-anchored reading routes for pipeline, runtime/value,
  native-output, and validation-limit questions; preserve exact links to the
  responsible pages and their source/spec anchors.
- Update the existing manifest only if a newly asserted implementation source
  requires coverage. Preserve explanatory/partial authority and navigation.

## Acceptance criteria

- **Form:** modify only the two existing pages, their manifest record if needed,
  and this frame; create no nested pages or directory.
- **Accuracy:** routes and summaries agree with the landed chapter boundaries;
  no summary promotes artifact evidence, comparisons, or navigation into proof.
- **Authority:** pages stay D1 explanatory; link normative claims to the
  existing chapter/spec authority rather than restating them as rules.
- **Scope:** do not edit crates, spec, catalog, conformance, CI, agent paths, or
  other compiler chapters.
- **Validation:** run applicable existing documentation tools only; never a
  local workspace build or repository-text test.
- **Release:** Librarian exact-SHA as-built review is the sole accuracy gate;
  Steward closes the umbrella in its batched campaign update after landing.

## Hard stops

Route to the Steward if re-indexing exposes a contradiction between landed
chapters, requires a new behavioral claim without source grounding, or needs a
scope beyond navigation and explanatory workflow.
