---
name: a-hand-written-de-bruijn-traversal-restates-the-kernels-binder-list-so-diff-it-and-pin-it-against-shift
description: >-
  Any hand-written de Bruijn traversal states a binder list, and the kernel's
  shift/subst_var already hold the authoritative one. A prover-path guard put
  Pair (a term former) in the binder arm next to Sigma; one grep against the
  kernel found it. The repair then restated the discipline in a second file,
  and the drift was pinned by an exact oracle computed from the kernel's own
  shift rather than documented as an unenforceable coupling. Also: a
  dangerous-thing predicate defaults to true; representation-level fail-safe
  is not semantic; attach evidence to "checked and sound" too.
metadata:
  type: feedback
---

# A hand-written de Bruijn traversal restates the kernel's binder list, so diff it, then pin it against `shift`

## Diff the traversal against the kernel

**Measured 2026-08-15 on `75a91d2ba...6ec9694fa`, in a new FO slice.**

```rust
Term::Pi(a, b) | Term::Lam(a, b) | Term::Sigma(a, b) | Term::Pair(a, b)
    => go(a, depth) || go(b, depth + 1),
```

The kernel, on the same constructor, twice:

```rust
subst.rs:36   Sigma(a, b) => (shift(a, cutoff), shift(b, cutoff + 1))   // binder
subst.rs:44   Pair (a, b) => (shift(a, cutoff), shift(b, cutoff))       // NOT
subst.rs:147  Pair (a, b) => (subst_var(a, j),  subst_var(b, j))        // NOT
```

⇒ **Any hand-written de Bruijn traversal states a binder list, and the kernel
already contains the authoritative one. Diff the two.** This took one grep
and is not a judgement call: `Pair` is a term former, and a guard that
increments depth for it is wrong **in both directions**. It misses
`Pair(_, Var(0))` (the unsound miss) and invents `Pair(_, Var(1))`.

⇒ **Look hardest at the constructor sitting next to a real binder in the enum.**
`Pair` is declared immediately after `Sigma` and shares its arity and shape;
that adjacency is how it joined the binder arm. Same instinct as
[[adjacent-arms-over-near-identical-shapes-are-the-cheapest-diff-in-a-recursive-walk]].

**A guard's default arm has a correct direction.** The same function ended
`_ => false` over ten constructors carrying term subterms. A predicate asking
"does this dangerous thing appear?" must default to `true`: here `true` yields
a refusal and `false` an erasure. The default is the answer given to every
constructor nobody has thought about yet.

**"By construction" can be true downstream and false upstream.** A quotation
boundary's `_ => Err` arm is a real construction-level refusal, but the guard
and the shift run *before* it, on unvalidated input; what saves them is that
the accepted grammar is a subset of the traversed constructors. Two
enumerations in two functions with nothing tying them together is a
coincidence, not a construction, and it expires at the first accepted shape
the traversal does not cover. Say which direction the coupling runs and what
would break it, rather than reporting "safe".

**Name the level at which a defensive guard is defensive.** The kernel's
`shift` refuses to underflow and leaves the index unchanged. That prevents a
wrapped, corrupt index; it does not prevent **capture** (the un-shifted
variable silently aliases whatever the next enclosing binder becomes).
Fail-safe at the representation level is not fail-safe at the semantic one,
and a comment promising "no corruption on any input" is easy to read as the
stronger claim.

**Rank a prover-path finding by reachability, out loud.** The route was
unreachable in production, the verdict was forbidden by spec, and both exits
converged on one hole: no soundness exposure. Saying so keeps the finding
credible; a defect in a guard inside a slice built to grow is worth fixing on
its own terms without borrowed urgency.

## Build the oracle from the function the duplicate must match

**Measured 2026-08-15 on `8fe2264c7...4674fe840`, the node built from that
finding.** The merge named its weakest seam honestly: the repaired guard
(`mentions_var0`, `crates/ken-elaborator/src/fo_kripke.rs`) restates the
kernel's binder discipline in a second file; exhaustiveness catches a *new*
`Term` variant, and nothing catches an *existing* variant changing binder
status. The predicate had an exact oracle in the function it duplicates:

```
mentions_var0(t)  ⟺  shift(shift(t, -1, 0), 1, 0) != t
```

A free `Var(0)` hits the underflow guard and stays, while every other free
index goes down and comes back, so the round trip is the identity iff no free
`Var(0)` occurs, and it holds under binders because `shift` raises the cutoff
itself. (Landed as `mentions_var0_agrees_with_shift_round_trip_oracle`.)

⇒ **When a duplicate must agree with an original, compute the duplicate's
answer from the original.** An oracle built from the thing you must match
cannot drift from it, so the coupling becomes structurally checked rather
than a documented risk.

⇒ **Recommend the differential test, not the rewrite.** Substituting the
round trip for the traversal removes the duplication and costs legibility.
Keeping the readable code and pinning it against the oracle gets the
guarantee without the obscurity.

⇒ **Name what the oracle depends on.** This one relies on the underflow guard
leaving the index unchanged; if that changed, the test breaks loudly. An
oracle with a stated dependency that fails loudly is worth more than one whose
assumptions are unexamined.

**Vacuity and missing-refusal are different risks** for a guard like this; see
[[a-candidate-collector-can-over-collect-and-the-design-note-only-reasons-about-missing]].

## A cleared attack gets the same evidence standard as a finding

In the same slice a sort-collapse had been cleared as "not exploitable:
freshness plus `Init`'s syntactic equality blocks any confusion." Neither
checks sorts. A QA probe built the ill-sorted formula and `check_cert`
accepted it; `Init` closes on syntactic equality, which a malformed formula
still has. The real mechanism was caller-side: the target type is strictly
larger than the embedding's image, and the probe lived in that excess.

⇒ **A cleared-attack paragraph becomes durable text exactly like a finding
does.** The wrong reason was written into a doc comment and stood until
someone probed it. Attach the same evidence standard to "checked and sound" as
to "here is a defect"; the first is read as settled and is far less likely to
be re-examined. When the correction is someone else's, say plainly that their
version is better. Sibling of
[[a-proposed-sentence-lands-verbatim-and-carries-its-defects-into-durable-text]].
