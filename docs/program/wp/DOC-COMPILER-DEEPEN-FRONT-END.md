# WP frame — `DOC-COMPILER-DEEPEN-FRONT-END`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This doc-only
child of `DOC-COMPILER-DEEPEN` deepens the front-end chapter. It establishes
one reusable corpus convention for the remaining compiler-guide chapters.

## Objective

Keep `library/guide/compiler/front-end.md` as the chapter landing page and
add nested explanatory depth pages. A reader unfamiliar with the front end can
use the landing page to find the source-input, lexing, parsing, resolution, and
elaboration-and-admission mechanisms, then distinguish what each stage does
from what it leaves to the next stage or to the kernel.

**Exit property:** from the front-end chapter, an engineer can navigate to the
exact source implementing each described stage and correctly state what it
does, what it guarantees, and what it refuses. Every claim is anchored to a
source or spec section, and no claim exceeds what its citation carries.

## Fixed inputs

Measured at `origin/main = 111c18c2c`:

- The existing `front-end.md` is an explanatory, partial chapter landing with
  sources for the elaborator entry module, compiler driver, CLI, and surface
  elaboration specification.
- `ken-elaborator` declares the implemented sequence `lex → parse → resolve →
  elaborate → kernel-check` in `crates/ken-elaborator/src/lib.rs`.
- The stage modules are `lexer.rs`, `parser.rs`, `resolve.rs`, and `elab.rs`.
  The CLI dispatch is in `crates/ken-cli/src/main.rs`.
- The kernel, not the elaborator, remains the admission authority. The depth
  pages describe the implemented route and do not create language rules.

These inputs authorize explanatory source navigation only. They do not
establish whole-front-end completeness, a second admission mechanism, or a
claim that successful source processing produces a native artifact.

## Deliverables

- Retain `library/guide/compiler/front-end.md` as the chapter landing and add
  links from it into a new `library/guide/compiler/front-end/` directory.
- Create the depth pages whose subjects are grounded by the source: source
  input and dispatch, lexing, parsing, resolution, and elaboration plus kernel
  admission. Combine subjects only where that improves the reader's path and
  preserves a separately anchored boundary.
- Establish the durable depth-page template: each page states its mechanism,
  cites the source that implements it, names its invariant or boundary, names
  a refusal path when the source supplies one, and points to the applicable
  specification when describing a normative language rule.
- Establish the corpus convention: nested Markdown pages, one manifest record
  per page, `kind` and `authority` both `explanatory`, `availability =
  "partial"`, and source entries naming the source that carries the page's
  claims. No page is literate `.ken.md` unless its own checked fences are
  necessary and exercised; no such need is assumed by this frame.
- Update `library/manifest.toml` and generated `library/STATUS.md` as required
  by the new registered pages. Do not hand-edit source attestations; use the
  generator only if an authorized release-point fold calls for it.

## Acceptance criteria

- **Navigation:** the landing page remains the overview and links to every
  depth page; every depth page links back to the landing page and to the next
  useful source-reading step.
- **Grounding:** each present-tense implementation claim has an adjacent or
  clearly associated source anchor. The Librarian can trace that anchor to a
  symbol or bounded source location that carries the claim.
- **Boundaries:** pages distinguish token recognition, syntax construction,
  name resolution, elaboration, and kernel checking. They do not imply that a
  front-end success proves source text, replaces kernel checking, or produces
  native output.
- **Authority:** all new pages are explanatory and partial. A language rule is
  attributed to its exact specification section; no normative rule is stated
  on the guide's authority.
- **Corpus convention:** new paths and manifest records pair exactly; existing
  chapter landing and manifest record remain valid. The new convention is
  explicit enough for the Librarian to ratify as the campaign exemplar.
- **Validation:** run the applicable existing documentation validation tools
  and prove every new or changed gate invocation detects a planted violation
  where the tool supports such a mutation. Do not create a repository-text
  test, revive an inert registry, or run a workspace build locally.
- **Scope:** the candidate changes only `library/` and this child frame under
  `docs/program/wp/`; it does not edit `crates/`, `spec/`, `catalog/`,
  `conformance/`, CI, or agent paths.
- **Release:** the Librarian's exact-SHA as-built approval is the sole accuracy
  review. It must ratify the exemplar's template and corpus convention before
  the doc leader cuts the next campaign child.

## Hard stops

Route to the Steward if source grounding reveals that a planned-only mechanism
would be required, the desired depth needs a claim the cited source does not
carry, another active candidate claims the same `library/` or manifest paths,
or the Librarian rejects the proposed corpus convention. Do not broaden this
child into kernel, artifact, runtime, or native-backend material.
