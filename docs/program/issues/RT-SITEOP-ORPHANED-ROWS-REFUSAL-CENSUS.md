---
id: RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS
title: "Three ignored rows name RT-SITEOP-CARRIED-WITNESS, which is merged, so no live node owns their next repair. Their annotations are the only account of what they do, and the annotations DISAGREE WITH EACH OTHER AND WITH THE LEDGER: each row carries a comment block naming RT-CARRIER-BYTESPAN-OBSERVE (also merged) with a BytesPointerLength/CarriedWord signature and the claim that the program never executes, a second comment naming RT-SITEOP-CARRIED-WITNESS D1a/D2, and an #[ignore] attribute predicting the BoundaryCarrier arity refusal -- while the ledger MEASURED all three as `label agrees? NO` with two different signatures again (rows 1 and 2 UnclassifiedRuntimeTrap terminal_value -1, row 10 the typed-consumer-projection planner refusal). FOUR ACCOUNTS, THREE OF THEM ANNOTATION, NONE MEASURED AT CURRENT MAIN. D0 measures the rows and censuses their annotations; it does not adjudicate between labels. THE NODE MAY DELIVER ZERO DECREMENTS: if the ledger's verdict still holds at current main the labels are simply stale and the first deliverable is a LABEL CORRECTION, which is re-labelling and not clearing."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, authored at origin/main c2b4854cb -- WHICH WAS ALREADY SIX LANDINGS STALE WHEN QUOTED AS CURRENT. Rows and labels RE-MEASURED unchanged at origin/main 2dcc67b08944059b411858692618f86bee3de52b, and the three status claims re-verified there; see 'The three rows'. Both bases are recorded and neither is live -- re-derive before acting. SUPERSEDES the WITHDRAWN RT-SITEOP-RESIDUAL-ARITY-SECOND-STOP (candidate 023fc5e01cd3a04a8c0a491de60138f976148f52, never landed, withdrawn at evt_75tner07q0qxt). That node asserted all three rows stop next at the carried-residual arity refusal, sourced from three byte-identical #[ignore] labels. The ledger had measured all three and recorded `label agrees? NO` on each, and said so in prose naming these exact rows: 'what they now do is unrelated to what the label predicts. Rows 1 and 2 trap at runtime; row 10 refuses on typed consumer projection.' The withdrawn node also contained a section warning that identical refusal text cannot say which site fired -- the fence was written and then crossed one level up, treating identical LABEL as identical BEHAVIOUR. Architect evt_3fpkye1tsvbc4 supplied the four D0 requirements this node is built on (partition on phase first; capture each trap's SITE because the sentinel signature carries no mechanism; scope conditional; state the zero-decrement branch in the motivation) and evt_2tkfcc2hqm2xv supplied the producer reading that relocates row 10's candidate mechanism. The scope condition resolved to ALL THREE ROWS IN: runtime-implementer evt_45beq3x7ajkq8 characterized their own fixtures, not row 10, which the Steward confirmed by reading row 10's program at this base."
---

> ## FRAMED 2026-09-19 — `ready`, size M, tier T1
>
> Frame: [`wp/RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS.md`](../wp/RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS.md)
>
> **The frame governs; this file is the origin record and the provenance.**
> Where the two differ, the frame is later. It carries three inputs that
> postdate this node:
>
> - the `-1` provenance chain (Architect, `evt_7v4gb9y06ft6t`) — `-1` is a
>   value the child WROTE after a clean hash-validated handshake, so the
>   producer set is exactly `ken_nc23_entrypoint`'s return;
> - `RT-BORROWED-INPUT-CARRIER-DURABILITY` **CLOSED REFUTED** (`4d0958988`),
>   which **weakens but does not eliminate** the borrowed-ingress candidate for
>   rows 1 and 2;
> - the live crux, unanswered: rows 1 and 2 declare `proc main (_input :
>   ProcessInput)` — **unused** — and it decides the routing either way.

> # OPERATIVE (Steward, 2026-09-18)
> #
> # SCOPE: three rows whose `#[ignore]` names a MERGED node. What they
> # actually refuse with is the question, and it is unmeasured at this base.
> #
> # THIS NODE MAY CLEAR NOTHING. Its opening measurement has a branch that
> # turns it into a re-label. An un-ignore is a RESULT here, never an
> # acceptance criterion, and the motivation below does not promise one.
> #
> # DO NOT infer a shared cause from the shared label. The labels are
> # byte-identical because one node wrote all three. That is the error this
> # node exists because of.

## The three rows

First measured by the Steward at `origin/main c2b4854cb`; **RE-MEASURED and
unchanged at `origin/main 2dcc67b08944059b411858692618f86bee3de52b`** (2026-09-18,
six landings later):

    crates/ken-cli/tests/px7f_resource_native.rs
      :315  fn linked_public_right_denial_preserves_exact_masks
      :349  fn linked_public_second_release_is_closed_and_the_handle_closes_once

    crates/ken-cli/tests/rt_escape_second_resource_native.rs
      :685  fn escaped_buffer_used_by_fanning_host_op_matches_interpreter

All three `#[ignore]` strings are byte-identical, md5 `e123c024` on each
individually. Same line numbers, same strings, at both bases.

**Why the second base is stated rather than the first being edited away.**
`c2b4854cb` was already six commits behind `origin/main` when this node was
authored — the Steward quoted it as current and it was not. The measurement
survived, so the node's substance is unaffected; what was wrong is the claim
about *when* it was true. A fixed input is a claim about a tree at a moment, and
re-anchoring it silently would destroy the only evidence that the original
anchor was stale. Both bases stay.

⇒ **Re-derive against current `origin/main` before acting on any row here, and
do not treat either SHA as live.** This paragraph is not a licence to trust
`2dcc67b0…` either; it will go stale the same way.

### The status claims, re-verified at the same base

    RT-SITEOP-CARRIED-WITNESS     status: merged     (named by all three labels)
    RT-CARRIER-BYTESPAN-OBSERVE   status: merged     (named by comment block 1)
    RT-SITEOP-RESIDUAL-ARITY-...  absent             (withdrawn, never landed)

None of the three rows appears in `.github/ignored-test-exemptions.toml`, so all
three sit in the **swept** population, not the registry-exempt one. That matters
for the ignored-test objective: these rows are counted as live sweep members
today.

### This node is the founding case for `merge-procedure.md` M7a

M7a — the merge step that catches ignored rows whose label names the node being
merged — landed after this node was authored. **These three rows are exactly what
it now catches:** their label names `RT-SITEOP-CARRIED-WITNESS`, and that node's
merge is what orphaned them. Had M7a existed at that merge, this census would not
have been needed.

That does not retire this node — the rows are already orphaned and still need
measuring. It bounds it: **this is cleanup of a class that should not recur**,
not the first of a series. Cf. `CI-IGNORED-SWEEP-EXEMPTION-LIVENESS`, which
covers the complementary set (the registry) and explicitly does not cover these.

## FOUR ACCOUNTS OF THESE ROWS, AND NO TWO AGREE

Every row carries the same three-layer annotation, plus the ledger's
measurement. Read them as four separate claims, because that is what they are:

    1. comment block   "Ignored pending RT-CARRIER-BYTESPAN-OBSERVE"
                       signature: "Effect: seat Argument(0) of FsReadFile
                       needs BytesPointerLength, which it cannot observe in
                       CarriedWord"
                       and: "It refuses at object emission, so the program
                       never executes"
                       owner RT-CARRIER-BYTESPAN-OBSERVE  -- MERGED

    2. second comment  "RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile
                       Argument(0) was site-bound: FileError SiteOperand(0)
                       could not project its carried word."

    3. #[ignore] attr  "RT-SITEOP-CARRIED-WITNESS D2: the carried SiteOperand
                       port succeeds; this row next refuses because a carried
                       recursive hypothesis is an eliminated value, not a
                       callable, but the call provides 1"
                       owner RT-SITEOP-CARRIED-WITNESS    -- MERGED

    4. ledger (measured at 04d4dd38a, `label agrees? NO` on all three)
                       rows 1, 2   UnclassifiedRuntimeTrap
                                   { terminal_value: -1 }
                       row 10      "source-specific inheritances at one
                                   generated entry disagree on their typed
                                   consumer projection, including the
                                   fresh-result route"

**Layers 1 and 3 contradict each other on PHASE**, which is why `D0`
partitions on phase before it compares anything. Layer 1 says the program never
executes; layer 4 reports rows 1 and 2 under a decoder whose input is a process
terminal value. **Both cannot describe the same run.**

**Both named owners are merged.** A merged node with a stale label is the exact
state that makes a row look owned and leaves it unroutable — the ledger says so
in those words, and these rows are its instance.

## THE SENTINEL SIGNATURE IS NOT A CLUSTER

`UnclassifiedRuntimeTrap { terminal_value: -1 }` is **where every unclassified
failure lands**. Rows 1 and 2 sharing it is not two rows sharing a mechanism; it
is two rows carrying no mechanism information at all. Producer, read at this
base:

    object_linker_packaging.rs:256  fn decode_signed_root_trap(terminal_value,
                                                               catalog)
      let refuse = || UnclassifiedRuntimeTrap { terminal_value };
      if terminal_value >= 0 { return Err(refuse()) }
      magnitude = (terminal_value as u64).wrapping_neg()
      root_trap_catalog_index(magnitude).ok_or_else(refuse)?
      catalog.get(index).cloned().ok_or_else(refuse)?

So `-1` is negative, passes the first guard, and **fails catalog lookup**: the
artifact's finite root-trap catalog has no entry for magnitude 1. The signature
records that the decoder could not name the trap, and nothing else.

**A trap has a SITE even when it has no class.** That is the only thing that
could relate rows 1 and 2, and `D0` captures it.

**Rank any signature by whether it could have differed before reading agreement
into it.** A default or sentinel bucket guarantees agreement and carries no
mechanism; a specific signature naming a condition a differently-failing row
would not carry is real evidence. Ledger class B — `"two host response cases
claim one operation constructor"`, four rows, `label agrees? YES`, live owner —
is the second kind and is not disturbed by anything here.

## ROW 10 IS NOT ON THE SAME ROUTE AS THE BRACKET FIXTURES

`RT-BRACKET-RELEASE-ORDER-PARITY`'s probes reached row 10's refusal **text**
from a two-call inner bracket body. That is not row 10. Read at this base, row
10's program `ESCAPE_BUFFER_THEN_READAT` contains exactly **one** host call:

    proc read_with_escaped_buffer (file) (buffer_closed) =
      bind ... (readAt AFull file (0 : Int) buffer_closed
                       (MkBufferWindow (0 : Int) (6 : Int)))
        (\outcome. Ret ... (ResourceBodyOk Unit Unit MkUnit))

One `readAt`, then a pure `Ret`. **Nothing can be a second call**, so row 10
cannot be on the two-call route at all.

The producer has **one** site, so the site is shared — but the guard is a
disagreement test, not a sameness test:

    aggregates.rs:7452-7458
      Some(confluence) if confluence.projection != projection => Err(...)

    SAMENESS    buys the COLLISION -- two entries land on one coordinate
    DIFFERENCE  is the REFUSAL -- their consumer projections disagree there

And there are **two routes to that one guard**. Immediately above it, every
inheritance in one continuation context is forced onto the first coordinate
seen for that context, discarding `call_origin` and `callee_origin` for all but
the first — so a collision can be manufactured by shared context alone, with no
operation identity involved (Architect `evt_2tkfcc2hqm2xv`). **Row 10, having
nothing to be a second call, can only be on the context route if it reaches this
producer at all.** That is what `D0` measures for it.

`CheckedIhGeneratedEntryCoordinate` carries no operation field. `callee_origin`
is the **named, UNVERIFIED** candidate carrier of operation identity; confirming
it is one read and it belongs to this node.

### The cheapest read in this area, and it needs no fixture

The `Some` arm is **three guards, not one**, and a collision that agrees is a
SUCCESS by design — the member joins the class and planning continues:

    if confluence.projection != projection            -> Err (row 10's refusal)
    if confluence.retarget_caller != retarget_caller  -> Err (different message)
    if !confluence.members.insert(member)             -> Err ("one source-call
                                                       identity was inserted
                                                       twice into one class")
    // otherwise: the member JOINS the class and planning continues

So "is a collision fatal?" is already answered — it is not. The live question
(Architect `evt_56mp2pds9p9zm`, replacing a control they withdrew because one
of its branches was structurally unreachable) is **whether the agreement path
is ever taken**:

> Does any `CheckedIhGeneratedEntryConfluence` ever hold more than one member?

Instrument the class sizes on an ordinary compile. No authored program, no
surface gymnastics.

    SOME CLASS > 1   agreement is real and reachable; the guard means what its
                     text says, and row 10's refusal is a genuine disagreement
                     between two members.
    ALL SINGLETONS   the agreement path is never exercised, so in practice
                     "a second member arrives" and "the refusal fires" are the
                     same event. The guard's text says the members DISAGREE;
                     the behaviour would be that a second member refuses, full
                     stop -- much stronger than the text, and the right
                     sentence for this node.

The third guard is what makes this worth asking: `"one source-call identity was
inserted twice"` exists only because the design expects multiple distinct
members. **If none ever arrives, that is a gap between the design and what the
planner produces**, and it is the sort of thing row 10 could be sitting on.

## WHAT THIS NODE IS WORTH, WITH THE DOWNSIDE FIRST

**The opening measurement has a branch that delivers zero decrements:**

    STILL DISAGREE   the labels are stale. First deliverable is a LABEL
                     CORRECTION -- re-labelling, not clearing. These rows
                     join the re-label bucket and the node decrements
                     NOTHING.
    NOW AGREE        behaviour changed between 04d4dd38a and this base.
                     That change is dated and it is the lead.

**The first branch is the favourite**, because the ledger measured it and
nothing since has been measured against it. This node is not justified by a
promised decrement and must not be described as if it were.

What justifies it is narrower and survives either branch: **three rows whose
only account of themselves is four mutually inconsistent annotations, two
merged owners, and no live node that would ever correct them.** That is
unroutable state, and it is cheap to end.

## RELATED, OUT OF SCOPE, FOR WHOEVER OWNS THE CENSUS

`RT-CARRIED-RESIDUAL-IH-ARITY` is **`closed`** at this base, and the four rows
that genuinely carry the arity refusal (ledger rows 3-6, in
`px7l_checked_host_recursive_bind.rs` and
`px7m_hostresult_computational_match.rs`) still name it. If a closed node leaves
its rows blocked, those four are orphaned the same way these three are, and the
orphan count is seven rather than three. **Not measured here and not this
node's to fix** — recorded because it was found while establishing that these
three rows do not carry that refusal.

## What must not happen

- **Do not adjudicate between the annotations.** All four accounts are claims.
  `D0` measures the rows; it does not decide which label was right.
- **Do not cluster rows 1 and 2 on the sentinel signature.** Partition on
  phase, then site.
- **Do not carry any characterization from `RT-BRACKET-RELEASE-ORDER-PARITY`'s
  fixtures.** They share a refusal text with row 10 and not a program shape.
- **Do not relax any refusal.** A diff that edits a guard to admit what it
  currently rejects is a hard stop to the Architect, not a repair.
- **Do not add an `#[ignore]`.** The count is what this work exists to move.
