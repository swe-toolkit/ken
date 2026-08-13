# RT-4B-UNIQUENESS-GATE-ATTRIBUTION — recover absence-vs-multiplicity at one gate, changing nothing about when either fires

**Owner: runtime. Size: S. Gate: none.**
**STATUS: `draft` pending the Architect's lawfulness ruling. Do not start.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`origin/main` = `7c080543` plus `RT-4B-ENUMERATION-INPUT-SIZE` (`81f46822`).

## Fixed inputs

| fact | site |
|---|---|
| the refusal, one line, both arms collapsed | `planning/static_transition.rs:10122-10126` — `if matching.len() != 1 { return Ok(None) }` |
| the documented lawful-refusal semantics | its doc: *"Absence and multiplicity are both refusals: 'the only edge' would be an existential and choosing among several would be a guess."* |
| the landed multiplicity hook, already able to force the second arm | `:10117-10123`, `#[cfg(test)] DUPLICATE_STATIC_BODY_TRIPLE` |
| the isolation argument, in the code's own voice | that hook's comment: *"the transport gate, the bindings, the exact consuming suffix and the input projection have all already been satisfied by the time this runs, so a candidate that disappears here disappeared at the uniqueness gate specifically"* |
| the sole call site | `:10332`, inside `enumerate_live_fusion_candidates` |
| the observer to report through | `D2fGateArrival`, `lowering/core.rs:540-560`, filled at `:2182-2194` |
| the population under investigation | four admitted discoveries in, `keys = []` out — `(4, 2, 0, 2, 1)` |

## D1 — widen the return, map both variants to today's control flow

Distinguish `matching.len() == 0` from `matching.len() > 1`. Both must map at
the call site to **exactly the `continue` they produce today**.

## D2 — carry the attribution to the existing observer

Report through `D2fGateArrival`. **No second channel and no parallel recorder.**

## D3 — RECORD THAT THE GATE WAS REACHED, not only which arm fired

**This is the Steward's addition and it is the difference between this
increment answering a question and returning silence.**

`fusion_unique_static_body_triple` is the **twelfth** of thirteen exits. A
candidate reaches it only after surviving eleven earlier ones. **Nothing
measured says the four candidates get that far.**

⇒ If the increment records only *which arm fired*, and the answer is that
neither did, the artifact is **indistinguishable from an increment that was
never wired up.** Record the reach count so zero-reach is a positive
measurement rather than an absence.

**Zero reach is a real and useful outcome**: it says the eliminations are
upstream of the twelfth exit, which is a bit we do not currently have. It is
only useless if it cannot be told apart from a broken instrument.

## Acceptance criteria — the first five are the Architect's, verbatim in substance

- **AC-1 — BEHAVIOURAL IDENTITY IS THE GATE, NOT THE DIFF SIZE.** Every new
  variant maps back to identical control flow: both arms still `continue`, and
  **no candidate survives that does not survive today.** Prove it as 4a's
  equality control did — produced plan and artifact identical before and after,
  by identity where identity is available. **If the interned key population
  changes by even one on any witness, that is a repair wearing an instrument's
  clothes.**
- **AC-2 — ATTRIBUTION, NEVER REPAIR.** The two arms and their conditions are
  untouched. This reports *which* fired and changes nothing about *when* either
  fires. **The live hazard is that the multiplicity arm looks improvable to
  whoever is already editing this function. It is not in scope. Do not improve
  it, and do not "clarify" it.**
- **AC-3 — EXACTLY ONE FUNCTION AND ONE CALL SITE.** If the widening requires
  touching a caller other than `enumerate_live_fusion_candidates`, or a second
  function's signature, **stop and return it.** That is a different object and
  it goes back to the Architect before it is built, not after.
- **AC-4 — THROUGH THE EXISTING RECORD.** `D2fGateArrival` already sits at the
  production site. A new parallel channel re-creates the proliferation the
  Architect refused when he ruled the observer already existed.
- **AC-5 — THE CONTROL SEPARATES ABSENCE FROM MULTIPLICITY, NOT MERELY "NOT
  ONE", AND THE RED IS ALREADY AVAILABLE.** Unmutated must attribute one arm;
  arming `DUPLICATE_STATIC_BODY_TRIPLE` must **flip** the attribution to the
  other. **A control that cannot flip it has measured nothing.**
- **AC-6 — reach is recorded, so zero-reach is a measurement** and not
  indistinguishable from an unwired instrument (D3).

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **multiplicity arm** | The shape was right and the edge population was not — several `StaticBody` edges into the producer unit where the gate requires exactly one. Informative, and still **not** a defect claim against the planner. |
| **absence arm** | **STILL NOT A FINDING AGAINST THE PLANNER** (Architect, `evt_6pyegcfswgf68`). No `StaticBody` call edge into the producer unit is **a fact about the edge population**, and it points **upstream again** — to whatever should have produced that edge. |
| **gate not reached** | The eliminations are upstream of the twelfth exit. One bit, honestly obtained; it attributes nothing. |

> **"The uniqueness gate ate our candidates" is the sentence that will be
> written wider than it is measured, and it would be the sixth time in this
> arc.** None of the three rows above licenses it.

## Banned scope

- Attributing the other twelve exits — measured and ruled out (`evt_ky5f547e6hjz`);
  none is distinguished, so it is an all-thirteen builder change. **The Architect's
  ruling that the full census is out stands unchanged; this is the one-function
  exception he named, not a reopening of it.**
- Improving, relaxing or reordering either arm's condition.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.

## Hard stops — return to the Steward

- **A caller other than `enumerate_live_fusion_candidates`, or a second
  signature, is required.**
- **The interned key population changes on any witness.**
- **The `DUPLICATE_STATIC_BODY_TRIPLE` control cannot flip the attribution.**

## Sequencing and contention

Runtime, one lane, after `RT-4B-ENUMERATION-INPUT-SIZE` lands. Touches
`planning/static_transition.rs` and `lowering/core.rs`.
