---
id: RT-NATIVE-THEN-CHAIN-SOURCE-SLOT-TRANSFER
title: "A user-level then-chain over FS actions, fs_then chained three deep with a Console write on the error arm, fails native build: a Vis constructor with a source-slot store obligation reaches generic carrier transfer, which has no selected-case environment, and refuses with \"a source slot constructor reached generic transfer without its creation-site suffix\" (aggregates.rs:1955). The refusal is fail-closed and independent of producer alternative sets. Route such a constructor through the creation-site builder, or rule the shape out of scope with a typed cause"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_573z7twxznjt9 on PX10-NATIVE-BRANCH-ALTERNATIVES D0-17: candidate 5, a NativeTested then-chain written as the labelled transport's exec fixture, refuses at a pre-existing guard (2edc10ac9) whether or not sets issue. Filed by the Steward on request. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A then-chain over FS actions builds natively

## Objective

A native program that chains FS actions through a user `fs_then`
combinator, three deep, builds and runs with the same (request, outcome)
pairs as the interpreter, or is refused at planning with a typed cause
the Architect rules. It never reaches generic transfer with a
source-slot obligation it cannot fill.

## Settled inputs (main `b0ec0a1a9`)

- **The refusal.** `aggregates.rs:1946` to `:1956`: generic constructor
  transfer has no selected-case environment from which to fill a
  residual suffix, so a position with `slot_store_obligation` refuses
  with `PlannerInvariant` "a source slot constructor reached generic
  transfer without its creation-site suffix". The guard is from
  `2edc10ac9` (RT-CARRIER-RESIDUAL-TYPED-OPERAND). The alternative arm
  has the same guard at `:3323`.
- **The fixture.** `fs_then a action next` binds `action` and, on `Ok`,
  calls `next MkUnit`; on `Err` it writes "fail" and returns the error.
  `main` chains `metadata_unit` (FsMetadata) on `switch.bin`, then twice
  on `held.bin`, then returns `Ok`, and writes a per-outcome Console line
  with distinct exit codes. Every operation is NativeTested. Source and
  logs: `/workspaces/ken/local/px10-native-then-chain-residual/`
  (`fs-then.ken` SHA-256 `2cff2ea8e07d…`, `d017-baseline.log`,
  `d017-empty-sets.log`, `d017-empty-sets-mutation.diff`); local and
  perishable.
- **Measured** (runtime-implementer, D0-17 on `1a756ef69`). The build
  refuses at `:1955` on transfer 487, occurrence 17, position 1, with
  Spec2 disposition. With issued sets emptied it refuses at the same
  site with the same disposition, so the cause is not the labelled
  transport.
- **Containment.** Until `PX10-NATIVE-SPAWN-PARITY` Increment B deletes
  it, BRANCH-ALTERNATIVES' AC-2g refuses this fixture earlier, with its
  containment message. Measure below it as AC-0 rules.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate that builds the fixture natively with interpreter parity,
or refuses it with the ruled typed cause, with the pins below.

## Acceptance

- **AC-0 (Architect).** Rule from a D0 whether the obligation-bearing
  constructor should take the creation-site builder (and from which
  environment), or whether the shape is refused earlier with a typed
  cause. Rule the fan-in: both guards (`:1955`, `:3323`) and every
  route that reaches generic transfer with an obligation.
- **AC-1.** The fixture, committed under the ken-cli native tests,
  meets the ruled outcome: native and interpreter (request, outcome)
  pairs equal for `switch.bin` present and absent, under a bounded wait;
  or the ruled typed refusal at planning. A mutation that reverts the
  repair returns it to `:1955`.
- **AC-2.** The existing carrier-residual and bracket suites stay
  green, and `trusted_base()` is unchanged.

## Stop conditions

- A kernel or trust change.
- Removing or weakening the `:1955` or `:3323` guard rather than
  satisfying it.

Checks: 2 fired: the refusal has two guards on the same predicate
(`:1955`, `:3323`), so AC-0 rules the fan-in before the site. 8 fired:
the empty-sets control refuses at the same site and disposition, so the
cause is separated from the labelled transport.
