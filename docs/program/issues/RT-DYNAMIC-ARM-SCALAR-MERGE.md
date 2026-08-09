---
id: RT-DYNAMIC-ARM-SCALAR-MERGE
title: "A carried Match arm carrying a nested-IH result cannot satisfy merge_scalar_operand -- measure what the arm actually produces before bounding the repair"
status: ready
owner: runtime
size: TBD
gate: none
depends_on: []
blocks: []
github: null
origin: Measured by KERNEL-NESTED-IND D5 at WIP 51c482a5 (evt_3evnpax25tckf, 2026-08-09). Kernel reached the native boundary after interpreter Nat-3 and provenance-gated erasure both passed, and stopped without Runtime edits exactly as the durable D5 ruling at main 46c12adb requires. Steward-filed (agents cannot create tracked work per COORDINATION §2). Steward owns the frame and AC/control placement.
---

> # `D0` AND `D1a` ARE DONE. START AT `D1b-role`.
>
> ⚠ **`D1a` closed 2026-08-09 (`evt_3g4n00s7ftd9q`) with a verdict my own
> taxonomy could not express**, and Architect ruling `evt_2wm35zk98p9nr` recut
> the repair. **`D1b-cov` and `D1b-rep` are both WITHDRAWN; the deliverable is
> `D1b-role`.** Read that section, not the fold-coverage framing that preceded
> it — and note that `D0`'s inductive-cascade mechanism story is retracted.
>
> ⚠ **`D1b-id` is ALSO superseded, recut 2026-08-09 as `D1b-role` on Architect
> ruling `evt_23eb7gp8sz4an`.** It is not a transport: the authority does not
> exist on the value path and must be produced. Two conclusions from the
> falsifying measurement are themselves false — see the superseded block.
>
> `D0` closed 2026-08-09 (`evt_1ct16entsqn94`) and answered all four questions
> with `file:line` evidence. **It also measured two of this frame's own fixed
> inputs FALSE and reported them instead of building around them**, which is
> what the perishable-anchor instruction below asks for. Both are corrected in
> place.
>
> **The repair is bounded now, and by a different question than this frame
> originally asked.** `AC-2` posed *scalar-representable vs structurally wider*;
> the measured answer is that it **is** representable, and the thing that
> actually bounds the fix is **where the Peano fold's induction broke**. `D1` is
> cut against that, with both outcomes pre-ruled.
>
> `size:` stays `TBD` until `D1a` reports, and that is still honest rather than
> lazy — one of `D1`'s two branches is a repair and the other is an Architect
> escalation, and they are not the same size.

Treat every anchor below as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around it.

## What it is

`KERNEL-NESTED-IND` `D5` made nested-inductive elimination work through the
elaborator, the interpreter, and checked-artifact erasure. It then reached
native lowering and refused:

```text
NativeLoweringOrExecution: a carried Match arm
  -- dynamic arms must produce scalar Int or Bool values
```

**This is a Runtime-owned capability gap, not a Kernel defect.** Kernel may not
edit `crates/ken-runtime`; the planner/lowering invariant is Runtime's, and a
Steward authorization to the contrary was overruled once already.

## Fixed inputs, measured at `main` `46c12adb`

| fact | value |
|---|---|
| refusal site | `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:15898-15900`, the `_ =>` arm |
| enclosing function | `merge_scalar_operand`, `:15749` |
| `ScalarMergeKind` | `:14138` — `Int`, `Bool`, `StructuralNat`, `ExitCode`, `RecursiveBackedge` |
| producing WIP | `51c482a5` on `wp/KERNEL-NESTED-IND-D5`, six paths, `crates/ken-runtime` diff **empty** |

⛔ **TWO FIXED INPUTS ABOVE AND BELOW WERE WRONG. `D0` measured them false and
they are corrected here.** The originals are struck through rather than deleted,
because a reader who saw the earlier version needs to recognise what changed.

**Correction 1 — the admitted set omitted three live arms.** It listed only
`StructuralNat`, nullary bool, `ProcessExitStatus`, and the checked-root-exit
path. The general match **also** admits:

| arm | site |
|---|---|
| `Lowered::Int` | `:15846` |
| `Lowered::Bool` | `:15853` |
| `Lowered::RecursiveBackedge` | `:15839` |

**Correction 2 — `D0` question 3's premise was false.** It asserted
`RecursiveBackedge` is *"a declared `ScalarMergeKind` variant that no arm here
produces."* **Two arms produce it** — `:15804` in the `required_kind ==
ExitCode` branch and `:15839` in the general match — and there is a third,
explicit `RecursiveBackedge` **refusal** on the carried path at `:15785`. ⚠ The
reachability *question* was still real and `D0` answered it; only its premise
was wrong.

So the corrected admitted set is: `StructuralNat`; nullary `bool_true`/
`bool_false` constructors; `ProcessExitStatus`; `Int`; `Bool`;
`RecursiveBackedge`; and any `lowered` under `checked_root_exit_representation`.

## `D0` RESULT — measured, all four questions answered

**Anchor:** `lowering/mod.rs` is the same blob `f9601b12` at `46c12adb`,
`c34317f3`, `51c482a5`, and the measuring tree, so every line number above is
interchangeable across all four.

**Q1 — the variant at the seat.** `Lowered::Constructor`, `Nat::Suc`, arity 1,
whose single argument is itself a `Constructor` — **an unfolded Peano chain**,
not a `StructuralNat`. Read from an instrument inside the `_ =>` arm, which is
what `AC-1` required.

**Q2 — scalar-representable, direction stated.** ⭐ **It IS representable and is
NOT structurally wider than the pair.** `StructuralNatV1` is a single `i64`
(`mod.rs:10143`), and the backend already folds Peano chains into it at
`mod.rs:17257-17267` and `core.rs:13940-13951`.

> ⭐⭐ **THE DISCRIMINANT IS NOT "SCALAR VS WIDER". `AC-2` ASKED THE WRONG
> QUESTION AND `D0` ANSWERED THE RIGHT ONE.**
>
> **Both folds are inductive on their own output.** `Suc` folds only if its
> predecessor **already** folded:
>
> ```rust
> if constructor == self.process_symbols.nat_suc {
>     if let [Lowered::StructuralNat(predecessor)] = lowered_args.as_slice() { ... }
> }
> // otherwise falls through to Lowered::Constructor
> ```
>
> ⇒ **A single unfolded link makes every enclosing `Suc` fall through too** —
> exactly the measured shape (`Suc` with `arg_kinds=[Constructor]`). The
> question that bounds the repair is therefore **where the induction broke**,
> not how wide the value is.

**Q3 — reachability, with a positive control that fired.** Over `-p ken-runtime
--lib`, 843 tests: 775 seat entries, 776 general-match arrivals, **0 refusals**;
carried path 0, `ExitCode` branch 0. Arrivals were `Constructor` 548, `Int` 110,
`ProcessExitStatus` 80, `Bool` 38. **`RecursiveBackedge` is not reached at this
seat**, zero at both producing arms.

⭐ **The positive control, and the failed first attempt that makes it
trustworthy.** Widening `:15839` to also admit `StructuralNat` printed **zero**
— because `StructuralNat` never reaches that match at all, so a zero would have
looked exactly like the wanted answer. That failure forced the branch
partition, which supplied a witness **chosen from the measured arrivals rather
than guessed**: widening with `ProcessExitStatus` fired **50 times**.

⚠ **Honest split, carried forward:** the carried path (`:15785`) and the
`ExitCode` branch (`:15804`) each take **0 arrivals**, so for those two the
claim is *"the branch is unreached"*, ⛔ **not** *"the arm is unreached"* — no
positive control is available for them from this population.

**Q4 — `D5` is the first arrival of this shape.** Zero refusals across 775
entries. Sharply: `Lowered::Constructor` reaches this seat **548 times and never
refuses**, because those are nullary `bool_true`/`bool_false` caught at
`:15866`. The `D5` value is a `Constructor` that is **neither nullary nor
bool**. ⇒ The variant is common here; **the shape is new.**

⚠ **Domain bound, stated by the measurement and not to be dropped:** this is the
`ken-runtime --lib` population plus the one `D5` cross-crate case. `ken-cli`,
`ken-verify`, and elaborator entries to this seat were **not** censused, so
*"no other caller is already refusing"* holds over the in-crate population, not
over every compilation entry.

## `D0` — measure the produced value. This is the whole first slice.

**Do not repair anything in `D0`.** Report:

1. The exact `Lowered` variant the refused arm carries, at the refusal, for the
   `LiftRose`/`Bag` Nat-3 case that `KERNEL-NESTED-IND` `D5` drives.
2. Whether that variant is **scalar-representable at all** in
   `NativeScalarPairV1`, or whether it is structurally wider than the pair.
   **These are materially different futures and they bound different repairs**
   — one is a missing arm, the other is a representation question.
3. Whether `RecursiveBackedge` is reachable at this seat. It is a declared
   `ScalarMergeKind` variant that **no arm here produces**, and
   `RT-MATCH-RECURSOR-CONSUMERS` records it as protocol-only and untouched.
   Say which it is; do not assume from the enum.
4. Whether any **other** caller of `merge_scalar_operand` already passes a
   value of the same shape and is therefore already refusing in production, or
   whether `D5` is the first arrival.

**`D0` closes when those four are answered with `file:line` evidence.** It does
not authorize a repair and it does not size one.

## `D1` — FRAMED 2026-08-09 against `D0`'s fold-induction discriminant

> **Both outcomes are ruled below, so `D1a` does not come back to me.** One
> branch proceeds straight to the repair; the other is a genuine stop. ⛔ Do not
> treat the stop as a failure — it is a different and larger design question,
> and reaching it is a real result.

### `D1a` — measure WHERE the fold's induction broke. One instrument.

**This is the measurement `D0` named as one instrument away and deliberately did
not take.** Walk the `Nat::Suc` chain the `D5` case presents and find the
**innermost link that failed to fold**, then report which of exactly two things
it is:

1. **Coverable** — the base `Zero`, or an intermediate link, failed to fold for
   a reason the existing fold could handle: an unmatched constructor spelling, a
   missing `Zero` base case, an argument arriving in a shape the `if let`
   pattern does not destructure.
2. **Genuinely dynamic** — the link's predecessor is not a compile-time-known
   constructor at all, so no inductive fold can reach it in principle.

⛔ **Report the innermost failing link, not the outermost.** Every enclosing
`Suc` falls through once one link does, so the outermost failure is guaranteed
and says nothing. ⚠ This is the same trap as a short-circuiting probe measuring
the first cause rather than the set.

> ## ⛔⛔ `D1b-cov` AND `D1b-rep` ARE BOTH WITHDRAWN. THE REPAIR IS `D1b-id`.
>
> **`D1a` measured a THIRD case my taxonomy did not contain, and Architect
> ruling `evt_2wm35zk98p9nr` named it: an identity-authority TRANSPORT defect.**
> Not missing fold coverage, and not a representation problem.
>
> ⛔ **My `D1b-cov` prohibition forbade the repair.** It banned Elaborator and
> compiler-driver edits — and the compiler driver is **half the fix**, because
> it is the producer of the checked constructor identities. A frame whose banned
> scope excludes the only route to its own AC is a defect in the frame, and this
> one was mine.
>
> ⚠ **`D0`'s inductive-propagation story is RETRACTED, by its own author.**
> `D0` reported that `Suc` folds only if its predecessor folded, so one broken
> link cascades. `D1a` measured that **the fold never engages on ANY link** —
> so cascade was never what was happening. Anything upstream of this node that
> repeats the cascade story, including this frame's earlier text and my
> `b92b3f3f` briefing, is wrong on the mechanism.

### What `D1a` measured

Innermost failing link is **depth 1, the base
`ctor:nested_inductive_pkg::Nat::Zero`**; the chain is `Suc(Zero)` and every
link is compile-time known. The fold compares against
`ctor:prelude::Nat::{Zero,Suc}`.

⭐ **A walker trap the implementer hit and corrected**, worth keeping: the first
walker descended on `*constructor == nat_suc` — **the very predicate under
test** — so it stopped at depth 0 and would have reported the *outermost* link
as innermost, which `AC-6` explicitly rejects. Descending on structure reached
the real base.

### Why this is NOT a user type, and why structural recognition is unlawful

**The measured `nested_inductive_pkg::Nat` is the PRELUDE `Nat`.** The `D5`
source declares `Bag` and `LiftRose` and refers to the prelude `Nat` already in
the live elaboration environment; `stable_symbols_for_env` renders every live
`GlobalId` through the package's stable-symbol table, so prelude `Nat` renders
package-qualified. No user type is being folded and no constructor identity is
being discarded.

⛔ **The structural Peano criterion is NOT lawful at this seat** (Architect).
Constructor *shape* is not checked constructor *identity*: an unrelated user
`Data` can be nullary-plus-unary-recursive and therefore Peano-isomorphic
without being Ken `Nat`. Folding on shape would erase an observable identity —
the blanket widening this node forbids.

### The actual defect

The `D5` differential helper erases a generic `CompilerDriverOutput` to
`RuntimeProgram` and calls the value runner, which reaches
`compile_expr_into_module` with **`process_symbols=None`**. Runtime substitutes
`NativeProcessSymbols::legacy_prelude()`, so the exact package constructors are
compared against `ctor:prelude::Nat::{Zero,Suc}` and miss. **The producer had
the right identities; the consumer never received them.**

> ### `D1b-id` IS SUPERSEDED BY `D1b-role`
>
> **Recut 2026-08-09 on Architect ruling `evt_23eb7gp8sz4an`.**
>
> **The `D1b-id` frame below was defective and the defect was mine:** its
> producer, `compiler_driver.rs:3336-3337`, runs only in the process-starter
> transaction, so the generic value package never materializes a role table at
> all. Runtime measured that (0 instrumented producer lines with the instrument
> confirmed present and the `D5` refusal confirmed firing) and stopped instead of
> building an inert transport. That was the right call.
>
> **But two conclusions drawn from those measurements are FALSE, including one I
> propagated when I withdrew controls #1 and #3 as unsatisfiable. They are not
> unsatisfiable.** Correcting both, because a recut built on either is wrong:
>
> | conclusion | status | what is actually true |
> |---|---|---|
> | *"zero `Data` rows ⇒ the package `Nat` identity is absent from `RuntimeProgram`"* | **FALSE** | erasure keeps `declarations` minimal but independently copies every `semantic.symbol` into `erased_core.symbols` (`erasure.rs:195-205`) and every `semantic.data_metadata` entry into `erased_core.metadata.checked_core.data_metadata` (`:5918-5952`), which `ir.rs:43-50` calls authoritative after proof erasure. The probe counted **executable declarations only** and inspected neither authoritative metadata lane. The correct statement is *"`Nat` is not an executable target declaration"* |
> | *"resolving the prelude `GlobalId` yields a legacy-prelude symbol, byte-equal to `legacy_prelude()`"* | **FALSE** | `emit_package_from_env` calls `stable_symbols_for_env(&manifest.package_name, ...)` at `compiler_driver.rs:2960`. That table maps every non-primitive declaration id through `declaration_symbol(package_name, name)` (`:3492-3496`) and constructors under the package-qualified parent (`:3499-3509`). Applied to **that** table, `resolve_id(prelude_env.zero_id/suc_id)` yields `ctor:nested_inductive_pkg::Nat::{Zero,Suc}` |
>
> ⇒ **The ids' prelude origin and their current artifact stable spelling are
> different axes.** Controls #1 and #3 become satisfiable the moment the producer
> is placed on the real generic package-emission path — which is the recut.
>
> **A conditional probe is what made both look settled.** The first `Data`-only
> probe could not separate *"no `Data` declarations"* from *"not on this path"*;
> the same conditional-probe mistake, in another guise, made the first `D1a`
> walker report the outermost link. Make probes unconditional.

### `D1b-role` — materialize a COMPLETE checked-runtime role record, carry it
### through erasure, and REQUIRE it at package-backed native compilation

**Not a transport of an existing authority — the authority does not exist on this
path and must be produced.** ⛔ **The scalar merge is still NOT the repair
site**: no new `merge_scalar_operand` admission, no `ScalarMergeKind` change.

**The executable-erasure boundary is preserved.** ⛔ Do **not** add `Nat`,
`Bool`, or any prelude type as `RuntimeDeclarationKind::Data` entries to teach
native lowering their roles. That widens the executable closure to solve a
**metadata** problem. This deliverable is a checked-artifact metadata extension
plus Runtime consumption — not a change to proof erasure, and not a change to
runtime value representation.

1. **Produce.** One versioned, hash-covered checked semantic record,
   conceptually `CheckedRuntimeSymbolsV1`, built **inside `emit_package_from_env`
   after the exact `stable_symbols_for_env` call** (`compiler_driver.rs:2960`)
   and **before the live `ElabEnv` is lost**. Derived from prelude `GlobalId`s
   through that exact table. ⛔ No source-name, suffix, package-name
   reconstruction, or structural Peano inference.
2. **Complete, not Nat-only.** The record must cover **every constructor role to
   which Runtime assigns special meaning**. The current complete population is
   the existing `NativeProcessSymbols` field set: Bool, Nat, Unit, Result/Option,
   process/list/product/exit, file/resource/progress and related constructors.
   ⛔ **A Nat-only sidecar reproduces this exact defect at the next special
   constructor** and does not discharge this deliverable.
3. **Store canonically.** In the checked package's semantic lane — the existing
   versioned `semantic.metadata` lane is lawful — so it participates in
   `core_semantic_hash` and survives serialized-package consumption. ⛔ A live
   `CompilerDriverOutput` sidecar is **not** lawful: the compiler's semantic
   input is `CheckedCorePackage`, not a retained `ElabEnv`.
4. **Decode and validate at erasure.** Into a typed Runtime field, preferably
   `RuntimeCheckedCoreMetadata.runtime_symbols: CheckedRuntimeSymbolsV1`.
   Validate every role symbol against `semantic.symbols`; constructor roles must
   also resolve **uniquely** through the existing `data_metadata`
   family/constructor entries with their recorded arity and recursive positions
   — for `Nat`, `Zero` nullary and `Suc` unary with the recorded recursive
   position. ⚠ These checks detect **stale or mismatched metadata**; they do not
   infer the `Nat` role from shape, and must not be written as if they could.
5. **Require at consumption.** Package-backed compilation requires the typed
   field. `compile_program_expr` passes its exact table to the lowerer. Missing,
   malformed, duplicate, or metadata-inconsistent authority **rejects before**
   `plan_static_transition_graph_with_symbols`.
6. **Remove the ambiguity structurally, not by discipline.** The inner package
   lowerer takes `&CheckedRuntimeSymbolsV1` (or `&NativeProcessSymbols`), **not**
   `Option<&...>`. `core.rs:1781-1783`'s `unwrap_or_else(legacy_prelude)` must
   not remain reachable from a `Some(program)` compile. Seed-only `compile_expr`
   **may** construct and pass `legacy_prelude()` **explicitly**, because its IR is
   deliberately minted in that namespace.
7. **Folds unchanged.** Both Nat folds stay **exact-identity and inductive**. ⛔
   No fold-code change, structural criterion, eager chain walk, whole-chain
   walker, declaration-shape recognizer, or name/suffix matching.
   `StructuralNat` remains the native representation of the checked `Nat`
   identified by that authority.

> ### SLICE `a` WIDENED IN PLACE 2026-08-09 — the immutable canonical role roster
>
> **Architect ruling `evt_6q4tvtenb1wps`, recorded here by the Steward so the
> enlarged diff is self-authorizing.** Decision `dec_7v589ezdeq321` rejected
> `aade3c2f` on one authority defect — **not** on the hash control, which stands.
>
> **The defect:** both producers select authority by **mutable source spelling**
> (`env.globals.get(name)` after package source elaboration). Mapping a
> name-selected id through `stable_symbols_for_env` does not cure that — the id
> is already the wrong one. Every Runtime constructor role must originate from an
> **immutable canonical prelude `GlobalId`** captured at prelude registration,
> before package source elaboration, and every stored symbol must be
> `exact_stable_table[canonical_role_global_id]`.
>
> ⛔ **Do NOT split this off as a preparatory `a0` WP.** The Architect ruled
> against it directly: the roster is the **only lawful implementation of item 1**,
> and `CheckedRuntimeSymbolsV1` embeds `CheckedHostSpineV1`, so produce/store/
> hash-cover cannot be an independently correct accepted partial while either
> producer still resolves by name. A split would draw an artificial boundary
> inside one authority producer and yield no separately usable contract. **One
> fresh slice-`a` candidate, one fresh QA/Decision lineage.**
>
> **Authorized scope widening** (this is what the carve-out above now grants):
> `prelude.rs`/`PreludeEnv` registration, **both** producer resolvers in
> `compiler_driver.rs`, the existing slice-`a` record/storage/hash paths, and
> focused tests. Prefer a **nested immutable roster type** inside `PreludeEnv`
> over expanding unrelated top-level fields. Use semantic fields or a **closed
> internal role enum** for collections — ⛔ do not replace the string lookup with
> a later string-keyed authority map, which reproduces the defect one layer up.
>
> **Completeness is every currently name-resolved entry of BOTH producers**, not
> the six former plan roles: host families, constructors, error/resource/progress
> roles, Bool/Unit, and the public operation identities. Existing private-operation
> ids remain valid roster members. Measured starting point, from the implementer's
> roster survey — this is the part that needed measuring and it is done:
>
> | roster state | roles |
> |---|---|
> | canonical id **exists** in `PreludeEnv` | `Nil` (`prelude.rs:130`), `Cons` (`:131`), `MkProd` (`:141`); and on the spine side `Some`, `Err`, `Ok`, `MkUnit` |
> | canonical id **MISSING**, must be captured at registration | `MkProcessInput`, `Success`, `Failure`; and on the spine side `True`/`False` and most resource/progress roles |
>
> **The committed discriminator must cover two properties SEPARATELY.**
> (1) **inventory completeness** — no name-resolved role remains in either
> producer; (2) **substitution resistance** — package declarations shadow
> representative constructor, family, and operation spellings while the emitted
> record still equals the exact canonical-id-to-stable-symbol projection,
> **including parent identities**. ⛔ **Bare-name containment is not evidence** —
> that is precisely the blindness that let the rejected candidate pass. If one
> fixture can lawfully shadow the full public roster, assert all entries;
> otherwise use a table-driven exact projection plus representative collisions
> per namespace/path class.
>
> **Retained unchanged:** the record-presence/version control and the
> semantic-hash mutation/removal pair, both already valid. **All partial-`a`
> negative boundaries still bind** — no decode/consumption, no executable `Data`,
> no fold/scalar work, no native admission, no `AC-K12` claim.
>
> ⚠ **`9d3273a8` is the blocked baseline only** (`aade3c2f` replayed onto
> `4a903d46`, content identical, object different). **No prior SHA-bound verdict
> transfers** — not QA's approval, not the Architect's earlier vote.

**Sizing: land this as up to three accepted partials, in this order.** Each is
independently reviewable and mergeable per the accepted-partial policy, and each
is roughly a one-hour turn; do not hold the whole chain for one PR.

| slice | scope | closes |
|---|---|---|
| `a` | produce + store + hash-cover the record (items 1-3) | control 1 |
| `b` | erasure decode + validation (item 4) | control 2 |
| `c` | require + consume + structural de-`Option` (items 5-7) | controls 3-6 |

### Required committed discrimination — all six

| # | control |
|---|---|
| 1 | on the **real generic `D5` package-emission path**, the produced Nat roles are exactly `nested_inductive_pkg::Nat::{Zero,Suc}` and **differ** from the explicit legacy pair. ⛔ The probe must fire **on that path** — an unconditional probe, for the reason recorded above |
| 2 | after erasure, the executable declaration set is **still exactly** `liftAdd` and `liftSize`, **while** `erased_core.checked_core.data_metadata` and the typed runtime-symbol record both carry the exact Nat family and pair. This pins metadata preservation **without** closure widening — both halves are required |
| 3 | **three separate** rejection controls: delete the semantic role record; corrupt its header; mutate **only** Nat `Zero` to the legacy symbol. Each must reject **before native semantic lowering**, through a **named** authority-validation lane. ⚠ Separately, a focused lowerer control with an **explicitly supplied** legacy table must leave the package-qualified chain as `Lowered::Constructor` — that proves the identity operand is **causal**, rather than resting only on preflight |
| 4 | `AC-10` re-run at the real `D5` seat: refusal **1 → 0** and the operand arrives as `StructuralNat`. ⛔ A green end-to-end result alone is insufficient |
| 5 | an unrelated **nullary-plus-unary-recursive** `Data` remains `Lowered::Constructor` and retains its constructor identity — the counterexample excluding structural widening |
| 6 | existing **explicit** legacy seed controls stay green, **and** a structural control proves no package-backed compile can reach an **implicit** legacy fallback. All **six** admitted merge shapes and the fail-closed catch-all preserved |

⛔ **No Kernel, interpreter, match-semantics, `ScalarMergeKind`, or `AC-K12`
claim follows from this ruling.** `AC-K12` **is** reachable on the current
architecture — the Architect states this — but this deliverable discharges only
the **first** native-lowering refusal. Verifier passage and interpreter/native
agreement remain separate gates.

### Acceptance for `D1`

| AC | criterion | control |
|---|---|---|
| `AC-6` | `D1a` names the **innermost** failing link with `file:line` and the constructor at that link | a report naming only the outermost `Suc` does not discharge it — that failure is entailed by any inner one |
| `AC-7` | `D1a`'s verdict is **coverable** or **genuinely dynamic**, stated in a direction | if it genuinely depends, name the discriminant, as `AC-2` required and `D0` did |
| `AC-8` | Any `D1b-cov` repair leaves the **six** currently-admitted shapes byte-for-behaviour unchanged | the corrected admitted set above, **not** the four the original frame listed |
| `AC-9` | `D1b-cov` keeps the fold **inductive**, not eager | the existing `if let [Lowered::StructuralNat(pred)]` shape survives; a whole-chain walk fails this row |
| `AC-10` | A positive control proves the repaired fold **actually folds the `D5` chain** at the seat | ⚠ re-run `D0`'s seat instrument: the refusal count at `_ =>` for the `D5` case must go 1 → 0, **and** the arrival must be `StructuralNat`. A green `D5` test alone does not discharge this — it could pass by a different arm admitting the `Constructor` |

⛔ **`AC-5`'s fail-closed requirement is unchanged and now matters more.**
Widening the fold must not widen the `_ =>`. A value outside the admitted set
still refuses with a diagnostic naming it.

### What `D1` still does NOT discharge

`KERNEL-NESTED-IND` `AC-K12` needs native lowering **and** the Cranelift
verifier **and** interpreter/native agreement. `D1` addresses the first refusal
only. ⛔ Do not report `AC-K12` as discharged because the `D5` case stops
refusing here; the verifier and the differential are separate stages and may
surface their own gaps.

## Acceptance

| AC | criterion | control |
|---|---|---|
| `AC-1` | `D0` names the exact `Lowered` variant at the refusal | the variant is read **at the seat**, not inferred from the arm's source expression. A characterization taken upstream of `merge_scalar_operand` does not discharge this |
| `AC-2` | The scalar-representability question is answered **in a direction**, not hedged | state whether it fits `NativeScalarPairV1` or exceeds it, and why. "It depends" is not an answer; if it genuinely depends, name the discriminant |
| `AC-3` | The `RecursiveBackedge` reachability claim carries a witness **or** an explicit "not reachable here, and this is how I established that" | a negative check passes for any reason, so an unreached-variant claim needs a positive control showing the instrument would see it if it fired |
| `AC-4` | Any repair (`D1`, when framed) leaves the four currently-admitted shapes **byte-for-behaviour unchanged** | `StructuralNat`, nullary bool, `ProcessExitStatus`, and the checked-root-exit path each keep their existing arm and result |
| `AC-5` | Any repair keeps the `_ =>` **fail-closed** | widening the admitted set must not convert the catch-all into an accept. A value outside the new admitted set still refuses with a diagnostic that names it |

## Forbidden

- **Blanket relaxation of the scalar contract.** Widening `merge_scalar_operand`
  to accept arbitrary `Lowered` values is not the repair, whatever `D0` finds.
  Same reasoning as [[RT-CARRIER-BYTESPAN-OBSERVE]]: availability is per seat,
  never a blanket phase relaxation.
- **Folding this into [[RT-CARRIED-RESOURCE-SCALAR]].** That node's refusal is
  an effect-seat `ResourceScalar`-in-`CarriedWord` shape — a different need on
  different seats. Its own frame warns against exactly this
  same-shape-different-population fold, and it is `draft` with no frame.
- **Folding this into [[RT-TERMINAL-ALL-ELIM-AUTHORITY]].** Different seat:
  that node owns `lowering/core.rs:6178-6183`, the `ComputationalRecursorClosure`
  remainder arm. This is `lowering/mod.rs:15898`. Checked, not assumed.
- Editing `crates/ken-elaborator`, `crates/ken-kernel`, or `crates/ken-interp`
  to make the arm produce something the existing seat already accepts. That
  moves a Runtime gap into Kernel's landed work.
  > ⚠ **NARROWED 2026-08-09 for `D1b-role` only, and WIDENED the same day by
  > ruling `evt_23eb7gp8sz4an`.** Read the whole carve-out; the first version of
  > it was too narrow to reach the repair.
  >
  > ⇒ **In scope for `D1b-role`, inside `crates/ken-elaborator`:**
  >
  > - `compiler_driver.rs` — `emit_package_from_env`, to build the
  >   `CheckedRuntimeSymbolsV1` record from the exact `stable_symbols_for_env`
  >   table while the live `ElabEnv` is still in hand, **and both producer
  >   resolvers** (`checked_runtime_symbols_v1` and `checked_host_spine_v1`);
  > - **`prelude.rs` / `PreludeEnv` registration**, to capture the immutable
  >   canonical role-`GlobalId` roster — added 2026-08-09 by ruling
  >   `evt_6q4tvtenb1wps`, see the block below;
  > - the checked package's versioned `semantic.metadata` lane, to carry that
  >   record under `core_semantic_hash`;
  > - `erasure.rs`, to decode and validate it into the typed Runtime field.
  >
  > ⛔ **Everything else in this bullet still binds**, and the original intent is
  > untouched: **you may not reshape what the arm *produces* to dodge the Runtime
  > gap.** The carve-out is for *carrying identity that already exists*, never for
  > changing the value. `ken-kernel` and `ken-interp` remain fully out of scope,
  > and so does the rest of `ken-elaborator`. Adding `Data` declarations to the
  > executable closure is forbidden by `D1b-role` item 2 regardless of crate.
  >
  > **This bullet forbade the only route to the repair for about forty
  > minutes, and its first repair was still too narrow to reach the second.**
  > Recorded because a Forbidden list is read on its own, far from the
  > deliverable that carves it out.

## Sequencing

**Runtime's next slice after the current `RT-MATCH-RECURSOR-CONSUMERS` work.**
Do not interrupt a slice in flight for it. `D0` is measurement and does not
contend with `D8`'s pin.

### No reverse edge, and the direction is deliberate

`KERNEL-NESTED-IND` `AC-K12` requires native execution, the Cranelift verifier,
and interpreter/native agreement, so that node **cannot close** until this one
lands. **That is an acceptance condition of the Kernel node, not a reverse
implementation dependency**, and `blocks:` stays empty here — the same call
[[RT-TERMINAL-ALL-ELIM-AUTHORITY]] records for the identical shape. Kernel's
`D5` work lands as an accepted partial in the meantime; it does not wait on
this node and this node does not wait on it.
