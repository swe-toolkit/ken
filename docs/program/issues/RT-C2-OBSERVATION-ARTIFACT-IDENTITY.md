---
id: RT-C2-OBSERVATION-ARTIFACT-IDENTITY
title: "The always-on `dasm-c2-observation` feature has no artifact-identity control, and the always-on choice is what makes the off-configuration unreachable from the crate the controls live in -- the sibling's nested-cargo A/B needs a carrier feature before it can be reused"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_7cyndqwye5sfr on the landed squash 6b3b5b40 (RT-DYNAMIC-ARM-SCALAR-MERGE c3), Findings 1 and 2. The gap originates in the Architect's c2-pre finding, whose heading clause the Steward's c3 frame did not derive a deliverable from. Every load-bearing fact below re-measured by the Steward against main 6b3b5b40 before filing."
---

## What this is

**A stated gap on a node that closed COMPLETE.** `RT-DYNAMIC-ARM-SCALAR-MERGE`
discharged one clause of a two-clause finding. This node is the other clause.

The Architect's `c2-pre` finding said the `dasm-c2-observation` gating is
asymmetric with its sibling **in the direction that makes an identity control
more necessary, and it is the one without one**. `c3` recorded the trade — which
was the finding's remedy sentence — and shipped. **Nothing measures whether
compiling with the feature on changes the emitted native artifact.**

`c3`'s claim is that the disabled observation path is *free*. The Architect
grounded that by **reading**: `lowered_value_kind` is `&Lowered → &'static str`,
one arm per variant, no `_ =>`, an ordinary production function with ~40 call
sites, therefore pure. That reading is sound and it is not a measurement. This
node supplies the probe.

## Fixed inputs, measured at `main` `6b3b5b40`

**1. The gap is real.** `dasm_c2` / `dasm-c2-observation` occur in five files
under `crates/`:

```
crates/ken-elaborator/Cargo.toml
crates/ken-elaborator/tests/nc14_data_match_lowering.rs
crates/ken-runtime/Cargo.toml
crates/ken-runtime/src/cranelift_backend.rs
crates/ken-runtime/src/cranelift_backend/lowering/mod.rs
```

The only test-side file **uses** the observation. Nothing compares a feature-off
artifact to a feature-on one.

**2. The control to copy already exists, for the sibling feature.**
`crates/ken-elaborator/tests/r3_c2_source_mixed_branch.rs:621`,
`r3_4b_observation_feature_is_native_artifact_identical`. It spawns a nested
`cargo test` with `--manifest-path`, `--no-default-features`, its own
`--target-dir`, and an artifact output dir, adding `--features r3-4b-observation`
for the on-run; then asserts the two emitted native objects are byte-identical
(`:641`).

**3. The reason it cannot simply be copied, and this is the whole design
content of the node.** The two features are carried differently in
`crates/ken-elaborator/Cargo.toml`:

```toml
:13   r3-4b-observation = ["ken-runtime/r3-4b-observation"]        # a ken-elaborator feature
:30   ken-runtime = { path = "../ken-runtime", features = ["dasm-c2-observation"] }   # dev-dep, pinned on
```

⇒ **The sibling is toggled through a `ken-elaborator` feature, so
`--no-default-features` reaches its off-configuration. `dasm-c2-observation` is
pinned in the dev-dependency declaration, which `--no-default-features` does not
touch.** From `ken-elaborator`, the off-configuration is unreachable, and that
is a direct consequence of the always-on choice `c3` correctly made.

## The design call — the DEFAULT-ON CARRIER ROUTE IS REFUTED BY MEASUREMENT

**The carrier route below was the Steward's front-loaded call. Runtime built
exactly the framed shape and measured that it does not work.** The frame
pre-authorized this exit ("if the carrier route turns out not to work, reporting
that with the measurement is a good outcome"), so this is the planned branch and
not a hard stop. The refuted route is kept because the reason it fails is the
node's central fact.

**The refuted route:** give `ken-elaborator` its own feature forwarding to the
runtime one, put it in `default`, drop the `features = [...]` from the
dev-dependency line.

```toml
[features]
default = ["dasm-c2-observation"]              # REFUTED — see below
dasm-c2-observation = ["ken-runtime/dasm-c2-observation"]
```

**Why it cannot work, measured by runtime-implementer 2026-08-14.** The
package-scoped `--no-default-features` build still compiled
`dasm-c2-observation`. `ken-elaborator` dev-depends on `ken-interp`
(`ken-elaborator/Cargo.toml:29`), and `ken-interp` depends back on
`ken-elaborator` **with default features** (`ken-interp/Cargo.toml:18`, no
`default-features = false`). Cargo unions feature activations, so the cycle
re-activates `ken-elaborator/default` — and therefore the carrier — in every
build that includes dev-dependencies, which is every `cargo test`. The
anti-vacuity worker caught it before the artifact comparison: *"nested build did
not compile the requested D5 observation configuration; left: true, right:
false"*. Confirmed independently by `scripts/ken-cargo tree -p ken-elaborator
--no-default-features -e features -i ken-runtime`.

⇒ **The general fact, and it is the durable content of this node:
`--no-default-features` is a NO-OP for any package that one of its own
dev-dependencies depends back on.** Putting a feature in `default` to keep it
always-on in the ordinary run is therefore the same knob as making its
off-configuration unreachable. At `ken-elaborator` the two properties this node
needs — "on in the ordinary targeted run" and "reachable off-configuration" —
are **mutually exclusive**, and no wording of a carrier feature separates them.

**This is also why the sibling control works and looked copyable.**
`r3-4b-observation` is **not** in `default`, so the cycle re-activating
`default` re-activates nothing. The sibling never depended on
`--no-default-features` doing anything; its off-configuration is the natural
state. That difference is invisible when reading the two features side by side.

**Note the in-tree comment at `ken-interp/Cargo.toml:15-17`** — *"elaborator's
own ken-interp dependency is dev-only (tests), so this is not circular."* It is
not circular for **linking**, and it is circular for **feature resolution**.
Anyone re-deriving this route will read that comment and reach the refuted
conclusion.

## The new route — host the nested build in `ken-cli`, and NO carrier

**Taken by the Steward, a test-topology call; the Architect reviews it on the
merge Decision. Every fact below re-measured against `main` `ca803dfc`.**

**`ken-runtime` is eliminated, not deprioritized.** The Adversary named
"`ken-runtime` or `ken-cli`" as the fallback and did not run either.
`ken-runtime` **cannot** host it: emitting the artifact requires elaborating a
Ken source, and `ken-runtime` has no `ken-elaborator` dependency
(`ken-runtime/Cargo.toml:37-45`) — the direction is `ken-elaborator →
ken-runtime`. Do not spend a turn discovering this.

**`ken-cli` can, and it has the property `ken-elaborator` lacks:**

1. **No dev-dependency cycle.** `ken-cli`'s only dev-dependency is `ken-runtime`
   with `px8-ds-test-support` (`ken-cli/Cargo.toml:25`), which does not depend
   back on `ken-cli`. Its one reverse dependency, `ken-verify/Cargo.toml:12`, is
   a normal dependency. So a `ken-cli` feature is honestly togglable from the
   command line.
2. **It can already emit the artifact.** `compile_native_program_sources` is
   `pub` at `ken-elaborator/src/compiler_driver.rs:2524` and **`ken-cli` already
   calls it.** The worker has the input it needs.
3. **The carrier disappears from the design.** `ken-cli` has no `[features]`
   section at all; add `dasm-c2-observation =
   ["ken-runtime/dasm-c2-observation"]` and **do not put it in `default`**. Off
   becomes the natural state and on requires `--features` — mirroring the
   sibling exactly, which is the shape already known to work.

**The decisive consequence: `ken-elaborator/Cargo.toml:30` is not touched.** The
ordinary `scripts/ken-cargo test -p ken-elaborator` keeps its pinned dev-dep, so
both `D5` seat controls keep running and the target stays at its measured 17/17.
`D3`/`AC-2` are preserved by **not touching them** rather than by re-measuring
after a change, and the "no removal of the always-on default" bar in "Not this
node" is honoured literally rather than argued around.

## Deliverables

**`D1` — the `ken-cli` feature stanza.** A new `[features]` section carrying
`dasm-c2-observation = ["ken-runtime/dasm-c2-observation"]` and **no `default`
key**. Report the `Cargo.toml` diff. **`ken-elaborator/Cargo.toml` must appear
in no diff for this node.**

**`D2` — the identity control, hosted in `ken-cli/tests/`**, modelled on
`r3_4b_observation_feature_is_native_artifact_identical`
(`ken-elaborator/tests/r3_c2_source_mixed_branch.rs:568-660`): an outer driver
that spawns a nested `cargo test --manifest-path <ken-cli>/Cargo.toml --test
<worker>` twice with **its own `--target-dir` each time**, adding `--features
dasm-c2-observation` on the on-run, and a worker that emits one Ken source's
native object into a per-configuration output dir. Assert the two objects are
byte-identical. **Reuse the sibling's `env!("CARGO")` detail and its reason** —
the outer `scripts/ken-cargo` already holds the machine-wide lock and taking it
recursively deadlocks (`:576-578`).

**`D3` — the `D5` seat controls are unchanged, verified not re-engineered.**
Report both test names and the target's test count from an ordinary
`scripts/ken-cargo test -p ken-elaborator --test nc14_data_match_lowering` with
no feature flag. Under the new route this deliverable is a **confirmation that
nothing moved**, since no `ken-elaborator` file is edited.

## Acceptance criteria

**`AC-0` — the control is TWO `-p`-SCOPED BUILDS, not an assertion inside the
workspace test run. This row decides whether `D2` measures anything.**

**Architect constraint, `evt_6cm8tg834zseb`, and it is the reason this node is
not one afternoon's work.** `ken-runtime` is both a normal dependency
(`ken-elaborator/Cargo.toml:18`) and a dev-dependency (`:30`). **Cargo unifies
features across a build graph**, so in CI's `--workspace` run any crate enabling
`ken-runtime/dasm-c2-observation` turns it on for the whole graph — and the
carrier being off in `ken-elaborator` will not produce a feature-free artifact.

⇒ **An identity assertion written to run inside the ordinary workspace test run
is vacuous by construction: it compares two artifacts unification has already
made identical.** It passes, and it passes for the wrong reason, which is the
exact failure this node exists to close.

**So the AC is about the INVOCATION, not the property.** Two `-p`-scoped builds
— `--no-default-features` versus default — **each with its own
`CARGO_TARGET_DIR`**, compared as artifacts. **If the control is expressible only
as "assert X in a test", it is the wrong control.** Report the two command lines.

**~~One thing the Architect measured that makes the carrier route clean:~~** —
**struck, false, and it was the sentence that made the refuted route look
safe.** `ken-elaborator` does have `default = []`, but the inference drawn from
it does not hold: adding a default-on carrier does **not** turn
`--no-default-features` into "exactly and only observation off", because the
`ken-interp` dev-dependency cycle re-activates `ken-elaborator/default`
regardless of the flag. See the refutation above. The true premise was `default
= []`; the false step was mine, in reading a suppressible default off it.

**What survives, and it is what `AC-0` was really for:** the invocation must be
two package-scoped builds with separate `CARGO_TARGET_DIR`s, and the feature must
be toggled at a package whose feature set the command line can actually reach.
`AC-0` is satisfiable at `ken-cli` and is **not satisfiable at
`ken-elaborator`** by any wording. Report the two command lines.

**`AC-1` — `D2` is exercised against a real difference, not merely green.**
Perturb the observation path so it *does* affect emission (for example, leave a
computation outside the `ENABLED` guard), confirm `D2` **reds** naming the two
artifacts, and restore. **Report the failing text.** A byte-identity assertion
that has never seen a difference is not known to be a control — that is the
shape this whole arc keeps retiring.

**`AC-2` — the default run is unchanged.** `D3`'s two controls pass with no
feature flag, and the count of tests in that target is what it was at
`6b3b5b40`. **If the count moves, stop and report it.**

**`AC-3` — the disabled path's freeness is now MEASURED, and say which claim
`D2` does and does not settle.** `D2` settles that emission is unaffected. It
does **not** settle timing or allocation. State that boundary rather than
letting a green `D2` read as the stronger claim.

**`AC-4` — separate target directories, and say where they went.** An
artifact-level A/B that shares a `cargo` target directory measures nothing —
this is `AC-0`'s second half and it is the half that silently no-ops. **This box
has filled seven times** — report the paths used and that they were cleaned up.

**`AC-5` — no production change.** `crates/ken-runtime/src/` is untouched apart
from any perturbation made and restored under `AC-1`. The six admitted merge
shapes and the fail-closed `_ =>` are not in scope. **The `D1` feature stanza is
not a production change because it carries no `default` key** — the ordinary
`ken-cli` build activates nothing new. Say so with the diff rather than leaving
it inferred.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never
a local `--workspace` run.

## Sizing

**`S`**, and the one-hour target applies. `D1` plus a failing-then-passing `D2`
is a releasable increment.

**Increment 1 (2026-08-14) spent its turn refuting the carrier route** and
returned no candidate and no source residue — the implementer restored every
edit, attempted no fallback, and perturbed nothing. That is the outcome the frame
asked for on that branch, and the measured impossibility is now a fixed input
above rather than a fact to rediscover. **Increment 2 is the `ken-cli` route and
starts from a clean tree.**

**`AC-0` is where this node is most likely to go wrong, and it will look
green while doing it.** If you find yourself writing `assert!` inside a test that
CI's workspace run executes, stop — that is the vacuous shape, not the control.

## Not this node

- **No change to the `match lowered` arm set, the admitted shapes, or the
  fail-closed `_ =>`.** That boundary is what `c1` and `c2` were cut to
  establish.
- **No removal of the always-on default.** The `D5` seat controls must keep
  running in the ordinary targeted test; a change that makes them opt-in is the
  trap `c3` avoided.
- **No second observation instrument.** Use the shape `c3` left —
  `dasm_c2_scalar_merge_observation_scope` at `lowering/mod.rs:16042`.
- Not the double-read residual on [[RT-DYNAMIC-ARM-SCALAR-MERGE]]; that rides
  the next candidate entering `lowering/mod.rs`.

## Sequencing

**Behind [[RT-NESTED-IH-NATIVE-REALIZATION]], which is the critical path.** This
node blocks nothing and gates no Kernel work. It exists so a measured gap on a
closed node is scheduled rather than remembered.
