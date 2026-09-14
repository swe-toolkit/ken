---
id: RT-TRAP-IDENTITY-EMISSION-COORDINATE
title: "Make a trap's identity carry its emission coordinate, so two eliminations of one family at two source sites are not equal as RuntimeTrap values and do not collapse to one PlannedTrapIdentity -- closing the attribution gap that left HS16-HS21 unable to say WHICH site fired, with a two-site discrimination acceptance criterion that the catalog-keying fix fails"
status: draft
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Architect ruling evt_2qq9jr1c1e4p9 (2026-09-14), CORRECTED at evt_7eqc0hmbanyzm after runtime-implementer refuted the Architect's own proposed population; recorded durably in ABI-S6 entry 35 (routed f0a735f14). Split out of RT-CHECKED-IH-RESULT-OBLIGATION-REKEY because it is fork-independent and that node's widened D1 is not: see 'Why this is its own node'. Fixed inputs measured by the Steward at 686ffa8ac; re-measure at D0."
---

> # DRAFT — pending Architect sanity read. NOT released to the runtime ring.
>
> The Steward committed (evt_66qhft3939qpr) to sending this frame to the
> Architect before the ring, because the boundary between the obligation and
> the mechanism implementing it is exactly where this cut can go wrong. The
> `D0` mechanism fork below is the specific thing that needs their ruling.

## What this is

Three structurally identical closed-default sites in `func61` (lines 367, 590,
1946) all emit trap code 43. "Trap 43 fired" therefore names a CODE, never a
SITE — and every attribution in HS16 through HS21 that concluded about WHERE
rested on that code. The ruled account of the failure was not refuted by this;
it was left UNATTRIBUTED, which is a state nothing can be ruled from.

This node makes the identity discriminate. It is the instrument the rest of the
arc needs before any conclusion about WHERE can be trusted, and the Architect
ruled it a DELIVERABLE rather than instrumentation that gets reverted.

## The obligation, in the corrected form

**A trap's IDENTITY must carry its EMISSION COORDINATE, so that two eliminations
of one family at two source sites are not equal as `RuntimeTrap` values.** The
catalog keying follows from that; it does not substitute for it.

**The superseded form, and why it is recorded rather than deleted.** The first
ruling named the population as "the trap catalog's entries, keyed by occurrence
rather than value." That fix is NECESSARY AND NOT SUFFICIENT. `intern_trap`
selects with `position(|candidate| candidate == trap)` — a predicate over
VALUES — so re-keying the catalog cannot separate two entries that are equal as
values, because the key ranges over the values. The repair addressed the
collapse at the catalog and left the indistinguishability at the value.

This is kept in the frame because the discarded fix is the thing the acceptance
criterion has to be able to FAIL. See `AC-1`.

## Fixed inputs

**MEASURED BY THE STEWARD at `686ffa8ac`** (the HS-arc WIP tip), except where a
line says otherwise. Re-measure every coordinate at `D0`; `RuntimeTrap` itself
lives in `ken-host` and is equally present on `origin/main`.

- **The equality that collapses them**, `crates/ken-host/src/effect_v1.rs:4067`:

      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct RuntimeTrap {
          pub code: RuntimeTrapCode,
          pub message: String,
      }

  Equality is over exactly these two fields. There is **no `serde` derive and no
  `repr`** on `RuntimeTrap` or `RuntimeTrapCode`.

- **The two colliding mint sites, and there are exactly two**:
  `crates/ken-elaborator/src/erasure.rs:2917` and `:6041`. Both build

      code: RuntimeTrapCode::PatternMatchFailure,
      message: format!("no runtime match case selected for {}", view.family_symbol),

  so two eliminations of one family agree on **both** fields. Verified field by
  field, not inferred from the message alone.

- **The other seven `erasure.rs` mints do NOT collide**, and the reason matters:
  `:3215`, `:3995`, `:4024`, `:4044`, `:4064`, `:7526`, `:7942` each carry a
  distinct fixed or differently-interpolated message. **They discriminate
  incidentally, by message content — not by any mechanism.** Nothing stops the
  next family-symbol-derived default from colliding again.

- **The interning predicate**, in
  `cranelift_backend/planning/static_transition/joins_traps.rs:633`:
  `intern_trap` returns the index of the first `position(|candidate| candidate
  == trap)`, else pushes. N equal-valued sites collapse to ONE
  `PlannedTrapIdentity`.

- **The trap catalog does NOT reach the emitted artifact as a value.** It is an
  in-process `Vec<RuntimeTrap>` (`planning/static_transition.rs:587`,
  `compiled.rs:25`, surfaced at `cranelift_backend/surface.rs:38`). The artifact
  carries an index/token resolved host-side through `root_trap_catalog_index`
  (`compiled.rs:56`). **This is the fact the FENCED assessment rests on** — see
  "Why this is FENCED".

- **The construction surface a required new field would touch: about 76
  production `RuntimeTrap { .. }` sites across 20 files**, concentrated in
  `object_linker_packaging.rs` (23), `ir.rs` (10), `erasure.rs` (9),
  `static_transition.rs` (7), `joins_traps.rs` (6),
  `native_process_entrypoint.rs` (6), `platform_runtime_support.rs` (5), plus
  test-side literal constructions. This count is the `D0` input and is an
  approximate census by grep, not a certified enumeration — `D0` produces the
  exact number.

- **The fact that was already in the tree, three hard stops early**,
  `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs:4999`,
  in a comment above a test named "Hard-stop #18 row 2":

      erasure.rs derives the case headers from the eliminated family and builds
      the default as format!("no runtime match case selected for
      {family_symbol}"). Two eliminations of one family in one declaration
      therefore agree on every field a header fingerprint can see.

  It was written for a NEIGHBOURING consumer (header fingerprints) and so never
  reached the consumer that needed it (trap identity). Recorded here because it
  is the reason this node exists at all, not as decoration.

## Deliverables

**`D0` — the mechanism read, and it is a REPORT with a ruling gate, not a
build.** Two mechanisms satisfy the obligation with blast radii that differ by
more than an order of magnitude, and they differ in whether they close the
CLASS or only the two known instances:

  - **(a) Fold the coordinate into the existing `message`** at the colliding
    mint sites. Touches about 2 sites. **Closes the named instances and not the
    class**: a third family-symbol-derived default added later collides again,
    and nothing reds when it does. It also makes user-facing diagnostic text the
    carrier of an identity, which is a second authority over the same fact.
  - **(b) Add a required emission-coordinate field to `RuntimeTrap`.** Touches
    the ~76 production sites above plus tests. **Closes the class by
    construction**: every future mint must supply a coordinate or it is a
    compile error.

**Report the exact site count for (b), the exact colliding-site count for (a),
and what each does to the message text. The Architect rules which mechanism
before any build.** This is a component-design call and it is theirs, not the
ring's and not the Steward's. The Steward's read is that (b) is the one that
matches the obligation as stated — "a trap's identity must carry its emission
coordinate" is a statement about every trap, and (a) satisfies it for two of
them — but the cost is real and the call is the Architect's.

**`D1` — make the identity discriminate**, by the mechanism `D0` rules. Two
distinct source occurrences eliminating the SAME family must produce
`RuntimeTrap` values that are **not equal**, and therefore must receive distinct
`PlannedTrapIdentity` values from `intern_trap`.

**Report the coordinate's source and that it is a static planner or elaborator
fact — not read off a runtime value, and not derived from the order in which
sites happen to be visited.** An ordering-derived coordinate is unstable under
unrelated edits and would make the identity a function of traversal rather than
of the source site.

**`D2` — the discrimination control, and it is the load-bearing deliverable.**
A test in which **two distinct source occurrences over the SAME decl** receive
**DIFFERENT planned identities**.

- **MEASURED** = two source sites eliminating one family get two distinct
  `PlannedTrapIdentity` values.
- **CLAIMED** = a trap identity individuates the emission site.
- **GAP** = a test that asserts the catalog's KEYING rather than the
  DISCRIMINATION cannot fail the defect.

**`D3` — report what the migration required**, if `D0` rules mechanism (b):
how many sites were touched, and whether any site could not supply a meaningful
coordinate. **A site that cannot name its own emission coordinate is a FINDING
to report, not a hole to fill with a placeholder** — a default value at such a
site silently restores the collapse for exactly that site.

## Acceptance criteria

**`AC-1` — the criterion is a DISCRIMINATION test, and the proof it is
load-bearing is that the superseded fix FAILS it.** Two distinct source
occurrences over the same decl receive different planned identities.

**Write it that way and the catalog-keying fix fails it.** "The catalog is keyed
by occurrence" would have PASSED under that fix while three sites still shared
code 43 — an AC that tests the KEY rather than the DISCRIMINATION cannot fail
the defect it exists to catch. This sentence is the AC's control and is not to
be dropped in a restatement.

**`AC-2` — a positive control on the non-colliding population.** Two
eliminations of **different** families continue to receive different identities,
and two references to the **same** emission site continue to receive the **same**
identity. Without the second half, an implementation that simply makes every
trap unique passes `AC-1` while destroying the interning the catalog exists for.

**`AC-3` — `func61`'s three sites, named.** After the repair, the three
closed-default sites at `func61` lines 367, 590 and 1946 carry three distinct
identities. **Re-measure those line numbers at `D0`** — they were measured on an
arc WIP and a line number is a claim about one tree.

**Not an acceptance criterion, and deliberately so:** "Trap 43 no longer
appears." The trap still fires; what changes is that it is attributable. An AC
keyed on the trap disappearing would be discharged by a repair that removed the
trap, which is not this node.

## Why this is its own node and not an amendment to the REKEY node

**Because `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s widened `D1` is blocked on a
held fork and this is not.** Folding a fork-independent, releasable repair into
a node whose headline deliverable cannot currently be written would strand this
work behind a design ruling it does not depend on. The two touch disjoint code:
this node is `ken-host` + `erasure.rs` + `joins_traps.rs`; the REKEY node is
`lowering/core.rs`.

The constraint is an Architect ruling, cited: `evt_2qq9jr1c1e4p9` as corrected
by `evt_7eqc0hmbanyzm`. It is not a Steward preference for a tidier graph.

## Why this is FENCED

**Compiler-internal.** It grows no TCB, adds no capability reachable from Ken
source, relocates no emission ownership, and adds no planner authority.

**And it changes no ABI or schema — this is the load-bearing half, and it is
measured rather than assumed.** `RuntimeTrap` carries no `serde` derive and no
`repr`; the trap catalog is host-side state and the artifact carries only an
index resolved through `root_trap_catalog_index`. Adding a discriminating
constituent to the value therefore changes nothing that crosses the ABI or gets
serialized into an artifact. **If `D0` finds that wrong — if any path serializes
a `RuntimeTrap` or pins its field layout — the FENCED assessment does not hold
and this comes back to the Steward before the build continues.**

It is a cross-crate Rust type change (`ken-host` is consumed by `ken-runtime`,
`ken-elaborator`, `ken-interp`, and `ken-cli` tests), which is a real consumer
inventory and is why `D3` exists — but a cross-crate Rust surface is not an ABI.

## Sizing / tier

**Size M, tier T1.** Under mechanism (a) the diff is small; under (b) it is a
wide mechanical migration with a narrow semantic core. Either way the review
turns on an argument rather than a byte count: that the coordinate is a static
fact rather than a traversal artifact, and that the control actually
discriminates two sites rather than asserting a property of the keying.
Architect required on the mechanism.

## Contention

Runtime ring. Touches `crates/ken-host`, `crates/ken-elaborator/src/erasure.rs`,
and `crates/ken-runtime/.../joins_traps.rs`, so the candidate is a CODE merge:
full CI, M8/M8a Adversary.

**Cross-lane contention is REAL here and must be checked at `D0`, unlike the
REKEY node.** `erasure.rs` is in `ken-elaborator`, which is L2 language's crate.
L2's current objective is the reserved infix glyph work. Confirm at `D0` that no
L2 candidate is in flight touching `erasure.rs`; if one is, the two need
sequencing and that is the Steward's call, not the ring's.

L3 foundation is on `catalog/` and does not contend.

## Not this node

- The held representation fork — widening the unit-keyed `(actual, demanded)`
  discharge to every emitted hand-off VERSUS making intra-function edges
  structurally unable to carry a mismatched word. **Held by the Architect.**
  Nothing in this node touches it, and nothing in this node may be used to
  argue it either way.
- `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s deliverables, which are retained and
  not reopened here.
- Removing, relocating, or suppressing the trap itself.
- Any change to what the runtime tag means.
