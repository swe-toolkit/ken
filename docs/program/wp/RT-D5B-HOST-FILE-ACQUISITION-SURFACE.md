# WP frame — `RT-D5B-HOST-FILE-ACQUISITION-SURFACE`

**Owner:** Team Runtime · **Size:** M · **Risk:** low (strictly additive after
the §4 ruling — the availability bit does not move, and two of the four sites
that would have to move are compiler-enforced) · **Tier:** T1 · **Gate:** none ·
**Deps:** none — touches no cranelift file and does not depend on the HS18
closure

**Origin:** operator directive 2026-09-16: *"factor small mergeable pieces out
of the long string of commits and merge those... Focus on small incremental,
achievable pieces. 'Must land as one commit.' is a trap that leads you to
unworkable situations."* Slice 4 of the `wp/ABI-S6-d5b-file-backed` drain, after
`RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER` (slice 1, `10321a158`),
`RT-D5B-BRIDGE-REALIZATION-PLANE` (slice 2, `49e5ebfbe`) and
`RT-D5B-LIVE-WIRING` (slice 3, `67684fa5d`).

## 1. Objective

Land the **host and interpreter** side of file-backed mapping acquisition:
the `MappingAcquireFile` operation's host surface, its elaborator prelude
global, and its interpreter evaluation path.

This is the one remaining cluster on the branch that is **orthogonal to the
contested backend work.** Slices 1-3 drained PR #3676's cranelift material;
this slice touches no cranelift file at all, so it can be reviewed and merged
without waiting on the HS18 closure.

## 2. Fixed inputs, measured

Measured by the Steward at backup tip
`0d94d58b60e2e7bb045d7204efd138e17df68b7b` against merge-base
`4bf1ad362b5a5cfa512636087df5229ee57db706`, which is the base the branch
actually forks from. **The residual the branch still carries as a whole is 21
commits, +18190/-6067 over 35 files** — this slice is the separable 9-file part
of it.

    4     0   crates/ken-elaborator/src/compiler_driver.rs
    17   29   crates/ken-elaborator/src/prelude.rs
    4     0   crates/ken-host/Cargo.toml
    1     1   crates/ken-host/effect_abi_v1.catalog
    49    0   crates/ken-host/src/abi_v1.rs
    329 231   crates/ken-host/src/effect_v1.rs
    10    0   crates/ken-host/src/lib.rs
    59    0   crates/ken-host/src/mapping_v1.rs
    30    1   crates/ken-interp/src/eval.rs
    ---------
    9 files, +503 / -262        cranelift files touched: ZERO (measured)

**The zero is a fixed input, not a hope.** `git diff --name-only` over this
file set greps `cranelift` at count 0. If a candidate's file set puts a
cranelift path in this slice, the cut was wrong and the slice stops.

**AS-BUILT: the slice landed SMALLER than this input, and that is D0's answer
rather than a measurement error.** Landed at `d4e977a6a`:

    7 files, +241 / -63         cranelift files touched: ZERO (confirmed)

The two file rows that fell away are `crates/ken-host/Cargo.toml` (dropped in
the F5 respin — an unused `[features]` table) and
`crates/ken-host/effect_abi_v1.catalog` (unchanged, because the availability bit
does not move here). **The numbers above are retained as the framed input, not
corrected in place**, because §2a's argument was built on `-262` and rewriting
the figure would leave that argument citing a number no longer in the frame.

### 2a. RESOLVED: the `-262` was not bookkeeping, and D0 removed it from the slice

**The framed question was whether `-262` could be treated as bookkeeping. It
could not, and the D0 it forced came back SEPARABLE, so the deletions left with
it.** This section is kept as the record of a question that was answered, not
renumbered against the as-built — renumbering would silently convert a resolved
D0 into a restated premise.

What the framed figure contained: `effect_v1.rs` carried **47 hunks** at the
backup tip, not confined to the promotion. Several rewrote `ResourceTableV1`
lifecycle internals — admission leases (`finish_admission`), slot states
(`Vacant`/`Closing`/`Retired`), and release readiness
(`ResourceReleaseReadinessV1`). **Those were a resource-table change sharing a
file with a file-acquisition change**, which is exactly the shape the operator
directive exists to break up.

**D0's determination, and what discharged it.** Static disjointness was
necessary and not sufficient — zero hunk overlap, no call dependency, no
signature coupling — and the deciding evidence was **compilation**: slice 4
built and its suites passed with none of the lifecycle cluster present. The
cluster is now [[RT-D5B-RESOURCE-TABLE-LIFECYCLE]]. The landed `-63` is the
file-acquisition surface's own deletions; the rest of the framed `-262` went
with the cluster.

⇒ **The general form, which is the part worth carrying to the next slice:** a
deletion count is not evidence of bookkeeping *or* of substance. It is a
question about what is being deleted, and the only instrument that settles it is
one that can fail — here, a build with the disputed material absent.

## 3. Deliverables

**D0 — SEPARABILITY, and it gates the rest of the cut.** Determine whether the
`ResourceTableV1` lifecycle hunks in `effect_v1.rs` are independent of the
`MappingAcquireFile` promotion. Report one of:

- **Separable** — then this WP is the promotion only, the lifecycle hunks
  become slice 5, and §2's numbers shrink. This is the preferred outcome and
  the one the directive points at.
- **Not separable, with the reason named** — the promotion depends on the new
  lifecycle states. Then they land together and D0's answer is the
  justification for a slice this size.

A bare "they're intertwined" does not discharge D0. Name the dependency: which
promotion line requires which lifecycle change.

**D1 — the host surface.** `abi_v1.rs`, `mapping_v1.rs`, `lib.rs`
(`resource_raw_fd_v1`), and the `px8-ds-test-support` feature in `Cargo.toml`.

**D2 — NOT the availability promotion.** The backup branch's version of this
cluster flips `MappingAcquireFile` to `NativeTested` and catalog row `0407` to
`native`. **Those edits are dropped from this slice** per §4's ruling; the
transplant is not a verbatim copy and the four sites in `AC-AVAIL` are what
says so. The control test in `AC-CONTROL` must be adjusted to assert the held
state, not deleted.

**D3 — prelude and interpreter.** The `PrivateMappingAcquireFile` global and
the `ken-interp/src/eval.rs` dispatch path.

## 4. RULED: LAND IT UNFLIPPED. The availability bit does NOT move here.

**Architect ruling `evt_3wtg8w8krmmt`, 2026-09-16.** The slice lands the
surface additively; `MappingAcquireFile` stays `RepresentedUnavailable` and the
catalog row `0407` stays `unavailable`. **The ruling is grounded in the
codebase's own stated criterion, not in a reviewer's caution** —
`ken-host/src/effect_v1.rs:249-251`:

    /// PX5's intended promotion set. Membership is a plan, not evidence: every
    /// operation remains `RepresentedUnavailable` until its artifact differential
    /// gates promote it explicitly.

⇒ Promotion requires an **artifact differential, per operation, explicitly.**
An unproven flip is not a judgment call that could go either way; it is the
thing that sentence forbids. `D4` (`cf894cdb5`) and `D5a-core` (`29f64ff6f`)
being merged is a reason to *expect* the differential to pass. It is not the
differential.

### 4a. The flip is FIVE coordinated sites, and two of them are compiler-enforced

**CORRECTED 2026-09-16. This section said FOUR and it was wrong.** The fifth was
found by runtime-implementer mid-slice (`evt_4v238vct498`), ruled in by the
Architect (`evt_4kxywcq3nzsyf`), and re-measured by the Steward at `origin/main`
`e11341c7b9d11cd74879d27d555d2a5729837847`. **`AC-AVAIL` is keyed to this list,
so the list being short made the AC short.**

    1  effect_abi_v1.catalog            25 native / 10 unavailable
    2  effect_v1.rs:193                 MappingAcquireFile => RepresentedUnavailable
    3  NATIVE_TESTED_TARGETS_V1    [HostOpV1; 25], MappingAcquireFile ABSENT
    4  static_transition/effects.rs:~624   10-op `=> None` arm, op NAMED
    5  effect_v1.rs:558-568   host_effect_wire_layout_v1's 10-op arm, op NAMED,
                              body is `return Err(OperationUnavailable(operation))`

All five agree at 10/25. Two enforce themselves, **but not in the way an earlier
draft of this paragraph said, and the difference matters.**

**Site 3's compiler enforcement is the LENGTH, not the membership.**
`NATIVE_TESTED_TARGETS_V1: [HostOpV1; 25]` is a fixed-length array type, so the
compiler rejects a candidate that adds an element without changing the `25`. It
does **not** check *which* 25. Swapping one op for another type-checks
perfectly. The membership claim is grounded **structurally** instead, and this is
the Architect's grounding rather than a second hand count: `HostOpV1` has
exactly **35** variants, the availability match has **no wildcard arm**, and the
unavailable arm names **10**. Exhaustiveness then forces the remaining **25** —
the partition is derived, not tallied. **Do not replace one hand count with
another here**; if the number is ever in doubt, re-derive it from the variant
count and the absence of a wildcard, which is a property a reader can check.

Site 4 is the one that enforces *membership*: it names its members rather than
wildcarding them — its own comment says *"naming them is what makes promoting one
to the admitted set a compile error here rather than an operation whose seats
silently answer `None`."* A partial promotion does not compile.

#### The real defect was the LIST, and the fix is a PREDICATE

**A list of four could not tell anyone it was missing a fifth.** `AC-AVAIL` was
keyed to an enumeration I measured once, so the AC's bar was silently whatever
my census happened to catch. The implementer found site 5 by building, not by
reading the frame — the frame had no way to say "and anything else meeting this
rule."

**Membership rule (this is the bar; the five above are its instances at
`origin/main` e11341c7b9d1, not the definition):**

> A site is availability-bearing for op `X` iff changing *only* `X`'s membership
> there changes whether `X` is represented as available — i.e. the construct's
> behaviour for `X` is *determined by* that membership.

**Its falsifier, which is what makes it checkable:** find a member whose removal
leaves `X`'s availability unchanged. Run it on the `:442` arm and it fails to
qualify — `:442` co-lists `ConsoleRead`, which is `native`, so membership there
is compatible with either availability and determines nothing. Run it on site 5
and it qualifies: the entire body is
`return Err(OperationUnavailable(operation))`, so membership *is* the unavailable
claim, and a control asserts exactly that for this op — **at `:6000-6005` as
landed**, inside
`abi_s6_d4_file_acquire_identity_and_unavailable_posture_are_pinned`
(`:5979-6006`). Earlier drafts of this frame cited `:5960-5965`; that was the
pre-landing coordinate and it now points inside a different test
(`abi_s6_d5a_promotes_the_atomic_anonymous_mapping_operation_set`). **The
coordinate decayed when the slice landed, which is the ordinary fate of a line
number in a frame — cite the enclosing test by name, and treat the line as the
perishable half.**

⇒ **Judge an arm by what its body means for the op, never by whether the op
appears in it.** Shape is not the criterion: sites 4 and 5 and the `:442` arm are
all multi-op `|` arms naming `MappingAcquireFile`, and only two of the three are
sites.

**`AC-AVAIL` is satisfied by the rule, not by the count.** A sixth site found
later is an instance of the same bar, not an amendment to it — surface it, do
not treat the list as closed.

**Site 5 splits into a PAIR of hunks and neither half is separable.** The flip
adds a real wire layout for the op *and* removes it from the unavailable arm.
Taking only the first yields an op with a layout that still refuses; taking only
the second yields an op that falls through to no layout. That is why this slice
takes neither, and why `host_effect_wire_layout_v1(MappingAcquireFile)` keeps
returning `OperationUnavailable` after it lands. **That is intended, not an
oversight** — the slice lands the types and dispatch plumbing while the op stays
unreachable **natively**, which is what an unflipped surface means.

> #### CORRECTED AS-BUILT: "unreachable" was NATIVELY unreachable all along
>
> This frame said "unreachable" unqualified, and the node's own title says
> *"unreachable from the interpreter until a later flip."* **That is false as
> landed**, and it is the adversary's Finding 1 (Architect ruling
> `evt_21f23zmgqfxsc`). The interpreter consults `availability()` **nowhere**, so
> after this slice a Ken program using `withMapping ... FileBacked` **succeeds
> interpreted and refuses natively.**
>
> **Every one of the five sites above is a native-availability site.** That is
> why the corrected membership predicate — a genuine improvement over the
> enumeration it replaced — still passes against this defect: the census was
> complete and answering a different question than it was read as answering.
> *Availability* asks "is this op marked and gated as unavailable?"
> *Reachability* asks "can a Ken program get here?" They coincided only for as
> long as the prelude stub refused, and this slice is where they came apart.
>
> **What the flip must know:** at flip time, changing site 2 **alone** opens the
> C ABI, and the interpreter path is already open. The remedy is the uniform
> refusal gate in `dispatch_host_op_v1` —
> [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] — which lands **before**
> [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] precisely so that site 2 becomes the
> single flip point rather than one of two acts, the second unwritten.
>
> **Architect F3 — the SAFETY-precondition split and the irreducible TOCTOU —
> is carried to [[RT-D5B-MAPPING-AVAILABILITY-FLIP]]**, not actioned here. It
> bears on what is safe to make *available*, not on what is safe to compile.

**Unrelated stale count, found while verifying this and recorded so the flip
slice does not inherit it.** Site 5's own comment (`:554-557`) reads *"the
twenty-two matched above are exactly the `NativeTested` set."* Measured at
`origin/main`: the arms above match **25** distinct variants, the unavailable
arm names 10, and `HostOpV1` has exactly 35 — a clean 25/10 partition, and
`NATIVE_TESTED_TARGETS_V1` is `[HostOpV1; 25]`. **The comment is stale by
three.** The mechanism it describes is sound (the correspondence is asserted by
name in tests, not left to two lists agreeing by coincidence); only the number
is wrong. Not this slice's to fix — it is a comment, and touching it would put a
non-surface edit in the file set — but whoever does the flip will be reading
that sentence while changing that arm.

**Site 4 lives in `cranelift_backend`.** That is a sequencing fact worth
carrying: **the later flip slice cannot be backend-free**, so it cannot be cut
while the HS18 surface is contested the way this slice can. This is the
structural reason the surface and the flip are two slices rather than one
reviewer's preference.

**One precision, from the Architect's census and worth repeating so nobody
over-reads it:** the `Op::MappingAcquireFile` occurrence at `effect_v1.rs:147`
and the arm at `:442` are **not** availability discriminators — `:442`
co-lists `ConsoleRead`, which is `native`. Only site 4's arm tracks the
unavailable set. An arm that merely mentions the op proves nothing about its
availability.

### 4b. This classifier has already failed once, in the safer direction

`effect_v1.rs:152-157` records it:

    /// This was a membership test against the native set with an `else`
    /// fallback, so a new operation was silently classified
    /// `RepresentedUnavailable` -- a plausible-looking default, which is what
    /// let it survive review.

That failure was *toward* unavailable and still earned a soundness comment and
an exhaustive rewrite. A flip on an unproven backend fails the other way: a
loud `OperationUnavailable` refusal becomes a native execution failure at a
site advertising support.

## 5. Acceptance

**AC-AVAIL (the bar). EVERY availability-bearing site for `MappingAcquireFile`
is unchanged.** The bar is §4a's membership rule, not a fixed count — a list
cannot report being incomplete, which is how this AC shipped keyed to four sites
when there are five. A one-site AC cannot see a partial promotion, and four
agreeing classifiers with one disagreeing is exactly this defect's shape.

The instances at `origin/main` `e11341c7b9d1`, **five of them**:

    1  effect_abi_v1.catalog       byte-identical to its origin/main blob
       control: git cat-file -e <cand>:crates/ken-host/effect_abi_v1.catalog \
                  && git cat-file -e origin/main:crates/...   FIRST, then compare
                git rev-parse <cand>:... == the origin/main blob
                (and row 0407 still reads `unavailable`)
    2  effect_v1.rs:193            MappingAcquireFile => RepresentedUnavailable
    3  NATIVE_TESTED_TARGETS_V1    still [HostOpV1; 25], MappingAcquireFile absent
    4  static_transition/effects.rs  MappingAcquireFile still named in the
                                      10-op `=> None` arm
    5  effect_v1.rs:558-568        MappingAcquireFile still named in
                                   host_effect_wire_layout_v1's 10-op
                                   OperationUnavailable arm; and
                                   host_effect_wire_layout_v1(MappingAcquireFile)
                                   still returns Err(OperationUnavailable),
                                   asserted in the test named below

**The `git rev-parse` in site 1's control FAILS OPEN and must be guarded.** Given
a path that does not exist at that rev it echoes its own input rather than
erroring, so an ABSENT file reads as DIFFERS — a false red, and worse, a shape
that reads as a genuine finding. Run `git cat-file -e` on both sides first; only
then is a `rev-parse` inequality a measurement.

Counts stay at **10 unavailable / 25 native** on every side, and that partition
is derived from exhaustiveness over 35 variants with no wildcard arm — see §4a;
do not re-tally it by hand. Sites 3 and 4 are compiler-enforced, **but note what
each compiler-enforces**: site 3's array type pins the *length* only, so the
membership half of site 3 rests on `:5999`
(`!NATIVE_TESTED_TARGETS_V1.contains(&MappingAcquireFile)`) and site 5's on
`:6000-6005`, both inside
`abi_s6_d4_file_acquire_identity_and_unavailable_posture_are_pinned`. **Those two
assertions are the sole non-compiler guard for sites 3 and 5.** Cite the test by
name; the line numbers moved once already when this slice landed.

**Site 5's held state is explicitly in scope, and its consequence is intended.**
After this slice, `host_effect_wire_layout_v1(MappingAcquireFile)` still refuses:
the slice lands the types and the dispatch plumbing while the op remains
unreachable through the wire layout. A reviewer should confirm that as the
intended meaning of an unflipped surface, not read it as an omission.

**If a sixth site is found, it is in scope by the rule and does not need this
frame amended** — report it, hold it, and say so in the PR body under `AC-D0`.

**The flip is a later slice and its AC is the artifact differential the code
already names** (`effect_v1.rs:249-251`), not a reviewer's assessment that the
backend looks ready. That slice necessarily touches `cranelift_backend` (site
4), so it cannot be cut while the HS18 surface is contested.

**AC-CONTROL. The promotion's own control test rides with it.**
`abi_s6_d5b_promotes_only_file_acquisition_from_the_unavailable_tail`
(`effect_v1.rs:5411` on the backup tip) is **absent from `main`** — measured 0
hits. It must be present and passing in the candidate, and under the **hold**
branch of `AC-AVAIL` it must be adjusted to assert the held state rather than
deleted. A control that is dropped because it contradicts the chosen branch is
the defect, not the fix.

> **CORRECTED: this AC presupposed its own remedy, and the correct discharge was
> ZERO TEST CHURN.** "It must be adjusted to assert the held state" assumes an
> adjustment is owed. It was not: the base's tests **already** assert the held
> state, so under the hold branch the right action was *none*, and the candidate
> landed with no churn in that control. **That is strictly stronger than an
> adjustment** — an untouched control that still passes is evidence about the
> candidate, whereas a control edited in the same diff it is meant to check is
> evidence about the editor.
>
> The general form: **an AC that names the remedy cannot report that no remedy
> was needed.** Phrase the criterion on the state to be true at the end
> (*"the control asserts the held state and passes"*), never on the act the
> author expects to be performed — otherwise "nothing needed doing" reads as
> a missed deliverable.

**AC-NO-BACKEND. The candidate's file set contains no cranelift path.**
Control: `git diff --name-only origin/main..<cand> | grep -c cranelift` is `0`.
This is what makes the slice reviewable without the HS18 closure, and it is the
reason the slice exists.

**AC-D0. D0's determination is recorded in the PR body** with the named
dependency or the named independence. If separable, the lifecycle hunks are
**not** in this candidate.

**AC-NO-REGRESSION. Workspace-green in CI**, not a local `--workspace` run.
Local verification is targeted only: `scripts/ken-cargo` scoped to
`-p ken-host`, `-p ken-elaborator`, `-p ken-interp`.

## 6. Contention

`crates/ken-host/`, `crates/ken-elaborator/src/prelude.rs`,
`crates/ken-elaborator/src/compiler_driver.rs`, `crates/ken-interp/src/eval.rs`.

`prelude.rs` and `compiler_driver.rs` are the two files outside Runtime's usual
surface.

**CONTENTION CHECK RUN BY THE STEWARD, 2026-09-16. NO LIVE CONTENTION — and
the first pass of this check got it wrong in the alarming direction, so the
refutation is recorded rather than the finding quietly dropped.**

A branch scan surfaced `wp/LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD` at
`b7a9101ac026fcafa8cc6bac0ebfc0c88686dc31` touching `prelude.rs`. **That ref is
a fossil:** its node reads `status: merged` on `origin/main` and the branch is
**273 commits behind** main. A branch ref is not a live candidate; the node's
status is the instrument, and the branch scan is not.

The guard is therefore already **on** `main`, which makes the real question a
precondition rather than a race — and it is answered:

    guard_constructor_spelling (data.rs:131) fires only when
      globals.get(name) exists AND env.constructor(existing_id).is_some()
      AND the id is not one of the declaration's own constructors

⇒ It rejects a new **constructor** whose spelling collides with an existing
**constructor**. `PrivateMappingAcquireFile` is a primitive global, not a data
constructor, so it does not reach the guard's predicate.

**And the transplant-signature worry is refuted too.** The guard added a
`&mut elab.ctor_decl_spans` parameter to `elab_data_decl`, and this slice's
base (`4bf1ad362`) predates it — so a transplant that touched a data
declaration would fail to compile against current `main`. It touches **zero**
`elab_data_decl` call sites (measured). Nothing to carry.

**No obligation transfers to any other candidate.** Recorded because the first
answer was "textually clean, semantically not" and the measurement says
otherwise; a contention warning left standing in a released frame costs the
ring real time chasing a hazard that is not there.

## 7. Why this slice and not the POSTCALL subject

The obvious alternative cut is the code
`RT-D5B-POSTCALL-REFUSAL-MECHANISM` is held on —
`fn checked_ih_post_call_residual`, which reads **0 on `origin/main`** and
**1 on the backup tip** (positive control: `impl` reads 45 in main's copy of
that file, so the absence is real and not an unresolved path). Landing it would
clear that node's base hold.

**It is not the next slice, because it is not small.** The symbol and its five
call sites all live in `cranelift_backend/lowering/core.rs`, which is one of
the branch's largest changed files and sits inside the contested HS18 surface.
Cutting it means cutting into the closure the Architect is still amending
(amendment 8, `388bcd8d3`). The host cluster is the piece that is both
independent and finishable, which is what the directive asks for.

`RT-D5B-POSTCALL-REFUSAL-MECHANISM` correctly stays `ready` and base-held. Its
`MECH-2` deliverable is answered and closed; the node is held on its base, not
its content.
