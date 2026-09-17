# WP frame — `ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION`

    owner   runtime        tier   T1        size   S
    depends ABI-S6-HS18-D5B-SUBSTRATE-PORT  (its only dependency)
    blocks  ABI-S6-HS18-MAIN-BASED-CLOSURE
    node    docs/program/issues/ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION.md

## 1. Objective

Decide, for every method where **both** the D5b port source and `main` moved
since their common base, which body `main` should end up with — and **show the
check for each one.**

This is the judgment surface of the D5b substrate port, split out of
[[ABI-S6-HS18-D5B-SUBSTRATE-PORT]] by that frame's `§3b` so that the mechanical
bulk could run on a T2 seat without anyone being asked to exercise judgment
they were not provisioned for.

## 2. Fixed inputs

### 2a. The population comes from the predecessor's worklist, not from this frame

**The ADJUDICATE list is a DELIVERABLE of [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]]**
(its `§5` item 6), produced by re-running the three-way instrument at that
node's candidate. **Take it from there; do not re-derive it here and do not
take a count from this frame as the contract.**

Two independent re-derivations at `origin/main`, given so the node can be sized
and **not** so it can be checked against:

    Architect's extractor      6 methods, of which dispatch_host_op_v1 arrives
                               ALREADY DECIDED (KEEP MAIN'S) => 5 live
    Steward's extractor       32 methods (~30 production, ~2 test)
    concentrated in   planning/static_transition/{continuations,aggregates,
                      responses,construction,immediate_bridge}.rs
                      ken-interp/src/eval.rs
                      ken-elaborator/src/{prelude,compiler_driver}.rs
                      ken-host/src/{abi_v1,effect_v1}.rs

**Different extractors produce different totals and these two differ by ~5x on
this bucket.** They also differ on AGREE (2064 against 2601), so the gap is the
extractors, not a disagreement about the tree. **Neither number is the
contract** — the contract is that every row on the predecessor's actual
worklist has a disposition. **Sized `S` on the range, not on either end.**

> **THE SHRINK IS AN ARGUMENT AGAINST THIS NODE EXISTING, so it is answered
> here rather than left implicit.** The Architect's census read **20** before
> the third-tree correction and **6** after (`evt_55pw9gwz3vqm4`) — a judgment
> surface of five live rows is small enough to ask why it is not simply folded
> back into the predecessor.
>
> **Because the split's basis was never volume.** It is that a T2 seat should
> not be handed a judgment call, and that deferring these rows is compile-safe.
> Five rows needing `dispatch_host_op_v1`-grade checking are still five rows a
> mechanical seat should not be guessing at, and the cost of the separate node
> is one frame.
>
> ⇒ **The move that WOULD have defeated the split is the opposite one.** Had
> the correction ballooned the judgment surface, the compile-safety argument
> would have been carrying a node too large to be an afterthought to a T2 port.
> It did not.

### 2b. THE THIRD TREE IS `origin/main`. The branch-based reading is an artifact.

Carried from the predecessor's `§3b`, because getting this wrong inflates
precisely this node's population:

    third tree = port branch 879f00c99    ADJUDICATE  154
    third tree = origin/main              ADJUDICATE   32

Running the classifier against the port **branch** makes *"the implementer
already ported this"* indistinguishable from *"`main` moved this"*. The control
is `units.rs`, where `main` is byte-identical to the base and therefore **no**
method can be "both moved" — the branch-based run nonetheless puts 25 of its
methods in this bucket.

### 2c. The worked example, and it is the reason this node exists

**`dispatch_host_op_v1` in `ken-host/src/effect_v1.rs`** — the Architect's own
falsifier, caught before the line total was published:

    b4c8df33 (base)    814 lines
    b601e2ec (source)  794 lines
    879f00c9 (port)     24 lines
    origin/main         24 lines     <- the port already EQUALS main

`main` refactored it into `dispatch_host_op_v1_for_promotion_evidence` after
the base. The source's 794-line body is the **pre-refactor** shape, and
transplanting it would undo `main`'s work — the
`AC-REDERIVED-NOT-TRANSPLANTED` failure.

⇒ **It scored +770 lines in this bucket and is not owed at all.** That is the
single largest item by line count and the correct disposition is KEEP MAIN'S.

**SIZE AND PRIORITISE ON METHOD COUNTS, NEVER ON LINE DELTAS.** The Architect's
sentence, carried verbatim as a fixed input: *a line delta measures how much
text moved, never how much is owed, and the two look identical until you check
one against `main`.*

## 3. The method: three dispositions, and each one is SHOWN

For every method on the worklist, record one of:

    KEEP MAIN'S    main's change supersedes the source's, or main refactored
                   past it. The port owes nothing. Default for anything
                   resembling dispatch_host_op_v1.
    TRANSPLANT     the source's change is genuinely owed and main's change is
                   orthogonal or absent in substance. State why main's move
                   does not conflict.
    RECONCILE      both changes are substantive and neither supersedes the
                   other. Write the merged body and say what each side
                   contributed.

**The showing is the deliverable, not the disposition.** A bare verdict per
method is an authored list, and this chain has already ruled once that an
item list carries no evidential weight (`ABI-S6-HS18-D5B-SUBSTRATE-PORT`
`§3a`): *derive the list FROM the tree property; never validate the tree
property against the list.*

⇒ **Each row cites what was compared.** The `dispatch_host_op_v1` row is the
template: four bodies named at four refs, and the conclusion falling out of
them.

**RECONCILE IS THE ANSWER THAT NEEDS THE MOST SCRUTINY AND WILL BE THE RAREST.**
A reconciliation is new code that existed in neither tree, so it inherits
neither tree's testing. **If a RECONCILE row cannot name what exercises the
merged body, say so in the row** — that is a coverage gap worth surfacing, not
a detail to leave implicit.

## 3a. A `#[test]` ROW IS A DIFFERENT ADJUDICATION, AND ITS WRONG ANSWER IS SILENT

**Two kinds of row wear one bucket name** (Architect, `evt_4qmypzdync30x`,
applying the production/test filter the predecessor's `§3b` requires but which
had not been applied to this bucket).

| row kind | "which body survives" is a question about | a wrong call is |
|---|---|---|
| production method | behaviour | caught by whatever exercises it |
| `#[test]` function | **what the suite ASSERTS** | **SILENT** |

⇒ **A test that keeps the weaker of two bodies still passes.** There is no red
to find, so nothing downstream reports the loss — the disposition is the only
place it can be caught.

**THE `dispatch_host_op_v1` DISCRIMINATOR DOES NOT TRANSFER TO THESE ROWS.**
Checking a test against `main` tells you **which assertions `main` currently
makes**, never which ones the port owes. `§2c`'s template answers a production
question and applying it to a test row produces a confident wrong answer.

⇒ **A test row's disposition must state the ASSERTION DELTA** — what each body
asserts that the other does not — and name which assertions survive. *"Checked
against `main`"* is not a disposition for these rows.

**They may not belong to the same reader.** A test-body adjudication is closer
to `runtime-qa`'s question than to the implementer's; routing them there is the
leader's call, not this frame's.

### The Architect's five, classified. Fixed input, not a target.

His extractor at `origin/main`, classified by the attribute immediately
preceding each `fn`:

    PRODUCTION (3)
      construction.rs   :254   Planner::new
      construction.rs  :1296   Planner::finish
      continuations.rs :6294   build_continuation_specialization_plan

    TEST (2)   both #[test] under #[cfg(target_os = "linux")]
      effect_v1.rs :6431  abi_s6_d4_file_acquire_enforces_rights_exact_
                          length_lineage_and_capacity
      effect_v1.rs :7915  abi_s1_duplicate_preserves_policy_rights_and_
                          revocation_lineage

**This classification is his extractor's five and has NOT been applied to the
Steward's 32.** The proportion does not carry across extractors — *"roughly two
fifths of the raw population is tests"* says nothing about which rows land in
**this** bucket. **Re-derive the split over the worklist you actually receive.**

## 4. D0 — answer before writing production

**D0-1. Does any row need a decision that is not the runtime ring's to make?**
A method whose two bodies encode **different intended semantics** is a design
question, not a merge. **Name it and stop** — that routes to the Architect, and
a hard stop here is a good outcome, not a failure of the node.

**D0-2. Is the predecessor's worklist stable at this node's base?** `main`
moves. A method that was ADJUDICATE when the predecessor handed it over can
have become AGREE or MAIN'S by the time this node starts. **Re-run the
instrument at this node's base and diff the two worklists**; report rows that
changed class and treat the fresh run as authoritative. **A row that left the
bucket needs no disposition, and recording one for it would be fiction.**

## 5. Deliverables

1. A disposition for every row on the predecessor's ADJUDICATE worklist, each
   with its check-against-`main` shown per `§3`.
2. The TRANSPLANT and RECONCILE bodies landed on `main`.
3. The re-run worklist diff from `D0-2`, with any class changes named.
4. Any row escalated under `D0-1`, named with the question it raises.

## 6. Acceptance

**AC-1 — EVERY ROW HAS A DISPOSITION AND A SHOWN CHECK.** Re-running the
three-way instrument at the candidate, third tree `origin/main`, reports
**ADJUDICATE = 0** over the pinned file set — every row having become AGREE
(transplanted or reconciled) or been recorded KEEP MAIN'S.

**The instrument states its population from the UNION**, per the predecessor's
`§3b`, and prints the file set it ran over. **Positive control required:** the
same instrument at this node's base must report ADJUDICATE **non-zero**. A zero
from real adjudication and a zero from a file set that matched nothing print
the same word.

**AC-1b — every `#[test]` row states its ASSERTION DELTA (`§3a`).** The
worklist is split production/test in the report, and each test row names what
each body asserts that the other does not, and which assertions survive.
**"Checked against `main`" does not discharge a test row** — that reports which
assertions `main` makes, not which the port owes.

**This is an AC whose evidence must be MANUFACTURED, which is why it is stated
separately.** Every test row will pass in both configurations no matter which
body is chosen — **a wrong call here produces no red at all.** A green suite is
not evidence for this criterion and must not be offered as it.

**AC-2 — no ADJUDICATE row was closed by transplanting over `main`'s work.**
For every TRANSPLANT row, the recorded check shows what `main`'s move was and
why it does not conflict. **The failure this catches is the one that already
happened**: `dispatch_host_op_v1` would have been transplanted at +770 lines on
the strength of its line delta alone.

**Negative control, and it is the informative half:** at least one row is
expected to be KEEP MAIN'S. **If every row resolves to TRANSPLANT, the check is
not running** — that is the signature of a disposition being read off the
source rather than derived from a comparison.

**AC-3 — `trusted_base()` delta is ZERO.** No kernel change is in scope.

**AC-4 — the mechanical buckets stay closed.** The instrument reports OWED = 0
and ABSENT = 0 at this candidate, as the predecessor left them. **This catches
a regression in the other direction**: a reconciliation that drops a
source-owed change re-opens a bucket this node did not own.

## 7. Base

Cut from a `main` containing the predecessor's landed candidate. Pin a literal
SHA, never the ref, and verify it at the moment you adopt it.

## 8. Contention

`crates/ken-runtime/src/cranelift_backend/**` is this ring's own territory. The
live overlap is [[RT-D5B-POSTCALL-REFUSAL-MECHANISM]] (`ready`, unstarted) —
see the predecessor's `§4` D0-3, which is the same semantic hazard and is not
re-opened here.

## 9. Not this node

- The mechanical buckets — OWED, ABSENT, MAIN'S are
  [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]]'s and land before this node starts.
- The refused `MappingAcquireFile` grant —
  [[RT-D5B-MAPPING-AVAILABILITY-FLIP]], deliberately `draft`.
- Repairing the HS10-inline cross-family guard. The predecessor records the
  limitation; widening the assertion is a change to a guarantee and needs its
  own decision.
- Increment A's own substance, and increments B and C.
- Any kernel change.

## 10. Related

- [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]] — the mechanical half and this node's only
  dependency. Its `§3b` is the authoritative statement of the split, the
  instrument, and the third-tree rule.
- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — what the pair unblocks.
- [[ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]] — the landed sibling port
  (`10e75cb93656d5ea787bceaf754b2500b78de166`); the worked precedent for review
  on this surface.

## 11. Symptom inventory

**This WP continues the HS18 chain and does NOT open a new one.** The
authoritative record is [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]] `§11`, which itself
continues [[ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]] `§1b`. **Append entries
there, never here** — two mutable copies of an append-only record is how the
record stops being one, and that section already carries a strike and three
tombstones from exactly this failure.

The Architect appends entries and owns the predicate check. The advancing
hard-stop count stood at **17, next trigger at 18**, when this node was cut
(`evt_7tx7a1n71qa9g`).
