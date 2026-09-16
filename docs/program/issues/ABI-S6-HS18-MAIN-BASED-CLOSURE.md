---
id: ABI-S6-HS18-MAIN-BASED-CLOSURE
title: "Close ABI-S6 HS18 on a main-based line, in three increments. Preserves the verifier substance the Architect protected by name (units.rs +4083, re-derived against main's five moved files) and drops the MappingAcquireFile capability grant they refused, which the preserved checkpoint carries across five coordinated sites. Increment A is the protected verifier substance; B extracts Q1's resume-exit repair, which never landed; C is amendment 8's consumer relocation."
status: ready
owner: runtime
size: L
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16, executing the operator's 2026-09-16 direction ('do not build on an unmerged commit... the base commit needs to be on main') after the Architect ruled RE-DERIVE rather than rebase (evt_ma144e8mt7sn) and narrowed their own no-revert by name. Supersedes the circulating name ABI-S6-HS18-CLOSURE-AMENDMENT-8, which runtime-qa read a kickoff against (evt_2e58kcb0zd4h9) and which never had a file behind it. CUT FROM origin/main 6f49f852141a66571c6126a569b954f63e2b6bde on 2026-09-16 -- this is a provenance RECORD of where the node was cut and must never be re-pointed; increment A's operative base is the same SHA and lives in frame 4a-pin, which is what increments B and C edit."
---

> # READY. Frame:
> `docs/program/wp/ABI-S6-HS18-MAIN-BASED-CLOSURE.md`.
>
> **Increment A's base is `origin/main` `6f49f852141a66571c6126a569b954f63e2b6bde`**
> — B and C pin their own cut points in frame 4a-pin, which is the only place
> either value is written. The two
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
