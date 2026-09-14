---
id: RT-D5B-POSTCALL-REFUSAL-MECHANISM
title: "What is the mechanism of the CheckedIhDetachedCallerCut refusal at lowering/core.rs:7720 -- the only distinct Packaging reason in abi_s6_mapping_file_backed_native and the cause of 8 of its 11 base reds, where a two-step SelfDefining post-call consumer receipt meets one computational eliminator frame. THREE mechanisms have been proposed and measured away in one session, all sharing the premise that the defect is reachable from the call site's inputs; the measurement says those inputs are internally consistent and the disagreement is upstream of them. The mechanism is UNKNOWN and this node exists to find it, not to carry a candidate"
status: ready
owner: runtime
size: M
gate: architect
depends_on: []
blocks: []
github: null
tier: T1
origin: "Cut by the Steward 2026-09-14, superseding two unlanded drafts withdrawn before commit (RT-POSTCALL-RECEIPT-ALIGNMENT-AUTHORITY, RT-SELFDEFINING-RECEIPT-FRAME-ARITY), each named for a mechanism refuted within minutes of being written. This node is deliberately named for the QUESTION. Chain: implementer evt_4y7sypns2j0v, Steward hold evt_5t0n56p3jvpck, implementer withdrawal evt_1281f1q5v7j9p, Architect evt_5x5cfwgp4bk24 + evt_41r81fdj0741f + evt_pzd11rqycqe, implementer measurements evt_49vn1p2c57e7b + evt_5ycnn29hy38qs + evt_3gyahpfpnja76. COORDINATION §1a fired by the Steward on the third refuted mechanism."
---

> # READ FIRST: THREE REFUTED MECHANISMS. THIS NODE CARRIES NO CANDIDATE.
>
> Each was proposed by a competent seat, was locally reasonable, and was killed
> by measurement — two of them by the proposer's own stated criterion. **Naming
> a fourth before reading this list is the failure mode this node was recut
> twice to avoid.**
>
> **1. TRIM PLACEMENT — VOID.** *"Where does trim authority live for `:7720`"*
> has no content: `:7720` is reached only under `SelfDefining`, where
> `executable_exits()` and `selected_case_exits()` return the **same value**.
> There is nothing to trim. (Architect fork `evt_5x5cfwgp4bk24`, voided by
> measurement `evt_49vn1p2c57e7b`.)
>
> **2. ACCESSOR CHOICE — DEAD EVERYWHERE, NOT JUST HERE.** Swapping the accessor
> at `:7720` changes nothing. Grounded by a **closed producer enumeration**, not
> by sampling: a `CheckedIhPostCallConsumer`'s selection is `None` or
> `SelfDefining` and `CallerCompleted` is **unconstructible** on it
> (`evt_5ycnn29hy38qs`).
>
> **3. MISSING ROUTE — REFUTED NUMERICALLY.** `:7720` not routing a
> required-consumer incoming edge is real (see the surviving census below) but
> **does not explain the mismatch**: `incoming_consumer_edge_index` is **0** at
> all 11 reaches, so `executable_exits()[0..]` is the whole 2-chain and
> `&eliminators[0..]` is the whole single frame. **Both halves of the
> correspondence are no-ops at index 0.** Routing yields the identical 2-against-1.
> (Architect account `evt_pzd11rqycqe` §5, refuted on its own stated criterion by
> `evt_3gyahpfpnja76`.)
>
> **The shared premise of all three: that the defect is reachable from this call
> site's inputs.** The measurement says those inputs are internally consistent at
> index 0, and the disagreement is **upstream of all of them**.

## The observation

`lowering/core.rs:7719-7720` at `0d94d58b6` — re-derive at your base (`AC-1`):

    let residual = self
        .checked_ih_post_call_residual(consumer.selected_case_exits(), eliminators)?;

Measured across the whole suite, no variation:

    11 x   site=7720  index=0  executable_exits=2  selected_case_exits=2
                              suffix=2            eliminators=1

The receipt describes a two-step exit chain; the site holds one computational
eliminator frame; the bounds guard at `core.rs:9256` refuses. **This is the only
distinct `Packaging` reason in the run and the cause of 8 of 11 base reds.**

## What is NOT in doubt

- **Completeness, not soundness.** The compiler **rejects a valid program**; it
  does not admit an invalid one. Rests on the `:9256` bounds guard and the
  `#[track_caller]` counts, which survived every correction.
- **The guard works and must not be weakened.** When a probe equalized the
  counts, the refusal **moved** to the per-frame content arm at `:9268` and the
  wrong defining call still failed to compile.
- **No length-derived trim, at any of the five call sites, ever.** Architect,
  `evt_41r81fdj0741f`: `caller_exit_index` is an alignment witness; a difference
  of two lengths is not. Structural and variant-independent — **and now vacuous
  here**, since there is nothing to trim.
- **`:7720` is the only site in the crate that stops after the first half of the
  two-step protocol.** Census at `evt_pzd11rqycqe` §4; the protocol is written
  out at `planning/static_transition/aggregates.rs:7826-7829` and
  `responses.rs:598-606`. **This survives the refutation of mechanism 3** — it is
  a real asymmetry, it is simply not the cause.

## Deliverables

> # EVERY DELIVERABLE IS ANSWERED. THE NODE IS NOT CLOSEABLE, AND THE REASON IS
> # ITS BASE, NOT ITS CONTENT.
>
> **Disposition recorded by the Steward 2026-09-14 on the Architect's
> `evt_w1vnqexkhmxx`** (*"`MECH` answered, `CTRL` complete, `R3` landed, `ROUTE`
> declined with reason ⇒ the node's deliverables are closed"*):
>
>     MECH    ANSWERED   ruling below, evt_5k7jd5agh133j + evt_11zby14s19hkh
>     CTRL    COMPLETE   88893210a (the class assertion, ruled as D3)
>     R3      BUILT      43e4451e3 + ff7638ff6, QA APPROVED
>     ROUTE   DECLINED   evt_w1vnqexkhmxx -- a theorem, not a deferral
>
> **"`R3` landed" means landed ON THE BRANCH, not on `origin/main`.** `43e4451e3`
> and `ff7638ff6` sit on `wp/ABI-S6-d5b-file-backed`, 24 commits deep, on top of
> four `ABI-S6 D5b WIP` commits carrying 12374 unlanded insertions.
> `git merge-tree --write-tree origin/main ff7638ff6` differs from
> `origin/main^{tree}`, so none of it is on `main`.
>
> ⇒ **DO NOT FLIP THIS NODE `merged`.** Deliverables being answered is not the
> node being closed; **closure follows the merge.** This node joins `D2`, `R1`,
> `R2` and entry-18 in the held-pending-D5b set, and it is held by the same
> thing they are: there is no clean base below the work. See
> [[RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]]'s held block.
>
> # AND DO NOT DISPATCH `CTRL-a`. IT IS BUILT. THE REMAINING OBSTACLE IS
> # LANDING, NOT A DELIVERABLE.
>
> **The Steward told the ring twice that `CTRL-a` was "startable now". That was
> wrong when it was said, not merely stale** — `88893210a` was already in the
> branch at the time (Architect, `evt_6zm1feakd1k6p`; verified independently):
>
>     git merge-base --is-ancestor 88893210a ff7638ff6   -> YES
>     git merge-tree --write-tree origin/main 88893210a  -> 4a52074f91586eb514e8b3116e5b1b1ef889f58d
>     git rev-parse origin/main^{tree}                   -> 07936923a39de217dfccf9c97bc30033d4eb8be0
>
> Ancestor of the branch tip; its merge against main differs from main's tree.
> **Built and unlanded, exactly like `R3`.** Dispatching it sends an implementer
> to rebuild what the branch already holds, and they would correctly hard-stop.
>
> **Read this deliverable list as a RECORD, never as a queue.** Every entry is
> answered or built; none is pullable. The one thing that would reduce it is a
> clean base under 24 commits, which is not work a build seat can pull.
>
> **State the scope beside every branch number, because three true numbers
> disagree:**
>
>     git diff --stat origin/main...ff7638ff6   35 files 18592+ 6144-   vs MERGE-BASE: what the branch ADDS
>     git diff --stat origin/main   ff7638ff6   47 files 18612+ 9062-   vs MAIN TIP: also reverses main's 21 commits
>     git diff --stat 4bf1ad362     5a9a840ba   34 files 12374+ 5644-   the four bottom WIP commits ALONE
>
> The Steward and the Architect each reported one of the first two without
> naming which question it answered, within an hour of reconciling the same
> defect shape one artifact down.
>
> **What is NOT a blocker, measured so nobody re-derives it:** `0bbe4a175`
> ("REFUTED ... DO NOT BUILD ON") leaves ZERO residue —
> `git rev-parse 936294421^{tree} 686ffa8ac^{tree}` returns
> `b41367855cc5f7b2a0ea098ed94a83b27c15cbc8` for **both**, and the diff between
> them is empty. Under squash-merge no intermediate commit lands at all. The
> chain's shape is not what holds this work.

> **DELIVERABLE LABELS ARE `MECH` / `CTRL` / `ROUTE`, DELIBERATELY NOT `D1`/`D2`/`D3`.**
> An earlier revision used `D2` for the control repair while
> `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`'s **held** `D2` was live in the same
> thread — **one term naming two artifacts**, which is the exact defect class
> this node was cut to chase, appearing in its own labels. The control-repair
> commit `88893210a` carries the old label. **Renamed rather than annotated:**
> annotation leaves a satisfied criterion and an unsatisfied one looking alike.

**`MECH` — FIND THE MECHANISM. Upstream of the call site, not at it.** Which
producer emits a two-step exit chain for a body the lowering site reaches with
one computational frame, and **where does the disagreement actually live?**
**Both operands are internally consistent at `:7720`** — start where they are
produced, not where they meet.

> **`MECH` ADMITS THREE OUTCOMES AND MUST NOT BE READ AS A TWO-WAY CHOICE**
> (Architect, `evt_53mb86qj64fj1`):
>
>   1. the receipt is wrong, **or**
>   2. the eliminator frame set is wrong, **or**
>   3. **BOTH ARE CORRECT AND THE CUT BETWEEN THEM IS MISPLACED.**
>
> **An earlier wording of this deliverable asked "which of the two is
> inconsistent with the program?", which presupposes outcome 3 away.** That is
> the shared premise of the three refuted mechanisms, reappearing in the sentence
> meant to escape it — and the evidence points at 3: the frames equal
> `receipt[1..]` elementwise, and the index producer names the consumer's **own**
> occurrence, so both operands may be correct under one convention applied at the
> wrong place. **Do not re-narrow it.**

**State the DIRECTION and report counts with the population traced** (`AC-2`).
**"I could not determine it" is an acceptable answer** and must be reported as a
finding with its argument, not as an absence.

**`CTRL` — repair the co-class control. Independent of `MECH`; does not wait on it.**
`wrong_defining_call_breaks_the_required_consumer_edge`
(`abi_s6_mapping_file_backed_native.rs:463`) asserts one of **three co-class
reason strings**, all emitted as `CheckedIhDetachedCallerCut`:

    :9256   "is longer than the exact local eliminator prefix"
    :9263   "does not match a computational local prefix"
    :9268   "does not match the exact local eliminator prefix"

All three mean *the edge broke*, so the control **cannot distinguish "the guard
fell" from "the guard moved"** — and on 2026-09-14 it reported the second as the
first, producing a false refutation that reached an Architect ruling before
anyone opened the panic line. Assert the refusal **class**; if an exact arm is
load-bearing, assert it separately and say why.

**This is the durable product of the session and no correction touched it.**

### `CTRL` HAS THREE PARTS AND THEY DO NOT SHARE A SEQUENCE

Folded here rather than cut as a sibling: same control, same file, same guard, so
a sibling node would only lengthen the path (`§4e` — a duplicate is a fold, not a
frame). **But the parts have different dependencies, and leaving that implicit is
what the Architect warned this must not become — an unwritten expectation.**

**`CTRL-a` — assert the refusal CLASS. Independent. Startable now.** The repair
described above. Depends on nothing in this node.

**`CTRL-b` — RETIRE `D5bHs17PostCallConsumerMutation::ReplayCompletedSelectedExit`.
Depends on `R3`.** Under the anchor join, accessor choice becomes
**unobservable**: the extra leading step in the untrimmed chain is exactly what
the anchor skips, so both paths land on the same window against the same frames.
**The hazard did not become undetectable — it ceased to be a hazard**, and those
two look identical on a red dashboard, which is why the distinction is written
here.

**Put the reason in the enum's DOC COMMENT, not only in the commit message**
(Architect, `evt_5b93c7nrh14r0`). An inert mutation left in place invites a
future reader to "restore" the control by weakening the join — the worst
available outcome. The next reader asking *"why is accessor choice not tested
here?"* must meet the answer at the code.

**`CTRL-c` — COVER THE JOIN'S TWO REFUSALS. Depends on `R3`. BOTH, not one.**

    no-match         perturb a receipt step's (eliminator_origin,
                     checked_frame_id) so frame 0 matches nothing -> must refuse
    multiple-match   duplicate the frame-0-matching step -> must refuse on
                     non-uniqueness

**Why this is required rather than thorough: `R3`'s entire soundness argument
rests on these refusals, and nothing currently exercises either.** *Manufacture
cannot refuse, a witness can* — the join is admissible **precisely and only**
because it fails closed on no-match and on multiple-match. **A soundness argument
resting on a refusal no test reaches is the defect class this whole node exists
to chase.** If those paths are unreachable, or are later "simplified" to a
sensible default, nothing catches it and the type goes on proving something it no
longer proves.

**One mutation is not enough.** They are different refusals on different
conditions; a control exercising only the first says nothing about whether the
second arm is live — the same reason `AC-2` demands a population rather than a
verdict.

**The direction argument, since re-pointing a sentinel requires one:** pre-`R3`
the movable operand at these sites was **which accessor** was passed; post-`R3`
that is fixed by construction and what remains movable is **the correspondence
between receipt steps and frames**. **A mutation control must move the operand
the contract actually depends on**, and `R3` changed which operand that is. This
is a re-point *along the contract*, not to whatever is convenient.

**`AC-6` is not in tension.** It forbids synthesizing an operand in the
**production** path to make a check pass. A **test-only** mutation that corrupts
an operand to verify a **refusal** is the opposite, and is what the mutation enum
exists for.

### STANDING RISK [DISCHARGED 2026-09-14, see below]: `by_identity_join` WAS TAKEN ON A PRODUCTION PATH THAT NO FIXTURE MEASURED

**The record below is kept in its original tense deliberately.** It states the
risk as it stood, then discharges it. A heading asserting a condition that no
longer holds is how a node mis-frames a reader who never reaches the body — the
defect this node was recut twice to avoid — so the discharge is in the heading
and the reasoning is preserved intact underneath.

**`CTRL-c` does NOT cover this**, and that is why it is written here rather than
left as a line in the thread. The two join refusals are exercised at the sites
the fixtures reach; **this is not one of them.**

Verified at `0d94d58b6` — the ambient caller of
`apply_required_consumer_incoming_edge` exists in **both** feature profiles:

    :4339   #[cfg(feature = "px8-ds-test-support")]
    :4349       self.apply_required_consumer_incoming_edge(edge, eliminators)?
    :4356   #[cfg(not(feature = "px8-ds-test-support"))]
    :4358       self.apply_required_consumer_incoming_edge(edge, eliminators)?

**The same logical call under the two features.** ⇒ with the test-support feature
off — the shipping configuration — this path still runs and still takes
`by_identity_join`, **with no measurement behind it in any current fixture.**

**Accepted by the Architect at `evt_5951xezxy3c00`, for a stated reason that is
the point of recording it:** the join **fails closed**, so an unexercised path
taking it **cannot mis-compare silently** — it either produces a witnessed anchor
or refuses. **Defaulting that path to anchor 0 would have been the dangerous
choice**, because a default cannot refuse. This is the same discriminator as `R3`
itself, applied to the question of what to do about an unmeasured path:
**manufacture cannot refuse, a witness can.**

**Do not close this node by claiming the join is exercised everywhere it runs.**
It is not, and the argument for accepting that is the fail-closed property, not
coverage.

### DISCHARGED 2026-09-14 — and the transfer to the shipping profile is a CONSTRUCTION, not a sample

**The risk above was written about the VALUE** — *"the anchor is 0 where the
EDGE arm runs is NOT proved"* — and that is what is now discharged. One census
answered it and `ROUTE`'s refuting criterion together (`evt_2e6r1rza68k5w`): the
ambient edge-arm at `:4366` / `:4383` had no fixture behind it and now has **15
measured reaches across two venues, all `index = 0, anchor = 0`, all accepted**.

**The cfg objection, raised and then closed rather than waved through.**
`ken-cli` enables `px8-ds-test-support` unconditionally (`Cargo.toml:28`), so all
15 reaches are the `#[cfg(feature = ...)]` copy and the shipping
`#[cfg(not(...))]` copy is executed by no fixture. **The two copies are NOT
identical** — the Architect opened both at `ff7638ff6` rather than reasoning from
"it is the same logical call", which was a paraphrase that had been carrying the
discharge:

    test copy       an extra `record_required_consumer_call_selection(edge)?` before the
                    apply, and a PRECEDING arm gated on `d5b_hs17_post_call_consumer_mutation()`
    shipping copy   neither

**Neither difference can reach this question, and the reason is a construction:**

- The anchor comes from
  `AnchoredEliminatorWindow::by_identity_join(edge.executable_exits(), eliminators)`
  and the index from `edge.incoming_consumer_edge_index()`. **Both expressions
  are textually identical in the two copies**, over `edge` and `eliminators`,
  which are bound BEFORE the `cfg` split and are not derived from
  `static_transition_plan`. `edge` is live across that `&mut` call and the crate
  compiles, so the recording provably cannot mutate or invalidate either input.
- The preceding arm is gated on an INJECTED MUTATION SELECTOR, not on a property
  of the program being compiled, and when it fires it diverts to the residual
  path and never reaches the edge arm. It can neither divert an unmutated program
  away from the edge arm nor contaminate an edge-arm observation.

⇒ **The anchor and index at the edge arm are profile-independent by
construction, so the 15 measurements transfer.** What remains unmeasured in the
shipping profile is EXECUTION, not the VALUE. **Do not re-state this as "logic
measured, shipping profile not"** — that hedge sounds narrower and is less
accurate, and it invites the census to be re-run for nothing.

**Verified independently by the Steward at `ff7638ff6`, `core.rs:4337-4390`,
before recording the discharge:** `required_consumer_edge` is bound at `:4337`,
ahead of the `cfg` split; both edge arms call
`by_identity_join(edge.executable_exits(), eliminators)` verbatim; the HS17 arm
routes to `checked_ih_post_call_residual`, not to the apply.

**What this does NOT license.** The join is still fail-closed rather than
coverage-complete, and that is what made the pre-discharge posture defensible —
see `ROUTE`'s converse above for the one configuration (`anchor != 0` with
`index > anchor`) that no constructor has been shown to exclude.

**SEPARATE, NOT A DEFECT CLAIM, AND NOT THIS NODE'S** (Architect): the shipping
build does not call `record_required_consumer_call_selection` at all. Whether
anything downstream needs that record is unestablished. A recording that exists
only under a test feature is worth someone's eye on its own terms.

### `ROUTE` IS DECLINED. NOTHING TO BUILD. THIS IS A THEOREM, NOT A DEFERRAL.

**Architect ruling `evt_w1vnqexkhmxx`, recorded here rather than left in thread.**
`ROUTE` was framed as a CORRECTNESS deliverable — route the required-consumer
incoming edge at `:7720`, following `aggregates.rs:7826-7829`.
**It is declined, and the reason is not "it measured zero."**

**Routing is the identity wherever the anchor is non-zero, by construction, and
the anchor is non-zero exactly when the ambient stack begins after the defining
occurrence — which is the fact `R3` encodes.** `ROUTE`'s subject and `R3`'s
premise are mutually exclusive and cannot both hold.

The two numbers are independent joins, which is what makes the relation real
rather than a tautology — different needles into the same receipt:

    anchor (by_identity_join)     needle = eliminators[0]'s (static_origin, checked_frame_id)
    index  (SelfDefining producer needle = required_call.destination().consumer_occurrence()
            responses.rs:2400-2412)

`index=0, anchor=1` therefore says: the destination's consumer occurrence sits at
`receipt[0]`, the ambient window begins at `receipt[1]`, **and the route's cut
point lies strictly BEFORE the window.** `index` names the consumer's own
defining occurrence; the ambient stack begins after it; cutting a list at a point
preceding its element 0 yields the whole list. ⇒ **At any ambient site with a
non-zero anchor, routing is the identity structurally — not because this program
happened to measure 0.**

The converse closes it, and the strictness matters: for `ROUTE` to have content
at an ambient site you need **`index > anchor`** — routing removes an element
only when `index - anchor > 0`. The ambient stack merely CONTAINING the
consumer's own defining occurrence is the weaker predicate `index >= anchor`,
which is **necessary and not sufficient**: at `index == anchor` the definition
is the window's FIRST element, so containment holds and cutting there removes
nothing. That boundary case is not hypothetical — it is exactly what
ambient-apply measured 15 times on 2026-09-14 (`index = 0, anchor = 0`).

**Two disjoint ways routing fails to have content, and this node exhibits both:**

    :7720           index <  anchor   0 - 1   cut precedes the window; UNREPRESENTABLE
    ambient-apply   index == anchor   0 - 0   cut is the window's head; the IDENTITY

**What is NOT established, stated plainly because an earlier version of this
paragraph asserted it.** That earlier text said if routing ever had content
"the anchor would be 0 and `apply_required_consumer_incoming_edge` would
ACCEPT", and concluded the refusal and `ROUTE`'s subject are *exactly
complementary* so declining costs no capability. **`index > anchor` does not
force `anchor == 0`.** The anchor is obtained by identity join and `:7746` says
it is "never computed as `index + 1`", so "the stack begins after the defining
occurrence" describes what was measured, not what a constructor enforces. A
site with `anchor != 0` AND `index > anchor` would have routable content and
would be refused by `R3`'s guard — the one combination in which declining costs
capability. **No such site is known or measured, and no constructor excluding
it has been found.** That case is what the refuting criterion below exists to
find; it is not excluded by argument.

⇒ The decline stands on what is measured and derived: at every observed ambient
site routing is the identity, by one of the two routes above. It does not stand
on a proof that no other configuration exists.

**The `index - anchor` repair is not merely unmeasured, it is UNREPRESENTABLE.**
At `:7720` it is `0 - 1`, and no `usize` names a position before the window
starts. The semantically correct answer in that regime is "the whole window" —
which is exactly what not routing already does.

**THE CENSUS THAT WARRANTED `ROUTE` IS REAL AND WAS MIS-READ. RECORD IT AS A TYPE
DIFFERENCE, NEVER AS A MISSING ROUTE.** The two halves of the protocol operate on
**different lists**:

    aggregates.rs:7826   planner    applies the index to STEPS  (required_consumer_executable_suffix)
    core.rs:7757         lowering   would apply it to FRAMES    (the ambient eliminator stack)

The planner's index is coherent because the list it indexes IS the receipt.
Lowering's ambient list is not the receipt and does not share its membership
convention. **The asymmetry is not lowering forgetting to route; it is a type
difference between the two halves of the protocol.**

> #### WHAT WOULD REFUTE THIS RULING — stated in advance, per `AC-7`
>
> **If any ambient post-call consumer site has `index > anchor` — equivalently,
> `window_anchor == 0` with `incoming_consumer_edge_index > 0` in the anchor-zero
> case — then routing has real content at an ambient site and this ruling is
> wrong.**
>
> **The strictness is load-bearing and was wrong when first written.** The
> original general form read `index >= anchor`, which the 2026-09-14 census
> SATISFIES at ambient-apply (`index = 0, anchor = 0`, 15 reaches) while routing
> there is `[0..]`, the identity. A refuting criterion that fires on data all
> parties agree is inert is worse than no criterion, because the hit looks like
> evidence. Caught by `runtime-implementer` against the census run to check it.
>
> **THE CENSUS HAS BEEN RUN — 2026-09-14, `evt_2e6r1rza68k5w`, `ROUTEPROBE` over
> every edge-bearing ambient site in both venues.** No hit; no variation:
>
>     site                          observations          index   anchor
>     ambient-apply  (:4366/:4383)  11 mapping + 4 px8f     0       0
>     :7720                         11 mapping              0       1
>
> **The criterion is retained as a STANDING one, not as outstanding work.** It is
> not blocking, and it does not license the ruling — declining is
> behaviour-neutral. It is what would REOPEN the question at any site that
> appears later.

**`ROUTE` is the FOURTH proposed view of `:7720`, and like the first three it is
the identity at the site.** What is new is that the reason is now derived rather
than measured. This is the same predicate `§1b` already closed — *a receipt index
meeting a list with a different membership convention* — and declining `ROUTE` is
that closure holding, not a fourth refutation of it.

## `MECH` IS ANSWERED, AND THE RULING IS RECORDED HERE RATHER THAN IN THREAD

**Outcome 3: both operands are correct and the cut between them is misplaced.**
The two site families build their eliminator lists under **different membership
conventions** while sharing **one index convention**.
`checked_ih_post_call_eliminators` emits exactly one frame per step or fails —
no skip, no filter, both in-body branches are error returns — so at the
constructed sites correspondence holds by construction. At `:7720` the list is
the **ambient** local frame list, where it is an assumption.

`incoming_consumer_edge_index` names the position of the consumer's **own**
defining occurrence, uniqueness enforced (`responses.rs:2366-2396`). At
`index=0`, `[index..]` is a no-op and `[index+1..]` matches — which is why
mechanism 3 measured as vacuous.

### THE RULING (Architect, `evt_5k7jd5agh133j` + `evt_11zby14s19hkh`)

**`R3` — THE FRAME LIST MUST CARRY ITS ANCHOR.** The eliminator slice must carry
**which receipt element its element 0 answers to**, as a type;
`checked_ih_post_call_residual` takes that type and never a bare slice.

**Two rejected repairs, and why — both remain rejected:**

- **`R1`, give `:7720` a constructed list** — rejected because it makes the
  guard's only discriminating check a tautology. **`:7720` is the one site where
  this guard has teeth**, and weakening it by other means is still weakening it,
  even though no arm's condition changes.
- **`R2`, a bare `[index + 1..]`** — rejected as re-encoding the same unstated
  convention one site over.

**THE DISCRIMINATOR, which generalizes beyond this node: MANUFACTURE CANNOT
REFUSE, A WITNESS CAN.** `index + 1` is a function of the index alone — it
**cannot fail**, and yields an anchor that makes the counts agree whether or not
the frames relate to the receipt at all. The identity join **fails closed**: no
match or multiple matches, and it refuses. That is the whole test, and it is why
the join is a witness where arithmetic is a synthesized one.

**The anchor must derive from identity ORIGINATING AT PRODUCTION, never from
arithmetic over the quantities being reconciled.** The Architect's earlier
phrasing — *"obtained from where the ambient frames are produced"* — was
corrected as unbuildable: no receipt is in scope on the
`:3320 → :3364 → :3411 → :3798 → :7720` descent, and the anchor is a property of
the **(frames, receipt) pair**, which first exists at `:7720`. **The line is the
source of authority, not the location.**

`core.rs:3330-3332` already declares this tuple the shared derivation —
*"the checked bridge must carry this exact tuple, and two spellings is how they
part"* — so the join **consumes** the mechanism the code provides rather than
re-spelling it.

**REQUIRED CONDITION — the join fixes WHERE the window starts, never WHAT is in
it.** `checked_ih_post_call_residual` must still compare the full anchored
window `receipt[anchor..]` against the frames **including element 0**, and still
enforce the length relation. **Do not short-circuit element 0 as redundant
post-join** — that turns `:7720` into exactly the tautology `R1` was rejected for
creating. Preserved and non-definitional: the join's existence and uniqueness,
the length relation from anchor to end, and the tail compared pairwise.

**THE RESIDUAL, RECORDED AS A TRADE AND NOT AS A FREE REPAIR:** at a length-1
frame list the pairwise content check is entirely definitional after the join, so
the guard there reduces to existence, uniqueness and length. **That is weaker
than today's positional check — in a case where today's positional check is
wrong.**

**The type takes exactly two constructors and no general one from a bare
`usize`:** anchor 0 from a constructed list (justified by
`checked_ih_post_call_eliminators` emitting one frame per step or failing), and
the identity join (the unique receipt position matching `frame[0]`'s
`(static_origin, checked_frame_id)`, or refuse). **A `usize` constructor would
let `index + 1` back in through the front door and the type would prove
nothing.**

**Sequencing, approved:** land the anchor type with the constructed sites at
anchor 0 — behaviour-identical and independently verifiable — with `:7720`
untouched until the ambient half follows.

**`required_consumer_executable_suffix` and `:7720` must not be decided
separately.** They read the same cut, and `aggregates.rs:7826` consumes it too.

## Acceptance criteria

**`AC-1` — re-derive every line number and count at your own base.**

**`AC-2` — every instrument reports counts and the population it searched, never
a bare verdict.** Non-negotiable on this arc: a bare red produced a false
refutation today, and a zero-hit grep on a wrong path produced a false absence.

**`AC-3` — an enumeration must be closed at its PRODUCERS, not sampled.** The
`SelfDefining` fact generalized only when every construction site was enumerated
and found closed at two. **A larger sample of instances would never have licensed
it** (`evt_5ycnn29hy38qs`). Any "by construction" claim in this node's closure
must name its producer set.

**`AC-4` — `CTRL`'s repaired control must be SHOWN to discriminate** *guard fell*
from *guard moved*. Passing is not sufficient. **Point it at the ruling
(Architect, `evt_11zby14s19hkh`): it must show a wrong defining call STILL
REFUSES under the anchored path, and the join's new refusals — no match, multiple
matches — belong in the asserted refusal class.** The residual above makes this
the check that carries the guard's remaining teeth at a length-1 frame list.

**`AC-5` — the guard is untouched.** No change to the conditions at `:9256`,
`:9263`, `:9268`.

**`AC-6` — nothing is synthesized to make two counts agree.** No fabricated
receipt, step, eliminator frame, or index. Manufacturing either operand is
indistinguishable from fixing the defect and tests a shape no program presents.

**`AC-8` — A MUTATION'S EVIDENCE IS A PAIR. THE MUTATED RUN ALONE IS
UNINTERPRETABLE.** Every mutation result must be reported with its **unmutated
counterpart**. `anchor=1, window=1, eliminators=2` reads equally as *"the join
skipped an element to hide a mismatch"* and as *"the join correctly skipped the
caller-completed prefix"* — **only the unmutated run separates them.** A control
that reports one run is reporting a verdict, not a measurement. This is `AC-2`
specialized to mutations, and it caught a nearly-reported unsound `R3` on
2026-09-14 by firing against its own author's build.

**`AC-7` — a proposed mechanism must be stated with its falsification criterion
before it is measured.** Both refutations that landed today worked because the
proposer named in advance what would refute them. Adopt it as the standard here.

## COORDINATION §1a HAS FIRED ON THIS CHAIN

**Chain: *"what is the mechanism of the `:7720` refusal."* Three stops, all
refuted by measurement, 2026-09-14.** The Steward's tracker is the count of
record and this is stop three. ⇒ **the Architect holds on proposing a fourth
mechanism, and research is called for a prior-art advisory.**

**The research seat is quota-dead** (fleet outage 2026-09-14, ~16 of 23 seats,
recovery near 2026-09-19). **The advisory is therefore QUEUED UNCONSUMED**, and a
participant line reading *"idle, awaiting a named research request"* is last-set
status, not liveness.

**This costs the lane nothing, and the reason is a distinction worth stating
plainly: `§1a`'s hold binds the Architect's RULING, not the ring's
MEASUREMENT.** `MECH` is a measurement over producers and `CTRL` is a control repair;
**neither needs a ruling to proceed, and both are the whole of the ring's next
work.** So there is no five-day block to trade against, and the
rule-unaided-provisionally deviation taken on the HS24/entry-18 arc **does not
apply here** — its premise was a lane blocked on a ruling, and this lane is not.

**What the hold actually prevents** is a fourth mechanism entering the tree as
design authority on the same unexamined premise as the first three. **That is
worth holding for, because it is exactly what the last three hours produced.**

**This is not a judgment on any seat.** Two of the three were refuted on their
proposer's own stated criterion, which is the system working. `§1a` exists for
exactly this shape: a design chain that has absorbed three competent attempts
sharing an unexamined premise. The implementer declined to propose a fourth and
named the pattern in their own record; the Architect named the shared premise in
theirs.

**Distinct from the `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `D2` chain** (*"how does
`D2` obtain discriminating evidence"*), which stands separately at **two**.

> ### THREE DISTINCT CHAINS RUN THROUGH THIS LANE. DO NOT MOVE A FACT BETWEEN
> ### THEM — THE STEWARD DID, AND IT NEARLY REACHED THE OPERATOR.
>
> | chain | count | advisory | status |
> |---|---|---|---|
> | *"what is the mechanism of the `:7720` refusal"* — THIS node | stop **3** | **QUEUED UNCONSUMED** at the quota-dead research seat | moot: measurement settled the question; revisit near 2026-09-19 only if the advisory CONTRADICTS it |
> | *"how does `D2` obtain discriminating evidence"* — [[RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]] | stop **2** | none called | separate |
> | ABI-S6's HS chain, of which **HS18** is a member | HS18 is stop **18**; its `§1a` was the **6th** trigger | **DELIVERED in three parts and CONSUMED** (`evt_2qdk2a3qwa4hs`, `evt_71qmyd36rwygd`, `evt_72yy838768mgk`) | the Architect states it is RULED TWICE — `evt_4ms8rwyhnvgs9`, `evt_1hnd8tta02f59`, plus a corrected Deliverable 2 at `evt_7eqc0hmbanyzm` |
>
> **The dead-seat advisory belongs to the FIRST row only.** The Steward wrote
> that HS18 was *"held at stop 3 behind an advisory queued against a seat that is
> quota-dead"* — **one true fact about one chain, applied to another.** Caught by
> the Architect at `evt_4twx5ypjqe0eh` before it reached Pat, where it would have
> put the block on a seat that had answered nine hours earlier and pointed the
> remedy at a 2026-09-19 reseat that changes nothing.
>
> **This node's own `§1a` section above is where that sentence was refuted** — it
> already said the queued advisory is the MECHANISM chain's, and that `§1a`'s
> hold binds a RULING and not the ring's MEASUREMENT. The Steward authored that
> and then contradicted it in a post the same hour.
>
> **The HS18 row is recorded as the Architect's statement, not as the Steward's
> finding, and the Architect has since REPLACED the caveat that sat here.** The
> earlier version said the three rulings "predate `R3` landing, so whether the
> projection deliverable survives the current tip is unmeasured. Check the three
> events against the tip." **That instruction was wrong, and so is the
> supersession reading offered in its place.** The three events have three
> different dispositions and must not be carried as a set:
>
> - **`evt_1hnd8tta02f59` is VOID, and not by supersession.** Its ruling ("ADD
>   the projection on the deficient route") was issued with three pre-committed
>   outcomes on a precondition read: **(a)** `ITree::Ret` carrier ⇒ the ruling
>   stands; **(b)** already a `Result` carrier ⇒ STOP, the Architect re-rules;
>   **(c)** already-projected contents ⇒ STOP, the Architect re-rules. The read
>   came back **(b)** — `686ffa8ac`'s own subject is *"restore landed D6a route
>   -- precondition read is (b)"*. The ruling self-voided on a branch written
>   before the evidence arrived. **It was then established that (a) was
>   STRUCTURALLY UNREACHABLE** — D6a's guard is
>   `if let Some((_return_index, _return_case)) = return_case`, so the one-binder
>   `::ITree::Ret` case was necessarily compared and missed, and **no value
>   reaching the D6a jump can be a Ret carrier on any route, ever.** The repair
>   would have projected field 0 of a non-Ret carrier for every program. There is
>   nothing to check against the tip, and that is the correct reason.
> - **`evt_4ms8rwyhnvgs9` is DISCHARGED, not withdrawn.** What was withdrawn
>   inside it was the static `Ok` selection, with Condition C revoked; its
>   operative act was to redirect the arc UPSTREAM, which is what produced
>   `evt_1hnd8tta02f59`. A withdrawal *in* a ruling is not a withdrawal *of* it.
> - **`evt_7eqc0hmbanyzm` IS LIVE and nothing here touches it.** Its corrected
>   Deliverable 2 — trap identity must carry its EMISSION COORDINATE, under a
>   DISCRIMINATION acceptance criterion that the originally proposed fix FAILS —
>   is the Steward's own stated input to `RT-TRAP-IDENTITY-EMISSION-COORDINATE`
>   `D1`, the one item in lane 1 with a clean base. **Recording all three as
>   superseded would kill the basis of the lane's only pullable work.**
>
> **The shape, for the fourth time in this lane today: one true fact about one
> member of a set, applied to the set.**

**THE `ROUTE` DECLINE DOES NOT ADVANCE THIS COUNT, AND THE STEWARD RECORDS IT AS
SUCH.** `evt_w1vnqexkhmxx` is hard-stop **one** on *"should `:7720` route"* — a
different design question from the mechanism chain, which closed at three. **No
research trigger fires.** Recorded here because the Steward's tracker is the
count of record and a decline that looks like a fourth refutation would otherwise
be read as one: the mechanism chain's stops were accounts that measurement
killed, whereas `ROUTE` was declined by DERIVATION from a fact `R3` had already
established. **A closure holding is not a stop.**

## A re-measure obligation this node creates elsewhere

**The 11 base reds were never a stable population.** Every measurement over
`abi_s6_mapping_file_backed_native` on 2026-09-14 inherited this refusal as a
mask. [[RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY]] records a venue
finding *"the mapping fixture cannot reach the code at all — zero population"* —
**a compile dying early here produces exactly that reading.** Re-take it after
this node lands, **before** that node's `D1` is answered from it.

## Sizing

**`M`, T1.** `CTRL` is small and independent. `MECH` is a reasoning deliverable over
producers; if it becomes an unbounded search, stop and report — that is `MECH`'s
stated acceptable outcome, not a failure.

## Not this node

- **Not any of the three refuted mechanisms**, re-proposed under a new name.
- **Not a repair of any refusal arm.** All three work.
- **Not `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` or its discharge ledger.** The nine
  *"finished generated-Result proof graph is not closed"* refusals behind this
  one are that mechanism — adjacent, and not evidence about the collision shape.
