---
id: DOC-GUIDE-PROOF-COMPLETENESS
title: "The proof-techniques guide does not state that a catalog package is finished only when its proofs are complete, and that computational tests are external to the package (a reader must trust them, not verify them) — the operator principle recorded as PRINCIPLES #16. Weave it into library/guide/proof-techniques.ken.md."
status: ready
owner: doc
size: S
gate: none
depends_on: []
blocks: []
github: null
tier: T2
origin: "Operator directive (Pat, this session): the proof-completeness principle 'should be recorded (if it is not already in PRINCIPLES.md) and should also be specifically mentioned in catalog/guide/.' Recorded as docs/PRINCIPLES.md #16 (landed main 939c1661d). catalog/guide/* are compatibility pointers ('moved', no checked examples) to library/guide/*, so 'catalog/guide/' resolves to the maintained library/guide/proof-techniques.ken.md (doc-track domain). Steward-filed and routed to the doc track (agents cannot create tracked work per COORDINATION section 2)."
---

# Objective

Add to `library/guide/proof-techniques.ken.md` a passage that states the
proof-completeness principle explicitly, in the guide's own voice and with its
own checked examples where the guide format calls for them:

- A catalog package is finished only when its proofs are complete. A tested
  computational implementation is an increment, not a completion.
- A proof is an intrinsic: the kernel re-checks it, so a reader verifies the
  guarantee. A computational test is external to the package: a reader cannot
  verify it, and must instead trust that both the package and its tests were
  implemented correctly. That is strictly inferior, and it weakens the promise
  Ken makes.
- Practical consequence for authors: treat proof completion as an acceptance
  criterion of the package, not a deferrable follow-on; when a proof is
  genuinely deferred (e.g. complexity bounds), say so and name the follow-on.

# Grounding

The canonical statement is `docs/PRINCIPLES.md` #16 (operator ruling this
session). This node does not restate or amend the principle — it makes the
guide, which authors actually read while writing catalog proofs, carry it. The
exemplars are `CAT-PRIORITY-QUEUE-LAWS` (the priority-queue proof follow-on) and
`CAT-REL-CLOSURE-LAWS`; the survey `CAT-PROOF-COMPLETENESS-SURVEY` will find the
rest. Cross-reference PRINCIPLES #16 so the guide points to the charter rather
than duplicating it.

# Deliverables

- An edit to `library/guide/proof-techniques.ken.md` adding the passage above,
  in the guide's established style. If the guide carries checked examples, keep
  any added example checked; if the natural placement is exposition, prose is
  fine — match the surrounding section.
- The compatibility pointer `catalog/guide/proof-techniques.ken.md` need not
  change (it forwards to the maintained file).

# Acceptance criteria

- `library/guide/proof-techniques.ken.md` states the proven-vs-tested
  distinction and the "finished only when proven" rule, and cross-references
  PRINCIPLES #16.
- Any checked example added still checks (the guide's existing build/check path
  stays green); no example is left unchecked in a file that checks its examples.
- The passage is exposition of the existing principle, not a new or modified
  rule. It does not contradict or extend PRINCIPLES #16.

# Not this node

- Amending `docs/PRINCIPLES.md` (#16 already landed).
- The catalog survey or any `*-LAWS` proof authoring.
- Rewriting the guide beyond the added passage.

# Sizing / tier

Size S, tier T2. Weaving a stated principle into an existing guide is bounded
doc authoring reviewed by the Librarian (the doc team's QA), not
reasoning-dense design.
