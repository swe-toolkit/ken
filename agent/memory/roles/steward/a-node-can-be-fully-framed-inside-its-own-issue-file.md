---
scope: roles/steward
audience: (see scope README)
source: 2026-08-08 and again 2026-08-12 — the `docs/program/wp/` existence test
  reported two fully-framed `ready` nodes as framing debt
---

# A node can be fully framed inside its own issue file

The §4e frontier check asks, for every successor of an in-flight node: is it
`status: ready`, **and does a shovel-ready frame exist?** The obvious way to
answer the second half is `test -f docs/program/wp/<ID>.md`.

**That test is wrong, and it has returned a false negative twice.**

A frame is a set of properties — deliverables, acceptance criteria with their
controls, scope, a contention check, fixed inputs measured at a named SHA. It is
**not a location**. Some nodes carry all of it inside
`docs/program/issues/<ID>.md`, below the frontmatter, with no separate `wp/`
file at all. `RT-DYNAMIC-ARM-SCALAR-MERGE` (45 KB) and `RT-CENSUS-CAVEAT-GUARD`
are both framed this way and both are correctly `ready`.

## It has cost a turn once and nearly cost one again

**2026-08-08:** `RT-CENSUS-CAVEAT-GUARD` was flipped `ready` to `draft` on the
stated ground *"no frame exists"*. False. The flip was reverted the same day,
and the node now carries a correction block at its head saying so in the first
line.

**2026-08-12:** the same existence test reported that node **and**
`RT-DYNAMIC-ARM-SCALAR-MERGE` as unframed successors — a frontier gap that does
not exist. The node's own correction block is what stopped it. The instrument
had not changed in between; nothing had been learned, because the lesson lived
only in the artifact it was about.

## The two failure directions are not symmetric

| reading | cost |
|---|---|
| false **unframed** | you write a duplicate frame for a node that has one, or flip a startable node to `draft` and withhold it from the frontier |
| false **framed** | a ring is kicked at a node with no deliverables and stalls, or the frontier looks one deep when it is zero |

The second is the one §4e exists to prevent, so the check must not be loosened
to "assume framed" either. **Neither direction is answered by a path.**

## How to apply

- **Frame the question as a property, not a path.** Ask whether the node states
  deliverables, ACs with controls, scope, and a contention check — wherever they
  live. `grep -n '^#\{1,4\} ' docs/program/issues/<ID>.md` shows the shape in one
  read; a node with `## Deliverables` and `## Acceptance criteria` is framed.
- **Size is a fast prefilter, not the answer.** A 126-line node can be a
  complete frame for an `S`; a 40-line node with only an `origin:` paragraph is
  not a frame at any size.
- **Read the head block before concluding anything about a node's status.**
  Correction blocks live there, and on both of these nodes the head block
  answers the exact question being asked. A frontmatter-only read is what makes
  the same mistake available twice.
- **When you catch a stale status or a bad flip, the fix goes in the corpus as
  well as in the node.** A correction that lives only in the artifact it
  corrects protects that one artifact and teaches nothing — which is why this
  check failed a second time on a node carrying the first failure's own
  reversal.

Related: [[a-frame-with-no-tracker-node-is-equally-consistent-with-done]] — the
inverse orphan, and the reason both directions need checking.
