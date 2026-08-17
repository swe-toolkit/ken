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

Frame: this node. **`D1` is DELIVERED at exact
`10256e8fbc4df22beb25a6b095d6b1be515e7e90`** — one line of test code plus a
mutation proof, QA-approved and resolved by the Architect at
`dec_797e7wbbb1ae9`.

**One live deliverable remains: `D2b`, the limitation note the Architect
required at the pin.** The original `D2` is **WITHDRAWN, and its two parked
items are now PERMANENTLY MOOT** — see that section.

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

## `D2b` IS HELD 2026-08-17, MID-DISPATCH. The limitation may be REMOVABLE, and a comment explaining an open gap is the wrong artifact if it can be closed.

> ### The Adversary hunted `D1` and refuted half the premise `D2b` was built on. `evt_10kfceqnt1fh1`.
>
> **Every mechanical claim verified by the Steward at `origin/main` before
> routing.** The ruling was that string matching is the only available
> predicate here. **The operative distinction is FRAGMENT versus EXACT, and
> exact is constructible.**
>
> **The refusal message is a `format!` with five interpolations**
> (`lowering/mod.rs:4728-4741`), and **the test supplies or can bind every
> one**: `constructor` is the literal it passes, `owner` comes from
> `planned_root_occurrence`, `position` is the `0` it passes, `field_origin`
> comes from `child_static_origin(owner, 0)`, and **`recognition` is RETURNED
> by `recognize`** — `-> Result<StaticWorkerRecognitionId, _>`
> (`mod.rs:4490-4497`), which derives `Debug` (`mod.rs:4442`).
>
> **The test DISCARDS that return** — `.expect("the real issuer mints the
> recognition")`, unbound. **Binding it is the entire cost of making the message
> reconstructible.**
>
> **This is the FIFTH discarded result in this campaign, and the first whose
> cost is a lost ASSERTION rather than a lost explanation.** The others were the
> sentinel's `_excluded_result`, the two `control.rs` trace helpers,
> `compiler_driver.rs`'s `map_err(|_| …)`, and `D2c`'s deliberate `let _ =`.
> **Here the discarded value was the one piece needed to make the strongest form
> of the pin constructible, and nobody noticed because discarding it compiles.**
>
> **One correction in the ruling's favour, which does not rescue the premise.**
> Pin 1's full equality is on a **constant** message with no `format!` at all —
> so *"the sibling pin already uses exact matching"* is true about the idiom,
> not about the difficulty. **The reconstruction is available anyway.** The
> ruling's two halves now fare differently: ***"or an id change"* is REFUTED**
> (the ids are bindable); ***"an incidental rewording"* STANDS and is now the
> whole argument.**
>
> ⇒ **The choice is not "string matching leaves no better option." It is
> brittle-and-total versus tolerant-and-phrase-shaped**, on a pin that after
> `D3` is the only assertion of that text anywhere.
>
> **Measured and standing regardless of the ruling:** *"prior to construction"*
> passes all three clauses with pin 1 green. **The synonym limitation is
> measured now, not merely ruled.**
>
> **WITH THE ARCHITECT** (`evt_3p3sbs83j6gr0`), because it is his ruling's
> premise. Three arms: **close it** (bind the recognition, assert full equality,
> delete `D1`'s clause as subsumed); **document it** with the justification
> corrected to the rewording trade alone; or **both**, if the two are worth
> having as independent arms.
>
> **`D1` STANDS MERGED under every arm.** The clause discriminates — reproduced
> against production by the Adversary independently of the ring's report.

## Deliverable `D2b` — record the pin's DOMAIN beside the pin. HELD; see above.

**One comment, in `control.rs`, immediately above
`refusal_pins_rehomed_static_worker_without_selector_exclusion`'s guard.** No
code change, no test change, nothing else in the range.

**What it must say, in the Architect's terms (`dec_797e7wbbb1ae9`):**

> **The clause pins one PHRASING of the refuted claim, not the claim.** A future
> message re-asserting the same refuted temporal semantics in different words —
> *"prior to construction"*, *"ahead of the build"* — passes all three clauses.

**Say also why that is not a defect here**, so the next reader does not file it
as one: there is no cheaper predicate for *"does not assert
erasure-before-construction"* than string matching, and **the false-positive
direction is the right one** — a future message legitimately using the phrase
reds this spuriously, and a red is a prompt to think, where the alternative is
a silent pass.

**And why `contains` rather than `==`** — **THIS CLAUSE IS WHAT THE HOLD IS
ABOUT. Do not write it as it stands.** It read: *pin 2's message is a `format!`
carrying runtime ids, so full equality would conflate a refuted claim returning
with an incidental rewording or an id change.* **The id half is refuted** — the
ids are bindable and a reconstructed message tracks them exactly. **Only the
rewording trade survives**, and whether it carries the conclusion is the
Architect's to rule.

> **Why this is a deliverable and not a note in the frame.** The failure it
> prevents is **over-citation of the pin** — a later reader treating it as
> covering the claim when it covers a phrase. That reader is at the pin, not in
> this node.

**This node stays `active` until `D2b` lands**, and it is the only thing between
[[RT-DESCENT-RETIRE]]'s `D3` and the frontier — so it is a short turn, not a
parking space.

## `D2` — WITHDRAWN 2026-08-17 before dispatch, and its items are now PERMANENTLY MOOT.

> ### THE ARM-1 RULING DISSOLVED BOTH PARKED ITEMS. They are not waiting for anything.
>
> They were parked as *"live the moment any `D2c` lands."* The Architect then
> ruled `D2c` **measurement-only and never merged** ([[RT-DESCENT-RETIRE]],
> `evt_4cw8rsesahmeh`). **No `D2c` ever lands**, so the condition never fires:
>
> | item | why it is moot |
> |---|---|
> | B, the `let _ = recursive_descent_residual(…)` because-clause | **The site never reaches `main`.** It exists only inside `f68b8c866`, which is an evidence artifact |
> | A, the *"let the remainder decide"* comment | **It stays TRUE on `main`** — the early `return` is only removed by a `D2c`, and none lands. `D3` deletes the function outright |
>
> **Both are struck, not re-parked.** The content stays below because the
> reasoning is the durable part, not because the work is pending.


> ### DO NOT WORK `D2`. Item A would REGRESS a comment that is currently TRUE.
>
> **`D2` was folded in from Architect `evt_2xm79ytjgz3j0`, raised as should-fix
> at the fresh `D2c`'s approval. That candidate then FAILED CI and PR #2509 is
> closed** — see [[RT-DESCENT-RETIRE]] for why no correct `D2c` can merge
> through the publisher.
>
> **I verified both coordinates against `origin/main` after the failure, and
> neither survives:**
>
> | item | at `origin/main` |
> |---|---|
> | B, the `let _ = recursive_descent_residual(…)` discard | **DOES NOT EXIST.** Zero hits — `D2c` introduced it |
> | A, the *"let the remainder decide"* comment | **CURRENTLY TRUE.** The early `return` `D2c` removed is still there |
>
> ⇒ **Item A is not a stale comment to correct; it is an accurate comment, and
> "correcting" it would make it false.** The comment goes false **only if** a
> `D2c` lands.
>
> **The general form, which is why this is recorded rather than deleted: a
> should-fix note attaches to a CANDIDATE, not to `main`. When the candidate
> does not land, its notes do not become work — they become conditional on it
> landing.** I folded these in while the candidate was in flight and did not
> re-check them when it failed.
>
> **Both items are PARKED, not dropped.** They become live the moment any `D2c`
> lands, and belong with whatever node carries that. **Content retained below
> verbatim so it is not reconstructed.**

### STRUCK CONTENT — retained for its reasoning, not as pending work

**Coordinates below are in candidate `f68b8c866`, NOT in `main`.**

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

**AC-3.** No production change. This node edits **one test guard** (`D1`) and
**adds one comment beside it** (`D2b`), and **deletes nothing**.
`select_body_emission_authority` and `lowering/core.rs` are **not touched** —
`D2` is withdrawn and struck.

**AC-6** (`D2b`). The limitation is recorded **at the pin**, and states three
things: the clause covers a **phrasing**, not the claim; the false-positive
direction is deliberate; `contains` was chosen over `==` because the message is
a `format!`. **A reader at the pin can tell what it does and does not cover
without leaving the file.**

**AC-4.** `D2c` `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f` untouched.

**AC-5.** No-regression, green **in CI** (`COORDINATION §12`). Local runs
targeted only — `-p ken-runtime` or `--test`, never `--workspace`.

## Banned scope

- **Rewriting pin 1.** It uses full equality and has no gap; changing it trades
  a stronger assertion for a uniform one.
- **Any `D3`-`D8` retirement work**, and **any `lowering/core.rs` edit at all**
  — `D2` is withdrawn and its two comment items are struck, not pending.
- **Weakening or rewriting the absence clause `D1` landed.** `D2b` documents
  the pin; it does not touch it.
- **Widening to other `contains`-style assertions in `control.rs`.** If the
  pattern looks general, **say so and stop** — that is a census, not this node.
- **A general `D6a` comment sweep**, and the two parked comment items with it.

## Standing

**The Adversary edge is report-only.** This node is the triage; no reply is
owed to `evt_3t7y5zwng8aba` and none may be sent.
