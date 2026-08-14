---
id: LANG-REACHABILITY-SUBSUMING-ARMS
title: "`ReachabilityError` carries only a span, so a redundant-arm diagnostic cannot say WHICH earlier arms subsume the dead arm -- the mirror of the gap `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD` closed on the exhaustiveness side, except `34 §4.2` does NOT mandate it, so this is ergonomics and must not be filed or read as a conformance obligation"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect carry registered in the LANG-MATCH-DIAGNOSTIC-PROSE approval (dec_75pvygp3aatab, resolved 2026-08-14T16:32:12Z), where he wrote 'Registering the shape only; sizing and cut are not mine.' Filed by the Steward because a carry recorded only in an approval verdict has no node and evaporates -- the same reason RT-CONTKEY-REFUSAL-PROFILE-SPLIT was filed the same day. Measured at origin/main before c2f285ee merged. FLIPPED ready and framed 2026-08-14 at main 6da108b6 by the Steward: the priority call the flip condition asked for is a sequencing call and it is the Steward's; every conformance-grounded Language alternative is gated on someone else (the convoy discriminator on an Architect ruling, the pattern-forms census on a cut, DecEq Char on the operator's TCB question), and an idle ring is worth less than an ungated ergonomics node. The vacuity paragraph was corrected the same day from the Adversary hunt evt_4d10j8tmjsbhj, which measured that BOTH subtleties are vacuous rather than one, and named the contingency that neither this node nor its parent had stated."
---

## What this is

`crates/ken-elaborator/src/error.rs`:

```rust
ReachabilityError { span: Span },
```

**A span and nothing else**, emitted from three production sites in `elab.rs`.
The rendered diagnostic tells a user an arm is dead and not why, so they read

> `redundant match arm at 12-40: pattern already covered by the earlier arms`

with **no indication of which earlier arms subsume it.** In a long match that
is the whole question.

**The information exists at the point of failure.** `34 §4.2` describes
detection as the same matrix walk, with an arm reaching an **empty residual
matrix** being unreachable -- so the arms that consumed the residual are known
where the error is constructed.

## THE CLASSIFICATION IS THE POINT OF THIS FILE

**This is the mirror of `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD`, and it is NOT
the same kind of obligation.** The distinction is easy to lose and would
mis-prioritise this node:

| | exhaustiveness side | reachability side (this node) |
|---|---|---|
| spec text | `§4.1` **requires** the error "names the unmatched pattern" | `§4.2` requires only that redundancy be **detected** |
| what was wrong | payload could not express a mandated fact | payload cannot express a **useful** fact |
| class | **conformance defect** | **diagnostic ergonomics** |

**`§4.2` in full says an arm subsumed by the union of earlier arms "is a
redundant-arm warning/error". It says nothing about what the diagnostic must
report.** ⇒ **Nothing in the spec is violated today.** A future reader who
finds this node next to the witness-payload one will be tempted to infer a
conformance gap by symmetry; there isn't one.

**That is why this is `draft` and unsized rather than `ready`.** Not because
the work is unclear -- the shape is a payload change plus three emission sites,
directly parallel to the node that closed the other side -- but because
**ergonomics competes with conformance work for the same ring, and that
priority call is not made by filing.**

## THE PRIORITY CALL IS MADE. This node is `ready`.

**The flip condition asked for Language's conformance-grounded backlog to be
clear. It is not clear -- it is GATED, which is a different fact and the one
that decides this.** Measured at `main` `6da108b6`:

| conformance-grounded Language candidate | why it cannot be released now |
|---|---|
| `LANG-CONVOY-ENCLOSING-FIELD` | needs an Architect ruling on the discriminator's shape |
| `LANG-MATCH-PATTERN-FORMS-ABSENT` | census filed `draft`; the cut is not made |
| `LANG-DECEQ-CHAR-LAWFUL-INSTANCES` | `gate: operator`, TCB question unanswered |
| `LANG-FOREIGN-NAME-FORMAT-CHARS` | `gate: operator`, threat model unanswered |

⇒ **Releasing this is a sequencing call, which is the Steward's (`ken-steward
§3`), not a priority call between ready WPs, which is the operator's.** Nothing
conformance-grounded is displaced, because nothing conformance-grounded can be
started. **If a ruling lands and unblocks one of the rows above, that node takes
the next turn -- it does not preempt this one mid-flight.**

**This does not reclassify the node.** It is still ergonomics, `§4.2` still
mandates only detection, and the table above this section stands exactly as
written.

## The parent's lesson, which transfers directly

**Do not widen the payload by convention.** Encoding arm indices into a
formatted `String` satisfies the letter and reproduces the defect, because the
next consumer still cannot tell a rendered description from structured data.
The parent's `AC-1` shape also transfers -- a test that fails against the old
payload and passes against the new one -- and there the discriminating case
needs a match with **at least two** earlier arms, since a single subsuming arm
makes "which arms" degenerate the same way a zero-arity constructor made "name
vs applied pattern" degenerate.

## Fixed inputs, measured at `main` `6da108b6`

Re-derive at your base.

- **The payload:** `crates/ken-elaborator/src/error.rs:214`,
  `ReachabilityError { span: Span }`.
- **The renderer:** `error.rs:547`.
- **Three production emission sites**, all in
  `crates/ken-elaborator/src/elab.rs`: `:1737`, `:2427`, `:8446`. These are the
  same three the Adversary independently enumerated as the `arm_used` mechanism
  at `evt_4d10j8tmjsbhj`.

## Deliverables

**`D1` -- give `ReachabilityError` a structured payload naming the subsuming
arms.** Structured, not rendered: the consumer must be able to read which arms
without parsing prose. The parent node chose its shape for the witness side;
follow that precedent unless you can say why it does not fit.

**`D2` -- supply it at all three emission sites.** If any site cannot name the
arms without a new traversal, **stop and report that site** -- the claim that
the information is in hand at the point of failure is a derivation from `§4.2`'s
matrix-walk description, not a measurement, and a site that refutes it is the
finding.

**`D3` -- the renderer at `error.rs:547` reports them.**

## Acceptance criteria

**`AC-1` -- a test that fails against the old payload and passes against the
new one, on a match with AT LEAST TWO earlier subsuming arms.** One subsuming
arm degenerates the question. Report the rendered diagnostic verbatim.

**`AC-2` -- the payload is structured.** A test reads the arms as data. A test
that only greps the rendered string does not discharge this and reproduces the
exact defect the parent node was filed for.

**`AC-3` -- detection is unchanged.** Every currently-red program stays red and
every currently-green one stays green. This node changes what the error *says*,
never what is *detected*.

**`AC-4` -- no-regression, in CI.** `COORDINATION §12`; build and test targeted,
`-p ken-elaborator`.

## Sizing

**`M`**, the parent's size, and for the same reasons: one payload, three
emission sites, one renderer, and a discriminating control. The parent landed at
that size with the same shape on the exhaustiveness side.

## Not this node

- **Not a change to reachability detection.** `§4.2`'s two subtleties -- a
  guarded arm does not cover, and a literal column never closes -- are
  behaviour, and **BOTH are vacuous today, not just the first.** Measured by
  enumerating the variants rather than by a grep that found nothing
  (Adversary, `evt_4d10j8tmjsbhj`, re-checked by the Steward at `6da108b6`):
  `MatchArm` is `{ pat, body, span }` with **no guard field**
  (`ken-elaborator/src/ast.rs:86`), and `PatKind` is exactly
  `Wild | Var | Ctor` with **no literal kind** (`ast.rs:167`).

  > **THE CONTINGENCY, WHICH IS THE PART WORTH CARRYING FORWARD.** The
  > reachability prose is accurate **because** those two surface features do not
  > exist. **Adding either makes both caveats live at once**, and `arm_used`
  > would then need the `§3.3` guard exception it does not have. The person who
  > adds guards or literal patterns will be reading `§4.2`, not this file --
  > which is why the same sentence is recorded in
  > [[LANG-MATCH-PATTERN-FORMS-ABSENT]], where that person is actually working.
  > **This node must not implement either feature**; it records the dependency
  > and stops.
- **Not an amendment to `34 §4.2`.** If the conclusion is that the spec *should*
  mandate the subsuming arms, that is a Spec-enclave question raised as one,
  never a deliverable here.
- **Not a general `ElabError` diagnostic-quality pass.**
