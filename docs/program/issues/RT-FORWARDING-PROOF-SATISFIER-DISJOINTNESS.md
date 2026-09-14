---
id: RT-FORWARDING-PROOF-SATISFIER-DISJOINTNESS
title: "Measure whether the three satisfier populations of the prove_forwarded_value guard are disjoint -- the measurement RT-CONSTRUCTOR-AUTHORITY-DISCHARGE D2a withdrew, held behind the fired COORDINATION section 1a research advisory because four unaided instrument passes failed on it"
status: draft
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Split out of RT-CONSTRUCTOR-AUTHORITY-DISCHARGE D2a by the Steward at evt_752h5xfry09dh, when D2a was re-derived to its second arm. Filed rather than carried, because a carry with no home is what evaporates. The four hard stops and the full four-cell measurement table are the Architect's evt_2x1yk1cnmw6dg and the implementer's evt_6tk9xskk67jza; durable at target/D2-ASSIGNMENT.md on the ring's base."
---

> # `draft` BECAUSE IT IS HELD, NOT BECAUSE THE FRAME IS OWED.
>
> **The `COORDINATION §1a` research advisory fired on this question and is
> unconsumed.** Research is quota-dead to roughly **2026-09-19**. The Steward
> ruled HOLD rather than deviate (`evt_752h5xfry09dh`): four unaided passes
> failed in two opposite directions, one of them the Architect's own, and this
> is the point §1a exists to defend.
>
> **`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` does NOT depend on this node and must
> not be sequenced behind it.** `D2a` took its other arm. Nothing in `D2` rests
> on the populations being disjoint — the property is **withdrawn**, not assumed.
>
> **Do not attempt a fifth instrument before the advisory returns.** That is the
> whole content of the hold.

## What this is

**The measurement `D2a` used to prescribe, kept alive on its own terms.** It was
taken off `D2`'s critical path because a successful result would have closed one
instance at one site rather than the class — not because the question is
uninteresting.

## The question

At `units.rs:3129` (measured at `686ffa8ac`), `prove_forwarded_value` satisfies
one proof by **three** independent arms:

    if authorities.get(&value).is_some_and(|a| a.identity == identity && a.word == value)
        || detached_consumer_authorities.get(&value).is_some_and(|a| {
               a.demanded_identity == identity && a.after_word == value })
        || call_seeds.get(&value) == Some(&identity)

**Are the three populations disjoint?** If they are not, a value whose authority
has been discharged can still be proved by a sibling map.

## WHY FOUR PASSES FAILED, SO THE FIFTH DOES NOT REPEAT THEM

The full four-cell table, measured by the ring:

    A  full suite, uninstrumented, unprovisioned  ->  ok (18 tests)
    B  --exact,    uninstrumented, unprovisioned  ->  STACK OVERFLOW
    C  --exact,    uninstrumented, PROVISIONED    ->  no overflow; lowering refusal
    D  full suite, WITH PROBE,     unprovisioned  ->  OVERFLOW, 0 ARMS lines

**B and D are each sufficient alone.** The path runs with less headroom than
either the probe or the isolation configuration costs.

**The untried cell is provisioned + full suite + probe, and it is not reachable
cheaply.** All 18 tests in `abi_s6_mapping_file_backed_native.rs` invoke
`build_native_program` (Steward, measured by enumerating all 18 direct call sites
plus the `differential()` helper — a `differential()`-keyed grep gives 6 and is
wrong). So provisioning for a probe run means provisioning all 18, which touches
`:565` (carries R1) and `:719` (carries red #2), both QA-approved at exact SHAs.

**That scope constraint is the Steward's and is a price, not a law.** It protects
verdict transfer. Name it back to the Steward if it binds; do not route around it.

## What the advisory was asked

Posed by the Architect at `evt_2x1yk1cnmw6dg`, narrowed to compiler and
code-generator ground and sharing the corpus of the standing HS24 ask, so one
pull answers both:

1. Is there a known technique for observing a recursive compiler analysis's
   internal decisions **without adding to the frames on its path** — out-of-band
   recording, a side channel, reconstruction from artifacts the compile already
   emits, or a pre-pass over the inputs rather than the traversal?
2. When a test's outcome depends on suite membership, what is the established way
   to get a trustworthy per-test measurement?
3. *(Answered already, and not research's.)* Whether "not measurable under this
   scope, proceed on a stated assumption" is acceptable. **The Steward ruled NO**
   — `D2a`'s *"it will not be accepted as an assertion"* binds. The resolution was
   to withdraw the property, not to assume it.

**Prior art may CORROBORATE a conclusion here and must never be this frame's
cited rationale.** Implementers build from `/spec`; a frame naming a reference as
its justification converts an authorized enclave read into an unauthorized
import, and the seat that pays has no licence to check the source.

## Deliverables

**`D1` — an instrument that observes the guard's arm selection without consuming
the headroom the measured path runs with.** Design informed by the advisory.
**Not a fifth unaided pass.**

**`D2` — the three-population disjointness result, or a recorded refusal.** If
the populations are not disjoint, that is a finding about
`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`'s closure and is reported there.

## Acceptance criteria

**`AC-1` — the instrument is shown not to perturb the outcome it measures.** The
control is the A/D pair above: the instrumented run must reproduce cell A's
result on the uninstrumented path, or the measurement is of a different compile.
**A green instrumented run is not evidence of this on its own.**

**`AC-2` — the result is stated per arm, not as an aggregate.** "The populations
are disjoint" over three maps is three pairwise claims. An aggregate cannot
distinguish which pair overlaps.

**`AC-3` — the census is re-derived at your base.** The guard had two arms at
`187895991` and three at `686ffa8ac`. **Read the guard at your own base; do not
count occurrences at a ref someone else named.** That exact error is what this
node was split out of.

**`AC-4` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run.

## Not this node

- **Not `D2`'s consumption change.** That is
  [[RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]] `D2a`, which proceeds without this.
- **Not the stack provisioning** of `abi_s6_mapping_file_backed_native.rs`. See
  [[TEST-STATED-STACK-SITE-RECONCILE]].
- **Not the isolation-only lowering refusal** observed in cell C. See
  [[RT-COMPILE-OUTCOME-RUN-CONFIGURATION-DEPENDENCE]].
