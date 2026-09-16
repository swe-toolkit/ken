---
id: RT-D5B-RESOURCE-TABLE-LIFECYCLE
title: "Slice 5 of the ABI-S6-d5b drain: transplant the ResourceTableV1 lifecycle cluster (ResourceReleaseReadinessV1, begin_release, and finish_admission's signature change with its callers) from the backup branch. Separable from the file-acquisition surface -- confirmed statically (zero hunk overlap, no call dependency, no signature coupling) AND by compilation: slice 4 built and passed with none of this cluster present."
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-D5B-HOST-FILE-ACQUISITION-SURFACE]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16 on the D0 separability question the slice-4 frame posed. runtime-implementer measured it (evt_740y400tqw54k) and the Architect accepted (evt_1w5eqhrcm9p2h). Backup tip 0d94d58b60e2e7bb045d7204efd138e17df68b7b against merge-base 4bf1ad362b5a5cfa512636087df5229ee57db706: effect_v1.rs has 47 hunks in three clusters -- A availability flip (3 + 3 shared with B), B file-acquisition surface (9 + 3, landed as slice 4), C this lifecycle cluster (6). The Steward held this cut until the compile test returned, because static disjointness is necessary and not sufficient; it returned separable, so the condition is met. Re-measure the cluster against the CURRENT backup-vs-main delta before framing -- slice 4 has landed since the 47-hunk census was taken."
---

> # DRAFT. Not framed, not released. Do not start.
>
> The separability question is answered; the scope is not yet measured against
> a post-slice-4 `main`. The Steward frames and releases this when the runtime
> lane reaches it.

# Objective

Transplant the `ResourceTableV1` lifecycle cluster from the ABI-S6-d5b backup
branch: `ResourceReleaseReadinessV1`, `begin_release`, and `finish_admission`'s
signature change together with the callers that must follow it.

# Separability, and why it is settled rather than assumed

The slice-4 frame posed this as a D0. It was answered three ways, and the third
is the one that counts:

    hunk overlap        B and C hunks                       0
    call dependency     B's added lines calling a C symbol  NONE
    signature coupling  every caller that must follow C's finish_admission
                        change is inside C's OWN hunks -- no B hunk calls it
    COMPILATION         slice 4 built and its suites passed with NO part of
                        this cluster present

**Static disjointness is necessary and not sufficient**; the compiler is what
can see a dependency text analysis cannot. The implementer proposed that test
themselves and committed in advance to reporting a named dependency rather than
widening the slice to make it build. It compiled clean, so the cluster stands on
its own.

One near-miss recorded because it bears on how this cluster gets re-measured:
the implementer's first classifier keyed on the literal `MappingAcquireFile`
token and reported 33 hunks as "neither," because most of cluster B is spelled
`try_new_mapped_file` / `resource_map_file` /
`mapping_acquire_file_source_rights`. **The residual bucket caught it, not the
classifier.** Keep a visible unclassified count on any re-census here; a
projection that is partial in the same direction as its own blind spot cannot
report its own miss.

# Not this node

- The availability flip (cluster A). It stays held; see
  [[RT-D5B-HOST-FILE-ACQUISITION-SURFACE]] §4a for the five availability-bearing
  sites and the membership rule that defines them.
- The capability-rights relaxation found during slice-4 construction. That is
  [[RT-D5B-MAPPING-WRITABLE-RIGHTS-RELAXATION]] and it is a ruling question, not
  work.
- The 23 test hunks in the backup's `effect_v1.rs` cluster. The implementer
  measured every one of them as belonging to the **flip**, so dropping the flip
  drops them. If a re-census finds test hunks that belong to this cluster
  instead, that is a finding — report it rather than absorbing them.

# Sizing / tier

**Size S, tier T1.** Six hunks in one file is small, and the review is not
byte-faithfulness: it turns on resource-lifecycle invariants — admission and
release ordering, and what `finish_admission`'s new signature obliges of its
callers. A transplant whose diff is small and whose argument is about
invariants is T1 work at S size.

# Contention

`crates/ken-host/src/effect_v1.rs`, shared with slice 4 and with the future flip
slice. Slice 4 is its `depends_on` and must land first — check its node
`status:` at `origin/main`, not for a branch ref. Zero cranelift expected;
confirm rather than assume at framing time.
