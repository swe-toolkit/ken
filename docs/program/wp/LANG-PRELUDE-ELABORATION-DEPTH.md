# LANG-PRELUDE-ELABORATION-DEPTH

**Owner:** language. **Size:** S. **Gate:** none.
**Predecessor:** `LANG-PRELUDE-COLLECTIONS` — this node consumes its `#2144`
measurement as a fixed input and must not re-run it.

Measure what a Ken compilation actually requires of the stack it is handed,
state that number where the next author finds it, and — only if the margin turns
out to be thin — replace the canary the `r3_4b` fix retires.

## The measurement, at `origin/main = c1b9a1e8`

**Re-derive at point of use.** Two of these are numbers, one is an absence, and
the absence is the finding.

| what | where |
|---|---|
| the measured frame budget | `crates/ken-elaborator/src/elab.rs:997` |
| the failing worker's source, calling no combinator | `tests/r3_c2_source_mixed_branch.rs:501-505` |
| the failing worker's spawn, a nested debug `cargo test` | `tests/r3_c2_source_mixed_branch.rs:556-580` |
| nine of thirteen `stack_size` sites, all 256 MiB | `ken-cli/tests` (6), `ken-elaborator/tests` (3) |
| the two `ken-runtime` sites, both under `#[test]` | `static_transition.rs:23720` (in the `#[test]` at `:23582`), `lowering/core/tests/control.rs:11129` |
| **production `stack_size` sites** | **none — the absence is the point** |

`elab.rs:997`, from `LANG-RECORD-STACK-OVERFLOW` (`b4d38b8a`):

> *"~115 KiB of headroom out of a 2 MiB thread stack remained at the deepest
> call after that node's repair -- cleared by inches, not a mile."*

and, in the same comment, why the margin erodes with no one touching the deep
path: **in an unoptimized build a new arm's locals in `check` are paid by every
call regardless of which arm runs.**

## The design call, front-loaded

**This node measures and states. It does not tune, and it does not add.**

Two levers were named while diagnosing `#2144` and both are out. **Reducing
per-frame cost on the `check`/`infer` path** is a real lever and it is a
different node — it changes behaviour, and mixing it with the measurement means
the number you publish describes a tree that no longer exists by the time it
lands. **`RUST_MIN_STACK`** is out for the reason the Architect gave and then
sharpened at `evt_26jk8jqrgxb13`: it is global and invisible, changing every
thread in the process, whereas a named worker's explicit `stack_size` is local
and legible. That distinction is the durable part; do not relitigate it.

**Measure the product path, not the harness.** The whole gap is that thirteen
measurements exist and every one of them is of a test thread. A fourteenth test
measurement adds nothing.

## Deliverables

- **D1 — the number.** Measure peak stack consumed by a full elaboration on the
  product path: a `ken-cli` compile of a small program, in both **debug** and
  **release**. Report actual peak, not "it did not crash." Two builds because
  `elab.rs:997` states the unoptimized cost is the one that grows.
- **D2 — shape or cumulative.** The four combinators are now **landed** in
  `prelude.rs` (`60b78c95`), so run this subtractively: remove all four, measure,
  then add them back **one at a time**, reporting the peak after each.
  **If the tipping declaration is `zip` or `filter`, the cost is
  shape-dependent** (nested matches). **If it is whichever lands fourth
  regardless of which one that is, the cost is cumulative** and the rule binds
  every prelude addition, recursive or not. This is the Architect's probe at
  `evt_54y1jadrfk9eq`; it is D2 because it decides the scope of the rule, not
  because the combinators are suspect. **Restore `prelude.rs` before landing** —
  D2's deliverable is five numbers, not a prelude change.
- **D3 — contingent, and only on D1's result.** If the product-path margin is
  thin, replace the canary the `r3_4b` stack fix retires: one check that reddens
  when elaboration's peak crosses a stated fraction of the stated minimum.
  **If the margin is wide, do not build it** — say so, record the number, and
  close. A watch on a wide margin is ceremony.
- **D4 — state the minimum where the next author reads it.** A required-stack
  figure recorded at the elaborator's entry point, next to or in the
  `elab.rs:997` comment that already carries the budget. One sentence and a
  number. **Not** an external contract document, and not a published API
  guarantee — Ken has no embedders and this is for the fourteenth site's author.

## Acceptance criteria

- **AC-1 — D1 is a measurement, not an inference.** State the method and the
  observed peak in bytes for each of the two builds. *"It completed"* is not a
  peak; a bound derived from frame sizes is not an observation. If the chosen
  method cannot produce a peak, say which method you tried and stop — an
  unmeasured number stated as measured is worse than the current absence.
- **AC-2 — D2 reports four data points, one per added declaration, in the order
  added.** A single before/after does not distinguish shape from cumulative,
  which is the only thing D2 exists to decide. **Also report the order you added
  them in**: "the fourth one tips it" means nothing without knowing which was
  fourth.
- **AC-3 — the D3 decision is stated either way, with D1's number as its
  ground.** *"Margin is N bytes, threshold for building a watch was M, so
  built / not built."* A silent omission of D3 is indistinguishable from
  forgetting it.
- **AC-4 — no change to any combinator, to `check`/`infer` frame layout, or to
  any `stack_size` value in the tree.** This node observes. If the measurement
  makes a tuning change look urgent, that is a finding to report, not a
  deliverable to fold in.
- **AC-5 — no new red in CI.** Targeted locally: `-p ken-elaborator`, `-p
  ken-cli`. Never `--workspace` on the box.

## Contention

`crates/ken-elaborator/src/elab.rs` (D4's one comment) and whatever measurement
harness D1 needs. **`crates/ken-elaborator/src/prelude.rs` is touched only
transiently by D2** — the four declarations are added one at a time to take
readings and **the file must be restored before landing**; D2's deliverable is
four numbers, not a prelude change. `LANG-PRELUDE-COLLECTIONS` owns the real
prelude edit and must land first.

Language owns all of it. Runtime is in `crates/ken-runtime`; Verify's lane is
`src/prover.rs`.

## Not this node

- **Reducing per-frame cost on `check`/`infer`.** Real, separate, and it
  invalidates the number this node publishes if done in the same diff.
- **`RUST_MIN_STACK` or any global thread-stack change.**
- **Fixing the `r3_4b` worker.** That is `LANG-PRELUDE-COLLECTIONS`'s superseding
  tip.
- **Adding `Array`, `Map`/`Set`, lawful `DecEq`/`Ord`, or the combinator laws.**
  This node measures what they will cost. It does not deliver them, and it does
  not gate them — if the margin is wide, they proceed unchanged.
- **Publishing an external stack contract.** Ken has no embedders. D4 is for
  this repo's next author.
