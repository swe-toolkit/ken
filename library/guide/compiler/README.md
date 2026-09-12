# Compiler implementation guide

> **Availability:** partial. **Authority:** explanatory.

This guide is for engineers reading Ken's Rust implementation. It is an index
to the implemented route from source text through checked core and runtime IR
toward a narrow native starter path. The
[specification](../../../spec/00-overview.md) remains the language and runtime
authority. These pages identify the responsible implementation seams and the
limits attached to each observation; they do not specify the compiler or prove
that all of its stages agree.

The broad route is `lex → parse → resolve → elaborate → kernel-check →
CheckedCorePackage v0 → erasure → RuntimeProgram → native lowering`. This is a
navigation sketch, not a promise that every source program reaches every later
stage. The [compiler program](../../../docs/program/07-compiler-program.md#2-boundary)
names `CheckedCorePackage v0` as the durable boundary between admitted source
and later execution stages. Each chapter begins where the preceding artifact
ends, and each preserves its own supported subset and refusal boundary.

## Choose a question

Start with [the front end](front-end.md) for a question about source text:
which tokens and declarations are accepted, how names are resolved, and where
elaboration produces explicit core terms. That chapter stops at the handoff to
the kernel. A successful parse or resolution is not a kernel-admission result,
and a local name-resolution outcome is not by itself a statement about a
package or native artifact.

Continue to [the kernel](kernel.md) for a question about the trusted core:
term representation, contexts and global declarations, bidirectional checking,
conversion, and inductive admission. The kernel is the place to follow an
explicit core term submitted for checking. It is not the package producer or a
backend; use the next chapter when the question concerns what a later consumer
receives.

Read [artifacts and erasure](artifacts-and-erasure.md) for the checked-core
package, its identity and metadata, and the transition to runtime IR. This is
the route for questions about which package is being consumed, semantic versus
artifact identity, preserved metadata, proof erasure, or lowerability records.
The linked runtime specification carries the normative erasure boundary; the
chapter explains the implementation route to it.

## Choose an execution route

[Interpreter and values](interpreter-and-values.md) is the route for the
reference interpreter, call-by-value evaluation, environments, and stored
values. It explains an interpreter observation without presenting that
observation as an independent compiler proof. Follow the source links there
when the question is about evaluation behavior or the values manipulated while
evaluating checked terms.

[The native backend](native-backend.md) is the route for runtime-IR admission,
Cranelift lowering, native artifact identity, and the starter executable lane.
It separates the runtime-IR subset accepted for lowering from source fallback,
and it separates object or smoke evidence from semantic authority. Use it only
after locating the consumed runtime artifact; native output is not inferred
from kernel admission or from a successful interpreter run.

[Validation and limits](validation-and-limits.md) is the cross-cutting route
when the question is instead what a report establishes. It distinguishes kernel
admission, checked-core/package validation, bounded runtime-artifact and
proof-erasure witness checks, evaluator observations and comparisons, native
preflight, and object/linker packaging evidence. Its refusal paths are part of
the map: an unavailable target should remain unavailable rather than being
silently redirected to a nearby successful stage.

## Follow source anchors

The [reading workflow](reading-workflow.md) turns these choices into a
repeatable source-navigation sequence. It starts from the reader's question,
links to the chapter responsible for that boundary, and then points to the
entry module or artifact API to inspect. Use the chapter's source links for
implementation detail and its specification links where a normative rule is
needed. This keeps the guide useful as an implementation index without giving
it authority that belongs to the specification.

The guide remains partial. Secondary backend targets, native-library output,
C or Rust interoperation, cross-package native linking, translation validation,
and whole-compiler verification are outside this implemented-path map. A
successful result in one of the listed routes is evidence with the scope named
by that route, not a replacement for the missing paths.
