# The trusted kernel

> **Availability:** partial. **Authority:** explanatory.

[`ken-kernel`](../../../crates/ken-kernel/src/lib.rs) is the trust root for
fully explicit core terms. Its public surface includes checking, conversion,
environments, inductives, substitution, and terms, and it forbids unsafe Rust.
The surrounding compiler may construct a candidate term, but it cannot turn that
candidate into a valid Ken proof without this checker accepting it.

The kernel is separate from the elaborator. The elaborator resolves and
elaborates surface declarations; the kernel checks the resulting core term
against its type and decides the conversion relations it implements. A
successful parser or resolver is consequently not an admission result.

## Explicit core terms

The [core `Term` type](../../../crates/ken-kernel/src/term.rs) is the kernel's
input language. Its variants represent universes, de Bruijn variables, global
declarations, dependent functions and pairs, applications, eliminators, and
other explicit forms the checker handles. It is not surface syntax: the
elaborator produces it before asking the kernel to admit a declaration.

In `Pi`, `Lam`, `Sigma`, and `Let`, the bound variable has index zero in the
body; the [substitution module](../../../crates/ken-kernel/src/subst.rs) supplies
the structural operations for that convention. The representation does not
repair a malformed de Bruijn reference. Later raw well-formedness and inference
consult a context and can refuse an out-of-scope index.

## Contexts and declarations

The [context and environment structures](../../../crates/ken-kernel/src/env.rs)
provide local and global lookup state. `Context::push` extends the local
telescope, and `Context::lookup` translates an index from its innermost
position. A lookup outside that telescope returns no type; `infer` is the later
consumer that turns this absence into a `VarOutOfScope` refusal.

`GlobalEnv` records top-level `Decl` values, including transparent definitions,
opaque declarations, inductive families, and primitives. It is append-only and
uses declaration identities for lookup. The declaration data records whether a
definition is transparent, while the conversion reducer later decides whether
and how it unfolds. These data structures do not themselves admit declarations.

## Checking and admission

The [checking implementation](../../../crates/ken-kernel/src/check.rs) provides
`infer`, which synthesizes a type for an inferable core term, and `check`, which
verifies a term against a supplied type. `check` handles introduction forms such
as lambdas and pairs against their expected shape. Its mode-switch path calls
`infer` then asks conversion whether the inferred and expected types agree.

`raw_well_formed` checks structural scoping only; it does not establish that a
term has a type. `infer` and `check` are the later mechanisms that decide typing
and can refuse an out-of-scope variable, unknown declaration, unsuitable
introduction form, or mismatch. `declare_inductive` creates candidate family
data, provisionally adds it to the environment, then removes it if validation
fails.

## Conversion and reduction

The [weak-head reducer](../../../crates/ken-kernel/src/conv.rs), `whnf`, reduces
head beta, transparent-definition delta, projection, eliminator iota, let, and
ascription redexes. A neutral application or eliminator can remain stuck;
`whnf` returns that residual rather than evaluating arbitrary program behavior.

`convert` decides definitional equality at a supplied type. It uses weak-head
reduction and type-directed eta cases for functions and pairs; `convert_type`
supplies the structural type comparison the checker uses. The checking
mode-switch is the later consumer that accepts or refuses an inferred type
against its expected type.

## Inductive-family admission

`declare_inductive` builds a candidate family's former and constructor types,
then invokes validation. The [positivity checker](../../../crates/ken-kernel/src/inductive.rs)
contains `check_positivity`, which rejects family occurrences in forbidden
parameter, index, constructor-argument, or target-index positions. It validates
an inductive declaration; it does not execute an eliminator.

Eliminator computation is later work. The conversion reducer calls `iota_reduct`
only for a constructor-headed scrutinee in the stated family. `iota_reduct`
refuses mismatched parameter, method, level-argument, or constructor-spine
arities; when no iota reduct is available, `whnf` retains a stuck eliminator.

The [kernel syntax](../../../spec/10-kernel/11-syntax.md),
[checking judgments](../../../spec/10-kernel/18-judgments.md),
[conversion](../../../spec/10-kernel/17-conversion.md), and
[inductive specification](../../../spec/10-kernel/14-inductive.md) are
normative. This page identifies the Rust mechanisms that apply them.

## What does not cross the boundary

The kernel does not receive Cranelift instructions, object files, host ABI
layout, or native machine-code behavior as proof evidence. Later compiler
stages consume checked artifacts and can report tests, validation, assumptions,
or unavailable lanes, but those reports do not extend the kernel's authority.

The runtime interpreter is outside the type-soundness trust root. It runs
already-checked terms and is an execution reference. Its observations are useful
evidence about execution, not proof that a term type-checks.

## Reading onward

The [front end](front-end.md) shows how source reaches checker invocation. After
admission, [artifacts and erasure](artifacts-and-erasure.md) explains the
checked-core package passed to runtime-oriented stages. The normative trust
boundary is the [trust model](../../../spec/60-security/64-trust-model.md).
