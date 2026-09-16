---
id: RT-NATIVE-RUN-OPTIONS-RESERVED-KEY-SILENT-OVERRIDE
title: "NativeEffectRunOptionsV1 silently discards a caller-supplied value for KEN_HOST_OBSERVATION_PATH: the launcher applies options.environment and then sets the reserved key, so Command::env's insert-or-update overwrites the caller's value before exec with no error, no warning, and no documented precedence. Refuse the reserved key instead of overwriting it."
status: ready
owner: runtime
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_a805g70ww27n, 2026-09-16, found while dispositioning row 5 of RT-IGNORED-PASSING-ROWS-DISPOSITION; the clobber itself was found by runtime-leader. Filed separately on the Architect's explicit instruction ('File it against the runtime lane; do not fold it into this WP') because the disposition node's scope is its eleven rows and this is production behaviour reachable by every caller of the API. Steward re-derived every fact below at origin/main c4a21bb0fa658477f04e670764485634485dd98f before filing; see the correction note at the end."
---

> ## RELEASED to Team Runtime 2026-09-16 — `ready`, size S, tier T2
>
> **One check before the spawn, returning the existing error type.** The
> ruling is settled (below); what is left is mechanical. The test-side edit to
> `px8x_single_schema_observation.rs` is **not** this node's — it belongs to
> `RT-IGNORED-PASSING-ROWS-DISPOSITION` row 5 and is already dispatched.

## The defect

`crates/ken-runtime/src/object_linker_packaging.rs:325-332`, at
`origin/main`:

    let output = Command::new(&artifact.executable_path)
        .args(&options.arguments)
        .env_clear()
        .envs(options.environment.iter().cloned())        // caller's values
        .env("KEN_HOST_OBSERVATION_PATH", &trace_path)    // LAST WRITE WINS
        .current_dir(&cwd)

`Command::env` **inserts or updates**. A caller who supplies
`KEN_HOST_OBSERVATION_PATH` through `options.environment` has that value
discarded before exec. **No error, no warning, and the precedence is documented
nowhere** — not on `NativeEffectRunOptionsV1`, not on
`run_bound_process_effect_observation`, not on its `_with_stdin` sibling.

The trace sink is launcher-owned and outside the capability root, so the
launcher **winning** is correct. **Winning silently is the defect.** The caller
made a request, the API accepted it, and the request had no effect.

## Why this is more than hygiene

**It is a fail-silent contract**, and it is the parent node's own thesis
appearing in production code rather than in a test label: *a claim in a position
of authority that nothing enforces.* The caller's environment map reads as the
program's environment. For one key it is not, and nothing says so.

**The measured consequence already exists.**
`crates/ken-cli/tests/px8x_single_schema_observation.rs:74-77` sets the key to
`"caller-controlled"` to exercise the filter. It never reaches the child. The
row's name — `linked_route_exposes_real_ordered_bindings_and_filters_reserved_input`
— claims coverage of a filtering decision that the row cannot observe, and the
row is one of eleven being readmitted from `#[ignore]` today. **The author built
the setup and never checked that it took, because nothing told them.**

## Do not read the census as evidence it is rare

Exactly **one** caller passes the reserved key through `options.environment`
today, and it is the inert one above. **That number measures observed instances
of a silently-tolerated mistake, not the rate at which it is made** — `1` and `0`
look identical here for the same reason. Do not size this off the count.

Full census at `c4a21bb0fa658477f04e670764485634485dd98f`:

| site | passes the reserved key? | affected |
|---|---|---|
| `px8x_single_schema_observation.rs:74` | yes, via `options.environment` | **yes — the instance above** |
| `px8f_buffer_native.rs:555,842` | no (`LD_PRELOAD`) | no |
| `px8f_write_partition.rs:284` | no (`LD_PRELOAD`, two `KEN_PX8F_*`) | no |
| `px4b_native_production.rs:246,483` | yes, but on a **direct `Command`** | no — bypasses the API |

`px4b`'s two sites spawn `Command` themselves rather than going through
`run_bound_process_effect_observation`, so they legitimately own the child's
environment and are not in this defect's population. **They are worth knowing
about because they are what the API's callers would have to fall back to** if
some caller genuinely needs to pass the key through.

## The closure

**Refuse a caller-supplied reserved key rather than overwriting it.** One check
before the spawn, returning the existing `NativeEffectRunErrorV1` error type.

**Documenting the precedence instead is the weaker option**, and it is
admissible only if some caller legitimately needs to pass the key through —
**and if one does, the current code is already wrong for that caller, silently.**
No such caller exists today.

The check belongs in `run_bound_process_effect_observation_with_stdin`
(`:294`), which is the single spawn path: the no-stdin entry point at `:284`
delegates to it, so one check covers both.

## Acceptance

**AC-1. A caller-supplied `KEN_HOST_OBSERVATION_PATH` is refused, not
overwritten.** Control: a test passing that key through
`options.environment` gets an `Err` of the existing error type. Cite the
variant used; do not add a new one unless you say why the existing set cannot
carry it.

**AC-2. The refusal is reached from BOTH entry points.** Control: the same
assertion through `run_bound_process_effect_observation` and through
`run_bound_process_effect_observation_with_stdin`. The delegation at `:289`
means one check should serve both — **show it, do not assume it.**

**AC-3. A non-reserved environment key is unaffected.** Control: a caller
passing `LD_PRELOAD` and a custom key still runs and still observes them. This
is the positive control and it is what keeps the change a refusal rather than a
restriction — `px8f_write_partition.rs:284` is the exact shape.

**AC-4. The precedence is stated where a caller reads it.** A doc comment on
`NativeEffectRunOptionsV1::environment` naming the reserved key and the
refusal. **The rule that is now enforced must also be findable** — the enforcement
and the documentation close different halves.

**AC-5. No change to the trace-sink mechanism.** Control: the diff does not
touch `trace_path` derivation, the C-shim filter loops
(`object_linker_packaging.rs:2197-2245`), or the arena sizing. **The shim's
filtering is correct and is not in scope.**

**AC-6. The two in-file test fixtures are untouched.** `:3531` and `:3698` set
the key on a direct `Command` inside `#[cfg(test)]` (module at `:2398`) and take
no caller environment. They are not instances of this defect.

**AC-7. No `#[ignore]` rows added, and no decorative glyphs in the diff.**

## Scope boundary

**The `px8x` row's rename and the deletion of its inert `environment` argument
belong to `RT-IGNORED-PASSING-ROWS-DISPOSITION` row 5**, dispatched to the
runtime implementer separately. A candidate for this node that renames a test
row is out of scope.

**Contention:** `object_linker_packaging.rs` is the runtime lane's live
surface. Check for an in-flight candidate against it before starting.

## Correction to the relayed coordinates

**The file was cited as `crates/ken-cli/src/object_linker_packaging.rs` by two
seats. That path does not exist at `origin/main`.** The file is
`crates/ken-runtime/src/object_linker_packaging.rs`; the line numbers in both
posts were correct. Every fact in this node was re-derived at
`c4a21bb0fa658477f04e670764485634485dd98f` rather than carried from the thread,
which is how the crate name was caught.

## Local build discipline

Targeted only, through `scripts/ken-cargo`, `-p ken-runtime`. **Never
`--workspace`.** `COORDINATION §12`.
