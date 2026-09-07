# Compiler implementation guide

> **Availability:** partial. **Authority:** explanatory.

This guide is for engineers reading Ken's Rust implementation. It follows the
implemented route from source text to checked core, runtime IR, and the native
starter path. The language and runtime contracts remain in the
[specification](../../../spec/00-overview.md); this guide explains where the
implementation realizes or carries those contracts.

The pipeline is `lex → parse → resolve → elaborate → kernel-check →
CheckedCorePackage v0 → erasure → RuntimeProgram → native lowering`. The
[compiler program](../../../docs/program/07-compiler-program.md#2-boundary)
identifies `CheckedCorePackage v0` as the durable boundary between admitted
source and later execution stages.

## Reading the pipeline

Start with the [front end](front-end.md) to see how source reaches kernel
admission. Then read the [kernel boundary](kernel.md), followed by
[artifacts and erasure](artifacts-and-erasure.md). From there, choose the
[interpreter and values](interpreter-and-values.md) path for runtime-IR
observation or the [native backend](native-backend.md) path for the starter
native route.

[Validation and limits](validation-and-limits.md) names the checks and the
boundaries that prevent an implementation detail from becoming an unwarranted
claim. The [reading workflow](reading-workflow.md) turns this map into a
repeatable source-navigation order.

## Scope

This is not a compiler specification or a claim of whole-compiler
verification. The implementation contains supported subsets and refusal paths;
each page labels the guide partial and names the relevant boundary. Secondary
backend targets, native-library output, and a whole-compiler proof are outside
this guide's implemented-path map.
