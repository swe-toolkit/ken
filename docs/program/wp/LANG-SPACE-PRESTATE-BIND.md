# LANG-SPACE-PRESTATE-BIND — bind `s_pre`/`s_post` and make `old` mean the pre-state

Owner: language. Size: M. Node: [[LANG-SPACE-PRESTATE-BIND]] (`ready`).

**Released 2026-08-11 on `main` `26b3d2c5`.** Re-derive your merge-base; do not
reuse a SHA from this frame. `LANG-SELECTOR-CLASSIFIER-RESIDUAL-DIAGNOSTIC` is
in the publisher as you receive this and touches `elab.rs` and `error.rs`, so cut
from `origin/main` after it lands rather than from the SHA above.

## What you are doing

`old` in a **block-space** operation's `ensures` refuses with
`OldPreStateUnsupported`. That refusal was correct when it was written — there
was no cell environment, so there was no pre-state for `old` to denote.
`SURF-SPACE-CELLS-P1` built the cell environment. **Make `old` mean what
`36 §4.3` says it means, and leave every other `old` refusing.**

## Fixed inputs — measured, do not re-derive

At `origin/main = 26b3d2c5`.

> ### THE SCOPE IS THE WHOLE CONTRACT SURFACE, NOT JUST `old`
>
> A block-space operation carrying **any** `requires` or `ensures` is refused
> today, `old` or no `old`. `elab.rs:5816-5822`:
>
> ```rust
> if !operation.requires.is_empty() || !operation.ensures.is_empty() {
>     return Err(ElabError::TypeMismatch {
>         span: operation.span.clone(),
>         reason: "space-operation contracts without `old` are staged with the pre-state successor"
>     });
> }
> ```
>
> **You are the pre-state successor that message names.** `SURF-SPACE-CELLS-P1`
> staged the entire block-space contract surface here deliberately. So the
> deliverable is `requires` and `ensures` on block-space operations *and* the
> `old` denotation — not `old` alone bolted onto a working contract path, because
> there is no working contract path for block spaces to bolt it onto.

**Two refusals guard the block-space path, and they fire in this order:**

| site | guard | disposition |
|---|---|---|
| `elab.rs:5811-5814` | a `first_old_span` pre-scan over `requires ++ ensures`, returning `OldPreStateUnsupported` | both go |
| `elab.rs:5816-5822` | the blanket contract refusal above | both go |

`first_old_span` (`elab.rs:5713`) is a structural walk that finds the first
`ROld` anywhere in a clause. **It is a pre-scan, not the elaborator** — it runs
before any binding exists, which is why it can only refuse.

**`OldPreStateUnsupported` has three production construction sites**, all in
`crates/ken-elaborator/src/elab.rs`: `:798` (checking arm), `:3026` (inference
arm), and `:5813` (the pre-scan above). **The checking and inference arms are
separately reachable** — `v1_acceptance.rs` exercises each deliberately, with a
comment saying so, precisely to stop one transparent arm surviving a repair.

> **`:798` and `:3026` must keep refusing for `space proc` and start succeeding
> for block spaces.** Once the pre-scan is gone, block-space clauses reach those
> same generic arms. **They cannot simply be deleted** — that un-fences the
> modifier form. They become conditional on a pre-state binding being in scope.
> This is the same discriminator as `AC-4`, seen from the other side.

**The blanket refusal at `:5816` has ZERO test controls.** Grepping its message
across `crates/` returns exactly one hit: the construction site itself. Nothing
reds when you remove it, and no existing negative control covers that branch —
**every control for the contract surface is yours to write.** Do not read a green
suite after deleting it as evidence of anything.

The desugaring target is built and live in
`crates/ken-elaborator/src/effects/state.rs`: `StateOp s = Get | Put s`,
`resp_state`, and the `get`/`put`/`run_state` declarations. **You are not
building an effect.**

## The semantics are given by the spec. Do not design them.

`spec/30-surface/36-effects.md §4.3`, and it is short enough to hold in full:

- `requires φ` constrains the **pre-state** `s_pre : S` and the parameters;
- `ensures ψ` relates `s_pre`, `result`, and the **post-state** `s_post`;
- a bare cell `cᵢ` in `ensures` is the **post-state** value; `old(cᵢ)` is the
  **pre-state** value;
- the obligation is **local, bounded, per-space Hoare** over `S`. The spec is
  explicit: **no separation logic, no frame rule, no global `\old`.** If you find
  yourself needing any of the three, stop — the design has drifted.

## Deliverable

Bind `s_pre` and `s_post` for a block-space operation, elaborate `requires` and
`ensures` against the state transformer `S → R × S`, and emit the `§4.3`
obligation. **Both block-space guards go** — the `first_old_span` pre-scan and
the blanket contract refusal — and the generic checking and inference arms
become conditional on a pre-state binding rather than unconditional refusals.

## Acceptance criteria

**AC-1 — the spec's own worked example, end to end.** `§4.3` supplies it:
`inc`'s `ensures n == old(n) + 1` denotes to `λ s. (tt, s with .n := s.n + 1)`,
giving the obligation `(s with .n := s.n + 1).n == s.n + 1`, which computes by
record-β/η (`13 §3`) to `s.n + 1 == s.n + 1` and is discharged by `refl`
(`16 §2`). **Elaborating it must produce an obligation that so discharges.** An
`ensures` that merely elaborates without the obligation computing is not this AC.

**AC-2 — the pre/post discriminator, and it needs a non-degenerate pair.** On
**one** operation that actually changes the cell, show that a bare `cᵢ` denotes
the post-state value and `old(cᵢ)` denotes the pre-state value, as two
assertions on the same shape. **The operation must write** — if the cell is
unchanged, `s_pre` and `s_post` agree and the case passes under a binding that
swaps them. State which value each side took.

**AC-3 — both arms, because one repair can leave the other refusing.** Cover
nested `old` (reaching the checking arm at `:798`) and proposition-root `old`
(reaching the inference arm at `:3026`) in a block space. `v1_acceptance.rs`
already distinguishes these two routes for the `space proc` form; mirror that
distinction for the block form.

> ### AC-4 — THE DECOY, AND IT IS FIVE-SIXTHS OF THE EXISTING CONTROLS
>
> **Six assertions currently pin the `old` refusal. Exactly one must flip.**
>
> | file | surface | disposition |
> |---|---|---|
> | `tests/surf_space_cells_p1.rs` `ac_s7_old_in_space_preserves_the_specific_fence` | **block** space, `mut n` | **flips to working** |
> | `tests/surf_space_cells_p1.rs` `ac_s7_old_in_pure_code_remains_unbound` | pure `fn` | stays `UnboundName(old)` |
> | `tests/v1_acceptance.rs:230` | `space proc`, nested `old` | stays refusing |
> | `tests/v1_acceptance.rs:245` | `space proc`, root `old` | stays refusing |
> | `tests/let5_checking_mode_let.rs:85` | `space proc` | stays refusing |
> | `tests/kenfmt_b3_layout.rs:122,126` | `space proc` | stays refusing |
>
> **The discriminator is `space proc` versus `space { }`, and the two read
> alike.** `space proc f (n : Nat) : Nat` is the **modifier** form: `n` is a
> parameter, there are no cells, and **there is no state `S`** for a pre-state to
> come from. Only the block form has the cell environment `§4.3` needs.
>
> ⇒ **A repair keyed on "are we inside a space?" rather than "is there a cell
> state?" flips the one test that must flip and silently un-fences the other
> five.** That direction accepts a term whose pre-state does not exist, which is
> the direction nothing reds on.
>
> **Report the five surviving refusals with their count as the control.** A claim
> that the block case works says nothing about the modifier form, and this frame
> will not accept the first without the second.

**AC-5 — no trusted-base delta.** `§4.1` is explicit that `becomes` is *not*
kernel mutation but a `Get`-then-`Put` on the pure tree, and binding a pre-state
does not change that. Assert `trusted_base()` is unchanged. **If a candidate
needs a mutable cell in the TCB, a premise has failed — stop and come back.**

**AC-6 — `requires` reaches the pre-state.** `§4.3` gives `requires` the
pre-state and the parameters. Bind it, and show one operation whose `requires`
constrains a cell. If `requires` turns out already to be handled by an existing
path, say so and show it rather than adding a second one.

## Excluded scope

- **`space proc` keeps `OldPreStateUnsupported`.** It has no cell state and the
  diagnostic is correct there. This is AC-4's control, not a follow-up.
- **A second `State`.** Reuse `effects/state.rs`.
- **Kernel and trusted-base changes** (AC-5).
- **`§4.4` concurrency and isolation** — `OQ-Space`, a separate concern.
- **`pub` and nested space placement.** Both are refused by an explicit
  `UnsupportedSpacePlacement` and stay refused; `36 §4` specifies neither.
- **No new capability.** If `§4.3` cannot be expressed against the landed cell
  environment, that is a hard stop and it comes back to me. Do not invent the
  missing surface.

## Contention

`crates/ken-elaborator` and its tests. Runtime is on `crates/ken-runtime` under
`RT-LEXICAL-RECURSOR-CONSUMERS`; Kernel's remaining `AC-K12` stage is
Runtime-owned. **No `spec/` or `conformance/` path, so no Spec vote on the merge
Decision** — you are implementing a spec that is already written, not amending
one.

`elab.rs` is the file the classifier-diagnostic candidate is landing in right
now. Cut after it lands and the intersection is empty by construction.

## Validation

Targeted only. `-p ken-elaborator`, or `--test <name>`, **never `--workspace`**.
Adding or changing an enum variant makes the floor a full `-p ken-elaborator`
test build, because a suite-scoped run cannot observe an exhaustive `match` in a
sibling target. "No regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. Both are good
outcomes. **If AC-1's obligation does not compute to `refl`, that is a finding
worth the turn** — report it with the obligation you actually got rather than
adjusting the AC to match.
