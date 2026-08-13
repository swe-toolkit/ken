---
id: SEC1-R3-MINIMAL-ROUTE
title: "SEC1-IFC-R3 was escalated as needing an SMT backend, and that named the one component the program has already deferred by policy while leaving the binding one unidentified -- prover.rs:317 lists FOUR deferred components and nobody has measured which is minimally sufficient for AC-R3c; also re-derive the recorded claim that widening decidable equality is vacuous, which is true of one registry and false of a second that already holds Char"
status: merged
owner: verify
size: S
gate: none
depends_on: []
blocks: [V3-KRIPKE-DECOMPOSITION]
github: 2124
origin: Steward re-derivation 2026-08-13 of SEC1-IFC-R3's 2026-07-27 escalation, prompted by the operator asking what capability the absent solver integration costs. The escalation named Z3; measurement showed Z3 is one of four components at prover.rs:317 and is the one 03-program-of-work.md:182 already defers until the catalog can measure throughput. Operator directed this lane 2026-08-13.
---

## What this is

**Two measurements and no build.** `SEC1-IFC-R3` has sat `draft` since
2026-07-27 behind an escalation that named the wrong component.

## Measurement one — which of the four

`crates/ken-elaborator/src/prover.rs:317-318` defers **four** things in one
comment:

> kernel whnf + decision procedure (`23 §3.1`) + Z3-backed arithmetic search +
> `Decidable` constructor extraction (`23 §3.2`)

**`SEC1-IFC-R3`'s `AC-R3c` needs a deliberately too-weak `Φ_post` to be
DETECTED, and detection is a refutation** — a kernel-accepted `q : φ → Bottom`
for a `product(c, ζ)` faithfulness obligation. **Which of the four is minimally
sufficient for that has never been measured**, and the answer decides whether
Sec1 sits behind an L-sized spine item or something much smaller.

**The solver is not the answer by default.** `03-program-of-work.md:182` defers
it deliberately — optional, off-by-default, sequenced after the catalog is large
enough to measure throughput — and that ruling stands. **If the answer is the
solver, say so and the ruling gets revisited on its merits. If it is not, Sec1
was never blocked on the deferral.**

## Measurement two — is the widening actually vacuous

`SEC1-IFC-R3` records, as a reason not to frame work: *"generalizing the prover
off `IntLit` has no second registered type to generalize to"*, grounded on
`declare_deceq_certificate` having exactly one caller.

**That caller count is correct** (`crates/ken-elaborator/src/numbers.rs:397`,
registering `Int`). **The conclusion may not be.** There is a **second
registry**: `decimal_char.rs:262-264` does
`numeric_env.set_eq_entry(char_id, EqEntry { op_id: eq_char_id })` — and that
is not the `deceq_certs` map `obs.rs:84` gates on. `eq_float` and `eq_float32`
also exist (`numbers.rs:467`, `:471`).

**And ADR 0013 may make the whole question the operator's rather than a build
one.** It records that `Int` is an opaque primitive with no induction, that the
kernel's `conv`/`whnf` **does not execute `PrimReduction::Op`**, and therefore
that the universal `DecEq Int` laws are **irreducibly trusted** — *"No kernel
move can make the universal `DecEq Int` laws `Axiom`-free."*

⇒ **If a second registrant adds trusted axioms, it is TCB growth**, which is
not a call this node or the Steward makes.

## Not this node

- **Building anything.** No decision procedure, no adapter, no registration.
- **Deciding whether to adopt a solver**, or revisiting the deferral. Report
  what is required; the ruling is the operator's.
- **Discharging any `AC-R3` row.** This says what would discharge `AC-R3c`.
- Re-running the `Verdict::Disproved` census. `SEC1-IFC-R3` already carries it
  and it is not in doubt.
