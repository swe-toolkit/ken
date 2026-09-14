---
id: TEST-STATED-STACK-SITE-RECONCILE
title: "Reconcile the 15 stated-stack sites to the ruling -- and the first deliverable is CLASSIFYING each into one of the three acts, because the twelve 256 MiB sites need a measured peak that nobody has ever taken"
status: ready
owner: runtime
size: M
gate: none
depends_on: [TEST-NATIVE-STACK-PROVISIONING-STANDARD]
blocks: []
github: null
origin: "Split out of TEST-NATIVE-STACK-PROVISIONING-STANDARD by the Steward when the Architect's census (evt_4rz7hp11f33wj, measured at origin/main 10101777) refuted that node's population claim -- 15 sites in 14 files across 4 crates carrying 3 constants, against an asserted 6 sites in 5 files in 1 crate. Filed rather than carried, because a carry with no home is what evaporates."
---

> # UNBLOCKED AND `ready` 2026-08-14. THE DEPENDENCY LANDED.
>
> **[[TEST-NATIVE-STACK-PROVISIONING-STANDARD]] merged as PR #2216**, exact
> `695eff8b`, so the standard these sites are reconciled *to* is now in the tree
> at `agent/playbooks/tools/stated-stacks.md`. The Steward flipped this node
> from `draft` to `ready` in the same turn.
>
> **This node was `draft` because nothing had landed behind it, never because
> the frame was owed** — the frame below has been complete and shovel-ready
> since it was filed. Runtime lost a turn on 2026-08-13 to the opposite shape,
> pulling a node whose own dependency had not landed and finding its premise
> false at `D1`.
>
> **The venue text is the fixed input now, not this node's summary of it.** Read
> `agent/playbooks/tools/stated-stacks.md` and classify against that.
>
> # IT IS ALSO NOT RUNTIME'S NEXT NODE WHEN IT DOES FLIP.
>
> **The operator's standing priority is the `RecursiveDescent` retirement**
> (*"that is the priority for the runtime team. prioritize that work over other
> runtime work"*). This node sequences **after** that chain, and it is filed now
> so the frontier is written rather than discovered. **Sequencing, not merit.**
>
> **It also depends on [[TEST-NATIVE-STACK-PROVISIONING-STANDARD]] landing** —
> there is nothing to reconcile the sites *to* until the ruling is in its venue.

> # AMENDED 2026-09-14 (Steward, `evt_752h5xfry09dh`). THE CENSUS IS STALE AGAIN,
> # AND IT IS BLIND IN TWO DIRECTIONS THAT MATTER MORE THAN THE COUNT.
>
> **Re-measured at `origin/main` `7b070004`: 27 `.stack_size` call sites in 21
> files across FIVE crates** — `ken-cli`, `ken-elaborator`, `ken-kernel`,
> `ken-runtime`, `ken-verify`. The table below says 15 / 14 / 4 and does not name
> `ken-kernel` at all. **`AC-1` already told you not to inherit it; this is the
> second time it has been wrong by roughly a factor of two.**
>
> **BLINDNESS 1 — the census enumerates sites that STATE a stack, so it cannot
> see a test that NEEDS one and states none.** The blindness is real and the
> population is LARGER than first recorded. **The specific account written here
> on 2026-09-14 has been REPLACED — read the correction below, and do not carry
> the earlier one into a site comment.**
>
> **CORRECTED 2026-09-14 (Steward), from the ring's measurement at
> `evt_2q1ggxnpemwyn` on base `686ffa8ac`:**
>
> - **The claim *"it passes in the full suite, so nothing reds"* is FALSE.**
>   `crates/ken-cli/tests/abi_s6_mapping_file_backed_native.rs`, provisioned at
>   that base, is **7 passed / 11 failed**. It is heavily red, not silently
>   green.
> - **The single named member was the wrong test.** Unprovisioned, the full
>   suite overflows in
>   `exact_required_consumer_edge_queries_resource_bracket_ok`, not in
>   `absent_required_consumer_disposition_preserves_direct_and_tail_routes`.
> - **8 of those 11 failures are one lowering refusal**
>   (`CheckedIhDetachedCallerCut`), which is a compile-behaviour finding and not
>   a stack finding at all. See
>   [[RT-COMPILE-OUTCOME-RUN-CONFIGURATION-DEPENDENCE]].
>
> **THE CHEAP GENERAL ANSWER, which is the part worth more than the correction:**
> **`RUST_MIN_STACK=268435456` provisions every test in a run with ZERO source
> edits.** No `.stack_size` call, no fixture change, no `AC-3`/`AC-5` exposure,
> no contention with anyone's in-flight work. **Establish whether a site needs a
> stack by running it under that env var before proposing any source change** —
> it separates "needs headroom" from "needs a stated constant" without touching a
> line, and this node's whole `D3` measurement problem is cheaper under it.
>
> ⇒ **`D1`'s population is "sites that state a stack" UNION "sites measured to
> need one and state none."** The second set is not grep-able, and it has **at
> least two** known members rather than the one recorded earlier. Record what you
> cannot enumerate as a `D4` residual; do not let the grep define the population,
> and **do not inherit either named test from this block — re-measure at your own
> base.**
>
> **BLINDNESS 2 — one site is PRODUCTION code and the three acts have no cell for
> it.** `crates/ken-verify/src/scenario.rs:658` sets
> `SCENARIO_COMPILER_STACK_BYTES` inside `execute_scenario`, reached from `pub fn
> run_scenario`, with **no `#[cfg(test)]` anywhere above it**. The ruling opens
> *"a test may set its own thread stack size"*; this is not a test. **`AC-5`
> forbids changing it and the taxonomy cannot classify it**, so a reader lands in
> a cell that does not exist and the node reads as complete.
>
> ⇒ **Classify it as `D4` residual and report it. Do not force it into an act,
> and do not modify it.** Whether production code may state a stack, and under
> what derivation, is a question for the parent standard and is not this node's
> to settle.

## What this is

**The stated-stack sites, brought into line with the ruling.** The ruling itself
is transcribed by the parent node; this one applies it.

**The parent node originally carried this as a one-line-per-site pass. That
sizing was built on a population of 6 in one crate and does not survive the
census.**

## The population, measured by the Architect at `origin/main` `10101777`

| constant | sites | crates |
|---|---|---|
| 256 MiB | 12 | `ken-cli` 7, `ken-elaborator` 4, `ken-verify` 1 |
| 8 MiB | 2 | `ken-runtime` `src/`, both under `#[cfg(test)]` |
| 1 MiB | 1 | `ken-runtime/tests/value_depth_totality.rs` |

**Anchors, at that SHA, to re-find rather than to check.** `ken-elaborator`:
`cc3_parsing_cursor_decoder_acceptance.rs`, `l3_strings_surface_acceptance.rs`,
`map_build_acceptance.rs`, `r3_c2_source_mixed_branch.rs`. `ken-verify`:
`px8f_write_partition.rs`. `ken-cli`'s seventh:
`dasm_c2_observation_artifact_identity.rs`. `ken-runtime` `src/`:
`lowering/core/tests/control.rs`, `planning/static_transition.rs`.

## THE THING THAT MAKES THIS `M` AND NOT A SWEEP

**The ruling requires, for act 2, "a measured peak and the headroom applied,
BOTH as numbers." For the twelve 256 MiB sites, that measurement does not
exist.** The parent node's own evidence is that the constant is *"the signature
of a value copied from a neighbour rather than derived from a measured depth"* —
which is precisely the claim that there is no peak behind it.

⇒ **You cannot discharge act 2 at those sites by writing a sentence.** Either
the peak gets measured, or the site is classified as something other than act 2,
or the honest answer is recorded as a residual.

**That is why `D1` is classification and not documentation.**

## Deliverables

**`D1` — classify every one of the 15 sites into act 1, 2 or 3.** Per site:
which act, and the evidence. **Act 1 is the forbidden one and its test is
objective** — is there an open measured regression on that test? A site in act 1
is a finding and is reported, not documented.

**`D2` — for every act-3 site, write the property being controlled** and why a
*fixed* size makes the control deterministic rather than machine-dependent.
`value_depth_totality.rs` is the in-tree worked example and it is already
compliant — **confirm that and leave it alone.**

**`D3` — for every act-2 site, the measured peak and the headroom, both as
numbers.** **If the measurement turns out to be a footprint investigation rather
than a read, STOP and report** — the parent node named that outcome in advance
and it is a legitimate one, not a failure. A fabricated peak is worse than a
recorded residual.

**`D4` — the residual, stated.** Any site you could not classify or could not
derive, named with what you ran. **Do not pick the convenient reading to empty
this section.**

## Acceptance criteria

**`AC-1` — the census is re-derived at your base and the count reported.** The
parent node's original count was wrong by a factor of two and a half, and one
`ken-cli` site post-dated its measurement. **Do not inherit the table above.**

**`AC-2` — every site lands in exactly one act, with its evidence.** A site
recorded as "act 2 or 3" is not classified.

**`AC-3` — no constant is changed.** This node records derivations; it does not
retune stacks. **If a site's derivation shows the constant is wrong, that is a
finding and a separate node** — say so and leave the number.

**`AC-4` — `crates/ken-cli/tests/px4b_native_production.rs` is NOT modified.**
It has an open measured regression, which is act 1. It becomes eligible only
when that regression is closed on its own terms, in its own node with its own
Decision.

**`AC-5` — no change to code that compiles into a NON-TEST build.** Stated by
**profile, not by path** — two of the fifteen sites are under
`crates/ken-runtime/src/` inside `#[cfg(test)]`, so a path-shaped exclusion
would forbid the only way to reach them.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run.

## Sizing

**`M`.** Fifteen sites, four crates, and `D3` is a measurement rather than a
sentence. **The one-hour target applies to `D1` alone** — if classification runs
long, hand back the classification and stop; it is the deliverable the rest
depends on.

## Contention

**Four crates: `ken-cli`, `ken-elaborator`, `ken-verify`, `ken-runtime`** —
test files only. This reaches into crates two other rings own, so **re-derive
the intersection against every WP in flight at candidate time**, not against
this list.

## Not this node

- **Not the ruling.** That is [[TEST-NATIVE-STACK-PROVISIONING-STANDARD]], and
  it must land first.
- **Not the `98e6ac51` repair**, and not `px4b_native_production.rs`. See
  `AC-4`.
- **Not retuning any constant.** See `AC-3`.
- **Not a footprint reduction** in the elaborator or anywhere else. If `D3`
  turns into one, that is the report.
