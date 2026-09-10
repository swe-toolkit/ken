---
id: LANG-CHECKED-IH-BODY-VIEW-CAUSE
title: "An ordinary binary-tree traversal does not compile natively, and the code discards the reason: compiler_driver.rs maps any failure of checked_core_declaration_body_view to MissingClosureMetadata with map_err(|_| ...), so the label is not a diagnosis. Surface the cause before sizing anything"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect evt_7msgce14888x4, 2026-08-16, ruling on the Steward's Q2 (evt_2cmabgypc18cq). Discovered as a side finding of RT-DESCENT-LANE-COMPLETENESS D5's two-recursive-position probe (runtime-implementer evt_6tveatdhcz72y). Ruled REAL on three grounds, none of them the error text. Steward-filed per COORDINATION section 2; QUEUED behind the operator's one-lane priority, lane 2 is quiet and this is not released."
---

> # COMPLETE 2026-09-10 — node MERGED at origin/main 9f91155d0
> # ("LANG-CHECKED-IH-BODY-VIEW-CAUSE: D0 dissolved, regression guard retained").
> # D0 OUTCOME A (Architect pre-committed procedure evt_7rw7e54drqq8f): the D5
> # two-recursive-position inorder-traversal probe COMPILES on current main and
> # runs (native artifact, terminal_error=None, exit_status=1 as reaching witness)
> # — no MissingClosureMetadata, no CheckedCoreBodyViewError. The 331db0a73 gap is
> # CLOSED: the interval work (candidate-closure specialization / the
> # RT-FNUNIT-MULTI-WORKER-CONTINUATION mirror) made the two-recursive-position
> # traversal reachable-and-lowerable. D0 dissolves — neither one case nor a class,
> # nothing to fix; SCT untouched, no census-site change. What landed is a durable
> # native regression guard ONLY: crates/ken-cli/tests/lang_checked_ih_body_view_cause.rs
> # (candidate 328b40b3, +153, sole path, mutation-proven) encoding the inorder
> # compiles+runs+exit-1 witness plus the three direct-recursion compile controls.
> # No production / TCB / census / SCT change. Gates on exact 328b40b3: Language QA
> # evt_795t98yvr2b82 + Architect respin evt_50htcap88147n; Decision dec_49dfdy6z63tvh
> # APPROVED. The surviving lesser compiler_driver.rs runtime-match-census
> # map_err(|_|) catch-all was correctly OUT of scope and untouched. NOTE for a
> # future reader: the body-view discard the original finding named was already
> # repaired independently (f9dd79f52, CheckedCoreBodyViewError is a discriminated
> # enum), and the gap itself then closed — a finding that dissolved from both ends.
> #
> # RELEASED 2026-09-10 (Steward) — the effective releasable language head.
> # LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN merged (888e6fe4) and
> # RT-MAPPING-MULTIOP-DISPATCH merged (1c48b6c5c), which clears the brief hold:
> # this node's subject file crates/ken-elaborator/src/compiler_driver.rs was in
> # RT-MAPPING's 17 changed paths, so releasing a language candidate before it
> # landed would have collided. It has landed. Operator L2 queue items 1-3
> # (pattern-forms literals / membership / deceq-char) are blocked or deferred; SCT
> # (item 4) merged; this is item 5, the next releasable head. status draft->active;
> # kicking the language ring.
> #
> # RE-MEASURED AT RELEASE — the D1 body-view site is ALREADY REPAIRED, D1 is
> # re-pointed to a probe re-run. The frame located the discard at
> # compiler_driver.rs:2013-2017 as `map_err(|_| MissingClosureMetadata)`. At the
> # current tree the checked_core_declaration_body_view call (now :2045-2050)
> # carries the cause: `map_err(|error| CheckedCoreBodyView { section, symbol,
> # error })` with error: CheckedCoreBodyViewError — landed independently by
> # f9dd79f52 (NATIVE-HANDLE-CARRIER), present since before RT-MAPPING's base. So
> # D1's "one line of plumbing to surface the cause" at the LOAD-BEARING site is
> # already done. What remains and defines the released first increment: RE-RUN the
> # D5 two-recursive-position inorder traversal probe at the current tree and
> # report the ACTUAL outcome — it now either compiles, or fails with the
> # now-carried CheckedCoreBodyViewError named (no longer "cause unknown"). Only
> # after that measurement returns does anyone size the gap or name a class. A
> # SECOND, lesser discard survives at the runtime-match-census site (now
> # :2051-2054, still `map_err(|_| MissingClosureMetadata)`); whether it matters is
> # exactly what the probe's carried cause tells you — do not fix it speculatively.
> # The ACTIVATION WARNING below stands unchanged. The D0 fork (is the remaining
> # gap one view case or a class) remains the ARCHITECT's call, routed to it; the
> # Architect authored the REAL-on-three-grounds ruling (evt_7msgce14888x4) and
> # should be told at pickup that the load-bearing discard it named is already
> # repaired.
> #
> # Original filing context below is preserved; the "no ring is released" line is
> # SUPERSEDED by this banner.

**Historical (SUPERSEDED by the banner above): this node was FILED and QUEUED.**
Lane 2 was quiet under the earlier one-lane directive; the finding was recorded,
not dispatched.

## The finding

**An ordinary recursive traversal over a binary tree does not compile through
`ken native-build`.** Measured at `331db0a73` by the runtime ring as a side
effect of [[RT-DESCENT-LANE-COMPLETENESS]]'s `D5` probe:

```
Driver(MissingClosureMetadata {
  section: "checked computational IH authoritative runtime body",
  symbol: StableSymbol { ... ["d5-two-recursive-position", "inorder"] } })
```

## Why it is REAL, on three grounds and none of them the error text

**Architect `evt_7msgce14888x4`.**

1. **A positive control forecloses the innocent reading.** The same prelude and
   checked `Program I main`, declaring
   `data D5Tree = D5Leaf | D5Node D5Tree Nat D5Tree`, **built successfully and
   selected `FunctionizedUnits`.** So this is not *"Ken does not do binary
   trees"* — the declaration is admitted; the traversal is what fails.
2. **The failure is a `CompilerDriverError`** — an internal-structure error
   naming a compiler section. On the same discriminator that settled
   [[RT-DESCENT-LANE-COMPLETENESS]]'s `D1`, **that is the compiler's own
   bookkeeping, not a claim about the program's denotation.** Nothing anywhere
   claims an inorder traversal has no meaning.
3. **Nothing declares it.** There is no `KNOWN-GAP.md` in the Rosetta corpus,
   and `oracle_for` forbids a silent skip, so **no artifact records this as
   expected native behaviour.**

## THE NAME WAS NOT A DIAGNOSIS — and the load-bearing site is now repaired.

As filed, the body-view site discarded the cause at
`compiler_driver.rs:2013-2017` with `map_err(|_| MissingClosureMetadata)`. That
site has since been repaired (independently, by `f9dd79f52` NATIVE-HANDLE-CARRIER
— present since before RT-MAPPING's base). At the current tree (re-measured at
release) it reads, at `:2045-2050`:

```rust
checked_core_declaration_body_view(checked_package, body_view_selection, owner)
    .map_err(|error| CompilerDriverError::CheckedCoreBodyView {
        section: "checked computational IH authoritative runtime body",
        symbol: owner.clone(),
        error,
    })?;
```

**The cause is now CARRIED** (`error: CheckedCoreBodyViewError`), not stamped
with a catch-all. So D1's one-line-of-plumbing at the load-bearing site is
already done — see the RELEASED banner. **A SECOND, lesser discard survives**
immediately below at the runtime-match-census site, `:2051-2054`:

```rust
let census = crate::checked_core::checked_runtime_match_census(&runtime_body.body)
    .map_err(|_| CompilerDriverError::MissingClosureMetadata {
        section: "checked computational IH authoritative runtime match census",
        symbol: owner.clone(),
    })?;
```

⇒ **The measurement located WHERE; the body-view code now surfaces WHY. The
released increment is to re-run the probe and read the carried cause, not to
re-plumb a site that already carries it.**

## Deliverable D1 (re-pointed at release): re-run the probe, read the carried cause.

The body-view site already surfaces the cause (see the banner). So the first
increment is no longer plumbing — it is measurement:

**Re-run the D5 two-recursive-position inorder traversal probe at the current
tree and report the ACTUAL outcome.** It now either (a) compiles — in which case
the gap has closed and this node is closeable — or (b) fails with the carried
`CheckedCoreBodyViewError` NAMED. Report which, and if (b), report exactly what
the carried error says.

**Do NOT fix the traversal, size the gap, name a class, cut a scoped successor,
or touch the surviving census-site `map_err(|_|)` before this returns.**
Architect, explicitly: whether this is one missing view case or a class is
**exactly what the now-carried error would tell you**, and guessing between
those is what would inflate the node. The one remaining plumbing question — does
the second (census) discard also need carrying — is answered by what the probe's
carried cause shows, not up front.

**The honest headline until then:** *an ordinary binary-tree fold was measured
not to compile natively at `331db0a73`; the cause is now carried in the code, so
the released step is to re-run the probe and read it.*

## The discriminating experiment, recorded so it is a RE-RUN and not a re-derivation

Half of it is already built and it should not be rebuilt from scratch:

- **Already established:** the two-recursive-position declaration is admitted
  and selects `FunctionizedUnits` (the positive control above).
- **Still missing:** a **traversal** over that type that reaches lowering.

## ACTIVATION WARNING: closing this makes an UNTESTED runtime guard reachable

> **Whoever closes this gap must read this before assuming the change is
> contained.**
>
> This failure currently **intercepts** every source program that would carry a
> two-recursive-position traversal into native lowering. Because of that,
> [[RT-DESCENT-LANE-COMPLETENESS]]'s construct 3 — the backend `Module` refusal
> for a recursive position the continuation specialization **projects no worker
> for** — has **UNDETERMINED source-reachability rather than none.**
>
> ⇒ **Closing this gap may make that guard reachable from real source for the
> first time.** Before assuming your change is contained, re-read the no-worker
> guard in `crates/ken-runtime/src/cranelift_backend/lowering/units.rs` (find it
> by its message, *"projects no worker for, so its induction-hypothesis prefix
> cannot be built"*) and [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]], which carries
> the mirror of this warning.

**Why this copy is the load-bearing one** (Architect, Q1): a dependency
recorded only on the dormant runtime node is **invisible to the one actor whose
change makes it live.** Whoever closes the elaborator gap reads *their own*
node and has no reason to open the runtime one. **The activating diff is in a
different crate, which makes the gap worse, not better.**

## Related

**Third instance of one defect family in this campaign** — the sentinel's
discarded `_excluded_result`, the two `control.rs` trace helpers repaired by
[[RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE]], and now a `map_err` that drops its
cause. **Three crates, and each one cost a measurement its explanation.**
