---
id: RT-REFUSAL-PIN-ABSENCE-CLAUSE
title: "Pin 2 of the re-homed refusal pins asserts two contains() clauses, so it pins what the refusal must SAY and nothing about what it must NOT say -- the temporal phrasing D1c refuted can be re-added beside both pinned clauses and every assertion still passes. Add the absence clause while both lanes still exist"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Adversary hunt evt_3t7y5zwng8aba on merged RT-REFUSAL-PINS-REHOMED (a48c28915...d6a9760a9), 2026-08-17. Confirmed in the tree by the Steward before filing. Cut separately rather than folded into RT-DESCENT-RETIRE because folding it would make the repair ride a gated node, which is the defect's own shape."
---

Frame: this node. **Two deliverables, both small: `D1` is one line of test code
plus a mutation proof; `D2` is two comment corrections at named coordinates.**
`D1` is the reason the node exists and `D2` is folded in because it is the same
increment shape in the same crate — **make the artifact say what is true.**

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

## Deliverable D2 — two comment corrections at EXACTLY these two sites

**Architect `evt_2xm79ytjgz3j0`, both raised as should-fix at `D2c`'s approval
and explicitly not blocking it.** Folded here rather than cut as a third node:
they are comment-only edits in the same crate, in the same increment shape as
`D1` — make the artifact say what is true. **Coordinates verified by the Steward
in the candidate at `f68b8c866`, not transcribed.**

**A. `lowering/core.rs:2410-2413` is now FALSE.** The comment above the
`#[cfg(test)]` exclusion block still reads:

> *"…and let the remainder decide -- so a program still retained by some other
> variant keeps the retained lane and cannot be mistaken for this position
> working."*

**Under `D2c` the remainder decides nothing and no program keeps the retained
lane** — the block's early `return` was removed and the function now always
returns `FunctionizedUnits`. The function's *doc* comment was correctly updated
for `D2c`; this inner one was missed.

**Why it is worth fixing now rather than at `D6a`.** [[RT-DESCENT-RETIRE]]'s
`D6a` exists precisely to sweep reachability-premised comments that went false
silently, and **`D2c` created one inside the function the whole campaign is
about.** `D3` deletes the function, but `D3` is gated on this node, so **the
false comment sits on `main` for as long as this node takes.**

**B. `lowering/core.rs:2424` needs a because-clause.** The construct is correct
and approved:

```rust
let _ = recursive_descent_residual(expr).or_else(|| { ... });
```

**It keeps the classifiers evaluating while discarding the answer** — routing to
nothing without *running* nothing, so the reroute cannot mask a classifier
defect. **State that at the site, in one line.**

> **This is the FOURTH discarded-result site in this campaign — and the ONLY
> deliberate one.** The others were the sentinel's `_excluded_result`, the two
> `control.rs` trace helpers, and `compiler_driver.rs`'s `map_err(|_| …)`; each
> cost a measurement its explanation. **A future auditor sweeping that family
> needs one line to tell this apart from those three.**
>
> **A discard that is correct should say so at the site.** Apply it here going
> forward rather than reconstructing the intent later.

## Acceptance criteria

**AC-1.** Pin 2 fails when the production refusal carries `"before
construction"` alongside both currently-pinned clauses. **Observed, not argued.**

**AC-2.** Pin 2 still passes at unmodified `origin/main`, and pin 1 is
untouched and green.

**AC-3.** No production BEHAVIOUR change. This node edits one test guard and two
comments, and **deletes nothing**. `D2`'s edits are comment-only: control by
`git diff` showing no change to any expression in `select_body_emission_authority`.

**AC-4.** `D2c` `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f` untouched.

**AC-5.** No-regression, green **in CI** (`COORDINATION §12`). Local runs
targeted only — `-p ken-runtime` or `--test`, never `--workspace`.

## Banned scope

- **Rewriting pin 1.** It uses full equality and has no gap; changing it trades
  a stronger assertion for a uniform one.
- **Any `D3`-`D8` retirement work**, other than `D2`'s two named comment edits,
  which the Architect raised at `D2c` approval and routed here.
- **Widening to other `contains`-style assertions in `control.rs`.** If the
  pattern looks general, **say so and stop** — that is a census, not this node.
- **A general `D6a` comment sweep.** `D2` covers **exactly the two named sites**
  and nothing else. Other stale reachability comments are `D6a`'s, and finding
  one is a report, not a fix to make here.

## Standing

**The Adversary edge is report-only.** This node is the triage; no reply is
owed to `evt_3t7y5zwng8aba` and none may be sent.
