---
id: RT-ROOT-AUTHORITY-BLAME-DOMAIN
title: "The three root-authority guards report a compiler-owned invariant failure through the unsupported-construct channel, which reverses the fault domain -- and the correct arm, BackendFailure::PlannerInvariant, already exists with 40 producers in the same crate"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Research advisory evt_nw85nh58a7dd, commissioned by the Steward on the operator's prompt 2026-08-16 to unpack the root-authority refusal. Research's question 1 was load-bearing and answered decisively. Every coordinate below was verified against the tree by the Steward before filing. Steward-filed per COORDINATION section 2."
---

> # ROW 1 IS NOT A CAPABILITY NARROWING. TAKE IT OFF THAT COUNT.
>
> **This node exists because the Steward briefed the operator that row 1 was a
> capability loss of the same kind as the static-worker refusal. That was
> wrong and was withdrawn** (`evt_5yfzkef97sx48`). This is the durable form of
> the correction.
>
> **The static-worker refusal is a statement about Ken programs.** Row 1 is a
> statement about **the lowering machine's own proof obligation** — it reached
> emit time without discharging something only it could discharge.
>
> ⇒ **They must not be measured as one population, and row 1 must not appear as
> a term in the operator's narrowing decision** (`evt_7b2vh3pjvfcc6`).

## The defect: three guards on the wrong side of a distinction Ken already draws

`CraneliftBackendError` (`surface.rs:169`) **already separates the two fault
domains**:

```rust
pub enum CraneliftBackendError {
    Unsupported(UnsupportedLowering),   // { construct, reason } -- the PROGRAM
    Backend(BackendFailure),            // the COMPILER
}

pub enum BackendFailure {              // surface.rs:187
    Target(String), Verifier(String), Module(String),
    PlannerInvariant(String),          // <- the correct arm for this
    ...
}
```

**All three root-authority guards take the `Unsupported` arm**, verified in
`lowering/mod.rs`:

| line | text | arm used |
|---|---|---|
| `:18296` | *"checked root answer authority returned through the wrong outer cursor"* | `unsupported("NativeJoinPlanV1", ...)` |
| `:18307` | *"checked root answer authority was duplicated across source control"* | `unsupported("NativeJoinPlanV1", ...)` |
| `:18340` | *"terminal answer has no affine checked-root authority"* | `unsupported("NativeJoinPlanV1", ...)` |

**`BackendFailure::PlannerInvariant` is not hypothetical vocabulary — it has 40
producers in this crate today.** The correct classification is established
practice a few thousand lines away.

## Why this is a defect and not a wording preference

**The token is compiler-owned end to end.** `RootTerminalAnswerAuthority`
(`mod.rs:16486`) is minted **after** the kernel has admitted the program, at two
sites (`core.rs:3453`, `units.rs:6117`), moved by compiler code, and consumed
once by a `.take()` at result emission.

**Research's decisive test** (`evt_nw85nh58a7dd`):

> **Who had the last valid chance to satisfy the condition?** If only the
> compiler could mint, route, and consume the authority after admitting the
> program, **the user cannot discharge it.** A user-facing "unsupported
> construct" diagnosis at that point **reverses the fault domain.**

⇒ A Ken user shown *"NativeJoinPlanV1 is unsupported"* is told **their program**
is the problem and asked to change it. There is no change they can make.

## The prior art is uniform, and one of the sources is Cranelift itself

Research surveyed rustc, GHC, MLIR, LLVM, Lean 4, and Cranelift. **Once source
has passed admission and the compiler itself creates an obligation needed by
lowering, failure to discharge it is classified as an internal compiler
invariant violation.** Systems differ on the **transport** — abort, ICE,
assertion, returned verifier error — **but not on who is at fault.**

> ### THE CLOSEST PRECEDENT IS THE BACKEND KEN IS BUILT ON
>
> **Cranelift's own `CodegenError` draws exactly this line:**
> `Verifier(VerifierErrors)` is documented as *"always represents a bug, either
> in the code that generated IR for Cranelift, or a bug in Cranelift itself"* —
> and it carries a **separate `Unsupported(String)` variant.**
>
> **A returned, recoverable error is NOT automatically a user diagnostic.** That
> is the inference this defect rests on, and Cranelift refutes it in the same
> enum Ken's backend sits behind.

**Also load-bearing:** rustc's MIR validator emits `span_bug` (an ICE) for
broken MIR; GHC treats a Core Lint failure as a GHC bug; MLIR's pass contract
aborts on a definitively broken invariant; Lean 4 shows **both** transports in
one compiler — recoverable checker errors for reportable IR invalidity,
`unreachable!`/`panic!` for states represented as impossible.

## The category-4 escape does not apply. Measured, not assumed.

Research qualified its own answer: this would be a legitimate declared
implementation limit **only if Ken independently specifies that absence as an
expected, admitted backend limitation with a source-visible precondition.**

**The Steward grepped `spec/` for it. Zero hits** — no clause names the
checked-root authority, the affine token, or this absence. **Nothing in the spec
admits it as a limitation**, so the escape is closed and this is category 3.

## Deliverables

**`D0` — confirm the classification per guard, and say whether the three
differ.** All three are compiler-owned on the face of it, but *"duplicated
across source control"* and *"wrong outer cursor"* are protocol violations
detected mid-flight, while *"no authority"* is an absence at consumption. **Say
whether they warrant the same transport** — a returned `PlannerInvariant`, an
assertion, or a debug-only check. **Research is explicit that the transport
choice is a cost and localization question, not a blame question**, so do not
re-derive blame here.

**`D1` — move the guards to the correct arm.** `BackendFailure::PlannerInvariant`
with the existing message text. Follow the 40 existing producers' shape.

**`D2` — update the two test pins.** `control.rs:978` and `:995` match on
`reason ==`, and `:6888` constructs the string. These assert the **wrong**
classification today, so they must move with it — **a green test pinning the
defect is not evidence the behaviour is right.**

## Acceptance criteria

**`AC-1`. No guard is deleted or weakened.** This node changes **which fault
domain the failure is reported in**, never whether it fires. A candidate that
makes any of the three stop refusing fails outright.

**`AC-2`. The message text is preserved.** These strings are the only
description of the invariant that exists; the spec has none. Reclassify the
carrier, not the content.

**`AC-3`. `NativeJoinPlanV1` stops appearing as an `unsupported` construct
tag.** Verify by grep that no `unsupported(` call site names it, and
**attribute every hit to its cfg profile before counting it.**

**`AC-4`. Do not fold in the static-worker refusal.** That one is a genuine
statement about Ken programs, is in tension with `45 §3`, and is
[[SPEC-45-CLOSURE-IN-CONSTRUCTOR-EXCEPTION]]'s subject. **Sharing an error
channel between them is what erased the blame distinction in the first place**
— do not repair one by reference to the other.

**`AC-5`. Say what this does to the `48 §5.4` binding obligation.**
[[RT-UNSUPPORTED-BINDING-ON-REFUSAL]] requires an `unsupported` binding for
refused constructs. **If row 1 stops being an `unsupported` construct, it stops
owing that binding** — state that consequence explicitly. **Do not implement it
here**; that node owns it.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Repairing whatever would cause the invariant to fail.** This node fixes the
  **reporting classification**. Whether the invariant can fail at all is
  `Sequencing` below.
- **The static-worker refusal**, and any of the five dead ledger dispositions —
  see [[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]] and
  [[RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]].
- **Implementing the `48` binding change.** See `AC-5`.
- **Adding a spec clause to admit the limitation.** That would convert a
  compiler bug into a declared restriction by fiat, which is the opposite of
  this node's finding.

## Sequencing

**Does not block lane 1** and does not contend with it — this is classification
at the error surface, not closure-boundary or `RecursiveDescent` work.

> ### THE REACHABILITY QUESTION CHANGES SHAPE ONCE THIS LANDS, AND THAT IS THE POINT
>
> While row 1 is an `unsupported` construct, *"is it reachable?"* reads as a
> **capability** question — which Ken programs hit it. **Reclassified, the same
> question becomes *"is there a compiler bug?"*** — and an unreachable internal
> invariant is an ordinary, healthy state for an assertion to be in.
>
> **That is why this comes before any incidence measurement of row 1**, and why
> [[RT-STATIC-WORKER-WITNESS-PROGRAM]]'s `AC-4` barred folding row 1 into its
> count. **Measuring row 1's incidence as though it were a capability would
> have produced a number that means nothing.**

## Provenance

Research advisory `evt_nw85nh58a7dd`, on the Steward's request `evt_65nfvahn43hhe`,
commissioned on the operator's prompt in session 2026-08-16. **Every coordinate
re-read against the tree by the Steward before filing:** the enum at
`surface.rs:169`/`:187`, the three guards at `mod.rs:18296`/`:18307`/`:18340`,
the token at `:16486`, the mint sites at `core.rs:3453` and `units.rs:6117`, the
40 `PlannerInvariant` producers, the two test pins at `control.rs:978`/`:995`
and the constructed string at `:6888`, and the zero-hit `spec/` grep.
