---
id: RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP
title: "The discharge path can neither exclude nor refuse: authority_grounds filters the disagreeing authority out of the evidence set, then :3424 clears grounds and breaks, collapsing 'I could not prove this' into 'I proved there is nothing here'. The arc's own contract forbids exactly that. It may NOT land without a witness that makes the check fire -- until then it is a documented invariant, not a check."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Ruled out of the ABI-S6 D5b candidate by the Architect at evt_2ctrn25j8fe39 (2026-09-15) after runtime-implementer measured the repair inert on px8f (evt_6zww772af95c4). The Architect had ruled it the floor and riding the candidate at evt_2ptnm0dyjx4kw; the warrant was that it repairs the failure there, and that was refuted by measurement. Steward-filed per COORDINATION section 2."
---

> # FILED 2026-09-15 AS `draft`, DELIBERATELY NOT RELEASED.
>
> **This must not be handed to the D5b ring as an addition.** It was just ruled
> OUT of that candidate. Releasing it while the same seats are hunting the px8f
> trap would re-create exactly the coupling the ruling removed.
>
> It also **must not be released without D0 below being achievable.** A node
> whose acceptance requires a witness nobody has found yet is a research task
> wearing a repair's clothes, and it should be sized as one if that is what it
> turns out to be.

## The defect, which survives the withdrawal

Two sites, both true of the code independently of any particular program:

- `authority_grounds` (`units.rs:3377`) filters foreign authorities out with
  `.filter(|(_, a)| a.identity == identity)` **before** the discharge loop runs.
  The disagreement is removed from the evidence set.
- `units.rs:3424` then responds to exhaustion with `grounds.clear(); break;` —
  a **proof-search outcome consumed as an emission decision**, with nothing
  connecting the two.

Together they collapse two different facts: *"I could not prove this"* and
*"I proved there is nothing here."* The absence manufactured by the filter is
read as agreement. The site must distinguish three outcomes, not two:

    identities agree                        -> emit
    path certifiably excluded (a real cut)  -> emit without that path
    NEITHER                                 -> REFUSE

Today the third collapses into the first. **That is a fail-open**, and it is
the defect whichever identity turns out to be authoritative.

The arc's own contract, `lowering/mod.rs:1314-1318`, already forbids it:

> These record the identity the producer actually emitted, including an
> identity different from a generated function's demanded Result. The latter is
> not silently dropped: the finished proof must either exclude its path or
> refuse, and it must never relabel the word.

A disjunction with a prohibition. Three permitted states, one forbidden, and
the site can reach the forbidden one.

## Why it is NOT in the D5b candidate, and must not be folded back in

It rode that candidate on one warrant: that it repairs the failure there. That
was **refuted by measurement**. At the correct population the implementer
measured `foreign_proof_sources=0` in the body that traps — nothing to exclude,
nothing to refuse — and px8f's trap was unmoved by the repair.

The Architect's own prediction, *"expect px8f to go from compiling-and-trapping
to refusing-to-compile"*, was the falsifiable half of that ruling and it died.
What survives is only **"this site is a defect"**, which is a reason for a node
and not a reason to ride someone else's candidate.

**"The principle survives" must not launder into "so ship the check."**

## D0 — THE WITNESS. This is the load-bearing deliverable, not a formality.

**An input on which the check FIRES.** A program whose discharge genuinely
reaches a proof source carrying a foreign identity, so that the refusal is
observed rather than argued for.

- **Search the corpus first.** If a witness exists it should be found, not
  built — a found witness also tells us the condition occurs in practice.
- **If it must be constructed, it is a mutation arm**, and it belongs beside
  `RT-MUTATION-ARM-CONTROL-COVERAGE` rather than standing alone. Say so
  explicitly in the closeout; do not quietly construct one and call it found.
- **Until a witness exists this is a documented invariant, not a check.** The
  arc has spent this week removing unfalsifiable guards. It must not add one.

## D0 DISCHARGES A SECOND DEBT AT THE SAME TIME

The witness **is** the positive control that the measured `foreign_proof_sources
= 0` currently lacks. They are the same artifact, so finding it answers both.

Two independent reasons that zero is not yet load-bearing, and **neither
substitutes for the other**:

1. **Does the selector range over the right set?** The population went 579
   foreign to 0 foreign in one step, and the narrowed selector has never been
   shown capable of reporting non-zero. "No foreign proof source in this body"
   and "my selector cannot see foreign proof sources" are one number. The first
   attempt at this narrowing was wrong by 579, so this is not a hypothetical.
   (Architect, `evt_2ctrn25j8fe39`.)
2. **Is the instrumented seat reached by the trapping dispatch at all?** The
   two DIAG lines report demanded `DenseRange{4362,37}` and
   `DenseRange{3318,37}`; whether either is the discharge for
   `decl:px8f_write_partition::Result` is unestablished. If neither is, the
   zero is a true statement about a population that excludes the failure.
   (Steward, `evt_67jjyszf7g155`.)

Both are the zero-versus-never distinction applied one level further out than
where it was correctly applied already.

## Acceptance criteria

- **AC-1 (the witness, and it gates everything else).** The check is
  demonstrated **firing** on a real input — refusal observed, message shown.
  A run in which it does not fire is not evidence it works.
- **AC-2 (both directions).** On an input where the identities agree, it does
  **not** fire. Show both, from real compiles.
- **AC-3.** The three outcomes are distinguished at the site, and the
  distinction is stated in prose there — a reader who meets `grounds.clear()`
  later must find the reason it is no longer sufficient.
- **AC-4.** `authority_grounds`'s filter no longer deletes the evidence the
  refusal depends on, or the refusal is taken before the filter. Say which.
- **AC-5.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Contention

Touches `units.rs` discharge and `authority_grounds`. **Contends with any
active backend arc in that plane**, which currently includes the D5b repair —
that is the reason for the `draft` status, not a scheduling nicety. Also
contends with `RT-MUTATION-ARM-CONTROL-COVERAGE` if D0 lands as a mutation arm.

## Related

- `RT-MUTATION-ARM-CONTROL-COVERAGE` — the sibling, and D0's likely home if the
  witness has to be constructed. Same underlying shape: an arm or a guard that
  looks like coverage and drives nothing.
- `RT-OBSERVATION-EXIT-STATUS-LAUNDERS-SIGNAL-DEATH` — the same night's other
  instrument defect.
