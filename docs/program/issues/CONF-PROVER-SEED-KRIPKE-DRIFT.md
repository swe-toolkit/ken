---
id: CONF-PROVER-SEED-KRIPKE-DRIFT
title: "Clear what the Kripke chapter merge left behind: a conformance seed asserting a settled trusted-base outcome the chapter re-opens, two sibling drift sites, and the one-clause witness-direction fix"
status: merged
owner: spec-enclave
size: S
gate: none
depends_on: [V3-KRIPKE-THEORY-CLOSURE]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/2328
origin: "Steward, 2026-08-15. Architect finding 2 on the V3-KRIPKE-THEORY-CLOSURE merge review (evt_2wrkjqj5cztxq), routed to me explicitly for an owner and a node before the seed rows harden. All three seed claims and both sibling sites re-verified against the tree by the Steward before framing; nothing below is taken from the report."
---

## What this is

[[V3-KRIPKE-THEORY-CLOSURE]] rewrites `spec/20-verification/23 §4` and is
strictly better than what it replaces. **It also falsifies three assertions in a
conformance seed that declares itself grounded in that chapter, and the merge is
what creates the contradiction.** The Architect declined to widen the node or
block on it — correctly, since holding the chapter hostage to its companions
would keep a worse chapter on `main`. This is the companion piece.

**`COORDINATION §14(4)` shape:** companion artifacts never assembled onto the
branch that merges.

## The three rows, re-verified by the Steward against the tree

`conformance/verify/prover/seed-prover.md`, row
`verify/prover/kripke-embedding-cert-rechecks-FO` (`:164-181`). The file's own
header (`:8`) says it is *"Grounded in the **landed** `23-prover.md`"*, which is
exactly the claim the merge breaks.

**1. The frame axioms are no longer external.** `:175-176` reads *"(The Kripke
frame axioms are external `(oracle/standard)`, shaping `φ#` — the classical
theory handed to Z3 — only; they are not Ken terms.)"* The merged chapter moves
`K(Σ)` **inside** `embed`, so no frame or forcing premise exists outside it.
Those axioms are now premises discharged inside the certificate, which is the
single largest soundness improvement in the diff — and this row denies it.

**2. The checker-soundness lemma is mis-stated.** `:172` reads
*"the checker-soundness lemma `check_cert (embed φ) π = true → φ`"*. The merged
`checker_soundness` concludes `classically_valid q`, **not** `φ`; adequacy is a
separate lemma. **This is semantic drift, not a spelling error** — the row states
one lemma doing two jobs.

**3. The row pins a trusted-base outcome the chapter re-opens, and this is the
one that matters.** `:173-174` asserts *"**nothing is added to
`trusted_base()`**"* and `:180` restates it as an *"**empty** `trusted_base()`
delta"*. The merged §7 says whether the approved home and the evaluator change
the trusted-base account **remains the Architect/operator placement question**,
and §4.4 explicitly reserves that decision.

> **A conformance seed that locks a settled TCB outcome would pre-decide a
> design question the spec deliberately left open.** That is a conformance
> artifact silently closing an Architect/operator call, and it is why this node
> exists rather than sitting in a backlog.

## The two sibling sites, also verified

- **`spec/SPEC-PROGRESS.md:80`** — the `23-prover.md` row still reads
  `V3 elaborated; implementation-ready` with *"named frame axioms remain
  `(oracle/standard)`"*. **Newly false, and it is on the status backbone**, which
  is the file every reader treats as the authority on where the spec stands.
- **`spec/20-verification/24-diagnostics.md:52` and `:68`** — both cite `23 §4`
  for `φ#`, and `:68` gives `(¬φ)# := ∀ w' ≥ w. ¬ φ#[w']`, a notation the
  rewritten section no longer defines. **Cosmetic by comparison** and explicitly
  the lowest priority of the three.

## The a priori best guess — build this

**Operator ruling, 2026-08-15: state the repair as an attackable claim and
attempt it. Do not open this with a survey of what the rows could say.**

> **The seed row stops asserting outcomes and states the open question instead.**
> Rewrite claim 1 to say the frame theory is discharged inside `embed`; rewrite
> claim 2 to name the two lemmas separately with their actual conclusions; and
> replace claim 3's settled *"nothing is added"* with the **reserved** status —
> the trusted-base account depends on the placement decision, which is the
> Architect's and the operator's and is not made.

**The judgment call inside `D1`, stated so the ring does not have to guess my
intent:** a conformance row whose expectation is *"this question is open"* is
weaker than one that asserts an outcome, and that is the point. **A seed row may
record that a value is not yet determined; it may not determine it.** If the
ring concludes a conformance row genuinely cannot express a reserved outcome and
must either assert or be deleted, **that is a real finding — hand it back rather
than picking one.**

## Deliverables

**`D0` — Architect finding 1, the witness-direction clause. FOLDED IN, not a
separate node.** `§4.3` says *"No rule infers an object-sort inhabitant"* and
constrains a witness to be *"well-sorted in the conclusion's parameter
context."* Read as **drawn from** that context the calculus is free-logic
correct; read as merely **sort-consistent with** it, `forall-left` may
instantiate with a fresh parameter of an object sort and `not (forall x : A.
bottom)` becomes derivable — false when `Obj(A)` is empty. **State the direction
explicitly for `forall-left` and `exists-right`**, the same treatment the chapter
already gives the four structural rows.

> **Neither theorem is defective and this is not a soundness defect.**
> `classically_valid` is derivability, so no model-theoretic soundness is
> invoked, and every formula `embedding_adequacy` consumes is `Dom_A`-guarded by
> construction. **It is a mechanization hazard**, and a specific one: whoever
> proves adequacy and mis-locates the protection — believing it comes from
> `Obj(A)` being allowed empty rather than from the `Dom_A` guard — will find
> the interpretation fails to typecheck over an empty Ken carrier, and **the
> natural repair at that moment is a domain-inhabitedness assumption**, which is
> exactly what `§4.2` forbids and what would destroy the control this node's
> predecessor treats as headline.

`spec-author` has Ken-only copy-ready wording at `evt_dw3p6k2xxtg8`. The enclave
owns the final phrasing.

> **Why this is folded rather than filed separately.** It is one clause, in one
> file, owned by the same seat, blocked on the same merge, in the same
> post-merge-cleanup class as `D2` and `D3`. A second node for it would lengthen
> the critical path to buy nothing. **It is a named deliverable precisely so it
> cannot be lost** — that was the whole worry, and a `D0` answers it better than
> a second kickoff. The exact candidate `aa9b2454` is **not** to be recut for it.

**`D1` — the seed row.** All three claims corrected in
`conformance/verify/prover/seed-prover.md`. Re-derive them against the **merged**
`23-prover.md` rather than against this frame's quotations.

**`D1b` — the coupled sites in the same file, added 2026-08-15 with the `AC-3`
amendment.** Found by the conformance validator's full-file citation sweep and
re-verified against `origin/main` by the Steward. **These are not a second
finding; they are the rest of the first one.**

| site | what the merge falsified |
|---|---|
| `:95`, `classically-valid-topos-invalid-cert-rejected` | calls the target `φ#`; the merged chapter defines `embed(Sigma, f)` |
| `:184`, `:191`, `bare-unsat-no-cert-is-unknown-not-proved` | cites a *"`23 §4` ledger row"* and *"§4 ledger"*; **merged §4 has no ledger.** The rule it wants is real and is at `§4.4:524-527`: *"A backend `unsat` with no constructible, accepted `Cert` is `unknown`, never `proved`."* Also carries `φ#` at `:185` |
| `:372`, the build-sequencing footer | says `φ#`/`World`/`P#` are **external** to the kernel. Merged §8 instead splits the external meanings of `World`/`Le`/`Dom`/`Force` from the Ken data `IForm`/`Form`/`Cert` |

**Repoint each citation to the text that exists and respell the notation. Do not
restate any row's claim or verdict** — `AC-3`'s provenance test is the boundary,
and a row whose staleness predates this merge is not yours.

**`D2` — the status backbone.** `spec/SPEC-PROGRESS.md:80` no longer says the
named frame axioms remain `(oracle/standard)`.

**`D3` — the diagnostics citations.** `24-diagnostics.md:52,68` cite notation
that exists. Lowest priority; **if `D1` and `D2` land without it, that is a
complete increment** — do not hold them to bundle it.

## Acceptance criteria

**`AC-1`.** No row in `seed-prover.md` asserts a `trusted_base()` delta for the
FO Kripke route in either direction. The reserved placement decision is
recoverable from the row itself, not only from the chapter.

**`AC-2`.** Every `23 §4` citation in the changed files resolves to text that
exists in the merged chapter. **Demonstrate it by resolving each citation, not
by asserting the sweep was done.**

**`AC-3`.** The changed population is bounded by **provenance, not by row
count**. A site in `seed-prover.md` may be changed only where **both** hold: its
text cites `23 §4` or restates a `23 §4` claim, **and the merged rewrite
falsified it**. Each such change is a respelling of the citation or the notation
— the row's `given`, its `expect`, and its verdict carry through unchanged.
Staleness that predates this merge is out of scope and stays.

> **Amended by the Steward, 2026-08-15, on the conformance validator's BLOCK of
> `c4b2001eb` (`evt_2spz43658dn7t`). The original text read "No other seed row is
> changed," and it was wrong.**
>
> **It forbade the only way to discharge the node's own purpose.** The frame's
> finding 1 is that *"the Kripke frame axioms are external `(oracle/standard)`"*
> at `:175-176` is a falsehood the merge created. **Line 372 asserts that same
> falsehood** — `` `φ#`/`World`/`P#` are **external** to the kernel `` — outside
> the row I scoped. So the node was required to correct a claim at one site and
> forbidden from correcting the identical claim two hundred lines later, leaving
> the file self-contradicting and the merge's largest soundness improvement still
> denied on `main`.
>
> **The framing error was scoping by ROW when the defect is scoped by CHAPTER
> REWRITE.** The two coincided in the material I checked and diverged in the
> material I did not: `φ#` also survives at `:95`, and `:184`/`:191` cite a *"§4
> ledger"* that the merged chapter does not contain, with the substantive rule
> they want sitting at `§4.4:524-527`. **A row is not the unit the drift travels
> in.**
>
> **The constraint AC-3 was standing in for is real and is unchanged** — this is
> not a general seed audit, which Banned scope states directly. Row count was a
> proxy for it, and the proxy failed on the first population that crossed a row
> boundary. The provenance test above is the constraint stated as itself.
>
> **The validator's refusal was correct.** Given two ACs that could not both be
> met, it blocked and routed the conflict rather than picking one — *"I will not
> tell the author to violate either."* That is the behaviour the seat is for.

**`AC-4`.** No placement, artifact-home, evaluator-posture, or trusted-base
decision is made here. **Recording that a question is open is the deliverable;
answering it is out of scope and is a hard stop.**

**`AC-4a`.** `D0` states the witness direction so that the unsound reading is
closed: with the clause in place, `not (forall x : A. bottom)` must not be
derivable over an empty `Obj(A)`. **The protection is the `Dom_A` guard, and the
clause must not be phrased in a way that relocates it to the emptiness of the
sort** — that mis-location is the hazard `D0` exists to prevent.

**`AC-5`.** `crates/` untouched.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Recutting `aa9b2454`.** The candidate is resolved and merge-imminent. `D0`
  lands as a follow-up on `main`, never as a recut of the approved SHA.
- **Re-opening the merged chapter.** It is approved and correct. If a seed row
  cannot be made true against it, the finding is about the row.
- **A general conformance-seed audit.** Out of scope; see
  [[CONF-BLOCKER-OWNER-RESOLVABILITY]] for the ownership question that is
  separately queued.

## Provenance

Architect finding 2, `evt_2wrkjqj5cztxq`, on exact
`aa9b2454348ed3d28d5f026edfdafd8093ec47b0`; `dec_7ydvwwwc8a2jz` resolved. The
Steward independently re-read `seed-prover.md:164-181`, `SPEC-PROGRESS.md:80`,
and `24-diagnostics.md:52,68` at `origin/main` `c4e622e93` and confirmed all
three seed claims and both sibling sites verbatim before framing.
