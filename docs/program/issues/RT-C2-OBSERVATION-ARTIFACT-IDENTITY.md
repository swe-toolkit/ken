---
id: RT-C2-OBSERVATION-ARTIFACT-IDENTITY
title: "The always-on `dasm-c2-observation` feature has no artifact-identity control, and the always-on choice is what makes the off-configuration unreachable from the crate the controls live in -- the sibling's nested-cargo A/B needs a carrier feature before it can be reused"
status: ready
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

## The design call, front-loaded — add a DEFAULT-ON carrier feature

**Taken by the Steward. It is a test-topology call, the same class as the calls
the `c3` frame front-loaded; the Architect reviews it on the merge Decision.**

**Price this route first, because it is one line and it makes input 2 directly
reusable:** give `ken-elaborator` its own feature that forwards to the runtime
one, put it in `default`, and drop the `features = [...]` from the dev-dependency
line:

```toml
[features]
default = ["dasm-c2-observation"]
dasm-c2-observation = ["ken-runtime/dasm-c2-observation"]
```

**This preserves the property `c3` was protecting and unblocks the control at
the same time.** An ordinary `cargo test -p ken-elaborator` still has the
feature on, so both `D5` seat controls in `nc14_data_match_lowering.rs` still
compile and run in the default targeted run — that was the trap `c3` avoided and
this must not re-open it. A nested `--no-default-features` run now reaches the
off-configuration.

**This is the "second default-on carrier" the `c3` rationale said an opt-in
dependency would need.** That rationale was correct about opt-in and is not
being overturned; what it did not consider is that the carrier can be
**default-on**, which costs one line and is not the opt-in shape it rejected.

**If measurement shows this route does not work, say so and stop.** The
Adversary named the fallback — drive the nested cargo from `ken-runtime` or
`ken-cli`, which do not carry that dev-dependency — and **explicitly did not run
it**. Report which route you took and why. Do not build both.

## Deliverables

**`D1` — the carrier.** Whichever route input 3 resolves to. Report the
`Cargo.toml` diff.

**`D2` — the identity control**, modelled on
`r3_4b_observation_feature_is_native_artifact_identical`: compile one Ken source
twice, feature-off and feature-on, into **separate target directories**, and
assert the emitted native objects are byte-identical.

**`D3` — the `D5` seat controls still run by default.** Report both test names
from an ordinary `scripts/ken-cargo test -p ken-elaborator --test
nc14_data_match_lowering` with no feature flag.

## Acceptance criteria

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
artifact-level A/B that shares a `cargo` target directory measures nothing.
**This box has filled seven times** — report the paths used and that they were
cleaned up.

**`AC-5` — no production change.** `crates/ken-runtime/src/` is untouched apart
from any perturbation made and restored under `AC-1`. The six admitted merge
shapes and the fail-closed `_ =>` are not in scope.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never
a local `--workspace` run.

## Sizing

**`S`**, and the one-hour target applies. `D1` plus a failing-then-passing `D2`
is a releasable increment; if the carrier route in input 3 turns out not to
work, **reporting that with the measurement is a good outcome** and the fallback
route is the next turn's, not this one's.

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
