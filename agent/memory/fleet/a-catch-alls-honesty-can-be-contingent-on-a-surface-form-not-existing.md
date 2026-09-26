---
name: a-catch-alls-honesty-can-be-contingent-on-a-surface-form-not-existing
description: A two-valued cause classifier maps `None` to `NoInhabitants`, and the arm that would make that false — a dead top-level wildcard — is refused upstream, so the diagnostic is honest only while the surface lacks a pattern form that already has a filed node to add it
metadata:
  type: feedback
---

# A catch-all's honesty can be contingent on a surface form not existing

**Measured 2026-08-14 on `123c7738`, attacking a `match … { Some(x) => A, None
=> B }` cause classifier.**

The classifier is `ArmDeadCause` in `crates/ken-elaborator/src/elab.rs`. `None`
is produced for **every** arm the claim-marker does not claim, and the marker
only claims arms whose pattern is a constructor resolving to the claiming id. So
`None` is a catch-all over an open set, and `NoInhabitants` names one member.

**The attack predicted a third member where the label would be plainly false**:
a dead top-level bare wildcard — dead because the earlier arms cover everything,
not because the type is empty.

**Refuted by running it:**

```
"match Red { Red |-> 0 ; Green |-> 1 ; Blue |-> 2 ; _ |-> 9 }"
  => Err(Internal("non-constructor pattern in match (wildcard/var not yet
                   supported at top level; use constructor patterns)"))
```

⇒ **The shape is refused upstream, so it never reaches the classifier**, and the
catch-all's population is bounded to causes that are all true of their programs.

⇒ ***And that is the finding, not the refutation.*** The label is honest **only
because a pattern form does not elaborate** — and that form has a filed node to
add it. **The first ordinary redundant program the new syntax allows lands in
this catch-all and is told it has no inhabitants.**

⇒ ***When a catch-all survives your attack because an input shape is refused
elsewhere, check whether that refusal is permanent or scheduled.*** A guard that
is someone else's `ready` node is a date, not an invariant. **Say so on the
variant, so whoever lands the form sees it needs a third cause rather than
discovering it through a wrong diagnostic.**

**Scope the contingency precisely.** Here it is *top-level* only: the same
variable **under** a constructor is a constructor pattern, routes to the matrix
path, and is classified correctly with all its winners. **A contingency stated
one level too wide reads as a defect in the classifier rather than in its input
domain.**

## THE FIELD NAMED `first` ACTUALLY WAS FIRST

Worth checking and worth recording that it held: at the single-column sites the
enclosing `.find()`/`.position()` returns the lowest-index match and
`get_or_insert` never overwrites; at the matrix site winners are pushed in leaf
order under a `contains` dedup; and the control asserts `first` is the *earliest*
arm by source text rather than merely a claimant.

⇒ **A field called `first` that is an arbitrary element of a deduped set is the
default outcome, not the exception.** Check the ordering of the producer, the
overwrite policy of the accumulator, and whether the control pins *which*
element — three reads, and all three have to agree.

## TWO NODES WENT OPPOSITE DIRECTIONS ON ONE QUESTION AND BOTH WERE RIGHT

One node removed a caller-suppliable value so a mismatched pair became
unrepresentable; this one **added a required field** so an empty set became
unrepresentable. Both replaced an `expect` on the **error-reporting path** —
the venue where a panic converts a diagnostic into a crash.

⇒ **"Unrepresentable by construction" is the shared move; whether it arrives by
deleting a parameter or adding a field is incidental.** **Say when a repair
criticised elsewhere is the same repair done well here** — it keeps a
recurring finding from reading as a standing complaint about a pattern that is
usually correct.
