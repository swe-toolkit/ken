# V3-VERDICT-CENSUS — count what the prover cannot close, and what that costs the trusted base

**Owner: verify. Size: S. Gate: none.**
**This node REPORTS. It builds nothing and improves nothing.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`3cfdfdce`.

## Fixed inputs

| fact | site |
|---|---|
| the entry point and its routing | `crates/ken-elaborator/src/prover.rs:221` (`attempt_obligation`), `:139` (`classify`) |
| the three fragment arms, and that all three converge | `:281` (D), `:332` (FO), `:352` (HO) — FO and HO call `attempt_ipc` unchanged |
| the only search engine | `ipc_search`, `:434` — depth cap 32, four rules: Pi-intro, Sigma-intro, hypothesis lookup, `Proj1`/`Proj2` |
| the only refutation route | `:298-300` — `Term::Eq(_, IntLit, IntLit)` with `left != right` |
| **the trusted-base cost** | `emit_unknown_hole`, `:493` — calls `declare_postulate`, so the goal's id enters `trusted_base()` |
| suites that drive obligations | `crates/ken-elaborator/tests/{v3,v4,sec1,t1}_acceptance.rs`, `crates/ken-cli/tests/t2_acceptance.rs`, and `src/ifc.rs` |

## D1 — the distribution

Over the obligations the existing suites actually produce, count
**`Proved` / `Disproved` / `Unknown`**, and for each `Unknown` record:

1. **the route** `classify` assigned — D, FO or HO;
2. **the syntactic shape of `φ`** that reached the hole, at whatever granularity
   distinguishes cases (top-level `Term` constructor is the floor; a finer
   grouping is better if it falls out);
3. **whether `ipc_search` returned `None`, or returned a candidate the kernel
   then rejected.** Those are different failures — the first is search
   incompleteness, the second is a search that proposes unsound certificates —
   and `try_ipc_cert` (`:385`) already distinguishes them internally.

## D2 — the trusted-base cost, stated as a number

**How many postulates does the corpus put in `trusted_base()` through
`emit_unknown_hole`?** Report it as a count and as a fraction of
`trusted_base()`'s total, so the reader can see whether prover holes are a
rounding error or the bulk of it.

**Report the count separately from the obligation count.** One obligation is
one postulate today, but do not derive one from the other — measure both, and
if they disagree that disagreement is the finding.

## D3 — name the shapes nothing can reach

From the `Unknown` shapes, name the ones **no amount of tuning the current
engine would reach**, and say why structurally rather than empirically. The
frame already knows three; confirm or refute each against your data:

- **goals with free variables in an equality.** `is_ground_decidable` (`:155`)
  requires `!has_free_vars`, and `is_first_order_intuit` (`:175`) excludes
  `Term::Eq` in its `_ => false` arm — so `Eq` with free variables routes to
  **HO**, and `ipc_search` has no `Eq` arm. `∀n. n + 0 = n` is a hole.
- **anything needing case analysis.** The kernel has no primitive disjunction
  (`crates/ken-kernel/src/term.rs:230-320` — no sum); it is a derived inductive,
  and `ipc_search` has **no `Constructor` and no `Elim` arm**. This bites
  *before* quantifiers or arithmetic.
- **quantifier instantiation.** `ipc_search`'s hypothesis lookup matches a
  hypothesis whose type **equals** the goal. It cannot use a hypothesis at a
  different instance.

## Acceptance criteria

- **AC-1 — the census is taken by running the corpus, not by reading the
  prover.** A shape inventory derived from `classify`'s source is a statement
  about what *could* route where. **This node needs what did.**
- **AC-2 — the `Unknown` count and the postulate count are measured
  separately** (see D2), and any disagreement is reported rather than
  reconciled.
- **AC-3 — search-incompleteness and kernel-rejection are distinguished** for
  every `Unknown`, per D1.3. **A single "failed" bucket does not discharge
  this** — the two have different fixes and only one of them is a soundness
  smell.
- **AC-4 — the report names what was NOT covered.** Which suites were run,
  which were not, and whether the corpus exercises any obligation the shipped
  language would generate but the tests do not. **A census over the tests is
  not a census over the language, and the report must not read as one.**
- **AC-5 — nothing in the prover changes.** Establish it: `git diff` touching
  `crates/ken-elaborator/src/prover.rs` is empty. Instrumentation that must
  live in the crate goes behind `#[cfg(test)]` and is reported as part of the
  method.

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **a distribution, whatever it is** | Sizing the fragments against each other, and `V3-KRIPKE-DECOMPOSITION`. **It licenses no claim that any particular fragment is the binding constraint** unless the numbers say so, and it licenses nothing about the solver question. |
| **few or no `Unknown`s** | **A real result, not a failed increment.** It would mean the corpus does not exercise the gap, which relocates the question to what the corpus omits — and AC-4 is what makes that readable. |

> **This node cannot conclude that the prover is inadequate.** It reports what
> the corpus produced. **"The search is weak" is not a finding available here**
> — the search is *known* to be four rules deep; what is unknown is what that
> costs.

> **And it cannot conclude anything about the solver.** No solver is integrated,
> so no measurement here compares one. A sentence of the form "Z3 would have
> closed N of these" is unsupported by anything this node can run.

## Banned scope

- Any change to `prover.rs`, `classify`, `ipc_search`, or the refutation arm.
- Registering a decidable-equality certificate for any type. **That is TCB
  growth under ADR 0013 and it is the operator's call** — see
  `SEC1-R3-MINIMAL-ROUTE`.
- Building the Kripke embedding, an SMT adapter, or a tactic engine.
- Adding obligations to the corpus to make the distribution look different.

## Hard stops — return to the Steward

- **The corpus cannot be driven without changing production code.**
- **`trusted_base()` cannot be attributed to `emit_unknown_hole` versus other
  postulate sources**, so D2's number cannot be isolated.

## Sequencing and contention

Verify, one lane. Reads `crates/ken-elaborator/src/prover.rs` and the
acceptance suites. **Language is concurrently in `ken-elaborator` on the surface
nodes** (`LANG-SURFACE-LITERAL-ESCAPES`, then `LANG-SURFACE-BLOCK-COMMENTS`) —
different files, same crate. **Do not edit anything under
`crates/ken-elaborator/src/` that the surface work could also touch**; AC-5
makes that a measurable condition rather than a request.

Local runs targeted only — `scripts/ken-cargo -p ken-elaborator --test <name>`.
**Never `--workspace`** (operator hard rule, `agent/COORDINATION.md §12`).
