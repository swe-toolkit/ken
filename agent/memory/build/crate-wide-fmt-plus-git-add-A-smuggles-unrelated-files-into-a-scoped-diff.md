---
scope: build
audience: (see scope README)
source: private memory
  `crate-wide-fmt-plus-git-add-A-smuggles-unrelated-files-into-a-scoped-diff`
  (R4 triage, 2026-09-26)
---

# `cargo fmt -p <crate>` formats the whole crate, and `git add -A` commits whatever it touched

`scripts/ken-cargo fmt -p <crate>` runs plain `cargo fmt -p <crate>` — `fmt`
is explicitly exempted from the wrapper's no-`--workspace` refusal because it
compiles nothing, but it is not exempted from `cargo fmt`'s own scoping:
`-p` formats every file in the package, not the files you edited. On a
scope-sensitive change (a WP whose acceptance depends on move-purity or path
discipline), that formatter run plus `git add -A` commits every pre-existing
formatting fix in the crate as if it were part of the change, and an
unexplained file in the diff forces a reviewer to prove a negative.

This recurred four times, each caught by something other than a deliberate
check:

- **RT-SPLIT slice 7 (2026-07-22).** `fmt -p ken-runtime` corrected a
  pre-existing wrap violation in an untouched file; `git add -A` committed
  it. Caught while looking for something unrelated.
- **`LANG-BARE-OPERATOR-ATOM-REJECTION` (2026-09-17).** The crate was not
  fmt-clean on `main`, so `fmt -p` rewrote 145 files; `git add -A` swept them
  into a 49-line parser change. Also caught by accident.
- **`LANG-TYPE-PROJECTION-SURFACE-FORM` (2026-09-17).** Running `rustfmt` on
  only the seven files actually edited still swept in every pre-existing
  violation in each of them — one file showed 560 changed lines against a
  ~60-line semantic edit. The risk unit is the **file**, not the crate;
  "I only formatted what I touched" is not a defense when that file was not
  clean at your base.
- **Same session, twice more, as a diagnostic rather than a fix.** Typing the
  plain formatting command while only meaning to ask "what would rustfmt
  change here" rewrote the whole package both times — once even after
  copying the file to `/tmp` first, which does not help: `-p` scopes to the
  package, not to any one file.

## What actually discriminates

- **`git diff -w` does not prove a rewrap is inert.** It ignores whitespace
  *within* a line, but a reflow that adds line breaks still produces a
  non-empty hunk. Comparing token streams (or parsed output) is the check
  that establishes semantic identity; `-w` emptiness is sufficient but not
  necessary, and its absence proves nothing.
- **Read both the file count and the per-file line count.**
  `git diff --stat <merge-base> HEAD` gives both. The file count alone
  catches a crate-wide `fmt`, a stray scratch file, or `git add -A` sweeping
  in an unrelated path — but the third incident above had exactly the right
  file set, and only the per-file line count (560 vs. an intended ~60)
  showed the churn.
- **Never trust `$?` after a pipeline** (`cmd | tail`, `sort | head`) — it
  reports the last stage's exit status, not the command's, and can hide a
  real `FAILED` line or a nonzero `fmt --check`. Capture with `tee <file>`
  and read `${PIPESTATUS[0]}`.
- **To ask what the formatter would change without changing anything**, use
  `cargo fmt -p <crate> -- --check` (hunk list only) or diff a copy of the
  file — do not run the plain formatting command "just to look."

## How to apply

- Before `git add`, run `git diff --stat <merge-base> HEAD` and read the file
  count against the WP's declared scope, then read each in-scope file's line
  count against the edit you believe you made.
- Prefer `git add <explicit paths>` over `git add -A` on any scope-sensitive
  change.
- If a crate-wide or per-file `fmt` run fixes a genuine pre-existing
  violation, treat that fix as someone else's change and drop it from the
  candidate rather than absorbing it for free.
- Before formatting a file you plan to keep formatted, check what the
  formatter would change at your base commit; a nonzero result means running
  it for real will smuggle in unrelated churn, so hand-shape your addition
  instead.
- To clean up an over-broad `fmt`, revert the whole affected part of the
  worktree and reapply your own edits from a saved copy — a path-scoped
  `git checkout` can look like a full revert while leaving a sibling
  directory (e.g. `tests/`) still rewritten.
- Self-report a smuggled file immediately rather than re-taking a branch
  you've already handed off to fix it — that recreates the handoff deadlock.
  State the options and let the branch's current holder decide.
