---
id: CAT-GCD-REFACTOR
title: "Refactor Gcd.ken.md to the catalog implementation standard — import Nat add/mul from Data/Numeric/Nat/Arithmetic and leq_nat/sub from Data/Numeric/Nat/Order instead of reimplementing them, and arrange the module top-down (divides_gcd first, fundamentals last)"
status: active
owner: foundation
size: S
gate: none
depends_on: [CAT-GCD, CAT-ORDER-PUB-EXPORT, CAT-ORD-NAT-CANONICAL-OWNER]
blocks: []
github: null
origin: "Operator directive 2026-08-22, after CAT-GCD merged (3283528c4): Gcd.ken.md redundantly reimplements generic Nat tools that already exist in the catalog, and is arranged bottom-up. This is a well-factoring / arrangement follow-up, not a soundness re-open — CAT-GCD stays closed. Steward-filed. Held until the foundation ring is reseated to pi (see reseat directive) so the standard's first application runs on the new seating."
---

> # AC-ARRANGE HARD STOP RULED 2026-08-26 — Architect carrier-first / headline-first-function layout (literal AC-ARRANGE PRESERVED)
>
> foundation-leader HS (evt_5712j1pm9h01): D0 confirmed the reuse/import premise.
> The implementer measured that `divides_gcd` before its `Divides` carrier fails
> (`UnresolvedCon Algorithm.Numeric.Gcd.Divides`), and moving only that carrier
> then fails on later `gcd_spec_greatest`. The A/B fork offered: (A) weaken
> AC-ARRANGE to prose-only literate exposition, or (B) a loader-prerequisite WP.
>
> SUPERSEDED. My interim ruling picked (A). The Architect then supplied a grounded
> mechanism-fidelity ruling (evt_1qew47n3zr0bc) that FALSIFIES the premise behind
> (A): literal AC-ARRANGE IS expressible on current `origin/main`, without
> weakening it to prose and without a loader prerequisite. The loader's SCC
> dependency ordering applies to one maximal run of function/proof declarations; a
> `data` declaration breaks that run — which is why moving only `Divides` exposed
> the next carrier break, NOT proof that top-down function order is impossible.
>
> OPERATIVE RULING (Architect, mechanism-fidelity — component/mechanism is the
> Architect's call; Steward reconciles the AC to it, evt below): the literal
> AC-ARRANGE stands. Achieve it with this exact shape —
> 1. imports;
> 2. the dependency-ordered carrier `data` declarations `Divides`, `BoolView`,
>    then `GcdSpec`;
> 3. `divides_gcd` as the FIRST function in the following uninterrupted
>    function/proof run;
> 4. every implementation helper and proof function after it, with no later `data`
>    declaration splitting that run.
>
> This puts the headline operation before every low-level helper (literal
> AC-ARRANGE) while the loader still sees carriers before their use. The Architect
> independently executed this transformation (including the four ruled imports and
> local-duplicate removal) in a disposable exact-base worktree and passed a
> roots-loader probe (`elaborate_module_from_roots([catalog_root],
> "Algorithm.Numeric.Gcd")`, 1 passed). Option (B) stays REJECTED (no capability
> gap). No loader repair, no proof duplication, no semantic change; `trusted_base`
> unchanged. One confirming gate: foundation-qa/CV verify the candidate against
> the (literal) AC-ARRANGE below + AC-REUSE + AC-LAWS. The ring resumed on this
> ruling and reported fresh D0 green on the exact layout (evt_4k24efhx3gzn5).
>
> Steward reconciliation (evt_qz3r769mmcrz handback): this doc-only supersedes the
> prior prose-only amendment; the literal reading is RESTORED as operative. AC
> body below re-states the literal layout accordingly.
>
> # BLOCKED on BOTH Order successors 2026-08-26 (Architect split evt_6f4h4mhejp4bm)
>
> The Order half split further: the Architect ruled the pub-only fix incomplete (an
> orphan `instance Ord Nat`), so the Order rework is now TWO nodes — the ownership
> migration [[CAT-ORD-NAT-CANONICAL-OWNER]] then the narrow
> [[CAT-ORDER-PUB-EXPORT]]. This Gcd-only refactor resumes only after BOTH land, and
> the import surface must be RE-VERIFIED at pickup: after the migration `leq_nat` is
> imported from `Core.Classes.LawfulClasses` (its canonical home), NOT from Order —
> Order re-exports it. `sub` is still imported from Order. Re-measure before
> building.
>
> # REFRAME 2026-08-26 — import surface RE-MEASURED; Order half extracted (operator: continue foundation)
>
> Operator ruled the three-lane feasibility trial passed and directed the
> foundation lane to continue. Re-measured the falsified premise against current
> main: the Arithmetic half is now RESOLVED — `add`/`mul` are `pub fn` and
> `Arithmetic.ken.md` imports `Core.Logic.Transport (cong, sym, trans)` (the old
> `UnresolvedCon "cong"` standalone-load failure is gone). The reuse mechanism is
> demonstrated (a `pub fn` WITH attached proofs already exports and elaborates).
> The remaining gap is the Order half: `Order.ken.md` still has ZERO pub exports, so
> `leq_nat`/`sub` are not importable. Per the 2026-08-22 hold's own scope ruling (do
> NOT expand this Gcd-only WP into dependency-package repairs), that Order
> remediation is extracted to [[CAT-ORDER-PUB-EXPORT]] (released now).
>
> This node stays `draft` (Gcd-only) and RESUMES after [[CAT-ORDER-PUB-EXPORT]]
> lands: import `add`/`mul` from Arithmetic and `leq_nat`/`sub` from Order, drop the
> local reimplementations, arrange top-down. `depends_on` retargeted from the draft
> `LANG-MODULE-IMPORT-SYSTEM` umbrella to the specific Order prerequisite. The
> import surface must be RE-VERIFIED at pickup, not inherited — that was the
> original defect. This is the PILOT instance of the wider catalog-modernization
> pattern (prelude-redundancy removal + import-reuse + top-down arrangement) the
> operator raised 2026-08-26; the campaign shape is a separate operator scope call.
>
> # HELD — FRAME PREMISE FALSIFIED, reframe owed (Steward, 2026-08-22)
>
> Released to the reseated pi foundation ring and immediately hard-stopped by the
> foundation-leader (evt_4qy0b6p16vg5b): the import-surface premise is FALSE.
> `add`/`mul` (Arithmetic) and `leq_nat`/`sub` (Order) are plain non-`pub` `fn`s,
> not importable public exports, and `Arithmetic.ken.md` does not even elaborate
> standalone (`UnresolvedCon { name: "cong" }`). Selective import cannot reach
> them, so this WP is not executable as framed. The Steward inherited the
> importable-public-export premise without verifying it.
>
> This is not a one-node defect: it means the catalog reuse standard's core
> mechanism (import canonical tools instead of reimplementing) has an unmet
> PREREQUISITE — the canonical modules must load standalone AND export their tools
> as `pub` before any package can reuse them. Foundation correctly HOLDS and does
> NOT expand this Gcd-only WP into dependency-package/public-export repairs. Owner
> of the reframe/replace: Steward, with an Architect design ruling on the reuse
> mechanism and an operator scope call (see the second-lane / catalog-reusability
> campaign question). Flipped active -> draft to reflect the hold. Do not release
> until the reuse prerequisite is resolved and this frame is rewritten.

## Objective

Bring the merged `catalog/packages/Algorithm/Numeric/Gcd.ken.md` into line with
the **catalog implementation standard** (`ken-conformance-validator`, "Catalog
implementation standard"): a package holds only what is specific to it, reuses
the catalog's generic tools rather than reimplementing them, and is arranged
top-down.

## Motivation

CAT-GCD is sound and stays closed — the Adversary hunted it clean
(evt_2cds3ty6qevch) and its acceptance oracle passed. The defect is **factoring
and arrangement**, which no soundness gate checks:

- `Gcd.ken.md` locally defines `add`/`mul`, already exported by
  `catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md`.
- `Gcd.ken.md` locally defines `leq_nat`/`sub`, already exported by
  `catalog/packages/Data/Numeric/Nat/Order.ken.md`.
- The module is arranged bottom-up (fundamentals first), so a reader meets the
  plumbing before the point of the module.

## Deliverables

- `Gcd.ken.md` imports `add`, `mul` from `Data/Numeric/Nat/Arithmetic` and
  `leq_nat`, `sub` from `Data/Numeric/Nat/Order`; the local redefinitions are
  removed. If any local variant is genuinely distinct from the catalog export
  (not a plain duplicate), keep it and say why in one line — but the default is
  import.
- The module is re-arranged **top-down**: it leads with the headline export the
  package is named for (`divides_gcd` / the gcd law), and the more fundamental
  pieces it is built from follow, most-fundamental last.
- Nothing gcd-specific is lost: `Divides`, the fuel/termination presentation,
  `GcdSpec`, and the proved divisibility laws stay.

## Acceptance criteria

- **AC-REUSE.** The foundation-qa name-shadow scan (`ken-build-qa`, "Catalog
  WPs") reports zero local definitions in `Gcd.ken.md` shadowing a public export
  of an existing catalog module (or each surviving one carries a one-line
  distinct-tool justification the Architect accepts).
- **AC-ARRANGE.** The module is arranged top-down with the headline operation
  before every low-level helper, in this exact loader-valid shape (Architect
  ruling evt_1qew47n3zr0bc): (1) imports; (2) the dependency-ordered carrier
  `data` declarations `Divides`, `BoolView`, then `GcdSpec`; (3) `divides_gcd` as
  the FIRST function in the following uninterrupted function/proof run; (4) every
  implementation helper and proof function after it, with no later `data`
  declaration splitting that run. The vocabulary carriers precede their use (so
  the checked loader's SCC ordering is satisfied) while the reader meets the
  headline `divides_gcd` before any low-level helper. This is the LITERAL
  AC-ARRANGE, verified at the real qualified roots-loader boundary — not a
  prose-only weakening. (The 2026-08-26 interim prose-only amendment is SUPERSEDED
  by this ruling; see the banner above.)
- **AC-LAWS.** The gcd divisibility laws still hold — the CAT-GCD acceptance
  oracle stays green (trusted_base delta unchanged, laws instantiated). This is a
  behavior-preserving refactor.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12). Local
  targeted checks only, never `--workspace`.

## Capability tier and sequencing

Tier T2 — a behavior-preserving import-and-rearrange against an existing,
proved module; the review is differential (same laws, same delta, fewer local
defs, top-down order). Size S.

HELD, not startable yet: released only once the foundation ring is reseated to
pi and the catalog implementation standard has landed in the playbooks, so the
first application of the standard is on the reseated ring. The Steward releases
it, then CAT-DEQUE/CAT-BSEARCH/CAT-VEC, all authored factored from the start.
