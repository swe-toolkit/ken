# RT-D5B-BRIDGE-REALIZATION-PLANE

Slice 2 of the PR #3676 re-cut. Lands the immediate-bridge **realization
plane** — the derivation, the plan field, and the mutation harness — on top of
the classifier that slice 1 landed, exercised by unit tests that fail without
it and still **not wired into the live planning path**.

    base            origin/main 10321a158bc69cf50e0cb753e1096fe10fa43ed1
    reference       0f71ab5b9267781ae1d91bc654011cad42b926af   READ-ONLY
                    (PR #3676's closed branch; our own work, no clean-room
                    question; nothing cherry-picked, nothing stacked)
    predecessor     RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER, landed 10321a158
    successor       slice 3, the live wiring — NOT this WP, see section 5
    tier            T1
    size            M

## 1. What slice 1 left on `main`

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      immediate_bridge.rs                          677 lines on main

      :59   impl ImmediateBridgeConsumer
      :96   impl ImmediateBridgeRealization        12 accessors
      :228  statically_selects_host_operation
      :238  statically_selected_case_reaches_host_effect
      :350  shifted_aggregate_ihs
      :354  #[cfg(test)] mod tests                 the slice-1 unit tests

    static_transition.rs                           one line: `mod immediate_bridge;`

The classifier is complete and tested. **Nothing on `main` calls it from a
production path**, which is the property slice 1 was defined by and which this
slice preserves.

## 2. What this WP adds — Stratum B, measured at the reference

Seven items, `immediate_bridge.rs` `:341-622` at `0f71ab5b9`:

    :341  derive_immediate_bridge_realizations       private, the derivation
    :442  relation_from_rows                         private helper
    :456  build_immediate_bridge_realization_plan    pub(super)
    :464  enum D5bHs10BridgePlanMutation             pub
    :485  struct D5bHs10BridgePlanMutationGuard      + :488 impl Drop
    :496  with_d5b_hs10_bridge_plan_mutation         pub, feature-gated export
    :514  publish_immediate_bridge_realization_plan  pub(super)
    :597  validate_immediate_bridge_realization_plan pub(super)
    :609  impl StaticTransitionPlan                  :610 immediate_bridge_realization
                                                     :617 immediate_bridge_realization_identities

Plus, in `static_transition.rs`, **the field**:

    immediate_bridge_realizations:
        BTreeMap<ContinuationCallIdentity, ImmediateBridgeRealization>

and in `construction.rs` **the initializer only** (`:298` at the reference):

    immediate_bridge_realizations: BTreeMap::new(),

## 3. THE TRAP. Three of them, and none is the one slice 1 had.

**Do not lift the reference's `static_transition.rs`.** Measured from the
merge-base `1dec48f33cc0697569e9e8765874fe61269332f4`:

    branch's own diff to static_transition.rs     +192 / -119, 69 hunks
    hunks mentioning immediate-bridge at all      6
    of those, already landed in slice 1           1  (`mod immediate_bridge;`)
    belonging to slice 3, excluded here           2  (see section 5)
    this WP's hunks                               3

**Sixty-three of the sixty-nine hunks are import reorganization and unrelated
test-support churn** — `px8-ds-test-support` feature blocks,
`d2g_declaration_body_relocated`, `d2j_rewrite_body`, `D2jCause`, and a
wholesale reshuffle of the module's `use` blocks. They are 38 commits of
unrelated evolution. None of it belongs here.

### 3a. The `use` hunk is a REPLACEMENT and lifting it deletes five live imports

At the reference, `@@ -81,3 +74,4 @@` reads:

    -pub(in crate::cranelift_backend) use semantic_ir::{
    -    BoolMatchCaseOrdinals, ConstructorIdentity, FieldIdentity, SynthesizedConstructorRole,
    -    SynthesizedFixedConstructorRole,
    +pub(in crate::cranelift_backend) use immediate_bridge::{
    +    classify_immediate_bridge, produces_deforestable_aggregate_with_ih,
    ...

It **replaces** the `semantic_ir` re-export block rather than adding beside it.
Those five names are live on `main`:

    BoolMatchCaseOrdinals            7 files
    ConstructorIdentity             13 files
    FieldIdentity                    8 files
    SynthesizedConstructorRole      11 files
    SynthesizedFixedConstructorRole 10 files

⇒ **The `immediate_bridge` re-export must be authored as a new block. The
`semantic_ir` block stays exactly as `main` has it.**

### 3b. The field hunk adds TWO fields, and the second does not exist on `main`

`@@ -575,2 +617,9 @@` adds `immediate_bridge_realizations` **and**:

    checked_ih_post_call_consumers: Vec<CheckedIhPostCallConsumer>,

`CheckedIhPostCallConsumer` is a **different mechanism** — 4 files at the
reference, and:

    git grep -c 'CheckedIhPostCallConsumer' origin/main -- crates/   ->  ZERO

⇒ **Lifting the hunk verbatim does not compile.** Add one field.

### 3c. Slice 1's trap has not gone away

The reference's `+31` hunk on `static_transition.rs` still carries
`InlineBridgeNoCall` and the `owns_seat` rewrite. It was excluded from slice 1
and it is excluded here. See section 5.

### 3d. AS BUILT — all three traps STAND, and the hunk count came in at 2

**Resolved 2026-09-16 against candidate `568d5d7eedad3d5ec957b0ee42189019b1cb0710`.
Do not read this as a retraction of §3a-§3c. Every claim in them reproduces at
the reference:**

    git diff -U0 1dec48f33 0f71ab5b9 -- .../static_transition.rs | grep -c '^@@'
      -> 69                                                    §3 says 69
    git diff --shortstat 1dec48f33 0f71ab5b9 -- .../static_transition.rs
      -> +192 / -119                                           §3 says +192/-119
    git grep -c 'CheckedIhPostCallConsumer' origin/main -- crates/
      -> no hits                                               §3b says ZERO

**§3b is the one most likely to be waved away, so state it plainly.** §3b names
two fields and warns about **the second**. `ImmediateBridgeRealization` — the
**first** field's type — resolves fine on `main`, because slice 1 landed it.
That is not §3b's subject. **`CheckedIhPostCallConsumer` still has zero hits,
so a verbatim lift of that hunk still does not compile.**

**As built, `static_transition.rs` is 2 hunks, not the 3 §3 predicts, and the
missing hunk IS §3a's.** The candidate leaves the `semantic_ir` re-export block
alone and does not author the classifier re-export at all, so the two remaining
additions coalesce into one contiguous block. **2 is the correct count for a
candidate that avoids the trap §3a forbids** — it is not a shortfall.

**All hunk counts here are `-U0`.** The same diff reads differently at other
context widths, so a reader reconciling against git's default will be off:

    -U0   69 hunks,  6 mentioning immediate-bridge     <- §3's numbers
    -U1   53         5
    -U3   41         5                                 <- git's default

**The general point, because it cost four separate re-derivations here.** A trap
warns about a hazard in the artifact you might lift. **Confirming the candidate
did not fall into it is evidence the warning WORKED, not evidence the hazard was
imaginary** — and at the candidate the two are indistinguishable. Only the
reference tells them apart. Three separate "refutations" of §3 were each a
correct measurement of the candidate read as a claim about the reference.

## 4. Why this slice is still not wired, and why that is not "inert"

At the reference, `construction.rs:1463` contains the live wiring:

    self.plan.immediate_bridge_realizations =
        publish_immediate_bridge_realization_plan(&self.plan)?;
    self.plan.install_static_response_context_plan_phase_b()?;

**That line is a production behaviour change** and it is the seam between this
slice and slice 3. This WP adds the field and its `BTreeMap::new()` initializer
— forced, because the struct must construct — and **stops there**.

**This is not the inert-module criterion the Architect ruled out for slice 1.**
The Stratum B functions are `pub(super)` and directly callable from unit tests
against a constructed `StaticTransitionPlan`. `derive_`, `build_`, `publish_`
and `validate_` each have observable behaviour without the live path, and
**AC-2 requires tests that fail without them**. A slice whose green proves only
that the crate compiles is the thing being avoided, and it is avoided here by
testing the functions directly, not by wiring them.

**The dead-code warning window is again the instrument, not debris.** Slice 1's
ruling (Architect `evt_5rbgwyamv4y2n`, accept the warnings, add no
`#[allow(dead_code)]`) governs this slice unchanged. The warning set is the only
live indicator that the plane is on no production path, which is the property
AC-3 defines the slice by.

## 5. Explicitly OUT of scope — slice 3

    construction.rs:1463    the publish_ wiring into the live path
    static_transition.rs    @@ -776,6 +828,9 @@   the owns_seat rewrite
                            @@ -1017 +1070,2 @@   the InlineBridgeNoCall doc
    the DeferredResponseSubCase::InlineBridgeNoCall variant itself

Slice 3 gets its own frame. **A candidate for this WP that touches
`construction.rs:1463` or introduces `InlineBridgeNoCall` is out of scope and
should be bounced, not reviewed.**

## 6. Acceptance

**AC-1. Stratum B only.** The diff to `immediate_bridge.rs` adds the seven items
in section 2 and nothing else. Control: the diff to that file contains no edit
to any item slice 1 landed.

**AC-2. Tests fail without the plane.** Unit tests over `derive_`, `build_`,
`publish_` and `validate_` that go red when the implementation is stubbed. Both
runs cited, with the stub shown. **This is the point of the WP.**

**AC-3. No change to any production code path.** Control: `construction.rs` is
touched at exactly one site, the field initializer, and that initializer is
`BTreeMap::new()`. Show `git diff` for `construction.rs` is that one hunk.

**AC-4. The five `semantic_ir` re-exports are still present and unmodified.**
Control: `git diff` shows the `semantic_ir` block unchanged; grep each of the
five names on the candidate and on `main` and show the counts agree. Section 3a
is the failure this catches.

**AC-5. `CheckedIhPostCallConsumer` appears nowhere in the diff.** Control: zero
grep hits on the candidate. Section 3b.

**AC-6. Forbidden names absent.** `InlineBridgeNoCall` has zero hits on the
candidate. Control: grep, and show `construction.rs:1463`'s wiring is absent.

**AC-7. Per-case provenance.** For every test expectation, say whether it is
**semantic** (lifted from an attested ancestor, cited verbatim by file and line
at `0f71ab5b9`) or **structural** (authored freely). **AC-2 measures coupling,
not faithfulness** — a test whose expectation was read off the implementation
goes red against a stub while proving nothing. Any expectation with no ancestor
is declared new intent by name.

**AC-8. The mutation harness is feature-gated and inert by default.** Control:
show `with_d5b_hs10_bridge_plan_mutation` and `D5bHs10BridgePlanMutation` are
exported only under `#[cfg(feature = "px8-ds-test-support")]`, and that a build
without that feature does not reference them.

**AC-9. Warning delta is stated, not suppressed.** Report the `ken-runtime`
warning count at `main` and on the candidate, and name each new warning. No
`#[allow(dead_code)]`. Slice 1's figure was 88 -> 101.

**AC-10. No `#[ignore]` rows added.** Control: the candidate's `#[ignore` count
under `crates/` equals `main`'s. This is a `D0` precondition and the population
is currently being measured on run `35055338045`.

**AC-11. No decorative glyphs in the diff.**

## 7. Sizing and contention

T1 and M. The seven items are ~282 lines at the reference and carry across
largely unchanged; **the hour goes into AC-2 and AC-7**, as it did in slice 1,
because the 36 commits proved this plane out through the live plan and there are
no direct unit tests to lift.

Contention: `static_transition.rs` and `construction.rs` are both high-traffic.
This WP's diff to them is 3 hunks and 1 hunk respectively. Coordinate with any
in-flight runtime candidate touching `planning/`.

## 8. Local build discipline

Targeted only, through `scripts/ken-cargo`, `-p ken-runtime`. **Never
`--workspace`.** The workspace build, the `--locked` gate and the conformance
suite run in CI. `COORDINATION §12`.
