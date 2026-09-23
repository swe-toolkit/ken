# Models

What is available, what each is suited to, and how to choose.

**Seat assignments live in `moot.toml` and nowhere else.** This file never
records which seat runs which model, what tier a named seat is, or what a lane
is currently seated on. To learn what a seat runs, read its pane or its
`[agents.<role>]` block. To change one, use
`playbooks/federation/steward/add-agent.md`, which owns the per-harness key
sets and the reseat procedure.

## Classify the work, not the seat

`T1`/`T2`/`T3` name a class of WORK. A frame records the tier its work
requires; whether a seat can meet that is a separate, seating question.

- **T1** — highest judgment. Clean-room design, spec elaboration, soundness
  rulings, abstraction-boundary pinning, genuinely hard implementation.
- **T2** — high-volume build and coordination. Code generation, mechanical
  gates, verification runs, review passes, doc observation.
- **T3** — lightweight. Deterministic reflow, canonicalization, fully specified
  renames, run-and-report.

**Refer to work by tier, never by a model name or a model characteristic.** The
tier survives a model swap; the name does not.

**A SEAT'S TIER IS AN OBSERVATION, NOT AN INFERENCE.** Never derive it from a
role name, from this file, or from what a seat ran last week. `tmux
capture-pane` the seat and read its model line. This matters most in the
direction that costs least to get wrong on paper: **before proposing a
downgrade, confirm what the seat is actually running.**

If you cannot tell which tier a deliverable needs, that is the operator's call,
not an inference from the node's prefix. The line is the WORK, not the node id:
refactoring, mechanical moves, census and ledger work and instrument repair sit
lower than the implementation they serve.

## Reading model names

Within one vendor family the names are ordered by MASS:

```
Astra  >  Sol  >  Terra  >  Luna
```

**The name does not carry across generations, and assuming it does is the
trap.** A later generation's `sol` can sit at an earlier generation's `terra`
price point, and a family may ship no `terra` at all. Re-derive the ordering
inside the family you are actually choosing from; do not map `sol` to `sol`.

## Find out what exists before naming one

```sh
pi update --models   # refresh the catalog FIRST
```

The catalog is `~/.pi/agent/models-store.json` and it goes stale: a model
released today is absent until refreshed. **A model string that is not in the
catalog fails the seat's launch**, so a name taken from a message and written
straight into `moot.toml` can take down every seat it touches at once. Read the
catalog for ids, context window and current per-token cost.

**A generation can carry its own usage allowance, separate from the rest of the
provider's catalog.** Measured: every seat on one generation refused with
`usage limit has been reached` while a seat on the previous generation ran
normally on the same account, same credential and same harness. So a working
pool does NOT imply a given model will run, and `pi auth check` reporting
`ready` proves the credential, not the allowance. When every seat on one model
fails at once and none has done any work, suspect the ALLOWANCE for that model
before the harness, the config or the seats: the discriminating test is one
seat on a different model, and it costs a single relaunch.

## What each is suited to

Record what was MEASURED here, not an impression, and say plainly when a family
is unproven.

- **Opus-class (Anthropic).** T1. Hard compiler implementation, clean-room
  design and soundness work. Warranted where single work packages run many
  hours and the difficulty is in the reasoning, not the volume.
- **Sonnet-class (Anthropic).** T2. Build and coordination volume.
- **GPT 5.6 family (`openai-codex`, `pi` harness).** Terra carried build rings
  through refactoring and mechanical work. Codex-shaped stranding has cost
  rings turns; budget for it on long mechanical runs.
- **GPT 6 family (`openai-codex`, `pi` harness).** **UNMEASURED.** Seated for
  evaluation on one lane. Nothing is established about its suitability at any
  tier yet — do not cite a tier for it until a lane has run on it and the
  finding is written here.

Pools are independent per provider, so splitting tiers across providers buys
sustained work in both at once; exhausting one does not stall the other.

## Cost is the second axis, and the spread is large

Per-token cost across an available catalog can span two orders of magnitude,
and the cheapest model in a new generation can undercut the mid model of the
previous one by ~20x. That makes a tier choice a real budget decision rather
than a rounding error, and it makes an unnecessary T1 seating expensive in a
way that is invisible until the pool runs out.

Read current numbers from the catalog rather than from memory or from here.

## Delegating the mechanical tail

An expensive seat may hand the mechanical tail of a turn to a cheaper-tier
subagent. Misapplied, it spends MORE.

- **Delegate if and only if the subagent's verification is a deterministic
  re-check cheaper than the doing.** Yes: 80-column reflow, canonicalization, a
  fully specified rename across N files, run-tests-and-report. No: seed prose,
  law statements, fork transcription, clean-room judgment, **any semantic
  verification**.
- **Do not delegate a grounding read you author against** — the facts return as
  a summary and you re-fetch anyway. **Do not delegate a reconcile or a "does
  §X say Y" pass** — that reintroduces the cite-versus-verify hazard. The
  delegation prompt is mechanical-only, no rewording; a subagent that helpfully
  rewords a normative sentence is a fidelity regression.
- **A subagent inherits the parent's model unless you override it.** Spawning
  one from a T1 seat without passing a cheaper model runs the tail on T1, which
  is the exact opposite of the tactic.
- **Resolving a tier to a dispatchable model.** A subagent's model comes from
  the Agent tool's own options, NOT from `moot.toml` — seat configuration does
  not apply to subagents, so there is nothing to look up there. Map by the tier
  definitions above: **T1 -> the highest-judgment option the tool offers
  (Opus-class), T2 -> the mid option (Sonnet-class), T3 -> the lightweight one
  (Haiku-class).** Pass the tier's model explicitly; a skill or playbook that
  calls for a T2 review resolves it here and names no model of its own.
- **Right-size it.** The gain is modest on edit-heavy turns and near zero on
  design-heavy ones, and a small tail is cheaper inline than a subagent's
  spin-up and context read. In doubt, inline.

## Effort is the tunable knob

Tier is the coarse choice; `effort` is set per role in `moot.toml` and is what
you tune on observed quality. Raise it on observed misses, not up front.
Soundness-adjacent verification is the likeliest candidate if quality lags.
Publisher-path work stays mechanical — the deep correctness review belongs to a
T1 seat on the merge Decision, not to a higher effort setting on a script.

## Clean-room is a role discipline, not a model property

The boundary belongs to the ROLE, not to the model in the seat
(`CLEAN-ROOM.md`). Any model in an enclave seat reads references under the same
discipline; any model in a build or coordination seat reads none. Only the
enclave may consult copyleft references, for approach and behavior only, under
the leakage recheck. **Never send copyleft material to a build or coordination
role** — only Ken's own words in `/spec` and `/conformance` pass to the build
tier. Ken's own MIT source goes anywhere. The AGPLv3 prototype is not mounted
and is consulted by nobody.

## Portability

Playbooks and `COORDINATION.md` are written model-agnostic, with no reliance on
any single model's idiosyncratic behavior, because the same coordination law
must hold whichever model sits in a seat. A rule that only works on one model
is a defect in the rule.
