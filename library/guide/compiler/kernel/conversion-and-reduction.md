# Conversion and reduction

> **Availability:** partial. **Authority:** explanatory.

The [weak-head reducer](../../../../crates/ken-kernel/src/conv.rs) is `whnf`.
It reduces head redexes, including beta, transparent-definition delta,
projection, eliminator iota, let, and ascription forms. A neutral application
or eliminator can remain stuck: `whnf` returns that explicit residual rather
than evaluating arbitrary program behavior.

`convert` decides definitional equality of values at a supplied type. It uses
weak-head reduction and type-directed eta cases for functions and pairs;
`convert_type` supplies the structural type comparison used by the checker. The
checking mode switch is the later consumer that uses `convert_type` to accept
or refuse an inferred type against an expected one.

Reduction and conversion are kernel mechanisms, not execution of a runtime
program or production of a native artifact. Later erasure and runtime stages
consume admitted terms under their own contracts. The normative conversion
rules are in the
[conversion specification](../../../../spec/10-kernel/17-conversion.md). Next
read [inductive-family admission](inductive-admission.md) for the family
mechanism behind eliminator reduction.
