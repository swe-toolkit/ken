---
name: compose-your-own-measurements-against-the-artifacts-relational-claims
description: Attacking one sentence of an artifact and clearing it is not auditing the artifact — my two measurements (guard present, capability absent) already refuted an adjacent sentence in the same bullet asserting a relation that needs both, and I reported the row clean
---

# Compose your own measurements against the artifact's relational claims

**Measured 2026-08-10. My finding on `fad92a1b` was confirmed and repaired
(`4911eb22`), but the repair needed a recut for a defect I had the evidence to
call and did not.**

I reported two measurements about the `old` scope guard:

1. **The guard is present** — `resolve.rs:1604` refuses `old` outside
   `PropCtx::SpaceOpEnsures` with `UnboundName("old")`.
2. **The capability is absent** — `elab.rs:5584` refuses every `old` inside a
   space-op contract with `OldPreStateUnsupported`.

Both correct, both load-bearing, and I stated them adjacently. **Composed, they
say: at HEAD, both sides reject — at distinct gates.** The adjacent conformance
row asserted the opposite in its own body:

> **Verdict-flips** with the case above: identical `old(…)` syntax, **space-op
> resolves / pure-view rejects**.

**That sentence is false at HEAD, and my own two numbers are its refutation.**
The Architect found it on the candidate; it cost a round-trip.

## Why I missed it: I audited a SENTENCE, not the artifact

I went to that row for one purpose — it contained a **reassurance** (*"the
reject is guard-gated, not coincidental"*), and
[[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]] says a
reassurance is the highest-yield target this seat has. So I attacked it, and
**it held** (correctly — dropping the guard would make the pure-view case
progress to the *later* fence, so the diagnostic identity does distinguish it).

Then I reported the row clean.

**The false sentence was three lines above the one I cleared, in the same
bullet.** The hunting frame selected a sentence; the artifact carried two
claims, and clearing one said nothing about the other.

⇒ **When a target is chosen because it contains one claim-shape you hunt,
write down every claim in it before you leave.** A reassurance that survives is
evidence about the reassurance, never about its paragraph — and reporting *"I
attacked X and it held"* reads to the recipient as *"the row is sound"* unless
you name the scope of what you checked. Sibling of
[[no-option-works-name-the-axis-you-enumerated]]: name the **sentence** you
audited, not the row.

## The general form: a relation needs BOTH sides

The steward's statement of it, which is broader than my case:

> **When a correction lands on one row of a stated relation, the other side of
> that relation is in scope by construction.**

Mine is the instrument-side twin:

⇒ **When your finding is "guard G present, capability C absent", every artifact
asserting a relation that requires both is false — go find them.** A
verdict-flip, a discriminating pair, a differential oracle, an
accept/reject twin, an A-versus-B control: all are relations, and all collapse
when one side is unlanded. **The composition is the finding; the two
measurements alone are only its inputs.**

The tell is that I had already written *"the guard landed and the capability did
not"* in my own report — one clause, containing the refutation, aimed at
attributing a residual rather than at auditing a neighbour.

## What the repair did better than the ask, worth copying

I asked for a deferral tag on one row. The landed repair **generalized the
convention** the tag belonged to:

> The tag names **implementation availability**, not whether the governing
> design decision is open.

My report had noted the convention was *keyed on one open decision* so the next
one got no tag. **The repair fixed the key, not the instance.** Ask for that
directly next time: when a convention fails for the second member of its class,
the finding is the key, and requesting one more tagged row invites exactly the
per-instance patch. Same shape as
[[a-corrections-sweep-population-is-its-own-diff-scope]] from the other side —
there the sweep was too narrow, here the *rule* was.
