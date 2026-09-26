---
name: a-candidate-collector-can-over-collect-and-the-design-note-only-reasons-about-missing
description: A discovery walk was justified by "a missed occurrence only costs completeness", but its sort rule admits any bare Const domain and the slice's own Bottom is a bare Const — so an antecedent Bottom adds a spurious candidate and the ambiguity check refuses a formula the slice can prove
metadata:
  type: feedback
---

# A candidate collector can over-collect; the note only reasons about missing

**Measured 2026-08-15 on `de551a4dd..427fc4069`**, in the FO slice's
signature discovery (`collect_signature_candidates`,
`crates/ken-elaborator/src/fo_kripke.rs`).

```rust
if let Term::Pi(domain, _) = term {
    if let Term::Const { id, level_args } = domain.as_ref() {
        if level_args.is_empty() { sort_ids.insert(*id); }   // ANY bare Const
    }
}
```

`⊥` **is** a bare `Const` — the same module's quoter accepts
`Const { id } if *id == env.bottom_id()` as the slice's `Bottom`. ⇒ **An
antecedent `⊥` contributes a spurious SORT candidate**, the mandatory
`forall`-bound sort makes two, and the ambiguity check refuses.

⇒ **Constructible and provable by the slice's own rules.** `∀x. (⊥ → (P x → P x))`
needs exactly imp-right, imp-right, init, and discovery refuses it before
quotation runs.

⇒ ***The design note reasons only about UNDER-collection*** — *"a missed
occurrence only costs completeness"* — **but a collector feeding an
exactly-one check fails the other way too, and over-collection is what an
ambiguity test converts into a refusal.** The conclusion (completeness, never
soundness) survives; **the stated reason does not cover the mechanism that
produced it.**

⇒ ***When a rule recognizes a role by SHAPE, list every value in the language
that already has that shape.*** Here `Type`-valued sorts and the propositional
constant `⊥` are both bare `Const`s, and the vocabulary the module itself
defines is the first place to look.

**Why no control caught it**: the existing fixture puts `⊥` in the
**consequent**. **A constant appearing on the safe side of an arrow is not
evidence about the other side.**

## VACUITY AND MISSING-REFUSAL ARE DIFFERENT RISKS NEEDING DIFFERENT CONTROLS

The merge recorded that a safety conjunct is never observed **refusing**, and
separately that if a kernel guard widened the conjunct would **silently return
`true` for everything**. The successor was framed at the first.

⇒ ***Observing a refusal may be hard or impossible to construct; proving
non-vacuity is usually trivial and is closer to the stated risk.*** Here the
refusal needs an input passing two strong earlier conjuncts and failing this
one; non-vacuity needs only asserting the comparison **distinguishes** two
different values.

⇒ **Say which one the stated risk actually is, and rank accordingly.** A gap
recorded as *"we never see it refuse"* and a risk described as *"it would start
accepting everything"* are not the same sentence, and the cheaper control
addresses the more dangerous one.

⇒ **And put it in the file that depends on it.** *"A property of the kernel, not
of this file"* is a cross-crate dependency nothing local asserts — **one someone
can retire without ever seeing this code.** Sibling of
[[a-hand-written-de-bruijn-traversal-restates-the-kernels-binder-list-so-diff-it-and-pin-it-against-shift]]

## HONOUR A FORWARD COMMITMENT OUT LOUD

The Adversary had said the thing to watch was not a gate weakening but a
**second exit bypassing it**. Censused on arrival: one production call site,
unconditional return, no bypass. ⇒ **Report a committed check even when it
passes** — a promise to look, unreported, is indistinguishable from not having
looked.
