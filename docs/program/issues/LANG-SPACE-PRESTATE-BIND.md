---
id: LANG-SPACE-PRESTATE-BIND
title: "`old` in a block-space operation's `ensures` still fails closed, though the cell environment it was waiting for now exists -- bind s_pre/s_post and elaborate the Hoare pair against the state transformer"
status: merged
owner: language
size: M
gate: none
depends_on: [SURF-SPACE-CELLS]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1848
origin: Steward measurement 2026-08-11 at origin/main=26b3d2c5, taken while running the stay-one-release-ahead check as LANG-SELECTOR-CLASSIFIER-RESIDUAL-DIAGNOSTIC entered the publisher. This is Shape A of EFF-SPACE-ENSURES-PRESTATE, which closed as Shape B (fail closed) precisely because no cell environment existed to elaborate against. SURF-SPACE-CELLS-P1 built that environment, so the successor its own closure block named as "framable once cells exist" is now framable.
---

## Why this exists now and not before

`EFF-SPACE-ENSURES-PRESTATE` closed 2026-07-27 as **Shape B — fail closed**.
`old` stopped silently meaning its operand and started refusing with a
span-carrying `OldPreStateUnsupported`. That closure was explicit that it was
partial by design, and equally explicit about **what the residual was waiting
on**:

> Shape A — elaborating `ensures` against the state transformer — was
> unavailable in this slice because the parser has only `space proc`: no
> `becomes`, no cell environment, no `s_pre`/`s_post` binding to elaborate
> against.

**That blocker is gone.** `SURF-SPACE-CELLS-P1` landed the block-space surface,
`mut` cells, `becomes`, and the `36 §4.1` desugaring onto `State S`. The state
`S` that `s_pre` and `s_post` must range over is now a real, constructed thing.

**This node is the successor that closure named**, and it is the last piece of
`36 §4.3`.

## The measurement

At `origin/main = 26b3d2c5`. **Re-derive at point of use.**

**The scope is the whole contract surface, not `old` alone.** A block-space
operation carrying **any** `requires` or `ensures` is refused today, `old` or no
`old` — `elab.rs:5816-5822` returns a `TypeMismatch` whose reason reads
*"space-operation contracts without `old` are staged with the pre-state
successor"*. **This node is the successor that message names.** `SURF-SPACE-CELLS-P1`
staged the entire block-space contract surface here deliberately, so there is no
working contract path for block spaces onto which `old` could be bolted.

Two guards sit on the block-space path and both go:

| site | guard |
|---|---|
| `elab.rs:5811-5814` | a `first_old_span` pre-scan over `requires ++ ensures`, returning `OldPreStateUnsupported` |
| `elab.rs:5816-5822` | the blanket contract refusal above |

`OldPreStateUnsupported` has **three production construction sites**, all in
`crates/ken-elaborator/src/elab.rs`: `:798` (checking arm), `:3026` (inference
arm), `:5813` (the pre-scan). The checking and inference arms are separately
reachable — a repair that fixes one leaves `old` refusing through the other —
and once the pre-scan is gone, block-space clauses reach those same generic
arms. **They cannot simply be deleted**; they become conditional on a pre-state
binding, because the modifier form still has none.

**The blanket refusal at `:5816` has zero test controls** — its message appears
exactly once in `crates/`, at the construction site. Nothing reds when it is
removed, so every control for the contract surface has to be written.

## The control census, and it is the whole risk in this node

**Six existing assertions pin the refusal. Exactly one of them must flip.**

| file | surface | disposition |
|---|---|---|
| `tests/surf_space_cells_p1.rs:459` `ac_s7_old_in_space_preserves_the_specific_fence` | **block** space with `mut n` | **MUST FLIP to working** |
| `tests/surf_space_cells_p1.rs` `ac_s7_old_in_pure_code_remains_unbound` | pure `fn` | must stay `UnboundName(old)` |
| `tests/v1_acceptance.rs:230` | `space proc` | must stay refusing |
| `tests/v1_acceptance.rs:245` | `space proc`, proposition-root `old` | must stay refusing |
| `tests/let5_checking_mode_let.rs:85` | `space proc` | must stay refusing |
| `tests/kenfmt_b3_layout.rs:122,126` | `space proc` | must stay refusing |

> ### THE DISCRIMINATOR IS `space proc` VERSUS `space { }`, AND THE TWO READ ALIKE
>
> `space proc f (n : Nat) : Nat` is the **modifier** form: `n` is a
> **parameter**, there are no cells, and there is **no state `S`** for a
> pre-state to be drawn from. `space Counter { mut n : Int = 0 ... }` is the
> **block** form, and only it has the cell environment `36 §4.3` needs.
>
> ⇒ **`old` becomes meaningful in the block form and must go on refusing in the
> modifier form.** Both spell the word `space`, both appear in an `ensures`, and
> five of the six controls above are the modifier form.
>
> **A repair keyed on "are we in a space?" rather than "is there a cell state?"
> passes the one test that must flip and silently un-fences the other five.**
> That direction fails toward accepting a term whose pre-state does not exist,
> which is the direction nothing reds on.

**Report the five surviving refusals as the control**, with their count. A claim
that the block-space case now works says nothing about the modifier form.

## What the spec settles, so do not design it

`spec/30-surface/36-effects.md §4.3` gives the semantics, and `§4.1` already gave
the desugaring. **This is a binding-and-elaboration job, not a design fork** —
the same character as `SURF-SPACE-CELLS`, and for the same reason.

- `requires φ` constrains the pre-state `s_pre : S` and the parameters.
- `ensures ψ` relates `s_pre`, `result`, and the post-state `s_post`.
- A bare cell `cᵢ` in `ensures` is the **post-state** value; `old(cᵢ)` is the
  **pre-state** value.
- The obligation is **local, bounded, per-space Hoare** over `S` — the spec is
  explicit that there is **no separation logic, no frame rule, and no global
  `\old`**.

The spec even supplies the worked example to check against: `inc`'s `ensures n ==
old(n) + 1` denotes to `λ s. (tt, s with .n := s.n + 1)`, giving the obligation
`(s with .n := s.n + 1).n == s.n + 1`, which computes by record-β/η to `s.n + 1
== s.n + 1` and is discharged by `refl`.

**That example is the acceptance target.** If it does not compute to a
`refl`-discharged obligation, the binding is wrong regardless of what elaborates.

## Scope

**IN:** binding `s_pre` and `s_post` for a block-space operation; elaborating
`requires`/`ensures` against the state transformer; `old(e)` denoting `e` in
`s_pre`; a bare cell denoting its `s_post` value; the emitted obligation for the
`§4.3` worked example.

**OUT:**

- **`space proc` — the modifier form keeps `OldPreStateUnsupported`.** It has no
  cell state and the diagnostic stays correct there.
- **A second `State`.** Reuse `crates/ken-elaborator/src/effects/state.rs`.
- **Kernel or trusted-base changes.** `becomes` is a `Get`-then-`Put` on the pure
  tree, not kernel mutation, and the pre-state binding does not change that. A
  candidate that grows the trusted base has failed a premise.
- **`§4.4` concurrency and isolation** (`OQ-Space`).
- **`pub` and nested space placement.** Both are refused today by an explicit
  `UnsupportedSpacePlacement`, which is a scoped limitation rather than a defect;
  see the note on [[SURF-SPACE-CELLS]].
