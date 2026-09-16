# WP frame — `ABI-S6-HS18-MAIN-BASED-CLOSURE`

**Owner:** Team Runtime · **Size:** L, decomposed into three increments ·
**Risk:** high (the protected substance is a 4083-line addition to a file that
has moved on `main`, and the line it came from carries a refused capability
grant) · **Tier:** T1 · **Gate:** none · **Deps:** none — cut from `main`

**Base:** `origin/main` `6f49f852141a66571c6126a569b954f63e2b6bde`.

**Origin:** Steward cut 2026-09-16, executing the operator's 2026-09-16
direction (*"do not build on an unmerged commit... the base commit needs to be
on main"*) after the Architect ruled RE-DERIVE rather than rebase
(`evt_ma144e8mt7sn`), on the measured ground that the preserved checkpoint
carries the `MappingAcquireFile` promotion the Architect refused in `z4664`.

> # THE NAME `ABI-S6-HS18-CLOSURE-AMENDMENT-8` IS SUPERSEDED BY THIS NODE.
>
> That name is in circulation — runtime-qa read a kickoff against it
> (`evt_2e58kcb0zd4h9`) — and it **never had a file behind it**. It is retired
> here rather than left as a second name for the same work. Amendment 8 is a
> ruling this node implements, not the node.

---

## 1. Objective

Close HS18 on a line based on `main`, preserving the verifier substance the
Architect protected by name and dropping the capability grant they refused.

**This node does not re-decide the mechanism.** Amendment 8 is the mechanism
(§6), and its rule, controls and not-authorized list are reproduced here in
full because a pointer is what a deferral evaporates through.

---

## 2. Why this is three increments and not one candidate

**Operator, 2026-09-16, verbatim:** *"do not create long stacks of unmerged
commits. This is an anti pattern. It creates kickoff confusion (which you have
observed)... Make what adjustments you need to to get the incremental commits
to pass CI, as long as we are progressing in the goal to clearing the tech
debt."*

The preserved line is 6 commits and **13408 changed lines**. Framed as one
candidate it reproduces the exact shape the operator named. It decomposes
because the measurements below say the pieces are separable — not because
smaller is nicer.

### 2a. The extent, measured at `6f49f8521`

    increment  substance                              extent                 status
    A          verifier substance (the PROTECTED part) 9 files, units.rs +4083  first
    B          Q1 resume-exit repair                   source.rs 935 lines      second
    C          amendment 8's consumer relocation       not yet written          third

**A goes first, and §2c is the measurement that puts it there.** Each
increment is cut from the `main` that exists when its predecessor lands. **No
increment is cut from another increment's branch.**

### 2b. The four-commit D5b prefix is NOT protected and it is where the refusal lives

The Architect narrowed their own no-revert by name (`evt_ma144e8mt7sn`):

    PROTECTED      verifier substance of 01d2ccb11 + 5d977ac79 (9 files,
                   principally units.rs)
    NOT protected  the four-commit D5b prefix
    REFUSED        the effect_abi_v1.catalog line + the availability() arm

`30d35f625` — *"ABI-S6 D5b: exit source frame before outer resume"* — is in the
unprotected prefix. It is **24 files, +9190/-5482**, and it bundles Q1's
`source.rs` repair together with `effect_v1.rs +445`, which is where the
refused flip enters. **Increment B must extract Q1 from that commit, not adopt
it.**

### 2c. Increment A does not reference Q1's machinery — measured, with the caveat stated

Three types are Q1's mechanism. Their diff-line count across the verifier
substance (`30d35f625..5d977ac79`):

    SourceMachineExit               0
    SourceDynamicMatchRequest       0
    SourceDynamicMatchScrutinee     0

⇒ **The protected substance makes no textual reference to Q1.** That is why A
precedes B rather than the reverse, and it inverts the ordering the commit
sequence suggests.

**What this does NOT establish, stated because the zeroes look conclusive.**
Three zeroes prove no *textual* dependency. They do not prove **semantic**
independence — the verifier substance could depend on behaviour Q1 establishes
without naming its types. **That is a witnesses-run question, not a grep
question**, and it is increment A's first act (§3, D0). If the witnesses say A
needs B, the order swaps and that is a finding, not a failure.

### 2d. Every target file has moved on `main`. This is why RE-DERIVE, quantified.

Blob compare, checkpoint base `30d35f625` against `origin/main`:

    cranelift_backend.rs   MOVED      calls.rs   MOVED      core.rs    MOVED
    mod.rs                 MOVED      units.rs   MOVED

**Five of five.** A transplant lands 4083 lines authored against a tree that no
longer exists at any of the five sites it touches.

---

## 3. D0 — the two questions increment A answers before it writes code

**D0-1. Is the verifier substance's EFFECT already on `main` by another route?**

Open since the checkpoint was cut, asked by the implementer, and **never
answered by anyone**. It is cheap if the answer is yes and it decides whether
increment A exists at all.

**The instrument is the witnesses RUN, not a blob compared.** A blob compare
answers *"is this the same text"*, which is not the question. §3a shows why a
name-keyed grep is worse than useless here.

**D0-2. Does the verifier substance build and pass against `main`'s five moved
files without Q1?** This is §2c's caveat discharged by execution. Targeted
build only — `scripts/ken-cargo -p ken-runtime`; **never `--workspace`**
(`COORDINATION §12`).

### 3a. A worked example of the instrument trap, from this frame's own measurements

I asked whether Q1's repair is on `main` and started with the obvious grep:

    ResumeOuter        main = 20 occurrences

**Twenty hits, and they are all the wrong thing.** `main`'s `ResumeOuter` is a
pre-existing `SourceContinuationTerminal` variant. Q1's actual mechanism:

    SourceMachineExit             main = 0    checkpoint = 25
    SourceDynamicMatchRequest     main = 0    checkpoint = 10
    SourceDynamicMatchScrutinee   main = 0    checkpoint = 13

**Q1 is absent from `main`, and the first instrument said it was present.** The
discriminating symbol is the one the change *introduces*, never the one it
*mentions*. Apply this to D0-1: pick a symbol that exists only if the verifier
substance is there.

---

## 4. The refused flip — the gate, its scope, and why the scope is the whole point

**`5d977ac79` carries the `MappingAcquireFile` promotion the Architect refused
in `z4664` and refused again under the confirmation-protocol ruling.** No
differential exists for that op, and `catalog.rs:315` would refuse to confirm
one if it did.

**A rebase is content-preserving, which is exactly wrong when part of the
content was rejected.** A *clean* rebase was the worst outcome available here:
the refused grant landing with no conflict to stop it.

### 4a. The census — five sites, and the asymmetry is the lesson

    site                                            main          5d977ac79     shape
    1  effect_abi_v1.catalog:97                     unavailable   native        change
    2  effect_v1.rs availability() arm              Unavailable   NativeTested  ADD
    3  effect_v1.rs ten-op refusal arm              10 ops        9 -- MAF GONE DELETE
    4  effect_v1.rs NATIVE_TESTED_TARGETS_V1        [_; 25]       [_; 26]       ADD
    5  lowering/effects.rs                          0 occurrences 5+, incl. a
                                                                  lowering arm    ADD

Census by runtime-implementer (`evt_4javg2nnyxcqt`), corroborated against
`RT-D5B-HOST-FILE-ACQUISITION-SURFACE`'s `AC-AVAIL`, which was written over the
same five and independently derived. **Treat five as a floor, not a census** —
nobody has proven no sixth exists, which is the argument for a predicate over
an enumeration.

**Four additions and one deletion.** Sites 1, 2, 4 and 5 produce new lines a
reviewer or a grep can see. **Site 3 produces silence.** It is also the site
that punches a hole in the ten-op uniform refusal gate that landed this
session — by subtraction, with no new line and a green build.

⇒ **An omission has no complainant.** Every instrument aimed at what a diff
*adds* is structurally blind to site 3.

### 4b. `AC-PREDICATE` — the gate. Keyed on the OPERATION, not on any site.

```sh
# Increment A. Each increment pins the LITERAL SHA it was cut from.
BASE=6f49f852141a66571c6126a569b954f63e2b6bde

# The base must be a SHA that CANNOT BECOME THE CANDIDATE.
# Keyed on the EFFECTIVE base, never the WRITTEN one -- see the block below.
if git merge-base --is-ancestor HEAD "$BASE"; then
  echo "STOP: HEAD is an ancestor of BASE (inclusive) -- the range is degenerate."
  echo "  BASE=$(git rev-parse "$BASE")  HEAD=$(git rev-parse HEAD)"
  echo "  equal     -> base pinned to the candidate itself"
  echo "  not equal -> arguments inverted, or the candidate is stale/landed"
  exit 1
fi
echo "base=$(git rev-parse "$BASE")  head=$(git rev-parse HEAD)"

git diff "$BASE" HEAD -- crates/ | grep -c '^[+-].*MappingAcquireFile'
# MUST be 0
```

**Three things are pinned independently, and getting one right says nothing
about the other two:** the **key** (the operation, not a site list), the
**domain** (pathspec `crates/`, below), and the **base** (§4f). **Each failed
once in this node's drafting while the other two were correct.**

The key/domain distinction is the Architect's, written as their own
self-diagnosis at `evt_5fhxfxvwrmyzh`: *"A pathspec is an enumeration that does
not present as one"* — a site roster invites the question *is it complete?*,
while a directory reads as scope and gets no scrutiny. **An operation-keyed
gate over the wrong population is not an operation-keyed gate.**

**The base must be a literal SHA that cannot become the candidate**
(runtime-implementer, `evt_68akgs73am580`). `origin/main` and `merge-base(...)`
both fail that, silently and later — see §4f, which is not optional reading.

> ### THE TWO-DOT BAN AND THE GUARD ARE ONE PROTECTION, NOT TWO.
>
> **`...` is rejected syntactically in this AC.** It is not a style preference
> sitting beside the guard — **it is what makes the guard sound**, and dropping
> it leaves a check that cannot fire. Measured, not reasoned (Architect,
> `evt_12m48m4q5x7wr`):
>
>     git diff origin/main...dcb848eaa -- crates/          0    VACUOUS
>     written base    origin/main            = 6f49f8521...
>     effective base  merge-base(main,GATE)  = dcb848eaa...
>     HEAD                                   = dcb848eaa...
>
>     GUARD keyed on WRITTEN base     SILENT   no warning
>     GUARD keyed on EFFECTIVE base   FIRES    VACUOUS BASE
>
> `origin/main` and `HEAD` differ, so the `rev-parse` comparison **passes**,
> while `...` has already collapsed the effective base onto `HEAD`. **The guard
> audits the base as WRITTEN; the vacuity lives in the base as RESOLVED.**
>
> **This is not a contrived repair.** §4b's original text *was*
> `git diff origin/main...HEAD`. The most likely response to a confusing red is
> to restore the line that used to be there — which reinstates the vacuity
> **and** brings the guard's blind spot back with it.
>
> ### THE FIX: KEY THE GUARD ON THE EFFECTIVE BASE.
>
> **The guard above compares `merge-base($BASE, HEAD)` against `HEAD`, not
> `$BASE` against `HEAD`**, which makes it **syntax-independent**
> (runtime-implementer, `evt_121sezzvdfhg1`). Steward-verified at
> `6f49f852141a66571c6126a569b954f63e2b6bde`:
>
>     scenario                              written-base guard  effective-base guard
>     landed increment, BASE=origin/main    PASSES (silent)     FIRES
>     BASE == HEAD                          fires               FIRES
>     pinned literal, HEAD truly descends   pass                pass
>
> **A guard on what the command SAYS is not a guard on what the command DOES.**
> The written base and the effective base are different values, and only one of
> them is what the diff ran against.
>
> **The failure direction is why this is a fix and not a note:** the written-base
> guard's silence and its success are the same output — nothing. It would have
> shipped looking like a second layer of protection while providing none, beside
> prose explaining the hazard it was failing to catch.
>
> **Keep the two-dot ban anyway, for legibility.** With the corrected guard it is
> no longer what stands between the AC and a silent zero, but a reader should not
> have to derive the effective base to know what the command compares.
>
> ### THE GUARD'S PREDICATE IS NOT VACUITY — SAY WHAT IT ACTUALLY TESTS.
>
> Full case table, measured by the Architect at `evt_5jypsxd06kyzj`:
>
>     CASE (base, HEAD)                                GUARD    COUNT
>     1  three-dot restore, landed  (main, GATE)       FIRES    0
>     2  two-dot origin/main, landed (main, GATE)      FIRES    0
>     4  self-pin, base == HEAD     (GATE, GATE)       FIRES    0
>     3' GOOD: literal base, HEAD descends (PAR, GATE) silent   0
>     3''GOOD: literal base, HEAD descends (ANC, LIT)  silent  22
>     5  REVERSED / stale: HEAD is an ANCESTOR of base FIRES   22
>
> **Case 5 is why the message says "degenerate" and not "vacuous."** It fires
> beside a count of **22**. Vacuous implies empty, the reader sees 22, and
> **contradictory signals point at the wrong repair** — someone hunting an empty
> comparison will not look for reversed arguments, which is precisely the error
> that put a 22 on screen reading as a refutation of a correct 0 (§4c).
>
> The predicate is **"HEAD is an ancestor of BASE, INCLUSIVE"**, and the two
> sub-cases separate on the endpoints the guard already prints.
>
> **State it in the direction the guard tests, because the obvious rewrite is
> false.** *"BASE is not an ancestor of HEAD"* looks like the same claim and is
> not: `--is-ancestor` is **inclusive**, so at `BASE == HEAD` it is **FALSE
> while the guard FIRES**. **Inverting the direction and negating the predicate
> are two different operations, and doing both lands somewhere neither**
> (Architect, `evt_5q4sb9m1j3206`; reproduced by runtime-implementer at
> `evt_71ngftt0mfetp`).
>
> ⇒ **One measured fact, two causes: name the fact and enumerate the causes.
> Never promote one cause to the finding.** That is what the four-line message
> does, and it is the durable half of this whole exchange.
>
> ### WHAT THE GUARD DOES NOT COVER — it is not belt-and-braces with the pin.
>
> A **divergent** base — neither commit an ancestor of the other, which is what
> a candidate cut from a preserved tree looks like — passes the guard silently
> and produces a large, meaningless diff. **The guard claims only that the range
> is not degenerate in one direction; it never claims the base is SENSIBLE.**
>
> That property is carried by `AC-BASE-ON-MAIN` and the pinned literal, not by
> the guard. They cover different things and neither is redundant with the
> other.
>
> **Case 3'' is the rebase hazard measured rather than asserted:** a legitimate
> forward diff scoring 22 is exactly §4f's loud red on content the candidate
> never authored.
>
> ### CHECK A CONTROL'S OWN ARMS FOR DISTINCTNESS BEFORE READING ITS VERDICT.
>
> **This guard produced a confident wrong answer for two different people inside
> ten minutes, and both times the command was fine and the FIXTURE was
> degenerate.** Both used `origin/main` as the "good case" base while
> `origin/main` *equals* the pinned literal `6f49f8521...` — so both arms named
> the same tree, and what got built was case 4 wearing case 3's label.
>
> ⇒ **A degenerate fixture returns a confident wrong answer with no outward
> sign.** The check is one `rev-parse` of each arm, and it costs nothing. The
> tell that saved it both times was noticing that *"the good case fires"* is not
> a plausible result for a guard that had just been correct three times.

**The run prints its resolved base and both endpoints and asserts they differ,
before counting** (Architect, `evt_f5th0rzsjg0p`). A `0` with no endpoints
beside it is not a measurement.

**The pathspec is `crates/`, and getting it wrong is how this AC fails
silently.** The predicate was first published at `crates/ken-host/`. Measured
against the known-bad tree:

    pathspec            positive control vs 5d977ac79   verdict
    crates/ken-host/     15   MISSES 27 of 42 lines -- blind to site 5
    crates/              42   catches all five sites
    .  (whole tree)     104   also catches docs/ -- unusable

Site 5 is in `crates/ken-runtime/`. The code population spans **three** crates:

    crates/ken-host/src/effect_v1.rs                                   13
    crates/ken-cli/tests/abi_s6_mapping_file_backed_native.rs          11
    crates/ken-runtime/.../lowering/effects.rs                          8
    crates/ken-runtime/.../planning/static_transition/effects.rs        7
    crates/ken-host/effect_abi_v1.catalog                               2
    crates/ken-runtime/.../planning/static_transition/aggregates.rs     1

**The predicate's virtue is that it keys on the operation rather than on a site
roster. A pathspec that keys on a directory defeats that virtue** — and it
defeats it invisibly, because a roster at least announces itself as a list
where a pathspec reads as scope.

**Whole-tree is not the fix.** `docs/` names the op legitimately;
`wp/RT-D5B-HOST-FILE-ACQUISITION-SURFACE.md` alone has 19 lines, and **this
frame adds more**. An AC that reds because a tracker node discusses the op by
name is a false-positive machine.

**All three controls, re-run at `crates/`:**

    POSITIVE    vs 5d977ac79                          42   fires
    NEGATIVE    main vs main                           0   silent
    RELOCATION  dcb848eaa, the gate, arm 558 -> 580    0   NO false positive

The relocation control is the one that had to survive the widening. It does:
the arm matches as context rather than as delete-plus-add, so a legitimate
relocation passes cleanly and the predicate can bind hard.

### 4c. Why a wide pathspec is safe here — and the direction that makes the numbers mean two different things

**`main` touches this operation legitimately and often.** Measured as
`diff(parent, commit)` — what each commit itself changed:

    d4e977a6a   RT-D5B-HOST-FILE-ACQUISITION-SURFACE      17    legitimate, on main
    cf894cdb5   ABI-S6 D4: governed file-backed acquire    62    legitimate, on main

**Those scores are NOT what the AC would report for them, and the difference is
argument order.** In the AC's shape — `diff(BASE, HEAD)` with `HEAD` descending
from `BASE` — both commits are **ancestors of the cut base**, so their content
is already inside the base and they score **0**:

    git diff $BASE cf894cdb5    -- crates/   ->  22    the UNDO direction
    git diff $BASE origin/main  -- crates/   ->   0    the AC's actual shape

⇒ **`git diff BASE <ancestor>` reports what would have to be removed to get
back to the ancestor**, so an ancestor scores non-zero and reads as a hit. Both
commits are correctly invisible to this gate. (Direction error caught by
runtime-implementer at `evt_4g3e88radqsar`, before it reached the frame; the
same reversal had a `22` on screen reading as a refutation of the Architect's
`0`.)

**What the 17 and the 62 actually establish** is the premise §4f depends on:
**this operation is under active, legitimate development on `main`.** A future
`main` commit touching `MappingAcquireFile` is not hypothetical —
[[RT-D5B-MAPPING-AVAILABILITY-FLIP]] is live.

⇒ **The bar is "this candidate contains zero", not "nobody may ever touch
it."** Authorization is a property of a **transition**, not of a state
(Architect, `evt_7jcex53r71gn6`), which is why only a diff against a named base
can express it, and why the gate is inherently one-shot.

### 4f. The pinned base fails LOUD on a rebase, and the obvious repair reinstates the silent bug

**Read this before repairing a red on `AC-PREDICATE`.**

The base is pinned precisely so that everything already inside it is invisible
(§4c). **If an increment is ever rebased forward, `git diff $BASE HEAD` picks up
every intervening `main` change to the operation and the gate goes RED on
legitimate content the candidate never authored.**

**This is the correct failure direction — loud, not silent — and it is not to
be changed.** It is written down because the red arrives looking like a
stale-base defect in the AC, and **the natural repair is to swap the literal
back to `origin/main`, which silently restores the vacuity** (Architect,
`evt_69rqebdpcpfn4`). The loud bug's obvious fix is the silent bug.

**Disposition:** a red on this AC after a rebase means **the base is stale, not
that the candidate is dirty** — **re-pin the literal to the new cut point and
re-run. Never replace the literal with a ref.** And since these increments are
not supposed to be rebased at all, a red here is also the signal that something
rebased one.

**Two distinct failures, and the frame ships a mechanism for one and prose for
the other** (runtime-implementer, `evt_4g3e88radqsar`):

    base == candidate            VACUOUS -- the `A..A` degenerate case.
                                 Caught mechanically by §4b's rev-parse guard,
                                 which fires exactly when the bad repair is made.
    base behind an unlanded main WIDER CLAIM, not vacuous. Not caught by the
                                 guard; this is the disposition line's job.

The guard matters because swapping the literal back to `origin/main` produces
`A..A` **once the increment lands** — which is the moment someone re-runs the
gate to confirm it held, and the moment a `0` is most likely to be read as
confirmation.

### 4d. `AC-UNAVAILABLE-ROSTER-INTACT` — the by-name diagnostic

A count of 42 does not say *which* site moved, and a reviewer reading a red AC
deserves the location. Both checks are diagnostics beside the gate, not the
gate:

```sh
BASE=6f49f852141a66571c6126a569b954f63e2b6bde     # literal, same pin as §4b
sel='/^        HostOpV1::ClockMonotonicNow$/,/^    };$/p'
diff <(git show "$BASE":crates/ken-host/src/effect_v1.rs | sed -n "$sel") \
     <(sed -n "$sel" crates/ken-host/src/effect_v1.rs)          # must be EMPTY

git grep -c 'Self::MappingAcquireFile => HostOpAvailabilityV1::NativeTested'   # must be 0
```

**`git show origin/main:...` carries the same defect as §4b's original and gets
the same fix** — a moving ref makes the diagnostic re-read as a different claim
each time `main` advances.

The arm oracle is **content-anchored**, so it survives the arm moving — which it
already has once today, 558 to 580, under the gate.

Stated positively, over the whole roster, never as *"the diff adds nothing"*:

> **the `effect_v1.rs` refusal arm lists EXACTLY TEN operations, and
> `MappingAcquireFile` is one of them.**

### 4e. This is a CANDIDATE GATE. It retires with the candidate. Do not land it as a test.

Ruled by the Architect (`evt_7jcex53r71gn6`) on a measurement, and the
measurement is worth more than the ruling because it generalizes.

The proposed durable phrasing was *"the arm lists exactly the
`RepresentedUnavailable` set of `availability()`"* — derived rather than
counted, which is the right shape for most of this file's ACs. **It was
measured against the known-bad tree and it PASSES:**

    tree        availability()                      refusal arm   relation
    main        10 RepresentedUnavailable / 25 NT   10 members    10 == 10  HOLDS
    5d977ac79    9 RepresentedUnavailable / 26 NT    9 members     9 ==  9  HOLDS

**The flip edits both sides, so every internal relation stays true.**

⇒ **A consistency invariant cannot detect a coordinated unauthorized change.**
Coherence and warrant are different properties and a relation test measures
only the first. The checkpoint's flip is internally *coherent*; what makes it
wrong is that it is *unauthorized and unproven*.

    derive    when you fear an INCOMPLETE list -- invariant under legitimate growth
    pin       when the change you fear IS the growth -- coordinated, by name, today

A count in a criterion is dangerous when it is **durable**. This one has no
durability: it dies with the candidate, which is exactly where a count is safe.

**The relation test is still worth having and is filed separately** — it closes
the *coherent-but-partial* hazard, where one site moves and the others do not.
Different hazard, different instrument, neither substitutes for the other. See
`RT-D5B-MAPPING-AVAILABILITY-FLIP`'s neighbourhood.

---

## 5. Increment B — Q1, and why amendment 8's control 5 does not mean what it says here

**Amendment 8 control 5 requires** `lowering/source.rs` to stay at blob
`38ec787dac261f02d46463ee1e9fca555c76d694`. Measured:

    b4c8df33a    7eff841d21c91cb12b2356892b6cef32288f971e
    origin/main  7eff841d21c91cb12b2356892b6cef32288f971e
    b601e2ec7    38ec787dac261f02d46463ee1e9fca555c76d694
    5d977ac79    38ec787dac261f02d46463ee1e9fca555c76d694
    30d35f625    38ec787dac261f02d46463ee1e9fca555c76d694

**The pinned blob exists on every unlanded tree and on none of the landed
line.** `b4c8df33a` is an ancestor of `main` and `source.rs` has not been
touched on `main` since — so this is not drift. Q1 simply never landed.

⇒ **On a main-based cut, control 5 is not a preservation control. It is a
DELIVERABLE**, and it is increment B. Read as written against `main` it is
unsatisfiable, and its failure would read as a regression rather than as a
missing prerequisite.

**The gap is 935 changed lines in `source.rs`**, introducing
`SourceMachineExit`, `SourceDynamicMatchRequest` and
`SourceDynamicMatchScrutinee`.

**Increment B extracts Q1 from `30d35f625`; it does not adopt that commit.**
`30d35f625` is 24 files including `effect_v1.rs +445`, and it is where the
refused flip enters the line. `AC-PREDICATE` binds increment B exactly as it
binds A.

---

## 6. Amendment 8 — the mechanism, reproduced in full

`docs/program/ABI-S6-HS18-closure-mechanism-amendment-8.md`. **Amendment 8 is
CURRENT** and carries no superseding banner. The live authority chain, verified
banner by banner: base/1/2 SUPERSEDED; **3 CURRENT** (extended, not superseded,
by 4); **4 CURRENT**; **5** supersedes nothing; **6 RULE SUPERSEDED**
(transfer-as-the-repair withdrawn); **7 ORIGIN superseded by 8, its forward
DESTINATION survives**; **8 CURRENT**.

### 6a. The rule

> **A producer seat never invokes the consumer. Its result returns to the seat
> holding the exact creation-site frame, and that seat originates the consumer
> transfer.**

The fenced alternative, stated so it is refused rather than forgotten:
**closure-conversion — the continuation as a first-class runtime value carrying
its own captures — is NOT ruled.** It requires a transport that is fenced out.

### 6b. The five controls. Each can fail, and each names the branch that refutes the amendment.

1. **The witness advances BY application.** Observe the four-arm match's tag
   query **executing** on the `ResourceBracketOk` word `0x0f09`. `Context3`
   accepting its input is not evidence; route-shape rows are not execution
   evidence.
2. **The relocated call is SATISFIED, not merely attempted.** At the
   creation-site function the call to context 0 must pass the frame filter on
   all three coordinates and both cardinalities (7 worker, 6 context). **If it
   still refuses, the mechanism is wrong — stop and report, do not widen the
   filter.**
3. **The wrong-site replay is GONE, not duplicated.** The producer-side
   function emits **no** detached post-call consumer replay. If both sites
   replay, the consumer is applied twice, which is strictly worse than the skip
   — a stop.
4. **Non-regression on ordinary routes.** This gate stands above most ordinary
   code; over-tightening is the design's plausible failure and control 4 is the
   only thing watching it.
5. **Retained green.** The px8f pre-object refusal stays honest; the entry-18
   verifier repair stands. **Q1's `source.rs` blob pin is re-read per §5 — it
   is a deliverable on this base, not a preservation.**

### 6c. Not authorized

No new carrier, phase tag, or second planner relation. No runtime object. No
ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound change. No
weakening of the pre-object refusal. No revert of the entry-18 verifier repair.
**No relocation of emission ownership.** No identity edit — the finished
identity chain is not reopened.

### 6d. Two equivocations that would let this cross its own fences

**1. "Closure conversion."** `core.rs:12038` calls the landed `D2` frame
mechanism *"the closure conversion the Architect ruled."*

| naming | what it is | fence |
|---|---|---|
| **Landed `D2`** | compile-time `ir::Value` operands in a function-local frame, consumed in the same function | inside |
| **The fenced arm** | a first-class continuation value whose state travels to another function | outside |

**Do not cite the landed `D2` comment as authority for a transported
continuation.** That is a fence crossing dressed as continuity with a prior
ruling.

**2. "Relocation."** Moving the consumer's **emission SITE** is not relocating
**emission OWNERSHIP**. Ownership is a property of the generated context, not
of its caller. Amendment 8 moves the former and leaves the latter exactly where
it is.

### 6e. The consumer-authority machinery is absent from `main` — all four spellings

    detached_return_context              0
    checked_ih_detached                  0
    detached_consumer_authorities        0
    detached_post_call_consumer_frames   0

Measured at `origin/main` over `crates/`. `constructed_context_frame` is
present at **11** occurrences — it is the landed `D2` singular `Option`
(`lowering/mod.rs:1308`), whose own comment states the operands *"cannot be
reused elsewhere."* **That is the structural basis of the rule, not a leftover
of the repair.** Do not read its presence as partial landing.

---

## 7. R1 and R2 — should-fix ON the candidate, not a separate cut

`ABI-S6.md:230`, verbatim: *"addressed on the eventual candidate (should-fix on
the candidate, per the prior ruling — not new work)."*

- **R1:** convert `generated_result_path_proof_..._certificate_corruption` from
  the widened four-way disjunction to the per-mutation-substring pairing
  already used by
  `generated_result_owner_certificate_rejects_each_finished_body_corruption`.
- **R2:** close the `location_access` escaped-via-call-argument gap (a later
  write through a pointer an earlier callee retained), **or argue it
  unreachable**, before landing.

**They attach to increment A**, which is the increment carrying the verifier
substance they are residuals of. A Steward ruling that put them at the top of
the queue as independent work was **withdrawn** — it quoted the ruling and
dropped its qualifier.

---

## 8. Acceptance

Every AC below binds **each** increment, cut from the `main` of its own moment.

- **AC-BASE-ON-MAIN.** The candidate's merge-base with `origin/main` is a
  commit on `origin/main`. Neither `preserve/` ref is an ancestor. **The
  preserved refs are EVIDENCE, NOT BASES**, and their
  `-not-a-candidate` names stay.
- **AC-PREDICATE.** §4b, at pathspec `crates/`, result **0**. Absolute. Three
  independent pins, each of which failed once during this frame's own drafting:
  - **key** — the operation, never a site list. A list cannot report being
    short.
  - **domain** — pathspec `crates/`. `crates/ken-host/` misses 27 of 42 lines.
  - **base** — **a literal SHA that cannot become the candidate**
    (runtime-implementer, `evt_68akgs73am580`). Never `origin/main`, never
    `merge-base(...)`, never `...`. The degeneracy guard keys on the
    **effective** base and reports *degenerate*, not *vacuous* (§4b).
  - The run **prints both endpoints and stops before counting** if HEAD is an
    ancestor of BASE. A `0` with no endpoints beside it is not a measurement.
- **AC-ATTRIBUTION-RESOLVED.** Before this frame or any successor is committed,
  **every attributed claim in it is resolved against the event that contains
  it** — not against recall (runtime-implementer, `evt_2z98ke2vey33b`). Three
  misattributions occurred in this node's drafting inside ninety seconds, across
  three different seats. The mechanism: **a quotation carries its text and drops
  its addressee**, so a pronoun that resolved correctly in-thread re-resolves
  against whoever the frame is about. Attribution attaches at the moment text is
  **lifted**, never reconstructed from the thread afterwards — the frame is what
  people read *instead of* the thread.
- **AC-UNAVAILABLE-ROSTER-INTACT.** §4d, both diagnostics, asserted positively
  by name.
- **AC-REDERIVED-NOT-TRANSPLANTED.** The substance is re-derived against
  `main`'s five moved files (§2d). A diff that reproduces checkpoint line
  ranges verbatim against a moved file is the failure this AC exists to catch.
- **AC-D0-ANSWERED.** §3's two questions answered by execution — witnesses run
  for D0-1 on a symbol that exists **only if** the substance is present (§3a),
  targeted build for D0-2.
- **AC-AMENDMENT-8-CONTROLS.** §6b's five, each reported by name with its
  measurement. Controls 2 and 3 are **stops**, not warnings.
- **AC-R1-R2.** §7, on increment A.
- **No-regression means GREEN IN CI**, never a local `--workspace` run
  (`COORDINATION §12`). Local work is `scripts/ken-cargo` scoped to the crate
  touched.

---

## 9. Contention

`crates/ken-runtime/src/cranelift_backend/` — principally `lowering/units.rs`,
`lowering/mod.rs`, `lowering/core.rs`, `lowering/calls.rs`, and for increment B
`lowering/source.rs`.

Overlaps `RT-D5B-MAPPING-AVAILABILITY-FLIP`'s neighbourhood at `effect_v1.rs`
only in the negative — this node must not touch it. Check live node status at
`origin/main`, **not for a branch ref**, before releasing each increment.

---

## 10. What this node is NOT

- **Not a promotion of `MappingAcquireFile`.** §4. That is
  `RT-D5B-MAPPING-AVAILABILITY-FLIP`'s, and it is blocked on a differential
  that does not exist.
- **Not a rebase, port, or transplant of either preserved tree.** §2d.
- **Not a re-opening of the mechanism.** Amendment 8 is the ruling; §6 is its
  reproduction, not a restatement open to amendment.
- **Not a revert of the entry-18 verifier repair** (§6c).
- **Not the durable relation test** (§4e), which files separately.
