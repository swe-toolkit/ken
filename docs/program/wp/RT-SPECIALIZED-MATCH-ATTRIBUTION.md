# RT-SPECIALIZED-MATCH-ATTRIBUTION — the D0 measurement

**Record only. No production change.** The fail-closed refusal is preserved
exactly; `crates/` is byte-identical to the base. `876450ab` is untouched.

| operand | value |
|---|---|
| base | `1f706520b2cadc164eb5fcc92fc5a96b5b0619a0` |
| node blob | `05e5254edbb9771c5ca2ccef3d68d3185e20ce71` |
| witness venue | `ac7377b1eabf11d85183ce59550ee35c734bbc5e`, tree `3958b824cab9d4ecf763de5d32266fc6f2f270e7` |

**The answer is `ComputationalRecursorClosure`, so the node's pre-committed
disposition returns this for a fresh mechanism ruling. It is not
`ProcessExitStatus` and it is not routed anywhere by resemblance.**

## The venue, and why it is not venue 4

The prior venues composed Kernel's held `dd3cd050` with Runtime at `f0217c67`.
That is now the wrong Runtime side: `RT-BODY-OCCURRENCE-PROVENANCE` merged as an
accepted partial, so the correction under measurement is on `main`. This venue
is therefore **re-derived, not transplanted**: `origin/main 1f706520` merged with
the projection snapshot `a577f136`, whose parent is `dd3cd050`.

Clean merge, no conflicts. All three are ancestors of the venue tip
(`1f706520`, `dd3cd050`, `a577f136`), and the provenance partial is present in
the venue tree. Kernel's branch was not moved, edited, named or committed to.

Reproduce:

```sh
git worktree add --detach <path> 1f706520
git -C <path> merge a577f136          # brings dd3cd050 as its parent
```

Then run, from that worktree:

```sh
scripts/ken-cargo test -p ken-elaborator --test nc14_data_match_lowering -- \
  nested_recursive_field_elaborates_checks_and_runs_from_checked_artifact
```

## The four fields

Measured at the firing arm, `lowering/core.rs` `SourceContinuation::MatchScrutinee`,
the `LoweringOperand::Specialized` remainder.

### Field 1 — the exact `LoweredVariant`

```text
ComputationalRecursorClosure
```

**Not `Specialized` alone, which is a phase.**
`Lowered::ComputationalRecursorClosure` carries `residual` and `invocation`
fields.

**The remainder is 15 variants wide, not 9.** `LoweredVariant` has **21**
members; six are accepted explicitly at this seat, so fifteen fall through.
An earlier count of mine said nine and was wrong — it came from a grep window
that truncated the enumeration at fifteen lines. The corrected enumeration is
`Int, Bool, ProcessExitStatus, CapabilityToken, ResourceToken, BoundedNat,
StructuralNat, ResponseBytes, HostResult, DynamicConstructor, Bytes,
BorrowedNativeValue, BorrowedOption, String, Constructor, Record, Closure,
DeclarationClosure, ComputationalRecursorClosure, RecursiveBackedge, Trap`.

### Field 2 — the eliminated family, the scrutinee occurrence, and the route

| field | value |
|---|---|
| `static_origin` | `StaticOriginId(53)` |
| case constructors | `ctor:nested_inductive_pkg::global_574::ctor_575`, `…::ctor_576`, `…::ctor_577` |
| incoming route | `SourceComputationalAnswerRoute::DirectScrutinee` |

#### `global_574` resolved from the checked artifact, not from its arity

| probe | result |
|---|---|
| `is_terminal_support(GlobalId(574))` | **`true`** |
| `all_support_origin(GlobalId(574))` | **`Some((host = g570, parameter = 0, sort = Type))`** |
| name of `g570` | **`Bag`** |
| source name of `g574` | **`None`** |
| `Bag` / `LiftRose` ids | `g570` / `g582` |
| constructor count of `g574` | 3 |

⇒ **`global_574` is the kernel-generated terminal All support for host `Bag` at
parameter 0. It is not `Bag`.** `Bag` is `g570` and is not the eliminated
family here. `g574` carries no source name at all, which is why it is rendered
by the `global_<id>` fallback.

**So the refusing `Match` is a Runtime elimination over the source-indexed All
inhabitant, not the surface-originating `match b` over `Bag`.**

#### The scrutinee occurrence immediately before `SOI(53)`

| field | value |
|---|---|
| occurrence | **`StaticOriginId(52)`** |
| `RuntimeExpr` kind | **`Var(0)`** |
| runtime binder index | **0** |
| binding role / value class | **`Value(Specialized(ComputationalRecursorClosure))`** |
| environment length | 3 |

The scrutinee is a **variable read**, and binder 0 already holds a
`ComputationalRecursorClosure`. Nothing at `SOI(52)` produces the value; it
reads one the environment was already carrying.

#### What `DirectScrutinee` does and does not say

**It is an exclusion, not a producer identity.** It rules out a value *raised by
an exact producer* and one arriving by composition or resume. It does **not**
name what produced this value, and reading it as one is an over-claim. The
producer identity that is actually available is the row above: binder 0's
binding.

### Field 3 — the continuation stack at arrival

```text
Terminal
```

The `MatchScrutinee` frame is popped into `control.continuation` before the
operand is matched, so the stack at arrival is the remainder beneath it: the
chain terminates immediately. **This match is the outermost pending
continuation** — nothing is waiting on its result.

The walker names each frame and would print `<unwalked-variant>` for any variant
it cannot descend; it printed `Terminal`, so the chain is complete rather than
truncated.

### Field 4 — the refusal, preserved

```text
Unsupported { stage: NativeLoweringOrExecution,
              construct: "Match",
              reason: "scrutinee is not a constructor value" }
```

Unchanged. All instrumentation lived on the disposable venue and none of it is
in this candidate.

## What this does and does not establish

**Established:** the rejected value is a `ComputationalRecursorClosure`, read
from runtime binder 0 at `SOI(52)` (`Var(0)`) and handed to the `Match` at
`SOI(53)`, which eliminates the **kernel-generated terminal All support for host
`Bag` at parameter 0** — not `Bag` itself and not the surface `match b`. The
pending continuation is empty.

**Not established, and deliberately not argued here:** whether the correct
mechanism is consumer widening at this seat, an upstream composition that should
have eliminated the recursor closure before the scrutinee position, or terminal
propagation. The Architect reserved that ruling, and the node forbids choosing
among them without a fresh one.

**One prior, weighed as a prior.** The `Carried(word)` arm immediately above
this seat records that an adjacent operand class once *"fell past every shape
test onto the refusal below — a true sentence about the wrong thing, naming a
cause that is not the cause"*, and that this seat was the only one of three
missing that arm. The measured variant is consistent with that shape. **It is
not evidence of it**, and the node says so; the disposition still requires the
ruling.

## The correction this record carries, and how the first version went wrong

**The first version of Field 2 claimed the case constructors "match the
witness's `Bag`" and concluded the refusing `Match` was the inner `match b`.
That was wrong, and it was wrong by resemblance.**

The generated All support for `Bag` has **three constructors mirroring its
host's**, so the constructor count is identical between the family I named and
the family actually present. **Counting could not discriminate them, and I used
a count.** Route only on a measured class, never on resemblance, is the node's
own instruction for this seat — and I broke it while recording evidence about a
seat that exists because of an earlier resemblance error.

The discriminator that settles it is the **issued relation**, not the shape:
`is_terminal_support` and `all_support_origin` are recorded by the kernel at
generation time and name the host and parameter outright. One call answers it,
with no arity, name-shape or case-count reasoning anywhere in the chain.

**Corrected in place rather than annotated.** The superseded sentences are gone
from Field 2, because a later note saying a claim is false leaves the false
claim sitting where a reader reaches it first.

## Controls and hazards

**No control here keys on `("Match", "scrutinee is not a constructor value")`.**
The `#[cfg(test)]` hook at `core.rs:6164` fabricates that identical pair, so a
control keyed on it cannot separate the production seat from the injection.
This record keys on the **seat and the measured operand** instead.

**The instrument was placed where it could not confuse the two.** The probe bound
the operand inside the production remainder arm itself, so it observes the value
that seat actually received rather than a message anyone could have produced.
