---
id: RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH
title: "The artifact-identity control's anti-vacuity self-check reads `ken-cli`'s `dasm-c2-observation` feature while the property that decides whether the two artifacts differ is `ken-runtime`'s -- they agree today and nothing holds them together, so a future `ken-cli` dev-dependency enabling the runtime feature would make the off side ON with the assertion still reading `disabled` and no signal at all"
status: merged
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: 2196
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

## Adversary hunt `evt_11qb3zdtctexr` on `79fddb0d` -- CONFIRMED, and it upgrades this node

**Triaged by the Steward. Accepted in full; the node is resized `XS` -> `S` on
the strength of it.** Three things this node did not say:

**1. The two disagreement directions have OPPOSITE severities, and only one is
loud.**

| disagreement | outcome |
|---|---|
| `ken-cli` **on**, `ken-runtime` **off** | **compile error** -- the enabled arm calls `ken_runtime::dasm_c2_scalar_merge_observation_scope`, which is `#[cfg(feature)]`-gated |
| `ken-cli` **off**, `ken-runtime` **on** | **silent pass** -- worker reports `disabled`, driver expected `disabled`, guard agrees, artifact compiled **with the observation in** |

⇒ **The control cannot detect the one state that makes it vacuous**, and that
state is increment 1's failure mode exactly: feature-on compared against
feature-on, green under precisely the swap the control exists to catch.

**2. The structural gap is ONE ARM, not one crate -- and this reframes the
fix.** The enabled arm carries a real probe of `ken-runtime`'s state:
`!rows.is_empty()` proves the observer ran during the compilation being
compared. **The disabled arm probes nothing.** Nothing asserts the observation
was *absent* for the feature-off build. **The asymmetry is the whole exposure,
and it is statable without naming a crate at all.**

**3. The trigger is ALREADY SPELLED in the same manifest, nine lines from the
feature it would defeat.** `crates/ken-cli/Cargo.toml`:

```toml
[features]
dasm-c2-observation = ["ken-runtime/dasm-c2-observation"]                      # :19
[dev-dependencies]
ken-runtime = { path = "../ken-runtime", features = ["px8-ds-test-support"] }  # :28
```

The nested build is `cargo test`, so dev-dependencies are in the graph. **Adding
`dasm-c2-observation` to the list at `:28`** -- the same idiom
`ken-elaborator/Cargo.toml:30` already uses, and the most natural way anyone
would "turn the observation on for `ken-cli` tests" -- puts `ken-runtime`'s
feature on in **both** nested builds while `ken-cli`'s own feature stays off in
the feature-off one.

⇒ This moves the finding from *"nothing holds them together"* to **"a one-line
edit of a line already present defeats it silently, and a sibling crate has
already made exactly that edit."**

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

**`D3` — make the DISABLED arm probe, which is the Adversary's reframing and may
be the whole fix.** The enabled arm proves the observer ran; the disabled arm
must prove it did **not**. If that assertion is expressible, it closes the
silent direction whether or not `D1`'s constant lands, and it is statable
without reference to which crate owns the feature.

**`D4` — one sentence at `dasm_c2_artifact_identity_worker`** recording that it
returns early and reports ok when `KEN_DASM_C2_ARTIFACT_OUTPUT` is unset, so an
ordinary `-p ken-cli` run shows a green test that measured nothing. That is the
correct driver/worker split and the driver's `fs::read` of `ken-entrypoint.o` is
the real backstop -- but **a test whose default-configuration behaviour is
*return immediately* is the shape that later gets "cleaned up" into an
unconditional body or deleted as dead.**

## Acceptance criteria

**`AC-1` — the guard still fires.** Re-run the parent's anti-vacuity proof:
drive the nested build with the wrong `EXPECTED_CONFIGURATION` and confirm the
assertion reds. **Report the failing text.** A guard rewritten without being
re-fired is not known to be a guard.

**`AC-2` — the new form catches what the old form cannot, and the mutation is
now NAMED rather than invented.** Add `dasm-c2-observation` to the existing
dev-dependency feature list at `crates/ken-cli/Cargo.toml:28`, confirm the
control now **reds** on the off side where it previously read `disabled` and
passed, then **restore**. **This is the exact one-line edit the Adversary
identified**, so the mutation is the real trigger rather than a synthetic
stand-in. Without this `D1` is an unmeasured refactor.

**`AC-2b` — the silent direction is the one under test.** The loud direction
(`ken-cli` on, `ken-runtime` off) compile-errors and needs no control. **Do not
let a red in that direction stand in for `AC-2`** -- it proves nothing about the
direction that actually passes vacuously.

**`AC-3` — the identity control still passes** unperturbed, and the AC-1
mutation from the parent still moves only the on artifact.

**`AC-4` — `ken-elaborator` is not touched**, and the `D5` seat controls stay at
17/17. Same bar as the parent.

**`AC-5` — no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run.

## Sizing

**`S`**, resized from `XS` after the Adversary hunt. The code is still small;
what grew is the measurement -- `AC-2` now has a named mutation, and `D3` may
turn out to close the exposure more directly than `D1` does. **If `D3` alone
closes the silent direction, say so and price `D1` separately** rather than
landing both because both were listed.

## Not this node

- **Not the CI-cost carry.** The Architect's carry (2) — two nested cold Cargo
  builds per suite run — is a separate question and is not addressed here. If it
  becomes a drag the recorded lever is `#[ignore]` plus an explicit job,
  **never** removing the separate target directories, which are the mechanism
  rather than hygiene.
- **No change to the A/B's route.** Hosting in `ken-cli` is settled.
- **No second observation instrument.**
