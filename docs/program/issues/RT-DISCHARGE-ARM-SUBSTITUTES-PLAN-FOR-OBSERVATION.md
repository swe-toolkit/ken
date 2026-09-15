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
    EXCLUDED     the 5 arm-2 refusals -- the proving arm declining, unchanged by
                 the closure, and cause 2 is the gate working.
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

### D1'S FIRST DESIGN QUESTION: PRODUCE vs PROVE. THE FORK HAS THREE ARMS.

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

> **HAVING NO AUTHORITY AT THE PUBLISHED WORD IS NOT SUFFICIENT TO CAUSE A
> REFUSAL.** `funcid60` is `Context`, 593 authorities, `None` at `v26`, and it
> **closes** — the existing instance of a body discharging with no authority at
> its published word. Two `Continuation` bodies (`funcid56`, `funcid57`) have
> the same property and also closed.

⇒ **PROVE may dominate.** This node's founding defect is that arm 1 substitutes
a PLANNED identity plus realization for an OBSERVED one. The remedy for that is
to establish the observed identity, and `prove_forwarded_value` is the existing,
already-trusted machinery that does exactly that — whereas producing an authority
at two words is the narrowest patch that happens to satisfy the closure. **The
repair may have been specified at the width of the closure rather than at the
width of the defect** (the Architect's own words). The sharper question D1
answers first: **is arm 1's weaker discharge condition the defect, rather than
absent production?**

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
