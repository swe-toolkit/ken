---
id: LANG-SELECTOR-CLASSIFIER-RESIDUAL-DIAGNOSTIC
title: "The selector's non-universe classifier arm reports the elaborator's own refusal as KernelRejected and fabricates a Type(?0) expectation that Omega would equally satisfy"
status: merged
owner: language
size: S
gate: none
depends_on: [LANG-SELECTOR-SORT-SPLIT-ELAB]
blocks: []
github: null
origin: Confirmed Adversary finding evt_72pmvy25tdg1x on 8f52d340, measured on c0757335, verified independently by the Steward at elab.rs:2822-2828. Filed as a successor rather than folded, because LANG-SELECTOR-SORT-SPLIT-ELAB had already merged; held ready rather than released because the operator authorized two concurrent lanes and Runtime and Kernel hold them.
---

## What it is

`crates/ken-elaborator/src/elab.rs:2822-2828`. After the sort split classifies
the selected hidden result, the `match` on the whnf'd classifier has a third arm
for a classifier that is neither `Term::Type` nor `Term::Omega`:

```rust
other => return Err(ElabError::KernelRejected {
    error: KernelError::TypeMismatch {
        expected: Box::new(Term::Type(Level::Var(LevelVar(0)))),
        found: Box::new(other),
    },
    span: span.clone(),
}),
```

**Three defects in one arm, and none of them changes behaviour.** Severity is
diagnostic quality; the direction is misleading toward the wrong component.

**1. The kernel did not reject anything.** `kernel_infer` succeeded four lines
above — its `?` passed and already maps a real kernel error to
`KernelRejected` — and `whnf` succeeded. This is the **elaborator's own**
classification declining, reported under the kernel's name. A reader takes it to
the kernel, which is not where it happened.

**2. The expectation is fabricated, and it is wrong in substance.**
`LevelVar(0)` was never solved or requested; it is a placeholder. And
`Term::Omega(_)` is **equally acceptable at this very site** — the arm directly
above accepts exactly that. So the diagnostic names one of two admissible forms
as *the* expectation and tells the author to make their result `Type`-classified
when `Omega` would have been fine.

**3. The idiom is the one the previous candidate was rejected for.** The
Architect blocked `b63e3daa` for building
`Term::app(Term::Type(Level::Var(LevelVar(7))), Term::var(0))` in a **control**.
**The same `Term::Type(Level::Var(LevelVar(n)))` fabrication survives here, in
production rather than in a test.** The rejection removed the instance; the
idiom moved.

## What is NOT in question

**The sort split itself is sound and stays.** `RecursiveResultSortAmbiguous`
having **zero construction sites** is correct and deliberate — verified again
here: three occurrences in `crates/`, all in `error.rs` (definition, `Display`,
variant listing). The unreachability argument behind that stands, and its four
premises are independent rather than one restated: `MetaCtx`'s shape, `Term`'s
lack of a sort metavariable, `zonk_term`'s constructor preservation, and
`classify`'s constructor match. **Do not reopen it** — see
[[LANG-SORT-META-CAPABILITY]].

**This arm is a different residual.** The ruled-unreachable state is *ambiguity
by unsolved metavariable*. This arm fires when the classifier **did not whnf to
a universe at all**, which is not the same condition.

**Reachability is not established and the deliverable does not depend on it.** A
defensive arm that misattributes its own failure and invents an operand is worth
fixing whether or not it fires. **If it genuinely cannot fire, the tree carries
two dead paths for one condition and the accurate one is the unused one** —
which is its own argument for repair.

## Deliverables

**1. Report the elaborator's own refusal, not the kernel's.** Whatever variant
carries it, it must not claim the kernel rejected when `kernel_infer` returned
`Ok`.

**2. Say what is true instead of asserting an expectation.** The honest claim is
that **the selected result's type is not classified by a universe** — it names
the actual condition and does not privilege `Type` over `Omega`, which are both
admissible one arm above.

**Whether that reuses `RecursiveResultSortAmbiguous` or introduces a new variant
is Language's call**, and I am not ruling it. If you reuse the reserved
variant, say so explicitly and update its comment, because that comment
currently states it has no construction site.

**3. No fabricated `Term` operands in the diagnostic payload.** If a payload
cannot be supplied truthfully, the diagnostic should not carry that field.

## Acceptance criteria

**AC-1.** `kernel_infer` returning `Ok` followed by a non-universe classifier
does **not** produce `ElabError::KernelRejected`. Show a control that
distinguishes the two origins: a genuine kernel inference failure at the
selector still reports as kernel-attributed, while this arm does not. **Both
directions, on the same selector shape** — one alone passes for an
implementation that relabels everything.

**AC-2.** No `Level::Var(LevelVar(n))` is constructed as a diagnostic payload
anywhere on the selector path. Report the count, and report it separately from
legitimate `fresh()` level metas so a zero is not read as covering those.

**AC-3.** The message does not name `Type` as required where `Omega` is equally
admissible. A reader following the diagnostic must not be pointed at a change
that was never necessary.

**AC-4 — reachability, reported honestly either way.** Establish whether the arm
can fire and say which. **If it cannot, that is a finding, not a failure** —
record it, and do not manufacture an input to reach it. **Do not fabricate a
malformed term to exercise this path**; that is precisely what got `b63e3daa`
rejected.

## Excluded scope

- **Not a re-opening of the sort split.** The two spellings, the classification
  rule, the association mechanism, the removal of the old surface, and the five
  `StructuralResult*` names are all settled.
- **Not the sort-meta capability.** That is [[LANG-SORT-META-CAPABILITY]] and it
  needs a ruling, not code.
- No `spec/`, `conformance/`, or runtime path.

## Stop conditions — return to me, do not decide

Any case where reporting the condition truthfully would require a spec change,
or where the honest diagnostic cannot be expressed without a new error payload
shape the spec pins otherwise.

## Contention and validation

`crates/ken-elaborator` and its tests. Runtime owns `crates/ken-runtime`; Kernel
is on test targets and a `conformance/` row. No `spec/` or `conformance/` path,
so no Spec vote on the merge Decision.

Targeted only — `-p ken-elaborator`, or `--test <name>` for one suite, **never
`--workspace`**. Adding or changing an enum variant makes the floor a full
`-p ken-elaborator` test build. "No regression" means green in CI.

## Sizing

One turn. **Released when a lane frees** — the operator authorized two
concurrent lanes and Runtime and Kernel hold them. This node is `ready` so it
enters the frontier the moment one closes; do not start it before I kick it.
