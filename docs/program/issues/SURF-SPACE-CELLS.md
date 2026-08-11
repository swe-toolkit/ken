---
id: SURF-SPACE-CELLS
title: "The `space` block surface — cells and `becomes` — is unbuilt, while its entire desugaring target (the `State` effect: Get/Put/run_state) is built and live"
status: merged
owner: language
size: M–L
gate: none
depends_on: []
blocks: [EFF-SPACE-ENSURES-PRESTATE]
github: https://github.com/swe-toolkit/ken/pull/1152
origin: Steward measurement 2026-07-27 at `origin/main = aea07d62`, taken while scoping the residual left by EFF-SPACE-ENSURES-PRESTATE (closed Shape B, PR #1115). Filed per COORDINATION §2.
---

> ## MERGED — Steward, 2026-08-11. The parked-P2 block was wrong and is replaced.
>
> **`SURF-SPACE-CELLS-P1` merged as PR #1152** at `origin/main = 05f259d7`
> (candidate `ec412eca`, landed tree `b1e4cc10`, blob-verified). **Measured at
> `origin/main = 26b3d2c5`, P1 discharged this node's entire section 4 IN list**,
> so the node is `merged` rather than a first phase awaiting a second.
>
> | this node's IN item | landed control |
> |---|---|
> | the `space` block surface | `ac_s1_spec_counter_example_parses_verbatim` |
> | `mut` cells, `§4.1` desugaring | `ac_s2_middle_write_preserves_both_neighbors`, `ac_s3_reads_each_of_three_pairwise_distinct_components` |
> | `becomes` as `Get`-then-`Put` | `ac_s4_write_core_is_bind_get_then_put` |
> | one effect label per space | `ac_s5_space_label_is_emitted_and_required` |
> | the `§7.3` class-4 error | `ac_s6_mut_outside_space_...`, `ac_s6_becomes_outside_space_...` |
> | `old` stays fenced (an OUT item, held) | `ac_s7_old_in_space_...`, `ac_s7_old_in_pure_code_...` |
>
> ### The two "carried blockers" were both addressed by P1. Corrected 2026-08-11.
>
> The replaced block claimed the Architect's two blockers against candidate
> `31e5f097` were carried into a parked residual and not fixed. **Measured at
> `26b3d2c5`, neither survives**, and the residual it defined therefore does not
> exist:
>
> - **Body-row inference — repaired.** `elab.rs:5852-5864` now calls
>   `infer_expr_row_type` on the operation body and runs `check_decl_poly`
>   against the declared row. `proc leak () : Int visits [S] = fs n` with an `FS`
>   callee is rejected as a false-purity/effect escape. The blocker described the
>   code as checking only `declared_row.concrete_effects()`; that check is still
>   there at `:5841`, but it is now the *first* of two, not the only one.
> - **`Pub(SpaceDecl)` — no longer an `Internal` fall-through.**
>   `modules.rs:1312-1328` refuses public and nested spaces up front with a
>   specific `UnsupportedSpacePlacement`, before qualification can turn the
>   unsupported surface into an internal error. The capability is still absent;
>   what changed is that it now fails closed and says so.
>
> **Why this was worth correcting rather than leaving.** The block was a claim
> about the past written in the present tense, sitting in the one artifact a
> future framer reads to size the work. Acting on it would have produced a WP to
> fix two things that are already fixed.
>
> ### Two live limitations, neither a defect, neither blocking
>
> - **`pub` and nested spaces are refused.** P1 admits only private block spaces
>   at the true file root. `36 §4` specifies neither placement, so this is a
>   scoped limitation rather than an unmet requirement — and per the
>   interrogate-the-constraint rule it does not earn a node until something
>   grounded demands it.
> - **`old` still fails closed**, deliberately — it was always OUT of this node.
>
> ### Successor, now `ready`: [`LANG-SPACE-PRESTATE-BIND`](LANG-SPACE-PRESTATE-BIND.md)
>
> `EFF-SPACE-ENSURES-PRESTATE` closed as Shape B because no cell environment
> existed to elaborate `s_pre`/`s_post` against, and named its successor as
> framable once cells exist. **Cells exist.** That successor is framed and
> released.

> ## ⭐ RELEASED 2026-07-27 to **Team Language** as
> **[`SURF-SPACE-CELLS-P1`](../wp/SURF-SPACE-CELLS-P1.md)**.
>
> ⭐ **This is the node that makes `old` buildable.** `EFF-SPACE-ENSURES-PRESTATE`
> closed as Shape B — `old` fails closed — *because* there was no cell
> environment and no `s_pre`/`s_post` to elaborate against. This node builds the
> cells. The pre-state binding is its **successor**, not its scope.

## 1. The measurement

At `origin/main = aea07d62`. ⛔ Re-derive at point of use.

| piece | spec | built? |
|---|---|---|
| `State S` signature — `Op = Get \| Put S`, `Resp Get = S`, `Resp (Put _) = Unit` | `36 §2.1`, `§4.1` | ✅ `effects/state.rs` — `StateOp s = Get \| Put s`, `resp_state` |
| `get` / `put` / `run_state` declarations | `36 §4.5`, `§4.2` | ✅ `effects/state.rs:571–585` |
| the direct monadic `[State s]` surface | `36 §4.5` | ✅ built |
| **the `space` block — `mut` cells** | `36 §4` | ❌ **absent** |
| **`becomes`** | `36 §4`, `§4.1` | ❌ **absent — zero non-comment occurrences in `crates/`** |
| **cell read / write desugaring** | `36 §4.1` | ❌ absent |
| **one effect label per space** | `36 §4.1` | ❌ absent |
| `becomes`-outside-a-space error | `36 §7.3` class 4 | ❌ absent |

The whole of what `space` does today (`crates/ken-elaborator/src/parser.rs:355`):

```rust
fn parse_space_view_decl(&mut self, start: usize) -> Result<Decl, ElabError> {
    self.advance(); // consume 'space'
    match self.peek().clone() {
        Token::KwProc => self.parse_view_decl(start, true, DefKeyword::Proc),
        other => Err(ElabError::ParseError {
            msg: format!("expected 'proc' after 'space', found {:?}", other), ... }),
    }
}
```

`space` is a **modifier on a `proc` declaration**. The block form in `36 §4` —

```
space Counter {
  mut n : Int = 0
  proc inc () : Unit  visits [Counter] = n becomes n + 1
  proc get () : Int   visits [Counter] = n
}
```

— does not parse. There are no space-block tests and no corpus usage.

## 2. ⭐ Why this is a good node rather than a big one

**The hard half is already built.** `36 §4.1` says a space *desugars to* a
`State` effect, and every piece of that target — the signature, the response
family, `get`, `put`, and the `run_state` fold — is live in
`crates/ken-elaborator/src/effects/state.rs`. This node is the **surface and the
desugaring onto it**, not a new effect and not new kernel machinery.

⛔ **`becomes` is not kernel mutation.** `§4.1`, verbatim: *"So `becomes` is
**not** a kernel mutation — it is a `Get`-then-`Put` on the pure tree."* This
node has **zero kernel and zero trusted-base delta**. If a candidate introduces
a mutable cell into the TCB, the premise has failed.

## 3. The desugaring is given, not designed

`36 §4.1` specifies it verbatim. ⛔ **Do not invent it.**

- state type `S = T₁ × … × T_m` — right-nested Σ / record (`13 §3`), with η so
  cell update reconstructs definitionally;
- one **effect label** per space; every operation `visits [<space>]` uses
  `State S`;
- cell access:

```
cᵢ            (read)   ⤳  bind (perform Get) (λ s. Ret (s.i))
cᵢ becomes e  (write)  ⤳  bind (perform Get) (λ s. perform (Put (s with .i := ⟦e⟧)))
```

where `s with .i := v` is the record/Σ update **reusing every other component**.

## 4. Scope

**IN:** the `space` block surface, `mut` cells, `becomes`, the `§4.1`
desugaring, the per-space effect label, and the `§7.3` class-4 error.

⛔ **OUT:**
- ⛔ **`old` / the pre-state binding.** `OldPreStateUnsupported` **stays**. It is
  the successor's subject and it has its own controls. A candidate that makes
  `old` work as a side effect has silently merged two WPs and left the second's
  controls unwritten.
- ⛔ **A second `State`.** Reuse `effects/state.rs`.
- ⛔ **Kernel / trusted-base changes.**
- ⛔ **`§4.4` concurrency & isolation** (`OQ-Space`) — a separate concern.
- ⛔ **Re-specifying `run_state`.** `§4.5.3` says it *is* `§4.2`'s fold at
  `F = 𝟘`, already built.
