# WP — LANG-MOD-STRICT-RESOLUTION (module/import WP-2): strict resolution for root-loaded package units — the soundness core

Language lane (module/import campaign [[LANG-MODULE-IMPORT-SYSTEM]]). One WP, one
branch `wp/LANG-MOD-STRICT-RESOLUTION`, one PR. Owner: language. Size: L. Gate:
none (no TCB growth; see Capability tier). Depends on: LANG-MOD-LOADER-ENTRY
(WP-1, landed `503817dcc`) — the roots entry this WP hardens. Blocks:
LANG-MOD-CATALOG-REALIZATION (WP-4). Ring order: WP-1 -> WP-3 -> WP-2 -> WP-4.

Source: Architect component framing `evt_hpnhqy1ex286` (WP-2), with the keying
REVISION `evt_xtscdw8r3q3k` (strict-all-root-loaded; drops the boundary-header
key). This is the SOUNDNESS CORE of the campaign: it is the WP that makes an
`import` mean something — that a bare name in a package unit resolves only
through what the unit actually brought into scope, and is `UnboundName`
otherwise.

Fixed inputs measured at `origin/main f554abfd4` (WP-3 landed; WP-1's loader
entry in place). M9 for WP-3 is doc-only (tracker + progress regen, no `crates/`)
so it does not move any code line cited below; release cuts from the then-current
`origin/main`.

## Objective

Make a root-loaded package unit resolve bare names STRICTLY: through the unit's
own locals, its explicit imports, kernel vocabulary, and the closed prelude floor
(`Bool`/`Char`/`List`) — and NOTHING else. A bare name that reaches neither is an
`UnboundName` surface error, not a silently-passed-through unresolved token. A
LEGACY single-file CLI unit is unchanged: it keeps the current passthrough
verbatim. The strict/legacy distinction is keyed on HOW the unit was loaded
(root-loaded via `elaborate_module_from_roots` = strict), threaded from the entry
so no global flag-day is needed.

## Fixed inputs (SETTLED, at `origin/main f554abfd4`)

1. The strict-vs-legacy KEY (Architect revision `evt_xtscdw8r3q3k`):
   strict-all-root-loaded. Every unit loaded through
   `elaborate_module_from_roots` (`crates/ken-elaborator/src/modules.rs`, the
   loader WP-1 routed catalog `ken check` through) resolves strict. A unit
   elaborated on the legacy single-file CLI path keeps passthrough. The
   boundary-header key from the original framing is DROPPED — the /spec candidate
   makes every root-loaded unit strict.
2. Fail-close seam 1 — `resolve_ref` (`modules.rs:182`). The passthrough is the
   `None => Ok(name.to_string())` arm (`:222`): on a scope miss it returns the
   bare name unresolved. The sibling `facade_only` arm just above (`:218`)
   already returns `ElabError::UnboundName` — the fail-close shape strict mode
   needs already exists here; strict mode routes the `None` arm to it instead of
   passthrough, unless the name is admitted by the floor (input 4).
3. Fail-close seam 2 — the value-level `elab.rs` RCon global lookup. The
   `RExpr::RCon` arm (`crates/ken-elaborator/src/elab.rs:3910`) does
   `cx.globals.get(name)` as its fall-through (`:3919-3926`, yielding
   `UnresolvedCon` on miss) after the local-dict and `Refl`/`Axiom`/sugar
   intercepts — this is the value-position ambient-global hit strict mode must
   close. The type-level `RType::RCon` lookup (`elab.rs:620`) is the sibling
   type-position seam; D0 confirms whether it too needs closing. In strict mode a
   name reachable via none of locals/imports/floor is `UnresolvedCon`/
   `UnboundName`, never an ambient global hit.
4. The admitted floor — `install_prelude_floor` (`modules.rs:83`): the closed
   prelude floor (`Bool`/`Char`/`List` per `spec/30-surface` §4) plus kernel
   vocabulary. Strict mode admits exactly locals + explicit imports + this floor;
   the floor-membership predicate that distinguishes an admitted bare name from an
   `UnboundName` is a D0 finding — there is no `is_prelude`/`env.rs` on current
   main, so the earlier framing's `env.rs:568-579` citation is stale.
5. The mode carrier. There is no per-unit strict flag threaded through today;
   this WP adds one, from the ENTRY (the roots-loader call vs the CLI single-file
   call) through `Scope` + the elaboration context to BOTH seams (2 and 3). This
   is the mechanism WP-1 deliberately did NOT add (WP-1 landed non-strict,
   behaviour-preserving).
6. The cross-cutting invariant + pin: flat-Σ / zero `trusted_base()` delta;
   `crates/ken-elaborator/tests/es3_modules_acceptance.rs`
   `module_elaborates_to_identical_flat_sigma` (`:28`). Strict resolution mints no
   kernel decl and changes no `GlobalId`; a resolved import name is the
   provider's existing `GlobalId`.

## Design judgment (front-loaded)

- **This is the soundness core; the review turns on an argument, not a diff.**
  The property is: in a root-loaded unit, a bare name resolves iff it is a local,
  an explicitly imported name, kernel vocabulary, or a floor member — and the
  resolution it gets is the provider's canonical id, never a fresh or ambient
  one. Over-acceptance (a name resolving that should be `UnboundName`) is the
  soundness failure this WP exists to close; under-acceptance (a legitimately
  in-scope name rejected) is a completeness bug. The frame's controls pin BOTH
  directions.
- **D0 buildability probe FIRST (Architect recommendation).** Before threading
  the mode, a D0 probe establishes: (a) the exact two fail-close seams and that
  the mode can be threaded to both without a wider refactor; (b) which current
  catalog/library units actually rely on the passthrough (the census that WP-4
  will migrate). D0 is a measurement/probe that does NOT flip enforcement, so it
  reds no catalog check and is releasable independently (sibling precedent:
  checker-soundness D0 and embedding-adequacy D0 both landed as probes on
  QA+Architect). D1 threads the mode and fail-closes both seams.
- **Fail-close by ROUTING to the existing UnboundName, not a new refusal.** Seam
  1 already produces `UnboundName` on the `facade_only` arm; strict mode sends
  the `None` arm to the same error unless the floor admits the name. Do not
  invent a parallel strict-resolution error taxonomy.
- **Legacy passthrough is UNTOUCHED.** A single-file CLI unit must resolve
  exactly as it does today (the `None => Ok(name.to_string())` arm stays live for
  legacy units). The mode gates which arm a unit takes; it does not delete the
  passthrough.
- **The floor is closed and small.** Strict mode admits kernel vocab + the
  prelude floor (`Bool`/`Char`/`List`) and nothing else — no ambient promotion,
  no implicit prelude beyond `install_prelude_floor`. Widening the floor is out
  of scope and would be a spec change.

## Deliverables

1. D0: a buildability/census probe — pins the two fail-close seams (2, 3), proves
   the mode can be threaded to both, and enumerates the catalog/library units that
   currently resolve any bare name via passthrough (the WP-4 migration census).
   Committed as a probe test; no enforcement flip.
2. D1: the strict-mode carrier threaded from the entry through `Scope` + the
   elaboration context to both seams; both seams fail-close to `UnboundName` for a
   root-loaded unit when the name is not a local, an explicit import, kernel
   vocab, or a floor member; legacy single-file units keep passthrough verbatim.
3. Tests: for a root-loaded unit — a bare name resolved via an explicit import
   (accepted, resolves to the provider's `GlobalId`); a floor member accepted; a
   kernel-vocab name accepted; a name reachable via none of these rejected with
   `UnboundName` at BOTH seams. For a legacy single-file unit — the same
   currently-passthrough name still resolves (passthrough retained). The flat-Σ
   pin extended, never weakened.

## Acceptance criteria (property + closure axis + loud failure)

- **AC-1 (strict rejects the ambient name — the soundness direction).** In a
  root-loaded unit, a bare name that is neither local, explicitly imported,
  kernel vocab, nor a floor member is `UnboundName` — at seam 1 (`resolve_ref`)
  AND seam 2 (`elab.rs` RCon lookup). CONTROL: the SAME name in the SAME unit
  made reachable by adding its explicit `import` resolves and elaborates — so the
  gate is shown to discriminate on import-provenance, not to reject-all. This is
  the over-acceptance closure; a mutation restoring the passthrough on the `None`
  arm for a root-loaded unit must red this.
- **AC-2 (strict admits exactly the floor + imports — the completeness
  direction).** A floor member (`Bool`/`Char`/`List`), a kernel-vocab name, and
  an explicitly-imported name each resolve in a root-loaded unit. CONTROL: drop
  the floor admission and the floor-member case reds (proving the floor test is
  load-bearing, not incidentally satisfied by another path).
- **AC-3 (legacy passthrough retained).** A single-file CLI unit resolves a
  currently-passthrough bare name exactly as at base `f554abfd4` — strict mode
  does not touch the legacy path. CONTROL: this is the differential that proves
  the mode is keyed per-load, not a global flag-day.
- **AC-4 (imported name is the provider's id).** A name brought in by `import`
  resolves to the provider's existing `GlobalId`, minting no new id — pinned so a
  future change that re-mints an imported name reds.
- **AC-5 (cross-cutting invariant).** Zero `trusted_base()` delta; the flat-Σ pin
  `module_elaborates_to_identical_flat_sigma` (`es3_modules_acceptance.rs:28`)
  stays green (extend, never weaken).
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION §12). Local builds
  targeted `-p ken-elaborator` (and `-p ken-cli` if the entry-mode wiring
  surfaces there), never `--workspace`. See Sequencing for the WP-4 co-gate on
  whole-catalog CI-green.

## Contention

Writes land in `crates/ken-elaborator/src/modules.rs` (resolve_ref `None` arm +
the mode on `Scope`), `crates/ken-elaborator/src/elab.rs` (the RCon lookup +
mode on the elaboration context), the entry sites (roots loader vs CLI
single-file), plus tests. WP-1 and WP-3 both touched `ken-elaborator` and are
LANDED (`503817dcc`, `f554abfd4`), so there is no live language contention. The
runtime priority-1 lane's in-flight slice is `crates/ken-runtime` only — no
overlap. The Steward routes both language and runtime merges to sequence any
window.

## Capability tier (§4h)

T1 (deep reasoning). Not a mechanical diff: the deliverable is a soundness
property (strict resolution admits exactly the intended set at two independent
seams, with the resolved id canonical), and its review is an argument about
over- vs under-acceptance, not byte-faithfulness. The Architect reviews this
hardest in the campaign. No TCB growth — resolution is elaborator-level and mints
no kernel decl — so it is not an operator TCB gate; the merge is the usual
Architect + CV + resolved Decision + full CI. Size L.

## Reviewers

- Architect — component fit and SOUNDNESS: the floor admits kernel + prelude and
  nothing else; both seams fail-close; the mode is keyed strict-all-root-loaded
  and threaded from the entry; the legacy passthrough is untouched; no ambient
  promotion; imported names keep the provider's id.
- conformance-validator — strict-vs-legacy is CV's discriminator-pair territory
  (`evt_2wejn8hekr4qw`): an accepted-via-import positive against a rejected
  ambient name, and the legacy-retained differential, each shown discriminating
  by an independent byte-restored mutation.

## Do-not guards

- Do NOT silently pass an unresolved bare name through in a root-loaded unit —
  that is the over-acceptance this WP closes. Fail-close to `UnboundName`.
- Do NOT touch the legacy single-file passthrough — a single-file unit resolves
  as at base.
- Do NOT widen the floor beyond `install_prelude_floor` (kernel vocab +
  `Bool`/`Char`/`List`); no ambient/implicit promotion. Widening is a spec
  change, out of scope.
- Do NOT re-mint or re-key an imported name — it resolves to the provider's
  existing `GlobalId`.
- Do NOT introduce a parallel strict-resolution error taxonomy; route to the
  existing `UnboundName`.
- Do NOT weaken `module_elaborates_to_identical_flat_sigma`.
- Do NOT rewrite any catalog file (that is WP-4).

## Sequencing (Steward-owned) — the WP-4 co-gate

Released after WP-3's merge (landed `f554abfd4`); the single-threaded language
ring takes WP-2 next. D0 (the buildability/census probe) is releasable and
mergeable on its own — it flips no enforcement and reds no catalog check.

The D1 enforcement flip CO-GATES with WP-4 (Architect `evt_xtscdw8r3q3k`):
making every root-loaded unit strict turns any catalog unit that currently
resolves a cross-package name via passthrough into an `UnboundName` failure until
WP-4 adds the explicit imports. So WP-2's WHOLE-CATALOG CI-green depends on WP-4's
catalog census migration. The D0 census (deliverable 1) measures exactly which
units that is. Two consequences the Steward carries:

1. The exact co-land mechanism for D1 (co-land WP-2-D1 + WP-4 in one window, vs
   land the strict machinery with enforcement gated until WP-4) is confirmed with
   the Architect at D1 framing — D0's census is the input to that call.
2. WP-4 is additionally gated on the Or/Inl/Inr fork the operator owes
   (`evt_6b9wrt1kwswcp`). So the D1/WP-4 co-land is downstream of an operator
   decision; D0 is not, and proceeds now. The Steward surfaces the D1<->WP-4<->Or
   sequencing to the operator when D0's census lands, rather than blocking D0 on
   it.

This WP does NOT change the loader entry (WP-1, landed), the `pub`-eligibility
gate (WP-3, landed), or rewrite any catalog file (WP-4).
