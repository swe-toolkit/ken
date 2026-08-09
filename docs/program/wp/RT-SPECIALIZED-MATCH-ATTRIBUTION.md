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

### Field 2 — the ordinary `Match`, its cases, and the producing route

| field | value |
|---|---|
| `static_origin` | `StaticOriginId(53)` |
| case constructors | `ctor:nested_inductive_pkg::global_574::ctor_575`, `…::ctor_576`, `…::ctor_577` |
| incoming route | `SourceComputationalAnswerRoute::DirectScrutinee` |

Three constructors on one source type, which matches the witness's `Bag`
(`Empty` / `One` / `Join`). So the refusing `Match` is the **inner** match on
`b`, not the outer match on `r`.

**The route is `DirectScrutinee`, and that is a fact about the producer rather
than the consumer.** The value was not raised by an exact producer and did not
arrive through a composition or resume path; it was produced by ordinary source
evaluation and handed straight to the scrutinee position. The pairing is read
from `RoutedAnswer { value, route }`, which the machine already carries for
exactly this purpose, rather than reconstructed.

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

**Established:** the rejected value is a `ComputationalRecursorClosure` arriving
by `DirectScrutinee` at the inner three-constructor `Match` at `SOI(53)`, with
an empty pending continuation.

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

## Controls and hazards

**No control here keys on `("Match", "scrutinee is not a constructor value")`.**
The `#[cfg(test)]` hook at `core.rs:6164` fabricates that identical pair, so a
control keyed on it cannot separate the production seat from the injection.
This record keys on the **seat and the measured operand** instead.

**The instrument was placed where it could not confuse the two.** The probe bound
the operand inside the production remainder arm itself, so it observes the value
that seat actually received rather than a message anyone could have produced.
