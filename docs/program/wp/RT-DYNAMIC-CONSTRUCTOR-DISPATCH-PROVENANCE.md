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

## Conditional D1 design boundary (do not pre-authorize)

- If the discriminator is an alien function-local handle: the repair is
  structural. A cloneable `Lowered::DynamicConstructor` may not carry a bare
  `cranelift_codegen::ir::Value` across emitted-function ownership. Either
  re-materialize the discriminator from an owner-independent semantic/source
  recipe in the current `FunctionBuilder`, or emit/consume the dynamic value
  entirely within the defining function. Merely attaching an owner label while
  still consuming the alien `Value` is a detector, not a solution. The required
  negative control constructs two functions whose same numeric `Value(u32)` index
  denotes different scalars; the path must refuse before CLIF emission or
  re-materialize correctly — never verify and silently consume the coincidental
  current-DFG value.
- If P0–P2 instead prove a wrong constant or CFG wiring: fix the single
  authority/edge actually shown by the dumped CLIF. No alternate
  dynamic-constructor representation, no ExitCode conversion, and no new magic
  status are allowed.

## Acceptance criteria

- AC-0 (probe artifact, not a count). D0-P0 produces the dumped exact function
  (name/owner/`vX`/immediate/`icmp`/`brif`/selected/next/merge/residual) for the
  function carrying the causal residual marker, under Cranelift verification. An
  inventory or origin-occurrence count does not satisfy this AC.
- AC-1 (single owner family selected). D0-P1's observed final status names exactly
  one owner family (`81..84` xor `91..94`), reproduced in one build. The claimed
  same-site `1`/tag-1 relationship is either confirmed at that pinned function or
  refuted (operand/constant/owner mismatch identified).
- AC-2 (class selected before any repair). The WP returns exactly one selected
  class from {alien SSA handle, wrong constant/site pairing, wrong successor/body
  exit, owner/provenance mispairing, backend defect}. No production mechanism is
  committed until this holds; a repair diff without the selecting probe result is
  out of scope.
- AC-3 (conditional repair matches the proven layer). If a D1 repair is included,
  it repairs ONLY the proven layer, adds no alternate dynamic-constructor
  representation / no ExitCode conversion / no new sentinel, and — for the alien
  handle case — carries the two-function same-index negative control that fails
  closed (refuse before emission or re-materialize), never silently consuming the
  coincidental value. Zero `trusted_base()` delta.
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

Touches `crates/ken-runtime` (`cranelift_backend/lowering/aggregates.rs`, and
`calls.rs`/`effects.rs` only if a proven D1 requires it) and `crates/ken-cli/tests`.
No overlap with lane 2 (language/elaborator) or lane 3 (foundation catalog
packages). Runtime ring exclusive.

## Capability tier

T1. Size M — one focused increment: the owner-bound D0 probe ladder to a
selected class, then at most the single proven-layer repair. Sized to reach a
class selection (or a genuine hard stop) within about an hour; a
class-selected-but-D1-needs-its-own-cut outcome is a good stop, not a miss.
