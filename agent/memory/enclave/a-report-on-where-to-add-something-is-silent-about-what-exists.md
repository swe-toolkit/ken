---
scope: enclave
audience: (see scope README)
source: RT-LEXICAL-R3 gate 4b, 2026-08-13
---

# A description of where to add something is not evidence that nothing is there

Runtime handed back a located stop for R3 gate 4b: *"the narrower seam is
immediately after `build_static_continuation_fusion_plan` at `core.rs:2167-2172`,
before installation/emission... exposing a read-only result there would be a
second Runtime seam."* The Steward characterized the increment from that
handback — *"observing it is a new seam rather than a wider read of an existing
one"* — and routed it to the Architect as an authorize-a-new-seam question.

**It was false.** Three lines below the proposed site, at `core.rs:2182-2194`,
`d2f_gate_note_arrival` already recorded the exact 4b population (the fusion
plan's `keys` and `descriptors`, the transition plan's fusion-definition count)
at the point production computes it — landed earlier under `D2f`, with its
rationale in the comment. The observer existed. What blocked the gate was that
its `#[cfg(test)]` gate is invisible across a crate boundary, and the dependency
runs one way, so no control could hold both the real-source witness and the
population.

**Why the report misled without being wrong.** A handback that answers *"where
would the seam go?"* is accurate about its own subject and **silent about
everything adjacent**. Silence about the three lines below is not a claim that
they are empty — but a reader converting the report into a design question hears
"there is nothing there," because that is what makes the question well-formed.
The failure is in the conversion, not the report.

**Why it matters beyond tidiness.** The two framings authorize different objects:
*build a new observation seam* is a larger, riskier increment than *make a landed
observation reachable*, and it invites a second observer beside one that already
works — the proliferation the subsume principle exists to prevent.

**How to apply.**

- **Before ruling on "may we add X here", read the site.** Not the handback's
  description of the site — the file, at the named lines, plus what surrounds
  them. The cost is one read; the ruling authorizes work.
- **Ask "is this missing, or merely unreachable?"** A missing mechanism needs
  building; an unreachable one needs a gate/visibility change, which is a
  materially smaller object with different risks. They present identically in a
  report written from the side that cannot see it.
- **When a mechanism *is* already there, rule on the gate, not the mechanism** —
  and require that the gate control only observation, never construction, with
  the enabled-vs-disabled identity proved rather than asserted. See
  [[a-feature-gate-in-one-crate-cannot-gate-a-call-site-in-another]] and
  [[a-cfg-test-reexport-is-a-production-only-red-the-test-profile-cannot-see]]
  for why `cfg(test)` is the recurring form of "unreachable".

Sibling of [[grep-the-producer-not-the-cited-proxy]]: there the cited proxy
misdescribed the producer; here an accurate report's *silence* was read as a
negative claim.
