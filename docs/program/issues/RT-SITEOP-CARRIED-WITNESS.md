---
id: RT-SITEOP-CARRIED-WITNESS
title: "Site-bound operand reader cannot witness a carried value — a synthesized SiteOperand demands a compile-time Lowered template from the same seat byte-span activation wants carried"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIER-BYTESPAN-OBSERVE]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: Hard stop returned by RT-CARRIER-BYTESPAN-OBSERVE D5, 2026-08-07, candidate 4244d082. The frame's own §1a recut clause fired — the 30 quarantined rows do not discharge from one mechanism. Steward-cut per that clause. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # MERGED 2026-08-17 — `D2` LANDED. PR #2557, exact `a388dc06`.

> **Architect Decision `dec_hmd070mq1h2b` resolved APPROVED (`evt_28demcmgkr8t`);
> Runtime QA exact-SHA approval `evt_2mp5ejpsangd2`.** Base `2e7daa622`, one
> non-merge commit, `+461/-74`, 14 paths, no `spec/` or `conformance/` path.
> Verified by blob identity from the declared merge-base on all 14.
>
> **The mechanism:** `host_effect_site_operand_slots` derives occurrence-bound
> slots from the **existing synthesized-result recipe**, not a second operation
> table, so adding or removing a `SiteOperand` moves the child relation and this
> population together. The `Direct`/`SiteOperandProjection` split keeps the
> exception off the seat-wide `Avail` relation, and the four `Fs*` `Argument(0)`
> seats' `SPECIALIZED_ONLY` row is **unchanged** for the direct consumer.
>
> ### THE 29 ROWS DID NOT SPLIT. §9's HARD STOP DID NOT FIRE.
>
> **13 un-ignored and passing, 16 retained** carrying the *later*
> eliminated-not-callable refusal. That is an **advancing refusal** — the port
> succeeded and those rows reached the next wall — **not two causes.** All 29
> stale pointers now credit `D1a`/`D2` instead of `RT-CARRIER-BYTESPAN-OBSERVE
> D5`.
>
> ### THREE NON-BLOCKING SHOULD-FIXES. Batched by the Architect, not recut.
>
> 1. **All 13 un-ignored tests keep a leading comment block that is now false** —
>    still naming `RT-CARRIER-BYTESPAN-OBSERVE` as owner and claiming the program
>    never executes. The true correction sits **below** `#[test]`, so a reader
>    meets seven false lines first. **This is the same class of defect this node
>    was chartered to correct**, fixed at the three `mod.rs` doc sites and missed
>    at the test sites people actually land on. The still-ignored 16 have a
>    smaller version: their *"Observed signature, exactly"* is no longer observed.
> 2. **The ledger's admissibility gate for the new route is partly
>    caller-attested** — planner-recipe membership is not re-derived from the
>    record, weakening a documented defense-in-depth recompute. Sound at one call
>    site; a second caller would inherit the weaker gate silently. The
>    self-contained fix, if wanted later: carry the site-bound fact on
>    `PlannedEffectSeat` so the ledger re-derives it.
> 3. **`site_operand_argument` masks a non-zero observation outcome** into a null
>    span, while the sibling consumer of the identical word raises a typed
>    three-valued refusal. Believed unreachable and safe, **but the reachability
>    argument is non-local and written nowhere at the projection.**
>
> **Scope note:** declared as "14 Runtime paths"; measured it is **9 `ken-cli`,
> 4 `ken-runtime`, 1 `ken-verify`**. The `ken-verify` path (`scenario.rs`) is
> test-annotation only.
>
> ⇒ **[[NATIVE-HANDLE-CARRIER]] is now fully unblocked** — all five of its
> `depends_on` are `merged`. Its frame was amended the same day (PR #2556) and
> **that amendment is load-bearing**: the input ref is `85dcee25`, not
> `c07e63c2`, and the identity arm has left `core.rs`.

> # `D1b` ANSWERED: PLUMBING AVAILABLE. RECUT IS IN THE FRAME'S §4a. SIZE IS `M`.
>
> **Corrected `D1b` at exact `02f255fc1` (`evt_5bz715jje5p8s`).** The
> discriminator was decisive: `px7m`'s `Some bytes |-> bytes` returns the bound
> bytes **unchanged**, and `write_bytes_then_line` passes them straight into
> `Console.write` — **no literal, equality, decode or length operation.**
> Verified at the tree by the Steward.
>
> ⇒ **A runtime-valued `Lowered` suffices, so the emitted-helper projection is
> not refuted.** `D2` is cut in the frame's **§4a**, and **`size: L` in the
> frontmatter is superseded by `M` there** — this is a port, not a
> representation change.
>
> **PREMISE (2) IS DISCHARGED** (Architect `evt_tmctzqr3858p`), so `D2` may
> dispatch. **It is not endorsed** — non-refutation is not selection, and `D2`
> still owns choosing the mechanism.
>
> **The confirmation rests on a CENSUS, not the fixture:** *"a fixture is
> EXISTENTIAL and the hard stop's condition is UNIVERSAL."* Every
> `ConstructorField::Specialized` read site was censused — **eight, not one** —
> and none demands compile-time content. `:7612` is the one that could have
> refuted it; it matches `Lowered::Closure` and lets every other field fall
> through. **A census stopping at "the sole constructor-field reader" would have
> missed it: there was never one reader.**
>
> **State the result DENOTATIONALLY.** *"Content not required during lowering"*
> is unsound — an interpreter-only control never lowers. The sound and stronger
> form: **the program's meaning never requires the content**, so a lowering
> demanding it asks for more than the semantics requires ⇒ **a missing port, not
> correct semantics.**
>
> **The per-row content-sensitivity obligation I briefly added to `D1a` is
> WITHDRAWN** — the census subsumes it, since the compile-time demand comes from
> the seat and not from what a source does with the bytes.
>
> **The two `D1b` runs below are retained because the correction is the point.**

> # HOW `D1b` GOT THERE — the first run's chain was right, its terminal step was not
>
> **`D1b` reported REPRESENTATIONAL (`evt_2vj52hacadmab`, routed
> `evt_5ka52dfc8z11q`), and the Architect ruled that the report does not
> establish it** (`evt_6f3exyz6we97n`). **The chain the walk traced is correct;
> its terminal step is misclassified.** So the ruled emitted-helper direction is
> **neither refuted nor confirmed**, and the question has moved.
>
> ### WHY THE TERMINAL STEP IS NOT A TEMPLATE READ
>
> `constructor_field_bindings`' arm (`lowering/mod.rs:4838`) is, in full, a clone
> and a wrap:
>
> ```rust
> ConstructorField::Specialized(value) =>
>     LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(value.clone()))
> ```
>
> **It never inspects the value, never reads content, and never requires anything
> to be compile-time known.** It demands a `Lowered` — a strictly weaker demand
> than a template. And **`Lowered` already has runtime-valued inhabitants**
> (`ResponseBytes`, `ResourceToken`, `CapabilityToken`, `BorrowedNativeValue`),
> which `site_operand_witness` maps to `Values(...)`. **If a site operand could
> only ever be a compile-time template, those arms would be dead code.**
>
> ### THE GENUINE TEMPLATE DEMAND IS UPSTREAM — IT IS THE REFUSAL, ONE STEP EARLIER
>
> `site_operand_argument` (`lowering/mod.rs:13574`) calls
> `seats.specialized(...)`, and `ClaimedEffectSeats::specialized`
> (`lowering/mod.rs:13434`) is documented **"Read one seat's compile-time
> template."** That is the only real template demand in the chain, and it sits
> **upstream** of the wall rather than downstream of it.
>
> **Its own doc frames the remedy rather than closing it:** the carried arm *"is
> the arm that would fire if a seat's `Avail` were ever widened **without a
> carried route being written for it**."* ⇒ **The code explicitly contemplates a
> carried route being written for a site operand** — the opposite of a settled
> representational wall. **Verified at the tree by the Steward**, along with the
> clone-and-wrap arm above.
>
> ### THE SHARPENED QUESTION, WHICH NO WALK CAN ANSWER
>
> **Is a runtime-valued `Lowered` (a `ResponseBytes`-shaped span) a legitimate
> site-operand value, or does `Lowered` in this position mean compile-time-KNOWN
> content?**
>
> That is **premise (2)** of the original ruling — the §2g/§2h question the frame
> governs. **Tracing shows what is passed; it cannot show what is permitted**,
> which is why the walk could not reach it.
>
> **The discriminating test is one fixture, not another walk.** `px7m` is the
> right witness and was read for the wrong thing: binding the path bytes is not
> the question — **what the program then DOES with them is.** If it consumes them
> in a way needing their content at compile time (a structural match on a
> literal), representational; if it only passes them on, a port.
>
> ### WHAT THE DELIVERABLE GOT RIGHT — it matters more than the verdict did
>
> **`AC-0` was report-only and the report was honest about what it did**, so this
> is not a defective deliverable — it is one that **stopped a question short of
> its own conclusion.** The refusal to retain any route-around, code, test or
> candidate was correct, and the branch is byte-clean at the base. **This is the
> second time today this ring declined to land a green change that would not have
> fixed the defect.**
>
> **The Architect attributes the wrong turn to its own condition**, not to the
> ring: *"I wrote 'if any downstream reader takes a template, my direction is
> wrong' — which invited a search for a reader, when the operative constraint was
> a producer demanding one. A condition phrased as 'find me a downstream X' gets
> you the first thing that looks like an X."*
>
> **The Steward's own share: I verified the MECHANISM and inherited the
> CLASSIFICATION.** I read the clone-and-wrap arm in the tree and described it
> accurately — then repeated "template reader" from the report without asking
> whether cloning-and-wrapping *is* reading a template. **Verifying that a cited
> coordinate says what it is quoted as saying is not verifying that it means what
> it is claimed to mean.**
>
> ### `active`, UNWORKED, RECUT HELD. `D1b` RE-RUNS AGAINST THE SHARPENED QUESTION.
>
> **Do not dispatch a recut and do not re-size.** The node's shape differs by a
> lot between the two answers — the exact sizing hazard the Architect flagged
> when it held sizing until deliverable 1 was in. `size: L` remains pre-ruling.
>
> **`D1a`'s stale-pointer repair is still owed and is now doubly warranted:** the
> `#[ignore]` reasons at `px7m_hostresult_computational_match.rs:149`/`:178` still
> say *"awaiting Steward recut"* and still credit `RT-CARRIER-BYTESPAN-OBSERVE
> D5`. **The recut happened — it is this node** — so a reader landing on either
> row cannot reach its live owner.

> # DISPATCHED 2026-08-17 — THE FORK IS RULED AND THIS NODE IS ON THE CRITICAL PATH
>
> **Kicked to the runtime ring at `evt_gwrw3dkpt577`, base `origin/main`
> `02f255fc1`, after the full handoff gate** (ring quiescent, all three home
> branches confirmed carrying current `agent/COORDINATION.md`, all three
> compacted and verified per-pane). **`active` from release** so the node is not
> invisible to a frontier audit while it is being worked.
>
> **The dispatch is `D1b` ONLY.** See the start-here block below.
>
> **The one bar was §3 of its frame: an open Architect fork on the mechanism.
> The Architect ruled it at `evt_559gymspqap8w`, and the ruling is pasted
> verbatim into `§3b` of the frame** — read it there, not from the event. Its
> sole `depends_on`, [[RT-CARRIER-BYTESPAN-OBSERVE]], is `merged`.
>
> **The ruled mechanism:** project the carried word to runtime `(pointer, len)`
> through an **emitted helper** and admit that as the site operand's value —
> §2g's sanctioned route, not the banned `Carried -> Lowered` inverse.
>
> ### THIS NODE ACQUIRED A DEPENDENT IT DID NOT HAVE. `blocks` was `[]`.
>
> [[NATIVE-HANDLE-CARRIER]] hard-stopped on **this exact gap**
> (`evt_4eynen6drs79x`, 2026-08-17): its first native refusal is *"seat
> `Argument(0)` of `FsReadFile` needs `BytesPointerLength`, which it cannot
> observe in `CarriedWord`"*. The Architect ruled the fix **does not belong in
> that node** — the component that must change is synthesized error-value
> construction and site-operand provenance. ⇒ **This node is its successor**,
> and through it heads **19 transitive dependents**.
>
> ### START AT `D1b`, AND STOP WHEN IT REPORTS.
>
> **`D1b` answers the one premise the Architect deliberately did not walk:** is
> the synthesized `FileError`'s child read as a **template** anywhere downstream
> (erasure, checked-core body view)? **If it is, the ruled direction is wrong by
> the Architect's own terms** and this returns to the Architect.
>
> **`size: L` is the PRE-RULING provisional and is not evidence of anything.**
> The Architect held sizing until `D1b` reports, *"because a plumbing answer and
> a representational answer are not the same node."* The recut is the Steward's.
>
> **Do not read `L` and plan a long campaign; do not read `ready` and start at
> `D2`.**

## The gap

Each `Fs*` path seat is consumed **twice**:

1. as a **wire span** — which `RT-CARRIER-BYTESPAN-OBSERVE`'s `D4` observer
   satisfies at every measured seat; and
2. as **`SiteOperand(0)`** of the synthesized `FileError`'s
   `Option::Some(<site path>)`, which demands a **compile-time `Lowered`
   template**.

Supplying (2) from a boundary word is the `Carried -> Lowered` inverse that §5
bans. So the same seat cannot be both `EITHER_PHASE` and a site-bound operand.

```rust
// lowering/mod.rs:11354-11362 — the sole template projection
fn site_operand_argument(&self, seat: StaticOriginId, index: u32,
                         seats: &ClaimedEffectSeats<'_>) -> Result<..> {
    let value = seats.specialized(EffectSeatSlot::Argument(index))?.clone();
    //                 ^^^^^^^^^^^ requires the compile-time template
```

`mod.rs:11650-11654` states the consequence in its own voice: a declared
`SiteOperand` whose claimed operand is carried *"refuses at that exact seat,
propagated from `specialized`. It does not reconstruct a template, widen the
carrier, borrow a sibling, or fall back — reconciliation needs a compile-time
witness, and there is none."*

## How it was established

**Two independent routes, which is why it is stated as measured rather than
diagnosed.**

- **Runtime implementer, stepwise at `4244d082`:** baseline refuses at
  `FsWriteFile Argument(0)`; flipping `Argument(0)` moves the refusal to
  `Argument(2)`, proving seat 5 is real; flipping both returns it to
  `Argument(0)`, now from the template projection, past the claim gate. All 26
  lowering refusals across ten files reduce to this one cause, with **zero
  failures of any other kind.**
- **Steward, structurally:** the two source sites above, read directly. Not a
  re-run of the implementer's measurement — a different route to the same
  place.

## What it owns

- **29 of the 30 `#[ignore]` rows** quarantined under
  [[RT-CARRIER-BYTESPAN-OBSERVE]], across 10 files.
- **The four seats left `SPECIALIZED_ONLY`** by that node's `D5`:
  `(FsReadFile, 0)`, `(FsWriteFile, 0)`, `(FsChangeMode, 0)`, `(FsOpen, 0)`.
- **The `D6` activation-gate discharge pass**, moved here because its premise
  is "the activation", and this node is where the activation completes.

## Frame

`docs/program/wp/RT-SITEOP-CARRIED-WITNESS.md`.
