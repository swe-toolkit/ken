---
id: LANG-ATOM-START-CLASSIFICATION-CLOSURE
title: "Make what-starts-an-atom ONE classification over ATOM FORMS, applied to every atom-start roster in the parser, so the places encoding it cannot drift apart -- the measured omissions (KwProof, TruncBar) close as a consequence, not as the deliverable. The first proposal, a total function over Token, FAILED: a Token-total map sends Ident(_) to atom and is finished, so the closure property holds for token-keyed forms and silently fails for contextual multi-word ones like `recursive result for xs`. Covers three roster pairs across both the expression and the type side, one of them uncensused, plus re-validating the parser.rs:3122-3125 infix-path soundness argument that reasons FROM the roster being narrow and whose premise this repair edits from 600 lines away."
status: ready
owner: language
size: L
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Raised by the Architect from their own carry list (evt_a1t4jpv9tvc0); two Spec-enclave rulings settle the objective and the node does not re-litigate them (frame section 2). Frame landed 2026-09-17 at main b0421afd0. RELEASED 2026-09-17 by the Steward. Runs CONCURRENTLY with LANG-STANDARD-INFIX-CALL-COMPLETION -- language-leader answered the sequencing question with a measurement rather than an estimate (evt_93qgz56ye40w): A1's remaining work touches standard_operators.rs, modules.rs, elab.rs, error.rs, numbers.rs with ZERO occurrences of parser.rs, and its landed work so far is 5 files +336/-0, also zero parser.rs. One flagged interaction, this node's to own: A1's D0 touches parser.rs:3131, one of the two call sites this classification replaces -- coordinate on that line specifically before either side edits it. AMENDED at release with AC-0 (Architect, evt_11nyyh216pqz2): every defect measurement in sections 3, 4, 4b and 5 is anchored at 6acd40705 while the implementation base is later, and no criterion required the defect to be re-confirmed there -- ten criteria about the fix, zero about the premise. AC-0 re-runs the frame's own two-sided probes at the implementer's own base and STOPS if any bare case parses."
---

> # RELEASED 2026-09-17 (Steward).
>
> The frame is `docs/program/wp/LANG-ATOM-START-CLASSIFICATION-CLOSURE.md` and
> it is the operative text. This file tracks status only.
>
> **Base: not a SHA.** The frame's header deliberately does not pin one — the
> publish queue moves `main` between release and the cut, and
> `LANG-STANDARD-INFIX-CALL-COMPLETION` is live in the same file. Cut from
> `origin/main`, **name the SHA in your first post**, and run **AC-0** there.
>
> ## AC-0 IS NEW AT RELEASE, AND IT RUNS BEFORE AC-1.
>
> Re-run the frame's own two-sided probes at **your** base and paste the
> output. If any bare case PARSES, **stop and return to the Steward** — the
> node's premise has changed, which makes its scope wrong rather than merely
> smaller.
>
> Raised by the Architect, who supplied most of the frame's evidence and named
> why they were the wrong seat to catch it: *"having spent the evening
> establishing that the defect is real, 'what if it isn't' is the question I
> was least able to ask."* The general form is
> `agent/memory/fleet/a-frame-controls-its-repair-and-never-its-premise.md`.
>
> ## DELIVER IN INCREMENTS — each is a COMPLETE turn.
>
>     AC-9 alone   the type side's two negative guards as form-keyed start
>                  conditions. THE HARD PART -- do it FIRST. If the encoding
>                  cannot express a negative start condition cleanly, that is
>                  a finding, not a failure.
>     AC-6 alone   the contextual-form enumeration.
>     AC-4 alone   the parser.rs:3122-3125 infix-path argument, restated
>                  against the widened classification or shown never to have
>                  depended on the roster for operator tokens.
>
> **Post any one of them and stop** rather than carrying a half-finished
> closure alongside unrun evidence.
>
> ## TWO MEASUREMENTS ARE CLOSED. DO NOT RE-DERIVE THEM.
>
> The pattern-roster census (section 4c, clean) and the contextual selectors at
> all three positions (section 4, clean) are recorded **as measurements with
> their instruments named** — not as assumptions, and not as open work.
>
> Reviewers: language QA + Architect. Then Steward M1-M3a, lieutenant M4-M9.
