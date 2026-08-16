---
id: RT-ROOT-AUTHORITY-REACHABILITY-WITNESS
title: "Can a Ken PROGRAM drive the lowering into the undischarged root-authority state -- and if one can, the remedy is to make that program inexpressible, because the state the compiler arrives at is genuinely invalid"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Operator, 2026-08-16, on the Steward's relay of research advisory evt_nw85nh58a7dd: 'the next step is to ask if there is a ken program that can reach this state, and not just a fixture in the test. If such a program can be found, then we should instead understand how to modify the language to make that program impossible to express, since the state the compiler arrives at seems genuinely invalid.' Steward-filed per COORDINATION section 2."
---

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
> | **no program found** | the guard is an assertion over a state nothing reaches | healthy; transport is [[RT-ROOT-AUTHORITY-BLAME-DOMAIN]]'s question, not a capability question |
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
struct fields directly. **The guard has never been shown to fire on input of
any kind.**

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
