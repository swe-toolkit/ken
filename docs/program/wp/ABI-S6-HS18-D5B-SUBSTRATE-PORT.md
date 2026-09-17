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
2. `crates/ken-runtime/tests/abi_s6_mapping_file_backed_native.rs`, currently
   absent in full. **Its expected state is not assumed green — see AC-4.**
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
`abi_s6_mapping_file_backed_native.rs` arrives with known reds (the parent
records 11 base reds, 8 of them one `CheckedIhDetachedCallerCut` Packaging
reason). **Do not repair toward green.** Land it with its actual state named
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
