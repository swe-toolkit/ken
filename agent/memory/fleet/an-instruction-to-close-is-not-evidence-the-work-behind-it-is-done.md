---
scope: fleet
audience: (see scope README) — anyone told to close a tracker node, mark a
  deliverable resolved, or retire a fallback, before checking whether the
  mechanism it names was actually built
source: private memory
  `an-instruction-to-close-is-not-evidence-the-work-behind-it-is-done`
  (R4 triage, 2026-09-26)
---

# An instruction to close is not evidence the work behind it is done

Being told to close a node — by a frame, a handoff, or the node's own banner —
answers a question about scheduling or timing, never the question of whether
the thing it closes was actually built. Closing an unimplemented node as
resolved can unblock a downstream deletion while the code it was meant to
replace is still the only thing serving live production paths.

## The instance

A frame instructed closing one node, and a note on the node itself read: "this
node no longer delivers directly — it closes when the terminal seam merges."
That sentence was read as license to close it. It is a statement about
closure **timing**, and says nothing about whether the mechanism was built. It
was not: the node was never implemented, and both classes it was meant to own
were still live in production. Closing it would have marked an unbuilt node
resolved and unblocked a sibling node's deletion of a fallback lane while
production could still select that fallback — precisely the "partial deletion
is worse than none" failure the sibling node's own scope banned.

The check that actually settles the question is never in the tracker: *are
the things this node owns still live in production?* One grep answers it. The
instruction to close had been inherited from an earlier plan written when two
nodes were meant to land in one atomic candidate — a plan since revised — and
nobody had re-derived whether the surviving entry still belonged in the set at
all. A same-day correction to the list (three nodes trimmed to two) made it
look freshly audited, which is part of why the remaining premise went
unquestioned.

Three instruments in the same episode each answered a question adjacent to the
one being asked, in the vocabulary of the one being asked: a same-shaped
control that "fired" had actually refused on its own lookup error, comparing
nothing; a whitespace-check green ran on an already-clean tree and compared
nothing; and the node's own closure banner answered "when," not "whether
closing is correct." None of the three failures was carelessness — each
artifact answered a plausible, adjacent question.

## How to apply

- Treat an instruction to close, whatever its source, as a scheduling claim
  only. Before acting on it, check independently whether the mechanism the
  node owns has actually been built and is no longer the sole path in
  production.
- A node's own banner about its closure timing is the least independent
  source available for whether that closure is sound — it was written by
  whoever scheduled the closure, under an earlier plan.
- Before trusting any artifact's answer, name the exact question you asked and
  check that the artifact's answer is to that question and not a nearby one
  phrased in the same vocabulary. Apply this to instruments and to prose
  alike.
- When a load-bearing premise was inherited from an earlier, since-revised
  plan, re-derive it rather than treating a recent correction to an adjacent
  number as evidence the whole premise was re-examined.
