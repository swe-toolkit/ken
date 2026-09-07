# Validation and limits

> **Availability:** partial. **Authority:** explanatory.

The compiler implementation records several different kinds of check. They
must not be collapsed into one claim. Kernel checking admits explicit core
terms. Checked-core validation verifies package identity, semantic hashes, and
lowerability metadata. Runtime and native stages each have their own supported
subset and refusal paths.

The [runtime artifact validator](../../../crates/ken-runtime/src/artifact_validation.rs)
recomputes a bounded set of facts from a `RuntimeProgram`. Its module states the limit directly: it does not certify
Cranelift, native execution, object layout, or whole-compiler correctness. A
separate proof-erasure boundary witness likewise checks facts recomputed from a
concrete runtime artifact; it is not a certificate for all lowering.

## Comparison evidence

The [runtime-IR evaluator](../../../crates/ken-runtime/src/runtime_ir_evaluator.rs)
can produce a direct observation and compare it to a caller-supplied interpreter
observation with matching artifact and target identity. The
[native artifact API](../../../crates/ken-runtime/src/cranelift_backend/artifact/api.rs)
can compare a native result with runtime-IR or interpreter observations. These comparisons are useful only for the supported
inputs they identify; disagreement and unsupported cases remain explicit
outcomes.

The [object-packaging route](../../../crates/ken-runtime/src/object_linker_packaging.rs)
verifies its selected reports and can perform a smoke run. It records toolchain and artifact facts, but native bytes and linker
success remain evidence artifacts rather than Ken semantic authority.

## Boundaries to keep visible

The current implementation does not make secondary backend targets,
native-library output, C or Rust interoperation, cross-package native linking,
translation validation, or whole-compiler verification available through this
route. Runtime effects, foreign paths, capabilities, and trust metadata may
also put a target outside a particular evaluator or backend subset.

These limitations are part of the implementation map, not omissions in the
reader's path. Return to the [architecture](README.md) to select another stage,
or use the [reading workflow](reading-workflow.md) to trace one supported path.
The normative erasure boundary is in the
[erasure and runtime-IR specification](../../../spec/40-runtime/47-erasure-runtime-ir.md).
