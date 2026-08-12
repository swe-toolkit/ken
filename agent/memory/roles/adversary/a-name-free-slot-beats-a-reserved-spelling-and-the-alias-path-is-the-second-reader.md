---
name: a-name-free-slot-beats-a-reserved-spelling-and-the-alias-path-is-the-second-reader
description: A binding made unpublishable by carrying an EMPTY name list is refused for every argument, not just the reserved one — check the alias/rename path as the second reader, and close a string-sentinel's consumer set by counting its producer's callers
---

# A name-free slot beats a reserved spelling, and the alias path is the second reader

**Measured 2026-08-10 on `41c28de7` (`LANG-STRUCTURAL-RESULT-ELAB`), asked to
find a second path that could publish an anonymous binding. There is none, and
the reason generalises.**

A wildcard binder had leaked a capability by being resolvable as `_`. The repair
could have been *"lookup refuses the name `_`"*. It was not:

```rust
fn push_anonymous(&mut self, span: Span) {
    self.bindings.push((Vec::new(), span));   // EMPTY name list
}
```

Every reader tests `names.iter().any(|c| c == name)`, which is **vacuously false
for any argument whatsoever**. ⇒ The slot is unpublishable by *any* spelling —
not by the one someone remembered to ban.

**This is the pattern to prefer and to recognise.** A reserved-spelling guard is
a blacklist and inherits every normalisation, encoding and aliasing question a
blacklist has. **An empty capability set has none**, and it keeps the positional
structure (de Bruijn depth, telescope alignment) that the slot exists for.
Constructive twin of [[use-parametricity-to-close-a-reachability-question]]: make
the answer structural and it holds for inputs nobody enumerated.

## Check the ALIAS/RENAME path — it is the second reader

The reader that hides this class is not the lookup. It is **anything that
attaches a name to an existing slot**:

```rust
fn push_alias(&mut self, alias: &str, name: &str) {
    if let Some((names, _)) = self.bindings.iter_mut().rev()
        .find(|(names, _)| names.iter().any(|n| n == name)) { names.push(alias.into()) }
}
```

An alias mechanism is exactly how a name-free binding gets a name **back**. Here
it closes, because it **locates its target by name** and so can never find an
anonymous slot to hang an alias on — the same predicate that makes lookup fail
makes aliasing fail.

⇒ **Whenever a construct is protected by having no name, enumerate every writer
of the name field, not just the readers.** Ask of each: could this attach a name
to the protected slot? Lookup is the obvious reader and the one the author
checked; alias, rename, re-export, import, and shadow paths are the ones that
are not.

## Close a string sentinel by counting its PRODUCER's callers

The wildcard travelled as the literal string `"_"` out of `resolve_pattern`, with
the anonymising guard at **one** consumer. A sentinel each consumer must remember
to interpret is normally the defect.

**It closed — and the closure is the deliverable, not the absence.**
`resolve_pattern` has exactly **two** callers: the guarded loop, and its own
recursion whose names flow up into that same loop. There is no third consumer.

⇒ **For a sentinel, count the callers of the function that MINTS it**, not the
occurrences of the sentinel. That bounds the consumer set by construction and
answers *"is there another path?"* with a number instead of a search
([[an-enumeration-needs-a-proven-closure-not-a-better-grep]]).

## The repaired population may not be the whole population

The residual worth naming: the token is a **wildcard only in pattern position**.
The lexer admits it as an ordinary identifier, so in *binder* position — params,
lambdas, `let` — it is a legal name pushed through the plain path, producing a
**resolvable** binding with that spelling.

⇒ **A repair that makes construct X identity-free is scoped to the grammar
position where X is special.** Ask where else the same token is legal and *not*
special — that population was never in the repair's scope and its author had no
reason to consider it. Report it as a **bounded question** with the cheap test
named (*does the association reject a binding that resolves but occupies no
field position?*), not as a defect, when you have not read the deciding path.

## A clean result is a deliverable when it comes with the reason

I filed **no finding** and the report was still worth sending: *the gate holds,
here is the construction that makes it hold, here are the three readers I
checked, here is the closed consumer count.* That transfers — the next reviewer
of this scope does not re-derive it — whereas *"I looked and found nothing"*
expires immediately. Same posture as
[[use-parametricity-to-close-a-reachability-question]]. **Say plainly which axes
you did not reach**, or a clean result on one reads as a clean sweep.
