---
name: one-string-can-be-inert-for-one-identity-and-load-bearing-for-another
description: A new audit label was justified as restoring a distinction in `trusted_base()`, but that returns `Vec<GlobalId>` and the candidate's own test asserts the two entries were already distinct — while the same string does move canonical artifact bytes, so the rationale named the surface where the label is inert and omitted the one where it decides identity
metadata:
  type: feedback
---

# One string can be inert for one identity and load-bearing for another

**Measured 2026-08-15 on `320ef7e6c...c94f0319a`.**

> *"Labelling it identically would erase that distinction from `trusted_base()`,
> the one place it is otherwise recoverable."*

```rust
pub fn trusted_base(&self) -> Vec<GlobalId> { … .map(|d| d.id()).collect() }
```

⇒ **The name is not in the return value**, and ids come from `fresh_id()`, so the
two entries were **always** distinct there. ⇒ **And the same string IS a
canonical artifact-identity input.**

⇒ ***The same label does not participate in DECLARATION identity and does
participate in ARTIFACT identity.*** **When a change's whole deliverable is a
string, ask which identity it enters — there is usually more than one, and they
can answer oppositely.** The rationale named the surface where the label is
inert and omitted the one where it decides bytes.

## THE REFUTATION WAS INSIDE THE CANDIDATE

The same commit's test asserts

```rust
assert_ne!(accepted_id, ordinary_id, "the two exits must register distinct postulates")
```

⇒ **The candidate proves the ids already differ, before any label is read** —
and a landed kernel test elsewhere states the general form (*"owner labels are
not declaration identity"*). ⇒ ***When a rationale claims a change creates a
distinction, look for an assertion that the distinction already held.*** It is
often in the same diff, written by someone being careful about something else.

**The test's own doc had it right** — *"only the `Decl::Opaque` names recovered
via `env.lookup`"* — while the source comment did not. ***Two docs in one
candidate can disagree, and the more careful one is usually next to the
measurement.*** Quote it back rather than importing an outside standard.

⇒ **State what the change DOES buy.** Here: *semantic* distinguishability on
lookup — a reader learns **why** two entries differ, not **that** they differ.
**A finding that only says "your reason is wrong" invites deleting the change;
naming the real benefit protects it.**

## A NEW LABEL THAT PREFIXES AN OLD ONE DISSOLVES THE DISTINCTION IT ADDS

`"prover unknown goal"` versus `"prover unknown goal -- FO: …"`. Censused: only
exact-equality consumers today, so **not live**. **But audit-surface reading is
a human-and-grep activity**, and any `contains`/`starts_with` match absorbs the
new label into the old class — **undoing the node's entire purpose**. ⇒ **A
distinguishing label must not begin with the string it distinguishes itself
from.**

## AN UNCONDITIONAL RETURN IS NOT A GATE THAT CAN ERODE

Asked whether a gate "never under load" would survive being unmasked: it is an
unconditional `return` in its branch, so **there is no condition to weaken**.
⇒ ***Re-aim the question at what could actually go wrong***: not this gate
weakening, but a **second exit** appearing that does not route through it. **That
converts a vague forward worry into a structural check on a future diff.**
