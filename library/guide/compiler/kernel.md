# The trusted kernel

> **Availability:** partial. **Authority:** explanatory.

[`ken-kernel`](../../../crates/ken-kernel/src/lib.rs) is the trust root for
fully explicit core terms. Its public surface includes checking, conversion,
environments, inductives, substitution, and terms. The crate explicitly forbids
unsafe Rust. The surrounding compiler may construct a candidate term, but it
cannot turn that candidate into a valid Ken proof without this checker accepting
it.

The kernel is separate from the elaborator. The elaborator resolves and
elaborates surface declarations; the kernel checks the resulting core term
against its type and decides the conversion relations it implements. This
separation is why a successful parser or resolver is not an admission result.

## Kernel mechanisms

Read [explicit core terms](kernel/core-terms.md) for the representation the
kernel receives, then
[contexts and declarations](kernel/contexts-and-declarations.md) for its lookup
state. [Checking and admission](kernel/checking-and-admission.md)
shows how `infer`, `check`, and declaration admission decide a candidate.
[Conversion and reduction](kernel/conversion-and-reduction.md) follows the
reducer and equality decision, while [inductive-family
admission](kernel/inductive-admission.md) covers positivity and the later iota
consumer.

## What does not cross the boundary

The kernel does not receive Cranelift instructions, object files, host ABI
layout, or native machine-code behavior as proof evidence. Later compiler
stages consume checked artifacts and can report tests, validation, assumptions,
or unavailable lanes, but those reports do not extend the kernel's authority.

The runtime interpreter is also outside the type-soundness trust root. It runs
already-checked terms and is used as an execution reference. Its observations
are useful evidence about execution, not proof that a term type-checks.

## Reading onward

The [front end](front-end.md) shows how source reaches a checker invocation.
After admission, [artifacts and erasure](artifacts-and-erasure.md) explains the
checked-core package passed to runtime-oriented stages. The normative account of
the trust boundary is the
[kernel specification](../../../spec/10-kernel/11-syntax.md) and the
[trust model](../../../spec/60-security/64-trust-model.md).
