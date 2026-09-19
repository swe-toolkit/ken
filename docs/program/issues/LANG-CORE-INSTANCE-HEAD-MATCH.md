---
id: LANG-CORE-INSTANCE-HEAD-MATCH
title: "core-side instance-head matching for the identity-keyed resolver: peel the application spine in elab_standard_operator to extract a parameterized carrier's head identity (P1), and match the registry's surface pattern against the CORE carrier term inside resolve_instance_dictionary_inner for the requested:None case (P2) -- the precursor LANG-MEMBERSHIP-OPERATOR-SURFACE hard-stopped on, cut to P1+P2 with the rest of the identity-keyed-registry closure named and deferred"
status: ready
owner: language
size: M
gate: none
tier: T1
depends_on: [LANG-STANDARD-INFIX-CALL-COMPLETION]
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
origin: "Cut by Architect ruling evt_1bgpexfk5e79q (2026-09-19) on the LANG-MEMBERSHIP-OPERATOR-SURFACE hard stop evt_4ehtrakx2ftdb, which the Steward verified at the object before escalating. A1's landed comment names this widening 'the identity-keyed-registry closure and a different node'; the Architect ruled the closure's other members (coherence, orphan check, module re-export, derive) are that closure's HORIZON and not this precursor's deliverable, because framing them in makes an XL that blocks B indefinitely. Steward-framed per COORDINATION §2."
---

# Core-side instance-head matching

**The identity-keyed resolver refuses every parameterized carrier, so
`LANG-MEMBERSHIP-OPERATOR-SURFACE` cannot resolve any of its own mandated
providers.** Measured at `origin/main` by language-implementer
(`evt_4ehtrakx2ftdb`) and independently confirmed by the Steward and the
Architect at the same objects.

    elab.rs:10778   elab_standard_operator matches the carrier head on
                    Term::Const and Term::IndFormer only; App(List, Nat)
                    takes the `_` arm and is refused BEFORE lookup
    elab.rs:9932    resolve_instance_dictionary_inner refuses every
                    head_param_count > 0 instance when `requested` is None,
                    and resolve_instance_dictionary_by_head_id passes None

**This is a documented V1 boundary, not a defect.** A1's own comment says
widening it "means matching instance heads against CORE terms, which is the
identity-keyed-registry closure and a different node." This is that node, cut
to its two smallest members.

## Why the carrier cannot simply be made non-parameterized

Architect `evt_1bgpexfk5e79q`, and it is forced by the class shape rather than
observed in a telescope. `Membership` carries `Query` as a **field of the
dictionary**, and a dictionary is per-instance, so `Query` is fixed at
instance-registration time. A single non-parameterized `ListView` would fix one
query type for every element type; varying it means varying the instance, and
varying the instance means parameterizing its head. **The packing that would
avoid this has to live in the container VALUE, and `Query` is not reachable
from the value.**

⇒ `head_param_count > 0` for all three `Membership` views is **forced**.
Recorded here because `B`'s `§3b` never names a carrier head at all, so "all
three are parameterized" reads as an inference from field lists when it is a
consequence of `§3a`.

## Why threading the surface type instead is unavailable

Not merely inelegant. At an expression site the carrier comes from
**inference** — `B`'s `§3d` infers the RHS carrier first — so in general there
is no surface type to thread. Threading one where it happens to exist makes
completion depend on whether the program is annotated, which is a surface-form
dependence in the one path `39 §6.9` requires to key on identity. Synthesizing
a surface type by rendering a core term backwards is running `globals` in
reverse, which this design has refused twice on non-injectivity.

# Frame

## Settled inputs — measured. Do not re-derive.

**1. The surface-against-surface matcher already exists.**
`match_instance_head(pattern, requested, head_param_count, &mut matched)` at
`elab.rs:9947`. What is missing is its core-side twin, not a new mechanism.

**2. The head-id scan is the right mechanism.** `B`'s `§3d` already specifies
resolution *"for its head"*. P1 and P2 complete that path; they do not replace
it.

**3. `P1` alone is NOT sufficient.** Peeling the spine gets past the `_` arm
and lands on the `requested: None` refusal. Both halves or neither.

## Deliverables

**`P1` — extract the head identity from a parameterized carrier.** Peel the
application spine in `elab_standard_operator` so `App(List, Nat)` yields
`List`'s identity instead of taking the `_` arm.

**`P2` — a core-side instance-head matcher.** Match the registry's surface
pattern against the **core** carrier term and produce the instantiation, for
the `requested: None` case.

## Two constraints bound by the Architect

**`C1` — match on `GlobalId` IDENTITY, never on spelling.** The matcher must
resolve the pattern's type constructors to `GlobalId`s. A matcher comparing
head **names** re-imports, one layer deeper and behind a core-looking
interface, exactly the non-injectivity `InstanceHeadSpellingsShareAnIdentity`
was added to detect. **The registry is name-keyed; the matcher must not be.**

**`C2` — the carrier confirmation SURVIVES and WIDENS.** Today
`resolve_instance_dictionary_by_head_id` confirms the resolved dictionary's
kernel-inferred type carries the expected carrier by comparing one head id.
With instantiation it must compare the **full instantiated carrier**.

> **This is the constraint most likely to be dropped, and the frame says so
> because once matching happens in core the confirmation FEELS redundant.** It
> is not. **The matcher's answer is a hint in exactly the way the spelling was,
> and the confirmation is what demotes it from a decision.** A1's header states
> the property and names its own refuter: the by-head-id path is an adapter and
> not a second dispatcher *"specifically because of the carrier-identity
> confirmation"*, and what would refute that is *"any dictionary SELECTION
> reading `class_env.instances` outside `_inner`."*
>
> ⇒ **`P2` LANDS INSIDE `resolve_instance_dictionary_inner`. NOT BESIDE IT.**

## Acceptance criteria

**`AC-1` — a parameterized carrier resolves end to end.** *Control:* the
reproducer that produced this node goes green —

    import Core.Operators.Standard (≤)
    fn probe (xs : List Nat) (ys : List Nat) : Bool = xs ≤ ys

— **and** it still fails with the two halves separated: with `P2` reverted,
`P1` alone must reach the `requested: None` refusal rather than succeed. A
positive check alone cannot tell a working matcher from a weakened
confirmation.

**`AC-2` — `C1` holds structurally, not by inspection.** *Control:* the
matcher's comparison is over `GlobalId`, and a fixture in which two distinct
instance heads share a spelling resolves to the **right** one, or refuses.
**A test that only passes on distinct spellings does not discriminate** — this
is the case `InstanceHeadSpellingsShareAnIdentity` exists for.

**`AC-3` — `C2` holds and the confirmation can still fail.** *Control:*
the confirmation compares the full instantiated carrier; **and** a mutation
that makes the matcher return a dictionary for the wrong instantiation is
CAUGHT by the confirmation, not by the matcher. If that mutation passes, the
confirmation has been demoted to a formality and the candidate has not met this
frame.

**`AC-4` — no second dispatcher.** *Control:* no dictionary **selection**
reads `class_env.instances` outside `resolve_instance_dictionary_inner`. This
is A1's own stated refuter, carried forward verbatim in substance.

**`AC-5` — the diff does not grow into the closure.** *Control:* the diff
touches no coherence check, orphan check, module re-export path, or `derive`
machinery. Those are the closure's horizon (below) and are out of scope; if
closing `P1`/`P2` appears to require one, that is a hard stop and a report.

## Reached but DEFERRED — the closure's horizon, not this node

Named so they are visible rather than discovered, and explicitly **not**
framed: coherence, the orphan check, module re-export, and `derive`. The
Architect ruled that framing them in makes this an XL that blocks
`LANG-MEMBERSHIP-OPERATOR-SURFACE` indefinitely.

## Stop condition

**If `P1` or `P2` cannot be closed without touching a deferred member above,
stop and report.** Do not widen into the closure, and do not add a second
dispatcher to avoid widening.

## Symptom inventory

Seeded at release per the Architect's ruling; the Architect appends one line
per hard-stop and it is never rewritten. **Count of record: hard-stop 1 on this
node, so no `§1a` trigger.**

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
NEXT PREDICATE CHECK = 3rd entry, then 6th, 9th, ...

1. all three mandated Membership views are parameterized, so the identity-
   keyed resolver refuses every one of its own providers before the role is
   reached -- keyed on the carrier's surface pattern, which an inference-site
   caller does not have
```
