# Artifacts and erasure

> **Availability:** partial. **Authority:** explanatory.

[`CheckedCorePackage v0`](../../../crates/ken-elaborator/src/checked_core.rs)
is the implemented artifact between kernel admission and execution-oriented
stages. It gives declarations stable symbols, separates
semantic inputs from the non-semantic artifact envelope, and carries hashes for
both. The compiler driver emits this package after elaborating source into an
admitted environment.

A package consumer validates the package and checks that each selected target
is lowerable before exposing a consumed package. This makes lowerability an
explicit boundary: a target marked unsupported, deferred, or otherwise blocked
does not silently proceed as a runtime target.

## Erasure is a consumer of checked core

The [erasure module](../../../crates/ken-elaborator/src/erasure.rs) consumes a
`CheckedCorePackage` and produces a `RuntimeProgram`. It validates the package, validates semantic integrity, and
retains runtime metadata for obligations, assumptions, trusted-base deltas,
dependencies, lowerability, effects, capabilities, and runtime checks. Raw
source identity may remain available for provenance, but source text is not an
input to runtime meaning at this boundary.

Proof-irrelevant material is not simply discarded by convention. The erasure
path can reject unsupported expression lowering and missing or inconsistent
runtime metadata. The resulting runtime IR makes executable data and control
explicit while retaining the metadata required to state supported and blocked
lanes.

## What the package does not establish

A checked-core package is not a native object and does not certify backend
lowering. Its stable identity and hashes bind the artifact being consumed; they
do not prove that a later native result agrees with source semantics. The
[erasure and runtime-IR specification](../../../spec/40-runtime/47-erasure-runtime-ir.md)
defines the normative boundary this implementation follows.

Continue with [Interpreter and runtime values](interpreter-and-values.md) for
the direct runtime-IR path, or [The native backend](native-backend.md) for the
implemented native route.
