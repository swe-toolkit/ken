---
id: V3-FO-KEN-LEVEL-CHECKER-AUTHORING
title: "Author the Ken-level check_cert, embed, Form and Cert so the conversion cost 23 section 4.4 names becomes measurable at all, and take that measurement"
status: draft
owner: language
size: L
gate: none
depends_on: [V3-FO-CONVERSION-LOAD-MEASURED]
blocks: []
github: null
origin: "Steward scope call evt_6m3q3tsvg09pz, 2026-08-15, on Architect review evt_7cmys9wyp7k8c of V3-FO-CONVERSION-LOAD-MEASURED at b52d160c8. The absence of check_cert from library/ and catalog/ was verified by the Steward against the tree at origin/main 30ee4dbf1 before filing. Steward-filed per COORDINATION section 2."
---

## Why this exists: the predecessor's `AC-2` named a quantity nobody can take

`V3-FO-CONVERSION-LOAD-MEASURED` `AC-2` required *"the measurement is of kernel
conversion."* Its cost model was explicit — kernel conversion must evaluate
`embed Sigma f` and then run `check_cert` over the whole derivation tree, forced
by `refl True` at

```
ok : check_cert (embed Sigma f) pi = True
```

**That measurement is not takeable, because the artifact does not exist.**
Verified at `origin/main` `30ee4dbf1`: **`grep -rn check_cert library/ catalog/`
returns nothing.** The only `check_cert` in the tree is
`ken_elaborator::fo_kripke::check_cert` (`fo_kripke.rs:807`), a **native Rust**
function recursing through `check_tree` (`:812`) over Rust `Cert`/`Sequent`
structs. `embed` is likewise Rust-side.

`conformance/verify/prover/seed-prover.md:49-50` states the distinction
normatively:

> `check_cert` = the **Ken-level reflective Bool checker** over quoted formulas
> (`23 §4` route (a)) — **an ordinary kernel-checked function, distinct from the
> kernel API `check`**.

⇒ **The predecessor measured the Rust reference checker.** Its `AC-2` was
amended to say so and to report the gap as a result; **this node is where the
gap is closed.**

## The Steward's framing error, recorded because the shape keeps recurring

The predecessor's frame argued the number was obtainable now because the
equation *"requires NEITHER theorem"* — true, and about the **metatheory** axis.
**It was read as clearing the way generally.** Nobody checked whether the thing
being measured existed.

> **A warrant that reaches one axis, read as reaching another.** The same shape
> cost lane 1 twice in the same campaign: a limit stated on the **instrumented
> sites** was read as a limit on the **population**.

## What this node must not be read as authorizing

**`23 §4.4` forbids `proved` until `embedding_adequacy` and `checker_soundness`
are kernel-checked in an approved home.** Both are unproved, unstarted, and have
no node. **Authoring the checker does not touch that**, and a working
`check_cert` in Ken is **not** grounds for a `proved` verdict.

**This node builds and measures. It does not discharge.**

## Deliverables

**`D0` — the quoted-syntax types in Ken.** `Form` and `Cert` as Ken inductives
matching `23 §4.3`'s slice subset, with the Rust `IForm`/`Cert` as the
reference for shape only. **State explicitly which Rust constructors have no Ken
counterpart and why**, rather than silently narrowing.

**`D1` — `embed` in Ken**, the quoted-formula-to-proposition map, matching what
the Rust `embed` computes on the same inputs.

**`D2` — `check_cert` in Ken** as an ordinary kernel-checked total function,
per `seed-prover.md:49-50`. **Not a primitive, not an axiom, not a kernel API
call.**

**`D3` — a differential control against the Rust checker.** For every
certificate the predecessor's corpus produced, the Ken `check_cert` and the Rust
`check_cert` must agree. **A disagreement is the most valuable result this node
can produce and is reported as one.**

**`D4` — the measurement `AC-2` originally named.** Wall-clock and, where
obtainable, reduction-step count that **kernel conversion** spends on
`check_cert (embed Sigma f) pi = True` via `refl True`. **Report the
distribution and the worst case, not an average**, and state the build profile.

**`D5` — termination, reported honestly.** Whether conversion terminated on
every case. `docs/design/fo-route-theorem-home.md` §4 names this the load class
that matters, leaning on the argued-not-mechanized half of `18 §6`. **A
non-terminating or pathological case is a result, not something to work
around.**

## Acceptance criteria

**`AC-1`.** `D4`'s measured interval contains **kernel conversion** and nothing
else. **Demonstrate it** — the predecessor's whole `AC-2` failure was a bracket
that contained no conversion at all, and it went unnoticed because the commit
message filed the fact under a different criterion where it read as a virtue.

**`AC-2`.** The Ken `check_cert` is **kernel-checked**, with no new primitive,
no trusted axiom, and no addition to `trusted_base()`.

**`AC-3`.** `D3` agrees with the Rust checker on the predecessor's full corpus,
**or the disagreement is reported rather than reconciled by changing either
side to match.**

**`AC-4`.** No FO `Proved` verdict. `23 §4.4`'s reservation is untouched.

**`AC-5`.** Deep cases run on the oversized test-thread stack helper
(`run_with_big_stack`, present in four `crates/` test files today), **so a
harness stack limit is never reported as a mechanism property.** That confusion
is exactly what the predecessor's `D4` had to be corrected for.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Proving `embedding_adequacy` or `checker_soundness`.** Unfiled successors.
- **Emitting `proved` for FO**, on any basis.
- **Widening the slice** beyond `23 §4.5`.
- **Changing the Rust checker to make `D3` agree.** It is the reference.

## Sequencing

**`draft`, and deliberately behind the two `ready` lane-2 nodes**
([[V3-FO-GUARD-SHIFT-DIFFERENTIAL]], [[V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT]]).

**Nothing currently blocks on this number.** The predecessor's `D1` measured
that **no Ken program in the repository produces an FO-quotable obligation** —
the entire corpus had to be authored. **A cost number for checking such an
obligation gates nothing while the population is zero**, and that is the whole
basis for amending `AC-2` rather than making this a prerequisite.

⇒ **Framing is owed before this goes `ready`**, and the trigger to promote it is
a real source-level population appearing, or the operator asking for the route
(a) cost.
