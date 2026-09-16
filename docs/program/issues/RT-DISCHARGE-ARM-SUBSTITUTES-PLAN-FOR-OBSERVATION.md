---
id: RT-DISCHARGE-ARM-SUBSTITUTES-PLAN-FOR-OBSERVATION
title: "Arm 1 of the constructor-identity discharge substitutes a PLANNED identity plus realization for an OBSERVED constructor identity at the returned word. It never consults `body.authorities` at any point -- its whole discharge condition is membership of the returned word in `realized_call_words`, and `independent_contract` is `emission.row.k_ret_identity()`, a planned value off the emission row. The plan is compared against the demand, the word is checked to be initialized, and NOTHING checks what the word actually holds. The two live symptoms are the two exits of this one rule, and ONE OF THEM IS THE GATE WORKING: repairing the emission refusal toward green converts a loud refusal into a silent miscompile. The closure makes the tree REDDER, and that red population is the deliverable."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_20wvshayvhvxj (2026-09-15, the two-exits predicate, and the standing prohibition on repairing cause 2 toward green), completed by evt_4tb8wvsdc93f after the Architect ran its own gating premise check at source and found the mechanism sharper than the ruling had it. The Architect directed the filing and will vote the node. The reduction that produced the evidence is runtime-implementer's: a 139-line program reaching three distinct failures by single-line edits, plus a native-vs-interpreter harness showing identical effect traces with different exit status. Steward-verified at source on 0f71ab5b9267781ae1d91bc654011cad42b926af before filing; the base-tree measurement in the banner below is the Steward's and refutes nothing in the ruling."
---

> # EVERY COORDINATE IN THIS NODE IS ON AN UNLANDED CANDIDATE. NONE OF THIS CODE IS ON `main`.
>
> **Measured by the Steward at filing**, `origin/main` =
> `ea9f4a6151ae82291950e4e7ff185b08d770a960`, candidate =
> `0f71ab5b9267781ae1d91bc654011cad42b926af` (PR #3676, the `ABI-S6 D5b` arc,
> merge-base `1dec48f33cc0697569e9e8765874fe61269332f4`):
>
> | probe in `lowering/units.rs` | `origin/main` | `0f71ab5b9` |
> |---|---|---|
> | file length | 8445 lines | 12623 lines |
> | `independent_contract` | **0 hits** | 10 hits |
> | `realized_call_words` | **0 hits** | 11 hits |
> | `k_ret_identity` | **2 hits** | 3 hits |
> | the arm-1 conditional | **absent** | `:4474` |
>
> ⇒ **The entire mechanism this node is about exists only on the candidate.**
> `:4474` on `main` is unrelated, error-free code, and nothing warns a reader
> that they are in the wrong tree — implausibility of the answer is the only
> detector, and the wrong answer here is perfectly plausible. This is the
> failure mode `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` documents at length for
> its own coordinates; it applies verbatim here.
>
> **AND NAME-PRESENCE IS NOT A RELIABLE WRONG-TREE DETECTOR HERE, WHICH IS THE
> ROW MOST WORTH READING.** Two of the three names resolve to nothing on `main`
> and would catch a misplaced reader. **`k_ret_identity` does not** — it has two
> hits on `main`, in unrelated code. It is also the name that anchors where
> `independent_contract` comes from, so a reader checking the three names gets a
> true negative twice and a **false reassurance on the one that matters most**.
> (Architect, re-measured by the Steward; both readings agree.)
>
> **Lead with the symbol name. The line number never travels without its
> anchor.** Whoever discharges this re-measures at the tip they are on.

## What this is

`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`, the parent, states the invariant in its
own title: make the discharge **the authority's consumption at the consumer**,
so that *"a hand-off with no authority is a missing VALUE and a planner error,
never a missing list entry and **never a silent pass**."*

**Arm 1 is a silent pass on a hand-off with no authority.** This node is not
new scope. It is the parent's stated invariant, unmet by the parent's own
in-flight implementation, cut out so the repair and its measured fallout do not
disappear into an already-`L` node with a held dependent set.

## The mechanism, verbatim at `0f71ab5b9`

`crates/ken-runtime/src/cranelift_backend/lowering/units.rs:4474-4523`:

```rust
let proven = if body.independent_contract == Some(identity) {
    ...
    if realized_call_words.contains(&publication.returned_word) {
        discharge_sources.insert(publication.returned_word);
        true
    } else { false }
} else {
    let cuts = derive_certified_cuts(...)?;
    let proof = prove_forwarded_value(..., &body.authorities, ...)?;
    if proof.valid && proof.grounded { ... true } else { false }
};
```

**The two arms are asymmetric. Arm 2 proves. Arm 1 does not consult
`body.authorities` at any point.** Arm 2 passes the map into
`prove_forwarded_value`; arm 1's entire discharge condition is set membership of
`publication.returned_word` in `realized_call_words`.

**And `independent_contract` is not an observation either.** At `:6962`,
`let independent_contract = emission.row.k_ret_identity()` — a **planned**
identity read off the emission row.

⇒ **Arm 1 compares the plan against the demand, checks the word is initialized,
and never checks what the word actually holds.**

**`None` means the absence of evidence, not a not-applicable marker.** The
reported identity comes from `body.authorities.get(&publication.returned_word)`
(`:4576-4578`, and again in the details tuple built at `:4587`), which is `None`
exactly when **that word has no authority entry at all**. This was the premise
the ruling was gated on; the Architect read it at source and it confirms.

That is why the trapping carrier is exactly the one with no authority. Not a
coincidence — **the arm is only reachable without one.**

### Arm 1 is not a rare branch. The conditional makes it the path, and `WRITE_ALL` shows it trafficked.

**Independent corroboration, from a measurement taken for a different question**
(`RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY`, 2026-09-14, venue finding
2): in the `WRITE_ALL` population, *"both bodies carrying an
`independent_contract` are demanded under exactly one identity, and that
identity **is** their own contract, **so arm 2 never runs**."*

⇒ Wherever the planned identity matches the demand, **arm 1 is the path taken
and the proving arm is not reached at all.**

**KEEP THE TWO HALVES OF THAT APART — they have different warrants and only one
of them is a measurement** (Architect, `evt_4eerxykb24edj`):

- **That arm 1 is taken whenever plan matches demand is a fact about the
  conditional at `:4474`.** It is read off the `if`/`else` and needs no
  measurement at all. As a statement of the *rule*, "the ordinary route for
  every body that carries a contract" is exactly right.
- **What `WRITE_ALL` adds is that the antecedent is actually satisfied in a real
  population**, so the branch is trafficked rather than hypothetical. That is
  its whole contribution. **It establishes no frequency anywhere else**, and
  read as a measurement rather than as the rule, "every body" would be an
  overclaim on a two-body sample.

A claim inherits the scope of the site you checked, not the scope you stated.
The measurement was made to answer whether two callers could collide on one
body; it happens to establish that arm 1 is trafficked, and it was not taken
with this node in view, which is what makes it worth citing.

## The two exits, and why one of them must be left alone

    plan == demand AND word realized   -> arm 1 discharges -> compiles -> traps at
                                          runtime if the actual value is not what
                                          the plan said                    (cause 1)

    plan != demand OR word unrealized  -> arm 1 declines, arm 2's real proof also
                                          fails -> missing -> emission refusal
                                                                           (cause 2)

> ### CAUSE 2 IS THE GATE WORKING. NOBODY REPAIRS IT TOWARD GREEN.
>
> **Architect, `evt_20wvshayvhvxj`, and it is restated here because this is the
> single most tempting action on this board.** Cause 2 is six of seventeen
> failing checks, it has a loud specific failure, and it has an obvious-looking
> repair. Removing the emission failure **without** adding an identity
> requirement converts a loud refusal into a silent miscompile.
>
> A seat that is woken, sees a red check, and reaches for the nearest fix lands
> here. runtime-implementer named this hazard unprompted while holding
> (`evt_6gwyey6pacv43`) and was right to hold.
>
> **2026-09-16: THE PROHIBITION IS UNTOUCHED. THE ATTRIBUTION ABOVE IS NOT.**
> "Six of seventeen" says which entries *instance* cause 2, and the BOUNDARY
> ruling measured that `funcid60`'s arm 2 **never runs in any compile** — so the
> arm-2 entries cannot be instances of an arm-2 verdict. **Do not re-derive the
> count from this block.** The rule "nobody repairs cause 2 toward green" is
> standing law and does not depend on how many entries turn out to be cause 2.

## What the author already saw, and why the defect is one level above it

The comment at `:4480-4488` records the hazard:

> THIS ARM DISCHARGES TOO, AND ITS WORD IS THE SAME ONE THE OTHER ARM ROOTS AT
> ... a ledger keyed per word but populated from one arm promises coverage over
> a domain its population never covers.

That note is about a **ledger**, and commit `c0de78286` acted on it — *"D2 fold:
the discharge ledger covers ARM 1 as well."* Both are correct as far as they go.

⇒ **The defect is one level up from what they guard. It is not that arm 1's
discharges are under-recorded. It is that arm 1 discharges at all without
consulting authority.** Recording a fail-open faithfully leaves it a fail-open.

## D0 — THE POPULATION. Measurement first, and it does not land.

**Apply the closure on the candidate branch, run, and report what newly
refuses. Do not land this increment.**

Arm 1 must consult `body.authorities` for `publication.returned_word` and must
refuse when there is no entry, or when the entry's identity differs from the
demand. **Realization stays necessary and stops being sufficient.**

The deliverable is a **census**: how many, and which, hand-offs carry no
constructor authority. Report it as a population with identities, not as a pass
or fail.

### D0 carries ONE added print, and it rides this rebuild — not a separate one

**Add `body.independent_contract` as a NINTH field of the details tuple at
`:4587-4625`, in the same build.** Architect, `evt_3rp08wzfyqr1q`, overruling
the Steward's *"do not add a print"*: *"not a separate build, not a census."*
The marginal cost is one field, because `D0` rebuilds `units.rs` anyway.

One field settles three questions at once, and each of them otherwise costs a
sampling campaign:

1. **Which arm ran** — for the whole population, structurally, rather than by
   classifying more shard logs one at a time. Case (c) cannot answer this
   (both arms produce it); `independent_contract` can.
2. **Whether `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` has its witness.** That
   node's `D0` is a conjunction whose second half is *"a body that REACHES THE
   PROOF PATH AT ALL"* — arm 2. This field says so directly. That node has been
   stalled for want of exactly this and is not to be given a separate hunt.
3. **Whether the keying defect is arm-correlated**, which is what sizes `D1`.

**Verify no content-addressed arena shift, cheaply**: read one known identity
constant before and after the change. This is a `backend_module(format!(...))`
error path rather than an interned trap message, so a shift is not expected —
but the fleet has already been bitten once by a diagnostic string reindexing an
arena and turning a true 402 into a measured 0. **Expected-no-shift is not
measured-no-shift.**

> **THIS INCREMENT MAKES THE TREE REDDER AND THAT IS THE POINT.** Cause 1's
> population converts from compile-and-trap into refuse-at-emission, so the
> refusal count goes **up**. Architect: *"expect the closure to surface further
> red, and that surfacing is the deliverable."*
>
> ⇒ **Do not write a workspace-green acceptance criterion on D0, and do not
> read its red as a regression.** The standing operator rule that red CI is a
> priority to fix, not to merge past, is about unexplained red. This red is the
> instrument's output, enumerated in advance. It is also the reason D0 does not
> land: a measured population is what tells us whether the upstream repair is
> small or is an arc.

## D1 — the closure proper. SIZED 2026-09-15 FROM D0'S CENSUS.

The arm-1 repair above, landed, **plus** the upstream repair D0's population
shows is needed. D0 has run; the guess this node existed to replace was not
made. **Architect ruling `evt_1xemdvdj9fghw`: for D0's measured population the
repair is PRODUCTION, not re-keying.** Steward-cut on that ruling.

    POPULATION   12 arm-1 hand-offs. Two published words, v1751 and v1386,
                 always at funcid58/funcid59, every table EMPTY.
                 4 demanded identities, 3 programs.
                 OPEN, NOT RECONCILED: this row says 3 PROGRAMS and the gating
                 partition tallies 6 refusals over 2 BINARIES
                 (px8f_write_partition 1, px8f_buffer_native 5). Those may be
                 different units of count or a stale figure; NOBODY HAS
                 MEASURED IT. Flagged rather than reconciled -- do not quietly
                 pick whichever number a later argument needs. Relevant because
                 BODIES (2 vs 4) turns on how many Modules are in play.
    PARTITION    SETTLED, AND IT IS NOT A COUNT OF REPAIRS. Re-run on the
                 GATING graph across BOTH binaries: 12 arm-1 + 5 arm-2, no
                 exceptions. The 7+5 grounding-graph split is SUPERSEDED.
                 ACCEPTED by the Architect (evt_2mw3vjp30yjbn); the partition
                 itself is no longer provisional.
                 BUT 12 IS 2 x 6, NOT TWELVE SITES. There are THREE distinct
                 funcids in all 17 entries: the 12 arm-1 entries are TWO bodies
                 once per refusal, the 5 arm-2 entries are ONE body in five.
                 The multiplicity is over REFUSALS -- compiles and identities --
                 not over places in the program where something is wrong.
                 A REPAIR IS AN EDIT TO THE PROGRAM. 12 and 7 are both counts
                 of OBSERVATIONS, i.e. (body, identity) pairs, and observations
                 are not edits. The repair count is neither.
                 ROOT/DOWNSTREAM IS A PROPERTY OF A (refusal, identity) PAIR,
                 NEVER OF A funcid -- funcid59 is downstream in 4 refusals and
                 a ROOT in 1. Every count here is a count of PAIRS.
                 Architect evt_9bz296v5099j, evt_18mtk5rr061nk, evt_2mw3vjp30yjbn.
    BODIES       TWO PLANNER ROLES. The "2 or 4 across two binaries" framing is
                 RETIRED and the SOURCE-SYMBOL method it prescribed DOES NOT
                 EXIST: units.rs:1562/:1594 mint ken_unit_{ordinal} and
                 ken_continuation_{ordinal} synthetically, with a comment
                 stating outright that the string is not an identity and that
                 nothing resolves a unit by parsing it.
                 THE QUESTION DISSOLVES RATHER THAN RESOLVING. The six compiles
                 are SIX DIFFERENT KEN PROGRAMS, so "the same body across
                 compiles" HAS NO REFERENT -- there is no shared source
                 definition to be the same. The answer could never have been 2.
                 THE WELL-DEFINED OBJECT IS THE PLANNER ROLE, and there are TWO,
                 and they were in the census the whole time:
                 Response(StaticResponseOwnerId(0)) and (1), for funcid58 and
                 funcid59 respectively, identical in both binaries.
                 STILL SETTLED BY MEASUREMENT, NEVER BY NUMBER -- BUT THE
                 SYMBOL READ IS NOT THAT MEASUREMENT, AND IT IS WITHDRAWN.
                 2026-09-16, ran and retracted by the Architect who ordered it
                 (evt_6psqbdtxd5jk6). The symbols came back exactly as minted:
                   funcid58 Response(StaticResponseOwnerId(0))
                            -> ken_static_response_0
                   funcid59 Response(StaticResponseOwnerId(1))
                            -> ken_static_response_1
                   funcid60 Context(ContinuationContextId(0))
                            -> ken_continuation_context_0
                 EVERY SYMBOL IS THE PLANNER ORDINAL WITH A PREFIX, minted FROM
                 the unit field that is already in the census. It therefore
                 CANNOT DISAGREE with that field: it is a restatement, not a
                 second source, and it cannot settle 2-versus-4. The instrument
                 is ENTAILED BY the thing it was offered to confirm, which is
                 the same entailment test this node had just applied elsewhere.
                 A NAME IS A RENDERING OF AN IDENTITY, NOT A SOURCE OF ONE. The
                 unit field -- Response(StaticResponseOwnerId),
                 Context(ContinuationContextId) -- IS the identity the machinery
                 keys on; it was in the census the whole time.
                 THE DEFECT IS RECORDED AGAINST THE INSTRUMENT, NOT AGAINST THE
                 SEAT THAT RAN IT. The Architect ordered the read and withdrew
                 it; the implementer's report absorbed it as its own and that
                 is generous rather than accurate. An inventory that files it
                 under the implementer would mislead a later reader.
                 THE INDEPENDENT REPLACEMENT IS A STRUCTURAL CLIF FINGERPRINT
                 per staged body -- block count, instruction count, authority
                 count, obligation count, published word -- computed from the
                 CLIF, which the planner ordinal does not feed. That is the
                 entailment test PASSED rather than restated.
                 ITS TWO DIRECTIONS ARE NOT SYMMETRIC, and this shape is what
                 survives retelling where prose does not:
                   DIFFERENT fingerprint => DIFFERENT body.       Sound.
                   IDENTICAL fingerprint => consistent with same. WEAKER than it
                       looks: these are COUNTS, and two different bodies can
                       share counts.
                 THE WEAK DIRECTION FAILS SAFE -- understating the evidence
                 concludes weight 1 where a little more was due, and nobody
                 over-acts on an understatement. So it is not strengthened.
                 THE CONTROL ARM IS THE LOAD-BEARING HALF: funcid58/funcid59
                 MUST DIFFER across compiles. If all three come back identical
                 the test is INCONCLUSIVE, not confirmatory -- that is also
                 what (b), one body reported repeatedly, looks like.
                 Architect evt_1vnsrqk27v53c, evt_6psqbdtxd5jk6; implementer
                 evt_7v8vwk1w4bqgt, evt_2ncdwex1gvhmw.
                 THE FOURTH TERM, and it is the Architect's: it offered DISTINCT
                 BODIES as the right KIND of object to replace a count of
                 entries -- and that object is itself UNDER-DEFINED over this
                 population. Key, then graph, then summary object, and then the
                 REPLACEMENT OBJECT HAD NO REFERENT EITHER. This is why SIZE
                 retired its trigger outright instead of re-pointing it: the
                 right thing to count turned out not to exist.
    SIZE         HELD AT M, AND THE TRIGGER IS RETIRED OUTRIGHT, NOT RE-POINTED.
                 The Architect retired its object (evt_2mw3vjp30yjbn): "roots
                 below ~5 -> re-cut to S" cannot fire correctly on a count whose
                 multiplicity is over compiles, and offered DISTINCT BODIES as
                 the right kind of input.
                 A COUNT IS THE WRONG KIND OF INPUT ALTOGETHER, whichever object
                 it ranges over. D1's size is set by TWO OPEN DESIGN FORKS, not
                 by how many sites the edit touches: the FIRST row's PRODUCE vs
                 PROVE vs SEED evaluation, and a SITE pending a declaration-path
                 source read. Neither moves when the population count moves.
                 Re-pointing the trigger at distinct bodies would have fired it
                 IMMEDIATELY -- 2 and 4 are both below 5 -- and re-cut to S on a
                 number the Architect explicitly declined to size on. That is
                 the guess this node exists to avoid, arriving through a repaired
                 trigger instead of a broken one.
                 D1 RE-SIZES WHEN A FORK CLOSES, NOT WHEN A NUMBER LANDS.
                 2026-09-16: THAT RULE HAS NOW FIRED, AND IT IS THE STEWARD'S
                 OWN, SO IT IS DISCHARGED IN THE OPEN RATHER THAN LEFT TO A
                 READER TO CHECK. The SITE fork CLOSED -- the declaration-path
                 read is done (Architect evt_2tzn77b3tcxj4, at source).
                 IT RESOLVED TOWARD MORE WORK, NOT LESS. The site is not the
                 one line it was held at; it is a THREE-STEP SEQUENCE spanning
                 ~90 lines on one emission path, and the step where the OBSERVED
                 identity is discarded was OUTSIDE the site as previously
                 stated. A repair must address the drop, the stamp, and the arm
                 that accepts them.
                 ⇒ SIZE STAYS M, now for a MEASURED reason rather than a held
                 one. One of the two forks closed and moved the size AWAY from
                 S; the other (PRODUCE vs PROVE vs SEED) is still open and is
                 now one printed bit from closing. THE NEXT RE-SIZE IS WHEN THE
                 DISCRIMINATOR BIT IS READ, and its two meanings are fixed in
                 the DISCRIMINATOR row BEFORE the reading, so the size cannot be
                 argued from the bit after the fact.
                 THE TRIGGER FIRED 2026-09-16. BOTH FORKS ARE NOW CLOSED AND
                 THE RE-SIZE IS RULED: SIZE STAYS M. Architect
                 evt_193ekcwm21t7w read the bit; the ruling below is the
                 Steward's, applying meanings fixed BEFORE the reading.
                 THE HONEST TEMPTATION IS S AND IT IS REFUSED. The
                 DISCRIMINATOR row's own pre-registered consequence for the
                 arm that fired says "SEED IS NARROW AND BUILDABLE", and
                 narrow argues S. TAKING IT WOULD BE SIZING ON HOW MANY LINES
                 THE EDIT TOUCHES, which this row already rejected as the
                 wrong KIND of input and rejected while the number in question
                 was smaller and more flattering.
                 WHAT ACTUALLY SETS THE SIZE, now that neither fork can:
                 the repair is THREE COORDINATED EDITS, not one. Stop the drop
                 at :6874/6877, or seed what it drops; make arm 1 at :4489
                 consult what SEED produces instead of realized_call_words
                 membership alone; and disposition the stamp at :6962-6971,
                 which is not automatically correct once the drop is fixed.
                 AND THE DELIVERABLE IS A RED POPULATION, WHICH IS THE
                 EXPENSIVE HALF. This node's thesis is that the closure makes
                 the tree REDDER and that the red population IS the
                 deliverable. Characterizing that population and writing ACs
                 that accept it -- while cause 2 must NOT be repaired toward
                 green -- is the work that does not shrink because the site
                 got located. A located cause makes the edit cheaper; it does
                 not make the acceptance criteria cheaper.
                 ⇒ M, and the trigger is now SPENT rather than re-pointed.
                 There is no third fork, so D1 does not re-size again; it is
                 sized until someone measures the red population and finds it
                 different from what the node predicts.
                 Steward decision; the Architect declined to rule size
                 (evt_18mtk5rr061nk, evt_2mw3vjp30yjbn) and did not rule it
                 here either -- it ruled the fork, which is its call, and the
                 size, which is mine.
    BOUNDARY     RULED, AND IT INVERTS. Every funcid60 pair is GATE-BLOCK'd on
                 funcid58 at the same identity, in every refusal where it
                 appears, and ARM 2 NEVER RAN IN ANY COMPILE. So the 5 arm-2
                 entries are DOWNSTREAM OF D1'S OWN ARM-1 POPULATION.
                 The direction recorded-but-not-ruled is the direction it went.
                 D1 MUST PARTITION ROOTS AND DOWNSTREAM, and the downstream half
                 is now NAMED rather than assumed.
                 Architect evt_4s7beb7qyrhqz, RULED at evt_2mw3vjp30yjbn.
    REPAIR       SEED, RULED 2026-09-16 ON MEASUREMENT. Architect
                 evt_193ekcwm21t7w, on the bit whose meanings were fixed in
                 the DISCRIMINATOR row before it was read.
                 SCOPED TO THREE COORDINATED EDITS, not one: the drop at
                 :6874/:6877, arm 1 at :4489 consulting what SEED produces
                 rather than realized_call_words membership alone, and a
                 disposition of the stamp at :6962-6971.
                 PRODUCE constructor authority remains the SHAPE of what is
                 seeded. NOT re-keying: there is nothing to re-key, the
                 tables are empty.
                 SEED MUST *PRODUCE*, NOT *CONSULT*, AND THAT IS NOW DIRECT
                 RATHER THAN INFERRED FROM THE CENSUS. The fingerprints'
                 third field is the AUTHORITY COUNT, measured in one run:
                   funcid58   829/2611/  0/ 1/v1751    ZERO authorities
                   funcid59   678/2090/  0/ 1/v1386    ZERO authorities
                   funcid60 12023/40524/593/21/v26      593 authorities
                 Arm 1 has nothing to consult because nothing PRODUCED an
                 authority, and funcid60's 593 in the SAME RUN is the control
                 that makes the zeroes a measurement rather than an absence.
                 Architect evt_1c971mqdjffam.
                 THE PROHIBITION IS UNCHANGED AND IT BINDS THE REPAIR: cause 2
                 must NOT be repaired toward green. The closure makes the tree
                 REDDER and that red population IS the deliverable.
                 ### SEED REACHES 12 OF 12, NOT 6. RULED 2026-09-16, Architect
                 evt_2qhwv3ywjaswp. THE SPLIT IS REAL IN THE INPUTS AND
                 DISSOLVES AT THE REPAIR.
                   funcid59's six  callee DOES declare; :6877 discards it.
                                   KDROP: constructed=(Some(id4362), true)
                   funcid58's six  callee declares NOTHING (8 of 8 compiles).
                                   KDROP: constructed=(None, false)
                                   Nothing was ever handed over to restore.
                 BOTH observe (None, true) at the same site in the same
                 compile. THE (true, None) THE CENSUS READS IS A UNION OF TWO
                 CONSTRUCTIONS, now MEASURED rather than argued.
                 THE REPAIR DOES NOT READ THE CONSTRUCTED PAIR AT ALL. It
                 reads the identity the emitted guards PROVE four lines below
                 the drop: require_i64 checks the carrier's ACTUAL tag against
                 k_ret_identity(), and tag_abi_word is INJECTIVE, so A PASSING
                 GUARD IS AN IDENTITY PROOF, not a plan assertion. The plan
                 supplies the HYPOTHESIS; the emitted guard is the
                 OBSERVATION. Same edit on both halves.
                 => OWNERSHIP OF THE ABSENCE AND REACH OF THE REPAIR ARE
                 DIFFERENT RELATIONS. funcid58's six can be genuinely upstream
                 in ORIGIN and still fully covered by SEED at the call site.
                 Do NOT let a (None, false) reading re-scope SEED to six.
                 ### FENCE 1 HAS A SECOND CLAUSE AND IT IS LOAD-BEARING FOR
                 THE AC. Architect evt_2yyn38q8hdtgb; Steward verified the cfg
                 block at source independently. THE PATH IS UNCONDITIONAL --
                 no runtime branch between the drop and the guards, only cfg
                 selection and `?` propagation. THE GUARDED VALUE IS NOT:
                   #[cfg(feature = "px8-ds-test-support")]
                   let ret_abi_word = if body_mutation == Some(VaryRet) {
                       exact_ret_abi_word.checked_add(1)?   DELIBERATELY WRONG
                   } else { exact_ret_abi_word };
                 and the feature is LIVE in the runs that matter -- BUT NOT BY
                 THE ROUTE THIS ROW FIRST CLAIMED. CORRECTED 2026-09-16,
                 Architect evt_5pxyvg17me2bv, re-verified at source by the
                 Steward against origin/main. AS FIRST WRITTEN: "ken-cli/
                 Cargo.toml:28 takes ken-runtime with features =
                 ["px8-ds-test-support"] on a REGULAR dependency edge, so every
                 ken-verify px8f fixture compiles that branch in." BOTH HALVES
                 ARE WRONG. The line number was right and the SECTION was not --
                 :28 is under [dev-dependencies]; the regular edge at :25 takes
                 no features, and ken-verify has only a plain [dependencies]
                 edge with no features and no dev edge at all.
                 => feature ON for ken-cli's OWN test targets, so
                 crates/ken-cli/tests/px8f_buffer_native.rs does compile the
                 VaryRet branch; OFF for crates/ken-verify/tests/
                 px8f_write_partition.rs under a -p-scoped run; ON for it under
                 CI's --workspace (resolver-2 unification, measured at fb99d0fc).
                 THE CORRECTION STRENGTHENS CLAUSE 2 RATHER THAN WEAKENING IT:
                 ret_abi_word is CORRUPTIBLE IN CI AND INCORRUPTIBLE IN THE
                 MANDATORY LOCAL RUN, from the same source, so a tying-form
                 repair would be silently correct under -p and silently wrong
                 under --workspace. Refusing on divergence compares two
                 independently produced values and is immune to that.
                 => expected_ret CAN DIFFER FROM exact_ret_identity
                 .tag_abi_word(). Registering exact_ret_identity would record
                 an authority THE GUARD NEVER PROVED, into exactly the
                 population this node measures. The program still traps, so
                 nothing unsound executes; THE DAMAGE IS TO THE RECORD, AND
                 THE RECORD IS THE DELIVERABLE.
                 THE AC, IN THE REFUSING FORM RATHER THAN THE TYING FORM:
                   At the registration site, REQUIRE exact_ret_identity
                   .tag_abi_word()? == ret_abi_word, and REFUSE ON DIVERGENCE
                   RATHER THAN RECORDING.
                 NOT "derive the registered identity from ret_abi_word" --
                 that would dutifully record the mutated identity and MAKE THE
                 MUTATION INVISIBLE. Refusing fails a VaryRet compile loudly at
                 the exact seam the mutation targets. A MUTATION THAT PRODUCES
                 A CLEAN RECORD IS A MUTATION THAT PROVED NOTHING.
                 This is fence 1's general shape, not an exception to it:
                 dominance by both guards is STRUCTURAL; this clause makes it
                 SEMANTIC -- the thing registered must be the thing GUARDED,
                 not merely downstream of a guard.
                 DEPENDENCY, RULED IN PART 2026-09-16. Defect ownership is
                 SETTLED AND IT IS D1'S -- the arm that fired routes to the
                 call site, so depends_on can NO LONGER be justified on "this
                 node's premise is the parent's unmet invariant", and the
                 schema WARN saying a team would find that premise false is
                 CORRECT. The edge is NOT deleted on that ground: defect
                 ownership and BUILD ORDER are different relations, and
                 whether the constructor-authority production path SEED calls
                 is the parent's deliverable or D1's own is the open question.
                 If the parent builds it, the edge is real AS A BUILD
                 DEPENDENCY and must say so in those terms; if D1 builds its
                 own, the edge goes.
                 ### CLOSED 2026-09-16. BOTH HALVES OF THIS ROW'S OWN
                 PRECONDITION ARE CLOSED. depends_on IS NOW [].
                 BUILD ORDER: NEITHER NODE BUILDS THE PRODUCER -- IT PRE-EXISTS
                 BOTH. A THIRD ARM THIS ROW NEVER ENUMERATED.
                   fn register_generated_constructor_authority
                       lowering/mod.rs:4080 on candidate 0f71ab5b9
                   live callers   calls.rs 1, core.rs 1 -- and NEITHER covers a
                       word produced by a CALL, which is why both arm-1 bodies
                       carry authorities = 0.
                   the PARENT describes it at its OWN :124 as machinery it
                       MEASURED at 5d977ac79, not as a deliverable it builds.
                       Its deliverable is what discharge MEANS (consumption at
                       the consumer), not creating the producer.
                 Three INDEPENDENT source reads: Architect evt_332b3rwem87jy,
                 runtime-implementer evt_3ahwb990vmg07, Steward at source.
                 OWNERSHIP OF THE REPAIR: SETTLED BY IT REACHING 12 OF 12 (see
                 the REPAIR row above), not by the census. The census reads the
                 bit AFTER :6874 mutates it and therefore cannot speak to
                 construction at all.
                 OWNERSHIP OF THE ABSENCE IS DELIBERATELY LEFT OPEN, exactly
                 where the ruling left it. funcid58's six may well be upstream
                 IN ORIGIN; that does not bear on the edge, because the edge
                 encodes build order and the repair's reach, not provenance.
                 Do NOT read "ownership settled" as covering the upstream
                 absence -- the :6874 node (fence 3) is still to be cut and
                 must not inherit a closure it was never given.
                 ### THE REFUTATION CONDITION THIS TURNED ON WAS DEFECTIVE, AND
                 THE CORRECTED TABLE IS RECORDED HERE RATHER THAN THE ONE
                 FIRST OFFERED. Architect withdrew it at evt_5wspvc2mf3kwg
                 BEFORE it gated this commit. As offered:
                   v1751 PRINTS -> ruling holds;  ABSENT -> ruling refuted.
                 THE SECOND ARM IS WRONG. KDROP prints from INSIDE the
                 `if let Some(obligation) = ...find(...)` body, so an absent
                 line means THE FIND DID NOT MATCH -- an instrument failure --
                 and the census already entails that the body ran on that
                 obligation. CORRECTED:
                   v1751 PRESENT  instrument sound; fence discharged.
                   v1751 ABSENT   KDROP CONTRADICTS THE CENSUS. One of the two
                                  is wrong and the RULING IS NOT WHAT THAT
                                  DECIDES. The v1386 reading would go with it,
                                  drawn from the same lookup. Fix the probe.
                 => THE v1751 LINE IS A CONTROL ON KDROP, NOT A TEST OF THE
                 RULING. What tests the ruling is CONTENT: whether the guards
                 are on that path and derive expected_ret from
                 k_ret_identity(). Steward verified that at source
                 (units.rs:6879-6897 on 0f71ab5b9): both guards sit OUTSIDE
                 the find block, on the straight-line path, so they run whether
                 or not the find matched.
                 THE RUN AGREEING IS NOT WHAT MAKES A CONDITION SOUND. The
                 favourable branch fired, so the defect cost nothing -- which
                 is the reason to record it rather than the reason not to.
                 WHAT REMOVING THE EDGE COSTS, STATED RATHER THAN ELIDED. The
                 schema WARN it raised was the only frontmatter-level brake on
                 pulling this node, and it goes with the edge. That brake was
                 MIS-KEYED: it releases when the PARENT lands, an event
                 unrelated to the real constraint, which is that every
                 coordinate here is on the UNLANDED candidate 0f71ab5b9. A
                 guard keyed on a ref other than the one its measurement is
                 taken at cannot fire correctly, and keeping it because it
                 happens to sit in the braking position is worse than removing
                 it and saying so.
                 THE REAL CONSTRAINT STAYS IN THE BANNER, which has no schema
                 home: the schema carries only depends_on and blocks, both over
                 issue ids, so "written against an unlanded base" is not
                 expressible. Its deletion obligation is filed in
                 RT-D5B-POSTCALL-REFUSAL-MECHANISM's "Obligations this node
                 creates elsewhere".
    SITE         CONFIRMED AND WIDENED 2026-09-16. It is NOT one line. It is a
                 THREE-STEP SEQUENCE on ONE emission path, spanning ~90 lines,
                 and :6971 is the SECOND step, not the site:
                   :6868-6878  DROPS the observed identity (obligation.identity
                               = None) and sets realization_required = true
                   :6879       let exact_ret_identity = k_ret_identity()  PLANNED
                   :6962-6971  STAMPS that PLANNED identity as
                               independent_contract
                   :4489       arm 1 discharges on realized_call_words
                               membership ALONE
                 THAT SEQUENCE IS THE SUBSTITUTION THIS NODE'S TITLE NAMES: the
                 plan is compared against the demand, the word is checked to be
                 initialized, and NOTHING CHECKS WHAT THE WORD HOLDS. The drop,
                 the stamp, and the arm that accepts them.
                 The first step -- where the OBSERVED identity is discarded --
                 was outside the site as previously stated. v1751/v1386 remain
                 where the symptom surfaces, not the site.
                 Architect evt_2tzn77b3tcxj4, read at source on b0a7c2945.
    DISCRIMINATOR ONE BIT PER OBLIGATION DECIDES THE PRODUCE/PROVE/SEED FORK,
                 AND BOTH MEANINGS ARE FIXED HERE BEFORE THE BIT IS READ.
                   (realization_required = FALSE, identity = None)
                       constructed that way => THE CALLEE DECLARED NOTHING
                       => UPSTREAM. RT-CONSTRUCTOR-AUTHORITY-DISCHARGE owns it.
                       SEED IS WRONG.
                   (realization_required = TRUE, identity = None)
                       NOT CONSTRUCTIBLE => produced by the drop at :6877
                       => a CALL-SITE defect. SEED IS NARROW AND BUILDABLE.
                 THE INVARIANT THAT MAKES THE SECOND PAIR IMPOSSIBLE AT
                 CONSTRUCTION: calls.rs:2479-2483 sets identity from
                 target.result_contract.map(...) and realization_required from
                 target.result_contract.is_some() -- BOTH functions of the SAME
                 Option, so identity.is_some() == realization_required always.
                 (units.rs:12050 also constructs one; it is under #[cfg(test)]
                 at :11886.)
                 EXACTLY TWO MUTATIONS OF .identity EXIST IN THE WHOLE BACKEND,
                 grepped over the entire cranelift_backend tree, not sampled:
                   core.rs:9637   -> Some(actual_result_identity()). UPGRADES
                                     declared to ACTUAL OBSERVED after checking
                                     agreement at :9628-9634 and REFUSING on
                                     disagreement. The mechanism working.
                   units.rs:6877  -> None. DROPS it. The response-owner K call.
                 So ONLY :6877 can produce (true, None).
                 THE INVARIANT FIXES WHAT THE BIT MEANS. IT DOES NOT FIX WHICH
                 VALUE IT HAS -- that is the measurement, and asserting the
                 value from invariant-plus-plausible-path is the entailment
                 mistake. NOT ASSERTED: that funcid59's obligation came
                 from :6877.
                 ### READ 2026-09-16. TRUE / None. ARM (b). FORK CLOSED.
                 Architect evt_193ekcwm21t7w, against meanings unchanged
                 since they were fixed above.
                   TRUE / None   at 12 arm-1 entries across BOTH binaries, and
                                 at NONE of the other 105 obligations.
                                 (First read, px8f_buffer_native alone: 10, and
                                 none of that binary's other 84.)
                   TRUE / Some   40 combined (32 buffer_native + 8
                                 write_partition). The DISCRIMINATION:
                                 realization and identity travel together
                                 everywhere else, so the bit is not a constant.
                   FALSE / Some  EMPTY, AND IT IS A MEASURED ZERO. A fourth
                                 cell the construction invariant predicts
                                 must be empty and no site can produce. It
                                 is empty. Confirmed 2026-09-16 against a
                                 pre-registration; Architect
                                 evt_1c971mqdjffam, evt_4m71wxrbs704c.
                 THAT CELL IS THE ONE EMPIRICAL CHECK ON THE MUTATION-SITE
                 CENSUS. THE CENSUS is what upgrades the bit from a plausible
                 site to an ENTAILED one. The census is a GREP and greps have
                 blind spots, so a measured (false, Some) = 0 is an
                 independent test that the grep missed no writer OF THE SHAPE
                 THE INVARIANT FORBIDS.
                 AND THAT IS THE LIMIT OF WHAT THE CELL CAN DO -- IT IS SILENT
                 ON A MISSED (true, None) WRITER, WHICH IS THE SHAPE THE SITE
                 CLAIM RESTS ON. The entailment is "units.rs:6874/6877 is the
                 ONLY producer of (true, None)", so a grep that missed a
                 second (true, None) writer would break it, and this cell
                 would not notice. ⇒ THE CENSUS, NOT THIS CELL, IS WHAT
                 ENTAILS THE SITE. Do not cite the cell to argue the census
                 is verified.
                 THIS IS THE parent == origin/main SHAPE, ONE ARTIFACT LATER:
                 a control at its strongest exactly where a failure would be
                 harmless, and silent exactly where one would be fatal.
                 Architect evt_227hnhfawfeqs.
                 AND THE ZERO IS ONLY WORTH THAT BECAUSE THE PARSE IS SHOWN
                 COMPLETE. A zero from a parser is worth nothing until you
                 show the parser saw everything, so the parsed totals are
                 cross-footed against the census's OWN obligation counts:
                   px8f_buffer_native    52+32+10 = 94
                                         4x(21+1+1) + 1x(1+1) = 94
                   px8f_write_partition  13+8+2   = 23
                                         1x(21+1+1)           = 23
                   combined              94+23    = 117
                                         65+0+40+12           = 117
                   per-cell  52+13=65   32+8=40   10+2=12     all exact
                 The per-refusal 21+1+1 is the OBLIGATION COLUMN OF THE
                 FINGERPRINTS -- funcid60 21, funcid58 1, funcid59 1 -- so the
                 predicted total is derived from a DIFFERENT measurement than
                 the parsed one. A genuine cross-foot, not a restatement, and
                 it is what makes the cell a CONTROL rather than a parser
                 report.
                 THE PRE-REGISTRATION RESOLVED TO ITS FIRST BRANCH. Recorded
                 because a pre-registration that is only cited when it fires
                 the inconvenient way is not one: the alternative branch was
                 STRIKE, not soften, with the census left standing on the grep
                 alone.
                 ⇒ THE CALL SITE DROPS A CONTRACT IT WAS HANDED. SEED.
                 THE QUANTIFIER IS 12 OF 12 -- TOTAL OVER THE CENSUS
                 POPULATION, READ 2026-09-16. Both binaries, six refusals,
                 against a 40-way (true/Some) and a 65-way (false/None)
                 control. It stood at "10 of 12" for one candidate, because
                 px8f_buffer_native is one binary and the remaining 2 were
                 unread; 10-for-10 could not plausibly have been overturned,
                 but "cannot plausibly be overturned" is not "measured" and
                 the reading was cheap. It was taken.
                 2 VERSUS 4 IS SETTLED AT 2. funcid58 and funcid59 are
                 byte-identical across ALL EIGHT compiles in both binaries:
                 829/2611/0/1/v1751 and 678/2090/0/1/v1386.
                 THE WEAK DIRECTION IS STILL WEAK AND IS USED WHERE IT FAILS
                 SAFE. Identical fingerprints are counts and counts can
                 coincide; the conclusion drawn is weight 2, not weight 12.
                 What lifts it well past bare coincidence is that BLOCK AND
                 INSTRUCTION COUNTS COME FROM FINALIZED CLIF and could have
                 disagreed with the planner identity across eight independent
                 opportunities, and did not. A measure that COULD have
                 discriminated and did not is corroboration. It is still not
                 identity, and the conclusion does not need it to be.
                 THE FENCE DIRECTLY ABOVE IS NOW DISCHARGED, AND BY A CENSUS
                 RATHER THAN BY THE BIT. The step from value to site needs
                 the mutation-site census to be COMPLETE, not merely to
                 contain a plausible path. On b0a7c2945, every write to
                 either field in the whole lowering/ tree:
                   .identity              core.rs:9637  = Some(actual)
                                          units.rs:6877 = None
                   .realization_required  calls.rs:2483 = contract.is_some()
                                          core.rs:9638  = true
                                          units.rs:6874 = true
                                          units.rs:12052 = false (cfg(test))
                 calls.rs:2479-2483 takes both from the SAME Option =>
                 (Some,true) or (None,false). core.rs:9637-9638 sets both =>
                 (Some,true). units.rs:6874/6877 sets required THEN drops
                 identity => (None,true).
                 ⇒ units.rs:6874/6877 is the ONLY producer of (true, None) in
                 the tree, so observing it ENTAILS that site ran on that
                 obligation. NOW ASSERTED, and only now: funcid59's
                 obligation came from :6877, and so did funcid58's.
                 THE callee FIELD IS WHAT MAKES IT ONE SENTENCE:
                   funcid58  [(v1751, None, true, Some(funcid62))]
                   funcid59  [(v1386, None, true, Some(funcid60))]
                 v1751 and v1386 are those bodies' OWN published words. Each
                 arm-1 body publishes the exact word it received from a call
                 whose identity was dropped at :6877. One obligation each, no
                 others. The defect stated with no inference step -- and only
                 sayable because the implementer added a field nobody asked
                 for.
    INSTRUMENT   THE REFUSAL'S OWN DETAILS TUPLE CANNOT REPRESENT THE DECIDING
                 EVIDENCE. At :4616-4621 it projects
                 body.call_obligations.iter().filter(|o| o.identity ==
                 Some(*identity)), so EVERY obligation with identity = None is
                 INVISIBLE to the instrument that reports the refusal -- and
                 those are exactly the ones the fork turns on.
                 This is NOT a gap in what was measured. It is a gap in what the
                 instrument CAN REPRESENT. Every number this node has argued
                 over was computed by a projection that structurally excluded
                 the evidence that decides it.
                 THE FIX IS ONE FIELD ON AN EXISTING PRINT, no new harness: in
                 the :4587 details tuple, add a projection over
                 body.call_obligations that does NOT filter on identity,
                 emitting (result_word, identity, realization_required) per
                 obligation -- the shape missing_realizations already uses at
                 the realization-graph error. Report the realization_required
                 bit per (refusal, identity) pair for every identity = None
                 obligation, specifically funcid59's obligation on funcid60,
                 which is the row the partition turned on.
                 BUILT 2026-09-16, AND BUILT WIDER THAN SPECIFIED. The tuple
                 as prescribed above was (result_word, identity,
                 realization_required); as built it carries a fourth element,
                 CALLEE. Architect evt_6psqbdtxd5jk6, accepting the addition.
                 CALLEE IS WHAT TURNS A SHAPE INTO A ROW. Without it the print
                 says "an obligation lost its identity"; with it, "FUNCID59's
                 obligation ON FUNCID60 lost its identity" -- the row the
                 partition turns on, named without a second lookup.
                 IT IS ALSO THE RIGHT KIND OF ADDITION, which is worth naming
                 beside the withdrawn symbol read in BODIES: the callee is a
                 FUNCID, an identity the system already carries, not a string
                 minted to stand for one. Same test, opposite verdict.
                 IT IS NESTED AT FIELD 8, NOT ADDED AS A THIRTEENTH. Rust's
                 Debug derive tops out at 12 elements, so field 8 is now a
                 PAIR: the old identity-filtered list, plus every obligation
                 UNFILTERED. Nesting rather than dropping a field is correct --
                 and it puts the unfiltered list directly beside the filter
                 (identity == Some(*identity)) that excludes the evidence, so
                 the gap this row describes is visible in one line of output.
    FIRST        D1 EVALUATES **PRODUCE AGAINST PROVE** BEFORE DESIGNING THE
                 REPAIR. "Production, not re-keying" was a TWO-way fork and
                 there is a third arm the code already implements; see below.
                 This does not resize D1 -- it stops D1 building the narrow
                 fix by default.
                 THE SEED ARM HAS ONE MEASUREMENT AGAINST IT AND IT IS NOT THE
                 OBVIOUS ONE. The seed-free control (below) proved funcid60
                 with an EMPTY certificate set, so the seed is NOT LOAD-BEARING
                 for the grounding. It is also NOT INERT: 6664 blocks / 7 cuts
                 with it against 7762 / 6 without. Those are different claims
                 and only the first bears on this fork.
    NOT-EXCLUDED the 5 arm-2 refusals. THIS EXCLUSION IS REFUSED ON MEASUREMENT
                 (Architect evt_2mw3vjp30yjbn). It read them as "the proving arm
                 declining, cause 2 is the gate working" -- but that gate NEVER
                 EXECUTED in any compile, so the ground credited for excluding
                 them was never observed to operate. They are downstream of D1's
                 own arm-1 population; see BOUNDARY.
                 THE PROHIBITION ON REPAIRING CAUSE 2 TOWARD GREEN IS UNTOUCHED.
                 What falls is the claim that these entries ARE cause 2.
                 THEIR MECHANISM IS NOW NAMED, and it is recorded as an
                 observation rather than cut as a node: :4467 over-demands, so
                 these refusals are GATING ARTIFACTS rather than verdicts. See
                 the recorded observation below, and note its THIRD FENCE,
                 which binds this row: it explains the mechanism of the 5 and
                 DOES NOT RE-OPEN THE COUNT. D1 stays at 12 + 5.
    EXCLUDED     the rt_parity (c1) entries -- PROVISIONALLY FOLDED, below. They
                 enter neither D1's scope nor its sizing.
    HOLE         abi_s6_mapping_file_backed_native is NOT MEASURED and is NOT
                 KNOWN TO CONTRIBUTE ZERO. It aborts (SIGABRT, stack overflow)
                 before producing a hand-off. Zero executions and never-ran are
                 two facts and one number; do not let this population enter D1
                 as a zero.

### D1 DISPATCH — BUILD THE SEED REPAIR. Steward, 2026-09-16.

**Both design forks are closed and the ring builds.** SITE closed toward more
work (three steps, not one line); PRODUCE-vs-PROVE-vs-SEED closed on the
measured bit at `evt_193ekcwm21t7w`. Owner runtime, size M, tier T1.

### THE BASE. CUT THE D1 CANDIDATE FROM `0f71ab5b9`, THE HEAD OF PR #3676.

**Steward ruling 2026-09-16, on runtime-implementer's escalation
`evt_6jz4a7nx5nwks`. This block previously named the tip to re-measure at and
never named the base to build from; that omission is the Steward's and this is
the repair.**

⇒ **`depends_on: []` is true as an ISSUE relation and FALSE as a BUILD
relation. Do not read it as "buildable on `main`."** The mechanism this node
repairs does not exist on `main` at all. Measured by the implementer over nine
symbols, and the sweep is the informative part: `realized_call_words` and
`independent_contract` have hits on `main` **only under `docs/`** — this node
and its siblings — and **zero under `crates/`**. The site exists on `main` as
prose.

**And the banner's hazard fired exactly as written.** `k_ret_identity` *does*
resolve on `main`, in unrelated code, so a reader using name-presence as a
wrong-tree detector gets a true negative on two names and a false reassurance on
the third. That is why the base is named here as a value rather than left to be
inferred.

    0f71ab5b9  wp/ABI-S6-d5b-file-backed        HEAD OF PR #3676 (OPEN, -> main)
               36 ahead of the merge-base, 46 behind main, tip 2026-09-15
    ff7638ff6  wp/RT-D5B-live-tip-ci-check      24 ahead, 82 behind, 2026-09-14
    0d94d58b6  wp/ABI-S6-d5b-file-backed-live   21 ahead, 82 behind, 2026-09-14
    b0a7c2945  (local) wp/ABI-S6-d5b-file-backed = 0f71ab5b9 + ONE UNPUSHED commit

**Why `0f71ab5b9`, and it is NOT A THREE-WAY CHOICE — IT IS TWO LINES.**
`0d94d58b6` is an **ancestor of** `ff7638ff6` (zero commits in the former not in
the latter, three the other way; Steward-measured, and the Architect reached it
independently). So the field is `0f71ab5b9` against one older line, not three
rival trees.

`0f71ab5b9` is **the open PR's head**, and this node was cut out of that arc
rather than alongside it, so the repair lands with the arc. It is also the
newest, the least behind `main` by nearly half, and the tree every AC coordinate
in this block cites — including D1-1's own control.

> **USE AN ABSENCE-BASED WRONG-TREE DETECTOR, NEVER A PRESENCE-BASED ONE.**
> This is the reusable half of the escalation that produced this ruling.
> `k_ret_identity` is present on `main` in unrelated code, so checking for it
> returns *"right tree"* on the wrong tree. A name with **zero** hits on `main`
> and nonzero on the candidate cannot do that:
>
>     git grep -c register_generated_constructor_authority -- crates   main 0, cand 3
>     git grep -c pending_call_result_obligations          -- crates   main 0, cand 7
>
> A presence check fails open; an absence check fails closed. **Pick the name
> that is missing, not the name that is there.**

**Why NOT `b0a7c2945`, even though every measurement in this arc was taken
there.** It is `0f71ab5b9` plus one local commit that **was never pushed**, and
all worktrees in this repository share one object database — so it reads
perfectly from any seat while `git branch -r --contains` returns nothing, and a
candidate cut from it would carry an ancestor chain absent from the remote.
That one commit touches px8f trap reporting only. **If any AC's control turns
out to depend on it, say so and escalate rather than cherry-picking it in
silently.**

**The 46-commit gap behind `main` is NOT this node's to close.** Do not rebase
the arc as part of D1. PR #3676's ordering is an open operator decision, and
D1-5 already requires cause 2 to stay red; a rebase here would entangle a
localized three-edit repair with that decision.

**TREAT EVERY ANCHOR IN THIS NODE AS PERISHABLE.** If a fixed input below turns
out false against the code you are on, say so and escalate — do not quietly
build around it. The banner at the head of this file is the sharpest case: every
coordinate here is on the unlanded candidate, and `k_ret_identity` resolves on
`main` in unrelated code, so name-presence is a false reassurance rather than a
wrong-tree detector. Lead with the symbol name; re-measure line numbers at the
tip you are on. `tag_abi_word` has the same hazard one level down — it has a
second hit at `lowering/core/tests/control.rs`, inside a string literal in a
test, not a second definition.

**The best guess, stated so a reviewer can attack it directly:** the three
coordinated edits named in the REPAIR and SITE rows are the whole repair, and
the arm-1 consultation change at `realized_call_words.contains(&publication
.returned_word)` is the one with design content. Build it. **One honest try plus
a handback** — if the repair cannot be made to work as scoped, report the
structure you found from inside the attempt; that is a better measurement than a
standalone probe and it is an accepted outcome.

**THE HARD STOP THAT IS A RESULT, NOT A FAILURE.** If the repaired arm 1 cannot
be made to refuse by permitted means, say so and stop. `RT-CONSTRUCTOR-AUTHORITY
-DISCHARGE:332` governs — *"if either is unavailable in principle at some site,
THAT IS THE FINDING."* Do not reach for a synthesized identity to close it; that
prohibition extends to controls (Architect `evt_5gws0pnssfqch`).

#### The deliverable: three coordinated edits

1. **The drop.** Stop the discard at `obligation.identity = None` /
   `realization_required = true` in the response-owner K-call block, or seed what
   it drops. (At `b0a7c2945`, around `units.rs:6874`/`:6877`.)
2. **Arm 1.** Make it consult what SEED produces rather than
   `realized_call_words` membership alone. (Around `units.rs:4489`.)
3. **The stamp.** Disposition the `independent_contract` stamp explicitly. It is
   **not automatically correct once the drop is fixed.** (Around
   `units.rs:6962-6971`.)

#### Acceptance criteria for D1

These are obligations, not hazards. Each names its control.

**D1-1. STRUCTURAL DOMINANCE (fence 1).** Registration happens only at a point
dominated by **BOTH** `require_i64` guards — the tag guard and the arity guard.
Soundness depends on the trap having fired for any continuing execution.
**Control:** name the guard pair and show the registration site sits below both
on the straight-line path, with no intervening branch or early return. A
registration reachable without both guards having executed fails this. At
`0f71ab5b9` both guards sit outside the `find` block precisely so that they run
whether or not the lookup matched; preserve that property, do not rely on the
line numbers.

**D1-2. SEMANTIC DOMINANCE — THE REFUSING FORM (fence 1, second clause).** At
the registration site, require `exact_ret_identity.tag_abi_word()? ==
ret_abi_word` and **REFUSE ON DIVERGENCE RATHER THAN RECORDING.**

**NOT the tying form.** Deriving the registered identity *from* `ret_abi_word`
would dutifully record the mutated identity and make the mutation invisible. The
program still traps either way, so nothing unsound executes — **the damage is to
the record, and the record is the deliverable.**

**THE FEATURE PROFILE IS THE STRONGER ARGUMENT FOR THE REFUSING FORM, AND IT IS
NOT THE ONE FIRST GIVEN.** An earlier statement of this fence — in this node's
REPAIR row, and in the Architect's ruling it came from — said
`ken-cli/Cargo.toml` puts `px8-ds-test-support` on a **regular** dependency edge
and that the `VaryRet` branch therefore compiles into every `ken-verify` px8f
fixture. **Both halves are wrong; the line number was right and the section was
not.** Corrected by the Architect at `evt_5pxyvg17me2bv`, re-verified at source
by the Steward against `origin/main`:

    ken-cli     [dependencies]      ken-runtime = { path = ... }   no features
                [dev-dependencies]  ken-runtime = { path = ...,
                                      features = ["px8-ds-test-support"] }
    ken-verify  [dependencies]      ken-runtime = { path = ... }   no features,
                                                                   no dev edge

⇒ The feature is **ON** for `ken-cli`'s own test targets, so
`crates/ken-cli/tests/px8f_buffer_native.rs` does compile the `VaryRet` branch.
It is **OFF** for `crates/ken-verify/tests/px8f_write_partition.rs` under a
`-p`-scoped run, and **ON** for it under CI's `--workspace` (resolver-2 feature
unification; the fleet measured this exact feature on this exact edge at
`fb99d0fc`).

⇒ **`ret_abi_word` is corruptible in CI and incorruptible in the mandatory local
run, from the same source.** A repair that *derived* the recorded identity from
`ret_abi_word` would be **silently correct under `-p` and silently wrong under
`--workspace`.** The REFUSE-on-divergence form is immune, because it compares
two independently produced values instead of trusting one.

**THE SWEEP, WITH ITS SCOPE AND ITS RESULT, BECAUSE A CLEAN SWEEP AND A SKIPPED
ONE LOOK THE SAME.** Swept `docs/` and `agent/` for the refuted phrase: **one
hit, and it is this node's own corrected quotation of itself above.** No other
artifact asserts that `ken-verify` carries the feature. **And the tree carries a
guard for the opposite claim** — `RT-MATCH-RECURSOR-CONSUMERS`'s `AC-9a` pins
that no `[workspace] members` entry enables `px8-ds-test-support` **on a normal
edge** to `ken-runtime`, with a mutation of the `ken-verify` normal edge as its
positive control. So the corrected reading is not merely re-read from
`Cargo.toml`; a live pin elsewhere in the tree exists to red if anyone makes the
refuted statement true.

**Control:** a `VaryRet` compile must fail loudly at this exact seam. If it
produces a clean record, the AC is **not** met. A mutation that produces a clean
record is a mutation that proved nothing.

D1-1 is structural, D1-2 is semantic, and D1-2 is not an exception to fence 1 —
it is fence 1's general shape: **the thing registered must be the thing GUARDED,
not merely something downstream of a guard.**

**D1-3. IDENTITY IS ARTIFACT-LOCAL (fence 2).** `pack_identity`'s own doc, at
`planning/static_transition/semantic_ir.rs`, verbatim: *"Artifact-local only.
This number is stable within one artifact's plane and carries no cross-artifact
meaning — spans depend on that artifact's own interning order. Do not persist
it, compare it across artifacts, or read it as a portable name."* The authority
map is function-local, so the repair satisfies this by construction.
**Control:** the artifact says so explicitly. A property that holds silently is
one nobody can check later.

**D1-4. DO NOT FOLD IN THE `:6874` QUESTION (fence 3).** Why the drop forces
`realization_required = true` on a callee that declared nothing is a **separate
node, not yet cut.** **Control:** the diff changes nothing about that mechanism,
and the report does not claim the upstream absence is closed. Ownership of the
absence is deliberately left open; the `:6874` node must not inherit a closure
it was never given.

**D1-5. THE RED POPULATION IS THE DELIVERABLE.** Cause 2's six checks are NOT
made green, and the diff is checked for it: no change removes an emission
refusal without adding an identity requirement. **Do not write a
workspace-green acceptance criterion on this increment and do not read its red
as a regression** — the closure makes the tree redder, and that surfacing is the
output. This restates arc-level AC-3 at the deliverable because a constraint
that lives only in a section two screens away is a constraint the build does not
fail.

**D1-6. THE RESIDUAL — DO NOT QUANTIFY OVER COMPILES, AND DO NOT POOL THE
DENOMINATOR.** **"`v1386` is always handed a declaration" is NOT what was
measured**, and no AC or report may assume it. Nor is the pooled eight-compile
figure the right denominator — **half of it is deliberately broken programs.**

**THE EIGHT COMPILES, CLASSIFIED RATHER THAN SUMMED.** runtime-implementer
`evt_4c3q2bae76c99`, after the Architect raised the attribution non-blocking at
`evt_5pxyvg17me2bv`; Steward re-verified three call sites at `b0a7c2945` —
`:997` plain, and `:1185`/`:1482` both wrapped in `Mutation::Exact` with
`applications == 0` asserted, which is the class the number turns on. The three
remaining mutant rows and `px8f_write_partition:509` were **not** independently
re-verified by the Steward.

    NO MUTATION APPLIED
      unmutated, no wrapper   px8f_write_all_native         ken-cli
                              px8f_write_partition          ken-verify
      exact arm               px8f_hs11_restored            ken-cli
                              px8f_write_all_plane_closed   ken-cli
    MUTANT, wrapper present, mutation applied, applications > 0
                              px8f_hs11_materialize_immediate
                              px8f_write_all_handler_owned_response_control
                              px8f_write_all_retained_result_closure_control
                              px8f_write_all_hs17_control

    v1751 (None, false)    2/2 unmutated, 2/2 exact-arm, 4/4 mutant = 8 of 8
    v1386 (Some(id), true) 2/2 unmutated, 2/2 exact-arm, 3/4 mutant = 7 of 8
          the sole (None, false) is hs17_control, A MUTANT

⇒ **THE SENTENCE THAT MAY BE WRITTEN: `v1386` is not always handed a
declaration UNDER AN INJECTED MUTATION, and it is handed one 4 of 4 over
compiles with no mutation applied.** Not 7 of 8 over a pooled population half of
which is mutant.

**THE EXACT ARM IS NOT A THIRD CONFIGURATION, AND THAT IS WHY 4 OF 4 IS NOT
ITSELF A RE-POOLING.** Grouping 2 unmutated with 2 exact-arm would otherwise
repeat the pooling one paragraph after withdrawing it, and `applications == 0`
is a **negative assert** — zero-applied and never-reached are one number, so it
cannot carry the claim. **The claim does not rest on the counter; it rests on
the default.** Architect `evt_56y19eg439rvr`, verified at source by the Steward
at `b0a7c2945` in `lowering/mod.rs`:

    thread_local D5B_HS11_MATERIALIZER_MUTATION
        = const { Cell::new(D5bHs11MaterializerMutation::Exact) }   the DEFAULT
    with_d5b_hs11_materializer_mutation  sets the cell to `mutation`
    its Restore drop guard               resets the cell to Exact

⇒ `with_..._mutation(Exact, body)` sets the selector to **the value an
unwrapped compile already holds.** An `Exact`-wrapped compile and a no-wrapper
compile are in the **identical selector state** on the HS11 axis. **Report the
structural fact, not the counter.**

**THE CLAIM IS PER-AXIS, NOT PER-CONFIGURATION.** A configuration is a vector,
not a scalar: the identity above is proved on the axis the wrapper *names*, and
*"at default on every axis"* is a different and unmade claim. What generalizes
is structural — every wrapper in `lowering/mod.rs` that sets a selector carries
a `Restore` that puts it back.

> **THE RECHECK TRAP, AND IT SITS ON THE COORDINATE ABOVE. "AT DEFAULT" IS
> DECIDED BY THE `Cell::new` INITIALIZER, NEVER BY THE TOKEN `Exact`.** Most
> selectors default to a variant spelled `Exact`, but not all —
> `D3cPositionSelection::MeasuredImmediate` and
> `BoundaryTransferInvokingSite::Direct` are at *their* defaults while matching
> no `Exact`. A reader re-deriving this by the obvious method (scan the
> selectors, confirm each reads `Exact`) gets a false negative and concludes
> axes are perturbed when they are not. **Carry the discriminator; this node
> deliberately states NO COUNT of the exceptions**, because a count is the part
> that manufactures the alarm it is trying to prevent.
>
> **Two independent ways a recheck returns a clean-looking wrong number, both
> measured tonight on this exact sweep:** the wrong discriminator (the token),
> and **line-local matching over a wrapped declaration.** These statics wrap —
> a name on one line, its type or its `const { Cell::new(...) }` on the next —
> so a line-local scan both **misses** a selector whose type wrapped
> (`D2K_BOUNDARY_TRANSFER_INVOKING_SITE`) and **over-counts** by attributing a
> wrapped initializer to the static declared above it (`D3C_ARMED`, which is a
> `Cell<bool>` defaulting to `false`, not a position selector). Same failure as
> the hard-wrapped-phrase grep already in the fleet corpus. **Read the
> declaration, not the line.**
>
> **NON-FINDING, RECORDED BECAUSE THE SWEEP HAPPENED.**
> `D3C_POSITION_SELECTION` is the one selector whose setter has no `Restore`
> guard, so it would be sticky across later compiles on the thread. It cannot
> reach these runs, and by a gate rather than an argument: it is `#[cfg(test)]`
> **alone** — not `any(test, feature = "px8-ds-test-support")` — and
> `pub(in crate::cranelift_backend)`, with zero callers under
> `crates/ken-cli/tests` or `crates/ken-verify/tests`. It does not exist in an
> integration-test build. No finding.

**THE `_control` SUFFIX NAMES THE CONTROL PROGRAM OF A MUTATION, NOT AN
UNMUTATED CONTROL** — on every one of these. `px8f_write_all_hs17_control` is
not a fixture file but an artifact name passed to `build_native_program` at
exactly one site, inside `assert_hs17_static_response_return_mutation_child`,
wrapped in `with_d5b_hs17_post_call_consumer_mutation(mutation, || ...)` with
`applications > 0` asserted. Single occurrence, so that attribution is total
rather than sampled.

⇒ **On that compile the two instruments AGREE; they do not CORROBORATE.**
`SEED-DECL` reads `target.result_contract` at the push site in `calls.rs`;
`KDROP` reads the obligation's own fields at the mutating site in `units.rs`. On
an unmutated compile their agreement is genuine corroboration — two sites, two
values, and they could have disagreed. **Here both sit downstream of one
injected divergence, so the agreement carries no independent weight.** The
earlier corroboration claim is withdrawn as written.

**THE FEATURE PROFILES ARE ALSO NOT POOLED.** Seven of the eight are `ken-cli`
test targets (`px8-ds-test-support` ON via the dev-dependency edge); one is
`ken-verify` (OFF under `-p`) — see D1-2. `v1386`'s exception sits entirely in
the feature-ON profile **and** in the mutant class at once, so those two
attributions are **not independent** and must not be reported as two facts.

> **DO NOT BANK `v1751` 8-OF-8 AS A ROBUSTNESS RESULT. IT IS NOT ONE, AND THE
> MEASUREMENT THAT WOULD SETTLE IT IS DELIBERATELY NOT ORDERED.** Architect
> `evt_56y19eg439rvr`. That `v1751` is `(None, false)` across unmutated,
> exact-arm and mutant reads as span, but if none of the four mutations can
> perturb the push site in `calls.rs`, then **six of those eight observations
> are ENTAILED by the mutations being downstream** — and an observation entailed
> by what it is offered to confirm carries no information. The coordinate's real
> support is the unmutated and exact-arm rows.
>
> ⇒ **It changes nothing, because the `v1751` coordinate discharged an
> EXISTENTIAL fence, not a universal one.** The pre-registered condition was
> *"if `v1751` is ABSENT the ruling is refuted"* — PRINTS needed **one**
> appearance where ABSENT would have needed all eight. **A denominator that is a
> harness artifact cannot damage an existential claim; it can only fail to
> support a universal one nobody made.** The bad denominator went into a
> sentence, not into the ruling, and the sentence is corrected here.
>
> **THE GENERAL TELL, and it is the reason this was hard to catch: a population
> you CHOSE has an EXCLUDED SET. A harness artifact excludes nothing, which is
> exactly why it looks complete.** Every row in it was real; what was missing
> was a criterion. State the inclusion criterion before the run and report what
> it rejected — *"eight compiles, four excluded as mutants"* is a population,
> *"eight compiles"* is whatever printed. **A denominator with no excluded set
> has been observed, not defined.**

**Nothing in the repair turns on any of this**, because the repair reads the
guard-proved identity rather than the constructed pair: `require_i64` checks the
carrier's actual tag against `k_ret_identity()`, and `tag_abi_word` is
injective, so **a passing guard is an identity proof, not a plan assertion.** A
compile where `v1386` also arrives `(None, false)` is just the other body's
shape appearing on that word. **This is one sentence in a frame, not a scope
change — fence 3 holds and it must not grow SEED.**

**Control:** every quantified claim in the report names its population, its
instrument, and its feature profile.

**D1-7. SEED REACHES 12 OF 12, NOT 6.** The split is real in the inputs and
dissolves at the repair — same edit on both halves. A `(None, false)` reading
must **not** be used to re-scope SEED to six. Ownership of the absence and reach
of the repair are different relations. **Control:** if the repair is proposed at
narrower than 12, the proposal states which of those two relations it is
arguing from.

### D1 MUST FIRST ASK WHETHER THE SPLIT IS A ROUTE SPLIT

**The two words are where the symptom surfaces, not necessarily where the cause
lives.** Across all 17 census entries the arm taken and the authority count are
perfectly correlated: every arm-1 entry has `authorities = 0`, every arm-2 entry
has 593.

**`independent_contract` is a ROUTE TAG, not a data-dependent property of a
body.** It is set at exactly three staging sites, and its presence is decided by
the site:

    :6971  Response(StaticResponseOwnerId)             Some(k_ret_identity())
    :7586  Continuation(ContinuationSpecializationId)  None
    :8286  Context(ContinuationContextId)              None

`k_ret_identity()` returns `ConstructorIdentity`, not an `Option`, and `:6978`
wraps it `Some(...)` unconditionally. So `independent_contract.is_some()` holds
**exactly** when `unit` is `Response`.

⇒ The likeliest explanation of the bimodality is therefore not a subtle causal
correlation but a **route split**: the 12 arm-1 entries are Response-route
bodies, and the 5 arm-2 entries may simply be Continuation/Context bodies whose
authority production runs elsewhere. **If so the finding is that the
authority-production path does not run on the RESPONSE-OWNER ROUTE** — a
locatable structural claim rather than a correlation to be tested.

⇒ **D1 answers this by reading one field it already has.** Every staged body
carries `unit: ExistingResultUnitIdentity` at `:2726`. Print its variant beside
the ninth field and re-read the same census: no new experiment, no new run.

    all 5 arm-2 entries Continuation/Context   the split IS a route split; scope
                                               D1 at the Response staging path,
                                               not at two words
    any arm-2 entry a Response body            NOT purely route -- that body
                                               carries an independent_contract
                                               and took arm 2 on a differing
                                               identity, so the equality in the
                                               arm condition is doing real work

**ARM 1 IS AN EQUALITY, NOT A PRESENCE TEST.** At `:4474` the condition is
`body.independent_contract == Some(identity)`. Arm 2 is its negation and covers
two distinct populations: `None` (Continuation/Context), and `Some(other)` — a
Response body whose `k_ret_identity()` differs from the demanded identity.
**Do not restate the census as "bodies carrying an `independent_contract` have
no authorities."** That is strictly stronger than what was measured, and the
`Some(other)` case would falsify it while leaving every measured number intact.

### THE ROUTE READ RAN. ROUTE SPLIT CONFIRMED, 17 of 17.

`evt_5q6tah71smnn5`, same-tuple same-build measurement, arm-2 population covered
**5 of 5** per the arm-2-first targeting: **zero counterexamples in the
`Some(other)` cell.**

    Response      12   auth = 0
    Context        5   auth = 593

⇒ **Authority production does not run on the RESPONSE-OWNER route.** Context
bodies in the same compile get 593 entries; Response bodies get none. **The site
is settled: the RESPONSE STAGING PATH, not `v1751`/`v1386`.** An earlier
provisional 3-of-17 read is retired by this one, not merged with it.
**WIDENED 2026-09-16:** that path is a three-step sequence and `:6971` is its
second step — see the SITE row.  The claim corrected here is *which route*, and
that half is unchanged.

> **THE EMPTY `Some(other)` CELL ESTABLISHES NOTHING ABOUT REACHABILITY, AND D1
> MUST NOT TREAT THAT ARM AS DEAD CODE.** In the implementer's terms:
> **unobserved-in-3-programs is not proven-impossible.** Five of five with zero
> counterexamples is a strong empirical result about three programs of one
> family; `:4474` is an equality, and nothing in the code bars a `Response` body
> from arm 2 whenever `k_ret_identity()` differs from a demand. **Scope at the
> route; do not scope at the emptiness of that cell.** Architect
> `evt_7r7snh6bhm8vy`.

### D1'S FIRST DESIGN QUESTION: PRODUCE vs PROVE. THE FORK HAS FOUR ARMS.

**Architect `evt_gjy1ed88b7nh`, adding an arm to its own ruling.** "Production,
not re-keying" stands as against re-keying — the tables are empty — but it was a
two-way fork, and **the code already implements a third arm**.

    PRODUCE   make authority exist at v1751/v1386        <- what was ruled
    RE-KEY    move authority that already exists         <- refuted, tables empty
    PROVE     route Response bodies through prove_forwarded_value -- the
              machinery arm 2 already uses and the kernel already trusts

`prove_forwarded_value` at `:3126` grounds a value on **any** of three
conditions — an `authorities` entry matching identity and word, a
`detached_consumer_authorities` entry, or `call_seeds.get(&value) ==
Some(&identity)` — and when none holds it **recurses through `value_def`**,
walking block params back through reachable predecessors for a grounding source
elsewhere. **Arm 2 therefore discharges bodies holding no authority at the
published word.** Arm 1 does none of it: its whole condition is
`realized_call_words.contains(&publication.returned_word)`, a set membership.

> ### RETRACTED — THIS BLOCK CARRIED A FALSE CLAIM AND IT WAS LOAD-BEARING.
>
> The retracted text asserted: *"Having no authority at the published word is
> not sufficient to cause a refusal"*, offering `funcid60` as a body that
> **closes** with `None` at `v26`, and `funcid56`/`funcid57` as two more.
> Architect-authored (`evt_gjy1ed88b7nh`), Steward-carried, retracted at
> `evt_9bz296v5099j`. **All three instances fail, and the first is refuted by
> the census printed beside it.**
>
> **A CENSUS ENTRY IS, BY CONSTRUCTION, A BODY THAT DID NOT CLOSE.** Verified at
> source, `units.rs:4555-4567` and `:4586-4589`:
>
>     missing  = required.iter().filter(|(target, identity)|
>                    !finished.iter().any(|c| c.unit == target_unit
>                        && c.target == *target && c.identity == *identity))
>     details  = missing.iter().filter_map(...)   <- the census IS built from missing
>
> Every element of `missing` is a `(target, identity)` pair for which **no
> finished certificate exists**. `funcid60` is a census entry — the fifth arm-2
> tuple, identity `{4362, 37}` — so `funcid60` did **not** close. D0's twelfth
> field says so independently: finished certificates for `funcid60` are `[]`.
>
> **`funcid56`/`funcid57` FAIL FOR A DIFFERENT REASON, AND IT IS THE SHARPER
> ONE.** The inference was *"no `Continuation` body appears in `missing`,
> therefore they closed."* But **`missing` is FILTERED FROM `required`**, so:
>
>     absent from `missing`  =  CLOSED  or  NEVER DEMANDED
>     which one              =  UNMEASURED
>
> A body never demanded is never in `required`, therefore never in `missing`,
> and **never had to close**. That is zero-executions-versus-never-called.
>
> ⇒ **One instance refuted, two undetermined: the claim has no support and is
> withdrawn.** It is recorded rather than deleted so the next reader does not
> re-derive it.

**WHAT DOES NOT FALL WITH THE RETRACTION — do not over-retract.** The PROVE
refutation below is measured **directly** and never rested on the withdrawn
claim. The route/shape mechanism and `funcid60`'s eleventh field both stand.

⇒ **PROVE AS STATED IS REFUTED, BY MEASUREMENT, AND THE FORK IS NOW FOUR.** D0
wired arm 1 to `prove_forwarded_value` and reported the result rather than
arguing from the signature: `valid = FALSE`, `grounded = FALSE`, for both
`Response` bodies.

    PRODUCE   make authority exist at v1751/v1386   <- what was ruled
    RE-KEY    move authority that already exists    <- refuted, tables empty
    PROVE     route Response through prove_forwarded_value
                                                    <- REFUTED, measured
    SEED      make `obligation.identity` be `Some` for the call whose result IS
              the published word, so the EXISTING certificate -> call_seeds path
              grounds it at `:3145`. No new machinery anywhere.

**THE CAUSE IS THE ABSENT SEED, AND THE REFUSING ARM IS DOWNSTREAM OF IT.** The
three direct grounding conditions sit at `:3139-3145`, **before** the
`value_def` match; the `ValueDef::Result` arm returning `ForwardingProof::
default()` is at **`:3234`**. Both `Response` bodies carry
`obligation.identity = None` at the published word, so no call seed exists,
`:3145` cannot fire, and the walk reaches `:3234` only afterwards. **Refusing
`ValueDef::Result` is CORRECT and must not be repaired** — an instruction result
is an ORIGIN, not forwarded from anywhere, so a forwarding proof declining to
invent its provenance is the function working. Teaching the walk to handle it
would be this node's own defect one level down: a plan standing in for an
observation.

**SEED IS THE ARM TO BEAT, AND IT CANNOT FAKE A DISCHARGE.** `call_seeds` is
populated from `certificate.identity`, and a certificate comes from `finished` —
a body that has **already discharged its own Result contract**. So a call seed
is an OBSERVED identity, not a planned one, which answers this node's founding
defect in its own terms. If no finished certificate matches,
`all_calls_finished` goes false and the body **waits** rather than closing: the
failure mode is a stall, not a false proof. PRODUCE has no such interlock.

**SEED'S SUFFICIENCY IS NOT A PROPERTY OF A BODY, AND THE `[]` MEASUREMENT
CANNOT SETTLE IT FOR A ROOT.** D0 measured `[]` finished certificates for both
callees — but measured them in a tree where SEED had **not** been applied, and
seeding changes which bodies are demanded at all (see SEED CREATES THE DEMAND IT
NEEDS, below). The cases split by ROLE IN A GIVEN REFUSAL, never by funcid:

    DOWNSTREAM pair   the callee is itself in THIS refusal's `missing` set, and
                      `missing` filters `required` at :4555, so that callee is
                      ALREADY DEMANDED and has ALREADY FAILED. Seeding makes the
                      entry wait on a demonstrated failure. SEED does NOT
                      close it.
    ROOT pair         the callee is NOT in this refusal's `missing`. Whether SEED
                      closes it is UNMEASURED: if the callee is not yet in
                      `required`, seeding CREATES that demand and the callee may
                      then certify. The pre-seed `[]` does not predict the
                      post-seed state.

⇒ **Do not read `[]` as evidence that SEED is insufficient.** For a downstream
pair it is exactly that; for a root pair it is a measurement of a world that
seeding would change. The open question **"is the callee in `required` at
all?"** decides the root case.

> **`[]` MEANS TWO DIFFERENT THINGS IN THE TWO ROWS, AND ONE NUMBER WAS REPORTED
> FOR BOTH.** Pre-SEED, *"no certificate"* is ambiguous between **never asked**
> and **asked and refused** — and SEED is exactly the change that resolves it,
> so a pre-SEED measurement cannot bound a post-SEED behaviour on the root half.
> **This is the same shape as absent-from-`missing` = CLOSED ∪ NEVER-DEMANDED,
> reproduced one column over**, in the certificate field instead of the
> membership field.

### IT IS A GRAPH. WE HAVE BEEN READING IT AS A LIST.

**Architect ruling `evt_9bz296v5099j`, and it supersedes "one shape repeated".**
The refusal string has said this since the first filing: *"the finished
generated-Result proof graph is **not closed**."*

    funcid59   published word = Result(inst2064, 0), the result of a call to funcid60
    funcid60   is ITSELF a census entry, failing on the SAME identity {4362, 37}
    funcid58   PROVE-EVAL identity {4362, 37}
    funcid59   PROVE-EVAL identity {4362, 37}

⇒ **`funcid59` cannot be seeded by any repair made AT `funcid59`. Its callee
never certifies**, and the thing SEED would make it wait for is **another member
of the same failing population**.

**WHY THIS CHANGES D1's SIZE AND NOT JUST ITS PROSE.** A fix applied at a
downstream entry moves nothing while its root still fails, and a repair sized at
12 when the roots number fewer is **sized at the wrong thing**. `size: M` was
already carried rather than re-derived; this is the second open fork that could
move it, and it is the one most likely to.

**THE MEASUREMENT THAT PARTITIONS IT — a join on fields D0 already has.** For
each of the 12: the callee at the published word (eleventh field), and whether
that callee is itself in `missing`.

    callee OUTSIDE the census   ->  ROOT
    callee INSIDE the census    ->  DOWNSTREAM

> **ROOT/DOWNSTREAM IS A PROPERTY OF A `(refusal, identity)` PAIR, NOT OF A
> BODY.** `funcid59` is downstream in 4 refusals and a ROOT in 1 — the refusal
> whose `missing` set is `{funcid58, funcid59}`, with no `funcid60` in it.
> **Tabulating this per-funcid yields a stable-looking but wrong answer for 4 of
> those 5 rows**, and a stable wrong answer is more dangerous than a flickering
> one. Every count in this section is a count of PAIRS.

> ### THE 7 IS PROVISIONAL — THE JOIN USED THE WRONG EDGE SET.
>
> **There are TWO graphs over the same bodies, and the partition asked the
> wrong one.** Verified at source:
>
>     GROUNDING graph   the ELEVENTH field -- the callee at the PUBLISHED WORD.
>                       Answers "what would ground this value". The join ran here.
>     GATING graph      ALL of `body.call_obligations`, looped at :4420 and gated
>                       at :4467 `if !all_calls_finished { continue; }` -- with the
>                       two failure edges at :4427 (realization) and :4440
>                       (identity/certificate). Answers "WHAT BLOCKS THIS ENTRY"
>                       and therefore "WHERE DOES A REPAIR LAND".
>
> **`:4467` sits BEFORE the arm split at `:4474`**, so gating is decided over
> every obligation a body carries, not just the one at its published word.
>
> ⇒ **A body can be ROOT on the grounding graph and DOWNSTREAM on the gating
> graph — same body, same refusal, opposite classification.** `funcid60` is the
> live candidate: it carries an identity-bearing obligation on `funcid58`, itself
> a census member.
>
> **This is the PAIR RULE one level up.** That rule fixed the KEY (a
> `(refusal, identity)` pair, not a funcid). This fixes the EDGE SET. A
> classification is only as well-defined as the graph it is taken over, and
> naming the key correctly does not save you from asking the wrong graph.
>
> **DO NOT ACT ON 7.** It is not yet a count of repairs. `GATE-BLOCK` /
> `GATE-NOPUB` decide it. Architect `evt_3xn88cqhg1msp`, line numbers corrected
> to `:4467`/`:4427`/`:4440` and verified at source here.
>
> **AND IT MAY INVERT THE BOUNDARY ENTIRELY.** If this measures out, the
> EXCLUDED arm-2 entries may be **downstream of D1's own arm-1 population**
> rather than independent "gate working" evidence — which would reverse the
> direction of the boundary question below. NOT RULED; waiting on the numbers.
>
> **RESOLVED 2026-09-16, AND IT DID INVERT.** The gating-graph re-run settled
> both halves: `7+5` is superseded by `12+5`, and the arm-2 entries are indeed
> downstream of D1's own arm-1 population. **This block's prediction was
> correct and its number was still the wrong kind of thing** — see the SUMMARY
> OBJECT block below, which is the term this block did not reach.

**SUPERSEDED — THE PARTITION RAN AGAIN ON THE GATING GRAPH: 12 + 5.**

    on the GROUNDING graph (2026-09-15)   12 arm-1  ->  7 ROOTS + 5 DOWNSTREAM
    on the GATING   graph (2026-09-16)    12 arm-1 entries, ALL arm-1
                                           5 arm-2 entries, ALL downstream
                                          across BOTH binaries, no exceptions

⇒ **A repair lands at a root; a downstream pair closes when its root does. That
twelve was a count of SYMPTOMS stands.** Architect ruling `evt_18mtk5rr061nk`.

**THE `7` IS RETIRED, NOT CORRECTED DOWNWARD.** The grounding-graph join asked
"what would ground this value"; the question D1 needs answered is "what blocks
this entry", and `:4467` gates over every obligation a body carries. The re-run
reproduces the `12`/`5` splits exactly from the published `missing=` sets, which
is the check that this is the same population re-measured rather than a
re-description of it.

**AND THE SURVIVING NUMBER IS STILL NOT A COUNT OF REPAIRS** — for a new reason
that has nothing to do with which graph it was taken over. See below.

> ### KEY, THEN GRAPH, THEN SUMMARY OBJECT. THE THIRD TERM.
>
> **Three repairs to one trigger, each one level up from the last, and all three
> were repairs to WHICH COUNT.** Nobody asked whether a count was the right
> object at all until the population was read by funcid.
>
>     KEY      a funcid            ->  a (refusal, identity) PAIR
>     GRAPH    grounding edges     ->  GATING edges (:4420 loop, :4467 gate)
>     OBJECT   a count of ENTRIES  ->  DISTINCT BODIES carrying the defect shape
>
> **THE TELL WAS AVAILABLE AT EVERY STAGE AND IS ONE LINE: there are three
> funcids in all 17 entries.** A 17-row table over a 3-element domain is
> announcing its own multiplicity, and the announcement survived two rounds of
> people correcting the table.
>
> ⇒ **Getting the key and the graph both right does not save you from
> summarizing over the wrong KIND of thing.** `12` and `7` are counts of
> observations; a repair is an edit to the program. The two are different
> objects and no arithmetic converts one into the other.
>
> **AND ALL THREE FAILURE MODES PRODUCE A COMPLETE, STABLE, PLAUSIBLE TABLE.**
> None of them is visible from the output. **This one additionally survived
> being explicitly marked PROVISIONAL**, which is the part worth carrying: the
> provisional marker attached to the number's VALUE, and the defect was in its
> KIND. *Marking a number uncertain does not put its units in question.*
>
> Architect `evt_2mw3vjp30yjbn`, correcting its own trigger twice and then the
> premise under both repairs.
>
> ### THE FOURTH TERM: THE REPLACEMENT OBJECT HAD NO REFERENT EITHER.
>
> `DISTINCT BODIES` was offered as the right KIND of object to replace a count
> of entries. **It is under-defined over this population** — the six compiles
> are six different Ken programs, so "the same body across compiles" names
> nothing. The well-defined object is the PLANNER ROLE, and there are two.
>
>     KEY      funcid              ->  (refusal, identity) PAIR
>     GRAPH    grounding edges     ->  GATING edges
>     OBJECT   count of ENTRIES    ->  distinct BODIES
>     REFERENT distinct BODIES     ->  no referent. PLANNER ROLE, and there are 2.
>
> ⇒ **Three successive repairs asked WHICH COUNT; the fourth found that the
> thing being counted did not exist.** This is the warrant for retiring the size
> trigger outright rather than re-pointing it: not merely that a count is the
> wrong kind of input, but that **the search for a better object to count can
> itself terminate in nothing.** Architect `evt_1vnsrqk27v53c`.

> ### WHY AN UNEXECUTABLE INSTRUCTION WAS ALLOWED TO LAND. IT FAILS SAFE.
>
> The `BODIES` row landed at `0929deedd` prescribing a source-symbol read that
> was measured, minutes later, not to exist. It was **not** pulled, and the
> durable reason is **not** that the only seat who would act on it had already
> superseded it — that was true that day and is a claim about the fleet, not
> about the artifact.
>
> **The row FAILS SAFE.** Its operative force is a PROHIBITION — *never settle
> this by number* — and that half was correct and protective. The prescription
> bolted to it was unexecutable, so the outcome of trying to follow it is **"not
> settled"**, which is exactly the state the row exists to enforce.
>
> ⇒ **An unexecutable instruction whose prohibition half is sound cannot produce
> a wrong answer; it can only produce a wasted lookup. That is a CHURN cost, not
> a CORRECTNESS cost**, and it does not warrant pulling a gated candidate.
> Architect `evt_4cs2zghzcwggt`. **The test is which half is load-bearing** — had
> the row's operative force been the prescription rather than the prohibition,
> the same defect would have been a correctness hazard and the answer would flip.

> **"ROOT" MUST NOT QUIETLY BECOME "REPAIRABLE LOCALLY."** `funcid62 NOT IN
> missing` inherits the exact ambiguity retracted above: **closed OR never
> demanded.** The classification is sound either way — "root" asserts only that
> the callee is not a census member — but the consequence is not:
>
>     funcid62 CLOSED for some identity  a certificate may already exist to ground on
>     funcid62 NEVER DEMANDED            no certificate exists and none is coming;
>                                        SEED at funcid58 must CREATE the demand
>
> One field settles it: **is `funcid62` in `required` at all?**

**AND THE SECOND CASE IS SEED'S STRONGEST ARGUMENT, NOT A PROBLEM.** Verified at
source, `:4300-4307`: the push to `required` is guarded by
`let Some(identity) = obligation.identity else { continue; }`.

⇒ **SUPPLYING `obligation.identity` AUTOMATICALLY ADDS `(callee, identity)` TO
`required`. SEED CREATES THE DEMAND IT NEEDS.** Seeding `funcid58` does not
merely let `funcid58` ground — it **obliges `funcid62` to certify**. And since
`missing` filters `required`, a callee that fails to certify **enters the census
as a new, more localized refusal**.

> **SEED THEREFORE CANNOT SILENTLY SUCCEED.** Either the callee certifies and the
> body closes on observed evidence, or the callee fails and the refusal gets
> louder and better localized — **never a quiet green.** That is the fail-closed
> direction this node exists to protect. **PRODUCE has no equivalent**: an
> authority asserts an identity because something wrote it, and demands nothing
> of anyone.

**A JOIN HAZARD IS RECORDED AND MUST NOT BE RE-DERIVED.** D0's `SEED-DECL` print
at the obligation creation site shows 116 call sites whose callee declares no
contract and 4 that do, with `v1386` apparently among the four. **That table is
NOT joinable to the census**: `SEED-DECL` does not carry the defining function,
and Cranelift value numbering is **per-function**, so a bare `v1386` match across
the two tables may name a different body's value. It is kept as DO-NOT-USE
rather than deleted, because a deleted hazard gets re-derived by the next reader.

**STILL OPEN, AND IT IS A SOURCE READ RATHER THAN A RUN.** Whether
`obligation.identity = None` arises because the callee **declares nothing**, or
because the call site **drops a contract the unit does have**. `DeclaredUnitCall`
is constructed with `result_contract: None` at several sites and with a real
contract at four. If the callee declares nothing, the defect is upstream at the
declaration and `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` owns it; if it does
declare, SEED is the repair and it is narrow. **The SITE line is NOT amended
until this is read** — it still says `:6971`.

> ### READ 2026-09-16. THE CALL SITE DROPS A CONTRACT IT WAS HANDED — arm (b).
>
> **`units.rs:6868-6878`, on the response-owner K-call path, verbatim:**
>
>     if let Some(obligation) = compiler
>         .function_local
>         .pending_call_result_obligations
>         .iter_mut()
>         .find(|obligation| obligation.result_word == returned.word)
>     {
>         obligation.realization_required = true;
>         // The owner's exact tag/arity guards refine this realized word;
>         // a declaration on the K call is not consumed as identity proof.
>         obligation.identity = None;
>     }
>
> **The comment concedes it** — *"a declaration on the K call is not consumed as
> identity proof"*, and you cannot decline to consume a declaration that was
> never made. **But a comment is not a proof**, and this is settled from an
> INVARIANT instead: see the DISCRIMINATOR row for the construction-site
> invariant (`calls.rs:2479-2483`) and the exhaustive two-mutation grep that
> together make `(realization_required = true, identity = None)` reachable only
> from `:6877`.
>
> ⇒ **THE SITE LINE IS NOW AMENDED, and it widened rather than moved.** It is
> the three-step sequence in the SITE row, of which `:6971` is the second step.
> **The question above is answered; the fork it fed is not closed** — the
> invariant fixes what the bit MEANS, not which value it has, and that bit is
> not yet printed. Architect `evt_2tzn77b3tcxj4`.

### THE CUT CONTRADICTS ITSELF AT THE BOUNDARY, AND ONLY THE GRAPH SHOWS IT

**Architect `evt_4s7beb7qyrhqz`. This is scope, not prose.**

**Stated as PAIRS, per the rule above — `funcid59` is downstream in 4 refusals
and a ROOT in 1, so this is about the 4, not about the body.**

    the 4 DOWNSTREAM pairs whose callee is funcid60
               the ENTRY is in D1's population (arm 1, one of the 12)
               the CALLEE funcid60 is in the EXCLUDED population
               (arm 2, "cause 2 is the gate working, unchanged by the closure")

⇒ **D1 is scoped to repair entries whose blocker is explicitly OUT of its
scope.** Those pairs cannot close while `funcid60` does not certify, and
`funcid60` is excluded **by name**. That is a contradiction in the cut whichever
side turns out to be right, and **it is invisible in a list** — it appears only
once the population is read as a graph.

**D1 DISPOSITIONS THIS BOUNDARY BEFORE REPAIRING ANY DOWNSTREAM ENTRY.**

**THE BOUNDARY WAS THE WHOLE BALLGAME, AND IT HAS NOW BEEN RULED — the block
below settles it and SUPERSEDES the hypothetical in this paragraph.** Recorded
as written because the ruling is best read against the question it answered:
all 5 arm-2 roots are `funcid60`, excluded as "the gate working", and some of
the 5 downstream arm-1 pairs are downstream of those. **If those arm-2 refusals
are correct, their downstream pairs are CORRECT PROPAGATIONS and must not be
repaired at all** — which would take D1's population below 7 as well. That was
what the `funcid60` ARM2-EVAL was framed to decide.

> ### RULED 2026-09-16, AND IT WENT THE OTHER WAY. Architect `evt_2mw3vjp30yjbn`.
>
> **The question above presupposed that `funcid60`'s arm-2 refusal is a verdict.
> It is not a verdict, because arm 2 never ran.** Measured across both binaries:
> every `funcid60` pair is `GATE-BLOCK`'d on `funcid58` at the same identity, in
> every refusal where it appears, and `:4467` sits before the arm split.
>
>     the hypothetical above   funcid60 arm-2 refuses CORRECTLY
>                              => funcid59 is a correct propagation
>                              => do not repair, population falls
>
>     what was MEASURED        funcid60 never REACHES arm 2
>                              => the excluded entries are downstream of D1's
>                                 own arm-1 population, not independent of it
>                              => the direction REVERSES
>
> ⇒ **THE EXCLUSION IS REFUSED, AND THE PROHIBITION IS NOT.** "Cause 2 is the
> gate working" remains standing law and nothing here licenses repairing it
> toward green. What falls is the claim that **these particular entries are
> cause 2** — a gate that never executed cannot be the gate working, and
> crediting it with a correct refusal is the `[]` union one more time, at the
> granularity of a whole arm.
>
> **`funcid59` IS STILL OVERDETERMINED** and that half is unchanged: it fails
> for its own arm-1 reason *and* because its callee fails. Both still
> disposition separately. What the ruling settles is the callee's side.

**D1 PARTITIONS ROOTS AND DOWNSTREAM, AND THE DOWNSTREAM HALF IS NAMED.** It is
the 5 `funcid60` arm-2 entries, plus whatever the arm-2 diagnostic returns for
`funcid60`'s first actual evaluation. That diagnostic is **fenced: it must not
land, it is not a repair, and a refusal is a first-class result** — see the
observation plan.

**`funcid60` IS NOW THE MOST INTERESTING BODY IN THE CENSUS.**

    funcid60   DOWNSTREAM on the GATING graph. Publishes Param(block7,0) --
               WALKABLE, unlike the Response bodies. 593 authorities.
               Eleventh field Some(None): no obligation names its PUBLISHED
               WORD -- which is a GROUNDING-graph fact and does NOT mean
               "blocked on no callee". It carries an identity-bearing
               obligation on funcid58 and is GATE-BLOCK'd on it at :4467.
               AND IT STILL FAILS TO CERTIFY {4362, 37}.

> ### THIS ENTRY'S OWN QUESTION WAS A TWO-ARM FORK MISSING THE MEASURED ARM.
>
> It read: *"Why does a walkable, authority-rich, callee-free body fail? Either
> the walk ran and found no grounding, or it hit `predecessors.is_empty()` under
> the cuts and returned the default."*
>
> **Both arms presuppose the walk was entered. It was not.** `funcid60` is gated
> out at `:4467`, before the arm split at `:4474`, in every compile — so the
> missing third arm, *the walk never ran*, is the one that was measured. The
> two-arm form also inherited `Some(None)` as "callee-free", reading a
> grounding-graph field as a gating-graph fact.
>
> ⇒ **The question "why does it fail?" was never answerable from this data**, and
> its two arms are the `[]` union again — this time wearing an exhaustive-looking
> fork. See the SUMMARY OBJECT block: **an EXHAUSTIVE split over one variable is
> not exhaustive over the thing you care about.**

**WHAT ACTUALLY STANDS.** `funcid60` sits directly upstream of arm-1 entries D1
is scoped to fix, and it has **never been observed to evaluate arm 2 at all**.
Its refusal here is a PROPAGATION, not an independent arm-2 verdict. The
arm-2 diagnostic is what would produce the first such verdict, and it is
two-sided on its own merits: with `funcid58`'s certificate present,
`:4455-4465` seeds `call_seeds[obligation.result_word]`, the third direct
grounding condition at `:3139-3145` grounds on exactly that seed, and the first
condition tests the 593-entry authority map. It can ground through authorities,
ground through the seed, or refuse.

> **These coordinates were RE-DERIVED at source here, not relayed.** `:4456-4459`
> is `call_seeds.insert(obligation.result_word, certificate.identity)` guarded by
> `if let Some(certificate)`; `:3139-3145` is a three-disjunct test whose first
> arm is the `authorities` map, whose second is `detached_consumer_authorities`,
> and whose third is `call_seeds.get(&value) == Some(&identity)`.
> **Note the conjunction the prose smooths over:** the seed route grounds only if
> the word being grounded IS `obligation.result_word` *and* the certificate's
> identity matches the demanded one. "A proof path exists" is a claim about
> two-sidedness, not a prediction that it grounds.

### RAN 2026-09-16 — ARM 2 EVALUATED FOR THE FIRST TIME, AND IT PROVED. ONE PAIR.

**Diagnostic only.** Env-gated on `KEN_DIAG_ARM2_BYPASS`, uncommitted, never
landed, supplying nothing but the `call_seeds` entry a real certificate would
have produced, at the `:4440` break only. **Bypass scope MEASURED, not
asserted:** exactly one triple fired, twice (two fixpoint iterations), nothing
else. Keyed to the `(refusal, identity)` pair over the GATING graph —
`ken-verify --test px8f_write_partition`, identity `id4362`.

    ARM2-EVAL  funcid60  Context(ContinuationContextId(0))  id4362
               published=v26  def=Param(block7,0)
               valid=true  grounded=true  sources={v3653}  cuts=7

    missing BEFORE   [(60,id4362), (58,id4362), (59,id4362)]
    missing AFTER    [(58,id4362), (59,id4362)]

⇒ **`funcid60`'s entry was a GATING ARTIFACT of `funcid58`'s arm-1 refusal.**
The exclusion "cause 2 is the gate working" is refuted for this pair by two
independent facts: the gate never executed, **and** when made to execute it
PROVES. The refusal narrows to exactly the two arm-1 bodies — the same shape as
the one pre-existing refusal that never carried `funcid60`. **That shape
prediction came true without being aimed at**, which is worth more than the
narrowing.

> ### ALL SIX PAIRS RAN 2026-09-16. ALL SIX PROVE. POPULATION 17 -> 12.
>
> `ken-cli --test px8f_buffer_native`, same env-gated diagnostic, same three
> fences. Bypass scope measured: exactly five triples fired, each twice, every
> one the same shape (`demander=funcid60`,
> `demander_unit=Context(ContinuationContextId(0))`).
>
> **The narrowing is real and measured: 17 entries to 12.** The 12 are 2 planner
> roles across 6 compiles, one defect shape. Architect `evt_s8jr4hf5hb5w`.
>
> **SIX AND FIVE RANGE OVER DIFFERENT OBJECTS, AND BOTH ARE RIGHT. `17 - 6` IS
> NOT THE ARITHMETIC.** Architect `evt_4rqft0c2k09s7`.
>
>     SIX   ranges over EVALUATIONS.   The diagnostic evaluated funcid60's
>                                      arm 2 in six compiles.
>     FIVE  ranges over CENSUS ENTRIES. funcid60 has a census entry in five.
>
>     arm-1   2 bodies x 6 compiles = 12
>     arm-2   1 body   x 5 compiles =  5     <- FIVE, not six
>                                      17 - 5 = 12
>
> **The sixth evaluation is the out-of-partition compile** — the one where
> `funcid60` never refused, which is why it has no census entry to remove. This
> is the same `sixth compile` held outside the partition below, stated here so
> that a reader doing the subtraction does not land on 11 and conclude the
> headline is wrong.
>
> ### AND THAT ASYMMETRY IS EVIDENCE, NOT BOOKKEEPING
>
> **The arm-1 bodies appear in 6 of 6 compiles; `funcid60` appears in 5 of 6.**
> `12 = 2 x 6` against `5 = 1 x 5` **is the same fact as "funcid60 did not refuse
> in one compile"** — and it is a COUNT, arrived at from the census, **not a
> reading off the interleaved 8-thread stream whose attribution was retracted.**
> The sixth compile's anomaly was sitting in the partition's own arithmetic the
> whole time.
>
> **ITS WARRANT DEPENDS ON ONE UNANSWERED QUESTION, AND THE NODE MUST NOT BANK
> THE UPGRADE BEFORE IT IS ANSWERED:** was the 17-entry census built from that
> same interleaved output, or from a separate structured collection?
>
> ### ANSWERED 2026-09-16, AND THE ANSWER IS A THIRD ARM NEITHER OF US NAMED
>
> **SAME CHANNEL, AND THE CHANNEL'S FAILURE MODE CANNOT TOUCH THIS PROPERTY.**
> Each refusal is **ONE ATOMIC LINE** carrying its own `missing=` and `details=`.
> ⇒ Interleaving can misattribute **which compile a line came from**; it cannot
> **split or merge a census**. The retracted claim was about attribution; this
> claim is about cardinality, and the defect does not reach it.
>
> **THAT IS A STRUCTURAL ARGUMENT, NOT AGREEMENT BETWEEN TWO RUNS** — which is
> what makes it a warrant. Two collections agreeing would have been two
> instruments capable of being wrong the same way. **The 17-entry census stands,
> and for a better reason than the question asked for.**
>
> **THE FORK ITSELF WAS MIS-KEYED, AND THE ARCHITECT OWNS IT:** `SAME`/`SEPARATE`
> keys on the SOURCE when the question is about the PROPERTY, and so omits the
> arm where the source is shared and the property is invariant to what the
> shared source gets wrong. Same defect as `(a)`/`(b)` below — **a classification
> keyed to the wrong entity, committed twice in one evening by the seat ruling on
> it.** Architect `evt_193ekcwm21t7w`.
>
> The two arms as originally posed, kept because they are why the question was
> worth asking:
>
>     SEPARATE  => the sixth compile's absence has a warrant INDEPENDENT of the
>                  retracted stream. (ii) stays refuted, (i) and (iii) stay live,
>                  and the absence no longer rests on a disowned instrument.
>     SAME      => the absence inherits the same defect, and FIVE-not-six is
>                  itself suspect: the census could be missing an entry for the
>                  same reason the stream lost a line. EVERY count in this node,
>                  12 included, would rest on it -- a SECOND projection beneath
>                  the one this node already established.
>
> **RESOLVED ABOVE, and by neither arm.**
>
> ### AND "SIX OF SIX" MAY BE WORTH ONE OBSERVATION, NOT SIX. — REFUTED 2026-09-16
>
> **THE RULING BELOW IS WITHDRAWN. ITS PREMISE IS MEASURED FALSE.** The
> fingerprints came back: `funcid60` is **THREE DISTINCT BODIES ACROSS SEVEN
> COMPILES**, so `(a)` — *the same generated construct in every program* — does
> not hold. **The weight is at least 2, not 1**, because the six evaluations span
> at least two genuinely different bodies. Architect `evt_193ekcwm21t7w`,
> withdrawing its own ruling.
>
> **AND THE REFUTATION IS WORTH MORE THAN THE CONFIRMATION WOULD HAVE BEEN: two
> DIFFERENT `funcid60` bodies produced byte-identical arm-2 verdicts.** The
> invariance is therefore **not body identity**, and what it is instead is
> unexplained and was correctly not explained.
>
> **`(a)`/`(b)` WAS A FALSE DICHOTOMY, and it is the same mis-keying as
> `SAME`/`SEPARATE` above.** *Same generated construct* or *instrument repeating
> one body* both key on the SOURCE of the verdicts. **The truth is the arm
> neither names: the body VARIES and the verdict DOES NOT.**
>
> **SEVEN COMPILES, NOT SIX, AND THE ERROR IS IN THE SENTENCE DIRECTLY BELOW.**
> *"These are six DIFFERENT Ken programs"* takes an `ARM2-EVAL` **emission
> count** as a **program count**. There are seven compiles. ⇒ **An observation
> count was inherited as an object count INSIDE the ruling whose entire subject
> was inheriting an observation count as an object count.** Fifth occurrence in
> this arc, and the first one located in the text of the rule itself rather than
> in something the rule was applied to.
>
> **ONE JOIN SETTLES THE WEIGHT EXACTLY, and it is data already in hand rather
> than a run:** which compile did each of the six `ARM2-EVAL`s come from, and
> what was that compile's `funcid60` fingerprint?
>
> The withdrawn ruling is kept below rather than deleted, because it is why the
> fingerprints were demanded, and because the reasoning is sound on a premise
> that simply turned out not to hold:
>
> Every one of the six verdicts is **byte-identical apart from the identity**:
>
>     published=v26   def=Param(block7,0)   sources={v3653}
>     reachable_blocks=6664   cuts=7
>
> **These are six DIFFERENT Ken programs.** They do not produce the same value
> number, the same block, the same grounding source, the same 6664-block
> reachable set and the same 7 cuts by coincidence. Two live readings:
>
>     (a) funcid60's Context body is the SAME GENERATED CONSTRUCT in every
>         program -- continuation-context boilerplate, emitted identically
>         regardless of the user program. BENIGN, and INFORMATIVE.
>     (b) the instrument is reporting ONE BODY REPEATEDLY.
>
> ⇒ **If (a), the six proofs are ONE OBSERVATION WITH MULTIPLICITY SIX, and the
> evidential weight of "uniform, no exceptions" is 1.** *"All six prove"* reads
> as six independent confirmations; six runs of the same generated body through
> the same walk is one confirmation repeated.
>
> **THIS IS THE SUMMARY-OBJECT LESSON APPLIED TO EVIDENCE RATHER THAN TO REPAIR
> COUNTS.** A count of observations is not a count of things — the same shape
> that made `12` read as twelve sites. **Fourth occurrence in this arc.**
>
> **The result is NOT weaker than its actual claim.** For *"funcid60's entries
> were gating artifacts, not independent arm-2 refusals"*, one body proving six
> times is entirely sufficient, because that claim is about `funcid60`. **The
> multiplicity only misleads when "six of six, no exceptions" is later cited as
> ROBUSTNESS**, which is work it cannot do. Do not cite it that way.
>
> **AND (a) WOULD CLOSE SOMETHING LEFT OPEN.** The earlier note that the same
> three funcids in both binaries was *"suggestive but NOT evidence of identity —
> deterministic numbering over a shared prelude would produce exactly that
> coincidence"* is **that hypothesis confirmed** if (a) holds. It also settles
> what *"the same body across compiles"* can mean: **not the same source
> definition, but the same GENERATED CONSTRUCT** — cleaner than the planner-role
> substitute in the BODIES row, and already present in the output.
>
> **THE INSTRUMENT THAT SETTLES (a) VERSUS (b) IS THE CLIF FINGERPRINT, NOT A
> SYMBOL NAME.** The symbol read was ordered, run, and withdrawn as entailed —
> it is minted from the `unit` field already in the census. The fingerprint is
> computed from the CLIF, which the planner ordinal does not feed. **Its
> asymmetry decides how this block may be read:** if `funcid58`/`funcid59`
> differ across compiles and `funcid60` does not, that is (a). **If all three
> come back identical the test is INCONCLUSIVE, not confirmatory** — identical
> counts are also exactly what (b) looks like. Full statement in the BODIES row.

**ONE PAIR IS NOT FIVE — the restraint below is SUPERSEDED by the six-pair pass
above, and is kept because it is why the pass was demanded.** The BOUNDARY
ruling rests on *arm 2 never RAN*, and
that is measured for all five. *What arm 2 SAYS when it runs* has **one data
point out of five**. If any of the remaining four REFUSES when let through, the
exclusion revives **for that pair specifically and for no other**. Both halves
are per-`(refusal, identity)` pair, as the pair rule requires.

> ### THE TRANSFER FENCE. IT PROVED THROUGH THE ONE INPUT D1'S POPULATION LACKS.
>
> `sources={v3653}` — one of `funcid60`'s own eight demand-matching authority
> words. The injected seed is keyed on `v451` and **is absent from `sources`**.
> So the **593 authorities** carried it, through the FIRST direct grounding
> condition at `:3139-3145`, which tests `authorities.get(&value)`.
>
>     funcid60   Context    authorities = 593    GROUNDED through them
>     funcid58   Response   authorities = 0
>     funcid59   Response   authorities = 0      every table EMPTY
>
> ⇒ **"Arm 2 proves" is a fact about a body with 593 authorities and DOES NOT
> TRANSFER to a population with zero.** The reading to refuse is *"the proving
> arm works, so PROVE is viable for D1."* The walk succeeded through the one
> input the arm-1 bodies do not have. **PRODUCE vs PROVE vs SEED STAYS OPEN.**

> ### IT ADDS ALMOST NOTHING TO THE ROUTE-LOCALIZATION CLAIM. Architect
> ### `evt_4cs2zghzcwggt`, correcting its own prior paragraph.
>
> It was first credited as *"a working mechanism demonstrated on one side of"*
> the claim that authority production does not run on the Response-owner route.
> **That credit is withdrawn as mostly ENTAILED.** What upgraded the
> localization was **`593` versus `0`, and that was in the census before this
> diagnostic ran.** Given a non-empty authority map holding a demand-matching
> entry, condition 1 at `:3139-3145` firing is close to definitional.
>
> **WHAT IT DOES ADD, STATED NARROWLY SO IT CANNOT BE STRETCHED.**
> `reachable_blocks=6664`, `cuts=7`, `def=Param(block7,0)`, grounded at `v3653`:
> the walk was entered, the cut derivation ran, reachability was computed over a
> real CFG, and the walk reached the published word and terminated at a
> demand-matching authority. **That is the end-to-end arm-2 MACHINERY
> functioning, never before observed on a non-Response body.** It is a fact about
> the machinery. It is **not** a fact about a population whose map is empty.
>
> **arm-taken and authority-count are BOTH functions of the ROUTE**, so their
> perfect correlation across the 17 entries is **entailed by the route split, not
> independent evidence for it.** Discriminating *"production does not run on the
> Response route"* from *"Response bodies lack authorities for some other
> reason"* needs **a Response body WITH authorities, or a Context body WITHOUT.**
> **NOTHING SO FAR HAS PRODUCED EITHER. THAT MEASUREMENT IS STILL UNRUN.**

> ### DO NOT READ THE ACCUMULATION OF FENCES AS ACCUMULATING EVIDENCE.
>
> The Architect and the Steward flagged the same correlation from two sides
> within minutes — *arm-taken and authority-count are both functions of route*,
> and *a grounding condition succeeding on the body that has the input is
> entailed by having the input.* **Those are one defect at two granularities,
> and NEITHER WAS DISCOVERED FROM THE DATA.** Each of us recognised a shape we
> had just finished writing about.
>
> ⇒ **Two independent-looking fences on the same claim are not two findings, and
> a growing list of caveats is not a growing case.** The discriminating
> measurement has not moved an inch: a Response body with authorities, or a
> Context body without. Recognition is cheap and reproducible; it is not
> evidence. Architect `evt_4cs2zghzcwggt`.

> ### PRE-REGISTERED: WHAT THE SEED-FREE CONTROL MEANS, FIXED BEFORE IT RUNS.
>
> `cuts=7` and `derive_certified_cuts` reads `call_seeds`, so the seed may have
> contributed EDGES even though `v451` is not the grounding source. **A grounding
> SOURCE and a graph EDGE are different roles; one value can fail one and serve
> the other.** The control is the bypass WITHOUT the seed — let `funcid60`
> through the gate and nothing more.
>
>     PROVES seed-free    funcid60 needed only to be LET THROUGH. The gate at
>                         :4467 demanded a certificate its proof never consumed:
>                         the precondition is STRONGER than the proof requires.
>                         A STRUCTURAL OBSERVATION.
>     REFUSES seed-free   the seed was load-bearing through the CUTS though not
>                         the grounding source. The certificate really was a
>                         dependency and the gate is correct as written.
>
> **NEITHER OUTCOME LICENSES "RELAX THE GATE." THIS IS A NEW DOOR TO REPAIRING
> TOWARD GREEN AND IT IS CLOSED.** If it proves seed-free, the honest statement
> is that the gate **over-demands relative to this proof** — not that it should
> be weakened. A gate that demands all certificates before proving is a
> **fixpoint-ORDERING discipline**: it cannot know what a proof will consume
> before running it, and *"only demand what the proof turns out to need"* is
> circular on its face. **Name the observation, file it, do not act on it.**
> Architect `evt_1vnsrqk27v53c`. This sits alongside the standing cause-2
> prohibition and is a SECOND entrance to the same prohibited move.

**STILL OPEN AFTER THIS PASS:** the planner-identity/linkage print, the (a)/(b)
generated-construct question above, and the sixth compile — all folded into one
single-threaded re-run. **The four remaining arm-2 pairs have since RUN; see the
six-pair block above.** The sixth compile stays OUTSIDE the partition.

> ### THE SIXTH COMPILE'S SPLIT IS MISSING AN ARM, AND THE TELL IS A TENSION
> ### BETWEEN TWO OF ITS OWN FINDINGS. Architect `evt_s8jr4hf5hb5w`.
>
>     (i)   a caller that SWALLOWS the refusal
>     (ii)  funcid60's certificate was NEVER DEMANDED there
>     (iii) THE REFUSAL WAS EMITTED AND NOT OBSERVED     <- MISSING
>
> **(ii) is refuted by measurement**: `id5242` appears in both the bypass list
> and the verdict list, so `funcid60` was demanded and gated there.
>
> **But (i) as stated contradicts the source read.** A test that does
> `result.expect("...")` does not *swallow* a refusal — **it PANICS on one.** So
> `px8f_buffer_native.rs:1185` expecting success is in tension with (i), and **a
> live tension between two of your own findings is the tell that the split is
> incomplete**, not a puzzle to resolve within it.
>
> ⇒ **(iii): "printed no refusal" is a measurement taken through the SAME
> 8-thread interleaved stream that was declared unsound for attribution.** An
> instrument that misattributes output across threads is an instrument that can
> **lose** it. The claim about *which* compile was retracted; the claim that **no
> refusal appeared came from the same stream and was not.** Absence of a printed
> refusal, read off interleaved output, is not absence of a refusal — **the union
> again, at the level of the observation channel.**
>
> **(i) MUST NOT BE ADOPTED BY ELIMINATION.** Two arms refuted out of three named
> is not a conclusion when the third was never on the list. The single-threaded
> pass settles all three at once, so nothing changes in what is being run.

**What is unsound is only the compile-to-test MAPPING**, read positionally off
interleaved 8-thread output and retracted by its own author — the `expect`-
success source read at `px8f_buffer_native.rs:1185` stands on its own.

**NOTHING HERE MOVES THE FORK.** PRODUCE / PROVE / SEED is untouched by the
six-pair result. The discriminator bit is still the only thing that closes it,
and it is one field on an existing print.

### THE SEED-FREE CONTROL PROVES, AND THE SEED WAS NOT INERT

**Measured 2026-09-16 on `px8f_write_partition`, identity `id4362`; implementer
`evt_2ncdwex1gvhmw`, accepted by the Architect at `evt_6psqbdtxd5jk6`.** The
bypass **supplies nothing.** It only declines to break at `:4440`, so `funcid60`
reaches `:4474` with no `call_seeds` entry from `funcid58` at all.

    ARM2-EVAL target=funcid60 unit=Context(ContinuationContextId(0))
              identity=id4362 published=v26 def=Param(block7, 0)
              valid=true grounded=true sources={v3653}
    missing   [(funcid58, id4362), (funcid59, id4362)]

⇒ **`funcid60` needed only to be LET THROUGH.** It proved with an **empty
certificate set**, grounding at the same authority word either way. The reading
was pre-registered before the run, not chosen after it.

**AND THE SEED WAS NOT INERT — the delta says so, and the distinction is the
point of recording it:**

    with seed    reachable_blocks=6664   cuts=7
    seed-free    reachable_blocks=7762   cuts=6

One fewer cut and 1098 more reachable blocks. The seed genuinely contributed a
certified cut and shrank the walk; the **proof** simply did not depend on it.
**"Not load-bearing" is the conclusion; "had no effect" would be false**, and the
stronger claim was available for free and refused. That is this node's union
caught *before* it formed rather than after.

### RECORDED OBSERVATION — `:4467` OVER-DEMANDS. NOT A NODE YET

Architect `evt_6psqbdtxd5jk6`; Steward scope ruling `evt_5k3r7anrwmr9j`.

**`:4467` makes a body's proof attempt conditional on EVERY call obligation
being finished**, so one unfinished obligation suppresses the attempt entirely
and the resulting refusal is a **gating artifact rather than a verdict.**

**The circular repair is "demand only what the proof turns out to need"** —
circular before the proof runs, correctly refused by the implementer who found
it, and not proposed. **The non-circular form does not demand less; it stops
making the demand a PRECONDITION:**

    Today     :4467 gates a body's proof on ALL its call obligations being
              finished. One unfinished obligation suppresses the attempt.

    Instead   attempt every staged body; any body that proves adds its
              certificates; re-attempt the bodies that failed; repeat until a
              round adds nothing.

Nobody decides in advance what a proof needs. A body needing no certificates
proves on round one; a body needing `funcid58`'s proves on round two; a body
that genuinely cannot prove is refused **after** the fixpoint, and that refusal
is then a real refusal. The seed-free control above is direct evidence it would
work here: `funcid60` proved with an empty certificate set, so a fixpoint closes
it on round one **whatever `funcid58` does.**

**THREE FENCES, RECORDED VERBATIM, INCLUDING THE ONE THAT BINDS THE STEWARD:**

1. **It does not close the fork and must not be offered as a substitute.** The
   12 arm-1 entries refuse on a genuine zero-authority population. A fixpoint at
   `:4467` does nothing for them. It eliminates the 5 gating artifacts and
   nothing else.
2. **It is not on the critical path**, because fixing the 12 makes the 5
   disappear anyway. Its independent value is **instrument integrity** — it makes
   every future refusal in this machinery honest.
3. **It must not be used to argue D1's size down.** The 5 were already
   partitioned out; this explains their mechanism and does not re-open the count.

**WHY IT IS AN OBSERVATION AND NOT A NODE.** Not blocking, not on the critical
path, no ring able to start it, no lane asking for it. And the deciding reason
is fence 2 used harder than it was written: **cutting a node now frames scope,
ACs and size over a population the in-flight work is expected to eliminate.**
That is framing against a moving target, and the cost lands on whoever inherits
the frame.

> ### THE RE-CUT TRIGGER ASKS ABOUT POPULATION, NOT WARRANT
>
> Architect `evt_3d373pbgfy01r`.
>
> An earlier form of this said that if the 5 do not survive the arm-1 repair,
> the observation stands as *"the explanation of why they were never real."*
> **That is this node's own union one more time, at the level of the trigger:**
>
>     The 5 vanishing is CONSISTENT WITH the gate having over-demanded.
>     It is EQUALLY CONSISTENT WITH the gate being exactly right and merely
>     UNSATISFIED -- the arm-1 bodies start proving, that supplies the
>     certificates, :4467 is now satisfied, and the 5 disappear with the
>     gate never having been wrong about anything.
>
> **A disappearance under a repair that changes the gate's INPUT cannot tell you
> the gate was too strict.**
>
> ⇒ **The warrant does not depend on the 5 and is already in hand.** The
> seed-free control measured the over-demand directly: `funcid60` proved with an
> empty certificate set, grounding at `v3653` with no `call_seeds` entry. The
> finding is established **today**, whatever becomes of the 5 tomorrow.
>
> **So the trigger is sound as a question about POPULATION — do the 5 survive,
> is there anything left to fix — and unsound as a question about WARRANT.**

### OPEN — THE ENUM HAS THREE VARIANTS; THE CENSUS MEASURED TWO

    ExistingResultUnitIdentity      census entries
      Response                          12   auth = 0
      Context                            5   auth = 593
      Continuation                       0   NOT MEASURED

**`Continuation` is absent from all 17 and is NOT structurally barred from being
there.** Demand is built as `Vec<(FuncId, ConstructorIdentity)>` at `:4293` —
from `body.required_contract` at `:4297`, and from call obligations via
`decode_direct_callee(&body.func, obligation.call)` at `:4304`/`:4307` — then
matched at `:4356`/`:4406` by `.filter(|(target, _)| *target == body.target)`.
**Demand matching is by `FuncId` and does not know the route.** So a
`Continuation` body whose target is demanded enters that loop exactly like any
other: **its absence here is a gap in the measurement, not a consequence of the
code.**

⇒ **This changes the repair's SHAPE, not its site.** The same
response-staging sequence either way — see the SITE row, which widened on
2026-09-16 to three steps of which `:6971` is the second.

    if Continuation ALSO produces authority   the split is Response-vs-rest, and
                                              Response is missing something two
                                              other routes have  -> ADDITIVE
    if Continuation produces NONE either      the split is Context-vs-rest, and
                                              only Context has it -- a different
                                              repair and a different size

**D1 establishes which before it designs the fix.** Same move that just paid:
the enum is the printed variable, and two of its three values is not exhaustive
over it. **It does not gate the route finding.**

**This measurement is OWED AND UNRUN, and it is not cheap.** The question is
`authorities.len()` on `Continuation` bodies **that appear in the census**, and
none do — so it needs a program whose continuation bodies **fail to close**, and
`px8f` has none. An adjacent pre-D0 staging reading exists (`Response` 0 of 2,
`Continuation` 4 of 6, `Context` 4 of 6 carrying an authority at the published
word) and points toward Response-vs-rest, but it is a different field, a
different build, and a different population. **It does not substitute for the
measurement.**

> **THE MIRROR BOUND ON THAT ADJACENT TABLE, because it runs in the direction
> that flatters the conclusion.** Its `Continuation`/`Context` legs are arbitrary
> staged bodies; its `Response` leg is **n=2, and both rows ARE the defect
> population** — `funcid58`/`funcid59` at `v1751`/`v1386`, the two words this
> node is about. So it compares twelve arbitrary bodies against two selected for
> being broken. ⇒ *"The Response route never produces authority"* remains
> measured only on Response bodies chosen for failing, and **`px8f` cannot fix
> that — every Response body it has is one of the two.**
>
> Note also that `Context` is **not** uniformly authority-bearing: `funcid60`
> and `funcid61` look exactly like the Response rows on this axis. **The
> predicate is not the route tag alone** — a route-level generalization died on
> the same table that produced it.

> ### THE CONTEXT CONTROL LOCALIZES A PATH. IT DOES NOT NAME A CAUSE.
>
> `Context → 593` beside `Response → 0` is the best row in the census and a real
> control on the production axis. But **the two differ in more than the route
> tag**: they are staged by different functions (`:8286` vs `:6971`), and
> `Context` carries a `required_contract` (`context.result_contract`) where
> `Response` carries an `independent_contract` and `None` for required.
>
> ⇒ The contrast licenses exactly *"scope at the Response staging path"* and
> **does NOT license *"port what Context does to Response."*** **A control that
> differs in several ways localizes the fault to a path; it does not identify
> which difference causes it.** If D1 reaches for `Context` as a template, that
> step needs its own argument.

### The census D1 is sized from

Delivered `evt_50wgwpaancsqh`, **counts corrected at `evt_1rvbsp2qrkapy`** —
the first report was parsed from a log still being written (the waiter keyed on
`test result:`, which that suite emits four times because it re-executes itself
as child processes). The corrected figures are the operative ones:

| | entries | field 5 `authorities.len()` | field 6 |
|---|---|---|---|
| **ARM 1** | 12 | `0` — every one | `[]` — every one |
| **ARM 2** | 5 | `593` — every one | 8 words — every one |

17 for 17, 4 demanded identities, 3 programs, **no intermediate case**.
`funcid60` takes arm 2 with 593 entries two functions away in the same compile:
an in-compile control nobody had to construct.

> **FIELD 6 IS NOT INDEPENDENT OF FIELD 5 AND MUST NOT BE COUNTED AS A SECOND
> MEASUREMENT.** At `:4609-4614` field 6 is `body.authorities.values().filter(…)`
> — with `authorities.len() == 0` it is `[]` **by construction**. "12 with
> `field5 = 0`, 12 with `field6 = []`" is one fact reported twice. **`field5 = 0`
> is decisive for (c2) on its own** (a body with no authorities has none for the
> demanded identity), so the conclusion stands at full strength and field 6 adds
> nothing to it. Recorded because two routes agreeing is informative exactly when
> they could have disagreed — Architect `evt_1xemdvdj9fghw`, correcting a
> Steward suggestion that named field 6 as the distinguishing measurement.

**The SIGABRT baseline run is NOT owed and is not to be spent now.** The
attribution is adequately fenced: the Steward's CI census attributed those shards
to stack overflow before this closure existed, which is strong but is not a
control at this SHA. The stack-overflow population is its own node. **A baseline
becomes owed the moment anyone claims the closure caused it.**

## Acceptance

**THESE ARE THE ARC'S ACCEPTANCE CRITERIA AND THEY ARE NOT THE WHOLE SET.** The
D1 repair carries seven more, `D1-1` through `D1-7`, in the **D1 DISPATCH**
block above — the three fences, the red-population prohibition, and the
population bound on the residual. `AC-3` and `D1-5` are the same requirement
stated at both levels deliberately; that is not a duplicate to tidy away.

**AC-1. The refusal is OBSERVED, not argued for.** An input on which the
repaired arm 1 refuses, run and recorded. The parent arc has one node already
stalled for want of exactly this (`RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP`'s
`D0`), so it is stated first here.

**This node is not at that risk and the difference is worth naming.** Its
witnesses already exist: runtime-implementer's reduction is a 139-line program
reaching three distinct failures by single-line edits, with `d1/d3/d4.ken` and a
native-vs-interpreter harness in the scratchpad. A node with a witness in hand
is a check; a node without one is a documented invariant.

**AND THE WITNESS MAY NOT BE MANUFACTURED BY SYNTHESIZING AN IDENTITY.**
`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE:333-335` forbids it in terms, and the
Architect **extended that prohibition to controls** as a ruling
(`evt_5gws0pnssfqch`) rather than leaving it to be read across. This is exactly
what stalled `RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY`: a mutation
could not produce its shape without synthesizing an identity, so the shape
stayed unreachable by any permitted instrument.

The governing line is that node's `:332` — *"if either is unavailable in
principle at some site, THAT IS THE FINDING."* **An in-principle unavailability
is a result to report, not an obstacle to engineer around.** If the repaired
arm 1 cannot be made to refuse by permitted means, say so and stop; do not
reach for a synthesized identity to close AC-1.

**AC-2. The census is a population, not a verdict.** Each newly-refusing
hand-off named with its target, its demanded identity, and what
`body.authorities` held at its returned word. A count alone does not satisfy
this — the distribution is what sizes `D1`.

**AC-3. Cause 2's six checks are NOT made green, and the diff is checked for
it.** State explicitly that no change removes an emission refusal without
adding an identity requirement. This is an AC because it is the failure mode the
ruling exists to prevent.

**AC-4. Arm 1 and arm 2 are shown to agree on what evidence they require.** The
asymmetry is the defect; a repair that leaves arm 1 checking something weaker
than arm 2 has relocated it. Name what each arm requires, side by side.

**AC-5. Each missing entry is classified into ONE of the four cases in the
fence below, and an entry folds in only on a ground the artifact prints.** A
report of the form *"not a differing identity, therefore this node's"* does not
satisfy this — that is the elimination the fence exists to prevent. **Nor does
"case (c), therefore same mechanism as cause 1"**, which is the elimination one
level down; see the ruling immediately below.

> ### CASE (c) IS A SYMPTOM, NOT A MECHANISM. BOTH ARMS PRODUCE IT.
>
> **Architect `evt_3rp08wzfyqr1q`, correcting its own four-case spec; verified
> at source by the Steward.** `FinishedUnitResultContract` is pushed in exactly
> **one** place, `:4542`, inside `if proven` at `:4524` — and **both arms feed
> that one `proven`**. So:
>
>     arm 1 taken, word not in realized_call_words  -> proven=false -> case (c)
>     arm 2 taken, proof not valid/grounded         -> proven=false -> case (c)
>
> ⇒ **No certificate does not distinguish the arms.** And arm 2 declining is
> **the proving arm doing its job** — it consulted `body.authorities`, found
> nothing usable, and refused. That is the opposite of arm 1's substitution.
> Folding an arm-2 refusal in as an instance of *"arm 1 substitutes a plan for
> an observation"* would put a false mechanism claim in the artifact that exists
> to record that mechanism.
>
> **The `a/b/c/d` split is exhaustive over AUTHORITY STATE. The fold is a claim
> about MECHANISM. Different variables** — the same shape as the defect this
> fence was written to fix, one level down.
>
> **THE ATTACHMENT GROUND IS THE KEYING FACT, WHICH IS ARM-INDEPENDENT.** Field 6
> of the details tuple lists authorities matching the demanded identity
> elsewhere in the table. Where it is non-empty, an authority for that identity
> **exists and is simply not keyed at `publication.returned_word`** — whichever
> arm ran. A population attached by a fact the artifact prints beats one
> attached by a mechanism it cannot see.

### Case (c) is TWO populations; one had a ground, and that ground is now PROVISIONAL

Measured across all six `rt_parity_native` shards (lieutenant, `evt_2cedrpvwyyv70`;
five shards carry only `funcid58`/`funcid56`, one additionally carries
`funcid55`):

| | `authorities.len()` | field 6 | reading |
|---|---|---|---|
| **(c1) MISPLACED** — `funcid58`, `funcid56` | 695 | `[v33002]` | evidence exists, keyed elsewhere |
| **(c2) ABSENT** — `funcid55` | **0** | `[]` | no evidence anywhere |

⇒ **(c1) folded on the keying ground — DOWNGRADED to provisional, see the block
below this paragraph. (c2) DOES NOT FOLD** — its table is empty, so
there is nothing misplaced and no keying fact to attach by. It does not fold on
the mechanism ground either, which is undetermined for it exactly as for the
others. **`funcid55` is currently attached to this node by nothing but the error
string, and is recorded here as UNATTACHED.** The `independent_contract` print
now exists and has run — **but on the px8f programs, not on these shards**, so
it has not settled `funcid55`.

> ### 2026-09-15: (c1) IS DOWNGRADED FROM FOLDED TO PROVISIONALLY FOLDED.
>
> **Architect `evt_1xemdvdj9fghw`, cutting against its own earlier fold.** D0's
> census came back bimodal 17 for 17: arm 1 only ever on an EMPTY table, arm 2
> only ever on a populated one. If that holds generally, a body with `auth=695`
> is an **arm-2** body — and an arm-2 refusal is the proving arm doing its job,
> the gate working, explicitly **not this node's defect**. The (c1) entries have
> exactly that shape.
>
> **It is a downgrade and not a removal, and the discipline that stops it being
> a removal is the one this node already enforces.** The bimodality is measured
> on px8f, three programs of one family. Extending it to `rt_parity` is the same
> cross-compile transfer that "a funcid is not an identity across two compiles"
> forbids — making the move with the *correlation* instead of with the number.
> The `rt_parity` arm was **never measured**: those entries were read from CI
> logs before the ninth field existed.
>
> ⇒ **The pending read is now a TEST OF THE BIMODALITY, not a classification,
> and that makes it worth more than when it was ordered:**
>
>     arm 2 on auth=695   the correlation HOLDS; those entries are the gate
>                         working and they leave this node entirely
>     arm 1 on auth=695   the correlation BREAKS -- arm 1 is reachable WITH a
>                         populated table, which is a NEW and LARGER finding
>                         than the one this node was cut for
>
> **Either answer is worth having and neither blocks `D1`**, which excludes these
> entries from its scope and its sizing either way. Venue and timing are the
> ring's call.

**Lead on what `funcid55` is, scoped because it is NOT measured here.**
`RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` records `derive_certified_cuts` —
the first line of **arm 2** — running exactly twice in the whole px8f compile,
for `funcid55` and `funcid60`. If that carries, `funcid55` reached arm 2 and its
refusal is the proving arm working, not this node's defect. **That measurement
was at `b0a7c2945` on the px8f compile, not at `0f71ab5b9` on an rt_parity
shard**: different SHA, different program. A funcid is not an identity across two
compiles. It is a lead about which arm to expect, and the print settles it.

## THE FENCE: the six `rt_parity_native` shards are NOT established as this node's

**Architect, explicitly: the attachment is by error signature today, and a
signature that reads identically is not a mechanism identity.** Do not fold them
into this node's population before the read.

### The first filing's two-way fork was NOT A PARTITION. This replaces it.

**Architect, `evt_4eerxykb24edj`, correcting its own discriminator spec; every
claim below re-verified at source by the Steward at `0f71ab5b9`.**

The refusal loop at `:4569-4586` runs over **every** missing entry before any
details are built, and it `continue`s in three cases — no staged body (`:4570`),
no publication (`:4573`), and **`None` authority (`:4576`)**. Only a **present**
authority reaches the mismatch arm at `:4579`.

⇒ **THE DIFFERING-IDENTITY OUTCOME IS FORECLOSED BY THE SIGNATURE THAT SELECTED
THESE SHARDS.** An artifact reading *"the finished generated-Result proof graph
is not closed"* (`:4626`) has already run that loop to completion, so **no**
missing entry in it has a present-but-differing authority. The old fork's second
branch cannot coexist with the error string used to pick the six shards. It was
not merely distinguishable from the artifact — it was excluded before anyone
opened one.

**The population the signature actually admits is four cases:**

    (a) no staged body for the target      -> DROPPED from details by the
                                              filter_map at :4587, so
                                              details.len() < missing.len()
    (b) body has no publication            -> details field 3 is None
    (c) authority at returned word is None -> details field 4 is None
                                              <- THE ONLY CASE THAT FOLDS IN
    (d) authority PRESENT and its identity -> details field 4 is
        EQUALS the demand, yet the check      Some((word, identity)) with
        is still missing                      identity == the demanded one

**(d) is the live fork and neither the ruling nor the first filing named it.**
It says the returned word carries the *correct observed* constructor authority
and the proof still did not close — which points at arm 2's
`prove_forwarded_value` failing `valid && grounded`, or at the certificate match
at `:4560-4564`. **It does not point at arm 1's fail-open at all**, so those
shards are not this node's. (a) and (b) are neither mechanism; each is its own
small finding about the staged set.

> **WHY THIS IS NOT A TIDY-UP.** A reader given the old two-branch fork who
> confirms *"not a differing identity"* concludes *"therefore outcome one, folds
> in"* — and thereby routes (d), (a) and (b) into this node **silently, by
> elimination against a split that never covered them.** The criterion would
> have been satisfied correctly and still landed in the wrong column.
>
> Architect, naming its own lesson against its own spec: *an exhaustive split
> over one variable is not exhaustive over the thing you care about.*

**How to classify, per missing entry, from the artifact alone.** Field 4 of the
details tuple is `authorities.get(returned_word).map(|a| (a.word, a.identity))`,
so it is printed:

    details.len() vs missing.len()   -> (a)
    field 3 is None                  -> (b)
    field 4 is None                  -> (c)
    field 4 is Some, identity == demanded -> (d)

`body.independent_contract == Some(identity)` is still worth reporting per
entry, but it is now the **second** question, not the first.

**So the discriminator is a read, not an experiment** — the original point, and
it survives the correction intact.

(The Architect's ruling cited the mismatch arm as `:4579-4585` and the authority
read as `:4603-4607`. The first resolves exactly; the second is a field inside
the details tuple begun at `:4587`. Both name the same code — recorded because a
coordinate off by a few lines is the kind of thing a later reader silently
repairs in the wrong direction. The Architect has taken both corrections.)

## Relationship to `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` — adjacent, not duplicate

Both are fail-opens in the same discharge path, at **different sites**, and the
distinction decides where a repair goes:

| | that node | this node |
|---|---|---|
| site | the `proof.sources` loop and its `grounds.is_empty()` sink | arm 1 of the `proven` conditional |
| mechanism | a foreign identity is **detected** and silently discarded | authority is **never consulted** |
| relation to arm 2 | inside the proof arm 2 runs | bypasses arm 2 entirely |
| witness | none found; `D0` is blocked on one | in hand |

**When arm 1 is taken, `prove_forwarded_value` never runs**, so that node's sink
is unreachable on this path. They are not two views of one defect and neither
repair implies the other. Both nodes' repairs point the same direction —
refuse where the code currently proceeds — which is a reason to read them
together, not a reason to merge them.

**A consequence for that node's stalled `D0`, stated because it cuts against
its current framing.** Arm 1 is taken whenever the plan matches the demand — the
rule, read off the conditional — so the population that reaches arm 2's proof at
all is smaller than a reader of that node would assume, and its witness hunt is
correspondingly harder for a reason that has nothing to do with foreign
identities. **How much smaller is not measured**, here or anywhere: the
`WRITE_ALL` reading establishes that arm 1 is trafficked, not how much traffic
arm 2 retains. Whoever picks that node up should measure its reachable
population before spending more on the witness. **Not a finding of this node;
a pointer, and it is unverified in that direction.**

Third sibling, distinct again: `RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY`
asks whether the *ledger* has a net — whether two callers can demand different
identities of one body. That is a reachability question about the ledger's
purpose, not about what arm 1 consults. It supplies this node with evidence
(above) and takes none from it.

## RELEASED 2026-09-15 21:30. The hold is discharged.

**The release condition this node set was:** *"the discriminator result is
posted. Nothing else gates it — `D0` does not depend on that read, only its
scope does."*

**It is posted.** The lieutenant read all six `rt_parity_native` shards
(`evt_2cedrpvwyyv70`): five carry `funcid58`/`funcid56` only, one additionally
carries `funcid55`. The scope that read decides is recorded above — (c1) folds,
(c2) is unattached.

> ### THE HOLD HAD BECOME CIRCULAR, AND THAT IS WHY THIS SAYS 21:30 RATHER THAN 20:55.
>
> The `independent_contract` ninth field is to be added **inside `D0`'s
> rebuild** — the Architect's words were *"not a separate build, not a census"*,
> on the stated ground that **`D0` already rebuilds `units.rs` with the closure
> applied.** That is true once `D0` is running, and `D0` is this node's first
> deliverable.
>
>     node release   waits on  the print
>     the print      waits on  D0's rebuild
>     D0             waits on  node release
>
> **All three legs were individually correct.** The Architect assumed `D0` was
> scheduled; it could not be, because the node holding it was unreleased — and
> the Steward was holding it for a measurement that `D0` was supposed to
> produce. The tell was five seats reading **identical ctx across a 30-minute
> interval** with work owed: not one stalled seat, a ring with no actionable
> next step.
>
> ⇒ **The conflation was mine: "the discriminator answered" is not "the
> population is fully classified."** The node's own condition asked for the
> first. I held for the second, which `D0` exists to deliver. **A release
> condition is discharged by the event it names, not by the certainty the
> author wishes it had bought.**

`funcid55` remains **UNATTACHED** — that is a recorded state of the population,
not an unmet release condition. `D0` settles it.

## Contention

`crates/ken-runtime/src/cranelift_backend/lowering/units.rs`, on the `ABI-S6
D5b` candidate branch. **That is the same file and the same branch the D5b arc
is respinning on**, so `D0` is a branch-local measurement that must not be
released as a parallel candidate against it. Sequence it with the D5b ring, not
beside it.

`D2`, `R1`, `R2` and entry-18 of the parent node are held pending D5b
stabilization; this node joins that set rather than jumping it.
