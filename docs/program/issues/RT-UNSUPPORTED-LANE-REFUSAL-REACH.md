---
id: RT-UNSUPPORTED-LANE-REFUSAL-REACH
title: "CLOSED, complete negative result: none of the five refused populations reaches the 48 unsupported lane -- every one returns Err before artifact construction, so the repair belongs at 48 section 5.4's native-artifact binding (owed even when no native bytes exist) and NOT at compiled.unsupported, and it is owed independently of the operator's narrowing decision"
status: closed
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-16, on the Architect's spec read at evt_7wzkzpjmttbht answering the question routed at evt_71jgtxcsy1b20. The Architect named this measurement explicitly as unmeasured and as the fact that decides the disposition. It gates the operator scope call on retiring RecursiveDescent. Steward-filed per COORDINATION section 2."
---

> # CLOSED. `D0` RETURNED A UNIFORM **NO** ACROSS ALL FIVE POPULATIONS.
>
> Runtime `evt_7b5qy026214z7` / `evt_6ekhab1erhbds`, measured at exact
> `63644c71d01767205839ab4ad36697c02ba4b8ac`. **No candidate exists and none
> was owed** — this node was a measurement, so it never reaches `merged`.
>
> | population | reaches the `48` lane? | what was returned instead |
> |---|---|---|
> | row 1 owned-scope | **NO** | pre-artifact `Unsupported(NativeJoinPlanV1, "terminal answer has no affine checked-root authority")`, `lowering/mod.rs:18335-18340` |
> | row 4 depth 1 conservation | **NO** | pre-artifact `Unsupported(StaticWorkerBinding, ...)`, `lowering/mod.rs:4726-4740` |
> | row 5 after-hole conservation | **NO** | same |
> | row 4 depth 2 `close` | **NO** | same |
> | row 4 depth 3 `close` | **NO** | same |
>
> **The four static-worker payloads are the exact per-row `D2k-0` conservation
> sentences.** They are **error payloads, not lane records** — that distinction
> is the whole finding, and `AC-1` was written to force it.

> ### THE STRUCTURAL REASON THE LANE IS UNREACHABLE FROM AN ERROR PATH
>
> **This block establishes the NO. It does not settle the remedy** — see the
> binding-layer block below, which corrects the Steward's first reading.
>
> **`compiled.unsupported` is a field on a compile that SUCCEEDED.** Every site
> that reads it has the identical shape:
>
> ```rust
> let compiled = compile_*(...)?;              // Err short-circuits HERE
> let unsupported = compiled.unsupported.clone();   // reached only on Ok
> ```
>
> ⇒ **A refusal that returns `Err` produces no `Compiled`, therefore no lane
> record, and no contract report from which lane/target/construct/reason could
> be emitted.** Not a wiring miss — the lane is unreachable from an error path
> **by construction**.
>
> **The implementer measured through `test_objects.rs:54`, which is
> test-support** (`pub(crate)`, importing `new_object_module_for_lowering_tests`)
> **and NOT the production path the Architect named.** The Steward therefore
> re-checked the four production sites directly —
> `artifact/api.rs:370`, `:417`, `:879`, `:945` — and **all four carry the same
> `?`-before-copy shape.** ⇒ **The conclusion generalizes to production**, but
> it does so because the structure is shared, **not because the measured
> emitter was the right one.** A successor must not cite `test_objects.rs` as
> evidence about production.

> ### THE REMEDY IS AT THE BINDING LAYER, NOT AT `compiled.unsupported`
> ### Architect `evt_6gsyts7v5eg43`. This CORRECTS the Steward's first reading.
>
> **The Steward initially concluded that populating the lane required converting
> these refusals from compile-time `Err` into compile-time success carrying a
> record — a change to what the compiler PRODUCES. That is the wrong layer**,
> and the Architect names it as the obvious next mistake: it would mean
> producing an artifact for a compile that failed, **which is worse than the
> gap.**
>
> **Two mechanisms share the word `unsupported` and must not be conflated:**
>
> | | what it is | reachable from a refusal? |
> |---|---|---|
> | `compiled.unsupported: Vec<String>` | a **fact on a produced artifact** — constructs in emitted bytes that are not natively executable | **No, by construction** |
> | `CraneliftBackendError::Unsupported` | an **error that aborts compilation**, so no artifact and no fact list exist | it *is* the refusal |
>
> **The obligation sits one level up, at `48 §5.4`** (verified by the Steward at
> `spec/40-runtime/48-executable-artifact-contract.md:168-175`): *"The
> native-artifact binding is required **even when no native bytes exist.** It has
> exactly one status"* — `available` / `unavailable` / `unsupported`, the last
> carrying a stable lane, target symbol, construct, and reason. **A failed
> compile owes a binding and today yields none at all.** That is the repair site.
>
> ⇒ **It is a RECORDING repair after all** — just at the artifact-binding layer
> rather than the artifact-fact layer. **The operator ask does not grow.**
>
> **`:180` adds a consequence neither reading had reached:** *"An `unavailable`
> or `unsupported` marker is part of the contract hash."* ⇒ the missing binding
> is not merely an unreported fact; **it changes the contract hash**, which is
> why this is owed rather than nice to have.

> ### THE DECOUPLING — THIS IS NOT A TERM IN THE OPERATOR DECISION
>
> **The `48 §5.4` binding is owed no matter how the operator rules**, because
> `§5.4` requires it for **any** unsupported construct, not for these five in
> particular. **So it must not be bundled into the narrowing decision.** If it
> rides along, accepting the narrowing will read as discharging the reporting
> obligation — and it does not: **the same hole stays open for every future
> unsupported construct.** Two artifacts, two decisions.
>
> **The narrowing itself remains acceptable-in-principle and the operator's to
> accept.** Nothing here revives the closure crossing, and `41 §2.1` clause 3
> still grants nothing.

> ### THE CONSTRUCT HALF OF THE LANE WAS NAMED AND NEVER WIRED
> ### Verified by the Steward, not accepted from the citation.
>
> ```rust
> pub enum ExecutableUnsupportedLane {   // executable_artifact_contract.rs:182
>     RuntimeIrNativePhaseGate,
>     RuntimeIrTarget,
>     RuntimeIrConstruct,                // :185
> }
> ```
>
> **`RuntimeIrConstruct` has exactly two occurrences in the tree** — its
> declaration at `:185` and its serialization string at `:1423`. **Zero
> producers.** The only marker constructed in production is `RuntimeIrTarget`
> at `:871`, whose `construct` field is the fixed literal
> `"RuntimeIrProgramReport.unsupported_targets"` — **the name of the map it came
> from, not a Ken construct.**
>
> The Steward's one refinement to the Architect's read: there is a **second**
> `RuntimeIrTarget` construction at `:2065`, but it sits under the `#[cfg(test)]`
> gate opened at `:1486`, so the production claim stands exactly as stated.
>
> ⇒ **`48`'s lane is half-implemented: the target half is wired, the construct
> half was given a name and left with no producer.** That is the missing
> surface, and it is what a successor acts on.

## Why this existed: it gated an operator decision, and it was one measurement

**HISTORICAL — the framing below is preserved as written before `D0` ran.**

**Two refusals are converging on one operator scope call** — the recursor rows
that fail closed on the functionized lane, and the depth-2/3 static-worker
constructs left by the exhausted ledger campaign. Both are the same trade:
**retiring `RecursiveDescent` costs capability.**

**The Architect ruled the disposition is a PERMITTED NARROWING, conditionally**
(`evt_7wzkzpjmttbht`). The condition is the whole of this node:

> **Each refused construct must land in the `48` `unsupported` lane with its
> stable reason.** `48:210` makes that lane normative — *"for targets or
> constructs that must fail before native execution"* — requiring a stable lane,
> target symbol, construct, and reason (`48:175`), with `48:213` forbidding a
> consumer from reinterpreting it. **Ken is specified to have a way to not
> support a construct**, provided the refusal is recorded there and fails loudly
> before native execution.

⇒ **If they reach the lane, the narrowing is sanctioned and the operator is
asked to accept two recorded limitations.** **If they do not, that is a gap —
but a `48` gap, repairable by RECORDING**, not a `41` gap requiring the closure
crossing to be built. **Those are materially different asks**, which is why this
is measured before the operator is asked rather than after.

> ### DO NOT RE-DERIVE THE SPEC QUESTION. IT IS ANSWERED AND IT WAS MALFORMED.
>
> **`41 §2.1` clause 3 is a BOUND, not a GRANT** — *"may exchange ... **only**
> within one live runtime domain"* restricts where exchange may occur and
> creates no obligation, and `41:116` says the chapter *"fixes the observable
> validity boundary, not its implementation."* The generated-unit boundary does
> sit inside one live runtime domain — inside one **compiled module**, in fact —
> **and that YES does not make the refusal a gap.**
>
> **Also do not repeat the vocabulary collision.** A refusal recorded as
> *"conservative clause-2"* is wrong on its citation:
> `BoundaryTag::PersistentClosure` names a **lifetime band**, not durability,
> and clause 2 governs *durable publication* — canonical bytes, content digest,
> storage identity. **This seam produces none of those.**

## `D0` — does each refused construct reach the lane?

**The mechanism is real and wired, and that is NOT the question.** The Architect
verified it: `compiled.rs:29`, surfaced through
`artifact/api.rs:370/417/879/945` into the contract report, with `api.rs:454`
reconciling it against a recomputation. **What is unmeasured is whether these
two populations populate it.**

**Report, per population, as a table:**

| population | reaches the lane? | lane / target symbol / construct / reason as emitted |
|---|---|---|

1. **The refused recursor rows.** Row 4 depth 1 and row 5 after-hole (the two
   conservation rows), plus row 1 owned-scope at `NativeJoinPlanV1`. **Row 1 is
   the one at risk of going quiet** — it has neither a repair nor a recorded
   refusal, and every other row has moved, so a reader sweeping the campaign
   sees motion everywhere and reads it as done.
2. **The depth-2/3 static-worker constructs.** The `close` refusal from the
   ledger campaign, whose message is
   `lowering/mod.rs:4728-4740` and reaches a user through
   `surface.rs:249-252`.

**A refusal that is a bare compile failure — an `Err` that never reaches the
contract report — is a NO for that row**, however clear its message text.
**Reaching a user is not reaching the lane**, and the two are easy to conflate
here because the `close` refusal has an unusually legible message.

**`D0` is a READ plus, if needed, one instrumented compile per population. No
repair, no lane entry authored.** Recording what is missing is this node's
output; **writing the missing entries is `D1` and is not authorized here.**

## Acceptance criteria

**ALL DISCHARGED at closure.** `AC-1` — five explicit NOs, with the returned
payload quoted per row; the "four required fields" clause was vacuous on a NO
and is not a shortfall. `AC-2` — row 1 owned-scope answered on its own row at
its own construct, which was the criterion's whole purpose. `AC-3` — reported
as a NO and handed back with **no repair attempted and no lane entry
authored**, exactly as framed. `AC-4` — no spec re-litigation occurred.
`AC-5` — no candidate, so no CI run was owed.

**`AC-1`. Each population gets an explicit YES or NO**, with the four required
fields quoted as actually emitted where the answer is YES. **A "the mechanism
exists" answer does not discharge this** — the mechanism's existence is a
premise of the node, not its finding.

**`AC-2`. Row 1 owned-scope is answered separately**, not folded into "the
recursor rows". It stands at a different construct from the other two and is the
one a sweep is most likely to skip.

**`AC-3`. A NO is reported as a NO and handed back**, not repaired in place. The
disposition of a missing lane entry is a `48` gap and the operator ask depends
on knowing its size.

**`AC-4`. No spec re-litigation.** `41 §2.1`'s reading is ruled. A finding that
the refusal is substantively wrong is a hand-back, not a re-derivation.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Authoring `unsupported` lane entries.** That is `D1`, unauthorized here.
- **Building the closure crossing**, or reopening any of the five dead ledger
  dispositions — see [[RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]] and
  [[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]].
- **Opening act 1** (factoring the machine terminal to delegate to the
  producer-side path). Unauthorized, and unrelated to this measurement.
- **Changing any refusal's message text.** That is
  [[RT-REFUSAL-CONSEQUENCE-RESTORE]] and it is a different node.

## Sequencing

**Lane 1. Cheap, and it is the gate on the operator scope call** — the Steward
does not carry that decision until this returns. It does not compete with
[[RT-BOUNDARY-IGNORED-CORPUS-MEASURE]] (different files, different question) and
can be taken by whichever seat is free.

## Provenance

Architect spec read `evt_7wzkzpjmttbht`, answering the Steward's routed question
`evt_71jgtxcsy1b20` — itself the question
[[RT-CLOSURE-CROSSING-ELIMINATE]] recorded as *"route it before the operator is
asked to choose"* and which had never been routed. **The Architect stated this
measurement as explicitly not done and as the deciding fact**, which is the
whole basis for this node.
