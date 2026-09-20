---
id: LANG-STANDARD-OP-GENERIC-CARRIER
title: "Standard-operator completion over an ABSTRACT carrier. AC-6's fork was FALSE AS POSED -- `the generic where Ord a case` names TWO programs with two refusal sites and two owners. Row 1 (the `where`-clause sugar) refuses at DECLARATION time in `resolve_instance_dictionary_inner`, before A1's completion arm is reachable at all: pre-existing, by construction, NOT a bug. Row 2 (the explicit `(d : Ord a)` encoding) has its dictionary genuinely in scope and is refused only by A1's own head-identity match, whose arms are `Term::Const` and `Term::IndFormer` while a Pi-bound carrier infers to a de Bruijn variable: A1-caused, a real repair. D0 gates Row 2 and must run BEFORE any code -- its grounding is a July 2026 enclave probe against a September tree."
status: ready
owner: language
size: S
gate: none
tier: T1
depends_on: [LANG-STANDARD-INFIX-CALL-COMPLETION]
blocks: []
github: null
origin: "Architect ruling evt_2nbvmrwfv055y in thr_3kmhbpep3vj2j, discharging AC-6 of LANG-STANDARD-INFIX-CALL-COMPLETION; re-posted self-contained at evt_3hb7tn83pgj33. Summarised to the Steward by the language-leader at evt_51rtc22a3tm74. PROVENANCE OF THE ROW-3 AMBIGUITY: it originated in the ruling, which uses the ordinal Row 3 for two different partitions about forty lines apart; the summary quotes the second and is downstream of it, not its source (Architect, evt_52nsn0p1qr26d; Steward verified both strings in the ruling's own text). Steward-filed per COORDINATION section 2; constraint interrogated per steward.md section 4c. The Architect states nothing is owed back on it and that A1 needs no further ruling -- A1's remaining ACs (AC-0, AC-8) need measurement only."
---

> # CORRECTION 2026-09-19 — THE `origin:` FIELD NAMES AN "AC-0" THAT A1 DOES NOT HAVE
>
> The `origin:` field closes: *"A1's remaining ACs (AC-0, AC-8) need
> measurement only."* **`LANG-STANDARD-INFIX-CALL-COMPLETION` has no AC-0.**
>
> Measured 2026-09-19 at `42a090129`, over the flattened frame so the count
> cannot miss a wrapped occurrence:
>
>     AC-1 x1  AC-2 x2  AC-3 x2  AC-4 x1  AC-5 x1
>     AC-6 x3  AC-7 x2  AC-8 x5  AC-9 x3
>     AC-0: ZERO
>
> A1's frame prescribes close to the opposite of a final pre-handoff run: `-p
> ken-elaborator` is **the loop on that node, not the finale**, and its
> no-regression AC is workspace-green **in CI** (`COORDINATION §12`). A real
> `AC-0` does exist on the **B-track** — `LANG-ATOM-START-CLASSIFICATION-CLOSURE`,
> an Architect release amendment — and that is the likely source of the carry.
>
> **THE `origin:` FIELD IS DELIBERATELY NOT CORRECTED.** It records what was
> summarised to the Steward at filing. Rewriting it to read correctly would
> erase the belief it is a record of, and the provenance section below exists
> precisely because this node's history is evidence. **A wrong sentence in a
> record of belief is not the same object as a wrong sentence in a
> specification** — the first is corrected beside it, the second in place.
>
> **WHAT IT COST, so the correction is not read as bookkeeping.** A1's
> implementer carried "AC-0" in their checkpoint and their posts for a full
> session without opening the frame, and stopped one step short of spending a
> fifth full `-p ken-elaborator` run — roughly 45 minutes of a contended
> machine-wide lock — discharging an AC that does not exist. **The tell is
> cheap: an AC you cannot quote from the frame is an AC you have not read.**
>
> **THIS IS THE SECOND INHERITED-CLAIM DEFECT IN THIS ONE NODE**, which is why
> it sits here rather than in a thread. Below, an ordinal was inherited from a
> ruling that used it two ways. Here, a third node's frontmatter summarised a
> second node's criteria and the summary outlived the fact. **A correction is
> naturally scoped to the artifact being corrected, and the inheriting artifact
> is always outside that scope** (Architect, 2026-09-19) — so the sweep after a
> correction has to run on the citing artifacts, not only the corrected one.

# "ROW 3" NAMES TWO DIFFERENT THINGS IN THE RULING ITSELF

**One ordinal, two partitions, one document.** `evt_2nbvmrwfv055y` says both of
these, about forty lines apart:

    "Row 3 -- CONCRETE CARRIERS. Delivered, eight ACs."

    under the heading "A THIRD ROW FOR THAT NODE":
    "...Route it as Row 3 of the same node; do not fold it into A1."
    (the `RVarTy` spelling finding)

**Verified by the Steward by reading `evt_2nbvmrwfv055y` in
`thr_3kmhbpep3vj2j` directly — both strings are in the ruling's own text.**

⇒ **A reader who takes the second is quoting the ruling, not misreading it.**
The language-leader's summary (`evt_51rtc22a3tm74`) carries the second reading
for exactly that reason and is downstream of the ambiguity, not its source.
Architect, owning it: `evt_52nsn0p1qr26d`.

**THE DISPOSITION, which the Architect confirms:** this node carries CONCRETE
CARRIERS as Row 3. **The `RVarTy` finding is NOT a row here** — it is routed to
`[[LANG-INSTANCE-REGISTRY-IDENTITY-KEY]]` as its single home, and the Architect
dropped it from this node's slot (`evt_3hb7tn83pgj33`). **Do not re-add it. One
home, not two.**

> ### THE LESSON IS THE AUTHOR'S, NOT THE RELAY'S. AN EARLIER REVISION HAD IT WRONG.
>
> That revision was headed *"READ THE RULING, NOT THE SUMMARY"* and filed this
> as relay drift — a count that matches over a membership that moves. **That
> instruction could not have protected anyone: BOTH readings are in the
> ruling.** Reading it is where the ambiguity lives.
>
> **What actually resolved it was noticing a disagreement between two sources
> and going to the artifact to adjudicate, rather than picking one.** That is
> the transferable move, and it is not the same as distrusting relays.
>
> **The author-side rule, which is the one that prevents recurrence:** never
> reuse an ordinal for a second partition in the same document, and when a
> finding rides alongside a disposition list, **give it a name, not the next
> number.**

# WHICH TREE EACH ROW IS GROUNDED ON, AND WHY IT DECIDES THE ORDER

**The ruling's coordinates are all at `18f3452fcf15625d7e52b9a5eb101c3521117e2f`
— A1's CANDIDATE head, not `main`.** The Architect said so explicitly. The
Steward re-grounded both rows independently, and the two rows do not sit on the
same tree:

    ROW 1  grounded on BOTH.  Its whole mechanism is on main today.
    ROW 2  grounded ONLY on the candidate. Its refusal site DOES NOT EXIST
           on main, because A1 is unlanded.

**Measured by the Steward at `origin/main`
`d272e361731c01fd3b6796309c8bfb42d812ad3a`:**

    git grep -n 'StandardOperator\|standard_operators' \
      crates/ken-elaborator/src/*.rs      ->  ZERO HITS

⇒ **A1's completion path is not on `main` in any form.** `elab.rs:10772`, the
site Row 2 is entirely about, is a coordinate in a candidate branch.

> ### THE CONSEQUENCE, AND IT IS THE FIRST THING WHOEVER TAKES THIS WILL HIT.
>
> **`D0` CANNOT BE RUN AGAINST `main`.** The program it elaborates is refused by
> a match that `main` does not contain. Run it against A1's landed result, or
> against the candidate at a named SHA — and if you run it against the
> candidate, say which, because that tree is still moving.
>
> **A `D0` run against `main` returns REFUSES-EARLIER for a reason that has
> nothing to do with the question**, and it collapses Row 2 into Row 1 falsely.
> That is a legal reading on a query that could not hit, it agrees with one of
> the two pre-committed outcomes, and nothing in the output flags it.
>
> This is why `depends_on: [LANG-STANDARD-INFIX-CALL-COMPLETION]` is set. It is
> not a scheduling preference.

# ROW 1 -- THE SUGAR. PRE-EXISTING, AND NOT A BUG.

    fn f ... where Ord a          with `a` abstract

**Refused at DECLARATION time, before the body elaborates.** `RDeclKind::View`
calls `resolve_instance_dictionary` once per constraint, which delegates on
`rtype_head_name(&constraint.head_type)`. For `Ord a` the carrier head is the
type variable's OWN SPELLING, `"a"`, via `rtype_head_name`'s
`RVarTy(_, name, _) => name.clone()` arm. The registry holds no `("Ord", "a")`,
so `NoInstance` fires. `elaborate_view_or_let` runs AFTER that loop, with
`local_dicts` already resolved.

**VERIFIED VERBATIM by the Steward, at `main` `d272e361` AND at `18f3452fc`:**

    main d272e361                        candidate 18f3452fc
      elab.rs:9523  rtype_head_name        (same fn, RVarTy arm identical)
      elab.rs:9525    RVarTy(_, name, _) => name.clone(),
      elab.rs:9675  head_name = rtype_head_name(requested)
      elab.rs:9680    .ok_or_else(|| ElabError::NoInstance {   :9891
      elab.rs:11125 RDeclKind::View -> resolve_instance_dictionary   :11604

**The ruling's `:11604` and `:9891` are the candidate's numbers; `:11125` and
`:9680` are `main`'s. Same two sites, both trees, no conflict.**

**THE LOAD-BEARING CONSEQUENCE:** **A1's completion arm is UNREACHABLE for this
program.** The `NoInstance` that FI-3 is read as describing never runs. **A
perfect completion arm still leaves this program refused**, at a site A1 never
touched.

**DISPOSITION: PRE-EXISTING. Not caused by A1, not reachable from A1's surface,
and not a defect.** Implicit resolution needs a concrete registered head BY
CONSTRUCTION — that is the mechanism, not a failure of it.

> **Do not write a repair for Row 1 into this node.** Making `where Ord a` work
> over an abstract carrier is a language-design change to implicit resolution,
> not a fix. If someone wants it, it is a separate node with a spec argument in
> front of it.

# ROW 2 -- THE EXPLICIT ENCODING. A1-CAUSED, AND A REAL REPAIR.

    fn f (a : Type) (d : Ord a) (x : a) (y : a) : Bool = x ≤ y

**The declaration is carrier-agnostic BY CONSTRUCTION, and the elaborator
already has production machinery for it.** At `18f3452fc`:

    class_name_for_dictionary_type  takes rtype_head_name of the DICTIONARY
                                    type -- "Ord" -- and asks class_env.class.
                                    The CARRIER IS NEVER CONSULTED.
    collect_bound_dictionary_params walks the RPi telescope collecting these.
    projected_field_row_type        already gives an RVar base its class field
                                    row off `bound_dict_classes`.

**Re-measured by the Steward at `main` `d272e361` — all three are on `main`
independently of A1:** `class_name_for_dictionary_type` at `elab.rs:10187`,
`collect_bound_dictionary_params` at `:10192` (called at `:10332`), and the
`bound_dict_classes` lookup at `:10122`.

⇒ **The dictionary IS in scope.** The only thing refusing `x ≤ y` is A1's own
head-identity extraction.

**VERIFIED VERBATIM at `18f3452fc`, `elab.rs:10768-10778`:**

    let head_id = match &carrier {
        Term::Const { id, .. } => *id,
        Term::IndFormer { id, .. } => *id,
        _ => {
            return Err(ElabError::NoInstance {
                class: "Ord".to_string(),
                ...

`infer(cx, x)` for `x : a` yields a de Bruijn variable; `whnf` cannot reduce it;
it is neither arm, so it takes `_`.

> **One thing the Steward checked and it is NOT a finding: the hardcoded
> `class: "Ord"` in that error is CORRECT.** The arm sits inside
> `StandardOperatorRole::Leq | StandardOperatorRole::Geq` (`18f3452fc`,
> `elab.rs:10759`), so `Ord` is the only class it can be resolving. Recorded
> because the string looks like a defect when the arm is quoted without its
> enclosing match, and the next reader should not spend the lookup twice.

**DISPOSITION: A1-CAUSED. A1 authored that match.** It is a **REPAIR, not a new
capability** — the resolution wants nothing from the registry, it wants the
binder.

## Why Row 2 SPLITS rather than FOLDS into A1

**It is A1's own defect and this node does not pretend otherwise.** The reason
is ordering, not ownership:

The missing arm needs a **bound-dictionary channel into `ElabCtx`**, which today
exists only in the effect-row path, built per-decl from the surface type
(`18f3452fc`, `elab.rs:10563`). **That is a NEW `ElabCtx` field.** A1 already
added one this node, and **`[[LANG-SEAL2-GATE-INCRATE-RELOCATION]]` — the
producer-walk node that would AUDIT a new field — is queued and not released.**

⇒ **There is no field gate of any kind at this moment.** Adding a field while
its auditing gate is un-landed is the wrong order.

> **That is the ONLY reason Row 2 is not a fold, and it is a STATE, not a
> property of the work.** If the seal2 gate lands, re-ask the question rather
> than inheriting this paragraph — a deferral whose precondition has dissolved
> has lapsed, not been satisfied. This session has already had one fold argument
> expire exactly that way.

# ROW 3 -- CONCRETE CARRIERS. DELIVERED, AND NOT THIS NODE'S.

Delivered under A1's eight acceptance criteria. **Recorded here so the row set
is complete and so nobody reads this node as "generic carriers are unsupported"
when the concrete case is done.** No work here.

# D0 -- THE PROBE THAT MUST RUN BEFORE ANY CODE. BOTH OUTCOMES PRE-COMMITTED.

    Elaborate, on a tree where A1's completion path EXISTS (see the tree
    section above -- NOT main):

        fn f (a : Type) (d : Ord a) (x : a) (y : a) : Bool = x ≤ y

    REFUSES AT the head-identity match (18f3452fc: elab.rs:10772)
        -> ROW 2 STANDS as written. The repair is the missing arm plus the
           bound-dictionary field channel.

    REFUSES EARLIER
        -> ROW 2 COLLAPSES INTO ROW 1. The explicit encoding is unsupported
           too, the node is smaller and differently shaped.
           RE-FILE. Do NOT repair the earlier site.
           NAME THE REFUSAL SITE. "Earlier" is informative ONLY once you
           know it is not "no such match on this tree" -- see below.

**Report WHICH site refused, not merely that it refused.** The two outcomes are
distinguished only by the coordinate, and "it does not compile" is consistent
with both.

> ### THE PRE-COMMITMENT WAS INCOMPLETE, AND THAT HOLE IS WHY THE CLAUSE EXISTS.
>
> A two-outcome pre-commitment is only sound if **both outcomes are reachable
> only for the reasons named.** REFUSES-EARLIER was not: on `main` it is also
> what you get when A1's completion path is absent entirely, which is a fact
> about the tree rather than about the program. **So the instrument would have
> accepted a reading it could not have produced honestly**, and the wrong
> branch — re-file smaller — is the one it would have licensed.
>
> Two independent guards, both required: the tree check (A1's completion path
> shown PRESENT before the run counts) and the named site (which refusal
> fired). Architect, `evt_52nsn0p1qr26d`, on their own pre-commitment.

> ### WHY `D0` EXISTS RATHER THAN THE NODE JUST ASSERTING ROW 2.
>
> Row 2's premise is grounded by READING the producer at `18f3452fc` **plus an
> enclave probe from 2026-07-03** that returned `Ok` for `(d).leq x y` over an
> abstract `k` at `main`. **That is a July measurement cited for a September
> tree.** The read is current; the behavioural half is not. It is a hypothesis,
> and the probe is what converts it.
>
> The Architect flagged this in the ruling rather than being caught at it. Keep
> the flag attached to the claim — a stale measurement quoted without its date
> reads exactly like a fresh one.

# Acceptance: THE UNIT OF DISPOSITION IS THE ROW

**Every row gets a WRITTEN disposition. Not the node, not "the generic case."**

    AC-1  D0 is run and its refusing SITE is named at a stated SHA, with the
          tree it ran on identified and A1's completion path shown present
          on that tree. A run against a tree lacking it is not a D0 run.
    AC-2  Row 1 is recorded as pre-existing-and-not-a-bug, with the two
          coordinates re-grounded at the SHA the work lands against.
    AC-3  Row 2 is either REPAIRED (arm + field channel, with the field
          passing whatever producer gate exists at that time) or recorded
          as collapsed into Row 1 per D0's second branch.
    AC-4  Row 3 is recorded as delivered under A1, with the AC ids.

> ### THE AC THIS REPLACES, AND WHY THE OBVIOUS PHRASING FAILS.
>
> **"The generic case is delivered, or recorded as not delivered" PASSES on the
> exact mistake this node exists to prevent.** The failure at risk was never a
> silent outcome — it was recording ONE not-delivered row when there are two,
> one of which A1 caused and one of which it did not. A single-disposition AC is
> satisfied by naming either row and is blind to the other.
>
> **Make the unit of disposition the ROW.** Architect's wording, kept.

# NAMED, DELIBERATELY NOT RULED, AND NOT A ROW HERE

Carrier identity is established by **SPELLING** at every site except the one A1
just fixed:

    (i)    where-clause resolution   elab.rs:11604 @18f3452fc / :11125 @main
                                                              carrier SPELLING
    (ii)   instance_search           elab.rs:10090/:10101     carrier SPELLING
    (iii)  A1's completion path      FIXED                    carrier IDENTITY

**One predicate, not three findings.** The structural closure is a single
carrier-identity function with those three as its callers.

**This is a DESIGN node someone has to frame, and it is neither A1's nor this
node's.** It is written here so it does not arrive as a fourth row somewhere
(Architect, explicitly). **Whoever takes this node must not absorb it.** Site
(ii) is already carried by `[[LANG-INSTANCE-SEARCH-SECOND-PATH]]`; the
`RVarTy` instance of site (i) is carried by
`[[LANG-INSTANCE-REGISTRY-IDENTITY-KEY]]`. **The closure across all three has no
node and the Steward has not ruled that it needs one.**

# The constraint, interrogated

**Row 2 is grounded.** A declaration that binds a dictionary explicitly and is
then refused at its use site is a capability the elaborator demonstrably has and
does not reach — `collect_bound_dictionary_params` and the `bound_dict_classes`
lookup are both on `main` and both already do this work for effect rows. The
constraint is "a surface the implementation already supports should not be
refused by an incomplete match," which is a defect claim, not a preference.

**Row 1 is NOT a constraint and no deliverable may be derived from it.** "An
abstract carrier cannot resolve an implicit instance" is the mechanism working.

**NOT grounded, and must not be written in:** this is not a TCB argument.
`docs/PRINCIPLES.md` §5 places the elaborator outside the trust root — the
kernel re-checks. A refusal that should have been an acceptance is an
incompleteness, not an unsoundness, and it fails CLOSED. Nothing here is a
safety-of-`main` argument either; `main` does not carry A1 at all.

# Size and tier

**`S`, and it is `D0`'s to overturn** — sized for the REPAIRED branch: one match
arm plus one context field, against machinery that already exists. If `D0`
returns REFUSES-EARLIER the node is re-filed rather than resized.

**`T1`, not `T2`, despite being small.** The diff is two edits; the work is
deciding what the new `ElabCtx` field may hold and reading whether the bound
dictionary is the right identity to key on. That is a reasoning diff wearing a
mechanical one's size, and `steward.md` §4h sizes on the thinking, not the
lines.

# Contention

`[[LANG-SEAL2-GATE-INCRATE-RELOCATION]]` gates new `ElabEnv`/`ElabCtx` fields
and is queued-and-unreleased. **It does not block this node, and this node does
not block it** — but the ORDER matters (see Row 2 above), and whoever releases
both decides it deliberately rather than by arrival.

No file overlap with `[[LANG-R-LAYER-EXPORT-RETRACTION]]` or
`[[LANG-INSTANCE-REGISTRY-IDENTITY-KEY]]` — those are `classes.rs`, this is
`elab.rs`.

# Why this is `draft`

**QUEUED by priority, and additionally gated on `depends_on`.** L1 (clearing the
ignored tests) is the operator's top priority as of 2026-09-17; the language
lane's own objective is `LANG-MODULE-IMPORT-SYSTEM`. **Not released, and not to
be started until the Steward releases it.**

**It is additionally not startable yet on its own terms**, which is the rarer
reason: `D0` needs a tree carrying A1's completion path, and `main` has none.

# RELEASED 2026-09-20 (Steward), SCOPED TO `D0` AND THE ROW DISPOSITIONS

**Both reasons this node was `draft` are discharged, and one of them was
discharged by main moving rather than by anyone acting on it.**

**The tree gate is GONE, and it was a mutable-state claim.** This file said
*"`D0` needs a tree carrying A1's completion path, and `main` has none."*
Measured at `origin/main` `cf4533b74`: `LANG-STANDARD-INFIX-CALL-COMPLETION`
is `merged`, landed at `e2e40e2b4` and closed at `3e15a7df4`, and
`collect_bound_dictionary_params` plus the `bound_dict_classes` channel are
both in `crates/ken-elaborator/src/elab.rs` on `main`. **`main` IS now a
qualifying `D0` tree.** That sentence was true when written and became false
without anything editing it, which is the whole reason the frame demands the
tree be shown present at the run rather than argued from this paragraph.

**The priority gate does not apply.** It read *"queued by priority, L1 is the
operator's top priority."* L1 and L2 are separate authorized lanes running
concurrently under the three-lane roster; L2 does not queue behind L1. L2 has
no live operator objective and works its backlog under the campaign root's
standing surface-syntax direction.

## SCOPE AT RELEASE: `D0` plus AC-1, AC-2, AC-4. **AC-3's REPAIR is DEFERRED.**

**Do not add a field to `ElabCtx` under this release.** Row 2's repair needs a
bound-dictionary channel into `ElabCtx`, and this frame's own ordering rule
governs: the node that would AUDIT a new field,
`[[LANG-SEAL2-GATE-INCRATE-RELOCATION]]`, is still `draft` and unreleased at
`cf4533b74`. **I re-asked the question rather than inheriting the paragraph, as
the frame instructs, and the precondition has NOT dissolved.** There is still no
field gate, so adding a field is still the wrong order.

**AC-3 is therefore discharged in this release ONLY by its second branch** --
recording Row 2 as collapsed into Row 1, if that is what `D0` returns. If `D0`
returns REFUSES-AT-the-head-identity-match, Row 2 STANDS and is recorded as
standing-and-deferred behind the field gate. **Recording it is the deliverable;
repairing it is not, and a repair folded in anyway is a Steward stop.**

**This ordering is not a delay tactic and `D0` is why.** `D0` can collapse Row 2
outright, in which case the field question never needed answering. Running the
probe before touching the gate is the cheap order, not the cautious one.

## A CROSS-REFERENCE IN THIS FILE IS NOW STALE. Site (ii) is NOT carried.

The "NAMED, DELIBERATELY NOT RULED" section says site (ii), the `instance_search`
carrier-spelling reads, is *"already carried by
`[[LANG-INSTANCE-SEARCH-SECOND-PATH]]`."* **That is no longer true.** That node
closed 2026-09-20 with the residual MEASURED AND UNREPAIRED: identity-grade
carrier material is absent from `ProjectionPurityCtx`, and supplying it needs
either kernel-state inference at the read or a new stored identity field, both
of which the Architect forbade there. Site (ii) is a recorded residual with no
owner, not an obligation someone else is discharging.

**This changes nothing about this node's scope and must not be absorbed into
it.** It is corrected here so the next reader does not treat site (ii) as
handled. The three-site closure across (i), (ii), and (iii) still has no node
and I have still not ruled that it needs one.

## Stop conditions at release

- **`D0` cannot be run because the tree lacks A1's completion path.** Do not
  work around it and do not run it anyway -- a run against such a tree is not a
  `D0` run by this frame's own guard. Name the tree and stop.
- **The repair looks foldable without a new `ElabCtx` field.** That contradicts
  the frame's measured premise. Report the mechanism and stop; do not build it.
- **Anything wants a fourth row.** The row set is closed at three.
