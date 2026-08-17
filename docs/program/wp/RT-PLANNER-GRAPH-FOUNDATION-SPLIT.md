# RT-PLANNER-GRAPH-FOUNDATION-SPLIT — the planner's shared substrate, moved first

**Cut item 3 of [[RT-BACKEND-MODULE-SPLIT]], and the first planner slice.**
`planning/static_transition.rs` carries the whole pre-emission planner in one
file. Six domain slices (items 4-9) are queued behind it, and every one of them
needs the same root plan type and the same identity/coordinate vocabulary. **This
node moves that shared substrate into its own module so the six can each land
against a stable foundation instead of six-way contending on one file.**

**Owner:** Team Runtime. **Branch:** `wp/RT-PLANNER-GRAPH-FOUNDATION-SPLIT`.
**Size:** unsized on purpose — see §4. **Risk:** medium; a pure move with a
large blast radius and a known `cfg`-profile trap.

---

## 1. Fixed inputs — the census is CURRENT for this region, and that is measured

Measured at `origin/main = c03331ad8`.

> ### THE PLANNER HAS NOT MOVED SINCE THE CENSUS. Do not re-take those inventories.
>
> `docs/program/backend-split-census*.md` pin measurement SHA
> `4de48651434dd6340f81ec9b1b7a5ac2ec8c0199`. Between that SHA and
> `c03331ad8`, `crates/ken-runtime/src/cranelift_backend/planning/` has **zero
> commits and an empty diff** — verified both ways by the Steward. **The census's
> planner rows are therefore current, not stale**, which is unusual and is worth
> the one command to re-confirm at pickup rather than assumed:
>
> ```sh
> git diff --stat 4de48651..<your-base> -- crates/ken-runtime/src/cranelift_backend/planning/
> ```
>
> **If that diff is non-empty at your base, the fixed inputs below are stale and
> you re-measure before moving anything.** This is the whole reason the campaign
> forbids carrying line counts into a frame: the number is only good until
> someone lands in the region.

| anchor at `c03331ad8` | what it is |
|---|---|
| `planning.rs` | the facade; re-exports the planner surface, and carries the `cfg`-gating warnings in §5 |
| `planning/static_transition.rs` | the planner monolith this node carves |
| `planning/static_transition.rs:2638` | `StaticTransitionPlan<'src>` — the root plan type |
| `planning/static_transition.rs:8-9` | `mod abi; mod semantic_ir;` — **the carve pattern that already worked twice in this exact file** |
| `planning/static_transition.rs:18381` | the inline `mod tests` |
| `backend-split-census-type-ownership.md` | 76 planner-owned type rows, with visibility and full external consumer sets per row |
| `backend-split-census-cochange.md` | which planner regions historically change together |

**The line counts are deliberately omitted from this table.**
`RT-BACKEND-MODULE-SPLIT:330-359` bars carrying today's counts into a frame, and
the campaign's own guardrails warn against optimizing for equal-sized files.
Measure at pickup if you need a number; do not plan to one.

## 2. Why this node is first, and why it is not a mega-diff

**Items 4-9 are six planner domain slices** — units/ABI, occurrences,
continuations, aggregates, effects, joins/traps. The census's type-ownership
ledger shows these as real families by name (`Continuation*`, `PlannedAggregate*`
/ `Synthesized*`, `EffectSeat*`, `Fusion*`, `Join*`, `*Unit*`). **They are
separable from each other. They are not separable from the root plan type and
the identity vocabulary they all quote.**

⇒ Move the shared substrate once, first. The alternative is six slices each
re-deciding where `StaticTransitionPlan` lives, which is six chances to get it
wrong and a guaranteed contention.

**This does NOT license a planner mega-diff** (`RT-BACKEND-MODULE-SPLIT:89-93`).
The foundation is the *smallest* set that unblocks the six, not everything that
could plausibly be called shared. **A type only one domain uses belongs to that
domain's slice, not here** — pulling it forward is how a foundation slice becomes
the whole planner.

## 3. THE CARVE PATTERN IS ALREADY PROVEN IN THIS FILE. Copy it.

`planning/static_transition.rs` already has two children carved out by
[[RT-NATIVE-FNSPLIT]]'s `B1`/`B1R` recuts — `abi.rs` and `semantic_ir.rs` —
declared at `:8-9` and consumed through `use abi::{...}` / `use semantic_ir::{...}`
with `pub(in crate::cranelift_backend) use` re-exports.

**Follow that shape.** It is in-tree, reviewed, and landed twice. Do not invent a
different module idiom, and do not create a facade that re-exports the monolith
wholesale — the campaign gates bar facade recreation.

> ### THE `cfg`-PROFILE TRAP IS NAMED IN THE FILE'S OWN DOC, THREE TIMES. It is real.
>
> `planning.rs` carries re-exports gated `#[cfg(test)]` and
> `#[cfg(any(test, feature = "r3-4b-observation"))]`, and its own comment warns
> that **an ungated *use* of a `cfg(test)`-gated re-export is an unresolved import
> in the production build that the test profile cannot see.**
>
> **A targeted test run is exactly the instrument that cannot catch this.** Every
> moved or re-exported name must be attributed to its `cfg` profile, and the
> library must be built on the profile that does *not* include tests. This is the
> same class the Adversary flagged on [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] —
> `cfg` context and path-relative resolution are what a whitespace-normalizing
> fidelity comparison structurally cannot see.

## 4. Deliverables — `D0` FIRST, and the node is unsized until it reports

> ### SIZING IS HELD. The node carries `size: TBD`, and that is the real value.
>
> **The precedent is [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]], and it is why that
> slice landed cleanly.** It was safe to move because
> [[RT-BACKEND-SPLIT-CENSUS]]'s `D6` had already returned a **bounded ownership
> proof** — one dispatcher, one caller, every helper call site inside. **No such
> proof exists for the planner foundation**, and inventing a foundation set
> without one is how a pure move turns into an exposed behavioural dependency
> mid-candidate.
>
> ⇒ `D0` produces that proof. **The Steward cuts `D1` onward against what `D0`
> returns, and not before.**

- **`D0` — the bounded ownership proof for the foundation set. THIS IS THE
  DISPATCH.**

  **Start from the census, do not re-derive it.** `backend-split-census-type-
  ownership.md` already records every planner-owned non-private type with its
  visibility and its full external consumer set. That is the input.

  **The hypothesis to test, stated so it can be refuted:** the foundation is
  `StaticTransitionPlan<'src>` (`:2638`) plus the identity and coordinate
  vocabulary the six domain families all quote. **This is a starting set read off
  the census, NOT the answer.** Report the set you can actually close.

  Produce:

  1. **The closed set** — every type, function and `impl` that must move for the
     set to compile in its own module, with each member's justification.
  2. **The boundary** — for each member, which of the six domains reference it.
     **A member referenced by only one domain is evidence it belongs to that
     domain's slice instead**, and should be argued out of the set rather than
     carried.
  3. **The `cfg` attribution** — every moved name's profile, per §3.
  4. **The exposed-dependency report** — anything whose move would require a
     signature, representation or behaviour change. **That is a hard stop, not a
     thing to repair inside a pure move** (`RT-BACKEND-MODULE-SPLIT:89-93`).

  **`D0` changes no code.** It is a report, like the census it draws on.

- **`D1` onward — the move.** Cut by the Steward against `D0`.

## 5. Acceptance criteria

**`AC-0` is the only one `D0` must meet. The rest are the campaign's standing
structural gates and are recorded here so the recut inherits them.**

- **`AC-0` — the set is CLOSED and the closure is demonstrated, not asserted.**
  For every member, the report shows why it must move; for every non-member the
  hypothesis proposed, it shows why it need not. **A set presented without its
  refuted candidates has not been closed — it has been chosen.**

- **`AC-1` — exact old/new symbol and test-property ledgers** (campaign gate).
- **`AC-2` — no representation, diagnostic, hash, serialization, behaviour or
  trust change** (campaign gate). No widened production API, no facade
  recreation of the monolith.
- **`AC-3` — the affected library AND the targeted test configurations both
  compile**, per §3's trap. A green targeted test run alone does not discharge
  this.
- **`AC-4` — each moved mutation reds the same reached property, with the same
  NONZERO denominator, and restores** (campaign gate).
- **`AC-5` (no-regression).** Workspace green **in CI** — never a local
  `--workspace` run (`COORDINATION §12`).

## 6. Research reports — cite them, do not inherit them

**Binding on this frame, per `RT-BACKEND-MODULE-SPLIT:330-359` (operator,
2026-08-08). Both are landed on `main` and both are referenced here so the
Architect has them in hand at review.**

| report | what it supplies |
|---|---|
| `research/compiler-refactoring-program.md` (#1630) | the two-arc program, the recommended module-ownership map (§4), the stage breakdown (§5), the recommended WP cuts (§6), nine named guardrails (§7) |
| `research/compiler-obligation-ir-refactor.md` (#1628, #1631) | canonical planned/generated terms, a closed source machine, a hybrid checked transducer, immediate Cranelift command interpretation, post-emission evidence |

> **REFERENCE IS NOT ADOPTION.** Both are marked advisory; neither is an
> architecture ruling, and the first says outright that the Steward and Architect
> own the node graph. **This node stays a behaviour-preserving split unless the
> Architect rules otherwise.** Do not import the IR architecture through a slice
> frame — the venue for triaging that recommendation is the campaign node, by the
> Architect.
>
> **Where the reports agree with a constraint here, that is a reason to trust the
> constraint, not to relax it.** Their guardrails independently warn against
> optimizing for equal-sized files, naming permanent modules after temporary
> campaign nodes, and combining pure moves with semantic rewrites — all three of
> which this frame already forbids.

**Naming:** the module this creates is permanent; **the node is not.** Do not
name the module after this node.

## 7. Banned scope

- **Any semantic change.** An exposed behavioural dependency stops the move and
  returns for a semantic ruling.
- **Pulling in a type only one domain uses.** It belongs to that domain's slice.
- **Claiming `#8` closure.** This is one accepted phase partial among eighteen.
- **Building `D1` in the same turn as `D0`.** The recut is the Steward's.
- **Re-taking the census inventories** — see §1; they are current for this region.
- **A `--workspace` build or test locally** (`COORDINATION §12`).

## 8. Hard stop

Stop and return the seam if `D0` cannot close a set smaller than the planner
itself, if any member's move requires a signature or representation change, or if
the six domain families turn out not to be separable at the boundary the census
suggests. **Any of those means the cut is wrong, and the recut is the Steward's.**
