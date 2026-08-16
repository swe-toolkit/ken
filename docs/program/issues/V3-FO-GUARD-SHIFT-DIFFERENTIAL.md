---
id: V3-FO-GUARD-SHIFT-DIFFERENTIAL
title: "Pin mentions_var0 against a shift-built oracle so the duplicated binder discipline cannot drift silently"
status: merged
owner: language
size: S
gate: none
depends_on: [V3-FO-QUOTE-GUARD-FAIL-CLOSED]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/2371
origin: "Steward, 2026-08-15, dispositioning Adversary hunt evt_4vnyb89s5ameq on the merged range 8fe2264c7...4674fe840. Closes the residual the Architect named in evt_1y00bx8za2532 and declined to require in that candidate. Steward-filed per COORDINATION section 2."
---

## The residual, named independently by two reviewers

`V3-FO-QUOTE-GUARD-FAIL-CLOSED` made `mentions_var0` exhaustive over `Term` with
no wildcard arm. **That protects against a NEW `Term` variant — it is a compile
error. It does nothing about an EXISTING variant changing binder status.**

`mentions_var0` now encodes `ken_kernel::shift`'s binder discipline **a second
time, in a second file**, and the `subst.rs` citations in its doc comment are
documentation, not a constraint. If `Pair` became a binder, or `Let` gained a
binding position, `mentions_var0` would **still compile** and be silently wrong
**in the false-negative direction** — the unsound direction, and the exact one
`D0` just fixed.

> **This is one shape, and it appeared three times in two lanes on 2026-08-15:
> two independent derivations of a single key with nothing proving they agree.**
> It is conjunct 4 of [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]]'s `D0` ruling,
> and it is what broke [[RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE]] the same
> morning. **The node exists because the third occurrence came with a way to
> close it.**

## The oracle, and why it is exact rather than merely another check

```
mentions_var0(t)  ⟺  shift(shift(t, -1, 0), 1, 0) != t
```

**Down-shift at cutoff 0:** a free `Var(0)` hits the underflow guard and
**stays** `Var(0)`; every other free `Var(i)` becomes `Var(i-1)`. **Up-shift:**
each `Var(i-1)` returns to `Var(i)`, but the stayed `Var(0)` becomes `Var(1)`.

⇒ **The round trip is the identity iff no free `Var(0)` occurs.**

Under binders it still holds: `shift` raises its own cutoff, so a bound `Var(0)`
is untouched in both directions, and a `Var(1)` under one binder round-trips to
`Var(0)` and differs — correctly, because it **is** a reference to the outer
index 0.

> **The property that makes this worth a node rather than a better test: the
> oracle is built from the exact function whose discipline must be matched, so
> it cannot disagree with it.** If `Pair` becomes a binder, the oracle tracks it
> automatically. The drift becomes **structurally impossible** rather than
> documented.

## Deliverables

**`D0` — the differential test.** Assert `mentions_var0(t)` agrees with the
round trip over **one term per `Term` variant per subterm position**. Not a
sample: the point is coverage of every position where the two enumerations could
disagree.

**`D1` — the binder positions are covered explicitly.** `shift`'s only
`cutoff + 1` arms are `Pi.b`, `Lam.t`, `Sigma.b`, `Let.body`. **The test must
include a case that distinguishes each**, and a `Pair` case, since `Pair` sitting
with the binders is the mistake that produced the parent node.

**`D2` — record the dependency at the test site.** The oracle relies on the
underflow guard leaving `Var(0)` unchanged — the semantics `D3` of the parent
node documented in `subst.rs`. Say so where the test lives, so a future change
to that guard is understood to break this test **by design**.

## Acceptance criteria

**`AC-1`.** The test **fails** against a `mentions_var0` with `Pair` restored to
the binder group. **Demonstrate it**, do not argue it — this is the mutation the
node exists to catch.

**`AC-2`.** `mentions_var0`'s body is **not** replaced by the round trip. The
traversal stays; the oracle is a test.

> **This is a legibility ruling and it is deliberate.** Replacing the body would
> also remove the duplication, and it is the obvious move. The traversal is
> readable and the round trip is a trick; a reader who must decide whether the
> guard is right is better served by the traversal plus a test that pins it than
> by a clever equivalence with a comment explaining why it works. **Adversary's
> own recommendation, and it is the right call.**

**`AC-3`.** No behavior change. This is a test-only node.

**`AC-4`.** No `proved` for FO, no slice widening, no new kernel primitive or
trusted axiom.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Rewriting `mentions_var0`.** See `AC-2`.
- **Changing `shift`.** It is trust-root; this node reads it and does not touch
  it.
- **Signature discovery** and **sort validation** —
  [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]] and
  [[CORE-FO-CHECK-TREE-SORT-VALIDATION]].

## Sequencing

**Independent of `D1`-`D3` of the signature node** and cheaper than either, so
it need not wait. It is size `S` and its value is highest before route FO goes
live, on the same cost argument that put the parent node ahead of `D1`-`D3`.

## Provenance

Adversary hunt `evt_4vnyb89s5ameq` on the merged range
`8fe2264c7...4674fe840`, read-only, which re-derived the landed repair
arm-by-arm and supplied the oracle. Architect review `evt_1y00bx8za2532` named
the same residual and explicitly did not require it in that candidate:
*"if there is ever a cheap way to derive one traversal from the other rather
than mirror it, that is the durable fix."* **There is, and this is it.**

The Adversary's `shift` claims (four `cutoff + 1` arms; `Elim`'s `..` covering
only `fam` and `level_args`) are theirs and were re-derived by them against the
tree. **The implementer should confirm them at the point of use**, per the
standing rule — and note that the same hunt retracts an earlier claim of its own
that this node's parent had transcribed into a deliverable.
