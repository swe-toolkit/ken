# RT-D5B-LIVE-WIRING

Slice 3 of the PR #3676 re-cut, and the last one. Wires the immediate-bridge
realization plane that slice 2 landed **into the live planning path**, so the
classifier and the derivation stop being reachable only from tests.

    base            origin/main 49e5ebfbe501a80c6ff2cd7104e4115e0696ba49
    reference       0f71ab5b9267781ae1d91bc654011cad42b926af   READ-ONLY
                    (PR #3676's closed branch; our own work, no clean-room
                    question; nothing cherry-picked, nothing stacked)
    predecessor     RT-D5B-BRIDGE-REALIZATION-PLANE, landed 49e5ebfbe
    tier            T1
    size            S

**This frame absorbs the slice-3 carry-forward** that was drafted as a §5a
block on the slice-2 frame while slice 2 was publishing. That block was never
merged — its whole rationale was *"slice 3's frame does not exist"*, and this
file is that frame. **One correction was found while writing this and is
carried below: the re-arm trigger the block proposed names the two functions
this slice does NOT make live.** See §4.

## 1. What slices 1 and 2 left on `main`

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      immediate_bridge.rs                        on main at 49e5ebfbe

      :240  statically_selects_host_operation
      :250  statically_selected_case_reaches_host_effect
      :362  shifted_aggregate_ihs
      :378  derive_immediate_bridge_realizations       private
      :485  relation_from_rows                         private
      :504  build_immediate_bridge_realization_plan    pub(super)
      :559  with_d5b_hs10_bridge_plan_mutation         pub, feature-gated
      :582  publish_immediate_bridge_realization_plan  pub(super)
      :675  validate_immediate_bridge_realization_plan pub(super)
      :705  #[cfg(test)] mod tests

    static_transition.rs
      :19   mod immediate_bridge;
      :90   use immediate_bridge::ImmediateBridgeRealization;      PRIVATE use
      :91   #[cfg(feature = "px8-ds-test-support")] re-export of the harness
      :590  immediate_bridge_realizations: BTreeMap<...>            the field

    construction.rs
      :296  immediate_bridge_realizations: BTreeMap::new(),         the init

**The plane is complete and has no production caller.** That is the property
slice 2 was defined by, and **this slice is the one that ends it.**

### The in-code comment that dates itself to this slice

`static_transition.rs:86-89`, authored by slice 2, verbatim:

    // The immediate-bridge realization plane. Only the descriptor the plan
    // field stores is named here: the Stratum A classifier is still consumed
    // by nobody outside its own module, and re-exporting it now would put a
    // name on the live surface ahead of the slice that uses it.

**Slice 3 is "the slice that uses it".** That comment is a live expiry
condition, not background, and §3a below is the decision it defers.

## 2. What this WP adds

At the reference, `construction.rs` carries the wiring immediately before
response phase B. **On `main` that anchor is `construction.rs:1455`:**

    self.plan.install_static_response_context_plan_phase_b()?;

The reference inserts, directly above it:

    // HS10: continuation identities, source occurrences, and transports are
    // now final. Classify the immediate bridge population once before
    // response phase B can decide whether an owner exists.
    self.plan.immediate_bridge_realizations =
        publish_immediate_bridge_realization_plan(&self.plan)?;

plus the import:

    use super::immediate_bridge::publish_immediate_bridge_realization_plan;

**That is the whole deliverable.** The ordering claim in that comment — that
the classification must happen after identities/occurrences/transports are
final and before phase B — is the substantive design content of the slice.
**Only its LOWER half is falsifiable at this slice**, which is why `AC-2` is
split into `AC-2a` (due now) and `AC-2b` (deferred with its enabling condition
named). See §6.

## 3. THE TRAPS. All three of §3 on the slice-2 frame are LIVE HERE.

An earlier reading held that only `§3a` carried into slice 3. **Withdrawn** —
all three were re-measured at the reference and all three stand. Slice 2's
frame §3d is the durable record; do not re-derive it a fifth time.

### 3a. The reference's `use` rewrite deletes NINETEEN live names, not five

The slice-2 frame says five. **Five is correct for the one hunk it quotes and
understates the hazard fourfold**, because the rewrite is **two** replacement
hunks and the second is larger. Measured at `0f71ab5b9` against `1dec48f33`,
each name resolved against `main` individually:

    @@ -81,3 +74,4 @@   use semantic_ir::{              deletes  5
    @@ -88,5 +84 @@     use responses::{                deletes 14
                                                        ----------
                                                                19

    DeferredResponseRow 4 files          StaticResponseEffectInput         5
    DeferredResponseSubCase 3            StaticResponseEnvironmentBinding  5
    ResponseDisposition 5                StaticResponseFrameSource         5
    SsaInfeasible 2                      StaticResponseOwnerId             5
    StaticResponseCapture 4              StaticResponseOwnerSpecialization 5
    StaticResponseContextDemand 5        StaticResponsePhaseA              2
    StaticResponseContinuation 5         StaticResponseContinuationId      2

    ConstructorIdentity 14   SynthesizedConstructorRole 11
    SynthesizedFixedConstructorRole 10   FieldIdentity 9
    BoolMatchCaseOrdinals 7

    19 of 19 live on main.

**The replacements are test-support hooks, not relocated exports:**

    d5b_hs17_post_call_consumer_mutation                 0 files on main
    record_d5b_hs17_post_call_consumer_application       0 files on main

⇒ **Nineteen live production re-exports out, two names that do not exist in.**
`§3a` and `§3b` arrive in one hunk. And the adjacent `@@ -86 +82 @@` flips that
block's `#[allow(unused_imports)]` to `#[cfg(feature = "px8-ds-test-support")]`,
**gating live production re-exports behind a test feature** — which fails in
the worst direction, because a feature-on build stays green and the breakage
appears only in an ordinary build.

### 3a'. AND THE DECISION MAY BE MOOT — CHECK THIS BEFORE AUTHORING ANYTHING

`publish_immediate_bridge_realization_plan` is **`pub(super)`**, and
`construction.rs` is a sibling module inside `static_transition`. So
`use super::immediate_bridge::publish_immediate_bridge_realization_plan;`
resolves **without any re-export on `static_transition`'s surface** — which is
exactly how the reference's own `construction.rs` import is spelled.

⇒ **The expected answer to §3a's decision is "author no re-export at all."**
The classifier's visibility question does not arise for this slice, because
the wiring does not cross the module boundary that a re-export exists to cross.

**Confirm that before concluding it** — if the candidate finds it needs a name
on `static_transition`'s surface, that is a real finding and the decision
below is due. But do not author a re-export because the reference has one: the
reference's re-export serves hunks this slice excludes.

### 3b. `CheckedIhPostCallConsumer` is absent from `main` and the hunk carries it

    git grep -c 'CheckedIhPostCallConsumer' origin/main -- crates/   ->  ZERO
    at the reference                                                     6 files

**This is not a name to avoid — it is an entire unbuilt mechanism**, and the
reference's `construction.rs` wiring hunk is entangled with it. The same hunk
region adds:

    use super::responses::{
        publish_checked_ih_post_call_consumers,
        validate_checked_ih_post_call_consumers,
    };
    ...
    checked_ih_post_call_consumers: Vec::new(),                 the field init
    self.plan.checked_ih_post_call_consumers =
        publish_checked_ih_post_call_consumers(&self.plan)?;    at :1498
    validate_checked_ih_post_call_consumers(...)?;

⇒ **A verbatim lift of the reference's `construction.rs` diff does not
compile**, and would not be repairable by renaming — the mechanism is absent.
`§3b`'s zero-hit check is therefore a **PRECONDITION on a different node**, not
a prohibition on this one. See §5.

### 3c. `InlineBridgeNoCall` and the `owns_seat` rewrite

    InlineBridgeNoCall on main      ZERO
    at the reference                responses.rs 8, static_transition.rs 2,
                                    and two test files

    owns_seat on main               static_transition.rs :790, :800
                                    (an EXISTING site — the reference REWRITES
                                    it, it is not an addition)

Both were excluded from slices 1 and 2 and are excluded here. See §5.

### 3d. Every hunk count in this file and in slice 2's §3 is `-U0`

    -U0   69 hunks,  6 mentioning immediate-bridge
    -U1   53         5
    -U3   41         5                                 <- git's default

A reader reconciling against git's default will be off by 28.

### 3e. The reading discipline that cost four re-derivations on slice 2

A trap warns about a hazard **in the artifact you might lift**, which is the
reference. The ACs are claims about **the candidate**. **Confirming the
candidate did not fall into a trap is evidence the warning WORKED, not evidence
the hazard was imaginary** — and at the candidate the two are
indistinguishable. Only the reference tells them apart. Three separate
"refutations" of slice 2's §3 were each a correct measurement of the candidate
read as a claim about the reference.

## 4. THE DEAD-CODE DIAGNOSTIC IS THIS SLICE'S INSTRUMENT, AND IT INVERTS HERE

Slices 1 and 2 were defined by the plane having **no** production caller, and
their `AC-3` asserted the four functions **are** in `ken-runtime`'s `never
used` diagnostic. **This slice is defined by the opposite**, so the same
instrument is read in the other direction.

### 4a. Assert on the diagnostic's NAMED ITEMS, never on the warning count

    without the substitution   methods `producer_owner` and `identity`
                               are never used
    with it                    method `producer_owner` is never used
                               106 warnings EITHER WAY

**rustc groups several never-used items into a single diagnostic**, so the
exact event this control exists to catch — an item becoming live — **changes
the text and leaves the count unmoved.** Slice 1 phrased `AC-9` as a delta
(`88 -> 101`), slice 2 as `100 -> 106`, and copying that phrasing here gives a
control that cannot fail in the direction it is for. From outside, the failure
looks exactly like *"warnings unchanged"*.

It is the same shape as the ignored-row sweep's `findings non-blocking` job
reporting `success` whatever it finds, and it takes the same fix: **record the
diagnostic's text, not the tally.**

**`AC-9`'s warning delta is a DISCLOSURE requirement and stays a count.** A
count is the right shape for a disclosure and the wrong shape only for this
control. Do not strip `AC-9`'s figure on the strength of this section.

### 4b. WHICH names leave the set — the carry-forward draft got this wrong

The §5a draft proposed: *"re-arm when the diagnostic stops naming `build_` and
`validate_`."* **That trigger would not fire on this slice.** The call graph
inside the module, read from the bodies at `main`:

    publish_   ->  derive_, relation_from_rows
    build_     ->  derive_, relation_from_rows
    validate_  ->  build_

Wiring `publish_` from `construction.rs` makes **`publish_`, `derive_` and
`relation_from_rows`** reachable. **`build_` and `validate_` stay dead** —
`validate_` has only test callers, and `build_`'s only non-test caller is
`validate_`, which is itself unreachable.

⇒ **The trigger named the two functions this slice does not move.** The earlier
call-site form was inert because it named a function with an intra-cluster
caller; the corrected form was inert because it named the two the wiring does
not reach. **Both failures come from naming functions instead of naming the
property**, so `AC-3` below asserts on the named SET changing and then reads
which names left — it does not pre-commit to a roster.

## 5. Explicitly OUT of scope

    the CheckedIhPostCallConsumer mechanism entirely
      construction.rs   the publish_/validate_checked_ih_post_call_consumers
                        wiring at :1498 at the reference
                        the checked_ih_post_call_consumers field + init
      the use super::responses::{...} import for them

    DeferredResponseSubCase::InlineBridgeNoCall, and the responses.rs work
    static_transition.rs   @@ -776,6 +828,9 @@   the owns_seat rewrite
                           @@ -1017 +1070,2 @@   the InlineBridgeNoCall doc

**The post-call consumer mechanism has its own node and it is not this one.**
`RT-D5B-POSTCALL-REFUSAL-MECHANISM` (`ready`, runtime, M, T1, gate architect)
is an open investigation into that mechanism's refusal, carrying **three
refuted mechanisms and no candidate**. Building it is not a prerequisite for
this slice — **this slice is cut specifically so that it is not.**

**A candidate for this WP that introduces `CheckedIhPostCallConsumer`,
`InlineBridgeNoCall`, or touches `owns_seat` is out of scope and should be
bounced, not reviewed.**

## 6. Acceptance

**AC-1. The wiring, and nothing else.** `construction.rs` gains one import and
one assignment placed before `install_static_response_context_plan_phase_b()`.
Control: `git diff` for `construction.rs` is those two hunks and no other.

**AC-2a. The LOWER bound of the ordering claim is tested, not quoted.** Show a
test that goes red when the call is moved **above the specialization installs
at `construction.rs:1382-1387`**. A
candidate that only shows the wired call producing the expected plan has tested
*that it runs*, not *that it runs there*. **This is the slice's design content
and the reason it is T1 rather than a two-line port** — choosing the
perturbation and the assertion is the judgment.

**The bound is the SPECIALIZATION INSTALLS, and the mechanism is emptiness.**

    :293-297  the Vec::new() inits
    :1382     continuation_specializations
    :1383     continuation_specialization_calls
    :1385     continuation_contexts
    :1387     install_continuation_specialization_abi -> continuation_descriptors
    :1406     install_continuation_context_abi(&mut self.plan.abi, ...)
    :1455     install_static_response_context_plan_phase_b()   <- the anchor

**There are two bounds in that window, both reachable by the fixture this slice
ships, and the guard at `continuations.rs:6867` — inside `continuation_units()`
— decides both:**

    if abi.continuation_descriptors.len() != continuation_specializations.len()

    above :1382    0 == 0, guard PASSES, continuation_units() returns EMPTY
                   => derive_ iterates nothing, plane empty, `left 0, right 1`
                      RED BY EMPTINESS

    in (:1385, :1387)  specializations assigned, descriptors still empty
                   => 0 != n, guard FIRES
                      RED BY Err

`:1387` is the tighter of the two and the one to name. A candidate may satisfy
this AC at either, but it must say **which mechanism** its red is — an empty
derivation and a refused one are different results and only one of them proves
the ABI install is an input.

### `:1406` IS NOT A BOUND. Measured, refuted, and recorded so it stays refuted.

An earlier form of this AC named `:1406` as the true bound, on the reasoning
that *"`continuation_units()` reads `abi` eight times, so `:1406` is the last
input to become final"*. **Both halves fail.** A read is not a dependency — the
value has to be able to differ — and the fields do not overlap at all:

    :1406 WRITES (abi.rs:1262)     derive_'s path READS from abi
      abi.context_descriptors        abi.continuation_descriptors  x2
      abi.context_inputs             abi.continuation_inputs
      abi.context_slots              abi.continuation_slots
      abi.context_affinities
                                   => DISJOINT

`continuation_units()` is the **only** reader of `abi` on that path;
`continuation_calls`, `relation_from_rows` and `occurrences.rs` read it zero
times. Every reader of a `context_*` ABI field in `planning/` is two sites —
`continuation_contexts()` at `:6953-6972` and the prefix validator at
`construction.rs:1565-1572` — and **`derive_` calls neither.** Its whole `plan.`
call set is `continuation_units`, `continuation_calls`, `planned_occurrence_expr`,
`child_static_origin`. The construction comment at `:1402-1405` says the same
thing from the writer's side: the generated contexts go into their own arenas
and are never appended to `continuation_descriptors`.

**A second refuted rationale, recorded because it is the more dangerous one.**
It held that the guard at `continuations.rs:6943` fires above `:1406`, so the
green there was a property of a fixture with an empty context population rather
than an absent bound. `:6943` is inside **`continuation_contexts()`**, a
different function from the `:6867` guard and one `derive_` never reaches.

⇒ **A context-bearing fixture would be green above `:1406` as well.** Do not
commission one to pin this bound; it would pin nothing. Both failures ran the
same way — a true local fact carried to a conclusion about a different scope —
and the check that settles either is two greps: **what does the write touch, and
what does the read path touch.**

**AC-2b. DEFERRED, and its enabling condition is named rather than dropped.**
The reference's comment also claims an upper bound — *"before response phase B
can decide whether an owner exists"*. **That half is not falsifiable at this
slice**, and the reason is the scope decision rather than an oversight:

    derive_ reads   abi, continuation_contexts, continuation_specializations,
                    continuation_specialization_calls, child_static_origin,
                    planned_occurrence_expr
    phase B writes  static_response_plan_installed,
                    static_response_continuations, static_response_deferred
                                                        (responses.rs:2296-2330)
    => DISJOINT

    every production reference to immediate_bridge_realizations on main:
      static_transition.rs:590   declaration
      construction.rs:296        init
      immediate_bridge.rs:679    inside validate_   -- DEAD
      immediate_bridge.rs:694,701  accessors
    => NOTHING on the live path reads the field

⇒ Moving the call to after phase B changes neither its inputs nor any
observable output, so **anywhere from the specialization installs onward — past
`:1406`, past phase B — is undetectable. No test can go red for that move.**
The undetectable interval is the one AC-2a leaves, and it is wider than the
`[1406, 1455]` an earlier form of this AC named, because `:1406` is not a
partition point. The decider that would
consult this classification is the `CheckedIhPostCallConsumer` mechanism, which
§5 puts out of scope and which has zero hits on `main`. **"Before phase B" is
unfalsifiable here precisely because the consumer that makes it matter is the
next node.**

**The obligation is inherited by `RT-D5B-POSTCALL-REFUSAL-MECHANISM`**, whose
enabling condition is: *a live consumer reads `immediate_bridge_realizations`.*
Do not attempt it here.

**The WRONG repair, named in advance so it is detectable.** A test-only
ordering probe — a hook recording that the call happened before phase B — is a
check that cannot fail for the reason anyone cares about, because there is no
behaviour on the other end of it. It satisfies `AC-2b`'s words while testing
the probe. **If a candidate arrives carrying an ordering assertion, ask what
observable differs, not whether the assertion passes.**

**One thing that could refute the deferral, and it is to be MEASURED rather
than inherited:** `with_d5b_hs10_bridge_plan_mutation` (`immediate_bridge.rs:559`)
is a feature-gated plan mutation hook. **If it can perturb the plan between the
wiring point and phase B, an ordering observation with a real behavioural end
may be constructible** — in which case `AC-2b` is due now. Nobody has measured
what it reaches. **That measurement is part of this WP**: report it either way.

**AC-3. The dead-code diagnostic's NAMED SET changes, and the named items are
quoted verbatim from both builds.** Report the diagnostic's text at `main` and
on the candidate. Expected: `publish_`, `derive_` and `relation_from_rows`
leave the set; `build_` and `validate_` remain. **A candidate that reports only
a warning count fails this AC**, and one that reports a roster it did not read
from a build fails it too. If the observed set differs from the expectation
above, that is a finding about the call graph and it is reported, not
reconciled away.

**AC-4. The five `semantic_ir` re-exports and the fourteen `responses`
re-exports are present and unmodified.** Control: `git diff` shows both blocks
untouched; grep each of the nineteen on the candidate and on `main` and show
the counts agree. §3a is the failure this catches.

**AC-5. `CheckedIhPostCallConsumer` appears nowhere in the diff.** Control:
zero grep hits on the candidate. §3b.

**AC-6. `InlineBridgeNoCall` appears nowhere in the diff, and `owns_seat` is
untouched.** Control: grep for the first; `git diff` for
`static_transition.rs` shows no edit at `:790` or `:800`.

**AC-7. The re-export decision is STATED.** Say whether the candidate needed a
name on `static_transition`'s surface. If it did not — the expected answer per
§3a' — say so and **delete the now-expired comment at
`static_transition.rs:86-89`**, since its condition has been reached. If it
did, state the visibility chosen and why, and do not inherit a neighbouring
line's `pub use` spelling.

**AC-8. The `px8-ds-test-support` block is unchanged.** Control: show the
`#[allow(unused_imports)]` on the `responses` block is still
`#[allow(unused_imports)]` and was not flipped to a `cfg(feature)` gate. §3a's
adjacent hunk.

**AC-9. Warning delta is stated, not suppressed.** Report the `ken-runtime`
warning count at `main` and on the candidate. No `#[allow(dead_code)]`. Slice
1 was `88 -> 101`, slice 2 `100 -> 106`. **This is a disclosure, not `AC-3`'s
control** — see §4a.

**AC-10. Per-case provenance.** For every test expectation, say whether it is
**semantic** (lifted from an attested ancestor, cited verbatim by file and line
at `0f71ab5b9`) or **structural** (authored freely). A test whose expectation
was read off the implementation goes red against a stub while proving nothing.
Any expectation with no ancestor is declared new intent by name.

**AC-11. No `#[ignore]` rows added.** Control: zero added `#[ignore]`
*attribute* lines in the diff — anchor on `^\s*#\[ignore`, not on the token,
or comments discussing the attribute join the population. **`main`'s own count
is 23 and moved during slice 2's window, so this AC's subject is what the
candidate ADDS, not an equality with `main`.**

**AC-12. No decorative glyphs in the diff.**

## 7. Sizing and contention

**S, and it should stay S.** The deliverable is one import and one assignment.
The hour goes into `AC-2a` — building an ordering test that actually goes red
when the call moves above the specialization installs — and into `AC-3`'s two
diagnostic reads. If
`AC-2a` cannot be satisfied without restructuring the planner, **that is the
finding: report it and stop.** Do not grow the node to chase it.

**T1 despite being S**, because `AC-2a` is a judgment about what the ordering
claim means and `AC-3` is a judgment about what the diagnostic is saying.
Neither is mechanical. §4b is what a mechanical reading of this instrument
produces. **`AC-2b`'s deferral is not a downgrade** — the half that remains is
the half that carries the judgment.

Contention: `construction.rs` is high-traffic and this WP's diff to it is two
hunks. Coordinate with any in-flight runtime candidate touching `planning/`.

## 8. Local build discipline

Targeted only, through `scripts/ken-cargo`, `-p ken-runtime`. **Never
`--workspace`.** The workspace build, the `--locked` gate and the conformance
suite run in CI. `COORDINATION §12`.
