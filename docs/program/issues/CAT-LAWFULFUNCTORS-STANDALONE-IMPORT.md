---
id: CAT-LAWFULFUNCTORS-STANDALONE-IMPORT
title: "Bring Core/Classes/LawfulFunctors.ken.md to standalone-clean by adding the single missing import of list_append from Data.Collections.Derived — the provider is now pub and standalone-clean, no cycle, no chained prerequisite, so this removes LawfulFunctors from the census standalone-failure set with one import line. It does NOT do the deferred law-carrying list_map/list_foldr reuse migration."
status: ready
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-DERIVED-PUB-EXPORT]
blocks: []
github: null
origin: "Steward, 2026-09-02, on an operator-concurred framing of the LawfulFunctors standalone/ownership repair. The catalog-reuse census (docs/program/cat-reuse-census.md) row LF marks Core.Classes.LawfulFunctors [higher]: standalone rejects UnresolvedCon list_append at 4204..4215. A Steward measurement (subagent, 2026-09-02) established the failure shape is the SIMPLEST one — a pure missing import, not an ownership wall: LawfulFunctors uses list_append with neither a local definition nor an import, relying on ambient full-catalog scaffolding to resolve it, so in isolation it becomes UnresolvedCon. The provider Data.Collections.Derived.list_append is now pub (CAT-DERIVED-PUB-EXPORT MERGED; Derived.ken.md:77) and census row D is [ok] standalone exit 0 with no provider ownership error; Derived does not import LawfulFunctors, so importing it introduces no cycle. list_append is the SOLE unresolved symbol (every other used helper is defined locally in the file). Repair = one import line, identical to the landed precedent Parsing.ken.md:42. Steward-filed per COORDINATION section 2."
---

> # OWNERSHIP AXIS MEASURED ABSENT — THIS IS A MISSING IMPORT, NOT A MIGRATION
>
> The operator's framing named a "standalone/ownership repair." **The ownership
> axis was measured and it is clean.** Unlike `Nat.Order` (census component `N`,
> which carries an orphan `Ord Nat` instance and a foreign attached
> `bool_or::eq_true_of_or`, and must move atomically to its class owner), the
> LawfulFunctors failure is a pure resolution gap:
>
> - The provider `Data.Collections.Derived.list_append` is already `pub`
>   (`Derived.ken.md:77`) and its module is `[ok]` standalone (census row `D`:
>   `standalone exit 0; no provider ownership error`).
> - `Derived` does not import `LawfulFunctors` (no reverse edge) — importing it
>   introduces **no cycle**.
> - `list_append` is the SOLE unresolved symbol; every other helper the module
>   uses (`idf`, `comp`, `bool_and`, `list_map`, `list_foldr`, `option_map`,
>   `option_foldr`, `monoid_mempty`, `fold_map_step`, `list_to_list`) is defined
>   locally in the same file.
>
> **So the repair is one import line and the node is S/T2, not a T1 migration.**
> If D0 re-measures at the release SHA and finds an ownership error or a second
> unresolved symbol, that is a HARD STOP to Steward + Architect — it would mean
> the module is not what this measurement found, exactly as `Order`'s split was
> earned. Do not patch a second symbol through under an import-repair frame.

> # THE LAW-CARRYING REUSE MIGRATION IS OUT OF SCOPE (deferred behind a ruling)
>
> Census row 28 proposes reusing `list_map→Prelude.map`, `list_foldr→Prelude.fold`
> and `bool_and→LawfulClasses.bool_and` inside this file. **Those are currently
> LOCALLY DEFINED, so they cause no unresolved error and are NOT standalone
> blockers.** The `list_map`/`list_foldr` half is a law-carrying substitution
> across distinct rigid heads (non-convertible), which the Steward has DEFERRED
> behind an Architect ruling. **Do not do that migration here.** This node adds
> the one missing import and nothing else; the private `comp` def is left
> untouched.

## Symptom inventory

Append one line per hard stop; never rewrite history.

## Objective

Make `catalog/packages/Core/Classes/LawfulFunctors.ken.md` elaborate standalone
at Omega by adding the single missing import that resolves `list_append` to the
now-public `Data.Collections.Derived.list_append`, removing the module from the
census §4.3 standalone-failure set.

## Fixed inputs

Measured by the Steward at `origin/main`
`e93210c423e2988639fb3627e5487b73433711d5`. **Line numbers below are a
markdown-stripped ken-source offset (`4204..4215`) and physical prose lines; D0
re-establishes the code use sites at the release SHA — reproduce them, do not
trust these.**

CURRENT-TREE, verified at `e93210c4`:

- `Data.Collections.Derived.list_append` is `pub`: `Derived.ken.md:77`,
  `pub fn list_append (a : Type) (xs : List a) (ys : List a) : List a = …`.
  Oracle: `git grep -nE '^\s*pub fn list_append' origin/main -- <Derived>`.
- `LawfulFunctors.ken.md` has **no `import` statement**. Oracle:
  `grep -c '^import' <LawfulFunctors>` returns 0.
- The landed spelling to copy is `import Data.Collections.Derived (list_append)`,
  exactly as `Capability/Parsing/Parsing.ken.md:42`. **Copy the landed
  precedent, not spec prose.**

INHERITED from the census at its evidence base, NOT re-verified by the Steward:
census row `LF` records `[higher]` — `standalone rejects UnresolvedCon
list_append at 4204..4215`. **This is what D0 reproduces before repairing and
confirms cleared after.**

## Deliverables

- **D0 — measure before changing anything.** At the release SHA, reproduce the
  standalone failure: `LawfulFunctors.ken.md` in isolation rejects with exactly
  `UnresolvedCon list_append` and **no other** unresolved symbol or ownership
  error. **A D0 that finds a second unresolved symbol, an ownership error, or a
  cycle from the import is a HARD STOP to Steward + Architect** — it means the
  module is a migration, not an import repair.
- **D1 — add the single import line** `import Data.Collections.Derived
  (list_append)`, in the module's first `ken` code block, using the landed
  `Parsing.ken.md:42` spelling. Change nothing else — no local def removed, no
  reuse substitution, `comp` untouched.

## Acceptance criteria, each with its control

- **AC-STANDALONE.** `LawfulFunctors.ken.md` elaborates standalone at Omega with
  no `UnresolvedCon` and no ownership rejection. Control: the standalone check is
  run on the module in isolation (not inside a consumer's closure), and D0's
  pre-repair run showing the `UnresolvedCon list_append` failure is the paired
  negative — the same check must FAIL before D1 and PASS after.
- **AC-IMPORT-RESOLVES.** The added import resolves `list_append` to the
  `Data.Collections.Derived` definition's `GlobalId`. Control: the resolution is
  read from the loader/elaborator, and a probe that deletes the import must
  reintroduce the exact `UnresolvedCon list_append` — an import witness that
  passes with the line removed is measuring nothing.
- **AC-SOLE-SYMBOL.** No symbol other than `list_append` becomes newly resolved
  or newly unresolved by this change. Control: a before/after unresolved-symbol
  census over the module shows the set went from `{list_append}` to `{}` and no
  other entry moved. This is the guard against silently repairing a second gap
  under the frame.
- **AC-NO-COMPUTATIONAL-CHANGE.** Only an import line is added; no function body,
  instance, or proof in the module changes. Control: a differential over the
  module's definitions showing them byte-unchanged except the inserted import.
- **AC-CLOSURE-TARGETS.** Re-run the COMPLETE AFFECTED-TARGET CLOSURE, not the
  diff-touched set: every target that loads any module whose closure this
  increment changes, whether or not the increment touches its file. Control: name
  the closure and show the target set was derived from it. Adding an import edge
  changes `LawfulFunctors`'s closure, so any fixture loading `LawfulFunctors` (or
  a module that loads it) is in scope even though only one file is edited.
- **AC-NO-REGRESSION.** Whole-suite green in CI. Local targeted only, via
  `scripts/ken-cargo -p <crate>`, never `--workspace`.

## What this unblocks, and what it does NOT close

Removes `Core.Classes.LawfulFunctors` from the census §4.3 standalone-failure
set, making it a usable standalone-clean module — the prerequisite for any
consumer that imports from `LawfulFunctors`.

**It does NOT do the census row-28 reuse migration** (`list_map→Prelude.map`,
`list_foldr→Prelude.fold`, `bool_and→LawfulClasses.bool_and`). Those symbols are
locally defined and cause no unresolved error; the law-carrying half is deferred
behind an Architect ruling. **Do not extend this node to touch them.**

## Note on ambient-resolved primitives (not a gap)

`cong` is used in the module (transport) without a local def and without an
import, yet the census does not flag it: the standalone elaborator resolves
transport primitives (`cong`/`sym`/`trans`) ambiently. It is therefore NOT a gap
under the current elaborator and is out of scope here. If a future node tightens
ambient resolution, `cong` is the one symbol besides `list_append` that is
neither locally defined nor imported — worth a one-line confirmation then, not
now.

## Contention check

Touches only `catalog/packages/Core/Classes/LawfulFunctors.ken.md` (one added
line). No other lane touches this file; the provider `Derived.ken.md` is already
merged and is not edited here. No `crates/` change beyond test-fixture closure;
no `/spec`, no kernel/TCB.

## Reviewers

foundation-qa (the module elaborates standalone, the import resolves to the
`Derived` `GlobalId`, no computational change, the sole-symbol guard holds) +
conformance-validator (catalog implementation standard compliance). A genuine
design/spec gap — an ownership error surfacing at D0, or a second unresolved
symbol — HARD-STOPS to spec/Architect; a gap finding is the payoff.

## Sequencing

Lane-3 (foundation). `ready` on release: the provider half `CAT-DERIVED-PUB-EXPORT`
is MERGED (so `depends_on` is satisfied), the file has no contention, and the
repair is a single import line proven by the `Parsing.ken.md:42` precedent. D0
re-measures at the release SHA. Tier T2 — applying a landed capability (selective
import of a pub provider) to one more catalog module, reviewed on the standalone
check and the import-resolution witness. If D0 surfaces an ownership or
second-symbol gap it escalates to T1 spec/Architect via hard stop.
