---
scope: fleet
audience: (see scope README) — anyone about to treat a lint's silence, a
  green build, or an empty diff as evidence for something broader than the
  question the tool actually asks
source: CB-HYGIENE, 2026-07-22 — Architect-rejected merge Decision
---

# A tool's silence is scoped to the question it asks

An `unused_imports` warning answers *"does anything name this today"*,
never *"what is the declared surface."* Treating a tool's absence-of-
complaint as authority on a broader question is a distinct defect from a
buggy instrument — falsifying the instrument would not catch it.

Moving `Px8trNestedRouteObject`
(`crates/ken-runtime/src/cranelift_backend.rs`, grep the symbol; also
`crates/ken-runtime/src/cranelift_backend/test_objects.rs`) into a private
child module, the facade re-export was omitted and justified in the code:

> *"The struct keeps its declared `pub(crate)` visibility at its new home;
> only its reachable path narrows, **which the compiler confirms is
> unobserved**."*

The evidence was an `unused_imports` warning on the re-export. The
Architect rejected the merge Decision:

> *"An `unused_imports` warning is evidence of no current named consumer,
> **not authority to narrow a declared surface**."*

The type had been nameable at
`crate::cranelift_backend::Px8trNestedRouteObject`. `pub(crate)` on a
declaration inside a **private** module does not preserve that reach — the
surface narrows even though the visibility keyword is unchanged. That the
one consumer inferred the return type without naming it proved only that
today's call compiles.

**Why this is its own defect class.** It is *not* the
instrument-narrower-than-the-claim family. The warning was **real,
accurate, and correctly computed**; falsifying or mutation-testing it would
have confirmed it. The error was **promoting a correct answer to question A
into authority on question B**. No amount of verifying the tool catches
that — only naming the question does.

The same session produced the mirror-image failure for comparison: an item
enumerator that omitted `impl` (a genuinely broken instrument, caught by
`E0624`). Different fix. That one needs falsification; this one needs
translation.

**How to apply:**

- Before treating any **absence of complaint** as evidence, state in one
  sentence **the question the tool actually asked**, then check it is the
  question being answered. Write it down; the gap is invisible otherwise.
- Common instances of the same shape:
  - `unused_imports` / dead-code → "nothing names it *today*", not "the
    surface may be narrowed".
  - A green `-p <crate>` build → scoped to that crate; blind to cross-crate
    text oracles and feature-gated regions.
  - An empty `git diff` → scoped to the paths passed.
  - A passing suite → scoped to what it asserts, not to the property.
- **Surface/reach questions are never answered by consumer counts.** "Who
  calls it now" and "what could name it" are different questions; only the
  second is about surface. Preserve the path, not the current usage.
- When you catch yourself writing *"the compiler confirms…"*, check whether
  the compiler was asked. It confirms compilation. It confirms nothing
  else.

Related:
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]
(the sibling — there a *correct* answer was promoted to a question it
didn't ask; here a *negative* answer is accepted without asking which
question produced it), [[grep-the-producer-not-the-cited-proxy]].

## Instance: an empty discriminator population is scoped to the file swept

*Merged from the former fleet lesson
`an-empty-discriminator-population-is-scoped-to-the-file-it-was-swept`
(2026-09-27 scope pass).* The tool here is a sweep of an existing test file,
and its silence was promoted from "no current test discriminates" to "no
discriminating program exists".

**Measured 2026-08-15 on `f08388396`, on a gap the Steward stated honestly:
*"the region-set-versus-positional-floor choice is behaviourally
unwitnessed … the file-wide discriminator population is empty, reproduced
independently."*** QA had swept the whole test file for a program
distinguishing the shipped guard from the prohibited alternative and found
none, so the design choice was recorded as unwitnessed.

Both the implementer and QA replaced the shipped membership guard with the
prohibited positional floor and ran the whole acceptance file: **identical
results.** True, and reproduced twice.

The design rationale states the discriminating shape in the same breath: a
positional floor fails *"because an intervening `let`/`λ` pushes a genuine
outer binder above the enclosing field region."* **Writing that program (the
existing fixture plus one `let`) separates the two guards immediately:**

| guard | plain fixture | with the interleaved `let` |
|---|---|---|
| shipped region set | `Ok` | `TypeMismatch` on the index |
| prohibited floor | `Ok` | **`NotTerminating`: SCT loses its decreasing param** |

- **A sweep of an existing file answers "does any CURRENT test
  discriminate", never "does a discriminating program exist."** When a design
  choice is recorded as unwitnessed, read the rationale for the shape it
  rejects the alternative on, and write that shape. The rationale is a
  specification for the missing witness, and it is usually one construct away
  from a fixture already present.
- **The direction can favour the shipped choice, and saying so matters.** Here
  the floor did not merely diverge: it degraded a type-index failure into a
  **termination-checking** failure, the soundness-adjacent gate. An adversary
  who only reports gaps trains the reader to expect the witness to be
  damaging; report it when the witness vindicates the decision.
- **A discriminator built for one gap is often evidence for a neighbouring
  one**, because both gaps are about the same mechanism's edges. A
  neighbouring node was filed with *"it is not yet established that this is
  independent of the merge above."* The same probe answers it for one
  variant: under the shipped guard the interleaved program fails with **the
  exact pre-remedy signature** (same head, one de Bruijn index apart), so it
  is not independent; it is the original defect standing on a shape the remedy
  does not cover. Run the probe against every open question on that mechanism
  while it is in hand.
- **Check whether your repro is the filed repro.** The new probe dies in the
  kernel; the filed one dies earlier, in branch-goal classification. Two
  failures at different depths may be one shape or two, and the difference
  decides whether the node has one deliverable or two. One read of the filed
  repro against the new one settles it; name the difference rather than
  assuming it away.
