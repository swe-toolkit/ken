# WP frame — `RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE`

**Owner:** Team Runtime · **Size:** S/M · **Risk:** medium (the gate sits in a
function **96** call sites traverse — 15 outside `effect_v1.rs`, 81 in its test
module) · **Tier:** T1 · **Gate:** none ·
**Deps:** none — this node blocks
[[RT-D5B-MAPPING-AVAILABILITY-FLIP]], not the reverse

**Origin:** adversary Finding 1 on the landed slice 4 (*"statements !=
enforcements"*), routed by the Steward to the Architect as a design question
rather than ruled, and **RULED** at `evt_21f23zmgqfxsc`. Cut as its own node on
the Steward's sequencing ruling; the Architect adopted the split at
`evt_3ws5c4xzbfxa5` (*"my heading was wrong, the paragraph is the ruling"*).

**RE-FRAMED 2026-09-16 after D0 completed and hit a hard stop.** D0 is
**ANSWERED** — its results are now §3, as fixed inputs. The stop was condition
2 (`evt_4w12zca5j1g03`), the Architect closed the design fork it raised with a
third option (`evt_1y4rvywv1y6fr`), and the Steward's sizing measurement
(`evt_5sxcg2m9qah9j`) **withdrew the stop**: under the ruled design, condition
2 does not bind. Size moves `S` to `S/M`. **Do not re-run D0.**

**CORRECTED 2026-09-16, mid-build, on runtime-implementer's findings against a
built candidate.** Four corrections, all to Steward-authored text, none changing
the ruled design or the sizing:

    §3a         a call-site population was read as a statement about REACH.
                Two ken-interp tests reach the gate transitively.
    §3d         the funnel is 23 call sites / 10 tests, not 21 / 8. The two
                missed sites pass their op through a VARIABLE.
    §4 point 6  "all of them dispatch available ops only" -- FALSE, withdrawn.
    §4 point 8  NEW. The ruled disposition of the two ken-interp tests
                (Architect evt_rsbhqs2tfamg, a REVISION of an earlier ruling),
                and the unit-error-type trap that forces positive controls.
    §9          NEW. The assertions this WP retires, verbatim, owed by
                [[RT-CLOCK-OP-NATIVE-PROMOTION]].

## 1. Objective

Make *"`RepresentedUnavailable` implies refused"* an **enforced** invariant
rather than a **stated** one, by gating `dispatch_host_op_v1` at the convergence
both executors traverse, and adding one control that derives the unavailable set
from `availability()`.

**This is REMEDIAL.** Four of the ten unavailable ops execute interpreted and
refuse natively **on `main` today**.

**Those four arrive by two different mechanisms, and that is the finding — not
the four.** State the split; compute the total at the point of use:

    3 via the op intern table   prelude.rs:190-192 (see §2)
    1 via a separately interned global   PrivateMappingAcquireFile
    6 with no Ken surface at all

⇒ **A node that records "4 reachable" invites a re-derivation that yields 3**,
because the obvious derivation reads the intern table and `MappingAcquireFile`
is not in it — and `MappingAcquireFile` is the op
[[RT-D5B-MAPPING-AVAILABILITY-FLIP]] exists for. The measured trap is in §2.

## 2. Fixed inputs

All measured at `origin/main` `de472b7dbb64fcbaf85c01bc803097c7d47f41f7`.
The two files this WP edits are **byte-identical** to the earlier measurement at
`09150fe8afad2baa88d404ef3e9bdba3e0b997ab`, so every number below carries
across that move by blob identity, not by assertion:

    crates/ken-host/src/effect_v1.rs   d90b78f1f433c242c4130f97ed633447c7055955
    crates/ken-host/src/abi_v1.rs      4bdf3ed4e2098bba9fafdb6d02c7a1550826112c

### 2a. The dispatch surface

    dispatch_host_op_v1          crates/ken-host/src/effect_v1.rs:2628
    native gate (STAYS)          crates/ken-host/src/abi_v1.rs:1551
    availability()               crates/ken-host/src/effect_v1.rs:158-196, 35 arms,
                                 no wildcard -> 25 NativeTested / 10 RepresentedUnavailable
    HostOpAvailabilityV1         :694-697, exactly two variants, so the partition
                                 is total and no op can hide in a third class

**Four production call sites, not three.** An earlier cut of this frame listed
three and resolved `native_effect_v1.rs:102` as unresolved:

    crates/ken-interp/src/eval.rs:5593              in fs_dispatch      (:5147)
    crates/ken-interp/src/eval.rs:5844              in ambient_dispatch (:5833)
    crates/ken-host/src/abi_v1.rs:2025              ken_host_dispatch_v1
    crates/ken-runtime/src/native_effect_v1.rs:102  in fn dispatch      (:97)

**`native_effect_v1.rs:102` is production and has ZERO production callers.** It
compiles in production builds — there is no `cfg` on it — but the only reference
to the module outside its own file is `mod native_effect_v1;` at
`ken-runtime/src/lib.rs:40`; the three `KenNativeInvocationV1` hits elsewhere are
comments. `#![allow(dead_code)]` at `native_effect_v1.rs:7` is what stops rustc
reporting this. **It is a latent surface, governed by the predicate the day it
gains a caller.** Falsifier, recorded so the ruling is checkable rather than
believed: *any production reference to it outside its own file flips this.*

### 2b. The two existing refusal sites, both producing `OperationUnavailable`

    effect_v1.rs:558-568   host_effect_wire_layout_v1 (fn begins :399)
                           ENUMERATES the ten by name -- see §4 point 5, the
                           enumeration is a deliberate build break, LEAVE IT
    abi_v1.rs:808-814      require_native_operation_v1
                           DERIVES from availability()

One stale artifact, recorded so nobody re-finds it: the comment at
`effect_v1.rs:555` says *"the **twenty-two** matched above are exactly the
`NativeTested` set."* Re-measured over the enclosing function — 35 variants
named, 10 in the refusal arm — **the number is 25, stale by three promotions.**
Only the prose is stale; the mechanism is correct. Fixing the comment is not in
this WP's scope and is not an AC.

### 2c. All ten unavailable ops have real executing arms

Measured by runtime-implementer (`evt_3ky3xsa3x0frj`) inside
`dispatch_host_op_v1` (`:2628-3435`):

    ClockMonotonicNow   :2895      ClockSleepUntil    :2898
    EntropyRandomBytes  :2902      FsSeek             :3008
    FsSetLength         :3026      FsSync             :3044
    FsGetInheritance    :3062      FsSetInheritance   :3080
    FsDuplicate         :3098      MappingAcquireFile :3200

These call into the backend and return real replies — `:2895` is
`Ok(CanonicalReplyV1::MonotonicInstant(backend.clock_monotonic_now()))`. (Each
op has a second hit around `:2697-2807`; that is the request-shape validation
match, not the executor. Do not count it twice.)

⇒ **Zero of the ten are un-implemented.** `RepresentedUnavailable` means
*"implemented and executing at the dispatch layer, refused at the native ABI
boundary"* — for all ten, not for a subset. This is what makes §3b's test
population meaningful and it is why the design is a split rather than a deletion.

### 2d. The reachability routes, and the census that gets them wrong

    route A   prelude.rs:190-192   op_clock_monotonic_now  => "MonotonicNow"
              (the op_* intern     op_clock_sleep_until    => "SleepUntil"
              table, 18 entries)   op_entropy_random_bytes => "RandomBytes"

    route B   prelude.rs:2148      "PrivateMappingAcquireFile" registered as a global
              prelude.rs:2165      let private_mapping_acquire_file_id = elab.globals[..]
              prelude.rs:2192      its declared Ken type
              eval.rs:5342         } else if op_id == fs.private_mapping_acquire_file_id {
              eval.rs:5360         -> HostOpV1::MappingAcquireFile

`prelude.rs` here is **`crates/ken-elaborator/src/prelude.rs`**, not
`ken-interp`.

**Both obvious single-instrument censuses return a clean wrong answer**, in
opposite directions — measured at this SHA:

    grep the 10 Rust variant names against prelude.rs
        -> MappingAcquireFile 5, all other nine 0
        -> reports ONE reachable op, and the 5 hits are substrings of the Ken
           identifier "PrivateMappingAcquireFile", not the Rust variant
    derive from the op_* intern table
        -> reports THREE, dropping MappingAcquireFile, which is not in it

Neither route alone reaches 4. **If you state the 4, state both routes beside
it.**

### 2e. `availability()` is read by two boundaries asking different questions

    abi_v1.rs:1551    "has native/interp parity been differentially confirmed?"   -> 10
    this WP's gate    "may a Ken program cause this to execute?"                  ->  4

Nothing in the tree records this distinction, which is why this WP kept
re-deriving it. The gate's predicate must nonetheless stay `availability()`:
`ken-host` cannot depend on the language surface without a layering inversion
worse than the bug. **The entry split (§4 point 6) is what reconciles the two.**

## 3. D0 — ANSWERED. These are inputs, not work.

`dispatch_host_op_v1` has **96** call sites — 81 inside `effect_v1.rs` and 15
outside. **The population is closed, and it is closed CHECKABLY rather than on
authority.** The obvious instrument returns a different number, so the
reconciliation is written out:

    git grep -c dispatch_host_op_v1 -- 'crates/**/*.rs'     ->  102 raw hits

     81   call sites in effect_v1.rs
      1   the DEFINITION at effect_v1.rs:2628 -- not a call site
      3   `use` imports    abi_v1.rs:15, scenario.rs:1778, native_effect_v1.rs:10
      2   COMMENTS         ken-cli/tests/rt_escape_second_resource_native.rs:562
                                                                          :597
     15   call sites outside effect_v1.rs
    ---
    102   every raw hit accounted for

⇒ **The two `ken-cli/tests/` hits are prose inside a `//` comment about
`ken_host_dispatch`, not call sites.** They are named here because a re-derived
census finds them, they sit in an integration test, and *"no integration-test
tail"* stated as a bare conclusion reads as refuted by them. The claim is about
**call sites**; the file genuinely contains the string.

`ken-verify` has zero production dispatch sites (`catalog.rs:14`
`deferred_named_lanes` is called at `:348`/`:352`, both inside the `cfg(test)`
mod at `:340`). The sweep covers all **505** `.rs` files under `crates/` at this
SHA.

### 3a. Fifteen outside `effect_v1.rs` — all resolved

    crates/ken-host/src/abi_v1.rs                     :2025  :4549
    crates/ken-interp/src/eval.rs                     :5593  :5844  :7168  :7200
                                                      :7234  :7264  :7316  :7641  :8428
    crates/ken-runtime/src/native_effect_v1.rs        :102
    crates/ken-runtime/src/object_linker_packaging.rs :3802  :3834
    crates/ken-verify/src/scenario.rs                 :4988

Four are production (§2a); eleven are test-only. **None of the fifteen names a
`RepresentedUnavailable` op at the call site.**

> **CORRECTED — the clause that used to follow that sentence was wrong, and it
> was the Steward's.** The withdrawn text read *"so none of them changes
> behaviour under the gate."* **The population here is CALL SITES; the gate's
> blast radius is TRANSITIVE REACH.** A test that never mentions
> `dispatch_host_op_v1` still changes behaviour if it calls a helper that does.
> Two `ken-interp` tests do exactly that, through `ambient_dispatch` (whose own
> production dispatch sites are `eval.rs:5593`/`:5844`), with
> `ClockMonotonicNow` and `ClockSleepUntil` — both `RepresentedUnavailable`:
>
>     eval.rs:8955   ac2_monotonic_readings_survive_a_wall_clock_step_backwards
>     eval.rs:9039   ac3_the_deadline_a_caller_passes_is_the_deadline_honoured
>
> Found by runtime-implementer against a built candidate, not by re-reading the
> census. **A census of call sites is complete and correct about call sites and
> silent about reach** — the same shape §7 records against `AC-AVAIL`, one
> layer over. §4 point 8 carries the ruled disposition; §9 carries what it
> defers.
>
> ⇒ **The reachability question this WP is about is asked of the transitive
> closure, never of a call-site roster.** Anyone re-deriving §3a's fifteen will
> get fifteen; that is the right answer to the wrong question.

**The method, because a text match gets it wrong.** Resolve each site by
**`cfg` satisfiability under `test=false`**, not by a literal `cfg(test)` text
match. `scenario.rs:4988` sits under `#[cfg(all(test, target_os = "linux"))]`;
a text match for `cfg(test)` does not fire on it and reports it as production —
a false trip of hard-stop condition 1. **Resolve the enclosing binding; an
attribute decorates the item beneath it, which may be a single `fn`.**

### 3b. The visibility split is a CRATE BOUNDARY, not a count

This is what the design turns on, so it is stated as a boundary — a count here
would name a population that does not exist (the Steward wrote "11 out-of-crate"
and it is 10; runtime-implementer corrected it at `evt_3gsr8z9y1zyhy`):

    IN-CRATE  (ken-host -- pub(crate) reaches these; E0603 does NOT)
        abi_v1.rs:2025   PRODUCTION   ken_host_dispatch_v1
        abi_v1.rs:4549   test         reaches it by the crate:: path today
        + all 81 sites in effect_v1.rs's own test module

    OUT-OF-CRATE (E0603 holds these on the gated entry, by the compiler)
        ken-interp   eval.rs x7
        ken-runtime  object_linker_packaging.rs x2, native_effect_v1.rs:102
        ken-verify   scenario.rs:4988

⇒ **`pub(crate)` is availability; E0603 coverage is reachability** — the WP's
own subject, one layer down. The boundary does most of the enforcement and does
not do all of it, and **the two in-crate sites are exactly where the AC has to
be** (§5, AC-IN-CRATE). `:4549` dispatches only available ops
(`MappingAllocate` / `MappingWriteView` / `MappingReadView` / `ResourceRelease`)
so it is free to stay on the gated entry — but nothing but an AC keeps it there.

### 3c. Eighty-one inside `effect_v1.rs`, resolved test-only by MEASUREMENT

**The obvious reasoning for that resolution is the reasoning §3a forbids**,
which is why the measurement is written out rather than the conclusion.

    #[cfg(test)]  :4109   decorates   mod tests  :4110
    mod tests is the LAST column-0 item in a 9465-line file -> it runs to EOF
    all 81 call sites lie in :4649-:9429 -> inside it
    :2628 is the DEFINITION, not a call site

**`effect_v1.rs` is an INSTANCE of §3a's hazard, not an exception to it.** It
carries **four** `#[cfg(test)]` attributes and **three decorate a single `fn`**:

    :1179 -> :1180   fn is_native_mapped
    :1387 -> :1388   fn insert_fs_handle_without_provenance_for_test
    :2184 -> :2185   fn force_generation_for_test
    :4109 -> :4110   mod tests                    <- the only one on a module

Read the file as having "one `#[cfg(test)]`, on a `mod`" and you will be
fractionally less careful in the one file where that care is the deliverable.

### 3d. The TEN tests, and why the sizing came back S/M

Ten tests in `effect_v1.rs`'s own test module dispatch unavailable ops today,
across **23** of the 81 in-file call sites. Most of them assert the executing
arm's behaviour — they are the implementation evidence the promotion protocol
consumes, not incidental coverage. Under a naive gate they all die, which is
what raised the fork.

The funnel, by enclosing test function, measured at the §2 blob:

    fn @ :4783  all_pre_resource_operations_share_one_semantic_dispatch   1  :4912
    fn @ :4958  ac4_entropy_reports_unavailable_rather_than_supplying...  1  :4965
    fn @ :5039  ac3c_entropy_needs_no_capability_token_while_a_gated...   1  :5045
    fn @ :6293  abi_s6_d4_file_acquire_enforces_rights_exact_length...    1  :6320
    fn @ :6451  abi_s6_d4_file_acquire_reads_real_source_from_offset_zero 1  :6460
    fn @ :7431  abi_s1_seek_and_set_length_preserve_cursor_extent...      5  :7460 :7477 :7498 :7518 :7536
    fn @ :7565  abi_s1_sync_modes_execute_and_share_the_write_right       2  :7589 :7606
    fn @ :7640  abi_s1_inheritance_get_set_are_typed_and_right_separated  2  :7669 :7686
    fn @ :7777  abi_s1_duplicate_preserves_policy_rights_and_revocation   3  :7804 :7820 :7916
    fn @ :8065  abi_s1_descriptor_failures_preserve_px9_file_error_ident  6  :8077 :8099 :8119 :8139 :8158 :8185
    ----
    10 tests, 23 call sites

⇒ **Re-point 23 call sites at the inner entry. No setup line and no assertion
moves.** That is why hard-stop condition 2 does not bind and why this is S/M
rather than a recut.

> **CORRECTED from 21 sites / 8 tests, which was the Steward's number
> (`evt_5sxcg2m9qah9j`) and the basis on which the hard stop was withdrawn.**
> runtime-implementer measured 23/10 against a built candidate. The sizing
> conclusion is unchanged; the roster was short by two.
>
> **The mechanism, because it will recur.** My classifier keyed on a
> `HostOpV1::<Variant>` literal appearing at the call site. The two missed
> sites carry the op **through a variable**, so no literal is adjacent to the
> dispatch and nothing matched:
>
>     :4912   `for (operation, request) in requests` — a vector of pairs whose
>             members include EntropyRandomBytes among many available ops
>     :5045   inside `let dispatch = |operation, request| { ... }`, invoked with
>             EntropyRandomBytes and with a gated op
>
> **A literal-keyed census cannot see an op that arrives as a binding**, and
> both of these tests exist precisely to exercise *several* ops uniformly —
> which is what makes them pass their op as data. The instrument that gets it
> right is the enclosing-function one used above: find every call site, resolve
> its enclosing `fn`, then ask whether that function's body mentions any of the
> ten. It returns 23/10 with no hand-matching.
>
> The earlier `~900 setup lines / 66 assertions` figures were measured over the
> eight-test population and are not re-derived here: nothing depends on them
> since AC-TEST-DIFF-SHAPE replaced the assertion-count criterion.

**One trap, named because both halves look alike.** `:4965`'s "unavailable" is
**backend-level** (`EntropySource::Unavailable`), not op-level
`RepresentedUnavailable`. Under a gate both halves die at
`.expect("entropy dispatch is total")`. Re-point it; do not reason about it as
though it were testing this WP's predicate.

## 4. RULED, and not open to re-litigation

From `evt_21f23zmgqfxsc`, `evt_3ws5c4xzbfxa5`, `evt_17w6mab5k1y4a` and
`evt_1y4rvywv1y6fr`:

1. **The invariant is `RepresentedUnavailable` implies REFUSED BY EVERY
   EXECUTOR.** "Permissive by design" was considered and rejected. The name is
   not doing double duty and `effect_v1.rs:249-251` does **not** need rewording.
2. **The gate goes in `dispatch_host_op_v1`.** The native gate at
   `abi_v1.rs:1551` **stays** — it is at the FFI boundary and returns an
   ABI-shaped code, correct for that layer. Double-gating is defense in depth.
3. **Do NOT relocate the gate to a caller.** The interpreter's two production
   callers are in *different helpers*. A gate in `fs_dispatch` — where the
   `MappingAcquireFile` finding pointed, and the obvious place — covers the op
   that started this and **misses clock and entropy entirely.** If you find
   yourself reasoning toward a caller, that is the failure this clause exists to
   catch. **Point 6's split is not a breach of this**: it keeps enforcement at
   the convergence and moves the *body* inward, not the *gate* outward.
4. **The refusal's shape is RULED — it is not a design fork and needs no
   proposal.** The function's existing error type already carries the right
   variant with the right payload:

       effect_v1.rs:2628  -> Result<HostDispatchReplyV1, TerminalErrorV1>
       effect_v1.rs:4068     TerminalErrorV1::OperationUnavailable(HostOpV1)

   At the top of the function, before the capability match at `:2638`:

       if operation.availability() == HostOpAvailabilityV1::RepresentedUnavailable {
           return Err(TerminalErrorV1::OperationUnavailable(operation));
       }

   **The layering objection does not apply, because this is the value the other
   two refusal sites already produce** — `effect_v1.rs:558-568` and
   `abi_v1.rs:808-814`. The ABI-shaped *code* is `abi_v1`'s business and stays
   at `:1551`; `TerminalErrorV1` is what `ken-host` already speaks here.

   **Returning anything else would undercut the objective.** The invariant is
   *one* refusal for an unavailable op regardless of executor; two paths refusing
   with different values hold it in the weak sense and fail it where anyone would
   test it. **Same variant, same payload, both paths.** Cite this in the PR body;
   do not re-derive it.

5. **Do NOT convert `effect_v1.rs:558-568` to a predicate.** That arm enumerates
   the ten by name and **the enumeration is the enforcement** — the comment at
   `:545-551` records that it replaced a `_` wildcard precisely so a new
   operation becomes `error[E0004]` instead of a runtime refusal. Deriving it
   from `availability()` would delete a build break. It looks like the WP's own
   value applied one screen further down. It is not.

6. **SPLIT THE ENTRY — the ruled design.** `dispatch_host_op_v1` keeps its name,
   its signature, and all four production callers, and **gains the gate above
   the capability match**. The body below the gate becomes a **`pub(crate)`
   inner entry** that the in-crate evidence tests call directly.

   **Visibility is the enforcement, and it is a COMPILER enforcement (E0603)** —
   the same rule as point 5: enforce where the compiler can. The out-of-crate
   callers cannot reach the inner entry.

   > **CORRECTED. The clause that used to close that sentence — *"and do not
   > need to; all of them dispatch available ops only"* — is FALSE, and it was
   > the Steward's.** Two `ken-interp` tests reach `dispatch_host_op_v1`
   > transitively through `ambient_dispatch` with `ClockMonotonicNow` and
   > `ClockSleepUntil` (§3a). They are out-of-crate, so `pub(crate)` gives them
   > **no evidence path at all** — E0603 is the enforcement working exactly as
   > designed, and the two tests are its first casualties.
   >
   > **This does not reopen the split.** Widening the inner entry to `pub` is
   > refused on the same mechanism as §4 point 7: `ambient_dispatch` is one of
   > the production out-of-crate callers, so a `pub` inner entry hands the
   > bypass to the very crate the gate is aimed at. The disposition is point 8.

   **Name it for what it is** — the unpromoted / implementation-evidence path.
   **Not** `unchecked`, `raw`, or `unsafe`: those name the absence of a check,
   and what this entry actually serves is the evidence that produces a
   promotion. A caller reading the name should understand why the path exists,
   not merely what it skips.

7. **The "relabel the unavailable ops" alternative is REFUSED, on a mechanism.**
   The reading that these ops are merely mis-labelled is refuted not by a comment
   but by `abi_v1.rs:808-809`, which derives the native gate live from
   `availability() == NativeTested`, consumed at `:1551`:

       abi_v1.rs:1551   if require_native_operation_v1(op).is_err() { return -(0x1_0000 + op) }

   ⇒ **Relabelling an op is not relabelling. It opens the native ABI path** on
   the strength of a label whose entire job is to record that differential
   parity evidence **does not yet exist**. That inverts the promotion protocol:
   it asserts the evidence instead of producing it. (`effect_v1.rs:250` says the
   same thing in prose — *every operation remains `RepresentedUnavailable` until
   its artifact differential gates promote it explicitly* — but a comment loses
   to a plausible relabel proposal in three months, and this does not.)

8. **THE TWO `ken-interp` TESTS BECOME REFUSAL TESTS, IN PLACE.** Architect,
   `evt_rsbhqs2tfamg` (a revision of an earlier ruling that moved them into
   `ken-host`; this text is the one that stands). Both stay in
   `crates/ken-interp/src/eval.rs` and invert: each asserts that dispatching its
   `RepresentedUnavailable` op through `ambient_dispatch` now **refuses**.

   **Why they do not move, and it is not a preference.** `ken-host`'s test
   backend is an **op-tag recorder, not a value recorder**:

       clock_sleep_until(_deadline)  pushes the tag and DISCARDS the deadline
       clock_monotonic_now()         pushes the tag and returns a CONSTANT

   So *"the deadline a caller passes is the deadline honoured"* and *"monotonic
   readings are non-decreasing under a backwards wall step"* are not merely
   awkward to relocate — they are **inexpressible** in `ken-host` without
   extending that backend to capture values. That is new surface in a WP
   carrying AC-NATIVE-UNTOUCHED and a diff-shape AC written to prevent exactly
   it. **The evidence is DEFERRED, not lost** (§9): the promotion WP must extend
   the backend anyway, because a differential against a constant-returning
   recorder is vacuous.

   **`ac2`'s third property does not move and does not need to.** The
   wall-vs-monotonic constructor distinction is a claim about two `GlobalId`s,
   so it survives without dispatching anything:

       assert_ne!(clock.mkinstant_id, clock.mk_monotonic_instant_id);

   **POSITIVE CONTROLS ARE MANDATORY HERE, and the reason is a type.** Carry
   this trap; it is the one nobody had named:

       fn ambient_dispatch<H: HostHandler>(...) -> Result<EvalVal, ()>

   **The error type is UNIT — the refusal REASON is erased at the `ken-interp`
   boundary.** An inverted assertion there can say *"it refused"* and cannot say
   *"because `OperationUnavailable`"*. It passes for **any** failure, including
   one this WP introduces by accident. An inverted test with no available-op
   companion is an unguarded negative.

       ac2   control is FREE -- ClockWallNow is NativeTested and already read in
             the same closure. Assert the wall reads still SUCCEED while the
             monotonic read refuses. Two-sided, and a better test than the one
             it replaces.
       ac3   has NO available op in it. It MUST dispatch one through the same
             helper and assert success alongside the refusal.

   **Relocating a test means relocating the CLAIM, re-expressed in the
   destination's observation vocabulary — never the test text.** The earlier
   ruling was refuted on exactly this: `ac3`'s assertions are written in
   `ClockTrace` / `CaptureHost` / `BigInt`, `ken-interp` types with zero
   occurrences in `ken-host`, so moving the text does not compile. That is an
   argument about vocabulary, and the backend measurement above is the
   independent argument about *expressibility*. Both point the same way.

## 5. Acceptance

**AC-GATE. A `RepresentedUnavailable` op is refused on the interpreter path.**
Demonstrated, not asserted: a test that goes **red without the gate**. A test
that passes both with and without it measures nothing.

**AC-PREDICATE. The control derives its set; it never lists it.**

    for every op in HostOpV1::ALL with availability() == RepresentedUnavailable,
        the interpreter path refuses it

Ten ops satisfy that today. **An op-specific test is a defect, not a smaller
version of this** — it reproduces exactly the enumeration failure already filed
against `AC-AVAIL`, and it would say nothing about the next unflipped surface.
**Control on the control:** adding a hypothetical eleventh unavailable op must be
covered with **zero edits** to the test. If satisfying that needs the test
amended, the set is being listed somewhere.

> **This does not make enumeration wrong, and §4 point 5 is the case that
> proves it.** The governing rule (Architect):
>
> **Enumerate when the COMPILER's exhaustiveness check is the enforcement.
> Derive when a TEST is the enforcement.**
>
> A named match arm makes adding a variant a build break, which is the only way
> to get one. A test over a named list cannot fail for an op absent from the
> list — which is `AC-AVAIL`'s defect exactly. Both constructs pick what actually
> enforces; they differ because the enforcing mechanisms differ. **AC-PREDICATE
> governs the new test. It does not govern a match arm.**

**AC-INNER-ENTRY. The inner entry is `pub(crate)` and named for the evidence
path it serves.** Not `unchecked`, `raw`, or `unsafe` (§4 point 6). The gated
`dispatch_host_op_v1` keeps its name and signature.

**AC-IN-CRATE. Both in-crate sites are dispositioned by name, and
`ken_host_dispatch_v1` calls the GATED entry.** The population is **two** —
`abi_v1.rs:2025` (production; must call the gated entry) and `abi_v1.rs:4549`
(test; dispatches available ops, stays on the gated entry). These are the two
sites `pub(crate)` does not enforce, so naming them *is* the enforcement. An AC
that names only `:2025` leaves the one site that can silently migrate later
uncovered — and it is a test, so it would migrate with no production symptom.

**AC-TEST-DIFF-SHAPE. The diff to the ten named tests touches only the dispatch
entry name, at the 23 call sites in §3d. No assertion line and no setup line
changes.** Checkable by `git diff` over ten named functions. (**23/10, corrected
from the Steward's 21/8** — see §3d for the two that a literal-keyed census
cannot see.)

**AC-INTERP-REFUSAL. The two `ken-interp` tests are inverted in place, each with
a two-sided control.** `ac2_monotonic_readings_survive_a_wall_clock_step_backwards`
(`eval.rs:8955`) and `ac3_the_deadline_a_caller_passes_is_the_deadline_honoured`
(`eval.rs:9039`), per §4 point 8. Each must, in one test:

    assert the RepresentedUnavailable op is REFUSED through ambient_dispatch
    assert a NativeTested op SUCCEEDS through the same helper

**The second half is not optional and it is not symmetry for its own sake.**
`ambient_dispatch`'s error type is `()`, so a bare `.is_err()` passes for any
failure whatsoever — including one this WP causes. A candidate whose inverted
test has no adjacent success is a check that cannot fail. `ac2`'s control is its
existing `ClockWallNow` reads; `ac3` has none and must add one.

**AC-DEFERRAL-CARRIED. §9's retired assertions are reproduced verbatim in this
node AND inherited by name in the node that will promote `ClockMonotonicNow` or
`ClockSleepUntil`.** Not *"see the gate WP"* — the assertions, spelled out, in
the promotion node. A deferral that is not carried by name evaporates, and the
carrier here is [[RT-CLOCK-OP-NATIVE-PROMOTION]].

> **Phrased this way on a finding against an earlier draft of it.** That draft
> said *"the eight tests keep their assertion count at 66"* — and **a count
> cannot fail for the reason it exists.** A re-expression that deletes one
> assertion and adds another elsewhere holds 66 exactly, which is precisely the
> lossy substitution this AC guards against, and "66 unchanged" is the number
> that would get quoted in the gate report. **A number that is sound as a
> MEASUREMENT is not thereby sound as a CRITERION, and the promotion between the
> two is invisible.** Discharge on what happened, not on a tally.

**AC-NATIVE-UNTOUCHED. `ClockWallNow` and the other 24 `NativeTested` ops are
unaffected.** The predicate gives this by construction — it derives the refusal
set from `availability()` rather than naming it — so the AC is discharged by the
control's *shape*, and a candidate that needs a separate allow-list to satisfy it
has the wrong shape.

**AC-CONSUMERS-DOC. `availability()` carries a doc comment naming its two
consumers**, and stating that `RepresentedUnavailable` is a claim about **native
parity evidence**, not about executability (§2e). Four lines. **This is the
artifact whose absence made this WP re-derivable** — the `10 on native parity /
4 on Ken reachability` distinction had to be carried in prose through three
seats because the tree records it nowhere, and the next reader starts from zero.

**AC-NO-REGRESSION. Workspace-green in CI**, never a local `--workspace` run.
Local verification is targeted only, through `scripts/ken-cargo`: `-p ken-host`
and `-p ken-interp`.

> **`ken-runtime` and `ken-verify` are expected unaffected, and that
> expectation is now CI's to confirm, not D0's.** §3a resolved them by call-site
> population, and the correction there is that a call-site population is silent
> about transitive reach — the same reasoning that missed the two `ken-interp`
> tests covered these two crates. Nothing suggests they are affected; the point
> is that the argument which said so does not hold, so **workspace-green in CI
> is the evidence, and a local `-p` pass on the two touched crates is not.**

> **A suite total is not evidence about a test whose subject it does not
> exercise.** `122/0` on `ken-host` was reported as a gate result for slice 4 and
> carried no information about the one test then in question. When you quote a
> total, say which of these ACs it bears on.

## 6. Contention

`crates/ken-host/src/effect_v1.rs`, shared with
[[RT-D5B-RESOURCE-TABLE-LIFECYCLE]] (node re-cut, **implementation not
started**) and [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] (draft, unsized, blocked on
this). **This node goes first** — it is remedial and small; the lifecycle
transplant is neither urgent nor started.

Files expected: `ken-host` only (`effect_v1.rs`, `abi_v1.rs`). Zero cranelift
expected; **confirm rather than assume** — `git diff --name-only <base>..<cand>
| grep -c cranelift` is `0`.

## 7. What this node is NOT

- **Not a stronger availability census.** Its subject is **REACHABILITY**.
  Every `AC-AVAIL` site is a native-availability site, so the corrected
  membership predicate — a real improvement over the enumeration it replaced —
  **still passes against this defect.** The census was complete and answering a
  different question than it was read as answering. These controls sit
  *alongside* `AC-AVAIL`, never inside it, and anyone folding them together will
  drop one.
- **Not the flip.** `MappingAcquireFile` stays `RepresentedUnavailable` here.
- **Not a deletion of the ten executing arms.** All ten stay live and reachable
  through the inner entry (§2c, §4 point 6). A candidate that deletes an arm has
  taken the refused branch of the closed fork.
- **Not a relabelling of any op** (§4 point 7).
- **Not an unblock of `conformance/surface/ffi-io/seed-mapping.md:53`.** That
  stays BLOCKED-ON-ABI-S6-D5b until the **flip**. The Architect cancelled the
  earlier F4 advice to the contrary by name.
- **Not a rewording of `effect_v1.rs:249-251`**, and **not** a removal of the
  native gate at `abi_v1.rs:1551`.
- **Not a repair of `ambient_dispatch`'s erased refusal reason.** Its
  `Result<EvalVal, ()>` means a Ken program cannot be told **why** an op was
  refused — this WP makes that erasure more visible by adding a refusal, and
  does not fix it. Filed as [[RT-INTERP-REFUSAL-REASON-ERASED]]. Architect ruled
  it out of scope; a candidate that widens the error type here has taken work
  this node does not carry. §4 point 8 is the local mitigation, not the fix.

## 8. The structural finding this node instances

Recorded because it predicts where the next one is, not as scope.

    1. gate location conflates the executor path with the implementation-
       evidence path       -- keyed on dispatch_host_op_v1 serving two consumers
    2. RepresentedUnavailable conflates native-parity-unconfirmed with
       must-not-execute    -- keyed on availability() serving two consumers

**Shared predicate: one artifact serves two consumers asking different
questions, and the tree records no distinction between them.** One at a
function, one at a classifier. The structural closure is not more gates — it is
that **a shared authority must name its consumers at its definition**, which is
what AC-CONSUMERS-DOC does for `availability()` and what naming the inner entry
does for `dispatch_host_op_v1`. Look for the next one wherever a single
`availability()`-shaped classifier is read by two boundaries.

## 9. DEFERRED EVIDENCE — retired here, owed by the promotion node

**These assertions are removed by this WP and are not replaced by anything it
lands.** They are reproduced verbatim, at their line numbers on `main`
`050da9bcefb5a7251003071a7181ed7f473fbea9`, because a deferral carried as a
pointer is a deferral that evaporates (Architect, `evt_rsbhqs2tfamg`, condition
1). [[RT-CLOCK-OP-NATIVE-PROMOTION]] inherits them by name and must reproduce
them — against a value-capturing backend — before promoting either op.

**What is lost, in one line each:**

    ClockMonotonicNow   monotonic readings stay non-decreasing while the wall
                        clock is scripted BACKWARDS, with the backwards step
                        asserted first so the result cannot be vacuous
    ClockSleepUntil     the deadline a caller passes is the deadline the host
                        observes, discriminated against a second deadline so it
                        cannot hold for a host that ignores the argument

### 9a. `eval.rs:9012-9030`

From `ac2_monotonic_readings_survive_a_wall_clock_step_backwards`
(`crates/ken-interp/src/eval.rs:8955`):

```rust
        assert!(
            nanoseconds(&wall_second) < nanoseconds(&wall_first),
            "positive control failed: the harness could not step the wall clock backwards, \
             so the monotonic result is vacuous"
        );

        // The property under test: the same perturbation leaves monotonic
        // readings non-decreasing.
        assert!(nanoseconds(&monotonic_second) >= nanoseconds(&monotonic_first));

        // D1 at the type layer: the two clocks do not merely differ in value,
        // they reify to DIFFERENT constructors, so a wall reading cannot be
        // substituted where a monotonic one is required.
        assert_eq!(constructor(&wall_first), clock.mkinstant_id);
        assert_eq!(
            constructor(&monotonic_first),
            clock.mk_monotonic_instant_id
        );
        assert_ne!(constructor(&wall_first), constructor(&monotonic_first));
```

**The last three lines are NOT deferred.** The constructor distinction survives
in place, re-expressed without dispatching (§4 point 8). Only the two `assert!`s
above them go.

### 9b. `eval.rs:9061-9068`

From `ac3_the_deadline_a_caller_passes_is_the_deadline_honoured`
(`crates/ken-interp/src/eval.rs:9039`):

```rust
        assert_eq!(
            sleep_with(4_242),
            vec![ClockTrace::SleepUntil {
                deadline: BigInt::from(4_242),
            }]
        );
        // Discriminator: the observation tracks the argument.
        assert_ne!(sleep_with(4_242), sleep_with(9_999));
```

**Both lines are deferred.** The `assert_ne!` is the half that matters: it is
what distinguishes a host that honours the deadline from one that ignores it,
and `ken-host`'s recorder discards the deadline, so reproducing it there is
exactly the backend extension the promotion node owes.
