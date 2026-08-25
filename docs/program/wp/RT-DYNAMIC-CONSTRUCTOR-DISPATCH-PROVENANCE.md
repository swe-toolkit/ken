# WP frame — RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE (M3 successor, recut)

> Replaces [[RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT]], which the Architect
> falsified as a product object after the hard-stop #3 research advisory
> (ruling evt_1vhmndq7fscd1, thr_305pn5gzx37h). The three consecutive hard stops
> shared one predicate — **a downstream semantic classification used as upstream
> producer/provenance authority** — and the ExitCode framing (objective, sites,
> controls, diagnostic) is built on exactly that error, so this is a replacement
> WP, not an in-place amendment. Owning team: runtime. Size M. Capability tier:
> T1 (a provenance/SSA-ownership investigation whose conditional repair is a
> soundness-bearing structural design boundary; the deliverable is a
> probe-selected class, not a diff). The new dynamic-provenance chain starts at
> hard-stop count zero — it is a different design question.

> # AMENDED 2026-08-25 (Architect hard-stop #1, evt_6h546ckyzsgtf, thr_1b16f1grspdq8)
>
> D0 completed and selected a class the frame's five-class closure did not name —
> a SIXTH class: an inactive `HostResult` template is eagerly materialized before
> the runtime sum choice. Proven at exact WIP `59dd27f18e...` (base 665059cfd,
> two probe-only files, +170/-5) and CLIF `/tmp/rt-dynamic-provenance-family-80.clif`
> (sha256 779005d6...): exact function `u2:48`/`funcid48`, owner `Predeclared(8)`,
> origin 34, occ 485, `ResourceError::Closed`, discriminator `v197 =
> Result(inst339,0) = select(v34 == reply-error-tag 3, 0, v35+1)`; tag-1 compare
> `v562 = icmp_imm eq v197, 1` with conventional blocks; the fresh-1/unchanged-RHS
> substitution reads true and unchanged-v197/fresh-1 reads false (final status 85)
> — a lawful current-function operand OUTSIDE the `ResourceError` tag set. Cause:
> `aggregates.rs::emit_carrier_transfer` calls `emit_carrier_transfer(ok)` then
> `emit_carrier_transfer(error)` BEFORE allocating/storing the `HostResult`
> discriminant, so a successful `BufferAllocate` runs the unselected error
> dispatcher over success-detail resource-token bits and reaches its lawful
> malformed residual. It is NOT alien SSA / wrong constant / wrong successor /
> owner mispairing / backend defect. The Architect AUTHORIZED D1 in place
> (branch-before-transfer, one active payload). Runtime is HELD until this
> amendment releases. New-chain hard-stop count is 1. Durable inventory:
> `architect/rt-dynamic-inventory @ c142aaa74`. The concrete D1 mechanism +
> controls REPLACE the "Conditional D1 design boundary" section below.

## Objective

Bind px8ta's causal residual to ONE actual generated function, owner, SSA
discriminator, compare instruction, and taken successor — then repair only the
proven layer. No production mechanism is authorized before the probe ladder
selects a class.

The causal residual is the direct `return_(-3)` at the bottom of
`emit_carrier_dynamic_constructor`'s alternative chain: the px8ta path carries
`Lowered::DynamicConstructor` at `StaticOriginId(34)`, the runtime discriminator
is reported as `1`, the emitted alternative list contains tag `1`
(`ResourceError::Closed`), yet the equality chain reaches its residual instead of
selecting that alternative. Mutating only the residual scalar `-3 -> 73` makes
the process status exactly `73`, forwarded unchanged by
`call_declared_unit_target` — proving the residual is causal and the status
bypasses every named exit consumer, the result slot, and carrier decode. The
process reporter's "malformed ExitCode::Failure payload" is a sentinel alias
(tracked separately; see [[RT-UNIT-FAILURE-STATUS-PROVENANCE]]).

## Fixed inputs (Architect ruling evt_1vhmndq7fscd1, measured at checkpoint 34ab178ac)

- Causal site:
  `crates/ken-runtime/src/cranelift_backend/lowering/aggregates.rs::emit_carrier_dynamic_constructor`,
  its residual direct return of `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS` (`-3`).
- Source CFG is conventional (`aggregates.rs:3021-3106`):
  `icmp_imm(Equal, dynamic.discriminator, alternative.tag)`,
  `brif(matches, selected, next)`, selected jumps to merge, next continues.
  Cranelift has no lawful semantics where one pinned I64 `1 == 1` takes the false
  edge (research evt_5yxw7qypv4w4q: `brif` first successor is true, blocks carry
  explicit terminators, the verifier reconstructs CFG/dominance).
- The claimed contradiction is NOT yet proven at the granularity of one emitted
  function. `StaticOriginId(34)` is plan-local, not a unique emitted-function
  coordinate: markers 100 and 106 are duplicate `ResourceError` sites with
  byte-identical inventories, and the high/low discriminator observations came
  from separate builds. A compile-time inventory or origin count is not evidence
  for this object.
- Open classes (exactly one to be selected by D0): (a) the branch compares a
  different SSA value than the instrumented discriminator; (b) the emitted
  compare constant differs from the printed tag inventory (wrong site/constant
  pairing); (c) `icmp` is correct but block/branch construction routes the true
  edge wrong, or the selected body exits early; (d) discriminator and alternative
  chain belong to different constructor/provenance owners though their numeric
  tags coincide; (e) backend defect.
- The plausible class per prior art (research evt_5yxw7qypv4w4q) is value-handle
  provenance, not integer equality: Cranelift `Value` is a bare `Value(u32)`
  index into one function's DFG with no owner in its Rust type
  (`cranelift-codegen-0.113.1/src/ir/entities.rs:71`). A `Lowered::DynamicConstructor`
  that carries a bare `Value` across emitted-function ownership can alias a valid
  same-index value denoting a different scalar; verification cannot catch a
  coincidentally-valid wrong-owner value. Investigate, do not presume — D0 selects.

## Anchor

Keep `34ab178acc65b4f6d165e1b2d40f5809d1c475d2` on
`wp/RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT` READ-ONLY as the load-bearing probe
checkpoint until the same causal residual is reproduced under the owner-bound
probes. Do NOT use its consumer-refactor production delta as the successor base
or candidate — that refactor is the falsified ExitCode object and is not shipped.
Base the successor branch on current `main`.

## Deliverables

- **D0-P0 — one emitted-function coordinate, one build.** At each candidate
  `emit_carrier_dynamic_constructor` call, record together: `builder.func.name`,
  `self.defining_function_id`, `self.defining_emission_owner`, `StaticOriginId`,
  the alternative occurrence + `ConstructorIdentity`, the DFG `value_def` and type
  of `dynamic.discriminator`, and the tag-1 `icmp` instruction/value and its
  selected/next blocks. Assign the duplicate `ResourceError` emissions distinct
  owner-keyed probe families. After `FunctionBuilder::finalize`, run Cranelift
  verification and dump ONLY the exact function containing the causal residual
  marker. The artifact must show the actual `vX`, immediate, `icmp`, `brif`,
  selected block, next block, merge, and residual. A compile-time inventory or
  origin count is not evidence for this AC.
- **D0-P1 — operands and comparison, without branch construction.** In that same
  build and at that exact tag-1 instruction, materialize RHS and compute
  `lhs_is_1 = discriminator == 1`, `rhs_is_1 = rhs == 1`,
  `original = discriminator == alternative.tag`. Use nested `select`s to return
  `81/82/83/84` for `(both-one + original-true)/(both-one + original-false)/
  (lhs-not-one)/(rhs-not-one)`, and `91/92/93/94` for the duplicate owner. The
  final observed status must name exactly one owner family. This subsumes the
  separate high/low logs.
- **D0 conditional ladder — run ONLY the arm P1 selects:**
  - P1 true (81): replace only that `brif` with fresh terminal true/false blocks
    returning distinct owner-keyed markers. If true wins, move a marker first to
    selected-block entry, then immediately before `jump(merge)`, to distinguish an
    early return inside allocation/field transfer from a different residual site.
  - operand/constant problem (83/84): compare a fresh SSA `iconst 1` independently
    against the unchanged RHS and the unchanged LHS. The one-sided substitution
    that restores selection distinguishes an alien discriminator (wrong SSA input)
    from a wrong emitted constant.
  - numeric/CFG correct but the baseline residual persists: pair provenance.
    Record the discriminator's defining function/owner/source and the
    alternative's planned occurrence/family; never use equal numeric tags as an
    owner check. Mutate only one suspected producer/tag from `1` to `17`: both
    changes together must preserve selection, either alone must reach the residual.
  - `icmp_imm` alone contradicts an SSA-constant comparison in the same verified
    function (82): preserve the exact CLIF and isolate a Cranelift reproducer. Do
    NOT redesign Ken around a backend defect.
- **D0 stop.** Return the exact dumped function, the one P1/P2 result, and the
  adjacent producer/consumer identities. Do NOT infer D1 from a marker count and
  do NOT green px8ta by changing the residual status.

## D1 — authorized mechanism (Architect ruling evt_6h546ckyzsgtf)

Choose runtime branch-before-transfer with ONE active payload. `Lowered::HostResult`
remains a compile-time choice node holding both templates plus one runtime
`success` value; its boundary representation is a runtime sum: **one discriminant
and exactly one active payload.** Structural preflight still examines both
templates (the compiled function must be admissible for either runtime outcome) —
preflight is not materialization. Runtime transfer must execute exactly ONE
recursive producer. Do NOT invent an inert unselected payload.

In the `Lowered::HostResult` arm of `aggregates.rs::emit_carrier_transfer`:

1. Normalize `success` once to the existing I64 truth word and derive
   `took_ok = success_i64 != 0` from that same SSA value. The branch predicate
   and the stored discriminant may not have separate authorities.
2. Emit `ok_block`, `error_block`, and a merge with one I64 block parameter.
3. `brif(took_ok, ok_block, error_block)` BEFORE either recursive
   `emit_carrier_transfer` call.
4. The ok block transfers only `ok`; the error block transfers only `error`; each
   jumps to the merge with its one `BoundaryWord`.
5. At the merge, allocate the existing `(InvocationHostResult, HostResult)` node
   with field count ONE, store `success_i64`, and store the selected word at
   field 0.

Do NOT use an SSA `select` over two already-materialized payload words (under CBV
that preserves the defect). Do NOT allocate an inert `ResourceError`, duplicate
the selected word into two semantically named fields, or add a new tag/version. A
sum has no semantic inactive payload; per-effect dummy construction invents values
from fields outside their domain and leaves a wrong-but-projectable child in the
generic field interface.

Reconcile the canonical one-child shape in the SAME D1:

- `boundary_value_clif.rs::define_host_payload` reads field 0 after exact
  `HostResult` class + arity-one validation; it no longer derives index 0/1 from
  `success`.
- The shared HostResult guard used by `host_success`/`host_payload` must REFUSE a
  HostResult node whose field count is not exactly one, rather than exposing a
  discriminant from a malformed physical shape.
- Rust-side `materialize_host_result`, the emitted HostResult producer fixture,
  comments, and tests must describe/store one selected payload. REMOVE the old
  "both arms are materialized" contract, do not retain a compatibility form.
- Keep carried consumers unchanged at their semantic interface: read
  `host_success`, read `host_payload`, then branch to `Ok`/`Err` — they already
  require only the active word.

Repair only the proven layer: no alternate dynamic-constructor representation, no
ExitCode conversion, no new magic status, zero `trusted_base()` delta.

## Acceptance criteria

- AC-0 (probe artifact, not a count). D0-P0 produces the dumped exact function
  (name/owner/`vX`/immediate/`icmp`/`brif`/selected/next/merge/residual) for the
  function carrying the causal residual marker, under Cranelift verification. An
  inventory or origin-occurrence count does not satisfy this AC.
- AC-1 (single owner family selected). D0-P1's observed final status names exactly
  one owner family (`81..84` xor `91..94`), reproduced in one build. The claimed
  same-site `1`/tag-1 relationship is either confirmed at that pinned function or
  refuted (operand/constant/owner mismatch identified).
- AC-2 (class selected — DONE). D0 selected the SIXTH class: an inactive
  `HostResult` template eagerly materialized before the runtime sum choice
  (Architect evt_6h546ckyzsgtf), refuting the five-class closure. Recorded in the
  top amendment banner; this is the proven layer D1 repairs.
- AC-3 (D1 controls — the branch-before-transfer proof). The candidate carries all
  of:
  1. a runtime success/error pair with distinct valid payloads: the same consumer
     interface selects the correct active payload in both directions and the
     physical field count is exactly one;
  2. a hostile-inactive pair reaching the production `emit_carrier_transfer` arm —
     success with valid ok + out-of-family dynamic error succeeds; failure with
     valid error + out-of-family dynamic ok succeeds — such that either eager
     transfer or a reversed branch reddens;
  3. the paired selected-hostile cases still reach
     `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS` (D1 did not suppress the exact
     dispatcher or turn malformed selected values into success);
  4. HostResult nodes of arity zero and two are REFUSED as malformed shape (the
     canonical helper cannot silently accept the retired two-child layout);
  5. re-run px8ta HALF B: the acceptance claim is ONLY that this exact eager
     inactive-error residual disappears — report `ConsoleIsTerminal` if reached,
     else name the first new causal obstruction; do NOT promise end-to-end green
     and do NOT change `-3` reporting;
  6. every D0 registry, dump, marker, `eprintln!`, and substitution is removed
     from the final candidate (the CLIF/WIP remains evidence, not production).
  Mutation-prove at least the branch-before-transfer site in BOTH directions:
  restoring eager ok+error transfer must red the success hostile-inactive witness;
  reversing `took_ok` must red the success/error pair. No text pin substitutes for
  either runtime property. Zero `trusted_base()` delta.
- AC-4 (residual honesty untouched here). This WP does NOT re-classify or renumber
  the `-3` reporter alias; that is [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] and must
  not be folded in. Changing the residual status to green px8ta is explicitly
  disallowed.
- AC-NO-REGRESSION. Whole-suite green in CI; the ExitCode `Success`=0 path and the
  entry-trap (`-2`) / borrowed-input (`-1`) behaviors stay green; local targeted
  `-p ken-runtime` / `--test` only.

## Reviewers

Architect (component fit: the probe must bind to generated-function/owner
identity rather than occurrence counts; the class selection must be justified by
the dumped CLIF; any conditional D1 must match the proven layer and carry the
two-function negative control) + runtime-qa (the owner-keyed probe families are
discriminating and the acceptance turns on the dumped artifact, not a count).
Research advisory evt_5yxw7qypv4w4q is the standing reference. Adversary advisory,
non-gating.

## Contention check

Touches `crates/ken-runtime`: `cranelift_backend/lowering/aggregates.rs`
(`emit_carrier_transfer`), `boundary_value.rs` and `boundary_value_clif.rs`
(`define_host_payload` + the shared `host_success`/`host_payload` guard,
`materialize_host_result`), and focused tests (`crates/ken-cli/tests` + runtime
boundary tests). No overlap with lane 2 (language/elaborator) or lane 3
(foundation catalog packages). Runtime ring exclusive.

## Capability tier

T1. Size M — one focused increment: the owner-bound D0 probe ladder to a
selected class, then at most the single proven-layer repair. Sized to reach a
class selection (or a genuine hard stop) within about an hour; a
class-selected-but-D1-needs-its-own-cut outcome is a good stop, not a miss.
