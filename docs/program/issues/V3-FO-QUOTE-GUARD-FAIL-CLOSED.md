---
id: V3-FO-QUOTE-GUARD-FAIL-CLOSED
title: "Make quote_fo's pre-quotation guards fail closed: Pair is not a binder, and a proof-variable-occurrence test must default to true"
status: merged
owner: language
size: S
gate: none
depends_on: [V3-FO-KRIPKE-SLICE]
blocks: []
github: 2346
origin: "Steward, 2026-08-15, dispositioning Adversary finding evt_235vyn7za92ry on the merged range 75a91d2ba...6ec9694fa. Filed as its own node rather than folded into V3-FO-OBLIGATION-SIGNATURE-DISCOVERY, which is already released and is about a different surface. Steward-filed per COORDINATION section 2."
---

## `Pair` is classified as a binder. The kernel says twice that it is not.

`mentions_var0` in `fo_kripke.rs`:

```rust
Term::Pi(a, b) | Term::Lam(a, b) | Term::Sigma(a, b) | Term::Pair(a, b)
    => go(a, depth) || go(b, depth + 1),
```

`ken_kernel::subst` on the same constructor:

```
subst.rs:36   Sigma(a, b) => sigma(shift(a, d, cutoff), shift(b, d, cutoff + 1))   // binder
subst.rs:44   Pair (a, b) => pair (shift(a, d, cutoff), shift(b, d, cutoff))       // NOT
subst.rs:147  Pair (a, b) => pair (subst_var(a, j, u), subst_var(b, j, u))         // NOT
```

⇒ **`Pair` is a term former, not a binder**, and grouping it with `Sigma` makes
the guard wrong **in both directions**:

| input | `mentions_var0` says | truth |
|---|---|---|
| `Pair(_, Var(0))` — the proof variable | **false** | it IS mentioned |
| `Pair(_, Var(1))` | **true** | it is not |

**The false positive is safe** — an extra `DependentProofUse` refusal. **The
false negative is the unsound direction:** it certifies the proof hypothesis
absent, and `shift(codomain, -1, 0)` then runs on a term that references it.

## The same root, wider: `_ => false` is fail-open

`Elim`, `Let`, `Ascript`, `Refl`, `Cast`, `J`, `Quot`, `QuotClass`, `QuotElim`,
and `Absurd` all carry term subterms and all fall to `_ => false`, so a `Var(0)`
inside any of them reads as absent.

> **A guard answering *"does the proof variable appear?"* must default to
> `true`.** Conservative `true` yields `Err(DependentProofUse)` — a refusal, the
> safe direction. `false` yields an erasure.

## THE STRUCTURAL DEFECT, which is the part that is not one line

`quote_iform`'s `_ => Err(UnsupportedTermShape)` **is** a construction-level
refusal, and out-of-slice shapes genuinely cannot survive it. **But
`mentions_var0` and `shift` run BEFORE that refusal, on unvalidated input.**

What makes them safe today is that `quote_iform`'s accepted grammar happens to be
a **subset** of the constructors `mentions_var0` traverses.

⇒ **Two enumerations, in two functions, maintained separately, with nothing tying
them together.** The slice is explicitly built to grow — `top`/`and`/`exists` and
a real atom theory are on its roadmap — and **the first accepted shape that
`mentions_var0` does not traverse turns a latent bug into a live one.**

**Fixing only the two arms leaves the coupling unfixed.** Prefer a structure where
adding an accepted shape cannot silently outrun the guard.

## Severity, stated without inflation

**Latent. There is no current soundness exposure.** Route FO is unreachable in
production, `23 §4.4` forbids `proved`, and both exits converge on
`emit_unknown_hole`. The `Pair` misclassification is **already wrong today and
merely unreachable.**

This is a defect in a guard inside a slice built to grow, filed so it is fixed
while it is cheap rather than when it is load-bearing.

> ### THE LATENCY HAS A SHELF LIFE, AND IT EXPIRES ON THE NEXT NODE'S `D1`-`D3`
>
> **Steward sequencing decision, 2026-08-15.** This node goes **ahead of**
> `V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` `D1`-`D3`, reversing the earlier note
> that it must not jump that node.
>
> **The reason the severity is "latent" is exactly what `D1`-`D3` remove.**
> Nothing reaches `quote_fo` in production today; `D1`-`D3` are the work that
> makes route FO reachable, and `mentions_var0` runs **before** `quote_iform`'s
> refusal, on unvalidated input. **Landing them first arms a guard already known
> to be fail-open.**
>
> **This is not a claim that the bug becomes dangerous.** With `AC-3` holding,
> route FO still cannot return `proved`, so a bad guard yields a wrong quoted
> formula and an `Unknown` verdict, not an unsound one. **The argument is cost,
> not danger:** this node is size `S`, the ordering is free while `D1`-`D3` have
> not started, and it stops being free afterwards.
>
> **The general shape, which outlives this instance:** a severity of "latent,
> unreachable" is a claim about the tree at a moment, and the node that makes it
> reachable is usually already scheduled. **Ask what would arm a latent defect and
> when that work lands, rather than filing the latency as if it were durable.**

## MERGED 2026-08-15 at `142cf2336`. All four deliverables landed.

Candidate exact `4674fe84044f473d78e78779e9a0773b085e4d67`, base
`8fe2264c70c90f7cb99d71f96c0f735de8aab407`, two commits, `+191/-10`, PR #2346.
Decision `dec_1yc4z7td2a9zw`, resolved by the Architect on that exact SHA.
Blob-verified 2/2 from the declared base.

**`D1` delivered more than the frame asked for.** The frame said "prefer
exhaustive matching over a wildcard". The candidate has **zero `_ =>` arms**,
with the six term-free leaves enumerated individually and returning `false` as
an exact structural fact. **A runtime "unknown means true" default — the obvious
fail-closed design — would have been weaker:** it keeps compiling while being
wrong about a new case.

**`D3` was corrected once under a QA block, and that is the substantive history
of this node.** The first attempt attributed `ForallRight`'s safety to
eigenparameter freshness. QA did not argue with it — they built a literal
checker probe and ran it, and `check_cert` returned `true` on a formula with a
world eigenparameter in an object slot. Architect ruling `evt_71g1xf5vkf1ek`
supplied the true mechanism, which is recorded at the site and, as a constraint
on unstarted work, as `AC-4a` on [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]].

## Deliverables

**`D0` — `Pair` is not a binder.** Traverse both subterms at the same depth,
matching `subst.rs:44`.

**`D1` — the default becomes `true`.** No constructor carrying term subterms may
reach a `false` default. Prefer exhaustive matching over a wildcard, so a new
kernel constructor is a compile error rather than a silent fail-open.

**`D2` — tie the two enumerations together, or state why they cannot be.** A
recorded mechanism explaining why the coupling is not expressible is a complete
deliverable; a silent restatement of the status quo is not.

> ### WHAT `D2` ACTUALLY BOUGHT, AND THE HALF IT DOES NOT COVER
>
> **Architect review `evt_1y00bx8za2532`, approving exact `4674fe840`. Recorded
> as a known coupling at his direction; explicitly NOT asked of this candidate.**
>
> **The delivered mechanism is compile-time exhaustiveness, and it is stronger
> than what was asked for.** No wildcard arm; the six term-free leaves are
> enumerated individually and return `false` as an exact structural fact rather
> than as a default. A runtime "unknown means true" default would have been the
> obvious fail-closed design and would have been **weaker** — it keeps compiling
> while being wrong about the new case.
>
> **The half it does not cover.** `mentions_var0` now encodes `shift`'s binder
> discipline a **second time, in a second file**, and nothing enforces that the
> two agree. The `subst.rs:44`/`:147` citations are documentation, not a
> constraint.
>
> ⇒ **Exhaustiveness protects against a NEW `Term` variant. It does not protect
> against an EXISTING variant changing binder status.** If `Pair` ever became a
> binder, or a new binding position were added to `Let`, `mentions_var0` would
> **still compile** and would be silently wrong **in the false-negative
> direction** — the same direction `D0` just fixed.
>
> **This is the shape that broke lane 1 the same day**
> (`RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE`), and it is conjunct 4 of the `D0`
> ruling on [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]]: **two independent
> derivations of one key, with nothing proving they agree.** Three occurrences
> in two lanes on 2026-08-15.
>
> **The durable fix, if one is ever cheap: derive one traversal from the other
> rather than mirror it.**
>
> ### RESOLVED 2026-08-15, AFTER THE MERGE: THE FIX IS CHEAP AND IT IS EXACT.
>
> **Adversary hunt `evt_4vnyb89s5ameq` on the landed range supplied an oracle
> built from `shift` itself:**
>
> ```
> mentions_var0(t)  ⟺  shift(shift(t, -1, 0), 1, 0) != t
> ```
>
> **Why it is exact.** Down-shift at cutoff 0 leaves a free `Var(0)` unchanged
> at the underflow guard while every other free `Var(i)` becomes `Var(i-1)`; the
> up-shift restores each `Var(i-1)` but sends the stayed `Var(0)` to `Var(1)`.
> The round trip is the identity **iff** no free `Var(0)` occurs. Under binders
> `shift` raises its own cutoff, so a bound `Var(0)` is untouched both ways.
>
> ⇒ **The oracle is built from the exact function whose discipline must be
> matched, so it cannot disagree with it.** If `Pair` became a binder, the
> oracle tracks it automatically — **the drift becomes structurally impossible
> rather than documented.**
>
> **Take it as a DIFFERENTIAL TEST, not as a rewrite.** Replacing the body with
> the round trip removes the duplication and costs legibility — the traversal is
> readable and the round trip is a trick. The test converts *"nothing enforces
> that the two agree"* into something that reds.
>
> **One dependency, and it fails in the right direction:** the oracle relies on
> the underflow guard leaving `Var(0)` unchanged — the exact semantics `D3` just
> documented. If that guard changed to panic or wrap, the differential test
> breaks **loudly**.
>
> Carried to [[V3-FO-GUARD-SHIFT-DIFFERENTIAL]].

**`D3` — two comment corrections where the conclusion holds and the stated reason
does not.** Both were checked and both conclusions stand; only the justifications
are wrong, and a later author will rely on them.

- **`shift`'s underflow guard prevents corruption, not capture.** `Var(0)` at
  `d = -1, cutoff = 0` is left unchanged, so no panic and no wrapped index — but
  the un-shifted `Var(0)` aliases whatever the next enclosing binder becomes once
  the other indices shift down. **Fail-safe at the representation level, not the
  semantic one.** Say so at the site.
- **The `ForallRight` sort collapse is not exploitable, but its stated reason is
  self-contradicting.** The module doc justifies the absent `QSort` by *"every
  relation and quantifier already fixes which sort each slot is"* — and
  `ForallRight` is the one site that consumes a quantifier **without recording
  which**. **That is the sentence a second sort would be added against.**

  > **CORRECTED 2026-08-15, AFTER THIS NODE MERGED. This bullet named the wrong
  > replacement mechanism, and the wrong one was mine to have shipped.**
  >
  > It read: *"Freshness plus `Init`'s syntactic equality is what actually
  > blocks confusion."* **That is false.** Neither freshness nor `Init` inspects
  > sorts at all. Language QA refuted it with a literal checker probe —
  > `check_cert` returns `true` on a hand-built ill-sorted `Form`, and `Init` is
  > exactly what accepts it, because syntactic equality is all `Init` needs.
  >
  > **The true mechanism is caller-side, not checker-side:** `Form` is strictly
  > larger than the image of `embed` on `IForm Sigma`, and the malformed formula
  > lives entirely in that excess. Architect ruling `evt_71g1xf5vkf1ek`; landed
  > in the `D3` recut and in `AC-4a` on
  > [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]].
  >
  > **Provenance of the error, because it is the reusable part.** The claim came
  > from Adversary finding `evt_235vyn7za92ry`, I transcribed it into this
  > deliverable, and the implementer built the comment to my wording. **Its
  > author retracted it unprompted** (`evt_4vnyb89s5ameq`): *"my conclusion held
  > and my reason did not, and the reason is what got written down."* A frame
  > that transcribes a cited reason inherits that reason's defects, and **the
  > conclusion surviving is what makes the wrong reason cheap to keep.**

## Acceptance criteria

**`AC-1`.** `D0` and `D1` are each demonstrated by a test that **fails before the
fix** — a `Pair(_, Var(0))` case and at least one wildcard-defaulted constructor
carrying `Var(0)`.

**`AC-2`.** No accepted `IForm` shape is traversable by `quote_iform` but not by
the occurrence guard. **Demonstrate the property, not the current instance** —
the point is that growth cannot outrun it.

**`AC-3`.** No behavior change for any obligation the slice accepts today. This
is a guard correction, not a widening.

**`AC-4`.** `proved` is still not returned for FO, and the slice is not widened.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Widening the slice.** `top`/`and`/`exists` and a real atom theory are later
  nodes; this node makes the guard safe **for** that growth, it does not perform
  it.
- **Signature discovery.** [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]].
- **The theorem home and the evaluator posture.** Neither is touched here.

## Provenance

Adversary finding `evt_235vyn7za92ry`, hunt on `75a91d2ba...6ec9694fa`,
read-only. The kernel citations (`subst.rs:36`, `:44`, `:147`) are the Adversary's
and are the basis of the `Pair` claim; **the implementer should confirm them
against the tree before building**, per the standing rule that a cited line is
verified at the point of use.

The Adversary also recorded three things as checked and sound, so a later pass
need not re-run them: `shift` is total, `IndFormer`/`Constructor` carry no term
subterms, and `check_tree`'s three rules enforce linearity rather than assuming
it.
