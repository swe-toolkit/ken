# WP frame — `DOC-COMPILER-IMPLEMENTATION-GUIDE`

Owner: doc ring (doc-leader, doc-author, Librarian as QA). This is a doc-only
candidate. It creates the explanatory compiler implementation guide requested
by the operator; it is not a compiler-program change and does not establish
new implementation or semantic commitments.

## Objective

Produce `library/guide/compiler/` for engineers reading the implemented Ken
compiler. The guide must let a reader follow the implemented pipeline without
mistaking a planned, partial, or untrusted stage for a completed or trusted
one.

**Exit property:** an engineer can start at the architecture page, select one
implemented path, and trace it from source through elaboration and kernel
admission to its applicable evaluation or native-artifact path. At every
boundary, the guide identifies the consuming artifact, the source of authority,
and whether the path is implemented, partial, or unavailable.

## Fixed inputs

Measured at `origin/main = 3f2445075`:

| input | measured value |
|---|---|
| `library/guide/compiler/` | absent |
| existing `library/guide/` pages | four explanatory pages; none is a compiler implementation guide |
| front-end crate | `crates/ken-elaborator/`, whose crate documentation names `lex → parse → resolve → elaborate → kernel-check` |
| trusted kernel crate | `crates/ken-kernel/`, whose public modules include checking, conversion, environment, inductives, substitution, and terms |
| checked-core artifact | `CheckedCorePackage v0` is emitted by `ken-elaborator::checked_core` and consumed by erasure |
| erasure boundary | `ken-elaborator::erasure` consumes checked-core artifacts and constructs `ken_runtime::RuntimeProgram` |
| runtime and native path | `crates/ken-runtime/` contains runtime IR, its evaluator, Cranelift backend, artifact validation, and executable packaging |
| compiler-program contract | `docs/program/07-compiler-program.md` fixes the durable boundary at `CheckedCorePackage v0` and the first native target as Cranelift |

The fixed inputs establish a subject distinct from the reader-facing language,
toolchain, and runtime material already in `library/`. They do not authorize a
claim that every compiler-program phase is complete.

## Deliverables

Create the following explanatory, human-reader pages under
`library/guide/compiler/`:

1. `README.md` — architecture and the code-reading map.
2. `front-end.md` — source text through parsing, resolution, elaboration, and
   kernel re-checking.
3. `kernel.md` — the trusted kernel boundary and the responsibilities that do
   not cross it.
4. `interpreter-and-values.md` — runtime values and the reference-evaluator
   role, only to the extent grounded by the implementation and applicable spec.
5. `artifacts-and-erasure.md` — checked-core packages, stable identity,
   metadata survival, erasure, and runtime IR.
6. `native-backend.md` — the implemented native route through runtime IR,
   Cranelift lowering, and packaging; it must separate this route from future
   secondary targets and native-library work.
7. `validation-and-limits.md` — implemented checks, differential/reference
   comparisons where present, and explicit limits or unavailable lanes.
8. `reading-workflow.md` — a source navigation workflow that links the pages
   into an implemented-path-first reading order.

Register every page in `library/manifest.toml` with `kind = "explanatory"`,
`authority = "explanatory"`, `audience = ["human-reader"]`,
`availability = "partial"`, its grounded source set, the standard document
validation inventory, and `owner = "doc"`. The author must add only sources
whose contents they have read and whose claims they carry.

Regenerate `library/STATUS.md`. If new citations require source-attestation
rows, generate the proposed ledger update with
`scripts/gen-source-attestations.sh`; do not hand-write a row.

## Required grounding

The author starts each page from the relevant implementation, then uses the
specification or compiler program only for the boundary that source alone does
not establish. The expected initial map is:

| guide subject | primary grounding |
|---|---|
| architecture and workflow | `docs/program/07-compiler-program.md`, `crates/ken-cli/src/main.rs`, and crate entry modules |
| front end | `crates/ken-elaborator/src/lib.rs`, `lexer.rs`, `parser.rs`, `resolve.rs`, and `elab.rs` |
| kernel | `crates/ken-kernel/src/lib.rs`, `check.rs`, `conv.rs`, `env.rs`, and `term.rs` |
| interpreter and values | `crates/ken-runtime/src/runtime_ir_evaluator.rs`, `values.rs`, `store.rs`, and applicable `spec/40-runtime/42-evaluation.md` / `41-values.md` sections |
| artifacts and erasure | `crates/ken-elaborator/src/checked_core.rs`, `compiler_driver.rs`, `erasure.rs`, and `spec/40-runtime/47-erasure-runtime-ir.md` |
| native backend | `crates/ken-runtime/src/cranelift_backend/`, executable packaging modules, and `docs/program/07-compiler-program.md` |
| validation and limits | implemented validation/differential modules and the exact applicable specification or program sections |

This table is a starting map, not a citation inventory. A page may narrow its
sources after grounding; it may not expand a claim merely because a nearby
program frame discusses a future phase.

## Acceptance criteria

- **AC-1 — exact page and manifest pairing.** The eight delivered page paths
  and their manifest records match exactly, in both directions.
- **AC-2 — implemented-path grounding.** Every present-tense implementation
  claim has a cited source file that establishes it. A source citation is not a
  decoration: the Librarian verifies that it carries the adjacent claim.
- **AC-3 — boundary preservation.** Each pipeline page distinguishes the
  elaborator from the kernel, checked core from source text, erasure from
  backend lowering, and execution from proof checking. No page says or implies
  that native output is kernel-checked.
- **AC-4 — availability truthfulness.** Every page is labelled `partial`; each
  unavailable, planned, or incomplete lane named in prose states that boundary
  precisely. Future secondary targets, native-library output, and whole-
  compiler verification are not presented as implemented.
- **AC-5 — path reachability.** The architecture page links to every other
  guide page, and every leaf page links back to the architecture page and to
  its next relevant reader step. The declared code-reading workflow can be
  followed without a repository-path guess.
- **AC-6 — no normative duplicate.** The pages explain implementation and
  point to the normative specification for language or runtime contracts. They
  do not introduce rules, syntax, or trust claims on their own authority.
- **AC-7 — corpus mechanics.** Manifest records use valid authority and
  availability classes; all source anchors resolve; the generated status and,
  if changed, generated source-attestation proposal agree with the candidate.
- **AC-8 — candidate scope.** The candidate touches only `library/` and this
  frame under `docs/program/wp/`; it changes no `crates/`, `spec/`, `catalog/`,
  CI, or agent path.
- **AC-9 — CI green.** The merge gate is green in CI. Local validation is
  limited to the documentation tools and any targeted crate command needed to
  exercise an applicable existing gate; never run `--workspace` locally.

## Review and validation

The doc-author validates links, manifest syntax, source anchors, status
regeneration, and the generated source-attestation proposal where applicable.
The Librarian independently grounds every page against its cited source,
reviews authority and availability labels, verifies the manifest and citation
status, and checks applicable existing documentation gates. The Librarian's
review is the QA turn for this WP.

This guide adds no checked Ken syntax or examples. It must not manufacture a
new gate merely to assert coverage; the independent review and existing corpus
mechanics are the available checks.

## Banned scope

- No change to compiler implementation, specifications, conformance, or tests.
- No promise of self-hosting, native libraries, foreign interoperation, or a
  secondary backend target.
- No assertion that the compiler, erasure, interpreter, runtime, or backend is
  proved unless an existing cited artifact establishes exactly that claim.
- No campaign, work-package, review, or merge history in reader-facing prose.
- No change to existing `library/guide/` pages or their manifest classes.
- No new repository-text test or revival of the inert validation registry.

## Contention and sizing

The active-worktree census found no live branch modifying `library/`; the
current build worktrees modify implementation paths, not this candidate's
`library/` and `docs/program/wp/` paths. The doc track may proceed concurrently
under its standing contention-free exception. Recheck the candidate path set at
release time; if another active candidate has claimed either path, defer to the
Steward rather than racing it.

Size **L**, one candidate, one sequential doc-ring turn. The page count is not
the sizing basis: the work requires independent grounding across the front end,
trust boundary, artifact boundary, runtime, backend, validation, and reader
workflow. The author must keep the turn active through source grounding,
writing, manifest work, validation, commit, and handoff.

## Hard stops

Stop and route to the Steward if any holds:

1. Grounding shows an intended page would only describe a planned path, with no
   implemented subject to explain.
2. A required implementation claim conflicts with its cited specification or
   compiler-program contract and the discrepancy is not structurally resolved
   by narrowing the prose.
3. The candidate needs a source-attestation row but the generator cannot
   produce a valid proposal.
4. Another active candidate claims `library/guide/compiler/`,
   `library/manifest.toml`, or this frame path.
5. The required page set grows beyond this one bounded compiler-reading path;
   route a recut instead of adding adjacent compiler-program material.
