# Native backend

> **Availability:** partial. **Authority:** explanatory.

The runtime's [native backend](../../../crates/ken-runtime/src/cranelift_backend.rs)
lowers runtime IR through Cranelift. The backend facade describes its native
boundary as narrow: scalar results can be returned
directly, while aggregate observations pass through an opaque Rust-side token
mechanism. Native addresses, object layout, allocation order, ABI details, and
Cranelift internals are not Ken-observable meaning.

Package-backed native operations first obtain a checked program admission. The
backend's package path refuses when it cannot establish the required authority
or when the runtime program lies outside the supported subset. It does not use
an unchecked source-text fallback for such a program.

## Object and executable path

The [artifact API](../../../crates/ken-runtime/src/cranelift_backend/artifact/api.rs)
compiles a selected runtime expression into a Cranelift object and records
object bytes, an object hash, target information, verifier status,
assumptions, and unsupported entries. The [packaging layer](../../../crates/ken-runtime/src/object_linker_packaging.rs)
can then write an object, create a starter stub, invoke a linker, and record a
linked executable artifact.

This is an implemented starter route, not a general native-library facility.
Packaging validates an entrypoint package, platform-support report, and
runtime-IR run report before it emits the object. It also requires a supplied
boundary resource profile rather than silently selecting one.

## Unavailable lanes

The compiler program names Cranelift as the first native target. It leaves
secondary targets as future work and separates native-library output from the
Ken-only executable route. The implementation's package types likewise report
unavailable lanes for library ABI, C and Rust interoperation, cross-package
native linking, host-effect or FFI execution, translation validation, and a
whole-compiler proof.

A backend verifier or a successful smoke run is evidence about that artifact;
it is not kernel checking of native output. For the intended compiler boundary
and fidelity vocabulary, see the [compiler program](../../../docs/program/07-compiler-program.md).
Read [Validation and limits](validation-and-limits.md) before treating a native
result as more than its recorded evidence.
