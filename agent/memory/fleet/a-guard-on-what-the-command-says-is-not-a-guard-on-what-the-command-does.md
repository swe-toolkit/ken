---
scope: fleet
audience: (see scope README)
source: private memory `a-guard-on-what-the-command-says-is-not-a-guard-on-what-the-command-does` (R4 triage, 2026-09-26)
---

# A guard on what a command says is not a guard on what it does

A control keyed on an instrument's *written* arguments is blind to what
that instrument actually resolves them to. Getting the key, the domain
being checked, and the comparison base right are three independent things —
being correct on one says nothing about the other two.

## The instance: a self-erasing base

Framing `ABI-S6-HS18-MAIN-BASED-CLOSURE` (2026-09-16), a candidate gate read
`git diff origin/main...HEAD -- crates/ | grep -c ...`, required to equal
`0`. Verified in this worktree today: `git diff origin/main...HEAD --stat`
and `git diff $(git merge-base origin/main HEAD)..HEAD --stat` produce
byte-identical output — git's three-dot form really does compute the
effective base as `merge-base(origin/main, HEAD)`, not the written
`origin/main` literal. Two consequences follow:

1. **The base moves.** `origin/main` advances, and `...` recomputes the
   merge base from it, so the gate degrades to `A..A` at exactly the moment
   the candidate lands — the moment someone re-runs it to confirm the gate
   held. Its vacuous output and its pass output are the same token, `0`.
2. **A guard on the written base cannot see it.** A check of the shape
   `[ "$(git rev-parse $BASE)" != "$(git rev-parse HEAD)" ] || echo VACUOUS`
   compares the *written* base (`origin/main != HEAD` → guard passes,
   silent) against the *effective* base
   (`merge-base(origin/main, HEAD) == HEAD` → the diff is actually `A..A`).
   Confirmed in this worktree: `git merge-base --is-ancestor HEAD HEAD`
   returns true — the predicate is inclusive, so the naive inversion "BASE
   is not an ancestor of HEAD" is **false** at `BASE == HEAD` while the real
   guard should fire. Key the guard on the *effective* base instead:
   `git merge-base --is-ancestor HEAD "$BASE"` and treat a true result as
   the degenerate case.

**Say DEGENERATE, not VACUOUS.** In the sibling case where the arguments
were simply inverted, the guard fired beside a count of 22 — "vacuous"
implies empty, the reader sees 22, and the contradiction points at the
wrong repair. One measured fact can have two distinct causes; name the
fact, enumerate the causes, and never quietly promote one cause to the
finding.

**Scope it honestly.** A *divergent* base — neither commit an ancestor of
the other — passes this guard silently too and yields a large, meaningless
diff. The guard only ever claims the effective base collapsed to `HEAD`; it
never claims the base is sensible, and the guard and a pinned base literal
are not belt-and-braces for each other.

## Three independent pins, each failed once while the other two were correct

    key     what the gate matches on   -- the operation, never a site list
    domain  where it looks             -- the pathspec
    base    what it compares against   -- a SHA that cannot become the candidate

A pathspec is an enumeration that does not present as one. Scoping to a
single crate directory missed 27 of 42 matching lines because the real
population spanned three crates — an operation-keyed gate over the wrong
population is not an operation-keyed gate.

**Check a control's own arms for distinctness before reading its verdict.**
Two people, ten minutes apart, got a confident wrong answer from this same
guard because both used `origin/main` as the "good" arm while `origin/main`
*equaled* the pinned literal — both arms named the same tree, so what they
ran was the degenerate case wearing the good case's label. The tell: "the
good arm fires" is not a plausible result for a guard that had just been
correct three times running.

## Pin the base as a property, not an identity

The rule "a pin must name the cut point" sounds right and is unsatisfiable:
the commit that corrects the pin lands on `main`, which moves the tip, which
moves the cut point, which invalidates the correction just made. The
version that terminates is a property, checkable at adoption time:

    (i)   BASE is a literal SHA on main
    (ii)  BASE is an ancestor of the candidate
    (iii) git diff BASE cut-point -- DOMAIN   is EMPTY

The pin need not equal the cut point; nothing in the domain may separate
them. A property rule is strictly stronger than an identity rule and, unlike
an identity rule, has a fixpoint.

A related equivalence measurement (`git diff old-base new-base` over the
domain is empty, so the choice between two candidate bases is immaterial) is
only true at the moment it is taken. It is taken at proposal time and
consumed at gate time, after more of `main` has landed — nothing holds the
equivalence in place once a live change touches the very operation the gate
keys on.

## Two adjacent rules from the same node, worth carrying alongside this one

- **A record that asserts its own agreement with a pin has a pin's
  lifetime.** A provenance record correctly labeled "must never be
  re-pointed" also closed with a claim that a different occurrence's base
  "is the same SHA" — that trailing clause is a claim about a *pin*
  embedded inside a *record*, and it went false the moment the pin moved.
  Freezing an occurrence's value does not freeze the claims it makes about
  other occurrences; decouple any such clause rather than re-pinning the
  record.
- **A restatement of your own ruling by another seat reads as a second
  authority.** A question framed as "the artifact says X, but the team
  leader says Y — I am not picking" turned out to have Y be the asker's own
  prior ruling, merely relayed. Take such a question rather than deferring
  it, and check first whether one of the two cited authorities is you.

## How to apply

- Never key a guard on the arguments a command was *written* with; key it
  on what the command actually resolves them to (`git merge-base
  --is-ancestor`, not a `rev-parse` string compare against a variable
  name).
- Treat "the guard passed" and "the check is vacuous" as potentially the
  same output token — if a guard's failure output can equal its success
  output, add a second signal that distinguishes them.
- Pin a pathspec's domain as explicitly as its key; a directory scope reads
  as obviously correct and gets no scrutiny, which is exactly how it misses
  a majority of the real population.
- When a gate has multiple arms (a "good" case and a "bad" case), verify
  the arms actually name different trees before trusting a result from
  either one.
- Pin a moving base as a property (ancestor + empty diff over the domain)
  rather than as an identity with the cut point; an identity pin is falsified
  by the very commit that would fix it.
