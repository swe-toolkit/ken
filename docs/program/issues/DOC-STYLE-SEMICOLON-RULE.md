---
id: DOC-STYLE-SEMICOLON-RULE
title: "Add a semicolon-restraint heuristic to the library-style AI-pattern guidance (agent/playbooks/tools/library-style.md), then apply a semicolon-only revision to the compiler guide (library/guide/compiler/). Operator finding: semicolons are overused (kernel.md has 8, all prose). Standard doc-ring workflow (doc-author revises, Librarian QA)."
status: ready
owner: doc
size: M
gate: none
depends_on: []
blocks: [DOC-LIBRARY-STYLE-ROLLOUT]
github: null
tier: T2
origin: "Operator ruling (Pat, 2026-09-13): 'The revised doc style guide is a big improvement, with one minor tweak: there is still an overuse of semicolons. For example in library/guide/compiler/kernel.md, there are 8 semicolons. This is unusual even for me, and I use semicolons much more often than average. In general, they are not used, and should be used sparingly, if at all. Typically some conjunctive word is used instead, or an initial phrase in the following sentence that clearly expresses the relation between the two. Ask the librarian to make that adjustment, to revise the use of semicolons in the compiler guide, and then to apply the revised guide to the rest of the library. Standard doc ring workflow.' The compiler-guide style pass (DOC-COMPILER-GUIDE-STYLE-PASS) landed and the operator judged the guidance a big improvement; this node folds the one requested correction and re-applies it to the same corpus before the rollout."
---

# Objective

Two coupled deliverables, in order:

1. **Amend the guidance.** Add a semicolon-restraint heuristic to
   `agent/playbooks/tools/library-style.md` (the library-style AI-pattern
   guidance, 46 lines at `origin/main`; re-measure at the cut). The rule, in the
   operator's own terms:

   > Semicolons should be used sparingly, if at all. In general prose does not
   > use them. Typically a conjunctive word is used instead, or an initial
   > phrase in the following sentence that clearly expresses the relation
   > between the two clauses.

   State it as a heuristic with the same shape as the others in that file: name
   the pattern, give the preferred rewrite (a conjunction, or a leading
   relational phrase that names the relation the semicolon was carrying), and
   carry the counterweight — a semicolon that is genuinely load-bearing and
   whose removal would lose or muddy a relation may stay. The bar is "sparingly,
   if at all," not "zero."

2. **Apply it to the compiler guide.** Do a semicolon-only revision of the eight
   files under `library/guide/compiler/` (`README.md`, `reading-workflow.md`,
   `front-end.md`, `kernel.md`, `interpreter-and-values.md`,
   `native-backend.md`, `artifacts-and-erasure.md`, `validation-and-limits.md`).
   Raw `;` counts at `origin/main` `0cc890f12` (upper bound, includes any in
   code): kernel.md 8, front-end.md 7, validation-and-limits.md 5,
   interpreter-and-values.md 4, README.md 4, native-backend.md 3,
   reading-workflow.md 2, artifacts-and-erasure.md 2 — 35 total. kernel.md's 8
   are confirmed all prose.

# How

Standard doc-ring workflow: doc-leader frames the team WP, doc-author revises,
Librarian QA (the doc team's QA and as-built authority; the operator named the
Librarian as the owner of this correction), Steward routes to the lieutenant.
This touches `agent/` and `library/`, not `crates/`, so it runs on the
concurrent doc track and the Librarian is the reviewer.

This is a semicolon-only pass, not a fresh full style pass. Replace each prose
semicolon per the rule: a conjunction (and, but, so, because, while, ...) or a
sentence break with a leading phrase that names the relation the semicolon
implied. Do not reflow paragraphs, re-word beyond the join, or vary precise
spec/impl terms. **Leave every semicolon inside a code fence, inline code span,
or Ken example untouched** — those are code or data, exactly as the wrap rule
exempts fences.

# Deliverables

- `agent/playbooks/tools/library-style.md` amended with the semicolon heuristic
  (and the one-line pointer in `agent/teams/doc/implementer.md` updated if it
  enumerates the heuristics).
- The eight `library/guide/compiler/*.md` files revised so prose semicolons are
  replaced per the rule, code/inline semicolons untouched.

# Acceptance criteria

- The guidance file states the semicolon rule with a preferred-rewrite pattern
  and an explicit counterweight for a genuinely load-bearing semicolon.
- Every prose semicolon removed from the compiler guide is replaced by a
  conjunction or a leading relational phrase that preserves the same meaning and
  relation; no technical fact, refusal, boundary, or cross-reference is lost.
- No code-fence, inline-code, or Ken-example semicolon is altered.
- No paragraph reflow or term-variation is introduced under cover of the pass.

# Not this node

- Applying the pass to `library/` beyond `compiler/` — that is
  `DOC-LIBRARY-STYLE-ROLLOUT`, which this node blocks.
- Any other prose-quality change to the compiler guide beyond semicolons.
- Any `catalog/`, `spec/`, or `crates/` change.

# Sizing / tier

Size M, tier T2. A bounded, rule-guided edit: one small guidance amendment plus
a mechanical-with-judgment semicolon revision of eight short files. The reasoning
is in the operator's rule; the application is careful editing.
