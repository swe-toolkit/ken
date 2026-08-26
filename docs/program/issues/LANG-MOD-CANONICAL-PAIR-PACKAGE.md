---
id: LANG-MOD-CANONICAL-PAIR-PACKAGE
title: "Pair floor realization — admit the compiler-origin Pair to the prelude type floor as its tenth member and the three companions {mk_pair, pair_fst, pair_snd} as a SEPARATE closed binding inventory, resolving all four to the EXISTING compiler-installed GlobalIds (prelude.rs:951-1041) with no declare_def, no allocated identity, no alias/registry/fallback route, and zero trusted_base delta: split the conflated PRELUDE_FLOOR_NAMES type/binding inventory in modules.rs, close every Pair-binding collision before allocation, keep Prod and arbitrary compiler globals non-ambient, and flip the RED-UNTIL Strict conformance rows green. Realizes the landed spec LANG-MOD-PAIR-FLOOR-PROVIDER; the spec names this WP as the gate on those rows (34-data-match.md:102-111)."
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-MOD-PAIR-FLOOR-PROVIDER]
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Spec enclave boundary ruling (spec-author evt_6nk4xxkppz3k5, spec-leader evt_w7v4dvvzjr8k) + Architect mechanism ruling (evt_53295hb0v21mw), on Component B hard stop #1. REDIRECTED 2026-08-26 by the operator + Architect recut (evt_7d0ecgkd8ate3) from a package-migration plan to the floor realization. REWRITTEN 2026-08-26 by the Steward against the ACTUALLY-LANDED spec text after LANG-MOD-PAIR-FLOOR-PROVIDER merged at 8f3b6fd2c, as that node's REDIRECTED banner required (rewrite against landed text, not before it). Operator 2026-08-26: 'first launch the internal-provision prelude recut and finish that effort, then return to the z3 integration campaign' — the spec WP alone does not make Strict green, so this realization is the completion of that effort. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # REWRITTEN 2026-08-26 against the landed spec — this is the FLOOR-REALIZATION build WP
>
> [[LANG-MOD-PAIR-FLOOR-PROVIDER]] merged at `8f3b6fd2c`. Its REDIRECTED banner
> required this frame be rewritten to the reuse-the-four-ids floor-realization
> contract **against the actually-landed spec text, not before it** — done here.
> The whole prior body (retire the four compiler-installed globals; define one
> public `Pair` in an ordinary catalog package; migrate every consumer by import)
> is **DELETED as superseded**, not annotated: the landed spec forbids exactly
> that route. `34-data-match.md:92-100` — "Floor installation reuses those
> identities and allocates no replacement declaration"; `:102-111` — the build
> "must admit the existing four identities through the closed floor path, with no
> package Pair, alias, registry, ambient fallback, or mixed-resolution route."
>
> **The spec names this WP by id as the gate on its own staging.** The RED-UNTIL
> rows in `conformance/surface/modules/seed-pair-strict-boundary.md`,
> `seed-modules.md`, and `surface/taxonomy/minimality.md` are gated
> `RED-UNTIL LANG-MOD-CANONICAL-PAIR-PACKAGE`. `30-taxonomy.md:186-190` states
> the gap plainly: "Until the floor-realization build captures and admits the four
> existing Pair-family identities, current Strict loading may still reject their
> bare names. That implementation gap is not a package boundary and does not
> authorize a second identity or fallback route."
>
> **This is the completion of the internal-provision prelude recut effort**
> (operator 2026-08-26). The spec WP fixed the contract; it explicitly does NOT
> make Strict green. This build does.

## Objective

Realize the landed prelude floor in the elaborator: admit the compiler-origin
`Pair` as the **tenth** floor type and `{mk_pair, pair_fst, pair_snd}` as a
**separate closed companion-binding inventory**, resolving all four names to the
**existing** compiler-installed `GlobalId`s. The realization **allocates
nothing**: no `declare_def`, no new identity, no alias, no registry, no ambient
fallback, no consumer migration — and no `trusted_base()` entry. Strict
resolution reaches the four names only through the closed inventories.

## Fixed inputs (measured at the landed spec; re-verify at pickup)

- The canonical family is installed at `crates/ken-elaborator/src/prelude.rs:951-1041`
  via `declare_def`, checked-transparent: `Pair` (`:951-973`, body
  `Λ A. Λ B. Σ(_:A). B`), `mk_pair` (`:975-1003`), `pair_fst` (`:1005-1022`),
  `pair_snd` (`:1024-1041`); each registered into `elab.globals` under its exact
  bare name. All four sit **outside** `trusted_base()`.
- The floor array that must be split is
  `crates/ken-elaborator/src/modules.rs:88-98` —
  `pub const PRELUDE_FLOOR_NAMES: [&str; 9]` (no `Pair`), consumed by
  `is_prelude_floor_name` (`:100-102`), which gates strict resolution. The
  FLOOR-PROVIDER contract (item 4) records that this array **conflates the type
  inventory with the constructor-parent source** and must be split.
- Landed normative text: `30-taxonomy.md` §4 (`:108-190`, membership rule,
  internal-provision arm `{Nat, Pair}`, ten-type floor, four-name reuse,
  implementation-staging note) and §5 (`:192-212`); `33-declarations.md` §3.3
  (`:257-280`), §4.3 (`:328-356`), §5.3 (`:490-533`); `34-data-match.md`
  (`:46-111`); `39-elaboration.md` §2.0 (`:69-143`) and §6.1 (`:1013-1039`);
  `50-stdlib/51-lawful-classes.md` §7 (`:360-381`); `50-stdlib/README.md`
  (`:34-76`).
- Precedent: `LANG-MOD-NAT-FLOOR-REALIZATION` already landed the signature-eight
  plus kernel-origin `Nat` nine-type floor. This extends **the same mechanism**
  to compiler-origin `Pair` (`seed-modules.md:1457-1474`).

## Authorized mechanism (from the landed spec; no design fork open)

1. **Reuse, never allocate.** Capture the four existing Pair-family identities
   from the canonical pre-source environment and admit them. Installation
   performs **no `declare_def`**, allocates no `GlobalId`, adds no
   `trusted_base()` entry, and creates no second identity. A same-shaped source
   definition of `Pair` allocates a **distinct** identity and neither replaces
   the floor family nor converts structural equality into floor provenance
   (`30-taxonomy.md:196-202`).
2. **Two inventories, split — not one list.** The type floor becomes the closed
   ten `{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result,
   Utf8Error}`. The three companions are a **separate** closed binding inventory
   admitted at their exact pre-source ids with checked types keyed to the exact
   floor `Pair`; they **do not increase the ten-type count**
   (`33-declarations.md:268-271`). Split the conflated `PRELUDE_FLOOR_NAMES`
   accordingly.
3. **Closed-inventory resolution only.** The four names resolve through the
   witnessed closed inventories, **never** because the compiler global map
   happens to contain them (`39-elaboration.md:100-101`). Any name outside the
   closed inventories, any constructor-parent or companion-type identity
   mismatch, and any failure to find a canonical type or companion is a
   **surface error** — none falls back to a lookalike or arbitrary
   implementation global (`:118-129`).
4. **Immutable and unshadowable.** All four floor names are immutable and
   unshadowable in per-unit scope; every Pair-binding collision closes **before
   allocation**.
5. **Non-members stay out.** `Prod` remains unavailable as the standing negative
   control: an implementation-private convenience registered outside the closed
   floor is **not** ambient authority for name resolution
   (`33-declarations.md:273-274`).
6. **Ownership and obligations survive.** Canonical parameterised
   `Ord (Pair a b)` / `DecEq (Pair a b)` are lawful only at the class-owner locus
   `Core.Classes.LawfulClasses`; no package or facade becomes a head owner
   (`33 §5.3:528-531`, `39 §6.1:1030-1039`, `51 §7:360-381`). Re-export preserves
   identity and does not make the republishing module the source owner
   (`33 §4.3:336-344`). `Ord Nat` ownership is not transferred, and the three
   owed attached-proof conversions (`pair_compare_eq_sound`,
   `pair_compare_lt_asym`, `bool_or_eq_true_of_or`) are **not discharged** by
   this build (`50-stdlib/README.md:66-72`).

**Out of scope:** no kernel former, trust entry, registry, or alias; no package
`Pair`; no consumer migration to fresh ids; no `×` infix surface spelling; no
retirement of the compiler-installed globals. The `Order -> LawfulClasses ->
Compare -> Pair` catalog re-entry is **unblocked by** this WP but is not
performed here.

## Deliverables

- The split floor/binding inventories and the elaborator floor-realization path
  (mechanism items 1-5), reusing the four `prelude.rs:951-1041` identities.
- The `RED-UNTIL` Strict conformance rows flipped green (AC-ROWS below), with the
  survives-realization controls rerun.
- No new declaration, identity, trusted entry, alias, registry, or fallback.

## Acceptance criteria

- **AC-TEN** (floor is exactly ten, Pair resolves to the pre-source id) — the
  configured type floor is the closed ten-member set; each of the four bare names
  in an otherwise well-formed declaration resolves to **the corresponding
  recorded pre-source id**, in four independent strict-roots runs. This is
  `surface/modules/strict-bare-pair-floor-name-matrix-accepts-exact-ids`.
- **AC-COMPANIONS** (separate inventory, count unchanged) — the three companions
  are admitted at their exact pre-source ids with checked types keyed to the
  exact floor `Pair`, and **do not** increase the ten-type count. The
  type-inventory and binding-inventory closures are asserted independently, not
  through one conflated list.
- **AC-ZERO-ALLOC** (reuse proven, not assumed) — the realization performs zero
  `declare_def` calls for the Pair family, allocates no `GlobalId`, and produces a
  **zero `trusted_base()` delta**. A control that would admit a freshly allocated
  same-shaped identity in place of the recorded one **rejects**.
- **AC-DISTINCT-IDENTITY** (no structural back door) — a same-shaped source
  `Pair` definition allocates a distinct identity and does **not** replace the
  floor family or gain floor provenance
  (`definitionally-equal-pair-is-a-distinct-identity`).
- **AC-ROWS** (the gated rows flip) — these `RED-UNTIL` rows go green and the
  post-realization controls rerun:
  `seed-pair-strict-boundary.md` —
  `strict-bare-pair-floor-name-matrix-accepts-exact-ids`,
  `pair-floor-remains-available-after-unrelated-loads`,
  `pair-floor-binding-collisions-reject-before-allocation`,
  `pair-reexport-is-identity-preserving-republication`,
  `pair-floor-beta-eta-are-definitional`,
  `kernel/inductive/floor-pair-positive-path-unfolds-to-sigma`,
  `pair-floor-closure-is-rederived-after-realization`;
  `seed-modules.md` — `closed-floor-accepts-arbitrary-global-does-not`;
  `surface/taxonomy/minimality.md` —
  `prelude-internal-provision-inventory-is-executable-and-closed` (its THE GAP
  note: the executable derivation and Strict resolution must match the closed
  inventories).
- **AC-COLLISION** (closes before allocation) — every Pair-binding clash is
  rejected **before** allocation, not after.
- **AC-NEGATIVE-PROD** (non-members stay non-ambient) — `Prod` and arbitrary
  compiler globals remain unavailable to name resolution; presence in the
  compiler global map confers nothing.
- **AC-BETA-ETA** (definitional behaviour preserved) — the β/η equations of
  `34-data-match.md:69-75` hold definitionally, and the positive path unfolds to
  the transparent kernel Σ.
- **AC-OWNERSHIP** (ownership and obligations survive) — the build does **not**
  transfer `Ord Nat` or Pair-instance ownership and does **not** discharge the
  three named attached-proof conversions; the corresponding AC7-style controls
  (`pair-floor-does-not-transfer-ord-nat-or-pair-instance-ownership`,
  `pair-floor-does-not-discharge-attached-proofs`) stay green.
- **AC-NO-REGRESSION** — whole-suite green in CI (COORDINATION section 12). Local
  targeted checks only (`-p ken-elaborator`, `-p ken-cli`), never `--workspace`.

## Reviewers

Architect (identity provenance: the four names resolve to the exact recorded
pre-source ids through the closed inventories with zero allocation and zero trust
delta; the type and companion inventories are genuinely split rather than one
conflated list; no alias/registry/ambient/mixed-resolution route survives;
collisions close before allocation; a same-shaped source definition stays a
distinct identity; class-owner placement and the attached-proof obligations are
untouched) + language-qa (AC-TEN / AC-COMPANIONS / AC-ZERO-ALLOC /
AC-DISTINCT-IDENTITY hold on their own evidence; the enumerated AC-ROWS actually
flip and the survives-realization controls rerun green; AC-COLLISION,
AC-NEGATIVE-PROD, AC-BETA-ETA, AC-OWNERSHIP hold) + conformance-validator (the
row inventory is exactly the gated set — no row silently left RED and no row
flipped that the spec did not gate; counts and pins reproduced independently).

A genuine spec gap discovered here HARD-STOPS to the Architect rather than being
closed by a fallback route; the spec forbids inventing one.

## Capability tier

T1 — soundness-bearing identity and resolution work reviewed on a provenance
argument (reuse-not-allocate, closed-inventory resolution, collision closure
before allocation, no second identity), not a differential diff. It extends a
landed mechanism (`LANG-MOD-NAT-FLOOR-REALIZATION`) rather than inventing one, so
size M, not L.

## Sequencing

Lane-2 (language). Realizes the merged spec WP [[LANG-MOD-PAIR-FLOOR-PROVIDER]]
and completes the internal-provision prelude recut effort; **Lane 2 returns to the
z3 integration campaign after this lands** (operator 2026-08-26). It is the gate
on foundation [[CAT-GCD-REFACTOR]]'s cluster and on the catalog
`Order -> LawfulClasses -> Compare -> Pair` re-entry, neither performed here.
Ordinary exact-SHA gates apply; no realization is authorized ahead of them.
