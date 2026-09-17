---
id: ABI-S6-HS18-MAIN-BASED-CLOSURE
title: "Close ABI-S6 HS18 on a main-based line, in three increments. Preserves the verifier substance the Architect protected by name (units.rs +4083, re-derived against main's five moved files) and drops the MappingAcquireFile capability grant they refused, which the preserved checkpoint carries across five coordinated sites. Increment A is the protected verifier substance; B extracts Q1's resume-exit repair, which never landed; C is amendment 8's consumer relocation."
status: ready
owner: runtime
size: L
gate: none
depends_on: [ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16, executing the operator's 2026-09-16 direction ('do not build on an unmerged commit... the base commit needs to be on main') after the Architect ruled RE-DERIVE rather than rebase (evt_ma144e8mt7sn) and narrowed their own no-revert by name. Supersedes the circulating name ABI-S6-HS18-CLOSURE-AMENDMENT-8, which runtime-qa read a kickoff against (evt_2e58kcb0zd4h9) and which never had a file behind it. CUT FROM origin/main 6f49f852141a66571c6126a569b954f63e2b6bde on 2026-09-16 -- this is a provenance RECORD of where the node was cut and must never be re-pointed; increment A's OPERATIVE base is a separate value living in frame 4a-pin and MAY DIFFER from this SHA -- it already does."
---

> # UNBLOCKED 2026-09-17. The port LANDED; increment A is releasable.
>
> **`status:` `draft` -> `ready`, Steward 2026-09-17, on the condition this
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

> # READY. Frame:
> `docs/program/wp/ABI-S6-HS18-MAIN-BASED-CLOSURE.md`.
>
> **Increment A's base is `origin/main` `10e75cb93656d5ea787bceaf754b2500b78de166`**
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
