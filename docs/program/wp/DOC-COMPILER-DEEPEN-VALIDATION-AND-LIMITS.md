# WP frame — `DOC-COMPILER-DEEPEN-VALIDATION-AND-LIMITS`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only child
of `DOC-COMPILER-DEEPEN` deepens the existing validation-and-limits chapter in
place under the ratified single-page campaign model.

## Objective

Expand `library/guide/compiler/validation-and-limits.md` into one coherent,
full-length explanatory chapter. A reader must be able to distinguish kernel
admission, checked-core/package validation, runtime artifact validation,
proof-erasure witnessing, supported-subset refusal, observation comparison, and
native/object evidence without treating any individual check as a general
compiler certificate.

**Exit property:** a reader can trace each check to the source that performs it,
state its artifact, predicate, result or refusal boundary, and correctly name
what it does not establish. The chapter links its neighboring stages rather than
absorbing their implementation detail.

## Fixed inputs

Measured at `origin/main = 4f8615b0`:

- The existing explanatory/partial page names artifact validation, runtime-IR
  evaluation, native artifact comparison, object-linker packaging, and the
  erasure/runtime-IR specification.
- Kernel admission, checked-core validation, runtime validation, and native
  handling are distinct mechanisms with distinct supported-subset/refusal and
  evidence boundaries.
- Artifact validation recomputes a bounded fact set from runtime IR; comparison
  and smoke evidence remain artifact-scoped observations rather than native or
  whole-compiler proof.
- This is the approved single-page campaign model: expand the existing chapter
  only; no subpages, directory, or crate changes.

## Deliverables

- Deepen `library/guide/compiler/validation-and-limits.md` in place as one
  paragraph-led, documentation-length chapter.
- Explain source-grounded validator inputs/results, recomputation and witness
  bounds, identity required for comparisons, explicit disagreement/refusal,
  native/object smoke evidence, and unavailable verification lanes. Name
  producer and later consumer/refuser when distinct.
- Keep explanatory/partial authority. Update the existing manifest record only
  if new responsible sources are necessary; cite normative erasure language to
  its exact specification section.
- Preserve page navigation; do not hand-edit generated attestations or add a
  currency gate.

## Acceptance criteria

- **Single-page form:** expand the existing page in place; create no nested
  pages or directory.
- **Grounding:** every implementation claim identifies the responsible
  admission, validation, evaluator, artifact, or packaging source symbol/location.
- **Boundary:** distinguish kernel admission from package validation; bounded
  runtime/artifact validation from proof of lowering; observation comparison
  from agreement as proof; object/linker/smoke evidence from semantic authority;
  and unsupported input from a fallback. State negative scope explicitly where
  the source does.
- **Authority:** keep the page explanatory and partial, with normative erasure
  claims attributed to the specification.
- **Scope:** modify only this page, its existing manifest record if needed, and
  this frame. Do not edit crates, spec, catalog, conformance, CI, agent paths,
  or other chapters.
- **Validation:** run applicable existing documentation tools. Do not add
  repository-text tests, revive an inert registry, or run a workspace build.
- **Release:** Librarian exact-SHA as-built review is the sole gate and checks
  claim grounding plus one-page coherence. Steward owns batched umbrella
  progress publication at campaign close.

## Hard stops

Route to the Steward if a required claim lacks source grounding, requires a
planned-only verification lane, conflicts with another candidate on this page or
its manifest record, or imports implementation detail owned by another chapter.
