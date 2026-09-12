# Interpreter and runtime values

> **Availability:** partial. **Authority:** explanatory.

The [reference interpreter](../../../crates/ken-interp/src/lib.rs) evaluates
core terms to runtime values in call-by-value order with sharing. Its `eval`
entry point takes an environment and an `EvalStore`; its result is an execution
observation. It does not constitute kernel evidence, source-level proof, native
backend validation, object validation, linker validation, or native execution.

The [runtime-IR evaluator](../../../crates/ken-runtime/src/runtime_ir_evaluator.rs)
is a separate consumer of `RuntimeExpr` in a `RuntimeProgram`. When it compares
its result with a caller-supplied interpreter observation, both observations
must bind the same runtime artifact and target. Agreement reports that bounded
comparison; it does not make either evaluator proof-producing.

## Values and closures

The [runtime values module](../../../crates/ken-runtime/src/values.rs) separates
the canonical carrier for scalar and closure-free compound data from the
operational representation of ordinary closures. A normal closure, or a graph
containing one, cannot thereby acquire canonical bytes, content-addressed
identity, or persistence merely by passing through a value API.

This is a boundary in representation, not a statement that every runtime result
is canonical data. Evaluation can produce closures as operational values when a
function remains to be applied. The canonical carrier describes closure-free
values that may participate in the value-store mechanism.

## Storage and value identity

The [value store](../../../crates/ken-runtime/src/store.rs) interns compound
canonical values in a space-owned arena and index. Equal canonical bytes yield
a new or existing slot, and resetting a space releases its pages. Slots, arenas,
and allocation lifetime are implementation mechanisms; they are not a portable
semantic identity rule for a Ken value.

Sharing is consequently an operational property of this evaluator and store.
It may make repeated equal canonical values reuse a slot, but a slot number is
not the language-level meaning of a value. The runtime value model specifies the
normative distinction.

## Supported evaluation boundary

Before evaluating a runtime program, the runtime-IR evaluator checks metadata
and expression shapes against its supported subset. Effects, capabilities,
runtime checks, trust metadata, unsupported entries, and non-supported
lowerability are preflight boundaries in that implementation. A refusal is a
supported-subset result; it is not a fallback that silently asks the native
backend to execute the program.

The [operational semantics](../../../spec/40-runtime/42-evaluation.md) and
[value model](../../../spec/40-runtime/41-values.md) are normative. This page
describes current evaluators and their boundaries. Read
[Artifacts and erasure](artifacts-and-erasure.md) for the checked package to
runtime-artifact transition, or [The native backend](native-backend.md) for the
later native route.
