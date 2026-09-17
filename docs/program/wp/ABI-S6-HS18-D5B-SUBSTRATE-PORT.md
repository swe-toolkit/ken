# WP frame — `ABI-S6-HS18-D5B-SUBSTRATE-PORT`

    owner   runtime        tier   T2        size   L
    blocks  ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION
            ABI-S6-HS18-MAIN-BASED-CLOSURE
    node    docs/program/issues/ABI-S6-HS18-D5B-SUBSTRATE-PORT.md

> ## RECUT 2026-09-17 (SECOND). THIS NODE IS NOW THE MECHANICAL HALF ONLY.
>
> The Architect re-ruled this WP **(c) mis-sized** (`evt_7tx7a1n71qa9g`) and
> asked the Steward to recut around **replacing the completion criterion**. The
> recut is `§3b`, and it does two things: it splits the judgment surface out
> into [[ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION]], and it replaces `AC-1`.
>
> **`§3b` is the operative text for scope, sizing and acceptance.** Everything
> above it that states a measurement — the regions, the extent, the six-site
> grant exclusion, `§3`'s reaching-consumer predicate — is **retained
> unchanged** and is still binding. A recut is not a licence to restart.

## 1. Objective

Land the **mechanically determined** part of the D5b prefix's production
residue on `main`: the methods `main` has not touched since the common base,
and the names `main` does not have at all. **Every method where both the port
source and `main` moved is OUT of this node** — it goes to
[[ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION]], and leaving it at `main`'s body is
compile-safe by construction (`§3b`).

**Exclude the refused `MappingAcquireFile` grant by construction, and do not
absorb increment A's own substance.**

## 2. Fixed inputs — measured, with the ref each was taken at

Every number below was taken by the Architect at prefix tip `30d35f625` against
`origin/main` `6036f9f5d` with base `2a74775ae` (`evt_6fr79bx4cjhsw`), except
where the Steward is named. **Re-measure before relying on any of it** —
`main` moves, and `§2d` gives you the predicate to re-run.

### 2a. The refuted premise, so it is not re-derived

`ABI-S6-HS18-MAIN-BASED-CLOSURE` `§2a` sized increment A as *"9 files,
`units.rs` +4083"*, a diff between two points **on the preserved line**. It
measures what A added to the checkpoint, never what A needs on `main`.

**Steward-verified independently** at `origin/main` `6036f9f5d`:

    b4c8df33a "ABI-S6: fold HS18 closure mechanism amendment 4"  IS AN ANCESTOR OF main
    lowering/{units,calls,source,effects}.rs   b4c8df33a vs main   ALL BYTE-IDENTICAL

⇒ **The prefix's changes to those files are ABSENT from `main`, not re-derived
into a different shape.** Those two readings are both consistent with *"main has
neither"*, and only one is fixable by landing something.

### 2b. The three regions — the scope-shaping fact

    REGION 1  ALREADY LANDED    5 files   ken-host/{mapping_v1,lib,abi_v1}.rs,
                                          ken-elaborator/{prelude,compiler_driver}.rs
                                          EXCLUDE -- re-landing is a no-op at best,
                                          a REVERT at worst
    REGION 2  CLEANLY ABSENT   13 files   main sits at the prefix base; the diff
                                          REPLAYS. Includes units.rs, calls.rs,
                                          source.rs, effects.rs, closure.rs,
                                          occurrences.rs, cranelift_backend.rs,
                                          ken-runtime/src/lib.rs, the catalog,
                                          ken-host/Cargo.toml
    REGION 3  DIVERGED         16 files   main moved on its own; the prefix does
                                          NOT replay. RECONCILE, do not replay.

**Region 1's exclusion rests on a vacuity control, not on the identity alone.**
*"Identical to `main`"* is also what you get when a prefix net-changed nothing.
For all five, prefix-**base** content DIFFERS from prefix-**tip** content, so
`main` matches the **changed** form. Steward reproduced this on
`mapping_v1.rs` and `prelude.rs`.

**Region 3 is the half `absent` cannot express.** On `immediate_bridge.rs`,
`main` built a **larger** version of the same new module independently:

    immediate_bridge.rs   main +1510/-0 from base    prefix +622/-0
    responses.rs          main +1210/-4              prefix +1074/-96

### 2c. The extent: ~117 items, not `+10472/-5307`

Prefix-minus-main by item inventory. The full table is in the node; the shape
is what matters here: **12 of the diverged files are FULLY SUBSUMED (zero
missing), including `immediate_bridge.rs` at 34 of 34.** The residue
concentrates in `lowering/mod.rs` (39), the absent acceptance test
`abi_s6_mapping_file_backed_native.rs` (24 of 24, file absent entirely),
`lowering/core.rs` (10) and `lowering/calls.rs` (9).

### 2d. ITEM NAMES ARE A PROXY FOR BEHAVIOUR. Cite by symbol; re-run the predicate.

A same-named function can have a different body. `§2c` bounds the **extent** —
it does not prove item-by-item equivalence, and nobody should read it as having
done so.

```sh
items(){ git show "$1:$2" | grep -oE "^\s*(pub(\([^)]*\))? )?(fn|struct|enum|const|type) [A-Za-z0-9_]+" | awk '{print $NF}' | sort -u; }
comm -23 <(items 30d35f625 "$f") <(items origin/main "$f")
```

`crates/ken-runtime/src/cranelift_backend/**` moves under active work. **Every
coordinate in this frame is a symbol name, and yours should be too.**

**THE PREDICATE IS ONLY DEFINED OVER FILES THAT CONTAIN COUNTABLE ITEMS, and it
does not say so itself — it AGREES instead.** On a manifest, a `.toml`, a doc
file or anything else with no `fn`/`struct`/`enum`/`const`/`type`, both sides of
the `comm` are empty and the empty result reads as **nothing missing**. That
fired on `crates/ken-runtime/Cargo.toml`, which was reported SUBSUMED while
holding a different feature line (`§3a`). **Restrict the file set before running
it, or the favourable answer is structural.**

### 2e. THE GRANT IS SIX SITES, NOT FIVE — and this node is NOT an enabler

Amended 2026-09-17, after three seats measured this independently. **Two things
this frame previously said are wrong.**

**1. The census of five was short.** The sixth site is
`CRANELIFT_HOST_EFFECT_CONSUMERS_V1` membership plus the matching removal from
the named-unavailable-lanes arm, both in
`planning/static_transition/effects.rs`, inside a **Region 2** file:

    roster contains MappingAcquireFile   origin/main  0     30d35f625  1
    represented-unavailable lane count   origin/main 10     30d35f625  9

**That is the site that actually operates the gate**, and the removal is a pure
deletion with **no added line to grep for** — site 3's signature in the planner
plane instead of the host plane. There is a seventh touch in `st/aggregates.rs`.
Found by the runtime-implementer before applying anything (`evt_316yppb8r4zxa`);
the Architect confirmed and amended their own ruling (`evt_1jfpng6yvy89v`).

⇒ **The roster ADMITS and the dispatch HANDLES; excluding either alone creates a
panic.** Land the roster without site 5's dispatch arm and the op passes the gate
at `:2812`, falls through `match operation` at `:3076`, and hits
`unreachable!()` at `:3713`.

⇒ **Exclude whole files rather than hand-separating hunks WHEREVER THE ONLY
BOUNDARY AVAILABLE IS YOUR JUDGEMENT**, as the node mandates for site 5. **That
is the rule's reason and its limit — see the `effect_v1.rs` split below, where
a compiler-drawn boundary
licenses a split and whole-file exclusion would have broken the build.**
**Treat six as a floor and close it with the
parent's `AC-PREDICATE` — zero diff lines naming the operation at pathspec
`crates/` — which catches all seven touches without anyone holding a complete
site list.** Measured: Region 2's 13 files carry 17 such lines; the 10 files
that remain after excluding the catalog, `lowering/effects.rs` and
`st/effects.rs` carry **zero**.

**THE `ken-host/src/effect_v1.rs` SPLIT, and why whole-file exclusion is
REFUSED.** Ruled by the Architect (`evt_2jzjsj3nhn2qm`), Steward-verified at
`origin/main` and `30d35f625`:

    ken-runtime/src/lib.rs   pub use ken_host::{with_d5b_file_source_admission_mutation,
                                                D5bFileSourceAdmissionMutation}
      origin/main   0 occurrences        30d35f625   1

`lib.rs` is a **Region 2 file this port replays** (`+9/-8`), and its replay adds
that re-export. **Exclude `effect_v1.rs` whole and Region 2 re-exports two
symbols that do not exist — `ken-runtime` does not compile.** A tree that does
not build produces one error and **no test results at all**, so whole-file
exclusion does not yield a node carrying honest reds; it yields a node that
**cannot report anything**, defeating the instrument this port exists to run.

**Site 5's whole-file rule does not transfer, and the reason is why.** That rule
exists because hand-picking 8 of 42 lines draws a boundary a reviewer must
**trust**. Here the boundary is drawn by the compiler:

    effect_v1.rs:1072 .. :1116   #[cfg(any(test, feature = "px8-ds-test-support"))]
      brackets exactly: the enum, two thread_locals, the Drop guard,
      and with_d5b_file_source_admission_mutation

**A `cfg` attribute is not judgement** — one line checks it. **LAND that block.**

**THE GRANT'S FOOTPRINT IN THIS FILE IS BIGGER THAN SITES 2/3/4.**
Steward-verified:

    effect_v1.rs:1118  fn mapping_acquire_file_source_rights   NOT cfg-gated
    effect_v1.rs:3263  called INSIDE the op's own match arm,
                       (HostOpV1::MappingAcquireFile, CanonicalRequestV1::MappingAcquireFile {..})
      origin/main  0 occurrences (whole file)      30d35f625  2

It computes how the promoted op resolves its source handle ⇒ by the boundary
criterion (*a site belongs to the grant if its presence is required for the
promoted op to function as promoted*) it is the **paradigm case, not a near
miss**. **EXCLUDE it and its `:3263` call site.** This is the third time the
grant census has grown — **six is a floor, and so is this.**

⇒ Within `effect_v1.rs`: **LAND** the cfg-gated instrument block; **EXCLUDE**
sites 2/3/4, `mapping_acquire_file_source_rights`, and `:3263`.

**2. This node does NOT unblock `RT-D5B-POSTCALL-REFUSAL-MECHANISM`.** The
Steward's sequencing ruling claimed it would give that node *"a reproduction on
main for the first time."* **Refuted by reading** (`evt_30p9m0j8bj1f2`),
measured at `origin/main` `b0eb29e71` and `30d35f625`:

    lowering/effects.rs :2812   if !CRANELIFT_HOST_EFFECT_CONSUMERS_V1.contains(&operation)
                                   { return Err(unsupported(...)) }   -- roster membership,
                                   NOT a RepresentedUnavailable test; the message says
                                   "represented unavailable lane" and the predicate does not

    abi_s6_mapping_file_backed_native.rs   13 #[test] fns
      raw strings in the whole file         exactly ONE   const SOURCE  :11 .. :106
      build/run call sites                          15
        passing SOURCE                              15
        passing anything else                        0
      SOURCE :67   (withMapping ... (FileBacked file (8 : Int)) ReadWrite ...)

**Thirteen tests, one program, and that program acquires a file-backed
mapping.** With the grant excluded the op is out of the roster, so **every test
is refused at `:2812` before lowering** and the refusal is unreachable.

**The node still proceeds** — the sequencing ruling's other leg (that node is
not workable on `main` today regardless, its site and evidence both absent)
is measured and stands on its own. What is withdrawn is the *benefit* claimed,
not the *disposition*. `RT-D5B-POSTCALL-REFUSAL-MECHANISM` now depends on
[[RT-D5B-MAPPING-AVAILABILITY-FLIP]].

**Do not run a dedicated experiment to confirm this.** Both arms are predicted
from the producer above, and this node's own AC-1/AC-4 build reports the answer
for free (§12). **The refuting observations are named and cheap to spot: any red
carrying the `CheckedIhDetachedCallerCut` Packaging reason, or a `:3713` panic.**
Either means the gate was read wrong — **say so loudly**; it returns to the
Steward.

## 3. THE DESIGN QUESTION, already ruled: which evidence instruments travel

Most of the ~117 items are evidence instruments, not production — `D5bHs*
Mutation`/`Observation` types, `with_*_mutation`, `record_*_observation`,
`d5b_hs*_*` helpers. The Architect named the fork rather than inheriting it:
instruments for HS series `main` may already have closed are **cost without
cover**; instruments covering the production cluster are **what make it
reviewable**.

⇒ **RULED (Steward): an instrument travels if and only if it has a REACHING
CONSUMER among the production items this node lands.** Not a name pattern, not
"its HS series is still open".

**Why a predicate and not a list.** The parent node's `§4a-pin` already carries
the reason: *"TAKE THE PREDICATE, NOT THE COUNT. Every enumeration of this has
been short."* It was enumerated twice there and came up short both times. **A
list cannot report being incomplete.**

## 3a. THE RECUT, 2026-09-17: ACCEPTANCE EVIDENCE IS A PROPERTY OF THE TREE

**Authored by the Steward on the Architect's named predicate**
(`evt_5htrzz3p2f6pb`), discharging `§11`'s predicate check at row 12. Per
`steward/escalation.md`, a named predicate is the Architect saying the
**representation** is the defect, and the recut is the Steward's to author.
**It is small by construction: it changes what `§4` spends effort on and how
`§6` is phrased. It does not change the mechanism, the region split, the
six-site exclusion, or anything already proved.**

### The predicate the twelve rows share

The twelve entries are not twelve findings. Read by **what the instrument's
UNIT was**, they are one finding re-derived at four strengths:

| rung | the question | rows | what can answer it |
|---|---|---|---|
| name | does the item exist | 4 | the census |
| shape | what KIND of thing is it | 5-8, 10 | reading |
| reachability | is it visible to a consumer | 11 | a CONSUMER'S BUILD |
| compilability | do its dependencies resolve | 12 | the RESULT compiling |

> **ITEM-COMPLETENESS DOES NOT IMPLY TREE-EQUIVALENCE.** The port's obligation
> is stated over a SET OF ITEMS; every property that has actually failed is a
> property of the RESULTING TREE. An item-keyed instrument cannot see any of
> them, by construction — and widening it (the seed of 5 closing to 29) makes it
> **complete without re-aiming it**.

**The predicate is UNRETIRED, not new**, and that is the part worth carrying. It
is the one already ruled against the grant — *"membership in a plan is not
evidence of native availability"* — generalised: **membership in the port's item
list is not evidence that the item works in the tree.** Ruled once, then
re-derived one property at a time across four hard stops.

### The closure, phrased so a reviewer can run it

> **A port's ACCEPTANCE EVIDENCE must be a property of the resulting tree. An
> item census is a WORK PLAN and carries no evidential weight.**

⇒ **Derive the list FROM the tree property; never validate the tree property
against the list.** The candidate's 18 compiler errors naming Region 3 items is
the closure already working by accident — a compiler-generated worklist is a
tree property that yields the enumeration as a by-product.

**What this does NOT say.** Not that the census was wasted: you cannot port
without knowing what to move. **The list is needed to ACT; the defect is using
it as EVIDENCE.** Two jobs, one artifact, and the chain has been running both.

### The closure's first confirmation, found the day it was written

`runtime-implementer`, `evt_662hds2t1j7g1`. The item predicate reported
`crates/ken-runtime/Cargo.toml` **SUBSUMED** — while that file held a genuinely
different feature line.

**It was not a bug in the predicate. The file contains zero items of the kinds
the predicate counts, so it compared two empty sets and returned the favourable
answer.** A manifest has no `fn`, `struct`, `enum`, `const` or `type`, and
`comm -23` over two empty lists is empty, which the instrument reads as nothing
missing.

⇒ **A vacuous comparison does not fail; it AGREES.** The rung below reachability
failing silently in the direction that says "proceed" — and the build is what
caught it. **This is the closure's own prediction firing on the day it was
written, by an instrument aimed at the tree rather than the list.**

**Pin for anyone re-running `§2d`'s predicate:** it is scoped to files that
**contain countable items**. Run it over a manifest, a `.toml`, a doc file or a
generated file and its silence is structural, not evidential.

### What the recut RETAINS, REPLACES, and does NOT freeze

**RETAINS — everything already proved.** A named predicate is not a licence to
restart. The three regions (`§2b`), the ~117-item extent (`§2c`), the six-site
grant exclusion (`§2e`), `§3`'s reaching-consumer predicate, and the candidate's
closed default-config compile gate all stand unchanged.

**REPLACES — only the evidential standing of the item list.** `§4`'s D0-2 and
`§6`'s AC-5 are re-aimed below. **No AC is added and none is removed** — the
Architect's reading is that the defect was never a missing AC.

**DOES NOT FREEZE THE CHAIN COUNT — a departure from the recut shape in
`steward/escalation.md`, stated rather than done silently.** That shape says
*"freeze the old chain's count and open a fresh one"*, and its reason is that a
changed representation of the **work** makes the old stops count against
something superseded. **Here the predicate names the representation of the
EVIDENCE; the mechanism chain is unchanged and continues.** The count is also
not the Steward's to move: `escalation.md` duty 1 is explicit that the Architect
holds it alone and *"their own record stands."*

**THE CATEGORY ARGUMENT ABOVE IS NOT THE STRONGEST ONE, and the stronger one is
what should be copied** (Architect `evt_7v1j0d8zw4qe8`, interrogating the call
rather than ratifying it). The case *for* freezing is real: if the closure works,
the stops it explains were caused by a defect now fixed, so continuing to count
means a later advisory fires on a question already answered.

⇒ **What defeats it is COST ASYMMETRY, not category.** The hard-stop trigger is
a safety net: firing it early costs one research pull; not firing it when needed
is what let an earlier chain reach ten. **And the decisive half — the closure is
UNTESTED.** Zeroing a counter on the assumption that a just-authored closure will
hold is *"one more round will crack it"* wearing bookkeeping clothes, and it
would zero the counter **immediately before the stops that would most need the
trigger.** If the closure holds, the count simply never reaches its next
threshold — **that is the outcome working, not the counter being wrong.**

**Two counters, and they are not the same number.** The advancing hard-stop count
that drives the research cadence stands at **16, delta 0, next trigger at 18**.
The inventory's predicate check is keyed on `§11` rows, was discharged at row 12,
and falls next **three appended entries after that discharge** — see `§11`, where
striking the unrecoverable rows makes an ordinal schedule unsafe.

## 3b. THE SECOND RECUT, 2026-09-17: A COMPLETION CRITERION, AND A SPLIT

**Authored by the Steward on the Architect's re-rule to (c) mis-sized**
(`evt_7tx7a1n71qa9g`). The Architect's own statement of the defect is the one
to keep: **the WP has no criterion that can report its own completion.**
`AC-1` was *"increment A compiles against this"*, and compiler-clean answers
*"does every name resolve"*, which is a different question from *"is the port
done"*.

### The instrument: a three-way per method, against a common ancestor

A common ancestor exists for the port source and the main line —
`b4c8df33add9a2d260a3733341485b474ff04f5a`, Steward-verified an ancestor of
`origin/main`. That makes owed-versus-divergent **mechanically separable**
instead of a reading exercise:

    B = body at b4c8df33a   (last point the source and the main line agreed)
    S = body at b601e2ec7898 (port source)
    M = body at origin/main  (the tree this port must land on)

    S != B  and  M == B   ->  OWED        the source moved it; main never did
    S == B  and  M != B   ->  MAIN'S      main moved it; not owed
    S != B  and  M != B   ->  ADJUDICATE  both moved
    S == B  and  M == B   ->  AGREE
    present in S, absent from M   ->  ABSENT

**It produces a worklist, not a build result**, and it can be re-run at any
commit to report how much is left. That is what makes it a completion
criterion rather than a progress anecdote.

### THE THIRD TREE IS `origin/main`, NEVER THE PORT BRANCH. This is not a detail.

The re-rule specified the third tree as `879f00c994c6`, **the port branch**.
Run that way the instrument conflates two different facts: *"`main` moved this
method"* and *"the implementer already ported this method onto the branch"*.
Both read as `M != B`.

**Steward-measured, one variable changed, same extractor on both runs:**

    third tree = port branch 879f00c99    ADJUDICATE  154
    third tree = origin/main              ADJUDICATE   32

⇒ **A five-fold inflation of the single number the sizing turns on**, in the
direction that makes the node look far bigger than it is. The control that
proves it is `units.rs`: `main` is **byte-identical to the base** there
(Steward-verified), so by definition *no* method in it can be "both moved" —
yet the branch-based run puts 25 of its methods in ADJUDICATE. Against
`origin/main` it has **zero**, and the remaining ADJUDICATE rows concentrate
in `planning/static_transition/**`, which is exactly the diverged Region 3.

**Do not read this as overturning the Architect's census.** His counts were
taken with his own extractor and are re-derivable; mine uses a different one
and reports different totals throughout (my AGREE is 2601 against his 2172), so
**the absolute numbers are not comparable and I am not replacing them.** The
claim here is the controlled one: *within one extractor, changing only the
third tree moves ADJUDICATE by 5x.* His **20** and my **32** are the same
finding; the branch-based **154** is an artifact.

### POPULATION FROM THE UNION, AND THE INSTRUMENT MUST SAY SO IN ITS OUTPUT

**This is a requirement on the instrument, not a caution.** Every instrument
this port has used so far defined its population by what both trees already
have, and **a comparison restricted to the intersection is structurally blind
to the names present in only one of them.** That is three occurrences of one
predicate on this node: the Architect's intersection-only body diff, the
implementer's *"of 94 methods present in both trees"*, and `§2d`'s item
predicate agreeing about a manifest that holds no countable items.

⇒ The instrument's population is the **union of method names across all three
trees**, and its output must **print the population and the file count it was
taken over**. An instrument that does not state its own population cannot be
audited for this failure, and this one has now exhibited it three times.

### The split, and why deferring ADJUDICATE is COMPILE-SAFE

**ADJUDICATE methods exist in `main` already** — that is what `M != B` means.
Leaving one at `main`'s body therefore resolves, links and builds; the question
it raises is *which body is correct*, never *does the tree stand up*. So the
judgment surface can be deferred to a second node without leaving a broken tree
behind, and that is the whole basis for splitting here rather than anywhere
else.

    THIS NODE              OWED, ABSENT, MAIN'S      mechanical      T2
    ADJUDICATION NODE      ADJUDICATE                judgment        T1

**The default for every ADJUDICATE method in this node is `main`'s body,
untouched.** Not the source's. The Architect's own falsifier is why:
`dispatch_host_op_v1` scored +770 lines in that bucket and is **not owed at
all** — `main` refactored it into `dispatch_host_op_v1_for_promotion_evidence`
after the base, and the source carries the pre-refactor shape. Transplanting it
would undo `main`'s work. **Take `main`'s body and record the name; do not
transplant into this bucket.**

### THE BOUNDARY BETWEEN THE TWO NODES IS A PREDICATE, NOT A LIST

**Any method this node cannot place mechanically moves to the adjudication
node.** The implementer does not have to decide anything to hand it over —
naming it is the whole act.

⇒ **A growing adjudication list is this split WORKING, not a re-scope.** The
T2 seat is never required to exercise judgment it was not provisioned for, and
the only way this node fails is if someone forces a call rather than moving the
row. This is the same shape as `§3`'s reaching-consumer predicate and for the
same stated reason: **a list cannot report being incomplete.**

### PARTITION BY REGION BEFORE SIZING ANYTHING

Sizing a transplant list without first asking which region each item sits in is
**how this node got mis-sized the first time.** Re-derived by the Steward at
today's `origin/main`, over every file the source moved from base:

    R1  already landed (main == source)      5 files   EXCLUDE
    R2  replays        (main == base)       14 files   file-level checkout
    R3  diverged       (both moved)         16 files   per-method transplant

**The frame's `§2b` says 13 for Region 2; it is 14.** One more file qualifies
now than when that was written.

**An OWED method inside an R2 file rides the file-level replay and is not an
individual transplant.** That is what collapses the largest single bucket:
`units.rs` is R2, and the ~57 names absent from `main` there close as one
checkout rather than as 57 placements.

⇒ **The unit of work in R2 is the FILE. The unit of work in R3 is the METHOD.**

**What the replay does NOT settle, so nobody inherits more than was measured:**
file-level replay is **conflict-freedom, not compilation**. An R2 file calling
into an unreconciled R3 neighbour still fails to build. `§2d`'s
compile-dependency keying already says this.

### PRODUCTION AND TEST FUNCTIONS ARE DIFFERENT OBLIGATIONS

The Architect flagged this as unmeasured: *"I have not distinguished production
functions from `#[test]` functions... The counts above are of `fn` definitions,
not of obligations."* Steward-measured, and it is material to sizing — roughly
**two fifths** of the raw `fn` population across the mechanical buckets is test
functions.

**Report the split in the instrument's output.** A test function that fails to
port is an acceptance-evidence question and routes to `AC-4`'s
report-don't-repair discipline; a production function that fails to port is
port debt. **Counting them together produces a number that answers neither.**

### The HS10-inline guard, folded in as a FIXED INPUT and explicitly NOT a fix

`runtime-implementer`, `evt_50ra77wgfzeh9`: the assertion in
`with_d5b_hs10_inline_response_mutation` that *"HS10 inline-response mutations
cannot nest"* is **intra-family**. The duplicating mutations
(`DuplicateResponseOnlyDemand`, `DuplicateProducerKRow`) live in a different
family behind a different `thread_local` and a different guard, and **nothing
prevents cross-family co-activation.**

⇒ **A guard that cannot fail in the only configuration it runs in is not a
guard.** It holds under HS10-inline alone, which is the only configuration its
own assert ever runs in — which is precisely why nobody would catch it.

**This node LANDS that instrument with its limitation RECORDED beside it, and
does NOT repair it.** Widening the assertion across families is a change to a
guarantee, needs its own decision, and is not a port. **Record it; do not fix
it; do not silently land it as though the wording were accurate.**

### What rides in unchanged, and is NOT to be re-derived

Named by the Architect as retained, and confirmed:

- the `D0-2` re-aim (`§4`)
- the instrument work already landed
- entry 13's four dead instruments (`§11`)
- the HS10-inline transplant and counter-split rulings

**And from the Steward's own measurement:** `units.rs` is inside the pinned
scope and is Region 2 — that question was named by the Architect as the first
one to settle and it is settled, in the direction that makes this node smaller.

### The counter, recorded not moved

`§3a` states the advancing hard-stop count as **16, delta 0, next trigger at
18**. The re-rule moved it to **17, delta +1** (`evt_7tx7a1n71qa9g`), an
advancing stop: the implementer built the previous ruling, hit a genuinely new
structural wall, and needed a new one. **Next trigger at 18, so no research
hold fires on this recut.** Recorded here because it is stale above; the count
is the Architect's and is not the Steward's to move.

## 4. D0 — answer before writing production. A hard stop here is a GOOD outcome.

**D0-1. ANSWERED AT FILE LEVEL BY THE STEWARD — do not re-derive the blob
comparison.** `main` sits byte-identical to the base for all **14** Region 2
files at today's `origin/main`, so `diff(base, source)` applied to `main` is a
file-level checkout and **cannot conflict**. Region 2 replays.

⇒ **What remains open is the half the blob check cannot reach: does the
replayed tree BUILD?** Replay is conflict-freedom, not compilation, and an R2
file calling an unreconciled R3 neighbour still fails. **That is a build
question and `AC-1a` is where it is answered** — not a reading exercise, and
not a reason to re-run the blob comparison. If a Region 2 file turns out not to
replay after all, say which and stop: that converts it to Region 3 and is a
re-scope for the Steward.

**D0-2. For Region 3, is reconciliation bounded per file? RE-AIMED BY `§3a` —
do NOT pre-clear files by reading.** The original phrasing asked for a per-file
subsumes-or-diverges verdict established **before** the tree exists. That is the
rung the predicate names: rows 5-8 and 10 are all shape changes a reading
audit was supposed to catch and **found only after** they had been read past
once. **Twelve of sixteen "fully subsumed" is an item-list claim and carries no
evidential weight.**

⇒ **Apply, build, and let the compiler produce the divergence worklist.** The
bound you report is the one the tree gives you — errors naming Region 3 items,
counted and named, driven to zero. **A count that GROWS mid-run is the method
working**, exactly as AC-1 already says. Reading stays in scope for
**interpreting** a red the build raised; it is no longer the instrument that
clears a file.

**D0-3. Does the checked-IH cluster collide with
[[RT-D5B-POSTCALL-REFUSAL-MECHANISM]]?** That node is `ready` and its subject
is the `CheckedIhDetachedCallerCut` refusal — the mechanism is **UNKNOWN** and
that node exists to find it. This node lands `bind_checked_ih_detached_caller_cut`
and siblings. **If landing them would fix or obscure the refusal that node is
chartered to diagnose, stop and say so** — that is a sequencing call and it is
the Steward's.

## 5. Deliverables

1. **The three-way instrument itself, committed with the candidate** (`§3b`).
   It is the completion criterion, so it must live in the tree rather than in a
   thread or in this frame — a criterion nobody can re-run is not one. Region
   2's replay, Region 3's per-method transplants, and the placement of absent
   names are all driven from its worklist.
2. The production cluster from the node's `§What is IN`, landed on `main`:
   generated-context-result authority; checked-IH post-call/detached;
   recursive-position calls; source dynamic match — **minus every ADJUDICATE
   method, which is [[ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION]]'s.**
3. `crates/ken-cli/tests/abi_s6_mapping_file_backed_native.rs`, currently absent
   in full. **Note the path: `ken-cli/tests/`, not `ken-runtime/tests/`** — this
   frame and the node both said `ken-runtime` until 2026-09-17. **Its expected
   state is not assumed green — see AC-4.**
4. The evidence instruments that pass `§3`'s reaching-consumer predicate, with
   the **excluded** ones named and the reason given, and with the HS10-inline
   cross-family limitation recorded per `§3b`.
5. Whatever `lowering/mod.rs` re-exports the above require (the largest single
   residue after `units.rs`).
6. **The ADJUDICATE worklist, handed over by name and file** — the input the
   adjudication node is framed against.

## 6. Acceptance

**AC-1 — THE THREE-WAY WORKLIST IS DRIVEN TO ZERO ON THE MECHANICAL BUCKETS.**
The satisfying act is **re-running the `§3b` instrument at the candidate**,
with `origin/main` as the third tree, and its output showing:

    OWED         0
    ABSENT       0
    ADJUDICATE   enumerated by name and file, non-zero, handed to the
                 adjudication node
    population   stated in the output: union of method names, over the
                 file set, with the production/test split

**This replaces *"increment A compiles against this"* as the completion
criterion.** A build answers *"does every name resolve"*; this answers *"is
anything still owed"*, which is the node's actual contract.

**POSITIVE CONTROL, REQUIRED, AND IT IS THE HALF THAT USUALLY GETS SKIPPED.**
Run the identical instrument at the candidate's **base** in the same report. It
must show OWED and ABSENT **non-zero**. A zero from a real port and a zero from
an instrument whose file set matched nothing **print the same word**, and this
node has already been bitten three times by exactly that — `ci-doc-only.py`'s
bare `except`, `unique demands=0` on an empty set, and the item predicate
agreeing about a manifest with nothing to count. **A green AC-1 with no base
reading beside it is not evidence.**

**A COUNT THAT GROWS MID-RUN IS THE METHOD WORKING.** The parent's inventory
was already short by one (`constructor_identity`). Report the number you
observe; do not reconcile it against any number in this frame, including the
Architect's census and the Steward's re-derivation — **different extractors
produce different totals and neither is the contract.** The contract is *zero
owed under YOUR extractor, with the base control beside it.*

**AC-1a — the resulting tree builds, and increment A compiles against it.**
`scripts/ken-cargo check -p ken-runtime --lib` exits 0, *"Checking ken-runtime"*
present in the log, and the same with increment A's tree applied on top.
**Necessary, not sufficient, and no longer the completion criterion** — this is
what answers `D0-1`'s open half, since file-level replay is conflict-freedom
and says nothing about whether the result compiles. **Zero errors, never "the
eleven are gone".**

**AC-2 — the refused grant is ABSENT, measured in the direction that fails
open.** `MappingAcquireFile` must not be promoted out of the
`RepresentedUnavailable` tail. The instrument is the parent's `AC-UNAVAILABLE-ROSTER`:
the refusal arm lists exactly **ten** operations and `MappingAcquireFile` is one
of them, with a **NativeTested promotion count of 0**. **State the count, not
"no promotion found"** — census site 3 of that node produces SILENCE, by
subtraction, with no new line and a green build, and every instrument aimed at
what a diff ADDS is structurally blind to it. **An omission has no complainant.**

**AC-3 — Region 1 is untouched.** `git diff` over the five Region 1 files
between this candidate and its base is **empty**. Positive control: name one
file the candidate DOES change in the same command, so a green AC-3 is
distinguishable from a command that matched nothing.

**AC-4 — the acceptance test's state is REPORTED, not assumed.**
`abi_s6_mapping_file_backed_native.rs` arrives with reds. **Do not repair toward
green.** Land it with its actual state named
per-red, and if any red is to be accepted it needs a row in
`.github/ignored-test-exemptions.toml` — whose schema is `class` +
`readmission` + `test_path`, i.e. **an accepted red names what would readmit
it.** Steward-verified at `origin/main` `6036f9f5d`: 44 lines, 8 `[[exemption]]`
entries, and **zero** matching `abi_s6`, `mapping`, `d5b`, `file_backed` or
`px8f`. **A red with no row is not an accepted state; it is just red.**

**READ THIS REGISTER FROM `origin/main`, NEVER FROM YOUR WORKTREE:**

```sh
git show origin/main:.github/ignored-test-exemptions.toml
```

**The failure mode is a FALSE NEGATIVE on the exact question this AC turns on,
and it is silent.** A branch that lags `main` has *fewer* rows, so a row added
to `main` after your branch point is invisible — and the answer you get,
*"no row exists, therefore this red is not accepted"*, is wrong in the direction
that convicts a red the fleet already accepted. **Demonstrated live** while this
frame was being written: the Architect first reported 34 lines, read from a
worktree on a lagging branch, and corrected it themselves to 44 at `origin/main`
(`evt_f3y5ffna32kv`). **The conclusion survived; the method did not.** A
measurement correct on one tree is not a claim about another without naming
which tree it was taken on.

**THE "11 BASE REDS, 8 OF THEM `CheckedIhDetachedCallerCut`" POPULATION IS
WITHDRAWN FROM THIS AC.** Those 11 were measured on the checkpoint tree, **with
the grant**. With the grant excluded they cannot recur — see `§2e`. **A report
that does not match them is this AC working, not failing.** Report the
population you actually observe, do not reconcile it against that one, and do
not read a missing red as a red you repaired.

**ONE RED IS PREDICTED IN ADVANCE. It carries NO information about the port.**
Named before the run so it is never read as a finding (Architect
`evt_2jzjsj3nhn2qm`):

    D5B_FILE_SOURCE_ADMISSION_APPLICATIONS -- only writer is
      mapping_acquire_file_source_rights  (effect_v1.rs:1121-1124)   EXCLUDED

    consumer  ken-cli/tests/abi_s6_mapping_file_backed_native.rs:607
      fn file_source_admission_uses_read_not_destination_protection_rights
      :614  assert_eq!(applications, 1, "...must reach one file acquisition")

On the ported tree `applications` is **0 by construction** and that assertion
fails. **Report it per AC-4 with the exclusion named as its cause.**

**WRITE THE CAUSE BESIDE IT OR IT WILL BE RE-DERIVED WRONGLY.** A
mutation-witness reading zero is the exact signature of a **vacuous
instrument**, and a later reader cannot distinguish *"the exclusion removed the
producer"* from *"this instrument never worked."* Those are two causes behind
one number.

**THE DISCRIMINATOR, which is the informative half:** if that test fails at
**any other** assertion — `exit_status`, `terminal_error`, `mutated_backing`,
the effect trace — **that is NOT predicted by the exclusion and IS a finding.**
**Quote which assertion failed, never only the test name.** Same discipline as
the `CheckedIhDetachedCallerCut` reds: name the emitting site, not the symptom.

**AC-5 — the excluded instruments are NAMED, and the list is DERIVED FROM THE
BUILD.** `§3`'s predicate produces exclusions; list them with the reason each
failed the reaching-consumer test. **An exclusion you can name is reviewable; a
silent one is indistinguishable from an oversight.**

**RE-PHRASED BY `§3a`: the satisfying act is a tree property, not an authored
list.** "Has a reaching consumer" is a **compile** question, so the answer comes
from the compiler and the list is its by-product — never an enumeration written
first and then checked. The candidate already ran it this way: *"`§3`'s predicate
answered by the compiler: 33 instruments have reaching consumers."* **Report
which build configuration produced the answer**, since an instrument reachable
only under `px8-ds-test-support` is a different fact from one reachable in the
default config, and the two configs have already disagreed on this node.

**AC-6 — `trusted_base()` delta is ZERO.** No kernel change is in scope.

## 7. Base

**Cut from a `main` containing `10e75cb93656d5ea787bceaf754b2500b78de166`** (the
checked-IH consumer port). Pin a literal SHA, never the ref. Verify
`git diff "$BASE" HEAD -- crates/` is empty **at the moment you adopt it**, not
when it was proposed — the parent's own pin went stale exactly this way when the
port landed, and its test caught it.

## 8. Contention

`crates/ken-runtime/src/cranelift_backend/**` is this ring's own territory and
no other lane is in it. **The live overlap is [[RT-D5B-POSTCALL-REFUSAL-MECHANISM]]**
(`ready`, unstarted) — see `§4` D0-3. It is not released to anyone, so there is
no concurrent-edit hazard today; the hazard is semantic, and D0-3 is where it
gets answered.

## 9. Not this node

- **Every ADJUDICATE method** — [[ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION]].
  Leave `main`'s body in place and hand the name over. **Moving a row there is
  not a re-scope and needs no ruling; forcing a call here is the only way to
  get this wrong.**
- **Repairing the HS10-inline cross-family guard.** Record the limitation, land
  the instrument, change nothing (`§3b`).
- The refused `MappingAcquireFile` grant — [[RT-D5B-MAPPING-AVAILABILITY-FLIP]],
  deliberately `draft`. **Do not hand-separate 8 of 42 lines.**
- Increment A's own substance, and increments B and C.
- Region 1's five files.
- Any kernel change.

## 10. Related

- [[ABI-S6-HS18-D5B-SUBSTRATE-ADJUDICATION]] — the judgment half, split out by
  `§3b`. Depends on this node and is framed against its ADJUDICATE worklist.
- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — what this unblocks.
- [[ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]] — the landed sibling port
  (`10e75cb93656d5ea787bceaf754b2500b78de166`); the worked precedent for extent
  and review on this exact surface.

## 11. Symptom inventory — CONTINUES `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT` `§1b`

**Armed by the Steward 2026-09-17** after the Architect reported this WP had
none (`evt_yg6f1x2cpwez`). The omission was the Steward's: per
`steward/escalation.md` arming this line is the Steward's act and *"an unarmed
trigger is not a trigger."* The Architect appends entries and owns the predicate
check.

**THIS WP CONTINUES THE PREDECESSOR'S CHAIN, so entries do NOT restart at 1.**
Same HS18 chain, same immediate predecessor, and the same node model — a port
from an unlanded prefix onto `main`, scoped by what is missing (Architect
`evt_73ynk31tang40`). The campaign-level inventory in
`docs/program/issues/ABI-S6.md` sits **above** both and stays where it is;
folding a WP-level chain into it would lose the level the predicate is about.

> **AUTHORITATIVE COPY OF ROWS 1-11: the predecessor's `§1b`.** Reproduced below
> in one-line form so the predicate check can run against an assembled list.
> **Amend a row THERE, never here** — two mutable copies of an append-only
> record is how the record stops being one.

**THREE PEOPLE GUESSED THIS NUMBER AND ONLY THE FILE SETTLED IT.** The Architect
wrote *"5"* (carried from context, withdrawn by them); the Steward wrote *"1"*
(no record found, and I had not read the predecessor); **the file says 12.**
Wrong in opposite directions, which is what a number nobody resolved against a
file looks like. **Take the number from the file.**

### The predicate this WP's entries are tested against

The predecessor's predicate, **already WIDENED once** (Architect
`evt_2qqye3tdnh4b1`, at row 11) from two shapes to three:

| shape | what the census sees | what it misses |
|---|---|---|
| ABSENT from `main` | its subject | — |
| PRESENT on `main` and CHANGING | the name is there, so unreported | the shape change (rows 5-8, 10) |
| PRESENT at the port and UNREACHABLE | every census PASSES them | the module edge (row 11) |

### Assembled rows

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
PREDICATE CHECK IS ANCHORED TO AN ENTRY, NOT TO AN ORDINAL.
  discharged at the effect_v1.rs entry (row 12) on 2026-09-17
  next check = the 3rd entry appended AFTER that one

  1-3  TOMBSTONE -- STRUCK 2026-09-17, content unrecoverable.
       Numbers RETAINED so every ordinal below keeps the value it has
       always had. Both WPs recorded these as a pointer at
       thr_6azxdz555c2qy; neither ever held the content.

  4    seed of 5 closes to 29 items                 D0 census
  5    RequiredConsumerProjection resident on main and RESHAPING   read, not census
  6    Copy DROPPED from the same derive; Ord/PartialOrd added     read, not census
  7    three accessors go TOTAL -> PARTIAL, signatures unchanged   read, not census
  8    ten variants behind a dev-dependency feature invisible to
       every permitted local build                                 manifest read
  9    a 26th host-effect consumer (HostOpV1::MappingAcquireFile),
       ROUTED OUT; recorded because the inventory records what the
       model failed to predict                                     D0b
 10    a second Copy drop, EliminatorRole                          D0b -- first row
                                                                   found by an instrument
 11    four types PRESENT at the port but UNREACHABLE from
       lowering; refused at the MODULE EDGE                        increment A's BUILD
 -- entries below are THIS WP's --
 12    a file holding BOTH grant content and an instrument the
       replayed region imports -- keyed on COMPILE DEPENDENCY
       crossing the exclusion boundary, not on what the lines mean
       (ken-host/src/effect_v1.rs)                                 evt_2jzjsj3nhn2qm
 13    four mutation instruments ported as DEFINITIONS with zero call
       sites (HS7, HS8, HS10-bridge-lowering; HS10-inline-response dead
       at both ends) -- keyed on the COMPILER WORKLIST being read for
       ERRORS while the defect is visible only in WARNINGS
                                                                  evt_2mq3v11w25k93
```

### The check at row 12: ANSWERED AND DISCHARGED

**Architect `evt_5htrzz3p2f6pb`, 2026-09-17. YES, the rows share a predicate.**
The answer and its closure are in `§3a`; the recut off it is authored there and
is the Steward's, per `steward/escalation.md`. **`§3a` is the operative text —
this section records that the check ran, not what it decided.**

**A candidate was offered first and WITHDRAWN BY ITS AUTHOR**, and the
withdrawal is kept because a discharged check should show what it ruled out:

> *withdrawn:* the node's model is a SET and every surprise has been an EDGE.

It failed on its own evidence. **Rows 5-8 and 10 are not edges at all** — they
are shape changes on a single resident item (`Copy` dropped, struct becoming
enum, accessors going total to partial), which is the majority of the list the
candidate claimed to explain. It was generalised from the three rows its author
had personally produced. **The surviving predicate covers all nine rows read.**

**SCOPE OF THE ANSWER, as given: nine of twelve rows.** The Architect read rows
4-12 and the widened three-shape table; **rows 1-3 they had not read, and
neither had anyone else — there is no copy to read.** They asked to be checked
against them, which is what produced the strike below.

⇒ **After the strike the answer stands over EVERY row that has content**, and
the caveat is discharged rather than outstanding. **The nine were never the
incomplete part; the twelve was.**

> **THE GAP IS THE SEED, AND IT IS THE STEWARD'S.** Rows 1-3 have never had a
> durable home. The predecessor's `§1b` stubs them exactly as this section did —
> *"the first three, from `thr_6azxdz555c2qy`"* — so **both copies of the record
> are a citation pointing at a thread**, and the Steward wrote the first of them
> at seeding on 2026-09-16. The predecessor's own opening line says why that
> fails: *"the running count lived only in one seat's working context — which is
> exactly what a compaction discards."* **The section's first row broke the
> section's own rule, at the moment it was armed.**
>
> ⇒ **A row whose content is a pointer is not a row.**

**ANSWERED AND STRUCK, 2026-09-17.** `runtime-implementer`, who appended them,
could not reproduce them and said so plainly rather than reconstructing
something plausible (`evt_662hds2t1j7g1`). They measured before answering:

    git grep 'thr_6azxdz555c2qy' origin/main -- docs/ agent/
      -> escalation.md, and the predecessor's stub line. Nothing else.

**The only surviving trace of rows 1-3 is the stub itself.** Rows 4-11 carry
content inline — populations, coordinates, which instrument found each — and
rows 1-3 carry a thread id. Their own context cannot supply it either, across
several compactions since: **the three rows that were never written down are
exactly the three that could not survive a compaction**, which is the section's
own stated rationale arriving from the other side. **An honest eight-of-eight
beats three placeholders that make the list read nine-twelfths-checked forever.**

> **THEY ARE TOMBSTONES, NOT DELETIONS, AND THE ORDINALS DO NOT MOVE** (Architect
> `evt_7v1j0d8zw4qe8`). Renumbering would have been a **silent three-entry delay
> in a safety trigger, arriving as a side effect of a bookkeeping fix** — the
> `effect_v1.rs` entry would become row 9, and *"next check at 15"* would then
> mean three more real events than it meant this morning. Nobody would choose
> that; it just happens if the numbers re-base and the schedule does not.
>
> ⇒ **General form, because it will recur: an append-only record whose ordinals
> carry SCHEDULING meaning cannot have rows removed.** Either the ordinals are
> stable — struck rows become tombstones keeping their number — or the schedule
> is anchored to entries rather than counts. **Tombstones are the smaller
> change**, and they preserve the evidence that three rows were once cited and
> never written. This section now does both: tombstoned numbers **and** a check
> anchored to the `effect_v1.rs` entry.

**Recorded as a visible re-base rather than a quiet edit**, the same way the
row-number correction was: the list was cited as **12** entries, holds **9** with
content, and the ordinals of all nine are unchanged.

