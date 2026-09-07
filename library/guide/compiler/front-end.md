# Source to checked core

> **Availability:** partial. **Authority:** explanatory.

The front end turns Ken source into declarations that the kernel can admit. Its
[crate entry module](../../../crates/ken-elaborator/src/lib.rs) states the
implemented sequence: lexing, parsing, resolution, elaboration, and kernel
checking. These are distinct stages:
parsing establishes source structure, resolution relates names to declarations,
and elaboration produces explicit core terms before the kernel checks them.

The front end does not make source text trusted. `ElabEnv` owns the
surface-level environment and calls the kernel checker when it discharges a
certificate. The kernel remains the authority for type checking and proof
validity; the elaborator prepares terms and metadata for that authority.

## Entry points

The [command-line driver](../../../crates/ken-cli/src/main.rs) selects `ken
check` for elaboration without I/O and `ken run` for elaboration followed by
program execution. Both construct an
elaboration environment and choose the ordinary or literate-source path from
the filename. A catalog-addressed entry uses the roots loader and then checks
that entry's checked fences.

These command paths are not a second admission mechanism. They feed the
elaborator's normal file or literate-file operations. A successful front-end
stage therefore means that the selected input reached its stated elaboration
path; it does not mean that a native artifact was produced.

## What follows admission

The [compiler driver](../../../crates/ken-elaborator/src/compiler_driver.rs) can
collect admitted declarations into a `CheckedCorePackage`. That package carries
stable symbols and semantic metadata for later consumers, rather than passing
raw source text to erasure. Read
[Artifacts and erasure](artifacts-and-erasure.md) for the boundary it creates,
or [The trusted kernel](kernel.md) for the checker that admits the core terms.

The accepted surface language and its normative rules are specified in the
[surface specification](../../../spec/30-surface/39-elaboration.md). This page
describes the implemented route, not additional syntax or elaboration rules.
