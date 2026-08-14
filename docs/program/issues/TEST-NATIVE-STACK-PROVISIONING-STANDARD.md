---
id: TEST-NATIVE-STACK-PROVISIONING-STANDARD
title: "Five landed `ken-cli` native-production tests each provision `stack_size(256 MiB)` while `LANG-RECORD-STACK-OVERFLOW` says stack-limit raises are refused -- but that refusal is a `Not this node` bullet whose rationale is about MASKING A REGRESSION, and baseline provisioning for a legitimately deep workload is a different act that no artifact has ever distinguished"
status: ready
owner: language
size: S
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect routed the scope question to the Steward at evt_1cx980f527awy (full ruling evt_8d0phqzr76vq), arising from the 98e6ac51 diagnosis, explicitly so it would not be settled as a side effect of unblocking a candidate. Steward re-measured every load-bearing fact against ca803dfc before filing."
---

## What this is

**A ratification, not a repair.** Nothing here unblocks any candidate, and the
node is filed precisely so that no candidate's urgency decides it.

## The two facts, measured at `ca803dfc`

**1. Five `ken-cli` tests provision a large stack.** Each spawns its
`build_native_program` work on a thread with **exactly**
`stack_size(256 * 1024 * 1024)`:

```
crates/ken-cli/tests/px8f_buffer_native.rs:207
crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:306, :341
crates/ken-cli/tests/rt_escape_second_resource_native.rs:684
crates/ken-cli/tests/rt_parity_native.rs:455
crates/ken-cli/tests/rt_span_prov_native.rs:301
```

**2. `px4b_native_production.rs` calls `build_native_program` eleven times and
provisions nothing.** It runs on the default test-thread stack. It is the file
that aborted.

## The apparent conflict, and why it is not one

`LANG-RECORD-STACK-OVERFLOW` (merged) says, at `:76-78`:

> **Raising a stack limit.** `RUST_MIN_STACK` and friends are refused. The
> gate-4a arc next door repaired a stack regression by reducing footprint, and
> that is the standard here.

**Read as a fleet standard, the five landed files are debt.** That reading is
what makes this look like a contradiction to resolve.

**It is not a fleet standard. It is a `## Not this node` bullet.** That section's
function is bounding one WP's scope, and every other bullet in it is
unmistakably node-local (`Tuning NESTED_MATCH_DEPTH`, `Building a new synthetic
depth fixture`, `view retirement`). **"That is the standard here" is one clause
inside a scope exclusion**, and "here" is doing unmarked work: it reads equally
as "in this node" and "in this codebase."

**The rationale is the part that actually decides it**, and it is stated: *the
gate-4a arc repaired a stack regression by reducing footprint.* That rationale
governs **repairing a regression**. It says nothing about **provisioning a
baseline for a workload that is legitimately deep**.

⇒ **Two different acts, never distinguished by any artifact:**

| act | what the rationale says |
|---|---|
| A measured regression appears; raise the limit so it passes | **Forbidden.** This is masking, and it is exactly what gate-4a refused. |
| A test drives a deep native compilation and provisions for it up front | **Not addressed.** No artifact has ever ruled on this. |

**`RUST_MIN_STACK` and a per-test `stack_size` are also not the same
mechanism**, and the bullet's "and friends" elides it. `RUST_MIN_STACK` is an
environment variable that silently raises every thread in the run, hiding
footprint problems fleet-wide and leaving no trace at the site. A
`Builder::new().stack_size(...)` is local, visible in the test that needs it,
and reviewable. A ban whose rationale is "do not hide a regression" does not
reach a mechanism that hides nothing.

## What the Steward has already decided, and it needs no ratification

**The `98e6ac51` repair may not be a stack raise.** The A/B attributed the abort
to the candidate, so it is a **measured regression**, which lands squarely in
the forbidden row above. That conclusion does not depend on how "here" is read,
and the Architect's refusal to let anyone "just give `px4b` 256 MB" is correct
on the rationale alone. **This node does not revisit it.**

## The question to ratify

**May a `ken-cli` test that drives `build_native_program` provision its own
thread stack as a baseline, independent of any regression?**

Three dispositions, and the node must pick one:

1. **Yes, and the five are correct.** Then the standard is amended to say so
   explicitly, and `px4b_native_production.rs` provisioning is a normal
   follow-up rather than a masking move.
2. **No, and the five are debt.** Then they are filed as such with a footprint
   target, and the honest consequence is stated: `ken-cli` native-production
   tests currently pass only because five files violate the standard.
3. **Yes but bounded** -- provisioning is permitted only where a named,
   recorded reason exists, and the size is not a copied constant.

**Disposition 3 is the Steward's recommendation and the reviewer should attack
it.** The tell that the current state is unconsidered is that all six sites
carry the **identical** `256 * 1024 * 1024`, which is the signature of a value
copied from a neighbour rather than derived from a measured depth. A standard
that permits provisioning without requiring a reason reproduces that.

## Deliverables

**`D1` — the ruling, recorded where it binds.** Amend
`LANG-RECORD-STACK-OVERFLOW:76-78` so the sentence states which act it forbids,
and put the standing standard somewhere a future candidate will actually read
it -- a `Not this node` bullet in a merged node is not that place. Name the
chosen venue.

**`D2` — the five sites reconciled to the ruling.** Under disposition 1 or 3,
each site gains the one-line reason the ruling requires. Under 2, each is filed
as debt. **Do not change the constant in this node**; it is a separate
measurement.

**`D3` — `px4b_native_production.rs`'s status stated.** Whether it should
provision, and if so that this is a follow-up node and explicitly **not** the
`98e6ac51` repair.

## Acceptance criteria

**`AC-1` — the ruling distinguishes the two acts in its own sentence.** A
reader who arrives with a red test must be able to tell, without reading this
node, whether their situation is the forbidden one. **The current text fails
exactly this**, which is why an Architect read it as a fleet standard and a
Steward read it as a scope bullet.

**`AC-2` — the mechanism distinction is stated.** `RUST_MIN_STACK` versus a
per-test `stack_size`: same ruling or different, said explicitly rather than
left to "and friends."

**`AC-3` — no candidate is unblocked by this node.** `crates/ken-cli/tests/px4b_native_production.rs`
is **not** modified here. If the ruling permits provisioning, that edit is a
separate node with its own Decision. **This is the criterion the whole node
exists to protect.**

**`AC-4` — the count is verified, not inherited.** Re-run the census at the
candidate base and report the site count and the constant at each. This node
asserts six sites in five files at `ca803dfc`; if that has moved, the ruling is
written against the new number.

**`AC-5` — no production change.** `crates/*/src/` is untouched.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run.

## Sizing

**`S`.** The measurement is done and is in this node; the work is a ruling and
a reconciliation pass. **The one-hour target applies to `D1`.** If `D2` turns
into a footprint investigation, that is a different node and the right move is
to say so and stop.

## Not this node

- **Not the `98e6ac51` repair**, and not any repair. See `AC-3`.
- **Not tuning the constant.** Whether 256 MiB is right is a measured question
  nobody has asked; this node ratifies whether provisioning is permitted, not
  how much.
- **Not a footprint reduction in the elaborator.** If the ruling lands on
  disposition 2, the footprint work is the next node.
- Not [[LANG-RECORD-STACK-OVERFLOW]]'s design pass on ACs 1-8, which stands.

## Why `gate: architect`

The Architect handed the question to the Steward rather than resolving it, and
the Steward is deciding **venue and cut** (§3) rather than the technical
standard. Ratifying which act is forbidden is a testing-practice call the
Architect reviews on the merge Decision. **It is not routed to the operator:
it grows no TCB, forks no roadmap scope, and changes no inter-team topology.**
