---
id: DOC-LIBRARY-STYLE-AI-PATTERNS
title: "Draft AI-pattern prose-quality guidance for library/ documentation into the library-style authoring skill plus a doc-author review checklist -- judgment heuristics (never an authorship test) that require every negative to name the specific operation, condition, and consequence, oriented to BOTH revising current docs and future authoring. Grounded in PRINCIPLES #2/#8/#14. Librarian-authored per operator ruling."
status: ready
owner: doc
size: S
gate: none
depends_on: []
blocks: [DOC-COMPILER-GUIDE-STYLE-PASS]
github: null
tier: T2
origin: "Librarian research note (evt_30kr0r0n3c4t8) + operator ruling (Pat, 2026-09-13): 'The librarian should draft the guidance with an eye to revising current documentation and future authoring. Then kick off the doc ring on a program to review the compiler guide as a first test of the new guidance.' External surveys converge on structural/rhetorical AI-writing signals but NONE is a reliable provenance detector (Wikipedia 'Signs of AI writing'; arxiv 2510.05136) -- so these are prose-quality heuristics, never an authorship test. Steward-framed; the LIBRARIAN authors this node (operator assignment), Steward reviews/routes (library-style is playbook corpus, AGENTS.md)."
---

# Objective

Draft the AI-pattern prose-quality guidance the Librarian proposed
(`evt_30kr0r0n3c4t8`) into the authoring corpus, in a form usable for **both**
revising current `library/` documentation and guiding future authoring:

- extend the `library-style` authoring skill (`agent/playbooks/tools/library-style.md`)
  with the avoid-list, and
- add a short review checklist to the doc-author overlay so the guidance is
  applied at authoring and review time.

The Librarian authors this node (operator assignment); the Steward reviews and
routes it (the `library-style` skill is playbook corpus).

# What the guidance is (and is not)

These are **prose-quality heuristics applied with judgment, never an authorship
test.** No external signal is a reliable provenance detector, and the guidance
must say so. The 12-point avoid-list and its sources are in the Librarian's note
(`evt_30kr0r0n3c4t8`); its themes:

- Lead with the mechanism or reader consequence, not an importance announcement
  or a heading that restates the first sentence.
- Do not use contrast scaffolding, manufactured symmetry, or summary restatement
  as a substitute for explanation; end a paragraph on the fact.
- Name the component, operation, input, and result; reuse the exact spec/impl
  term rather than varying it for style.
- Name the actual rejecting/handoff stage rather than claiming a smooth pipeline;
  state a limit once, at the boundary where it applies, with its source.
- No totalizing inventory unless it is proved complete; bound the enumeration.

**The counterweight is load-bearing.** Ken legitimately needs explicit refusal
and trust-boundary prose. The rule is NOT "avoid negatives"; it is "every
negative must name the specific operation, condition, and consequence." A
heuristic that would strip a precise refusal is misapplied.

# Why this is grounded

`docs/PRINCIPLES.md` #2 (agents-write / humans-read -- the docs exist for the
human reader, so readability is the product), #8 (be honest about the boundary),
and #14 (nothing required lives in a comment -- state it precisely in the text).
The guidance is these commitments expressed as prose discipline; it is not an
"AI detector" and not an aesthetic preference.

# Deliverables

- An edit to `agent/playbooks/tools/library-style.md` adding the avoid-list as
  judgment heuristics with the counterweight stated up front, oriented to revision
  and authoring alike.
- A short doc-author overlay review checklist (the operative subset a reviewer
  runs on a draft).
- Both cross-reference PRINCIPLES #2/#8/#14 and state the "not an authorship test"
  limitation explicitly.

# Acceptance criteria

- The `library-style` skill states each heuristic with the "name the specific
  operation, condition, consequence" counterweight, and explicitly disclaims any
  authorship/provenance-detection reading.
- The doc-author checklist is short enough to run per-draft and maps to the skill.
- The guidance is heuristics, not mechanical lint: nothing in it would force the
  removal of a precise refusal or trust-boundary statement.

# Not this node

- Revising any actual `library/` document -- the first application is
  `DOC-COMPILER-GUIDE-STYLE-PASS`, and further per-guide passes are their own
  nodes.
- Any authorship-detection tooling or claim.

# Sizing / tier

Size S, tier T2. Bounded authoring of guidance the Librarian already drafted in
note form into the skill + checklist; reviewed by the Steward (playbook corpus).
