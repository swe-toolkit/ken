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

> # THE §1a HOLD IS DISCHARGED — BY MEASUREMENT, NOT BY DEVIATION (2026-09-14).
>
> **The hold stood for about ninety minutes and then the ring dissolved the
> question it was protecting.** §1a fired because four unaided passes had failed
> in two opposite directions on *how to instrument this*. The answer is that
> there was nothing wrong with the instrument: **the venue had a zero
> population** (see the section below). At `px8f_buffer_native` the measurement
> is cheap and has already returned **20 discharges, zero double-claims**.
>
> **So the hold is lifted on its own terms.** I did not deviate, the Architect
> did not rule unaided, and no fifth instrument was designed. §1a bought exactly
> what it exists to buy: it stopped a fifth pass, and the fifth pass would have
> been wrong in the same way as the first four.
>
> ### WHAT IS DISCHARGED IS THE INSTRUMENT ASK. THE QUESTION IS NOT.
>
> **Narrowed 2026-09-14 at the Architect's request (`evt_1y9rk2s98qxf3`), and
> the distinction is load-bearing:**
>
>     DISCHARGED   how to observe a recursive analysis without adding to the
>                  frames on its path -- dead, the function was never on the
>                  overflowing path
>     NOT          whether the three satisfier populations are disjoint. Exactly
>                  as unmeasured as it was this morning. This node is `draft`
>                  with no result.
>
> ⇒ **Nothing here reopens sequencing `D2` behind this node.** That path was
> closed on its own grounds and **stays closed** — the discharge STRENGTHENS the
> closure, because sub-question 2 is the weakened remainder and disjointness is
> therefore not getting measured soon. **If anyone reads "the hold is discharged"
> as reopening it, that is the misreading to head off.**
>
> **RESEARCH SUB-QUESTION 1 IS WITHDRAWN — its premise is false.** It asked how
> to observe a recursive compiler analysis *"without adding to the frames on its
> path."* `prove_forwarded_value` was never on the overflowing path. There is no
> frame-budget problem to solve. **Do not leave a falsified question standing in
> a research queue** — sub-question 3 was already ruled (frame authorship), so
> only sub-question 2 survives, and correction 4 weakens its evidence too.
> The standing HS24 advisory is unaffected and stays fired.
>
> **`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` still does NOT depend on this node.**
> `D2a` took its other arm and the ring has now BUILT it (`9ede2e995`). Nothing
> in `D2` rests on the populations being disjoint — the property remains
> **withdrawn**, not assumed. The measurement below is worth having on its own
> merits; it is not on anyone's critical path.
>
> **WHAT IS NOT YET DONE, and it is the half that reads as done:** *"zero
> double-claims"* is a **reading, not a result.** `AC-1`'s perturbation control
> has not been run and `AC-2`'s per-arm statement has not been made. Three maps
> are three pairwise claims; an aggregate cannot say which pair overlaps. **Do
> not record this as measured until both are discharged.**

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

## WHY FOUR PASSES FAILED — THE POPULATION AT THAT VENUE IS ZERO

**REPLACED WHOLESALE 2026-09-14 (Steward). The four-cell stack table that stood
here is FALSIFIED and is not preserved beside this, because a superseded reading
is what the next implementer reads first.** Measured by the ring at
`evt_2q1ggxnpemwyn`; the structural half re-verified by the Steward against the
object DB rather than relayed.

**`prove_forwarded_value` is NEVER CALLED from
`abi_s6_mapping_file_backed_native`.** Its only non-recursive entries are
`units.rs:4366` in **`close_and_define_staged_result_bodies`** (`:4117`) and
`:3370` inside `derive_certified_cuts`, whose sole caller is `:4358` in that
same function — which IS the `required`-driven loop.

**CORRECTED 2026-09-14** (Architect, `evt_1y9rk2s98qxf3`): this first named
`exact_staged_unit`, a 19-line helper at `:4097-4115` containing neither call. A
"last `fn` before line N" scan matched `^pub fn ` and skipped `pub(super) fn`,
landing one function short. Conclusion unchanged.

    suite                                  enter  visit  discharge
    abi_s6_mapping_file_backed_native         0      0        0
    abi_s6_mapping_surface_native            18      0        0
    px8f_buffer_native                        6     22       20

⇒ **Four instrument passes probed a function that venue cannot reach.** Not a
stack problem. Not an isolation problem. **A zero population**, which returns the
same silence a broken instrument does.

**What that falsifies, named so nobody rebuilds on it:**

- **"The path runs with less headroom than the probe costs."** The overflow was
  never on `prove_forwarded_value`'s path — it is not on the path at all. The
  misattribution named the wrong function, so every conclusion keyed to it goes.
- **Cell A ("full suite, uninstrumented, unprovisioned -> ok").** Does not
  reproduce on a byte-identical tree; it overflows in a different test entirely
  (`exact_required_consumer_edge_queries_resource_bracket_ok`). The original cell
  A carried no SHA.
- **Cell D's "0 ARMS lines."** Over-determined — zero with unlimited stack too,
  because the population is zero.
- **THE SCOPE COLLISION THAT SHAPED THIS WHOLE NODE DOES NOT EXIST.**
  `RUST_MIN_STACK=268435456` provisions all 18 tests with **zero source edits** —
  no fixture change, no touch of `:565` (R1) or `:719` (red #2), no verdict
  transfer at risk. **My measurement that all 18 invoke `build_native_program`
  was correct; the conclusion I drew from it was wrong**, because it assumed
  provisioning must be written into the source. It was reachable by a knob nobody
  considered.

**The lesson that generalizes past this node: I priced my own constraint as the
binding one and it was never binding.** I recorded *"price your own constraint
instead of letting it be quoted back as a wall"* on this same arc this morning —
and then the wall turned out to be imaginary in a second, independent way. **A
constraint you authored is not evidence about the world.**

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
