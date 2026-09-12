# WP frame — `DOC-COMPILER-DEEPEN-FRONT-END-CONSOLIDATION`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only
corrective child of `DOC-COMPILER-DEEPEN` restores the operator's intended
single-page chapter model. It consolidates the already approved front-end
detail; it does not broaden into kernel work.

## Objective

Make `library/guide/compiler/front-end.md` one coherent, full-length
explanatory chapter. Fold into it the source-input, lexing, parsing,
resolution, and elaboration/admission detail currently in its nested pages.
Delete the `library/guide/compiler/front-end/` directory and remove its five
manifest records. The result is the corrected campaign exemplar: one existing
chapter page deepened in place, not a landing page plus a page hierarchy.

**Exit property:** an engineer unfamiliar with Ken's front end can read the
single chapter and navigate to the exact source for input dispatch, lexing,
parsing, resolution, elaboration, and kernel admission; correctly state what
each mechanism does, guarantees, and refuses; and distinguish a stage's
producer from a later stage that consumes or rejects its result. Every claim is
anchored to source or spec, and no claim exceeds what its citation carries.

## Fixed inputs

Measured at `origin/main = feb5831d`:

- The front-end landing page and five nested pages are already librarian
  approved for factual accuracy. Their content is the material to consolidate,
  not a license to add new implementation claims.
- The five existing detail subjects are source input and dispatch, lexing,
  parsing, resolution, and elaboration plus kernel admission.
- The front-end sources are `crates/ken-cli/src/main.rs`,
  `crates/ken-elaborator/src/lib.rs`, `lexer.rs`, `parser.rs`, `resolve.rs`,
  `elab.rs`, and `compiler_driver.rs`, with applicable surface-spec sections.
- The operator's correction settles the structure: detail is in one page per
  chapter, not nested pages or a subdirectory. The kernel hierarchy is a
  separate later consolidation and is outside this candidate.

## Deliverables

- Rewrite `library/guide/compiler/front-end.md` only as needed to integrate the
  already approved detail into a coherent single-page explanation with durable
  section transitions.
- Delete the five nested front-end Markdown pages and their directory.
- Remove exactly the five corresponding `library/manifest.toml` document
  records; expand the surviving front-end record's source list to cover the
  consolidated claims.
- Preserve the explanatory/partial labels, source anchors, and links to the
  compiler's adjacent chapter pages. Replace internal depth-page links with
  in-page navigation or prose transitions.
- Update generated status output only as supported by the existing
  release-point policy. Never hand-edit attestations or install a currency gate.

## Acceptance criteria

- **Single-page form:** `front-end.md` is the only front-end chapter document;
  no `library/guide/compiler/front-end/` directory or manifest record remains.
- **Detail preserved:** all five approved subject areas remain materially
  explained in the surviving page. Consolidation may improve sequence and
  transitions, but it must not delete a mechanism, boundary, refusal, or source
  anchor merely to make the page shorter.
- **Coherence:** the page reads as one documentation-length explanation rather
  than five pasted leaf pages. It has paragraph-led transitions and source
  navigation appropriate to one chapter.
- **Grounding:** every present-tense implementation claim remains tied to the
  responsible source symbol or bounded location. Where producer and consumer
  or refuser differ, name both.
- **Authority:** the surviving record and page remain explanatory and partial;
  normative language rules are attributed to exact specification sections.
- **Scope:** only `library/` and this child frame under `docs/program/wp/` may
  change. Do not touch `kernel.md`, `library/guide/compiler/kernel/`, crates,
  spec, catalog, conformance, CI, or agent paths.
- **Validation:** run applicable existing documentation validation tools. Do
  not add repository-text tests, revive an inert registry, or run a workspace
  build locally.
- **Release:** the Librarian's exact-SHA as-built review is the sole gate. It
  reviews both accuracy preservation and whether the result reads as one
  coherent page. On landing, the Steward records progress on the umbrella.

## Hard stops

Route to the Steward if consolidation cannot preserve a source-grounded claim,
requires an ungrounded new claim, would touch kernel material, or conflicts
with another candidate on the front-end or manifest paths. Do not pre-empt the
separate kernel consolidation.
