---
id: RT-CLOCK-OP-NATIVE-PROMOTION
title: "ABI-S3 merged the monotonic clock, deadline sleep and kernel entropy as host operations, but ClockMonotonicNow, ClockSleepUntil and EntropyRandomBytes are still RepresentedUnavailable, so neither the interpreter nor a native build runs them and PX12's deadlines have no clock to read. Promote all three to NativeTested with interpreter parity, carrying by name the two clock properties RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE retired"
status: merged
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]
blocks: [PX12, LINUX-ABI-EXIT-SERVICE-SUPERVISOR]
github: null
origin: "Steward cut 2026-09-16 as the carrier required by Architect evt_rsbhqs2tfamg condition 2. Framed 2026-11-22 on operator direction (\"Create WPs and update the campaign docs as necessary to capture the gaps you have identified\"): program §6 requires every operation NativeTested or probed, and these three are unavailable only because nobody promoted them. Entropy is folded in because it needs the same non-exact comparator. L1 Linux ABI program, Steward order."
---

# Native monotonic clock, sleep and entropy

## Objective

A Ken program that reads `MonotonicNow`, sleeps with `SleepUntil` and
reads `RandomBytes n` runs under the interpreter and as a native binary,
with matching observations. All three operations are `NativeTested` in
`effect_abi_v1.catalog`.

## Settled inputs (main `ccfc01de1`; anchors re-checked at release)

- **Availability.** `effect_v1.rs` `:206`, `:207` and `:237` mark the
  three operations `RepresentedUnavailable`. `executor_admits` admits
  only the seven Net operations without native parity, so the
  interpreter refuses all three today.
- **Surface.** The prelude declares `data ClockOp = WallNow | MonotonicNow
  | SleepUntil Deadline` and `data EntropyOp = RandomBytes Int`
  (`prelude.rs` ~`:1050`). `ClockWallNow` is the only promoted clock
  operation. Its package home is `Capability.Time.WallClock (wall_now)`
  (spec 38 ~`:1402`).
- **Interpreter.** The real host implements all three
  (`eval.rs` ~`:4330` monotonic baseline, ~`:4334` `/dev/urandom`,
  ~`:4350` sleep). The decoder is `decode_clock_request` (~`:5968`).
- **Verification host.** `ken-verify/src/host.rs` ~`:313` scripts
  monotonic readings, records sleep deadlines and scripts entropy bytes.
  It is the value-capturing backend this node's 2026-09-16 draft said
  was missing. Only `ken-host`'s own test backend (`effect_v1.rs`
  ~`:5179`) is an op-tag recorder.
- **Native.** The planner names all three among the operations refused
  before any seat is derived
  (`static_transition/effects.rs` ~`:690`). `ClockWallNow`'s lowering
  (`lowering/effects.rs` ~`:3244`) and its zero-arity seat ledger
  (~`:831`) are the template.
- **Promotion protocol.** `confirm_native_tested_transition`
  (`ken-verify/src/catalog.rs:315`) requires the operation to be
  `NativeTested` already. So the availability change and the
  differential land in one change, with the change made first
  (Architect `evt_11kc7kd57v10b`). `ClockWallNow` uses its own
  comparator, `compare_clock_wall_now` (`scenario.rs:399`). Monotonic
  readings and entropy bytes are not byte-equal across lanes either.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Inherited obligations (Architect `evt_rsbhqs2tfamg`)

`eval.rs` holds the two refusal forms that must be inverted back:
`ac2_monotonic_is_refused_while_the_wall_clock_still_reads_and_steps_back`
(~`:10650`) and
`ac3_sleep_until_is_refused_on_a_path_that_still_serves_an_available_op`
(~`:10750`). Their post-promotion bodies are in tree `b601e2ec7898`, in
the same module (`px5b_effect_observation_tests`), as
`ac2_monotonic_readings_survive_a_wall_clock_step_backwards` and
`ac3_the_deadline_a_caller_passes_is_the_deadline_honoured`. Start from
those bodies, found by symbol name.

1. **Monotonic under a backwards wall step.** The scripted wall clock
   runs backwards while the monotonic source advances, and the monotonic
   readings do not decrease. The test asserts the backwards wall step
   first, so a harness that cannot perturb the wall clock fails.
2. **The deadline passed is the deadline honoured.** The host observes
   exactly the deadline supplied. A second, different deadline gives a
   different observation (`ac3b_the_deadline_decoder_refuses_a_second_argument`
   ~`:10815` is the existing decoder control).

## Scope

`ken-host` availability and catalog, the `ken-interp` tests above, native
planning and lowering for the three operations, `ken-verify` evidence
and comparators, package homes if AC-0 rules any, and spec 38's status
lines. No change to `ClockWallNow`, to the Deadline type, or to ABI-S3's
no-substitute entropy rule (no userspace PRNG, no fallback).

**Scope amendment** (Architect AC-0 `evt_4zbmaff96r9wv`; Steward
`evt_6wykpd3k24pj6`, `evt_5bpcsk6h7z1hp`, `evt_12dr33x85kn5r`). The
settled inputs on the native host, the C reply, package homes and the
verify host did not hold on `b9c1853d6`. The following are admitted as
enablers:

- **Spec child.** A spec 38 §1.15 for the clock, sleep and entropy
  contract, with its status beside it. It adds the `MonotonicInstant` and
  `Deadline` floor rows in 30-taxonomy and 30-intrinsic-ledger, and the
  55 to 57 floor sync in 33-declarations §33.3.3 and 39-elaboration.
- **Catalog child.** `Capability.Time.Monotonic`, `Capability.System.Entropy`,
  `host_entropy` added to `Capability.Process.Exit`, and the WallClock
  pointer. It also carries their elaborator registration and floor
  sentinels.
- **Host and verify.** `ken-host`'s three shared functions and the
  `set_reply` carrier, and `ken-verify`'s real-source modes and three
  comparators.
- **Mechanical consumers.** A mechanical consumer of an admitted item is
  in scope when its paths are disjoint from INHERITANCE-DUPLICATE's. It
  is named in the handoff.

Entropy stays in this WP. Children touching INHERITANCE-DUPLICATE's paths
branch from main after that WP lands.

## Acceptance

- **AC-0 (Architect, before code).** Rule the package home of each
  operation, or that the prelude constructor stays the only surface. Rule
  each comparator: monotonic (non-decreasing within a lane), sleep
  (deadline exact) and entropy (count and outcome exact, bytes not
  compared).
- **AC-1.** One differential row per operation runs the same program
  under the interpreter and natively, and
  `confirm_native_tested_transition` returns `Ok(NativeTested)` for each.
  Each row has a negative control that perturbs the evidence, and the
  confirmation then fails.
- **AC-2.** Both inherited properties pass in their post-promotion form,
  each with its control. An entropy row requesting `n` bytes observes
  exactly `n`. Two reads of 32 bytes differ. An unavailable kernel source
  reports `Unsupported` and returns no bytes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Inventory

1. The out-of-domain sleep refusal had no lawful native terminal carrier.
   Ruling 4 named the class but not the seat, and the only pre-dispatch
   failure was require_true's unclassified -1. Keyed on: the native lane's
   program-reachable refusal must be a planner-catalogued trap
   (`evt_2tys8csd36b58`).
2. Ruling 3's out-of-domain comparator assumed that lane transport is
   identical across lanes: the effect-trace reading width (i128 native,
   minimal interpreter) and native stderr (starter report plus launcher
   line). Keyed on: what a lane emits on its transport outside the
   semantic observation (`evt_4630qt1rw0at9`).

§1a is at 2 (next trigger 3). The §1b predicate question is not yet due.

## Stop conditions

- A comparator that needs a host-side value the native lane cannot
  report.
- The promotion needs a kernel or trust change, or an entropy fallback.

Checks: 4 fired. I read the three availability lines, the interpreter and
verify-host bodies and the planner's refusal list on `ccfc01de1`. The
draft's missing value-capturing backend exists in `ken-verify/src/host.rs`.
Only the `ken-host` test backend records tags alone. 8 fired. AC-2's
backwards wall step and its second deadline each fail on a host that
ignores its input, and the entropy count row fails on a host that
returns a fixed buffer.
