---
id: CONF-VERIFY-OLD-ROW-UNSATISFIABLE
title: "The seed's only unclaimed row states expect: accepts against a landed elaborator that rejects unconditionally, and the Coverage map rolls it up as a satisfied family"
status: ready
owner: spec-enclave
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary report evt_315kg9tshgnm7 (2026-08-10) on merged fad92a1b, measured at cf3b77b7. Steward-triaged as a confirmed defect in the row, NOT in the elaborator and NOT in the merge. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> # THE DEFECT IS IN THE ROW, NOT IN THE ELABORATOR AND NOT IN THE MERGE
>
> Direction is **over-claim**: the corpus states a requirement stronger than
> HEAD. The seed's declared layer is untrusted (`:12-15` — *"a bug is a wrong
> verdict or a poor diagnostic, never unsoundness"*). **Not soundness.**
> **Do not "fix" the elaborator** — the capability is deliberately deferred.

## What it is

`conformance/verify/spec-syntax/seed-spec-syntax.md:161-171`,
`verify/spec-syntax/old-resolves-in-space-op-ensures`:

> given: a `space` op `inc` over a cell `n : Int` with `ensures n == old(n) + 1`
> expect: **accepts**; `old(n)` resolves to the pre-state `s_pre.n`

The landed elaborator refuses it unconditionally —
`crates/ken-elaborator/src/elab.rs:5584-5586`, inside the space-desugaring loop,
returns `ElabError::OldPreStateUnsupported` for **every** `old` in a space-op
contract. No branch, no capability check. Confirmed by a **green committed
test** on the row's own shape, not by reading:
`crates/ken-elaborator/tests/surf_space_cells_p1.rs:458-476`.

**The capability is deferred openly in three places** —
`spec/90-open-decisions.md:183` (`OQ-Space`),
`docs/program/issues/SURF-SPACE-CELLS.md:123` (*"`OldPreStateUnsupported`
stays"*), and `docs/program/issues/EFF-SPACE-ENSURES-PRESTATE.md:44`.

## The invariant violated is the seed file's own preamble

`:6-10`: *"Expected results are grounded in the **landed** `21`, the V0
elaborator — not the WP frame's prose (the perishable-frame discipline)."*
An `expect: accepts` against a landed elaborator that rejects is exactly what
that sentence exists to prevent.

**And the file already carries the convention for this and did not apply it.**
`:44-47`: *"Cases that would exercise that spelling are tagged
`[deferred — §5.5]` and assert the model, never un-landed grammar."* The
convention is keyed on one deferral (`OQ-syntax`) and used at `:275`;
`old`/pre-state is deferred under a **different** open decision (`OQ-Space`) and
so got no tag. **A convention keyed to one deferral does not generalize to the
next one.**

## Where the cost actually lands

The Coverage map at `:372-373` reads:

> **#3 `old` semantics** — `old-resolves-in-space-op-ensures` /
> `old-out-of-scope-rejects` (the flip pair).

That is an acceptance roll-up against `21 §9` presenting `#3` as a **satisfied
family**. One half is unlanded and unmarked, and after `fad92a1b` the row has no
test claiming it — **so no instrument in the repo connects the two.**

## Deliverables

**D1 — Tag the row `[deferred — OQ-Space]`**, in the shape the file already uses
at `:275`, and make it assert the model rather than un-landed behaviour.

**D2 — Mark the `#3` roll-up at `:372-373`** so the Coverage map stops
presenting a half-unlanded family as satisfied.

**D3 — Decide whether the tagging convention at `:44-47` should be stated
per-deferral rather than keyed to `OQ-syntax`.** It is one sentence and it is
the reason this row was missed. If you decide against, say why.

## Acceptance

- **AC-1 — No row in this file states `expect: accepts` for behaviour the
  landed elaborator refuses, unless tagged as deferred.** Enumerate, do not spot
  check.
- **AC-2 — The `#3` roll-up no longer reads as satisfied.**
- **AC-3 — The reject half is untouched.** `old-out-of-scope-rejects` is
  **sound and was attacked**: `crates/ken-elaborator/src/resolve.rs:1604-1610`
  discriminates on `PropCtx::SpaceOpEnsures` one layer *above* the deferral
  fence, so outside a space-op `ensures` `old` never becomes `ROld` at all and
  fails as `UnboundName("old")` — which is what the row asserts. The guard
  landed; only the capability did not. **Do not touch it.**
- **AC-4 — No elaborator change.** The refusal is deliberate.

## Scope

**In:** `conformance/verify/spec-syntax/seed-spec-syntax.md` only.

**Out, and these are bans:** no `crates/` change; no new tests; no edit to
`old-out-of-scope-rejects`; no re-litigating the `fad92a1b` claim removal, which
is **confirmed correct** — see below.

## The removal that surfaced this is corroborated, not undermined

I flagged at `evt_67ka82h7djs7x` that the removal of
`old-fails-closed-without-pre-state` rested on three readings of one spec
section, which is **not** three discriminators. It now has a fourth corroborator
that is **not** a reading of `21 §6.4`: the elaborator implements exactly that
discrimination at `PropCtx::SpaceOpEnsures`. The guard is real and the
capability is deferred — which is precisely why the residual belongs to **this
unclaimed row** and not to the removed claim.

## Two cheap notes on the redirect, folded here rather than filed separately

Neither is a defect — 6 rows carry multiple claims at HEAD, 5 predating
`fad92a1b`.

1. **The multi-claim labelling convention was not carried.**
   `verify/spec-syntax/result-scope` is claimed twice as `result-scope (a) —
   accepts in ensures` and `(b) — rejects in requires`. The new pair on
   `requires-elaborates-to-pi-proof-arg` are two bare identical claims.
2. **The two claimants partition that row's `expect` and neither covers it
   alone.** The row asserts a conjunction — the emitted **type** is the Π chain
   **and** the **body** is the bare carrier. `requires_elaborates_to_pi_proof_arg`
   (`v1_acceptance.rs:54-77`) checks the Πs and never touches the body;
   `requires_on_final_param_unaffected` (`:626-660`) pins the body and never
   counts the Πs. **Jointly complete, severally not, and with no `(a)`/`(b)`
   discriminator nothing records that.**

Applying the `(a)`/`(b)` labels is in scope for `D1`'s file if you take it.

> ### FRAMING NOTE — the census was run in one direction only
>
> `CONF-VERIFY-SPEC-SYNTAX-PHANTOM-CLAIMS`'s `AC-2` fixed the direction
> precisely: *"exactly one heading per claim."* **Claim to heading. The frame
> never asked heading to claim**, and the mirror query is one `comm` over the
> two populations the node had already assembled.
>
> **The row was in front of the enclave** — the frame's `§2c` names
> `old-resolves-in-space-op-ensures` by hand while enumerating the `old` family
> to decide the fourth claim. **The question that would have flagged it was
> outside the frame, not outside the reader's view.** That frame was mine.
>
> Base rate, so the single gap reads as signal: **17 of 18** rows in this seed
> are claimed by a test (`surface/numbers`: 25 of 31). Claiming is near-universal
> here; this is the one exception.
