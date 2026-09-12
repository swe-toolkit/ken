---
id: DOC-COMPILER-DEEPEN
title: "Deepen library/guide/compiler/ one level: elaborate each of the 8 overview chapters IN PLACE as a single fuller page (operator correction 2026-09-12: detail, not structure -- no nested depth pages/subdirectories), executed one chapter at a time in pipeline order, the consolidated front-end.md as the exemplar; explanatory/derived, source-anchored per claim, librarian as-built as the sole accuracy oracle"
status: active
owner: doc
size: L
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward cut 2026-09-12 on operator direction (Pat, 2026-09-12: the compiler guide is a reasonable overview but lacks depth; take it one level deeper, the texture of each paragraph becoming a page). Operator settled two calls: full program committed now (not a pilot); accuracy review is the librarian as-built pass only (no build-team review). Framed in docs/program/12-documentation-program.md §5. Source spine measured at origin/main = 1bd3a5667: library/guide/compiler/ is 8 pages, ~2263 words, an implemented-path map. Wave 3 (the conceptual guide, under which library/guide/ landed) is complete, so nothing gates this. OPERATOR CORRECTION 2026-09-12: 'paragraph expanded to page' meant elaborating DETAIL, not splitting STRUCTURE. Each chapter is deepened IN PLACE as one page; NO nested depth pages or per-chapter subdirectories. The front-end chapter's initial nested cut (1b9afcec0) was consolidated back to a single front-end.md (DOC-COMPILER-DEEPEN-FRONT-END-CONSOLIDATION, landed cb74d2cae) and is now the exemplar; the kernel chapter's nested cut (d51d99d2) awaits the same consolidation."
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
level deeper — each chapter elaborated in place to the length and detail people
usually expect of a documentation page (operator correction 2026-09-12: this is
about DETAIL, not structure — one fuller page per chapter, not a page split into
nested sub-pages); depth varies by topic richness. The full program is committed;
the librarian's as-built pass is the sole accuracy oracle (no build-team review).

## Structure

Deepen in place, as one page. Each chapter stays a single
`library/guide/compiler/<chapter>.md` and grows from its ~280-word overview into
a full page — no landing-plus-depth split, no per-chapter subdirectory, no
per-page manifest records for nested pages. (Operator correction 2026-09-12: the
earlier "nest depth pages beneath a kept landing" design was a mis-scope of
Pat's instruction, which was about detail, not structure.) The single-page shape
is settled on the consolidated front-end chapter and then held.

## The eight chapters, in execution order

Pipeline order, one slice at a time (the doc ring does not fan out):

1. `front-end` — the exemplar (see below). Initial nested cut landed
   `1b9afcec0` (PR #3538), then CONSOLIDATED to a single `front-end.md`
   (DOC-COMPILER-DEEPEN-FRONT-END-CONSOLIDATION, landed `cb74d2cae`); the
   single-page shape is ratified.
2. `kernel` — nested cut landed `d51d99d2`; its single-page consolidation is the
   next WP to cut (fold the 5 kernel/ sub-pages into `kernel.md`).
3. `artifacts-and-erasure`.
4. `interpreter-and-values`.
5. `native-backend`.
6. `validation-and-limits`.
7. `reading-workflow` and `README` — re-indexed last; they map the others.

## Progress

Each chapter lands as a `docs/program/wp/DOC-COMPILER-DEEPEN-<CHAPTER>.md` frame
(not a per-chapter `issues/` node); completion is recorded here on the umbrella,
which stays `active` until every chapter has landed, then `closed`. The standard
per-node status flip does not apply to the child frames — this section is their
status carrier.

- front-end (initial nested cut): LANDED (`1b9afcec0`). Librarian as-built review
  took one reject (a false resolver-stage refusal claim in `resolution.md`,
  corrected to the `RCon`-to-elaborator handoff) then APPROVE on the respin — a
  clean round count, not the high-round signal the sizing trip-wire watches for.
- front-end CONSOLIDATION: LANDED (`cb74d2cae`, PR #3546). Under the operator
  correction, the 5 nested `front-end/` sub-pages were folded back into a single
  `front-end.md` (subdirectory deleted, manifest updated), Librarian sole
  as-built APPROVE `evt_796cvne2qevn6`. This consolidated single page is the
  exemplar the remaining chapters inherit.
- kernel: nested cut LANDED (`d51d99d2`); awaits its single-page consolidation WP
  (fold the 5 `kernel/` sub-pages into `kernel.md`), to be cut against current
  main following the front-end pattern.

## The consolidated front-end chapter is the exemplar

The consolidated `front-end.md` (single page) establishes, as durable artifacts
the other chapters inherit:

- the per-page enrichment depth (what a full chapter page covers: each mechanism
  it explains, the source it anchors to, the invariant/boundary it names, the
  refusal path where one exists, the spec section it derives from) — all within
  ONE page, not split across nested pages;
- the corpus convention (single `<chapter>.md`, its manifest record,
  authority/currency labels, whether the page is literate `.ken.md`);
- the source-anchoring discipline applied per claim.

The librarian ratifies the exemplar; the remaining chapters follow its
single-page shape and depth.

## Exit property (per chapter, and for the campaign)

An engineer unfamiliar with a given compiler stage can, from its chapter,
navigate to the exact source that implements it and correctly state what that
stage does, what it guarantees, and what it refuses -- with every claim anchored
to source or spec, and no claim the cited code does not carry.

This is a property with a predicate, not a page count (12-doc-program §4a). The
final clause is the confidently-wrong guard, and the librarian's T1 as-built
review is its oracle.

## Acceptance criteria (campaign-level; each child WP restates its own)

- Every deepened chapter page declares its authority class (`explanatory`) and
  sources in the manifest, and introduces no normative language (D1).
- Every claim on a deepened chapter page is anchored to a `crates/` source (file
  plus symbol or line) or a spec section, at a recorded revision.
- No deepened chapter page states a claim the cited code does not carry (the
  librarian's
  as-built review is the oracle; a page that reads authoritatively while its
  evidence does not carry it is a defect regardless of prose quality).
- Each chapter remains a single `<chapter>.md`: the former overview is deepened
  in place, not discarded, duplicated, or split into nested depth pages.
- Prefer stable invariants/boundaries over volatile line-level detail where the
  two diverge.

## Not this node

- Any normative statement about the compiler (that is `spec/`, not `library/`).
- A standing per-merge currency gate for the deep pages (foreclosed;
  12-doc-program §4b Wave 6). Currency is the librarian's as-built mandate plus
  the release-point policy; attestations drift between release points and that
  is accepted.
- Editing `crates/`. The deepened pages read the source to describe it; they do
  not change it, which is what keeps this contention-free with the build lanes.

## Sizing / tier

Size L (campaign; ~8 chapter WPs, each S-M). Authoring is the standing T2
doc-author; review is the T1 librarian as-built. Trip-wire: if the front-end
chapter's review round count runs high (repeated confidently-wrong drafts), that
is the signal to revisit author provisioning for this campaign, per the
sizing-diagnostic pattern -- surfaced to the Steward, not reseated pre-emptively.
