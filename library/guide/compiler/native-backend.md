# Native backend

> **Availability:** partial. **Authority:** explanatory.

The runtime [native backend](../../../crates/ken-runtime/src/cranelift_backend.rs)
lowers runtime IR through Cranelift. Its native boundary is deliberately narrow:
scalar results return directly, while aggregate observations use an opaque
Rust-side token mechanism. Native addresses, object layout, allocation order,
ABI details, and Cranelift internals are not Ken-observable meaning.

Package-backed native work first obtains checked program admission. The packaging
path calls `native_program_admission` before any native work and refuses a
program whose authority does not close against its own checked package. It does
not fall back to unchecked source text or legacy prelude spellings.

## From runtime IR to an object

The artifact API emits an object from a runtime program only through the
admitted authority path. The resulting object records bytes, an object hash,
target information, verifier status, assumptions, and unsupported entries. This
is evidence about one emitted artifact, not a correctness proof or kernel check
of native output.

Runtime-IR lowering is distinct from native-library capability. The current
starter path targets a narrow Ken-only executable route; it records unavailable
lanes for library ABI, C and Rust interoperation, cross-package linking, foreign
ABI, host-effect or FFI execution, translation validation, and whole-compiler
proof. An unavailable lane is reported rather than silently represented as a
supported library interface.

## Packaging and refusal

The [object linker packaging layer](../../../crates/ken-runtime/src/object_linker_packaging.rs)
validates a supplied resource profile, entrypoint package, platform support
report, runtime-IR run report, and native comparison before emitting and linking
the starter artifact. Each record is required to bind the exact runtime artifact
and target. Missing or stale bindings, unsupported platform targets, linker
failures, and failed smoke execution return packaging errors.

A boundary resource profile is deployment policy. `validate_options` refuses
its absence before object emission or linking; packaging does not invent or
silently default that policy. The packaging route writes an object and starter
stub, invokes a linker, reads linked bytes, and records hashes and smoke-run
facts. These are build artifacts and tested observations, not semantic authority
or proof evidence.

## Evidence boundary

A successful backend verifier, object emission, linker invocation, native
comparison, or smoke run establishes only the evidence explicitly recorded for
that artifact and target. It neither re-admits the source nor proves that native
behavior agrees with Ken semantics. The [compiler program](../../../docs/program/07-compiler-program.md)
and [validation and limits](validation-and-limits.md) describe the broader
boundary. This page explains the current implementation; native compilation is
partial and explanatory.
