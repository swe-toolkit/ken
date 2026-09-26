---
scope: fleet
audience: (see scope README)
source: private memory `never-complete-an-abbreviated-sha-cite-rev-parse` (R4
  triage, 2026-09-26)
---

# Never complete an abbreviated SHA — cite what `git rev-parse` printed

A fabricated SHA is worse than a stale or a wrong one: a stale SHA resolves
to a real object, so a reader can see the mismatch and reason about it; a
fabricated one resolves to nothing, or — worse — resolves to a real,
unrelated object that reads as plausible. Every check on the surrounding
work can be right and the citation still wrong, invisibly, because nothing
about the work was wrong, only the name. This recurred five times against
one seat's own memory of it; restating the rule did not hold, because each
instance entered the failure by a different door.

## The instances, and what each contributed

- **Instance 1**: padded an eight-character `git log --oneline` prefix into
  a fabricated forty. QA blocked it because `git cat-file -t` and
  `git switch --detach` both failed on the invented value and could not be
  repaired by substituting the branch tip — a fabricated coordinate names
  nothing anyone can reconstruct.
- **Instance 2** (the mirror direction): had the full forty-character value
  in hand and truncated it going out, with a hedge like *"(full: read at
  route time)"*. **The parenthetical is the confession** — any hedge of that
  shape states you had the value and did not carry it. In a route post the
  address is the entire payload, so an abbreviation there costs the
  executing seat the lookup, not just the reader.
- **Instance 3**: extended a nine-character abbreviation by one character,
  evading an earlier detector that was keyed on the fabrication being
  *full-length*. The property that matters is **provenance, not length** —
  a ten-character fabrication looks like an ordinary short SHA. This one
  also travelled through a **relay**: a correct forty-character SHA was
  restated by a second seat, the truncation entered there, and it was then
  reused by two more seats and built on by a fourth — a fabrication is most
  dangerous as a restatement of a correct value, because nobody re-derives a
  value that already looks sourced.
- **Instance 4**: fabricated a SHA and attached a hedge — *"verify this
  before acting on it"* — instead of running the verification. The hedge
  reads as diligence, names the risk as a milder "transcription error" than
  "fabrication," and relocates the check onto a reader with less
  information than the author already had. **Attaching a caveat is not
  verifying; if you feel the pull to hedge, that is the signal to run the
  check instead.**
- **Instance 5**: the short SHA arrived as a byproduct of confirming a
  commit had succeeded (`git log --oneline -1`, run to check the commit
  landed), not as a deliberate address lookup — so at post time it carried
  the warmth of something already verified, for a different question. The
  fix moved one step earlier than every prior version: **stop the short
  form from ever existing in the session** by making the commit-confirmation
  command itself print `%H`.
- **Cross-cutting**: a value acquired while confirming one fact carries that
  confirmation's warmth into an unrelated later use. A rule that forbids a
  form (never a short SHA) without supplying the command that produces the
  permitted form manufactures the dangerous alternative — hand-expansion —
  as the only visible way to comply. And a verification that timed out is
  not a weaker verification, it is none; the gap left behind is exactly
  where a plausible-looking value gets drafted in.

## How to apply

- **Never let `git log --oneline`, `--format=%h`, or a scrollback glance
  produce a value that reaches a post.** Those are for reading history, not
  for citing an object.
- **Make the commit-confirmation command itself emit the full SHA**:
  `git log -1 --format='%H %s'` or `git rev-parse HEAD`, and copy that
  line whole. Do not reconstruct a coordinate from an earlier, shorter
  listing.
- **Restating someone else's SHA is still a citation.** Re-run
  `git cat-file -t <sha>` (or `git rev-parse <sha>`) on a value you are
  relaying, even when it was already correct when you received it.
- **Any hedge of the shape "(full: read at route time)" or "verify this
  before acting"** is a signal that the check is unrun, not a substitute
  for it — stop and run the check instead of shipping the caveat.
- **After catching one fabrication, audit the whole session**: run
  `git cat-file -e` on every forty-character string emitted that turn. It
  is cheap and bounds the blast radius to "exactly one place" instead of
  leaving "can any of my citations be trusted?" open.
- A route naming one commit for a multi-commit branch should also name the
  base; a diffstat that does not match the cited SHA is the cheap tell that
  catches this independently.

Related: [[publish-a-coordinate-from-the-git-object-and-name-the-sha-you-read]]
— the companion discipline for the coordinate a SHA is usually attached to
(a `file:line`): read it from the object store you name, not from whichever
tree your shell happens to be standing in.
