---
name: an-owner-fork-can-be-closed-by-reading-when-running-the-repro-is-forbidden
description: A divergence was dispositioned to the language track with no TCB contact before the frame was located, and the frame is a full normalize call at the head of the kernel's strict-positivity check — located by reading, because the reproduction was resource-banned, and confirmed by explaining all five of the finding's isolating facts
metadata:
  type: feedback
---

# An owner fork can be closed by reading when running the repro is forbidden

**Measured 2026-08-16 on `c67201f2f..8d6d7d545`.** A merge reported an
elaborator divergence, dispositioned it to the Language track as
*liveness-not-soundness with no TCB contact*, and said plainly that the
conclusion was drawn **before the diverging frame was located**.

⇒ **The reproduction was resource-banned**, so the usual move — run it and
watch — was unavailable. ⇒ ***The bisection table in the finding was enough to
locate the frame by reading***, because a correct attribution has to explain
**every** isolating observation, and a wrong one will fail at least one.

```
inductive.rs:442   check_pos_arg(...)                              // per ctor argument, UNGATED
inductive.rs:97        let normalized = normalize(env, &Context::new(), a);   // FULL, not whnf
```

(At that SHA; `check_pos_arg` has since moved to `whnf`.) **Five for
five**, including the one the author had used to *exclude* a cause:
*"non-indexed, non-self-referential diverges identically"* is explained by
**`normalize` running before any occurrence test**.

⇒ ***An observation used to rule a cause OUT is often the one that points at the
real frame.*** The author excluded indexing and self-reference; that exclusion is
precisely the signature of a check that normalizes first and inspects second.

## SEPARATE THE THREE PARTS OF A DISPOSITION AND RULE ON EACH

*Liveness-not-soundness* held. *Language-track owner* and *no TCB contact* did
not — the frame is kernel code in the inductive **admission gate**. ⇒ **A
disposition is several claims wearing one sentence**; **grant the ones that
survive explicitly**, or a two-thirds correction reads as a wholesale rejection.

## SAY WHAT YOU DID NOT RUN, AND PRICE THE CONFIRMATION

⇒ **A read-based attribution must be labelled as one**, however well it fits.
**Then name the cheapest confirmation compatible with the ban** — here, a
counter at the suspected line plus the external bound the ring had already
tooled, so entry count and memory climb can be correlated **without lifting the
resource prohibition.**

**A second finding at the same line is worth stating separately**: the call
normalizes an **open** term under an **empty** context, while the term's free
indices refer to earlier telescope entries. **Whether or not it causes the
divergence, do not fold an independent defect into the one you came for.**
