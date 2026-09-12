# WP frame — `DOC-COMPILER-DEEPEN-KERNEL`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only
child of `DOC-COMPILER-DEEPEN` deepens the kernel chapter using the ratified
front-end exemplar. It does not alter the kernel, its specification, or its
trust boundary.

## Objective

Keep `library/guide/compiler/kernel.md` as the chapter landing and add nested
explanatory depth pages. A reader unfamiliar with the kernel can use the
landing page to find the explicit core syntax, contexts and declarations,
type checking and admission, conversion and reduction, and inductive
admission mechanisms; they can distinguish what the kernel decides from what
later compiler stages merely consume or observe.

**Exit property:** from the kernel chapter, an engineer can navigate to the
exact source implementing each described kernel mechanism and correctly state
what it does, what it guarantees, and what it refuses. Every claim is anchored
to a source or spec section, and no claim exceeds what its citation carries.

## Fixed inputs

Measured at `origin/main = dd0ac70b`:

- `library/guide/compiler/kernel.md` is an explanatory, partial chapter
  landing. It names the kernel as the trust root for explicit core terms and
  directs readers to the front end, artifacts and erasure, and normative
  kernel/trust documents.
- `crates/ken-kernel/src/lib.rs` exposes the kernel's checking, conversion,
  environment, inductive, substitution, and term modules and forbids unsafe
  Rust.
- The primary implementation sources are `term.rs`, `env.rs`, `check.rs`,
  `conv.rs`, and `inductive.rs`. Their claimed mechanisms must be grounded at
  the responsible symbol, not inferred from a module name or comment.
- Kernel syntax and trust claims remain normative only in `spec/10-kernel/`
  and `spec/60-security/64-trust-model.md`; this work describes the
  implementation's realized boundary without adding rules.

## Deliverables

- Retain the kernel landing page and link it to a new
  `library/guide/compiler/kernel/` directory.
- Create depth pages for source-grounded kernel mechanisms: explicit core
  terms; contexts and declarations; bidirectional checking and admission;
  conversion and reduction; and inductive-family admission. Combine subjects
  only where doing so improves source navigation and preserves separate
  mechanism, boundary, and refusal anchors.
- Apply the ratified page template: each depth page explains one mechanism,
  cites its implementing source, names its invariant or boundary, identifies a
  refusal or non-authority path when source supplies one, and points to an
  exact spec section when describing a normative language rule.
- Apply the ratified corpus convention: nested Markdown pages, one manifest
  record per page, `kind` and `authority` both `explanatory`, `availability =
  "partial"`, source entries that carry the pages' claims, landing-page and
  next-step navigation, and no literate `.ken.md` page unless a necessary
  checked fence is exercised.
- Update `library/manifest.toml` and generated `library/STATUS.md` only as
  supported by the established release-point policy. Never hand-edit source
  attestations or create a standing currency gate.

## Acceptance criteria

- **Navigation:** the landing remains the overview and indexes every depth
  page; each depth page links back to it and to its next useful reading step.
- **Grounding:** every implementation claim is tied to the source symbol or
  bounded location that carries it. Where producing and refusing stages differ,
  identify both; a file-level citation alone does not discharge that boundary.
- **Boundary:** pages distinguish explicit core construction from kernel
  admission; raw well-formedness from type checking; conversion/reduction from
  an unbounded execution claim; kernel authority from runtime, backend, and
  host behavior. They must not imply that later artifacts or tests become
  proofs by passing through adjacent compiler stages.
- **Authority:** every new page is explanatory and partial. Normative syntax,
  typing, conversion, and trust rules are attributed to their exact spec
  sections rather than stated on the guide's authority.
- **Corpus convention:** the new nested paths and manifest records pair
  exactly, and the landing page remains valid. The candidate follows the
  front-end exemplar rather than introducing a new convention.
- **Validation:** run applicable existing documentation validation tools. Do
  not introduce repository-text tests, revive an inert registry, or run a
  workspace build locally.
- **Scope:** change only `library/` and this child frame under
  `docs/program/wp/`; do not edit `crates/`, `spec/`, `catalog/`,
  `conformance/`, CI, or agent paths.
- **Release:** the Librarian's exact-SHA as-built approval is the sole accuracy
  gate. On landing, the Steward records completion on the umbrella Progress
  section; no per-child tracker node or status flip is created.

## Hard stops

Route to the Steward if the proposed pages require a claim not carried by the
source, a planned-only mechanism, a competing candidate on the same library or
manifest paths, or a departure from the ratified corpus convention. Do not
broaden this child into front-end, artifact, runtime, or native-backend work.
