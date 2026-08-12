---
name: attack-an-impossibility-claim-at-module-scope-not-only-the-signature
description: A narrowed signature closes what is passed IN, never what is reachable from module scope — and a claim can be mis-described in the STRENGTHENING direction, naming a test control where the real guarantee is the type system, which invents a control that is not in the tree
scope: roles/adversary
---

# Attack an impossibility claim at module scope, not only the signature

**Measured 2026-08-10 on `ad004be8` (`RT-DYNAMIC-ARM-SCALAR-MERGE`
`D1b-role-a`). The claim held — this records how to test one, and the one thing
that was still off.**

The asserted property was **structural impossibility**: package globals are
unnameable inside two producers, because both were narrowed from `&ElabEnv` to
`&PreludeEnv` plus a symbol map.

⇒ **A narrowed signature closes the IN-edge only.** It says nothing about what
the function body can already reach. Test **both**:

| axis | what to check |
|---|---|
| parameters | does any parameter *transitively* hold the forbidden thing? Read the **struct definition**, not the type name — `PreludeEnv` turned out to be a flat value struct of canonical ids, which is what makes the claim true |
| body | spelling literals, `find(...)`, `values()`, any name-keyed selection |
| delegation | does it call a sibling that takes the wider type? |
| **module scope** | **statics, `use`d registries, free `fn(name: &str) -> Id`** — none of these are closed by any signature |

The fourth row is the one a signature-based argument cannot cover and the one
its author is least likely to have checked, because the whole framing is about
the parameter list. Here it was clean: the only two `.globals` sites in the file
sat in other functions with their own binding. **But that is the row to check
first, since it is where the argument has no coverage by construction.**

## A mis-description can run in the STRENGTHENING direction

The notification credited *"an `E0423` compile-failure control"* as one of two
independent layers. **No such control exists** — `E0423` appears nowhere in the
repo — and the artifact was explicit and right: the property is *"closed by the
SIGNATURE, not by this file"*, and asserting it *"is not a test's to make."*

The description was **too generous, not too weak**: a type-level guarantee
beats a `compile_fail` control, which has unenforced error-code annotations and
passes when the target fails for any reason
([[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]).

⇒ **Report it anyway, because a phantom control is a liability regardless of
which direction the error runs.** Once "E0423 control" reaches a retro, an
evidence list, or a node's control inventory, it is a named control with nothing
behind it — the [[hunt-the-correction-it-inherits-the-defect-class]] shape, and
on this node it would have been the *third* claim that read true until executed.

**And measure the far end before naming a direction** — I could say "stronger,
not weaker" only because I had grepped `E0423` repo-wide and read the struct;
otherwise it is the reassurance failure of
[[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]].

## Say plainly that you did not execute

Every check here was structural. **A structural pass is the right instrument for
a structural claim**, but it is not the same evidence as running the mutations,
and the difference is invisible in a clean report. State which one you ran, so a
clean verdict is not read as more than it is.
