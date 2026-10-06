---
id: LANG-LET-BINDING-EQUATION
title: "Since LANG-ENSURES-PER-PATH-REALIZATION, a postcondition over a let in result position is realized at the let body's leaf with the binder bare: no Eq A x e is in the context, so the goal is closed over an unconstrained x and cannot be discharged. Spec 22 §3 requires the let equation. Push it at the let binder for the body's extent"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-ENSURES-PER-PATH-REALIZATION]
blocks: []
github: null
origin: "Adversary finding evt_6rt513nmg3pzx on b81ecf220. A completeness regression for ensures and literal refined returns (before the merge the whole-body goal reduced through the let); it fails closed, and no catalog, library or examples program uses the shape. The named-return row has behaved this way since LANG-REFINEMENT-INTRODUCTION-OBLIGATION. Steward-filed per COORDINATION section 2."
---

# A let binding carries its equation

## Objective

Every per-leaf obligation under `let x = e in body` is closed with the
binder `(x : A)` and, where `A` is informative, the hypothesis
`(_ : Eq A x e)`, as spec 22 §3 says. A postcondition the whole-body goal
could discharge before LANG-ENSURES-PER-PATH-REALIZATION is dischargeable
again.

## Settled inputs (Adversary `evt_6rt513nmg3pzx`, on `b81ecf220`)

- **The site.** `check_let` (`crates/ken-elaborator/src/elab.rs:1316-1327`)
  pushes the binder as a bare `UserLocal` and forwards the result
  predicates into the body. `close_refinement_goal_with`
  (`:11444-11469`) closes the leaf goal over `cx.ctx.types`, so `r` is an
  unconstrained Π. No elaborator code builds a let equation.
- **Rows**, through `elaborate_decl_v1`, with opaque `Q : Int -> Ω`,
  `P2 : Int -> Int -> Ω` and `def QPos = { x : Int | Q x }`:
  - `ensures Q result = let r = n in r` closes `Π n. Π (Q n). Π r. Q r`.
  - The literal return `{ x : Int | Q x } = let r = n in r` and the named
    return `QPos = let r = n in r` close the same goal.
  - `ensures P2 n result = let r = n in r` closes `Π n. Π r. P2 n r`,
    which is false in general.
  - A match arm `True |-> let r = n in r` drops the equation, while the
    arm's `Eq Bool b True` is present.
  - Controls: the same bodies without the let close `Π n. Π (Q n). Q n`.
- **Not affected:** `if` and constructor path equations, and the induction
  hypothesis channel (non-decreasing self-calls are refused by SCT).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The let equation is in the context for the extent of the let body, in
every result-position check that realizes a postcondition per leaf
(ensures, literal and named refinements). The Architect rules where it is
pushed and what "informative" admits, from the implementer's first measure
of the rows above.

## Acceptance

- **AC-1.** Each row above becomes a test. The ensures, literal, named and
  match-arm rows close with `Eq Int r n` in the goal's context and
  discharge. The two-parameter row's goal is `P2 n n` after rewriting, or
  carries the equation that makes it so.
- **AC-2 (controls).** The let-free controls' goals are unchanged. The
  ENSURES suites (`lang_refinement_introduction`,
  `decimal_char_acceptance`, `v1_acceptance`, `v2_acceptance`,
  `rtp1_elim_reduce_ih_perf_acceptance`, `rosetta`) stay green.
- **AC-3 (mutation, QA).** Dropping the pushed equation reddens the rows.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A checked catalog, library or example program gains an obligation it
  cannot discharge.

## Closeout

Merged `c49297983` from exact `de2f4fa48` (PR #4543). Language QA
`evt_t5p85baddcyq`, Architect `evt_5dqn322zs91p7`, Decision
`dec_54ca03kh3jrk4`.

- Both let sites push `Eq A x e` onto `path_conditions` for the body when
  `whnf(sort(A)) = Type _`; Ω proof-lets push nothing. Emitted core is still
  an ordinary `Term::Let`.
- Obligations closed through `close_refinement_goal_with` consume the
  equation, and certificates are kernel-checked against the hole type.
- Carry: `close_goal` callers (call-site `requires`, `PartialPrim`) still
  ignore path facts, framed as `LANG-CLOSE-GOAL-PATH-CONDITIONS`.
