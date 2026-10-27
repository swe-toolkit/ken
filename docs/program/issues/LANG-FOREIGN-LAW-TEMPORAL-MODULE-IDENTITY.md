---
id: LANG-FOREIGN-LAW-TEMPORAL-MODULE-IDENTITY
title: "A foreign, law or temporal declaration inside a module keeps a bare globals key, so a client that never imported it uses it under Legacy and the direct API while Strict refuses. Give these kinds the module-qualified, scope-bound identity that data, class and record declarations have"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-RECORD-DECL-MODULE-IDENTITY]
blocks: []
github: null
origin: "Architect residual (b) in the LANG-RECORD-DECL-MODULE-IDENTITY AC-0 (evt_4jppeshybh3q7), measured on the patched scratch: the next consumer of the same is_qualifiable predicate. Serves the L2 objective of one resolution mode. Steward-filed per COORDINATION section 2."
---

# Foreign, law and temporal declarations have module identity

## Objective

A `foreign`, `law` or `temporal` declaration inside a module is bound by
the module's scope under its qualified name. A client that does not
import it cannot use it in any resolution mode.

## Settled inputs (main `a4958ae83`, Architect `evt_4jppeshybh3q7`)

- **The predicate.** `is_qualifiable` (`crates/ken-elaborator/src/modules.rs:3422`)
  lists view, space, let, prop, theorem, axiom, attached proof, data,
  explicit data, type alias and class. LANG-RECORD-DECL adds record.
  `ForeignDecl`, `LawDecl` and `TemporalDecl` are absent, so they enter
  `globals` under their bare name from inside a module.
- **The leak, measured.** A private `foreign leak_f` in unit A is used
  unimported by a client: Legacy accepts it and Strict gives
  UnboundName. `law LeakLaw` and `temporal leak_t` also enter `globals`
  bare from module A.
- **The pins.** `tests/lang_mod_nat_floor_realization.rs` pins
  `LocalLaw` (`:783`), `LocalForeign` (`:807`) and `LocalTemporal`
  (`:813`) bare. LANG-RECORD-DECL migrates `LocalRecord` the same way.
- **Pub eligibility.** `pub_eligibility` refuses all three
  (`crates/ken-elaborator/src/parser.rs:5787-5789`). Whether each kind
  introduces a name with a module-interface identity (specs 32 §1 and
  33 §4.1), and so becomes `pub`-eligible, is the Architect's AC-0.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

**AC-0 ruled (`evt_4dc9n9em4kctp`).** Foreign is qualified, scope-bound
and pub-eligible. Law, including its minted field keys, and temporal are
qualified and scope-bound, and both stay pub-ineligible. An unimported
temporal gets the per-mode surface class instead of today's
`Internal("no type for global ...")`. Start from
`local/architect-shared/flt-d0/probe.diff` on main after RECORD-DECL, and
migrate the three pins the ruling names.

Before the implementer starts, the Architect rules, per kind, whether it
is qualified and scope-bound like a record, whether `pub` becomes
eligible, and how the native stable symbol of a `foreign` is kept
independent of its qualified key. Then the change in `modules.rs` and,
if ruled, `parser.rs`, with the bare-key pins migrated.

## Acceptance

- **AC-0 (Architect).** The per-kind ruling and a census of bare-key
  consumers, by name and by mechanism, on the landed base.
- **AC-1.** For each kind, an unimported use from another module is
  refused in Strict roots, Legacy roots and the direct API, with the
  per-mode error class. An imported or in-module use still resolves.
- **AC-2 (control, amended `evt_7j2wfap04sdfe`).** (a) Two modules that
  each declare the same-spelled `foreign f` both check, with keys `M.f`
  and `N.f`. Both bindings carry the source symbol and library, and there
  is no bare key. (b) `stable_symbols_for_env` over that env gives two
  distinct declaration symbols, with no DuplicateStableSymbol. (c) An
  `[FS]` foreign called from a pure `fn` in the same module is refused
  with EffectEscapes, the same holds through an imported `pub foreign`,
  and a `proc ... visits [FS]` caller is accepted, in all four modes. The
  LANG-RECORD-DECL and LANG-CLASS-IDENTITY rows are unchanged, and
  `trusted_base()` is unchanged as an identity set. The original first
  sentence was vacuous: native lowering refuses every foreign today.
- **AC-3 (mutation, QA).** Removing one kind's arm from the
  qualification makes that kind's unimported row accept again under
  Legacy.

## Stop conditions

- A spec change, or a kernel or trust change.
- A change to a native symbol, library name or ABI word.
