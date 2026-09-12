# WP frame — `DOC-COMPILER-DEEPEN-ARTIFACTS-AND-ERASURE`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only child
of `DOC-COMPILER-DEEPEN` deepens the existing artifacts-and-erasure chapter in
place under the ratified single-page campaign model.

## Objective

Expand `library/guide/compiler/artifacts-and-erasure.md` from its overview into
one coherent, full-length explanatory chapter. It must let a reader trace the
boundary from admitted declarations through `CheckedCorePackage` identity and
validation to erasure into runtime IR, while distinguishing what this boundary
establishes from native lowering, backend behavior, and compiler verification.

**Exit property:** an engineer unfamiliar with this compiler stage can navigate
from the chapter to the exact source that constructs, validates, consumes, or
refuses the artifact; correctly state what is preserved, what is erased, and
what remains blocked; and distinguish artifact provenance from an execution or
native-correctness guarantee. Every claim is anchored to source or spec, and no
claim exceeds what its citation carries.

## Fixed inputs

Measured at `origin/main = aa90da94`:

- The existing page is explanatory and partial, with sources for checked core,
  compiler driver, erasure, and the erasure/runtime-IR specification.
- `checked_core.rs` defines stable checked-core identity and semantic encoding;
  `compiler_driver.rs` emits the package after source elaboration and admission;
  `erasure.rs` validates and consumes that package into runtime IR.
- The normative erasure boundary is `spec/40-runtime/47-erasure-runtime-ir.md`.
  It names preserved metadata, proof-erasure constraints, loud refusals, and
  non-goals that the page must cite rather than restate normatively.
- The single-page front-end and kernel chapters are the approved structural
  exemplar. This candidate adds detail to the existing page only: no subpages,
  no new directory, and no crate edits.

## Deliverables

- Deepen `library/guide/compiler/artifacts-and-erasure.md` in place as one
  coherent, paragraph-led documentation-length page.
- Explain the responsible source-grounded mechanisms: stable symbols and
  semantic versus envelope inputs; compiler-driver package production and
  target/lowerability selection; package and semantic-integrity validation;
  erasure to runtime IR; metadata survival; proof-erasure and unsupported
  lowering refusal; and the boundary to later interpreter/native consumers.
- Update the existing manifest record only as necessary for the sources that
  carry the expanded claims. Keep `kind` and `authority` explanatory,
  availability partial, and the standard validation inventory.
- Preserve adjacent chapter navigation and point language/runtime contracts to
  exact specification sections. Update generated status only within the
  release-point policy; never hand-edit attestations or add a currency gate.

## Acceptance criteria

- **Single-page form:** the existing chapter is expanded in place; no nested
  chapter pages or directory are created.
- **Grounding:** every present-tense implementation claim identifies its
  responsible source symbol or bounded location. Name both producer and
  consumer/refuser where they differ.
- **Boundary:** the page distinguishes checked-core identity/provenance from
  semantic input, package validation from target admission, erasure from native
  lowering, and runtime IR from a proof of execution or native correctness.
  A lowerability or unsupported-erasure refusal is never described as a silent
  fallback.
- **Authority:** the page remains explanatory and partial. Normative erasure,
  runtime-IR, trust, and non-goal claims cite their exact spec sections.
- **Scope:** modify only this page, its existing manifest record if needed, and
  this frame. Do not edit crates, spec, catalog, conformance, CI, agent paths,
  or other compiler-guide chapters.
- **Validation:** run applicable existing documentation validation tools. Do
  not add repository-text tests, revive an inert registry, or run a workspace
  build locally.
- **Release:** the Librarian's exact-SHA as-built review is the sole accuracy
  gate and verifies both claim grounding and one-page coherence. The Steward
  records chapter progress on the campaign umbrella after landing.

## Hard stops

Route to the Steward if a needed detail is not source-grounded, requires a
claim about a planned-only path, conflicts with another candidate on this page
or its manifest record, or needs scope beyond the artifact-to-runtime-IR
boundary. Do not pull interpreter, native-backend, or validation material into
this chapter merely because it is downstream.
