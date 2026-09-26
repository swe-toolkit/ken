---
name: an-empty-discriminator-population-is-scoped-to-the-file-it-was-swept
description: QA swept a whole test file for a program distinguishing the shipped guard from the prohibited alternative and found none, so the design choice was recorded as unwitnessed — the discriminating program was the fixture already in that file plus one `let`, and it is the shape the design rationale itself names
metadata:
  type: feedback
---

# An empty discriminator population is scoped to the file it was swept

**Measured 2026-08-15 on `f08388396`, on a gap the Steward stated honestly:
*"the region-set-versus-positional-floor choice is behaviourally
unwitnessed … the file-wide discriminator population is empty, reproduced
independently."***

Both the implementer and QA replaced the shipped membership guard with the
prohibited positional floor and ran the whole acceptance file: **identical
results.** True, and reproduced twice.

⇒ **The design rationale states the discriminating shape in the same breath:** a
positional floor fails *"because an intervening `let`/`λ` pushes a genuine outer
binder above the enclosing field region."* **Writing that program — the existing
fixture plus one `let` — separates the two guards immediately:**

| guard | plain fixture | with the interleaved `let` |
|---|---|---|
| shipped region set | `Ok` | `TypeMismatch` on the index |
| prohibited floor | `Ok` | **`NotTerminating` — SCT loses its decreasing param** |

⇒ ***A sweep of an existing file answers "does any CURRENT test discriminate",
never "does a discriminating program exist."*** **When a design choice is
recorded as unwitnessed, read the rationale for the shape it rejects the
alternative on, and write that shape.** The rationale is a specification for the
missing witness, and it is usually one construct away from a fixture already
present.

**The direction can favour the shipped choice, and saying so matters.** Here
the floor did not merely diverge — it degraded a type-index failure into a
**termination-checking** failure, the soundness-adjacent gate. **An adversary
who only reports gaps trains the reader to expect the witness to be damaging;
report it when the witness vindicates the decision.**

## THE SAME PROGRAM ANSWERED A SECOND, SEPARATELY-FILED QUESTION

A neighbouring node was filed with *"it is not yet established that this is
independent of the merge above."* The same probe answers it for one variant:
under the shipped guard the interleaved program fails with **the exact
pre-remedy signature** — same head, one de Bruijn index apart — so it is not
independent, it is the original defect standing on a shape the remedy does not
cover.

⇒ **A discriminator built for one gap is often evidence for a neighbouring one**,
because both gaps are about the same mechanism's edges. **Run the probe against
every open question on that mechanism while it is in hand.**

**And check whether your repro is the filed repro.** The new probe dies in the
kernel; the filed one dies earlier, in branch-goal classification. **Two
failures at different depths may be one shape or two, and the difference decides
whether the node has one deliverable or two** — one read of the filed repro
against the new one settles it; name the difference rather than assuming it
away.

Related: [[a-tools-silence-is-scoped-to-the-question-it-asks]] (a sweep's
silence answers "does any current test discriminate", not "does a
discriminating program exist").
