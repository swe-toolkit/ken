---
id: RT-ROOT-AUTHORITY-REACHABILITY-WITNESS
title: "ANSWERED, bounded negative on all three guards: three Ken-source programs attacking absence-at-consumption, wrong-outer-cursor and duplication all built and exited 0, so no language restriction is triggered -- three stress shapes are a search, not a proof of unreachability"
status: closed
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Operator, 2026-08-16, on the Steward's relay of research advisory evt_nw85nh58a7dd: 'the next step is to ask if there is a ken program that can reach this state, and not just a fixture in the test. If such a program can be found, then we should instead understand how to modify the language to make that program impossible to express, since the state the compiler arrives at seems genuinely invalid.' Steward-filed per COORDINATION section 2."
---

> # ANSWERED AND CLOSED, 2026-08-16. BOUNDED NEGATIVE ON ALL THREE GUARDS.
> # Runtime `evt_26zesecxs7ndt`. Empty range — no candidate, no retained probe.
>
> **Closed rather than merged: measurement-only.** Nothing lands.
>
> **Three Ken-source programs** built through `ken native-build` →
> `native_build_file` → `ken_cli::build_native_program` →
> `compile_native_program_sources` at `1b86202dd`. Steward verified
> `native_build_file` at `ken-cli/src/main.rs:81`. **Each built an executable and
> exited 0. None fired its guard.**
>
> | guard | attack shape |
> |---|---|
> | absence at consumption (`:18340`) | a single checked host-effect episode |
> | wrong outer cursor (`:18296`) | nested `Result`, `ProcessInput`/list matches, host bind |
> | duplication (`:18307`) | two sequential `Vis` nodes in one bind |
>
> **The bound, stated as reachability:** *"evidence of three deliberately chosen
> source-control stress shapes, not a universal proof."* **Correct shape, and
> the ring reached it unprompted for the second node running.**
>
> ⇒ **No enclave language restriction is triggered.** The operator's conditional
> — *if* a program is found, make it inexpressible — **did not fire.** The
> guards stand as assertions over a state **no Ken source program has been
> shown to reach**. **CORRECTED 2026-08-16 — this clause originally read
> "a state nothing has been shown to reach", and a `RuntimeExpr` fixture does
> reach it. See the correction block below.**
>
> **What this does NOT establish:** that the state is unreachable. Three shapes
> is a search, not a proof, and the deeper reading — whether the affine protocol
> makes the state unreachable *by construction* — was not attempted and is not
> owed by this node.
>
> **`AC-6` discharged:** the `PlannerInvariant` reclassification *"remains
> correct regardless of this negative result, and was not implemented."*
> That is [[RT-ROOT-AUTHORITY-BLAME-DOMAIN]], now unblocked.

> # THE DISPOSITION LADDER IS INVERTED FROM THE STATIC-WORKER NODE. READ THIS FIRST.
>
> **In [[RT-STATIC-WORKER-WITNESS-PROGRAM]], finding a witness would have argued
> FOR building something** — the prior art materializes a boxed closure exactly
> where Ken refuses, so a reachable program meant a capability Ken was declining
> to provide.
>
> **Here it is the opposite.** Research `evt_nw85nh58a7dd` classifies this state
> as an **internal lowering-protocol failure** — the compiler minted an
> obligation after admission and reached emit time without discharging it.
> **That state is not a capability Ken is withholding. It is invalid.**
>
> | outcome | what it means | what follows |
> |---|---|---|
> | **no program found** | no *Ken source* program reaches it. **NOT "nothing reaches it" — corrected 2026-08-16, a `RuntimeExpr` fixture does** | transport is [[RT-ROOT-AUTHORITY-BLAME-DOMAIN]]'s question, not a capability question |
> | **a program IS found** | **the language admits a program whose lowering has no valid terminal** | **make that program inexpressible.** Not: represent it, not: support it |
>
> ⇒ **A positive result here is bad news about the LANGUAGE, not a feature
> request against the backend.** Operator, 2026-08-16: *"we should instead
> understand how to modify the language to make that program impossible to
> express, since the state the compiler arrives at seems genuinely invalid."*

## What exists today is weaker than a fixture — it is direct struct manipulation

`control.rs:958` `distinguished_root_authority_is_checked_affine_and_cursor_bound`
does not compile a program of any kind. It reaches in and mutates the lowering's
internal state:

```rust
lowering.mint_terminal_answer_authority()   // consume it
lowering.mint_terminal_answer_authority()   // ... and mint again -> refusal

authority.outer_cursor = Some(ContinuationCursorId(7));
lowering.restore_root_terminal_authority(Some(authority), ContinuationCursorId(8))
```

**That is a legitimate test and it proves the affine discipline is enforced when
violated by hand.** It says **nothing** about whether anything can violate it.

⇒ **This is a weaker starting position than the static-worker case had.** There,
at least, the fixtures built `RuntimeExpr`s. Here the only demonstrations set
struct fields directly.

> # CORRECTED 2026-08-16. A `RuntimeExpr` FIXTURE REACHES THE
> # GUARD, AND DID ALL ALONG.
>
> **This paragraph used to end *"the guard has never been shown to fire on input
> of any kind."* That sentence was false when written.**
>
> **Surfaced by CI on `RT-ROOT-AUTHORITY-BLAME-DOMAIN` `D0`-`D2`** (PR #2409,
> red at `de6cc12c1`). `d2k_1b_unmarked_seeds_refuse_and_resolve_no_fusion_plane`
> (`control.rs:35922`) pins `row1-owned-scope` —
> `px8j_layered_recursive_result(1, 1)` — to a refusal, and the reclassification
> candidate's own updated pin renders that refusal as
>
> ```
> Backend(PlannerInvariant("terminal answer has no affine checked-root authority"))
> ```
>
> ⇒ **That is the `:18340` absence-at-consumption guard, fired by a
> `RuntimeExpr` fixture.** Not hand-set struct fields — a built expression.
>
> **What this does NOT touch: the bounded negative.** `D0`/`D1` were scoped to
> **Ken source through `ken native-build`**, and a `RuntimeExpr` fixture is not
> that. Runtime's result (`evt_26zesecxs7ndt`) stands exactly as stated, and its
> closure is unaffected.
>
> **What it changes is the shape of what remains open.** The guard is **not** an
> assertion over a state nothing reaches — the disposition table below reads
> *"the guard is an assertion over a state nothing reaches"* on the
> no-program-found branch, and **that gloss is now wrong**; only the
> Ken-source-specific claim survives. The live question is narrower and better
> posed: **can Ken source produce this `RuntimeExpr`?**
>
> **`px8j_layered_recursive_result` is a far better starting point for that than
> the three shapes chosen blind**, and it is already a lane-1 campaign fixture.
> **This is recorded, not filed** — a successor is a third lane under the
> 2026-08-15 directive and queues.
>
> **How it was missed:** `AC-1` barred the `control.rs:958` struct-manipulation
> route and directed the search at Ken source. Nobody swept `control.rs` for
> *other* fixtures already reaching the guard, because the frame treated that
> file as the thing to get past rather than as evidence. **A file excluded as a
> weak route was never read as a witness corpus.**

## `D0` — attempt to reach the state from Ken source

**Same bar as [[RT-STATIC-WORKER-WITNESS-PROGRAM]] `AC-1`, which discharged
cleanly:** Ken source compiled the way a user's program is compiled — `ken
native-build` → `ken_cli::build_native_program` →
`ken_elaborator::compiler_driver::compile_native_program_sources`. **Name the
entry point.**

**Target all three guards, and say which you attacked** — they may not be
equally reachable:

| guard | `mod.rs` | the state |
|---|---|---|
| absence at consumption | `:18340` | reached emit with no authority |
| wrong outer cursor | `:18296` | authority returned through a cursor it does not belong to |
| duplication | `:18307` | two live authorities across source control |

**The most promising direction, stated so it is not rediscovered:** the token
carries an `outer_cursor` and the guards concern source-control episodes, so
**nested or sequential control structure around a checked invocation root** is
where the protocol has the most room to go wrong. That is a hint, not a
prescription.

## `D1` — report which world, and stop there

**Reachability, never absence of expectations.** A zero result must say **"I
tried to drive it and could not,"** with the search bound stated, **not** "I
found no tests expecting it."

**If you find one: hand back immediately with the program.** Do not repair, do
not propose the language change, do not touch the guard. **The remedy is a
language-design question and it is the enclave's** — this node's job ends at the
witness.

## Acceptance criteria

**`AC-1`. Ken source through the production path.** Not a `RuntimeExpr`, and
**explicitly not** the `control.rs:958` route of setting fields on the lowering
struct. **That route is what this node exists to get past.**

**`AC-2`. A negative result is a success**, and is reported as a **search
bound** with what was attempted. **`RT-STATIC-WORKER-WITNESS-PROGRAM` is the
model** — it returned *"no reasonable source program was found … this is not a
universal proof"*, which is exactly the right shape.

**`AC-3`. Do not make the guard stop firing.** If the invariant is violable, the
guard firing is correct behaviour and the defect is upstream of it. A candidate
that weakens, deletes, or widens any of the three guards **fails outright**.

**`AC-4`. Do not propose or implement a language restriction.** If `D0` finds a
program, **the remedy is out of scope** — see Sequencing. **Naming the shape of
the program is in scope; naming the fix is not.**

**`AC-5`. Do not fold in the static-worker refusal.** Different fault domain,
different ladder, different node. It closed on a bounded negative
(`evt_6ttpazvf9hbx9`) and its clause question is
[[SPEC-45-CLOSURE-IN-CONSTRUCTOR-EXCEPTION]]'s.

**`AC-6`. State the interaction with the reclassification, do not perform it.**
[[RT-ROOT-AUTHORITY-BLAME-DOMAIN]] moves these guards to
`BackendFailure::PlannerInvariant`. **That reclassification is correct
regardless of this node's outcome** — even a reachable state does not make the
construct "unsupported", because the user still cannot discharge the obligation
by editing it. **Say so; change nothing.**

**`AC-7`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only — `-p ken-runtime` / `-p ken-cli`, never `--workspace`.

## Banned scope

- **Repairing the invariant, the guard, or the protocol.** See `AC-3`.
- **Designing the language restriction.** See `AC-4` and Sequencing.
- **The static-worker refusal** and the five dead ledger dispositions — see
  [[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]] and
  [[RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]].
- **Deleting or weakening `control.rs:958`.** It tests something real. It is
  simply not evidence of reachability.

## Sequencing

**Runs alongside [[RT-ROOT-AUTHORITY-BLAME-DOMAIN]] and neither gates the
other.** The reclassification is right whatever this finds; this is right
whatever arm the error takes.

> ### IF `D0` FINDS A PROGRAM, THE REMEDY ROUTES TO SPEC — AND IT IS NOT SMALL
>
> *"Modify the language to make that program inexpressible"* is the operator's
> direction and it is the right shape for an invalid state. **It is also a
> language restriction**, which has its own cost: it can forbid programs a user
> would reasonably write, and the restriction has to be stated in the language's
> own vocabulary rather than the backend's.
>
> ⇒ **The witness is the input to that decision, not the decision.** File the
> remedy as a Spec question with the program attached; do not let this node grow
> into it.
>
> **`45 §2`'s `AC1` is worth having in hand when that conversation happens:** a
> backend bug is *"a wrong value, never a false `proved`"*, because the term was
> kernel-admitted before the backend ran. **So even a reachable invalid lowering
> state is not a soundness hole** — it is a compile that cannot complete. That
> bounds the urgency without excusing the defect.

## Provenance

Operator instruction in session, 2026-08-16, on the Steward's relay of research
advisory `evt_nw85nh58a7dd`. **Coordinates verified against the tree by the
Steward before filing:** the three guards at `mod.rs:18296`, `:18307`, `:18340`;
the token at `:16486`; the mint sites at `core.rs:3453` and `units.rs:6117`; the
struct-manipulation test at `control.rs:958` with its assertions at `:978` and
`:995`.
