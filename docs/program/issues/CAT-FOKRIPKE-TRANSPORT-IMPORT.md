---
id: CAT-FOKRIPKE-TRANSPORT-IMPORT
title: "FoKripke keeps three private, alpha-identical copies of Transport's cong, sym and trans. Import Transport's and retire the copies, preloading Transport at the 20 flat-source test consumers"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-TRANSPORT-COMBINATOR-CONSOLIDATION]
blocks: []
github: null
origin: "Architect nomination evt_4n52fvm5rb8tx on Steward evt_7pkmgh42h07yt: next L3 proof-backfill slice (operator 2026-09-13), catalog §2a; the FoKripke carry left out of CAT-TRANSPORT-COMBINATOR-CONSOLIDATION. Steward-filed per COORDINATION section 2."
---

# FoKripke reuses Transport's combinators

## Objective

`Tooling/Verification/FoKripke.ken` proves its equalities with
`Core.Logic.Transport`'s `cong`, `sym` and `trans`, and keeps no private copy
of them.

## Settled inputs (Architect `evt_4n52fvm5rb8tx`, read at `e893ecb7a`)

- **The copies.** Each is private, alpha-identical to the Transport export,
  and takes the same argument order.

  | Copy | Becomes | References outside the definition |
  |---|---|---|
  | `fok_cong` `:1308` | `cong (ty ty2 x y f p)` | 24, first `fok_nat_eq_sound` `:1345` |
  | `fok_eq_sym` `:2845` | `sym (ty x y p)` | 15, first `fok_absurd_right_intro` `:2867` |
  | `fok_eq_trans` `:2848` | `trans (ty x y z p q)` | 17, first `fok_absurd_right_intro` `:2862` |

- **Kept.** `fok_cong2` (`:1313`) has no pub provider in the catalog.
  `fok_false_elim` (`:1677`) and `fok_option_domain_iff` (`:3025`) are
  specific to FoKripke. The `fok_subst_*` functions are first-order term
  substitution, not equality transport.
- **Spellings.** FoKripke states its goals with the prelude alias `Equal`
  (`prelude.rs:802`). Transport already discharges `Equal` goals in landed
  code (`LawfulClasses.ken.md:1846`). FoKripke has no bare `cong`, `sym` or
  `trans` definition or binder.
- **Consumers.**
  - 20 test files in `crates/ken-elaborator/tests/` read the file with
    `include_str!` and call `elaborate_file` on that flat source after
    `catalog_or::load_core_logic_or(&mut env)`. Together they have 33 load
    sites. A flat `elaborate_file` resolves an `import` only against
    providers already loaded, so every such site must also preload
    Transport. In delivered vocabulary, for
    `crates/ken-elaborator/tests/support/catalog_or.rs`:

    ```rust
    pub fn load_fokripke_providers(env: &mut ElabEnv) {
        load_core_logic_or(env);
        env.elaborate_module_from_roots_strict(&[catalog_root()], "Core.Logic.Transport")
            .expect("Core.Logic.Transport must load through strict catalog resolution");
    }
    ```

  - Two files have more elaborations than loads:
    `..._wrong_sort_controls` (1 load, 4 elaborations) and
    `..._freshness_guard_mutation_proof` (1 load, 2 elaborations).
  - The strict-floor census `lang_mod_strict_resolution_d0.rs:1144` lists
    `Tooling.Verification.FoKripke` as clean. Transport is already in that
    clean set.
  - None of the three names occurs in `crates/*/src`, `conformance/`,
    `examples/`, `scripts/` or `docs/`. `fo_kripke.rs`
    `with_catalog_globals` resolves none of them.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **FoKripke.** Add `import Core.Logic.Transport (cong, sym, trans)` next to
   `import Core.Logic.Or (Or, Inl, Inr)` (`:16`). Delete the three copies and
   rewrite their 56 references to the Transport names.
2. **Loader.** Add `load_fokripke_providers`. Switch every
   `load_core_logic_or` call that precedes a FoKripke elaboration to it. The
   one caller that loads no FoKripke source stays unchanged.
3. **Falsifiers**, in `r_layer_tests/cat_transport_pub_export.rs` as landed
   by CAT-TRANSPORT:
   - add the retirement row `("Tooling.Verification.FoKripke", &["fok_cong",
     "fok_eq_sym", "fok_eq_trans"])`;
   - add the identity witnesses `("fok_nat_eq_sound", "cong")`,
     `("fok_absurd_right_intro", "sym")` and
     `("fok_absurd_right_intro", "trans")` to
     `migrated_proof_bodies_use_preloaded_transport_global_identities`.

## Acceptance

- **AC-0 (measure; no edit).** At the frame SHA, re-measure:
  - the copies, their reference counts and the absence of a colliding
    spelling;
  - each load site and each elaboration in the two files that have more
    elaborations than loads, classified as needing the preload or not.
- **AC-1.** `ken check` and `ken fmt --check` pass on `FoKripke.ken`. All 20
  consumer suites are green, and the loader call is their only change. That
  includes their `trusted_base` before/after pins and the five-constructor
  and three-variant counts.
- **AC-2 (falsifiers).**
  - The retirement row is red on base.
  - Restoring a local `fok_eq_sym` turns it red.
  - Each identity witness references Transport's preloaded GlobalId.
- **AC-3.** The strict census passes 1/1, unchanged. `trusted_base()` has no
  delta.

## Stop conditions

- A copy that is not alpha-identical, or whose argument order differs.
- A consumer that loads FoKripke by a route other than the 20 counted: stop
  and name it.
- Any new primitive, postulate or axiom, or an import cycle.
