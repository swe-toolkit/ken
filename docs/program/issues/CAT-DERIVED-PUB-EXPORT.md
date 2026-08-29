---
id: CAT-DERIVED-PUB-EXPORT
title: "Bring catalog Data/Collections/Derived.ken.md to the pub-export standard — mark its census-recorded exported operations pub so consuming packages can selectively import them instead of reimplementing. The provider prerequisite that unblocks census group 4 (derived-list reuse) and is a necessary half of the LawfulFunctors standalone repair."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-NAT-REUSE-CONSUMERS]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/3079
origin: "Steward, 2026-08-28. The catalog-reuse census (docs/program/cat-reuse-census.md) records Data.Collections.Derived as [all-private] with a clean standalone state, and §4.2 lists its export set. CAT-NAT-REUSE-CONSUMERS drained census groups 2 and 3; of the five remaining low-risk groups, group 4 is the only one whose provider is NOT in the §4.3 standalone-failure set, so it is the next releasable lane-3 node. Shaped on the landed CAT-ORDER-PUB-EXPORT precedent. Steward-filed per COORDINATION section 2."
---

> # SCOPE IS PROVIDER-DERIVED, NOT CONSUMER-DERIVED — read this before sizing
>
> An earlier Steward roster note scoped this node to "exactly `list_append`,
> `length`, `reverse`, `concat_map`". **That set was derived from what census
> group 4 consumes, not from what the census says this provider exports**, and it
> silently dropped two names. Census §4.2's `Data.Collections.Derived` export set
> is NINE names: `list_append`, `length`, `reverse`, `concat_map`, `eq_from_ord`,
> `count`, `Perm`, `insert`, `sort`.
>
> **This node takes SIX of them** — the §4.2 set minus the three that §4.3
> deliberately ungroups as higher-risk attached-law ownership (`Perm`, `insert`,
> `sort`). Doing the provider in one pass is the point: a second pub-export WP
> over the same file later is churn, and it re-contends a file this lane already
> has to sequence around.
>
> **The two-name difference is a measurement D0 owns, not a Steward ruling.**
> `eq_from_ord` and `count` are included because they are in §4.2 and are not in
> §4.3's excluded set. If D0 finds either carries attached-law ownership, it is
> DROPPED to the excluded set and reported — not forced through. Dropping a name
> on measured ownership grounds satisfies this node; forcing one does not.

> # WHY THE `CAT-ORDER-PUB-EXPORT` SPLIT HAZARD IS MEASURED ABSENT HERE
>
> `CAT-ORDER-PUB-EXPORT` was split by an Architect ruling (`evt_6f4h4mhejp4bm`)
> because `Order.ken.md` was authored against ambient-provider scaffolding: it
> failed standalone inside `leq_nat::antisym` and carried an orphan `Ord Nat`
> instance, so a visibility patch could not be lawful closure.
>
> **The census measured that predicate and it does not hold for this module.**
> Row `D`: `[ok] standalone exit 0; no provider ownership error`, `ambient=-`.
> §4.3 names four modules that fail standalone (`Order`, `LawfulFunctors`,
> `BytesKeys`, `Cursor`) and `Derived` is not among them; §4.3 also states the
> orphan/foreign-attached ownership predicate is "currently measured only for the
> Nat-order component."
>
> **That is why this is an S/T2 execution node and not a migration.** It is also
> a measurement inherited from the census base, so D0 re-establishes it at the
> release SHA rather than trusting this paragraph. A standalone or ownership
> failure appearing at D0 is a HARD STOP to Steward + Architect, not something to
> patch through — that is exactly how Order's split was earned.

> # ATTACHED PROOFS ON AN EXPORTED FN ARE PROVEN TO WORK — DO NOT TREAT AS A GAP
>
> `reverse` carries `proof involutive for reverse`. A `pub fn` WITH attached
> proofs already elaborates and exports successfully: `Arithmetic.ken.md`
> pub-exports `add`/`mul` while carrying `proof zero_r for add` and friends, and
> that is the working reference `CAT-ORDER-PUB-EXPORT` was built on. The presence
> of an attached proof on an exported name is therefore NOT a hard stop here. What
> would be a hard stop is an ownership error — a proof attached to a subject this
> module does not own.

## Symptom inventory

Append one line per hard stop; never rewrite history.

## Objective

Bring `catalog/packages/Data/Collections/Derived.ken.md` into line with the
catalog implementation standard's pub-export requirement: the module's
census-recorded exported operations must be `pub` so a consuming package can
selectively import them instead of reimplementing them.

## Fixed inputs

Measured by the Steward at `origin/main`
`35b9d3fa1881e28e696d712bda69bb8ad86b14e3`, with provenance separated from
inheritance. **Do not treat an inherited number as a current measurement, and do
not treat the line numbers below as stable** — see Sequencing: this file moves
before release.

CURRENT-TREE, verified at `35b9d3fa1`:

- `catalog/packages/Data/Collections/Derived.ken.md` has **zero** `pub`
  declarations. Oracle: `grep -c '^pub' <file>` returns 0.
- All six in-scope names exist as bare top-level `fn`. Oracle:
  `grep -n '^fn \(list_append\|length\|reverse\|concat_map\|eq_from_ord\|count\)'`.
- The landed spelling to copy is `pub fn <name>`, as at
  `Nat/Order.ken.md:49,59,69,79` and `Nat/Arithmetic.ken.md:18,24`. **Copy the
  landed precedent, not spec prose.**
- The module imports `Core.Logic.Compare`, `Core.Logic.Or`,
  `Core.Logic.OrdResult`, and `Core.Logic.Transport`.

INHERITED from the census at its evidence base, NOT re-verified by the Steward:
row `D` records `[all-private]`, `absent=-`, `ambient=-`, and
`[ok] standalone exit 0; no provider ownership error`. **These are what D0
reproduces or corrects.**

## Deliverables

- **D0 — measure before changing anything.** At the release SHA, re-establish:
  the current `pub` count, the six names' declaration forms, and that the module
  elaborates standalone at Omega with no `UnresolvedCon` and no ownership error.
  Report per-name whether any of the six carries attached-law ownership. **A D0
  that finds the standalone/ownership state no longer clean is a HARD STOP to
  Steward + Architect** — it means this module joined the §4.3 set and the node
  is a migration, not a visibility patch.
- **D1 — mark the six in-scope operations `pub fn`**, using the landed spelling.
  `Perm`, `insert`, and `sort` are NOT marked here and are not this node's to
  take. Any name D0 dropped on measured ownership grounds is not marked either.
- **D2 — import-resolution witness.** A minimal probe selectively imports the
  marked names from `Data.Collections.Derived` and resolves each to the `Derived`
  definition's `GlobalId`. Re-verify the import surface at pickup, not inherited.

## Acceptance criteria, each with its control

- **AC-PUB.** Each in-scope name is `pub fn` and passes the
  `LANG-MOD-PUB-ELIGIBILITY` gate (top-level, public-typed subject). Control: the
  eligibility gate is actually run and its verdict reported per name, not
  inferred from the module elaborating.
- **AC-STANDALONE.** `Derived.ken.md` elaborates standalone at Omega with no
  `UnresolvedCon` and no eligibility rejection. Control: the standalone check is
  run on the module in isolation, not as part of a consumer's closure.
- **AC-IMPORT-RESOLVES.** A selective import resolves each marked name to the
  `Derived` `GlobalId`. Control: the probe must FAIL for a name deliberately left
  unmarked — an import witness that passes regardless of the `pub` marking is
  measuring nothing.
- **AC-EXCLUDED-UNMARKED.** `Perm`, `insert`, and `sort` are NOT PUBLISHED BY THE
  LOADER, and any name D0 dropped is not published either. **The criterion is
  export visibility, not source spelling, and it is measured with the SAME
  INSTRUMENT as `AC-IMPORT-RESOLVES`** — the roots loader / eligibility
  mechanism, never a text scan. Control, in two parts:
  - **Population from the file, verdict from the loader.** Derive the candidate
    population mechanically from the module's own top-level definitions, ask the
    loader which of them `Data.Collections.Derived` publishes, and assert that
    the published set EQUALS the in-scope set. **Assert equality, never per-name
    privacy** — a probe asking whether each excluded name resolves as private
    stays true for reasons other than the marking, so a stray export does not
    disturb it, whereas equality is falsified by any addition. Do NOT hand-write
    the population: an enumerated roster is satisfied by editing the roster.
  - **Required mutation, and it is CV's exact evasion.** Exporting an excluded
    name in ANY compile-preserving spelling must RED this control — including one
    leading space before `pub`, which the roots loader accepts and publishes at
    the exact provider `GlobalId`. Byte-restore afterwards and show the
    restoration.

  > **Why this replaced a `^pub` source census (Steward ruling 2026-08-29, on
  > conformance-validator `evt_a2130csdh28r` at exact `2d216d849`).** The census
  > was a PROXY for export visibility, resting on an unstated invariant that
  > `pub` always sits at column 0. CV refuted the proxy by measurement, not by
  > argument: ` pub fn mem` compiles, the roots loader selectively imports `mem`
  > at the exact provider `GlobalId`, and a `strip_prefix("pub ")` census stays
  > green 3/3. **The tell was an asymmetry visible in this AC list itself — the
  > positive arm asked the loader and the negative arm asked the text.** Two arms
  > of one property measured by two instruments means the weaker arm is a proxy,
  > and a proxy is only as strong as the invariant nobody wrote down. **General
  > form: measure a negative arm with the instrument that DECIDES the property,
  > not with one that merely usually agrees with it.**
  >
  > **The candidate's source state at `2d216d849` was already correct** — exactly
  > the six in-scope names carry `pub fn` at column 0 and `Perm`/`insert`/`sort`
  > are bare `fn`, Steward-verified independently at that SHA. **What is repaired
  > here is the CONTROL, not the tree. Do not touch the markings.**
- **AC-NO-COMPUTATIONAL-CHANGE.** `pub` is an export-visibility change only; no
  computational content of any operation changes. Control: a differential over
  the function bodies showing them byte-unchanged.
- **AC-CLOSURE-TARGETS.** Re-run the COMPLETE AFFECTED-TARGET CLOSURE, not the
  diff-touched set: every target that loads any module whose closure this
  increment changes, whether or not the increment touches its file. Control: name
  the closure and show the target set was derived from it. **This criterion is
  here because a diff-touched target set is blind to exactly the consumers an
  increment breaks by changing a closure rather than a file** — measured on the
  D6 respin, where 28 diff-touched targets passed 170/170 and CV still found a
  candidate-caused red at `lang_mod_catalog_completeness.rs:97`, in a file the
  increment never touched.
- **AC-NO-REGRESSION.** Whole-suite green in CI. Local targeted only, via
  `scripts/ken-cargo -p <crate>`, never `--workspace`.

## What this unblocks, and what it does NOT close

Unblocks census **group 4** (derived-list reuse) — that group's provider is this
module, and it is the only one of the five remaining low-risk groups whose
provider is not in the §4.3 standalone-failure set.

**It does NOT close `Core.Classes.LawfulFunctors`.** That module fails standalone
on `UnresolvedCon list_append` at `4204..4215`, and making `list_append` public
here is a necessary half of that repair, not the whole of it. The census is
explicit: those closure facts are "not permission to guess an import repair," and
each needs its own standalone/ownership prerequisite. **Do not extend this node
to touch `LawfulFunctors`.**

## Contention check

Touches `catalog/packages/Data/Collections/Derived.ken.md`. **That is the same
file `CAT-NAT-REUSE-CONSUMERS` D6 edits**, which is the whole reason this node is
sequenced behind it. Re-check at release — D6 has hard-stopped twice and had a QA
approval retracted, so its final footprint is not yet fixed.

## Reviewers

foundation-qa (the `pub` markings pass eligibility, the module elaborates
standalone, import resolution reaches the `Derived` `GlobalId`s, no computational
change) + conformance-validator (catalog implementation standard compliance). A
genuine design/spec gap — eligibility, attached-proof ownership — HARD-STOPS to
spec/Architect; a gap finding is the payoff, not a setback.

## Sequencing

Lane-3 (foundation). **`draft` on purpose, and the frame is COMPLETE — the status
is the honest part.** Two things gate it, and neither is framing work:

1. **File contention.** D6 edits this exact file. `depends_on` names
   `CAT-NAT-REUSE-CONSUMERS` for that reason — the dependency is sequencing and
   contention, not semantics. A prose-only gate gates nothing, so it is encoded
   where `gen-progress.sh` reads it.
2. **Fixed-input staleness.** The line numbers above resolve at `35b9d3fa1` and
   D6 will move them. D0 re-measures at the release SHA; the oracles are given as
   greps precisely so the ring reproduces the values rather than trusting mine.

**Flip it `ready` and release once D6 resolves and the batch closes**, measuring
D0 at the post-D6 SHA. Tier T2 — applying a landed capability (pub export, proven
by `Arithmetic.ken.md`) to one more catalog module, reviewed on the export
surface and the import-resolution witness. If D0 surfaces a genuine ownership or
standalone gap it escalates to T1 spec/Architect via hard stop.
