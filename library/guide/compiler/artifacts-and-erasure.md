# Artifacts and erasure

> **Availability:** partial. **Authority:** explanatory.

`CheckedCorePackage v0`, defined in the
[checked-core module](../../../crates/ken-elaborator/src/checked_core.rs), is the
implemented artifact between kernel admission and execution-oriented stages. The
[compiler driver](../../../crates/ken-elaborator/src/compiler_driver.rs) emits a
package after it elaborates source into an admitted environment. Its stable
symbols and hashes identify the package and its semantic inputs, while its
artifact envelope can retain non-semantic provenance.

That distinction matters at the next boundary. Source identity can remain in an
envelope for diagnostics and provenance, but the erasure route does not use raw
source text to determine runtime meaning. The checked-core package, not a source
file or its spelling, is the consumer input.

## Selection and validation

The compiler driver's target selection records the package identity, semantic
and artifact hashes, selected targets, lowerability, unsupported lanes,
obligations, assumptions, and trusted-base delta in `TargetSelectionReport`.
This report shapes target metadata; it is not runtime lowering, native-artifact
production, or a validation result about a later backend.

Before erasure exposes a `RuntimeProgram`,
`erase_checked_core_package_for_target` validates the package and then validates
its semantic integrity. It also consumes the selected target closure and rejects
reachable unsupported lanes. Thus validation rejects malformed or inconsistent
checked-core input at the erasure boundary. It does not replace kernel
admission: the compiler driver is the earlier producer of a package whose
source declarations have already been elaborated and admitted.

## Erasure to runtime IR

The [erasure module](../../../crates/ken-elaborator/src/erasure.rs) converts a
validated `CheckedCorePackage` and selected target closure into a
`RuntimeProgram`. It constructs `RuntimeMetadata` from semantic inputs,
including obligations, assumptions and their trust metadata, trusted-base delta,
dependency semantic hashes, lowerability, unsupported entries, runtime checks,
capabilities, and effects. It then lowers selected declarations into explicit
runtime declarations and expressions.

This is not proof irrelevance by informal convention. Unsupported checked-core
expression lowering, missing runtime metadata, malformed runtime-role authority,
and inconsistent proof-erasure witnesses return `ErasureError`. A rejected route
does not silently become executable runtime IR. Conversely, successful erasure
says that this checked package has crossed the stated erasure boundary; it does
not establish native lowering or a machine artifact.

## Runtime and native boundaries

`RuntimeProgram` is an execution-oriented IR: it carries package identity and
semantic/artifact hashes alongside erased executable core and lowered
declarations. Its identity and metadata connect a runtime consumer to the
checked package it received, but they do not prove that runtime behavior agrees
with source semantics. Runtime interpretation is a later consumer under its own
contract.

Native production is later still. The compiler driver has a distinct native
build output containing a runtime program and an artifact, and its errors
separate driver, admission, erasure, packaging, and unavailable-lane failures.
Therefore neither checked-core emission nor erasure certifies Cranelift lowering,
object construction, linking, host ABI behavior, or native execution.

The [erasure and runtime-IR specification](../../../spec/40-runtime/47-erasure-runtime-ir.md)
is normative. This page explains the current implementation boundaries rather
than adding an admission or correctness guarantee. Continue with [Interpreter
and runtime values](interpreter-and-values.md) for the direct runtime-IR path,
or [The native backend](native-backend.md) for the native route.
