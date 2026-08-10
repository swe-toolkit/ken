---
id: LANG-SORT-META-CAPABILITY
title: "Rule whether a term/sort metavariable representation is authorized -- the elaborator cannot today leave a selected result undecided between Type and Omega, so the spec's conditional ambiguity clause has an unreachable antecedent"
status: draft
owner: spec-enclave
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Split out of LANG-SELECTOR-SORT-SPLIT-ELAB when Language measured the representation boundary at evt_7w2ctbaswz58j and spec-leader routed it at evt_d2vqvnfkfv18. Filed draft rather than ready because it needs a ruling before it can be framed; the parent WP proceeds without it, since the code required there is identical under either disposition.
---

> # THIS IS A RULING REQUEST, NOT SHOVEL-READY WORK.
>
> It is `draft` deliberately. **Nobody should start building against it**, and
> its `draft` status is not framing debt — see the closing section.

## The measured boundary

Language established this at `evt_7w2ctbaswz58j` and I am recording it here so
it survives the thread:

1. `ElabCtx::metas` is `MetaCtx { metas: Vec<Option<Level>> }`. `fresh()` mints
   only a `Level::Var`; `solve()` accepts only a `Level`.
2. Core `Term` has **no term or sort metavariable variant**.
3. `MetaCtx::zonk_term` substitutes levels recursively while **preserving every
   term constructor**, including preserving `Term::Type` versus `Term::Omega`.
4. Kernel `classify` infers a term and matches the resulting WHNF constructor as
   `Term::Type` or `Term::Omega`.

⇒ **A level solution can change the level payload and never which of those two
constructors is present.** No metavariable in the current representation can
leave a selected hidden result genuinely undecided between `Type` and `Omega`
and later resolve that choice while retaining the same selected result term.

The empirical probe was causal and restored byte-identically: with the same
selector, association, slot and selected term `Term::Type(Level::Var(LevelVar(0)))`,
inference returned `Ok` with the meta unsolved **and** after solving it to
`Level::Zero`. The Type constructor already determined the classifier.

## What this does and does not put in question

**It does not breach the landed spec.** `spec/30-surface/39-elaboration.md:256`
and `34-data-match.md:351` state the clause **conditionally** — *"If unsolved
metavariables leave the result type ambiguous between `Type` and `Omega`, the
selector rejects with `RecursiveResultSortAmbiguous`"* — beside the operative
guarantee that *"the elaborator never defaults."* **A conditional with an
unreachable antecedent is satisfied.** The spec nowhere asserts the ambiguous
state is reachable.

**What is genuinely open is whether it should be reachable.** That is a
capability question, and it has two halves that route differently:

| question | owner |
|---|---|
| Should the spec's ambiguity clause be restated to say the antecedent is unreachable in the current core, or left conditional and contingent? | Spec enclave (spec-author formulation routed at `evt_d2vqvnfkfv18`) |
| Is a term/sort metavariable representation the right mechanism, and where does it live? | Architect |
| **Does adding a metavariable variant to core `Term` grow the TCB?** | **Operator** — I forward this, I do not decide it |

**The third row is the one that must not be skipped.** An elaborator-side sort
meta that zonks away before kernel checking is one thing; a variant in the
kernel's own term language is another, and the second is TCB growth. **Establish
which is being proposed before anyone rules on whether it is authorized** — the
two answers can differ, and a ruling that does not say which it granted is worse
than no ruling.

## Why the parent WP is not held on this

[[LANG-SELECTOR-SORT-SPLIT-ELAB]] proceeds now. Under **both** live spec
dispositions the code it must produce is the same: delete the over-broad
syntactic `LevelVar` fallback that raises the ambiguity diagnostic on any
inference failure, keep `RecursiveResultSortAmbiguous` defined with no
production raise site and a comment saying why, and prove the non-defaulting
guarantee that *is* testable today. **A ruling that changes nothing about the
candidate is not a blocker for it.**

Only an authorization of sort metavariables would change the code, and that
would be this node's work, on its own branch, after a ruling.

## Why `draft` rather than `ready`

`ready` asserts a shovel-ready frame exists and a team could start. Neither is
true and neither should be until the three questions above are answered — the
deliverable's shape depends entirely on which mechanism is authorized. **A
`draft` here is the honest state, not framing debt**, and it is exempt from the
stay-one-release-ahead policy for that reason: it is not a successor waiting on
a frame, it is a question waiting on an answer.

When the rulings land, this node gets a real frame or gets `closed` as
resolved-without-landing.
