---
id: V3-KRIPKE-THEORY-CLOSURE
title: "Spec 23 section 4 labels its own domain and monotonicity axioms (oracle / standard) and never fixes the reflective Form/Cert language, so the adequacy and checker-soundness theorems have no statements -- the decomposition report's hard stop is a spec gap, and no prover increment exists until it closes"
status: merged
owner: spec-enclave
size: M
gate: none
depends_on: [V3-KRIPKE-DECOMPOSITION]
blocks: [CONF-PROVER-SEED-KRIPKE-DRIFT]
github: https://github.com/swe-toolkit/ken/pull/2323
origin: "Steward, 2026-08-15. V3-KRIPKE-DECOMPOSITION merged its report at docs/program/v3-kripke-decomposition.md with blocks: [] and no successor filed; its D3 verdict names the missing inputs and assigns them to Spec. This node is that successor. Steward-filed per COORDINATION section 2."
---

> # MERGED 2026-08-15 — PR #2323, squash `5f860a003`. THE SPEC GAP IS CLOSED.
>
> **Exact `aa9b2454348ed3d28d5f026edfdafd8093ec47b0`, three commits from
> `d9202e464`, sole path `spec/20-verification/23-prover.md`, `+435/-123`,
> `crates/` byte-identical.** Blob identity MATCH. Decision `dec_7ydvwwwc8a2jz`
> read `resolved` from the object; Spec/Fidelity `evt_5ewhzwadxkmhe`, Architect
> `evt_2wrkjqj5cztxq`.
>
> **The declared base was 39 commits stale, and I re-derived that rather than
> inheriting it: none of the 39 touches `23-prover.md`, and `merge-tree` against
> current `main` produced zero conflict markers.**
>
> **The write-then-attack direction worked, and the evidence is two rejected
> cuts.** The kickoff said to write the strongest statement believed provable
> over a minimal fragment and attack it, rather than surveying what the theorems
> could say. The conformance validator returned **request changes twice**, and
> both blockers were real defects in stated theorems rather than presentation:
>
> 1. **Adequacy was false for a carrier the quotation contract accepts** — with
>    `C(A) = Empty` and `f = exists x : A. Top` the emitted target was derivable
>    from its own `domain-inhabited-A` premise while the Ken conclusion had no
>    inhabitant. Repaired by permitting empty object domains, **not** by
>    inventing witness metadata.
> 2. **The structural rules were directionally under-specified**, admitting a
>    reading that accepts `[] => [P]` from a `P => P` child.
>
> A third, on the second cut, was `D5` claiming an emitted `exists` that repair 1
> had made unreachable. **A survey would have produced none of these**, because
> each is a defect in a specific committed statement.
>
> ## The two strengthenings, recorded because a successor must not undo them
>
> **`K(Σ)` moved inside `embed`.** The frame axioms were an assumed
> `(oracle/standard)` ledger row and are now premises discharged inside the
> certificate. That retires an assumption **by construction**.
>
> **`classically_valid` is proof-theoretic** — derivability in the fixed calculus
> — which is why `checker_soundness` is purely syntactic and no completeness
> result is needed on the discharge path.
>
> ## What is deliberately NOT settled, and what it left behind
>
> Artifact home, evaluator posture, and the trusted-base consequence are
> surfaced and open for the Architect and the operator. **Until both theorems
> are kernel-checked in an approved home, FO cannot return `proved`.**
>
> ⇒ **This merge makes a conformance seed contradict its normative chapter**,
> including a row asserting a settled `trusted_base()` outcome that §4.4
> explicitly reserves. That, plus the one-clause `forall-left`/`exists-right`
> witness-direction fix, is [[CONF-PROVER-SEED-KRIPKE-DRIFT]] — filed rather
> than absorbed here, and not a reason to recut this SHA.

> # THE PREDECESSOR PRICED THIS AND GOT "UNSIZEABLE". THIS NODE IS WHY.
>
> [[V3-KRIPKE-DECOMPOSITION]]'s `D3` reads: *"There is no honest one-hour
> prover-side first increment on the current inputs... its size is not merely
> 'more than one hour'; it is presently **unsizeable**. Guessing an hour count
> would convert missing contracts and a feasibility risk into an effort
> estimate."*
>
> **That is not a verdict about difficulty. It is a verdict about missing
> normative text**, and every one of the four missing pieces it names is
> assigned to Spec. **This node closes them, and nothing downstream can be cut
> until it does.**
>
> **The z3 half of the lane is NOT waiting on this.** `23 §3.2` is normatively
> closed and [[V3-D-OPEN-GOAL-WITNESS-ROUTE]] is released against it. Do not
> couple them.

## The gap, quoted from the spec itself

`spec/20-verification/23-prover.md §4` gives the translation clauses for `φ ↦
φ#` and then says, in its own parenthesis:

> *"(Exact domain/monotonicity axioms **(oracle / standard)** — the Kripke-sheaf
> semantics of the topos.)"*

⇒ **The frame conditions the adequacy theorem quantifies over are marked
assumed, in the normative document, at the exact point where they would have to
be stated.** Route (a) is `OQ-12`-DECIDED and the architecture is settled; what
is missing is the theory it ranges over.

The decomposition report's table names four pieces. **Three of the four are
blocked on this one**, because the adequacy statement changes when any frame
axiom or translation clause changes.

## Deliverables

**`D1` — close the `World` theory.** The world sort, the accessibility preorder,
the domain function and its growth condition, and the monotone forcing
predicate, stated normatively rather than as `(oracle / standard)`. Say which
atom theories are supported and which are refused.

**`D2` — fix the supported source fragment.** Which Ken `Term` shapes translate.
`23 §4`'s clause table covers the connectives and quantifiers; it does not say
what happens to the shapes Ken has and the clause table does not mention.
**A total translation with an explicit refusal boundary is the deliverable; a
partial one with an implicit boundary is the defect.**

**`D3` — specify the reflective data.** The quoted `Form` and `Cert` inductives
and the certificate-rule semantics `check_cert : Form → Cert → Bool` ranges
over. `23 §4` names `check_cert` and distinguishes it from the kernel API
`check`; it does not define either datatype. **Nothing in production has these
types** — the report verified that, and it also verified that the diagnostics
`FormRef`/`KripkeCountermodel` are advisory and are **not** a head start.

**`D4` — state the two theorems.** `classically_valid(φ#) → φ`, and the
soundness of `check_cert` supporting the discharge `sound φ π (refl true) : φ`.
**Statements, not proofs.** The predecessor established both are kernel-facing
and TCB-adjacent, and that where they live is an Architect and operator
disposition — surface that question with what you have closed, do not answer it.

**`D5` — the minimal fragment.** `23 §4`'s own feasibility argument requires an
end-to-end vertical slice. **Name the smallest fragment of `D1`-`D4` for which
that slice is coherent**, so the Architect and the operator are choosing over a
priced object rather than the whole theory.

## Acceptance criteria

**`AC-1`.** No `(oracle / standard)` or equivalently-assumed marker survives in
the region `D1` closes. **If one must survive, it is named explicitly as an
open decision with its consequence** — `spec/90-open-decisions.md` is where that
goes, not a parenthesis in normative text.

**`AC-2`.** Each theorem statement in `D4` is well-formed against the data `D3`
fixes. **A statement that mentions a type `D3` did not define fails this.**

**`AC-3`.** The translation is total over the fragment `D2` declares, with an
explicit refusal for everything outside it. Route totality (`23 §2.1`) is the
existing precedent and the shape to match.

**`AC-4` — no trusted-base claim is settled here.** Whether either theorem is
proved, admitted, or relocated is **not** this node's to decide. Recording
"assumed" as a spec status would convert the hard stop into a licence.

**`AC-5`.** `crates/` byte-identical to the candidate base. This node writes
spec, not code.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Building any of it.** Not the translation, not the `World` sort, not the
  theorem proofs, not the checker. The predecessor banned this and the ban
  stands.
- **Deciding whether V3 proceeds.** Priority, and the operator's.
- **Ruling where the adequacy lemma lives.** Architect's. `D4` surfaces it.
- **Route (b).** `23 §4` retains reconstruction as a hedge; the report
  established it needs a proof-evidence dialect and rule mapping that are also
  absent, and it is **not** a smaller version of route (a). Out of scope here.
- **The D fragment and the solver.** [[V3-D-OPEN-GOAL-WITNESS-ROUTE]] and
  [[V3-Z3-PROCESS-ADAPTER]].

## Why this earns a slot

**The report cost a full node to establish that the next step is spec work, and
then nothing was filed to do it.** A decomposition whose conclusion is
unscheduled is worth what an unfiled finding is worth.

**Its output is what makes the operator's fork decidable.** The predecessor
priced the D fragment (two irreducible trusted-base postulates per registrant,
`check.rs:1253`/`:1302`/`:1308`) and left the Kripke side unpriced beyond
"unsizeable". `D5` is the first thing that turns the second row into a number.
