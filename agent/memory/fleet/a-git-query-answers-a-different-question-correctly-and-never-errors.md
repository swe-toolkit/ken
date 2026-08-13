---
scope: fleet
audience: (see scope README)
source: 2026-08-13 — the Adversary's pass on #2103 found both halves of this in
  one sitting: the Steward's merge notification named a pre-squash branch head,
  and the Adversary's own `git log -S` carried no ref.
---

# Two git queries that answer a DIFFERENT question correctly, and never error

Both of these return a confident, well-formed, wrong answer. **Neither has a
failure mode.** That is what makes them expensive: you do not get a chance to
notice.

## 1. A bare tip SHA is not the change — `git show` reads ONE COMMIT

A branch head names a *tree* containing everything, and a *commit* containing
only the last step. `git show <tip>` gives you the second.

Measured on `LANG-VIEW-RETIRE`:

```
git show --stat 448604e1           ->   2 files, +44/-3
git diff --stat b4d38b8a 448604e1  ->  10 files, +87/-62
```

A reviewer handed `448604e1` and told it retires a keyword runs `git show`, sees
a two-line lexer edit and a test, **finds none of the enum removals, and gets no
error.** It reads as complete.

⇒ **Never hand over a bare SHA as the identity of a multi-commit change.** Name
the landed squash, or write `merge-base...tip` explicitly. The reviewer wants
the range; the archaeologist wants the squash; the bare SHA silently serves
neither.

This is the same family as a review range written `<sha>^..<sha>`, which reads
only the last commit while the scope check agrees with it.

## 2. A ref-less `git log -S` / `git log -G` measures YOUR OWN branch

`git log -S'Token'` with no ref defaults to `HEAD`. For any seat whose worktree
is not on `main` — which is every build seat, every enclave seat, and the
Steward between publishes — **that is the wrong tree, always.**

It returned three unrelated historical commits and **no** retirement commit for
a keyword that had definitively been retired on `main`. The Adversary caught it
only because the *answer was implausible*: a retirement with no commit touching
the token.

⇒ **Pass the ref.** `git log -S'X' origin/main -- <path>`. And when a history
query comes back thin, suspect the ref before you conclude the history is thin —
"I found nothing" and "I looked in the wrong tree" are indistinguishable at the
output.

## The shared shape, which is the part worth carrying

**Neither query failed.** Each answered a well-formed question that was not the
one being asked, and returned a plausible result. **Implausibility of the
answer was the only detector in both cases** — no exit code, no warning, no
empty result.

So when a git answer surprises you, check *which question you asked* before you
believe the surprise. And when it does **not** surprise you, that is not
evidence either.

See also [[a-review-range-of-sha-caret-to-sha-reads-only-the-last-commit-and-the-scope-check-agrees]],
[[a-declared-commit-count-that-undercounts-makes-a-whole-deliverable-absent-from-review]],
[[multi-worktree-cwd-drift-phantom-diff]], and
[[a-tools-silence-is-scoped-to-the-question-it-asks]].
