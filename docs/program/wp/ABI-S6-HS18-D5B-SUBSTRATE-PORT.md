# WP frame — `ABI-S6-HS18-D5B-SUBSTRATE-PORT`

    owner   runtime        tier   T1        size   L
    blocks  ABI-S6-HS18-MAIN-BASED-CLOSURE  (its only dependency)
    node    docs/program/issues/ABI-S6-HS18-D5B-SUBSTRATE-PORT.md

## 1. Objective

Land the D5b prefix's **production** residue on `main`, so
`ABI-S6-HS18-MAIN-BASED-CLOSURE` increment A has a substrate to compile
against. **Exclude the refused `MappingAcquireFile` grant by construction, and
do not absorb increment A's own substance.**

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
is the rule's reason and its limit — see `§1b`, where a compiler-drawn boundary
licenses a split and whole-file exclusion would have broken the build.**
**Treat six as a floor and close it with the
parent's `AC-PREDICATE` — zero diff lines naming the operation at pathspec
`crates/` — which catches all seven touches without anyone holding a complete
site list.** Measured: Region 2's 13 files carry 17 such lines; the 10 files
that remain after excluding the catalog, `lowering/effects.rs` and
`st/effects.rs` carry **zero**.

**1b. `ken-host/src/effect_v1.rs` IS SPLIT, and whole-file exclusion is
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

## 4. D0 — answer before writing production. A hard stop here is a GOOD outcome.

**D0-1. Does Region 2 actually replay onto `main`?** `§2b` says `main` sits at
the prefix base for 13 files — that is a **content** claim, and applying a diff
is a different act from matching a blob. Apply and report, per file. **If some
subset does not replay cleanly, say which and stop**; that converts Region 2
into Region 3 and is a re-scope for the Steward, not something to force.

**D0-2. For Region 3, is reconciliation bounded per file?** Twelve of sixteen
are fully subsumed, so the real question is the four that are not. **Name, per
file, whether `main`'s independent version SUBSUMES the prefix's behaviour or
DIVERGES from it.** Subsumption is the favourable answer and makes those files
no-ops.

**D0-3. Does the checked-IH cluster collide with
[[RT-D5B-POSTCALL-REFUSAL-MECHANISM]]?** That node is `ready` and its subject
is the `CheckedIhDetachedCallerCut` refusal — the mechanism is **UNKNOWN** and
that node exists to find it. This node lands `bind_checked_ih_detached_caller_cut`
and siblings. **If landing them would fix or obscure the refusal that node is
chartered to diagnose, stop and say so** — that is a sequencing call and it is
the Steward's.

## 5. Deliverables

1. The production cluster from the node's `§What is IN`, landed on `main`:
   generated-context-result authority; checked-IH post-call/detached;
   recursive-position calls; source dynamic match.
2. `crates/ken-cli/tests/abi_s6_mapping_file_backed_native.rs`, currently absent
   in full. **Note the path: `ken-cli/tests/`, not `ken-runtime/tests/`** — this
   frame and the node both said `ken-runtime` until 2026-09-17. **Its expected
   state is not assumed green — see AC-4.**
3. The evidence instruments that pass `§3`'s reaching-consumer predicate, with
   the **excluded** ones named and the reason given.
4. Whatever `lowering/mod.rs` re-exports the above require (39 items missing
   there, the largest single residue).

## 6. Acceptance

**AC-1 — increment A compiles against this.** The satisfying act is
`scripts/ken-cargo check -p ken-runtime --lib` exiting 0 **with increment A's
tree applied on top of this node's landed `main`**, and *"Checking ken-runtime"*
present in the log. That is the whole point of the node, so it is the first
criterion. **Zero errors, never "the eleven are gone"** — the parent's inventory
was already short by one (`constructor_identity`), so the count may GROW when
re-attempted. **A count that goes up is the predicate working.**

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

**AC-5 — the excluded instruments are NAMED.** `§3`'s predicate produces
exclusions; list them with the reason each failed the reaching-consumer test.
**An exclusion you can name is reviewable; a silent one is indistinguishable
from an oversight.**

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

- The refused `MappingAcquireFile` grant — [[RT-D5B-MAPPING-AVAILABILITY-FLIP]],
  deliberately `draft`. **Do not hand-separate 8 of 42 lines.**
- Increment A's own substance, and increments B and C.
- Region 1's five files.
- Any kernel change.

## 10. Related

- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — what this unblocks.
- [[ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]] — the landed sibling port
  (`10e75cb93656d5ea787bceaf754b2500b78de166`); the worked precedent for extent
  and review on this exact surface.

## 11. Symptom inventory

**Armed by the Steward 2026-09-17, after the Architect correctly reported that
this WP had none** (`evt_yg6f1x2cpwez`). The omission was the Steward's: per
`steward/escalation.md`, arming this line is the Steward's act and *"an unarmed
trigger is not a trigger."* The Architect appends entries and owns the predicate
check.

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
NEXT PREDICATE CHECK = 3rd entry, then 6th, 9th, ...

1. a file holding BOTH grant content and an instrument the replayed region
   imports -- keyed on COMPILE DEPENDENCY crossing the exclusion boundary,
   not on what the lines mean          (ken-host/src/effect_v1.rs, evt_2jzjsj3nhn2qm)
```

**THE NUMBERING RESTARTS HERE, AT 1.** The Architect first wrote this entry as
*"5."* and then withdrew the number themselves, because there is no durable
record of entries 1-4 **on this WP** — the count was carried from context.
**A number a reader cannot resolve against this file is not a record.** If this
inventory is meant to continue a predecessor WP's, name that WP and its entries
here; otherwise 1 is correct and the predicate check falls at the 3rd entry
appended below.

