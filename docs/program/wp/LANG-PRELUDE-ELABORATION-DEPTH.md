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

## D5, added 2026-08-14: the other currency a prelude addition is paid in

Adversary finding at `evt_60pwz0y927h6g`, **confirmed by a Steward read it named
as unrun** — and the read came back the way that makes it live rather than
nominal.

`LANG-PRELUDE-COLLECTIONS`'s `AC-5` asserts **per name** that `map`/`fold`/`zip`/
`filter` are each absent from `trusted_base()`. Its doc comment claims something
wider: *"the trusted base does not grow."* **Those are different claims, and an
entry added under a name other than the four satisfies every assertion.**

**That is not hypothetical.** `prover.rs:493-501`:

```rust
fn emit_unknown_hole(env: &mut GlobalEnv, phi_closed: &Term) -> Verdict {
    let hole_id = declare_postulate(env, "prover unknown goal".to_string(), ...)
```

⇒ **an undischarged obligation registers a `trusted_base()` entry named
`"prover unknown goal"`, not the name of the declaration that raised it.**
`bytes.rs` does the same under `"BytesRoundTripLaw"`. The paths that do use the
declared name are the recursive/mutual pre-admissions (`elab.rs:6752`, `:6846`,
`:7103`) and the `Axiom` sugar (`:1045`). **The environment holds both kinds and
a per-name check sees only one.**

**The four landed definitions add nothing in fact** — structural recursions
raising no obligation, so `AC-5` passes truthfully. The defect is the sentence a
later reader inherits, and **the population it will be read against is exactly
the obligation-bearing one**: `sort` is excluded for this precise reason, and
`Array`, `DecEq`/`Ord` and the laws are all still owed.

- **D5a — narrow the inherited claim.** Change that doc comment to say what the
  test establishes: *these four are transparent definitions, not postulates.*
  One sentence, same cost, and true.
- **D5b — assert the full `trusted_base()` enumeration from a bare
  `ElabEnv::new()`**, every entry by name. **Not a count and not a differential.**
  The original doc rejected a frozen *size* because a coincidental `+1/-1`
  elsewhere could mask a real regression — **a full enumeration has no such
  masking**, which is the whole difference between a count and an absolute
  matrix. A live differential is separately impossible here: post-landing the
  four are registered inside `ElabEnv::new()` itself, so there is no "before"
  environment to capture.
  **Its maintenance cost is the feature.** A prelude addition that changes the
  trusted base should be forced to say so explicitly, which is the property this
  program wants on the exact path `sort` and the lawful instances are queued on.
  **If the enumeration is large enough that a literal list is unreadable, say so
  with the count and stop** — that is a finding about the shape of the trusted
  base, not a reason to fall back to the per-name check.

**Why it lands here rather than as its own node.** This node already owns *what
does a prelude addition cost, and what watches it.* Stack depth and trusted-base
entries are that question in two currencies, and `D3`'s watch and `D5b`'s
enumeration are the same kind of instrument.

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
- **AC-6 — `D5b`'s enumeration reddens on a real addition, demonstrated.**
  Temporarily add one obligation-raising declaration to the prelude — `sort` is
  the natural witness, since its `is_sorted ∧ Perm` obligation is the exact
  mechanism — and show the enumeration failing with the new entry named in the
  diff. Restore before landing. **A membership assertion that has never been
  seen to fail is indistinguishable from one asserting the empty condition**,
  and this one exists precisely to catch an entry nobody predicted the name of.
- **AC-7 — `D5a` and `D5b` do not amend `AC-5`'s existing per-name assertions.**
  Adding is fine; the per-name check is the right shape and the Adversary
  explicitly did not ask for it to change. Changing a control while widening
  the claim it backs forfeits the control.

## Carried addendum: one comment line, unrelated to D1-D5

**This is not a deliverable and it gates no AC.** It is a one-line repair
riding this node's turn because it is too small to be a node of its own, and it
is listed separately so its diff is attributable rather than folded into a
measurement.

`LANG-TRIVIA-KIND-MAPPING-PIN` (`193d8944`) pins all four `CommentKind` arms,
but the `Line` arm is discharged by a fixture in **another file** —
`crates/ken-elaborator/tests/kenfmt_b1_lossless.rs:59` — whose discriminating
property is its **configuration**: same line after a declaration, with a
following declaration. Moving it onto its own line, dropping the following
`const b`, or rewriting it for an unrelated formatter reason silently falsifies
the other file's four-arm claim, and **nothing reds.** Architect at
`evt_7p1aw2pq52hmm`.

**Add one comment at that fixture** recording that its
same-line-after-with-a-following-declaration shape is load-bearing for
`LANG-TRIVIA-KIND-MAPPING-PIN`'s `Line` arm, and that a `Line`/`DocLine`
transposition is what it catches there. **Change no assertion and no fixture
text** — the shape is correct as it stands; only the reason it must stay that
shape is missing. If writing it truthfully requires touching the fixture
itself, stop and say so rather than adjusting it.

## Contention

`crates/ken-elaborator/src/elab.rs` (D4's one comment), whatever measurement
harness D1 needs, and — from `D5` — `crates/ken-elaborator/tests/lang_prelude_collections.rs`
for its `AC-5` doc sentence and the added enumeration. The carried addendum
touches `crates/ken-elaborator/tests/kenfmt_b1_lossless.rs`, comment only.

**`crates/ken-elaborator/src/prelude.rs` is touched only transiently**, by `D2`
and by `AC-6`, and **must be restored before landing** in both cases. `D2`'s
deliverable is five numbers and `AC-6`'s is a demonstrated red; neither is a
prelude change. `LANG-PRELUDE-COLLECTIONS` landed at `60b78c95`.

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
