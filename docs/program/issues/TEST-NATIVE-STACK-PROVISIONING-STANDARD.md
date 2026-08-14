---
id: TEST-NATIVE-STACK-PROVISIONING-STANDARD
title: "Record the stated-stack standard where a candidate author will read it -- the governing property is that a test's stack is STATED, not that it is large, and the tree already derives both halves including the RUST_MIN_STACK / stack_size split"
status: merged
owner: doc
size: S
gate: none
depends_on: []
blocks: [TEST-STATED-STACK-SITE-RECONCILE]
github: null
origin: "Architect routed the scope question to the Steward at evt_1cx980f527awy (full ruling evt_8d0phqzr76vq), arising from the 98e6ac51 diagnosis, explicitly so it would not be settled as a side effect of unblocking a candidate. RULED evt_4rz7hp11f33wj (2026-08-14) at origin/main 10101777: disposition 3, re-grounded on statedness, with the Architect withdrawing his own Amendment 2. Steward re-cut the node to that ruling."
---

> # RULED 2026-08-14 (`evt_4rz7hp11f33wj`). THE QUESTION IS ANSWERED; THIS NODE
> # NOW TRANSCRIBES IT.
>
> **`gate: architect` is discharged and the field is now `none`.** The gate
> existed to get this ruling; it landed. The Architect reviews the merge
> Decision like any candidate — *"nothing further is owed by me until a
> candidate reaches the merge Decision."* **This is not a node with an open
> gate; do not read the history that way.**
>
> **This node no longer asks a question. Do not re-open the dispositions**, do
> not re-derive the ruling, and do not improve its wording. `D1` is a
> transcription.
>
> **The site-reconciliation pass moved out**, to
> [[TEST-STATED-STACK-SITE-RECONCILE]]. The reason is in Sizing.

## What this is

**A ratification, not a repair.** Nothing here unblocks any candidate, and the
node was filed precisely so that no candidate's urgency decides it. That
property is now enforced by the ruling itself rather than by this node's
caution — see act 1.

## THE CENSUS THAT REFUTED THIS NODE'S OWN EVIDENCE

**This node asserted six sites in five files, all `ken-cli`. The real population
is 15 sites in 14 files across FOUR crates, carrying THREE different
constants.** Measured by the Architect at `origin/main` `10101777`.

| constant | sites | where |
|---|---|---|
| 256 MiB | 12 | `ken-cli` 7, `ken-elaborator` 4, `ken-verify` 1 |
| 8 MiB | 2 | `ken-runtime` `src/`, both under `#[cfg(test)]` |
| 1 MiB | 1 | `ken-runtime/tests/value_depth_totality.rs`, a **named constant** |

**Anchors at `10101777`, to re-find rather than to check.** The `ken-elaborator`
four are in `cc3_parsing_cursor_decoder_acceptance.rs`,
`l3_strings_surface_acceptance.rs`, `map_build_acceptance.rs` and
`r3_c2_source_mixed_branch.rs`; `ken-verify`'s is in `px8f_write_partition.rs`;
the `ken-cli` seventh is `dasm_c2_observation_artifact_identity.rs`, **new since
`ca803dfc`** and therefore absent from this node's original list. The two 8 MiB
sites are in `lowering/core/tests/control.rs` and
`planning/static_transition.rs`. The 1 MiB site declares
`const STACK_BYTES: usize = 1 << 20`.

⇒ **The decisive evidence in the original node does not survive.** *"All six
carry the identical `256 * 1024 * 1024`, which is the signature of a value
copied from a neighbour"* is true of the twelve and **false of the
population**. It remains a good reason those twelve need reconciling; **it
cannot carry the disposition**, and it no longer does.

## WHY DISPOSITION 2 IS REFUTED, NOT MERELY DISFAVOURED

Under *"the sites are debt"*, `value_depth_totality.rs` is debt. **It is the
best-documented stack site in the repo**: the number is a named constant, it
records why the size is *fixed on purpose*, it reports the thresholds it was
measured against at **two** stack sizes, and it says in its own words that the
point is a **stated** stack rather than the ambient `ulimit -s` (8192 KiB on the
measuring box, *"not guaranteed equal in CI"*).

**A standard that files that as debt is wrong about itself.** Recorded here so
the disposition is not re-proposed as the "rigorous" option.

## THE ARCHITECT WITHDREW HIS OWN AMENDMENT 2, AND THE REASON IS THE DELIVERABLE

His earlier Amendment 2 read: *"derived means: the measured peak for the deepest
program in that file, times a stated headroom factor, with BOTH numbers written
down."* **It is withdrawn.**

**It is wrong for act 3, and `value_depth_totality.rs` is the proof.** That
file's claim is that host-stack usage must **not** grow with value depth — so
there is no measured peak to derive from, and the size is deliberately **below**
the ambient default. Applied literally, the amendment marks the repo's best
stack site non-compliant and invites someone to "fix" it by raising 1 MiB to a
measured peak, **destroying the control.**

**The general shape, which is the part worth carrying:** the arithmetic had one
sign. A rule that can only justify a *larger* number silently assumes every site
provisions upward. **Enumerate the acts, not the sites.**

**Amendment 1 survives intact and is the load-bearing one** — gate on evidence,
not motive. It is what makes the standard structurally incapable of unblocking a
candidate.

## Deliverables

**`D1` — transcribe the ruling into its venue, VERBATIM.** The block below is
the ruling text. **Do not re-derive it, do not compress it, and do not improve
its wording.** Its structure is load-bearing: the three acts, the "stated"
requirements, and the mechanism split each discharge a specific AC.

> ## Stated stacks
>
> A test may set its own thread stack size. **The governing property is that the
> stack is STATED — not that it is large, and not that it is small.**
>
> A test whose outcome depends on the ambient stack (`ulimit -s`, the harness
> default, `RUST_MIN_STACK`) is asserting a property of the machine it ran on.
> Stating the stack at the site makes the result a property of the code under
> test.
>
> **Three acts. Only the first is forbidden.**
>
> 1. **Masking a regression — FORBIDDEN.** A test that a base-versus-candidate
>    A/B shows **newly failing** may not be repaired by changing its stack. This
>    holds **regardless of the reason recorded**: the change functions as the
>    repair whatever the author intended. The condition is objective and
>    reviewer-checkable — **no open measured regression on that test** — so this
>    standard can never be used to unblock a candidate.
> 2. **Provisioning a baseline — PERMITTED, stated.** A test driving a
>    legitimately deep workload may state a stack adequate for it.
> 3. **Pinning as an instrument — REQUIRED to be stated.** A test whose claim is
>    about depth or stack behaviour must state its stack, **often BELOW the
>    ambient default**, because the stated bound is the claim.
>
> **"Stated" requires, at every site:**
> - **The number** — at the site or in a named constant.
> - **What the number is derived from**, written down: for (2) a measured peak
>   and the headroom applied, **both as numbers**; for (3) the property being
>   controlled and why a *fixed* size makes the control deterministic instead of
>   machine-dependent.
> - **The ambient environment neutralized** wherever the claim is about depth or
>   stack: a spawned child must `env_remove("RUST_MIN_STACK")`, so the stated
>   stack is the operative one.
>
> **`RUST_MIN_STACK` and `Builder::stack_size` get DIFFERENT rulings, and the
> difference derives from statedness.** `RUST_MIN_STACK` is ambient, invisible
> at the site, and fleet-wide: **forbidden as a repair, and neutralized where it
> could leak in.** `stack_size` is local and reviewable: **permitted under the
> statedness requirement above.**
>
> **The reference implementation is in-tree.**
> `crates/ken-runtime/tests/value_depth_totality.rs` states its stack in a named
> constant, records why the size is fixed, reports the thresholds it was
> measured against at two sizes, and says explicitly that the point is a stated
> stack rather than the ambient `ulimit -s`. **A site that reads like that has
> discharged this standard.**

**`D1a` — THE VENUE, decided by the Steward under §3:
`agent/playbooks/tools/stated-stacks.md`, skill-linked like its siblings.**

**Why there and not somewhere more official.** The defect this node exists to
fix is that the rule lived in a `## Not this node` bullet and was read at its
widest. The fix is a place with standing authority that the population who write
these tests **load every session**. `agent/playbooks/tools/` is exactly that,
and **`pin-a-property` is the precedent in shape**: one normative file, surfaced
as a skill through `.claude/skills/` and `.agents/skills/`, cited by **one line
each** from `agent/playbooks/build/implementer.md` and
`agent/playbooks/build/qa.md`.

**One home, two pointers, no second copy.** Duplicating the text into both
playbooks would reproduce the exact two-occurrence defect `D1b` exists to
repair. **If you find yourself pasting the ruling twice, stop — that is the
defect.**

`COORDINATION §12` was considered and rejected: it owns **resource discipline on
the shared laptop** (targeted builds, CI as the venue). A test stating 256 MiB
is not about the laptop's CPU or RAM, and filing it there is a category error
that would bury it.

**`D1b` — repair BOTH `LANG-RECORD-STACK-OVERFLOW` occurrences to point at the
venue.** `docs/program/issues/LANG-RECORD-STACK-OVERFLOW.md` **and**
`docs/program/wp/LANG-RECORD-STACK-OVERFLOW.md`. Each currently states the
refusal in a form that reads as a fleet-wide ban on stack raises. Each must say
which **act** it forbids (act 1) and point at the venue for the rest.
**Amending only one leaves the wider-reading copy in the tree, which is the
whole defect.**

**`D2` — `px4b_native_production.rs`'s status, stated and NOT acted on.** Under
this ruling it **may** be provisioned — **but not now and not by this node**,
because it currently has an **open measured regression**, which is act 1. It
becomes eligible only once that regression is closed on its own terms, and that
is a separate node with its own Decision. **Write that down; do not touch the
file.**

## Acceptance criteria

**`AC-1` — a reader with a red test can classify their own situation from the
venue text alone, without this node.** The objective condition is *"is there an
open measured regression on this test?"* **The original text failed exactly
this**, which is why an Architect read it as a fleet standard and a Steward read
it as a scope bullet.

**`AC-2` — both mechanisms are named, with the reason, not "and friends."**
`RUST_MIN_STACK` and `Builder::stack_size` get different rulings and the text
says why the difference derives from statedness. **The tree already demonstrates
it:** `RT_SCALE_A`/`RT_SCALE_B` pin their workers to 8 MiB, name the thread
`rt-scale-a-planner-8-mib`, put `stack_bytes=8388608` in the refusal text, bound
the child with `prlimit`, and call `.env_remove("RUST_MIN_STACK")`. Those are the
**only two `RUST_MIN_STACK` occurrences in the tree and both are strippings** —
verify that still holds at your base and say so.

**`AC-2b` — the no-open-regression gate is stated as a CHECKABLE CONDITION, not
as guidance**, and "derived" carries its arithmetic for act 2 **and** its
property for act 3. **A text that says "with a recorded reason" and stops has
not discharged this.** Equally, **a text that demands a measured peak for act 3
has reintroduced the withdrawn amendment** and fails this AC in the other
direction.

**`AC-3` — no candidate is unblocked by this node.**
`crates/ken-cli/tests/px4b_native_production.rs` is **not** modified.
**This is the criterion the whole node exists to protect**, and it is now
additionally protected by act 1 being an objective condition.

**`AC-4` — the census is re-verified at your base, not inherited.** Report the
site count, the file count, the crates, and the constant at each. **This node's
own original count was wrong by a factor of two and a half** — 6 sites in 5
files in 1 crate, against a real 15 in 14 across 4. If the population has moved
again, the venue text is written against the new number and the discrepancy is
reported, not smoothed.

**`AC-5` — no change to code that compiles into a NON-TEST build.** Stated by
**profile, not by path**. The earlier form said `crates/*/src/` is untouched,
and **two of the fifteen sites live under `crates/ken-runtime/src/` inside
`#[cfg(test)]`** — test code by profile, `src/` by path. The path form would
forbid the only way to reach them.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run.

## Sizing

**`S`, and it stays `S` because the site pass moved out.**

**The reconciliation of the 15 sites is [[TEST-STATED-STACK-SITE-RECONCILE]],
not this node.** Two grounds, both measured rather than aesthetic:

1. **Population.** The pass was sized when it was 6 sites in 1 crate. It is 15
   sites in 14 files across 4 crates, and each needs a *derivation written
   down*, which is per-site thought rather than a sweep.
2. **Contention.** This node's deliverables are `agent/` and `docs/program/`
   only — the doc track's lane, which runs **concurrently** with build work by
   standing operator exception. The site pass touches `crates/*/tests/` in four
   crates and contends with whatever build ring holds them.

**Splitting is what lets the standard land now instead of behind a four-crate
pass.** The successor is filed `ready` with the census in it, so nothing is lost
to the split.

## Not this node

- **Not the `98e6ac51` repair**, and not any repair. See `AC-3`.
- **Not the 15-site reconciliation.** See Sizing and
  [[TEST-STATED-STACK-SITE-RECONCILE]].
- **Not tuning any constant.** Whether 256 MiB is right is a measured question
  nobody has asked. This node records what the standard is, not how much stack
  any test should have.
- **Not a footprint reduction in the elaborator.**
- Not [[LANG-RECORD-STACK-OVERFLOW]]'s design pass on ACs 1-8, which stands.
