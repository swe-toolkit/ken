---
id: TEST-STATED-STACK-SITE-RECONCILE
title: "Reconcile the 15 stated-stack sites to the ruling -- and the first deliverable is CLASSIFYING each into one of the three acts, because the twelve 256 MiB sites need a measured peak that nobody has ever taken"
status: draft
owner: runtime
size: M
gate: none
depends_on: [TEST-NATIVE-STACK-PROVISIONING-STANDARD]
blocks: []
github: null
origin: "Split out of TEST-NATIVE-STACK-PROVISIONING-STANDARD by the Steward when the Architect's census (evt_4rz7hp11f33wj, measured at origin/main 10101777) refuted that node's population claim -- 15 sites in 14 files across 4 crates carrying 3 constants, against an asserted 6 sites in 5 files in 1 crate. Filed rather than carried, because a carry with no home is what evaporates."
---

> # `draft` BECAUSE ITS DEPENDENCY HAS NOT LANDED — NOT BECAUSE THE FRAME IS
> # OWED. The frame below is complete and shovel-ready.
>
> **The one thing between this and `ready` is
> [[TEST-NATIVE-STACK-PROVISIONING-STANDARD]] merging**, at which point the
> Steward flips it. `check-issue-schema.sh --strict` fails a `ready` node whose
> dependency is still `ready`, and it is right to: there would be nothing in the
> tree to reconcile these sites *to*. Runtime lost a turn to exactly this shape
> on 2026-08-13, pulling a node whose own dependency had not landed and finding
> its premise false at `D1`.
>
> **Do not read `draft` as "unframed" and do not re-frame it.**
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

## What this is

**The 15 stated-stack sites, brought into line with the ruling.** The ruling
itself is transcribed by the parent node; this one applies it.

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
