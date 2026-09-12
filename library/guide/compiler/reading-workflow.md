# Reading workflow

> **Availability:** partial. **Authority:** explanatory.

Use this workflow when a compiler question begins with a Ken source file or an
emitted artifact and you need to locate the implementation boundary that
answers it. It follows explicit producer and consumer seams. A successful
command or report is not evidence that every later stage ran, and this workflow
is not a proof that the stages agree. Start from the question you have, then
stop at the chapter whose boundary owns the answer.

## Start with source admission

For lexical, grammar, declaration, or name-resolution questions, begin with
[Source to checked core](front-end.md). Its route begins at the
[elaborator entry module](../../../crates/ken-elaborator/src/lib.rs) and then
separates lexing, parsing, resolution, and elaboration. Follow a file or
declaration API into the relevant front-end module before drawing a conclusion
about the resulting term. The source route reaches the kernel only when
elaboration submits an explicit core term for checking.

A question such as “why did this source name not resolve?” belongs in the
front-end route. A question such as “does this explicit term type-check?”
belongs in [the kernel](kernel.md). Keep those domains distinct: elaboration
produces core input, while the kernel checks core input against its context and
global environment. Neither result implies that a checked-core package exists,
that erasure can consume it, or that a native target is available.

## Cross the package boundary

For a question about what later stages consume, turn to
[Artifacts and erasure](artifacts-and-erasure.md). Follow
[`CheckedCorePackage`](../../../crates/ken-elaborator/src/checked_core.rs) to
the [erasure entry point](../../../crates/ken-elaborator/src/erasure.rs), then
inspect the produced `RuntimeProgram` and its metadata. This is the route for
package identity, semantic and artifact hashes, lowerability, preserved
obligations or trust metadata, and the distinction between checked core and a
runtime artifact.

Do not use the package route to infer admission retrospectively. Checked-core
validation concerns the package-facing representation after elaboration and
kernel admission. It also does not settle whether every reachable target can
be lowered: lowerability and unsupported records are consumer-facing facts that
may refuse a particular route. For the normative meaning of erasure and runtime
IR, follow the specification link in that chapter rather than treating this
workflow as a second statement of the rule.

## Select an observation route

For interpreter behavior and runtime values, continue to
[Interpreter and runtime values](interpreter-and-values.md). That chapter
connects the reference-interpreter API, evaluator, environment, and store. It
is the appropriate route for an observation made by interpreting a supported
term. An interpreter observation is useful evidence about that run; it does
not by itself validate runtime IR, native lowering, object emission, or linker
behavior.

For direct execution of the runtime artifact, continue from the artifact
chapter to the
[runtime-IR evaluator](../../../crates/ken-runtime/src/runtime_ir_evaluator.rs).
Its reports bind observations to an artifact and target identity, and its
comparison route makes agreement, mismatch, and unsupported distinct outcomes.
Read its preflight and unavailable evidence lanes before treating a comparison
as applicable. The [validation chapter](validation-and-limits.md) explains why
this remains a bounded comparison rather than a source-semantics or
whole-compiler proof.

For native output, use [The native backend](native-backend.md), then the
[native artifact API](../../../crates/ken-runtime/src/cranelift_backend/artifact/api.rs).
Read the admission or preflight path before the selected emission or execution
function. This ordering shows whether the runtime artifact and its reachable
target are accepted for the narrow backend lane. It does not make a fallback
path available: an explicit rejection remains the result for that target.

## Identify the evidence boundary

Finish at [Validation and limits](validation-and-limits.md) whenever the
question is what a result establishes. Classify the result as kernel admission,
package validation, bounded runtime-artifact validation, a proof-erasure
witness, evaluator observation, differential comparison, native report, emitted
object, or linked-artifact smoke evidence. The categories share artifacts but
not authority. In particular, object and linker facts demonstrate properties of
a selected build route; they are not Ken semantic authority.

If a route reports an unsupported construct, missing lowerability metadata, or
a blocked target, preserve that refusal in the investigation. Do not substitute
a nearby successful evaluator, package, or smoke report as though it answered
the rejected question. Return to the [architecture](README.md) to choose a
different question path, or to the
[compiler program](../../../docs/program/07-compiler-program.md) for the
project-level boundary and non-goals that this implementation guide does not
replace.
