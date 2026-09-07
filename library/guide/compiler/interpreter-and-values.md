# Interpreter and runtime values

> **Availability:** partial. **Authority:** explanatory.

The [runtime-IR evaluator](../../../crates/ken-runtime/src/runtime_ir_evaluator.rs)
executes `RuntimeExpr` from a `RuntimeProgram`. Its own module draws a strict
line around that result: a successful direct
runtime-IR evaluation is an observation, not kernel evidence, native-backend
validation, object validation, linker validation, or a source-level proof.

The evaluator can also compare its observation with a caller-supplied
interpreter observation when both bind the same runtime artifact and target.
Agreement is reported as agreement between those observations. It does not make
the evaluator or the supplied interpreter a proof-producing component.

## Values and storage

The [runtime values module](../../../crates/ken-runtime/src/values.rs) has a
value carrier for scalar and closure-free compound data, and a separate
operational carrier for ordinary closures. It excludes ordinary closures from
the canonical carrier. That prevents a closure, or a
graph containing one, from acquiring canonical bytes, a content-addressed
identity, or persistence by accident.

The [value store](../../../crates/ken-runtime/src/store.rs) interns compound
canonical values in a space-owned arena and index.
It returns a new or existing slot for equal canonical bytes and releases a
space's pages on reset. Those slot and allocation details are implementation
mechanisms, not the value model's portable identity rule.

## Supported evaluation boundary

Before evaluating a program, the runtime-IR evaluator refuses metadata and
expression shapes outside its supported subset. Effects, capabilities, runtime
checks, trust metadata, unsupported entries, and non-supported lowerability
are among the preflight boundaries represented in the implementation. A refusal
is therefore a result of the supported-subset boundary, not a fallback to
native execution.

For the normative meaning of evaluation and values, read the
[operational semantics](../../../spec/40-runtime/42-evaluation.md) and the
[value model](../../../spec/40-runtime/41-values.md). Continue to
[Artifacts and erasure](artifacts-and-erasure.md) to see how a checked package
becomes the runtime artifact the evaluator reads.
