---
id: DOC-COMPILER-DEEPEN
title: "Deepen library/guide/compiler/ one level (paragraph-to-page texture): nest depth pages under each of the 8 overview chapters, executed one chapter at a time in pipeline order, front-end first as the template-setter; explanatory/derived, source-anchored per claim, librarian as-built as the sole accuracy oracle"
status: ready
owner: doc
size: L
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward cut 2026-09-12 on operator direction (Pat, 2026-09-12: the compiler guide is a reasonable overview but lacks depth; take it one level deeper, the texture of each paragraph becoming a page). Operator settled two calls: full program committed now (not a pilot); accuracy review is the librarian as-built pass only (no build-team review). Framed in docs/program/12-documentation-program.md §5. Source spine measured at origin/main = 1bd3a5667: library/guide/compiler/ is 8 pages, ~2263 words, an implemented-path map. Wave 3 (the conceptual guide, under which library/guide/ landed) is complete, so nothing gates this."
---

# Deepen the compiler implementation guide

The frame and its settled decisions live in
`docs/program/12-documentation-program.md §5`. This node is the campaign
umbrella; the doc-leader cuts one child WP per chapter, following the front-end
exemplar.

## What this is

`library/guide/compiler/` is a reasonable overview (8 pages, ~2,263 words, an
implemented-path map: each paragraph points at a `crates/` module with one level
of "what this stage does and where its boundary is"). The operator wants it one
level deeper — the texture of each paragraph becoming a page, not literally
one-to-one; depth varies by topic richness. The full program is committed; the
librarian's as-built pass is the sole accuracy oracle (no build-team review).

## Structure

Nest, do not replace. Each existing page becomes the chapter landing (the
overview, kept), with depth pages beneath it. The corpus convention for the
nested pages is settled on the front-end chapter and then held.

## The eight chapters, in execution order

Pipeline order, one slice at a time (the doc ring does not fan out):

1. `front-end` — the template-setter (see below).
2. `kernel`.
3. `artifacts-and-erasure`.
4. `interpreter-and-values`.
5. `native-backend`.
6. `validation-and-limits`.
7. `reading-workflow` and `README` — re-indexed last; they map the others.

## The front-end chapter is the template-setter

The first child WP expands `front-end.md` into its depth pages AND establishes,
as durable artifacts the other chapters inherit:

- the per-page enrichment template (what a depth page contains: the mechanism it
  explains, the source it anchors to, the invariant/boundary it names, the
  refusal path where one exists, the spec section it derives from);
- the corpus convention (directory shape, manifest records, authority/currency
  labels, whether any page is literate `.ken.md`);
- the source-anchoring discipline applied per claim.

The librarian ratifies the exemplar before the remaining chapters are cut.

## Exit property (per chapter, and for the campaign)

An engineer unfamiliar with a given compiler stage can, from its chapter,
navigate to the exact source that implements it and correctly state what that
stage does, what it guarantees, and what it refuses -- with every claim anchored
to source or spec, and no claim the cited code does not carry.

This is a property with a predicate, not a page count (12-doc-program §4a). The
final clause is the confidently-wrong guard, and the librarian's T1 as-built
review is its oracle.

## Acceptance criteria (campaign-level; each child WP restates its own)

- Every depth page declares its authority class (`explanatory`) and sources in
  the manifest, and introduces no normative language (D1).
- Every claim on a depth page is anchored to a `crates/` source (file plus
  symbol or line) or a spec section, at a recorded revision.
- No depth page states a claim the cited code does not carry (the librarian's
  as-built review is the oracle; a page that reads authoritatively while its
  evidence does not carry it is a defect regardless of prose quality).
- The chapter landing (the former overview page) survives as the index into its
  depth pages; it is not discarded or duplicated.
- Prefer stable invariants/boundaries over volatile line-level detail where the
  two diverge.

## Not this node

- Any normative statement about the compiler (that is `spec/`, not `library/`).
- A standing per-merge currency gate for the deep pages (foreclosed;
  12-doc-program §4b Wave 6). Currency is the librarian's as-built mandate plus
  the release-point policy; attestations drift between release points and that
  is accepted.
- Editing `crates/`. Depth pages read the source to describe it; they do not
  change it, which is what keeps this contention-free with the build lanes.

## Sizing / tier

Size L (campaign; ~8 chapter WPs, each S-M). Authoring is the standing T2
doc-author; review is the T1 librarian as-built. Trip-wire: if the front-end
chapter's review round count runs high (repeated confidently-wrong drafts), that
is the signal to revisit author provisioning for this campaign, per the
sizing-diagnostic pattern -- surfaced to the Steward, not reseated pre-emptively.
