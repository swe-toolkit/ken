---
id: ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION
title: "Adjudicate the D5b substrate port's judgment surface: every method where BOTH the port source and main moved since their common base b4c8df33a. Each row gets one of KEEP MAIN'S / TRANSPLANT / RECONCILE with the check-against-main SHOWN, on the template of dispatch_host_op_v1 -- which scored +770 lines in this bucket and is not owed at all, because main refactored it after the base and the source carries the pre-refactor shape. Split out of ABI-S6-HS18-D5B-SUBSTRATE-PORT so the mechanical bulk could run on a T2 seat; deferring these rows is compile-safe because an ADJUDICATE method already exists in main, so leaving main's body resolves and builds. Completion is ADJUDICATE = 0 under the three-way instrument with origin/main as the third tree."
status: draft
owner: runtime
size: S
gate: none
depends_on: [ABI-S6-HS18-D5B-SUBSTRATE-PORT]
blocks: [ABI-S6-HS18-MAIN-BASED-CLOSURE]
github: null
tier: T1
origin: "Steward cut 2026-09-17, authoring the recut the Architect asked for when re-ruling ABI-S6-HS18-D5B-SUBSTRATE-PORT (c) mis-sized (evt_7tx7a1n71qa9g). The re-rule's stated defect was that the WP had no criterion that could report its own completion, and its item 3 named ADJUDICATE as the human surface that should be sized separately from the three mechanical buckets. This node is that surface."
---

> # DRAFT. Frame: `docs/program/wp/ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION.md`.
>
> **Held `draft` deliberately: its population is a DELIVERABLE of
> [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]], not of this node.** Flipping it `ready`
> before that worklist exists would frame a node against a count nobody has
> produced — which is the failure the predecessor was re-cut for.

# Why this node exists

The Architect re-ruled [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]] **(c) mis-sized**
(`evt_7tx7a1n71qa9g`) and named the fork: the three mechanical buckets (methods
`main` never touched, names `main` does not have, methods that are `main`'s own)
are **not adjudication**, and sizing a node on a review of every differing
method conflates them with the handful that genuinely need a decision.

**The split's basis is compile-safety, not tidiness.** An ADJUDICATE method is
one where `main` moved too — so it **already exists on `main`**, and leaving
`main`'s body in place resolves, links and builds. The judgment surface can
therefore be deferred without leaving a broken tree behind. That is why the cut
falls here and not somewhere else.

# The three dispositions

    KEEP MAIN'S    main's change supersedes the source's, or main refactored
                   past it. The port owes nothing.
    TRANSPLANT     the source's change is owed and main's is orthogonal.
                   State why main's move does not conflict.
    RECONCILE      both are substantive and neither supersedes the other.
                   Write the merged body; say what each side contributed.

**The showing is the deliverable, not the verdict.** This chain already ruled
that an item list carries no evidential weight: *derive the list FROM the tree
property; never validate the tree property against the list.*

# The worked example, which is also the reason

`dispatch_host_op_v1` in `ken-host/src/effect_v1.rs`, the Architect's own
falsifier, caught before he published the line total:

    b4c8df33 (base)    814 lines
    b601e2ec (source)  794 lines
    879f00c9 (port)     24 lines
    origin/main         24 lines     <- the port already EQUALS main

⇒ **The largest item in the bucket by line count is not owed at all.** Size and
prioritise on **method counts**; *a line delta measures how much text moved,
never how much is owed, and the two look identical until you check one against
`main`.*

# The third tree is `origin/main`, and the correction moves THREE buckets

Measured independently in two extractors, one variable changed in each:

| | OWED | ADJUDICATE | ABSENT |
|---|---|---|---|
| Steward, third tree = branch | 252 | 154 | 90 |
| Steward, third tree = `origin/main` | **372** | **32** | **151** |
| Architect, third tree = branch | 200 | 20 | 88 |
| Architect, third tree = `origin/main` | **322** | **6** | **131** |

Classifying against the port **branch** makes *"already ported onto the
branch"* indistinguishable from *"`main` moved it"*. The control is `units.rs`,
byte-identical between base and `main`, where no method can be "both moved" —
the branch-based run still puts methods here; against `main`, both extractors
report zero.

**ADJUDICATE falls several-fold while OWED and ABSENT rise by roughly half.**
This node shrinks; the predecessor grows. No number here is a target.

A **third** extractor, the implementer's committed `cd17424e7`, first read
ADJUDICATE **157**. Run down rather than absorbed: its classifier asked *"differs
from base on both sides"*, which is **vacuously true on both sides when the
method is new and there is no base** — 130 of the 157 had no base and **120 of
those had byte-identical source and main bodies.** Correcting that one branch
gives **37** (31 production, 6 test) with every other bucket unmoved, which is
the Steward's 32 plus the byte-exact/normalised difference the implementer had
already measured. See the predecessor's `AC-1c`: **neither named control could
reach the broken branch**, because main-equals-base makes "absent from base" and
"absent from the third tree" the same condition, routing those keys to ABSENT
before the classification runs.

# A `#[test]` row is a different adjudication, and its wrong answer is silent

For a production method *"which body survives"* is a question about behaviour.
For a `#[test]` both trees moved, it is a question about **what the suite
asserts** — and **a test that keeps the weaker of two bodies still passes**, so
there is no red to find. The `dispatch_host_op_v1` discriminator does not
transfer: checking a test against `main` reports which assertions `main` makes,
never which ones the port owes. See the frame's `§3a`.

# Related

- [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]] — the mechanical half and the only
  dependency. Its `§3b` holds the authoritative statement of the split, the
  instrument, and the third-tree rule.
- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — what the pair unblocks.
- [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] — holds the refused grant, `draft`.
