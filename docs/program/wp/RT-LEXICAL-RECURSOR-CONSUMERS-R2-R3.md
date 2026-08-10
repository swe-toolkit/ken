# RT-LEXICAL-RECURSOR-CONSUMERS — `R2`/`R3` root evidence

Node: `docs/program/issues/RT-LEXICAL-RECURSOR-CONSUMERS.md`.
**Evidence only: no repair, no control, `crates/` byte-identical.**

## 0. Provenance — two coordinates, and they are NOT the same

| coordinate | exact | moves? |
|---|---|---|
| **measurement base** — the tree every figure below was taken on | `760a0eff`, the `D2a` merge | **no.** A commit on `main`; nothing this branch does can move it |
| **candidate merge-base** — where this candidate sits | `06eae51bb5e175a57605b2f1c3754ad1ae3310ca` | **yes.** Every re-anchor moves it |

⛔ **These were one sentence in an earlier revision — *"measured at `760a0eff`,
which is also this candidate's base"* — and a re-anchor made the second half
false while the first stayed true.** They are recorded separately here because
they are separately true, and because one of them is not stable.

⛔ **The measurement is preserved as HISTORICAL EVIDENCE at `760a0eff`, and is
not re-attributed to the candidate base.** It is not withdrawn: it was taken on
a real tree and it stands as a statement about that tree.

⛔ **Path-disjointness does NOT make the two bases equal, and no such argument
is made here.** That the re-anchor touched no path this record measures is a
reason the evidence remains *useful*; it is not a reason to call `06eae51b` the
tree the numbers came from. Conflating "the measurement is still applicable"
with "the measurement was taken here" is the error this section exists to
prevent — re-measure if you need the stronger claim.

> # ⛔ TWO ROOTS, NOT ONE. `D2b` IS NOT AUTHORIZED BY THIS EVIDENCE.
>
> `R2` and `R3` refuse inside the **same walk**,
> `Lowered::boundary_transfer_admissibility`. That is where the resemblance
> ends. They differ in **operand representation**, in **first missing owner**,
> and in **route** — the three axes the ruling named — so this evidence returns
> a **split**, and the sizing decision is the Steward's.

## 1. Why the shared name proves nothing

Both rows render a refusal from `boundary_transfer_admissibility`, and both
refusals are about a closure-ish thing crossing a boundary. **That is a shared
consumer, not a shared root.**

This node has already paid once for a string-keyed root verdict: the `D0`/`D1`
record concluded `R1` did not share `#6c`'s root *because the refusal strings
differed*, and the operand turned out to be the same `RecursiveBackedge`. The
instrument was wrong in that direction; here it would be wrong in the other —
**same string, different roots**. So every row below is measured, not read.

## 2. The measurement

Instrumented at the measurement base, at `transfer_into_carrier`
(`lowering/mod.rs`) — the **sole** production entry to the walk — capturing the
operand's representation, the static origin, and the calling owner at the moment
admissibility fails. Run under B-only exclusion through
`px8j_capture_source_trace`.

| axis | `R2` — row 3 | `R3` — row 5, before-hole |
|---|---|---|
| **operand representation** | **`Closure`** | **`Constructor`** |
| refusing arm | `Lowered::Closure \| DeclarationClosure` — the value **itself** | `Lowered::ComputationalRecursorClosure` — a **child**, reached by the walk's recursion |
| static origin | `StaticOriginId(5)` | `StaticOriginId(23)` |
| **first missing owner** | `call_declared_unit_target` | **`lower_carried_computational_match`** |
| **route into the transfer** | `claim_and_call_resolved_continuation` → declared-unit call argument | carried computational-match elimination |
| enclosing seat | `lower_computational_match_expr` | `lower_computational_match_expr` |

**The representations are not the same kind of thing.** `R2`'s value *is* a
closure and is refused by the walk's first arm. `R3`'s value is a
**constructor** whose recursive descent finds a recursor-closure child. A repair
that consumed `R2`'s closure at its owner would not encounter `R3`'s value at
all, and vice versa.

**The owners are different functions reached by different routes.** `R2` arrives
while claiming and calling a resolved continuation, as a declared-unit call
argument. `R3` arrives inside a carried computational-match elimination. Only
the enclosing seat is common, and an enclosing seat is not an owner — `D2a`'s
own ruling turned on exactly that distinction.

## 3. A second finding: row 5 is not homogeneous

Row 5 compiles **two** expressions, before-hole and after-hole. Post-`D2a` they
no longer fail the same way:

| row 5 compile | first refusal at the measurement base |
|---|---|
| before-hole | `ComputationalMatch` — *a computational recursor closure names an in-flight activation* (**`R3`**) |
| after-hole | **`StaticWorkerBinding`** — *a `Var` in value position … has no value representation* |

⇒ **The after-hole compile has already advanced past `R3` to `D2a`'s successor
wall.** So "row 5 is `R3`" is true of one of its two compiles. Any sizing that
treats row 5 as a single unit of `R3` work will be wrong by one compile, and
that compile is blocked behind a wall this node does not own.

⛔ Recorded, not acted on: I did not build against `StaticWorkerBinding`.

## 4. What this evidence does and does not support

**Supports:** `R2` and `R3` are **two roots**. Splitting them is the shape the
measurement implies.

**Does not support:** any claim about how *hard* either is, or that either is
small. This is a root-identity measurement, not a sizing one. Two roots means
two repairs, and neither has been attempted.

**Not covered:** whether `R2`'s owner and `R3`'s owner share an upstream
*producer* further back than the first missing consumption point. The ruling
asked for the first missing owner and route, and that is what is measured; a
common ancestor beyond it would be a different question and would need its own
evidence.

## 5. Scope

No repair. No control. No `D3` debt touched. `R4` untouched and out of this node.
No retirement, no lane deletion, no tracker `status:` change. `crates/` is
byte-identical to the candidate merge-base; the instrument was temporary and is
removed
(`lowering/mod.rs` sha256 `3eea88f9…7e00` matches base).
