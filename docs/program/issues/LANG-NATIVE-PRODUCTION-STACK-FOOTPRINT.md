---
id: LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT
title: "`ken-cli` native production runs `px4b_native_production` at effectively zero stack margin -- base passes with a few hundred bytes to spare, so any candidate adding a few hundred bytes aborts it, and `98e6ac51` is the trigger that exposed this rather than its cause"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_81rtcqabscq5 in thr_4895dsr8yt6jv, arising from the 98e6ac51 CI abort and two recut attempts. The Architect withdrew their own premise that register_prelude's frame was live, and adopted the implementer's read of the second recut's clean negative. Sequencing routed to the Steward. Steward re-measured main at 79fddb0d before filing."
---

## What this is

**A `main`-level defect exposed by a candidate, not a candidate defect.**
`ken-cli::px4b_native_production two_vis_nodes_resume_once_in_source_order`
aborts with `fatal runtime error: stack overflow` (SIGABRT) when compiled
against `98e6ac51`, and passes at base.

**Both of the following are true, and they are compatible:**

- an A/B **attributes** the abort to `98e6ac51`;
- moving that candidate's two `BTreeSet` snapshots into non-inlined helpers
  **did not measurably move the failure**, because they were never a meaningful
  fraction of the deficit.

⇒ **The margin is effectively zero.** Base passes with a few hundred bytes to
spare; a few hundred bytes added anywhere on that path aborts it. *"Not a
meaningful fraction of the total"* and *"enough to cross the line"* are both
true at once.

**So the path is armed for whatever lands next**, and the next candidate to
trip it will look equally guilty and equally not be the cause.

## Fixed inputs, measured at `main` `79fddb0d`

**1. The abort.** `test shard 4/4`, run `31778136605`:

```
SIGABRT [0.196s] (18/629) ken-cli::px4b_native_production two_vis_nodes_resume_once_in_source_order
fatal runtime error: stack overflow, aborting
```

**2. The sequential structure that decides where the frame is.**
`crates/ken-elaborator/src/compiler_driver.rs:2088-2096`:

```rust
2088:    let mut env = ElabEnv::new()          // register_prelude lives here, and RETURNS
2090:        .map_err(NativeProgramBuildError::Driver)?;
2091:    let mut admitted_ids = Vec::new();
2092:    for source in &sources {
2096:            env.elaborate_file(&source.text)   // the user program: deep elaboration
```

**`ElabEnv::new()` returns before `elaborate_file` runs.** They are sequential
siblings, not nested, so `register_prelude`'s frame and the user-program
elaboration frame **are not on the stack at the same time**. This is the fact
whose absence sent two recuts at the wrong frame.

**3. `px4b_native_production.rs` provisions no stack** and calls
`build_native_program` eleven times, unlike five sibling `ken-cli` tests. **That
asymmetry is NOT this node's to resolve** -- see "Not this node".

## `D0` -- DISCHARGED. The node ships aimed.

**Measured by language-implementer, reported at `evt_44qzssyvaay0b`
(language-leader relay `evt_3d8j56s1wnvb7`).** The Architect's marker was
inserted after `compiler_driver.rs:2090`:

```rust
eprintln!("KEN-PROBE elabenv-new-returned");   // temporary, do not commit
```

**The marker NEVER PRINTED.** The targeted run exits 101 with the same
`two_vis_nodes_resume_once_in_source_order` SIGABRT and no `KEN-PROBE` line.
Byte-identical restoration and a clean branch/worktree confirmed.

⇒ **The overflow is inside `register_prelude` itself, not user-program
elaboration.** `elaborate_file` is never reached.

**Do not re-run this.** `D1` starts from it.

**What that does and does not license.** It locates the overflow to
`register_prelude`'s frame -- but the second recut already measured that
**`98e6ac51`'s two snapshots are not the deficit**. Both hold together only one
way: the deficit is the **rest** of that function's ~450-declaration frame,
which sits near the limit before the candidate adds anything. **`D1` targets
that frame's footprint, not the guard.**

## Deliverables

**`D1` — the deficit located to a named frame**, with the measurement that
locates it. A depth or footprint number, not a narrative.

**`D2` — the footprint reduced on that frame**, by the same standard the gate-4a
arc used: reduce what the frame holds, do not raise what it is given.

**`D3` — the margin stated as a number.** After `D2`, how much headroom does
this path have? **The node's real product is that this number exists**, because
its absence is why a few hundred bytes read as a candidate defect for two
recuts.

## Acceptance criteria

**`AC-1` — `98e6ac51` merges unchanged on top of this.** The control is the
exact frozen SHA, not a re-authored equivalent: compile `ken-cli` against it
after `D2` and confirm `px4b_native_production` passes. **If it does not, this
node has not finished**, whatever the footprint numbers say.

**`AC-2` — the margin is measured, not inferred.** `D3`'s number comes from a
probe (a deliberate frame-size perturbation that finds the cliff), not from
arithmetic on struct sizes. **A margin nobody has probed is how this arrived.**

**`AC-3` — the fix is a reduction, not a raise.** No `RUST_MIN_STACK`, no
stack-limit raise, no `stack_size` added to `px4b_native_production.rs`.
`LANG-RECORD-STACK-OVERFLOW` refuses it and the Architect restated the refusal
in the ruling. **This is the criterion most likely to be reached for under
pressure.**

**`AC-4` — the two frames are distinguished in whatever is written down.**
`ElabEnv::new()` and `elaborate_file` are sequential siblings. Any comment or
node text this produces states that, so the next reader does not re-derive the
conflation that cost two recuts.

**`AC-5` — no weakening of the trusted-base guard.** Nothing here touches
`98e6ac51`'s design. The guard is correct and its approval stands; it cannot
merge onto a path with no headroom, and that is a property of the path.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run.

## Sizing

**`M`.** `D0` is minutes and may already be done. `D1` is the real work and is
where the hour goes. **If `D1` shows the deficit is spread rather than
concentrated in one frame, stop and report that** -- a diffuse deficit is a
different node and probably a different owner.

## Not this node

- **Not the candidate.** `98e6ac51` stays frozen and is **not** re-authored. It
  is re-voted unchanged on a fresh Decision after this lands.
- **Not the provisioning standard.** Whether `ken-cli` tests may provision their
  own stack is [[TEST-NATIVE-STACK-PROVISIONING-STANDARD]], `gate: architect`.
  **Its outcome must not be used to unblock this node or the candidate**, and a
  ruling there permitting provisioning does **not** retire the zero-margin
  hazard this node exists to close.
- **Not tuning any depth constant.**
- Not [[LANG-RECORD-STACK-OVERFLOW]], which is merged; its refusal of
  stack-limit raises is inherited here as `AC-3`.

## Sequencing

**This lands on `main` before `98e6ac51` is re-voted.** That order is the
Architect's recommendation and the Steward's call: the footprint node is the
substantive fix, and re-voting the candidate first would merge it onto the same
zero-margin path.

**This is Language's next WP after [[LANG-GADT-SEQUENCE-TRACKER-GAP]]**, or
before it if the implementer prefers to finish what they have context on --
that choice is language-leader's, and both are `gate: none`.
