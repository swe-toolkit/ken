# WP frame — `DOC-COMPILER-DEEPEN-KERNEL-CONSOLIDATION`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only
corrective child of `DOC-COMPILER-DEEPEN` applies the landed single-page
front-end exemplar to the already-landed kernel hierarchy.

## Objective

Make `library/guide/compiler/kernel.md` one coherent, full-length explanatory
chapter. Fold in the approved detail from its core-terms,
contexts-and-declarations, checking-and-admission, conversion-and-reduction,
and inductive-admission pages. Delete `library/guide/compiler/kernel/` and
remove its five manifest records. The result is one deepened chapter page, not
a landing page plus hierarchy.

**Exit property:** an engineer unfamiliar with Ken's kernel can read the
single chapter and navigate to exact sources for its explicit terms,
environments, checking and admission, conversion and reduction, and inductive
admission; correctly state what each mechanism does, guarantees, and refuses;
and distinguish a producer from any later consumer or refuser. Every claim is
anchored to source or spec, and no claim exceeds what its citation carries.

## Fixed inputs

Measured at `origin/main = cb74d2ca`:

- The kernel landing and five nested kernel pages are already Librarian-approved
  for accuracy. Their content is to be consolidated, not expanded with new
  implementation claims.
- The approved subjects are explicit core terms; contexts and declarations;
  checking and admission; conversion and reduction; and inductive admission.
- Responsible sources include `crates/ken-kernel/src/lib.rs`, `term.rs`,
  `subst.rs`, `env.rs`, `check.rs`, `conv.rs`, and `inductive.rs`, plus the
  applicable kernel and trust specification sections.
- The operator settled the chapter shape: one existing page deepened in place.
  Remaining chapters will follow this corrected exemplar after it lands.

## Deliverables

- Integrate the approved kernel detail into `kernel.md` as one coherent,
  paragraph-led chapter with durable section transitions.
- Delete only the five nested kernel pages and their directory.
- Remove exactly their five manifest records and expand the surviving kernel
  record's sources to carry every consolidated claim.
- Preserve explanatory/partial status, exact source anchoring, and links to
  adjacent compiler chapters. Replace depth-page links with in-page navigation
  or prose transitions.
- Update generated status only within the release-point policy. Never
  hand-edit attestations or install a standing currency gate.

## Acceptance criteria

- **Single-page form:** `kernel.md` is the only kernel chapter document; no
  `library/guide/compiler/kernel/` directory or manifest record remains.
- **Detail preserved:** all five approved mechanisms, boundaries, refusal
  paths, and source anchors remain materially explained; consolidation must not
  discard them merely to shorten the chapter.
- **Coherence:** the result reads as one full documentation-length chapter,
  not five pasted leaves.
- **Grounding:** every present-tense claim is tied to its responsible source
  symbol or bounded location; where producer and consumer/refuser differ, name
  both.
- **Authority:** the surviving page and record remain explanatory and partial;
  normative syntax, checking, conversion, and trust rules cite exact spec
  sections.
- **Scope:** only `library/` and this frame may change. Do not edit front-end,
  later compiler chapters, crates, spec, catalog, conformance, CI, or agent
  paths.
- **Validation:** run applicable existing documentation tools. Do not add
  repository-text tests, revive an inert registry, or run a workspace build.
- **Release:** the Librarian's exact-SHA as-built review is the sole gate and
  verifies accuracy preservation plus single-page coherence. The Steward then
  records umbrella progress.

## Hard stops

Route to the Steward if consolidation cannot preserve a source-grounded claim,
requires an ungrounded new claim, touches another chapter, or conflicts with
another candidate on the kernel or manifest paths. Do not cut later chapters
until this corrected kernel exemplar lands.
