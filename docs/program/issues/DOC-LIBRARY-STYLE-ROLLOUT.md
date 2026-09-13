---
id: DOC-LIBRARY-STYLE-ROLLOUT
title: "Apply the revised library-style AI-pattern guidance (including the new semicolon-restraint rule) to the rest of the library prose beyond the compiler guide. Standard doc-ring workflow (doc-author revises, Librarian QA), sliced into one-turn increments by the doc-leader."
status: draft
owner: doc
size: L
gate: none
depends_on: [DOC-STYLE-SEMICOLON-RULE]
blocks: []
github: null
tier: T2
origin: "Operator ruling (Pat, 2026-09-13): '...then to apply the revised guide to the rest of the library. Standard doc ring workflow.' The operator judged the compiler-guide style pass (DOC-COMPILER-GUIDE-STYLE-PASS) a big improvement, which is the go-ahead for the corpus-wide rollout the compiler-guide node deferred ('Revising library/guide/ beyond compiler/ -- gated on this test's outcome'). Gated here on DOC-STYLE-SEMICOLON-RULE so the rollout carries the corrected guidance."
---

# Objective

Apply the revised library-style AI-pattern guidance
(`agent/playbooks/tools/library-style.md`, including the semicolon-restraint
rule added by `DOC-STYLE-SEMICOLON-RULE`) to the library prose beyond the
compiler guide, which was the first test case.

# Scope and the boundary this node draws

The operator said "the rest of the library." The guidance is prose-quality
guidance, so the target is the **prose documentation** in `library/`, not
structured or code content. Concretely:

- **In scope:** `library/guide/` beyond `compiler/` (the four top-level guide
  files and any other guide areas), and any other genuine prose docs under
  `library/` (READMEs, overviews, explanatory text).
- **Out of scope:** `catalog/` law statements and Ken source, code fences and
  inline code everywhere, generated or tabular content, and any file that is
  data rather than prose.

Raw `;` census at `origin/main` `0cc890f12` (upper bound, includes code):
`library/guide/` 124 across 12 md files (35 in `compiler/`, so ~89 in the rest);
all of `library/` 631 across 98 md files. These are raw counts to size the work,
not a target — the prose subset is smaller, and semicolons are only one of the
guidance heuristics this pass applies.

**Steward note for the operator:** this node reads "the rest of the library" as
the prose docs under `library/`, excluding `catalog/` code and structured
content, because the AI-pattern guidance is about prose. If a wider or narrower
reading was intended, it is a one-line scope adjustment on this node.

# How

Standard doc-ring workflow, on the concurrent doc track: doc-leader frames the
team WP(s), doc-author revises, Librarian QA (the doc team's QA and as-built
authority), Steward routes to the lieutenant. **The doc-leader slices this into
one-turn increments** (per area or per file group) rather than one large WP — a
corpus-wide pass is too big for a single implementer turn (Steward playbook
§4b).

Apply the full revised guidance, not semicolons alone — this is the style pass
the compiler guide received, now run corpus-wide with the corrected guide.
Preserve every technical fact, cross-reference, refusal, and boundary; do not
vary precise spec/impl terms; leave code/inline/Ken examples untouched.

# Deliverables

- The in-scope `library/` prose files revised under the revised guidance, edited
  only where the guidance genuinely improves clarity and unchanged where the
  prose is already precise.
- A short outcome note per increment (or a closing summary) recording where the
  guidance applied cleanly and any area where it did not.

# Acceptance criteria

- Every revised passage states the same technical content; no fact, refusal,
  boundary, or cross-reference is lost or weakened.
- Prose semicolons are handled per `DOC-STYLE-SEMICOLON-RULE`; code/inline/Ken
  semicolons are untouched.
- Changes trace to a specific guidance heuristic; no stylistic drift or
  term-variation against the guidance.
- Out-of-scope content (`catalog/`, code, structured/generated files) is not
  touched.

# Not this node

- Amending the guidance itself (that is `DOC-STYLE-SEMICOLON-RULE` /
  `DOC-LIBRARY-STYLE-AI-PATTERNS`).
- The compiler guide (already covered by `DOC-COMPILER-GUIDE-STYLE-PASS` and the
  semicolon fix in `DOC-STYLE-SEMICOLON-RULE`).
- Any `catalog/`, `spec/`, or `crates/` change.

# Sizing / tier

Size L (corpus-wide), tier T2. Large in aggregate but low in per-increment
reasoning; the doc-leader slices it so each implementer turn is a bounded,
rule-guided revision of one area.
