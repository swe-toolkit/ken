---
id: LANG-OLD-OPERAND-START-ADMISSION
title: "`old`'s operand is parsed by a direct `parse_atom_expr()` call that consults no atom-start roster and no exclusion, so forms refused in ordinary argument position -- `old if a then b else c`, `old proof p for s` -- are admitted after `old`. A live over-admission on main. The repair direction needs a design ruling first: whether `old`'s operand is an application_atom position at all (`21 §6.4`, `32 §3`)."
status: merged
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Found live by language-implementer while sizing increment B of LANG-ATOM-START-CLASSIFICATION-CLOSURE, raised through language-leader (evt_5s6yxwcv4dw93). Steward ruled it SPLIT rather than folded into AC-10, and re-measured the call-site population independently. The implementer explicitly declined to adjudicate whether the admission should be legal, which is correct -- that is a spec/design call and this node carries it as an open question rather than an assumed direction."
---

## The defect, measured at `b53dd9fcf`

`crates/ken-elaborator/src/parser.rs:3771-3776`, the `Token::KwOld` arm of
`parse_atom_expr_base`:

```rust
// `old e` — pre-state reference (`21 §6.4`)
Token::KwOld => {
    self.advance(); // consume 'old'
    let arg = self.parse_atom_expr()?;
    let end = arg.span().end;
    Ok(Expr::EOld(Box::new(arg), Span::new(start, end)))
}
```

**No roster consult, no exclusion, no predicate call of any kind.** The operand
is whatever `parse_atom_expr` will parse.

Measured by the implementer with a positive control, 5 forms, 3 agreeing with
argument-position behaviour and 2 disagreeing:

    old if a then b else c      PARSES as old's operand   REJECTS as an app argument
    old proof p for s           PARSES as old's operand   REJECTS as an app argument

The two forms are refused in argument position by **two different mechanisms** --
`KwIf`'s hand-written case and `ProofSelector`'s `StartExclusion` -- and `old`
bypasses both, because it does not reach either. It never calls a predicate.

## THE CALL-SITE POPULATION IS ONE, AND THAT IS MEASURED

`self.parse_atom_expr()` has three direct call sites. The finding surfaced at one
of them; the Steward checked whether it generalises, because *"scope the repair
to the population, not to wherever the finding happened to surface"* is this
node's sibling frame's own rule:

    2929   parse_app_expr, the application HEAD   UNGATED BY DESIGN -- head
                                                  position never consults the
                                                  roster; see the §5 position
                                                  taxonomy in
                                                  LANG-ATOM-START-CLASSIFICATION-CLOSURE
    2960   the application ARGUMENT loop          GATED -- sits inside the loop
                                                  whose guard rejects `if` two
                                                  lines above
    3773   `old`'s operand                        UNGATED, AND NOT BY DESIGN

**Only `3773` is the defect.** The other two are correct, for different reasons,
and neither is an instance. Do not widen this node to them.

## Why this is its own node and not an AC on the closure

**It is a different mechanism, and guarding rosters harder does nothing for it.**
The `KwProof` hardening closed *"a roster function that does not apply the
exclusions itself"*. This is *"a caller that never calls a roster function at
all"*. A repair that makes every roster self-guarding leaves `3773` exactly as
it is.

**The repair DIRECTION is not determined.** Whether `old if a then b else c`
should be legal is a design question about whether `old`'s operand is an
application_atom position (`21 §6.4` for `old`, `32 §3` for `application_atom`).
Folding it into increment B would make that increment's closure wait on a ruling
neither the implementer nor the node owns.

**It is live on `main` today**, independently of whether the closure node ever
lands, so it has its own urgency and its own acceptance criteria.

## THE DESIGN QUESTION -- ANSWERED 2026-09-20. THE YES ARM IS RELEASED.

**Is `old`'s operand an application_atom position?**

- **If YES:** the repair is to route it through the same classification every
  other atom-start consumer uses, and the two forms above become rejections. The
  sanctioned spellings are then `old (if a then b else c)` and
  `old (proof p for s)`.
- **If NO** -- `old` takes a full expression by design -- then the current
  behaviour is correct and the defect is that **`21 §6.4` does not say so**, the
  repair is a spec clarification plus a test pinning the admission as
  deliberate, and this node becomes spec-owned.

**Do not pick one to get started.** The two answers have disjoint deliverables
and opposite tests. Routed to the Architect and the Spec enclave by the Steward.

> ### RULED YES (Architect, `evt_5x6r1tsgvjwsc`). Released by the Steward 2026-09-20.
>
> **`old`'s operand IS an `application_atom` position.** The YES arm above is
> live, this node is language-owned parser repair, and **no Spec ruling or spec
> amendment is a prerequisite.** AC-1 is satisfied by citing
> `evt_5x6r1tsgvjwsc`; do not re-open the category question.
>
> Determined by the published grammar rather than by preference:
> `21 §6.1` makes a proposition an ordinary `expr` and `old` an ordinary
> identifier at parse time, so `old e` is ordinary application; `32 §3` defines
> application as `expr application_atom`, whose production is `primary` plus
> projections, and **`if` is a leading `expr` form and `proof_ref` is its own
> `expr` alternative -- neither is a primary**, so `old if ...` and
> `old proof ...` have no derivation while the grouped forms do. `21 §6.4`
> changes the meaning and scope of `old`, not the syntactic category of its
> operand: a pre-state capture is not a second unrestricted-application
> production.
>
> **This EXTENDS a settled boundary rather than drawing a new one.** The
> Architect's earlier B3 ruling `evt_1zm0d3waytsxt` already found `old`'s
> operand atom-tight -- dropping the parentheses in `old (f x)` changes the tree
> to `(old f) x` -- and the accepted repair `c68ef42a4` introduced
> `ExprContext::OldOperand` with the atom-tight `< 8` boundary, pinned today by
> `ac4_old_atom_boundary_parentheses_preserve_meaning`. The dedicated
> `Expr::EOld` arm is an implementation shortcut for that syntax and must
> preserve the same boundary; its direct `parse_atom_expr()` call bypassing the
> argument-position classification is therefore a parser defect, not a design
> choice.
>
> ### Implementation envelope (Architect, binding)
>
> - After consuming `old`, consult the **same expression-position atom-start
>   classification used for application arguments** before parsing exactly one
>   atom.
> - Any expression-position exclusion is a rejection in this consuming position:
>   reject bare `if` and bare `proof` at their leading token and direct the
>   author to grouping.
> - **Do NOT change `StartExclusion::ProofSelector`'s ordinary application-loop
>   `argument_diagnostic()` from `None` globally.** Its quiet yield is
>   load-bearing for `proof` beginning the next declaration. `old` has already
>   consumed its prefix and still owes an operand, so it must speak LOCALLY
>   rather than change declaration-boundary behaviour elsewhere.
> - Preserve `old (f x)`, `(old x).field`, ordinary atomic operands, and the
>   existing `ExprAtomForm::Old` head/argument classification.
> - AC-2 carries BOTH bare rejections and grouped admissions; AC-3's revert to
>   the current direct unguarded call must red those rows.
>
> **Anchors are perishable.** The defect is measured at `b53dd9fcf` and the
> call-site population of one at `parser.rs:3773`. The closure node has since
> landed five further increments and is complete. **Re-measure the
> call site and re-confirm the two bare forms are still admitted at YOUR base
> before repairing.** If either is already refused there, stop and report: the
> defect would have been closed by that node's own work, which changes this
> node's scope rather than merely shrinking it.

## Acceptance criteria

- **AC-1.** The design question above is answered by a cited ruling
  (`evt_`/`dec_` id) before any parser edit. The node records which arm was
  taken.
- **AC-2.** The chosen behaviour is pinned by a test that carries BOTH
  directions: the admitted spelling parses and the refused spelling is refused,
  with the diagnostic named. A test that only asserts the new behaviour cannot
  tell a fix from a no-op.
- **AC-3 (control, REQUIRED).** Mutation: revert the `3773` call to its current
  unguarded form and confirm the AC-2 rows RED. If they stay green the test is
  not reaching the site.
- **AC-4.** State whether any catalog or conformance source uses `old` with a
  non-atom operand, measured and with the extraction named. If the YES arm is
  taken this bounds the blast radius; if NO, say so -- *this position is unused
  today* is a real answer.

## Relationship to `LANG-ATOM-START-CLASSIFICATION-CLOSURE`

That node's thesis is that consumers of the atom-start classification drift from
it, and its symptom inventory already contains a **zero-consultation** case
(`parse_decls:592`, which consults the roster zero times and is affected
transitively). This finding is a second instance of that class.

**The closure node must NAME this and scope itself as NOT covering direct-call
consumers.** Its own §1 remit warns that a node scoped to where a finding
surfaced *"fixes two of three occurrences and leaves an identical third behind a
different function name"* -- and AC-10 claiming closure over a population that
excludes `3773`, silently, would be that failure committed by the frame that
states it.
