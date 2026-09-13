# WP frame — `DOC-GUIDE-PROOF-COMPLETENESS`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only WP
implements the operator directive to mention proof completeness in the
maintained proof-techniques guide; `catalog/guide/` remains compatibility
pointers and is not edited.

## Objective

Weave the proof-completeness principle into `library/guide/proof-techniques.ken.md`
in that guide's voice. A catalog package is finished only when its proofs are
complete. A proof is an intrinsic the kernel re-checks, so a reader verifies the
guarantee; a computational test is external, requiring trust that both package
and tests were implemented correctly, and is strictly inferior. Treat proof
completion as acceptance; name an explicit follow-on when a proof is genuinely
deferred.

Cross-reference `docs/PRINCIPLES.md` #16 rather than restating or amending it.

## Acceptance criteria

- Modify only `library/guide/proof-techniques.ken.md` and this frame; update an
  existing manifest record only if source coverage requires it.
- Keep the guide explanatory and source-grounded; do not edit `catalog/guide/`,
  crates, spec, tests, CI, or PRINCIPLES #16.
- Any added example must be checked using the applicable existing guide tooling.
- Run applicable targeted documentation validation, never a workspace build.
- Librarian exact-SHA as-built review is the sole gate; Steward publishes after
  acceptance.

## Hard stops

Escalate if the maintained guide lacks a suitable location without a structural
rewrite, if the directive conflicts with the guide's source-grounded content, or
if a checked example requires scope beyond this WP.
