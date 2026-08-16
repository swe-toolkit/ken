---
id: RT-ROOT-AUTHORITY-BLAME-DOMAIN
title: "The three root-authority guards report a compiler-owned invariant failure through the unsupported-construct channel, which reverses the fault domain -- and the correct arm, BackendFailure::PlannerInvariant, already exists with 40 producers in the same crate"
status: merged
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

> ### THE WARRANT IS STRONGER THAN DIAGNOSIS, AND THE SECOND HALF
> ### IS THE LOAD-BEARING ONE
>
> **Architect `evt_7f3216zdn03yy`, added at their direction.** The frame above
> rests the case on **diagnosis** — a user told "unsupported" has no change to
> make. **True, and sufficient. It is also not the worst consequence.**
>
> `differential_error_report` and `runtime_ir_comparison_error_report`
> (`artifact/api.rs:1062`, `:1092`) map `Unsupported` to verdict `Unsupported`
> and `Backend` to verdict `BackendFailure`.
>
> ⇒ **While these three guards were `Unsupported`, a planner invariant firing
> under differential testing rendered as a SKIP** — the example silently dropped
> from the comparison, **indistinguishable from a genuine capability gap.**
>
> `45 §4` BE-Differential nets only what it actually compares, and `45 §2` puts
> the backend outside the TCB **on the strength of that net**. **So the
> misclassification was not only telling the user the wrong thing — it was
> removing the failure from the mechanism that substitutes for trust.**
>
> **This candidate converts three silent skips into loud failures. That is a
> soundness-adjacent gain, not a wording preference.**

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

**`D2` — update the test pins.** `control.rs:978` and `:995` match on
`reason ==`, and `:6888` constructs the string. These assert the **wrong**
classification today, so they must move with it — **a green test pinning the
defect is not evidence the behaviour is right.**

> ### THOSE THREE ARE A STARTING POINT, NOT THE CENSUS. MEASURED THE HARD WAY.
>
> **`D0`-`D2` at `de6cc12c1` went red in CI on a FOURTH pin** the frame never
> named: `d2k_1b_unmarked_seeds_refuse_and_resolve_no_fusion_plane`, assertion
> `control.rs:35922`, expected value `:35936`, still expecting
> `Some("NativeJoinPlanV1")` for `row1-owned-scope`.
>
> **`row1-owned-scope` occurs 15 times in `control.rs`**, and 21
> `NativeJoinPlanV1` mentions survive at the candidate. **Most are correct** —
> other `NativeJoinPlanV1` refusals that genuinely stay `Unsupported` — so this
> is an attribution job, never a bulk replace.
>
> ⇒ **Run a census over the file and state the command and its count in the
> handback.** Do not enumerate from this frame's list. **That list was mine and
> it was wrong.**
>
> **The run also cancelled 3 tests still executing**, so a red list from a
> cancelled run is a lower bound. Re-run to completion before handing back.

> ### THREE CENSUS MISSES ON THIS ONE NODE. THE PATTERN IS THE LESSON.
>
> | miss | whose | what it measured instead |
> |---|---|---|
> | `AC-3` written as a universal | Steward | a line-local grep, blind to the multi-line calls that ARE its subject |
> | the `63` in the `D0`-`D2` handback | ring, unchecked by QA | no population at all — not 32, not 53, not 114 (Architect `evt_7f3216zdn03yy`) |
> | `D2`'s three-pin list | Steward | three of at least four live pins |
>
> **The `AC-3` amendment exists specifically to teach that a census can fail to
> see its own subject, and the failure then recurred twice inside its own
> discharge.** State the command and the number, so the next reader re-runs it
> rather than trusting it.

## Acceptance criteria

**`AC-1`. No guard is deleted or weakened.** This node changes **which fault
domain the failure is reported in**, never whether it fires. A candidate that
makes any of the three stop refusing fails outright.

**`AC-2`. The message text is preserved.** These strings are the only
description of the invariant that exists; the spec has none. Reclassify the
carrier, not the content.

**`AC-3` (AMENDED 2026-08-16 — the original was a universal the Steward never
measured; see the block below). The three named root-authority messages no
longer use an `unsupported("NativeJoinPlanV1", ...)` carrier**, and **every
remaining hit is attributed as outside the root-authority guard set.**
Attribute each hit to its cfg profile before counting it. **Wording supplied by
runtime-leader `evt_78ahsj1ge0npp` and confirmed verbatim by the Steward.**

> ### THE ORIGINAL `AC-3` DEMANDED ~33 SITES. `D1`/`D2` SCOPE THREE.
>
> It read *"verify by grep that no `unsupported(` call site names it."*
> **Discharging that literally would have reclassified the complete
> `NativeJoinPlanV1` protocol** — far outside the three messages and two test
> pins this node frames. Runtime-leader caught it before widening and asked
> rather than guessing. **That was the correct move and the scope stays where
> `D1`/`D2` put it.**
>
> **Steward's own re-census, 2026-08-16, `origin/main`** — `unsupported(`
> called with `"NativeJoinPlanV1"` as the construct tag:
>
> | file | sites | profile |
> |---|---|---|
> | `lowering/mod.rs` | 19 | production — the `cfg(test)` blocks begin at `:22241`, after every hit |
> | `lowering/core.rs` | 13 | production |
> | `planning.rs` | 2 | `:170` production; `:336` is a test assertion under the `cfg(test)` block at `:219` |
>
> ⇒ **~33 production sites carry the tag. Exactly three are this node's.**
>
> **How the AC got written: a line-local grep.** The three guards span the
> `unsupported(` call across multiple lines, so a single-line
> `grep 'unsupported("NativeJoinPlanV1"'` returns **one** hit — `planning.rs:170`,
> which is not one of them. **The census that produced "no call site names it"
> could not see its own subject.** A multi-line-aware grep returns 34.
>
> **The tag is therefore not a discriminator for the root-authority guards**,
> and any future AC keyed on it must name the guards, never the tag.

> ### THE OTHER ~30 SITES ARE A QUESTION, AND THEY ARE NOT THIS NODE'S
>
> The same fault-domain test that condemns these three — *"who had the last
> valid chance to satisfy the condition?"* — has **not** been applied to the
> rest of the protocol, and some of them may be equally misclassified. **Others
> will be genuine `Unsupported`**, since `NativeJoinPlanV1` decode failure on
> client-supplied metadata (`planning.rs:170`) is a real external-input case.
>
> **Recorded so it is not lost, deliberately not filed.** It is a third-lane
> filing under the operator's 2026-08-15 two-lane directive and it queues.
> **Do not let it grow this node** — `AC-1` through `AC-6` are unchanged.
>
> #### SUPERSEDED TWICE. THE ANSWER IS A DECISION PROCEDURE, NOT A SURVEY.
>
> **This block has been wrong in two different directions and the history is
> kept because the second correction is easy to mistake for the first.**
>
> **v1 (mine):** *"some may be misclassified, others genuinely `Unsupported`."*
> Assumed each site sorts whole.
> **v2 (Architect `evt_7f3216zdn03yy`):** `mod.rs:18369` is **mixed-blame** —
> one refusal over a disjunction spanning both domains, so it must be split.
> **v3 (Architect `evt_4t0k30t0yet13`) — CURRENT. v2 IS WITHDRAWN BY ITS OWN
> AUTHOR.**
>
> ### THE LINE IS THE MINT
>
> `:7797`'s guard lives in `take_distinguished_root_answer_authority`
> (`mod.rs:18649-18677`) and fires when **the checked package's own metadata**
> supplies no qualifying distinguished-root site. **That is input-determined, so
> `Unsupported` is correct for it** — and for its sibling, *"multiple
> distinguished root join sites"*.
>
> ⇒ **Before the mint, the authority's existence depends on the input artifact.
> After the mint, only the compiler can lose, misroute, or duplicate the token.**
>
> **Why that settles `:18369` rather than splitting it.** The mint-time filter
> has **already established every one of its input conjuncts** for the site the
> authority names — `runtime_frame_fingerprint`, `checked_occurrence_path`,
> `answer_kind`, `process_object`, and `occurrence_binding_fingerprint` against
> a compiler-computed value. **So a POST-MINT failure of an input conjunct means
> the plan changed under the compiler, or the authority names a different site.
> Compiler bug either way.**
>
> ⇒ **`:18369` is FULLY compiler-owned, not mixed. So is `:18351`**, which v2
> left as an open question. Both are `PlannerInvariant` candidates on the same
> reasoning as the three that moved in this node.
>
> ### WHAT THE QUEUED THIRD-LANE NODE SHOULD SAY
>
> **Not "survey ~30 sites case by case." One question:**
>
> > **Is the guard upstream or downstream of the mint?** Downstream is
> > compiler-owned, **because the input facts were validated upstream.**
>
> **This is the shape the Architect explicitly asked the Steward to carry**
> (`evt_4t0k30t0yet13`), and it is worth more than any number of individual
> findings: it converts an open-ended audit into a test. **It is scoped to this
> token** — do not assume it generalizes to other refusal families without
> re-deriving the boundary for each.

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

> **Discharged, and the consequence is carried.** Row 1 no longer owes the
> binding. [[RT-UNSUPPORTED-BINDING-ON-REFUSAL]]'s **scope is unaffected** — its
> lede binds the obligation to **any** unsupported construct, explicitly *"not
> for the five that `RT-UNSUPPORTED-LANE-REFUSAL-REACH` measured"*. **What is
> stale is its `D1` input list**, which offers those five populations as
> starting material. One of them has left the set. A line there, not a reframe.

> ### THE NEW EXPECTATION IS LINE-SPLIT, AND A FUTURE AUDIT WILL NOT SEE IT
>
> **Architect `evt_4t0k30t0yet13`, non-blocking, recorded rather than recut.**
> The recut's expectation splits the message across a `\` continuation
> (`control.rs:35107-35109`), so a whole-message grep misses it.
>
> **Measured at the recut:** `affine checked-root authority` occurs **6** times
> in `control.rs`; a whole-message grep finds **2**. **Four are invisible to
> the obvious probe**, and the result comes back clean rather than empty —
> the failure direction that reads as success.
>
> ⇒ **Same hazard class as the line-local grep that produced the `AC-3`
> amendment at the top of this node.** The difference is that this one is
> **recorded before it bites** instead of after a red CI run. Any future audit
> of this message must be multi-line-aware.

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

## Post-merge: the Adversary challenged the reclassification and it was ruled

**Adversary `evt_3x1r2e6b0fzh5`, Steward routing `evt_4d60jszj7jvwf`, Architect
ruling `evt_2n4d1pheyw3se`. Recorded here because a ruling that lives only in a
thread is invisible to whoever reopens this node.**

**The challenge:** `Unsupported` was cause-**neutral** — *"this construct is not
supported"* is true whether the cause is a compiler defect or a capability gap.
`PlannerInvariant` renders *"please report this compiler bug"*, which **asserts**
the cause, and everything shown to reach these three guards is a hand-built
`RuntimeExpr` fixture. The messages occur only in `mod.rs` and `control.rs` —
nowhere a Ken source program is compiled.

**Ruled: it does not hold, and no recut is asked for.**

> **`PlannerInvariant` is not a description of evidence. It is a causal category
> this codebase defined before this node** — `static_transition.rs:12753`:
> *"ambiguity here is a compiler bug rather than a program the backend cannot
> handle — so it is a `PlannerInvariant`, not a capacity refusal."*

⇒ The string does not **assert** a cause from fixture evidence; it **renders a
category whose membership criterion is itself causal.** The only question this
node had to answer is whether the three guards belong in that category, and the
mint boundary answers it structurally. **The instruction clause is licensed:
its sole precondition is fault, and fault is what the category establishes.** It
would be unlicensed only if the category admitted non-compiler causes, which
`12753` denies for the category and the mint boundary denies for these three.

⇒ **A population test was applied to a claim that is not a population claim.** A
guard that never fires is neither more nor less compiler-owned than one that
fires hourly. The measurement is accurate and bears on **how often a user meets
the message, not on whether it is true when they do.**

**The census cause is now closed too.** The Adversary ruled out the metric
explanation and reported cause unknown; it is the ref. `de6cc12c1` — the first,
CI-red cut — is what yields 21. **State the base, the tip, and the count as
three fields**; the Architect published a wrong figure within the hour by
measuring against a worktree branch tree rather than an explicit `origin/main`
ref, which is this discipline failing in its own author's hands.

> ### BUT THE SAME STRING CARRIES A FALSE CLAUSE, AND IT IS NOT THIS ONE
>
> Neither the Adversary nor the Steward named it. The message reads *"**native
> static transition planner** invariant failed; please report this compiler bug:
> {msg}"* — and **16 direct producers are resident in lowering** (`core.rs` 13,
> `mod.rs` 3, including the three this node moved). For those it names a
> subsystem the failure did not occur in.
>
> **This node INHERITED that defect rather than creating it** — it has been wrong
> for the 13 `core.rs` producers since before the node existed, which is also why
> reviewing the diff could not have caught it. **This node is correct and stays
> merged.** The repair is [[RT-PLANNER-INVARIANT-MESSAGE-LOCALIZATION]].

## Provenance

Research advisory `evt_nw85nh58a7dd`, on the Steward's request `evt_65nfvahn43hhe`,
commissioned on the operator's prompt in session 2026-08-16. **Every coordinate
re-read against the tree by the Steward before filing:** the enum at
`surface.rs:169`/`:187`, the three guards at `mod.rs:18296`/`:18307`/`:18340`,
the token at `:16486`, the mint sites at `core.rs:3453` and `units.rs:6117`, the
40 `PlannerInvariant` producers, the two test pins at `control.rs:978`/`:995`
and the constructed string at `:6888`, and the zero-hit `spec/` grep.
