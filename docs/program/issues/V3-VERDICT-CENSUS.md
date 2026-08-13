---
id: V3-VERDICT-CENSUS
title: "Every obligation the prover cannot close is registered as a postulate in trusted_base(), so weak proof search is not a convenience gap but a trusted-base gap -- and nobody has measured how large it is; census the verdict distribution over the existing obligation corpus, and for each Unknown record the fragment it routed to and the syntactic shape that defeated the search"
status: ready
owner: verify
size: S
gate: none
depends_on: []
blocks: [V3-KRIPKE-DECOMPOSITION]
github: null
origin: Operator question 2026-08-13 -- what capability does Ken lack from the absent solver integration, and which terms become checkable. The Steward's answer established that no term becomes checkable (the solver is oracle-not-authority, spec 23 line 244) and that the real cost is that unproved obligations enter trusted_base() via emit_unknown_hole (prover.rs:493). The size of that cost has never been measured. Operator directed this lane 2026-08-13.
---

## What this is

**A census, not a repair.** It builds no prover capability and fixes no gap.

## Why it is the first thing

`emit_unknown_hole` (`crates/ken-elaborator/src/prover.rs:493`) calls
`declare_postulate`, so an `Unknown` goal's id appears in `trusted_base()`.

⇒ **An obligation the search cannot close is not merely unproved. It is
assumed.** In a system whose thesis is a small auditable TCB, that makes proof
search a trusted-base concern rather than an ergonomics one.

**Nobody has measured how many postulates that is, or what shapes cause them.**
Every downstream decision — which fragment to invest in, whether the Kripke
embedding is the binding constraint, what an SMT adapter would buy — turns on a
distribution nobody has counted.

## What makes this cheap

The prover is small and the whole of it is reachable from one entry point.
`classify` (`prover.rs:139`) routes to three arms, and **all three currently
converge on the same engine**: `attempt_fo` (`:332`) and `attempt_ho` (`:352`)
call `attempt_ipc` unchanged, and `attempt_d` (`:281`) tries it first. So the
census is over one search procedure, not three.

## Not this node

- **Improving the search.** Any of it. A candidate that closes a goal the census
  was supposed to count has destroyed its own measurement.
- **Registering a decidable-equality certificate for any type.** See
  `SEC1-R3-MINIMAL-ROUTE` — that question turns on ADR 0013 and it is not a
  build decision.
- Building the Kripke embedding, an SMT adapter, or any tactic.
- Judging whether the distribution is acceptable. **Report it; the disposition
  is the Steward's and the operator's.**
