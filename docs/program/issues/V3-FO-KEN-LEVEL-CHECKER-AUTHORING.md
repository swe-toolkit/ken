---
id: V3-FO-KEN-LEVEL-CHECKER-AUTHORING
title: "Author the Ken-level check_cert, embed, Form and Cert so the conversion cost 23 section 4.4 names becomes measurable at all, and take that measurement"
status: active
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
**Re-verified at `origin/main` `6e3b58009`, 2026-08-16:**
`grep -rn check_cert library/ catalog/` **still returns zero.** The only
`check_cert` in the tree is `ken_elaborator::fo_kripke::check_cert`, a **native
Rust** function recursing through `check_tree` over Rust `Cert`/`Sequent`
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

## Fixed inputs, measured at `origin/main` `6e3b58009`, 2026-08-16

**The coordinates in the original filing were taken at `30ee4dbf1` and have
moved. These are the current ones. Re-measure before you baseline; do not cite
this table's line numbers from memory once your branch is anchored.**

| artifact | `crates/ken-elaborator/src/fo_kripke.rs` |
|---|---|
| `pub enum IForm` | `:382` — **five constructors**: `Bottom`, `Atom(IVar)`, `Or`, `Imp`, `Forall` |
| `pub struct Sequent` | `:465` — `{ gamma: Vec<Form>, delta: Vec<Form> }` |
| `pub enum Rule` | slice subset, **three variants**: `Init { left, right }`, `ImpRight { right }`, `ForallRight { right, eigen }`. **The general `23 §4.3` `Rule` has ~20.** |
| `pub struct Cert` | `:489` — `{ conclusion: Sequent, rule: Rule, children: Vec<Cert> }` |
| `pub fn embed` | `:851` |
| `pub fn check_cert` | `:861`, recursing through `fn check_tree` `:866` |
| `pub fn find_certificate` | `:999` |
| `pub fn quote_fo` | `:575`; `pub fn discover_and_quote_fo` `:348` |

**`attempt_fo` has moved and changed since the DAG text describes it.** It is
`crates/ken-elaborator/src/prover.rs:550` — **not** `ken-verify`, and **not**
a bare `attempt_ipc` passthrough. It discovers, quotes, searches, checks, and
returns an honest `Unknown` through `emit_unknown_hole_fo_withheld` when a
certificate is genuinely accepted, falling back to `attempt_ipc` only when
discovery or search fails. **The `Unknown`-not-`Proved` fail-safe is
`attempt_fo_with_signature`'s documented contract; `AC-4` protects it.**

## The increment cut — this is an `L` and an `L` cannot be released whole

**Dispatch ONE increment per turn** (`§4b`, the one-hour turn). Each is a
releasable candidate on its own:

| increment | deliverables | why it stands alone |
|---|---|---|
| **1** | `D0` + `D1` | types plus `embed`; testable against the Rust `embed` on quoted inputs before any checker exists |
| **2** | `D2` | `check_cert` itself, the largest single piece |
| **3** | `D3` | the differential control, which needs 1 and 2 and nothing else |
| **4** | `D4` + `D5` | the measurement and its termination report, which need a working checker |

**A hard stop inside any increment is a good outcome and is reported as one.**
Do not carry an unfinished increment into the next turn to make it look whole.

> ### `Form` IS STRICTLY LARGER THAN `embed`'s IMAGE, AND
> ### `check_cert` IS TOTAL OVER IT
>
> **Read `fo_kripke.rs`'s own doc comment above `IVar` before authoring `D2`.**
> `Form`/`QTerm` are **untyped**, and `check_tree` performs **no sort
> validation**: a hand-built ill-sorted target — a world eigenparameter in an
> object slot — **closes and returns `true`**, because `Init` needs only
> syntactic `Form` equality.
>
> **The safety mechanism is at the CALLER.** `quote_iform` admits only an
> in-scope object `Var` of the declared sort, so every `IForm` it produces
> carries object-sort indices only; the malformed formulas live entirely in
> `Form`'s excess and **no `IForm` maps to them.**
>
> ⇒ **Two consequences, both load-bearing.** A faithful Ken `check_cert`
> **inherits this property**, and that is correct — it must match the
> reference, not improve on it. And **`D3` agreeing does not certify
> sort-safety**; it certifies agreement. Whether the checker should validate
> sorts itself is [[CORE-FO-CHECK-TREE-SORT-VALIDATION]]'s question and is
> banned here.

## Deliverables

**`D0` — the quoted-syntax types in Ken.** `Form` and `Cert` as Ken inductives
matching `23 §4.3`'s slice subset, with the Rust `IForm`/`Cert` as the
reference for shape only. **State explicitly which Rust constructors have no Ken
counterpart and why**, rather than silently narrowing.

**The counts make that statement checkable:** `IForm` has **five**
constructors, `Rule` **three** slice variants against the general `§4.3`
`Rule`'s ~20. **A Ken `Rule` with three variants is correct and is the slice;
one with five or twenty is out of scope** (`23 §4.5`, and Banned scope below).
Say which of the ~20 you are not carrying — naming the gap is the deliverable,
not closing it.

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

> **Carry from the Architect's non-blocking review of
> `V3-FO-SEARCH-FUEL-STACK-AGREEMENT` (merged #2393).** That node's fuel/stack
> comment **does not state the fuel the
> probe used**, so the number it records cannot be re-derived and a
> re-measurement needs a fuel override to reproduce it. **State the fuel this
> node's measurement runs under, in the artifact itself** — a measurement whose
> budget is not written down is not re-takeable, which is the same defect one
> level up.

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
(`run_with_big_stack`, in **five** `crates/` files at `6e3b58009`), **so a
harness stack limit is never reported as a mechanism property.** That confusion
is exactly what the predecessor's `D4` had to be corrected for.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Proving `embedding_adequacy` or `checker_soundness`.** Unfiled successors.
- **Emitting `proved` for FO**, on any basis.
- **Widening the slice** beyond `23 §4.5`.
- **Changing the Rust checker to make `D3` agree.** It is the reference.
- **Adding sort validation to the checker.** See the `Form`-excess block above;
  that is [[CORE-FO-CHECK-TREE-SORT-VALIDATION]]'s, and folding it in here would
  make `D3` disagree with the reference **by design**, destroying the control.

## Sequencing

**PROMOTED `draft` → `ready`, 2026-08-16. Both conditions the node set for
itself are met.**

**1. The two nodes it was held behind are merged.**
[[V3-FO-GUARD-SHIFT-DIFFERENTIAL]] (#2371) and
[[V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT]] (#2375), both verified `status: merged`
at `6e3b58009`. Its `depends_on`, [[V3-FO-CONVERSION-LOAD-MEASURED]], is merged
too.

**2. The operator asked for the route (a) cost.** That was one of the two
triggers this node named, and it had **already fired when the node was filed** —
`docs/design/fo-route-theorem-home.md` §4, settled 2026-08-15:

> *"Nothing ventured, nothing gained. We will only know the cost if we build it
> and test it on real programs, so we should do that."*

**The same note names this node's exact subject as the obstacle:** *"the
definitions the theorems are about have not been authored either, which is what
makes the cost untakeable rather than merely untaken."* **That is `D0`-`D2`.**

> ### THE ZERO-POPULATION ARGUMENT DOES NOT BLOCK THIS, AND SAYING WHY MATTERS
>
> The original Sequencing reasoned: no Ken program produces an FO-quotable
> obligation, so a cost number for checking one **gates nothing**. **That is
> still true and it is still not a reason to hold the node.**
>
> The operator's settlement is *build it and measure it* precisely **because**
> the cost is unknown — *"predicting a blowup is not measuring one."* **A
> measurement commissioned to find out what something costs cannot be
> deferred for not yet having a caller**; the caller is what the measurement
> informs.
>
> **Gates-nothing is a statement about urgency. It was read as a statement
> about readiness.** The two came apart here, and lane 2 sat idle across the
> gap.

**This is lane 2 under the operator's 2026-08-15 two-lane directive** — the FO
Kripke embedding half. It does not contend with lane 1 (runtime, `RecursiveDescent`)
or with verify's [[V3-Z3-EMISSION-CONTROL]].

**Not a prerequisite for anything, and nothing waits on it.** A hard stop or a
pathological measurement is a complete result; see `D5`.
