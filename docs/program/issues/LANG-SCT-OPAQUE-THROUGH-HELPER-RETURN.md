---
id: LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN
title: "Ken's SCT termination checker traces a structural decrease only through a direct pattern match feeding the recursive call, so factoring a shared guard into a non-recursive helper reds NotTerminating -- forcing duplication at exactly the sites a checker wants one guard"
status: ready
owner: language
size: S
tier: T2
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-16, on Architect evt_2ee9qfch79vgg reviewing V3-FO-KEN-LEVEL-CHECKER-AUTHORING D2: 'a real Ken ergonomics data point, not a defect in this WP ... worth a line to the Language/Ergo track as an observed limitation with a concrete reproduction, rather than leaving it as a comment in one catalog file.' Reproduction re-verified by the Steward against candidate 7726c108c and against origin/main c8fa12c9b before filing."
---

 # RELEASED 2026-09-10 (Steward). KERNEL-CONV-CONGRUENCE-CLOSURE MERGED
> # (0c68628f5), the temporary L2 kernel reseat retired, and the L2 slot returned
> # to the language ring — so this, the effective releasable language head, is
> # released. status draft->ready; kicked the language ring. Tier T2: the RING
> # work is reproduce + document; the D0 fork (defect / documented-limitation /
> # intended) is the ARCHITECT's call, routed to it. Coordinates below were
> # RE-MEASURED at 0c68628f5 (they had drifted from the filing SHA).
> #
> # L2 LANGUAGE RESUME HEAD (framed 2026-09-10). Was QUEUED; now the effective
> # releasable language head.
> #
> # After KERNEL-CONV-CONGRUENCE-CLOSURE lands and the L2 slot reseats from the
> # kernel ring back to language, this is the effective releasable language head:
> # the operator's L2 queue top three (pattern-forms LITERALS, membership,
> # deceq-char) are all blocked or deferred (membership recut NOT BUILDABLE;
> # literals + deceq-char gate on the operator's numeric-equality TCB question).
> # The Steward releases THIS then — flip status draft->ready + kick the language
> # ring. Tier T2: the RING work is reproduce + document. The D0 FORK (defect /
> # documented-limitation / intended) is the ARCHITECT's call, routed to it; if
> # D0 concludes DEFECT it becomes a SEPARATE kernel TCB node with a soundness
> # review (AC-3), NOT this node. Coordinates below are as-of-filing; D0
> # re-measures at pickup (sct.rs line numbers drift).
> #
> # Original filing context (operator priority 2026-08-15): filed so the twice-hit
> # limitation is not lost. [[V3-FO-KEN-LEVEL-CHECKER-AUTHORING]] handled it
> # correctly by inlining; this node is about the limitation, not that candidate.

## The observed limitation

**The kernel's SCT checker traces a structural decrease only through a direct
pattern match feeding the recursive call — not through the return value of an
intervening non-recursive helper.**

Measured empirically during `V3-FO-KEN-LEVEL-CHECKER-AUTHORING` `D2`. An earlier
draft factored the *exactly one child* guard through

```
fok_single_cert : List FokCert -> Option FokCert
```

and the kernel rejected the mutual-recursion clique with:

```
NotTerminating("SCT: idempotent self-loop has no strictly-decreasing parameter")
```

The clique is `fok_check_tree` -> `fok_check_rule` -> `fok_check_forall_right`
-> `fok_check_tree`. **The fix was to inline the singleton check as a direct
`Cons`/`Nil` match at each of the two call sites**, restoring the
destructure-feeds-recursive-call relationship. That is what landed.

**Verified coordinates:**

Re-measured at `origin/main` `0c68628f5` (the frame's original coordinates had
drifted; re-measure again at pickup):

| what | where |
|---|---|
| the recorded finding | `catalog/packages/Tooling/Verification/FoKripke.ken:484` (the "factoring it out reds `SCT: idempotent self-loop has no…`" note) |
| the two inlined sites | same file, `fok_check_forall_right:969`, `fok_check_rule:991` |
| the checker | `crates/ken-kernel/src/sct.rs` |
| the acceptance criterion | `sct.rs:7` — *"Accept iff every idempotent self-loop has ≥1 `↓` on the diagonal"* (the module doc, lines 1-7, is D1's candidate home; the exact error string is emitted at `sct.rs:753`) |

## This is the second recorded encounter, which is the reason to file it

The same error string is already documented at
`crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs:564`
(re-measured at `0c68628f5`; was `:553` at filing).

⇒ **The limitation has now been hit twice, in unrelated work, and both times the
record was left as a comment beside the workaround.** The next author will
rediscover it from scratch, because a limitation recorded at its point of
occurrence is findable only by whoever is already standing there.

## What this node is for

**`D0` — decide whether this is a defect, a documented limitation, or intended.**
That is the fork, and it is not the Steward's to settle. SCT is a deliberately
conservative, small-TCB criterion (`docs/PRINCIPLES.md`: small auditable TCB), so
*"the checker is incomplete here and that is the accepted price"* is a legitimate
answer. **It is only unacceptable that the answer is unwritten.**

**`D1` — record the answer where an author hits it**, not in a catalog file.
`sct.rs`'s own module doc is the candidate home.

## Acceptance criteria

**`AC-1`. The reproduction is stated, not just the rule.** A one-paragraph
description of what SCT cannot see is the shape that already failed twice —
carry the helper signature, the clique, and the exact error string.

**`AC-2`. If the answer is "intended", say what to do instead.** *"Inline the
guard at each call site"* is the working remedy and belongs in the record.
**A limitation documented without its workaround is half a document.**

**`AC-3`. No change to SCT's acceptance criterion** unless `D0` concludes defect,
which would be a kernel change and a different node with a soundness review.
**Widening a termination checker is TCB work** and does not happen as a
by-product of documenting one.

**`AC-4`.** No-regression, in CI (`COORDINATION §12`). Targeted local validation
only.

## Banned scope

- **Rewriting `FoKripke.ken`'s inlined guards.** They are correct and merged.
- **Widening SCT** — see `AC-3`.
- **Treating this as a finding against
  [[V3-FO-KEN-LEVEL-CHECKER-AUTHORING]].** It is not; that ring diagnosed the
  limitation, worked around it correctly, and reported it.

## The general shape, worth keeping whichever way `D0` goes

**A totality checker that cannot see through a helper forces duplication at
exactly the places a checker wants a shared guard** (Architect,
`evt_2ee9qfch79vgg`). The pressure is toward copies of a guard in a language
whose whole proposition is that the machine checks what you wrote — and the
duplication it forces is invisible in review, because each copy is individually
correct.
