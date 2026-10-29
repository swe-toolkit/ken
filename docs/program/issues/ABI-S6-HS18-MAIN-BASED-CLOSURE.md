---
id: ABI-S6-HS18-MAIN-BASED-CLOSURE
title: "Close ABI-S6 HS18 on a main-based line, in three increments. Preserves the verifier substance the Architect protected by name (units.rs +4083, re-derived against main's five moved files) and drops the MappingAcquireFile capability grant they refused, which the preserved checkpoint carries across five coordinated sites. Increment A is the protected verifier substance; B extracts Q1's resume-exit repair, which never landed; C is amendment 8's consumer relocation."
status: merged
owner: runtime
size: L
gate: none
depends_on: [ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT, ABI-S6-HS18-D5B-SUBSTRATE-PORT]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16, executing the operator's 2026-09-16 direction ('do not build on an unmerged commit... the base commit needs to be on main') after the Architect ruled RE-DERIVE rather than rebase (evt_ma144e8mt7sn) and narrowed their own no-revert by name. Supersedes the circulating name ABI-S6-HS18-CLOSURE-AMENDMENT-8, which runtime-qa read a kickoff against (evt_2e58kcb0zd4h9) and which never had a file behind it. CUT FROM origin/main 6f49f852141a66571c6126a569b954f63e2b6bde on 2026-09-16 -- this is a provenance RECORD of where the node was cut and must never be re-pointed; increment A's OPERATIVE base is a separate value living in frame 4a-pin and MAY DIFFER from this SHA -- it already does."
---

> # CLOSED 2026-10-29: increment C ruled (Z), already satisfied.
>
> The Architect's AC-C0 (`evt_fge5r2byetm2`, base `610a2d84a`) found the
> following on main:
> - the frame writer is deleted, and no lowering site emits a detached
>   consumer;
> - the one detached row (`rt_write_writable_stage`) returns the producer's
>   result to the creation-site seat, with realization checked on the
>   finished CLIF and native and interpreter traces equal.
>
> C closes without code, and HS18 closes with it, with increments A
> (`bd5c64ed1`) and B (`afc5e0fae`) merged. Control 1 (the `0x0f09` tag query
> on a file-backed Mapping) is not observable on main and moves to
> `RT-D5B-MAPPING-AVAILABILITY-FLIP`. The census in frame §6e is superseded by
> the ruling's §1.

> # INCREMENT C, released after RT-CERTIFIER-FRAME-CALL-EXEMPT-CALLER-PIN.
>
> This banner supersedes the banners below wherever they conflict.
> Increments A (`bd5c64ed1`) and B (`afc5e0fae`) are merged. C is amendment
> 8's consumer relocation (frame §6). It is L, T1, gate architect. Its base is
> the main named at kickoff, written into frame §4a-pin.
>
> - **The census moved.** Frame §6e was measured before A landed. On
>   `fd2f57557` over `crates/`, `constructed_context_frame` has 0
>   occurrences (§6e: 11), `detached_return_context` has 12 (§6e: 0), and
>   `detached_post_call_consumer_frames` has 4 (§6e: 0). Amendment 8's
>   writer and readers (`core.rs:12081`, `calls.rs:1165`, `core.rs:14827`)
>   were measured on WIP `b601e2ec7`, not on main.
> - **AC-C0 first (D0, Architect, on the kickoff base).** Re-measure
>   amendment 8's census on main: the frame writer, its readers, and every
>   site that emits a detached post-call consumer. Rule one of:
>   - **(R) relocate:** a wrong-site consumer replay exists on main. Name
>     the creation-site function and the producer-side site, and the
>     implementer relocates per §6 with all five §6b controls.
>   - **(Z) already satisfied:** main has no producer-side consumer replay,
>     because A's static-response route realizes the exact call word at the
>     owner (`evt_3adhrn49d55h3`). Name the measured witness for control 2
>     (the call is satisfied at the creation site) and control 3 (no
>     wrong-site replay). C then closes without code, and HS18 closes with it.
> - **Fences.** §6c's not-authorized list holds in full. A ruling that
>   needs a new carrier, phase tag, second planner relation or runtime
>   object is a stop for the operator, not a widening.
>
> Checks: 3 fired: the four §6e spellings and `constructed_context_frame`
> were recounted over `crates/` on `fd2f57557`, and three counts moved, so
> the frame's census is not reused. 8 fired: (Z) is accepted only with the
> control-2 and control-3 witnesses, which a main that still replays at the
> producer site would fail.

> # ACTIVE 2026-10-27: INCREMENT A REBASES DIRECTLY ONTO MAIN.
>
> This banner supersedes every banner below wherever they conflict,
> including the BLOCKED banner. `ABI-S6-HS18-D5B-SUBSTRATE-PORT` closed
> without a port (concept D0 `evt_pkj06ks8sv44`), and
> `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT` is merged. Kicked
> `evt_5kne1n2vh7hk1`, amended `evt_7a0nvvb4f7y1y`. L, T1, gate architect.
>
> **Increment A** is `git diff 30d35f625 5d977ac79` minus the AC-1e
> acceptance file, rebased onto current main.
>
> - The unit is A's delta applied to main's body, per fn. Never take a
>   whole `5d977ac79` body, which re-imports the prefix context. Never
>   resolve per hunk (`evt_387nkc09gkmyt` point 1).
> - A changes 81 fns: 64 new, 16 main-moved and present on main (calls.rs
>   2, core.rs 3, core tests 6, mod.rs 1, units.rs 4), and one prefix-only
>   fn, `Lowering::validate_checked_ih_detached_result_shape`. The 38
>   increment-1 conflict hunks fall inside the 16. Row 17's delta has no
>   host on main and drops; it is never re-hosted by importing the prefix
>   fn or `post_call_consumer`. No lowering path attaches a detached-return
>   consumer to a transport result (`ClaimedContinuationResult` carries
>   none); detached demands reach lowering only as static-response selected
>   callers, whose owner emission realizes the exact call word (Architect
>   `evt_3adhrn49d55h3`). If one does, stop and post to the Architect.
> - The result contract's planner fn is already on main, identical. A's
>   rewrite of its lowering half is A's substance and lands here.
> - Calls into `backend_module` and `unsupported` take main's
>   `LoweringCause` variant.
> - The refused grant stays out: zero added lines name
>   `MappingAcquireFile` or `mapping_acquire_file`, and the six-site
>   exclusion holds under frame §4b and §4d, with the base pinned by §4a-pin.
>   The `preserve/` refs are evidence, never bases.
>
> **Acceptance.**
>
> - AC-1: `scripts/ken-cargo check -p ken-runtime --lib` and
>   `--profile test` are both RC 0.
> - AC-2: A's own pins pass on the rebased tree, each with a base control
>   on main, plus main's `rt_parity_native
>   checked_ih_generated_entry_admission_population_is_total`. A's
>   acceptance file stays AC-1e DEFERRED.
> - AC-3 (amended per Architect `evt_3qbh84mhezt6x`): QA reverts one
>   main-moved fn to A's whole body, adding only interface shims that
>   supply no main-side behaviour: S1, an unused parameter main added; S2,
>   a fixed `PlannerInvariant` refusal arm for a variant main added; S3,
>   `None` for an Option field main added; S4, main's cause at an error
>   constructor whose arity main changed. Anything else makes the row
>   inadmissible. The designated mutant is `transfer_constructor_operands`
>   (`local/architect-shared/main-based-closure/ac3-mutant/ac3-mutant.patch`,
>   sha256 `dbaa29da…`), with pin `rt_parity_native
>   generated_entry_capsule_outer_carried`. The mutant must compile (RC 0
>   on the test profile), and the pin must be green on the unmutated tip
>   and red on the mutant. `transfer_unit_result_into_carrier` is
>   inadmissible.
>
> **Base and deferral (Steward `evt_3ssd5awf9p3v4`).** A's operative pin is
> the main its candidate is cut from (`b8fb28e3f` at the `bdf0e4755`
> handoff), adopted by §4a-pin's empty-`crates/`-diff condition at
> handoff. Frame §4a-pin holds it as a literal, re-pinned at each no-rebase
> main merge (now `c4c8a0ec5`, the cut of `5ec884387`; DOC-BUNDLE-315). Runtime
> QA `evt_7tnep6swjbag0` showed a computed merge-base breaks §4b and §8.
> Older literals elsewhere are records. R1's per-mutation pairing
> (`generated_result_path_proof_rejects_each_certificate_corruption`) lives
> only in the AC-1e file, so it is deferred with AC-1e to the increment
> that restores that file.
>
> The Architect reviews the per-fn adjudication of the 16 before the
> implementer finishes them. Increments B and C are framed after A lands.

> # BLOCKED AGAIN, on a SECOND dependency. `ABI-S6-HS18-D5B-SUBSTRATE-PORT` LANDS FIRST.
>
> **`status:` back to `draft`, Steward 2026-09-17, SAME DAY as the flip below
> and for a different reason.** The port dependency is genuinely discharged —
> that part of the banner stands and is not retracted. What was discovered
> after it is that **increment A has a second, previously unnamed
> dependency**, and `ready` is what a team pulls on.
>
> **The premise that broke is `§2a`'s DECOMPOSITION, not the objective.**
> Increment A was sized as *"9 files, `units.rs` +4083"* — a diff between two
> points **on the preserved line**, which measures what A ADDED to the
> checkpoint, never what A NEEDS on `main`. Increment A is the top of a
> six-commit stack whose bottom four never landed.
>
> ⇒ **A measurement correct on one tree becomes a cross-tree claim only through
> a carry argument, and the carry is what fails.** Read
> `ABI-S6-HS18-D5B-SUBSTRATE-PORT` before re-planning this node; do not
> re-derive the finding here.
>
> Ruled by the Steward (`evt_2dgjc1h7as88w`) on `runtime-leader`'s measured fork
> (`evt_1yma657vq6bkg`), with both extent questions answered by the Architect
> from measurement (`evt_6fr79bx4cjhsw`). **The disposition is NEITHER of the
> two that were offered:** widening A to carry the substrate makes A the whole
> substance again — which the increment split exists to prevent, as this banner
> says below — and re-sequencing behind B fails because B supplies `source.rs`,
> one row of an eight-row table.
>
> ## The port dependency, which IS discharged
>
> **`status:` `draft` -> `ready` was correct on 2026-09-17, on the condition this
> banner itself set.** `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT` landed as squash
> `10e75cb93656d5ea787bceaf754b2500b78de166` (candidate
> `93fa1d2639144b89f432211c8d606dc4cd229117`).
>
> **Measured by BLOB COMPARE, which is what the condition below demanded — not
> by the approval.** All ten of the candidate's paths are byte-identical to
> `origin/main`, each guarded with `git cat-file -e origin/main:<path>` before
> the compare, because `git rev-parse <sha>:<path>` fails open: it exits non-zero
> and still echoes its input, so an ABSENT file reads as DIFFERS. Ancestry was
> not consulted and must not be — the publisher squashes, so a routed commit is
> never an ancestor of `main`.
>
> **The condition, preserved verbatim because it is the reason this node was
> right to sit at `draft` for a day:**
>
> > **`status:` corrected `ready` -> `draft`, Steward 2026-09-16.** This banner
> > said BLOCKED while the frontmatter said `ready`, and `ready` is what a team
> > pulls on. `check-issue-schema` had been reporting the contradiction on every
> > run -- *"depends_on ... is 'ready' (nothing has landed) -- a team pulling
> > this node will find its premise false"* -- and the Steward dismissed it twice
> > as pre-existing. It is not pre-existing to anyone; it is this lane. **Flip
> > back to `ready` when the port LANDS, measured by blob compare, not when it is
> > approved-as-built.**
>
> **What is now true that was not:** increment A's remaining compile errors were
> *"planner machinery `main` never grew"*. `main` has now grown it — the landed
> squash carries `continuations.rs` +1057 and `responses.rs` +1210 among ten
> files in `cranelift_backend`. **Re-measure the error set against the landed
> `main` before planning; do not carry the 21 / 11 / 10 split below**, which was
> taken against a `main` that predates the port.
>
> **Added 2026-09-16 on the Architect's category-B ruling
> (`evt_6mptkrtvysd8s`).** Increment A's D0-2 characterisation found 21 compile
> errors splitting 11 / 10: the eleven were incomplete re-derivation and are
> **now fixed** (`bd3e1ff676444f48771d261371982a8f289ce26a`, still not green,
> not a candidate). **Every remaining error is planner machinery `main` never
> grew**, which increment A holds the call sites for and none of the
> definitions.
>
> **The ordering is structural, not a preference: increment A cannot compile
> without that node.** It is therefore not increment C (C is sequenced after A,
> so A would never reach green and its build gate could never fire) and it is
> not folded into A (that would make A the whole substance again, which is what
> the increment split exists to prevent).
>
> **Increment A's exit criterion is ZERO ERRORS, never "the eleven are gone."**
> The category-B inventory was already short by one — `constructor_identity`,
> found only by attempting a fix — so it may grow when re-attempted. That is
> expected under the predicate and is not a defect in it.
>
> B and C are unaffected.

> # Frame: `docs/program/wp/ABI-S6-HS18-MAIN-BASED-CLOSURE.md`.
>
> **Not releasable while the banner above stands.** The pin below is current and
> correct; it is not what blocks.
>
> **Increment A's base is `origin/main` `c4c8a0ec5bb4af4c1a9a8e0d9311e57066ea4811`**
> (re-pinned 2026-10-28 to the cut of `5ec884387`; frame §4a-pin)
> — B and C pin their own bases in frame 4a-pin, which is the only place
> either value is written.
>
> **PIN MOVED 2026-09-17, `537fa2afbdab8962717d43640afbd04719d1b258` ->
> `10e75cb93656d5ea787bceaf754b2500b78de166`.** The port node
> `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT`
> landed ten files in `crates/`, so the old value could no longer satisfy the
> empty-`git diff` condition below — **the pin did not rot, its own test
> refused it**, which is the whole reason that condition is a measurement taken
> at adoption rather than an argument quoted from when it was proposed.
>
> **This banner is a PIN, not a record**, by frame §4a-pin's own predicate: *an
> occurrence is a PIN if and only if it asserts what increment A's base IS.*
> It therefore moves with §4a-pin and must move again at increment B. The
> `origin:` line in this file's frontmatter is the opposite case — a provenance
> RECORD of where the node was cut — and **must never be re-pointed.** The two
> are the same forty characters and they age in opposite directions.
>
> Flagged by `runtime-implementer` (`evt_f51swjy5k7rf`), who moved the frame's
> two pins and deliberately left this one rather than edit a file they do not
> own. Correct call: the third pin is the one every enumeration of this has
> missed. **The pin need not equal the cut point**; it must
> satisfy frame 4a-pin's three conditions, of which the operative one is that
> `git diff "$BASE" HEAD -- crates/` is empty, re-run at the moment the pin is
> adopted. Requiring `pin == cut point` regresses: every commit that corrects
> the pin moves `main`. The two
> `preserve/` refs are **EVIDENCE, NOT BASES**. Their `-not-a-candidate`
> names stay, and neither may become an ancestor of a candidate.
>
> **The preserved checkpoint carries a REFUSED capability grant.** Read frame
> §4 before touching either tree. `5d977ac79` promotes `MappingAcquireFile`
> across five coordinated sites — the flip the Architect refused in `z4664` and
> refused again under the confirmation-protocol ruling. **A clean rebase was
> the worst outcome available**: the grant landing with no conflict to stop it.
>
> **Three increments, each cut from the `main` of its own moment.** The frame's
> §2 has the measurements that put them in that order, including the one that
> inverts what the commit sequence suggests.

# Objective

Close HS18 on a line based on `main`, preserving the verifier substance the
Architect protected and dropping the capability grant they refused.

The mechanism is not re-decided here. Amendment 8
(`docs/program/ABI-S6-HS18-closure-mechanism-amendment-8.md`) is **CURRENT**
and is the ruling this node implements; frame §6 reproduces its rule, its five
controls and its not-authorized list in full, because a pointer is what a
deferral evaporates through.

# The three increments

    increment  substance                               extent                  order
    A          verifier substance (the PROTECTED part)  9 files, units.rs +4083  first
    B          Q1 resume-exit repair                    source.rs 935 lines      second
    C          amendment 8's consumer relocation        not yet written          third

**A precedes B, which inverts the commit sequence.** The verifier substance
makes zero textual reference to Q1's three types
(`SourceMachineExit`, `SourceDynamicMatchRequest`,
`SourceDynamicMatchScrutinee` — 0 diff lines each across
`30d35f625..5d977ac79`). **Three zeroes prove no textual dependency, not
semantic independence** — that is increment A's D0, answered by witnesses run,
not by a grep.

# REFERRED IN: increment A's re-derivation DROPPED A BINDING

**Recorded by the Steward 2026-09-16 from the port ring's referral
(runtime-implementer `evt_5vqstf7w037sh`, ruled by the Architect
`evt_2qqye3tdnh4b1`). It is filed here because increment A is this node's, and
the ruling was that the port must NOT repair it.**

    error[E0425] cannot find value `constructor_identity`
                 lowering/core.rs:13266, in increment A's re-derivation
    constructor_symbol_identity      52 refs / 12 files, RESIDENT on `main`

⇒ **The producer is on `main` and byte-identical. The defect is a USE without
its `let`** — increment A's own re-derivation dropped the binding. `§2a` called
this cause unestablished; it is established now.

**Why the port was forbidden to fix it, and why that matters here.** Repairing
it there would put a fix on `main` whose motivating error is not on `main`, and
**would hide from this node's own review that the re-derivation dropped a
binding.** That is a finding about increment A's METHOD, not a compile error to
clear — it is the one measurement anyone has of whether re-derivation loses
things, and it must be read that way when increment A is built.

**It is the residual of `11 -> 1`**: increment A against the ported tree went
from eleven errors to one, and the one left is this. **The port is not what is
blocking increment A.**

# Q1 never landed, and that changes what a control means

Amendment 8's control 5 pins `lowering/source.rs` to blob
`38ec787dac261f02d46463ee1e9fca555c76d694`. That blob is on **every unlanded
tree and on none of the landed line**; `main` and `b4c8df33a` both carry
`7eff841d21c91cb12b2356892b6cef32288f971e`, and `source.rs` has not been
touched on `main` since — so this is not drift.

⇒ **On a main-based cut, control 5 is not a preservation control. It is a
deliverable**, and it is increment B. Read as written against `main` it is
unsatisfiable, and its failure would read as a regression rather than as a
missing prerequisite.

# The refused flip is a HAZARD on the preserved refs, not a head start

Five coordinated sites, four additions and one deletion. **Site 3 — the
`effect_v1.rs` ten-op refusal arm losing `MappingAcquireFile` — produces no
line to grep for**, and it is the site that punches a hole in
[[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]], which landed this session, by
subtraction.

The gate is a diff predicate keyed on the **operation** rather than on any site
roster, at pathspec `crates/` (frame §4b). The pathspec matters: at
`crates/ken-host/` it misses 27 of 42 lines, because the population spans three
crates.

**It is a CANDIDATE GATE and retires with the candidate** (Architect,
`evt_7jcex53r71gn6`). The derived alternative was measured against the
known-bad tree and **passes** there — the flip edits both sides, so every
internal relation stays true. A consistency invariant cannot detect a
coordinated unauthorized change.

# Not this node

- Any promotion of `MappingAcquireFile` — that is
  [[RT-D5B-MAPPING-AVAILABILITY-FLIP]], blocked on a differential that does not
  exist.
- A rebase, port or transplant of either preserved tree.
- Re-opening the mechanism. Amendment 8 is current.
- Any revert of the entry-18 verifier repair.
- The durable relation test for the coherent-but-partial hazard, which files
  separately.

# Contention

`crates/ken-runtime/src/cranelift_backend/` — principally `lowering/units.rs`,
`lowering/mod.rs`, `lowering/core.rs`, `lowering/calls.rs`, and for increment B
`lowering/source.rs`. Touches `effect_v1.rs` only in the negative. Check live
node status at `origin/main`, not for a branch ref, before releasing each
increment.

# Related

- [[ABI-S6]] — the parent node; HS18 is its current live hard stop.
- [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] — owns the promotion this node refuses
  to carry.
- [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] — the gate site 3 holes by
  subtraction.
