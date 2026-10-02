---
id: CAT-TRANSPORT-COMBINATOR-CONSOLIDATION
title: "Transport's header lists subst as public API but declares it private, and eleven body-identical private copies of Transport's combinators sit in seven packages. Publish subst at zero TCB, correct the header for cast, and retire the eleven copies to Transport"
status: active
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [CAT-NAT-SUB-ADD-CANCEL]
blocks: []
github: null
origin: "Architect nomination evt_5bz9sfwt86h24 on Steward evt_49m8kqg38v2gg: next L3 proof-backfill slice (operator 2026-09-13), catalog §2a; the subst finding is the follow-on in dec_cj27ejft0j62. Steward-filed per COORDINATION section 2."
---

# Packages reuse Transport's combinators

## Objective

A client transports along an equality with the public `subst`. No catalog
package keeps a private copy of `subst`, `cong`, `sym` or `trans`.

## Settled inputs (Architect `evt_5bz9sfwt86h24`, read at `10daa9242`)

- **Transport** (`Core/Logic/Transport.ken.md`).
  - `fn subst` (`:44`) is private, while the header's Public API line
    (`:152`) lists `subst`, `cong`, `cast`, `sym` and `trans`.
  - `cong`, `sym` and `trans` are `pub theorem` (`:53`, `:71`, `:78`) over
    native `Eq ty`, with `(ty : Type)`.
  - `fn cast` (`:65`) has no consumer in `catalog/`.
- **The eleven copies.** Each is body-identical to Transport's combinator,
  with the same argument order.

  | Package | Copy | Becomes |
  |---|---|---|
  | Algorithm/Numeric/Gcd (after `CAT-NAT-SUB-ADD-CANCEL`) | `subst` | `subst` |
  | Algorithm/Searching/OrderedSearch `:96`, `:99` | `search_sym`, `search_trans` | `sym`, `trans` |
  | Capability/Filesystem/Path/Posix `:152`, `:155`, `:160` | `path_equal_sym`, `path_equal_trans`, `path_equal_cong` | `sym`, `trans`, `cong` |
  | Capability/Filesystem/Path/Posix `:1498` | `path_equal_cong0` | deleted, unused |
  | Application/Configuration/Decoder `:552`, `:558` | `env_config_sym`, `env_config_trans` | `sym`, `trans` |
  | Application/Input/Schema `:460` | `schema_sym` | `sym` |
  | Capability/Parsing/Decoder `:727` | `decoder_equal_chain` | `trans` |
  | Data/Collections/Deque `:73` | `deque_cong` | `cong` |

- **Consumers.** Outside their own packages, the retired names occur only in
  `crates/ken-elaborator/tests/cat_deque_closeout.rs` (one reference). No
  kenfmt byte pin covers a migrated package. Steward grep on `7ba09867b`:
  `kenfmt_signature_layout.rs` pins only LawfulFunctors and Order.
- **Spellings.** Posix's copies use `Eq a` and the rest use `Equal`. Both
  already meet Transport elsewhere in the catalog.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Transport.** `fn subst` becomes `pub fn subst`, with name and body
   unchanged. The header's Public API line says `cast` is package-local.
2. **The seven packages.** Delete the eleven copies and rewrite their call
   sites to Transport's `subst`, `cong`, `sym` and `trans`. Add those names
   to each package's Transport import, or add the import.
3. **Tests.**
   - Migrate `cat_deque_closeout.rs`'s reference.
   - Add `cat_transport_pub_export` in `r_layer_tests`.
   - Add one whole-root absence test.

Out of scope, deliberately:

- `decoder_equal_after_left_replacement` (Parsing/Decoder `:612`), a
  `trans ∘ sym` composite rather than a copy;
- FoKripke's `fok_cong` and its sym- and trans-shaped helpers;
- the specialised `J` uses in LawfulClasses, Arguments, Parsing and Order.

## Acceptance

- **AC-1.** `ken check` passes on Transport and the seven packages, and
  `subst` is public.
- **AC-2 (falsifiers, committed as test rows).**
  - A clean-environment client imports only `Core.Logic.Transport (subst)`
    and transports an open `Nat`-indexed family. It fails on base because
    the name is private.
  - A swapped-endpoint mutant uses `p : Eq x y` to go `fam y → fam x` and is
    rejected with a typed `TypeMismatch`, not by name resolution.
  - Each migrated package resolves `cong`, `sym`, `trans` and `subst` to
    Transport's loader identity, checked as on `CAT-NAT-SUB-ADD-CANCEL`.
  - The absence test lists the eleven retired names in their packages. It
    is red on base, so a re-added copy goes red.
- **AC-3.**
  - `trusted_base()` is unchanged.
  - The catalog census verdicts are unchanged.
  - These stay green: `surface_transport_acceptance.rs`,
    `lang_mod_catalog_realization.rs`, `cat_deque_closeout.rs`,
    `cat_bsearch_acceptance.rs`, `cc7_argparse_acceptance.rs`,
    `cc8_env_config_decoder_acceptance.rs`, the five
    `cat_tier_{d,e}_*_import` r-layer rows and the Gcd acceptance target.
- **AC-4 (readability).** The migrated packages follow
  `docs/program/07-catalog-style-guide.md` and
  `agent/playbooks/tools/write-ken.md`.
  - Each rewritten call site reads at least as clearly with Transport's name
    as with the retired local one. Where a retired name carried domain
    vocabulary the bare combinator loses, the proof says so in prose or with
    a named intermediate, not with a new wrapper.
  - Branch placement and effect order are unchanged.
  - The plan reserves a final exposition pass after the proofs close.
  - Foundation QA reviews local naming and formatter layout independently,
    with a positive layout check of each touched form. `ken fmt --check`
    alone does not count as that review.

## Stop conditions

- A copy that is not body-identical, or whose argument order differs.
- Any new primitive, postulate or axiom, or an import cycle.
- A consumer of a retired name that was not counted: stop and name it.
