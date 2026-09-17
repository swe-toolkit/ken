---
id: ABI-S6-HS18-D5B-SUBSTRATE-PORT
title: "Port the D5b prefix's PRODUCTION residue onto main so ABI-S6-HS18-MAIN-BASED-CLOSURE increment A has a substrate to stand on. Increment A was decomposed as a diff between two points on the preserved line, which measures what it ADDED to the checkpoint rather than what it NEEDS on main; the four prefix commits below it never landed. Scope is the ~117-item prefix-minus-main gap NARROWED to its production cluster (generated-context-result authority, checked-IH post-call/detached, recursive-position calls, source dynamic match, the absent acceptance test), not the raw +10472/-5307. Excludes the refused MappingAcquireFile arm BY CONSTRUCTION -- it is RT-D5B-MAPPING-AVAILABILITY-FLIP's, deliberately held."
status: ready
owner: runtime
size: L
gate: none
depends_on: []
blocks: [ABI-S6-HS18-MAIN-BASED-CLOSURE]
github: null
tier: T1
origin: "Steward cut 2026-09-17, ruling runtime-leader's measured scope fork (evt_1yma657vq6bkg) after the Architect answered both extent questions from measurement (evt_6fr79bx4cjhsw). The fork offered two dispositions -- re-scope increment A, or re-sequence behind B -- and the ruling is NEITHER (evt_2dgjc1h7as88w): widening A makes A the whole substance again, which the increment split exists to prevent, and B supplies source.rs, one row of an eight-row table. This node is the third disposition, a prerequisite increment in front of A."
---

> # READY. Frame: `docs/program/wp/ABI-S6-HS18-D5B-SUBSTRATE-PORT.md`.
>
> **This node exists because a DECOMPOSITION was refuted, not because the
> objective changed.** `ABI-S6-HS18-MAIN-BASED-CLOSURE` `§2a` decomposed
> increment A as *"9 files, `units.rs` +4083"* — a diff between two points on
> the preserved line (`30d35f625` -> `5d977ac79`). That is sound arithmetic
> about **that line** and it is not a claim about `main`; the frame never
> supplied the carry argument that would make it one.
>
> ⇒ **A measurement correct on one tree becomes a cross-tree claim only through
> a carry, and the carry is what fails.** Increment A's objective — preserve the
> protected verifier substance, drop the refused grant — is untouched.
>
> **Increment A is the top of a six-commit stack whose bottom four never
> landed:**
>
>     b0ee2b048  D5b WIP: expose file-backed native path
>     fee133142  D5b WIP: align offset-less file source
>     37390dfcf  D5b WIP: plan immediate bridge responses
>     30d35f625  D5b: exit source frame before outer resume     <- prefix TIP
>     01d2ccb11  HS18: certify generated Result path cuts       <- increment A
>     5d977ac79  HS18: audit generated Result protocols         <- increment A / checkpoint
>
> **Three of the four are literally named WIP.** That is the substrate this node
> lands.

# Do NOT read "the prefix is absent from main" as the scope

It is the half-truth that would size this node at `+10472/-5307`. **The prefix
has three regions and they want opposite treatment** (Architect,
`evt_6fr79bx4cjhsw`, measured prefix tip `30d35f625` against `origin/main`
`6036f9f5d` with base `2a74775ae`):

    REGION 1  ALREADY LANDED       5 files   host + elaborator half    EXCLUDE
    REGION 2  CLEANLY ABSENT      13 files   main sits at prefix base  REPLAYS
    REGION 3  DIVERGED            16 files   main moved independently  RECONCILE

**Region 1 must be excluded — re-landing it is a no-op at best and a REVERT at
worst.** `mapping_v1.rs`, `lib.rs`, `abi_v1.rs` (ken-host), `prelude.rs`,
`compiler_driver.rs` (ken-elaborator) are byte-identical at the prefix tip
because they landed by another route.

**The vacuity control was run before that was believed, and the Steward
reproduced it on two of the five:** for each file, prefix-base content DIFFERS
from prefix-tip content, so `main` matches the **changed** form, not the
starting form. *"Identical to main"* is also what you get when a prefix
net-changed nothing, and that reading is refuted here rather than assumed away.

**Region 3 is the half that `absent` cannot express.** On
`immediate_bridge.rs`, `main` built a **larger** version of the same new module
by an independent route (`main` +1510/-0 from base; prefix +622/-0). For these
files the work is a reconciliation of two developments, not a replay of one.

# THE EXTENT: an enumeration of ~117 items, not a raw diff

Prefix-minus-main, by item inventory (`fn`/`struct`/`enum`/`const`/`type`):

    12 files                              0 missing   fully subsumed, incl.
                                                      immediate_bridge.rs (34 of 34;
                                                      main has 69)
    lowering/mod.rs                      39 of 553
    abi_s6_mapping_file_backed_native.rs 24 of 24     FILE ABSENT ENTIRELY
    lowering/core.rs                     10 of 211
    lowering/calls.rs                     9 of 38
    ken-host/src/effect_v1.rs             7 of 270
    lowering/source.rs                    6 of 86
    continuations.rs 5 · responses.rs 5 · px8f_buffer_native.rs 4 · eval.rs 2
    units.rs 2 · control.rs 2 · aggregates.rs 1 · host_call_carrier.rs 1

**An order of magnitude narrower than the raw diff, and an enumeration rather
than an estimate.**

**ITEM-NAME PRESENCE IS A PROXY FOR BEHAVIOUR, NOT BEHAVIOUR.** A same-named
function can have a different body. This bounds the **extent** and converts an
unbounded question into an enumerable one; it does not prove item-by-item
equivalence. **Take the predicate, not the list, and re-run it when `main`
moves:**

```sh
items(){ git show "$1:$2" | grep -oE "^\s*(pub(\([^)]*\))? )?(fn|struct|enum|const|type) [A-Za-z0-9_]+" | awk '{print $NF}' | sort -u; }
comm -23 <(items 30d35f625 "$f") <(items origin/main "$f")
```

# What is IN: the production cluster

    generated-context-result authority   GeneratedContextResultAuthority,
                                         register_/consume_/close_*_forwarding,
                                         generated_context_result_word_is_authorized,
                                         register_*_join
    checked-IH post-call / detached      checked_ih_post_call_residual,
                                         realize_checked_ih_post_call_steps,
                                         validate_checked_ih_consumed_active,
                                         validate_checked_ih_detached_result_shape,
                                         CheckedIhStaticResponseReturnReceipt,
                                         TailCheckedIhTransportResult,
                                         bind_checked_ih_detached_caller_cut[_from_call]
    recursive-position calls             RecursivePositionCallInputs,
                                         resolve_declared_recursive_position_call,
                                         call_declared_recursive_position_closure_unit
    source dynamic match                 SourceDynamicMatchRequest/Scrutinee,
                                         lower_source_dynamic_match_request,
                                         SourceMachineExit
    the acceptance test                  abi_s6_mapping_file_backed_native.rs, absent in full

# THE EVIDENCE-INSTRUMENT FORK, RULED: a REACHING CONSUMER, never a name pattern

**Most of the ~117 are evidence instruments, not production** — `D5bHs3/5/7/8/9/10/11
*Mutation` and `*Observation` types, `with_*_mutation`, `record_*_observation`,
`d5b_hs*_*` helpers. The Architect named the fork and declined to inherit it:
instruments for HS series `main` may already have closed by another route are
**cost without cover**; instruments covering the production cluster are **what
make it reviewable**.

**RULED (Steward): an instrument travels if and only if it has a reaching
consumer among the production items this node lands.** Not "if its name matches
`d5b_hs*`", and not "if its HS series is still open."

⇒ **This is a predicate, not an enumeration, and that is deliberate.** This
node's parent already carries the reason, in `§4a-pin`: *"TAKE THE PREDICATE,
NOT THE COUNT. Every enumeration of this has been short."* It was enumerated
twice there and came up short both times. **A list cannot report being
incomplete; a predicate can be re-run.**

**Report which instruments the predicate EXCLUDED and why** — an exclusion you
can name is reviewable, and a silent one is indistinguishable from an oversight.

# NOT this node

- **The refused `MappingAcquireFile` grant.** `lowering/effects.rs` +46/-5 is
  census site 5, 8 of 42 lines. **Do NOT hand-separate it.** It is already
  carved out as [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] (`draft`), held out of
  slice 4 on Architect ruling `evt_3wtg8w8krmmt` — *"Membership is a plan, not
  evidence"* — and unblocked for **cutting, not release** (`evt_21f23zmgqfxsc`).
  **Excluded here BY CONSTRUCTION**, which is a boundary a reviewer can check
  rather than one they must trust.
- **Increment A's own substance.** Nine files, `units.rs` +4083. This node
  makes it landable; it does not absorb it.
- **Increments B and C.** B is `source.rs`'s resume-exit repair, C is amendment
  8's consumer relocation.
- **Region 1's five files.** See above — re-landing is a revert risk.

# Related

- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — what this unblocks; gains this as its
  only dependency. Its `§2a` decomposition is the refuted premise.
- [[ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]] — the sibling port, LANDED at
  `10e75cb93656d5ea787bceaf754b2500b78de166`. Same shape of work, and the
  worked precedent for extent and review.
- [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] — holds the refused grant. Deliberately
  `draft`.
- [[RT-D5B-POSTCALL-REFUSAL-MECHANISM]] — `ready`, `gate: architect`. The
  `CheckedIhDetachedCallerCut` refusal mechanism is UNKNOWN and that node exists
  to find it. **Overlaps this node's checked-IH cluster; check contention before
  releasing both.**
