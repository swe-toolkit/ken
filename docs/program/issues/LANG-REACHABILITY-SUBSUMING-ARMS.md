---
id: LANG-REACHABILITY-SUBSUMING-ARMS
title: "`ReachabilityError` carries only a span, so a redundant-arm diagnostic cannot say WHICH earlier arms subsume the dead arm -- the mirror of the gap `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD` closed on the exhaustiveness side, except `34 §4.2` does NOT mandate it, so this is ergonomics and must not be filed or read as a conformance obligation"
status: draft
owner: language
size: unsized
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect carry registered in the LANG-MATCH-DIAGNOSTIC-PROSE approval (dec_75pvygp3aatab, resolved 2026-08-14T16:32:12Z), where he wrote 'Registering the shape only; sizing and cut are not mine.' Filed by the Steward because a carry recorded only in an approval verdict has no node and evaporates -- the same reason RT-CONTKEY-REFUSAL-PROFILE-SPLIT was filed the same day. Measured at origin/main before c2f285ee merged."
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

## Flip condition

**Flip to `ready` and frame it when Language's conformance-grounded backlog is
clear, or when a user-facing report makes the ergonomic cost concrete.**
Language's ungated queue at filing time is `LANG-WITNESS-ARITY-DERIVED`
(`ready`), and two `gate: operator` nodes.

**If it is framed, the parent node's hard-won lesson transfers directly and
should be written into the frame:** do **not** widen the payload by convention.
Encoding arm indices into a formatted `String` satisfies the letter and
reproduces the defect, because the next consumer still cannot tell a rendered
description from structured data. The parent's `AC-1` shape also transfers --
a test that fails against the old payload and passes against the new one --
and there the discriminating case needs a match with **at least two** earlier
arms, since a single subsuming arm makes "which arms" degenerate the same way
a zero-arity constructor made "name vs applied pattern" degenerate.

## Not this node

- **Not a change to reachability detection.** `§4.2`'s two subtleties -- a
  guarded arm does not cover, and a literal column never closes -- are
  behaviour, and **guards are not implemented in the surface at all**, so the
  first is vacuous today.
- **Not an amendment to `34 §4.2`.** If the conclusion is that the spec *should*
  mandate the subsuming arms, that is a Spec-enclave question raised as one,
  never a deliverable here.
- **Not a general `ElabError` diagnostic-quality pass.**
