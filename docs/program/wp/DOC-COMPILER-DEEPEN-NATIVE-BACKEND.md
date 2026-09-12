# WP frame — `DOC-COMPILER-DEEPEN-NATIVE-BACKEND`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only child
of `DOC-COMPILER-DEEPEN` deepens the existing native-backend chapter in place
under the ratified single-page campaign model.

## Objective

Expand `library/guide/compiler/native-backend.md` into one coherent,
full-length explanatory chapter. A reader must be able to trace the native
backend from an admitted runtime program through supported-subset checks,
Cranelift object production, and optional executable packaging, while knowing
which reported evidence is artifact-local and which intended lanes remain
unavailable.

**Exit property:** a reader can navigate from every implementation claim to its
responsible backend, artifact, or packaging source; distinguish runtime-IR
lowering from native observation and output packaging; and avoid reading an
object, verifier, smoke run, or executable artifact as kernel evidence or a
whole-compiler correctness proof.

## Fixed inputs

Measured at `origin/main = 98dd8517`:

- The existing explanatory/partial chapter names the native backend,
  artifact API, object-linker packaging, compiler program, and validation
  chapter as its authority boundary.
- `cranelift_backend.rs` owns runtime-IR lowering and package-backed admission;
  the artifact API records object-level results; object-linker packaging owns
  the starter object-to-executable path.
- The currently implemented route is bounded: native library/ABI, interop,
  cross-package linking, host-effect/FFI execution, translation validation,
  and whole-compiler proof remain unavailable or out of scope.
- The preceding chapters are the approved one-page structural exemplar. This
  candidate expands the existing page only: no subpages, directory, or crate
  changes.

## Deliverables

- Deepen `library/guide/compiler/native-backend.md` in place as one
  paragraph-led, documentation-length chapter.
- Explain source-grounded package admission, runtime-IR supported-subset
  boundaries, native lowering and observation result forms, selected-expression
  object compilation, recorded object evidence, packaging/linking steps, and
  explicit refusals/unavailable lanes. Name producer and later consumer/refuser
  where distinct.
- Retain explanatory/partial authority. Update the existing manifest record
  only where new responsible sources are necessary. Link program and validation
  material as boundary vocabulary, not as a substitute for source grounding.
- Preserve navigation and release-status conventions; never hand-edit
  attestations or add a currency gate.

## Acceptance criteria

- **Single-page form:** expand the existing page in place; create no nested
  page or directory.
- **Grounding:** every present-tense implementation claim identifies its
  responsible backend, artifact, packaging, or program source symbol/location.
- **Boundary:** distinguish checked-program admission from an unchecked source
  fallback; runtime-IR lowering from native-library/interop capability; object
  bytes and hashes from semantic or execution proof; and verifier/smoke-run
  observations from kernel checking or translation validation. Refusal is never
  described as a silent fallback.
- **Authority:** keep the page explanatory and partial; cite normative/future
  claims to their actual program or specification authority.
- **Scope:** modify only this page, its existing manifest record if needed, and
  this frame. Do not edit crates, spec, catalog, conformance, CI, agent paths,
  or adjacent compiler chapters.
- **Validation:** run applicable existing documentation tools. Do not add
  repository-text tests, revive an inert registry, or run a workspace build.
- **Release:** Librarian exact-SHA as-built review is the sole gate and checks
  source grounding plus one-page coherence. Steward records umbrella progress
  on landing.

## Hard stops

Route to the Steward if a needed fact lacks source grounding, requires a
planned-only lane, conflicts with a concurrent manifest/page candidate, or
pulls validation-and-limits detail into this chapter rather than linking it.
