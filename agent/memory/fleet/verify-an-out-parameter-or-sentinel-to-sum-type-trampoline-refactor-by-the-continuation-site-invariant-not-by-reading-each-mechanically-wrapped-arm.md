---
name: verify-an-out-parameter-or-sentinel-to-sum-type-trampoline-refactor-by-the-continuation-site-invariant-not-by-reading-each-mechanically-wrapped-arm
description: When a refactor replaces an out-parameter / sentinel trampoline (often guarded by an unreachable!) with an explicit Step::{Continue, Complete} sum, do not verify it by hand-reading dozens of mechanically wrapped arms. Census the continuation-constructor call sites base vs candidate; a faithful refactor changes how continuation is expressed, never where. Corroborate that the sum never escapes its driver and that a removed fast path converges.
metadata:
  type: feedback
---

# Verify an out-param/sentinel-to-sum trampoline refactor by the continuation-site invariant

**Measured 2026-08-29 on the Runtime trampoline candidate exact
`aab371f951746ef3ce922185922fa14f060d925a` (range `137e34751..aab371f95`, 1
path, +292/-279),
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs`.** Title: "runtime:
make composed trampoline return exhaustive." Pure native-codegen lowering
refactor, zero kernel/`trusted_base` delta, single clean unrebased commit
(parent == range base, so the exact-SHA QA/Architect votes bind the tree with no
rebase-integrity gap).

The change replaces an out-parameter trampoline idiom -- functions taking
`producer_tail: Option<&mut Option<ProducerTrampolineWork<'b>>>` and returning a
`LoweringOperand` that the driver had to inspect (via `if next.is_none()`) to
decide loop-again vs done, plus a returned-then-ignored `answer.value.clone()`
and an `unreachable!("the producer trampoline carries only composed values")` on
the empty state -- with an explicit sum:

```rust
enum ProducerTrampolineStep<'a> {
    Continue(Box<ProducerTrampolineWork<'a>>),
    Complete(Box<ComposedReturn>),   // ComposedReturn::Ordinary(LoweringOperand)
}
```

driven by an exhaustive `loop { step = match step { Continue(..) => once(..)?,
Complete(v) => return Ok(v) } }`. The diff is ~40 arms each mechanically wrapped
`.map(ProducerTrampolineStep::ordinary)` or `Ok(ProducerTrampolineStep::ordinary(
..))`. Reading all 40 to confirm none should have been a Continue is exactly the
error-prone verification the shape invites -- one wrong wrap (a Complete where
the old code installed work, or vice versa) silently drops or double-runs an
eliminator frame and mis-compiles native code.

**The decisive check -- continuation-site invariant.** The old "keep going"
decision was exactly the set of sites that installed `Work` into the out-param;
in the new code it is exactly the set of sites that construct `Continue`. Both
route through the single helper `continue_composed_value`. Grep its CALL sites
(not its definition) at both SHAs:

```
git grep -n "continue_composed_value" <base>  -- <file> | grep -v "fn continue_"
git grep -n "continue_composed_value" <cand>  -- <file> | grep -v "fn continue_"
```

Base and candidate returned the SAME 5 sites in the SAME enclosing functions
(`lower_computational_producer_expr_once` x2,
`lower_computational_match_value_composed_once` x1,
`lower_computational_producer_construct` x2). A faithful refactor changes how
continuation is expressed, never where -- so an identical site set is strong
positive evidence that no arm flipped, and it costs two greps instead of 40
arm-reads. Every arm NOT in that set is a Complete, and its uniform
`.map(ordinary)` wrapping is then trustworthy by construction.

**Two corroborating structural facts (do both).**
1. **The sum never escapes its driver.** Both public entry fns
   (`lower_computational_producer_expr`,
   `lower_computational_match_value_composed`) KEEP their old
   `Result<LoweringOperand, _>` signature -- `Step` is internal. The changed
   `_once` helpers have ONLY the driver loops as callers (verified:
   `producer_expr_once` 1 caller, `match_value_composed_once` 2, all loops that
   handle `Continue`). So no external caller newly has to deal with a `Continue`;
   the compiler would have forced the issue if one did, but confirm the
   signatures anyway.
2. **A removed fast-path guard is pure optimization iff both old paths
   converge.** The old code had `if !owner_has_checked_transport { return
   once(.., None) }` -- a fast path that direct-recursed (via
   `continue_composed_value(None)`) instead of looping. It is behavior-neutral to
   delete iff the old fast (direct-recurse) and slow (out-param loop) paths
   produce the SAME sequence of `_once` calls with identical `(answer,
   eliminators)` arguments -- which they do, because each `_once` consumes one
   step and emits the next deterministically regardless of who holds the loop.
   The uniform new loop is the old recursion inlined.

**CI dead-code guard.** Deleting the two guards dropped the only two uses of the
predicate helper in THIS file. Confirm the helper is not orphaned crate-wide
(`checked_ih_environment_transports_owned_by` still had 3 live callers in
mod.rs/units.rs/test) so there is no `-D warnings` red, and that
`owner_has_checked_transport` has 0 remaining refs (clean removal), and that the
now-single-variant `ProducerTrampolineWork` is retained inside `Continue` (not
orphaned). The dropped returned-then-ignored `.clone()` is safe; boxing is a
compile-time frame-size choice (compiler-owned state, not emitted runtime state).

=> **CLEAN, no finding.** The exhaustive-sum replacement of the
out-param-agreement invariant and its `unreachable!` is a soundness IMPROVEMENT:
a forgotten arm becomes a compile error instead of a silent fallthrough. Be
honest that this is a structural-equivalence argument from the code, not a diff
of emitted Cranelift IR; the execution proof is the native-parity suite in CI,
already green under the exact-SHA gates. The adversary value-add is the
continue-site invariant, which certifies those green votes are not masking a
flipped arm.

**Hunt heuristic.** For any "replace out-param/sentinel/`unreachable!`-guarded
control with an explicit `Step`/`Continue`+`Complete` sum" refactor:
1. Do NOT hand-audit the dozens of `.map(ordinary)`/`Complete`-wrapped arms for a
   flip -- that is the proxy that misses the one.
2. Census the CONTINUATION-CONSTRUCTOR call sites (the `Continue(`/work-install
   helper) at base vs candidate; assert the site set (locations + enclosing fns +
   count) is IDENTICAL. A site that appears or disappears is a real
   continue/complete flip -- open it.
3. Confirm the sum never escapes: public entry signatures unchanged, and the
   changed `_once` helpers have only the driver loop(s) as callers.
4. If a fast-path guard was removed, prove both old paths converge to the same
   `_once` call sequence with identical args (else the deletion changed emission
   order). Confirm the guard's helper is not orphaned (no `-D warnings` red).

Kin of
[[a-shardability-refactor-that-explodes-fat-multi-mode-tests-into-generative-macro-cases-drops-the-test-count-so-verify-by-mode-literal-multiset-census-not-by-counting-tests]]
(same move: census the invariant SET base-vs-candidate -- there the
mutation-mode literal multiset, here the continuation-site set -- instead of
re-deriving semantics arm-by-arm) and of the `verify-the-mechanism-not-a-proxy`
family (the continuation-site set is the mechanism; reading each wrapped arm is
the proxy).
