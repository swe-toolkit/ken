---
scope: roles/adversary
audience: (see scope README)
source: private memory `a-mutation-campaign-needs-a-grid-not-a-count` (R4
  triage, 2026-09-26)
---

# A mutation campaign is judged by the grid it spans, not by how many mutations reddened

Two mutations that each redden correctly can still leave a cell of the
mechanism's space completely unwitnessed, and a passing campaign gives no
signal pointing at the cell nobody wrote — the occupied cells being green is
exactly what makes the empty one invisible.

Measured 2026-08-09, `RT-MATCH-RECURSOR-CONSUMERS` `4a.1` (blocked twice on
the same axis by runtime-qa, once after an earlier "fix"). A transport
carried an identity across a process boundary and had two paths (serial,
concurrent) crossing two directions the value could travel (in, out):

| | in (parent hands to child) | out (child writes back) |
|---|---|---|
| serial | — | mutation 5 |
| concurrent | mutation 4 | EMPTY |

Both mutations reddened the intended assertion, and the campaign was reported
as proof the control was non-vacuous. A transport that substituted the parent
identity **on the way out** of the concurrent child still passed everything,
because the sessions stayed distinct by construction and the union key stayed
unique regardless.

The distinction that generalizes: asserting what a boundary crossing was
handed **in** is a different claim from asserting what it produced **out** —
one tests the hand-off, the other tests the writer, and a campaign holding
only the first is insensitive to every defect in the second. Any value that
crosses a boundary (process, thread, file, wire, FFI) needs both checks
before the campaign counts as spanning it.

The gap here was closed, at first, by an inline check that had drifted from
its sibling on the other path — patching the one empty cell rebuilds the same
hazard at a smaller size. The durable repair routes every path through one
shared validator, so no path can validate one direction while skipping the
other.

## How to apply

- Before running any mutations, write the grid explicitly: enumerate the
  mechanism's **paths** (serial / concurrent / error / retry, or whatever the
  code actually branches on) against the **directions** a value travels
  (in / out), and name which mutation occupies each cell.
- Report an empty cell as "no witness for `<cell>`," never as a silent
  omission. A campaign is judged by the grid it spans, never by the count of
  mutations that reddened.
- When two probes take visibly different routes and both agree, treat that
  agreement as suggestive, not corroborating, until you have checked that
  they occupy different grid cells and not just different-looking code paths
  over the same cell (see
  [[agreement-is-not-corroboration-when-a-premise-was-inherited]]).
- Fix a discovered gap by routing every path through one shared validator,
  not by adding a matching inline assertion on the empty path — an inline
  duplicate assertion is what drifted and opened the gap the first time.
- The tell that you are about to patch a cell instead of fixing the
  mechanism: the assertion you are about to add closely resembles one that
  already lives elsewhere in the same file.
