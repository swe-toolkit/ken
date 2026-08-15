---
id: V3-D-OPEN-GOAL-WITNESS-ROUTE
title: "The z3 round-trip's own stated population -- linear arithmetic over Int with universally-quantified parameters -- appears to reach neither fragment D nor FO, because is_first_order_intuit excludes Term::Eq outright and is_ground_decidable demands both sides be IntLit; the first obstacle to the round-trip is the classifier, not the solver"
status: merged
owner: verify
size: S
gate: none
depends_on: []
blocks: [V3-Z3-PROCESS-ADAPTER]
github: null
origin: "Steward measurement 2026-08-15 at origin/main 65bbc21d5 while framing the operator's directed z3 round-trip lane. spec 23 section 3.2 names the population normatively; prover.rs:139-186 is the classifier that appears not to admit it. Steward-filed per COORDINATION section 2."
---

> # LANE-2 PRIORITY WORK. NOT the Kripke embedding, and NOT blocked by it.
>
> **Operator, 2026-08-15: two lanes and nothing else gets a ring — runtime
> retires `RecursiveDescent`; language and verify do the z3 round-trip and the
> FO Kripke embedding.** This node is the first releasable increment of the z3
> half.
>
> **The two halves have different blockers and that is why they split.**
> [[V3-KRIPKE-DECOMPOSITION]]'s report reached a specification hard stop: `23
> §4`'s exact Kripke theory, its reflective `Form`/`Cert` data, and its theorem
> statements are not closed, so no prover increment for the FO route is
> sizeable. **That verdict is about `23 §4`. It does not carry to `23 §3.2`,
> which is normatively closed** — the solver searches, the kernel re-derives
> validity, and no new theorem is owed. The spec closure the FO half needs is
> [[V3-KRIPKE-THEORY-CLOSURE]].
>
> ⇒ **Do not read "the embedding is unsizeable" as "the round-trip is
> blocked."** They are different fragments with different contracts.

## The reading that produced this node, and it is a READING

`spec/20-verification/23-prover.md §3.2` states the population verbatim:

> *"For decidable goals with free variables (e.g. linear arithmetic over `Int`
> with universally-quantified parameters), Z3 searches; on success the result is
> turned into a kernel certificate by reflection (instantiating a verified
> arithmetic decision procedure) or by reconstructing the proof (SMTCoq-style)
> and re-checking. The solver finds the witness/cut; the kernel re-derives
> validity."*

Read against the classifier at `crates/ken-elaborator/src/prover.rs:139-186`,
that population appears to reach no route that can serve it:

| gate | source | what it appears to do to `∀ x : Int. f x = c` |
|---|---|---|
| `is_ground_decidable` (`:155`) | `!has_free_vars` **and** (`is_const_atom` or `is_literal_equality`) | rejects — the goal has a bound variable, and `is_literal_equality` (`:163`) additionally requires **both** sides to be `Term::IntLit` |
| `is_first_order_intuit` (`:175`) | `Pi`/`Sigma`/`App`/`Omega`/`Var`/`Const` recurse; **everything else returns `false`** | rejects — the body is a `Term::Eq`, and `:184` sends `Eq` to HO by an explicit comment |
| `Route::HO` (`:147`) | the always-applicable default | accepts, and `attempt_ho` (`:352`) calls `attempt_ipc`, which has no arithmetic |

> ## THIS IS READ FROM SOURCE AND IT IS NOT MEASURED. `D1` IS THE MEASUREMENT.
>
> **`D1` exists because a claim about which branch a mechanism takes is a
> prediction about a run.** The Architect made exactly this error twice in one
> session on 2026-08-15 — asserting a control's discriminating power from the
> fixture's shape, refuted by QA running the mutation — and the standing lesson
> is that **a mechanism picture built from reading needs a probe, not more
> reading.**
>
> **If `D1` refutes the table, that is a complete and valuable outcome of this
> node** and the rest of it is re-cut. Report what the routes actually are.

## Deliverables

**`D1` — measure where the `23 §3.2` population actually lands. This gates
everything below it.**

Construct at least two goals of the stated shape (an equality and an inequality
over `Int` under a `Pi`), and for each report: what `classify` returns, what
verdict `attempt_obligation` produces, and whether `trusted_base()` gains the
hole. **Report the routes as measured, not as expected.**

**If the goals route to `D` or `FO` after all, stop and report.** The premise of
this node is then false and the cut is mine to redo.

**`D2` — the routing repair, if `D1` confirms the gap.** Whatever admits the
`23 §3.2` population to a route that can serve it. **`23 §2.1`'s
exhaustive-by-construction property is not negotiable**: routing stays total,
HO stays the always-applicable default arm, and no arm gains a `_ => skip`.

**`D3` — the witness route, solver-agnostic.** The sound shape is already in the
tree and this deliverable generalizes it rather than inventing it:
`attempt_d`'s Int-literal arm (`prover.rs:298-315`) selects a refutation
candidate by an untrusted comparison and then hands it to
`attempt_with_refutation` (`:254`), where **the kernel** checks `q : φ → Bottom`
before `Disproved` is returned. A failed check yields `Unknown` (`:265`).

⇒ **A candidate assignment from any search — including a bounded in-tree
enumerator — substituted into the goal so that the existing ground path
produces the kernel-checked refutation, adds nothing to the trusted base.** The
search proposes; the kernel decides. Build the seam so a solver can be attached
to it without the soundness argument changing.

**The actual z3 process is [[V3-Z3-PROCESS-ADAPTER]], not this node.** Do not
add a solver dependency here. If `D3` cannot be built without one, that is a
hard stop and it goes to the Architect.

## Acceptance criteria

**`AC-1` — the cardinal rule is untouched** (`23 §1.5`). `proved` is returned
only through `check(env, Γ, cert, φ)`; `Disproved` only through
`attempt_with_refutation`'s kernel check. **Control: the existing verdict-flip
tests stay green, and `trusted_base()` deltas for `proved` are unchanged.**

**`AC-2` — a bad candidate yields `Unknown`, demonstrated.** Feed the new route
an assignment that does not refute the goal and show the verdict is `Unknown`,
not `Disproved`. `v3_acceptance.rs:416-421` already pins this for the existing
seam; **this AC is that the NEW path inherits it, run, not argued.**

**`AC-3` — zero trusted-base growth.** No new postulate, no new registrant, no
kernel enlargement. **If the route you find needs a new decidable-equality
registrant, STOP** — that costs two irreducible trusted-base postulates per
registrant (`check.rs:1253`, `:1302`, `:1308`), which is an operator TCB
decision and is the exact fork [[V3-KRIPKE-DECOMPOSITION]] left open.

**`AC-4` — routing stays total.** `23 §2.1`. Demonstrated by the existing
classifier tests staying green plus the shapes `D1` measured.

**`AC-5` — no solver dependency.** `Cargo.toml` files byte-identical to the
candidate base.

**`AC-6` — no-regression, in CI** (`COORDINATION §12`). The V1/V2/V3/V4/T1 and
Sec1 suites are the affected population; targeted locally, workspace in CI.

## Banned scope

- **The FO/Kripke route.** `attempt_fo` (`prover.rs:332`) and its four
  placeholders stay exactly as they are. That route is spec-blocked and its
  successor is [[V3-KRIPKE-THEORY-CLOSURE]].
- **Any solver dependency or process invocation.** [[V3-Z3-PROCESS-ADAPTER]].
- **New trusted-base entries.** `AC-3`.
- **The diagnostics `KripkeCountermodel` evaluator** (`diagnostics.rs:61-75`,
  `:143-188`). It is advisory, it does not affect a verdict, and the
  decomposition report already established it is not a certificate language.

## Stop condition — return to the Architect

**If `D2`'s routing repair cannot be written without changing what `classify`
means** — a new `Route` variant, a non-syntactic classification, or anything
that makes routing depend on a search result — that is a design call and his,
not the ring's. Report the shape you need and why the syntactic gates cannot
express it.

## Why this earns a slot

**It is the first thing standing between the tree and the operator's directed
round-trip, and it is not the solver.** A z3 adapter attached today would be
handed goals the classifier has already sent to HO.

**Its cheapest outcome is a measurement that refutes the premise**, which costs
one run and re-cuts the lane correctly. Its expected outcome is a route and a
seam that a solver plugs into without touching the soundness argument at all.
