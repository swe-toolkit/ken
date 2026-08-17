---
id: RT-REFUSAL-PIN-ABSENCE-CLAUSE
title: "Pin 2 of the re-homed refusal pins asserts two contains() clauses, so it pins what the refusal must SAY and nothing about what it must NOT say -- the temporal phrasing D1c refuted can be re-added beside both pinned clauses and every assertion still passes. Add the absence clause while both lanes still exist"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Adversary hunt evt_3t7y5zwng8aba on merged RT-REFUSAL-PINS-REHOMED (a48c28915...d6a9760a9), 2026-08-17. Confirmed in the tree by the Steward before filing. Cut separately rather than folded into RT-DESCENT-RETIRE because folding it would make the repair ride a gated node, which is the defect's own shape."
---

Frame: this node. **One deliverable, one line of test code, one mutation.**

## The finding

**Pin 2, `refusal_pins_rehomed_static_worker_without_selector_exclusion`, asserts
by `contains`:**

```rust
}) if reason.contains("this recognition's own transport never reaches a consumer at an exact-Var call")
    && reason.contains("a constructor carrying an unconsumed static worker denotes a value containing the callable and has no runtime representation")
```

**Both consequences are genuinely pinned, one `contains` each**, and those are
exactly the two clauses [[RT-REFUSAL-CONSEQUENCE-RESTORE]] restored. That half
works.

**`contains` asserts what IS there and says nothing about what ELSE is.**
[[RT-SECOND-RECOGNITION-ERASURE]] exists because this message once read *"nor
erased **before construction**"* — the temporal phrasing that invites the
transfer reading `D1c` **refuted**.

⇒ **That phrasing can be re-added beside both pinned clauses and every assertion
still passes. The two restorations are guarded; the removal is guarded by
nothing.**

## Why this is not hypothetical

**`"before construction"` is live vocabulary in the same crate**, verified at
`origin/main` by the Steward: `control.rs:35025`,
`lowering/mod.rs:4270,4301,4532,4675`, and `planning/static_transition/abi.rs`.
A future edit reaching for that wording is reaching for language already in use
next door.

**Pin 1 has no such gap** — it asserts `reason == "..."` in full equality.
**The two pins are not equally strong, and only one of them was measured for
this property.**

## Why it is worth a node now rather than at `D6`

**After [[RT-DESCENT-RETIRE]]'s `D3`, this pin is the only thing asserting this
refusal's text at all.** The re-homing exists precisely so the assertion
survives the deletion; an assertion that survives but cannot detect the
regression it was written for survives in name only.

**Folded into the retirement it would ride a gated node and stay unfixed while
being green** — the same reasoning the Architect used to cut the helper-evidence
defect separately, and the same shape the defect itself has.

## Deliverable D1 — add the absence clause and prove it detects

**One line, in the same guard:**

```rust
&& !reason.contains("before construction")
```

**Then prove it is a detector, not a decoration** (`AC-9`'s standing
requirement, and this campaign's repeated finding): in a **disposable** tree,
re-add the temporal phrasing to the production refusal, observe **pin 2 red**,
report the perturbation and the failure, and **revert before offering any
candidate**.

**Confirm pin 1 stays green under that perturbation** — the arms must remain
independently meaningful.

## Acceptance criteria

**AC-1.** Pin 2 fails when the production refusal carries `"before
construction"` alongside both currently-pinned clauses. **Observed, not argued.**

**AC-2.** Pin 2 still passes at unmodified `origin/main`, and pin 1 is
untouched and green.

**AC-3.** No production change. This node edits one test guard and **deletes
nothing**.

**AC-4.** `D2c` `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f` untouched.

**AC-5.** No-regression, green **in CI** (`COORDINATION §12`). Local runs
targeted only — `-p ken-runtime` or `--test`, never `--workspace`.

## Banned scope

- **Rewriting pin 1.** It uses full equality and has no gap; changing it trades
  a stronger assertion for a uniform one.
- **Any `D3`-`D8` retirement work.**
- **Widening to other `contains`-style assertions in `control.rs`.** If the
  pattern looks general, **say so and stop** — that is a census, not this node.

## Standing

**The Adversary edge is report-only.** This node is the triage; no reply is
owed to `evt_3t7y5zwng8aba` and none may be sent.
