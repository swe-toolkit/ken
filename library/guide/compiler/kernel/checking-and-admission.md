# Checking and admission

> **Availability:** partial. **Authority:** explanatory.

The [checking implementation](../../../../crates/ken-kernel/src/check.rs) has
two complementary entry points. `infer` synthesizes a type for an inferable
core term, while `check` verifies a term against a supplied type. `check`
handles introduction forms such as lambdas and pairs against their expected
shape; its mode-switch path calls `infer` and then asks conversion whether the
two types agree.

`raw_well_formed` checks only structural scoping. It does not establish that a
term has a type. The later `infer` and `check` mechanisms decide typing and can
refuse an out-of-scope variable, an unknown declaration, an unsuitable
introduction form, or a mismatch between an inferred and expected type.

`declare_inductive` is an admission path for inductive declarations: it builds
the family data, provisionally adds it to the environment, and removes it again
if validation fails. The [inductive-family page](inductive-admission.md) follows
that specialized boundary. The normative typing and declaration-admission rules
are in the [checking specification](../../../../spec/10-kernel/18-judgments.md).
