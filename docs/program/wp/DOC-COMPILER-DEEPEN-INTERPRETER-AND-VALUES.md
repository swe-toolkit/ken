# WP frame — `DOC-COMPILER-DEEPEN-INTERPRETER-AND-VALUES`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only child
of `DOC-COMPILER-DEEPEN` deepens the existing interpreter-and-values chapter in
place under the ratified single-page campaign model.

## Objective

Expand `library/guide/compiler/interpreter-and-values.md` into one coherent,
full-length explanatory chapter. A reader must be able to trace runtime-IR
evaluation, the observation comparison boundary, value representation,
canonical-value and closure boundaries, storage interning, and supported-subset
refusal, without mistaking execution evidence or implementation allocation for
kernel proof or portable value semantics.

**Exit property:** an engineer unfamiliar with this stage can navigate from the
chapter to exact evaluator, value, and store sources; correctly state what is
observed, represented, interned, or refused; and distinguish those mechanisms
from source proof, native validation, and normative value/evaluation rules.
Every claim is anchored to source or spec, and no claim exceeds its citation.

## Fixed inputs

Measured at `origin/main = 2aa6f3c0`:

- The existing chapter is explanatory and partial, grounded in
  `runtime_ir_evaluator.rs`, `values.rs`, `store.rs`, and the evaluation and
  value-model specifications.
- The evaluator executes runtime IR and reports observations; its direct result
  is not kernel evidence, native validation, or a source-level proof.
- The values module separates canonical values from ordinary closures, while
  the value store manages canonical-value interning and space-owned storage.
- The front-end, kernel, and artifacts chapters establish the campaign's
  single-page model. This work adds detail only to the existing chapter: no
  subpages, no directory, and no runtime edits.

## Deliverables

- Deepen `library/guide/compiler/interpreter-and-values.md` in place as one
  paragraph-led, documentation-length chapter.
- Explain source-grounded evaluator setup and preflight, runtime-IR evaluation
  and observation comparison, value carriers and canonicalization boundaries,
  closure handling, store interning/reset behavior, and supported-subset
  refusal. State the next consumer or refuser when it differs from the producer.
- Retain explanatory/partial authority and the existing manifest record,
  expanding its sources only where needed to carry the new claims. Link exact
  evaluation/value-model sections for normative meaning.
- Preserve adjacent chapter navigation. Update generated status only under the
  release-point policy; never hand-edit attestations or add a currency gate.

## Acceptance criteria

- **Single-page form:** expand the existing page in place; create no nested
  page or directory.
- **Grounding:** every implementation claim names its responsible evaluator,
  values, store, or specification source symbol/location; producer and
  consumer/refuser are both named where distinct.
- **Boundary:** distinguish runtime-IR observation from proof and native
  validation; canonical value identity from closure-bearing operational state;
  store allocation details from portable value semantics; and supported-subset
  refusal from a fallback to native execution.
- **Authority:** keep the page explanatory and partial; attribute normative
  evaluation and value-model rules to exact spec sections.
- **Scope:** modify only this page, its existing manifest record if needed, and
  this frame. Do not edit crates, spec, catalog, conformance, CI, agent paths,
  or adjacent compiler chapters.
- **Validation:** run applicable existing documentation tools. Do not add
  repository-text tests, revive an inert registry, or run a workspace build.
- **Release:** Librarian exact-SHA as-built review is the sole gate and checks
  accuracy plus one-page coherence. Steward records umbrella progress on
  landing.

## Hard stops

Route to the Steward if required detail lacks a source-grounded claim, depends
on a planned-only runtime path, conflicts with a candidate on this page or its
manifest record, or pulls native-backend/artifact material into this chapter.
