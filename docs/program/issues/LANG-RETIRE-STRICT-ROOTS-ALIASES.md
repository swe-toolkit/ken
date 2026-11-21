---
id: LANG-RETIRE-STRICT-ROOTS-ALIASES
title: "The elaborator has one resolution mode, but ElabEnv and modules still export elaborate_module_from_roots_strict and elaborate_module_from_roots_strict_v1, aliases that forward to the ordinary roots entries. Their names advertise a second mode that no longer exists, and 37 elaborator files plus other crates and three conformance seeds call through them. Delete both aliases and move every caller to the ordinary entry"
status: active
owner: language
size: M
tier: T2
gate: architect
depends_on: [LANG-DELETE-LEGACY-RESOLUTION]
blocks: []
github: null
origin: "Operator 2026-09-25: one resolution mode (L2 objective). LANG-DELETE-LEGACY-RESOLUTION I4 (fcd7e8156) deleted Legacy and left the Strict-named entries as same-behaviour aliases. PRINCIPLES subsume-don't-proliferate: one behaviour, one entry. Steward-filed per COORDINATION section 2."
---

# One roots entry, not two names for it

## Objective

`elaborate_module_from_roots_strict` and
`elaborate_module_from_roots_strict_v1` no longer exist. Every caller uses
`elaborate_module_from_roots` or `elaborate_module_from_roots_v1`, and the
same behaviour results.

## Settled inputs (main `24f47951f`; anchors re-checked at release)

- **The aliases.** `modules.rs:3100` (`_strict`) forwards to
  `elaborate_module_from_roots`, and the `_strict_v1` function just above it
  forwards to `elaborate_module_from_roots_v1`. `ElabEnv` re-exports both at
  `lib.rs:798` and the method after it. Each doc comment calls itself an
  alias.
- **The callers.** 196 call sites across 37 files under `crates/`,
  including `ken-elaborator` tests and 44 hits in other crates. Three
  conformance seeds name the alias as the route a case runs through:
  `surface/modules/seed-modules.md:1590`,
  `surface/modules/seed-strict-package-homes.md` (`:24`, `:160`, `:297`,
  `:340`) and `surface/taxonomy/minimality.md:188`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: Spec child (2026-11-21)

Language implementer `evt_4wne1edv0x30g` stopped at the `spec/` condition:
`spec/30-surface/33-declarations.md:314` and `30-taxonomy.md:766` name the
`_strict` aliases as live entries. Spec Leader `evt_6fp5mjep6hwtj` confirmed.
Steward ruling `evt_70xphqdhcqhtq`: scope adds a Spec child on those two
paths only, aligning each sentence with the surviving roots entries with no
rule change. The Language Leader assembles it with WIP `e879141da`. The
Architect votes on `/spec` as well, and AC-1's sweep extends to `spec/`.

## Scope

The two alias definitions and their `ElabEnv` methods, every `crates/`
caller, and the three seeds' route names (CV votes on `conformance/`). No
resolution rule, error variant, return type or `spec/` change. A caller that
relies on the alias's narrower return type (`Vec<GlobalId>` versus
`_v1`'s results) keeps that type by calling the matching ordinary entry.

## Acceptance

- **AC-1.** `git grep -n 'from_roots_strict'` over `crates/` and
  `conformance/` returns nothing on the candidate.
- **AC-2.** Each changed test keeps its assertion text and expected values;
  only the called entry's name changes. QA spot-checks the five files with
  the most changed sites (`modules.rs`, `lang_mod_pair_floor_realization.rs`,
  `lang_mod_nat_floor_realization.rs`,
  `lang_roots_loader_local_instance_dict_scope.rs`,
  `lang_mod_strict_resolution_d1.rs`) for a behaviour change.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A caller observes a behaviour difference between an alias and its
  ordinary entry. That is a defect in the I4 claim, not a rename; report it
  to the Architect.
- The change needs a `spec/` edit.

Checks: 4 fired. I read both alias bodies on `24f47951f`; each is a
one-line forward. 3 fired. The caller count is `git grep -c` over `crates/`,
and the conformance hits are the full `conformance/` sweep. 8 fired. AC-1
is a total sweep rather than a sample, so a missed caller fails to compile
or fails AC-1.
