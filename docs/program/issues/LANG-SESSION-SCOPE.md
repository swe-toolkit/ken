---
id: LANG-SESSION-SCOPE
title: "Give the incremental entry points (elaborate_decl, the REPL, harness declare_postulate_raw and tests' globals.insert) a per-session scope that resolution reads as locals, separate from the prelude's root scope, so no source-visible name depends on the flat globals table outside the prelude; L2-2 of the minimal-prelude program"
status: active
owner: language
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect program design evt_4s5he6tnf3xs4, L2-2: the incremental API and harness postulates bind into a per-session scope read as locals, the one legitimate source-visible use of non-prelude globals. Population: LANG-PRELUDE-FLOOR-FIFTEEN AC-0 (d). Also carries the Architect's ownership note from LANG-QUALIFIED-CONSTRUCTOR-PRIVACY D0 (evt_1nrnwakvjy2gj). Serves the operator's 2026-09-25 one-resolution-mode ruling. Steward-filed per COORDINATION section 2."
---

# Session scope for incremental and harness units

## Objective

A name bound by an incremental or harness entry point lives in a
per-session scope that resolution reads as a local. Outside the prelude,
nothing a source unit can see depends on the flat `globals` table. That is
the last source-visible use of `globals` the L2 flip has to replace.

## Settled inputs -- Architect `evt_4s5he6tnf3xs4`; to measure at the base

- **Population.** `LANG-PRELUDE-FLOOR-FIFTEEN` AC-0 (d) (merged) lists every
  non-source path into `globals`: `declare_postulate_raw` (`lib.rs:379`),
  sequential `elaborate_decl` (`lib.rs:389`), REPL sessions, and tests'
  `globals.insert`. Re-measure it at the base before building, and reconcile
  it against the D0 list.
- **Behaviour is unchanged under the current resolution mode.** Every
  existing incremental, REPL and harness suite still resolves the same names
  to the same `GlobalId`s.
- **Ownership (Architect note on `evt_1nrnwakvjy2gj`).** Ambient user code
  counts as the owner of every prelude family because it shares the
  prelude's root scope: the `owner_member` test at `modules.rs:730` reads the
  root scope's `checked_local_ids`. With a session scope, a session unit is
  not the prelude's scope. The Architect rules at D0 whether the session
  scope also removes that ownership, or whether that waits for the flip.
- **Out of scope.** The floor additions for keyed names, the flip that
  deletes legacy mode and the census, and the no-binding rule.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`ElabEnv` holds a session scope distinct from the prelude's root scope. The
incremental API, the REPL and the harness entry points bind into it, and
resolution reads it as locals ahead of any fall-through to `globals`. Tests
that installed names with `globals.insert` use the session entry point
instead. No kernel, `trusted_base()` or spec change.

## Acceptance

- **AC-0 (D0).** The re-measured population, as (entry point, file, count),
  and the Architect's ruling on the form and on ownership.
- **AC-1 (behavioural).** With the `globals` fall-through disabled in
  scratch for session-bound names only, the incremental, REPL and harness
  suites stay green, which shows they now resolve through the session scope.
  On base the same scratch change reds them. Report the red set as the
  measurement.
- **AC-2 (identity).** A name bound in a session resolves to the exact
  `GlobalId` its binding produced, in expression, pattern and type position.
  A later session binding of the same spelling does not change an earlier
  unit's checked reference.
- **AC-3 (controls).** The strict-resolution census is byte-unchanged. The
  prelude's hidden constructors stay unreachable
  (`lang_qualified_constructor_privacy`). Targeted builds only, through
  `scripts/ken-cargo`; no-regression means green in CI.

## Stop conditions

- Any kernel, `trusted_base()` or spec change, or a changed census row.
- A session-bound name would resolve to a different `GlobalId` than on base.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
