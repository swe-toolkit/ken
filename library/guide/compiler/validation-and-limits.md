# Validation and limits

> **Availability:** partial. **Authority:** explanatory.

A compiler pipeline has several places that can reject an input, and each
place establishes a different fact. Reading a successful later-stage report as
if it subsumed every earlier check makes the implementation look stronger than
it is. This page separates the checks currently represented in Ken: kernel
admission of core declarations, package validation, bounded runtime-artifact
validation, execution observations, and native packaging evidence. The
[erasure and runtime-IR specification](../../../spec/40-runtime/47-erasure-runtime-ir.md)
is the normative authority for the boundary below checked core.

## Admission and package validation

The [kernel checker](../../../crates/ken-kernel/src/check.rs) is the admission
point for explicit core terms and declarations. Its bidirectional `check` and
`infer` operations establish typing relative to a context and global
environment; declaration admission also rechecks inputs and applies the
inductive positivity gate. That is a claim about the kernel's core language.
It is not a claim that a package can be serialized, erased, evaluated, or
lowered to native code.

After elaboration and kernel admission, the
[checked-core representation](../../../crates/ken-elaborator/src/checked_core.rs)
forms a package-facing boundary before erasure and runtime IR. It replaces
producer-local kernel identifiers with stable package symbols and canonically
encodes the semantic inputs used for the package hashes. Module-path atoms are
therefore package data rather than new kernel features. Its package validation
checks the header, semantic contract, and dependency-hash lane coherence; a
consumer that wants a target closure also checks the recorded lowerability
metadata.

These checks are complementary, not interchangeable. Kernel admission says
that a core declaration meets the kernel's rules. Checked-core validation says
that a package has the expected identity and internally coherent encoded
metadata. A valid package is not thereby a proof that every declaration is
lowerable. Conversely, a lowerability block is a deliberate consumer refusal,
not evidence that kernel admission was unsound.

## The erasure boundary

Erasure consumes a `CheckedCorePackage`, not raw source text. The specification
requires stable symbols, semantic and artifact hashes, obligations,
assumptions, trust metadata, dependency information, effect and capability
facts, and lowerability status to remain available as runtime-artifact
metadata. It also requires a consumer to refuse before backend work when a
reachable closure contains an unsupported entry, missing or blocking
lowerability metadata, an unmodelled effect or foreign boundary, or a case
whose meaning would depend on backend layout or native ABI.

That refusal is significant. It is not a source-language fallback and it does
not authorize a backend to invent an operation for an unsupported construct.
The runtime IR is backend-neutral: it makes selected data, control, effects,
traps, and calls operational, but it does not define Cranelift instructions,
object format, pointer identity, or native ABI semantics. Passing through this
boundary is thus neither a lowering proof nor a whole-compiler correctness
claim.

## Bounded artifact validation

The [runtime artifact validator](../../../crates/ken-runtime/src/artifact_validation.rs)
recomputes facts from one concrete `RuntimeProgram` and compares them with a
certificate bound to that program's package identity, core semantic hash, and
artifact hash. Its supported-artifact report covers a bounded subset: package
and declaration effects, capabilities, runtime checks, and trust metadata;
reachable unsupported entries and lowerability; and foreign or effectful
boundaries. A missing claim, identity mismatch, or recomputed fact mismatch is
an error rather than a successful validation with weakened meaning.

The same module can recompute a proof-erasure boundary witness. That witness
records concrete runtime declaration targets, field statuses, lowerability,
unsupported entries, obligations, assumptions, and trust data. It is useful
because its lanes must agree with the exact runtime artifact. It remains a
bounded witness: the module explicitly does not certify Cranelift, native
execution, object layout, or whole-compiler correctness. In particular, a
successful witness cannot turn a later lowering result into a semantic proof.

## Evaluation and comparison

The [runtime-IR evaluator](../../../crates/ken-runtime/src/runtime_ir_evaluator.rs)
executes `RuntimeExpr` directly from a runtime artifact and attaches the exact
artifact and target identities to its observation. Its differential report can
compare that observation with a caller-supplied interpreter observation. The
outcomes are intentionally three-way: agreement, mismatch, or unsupported.
Unsupported is not agreement with an omitted value, and mismatch remains a
reported difference rather than a diagnosis of which implementation is wrong.

The comparison relation is restricted to supported returned ground values and
explicit traps. It does not compare native addresses, object layout, timing,
allocation order, optimization behavior, backend traces, or proof objects. An
evaluator run also records unavailable evidence lanes for native backend,
object artifact, linker, and source-level proof validation. Therefore evaluator
success is a runtime-IR observation, not kernel evidence, source-semantics
proof, native validation, or broader artifact-validation evidence.

The [native artifact API](../../../crates/ken-runtime/src/cranelift_backend/artifact/api.rs)
uses the bounded certificate before its validated seed route, then performs
program admission and rejects remaining blockers before native execution. This
order preserves the distinction between admitting an artifact to the bounded
lane and determining whether its reachable target subset can proceed. Native
comparison reports can be useful evidence for their identified inputs; they do
not extend the evaluator's comparison relation into a proof of lowering.

## Packaging evidence

The [object-packaging route](../../../crates/ken-runtime/src/object_linker_packaging.rs)
records object, linker, build, and smoke-run facts for a narrow starter host
target. A smoke run can demonstrate that a selected artifact was built and
launched in that environment. It cannot make native bytes, linker success, or
toolchain behavior into Ken semantic authority. The module states that these
are evidence artifacts rather than proof evidence.

Secondary backend targets, native-library output, C or Rust interoperation,
cross-package native linking, translation validation, and whole-compiler
verification are not supplied by this route. Effects, foreign paths,
capabilities, trust metadata, and unsupported lowerability can also make a
target unavailable to a particular evaluator or backend lane. Treat every
report according to its named artifact, target, and evidence tier, then return
to the [compiler architecture](README.md) or the
[reading workflow](reading-workflow.md) to trace the appropriate boundary.
