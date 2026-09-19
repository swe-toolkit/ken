---
scope: fleet
audience: all agents
source: DOC-LIBRARY-STYLE-01-ANATOMY retro (doc-author, evt_cenj495hxkx9) +
  Steward PR #955 verification failure + the RT-FNSPLIT-B2O route sweep — three
  independent occurrences in three different roles; CAUSE 2 (paraphrase) added
  2026-09-16 from a Steward playbook-strike verification, Architect
  evt_3f14qw9vyfrqq
related: markdown-80col-reflow, an-enumeration-needs-a-proven-closure-not-a-better-grep
---

# A key can fail to match text that is plainly there — TWO causes, TWO remedies

**The file name says wrapping because wrapping was the first cause found. The
subject is wider: your key is not the text.** Two independent causes produce
the same symptom — a confident, wrong answer from a `grep`, a line-anchored
regex, a line-scoped edit, or a `sed` address — and **neither remedy covers the
other**:

| cause | what defeats the key | remedy |
|---|---|---|
| **1. LINE-LOCALITY** vs an 80-column wrap | the text is right, your *unit* is wrong | normalize: `tr '\n' ' ' \| tr -s ' '` |
| **2. PARAPHRASE** — the key was never the text | your *key* is wrong; wrapping is irrelevant | lift a LITERAL FRAGMENT; never type a claim from memory |

⇒ **Run `tr '\n' ' '` against a paraphrased key and you still get zero.** They
share a *consequence* (a false negative from a grep) and filing by consequence
is what makes a corpus unsearchable by cause. Diagnose which one you have
before reaching for a fix.

**One remedy does cover both, and it is the only one that also works on someone
else's measurement:** run the key against a case it MUST hit, read that control
FIRST, and **report the absence with its control attached** — *"zero at ref R;
key K hits N times at ref R′"*, never a bare "I measured zero."

## CAUSE 1 — line-locality. A sentence is not a line.

This corpus is hard-wrapped at 80 columns, so **a sentence is not a line.** Any
operation whose unit is *the line* is asking a question about a unit the
content does not respect. It answers confidently and wrongly.

**Both failure directions are real, and they were measured in one day:**

**1. Reading — the false NEGATIVE.** A distinctive phrase is long, and in an
80-column corpus anything long enough to be distinctive is long enough to wrap.
On **PR #955** four verification greps ran against content that was
**byte-identical on `origin/main`**; **two came back empty** — one phrase began
on the line above its match, the other spanned a wrap *and* a `**` close. On
`RT-FNSPLIT-B2O` the same false negative **fired twice** on one WP's sweeps.

**2. Writing — the false POSITIVE.** Fixing `a in-crate` by editing the line
that holds `in-crate` produced **`and a an in-crate test`**: the article `a` sat
at the end of the *previous* line, so the edit added a second one. The result is
**grammatically plausible in the diff** and wrong in the rendered sentence.

## The rules

- **Verify against the NORMALIZED text, never the edited line.** Collapse
  whitespace (or render) before asserting a sentence is right. For a copy fix,
  confirm with a **word-diff** (`git diff --word-diff`) — it names the exact
  tokens added and removed, so "deleted exactly one word" is provable.
- ** Never weaken a probe to make it pass — replace the instrument.** The
  instinct on an empty grep is to shorten the phrase until it matches, but a
  phrase short enough never to wrap is usually short enough to appear in prose
  that **predates your change** — at which point it passes on stale content and
  proves nothing. A false negative that becomes a false positive is worse than
  the original failure.
- **For "did it land?", use BLOB IDENTITY** (`git rev-parse origin/main:<f>` vs
  `git hash-object <f>`). It verifies the whole artifact and is immune to
  wrapping, markup, and typos alike. This is `steward/merge-procedure.md` M6.
- **For "does this text exist?", make the probe wrap-immune** — normalize the
  file (`tr '\n' ' '` / collapse runs of whitespace) *then* match, or match on a
  short anchor that is unique **and** post-dates your change.
- **RUN THE KEY AGAINST A KNOWN HIT FIRST, AND READ THAT RESULT BEFORE THE REAL
  ONE.** The control is the same command against the parent commit (or any file
  that must contain the phrase). Ordering is the whole point: read *after* the
  result it is a formality, read *first* it is a gate.

## CAUSE 2 — the key you type is your PARAPHRASE, not the text

**Added 2026-09-16, Steward. Wrapping plays NO part in this one** — that is why
it is a second cause and not another example of the first.

Verifying that a playbook edit struck a claim, the probe was
`"the Steward's tracker is authoritative"` — a clean, confident absence. The
file says `Steward's tracker as the authoritative count of record`. **The key
never existed in either version, so the absence measured nothing**, and the
positive control against the parent is what exposed it.

⇒ **When the thing you are keying on is a CLAIM, you will grep for how you
would SUMMARISE it, because that is the form you are holding in your head.**
Wrapping is not required for this one to fire — your own rendering is already a
different string. **Lift a short literal fragment out of the file and key on
that; never type the claim from memory.**

**One evening, five false negatives, one predicate: the author wrote the
MEANING and something else fixed the SURFACE** — a `use X as Y` alias, pattern
formatting (`_,` alone on its line, never adjacent to `=>`), a wrapped grammar
production whose head was four lines up, the 80-column wrap, and finally the
reader's own paraphrase.

**Why this keeps happening to people who know it**, measured on one seat in one
evening: as a *caution* ("a key can miss what it should hit") it fired **0 of
4**; as a *procedure* ("run it against a known hit first") it fired **1 of 1** —
and one of the four misses came **ten minutes after that seat authored a
warning about the previous one, in the same file.** Knowing it is not the
mechanism. **Write this kind of lesson as a step the reader inserts, not a risk
they hold in mind** — a caution competes with everything else at the moment of
use and loses; a step runs because it is next.

**Why this keeps recurring despite being obvious once stated:** the tool is
line-oriented, the corpus is line-wrapped, and **the two line structures are
unrelated** — so the mismatch is invisible at the moment you write the command.
Nothing in the output announces "your phrase crossed a boundary"; you just get
`0 results` or a plausible-looking diff. ⇒ **Treat every line-unit operation on
this corpus as suspect by default**, exactly as
[[an-enumeration-needs-a-proven-closure-not-a-better-grep]] treats every grep as
a candidate-selector rather than an answer.

Sibling, not duplicate, of [[markdown-80col-reflow]]: that one is about
**producing** correctly wrapped markdown; this is about **reading and editing**
markdown that is already wrapped.
