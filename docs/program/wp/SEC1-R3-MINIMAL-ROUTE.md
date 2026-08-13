# SEC1-R3-MINIMAL-ROUTE — which deferred component actually blocks Sec1, and is the widening really vacuous

**Owner: verify. Size: S. Gate: none — this reports; `SEC1-IFC-R3` keeps `G-Sec`.**
**This node REPORTS. It builds nothing and registers nothing.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`3cfdfdce`.

## Fixed inputs

| fact | site |
|---|---|
| the four deferred components, in one comment | `crates/ken-elaborator/src/prover.rs:317-318` |
| the sole production route to `Disproved` | `:298-300` — `Term::Eq(_, IntLit, IntLit)`, `left != right`, discharged by `λx.x` against the registered `Int` certificate |
| the fallthrough everything else takes | `:320` — `emit_unknown_hole` |
| the row this exists for | `SEC1-IFC-R3`'s `AC-R3c` — a deliberately too-weak `Φ_post` must be **detected** |
| the one deceq registrant | `crates/ken-elaborator/src/numbers.rs:397` — `declare_deceq_certificate(env, int_id, eq_int_id, …)` |
| **the second, different registry** | `crates/ken-elaborator/src/decimal_char.rs:262-264` — `numeric_env.set_eq_entry(char_id, EqEntry { op_id: eq_char_id })` |
| what the prover actually gates on | `obs.rs:84` — `env.deceq_cert(*id).is_some()` |
| other equality primitives of the right shape | `numbers.rs:467` (`eq_float`), `:471` (`eq_float32`) |
| the posture that may make this the operator's call | `docs/adr/0013-int-decidable-equality-kernel-posture.md` |
| the deferral that must not be quietly revisited | `docs/program/03-program-of-work.md:182` |

## D1 — which of the four is minimally sufficient for `AC-R3c`

Take the `product(c, ζ)` reduction-faithfulness obligation as `SEC1-IFC-R3`
describes it — variable renaming, `lowEq_ζ`, the `coterminates_ζ` conjunct — and
determine **what would have to exist for a too-weak `Φ_post` to produce a
kernel-accepted `q : φ → Bottom`.**

Answer against the four named components **individually**: kernel whnf, a
decision procedure (`23 §3.1`), solver-backed arithmetic search, and `Decidable`
constructor extraction (`23 §3.2`). **For each: required, not required, or
required only in combination with a named other.**

**"All four" is a permitted answer and it is a strong claim** — if that is the
answer, say what makes each individually necessary rather than listing them.

## D2 — is the recorded "vacuous widening" claim true

`SEC1-IFC-R3` records that generalizing off `IntLit` has no second registered
type to generalize to. **Re-derive it against three things the claim did not
consider:**

1. **The second registry.** `Char` already has an `EqEntry`. Does anything
   bridge `numeric_env`'s eq entries to `deceq_certs`, and if not, is bridging
   them a registration or a design change?
2. **`eq_float` / `eq_float32`.** These have the right shape. **Float equality is
   the case where trusting an operation to decide propositional equality is
   wrong** — NaN and signed zero. State that as a reason, not as an oversight,
   so the next reader does not "fix" it.
3. **ADR 0013's finding.** The kernel does not execute `PrimReduction::Op`, so
   `eq_int 5 5` is neutral at type-checking time and the universal laws are
   irreducibly trusted. **Does a second registrant therefore add trusted
   axioms?** If yes, this is TCB growth and D3 applies.

## D3 — route the TCB question, do not resolve it

If any answer above requires growing what is trusted, **that is the finding and
it goes to the Steward for the operator.** Do not weigh it, do not recommend
adopting it, and do not scope a node around it.

Equally: **do not treat the solver deferral as revisitable here.** If the answer
is that only a solver suffices, report that; the ruling at
`03-program-of-work.md:182` is the operator's to revisit on that evidence.

## Acceptance criteria

- **AC-1 — D1 answers per component, not as a set.** Four verdicts with
  reasons. **A single "the prover backend is needed" restates the escalation
  this node exists to correct.**
- **AC-2 — every claim about what a component would enable is grounded at a
  `file:line` or a spec section**, not in what the component is generally for.
  The escalation being corrected was itself a plausible generalization.
- **AC-3 — D2 names the disposition of all three considerations**, including
  the float one. **Silence on float reads as an omission a later reader
  repairs.**
- **AC-4 — anything that grows the trusted base is REPORTED AS SUCH and left
  unweighed** (D3). A recommendation to adopt it, however hedged, is out of
  scope and the report must not contain one.
- **AC-5 — the report states what it did not try**, so the next reader does not
  re-run the same search. **Distinguish "the language forbids this" from "I did
  not find a way"** — they license different next steps.
- **AC-6 — nothing is registered, built or changed.** `git diff` under
  `crates/` is empty.

## Pre-stated licensing — read BEFORE reporting

| answer | what it licenses |
|---|---|
| **one component, and it is not the solver** | `SEC1-IFC-R3` gets re-scoped around that component and **the solver deferral is untouched.** Sec1 was never blocked on it. |
| **the solver, minimally and unavoidably** | The deferral goes back to the operator **with this evidence**. It does not become revisitable by anyone else, and no node is framed on it. |
| **the widening is NOT vacuous and does not grow the TCB** | A small registration node becomes framable. **This node does not frame it and does not do it.** |
| **the widening grows the TCB** | Operator decision. `SEC1-IFC-R3`'s "vacuous" recording gets **corrected in place** — it would be right about the outcome for the wrong reason, and a wrong reason survives to justify the next thing. |

> **No answer here discharges any `AC-R3` row**, and none of them says whether
> Sec1's reduction is sound. This says what would have to exist.

## Banned scope

- Registering any certificate or eq entry; bridging the two registries.
- Implementing any of the four components, even minimally, even to test the
  answer. **A probe that would ship is a build.**
- Framing the successor node. That is the Steward's, and it is deliberate:
  three of the four licensing rows do not lead to one.
- Revisiting the solver deferral, or arguing its merits.
- Changing `SEC1-IFC-R3` — report; the Steward edits the node.

## Hard stops — return to the Steward

- **Answering D1 would require building one of the four to find out.**
- **The `product(c, ζ)` obligation cannot be stated concretely enough to reason
  about** from `SEC1-IFC-R3` plus the spec. That is a spec gap, not a search
  failure, and it routes differently.

## Sequencing and contention

Verify, one lane, beside or after `V3-VERDICT-CENSUS` — **they do not
contend**: this reads `prover.rs`, `numbers.rs`, `decimal_char.rs`, `obs.rs`
and ADR 0013 and writes nothing under `crates/`.

**Language is concurrently in `ken-elaborator`** on the surface nodes. AC-6's
empty-diff condition is what keeps that safe.
