---
id: RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP
title: "The discharge path DETECTS a foreign producer identity and then silently discards the proof instead of refusing: the loop over proof.sources clears grounds and breaks at four sites, and an empty grounds falls through to `continue`, collapsing 'I could not prove this' into 'there is nothing here'. The arc's own contract forbids exactly that. It may NOT land without a witness that makes the check fire -- until then it is a documented invariant, not a check."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Ruled out of the ABI-S6 D5b candidate by the Architect at evt_2ctrn25j8fe39 (2026-09-15) after runtime-implementer measured the repair inert on px8f (evt_6zww772af95c4). The Architect had ruled it the floor and riding the candidate at evt_2ptnm0dyjx4kw; the warrant was that it repairs the failure there, and that was refuted by measurement. Steward-filed per COORDINATION section 2. AMENDED 2026-09-15 after the Architect reported two coordinate/AC defects (evt_7wsf1z27e679a) and the Steward's verification of them found a third, which refutes the mechanism the first filing asserted. AMENDED AGAIN the same night: the Architect ruled the filter question the Steward routed back (evt_5ey9ngwwf0dz8) -- the filter is not a fail-open and :3429 is a dead arm -- and runtime-implementer answered the Steward's open question by measuring that the discharge seat covers two bodies while six functions emit the trap (evt_6k9xx4trry0hc), which makes D0 a conjunction."
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

## READ THIS BEFORE ANY COORDINATE BELOW: THE SITES ARE CANDIDATE-ONLY

**Every `units.rs` and `mod.rs` coordinate in this node is at
`b0a7c2945`, the unlanded D5b candidate. NONE of them exist on `main`.**
Measured at `origin/main` = `80e39a64a` (and unchanged at `9f9010dcb`):

| probe | `origin/main` | `b0a7c2945` |
|---|---|---|
| `authority_grounds` in `lowering/units.rs` | **0 hits** | `:3377` |
| `grounds.clear` in `lowering/units.rs` | **0 hits** | 4 hits |
| `lowering/units.rs` length | 8445 lines | 12623 lines |
| the contract doc-comment | **absent** | `lowering/mod.rs:1318` |

**The bare line numbers resolve on `main` to real, unrelated, error-free code**
— `:3377` and `:3424` are both `#[cfg(...)]` attributes on
`px8-ds-test-support`. There is no signal to a reader that they are in the
wrong tree; implausibility of the answer is the only detector, and these
answers are perfectly plausible.

This matters more here than in the case that taught the fleet the lesson,
because **this node is `draft` and will outlive the candidate.** (c) has been
ruled out of D5b, the candidate is respinning, and nothing guarantees these
sites ever reach `main` at these offsets — or at all, in this shape.

⇒ **Lead with the symbol name; the number never travels without its anchor.**
Whoever discharges this re-measures the coordinates at the tip they are working
on. Do not trust a number in this file against a tree that is not
`b0a7c2945`.

## The defect, which survives the withdrawal

True of the code independently of any particular program. All coordinates at
`b0a7c2945`, `crates/ken-runtime/src/cranelift_backend/lowering/units.rs`.

**The filter.** `authority_grounds` (`:3377`) is built from `body.authorities`
keeping only `authority.identity == identity`, so foreign-identity producers
are absent from it.

**The loop.** Over `proof.sources` (`:3422` onward), each source reaches one of
**four** `grounds.clear(); break;` sites — `:3425`, `:3429`, `:3437`, `:3445`.
They are not one site and they do not have one cause:

    :3425   authority.identity != identity || authority.word != *source
            -- the foreign identity, read from the UNFILTERED body.authorities
            -- THIS IS THE NODE'S SUBJECT
    :3429   authority_grounds.get(source) returned None
            -- RULED DEAD (derived, not yet measured; see the filter ruling below)
    :3437   value_def of a proof-call seed is not a Result       -- unexamined
    :3445   source is neither an authority nor a proof-call seed -- unexamined

**Then the drop.** After the loop, `if grounds.is_empty() { continue; }` — the
tag query yields no carrier fact and the next one is tried.

Together these collapse two different facts: *"I could not prove this"* and
*"I proved there is nothing here."* The site must distinguish three outcomes,
not two:

    identities agree                        -> emit
    path certifiably excluded (a real cut)  -> emit without that path
    NEITHER                                 -> REFUSE

**That is a fail-open**, and it is the defect whichever identity turns out to
be authoritative.

## THE MECHANISM THIS NODE FIRST ASSERTED IS REFUTED. The defect is worse.

The original filing said the filter *removes the disagreement from the evidence
set*, and that the site then *responds to exhaustion* by clearing. **Measured at
`b0a7c2945`, that is not what happens, and the correction cuts toward severity
rather than away from it.**

`:3424` tests `authority.identity != identity` against **`body.authorities`,
the unfiltered map.** The foreign identity is therefore *detected, explicitly,
at the site*. The clear-and-break at `:3425` is a deliberate response to having
found the disagreement — not blindness manufactured upstream by the filter.

⇒ **The site knows, and discards anyway.** Detection followed by silent discard
is a strictly worse defect than the blindness first described, and it removes
the most sympathetic reading of the code.

A consequence worth stating plainly: because `:3425` is reached only when the
identity matched is *false*, and `:3429` is reached only after `:3424` found
the identity matching, **the filter's deletion cannot be what drives `:3429`** —
an identity-matching source is in `authority_grounds` by construction. The
filter looks load-bearing from the outside and, on this path, is not.

**What this does NOT establish, and the discharging seat must not inherit it:**
what the compiler does downstream with the missing carrier fact. `continue`
skips the tag query; whether the unit then emits without the fact, or a later
stage refuses, is **unmeasured here**. The fail-open claim above rests on the
contract, not on a traced emission. Trace it before repairing it.

## The contract it violates

`lowering/mod.rs:1318` **at `b0a7c2945`** (absent from `main`), on
`generated_constructor_authorities`:

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

**IT IS A CONJUNCTION OF TWO REQUIREMENTS, AND THE SECOND IS THE ONE THAT WILL
BE MISSED.** The witness must:

1. **carry a foreign producer identity**, and
2. **be a body that REACHES THE PROOF PATH AT ALL.**

A witness satisfying only (1) — the obvious reading — could never reach the
seat, and it would look exactly like a passing AC-1. This is not hypothetical:
measured at `b0a7c2945`, `derive_certified_cuts` runs **twice in the entire
px8f compile** (`funcid55`, `funcid60`), while **six** distinct functions emit
the failing trap (44, 60, 61, 63, 64, 65). Five of the six never reach the seat.
(runtime-implementer `evt_6k9xx4trry0hc`; Architect `evt_5ey9ngwwf0dz8` naming
it a conjunction.)

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

**BOTH REASONS ARE NOW SETTLED, AND THEY SETTLED IN OPPOSITE DIRECTIONS** —
which is precisely why neither could substitute for the other. Reason 2 was
confirmed by measurement: the zero was TRUE, and it was about a population
excluding most of the failure. Recorded as stated because the pair is the
reusable part.

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

   **ANSWERED, AND IT WAS THE SECOND THING.** Measured at `b0a7c2945`
   (runtime-implementer, `evt_6k9xx4trry0hc`): `derive_certified_cuts` runs
   exactly twice — `funcid55` (`authorities=1`, demanded `DenseRange{3318,37}`)
   and `funcid60` (`authorities=593`, demanded `DenseRange{4362,37}`), with zero
   early returns. Of those, `funcid55` emits the trap 0 times and `funcid60`
   emits it 39 times, while **six** functions emit it overall. So the zero was
   a true statement about a two-body population, and five of the six emitters
   never reach the seat.

   The trap could not be mapped to its emitter directly because
   `PlannedTrapIdentity` is a pure dedup index keyed on trap **value**, with no
   source-origin — the same non-discrimination `RT-TRAP-IDENTITY` measured. The
   mapping came from instrumenting the emission seat in `joins.rs` instead.

Both are the zero-versus-never distinction applied one level further out than
where it was correctly applied already.

## Acceptance criteria

- **AC-1 (the witness, and it gates everything else).** The check is
  demonstrated **firing** on a real input — refusal observed, message shown.
  A run in which it does not fire is not evidence it works.
- **AC-2 (agree, does not fire).** On an input where the identities agree, it
  does **not** fire. Show it, from a real compile.
- **AC-3 (THE MIDDLE ARM — a foreign identity that IS legitimately cut).** An
  input carrying a foreign producer identity **whose path is certifiably
  excluded**, on which the check does **not** fire. This is a third input,
  distinct from AC-2's: AC-2's program has no foreign identity at all and
  therefore cannot reach the exclusion path.

  **This AC exists to see the over-refusal direction, which AC-1 and AC-2
  cannot see between them.** An implementation that refuses whenever it meets a
  foreign authority satisfies both of them perfectly and breaks every
  legitimately-cut program. The machinery is real rather than hypothetical:
  `CertifiedInfeasibleEdge` is constructed at `units.rs:3577` and consulted at
  `:3049` and `:3074` (all at `b0a7c2945`), so the arm has code behind it and
  can be wrong.

  **Discharging this by showing the arm is UNREACHABLE is an acceptable and
  valuable result**, not a cop-out — a dead branch in the contract's
  disjunction is worth knowing and simplifies the site to two outcomes. If that
  is the finding, record *why*, with the measurement. What is **not** acceptable
  is the arm living only in AC-4's prose: a mechanism claim in a comment is
  structurally exempt from execution, and an unexercised third arm is the
  unfalsifiable guard this node exists to prevent, one level in.

  (Architect, `evt_7wsf1z27e679a`. Whether such a program is constructible is
  open, and is the same search as D0's — this may cost nothing beyond an AC
  recording that.)
- **AC-4.** The three outcomes are distinguished at the site, and the
  distinction is stated in prose there — a reader who meets `grounds.clear()`
  later must find the reason it is no longer sufficient. **All four
  clear-and-break sites are in scope**; state which of them the repair changes
  and which it deliberately leaves, with the reason.

  Two are already settled and must not be re-litigated: **`:3425` is the
  subject** (detect-then-discard), and **`:3429` is ruled dead** by the census
  above. `:3437` and `:3445` are unexamined — do not assume they pattern with
  either.
- **AC-4a (cheap, and it converts a derivation into a measurement).** Confirm
  at runtime that **`:3429` never fires** across both staged bodies. The
  standing instrumentation already reaches it. If it *does* fire, the census
  derivation is wrong and the ruling above reopens — say so loudly rather than
  quietly repairing it.
- **AC-5.** State what drives the discard on each path actually repaired.
  The first filing named the `authority_grounds` filter; that was measured
  wrong for `:3425` and has since been **ruled not a fail-open at all**. Do not
  re-inherit it in either direction — neither as the cause, nor as a site
  needing repair.
- **AC-6.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## RULED: the filter is NOT worth repairing, and `:3429` is a DEAD ARM

The mechanism correction above changed what a repair is aimed at, and the
Steward routed the remaining question — *is the filter worth repairing on its
own terms?* — to the Architect rather than presuming an answer. **Ruled at
`evt_5ey9ngwwf0dz8`, by census rather than by opinion.**

`authority_grounds` has **exactly two occurrences under `crates/`** at
`b0a7c2945`, both in `lowering/units.rs` (Steward-verified independently,
repo-wide):

    :3377   the definition -- body.authorities.filter(|a| a.identity == identity)
    :3428   the ONLY consumer -- authority_grounds.get(source), else clear+break

The consumer is reached **only after `:3424` has established
`authority.identity == identity`.** Every source arriving at the lookup is
therefore a key of `authority_grounds` by construction, and there is no second
consumer where that guard is missing.

⇒ **The filter cannot manufacture an absence for anyone. It is not a fail-open;
it is redundant with its only consumer's guard.** It does not get a repair.

⇒ **`:3429` is UNREACHABLE — a dead guard**, sitting inside the node whose whole
subject is dead guards. AC-4 asks which of the four clear-and-break arms the
repair changes and which it leaves: this one is recorded as **dead, with the
derivation**, not repaired.

**The bound on that claim, stated because an unreachability claim is exactly
the kind this arc refuses from others:** it is derived from a two-occurrence
census plus the `:3424` guard. **It is not measured at runtime.** The
instrumentation already standing is in the right place to confirm it — if
`:3429` never fires across both staged bodies, that is the measurement, and it
costs nothing that is not already being printed. Until then the arm is
*derived* dead, and the node says so rather than rounding it to *measured*.

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
