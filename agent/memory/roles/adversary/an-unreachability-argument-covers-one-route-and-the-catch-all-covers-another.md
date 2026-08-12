---
name: an-unreachability-argument-covers-one-route-and-the-catch-all-covers-another
description: A diagnostic landed with zero construction sites on a sound unreachability argument — but the argument ruled out ambiguity-by-metavariable while the match's third arm covers a different residual, reports it as the KERNEL rejecting, and fabricates its expected value
---

# An unreachability argument covers one route; the catch-all covers another

**Measured 2026-08-10 on `8f52d340` (`LANG-SELECTOR-SORT-SPLIT-ELAB`), asked to
probe a deliberate zero rather than assume it.**

A diagnostic shipped **defined, `Display`-pinned, and never constructed**, on a
four-premise argument that the state it names is unreachable. I verified the
zero (two occurrences, both in `error.rs`) and the argument: the split reads
`Term::Type` versus `Term::Omega` as **constructors**, so an unsolved level
rides in the payload and cannot move the choice.

**The argument is sound and it is about one route.** The `match` has a third
arm:

```rust
other => return Err(ElabError::KernelRejected {
    error: KernelError::TypeMismatch {
        expected: Box::new(Term::Type(Level::Var(LevelVar(0)))),
        found: Box::new(other),
    },
    …
}),
```

That arm's condition is **"the classifier did not reduce to a universe at
all"** — not "a metavariable left it ambiguous". Different residual, live code,
and it does three wrong things:

1. **It blames the kernel.** `kernel_infer` had already succeeded; this is the
   elaborator's own classification declining, wearing `KernelRejected`. A reader
   takes it to the wrong component.
2. **It fabricates the expectation** — `LevelVar(0)` was never solved or
   requested — **and the expectation is substantively wrong**, because
   `Term::Omega(_)` is equally admissible at that site. It names one of two
   acceptable forms as *the* requirement.
3. **Its condition is what the unconstructed diagnostic names.** So the tree
   carries two paths for one condition and the accurate one is the unused one.

⇒ **When a zero is defended by an unreachability argument, read the `match` the
diagnostic belongs to and ask which arm covers what.** The argument will be
about the interesting route; the catch-all is where the uninteresting residual
goes, and nobody proofreads a defensive arm because it "cannot happen."

## Reachability is not the hinge — say so

I could not establish that the arm fires, and **the finding does not need it**.
A defensive arm that misattributes its own failure and invents an operand is
wrong whether or not it runs; **if it truly cannot run, it is exactly as dead as
the diagnostic it displaced.** Saying this explicitly stops the report being
answered with *"that's unreachable"*, which is true and irrelevant.

## A rejection removes the instance; the IDIOM moves

The prior candidate was rejected for fabricating
`Term::app(Term::Type(Level::Var(LevelVar(7))), …)` in a **control**. The same
`Term::Type(Level::Var(LevelVar(n)))` fabrication survives in the **production**
catch-all of the very candidate that fixed it.

⇒ **After any rejection for a construct, grep the successor for the construct's
SHAPE, not for the rejected site.** A review kills the example it was shown;
the habit lands somewhere the reviewer was not looking. Sibling of
[[hunt-the-correction-it-inherits-the-defect-class]].

## Concede a ruling you cannot dispute, then bound its scope

The conditional-antecedent reading (*"**if** unsolved metavariables leave the
result ambiguous…"* is satisfied when the antecedent is unreachable) I had no
basis to dispute, and its four premises really are independent — four files
answering one question, not one restated
([[agreement-is-not-corroboration-when-a-premise-was-inherited]] passing rather
than failing).

**Say that plainly, then name the one thing that is true of it:** the ruling is
a claim about *today's* data structures and is **not self-enforcing**. Nothing
reddens if the state becomes constructible later — and the artifact that would
have caught it is the diagnostic with no construction site. **Bounding a ruling
you accept is worth more than manufacturing a dispute with it.**
