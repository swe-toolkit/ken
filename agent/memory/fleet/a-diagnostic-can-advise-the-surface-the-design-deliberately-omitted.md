---
name: a-diagnostic-can-advise-the-surface-the-design-deliberately-omitted
description: A new surface syntax deliberately added no type-annotation spelling, and the error message it ships tells users to write one — two advised remedies, both parse errors, while the one working position is demonstrated in the same candidate's own test and never named
metadata:
  type: feedback
---

# A diagnostic can advise the surface the design deliberately omitted

**Measured 2026-08-16 on `392f228b8..4b2d4cd9a`.** A WP added a formation
spelling in **expression** position only, and made that omission the
load-bearing premise of a special case elsewhere.

**Its own diagnostic:**

> *"…add an ascription `(trunc_intro a : ‖A‖)` or place it where the expected
> type is already known (e.g. a declaration's declared type)"*

| advised remedy | measured |
|---|---|
| the ascription | **parse error: expected a type** |
| a declared type | **parse error: expected a type** |

⇒ ***Both remedies require exactly the spelling the WP deliberately did not
add.*** **The diagnostic assumes the surface the premise asserts does not
exist** — the two are written in the same candidate and contradict each other.

**And the one working position was demonstrated in that candidate's own test
and not named**: the eliminator's **motive** argument is an expression position,
so the formation is writable there and supplies the expected type.

⇒ ***When a design's whole point is that some position is unsupported, grep the
error strings for advice to use it.*** A diagnostic is written from the mental
model of the feature working everywhere; **the restriction lands in the parser
and not in the message.**

## VERIFY A "WHOLE SURFACE" NEGATIVE BY FINDING THE GRAMMAR

The premise was *"no other surface path can produce a value of this type."* ⇒ I
attacked return type, ASCII return type, binder annotation, `let` annotation and
ascription — **all five rejected with the same parse error**, which is the tell:
***the type parser has no production for the token at all.*** **Five failures
are an anecdote; the shared error message identifies the single mechanism**, and
that is what makes it a claim about the surface rather than about five guesses.

## PROBE THE INPUT A ONE-LINE CHANGE EXISTS FOR

The only trivia-machinery edit added one arm to a span collector. The shipped
tests reach that path but with **comment-free** sources, so the validation it
feeds runs vacuously. ⇒ **Probe the input the arm exists for** — a comment
inside the new delimiters, in each position and each spelling. **All attached
and round-tripped: no defect, and now measured rather than assumed.**

**Say when an author applied your own prior finding unprompted.** This
candidate stated that no corpus gate can exercise the new token *because nothing
in the corpus uses it*, and pinned the behaviour locally instead — **the
membership-rule argument, made before anyone asked.**
