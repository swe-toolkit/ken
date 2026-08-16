---
id: LANG-CHECKED-IH-BODY-VIEW-CAUSE
title: "An ordinary binary-tree traversal does not compile natively, and the code discards the reason: compiler_driver.rs maps any failure of checked_core_declaration_body_view to MissingClosureMetadata with map_err(|_| ...), so the label is not a diagnosis. Surface the cause before sizing anything"
status: draft
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect evt_7msgce14888x4, 2026-08-16, ruling on the Steward's Q2 (evt_2cmabgypc18cq). Discovered as a side finding of RT-DESCENT-LANE-COMPLETENESS D5's two-recursive-position probe (runtime-implementer evt_6tveatdhcz72y). Ruled REAL on three grounds, none of them the error text. Steward-filed per COORDINATION section 2; QUEUED behind the operator's one-lane priority, lane 2 is quiet and this is not released."
---

**This node is FILED and QUEUED. No ring is released on it.** Lane 2
(language + verify) is quiet under the operator's one-lane directive; this is a
recorded finding, not a dispatch.

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

## THE NAME IS NOT A DIAGNOSIS. The site throws the real error away.

At `crates/ken-elaborator/src/compiler_driver.rs:2013-2017`, verified in the
tree by the Steward:

```rust
checked_core_declaration_body_view(checked_package, body_view_selection, owner)
    .map_err(|_| CompilerDriverError::MissingClosureMetadata {
        section: "checked computational IH authoritative runtime body",
        symbol: owner.clone(),
    })?;
```

**`map_err(|_| ...)` discards the cause.** `MissingClosureMetadata` is a
**catch-all applied regardless of why the view failed** — it is the label the
site stamps on everything, not a finding. **The same pattern repeats
immediately below at `:2019`** for the runtime match census.

⇒ **The measurement located WHERE and the code discarded WHY.**

## Deliverable D1: surface the cause. That is the whole first increment.

**Replace the discarding `map_err` at both sites so the underlying error is
carried, then re-run the probe and report what it actually says.**

**Do NOT fix the traversal, size the gap, name a class, or cut a scoped
successor before this returns.** Architect, explicitly: the first step is one
line of plumbing, and **it may turn a mysterious metadata failure into an
ordinary named refusal.** Whether this is one missing view case or a class is
**exactly what the discarded error would tell you**, and guessing between those
is what would inflate the node.

**The honest headline until then:** *an ordinary binary-tree fold does not
compile natively, cause unknown.* One measured program, unknown cause.

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
