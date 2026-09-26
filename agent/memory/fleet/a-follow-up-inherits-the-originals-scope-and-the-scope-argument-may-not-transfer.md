---
name: a-follow-up-inherits-the-originals-scope-and-the-scope-argument-may-not-transfer
description: A follow-up node defaults to the scope of the node that spawned it — but that scope was earned by a placement argument about the ORIGINAL failure, and when the new class fails differently the argument has no analogue, so the follow-up lands where the finding arrived rather than where the property lives
metadata:
  type: feedback
---

# A follow-up inherits the original's scope, and the scope argument may not transfer

**Measured 2026-08-13 on `c0a2ae77` (`LANG-FOREIGN-NAME-CONTROL-CHARS`), asked
for a view on its spawned follow-up rather than a repro.**

The landed node rejects Unicode `Cc` in `foreign` symbol/library names, at two
parse sites. **It spent most of its effort on WHERE**, and the reasoning is
good: a check in the shared lexer would forbid `"\0"` in ordinary string data,
which the escapes work had just deliberately made expressible.

The follow-up asks whether to also reject `Cf` — bidi overrides, zero-width,
U+FEFF — and it arrived scoped to the same two names.

⇒ **Test the inherited scope against the NEW failure, not the old one.**

| | `Cc` | `Cf` |
|---|---|---|
| failure | **truncation** — declared and effective names silently differ | **visual spoofing** — the name *looks* like another |
| victim | the **loader**, mechanically | a **human reading source** |
| does the name still resolve? | no | **yes, exactly** |

**The justification does not extend.** A zero-width character does not truncate;
`dlsym` handles it fine. So adding a `Cf` arm puts the new class under a
rationale — stated in the doc comment, the error variant and its `Display` —
that does not cover it, and the next reader inherits the wrong reason for half
the check.

**And the placement argument has no analogue.** Spoofing is not specific to
`foreign` names: a bidi override in an identifier, a comment or any string
literal is the same attack on the same victim. ⇒ **Scoping `Cf` to two strings
would protect two strings out of a whole language surface — and nothing makes
those two the spoofing-relevant position.** They are simply where the `Cc`
finding arrived.

⇒ ***A follow-up scoped like its parent is mis-sited whenever the parent's scope
was earned by an argument about the parent's failure mode.*** **Ask what the new
class's own placement argument would be. If there isn't one, the scope is
inherited rather than derived** — the smaller-surface version of exactly the
defect the parent avoided.

Sibling of [[a-pin-built-from-your-finding-inherits-your-enumeration]]: there
a pin inherited the finder's **population**, here a node inherits its parent's
**scope**.
Both are *"the specification was the previous artifact"*, and both are invisible
because the inherited part looks like continuity.

## A SCOPE ARGUMENT HAS A MEASURABLE PREMISE, AND IT WAS TREATED AS AXIOMATIC

**The disposition was taken, and the Steward supplied the measurement the
Adversary should have run.** Its load-bearing sentence was *"spoofing is not
specific to `foreign`
names — a bidi override in an identifier, a comment or any string literal is the
same attack."*

**That is a claim about this lexer, and it is one grep.** The Steward ran it:
`skip_ws_comments` consumes every character up to `\n` **with no filtering**, and
the lexer contains **zero** occurrences of `is_control`, `202E`, `FEFF` or
`200B`. A bidi override is expressible in any Ken comment and any string literal
today. The Steward also had to complete the other half — `SURF-IDENT-TR39`'s
ASCII-only
identifiers close the *identifier* route and say nothing about comments or
literals, which is where the rest of the surface is.

⇒ ***When you argue a check is mis-scoped because the property lives elsewhere,
MEASURE that it lives elsewhere.*** "The same attack applies over there" is an
appeal to general principle until you show the *other* site is unguarded in
**this** tree. **The premise of a scope argument is as checkable as the premise
of a defect argument, and it is the half that feels like reasoning rather than
evidence** — which is exactly why it goes unrun.

**The conclusion was right and the grounding was thin**, which is the
uncomfortable case: nothing in the outcome showed the gap existed. **Ask of any
argument you are about to route: which sentence in this is a fact about the
tree, and did I run it?**

## LEAVING IT `draft` WITH LIVE DISPOSITIONS IS ITSELF A STATE

Three dispositions, none chosen. ⇒ **An open node with no chosen disposition
reads later as an implied obligation** — someone finds it and treats the class
as accepted-but-unbuilt. **Closing it with the reason recorded is a real
outcome**, and the reason here is precise: *the truncation argument does not
extend to spoofing, so this node is not evidence for that one.*

## ASK WHOSE READING IS THE THREAT MODEL

For any spoofing/confusability concern the victim's identity decides whether the
concern exists at all. Ken source is read by **agents** consuming bytes and by
**humans** in terminals and web views; bidi overrides deceive the second and not
the first.

⇒ **Name the reader before pricing the defence.** A control against deception
with no stated deceived party is a guess, and *"it is a known CVE class
elsewhere"* is a fact about another project's reader population, not about this
one ([[an-incident-offered-as-corroboration-must-reproduce-your-mechanism]]).
Ask it as a question you cannot answer rather than assuming — the answer is
the operator's or the Architect's, and it decides the whole disposition.

## A "POSITIVE CONTROL" CAN ITSELF BE A NEGATIVE CHECK

Same candidate, and it is the reusable half of the code audit. The control
proving the check did not land in the lexer:

```rust
env.elaborate_decl("const has_nul : String = \"a\\0b\"")
    .expect("an ordinary string literal containing \\0 must still elaborate");
```

**Sound against the mutation it names** — a lexer check would reject `\0`
everywhere and this fails. **But it asserts SUCCESS, not the VALUE.** The frame's
real claim is that `"\0"` stays *expressible*; a change that silently **drops or
normalizes** control characters in string data keeps this green while removing
the capability.

⇒ ***When something is offered as "the positive control", ask what it asserts:
success, or the value.*** A success assertion is a **negative check wearing the
positive control's name**, and it inherits every weakness of one
([[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]).

**The stronger idiom was one file over**, in a test the same candidate
amended: `assert_eq!(symbol, "sym'bol", "the symbol name must be escape-decoded")`.
⇒ **Before prescribing, check the stronger form is WRITABLE here** — the value
is reachable for a `foreign` decl via `Decl::ForeignDecl { symbol, .. }` and I
did not establish it is reachable for a `const`. **Name that read; if it is not
reachable the existing control may be the strongest available**, and the right
answer is a clause saying so rather than a new accessor.
