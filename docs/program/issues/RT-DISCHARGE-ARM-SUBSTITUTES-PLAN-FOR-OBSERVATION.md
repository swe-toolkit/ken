---
id: RT-DISCHARGE-ARM-SUBSTITUTES-PLAN-FOR-OBSERVATION
title: "Arm 1 of the constructor-identity discharge substitutes a PLANNED identity plus realization for an OBSERVED constructor identity at the returned word. It never consults `body.authorities` at any point -- its whole discharge condition is membership of the returned word in `realized_call_words`, and `independent_contract` is `emission.row.k_ret_identity()`, a planned value off the emission row. The plan is compared against the demand, the word is checked to be initialized, and NOTHING checks what the word actually holds. The two live symptoms are the two exits of this one rule, and ONE OF THEM IS THE GATE WORKING: repairing the emission refusal toward green converts a loud refusal into a silent miscompile. The closure makes the tree REDDER, and that red population is the deliverable."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]
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
                 STILL SETTLED BY MEASUREMENT, NEVER BY NUMBER. The linkage
                 symbol is being printed alongside, so the claim rests on a
                 measurement rather than on a reading of that comment.
                 Architect evt_1vnsrqk27v53c; implementer evt_7v8vwk1w4bqgt.
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
                 PROVE vs SEED evaluation, and a SITE still pending a
                 declaration-path source read. Neither moves when the population
                 count moves.
                 Re-pointing the trigger at distinct bodies would have fired it
                 IMMEDIATELY -- 2 and 4 are both below 5 -- and re-cut to S on a
                 number the Architect explicitly declined to size on. That is
                 the guess this node exists to avoid, arriving through a repaired
                 trigger instead of a broken one.
                 D1 RE-SIZES WHEN A FORK CLOSES, NOT WHEN A NUMBER LANDS.
                 Steward decision; the Architect declined to rule size
                 (evt_18mtk5rr061nk, evt_2mw3vjp30yjbn).
    BOUNDARY     RULED, AND IT INVERTS. Every funcid60 pair is GATE-BLOCK'd on
                 funcid58 at the same identity, in every refusal where it
                 appears, and ARM 2 NEVER RAN IN ANY COMPILE. So the 5 arm-2
                 entries are DOWNSTREAM OF D1'S OWN ARM-1 POPULATION.
                 The direction recorded-but-not-ruled is the direction it went.
                 D1 MUST PARTITION ROOTS AND DOWNSTREAM, and the downstream half
                 is now NAMED rather than assumed.
                 Architect evt_4s7beb7qyrhqz, RULED at evt_2mw3vjp30yjbn.
    REPAIR       PRODUCE constructor authority.
                 NOT re-keying: there is nothing to re-key, the tables are empty.
    SITE         the RESPONSE STAGING PATH at :6971 -- NOT the two published
                 words. v1751/v1386 are where the symptom surfaces. Settled by
                 the route read below; see also the open Continuation question,
                 which decides the repair's SHAPE but not its site.
    FIRST        D1 EVALUATES **PRODUCE AGAINST PROVE** BEFORE DESIGNING THE
                 REPAIR. "Production, not re-keying" was a TWO-way fork and
                 there is a third arm the code already implements; see below.
                 This does not resize D1 -- it stops D1 building the narrow
                 fix by default.
    NOT-EXCLUDED the 5 arm-2 refusals. THIS EXCLUSION IS REFUSED ON MEASUREMENT
                 (Architect evt_2mw3vjp30yjbn). It read them as "the proving arm
                 declining, cause 2 is the gate working" -- but that gate NEVER
                 EXECUTED in any compile, so the ground credited for excluding
                 them was never observed to operate. They are downstream of D1's
                 own arm-1 population; see BOUNDARY.
                 THE PROHIBITION ON REPAIRING CAUSE 2 TOWARD GREEN IS UNTOUCHED.
                 What falls is the claim that these entries ARE cause 2.
    EXCLUDED     the rt_parity (c1) entries -- PROVISIONALLY FOLDED, below. They
                 enter neither D1's scope nor its sizing.
    HOLE         abi_s6_mapping_file_backed_native is NOT MEASURED and is NOT
                 KNOWN TO CONTRIBUTE ZERO. It aborts (SIGABRT, stack overflow)
                 before producing a hand-off. Zero executions and never-ran are
                 two facts and one number; do not let this population enter D1
                 as a zero.

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
is settled: `:6971`, not `v1751`/`v1386`.** An earlier provisional 3-of-17 read
is retired by this one, not merged with it.

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

**ONE PAIR IS NOT FIVE.** The BOUNDARY ruling rests on *arm 2 never RAN*, and
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

**STILL OPEN AFTER THIS PASS:** the four remaining arm-2 pairs in
`px8f_buffer_native`, the planner-identity/linkage print, and the sixth compile
— all folded into one single-threaded re-run. **The sixth compile stays OUTSIDE
the partition until it lands.** Its test does `expect` success
(`px8f_buffer_native.rs:1185`), so explanation (i) is dead for whichever compile
it is; what is unsound is only the compile-to-test **mapping**, read positionally
off interleaved 8-thread output and retracted by its own author.

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

⇒ **This changes the repair's SHAPE, not its site.** `:6971` either way.

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
