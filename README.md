# Ken

Ken is an MIT-licensed, verified software-engineering language designed for a
world in which agents write programs and humans review them.

The scarce resource in that world is not typing code. It is understanding what
the code promises and deciding whether those promises are justified. Ken makes
that boundary part of the language: programs state properties alongside their
implementations, the toolchain proves what it can, and the remaining claims are
identified honestly as tested, delegated, or unknown.

Ken is intended for systems-adjacent through application-level software. It
combines a dependently typed language with a small, permanent proof kernel, a
strict reference interpreter, and a native compiler. The kernel re-checks proof
certificates produced by the rest of the toolchain, so the trusted foundation
remains small enough to audit.

Read [`docs/PRINCIPLES.md`](docs/PRINCIPLES.md) for the project's reasoning
charter and [`spec/00-overview.md`](spec/00-overview.md) for the language
overview.

## Language

Ken brings specifications, proofs, programs, and their trust boundaries into
one readable source language.

- **Dependent types.** Pi and Sigma types, inductive families, predicative
  universes, refinements, and total pattern matching express relationships that
  ordinary type systems leave to tests or comments.
- **Observational equality.** Equality, casts, proof irrelevance in `Omega`,
  quotients, and decidable conversion live behind a compact kernel interface.
- **Proofs and specifications.** Functions can carry requirements and
  guarantees. The elaborator generates obligations, the prover produces
  certificates, and the kernel checks them independently.
- **Honest verification status.** Ken distinguishes claims that are `proved`,
  `tested`, `delegated`, or `unknown` instead of presenting every successful
  build as the same kind of assurance.
- **Everyday data.** Arbitrary-precision and fixed-width numbers, strings,
  bytes, sums, records, collections, recursive data, and nested patterns are
  available in the surface language and packages.
- **Canonical type classes.** Classes support reusable interfaces while a
  coherence discipline keeps instance resolution predictable and readable.
- **Effects and foreign boundaries.** Interaction-tree effects, capabilities,
  information-flow labels, constant-time annotations, and explicit foreign
  interfaces keep environmental assumptions visible.
- **Modules and packages.** Names can be organized, imported, qualified, and
  shared across package boundaries without making source-file layout the
  semantic authority.
- **One canonical format.** `ken fmt` formats both `.ken` source and literate
  `.ken.md` files; `ken fmt --check` makes formatting enforceable in automation.

Ken optimizes the permanent form for reading and checking. Rich mathematical
notation has an ASCII transliteration, proof-relevant choices remain visibly
different from proof-irrelevant facts, and ambiguity is rejected rather than
resolved by hidden convention.

## Catalog

[`catalog/`](catalog/) contains checked package sources and compatibility
pointers to the maintained authoring guides. The guides now live in
[`library/guide/`](library/guide/); `catalog/guide/` is not a second maintained
copy. The checked packages include laws, proofs, examples, and design rationale
across core abstractions, collections, parsing, effects, and other areas. The
[package subject index](library/reference/catalog/subjects.md) and package
cards provide a partial, reader-oriented reference; the catalog sources remain
the checked basis for those entries.

The catalog is a useful place to read Ken before consulting the normative
specification.

## Interpreter and compiler

Ken has two execution paths with deliberately different roles.

The **reference interpreter** evaluates supported Ken programs using strict
call-by-value evaluation over the content-addressed value model and drives
supported effects through explicit capabilities. The REPL and `ken run` use
this execution path. The interpreter is the reference semantics. Native
correctness is agreement with the interpreter on the differential corpus.

The **Rust bootstrap compiler** consumes kernel-admitted checked core, erases
proof-only content, lowers executable code to Ken runtime IR, and uses
Cranelift to emit native artifacts for a narrow Ken-only executable route.
Compiler and runtime checks record bounded evidence about admission, lowering,
artifacts, and selected execution comparisons. Native emission and smoke
results are not proofs of semantic equivalence. The compiler is not part of the
type-soundness trust root: compilation bugs must not become false proofs.

The compiler and interpreter provide distinct execution routes. The
[compiler implementation guide](library/guide/compiler/README.md) describes
their boundaries and current limitations.

## Command-line tools

The `ken` driver provides `check`, `run`, `native-build`, `fmt`, `repl`,
`version`, and `help`. The current command forms are documented in the
[toolchain reference](library/reference/toolchain/README.md).

`check` elaborates source and checks literate fence expectations without
running a program. `run` executes an admitted `main` entry point through the
reference interpreter. `native-build` emits a native artifact for a supported
program and requires an output directory. `fmt` canonicalizes plain and
literate Ken source. Native execution and host-effect support remain scoped
subsets; the compiler guide and toolchain reference document their refusal and
availability boundaries.

## Repository map

- [`catalog/`](catalog/) — checked standard packages and compatibility
  pointers to guides.
- [`crates/`](crates/) — the kernel, elaborator, prover-facing surface,
  interpreter, runtime/compiler support, content-addressed foundation, and CLI.
- [`spec/`](spec/) — normative language and runtime specification.
- [`conformance/`](conformance/) — black-box conformance cases and seeds.
- [`docs/adr/`](docs/adr/) — architecture decisions.
- [`docs/PRINCIPLES.md`](docs/PRINCIPLES.md) — the reasoning charter.
- [`CLEAN-ROOM.md`](CLEAN-ROOM.md) — provenance and clean-room policy.

## Build

Ken is implemented in Rust. Build and run the CLI with:

```bash
cargo build -p ken-cli --locked
cargo run -p ken-cli -- help
```

To run the CLI crate's tests, use `cargo test -p ken-cli --locked`.

## License

Ken is licensed under the [MIT License](LICENSE). Programs written in Ken may
use any license their authors choose.
