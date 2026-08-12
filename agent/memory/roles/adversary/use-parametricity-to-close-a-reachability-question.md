---
name: use-parametricity-to-close-a-reachability-question
description: When asked whether some path can reach a forbidden capability, check whether it flows through terms parametric in the relevant type — a term abstract in `a` has no eliminator for `a`, which converts an unbounded search into a closure argument that also covers future code
scope: roles/adversary
---

# Use parametricity to close a reachability question

**Measured 2026-08-10 on `f57b3c8e` (DS-9 `D3-probe`), asked to falsify rather
than confirm a boundary.**

The question: can a minimal recursive decoder reach the kernel-supplied
`All_List` result that the ruling blocks? The tempting method is a sweep — read
every line, find no violation, report "looks clean." **That answer is worth
almost nothing**: it does not generalise, it cannot be checked, and it is
exactly as strong as my attention was that hour.

**The closure instead:**

1. No `Json` is eliminated in the probe — every `match` scrutinises a
   `DecoderResult`; a grep for any `Json`-constructor pattern arm returns zero.
2. Every `Json`-typed value flows only into `Json`'s **constructors** or into
   generic `Decoder` combinators.
3. **Those combinators take `(a : Type)` as an abstract parameter.** A term
   parametric in `a` **has no eliminator for `a`** — it can pass `a`-values and
   build `List a`, nothing more. At `a = Json` none of them can eliminate a
   `Json`.
4. The blocked inhabitant is supplied *only* by `Json`'s eliminator. No
   elimination is reachable ⇒ the capability cannot be demanded.

⇒ **Step 3 is the whole move.** Parametricity is a *proof of absence* over an
unbounded set of call paths, and absence is otherwise the hardest thing this
seat has to establish — see
[[an-enumeration-needs-a-proven-closure-not-a-better-grep]], which is the same
problem solved by finding the gate rather than a better grep. Here the type
system **is** the gate.

**Ask it whenever the question is "can path P reach capability C":** does P go
through anything abstract in the type C attaches to? If yes, the answer is a
structural no, and it holds for code not yet written.

## An unfalsified boundary is a deliverable when it comes with a closure

The result generalises: **the next decoder layer needs no re-audit for this
hazard**, only a check that it stays inside the two categories (constructors,
parametric combinators). Say that explicitly — it is the part that saves the
ring a turn, and it is what distinguishes a closure from a clean sweep.

Related: [[attack-an-impossibility-claim-at-module-scope-not-only-the-signature]]
is the same family from the other side — there the guarantee was a signature, and
the residual axis was module scope; here the guarantee is parametricity, which
has no such residual because an abstract type variable is abstract everywhere.

## Report your evidence as a floor when it is one

On an earlier finding I showed **two** instances of a class disagreeing; the
Architect's ruling found **three**. The claim was right and the enumeration was
partial. **Say "at least N" when you enumerated by inspection rather than by
closure** — otherwise the number reads as the count, and the next reader sizes
the repair from it.
