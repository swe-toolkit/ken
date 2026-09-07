# Reading workflow

> **Availability:** partial. **Authority:** explanatory.

Use this workflow when a compiler question begins with a Ken source file and
you need to locate the implemented stage that answers it. It follows artifacts
and module boundaries instead of treating a successful command as evidence
that every later stage ran.

## Start with the path you need

For a source-admission question, begin with
[Source to checked core](front-end.md). The
[elaborator entry module](../../../crates/ken-elaborator/src/lib.rs) names the
lex, parse, resolve, elaborate, and kernel-check sequence. Follow the
declaration or file API into the relevant front-end module, then cross the
boundary only when a core term is submitted to the kernel.

For a type-checking or proof-validity question, move next to
[The trusted kernel](kernel.md). Keep the elaborator and kernel as separate
search domains: the first produces explicit terms and the second checks them.
Do not infer native behavior from either result.

## Follow the artifact

For a question about what later stages consume, open
[Artifacts and erasure](artifacts-and-erasure.md). Follow the
[`CheckedCorePackage`](../../../crates/ken-elaborator/src/checked_core.rs) into
the [erasure entry point](../../../crates/ken-elaborator/src/erasure.rs), then
inspect the produced `RuntimeProgram` and its metadata. The artifact's identity
and lowerability records tell you which package and target the next stage is
considering.

For an execution observation, continue to
[Interpreter and runtime values](interpreter-and-values.md), then the
[runtime-IR evaluator](../../../crates/ken-runtime/src/runtime_ir_evaluator.rs).
For native emission, continue to [The native backend](native-backend.md),
then the
[native artifact API](../../../crates/ken-runtime/src/cranelift_backend/artifact/api.rs).
In both cases, look first for the preflight or admission path and then for the
selected execution or emission function.

## Close with the evidence boundary

Finish at [Validation and limits](validation-and-limits.md). Identify whether
the result is kernel admission, a bounded artifact validation, an evaluator
observation, a differential comparison, an emitted object, or a linked-artifact
smoke result. Each has a different scope. If the path is unsupported, retain
the refusal rather than substituting a nearby successful route.

Return to the [architecture](README.md) to choose a different path. The
[compiler program](../../../docs/program/07-compiler-program.md) provides the
project-level boundary and non-goals that this implementation guide does not
replace.
