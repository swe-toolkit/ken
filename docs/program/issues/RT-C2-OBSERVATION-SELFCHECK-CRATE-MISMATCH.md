---
id: RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH
title: "The artifact-identity control's anti-vacuity self-check reads `ken-cli`'s `dasm-c2-observation` feature while the property that decides whether the two artifacts differ is `ken-runtime`'s -- they agree today and nothing holds them together, so a future `ken-cli` dev-dependency enabling the runtime feature would make the off side ON with the assertion still reading `disabled` and no signal at all"
status: ready
owner: runtime
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect carry (1) on the RT-C2-OBSERVATION-ARTIFACT-IDENTITY merge Decision dec_3bjbha9s3rgtr, verdict evt_50h83hackbgen. Explicitly non-blocking and explicitly named a follow-up node. Filed by the Steward at merge time rather than carried, because the immediately preceding node on this same arc lost a clause exactly this way."
---

## What this is

**A carried clause from an approved merge, filed as a node so it is scheduled
rather than remembered.** `RT-C2-OBSERVATION-ARTIFACT-IDENTITY` landed a working
artifact-identity A/B. Its anti-vacuity guard asserts that the nested build
compiled the requested configuration:

```rust
let actual = if cfg!(feature = "dasm-c2-observation") { "enabled" } else { "disabled" };
assert_eq!(actual, expected, "nested build did not compile the requested D5 observation configuration");
```

**That `cfg!` reads `ken-cli`'s feature.** The property that actually decides
whether the two artifacts were built differently is **`ken-runtime`'s**
`dasm-c2-observation`. Two different facts.

**They agree today** — `ken-cli`'s feature forwards to the runtime one and
nothing else enables it — and the Architect measured that agreement at
`d2584baf`: the off side resolves `ken-runtime` with `default` +
`px8-ds-test-support` and `dasm-c2-observation` absent; the on side adds exactly
`dasm-c2-observation`. **Nothing holds them together going forward.**

## The failure mode, which is silent in the worst direction

Add a `ken-cli` dev-dependency that enables `ken-runtime/dasm-c2-observation`
— the same shape `ken-elaborator/Cargo.toml:30` already has — and:

- the **off** side is built with the runtime feature **ON**;
- the self-check still reads `ken-cli`'s feature, finds it off, and reports
  **`disabled`**;
- the A/B compares **on against on**, is byte-identical, and passes.

⇒ **The guard that exists to prevent a vacuous pass becomes the thing that
certifies it.** There is no red anywhere, and the control reports success under
exactly the swap it was built to catch.

**This is not hypothetical for this crate family.** The dev-dep pin at
`ken-elaborator/Cargo.toml:30` is precisely this shape and is why the control
could not be sited in `ken-elaborator` at all — see the refuted carrier route on
[[RT-C2-OBSERVATION-ARTIFACT-IDENTITY]].

## The fix, front-loaded by the Architect

Have `ken-runtime` export the fact about itself, and assert on that:

```rust
pub const DASM_C2_OBSERVATION_COMPILED: bool = cfg!(feature = "dasm-c2-observation");
```

The control then measures the crate whose compilation actually differs, and the
two facts cannot drift apart because there is only one.

**This was correctly out of scope on the parent node** — `crates/ken-runtime/`
was a forbidden path there (`AC-5`, no production change), so the parent could
not have fixed it even having seen it.

## Deliverables

**`D1`** — the `pub const` in `ken-runtime`, and the worker's self-check
switched to it.

**`D2`** — a comment at the assertion naming why it reads the runtime constant
rather than the local `cfg!`, so the next author does not "simplify" it back.

## Acceptance criteria

**`AC-1` — the guard still fires.** Re-run the parent's anti-vacuity proof:
drive the nested build with the wrong `EXPECTED_CONFIGURATION` and confirm the
assertion reds. **Report the failing text.** A guard rewritten without being
re-fired is not known to be a guard.

**`AC-2` — the new form catches what the old form cannot.** Add the
`ken-runtime/dasm-c2-observation`-enabling dev-dependency to `ken-cli`
temporarily, confirm the self-check now **reds** on the off side where the old
`cfg!` read `disabled` and passed, then **restore**. This is the whole point of
the node; without it `D1` is an unmeasured refactor.

**`AC-3` — the identity control still passes** unperturbed, and the AC-1
mutation from the parent still moves only the on artifact.

**`AC-4` — `ken-elaborator` is not touched**, and the `D5` seat controls stay at
17/17. Same bar as the parent.

**`AC-5` — no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run.

## Sizing

**`XS`.** One `pub const`, one assertion, and the `AC-2` temporary edit. The
measurement is the work; the code is three lines.

## Not this node

- **Not the CI-cost carry.** The Architect's carry (2) — two nested cold Cargo
  builds per suite run — is a separate question and is not addressed here. If it
  becomes a drag the recorded lever is `#[ignore]` plus an explicit job,
  **never** removing the separate target directories, which are the mechanism
  rather than hygiene.
- **No change to the A/B's route.** Hosting in `ken-cli` is settled.
- **No second observation instrument.**
