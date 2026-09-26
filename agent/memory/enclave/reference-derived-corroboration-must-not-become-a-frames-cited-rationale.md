---
scope: enclave
audience: (see scope README)
source: private memory `reference-derived-corroboration-must-not-become-a-frames-cited-rationale` (R4 triage, 2026-09-26)
---

# Reference-derived corroboration must not become a frame's cited rationale

`CLEAN-ROOM.md` splits the reference corpus by **seat**, not by content. The
Spec enclave, the research agent, and the adversary agent may read the
permissive shelf under `local/refs/` to understand and to resolve
`(oracle)`-tagged spec details, and may read the copyleft shelf for approach
and behavior only (never vendored, never transcribed). Implementer agents
build only from `/spec`, never from `local/refs/`, and are not on either
roster. A work-package frame is read by implementers.

**The boundary is crossed by the frame, not by the read.** A prior-art read
that corroborates a conclusion already reached from Ken's own evidence is
authorized and clean. The same read, cited in a WP frame as the
justification for that conclusion, silently converts an authorized read into
reference-derived design delivered straight to an implementer — the seat
with no licence to check the source and no way to know it should.

## The case this was ruled on

An adversary prior-art read was routed back toward a WP frame. The frame's
criterion — "a control's placement must be justified by what fires *before*
it" — had been reached independently, from findings measured in Ken's own
system that afternoon. Prior art then agreed with it from several unrelated
directions. That ordering (Ken's own evidence first, prior art agreeing
after) is the clean provenance, and it is worth stating explicitly, because
nothing else records it — nobody re-derives the ordering by reading the
artifact later.

Had the frame instead read "per the reference's coverage gate…" as its
stated justification, the authorized adversary read would have become the
frame's rationale, and the implementer downstream would have inherited
reference-derived design with no way to check it — the exact failure the
seat split in `CLEAN-ROOM.md` exists to prevent.

## How to apply

- **Corroboration yes, rationale no.** Write the criterion in Ken's own
  terms, justified by what was measured in Ken. Cite Ken's own findings on
  the arc, not the prior-art lines that happened to agree with them.
- **Independent arrival is what makes it clean.** Reach the conclusion from
  Ken's own evidence first; let a permissive or copyleft reference agree
  afterward. State that ordering explicitly in the routing post, since there
  is no gate, diff, or check that fires when a frame cites a reference — the
  failure is otherwise silent.
- **Watch for "adopt the reference's mechanism" creeping into scope.** A frame
  that ports the reference's *mechanism* imports the design; one that states
  only the *principle*, reached independently, does not. Be explicit about
  which parts of a read do and do not port.
- **The read itself follows `CLEAN-ROOM.md`'s existing seat roster** — this
  lesson is only about what the artifact the read flows *into* may say, not
  about what the read may touch.

Related: [[cleanroom-is-role-discipline-not-host]] — the companion boundary:
clean-room protection travels with the role reading the reference (enclave,
research, adversary), not with any model or host in the seat. Together they
cover both ends of the same discipline — who may read, and what the read may
become once written down.
