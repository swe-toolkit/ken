---
id: RT-TRAP-IDENTITY-EMISSION-COORDINATE
title: "Make a trap's identity carry its emission coordinate, so eliminations of one family at distinct source sites are not equal as RuntimeTrap values and do not collapse to one PlannedTrapIdentity -- closing the attribution gap that left HS16-HS21 unable to say WHICH site fired, with a five-occurrence in-tree discrimination criterion that both the catalog-keying fix and a type-keyed fix fail"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Architect ruling evt_2qq9jr1c1e4p9 (2026-09-14), CORRECTED at evt_7eqc0hmbanyzm after runtime-implementer refuted the Architect's own proposed population; mechanism ruled (b) and AC-2 ratified at evt_5kzahxdv9w8d1; recorded durably in ABI-S6 entry 35 (routed f0a735f14). Split out of RT-CHECKED-IH-RESULT-OBLIGATION-REKEY because it is fork-independent and that node's widened D1 is not: see 'Why this is its own node'. Fixed inputs first measured by the Steward at 686ffa8ac, then re-measured on origin/main; re-measure again at D0."
---

> # RELEASED. Architect read and approved at `evt_5xtean9vczn36`.
>
> **The mechanism fork is closed.** The Architect ruled `(b)` at
> `evt_5kzahxdv9w8d1` and ratified `AC-2` as authored here. They then read this
> frame — not a description of it — and approved it for release.
>
> **What this frame changed after the ruling**, by grounding the handed-over
> witness against the tree instead of transcribing it:
>
> 1. `AC-1` is written on the **real five-occurrence fixture**, and splits into
>    `AC-1a` / `AC-1b` because that fixture refutes a **second** plausible fix —
>    keying on the instantiated type — which an authored two-site pair would
>    have passed. The Architect's verdict: a control they did not know to
>    specify, and it exists only because the real fixture was used.
> 2. Two of the three relayed counts for that fixture were wrong (**four**
>    instantiations, not three; occurrence 2 is a `proc`). Corrected in place,
>    with the grep that produces the second error named so it is not re-made.
>    The wrong instantiation count never reached the durable record — ABI-S6
>    entry 35 (blob `c62d2288`) carries no instantiation count — so no erratum
>    is owed there.
>
> **One correction came back and is applied.** The mint-site line numbers do NOT
> drift between `686ffa8ac` and `origin/main`: they are `:2919` / `:6043` at
> both. The Steward's claimed two-line shift was a number carried from this
> frame's own earlier prose rather than re-derived, inside a re-measurement
> undertaken to avoid exactly that. The original citation was right at both refs.

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
  `crates/ken-elaborator/src/erasure.rs:2919` and `:6043`. **Measured at BOTH
  `origin/main` and `686ffa8ac` and identical at both** — carry the SHA beside
  the number, but there is no drift between these refs to correct for. `D0` still
  re-measures at the SHA it builds on. Both build

      code: RuntimeTrapCode::PatternMatchFailure,
      message: format!("no runtime match case selected for {}", view.family_symbol),

  so two eliminations of one family agree on **both** fields. Verified field by
  field, not inferred from the message alone.

- **The other seven `erasure.rs` mints do NOT collide**, and the reason matters:
  `:3215`, `:3995`, `:4024`, `:4044`, `:4064`, `:7526`, `:7942` each carry a
  distinct fixed or differently-interpolated message. **They discriminate
  incidentally, by message content — not by any mechanism.** Nothing stops the
  next family-symbol-derived default from colliding again.

- **WHAT `family_symbol` ACTUALLY INDIVIDUATES, and it is the root of the
  collapse.** It is a `StableSymbol` in the **`Declaration` namespace**, built
  from the declaring package plus the family's dotted name
  (`compiler_driver.rs:4159-4165`). **It carries no type arguments and no source
  coordinate.** Every elimination of one family declaration therefore produces a
  byte-identical message, whatever its instantiation and wherever it sits.

- **THE FIVE-OCCURRENCE FIXTURE, ALREADY IN THE TREE AND ALREADY COLLAPSING.**
  `crates/ken-verify/tests/px8f_write_partition.rs`, the `WRITE_ALL_PARTITION`
  program constant beginning at line 15. **Byte-identical on `origin/main` and
  in the Steward's worktree** (`git diff origin/main` empty for this path).
  Five `match` sites eliminate the `Result` family, in five distinct enclosing
  declarations:

  | # | line | enclosing declaration | scrutinee instantiation |
  |---|---|---|---|
  | 1 | 18 | `fn body_from_write` (16) | `Result ResourceError Unit` |
  | 2 | 41 | `proc after_read` (37) | `Result ResourceError ReadProgress` |
  | 3 | 71 | `fn buffer_bracket_body` (68) | `Result ResourceError (ResourceBracketResult Unit Unit)` |
  | 4 | 102 | `fn file_bracket_body` (99) | `Result FileError (ResourceBracketResult Unit Unit)` |
  | 5 | 132 | `fn finish` (130) | `Result FileError (ResourceBracketResult Unit Unit)` |

  **Four distinct instantiations across five sites — occurrences 4 and 5 share
  one.** All five share the single `family_symbol`
  `decl:px8f_write_partition::Result`, so all five mint **one** `RuntimeTrap`
  value and `intern_trap` collapses them to **one** `PlannedTrapIdentity`.

  **Two counts here are corrections to the numbers relayed when this witness was
  handed over** ("five matches, five declarations, three instantiations"): the
  instantiation count is **four**, not three, and occurrence 2's enclosing
  declaration is a **`proc`**, not a `fn`. The second correction is the one that
  matters to anyone re-running this: a `^fn ` grep silently attributes
  occurrence 2 to the preceding `fn read_eof_body`, which does not mention
  `Result` at all. Enumerate `fn` and `proc` together.

  The shared instantiation at 4/5 is not an inconvenience in the fixture — it is
  the half that makes `AC-1b` possible, and it is why this fixture is used
  instead of an authored pair.

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

- **The construction surface is 34 PRODUCTION SITES ACROSS 11 FILES — not the
  ~76 this frame first estimated.** `D0` certified it (`evt_2kv61375ae400`) and
  the frame's own number was flagged as an approximate grep census; it was, and
  it was wrong in a specific way: **it counted in-file `#[cfg(test)] mod tests`
  blocks as production.** Of 257 `RuntimeTrap {` sites at `686ffa8ac`, 223 are
  test-side.

  | count | file |
  |---|---|
  | 9 | `ken-elaborator/src/erasure.rs` |
  | 6 | `ken-runtime/.../static_transition/joins_traps.rs` |
  | 6 | `ken-runtime/src/ir.rs` |
  | 3 | `ken-runtime/src/object_linker_packaging.rs` |
  | 2 | `ken-runtime/.../planning/static_transition.rs` |
  | 2 | `ken-runtime/src/platform_runtime_support.rs` |
  | 2 | `ken-runtime/src/runtime_ir_evaluator.rs` |
  | 1 | `ken-host/src/effect_v1.rs` |
  | 1 | `ken-runtime/.../lowering/aggregates.rs` |
  | 1 | `ken-runtime/.../lowering/core.rs` |
  | 1 | `ken-runtime/.../lowering/mod.rs` |

  **This roughly HALVES the migration surface**, which matters for a size-M node.

  Two things about how that number was obtained are worth keeping. **The first
  instrument returned 30 and was not reported** — `#[cfg(test)]` in this tree is
  overwhelmingly ITEM-level rather than module-level, and a brace matcher
  treating each as opening a block swallowed following code. A plausible number
  from an instrument that could not discriminate. The replacement was
  hand-validated on five files and agrees on all five, and it is independently
  corroborated here: this frame states `erasure.rs` has 2 colliding mints plus 7
  incidental discriminators, and **2 + 7 = 9 matches the census's 9** by a
  different method.

  **One residual, named rather than absorbed:** `cranelift_backend/test_objects.rs`
  (2 sites) is excluded because its module declaration is `#[cfg(test)]`-gated in
  the PARENT — a gate a per-file classifier structurally cannot see. If another
  module is gated the same way the count is 2 high per such module. That one was
  checked because its name invited it; **there has been no sweep for others, and
  `D3` owes one.**

- **THE `RuntimeTrap` DECLARATION LINE DRIFTS BETWEEN REFS**, and the header's
  correction does not cover it — that one is about the MINT sites, which genuinely
  do not drift. The struct does:

      effect_v1.rs   derive at :4067  (686ffa8ac)
                     derive at :3989  (origin/main)   -- 78 lines apart

  Anyone building on `origin/main` and going to `:4067` lands 78 lines off.

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

**`D0` — the ruled migration. THE MECHANISM IS NO LONGER A FORK.** The Architect
ruled mechanism **(b)** at `evt_5kzahxdv9w8d1`: **add a required
emission-coordinate field to `RuntimeTrap`**, touching the ~76 production sites
above plus tests. Every future mint must supply a coordinate or it is a compile
error.

**The refuted alternative, recorded because `AC-1` must be able to fail it:**
(a) folding the coordinate into the existing `message` at the colliding mint
sites, about 2 sites. It was refuted on three grounds, and the first is
measured in this frame rather than argued:

  - **It closes the named INSTANCES, not the CLASS.** The fixed inputs above
    record that the other seven `erasure.rs` mints discriminate **incidentally,
    by message content, with no mechanism**. An incidental discrimination is not
    a property — a third family-symbol-derived default added later collides
    again and nothing reds when it does.
  - **It puts a machine identity into user-facing diagnostic text**, making one
    string serve two consumers with different requirements. That is this arc's
    own predicate arriving inside the repair for it.
  - **The compile-error form is actually available here**, which is exactly what
    it is NOT for `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s `D1`. Where the
    structural form is reachable, take it. 76 sites is mechanical, the failures
    are compile errors, and cost does not outrank a class closure whose only net
    is an unsound pass.

**`D0` IS REPORTED AND THE GATE PASSES** (`evt_2kv61375ae400`). All three halves
of the FENCED premise measured, and the third positively rather than as an
absent grep:

  - **No `serde`** anywhere in `effect_v1.rs`, at both refs.
  - **No `repr`** on either type. The file's four `repr(..)` attributes sit on
    `HostOpV1`, `CapabilityTokenV1`, `ResourceTokenV1` and `FdInheritancePolicyV1`.
  - **Nothing serializes it or pins its layout** — zero hits for `RuntimeTrap` in
    any serialize/encode/to_bytes/section context, and zero for
    `size_of::<RuntimeTrap>` / `transmute` / `offset_of`. **Positively: what
    crosses into generated code is an `i64`** — `emit_current_trap` stores
    `identity.abi_word()` or a shifted root token into the trap slot. The struct
    never crosses.

⇒ **FENCED holds. No stop.** The come-back condition remains live for the rest
of the build: if anything later serializes a `RuntimeTrap` or pins its field
layout, that returns to the Steward before continuing.

**L2 CONTENTION: CLEAR, AND MEASURED WITH A POSITIVE CONTROL.** No branch ahead
of `origin/main` touches `erasure.rs` except runtime-lane ones. The control that
makes the zero meaningful: **L2 branches do exist and are active in
`ken-elaborator`** — `LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD`,
`LANG-SURFACE-RECORD-DECL-FORM-RECUT`, `LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES` —
and every one of them avoids `erasure.rs`. The negative is a measurement, not an
artifact of looking somewhere empty. **No sequencing is owed.**

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
A test over the five-occurrence `WRITE_ALL_PARTITION` fixture in which all five
`Result` eliminations receive **five distinct planned identities**, including
the same-instantiation pair at occurrences 4 and 5.

- **MEASURED** = five source sites eliminating one family declaration get five
  distinct `PlannedTrapIdentity` values.
- **CLAIMED** = a trap identity individuates the emission site.
- **GAP** = a test that asserts the catalog's KEYING, or that keys on the
  instantiated type, cannot fail the defect.

**Use the fixture that is already there.** Adding a fresh two-site program
alongside it would test the repair against an example authored after the fix was
known — the existing five-site program predates it and collapses today.

**`D3` — report what the migration required**: how many sites were touched, and
whether any site could not supply a meaningful coordinate. **A site that cannot
name its own emission coordinate is a FINDING
to report, not a hole to fill with a placeholder** — a default value at such a
site silently restores the collapse for exactly that site.

## Acceptance criteria

**`AC-1` — a DISCRIMINATION test over the five-occurrence in-tree fixture. The
five `Result` eliminations in `WRITE_ALL_PARTITION` receive FIVE distinct
planned identities.**

This is not an authored pair. The fixture is already in the tree and already
collapses, and it is a stronger witness than a two-site construction because it
refutes **two** plausible fixes rather than one.

**`AC-1a` — the superseded catalog-keying fix must FAIL this criterion.** "The
catalog is keyed by occurrence" passes a test written over the KEY while all
five sites still share one identity, because `intern_trap` selects with
`position(|candidate| candidate == trap)` — a predicate over VALUES. An AC that
tests the keying rather than the discrimination cannot fail the defect it exists
to catch.

**`AC-1b` — keying by the INSTANTIATED TYPE must also FAIL this criterion, and
this is the half only the real fixture provides.** Occurrences 4 and 5 carry the
**identical** instantiation `Result FileError (ResourceBracketResult Unit Unit)`
in two different declarations. Any coordinate that is a function of the type
rather than of the source site leaves that pair collapsed. An authored two-site
pair with distinct types would have passed a type-keyed fix and hidden this.

Neither sub-criterion is to be dropped in a restatement: a criterion whose
refuted alternatives are deleted can no longer be shown to discriminate.

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

**Size M, tier T1.** Under the ruled mechanism (b) this is a **wide mechanical
migration with a narrow semantic core** — ~76 sites of bookkeeping around one
design question. The review turns on an argument rather than a byte count: that
the coordinate is a static planner or elaborator fact rather than a traversal
artifact, and that the control actually discriminates five sites rather than
asserting a property of the keying.

**The T1 estimate is for the semantic core and the control, not the 76 sites.**
If the ring wants to split the mechanical migration onto a cheaper seat once the
coordinate's source is settled at `D1`, that is a reasonable cut and it is the
Steward's to make — bring it back rather than absorbing it.

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
