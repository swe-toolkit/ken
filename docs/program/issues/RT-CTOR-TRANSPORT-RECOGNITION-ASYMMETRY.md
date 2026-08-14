---
id: RT-CTOR-TRANSPORT-RECOGNITION-ASYMMETRY
title: "Why constructor 26 field 0's worker transport is not on the recognized-transport path that constructor 36's is, and whether the route repair is what diverts it"
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-DEPTH2-VAR-PARENT-DERIVATION]
blocks: []
github: null
origin: "Architect mechanism ruling evt_57atrppgx2exe (2026-08-14) on RT-DEPTH2-VAR-PARENT-DERIVATION's measurement. Branch (b) selected, the route fork DISSOLVED rather than decided, and this asymmetry named as the successor. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> # THE FORK IS DISSOLVED, NOT DECIDED. NEITHER BANNED SURFACE IS ESTABLISHED.
>
> **Do not arrive here looking for a vehicle.** The predecessor's measurement
> settled that the depth-2/3 residual does **not** demand
> `ContinuationTemplate` population or continuation-source projection. The
> question *"which banned surface does this residual demand?"* has no answer,
> because the residual does not demand one.
>
> **Flip to `ready` when [[RT-DEPTH2-VAR-PARENT-DERIVATION]] is `merged`.**

## What the predecessor measured, and what it settled

| field | value |
|---|---|
| `parent_arm` | `RuntimeExpr::Construct` |
| `parent_origin` | `StaticOriginId(26)` |
| `parent_construct` | `ctor:fixture::PX8JScopeTree::Node` |
| child position 0 | the measured `Var(0)` at origin 25 |
| demanding arm | `core.rs:7494` |

- **(a) refuted** — not a call whose callee slot lost the exact-`Var` shape.
  There is no callee here at all.
- **(c) refuted** — an ordinary constructor argument in the source machine, not
  a continuation-source surface that only a projection could rebind.
- **(b) selected** — a genuine value demand in a constructor field.

⇒ **That is exactly what `close()` already legislates** (`lowering/mod.rs:4607`):
a static worker in a constructor field must be *erased before construction* or
*consumed at an exact-`Var` call*. **The need is not novel and does not require
a new surface to state.**

⇒ **Row 4 deep and row 5 are ONE need** — both terminate in a worker-bearing
constructor field with nothing rebinding it. The Architect ruled they share a
need **while keeping their evidence separately reported**: probe stop condition
4 forbids averaging the *measurements*, not recognizing a common root once
measured.

## The Architect corrected his own flag, and the correction matters here

The predecessor's frame led with his `needs confirmation` on whether the
depth-2/3 boundary had **changed class** since `D2k-1c`, which would have made
its banned-surface attribution stale. **With the parent known, that reading does
not hold.** `D2k-1c` said *"a further worker-bearing-constructor boundary for
depths 2/3"* — and the parent of the failing `Var` **is** a worker-bearing
constructor. **Same boundary, reported at a different enforcement point.** The
attribution was not stale in substance.

**What survives, and it is the narrower and more useful claim:** `value_at` and
`close()` are two different laws, and **reading the refusal at the pointwise one
is what made the constructor parent invisible.** That distinction is what
produced the derivation. The disposition now rests on *the need is uniform*
rather than on *the premise is stale*.

> **Do not restate the retired framing.** A node that leads with "the inherited
> premise is stale" is repeating a claim its own author withdrew.

## THE TWO FACTS BELOW COME FROM TWO DIFFERENT TREES. DO NOT FUSE THEM.

**The Architect named this as the piece most likely to be lost, and it is the
first thing to check in any report this node produces.**

| fact | tree |
|---|---|
| origin 25 refuses at `value_at`, `PredeclaredFunctionId(0)` | the **route-applied** tree (the earlier probe) |
| origin 25's parent is `Construct` origin 26, and the compile ends at the conservation refusal for constructor **36** | **base `f26167e22`**, no route repair |

**On the base tree the depth-2 compile does not stop at origin 25 at all — it
stops at constructor 36's conservation refusal.** The `value_at` stop at origin
25 appears **only once the route is re-applied.**

⇒ **Anyone writing "origin 25 fails at `value_at` and its parent is a
`Construct`" as one sentence has fused two tree states.** Name the tree beside
every observation in this node's report. This is the same class as the
lane's standing error — measure in one tree or at one level, then infer about
another.

## The question

**The same depth-2 compile contains both shapes:**

- constructor **36**'s worker transport is **recognized** and reaches `close()`;
- constructor **26 field 0**'s is **not**, and on the route-applied tree dies at
  a raw value read instead.

**Why is constructor 26 field 0's worker transport not on the
recognized-transport path that constructor 36's is — and is the route repair
what diverts it there?**

## The two outcomes, and what each implies

- **Recognition *should* cover it.** Then this is a **recognition gap**, the fix
  lives in recognition, **neither banned surface is touched**, and **both
  residuals close together** — row 4 deep and row 5, since they are one need.
- **Recognition deliberately excludes it.** Then **that exclusion's reason is
  the real constraint**, and only then does a vehicle question arise at all —
  on a properly stated need rather than an inferred one.

## Deliverable

**D1.** Determine which of the two outcomes holds, and report the evidence with
the tree named beside each observation.

## Acceptance criteria

**AC-1.** Every reported observation names the tree it was taken on — base
`f26167e22` or route-applied. A report that states a `value_at` fact and a
parentage fact without distinguishing them does not satisfy this, **even if
both facts are individually true**.

**AC-2.** The recognized/not-recognized asymmetry between constructor 36 and
constructor 26 field 0 is reported from the mechanism that decides recognition,
cited at `file:line`, not inferred from the refusal that follows it.

**AC-3.** No branch of the dissolved fork is taken. `ContinuationTemplate` is
not populated and no continuation-source projection surface is added.

**AC-4.** If the answer is "recognition deliberately excludes it", the
exclusion's **reason** is reported — that reason is the deliverable, not the
fact of exclusion.

**AC-5.** No repair is retained beyond what the outcome requires. Blob identity
on any file this node does not intend to change.

## Banned scope

- **Neither banned surface**, on this evidence. **If this node reaches a point
  where one is genuinely required, it returns to the Architect with the need
  surfaced and the vehicle open** — exactly as its predecessor did, which is
  the reason the fork could be dissolved instead of guessed.
- Do not average rows 4 and 5's measurements. They share a need; their evidence
  stays separately reported.

## How the predecessor earned this disposition

Recorded because it is the standard for the report this node owes. It answered
the question asked and **explicitly selected none of (a)/(b)/(c)** although (b)
was legible from its own data — which is what let the Architect rule on the
measurement rather than on the ring's reading of it. It **quoted the `D2k-1c`
record verbatim** instead of paraphrasing, which is the only reason he could
check his own staleness claim and find it wrong. It reported the
origin-35/constructor-36 event **separately** and refused to substitute it. And
it reported its **failed first probe** — the privacy error, and that a filter
was replaced with raw reporting of every index-0 occurrence — which is what
makes the reported origin trustworthy rather than selected.
