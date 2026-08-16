---
id: RT-UNSUPPORTED-BINDING-ON-REFUSAL
title: "A compile that refuses a construct owes a 48 section 5.4 native-artifact binding with status unsupported, and today yields none at all -- the construct half of the lane was named as RuntimeIrConstruct and left with zero producers"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-16, on Architect disposition evt_6gsyts7v5eg43 answering runtime's RT-UNSUPPORTED-LANE-REFUSAL-REACH D0 negative result (evt_7b5qy026214z7). The Architect explicitly declined to authorize or size a build and handed the framing and sequencing to the Steward. Every cited coordinate below was re-verified against the tree by the Steward before filing. Steward-filed per COORDINATION section 2."
---

> # THIS IS NOT A TERM IN THE OPERATOR'S NARROWING DECISION. DO NOT BUNDLE IT.
>
> **`48 §5.4` requires the binding for ANY unsupported construct** — not for the
> five that `RT-UNSUPPORTED-LANE-REFUSAL-REACH` measured, and not as a condition
> on retiring `RecursiveDescent`.
>
> ⇒ **If this rides along with the narrowing decision, accepting that narrowing
> will read as discharging the reporting obligation — and it does not.** The
> same hole stays open for every future unsupported construct, and the next
> person to look will find an accepted decision sitting where the repair should
> be. **Two artifacts, two decisions** (Architect, `evt_6gsyts7v5eg43`).
>
> **It also does not contend with lane 1.** This is a recording repair at the
> artifact-binding layer; it does not touch the closure boundary, the ledger
> campaign, or `RecursiveDescent`, and **it inherits none of that campaign's
> five dead dispositions.**

## The defect: a failed compile owes a binding and produces nothing

`48 §5.4`, verified at
`spec/40-runtime/48-executable-artifact-contract.md:168-175`:

> *"The native-artifact binding is **required even when no native bytes
> exist**. It has exactly one status"* — `available`; `unavailable`, with a
> stable lane and reason; or `unsupported`, **with a stable lane, target symbol,
> construct, and reason.**

**Today a refusal produces no binding of any status.** Every compile path
aborts with `Err(CraneliftBackendError::Unsupported(...))` before any artifact
or report exists, so nothing downstream can record what was refused or why.

**`:180` is why this is owed rather than desirable:** *"An `unavailable` or
`unsupported` marker is part of the contract hash."* ⇒ the missing binding is
not an unreported fact on the side — **it changes the contract hash.**

> ### DO NOT ROUTE THIS TO `compiled.unsupported`. WRONG LAYER, AND IT IS THE
> ### OBVIOUS NEXT MISTAKE — the Architect named it as such.
>
> | | what it is | reachable from a refusal? |
> |---|---|---|
> | `compiled.unsupported: Vec<String>` | a **fact on a produced artifact**: constructs in emitted bytes that are not natively executable | **No, by construction** |
> | `CraneliftBackendError::Unsupported` | an **error that aborts compilation** — no artifact, no fact list | it *is* the refusal |
>
> **Wiring refusals into `compiled.unsupported` would require producing an
> artifact for a compile that failed, which is worse than the gap.** The
> measured evidence for the unreachability is in
> [[RT-UNSUPPORTED-LANE-REFUSAL-REACH]]: every site does
> `compile_*(...)?` **before** `compiled.unsupported.clone()`, at
> `artifact/api.rs:370`, `:417`, `:879`, `:945`.

## The construct half of the lane exists in name only

```rust
pub enum ExecutableUnsupportedLane {   // executable_artifact_contract.rs:182
    RuntimeIrNativePhaseGate,
    RuntimeIrTarget,
    RuntimeIrConstruct,                // :185
}
```

**`RuntimeIrConstruct` has exactly two occurrences tree-wide** — the
declaration at `:185` and its serialization string `"runtime_ir_construct"` at
`:1423`. **Zero producers.**

The only marker constructed in production is `RuntimeIrTarget` at `:871`, and
its `construct` field is the fixed literal
`"RuntimeIrProgramReport.unsupported_targets"` — **the name of the map it came
from, rather than a Ken construct.** A second `RuntimeIrTarget` construction at
`:2065` sits under the `#[cfg(test)]` gate opened at `:1486` and is not a
production producer.

⇒ **`48 §6`'s lane is half-implemented.** The vocabulary for this repair was
already built and never wired, which is why this is a wiring-and-plumbing node
rather than a design one.

## Deliverables

**`D0` — establish where a refusal can still produce a binding.** The refusal
aborts compilation, so the binding must be emitted by a caller that survives
the `Err`, not by the compile itself. **Name that site and show it has the four
required fields available** — stable lane, target symbol, construct, reason.
If no caller has all four, that is the finding and it is a hand-back, not a
licence to synthesize a placeholder.

> ### `D0` MUST ANSWER PER SITE-CLASS, NOT ONCE. Adversary `evt_3t1vb90y5yxwj`.
>
> **"All four sites share the shape" is true for the negative result and may be
> false for the repair.** The four split two and two:
>
> | site | constructs |
> |---|---|
> | `api.rs:370`, `api.rs:417` | `CraneliftObjectArtifact` |
> | `api.rs:879`, `api.rs:945` | `CraneliftRunReport` |
>
> ⇒ **`48 §5.4`'s obligation is a NATIVE-ARTIFACT binding.** The two
> run-report sites may fall outside it entirely, or may need a different
> treatment. **`D0` must say which**, because a single answer covering all four
> would either over-scope the repair onto run reports or silently skip them.
>
> **This distinction was absent from the node as first filed** and is a sizing
> input, not a detail.

**`D1` — emit the `unsupported` binding on the refusal path**, with
`ExecutableUnsupportedLane::RuntimeIrConstruct` as the lane and the refusal's
own stable reason. `D0`'s answer determines the site.

**`D2` — a control proving the binding appears**, keyed on a construct that
actually refuses. The five populations measured by
[[RT-UNSUPPORTED-LANE-REFUSAL-REACH]] are available as inputs.

## Acceptance criteria

**`AC-1`. No artifact is produced for a compile that failed.** The binding is
emitted **without** manufacturing a `Compiled`, a `CraneliftObjectArtifact`, or
emitted bytes. A candidate that makes a refusing compile return `Ok` to reach
the recording path **fails this AC outright** — that is the wrong-layer repair
this node exists to avoid.

**`AC-2`. All four fields carry real values.** Stable lane, target symbol,
construct, reason. **A fixed literal naming the map or the call site is not a
construct** — that is exactly the shortfall on the existing `RuntimeIrTarget`
producer at `:871`, and reproducing it here discharges nothing.

**`AC-3`. `RuntimeIrConstruct` acquires a production producer.** Verify by
grep: the variant must occur outside its declaration and serialization string,
in a non-`cfg(test)` path. **Attribute every hit to its cfg profile before
counting it.**

**`AC-4`. The control fails when the binding is dropped.** Demonstrate by
mutation — suppress the emission, show the control red, revert. **A green
assertion nobody has seen fail is not evidence the binding is emitted.**

**`AC-5`. No re-litigation of the narrowing.** Whether retiring
`RecursiveDescent` is acceptable is the operator's open decision and is not a
term here. A finding that some refusal is substantively wrong is a hand-back.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Local validation
targeted only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Making a refusing compile succeed** in order to reach the recording path.
  See `AC-1`; this is the specific failure the Architect flagged.
- **Building the closure crossing**, or reopening any of the five dead ledger
  dispositions — see [[RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]] and
  [[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]].
- **Changing any refusal's message text.** That is
  [[RT-REFUSAL-CONSEQUENCE-RESTORE]].
- **Repairing the existing `RuntimeIrTarget` producer's literal `construct`
  field.** Real, adjacent, and a separate cut — note it, do not fold it.

## Sequencing

**Queues behind lane 1's active chain.** It is `48` contract work rather than
`RecursiveDescent` retirement, so it does not compete for the same files, and
the Architect confirmed it does not contend. **Not started before the operator
has ruled on the narrowing** — not because it depends on that ruling (it does
not, and that is this node's governing point) but because lane 1 holds the
runtime ring's attention until then.

## Provenance

Architect disposition `evt_6gsyts7v5eg43`, on runtime's `D0` negative result
`evt_7b5qy026214z7` / `evt_6ekhab1erhbds`. **The Architect explicitly declined
to authorize a build or size one**, and handed framing and sequencing to the
Steward; the size above is the Steward's and is unvalidated by the ring.
