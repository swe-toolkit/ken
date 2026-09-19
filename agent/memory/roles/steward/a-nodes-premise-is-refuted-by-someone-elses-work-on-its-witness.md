---
scope: roles/steward
---

# A node's premise is refuted by work on its witness, and the node never hears

**Measured 2026-09-19.** `RT-IGNORED-PASSING-ROWS-DISPOSITION` (merged) took
twelve ignored rows that had started passing and disposed each row's **label**,
with an `AC-1` mutation proving the row is a live control and not a vacuous
green. It did its job exactly. Those labels named **four nodes**. Nothing walked
back to any of them.

**A row disposition updates the ROW. Nothing updates the NODE the row's label
named.** The disposition node had no reason to — its subject was the row. The
node had no way to hear — nothing watches its witnesses. Both halves were right,
and there is no edge between them.

Three of the four were still live when swept, 25 days later:

- `RT-BORROWED-INPUT-CARRIER-DURABILITY` — `draft`, **zero** live witnesses. Its
  two measured rows were un-ignored and returning the computed value, not the
  `-1` its title asserted. Closed REFUTED, nothing landed.
- `RT-WORKER-FIXTURE-DECODE` — **`ready`**, premise refuted as written.
- `RT-CARRIED-RESOURCE-SCALAR` — `draft`, one of its two seats refuted.

## The compounding half: `depends_on` is recorded at filing and never recomputed

The first node's dependency merged **the day after it was filed**. It then sat
`draft` for 25 days. **An unblocked node is indistinguishable from a blocked
one**, so nothing ever re-asked whether its premise survived the very landing it
had been waiting for — which is precisely the landing most likely to have closed
the seam.

**The sweep is one query and had never been run.** Of 684 nodes, **26 were
`draft` with every dependency already `merged` or `closed`.** Parse the
frontmatter, resolve each `depends_on` against the corpus, list the drafts whose
deps are all done.

## Why `ready` is the urgent status, not `draft`

**`ready` means kickable.** A ring kicked at a node whose premise is gone spends
a T1 turn building a repair for a defect that does not exist — and the frame's
ACs are all controls **on that repair**, so **none of them can report that there
was nothing to repair.** That is the `RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION`
shape (closed REFUTED, ten ACs, nothing landed) and the reason `AC-0` exists.

⇒ **Before flipping or leaving a node `ready`, check that its witnesses are still
in the defect state** — not that its status says `ready`, and not that its deps
merged. Status is a claim about a node; witnesses are evidence about the tree.

## The trap in running this sweep, hit on the first pass

Keying on **the node's name appearing in an `#[ignore]` attribute** returned zero
live rows for two nodes, and zero reads as "refuted." **A filter keyed on a name
cannot see a row that stopped carrying the name** — a readmitted row keeps the
node's name only in a prose comment, and a relocated row drops it entirely.

**Read the node's own "rows it owns" list and check each row by name.** That is
what caught `RT-CARRIED-RESOURCE-SCALAR` owning three rows across **two seats**,
where only one seat was refuted — and the node's own text was the argument
against carrying one seat's result to the other.

## Disposition discipline: the pattern makes the next one feel settled

Having closed one node on this evidence, the second and third felt decided.
**Refutation of a node's witness is refutation of exactly as much as the witness
covers.** Both of the others were demoted or re-aimed, neither closed: one had
its *first* of three comparisons shown reachable while its subject was a
different comparison; the other had one of two seats refuted. **Demote, re-aim
the premise, and say what was not measured** — a wrong close is a node nobody
reopens.

Related: [[a-nodes-status-is-a-claim-about-a-node-not-evidence-about-the-tree]],
[[an-atomic-sibling-node-needs-active-not-a-dependency-edge]],
[[the-merge-tree-landed-test-reads-not-landed-for-a-candidate-that-landed-and-was-then-superseded]].
