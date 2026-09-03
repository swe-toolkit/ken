---
id: CAT-MIGRATE-TIER-B-CLASSES
title: "Scaffold-retirement Tier B (class-owner relocation + primitive-instance consolidation): relocate the three orphaned primitive DecEq instances (UInt8, Bytes from BytesKeys; String from StringKeys) with their eq/sound/complete wiring and injectivity certificates into the class owner LC (LawfulClasses); consolidate EmptyDec's byte-identical duplicate class DecEq / fn bool_eq / instance DecEq Bool into an import from LC; and give the three scaffolded modules (BytesKeys, StringKeys, EmptyDec) real import blocks so they elaborate standalone with zero fixture-scaffolding dependence. LC/LF/EC are already CLEAN (census fold), so no Core.Classes module-migration remains — the tier is the DecEq relocation + the three consumers' standalone-ification."
status: active
owner: foundation
size: M
gate: none
tier: T1
depends_on: [CAT-MIGRATE-TIER-A-PROVIDERS, CAT-MIGRATE-TIER-B-PROVIDERS]
blocks: [CAT-SCAFFOLD-RETIREMENT]
github: null
origin: "Steward, 2026-09-03. Tier B of the scaffold-retirement migration (parent CAT-SCAFFOLD-RETIREMENT; Architect 5-tier DAG decomposition evt_2e0pee5jxzv07). Carries the operator ruling 2026-09-02 Flag 2 (verbatim 'Move DecEq UInt8, etc to LC'; lanes.md OPERATOR RULING 2026-09-02 block, commit 43f1c20a2): the orphaned primitive-type DecEq instances RELOCATE with their wiring to LC (the class owner), NOT a new prelude; BytesKeys/StringKeys become pure consumers; EmptyDec's duplicate local class/fn/instance CONSOLIDATE into LC. Fixed inputs measured at origin/main f01266423 (Steward measurement 2026-09-03): the three instances + their spans + helper spans + the EmptyDec byte-identity + the per-module scaffolding verdicts are in Fixed Inputs below. The tier's LF/EC/LC module-migration FOLDED OUT — all three of Core.Classes' modules already carry real import blocks (LF via the just-landed CAT-LAWFULFUNCTORS-STANDALONE-IMPORT, 6c95c6ff3), so the only scaffolding to retire in this tier is in BytesKeys, StringKeys, EmptyDec. Re-measure at your build SHA via the STEP-2 census (D0); a line number decays."
---

> # RECUT (Architect evt_21c0cdvnmv3f3 + Steward evt_7bzffq1q90rr8, 2026-09-03) —
> # this node is the RELOCATION SUCCESSOR, gated on the provider-publication
> # predecessor [[CAT-MIGRATE-TIER-B-PROVIDERS]] (P). P HAS LANDED (squash
> # 7722f4c26 on main; the Steward re-released this node active 2026-09-03). This
> # banner supersedes the census-fold / Fixed-Inputs claims below.
> #
> # POST-P SURFACE (re-measured by Steward at origin/main a7f1fdfd8; D0 reconfirms):
> # LC now carries `pub class DecEq a` (:75) and `pub fn bool_eq` (:329) — the
> # relocated instances resolve their class from LC's published surface, no longer
> # ambiently. StringBijection publishes `pub theorem string_to_list_char_injective`
> # (:18; axiom string_to_list_char_retraction :15 stays private) — so DecEq String
> # is NO LONGER deferred and its injectivity cert has a real provider. LC still
> # imports Transport `(cong, sym)` (:47) — the `trans` piece for the UInt8 cert is
> # the live design judgment (below).
> #
> # FOLD-IN (P's carry — Architect Eq-prose ruling evt_46yd7nc0mbhqh, carried by
> # foundation-leader + CV flag + Adversary top-level sweep): LC's "Public API"
> # prose (:2281-2286) lists `class Eq` as public, but `class Eq` (:61) is
> # deliberately PRIVATE and has no consumer — do NOT publish it; REMOVE it from the
> # prose and sweep the whole Eq entry to actual loader visibility. This folds into
> # THIS WP (it already edits LC); no separate node. Adversary confirmed `class Eq`
> # :61 is the ONLY mis-declared top-level NAME (IsTrue/bool_or/class Ord/leq_nat/
> # class DecEq/bool_eq all genuinely pub); the Eq INSTANCE lines (`instance Eq Int`
> # :205, `instance Eq Bool` :380) are the separate visibility sweep in the carry.
> # See deliverable D-EQ-PROSE + AC-EQ-PROSE-ACCURATE below.
>
> The foundation D0 (evt_5bt85erc05fa4) found the relocation consuming two
> UNPUBLISHED provider surfaces (LC's private `class DecEq`/`bool_eq`;
> StringBijection's private, scaffolded `string_to_list_char_injective`) and one
> census error (EC not clean). The Architect converged on the RECUT (over its own
> crossed rescope-in-place evt_4rvss5eaqqbzw, WITHDRAWN). Net structure:
>
> - PREDECESSOR P = [[CAT-MIGRATE-TIER-B-PROVIDERS]] (active, released): publishes
>   (a) LC `class DecEq` + `fn bool_eq` and (c) StringBijection's cert + off-scaffold
>   clean-ification. This node's `depends_on` now includes P.
> - THIS node (recut) = the WHOLE relocation, gated on P: DecEq UInt8 + Bytes +
>   STRING (String is NO LONGER deferred — P publishes StringBijection's cert) into
>   LC + EmptyDec consolidation (LC now exports DecEq/bool_eq, so the byte-identical
>   duplicate becomes an import) + BytesKeys/StringKeys/EmptyDec standalone. The
>   `trans` "design judgment" below is a NON-ISSUE (Architect Finding 4: LC has no
>   bare top-level `trans`, only namespaced `proof trans for <rel>`; change LC's
>   selective import :47 `(cong, sym)` -> `(cong, sym, trans)`, no proof edit, no
>   Transport export-surface change — the :168-180 note is historical).
> - OUT of this node (census error): EC (EffectfulClasses) standalone-cleanness on
>   private LF `Functor`/`comp`/`idf`/`list_map` — orthogonal to the DecEq chain, a
>   SEPARATE Core.Classes node the Steward frames off the critical path (not a
>   regression fix; EC still elaborates in the full catalog build via ambient
>   resolution).
>
> Class-uniformity stays the Architect's candidate criterion (after D2, exactly one
> `class DecEq` catalog-wide, EmptyDec's duplicate GONE not shadowed). D0 count:
> 1st D0 stop on this WP. The Fixed Inputs below stay as measured reference; where
> they conflict with this banner (LC/EC clean; String deferred; `trans`), the
> banner governs.
>
> # Tier B of the scaffold-retirement migration: the DecEq class-owner
> # relocation + the three primitive-instance modules' standalone-ification.
> # The class-owner relocation GATES every DecEq/class consumer in Tiers C-E.
>
> The Architect's bottom-up DAG (evt_2e0pee5jxzv07) places Core.Classes above the
> primitive providers (Tier A) and below the value/capability/serialization
> consumers (Tiers C-E). The operator ruled (2026-09-02, Flag 2) that the three
> orphaned primitive DecEq instances belong with their class owner, not in a new
> prelude, and that EmptyDec's duplicate class machinery consolidates into that
> owner. This node lands that relocation + consolidation and, in the same pass,
> retires fixture scaffolding from the three modules it touches so they elaborate
> standalone. LC/LF/EC are already clean; the census (D0) confirms that fold.

## Fixed inputs (measured at origin/main `f01266423`; re-measure at your build SHA)

**LC = the class owner `catalog/packages/Core/Classes/LawfulClasses.ken.md`**
(CLEAN — real imports :42-50). Owns `class DecEq a` (:75-79), `fn bool_eq`
(:329-337), and instances `DecEq Int` (:199), `DecEq Bool` (:420-442), `DecEq
Char` (:623-627), `DecEq (Pair a b)` (:1613), `DecEq (List a)` (:2156). Imports
Transport as `(cong, sym)` (:47) and calls out a known cross-file `trans`
collision at :174 — see the design judgment below.

**The three DecEq instances to relocate, with their wiring (all in SCAFFOLDED
modules with NO import block — every external symbol resolves ambiently today):**

- `catalog/packages/Data/Binary/BytesKeys.ken.md`
  - `instance DecEq UInt8` (:52-56) = `eq/sound/complete = uint8_deceq_eq /
    uint8_deceq_sound / uint8_deceq_complete`. Helpers that move with it:
    `fn uint8_deceq_eq` (:31-32), `theorem uint8_deceq_sound` (:34-42),
    `theorem uint8_deceq_complete` (:44-50), `theorem uint8_to_int_injective`
    (:12-29, the injectivity cert `sound` rides — this is what needs `trans`).
  - `instance DecEq Bytes` (:109-113), attached-proof form. Helpers:
    `fn bytes_deceq_eq` (:87-88), `proof sound for bytes_deceq_eq` (:90-99),
    `proof complete for bytes_deceq_eq` (:101-107), `theorem
    bytes_to_list_injective` (:62-85). **Bytes DEPENDS ON UInt8** —
    `bytes_deceq_eq` calls `uint8_deceq_eq` and the proofs reference
    `DecEq_instance_UInt8`; the two blocks relocate together.
- `catalog/packages/Data/Text/StringKeys.ken.md`
  - `instance DecEq String` (:34-38), attached-proof form. Helpers:
    `fn string_deceq_eq` (:10-13), `proof sound` (:15-24), `proof complete`
    (:26-32). (The sibling `Ord String` at :40-90 does NOT feed `DecEq String`
    and is out of the relocation.)

**EmptyDec `catalog/packages/Core/Logic/EmptyDec.ken.md` — byte-identical
duplicate to consolidate (diff-verified against LC):** local `class DecEq`
(:88-92 == LC :75-79), local `fn bool_eq` (:94-102 == LC :329-337), local
`instance DecEq Bool` (:104-126 == LC :420-442). Consumed locally by `fn
dec_eq_decides` (:156-172) and the section-3 examples (:182, :185). EmptyDec also
inlines `theorem sym`/`theorem trans` (:137-144) "copied from Transport.ken for
self-containment" — retire these for the real Transport import too.

**Scaffolding verdicts (Steward measurement, D0 re-confirms):** LC CLEAN, LF
(`LawfulFunctors`) CLEAN, EC (`EffectfulClasses`) CLEAN — **all three fold out,
no Core.Classes module-migration in this tier.** BytesKeys SCAFFOLDED (no
imports), StringKeys SCAFFOLDED (no imports), EmptyDec SCAFFOLDED (inlined
re-declarations despite one clean import). These three are the tier's retirement
targets.

## The design judgment, front-loaded: the `trans` import collision (Architect)

LC deliberately imports Transport as `(cong, sym)` and documents a cross-file
`trans` collision at LC :174. `uint8_to_int_injective` (rides on `DecEq UInt8`'s
`sound`) uses `trans`. So the UInt8 relocation forces LC to bring `trans` into
scope, which is the exact collision LC avoided. **This is the one piece that is
not a mechanical move and is why the tier is T1 with the Architect as required
reviewer.** Resolve it within LC's existing import discipline (a qualified or
locally-aliased `trans`, or the collision's already-documented resolution) WITHOUT
a scaffolding workaround and WITHOUT weakening any existing LC proof. **If it
cannot be resolved without changing the Transport export surface or an existing
LC proof, that is a HARD STOP to the Architect** (a design fork on the Transport
import surface, not a Foundation-owned mechanical migration).

## Deliverables

- **D0 — STEP-2 census at the build SHA.** Reproduce the Fixed Inputs above at
  the SHA you build on: confirm LC/LF/EC still CLEAN (fold out), the three
  instance spans + helper spans, the EmptyDec byte-identity, and — for each of
  BytesKeys/StringKeys/EmptyDec — the EXACT set of ambient external symbols each
  needs a real import for, and that every one of those symbols' providers is
  already published (Tier A + the clean Core.Classes/Logic modules). A symbol
  whose provider is NOT yet published is a scoped fold-out with a cited reason
  (that module's full clean-ification defers), NOT a hard stop; a MISSING class
  provider is a hard stop to the Architect.
- **D1 — relocate the three DecEq instances + their helpers into LC**, resolving
  the `trans` collision. Move `DecEq UInt8`/`DecEq Bytes` (+ uint8_/bytes_
  helpers + both injectivity certs) and `DecEq String` (+ string_ helpers) into
  LC alongside its existing DecEq instances; extend LC's loader-visible inventory
  to the relocated names. The eq/sound/complete function BODIES move
  byte-unchanged — only their home module changes.
- **D2 — consolidate EmptyDec's duplicate into LC.** Retire the byte-identical
  local `class DecEq` / `fn bool_eq` / `instance DecEq Bool` and the inlined
  `sym`/`trans`; import `DecEq`/`bool_eq` from LC and `sym`/`trans` from
  Transport. `dec_eq_decides` and the section-3 examples still elaborate against
  LC's `DecEq_instance_Bool`.
- **D3 — the three modules go CLEAN standalone.** Give BytesKeys, StringKeys, and
  EmptyDec real import blocks so every external symbol resolves through a real
  `import`, with the relocated DecEq instances consumed from LC — zero ambient /
  fixture-scaffolding resolution. (A module whose D0 census found an unpublished
  provider defers its full clean-ification with the cited reason and stays a
  partial consumer for that symbol only.)
- **D-EQ-PROSE — correct LC's Public API prose for `class Eq` (P's carry).** In
  LC's "Public API" prose block (:2281-2286 at a7f1fdfd8; re-measure at D0), REMOVE
  `class Eq` — it is private (:61, bare, no `pub`) and unconsumed, so listing it as
  public is a documentation defect (do NOT resolve it by publishing `class Eq`;
  the ruling is that it stays private). Then sweep the WHOLE Eq entry in that prose
  against ACTUAL loader visibility: for each Eq name it lists (`class Eq`,
  `instance Eq Int` :205, `instance Eq Bool` :380), verify real loader-visibility
  and align the prose line to reality — a name that is genuinely visible stays, a
  name that is not is removed. This is prose-only in LC and does not touch the
  relocation; it lands in the same LC edit.

## Acceptance criteria, each with its control

- **AC-RELOCATED (positive, per instance).** Each of `DecEq UInt8`, `DecEq
  Bytes`, `DecEq String` is LOADER-VISIBLE from LC — a selective import `import
  Core.Classes.LawfulClasses (...)` (or the instance's generated dictionary)
  resolves to LC's provider `GlobalId`, measured by the loader, not a `^instance`
  grep. Control: the probe resolves from LC; a probe for the instance as a
  BytesKeys/StringKeys-OWNED local declaration no longer finds one there.
- **AC-EXACT-INVENTORY.** LC's loader-visible inventory equality (the
  `cat_*_export`-analogue control for LawfulClasses) equals its prior set PLUS
  exactly the relocated names and nothing else — EQUALITY, population from the
  module's own definitions, verdict from the loader, per-instance reddening
  mutation (remove `DecEq UInt8`; remove `DecEq Bytes`; remove `DecEq String`)
  each reds the equality distinctly. Never a per-name spot-check.
- **AC-DUPLICATE-RETIRED (EmptyDec).** EmptyDec declares NO local `class DecEq`,
  `fn bool_eq`, `instance DecEq Bool`, or inlined `sym`/`trans`; it imports them.
  Control: the whole-catalog build has exactly ONE `class DecEq` (LC's) and ONE
  `instance DecEq Bool` — the duplicate is GONE, not shadowed. `dec_eq_decides`
  and the section-3 examples elaborate.
- **AC-STANDALONE-CLEAN (per module, the retirement end-state).** BytesKeys,
  StringKeys, EmptyDec each elaborate STANDALONE (exit 0) resolving every
  external symbol through a real import — no ambient/scaffolding fallback. Control
  the "no ambient fallback" directly (e.g. the module loads with only its
  declared imports available), not merely "exit 0 under the full fixture". A
  deferred-provider symbol (D0-cited) is the one allowed exception, named
  explicitly.
- **AC-CLASS-UNIFORMITY (operator invariant).** The operator's ruling that a
  typeclass is identical across a compilation holds after consolidation: no
  module redeclares `class DecEq`, and every `DecEq Bool` use resolves to LC's
  single instance. This is the enforcement the operator named as intended, not
  incidental.
- **AC-NO-COMPUTATIONAL-CHANGE.** The relocated eq/sound/complete function bodies
  are byte-unchanged; a differential over the moved definitions shows only their
  module home changed, not their text. Consumers of these instances compute the
  same results.
- **AC-EQ-PROSE-ACCURATE (P's carry).** LC's "Public API" prose lists exactly the
  loader-visible public names in the Eq entry: `class Eq` (private :61) is REMOVED,
  and every remaining Eq name it lists is corroborated loader-visible. Control: the
  removed `class Eq` is verified still bare/private at :61 (not published to
  satisfy the prose); for the instance sweep, a listed Eq-instance name that the
  loader does not actually expose reds the check (prose-vs-loader differential),
  not a bare grep for the word `Eq`. No computational change — prose only.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  that loads LC, or a migrated module, or a module whose closure this changes),
  scoped by which PATHS changed. Targeted via `scripts/ken-cargo`, never
  `--workspace`; whole-suite green is CI's job.

## Contention check

Production touch: `catalog/packages/Core/Classes/LawfulClasses.ken.md` (LC — the
relocation target + `trans` import), `Data/Binary/BytesKeys.ken.md`,
`Data/Text/StringKeys.ken.md`, `Core/Logic/EmptyDec.ken.md`, plus LC's
loader-visible inventory test fixture. All in `catalog/` (lane 3). No other
active lane touches these: lane 1 (runtime) = `ken-runtime` + `ken-cli`
rt-parity; lane 2 (language) = `ken-elaborator` + `ken-kernel`. The Tier-A
predecessor (`CAT-MIGRATE-TIER-A-PROVIDERS`, merged `f1d7d4133`) published
Derived's surface; this tier does not touch Derived. No collision.

## Capability tier: T1

The bulk is a behavior-preserving relocation + a byte-identical
duplicate-retirement, which is T2-mechanical. The tier is T1 because the review
turns on TWO judgments the diff does not settle: (1) the `trans` import-collision
resolution on LC's proof surface (does bringing `trans` into LC break or
silently weaken an existing LC proof?), and (2) the class-uniformity soundness
(is the duplicate genuinely gone and every `DecEq` use resolving to the single
owner, or is a shadow left?). The Architect's required review is on those, not
the byte-count.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch, no operator authorization — the operator already
ruled the relocation). Reviewed by the **Architect (required soundness/design
reviewer per tier, esp. the `trans` collision + class-uniformity)** + **Foundation
QA + CV**, standing Adversary hunt independent. Steward routes M1-M4; lieutenant
M5-M9.

This is Tier B of [[CAT-SCAFFOLD-RETIREMENT]]. Its successor is Tier C (Data value
modules: SB, BK, NE, Map, Codec, Validation, Deque, Vector, Sums.Combinators),
which consumes the DecEq class this tier relocates — Tier C sequences after this
lands, framed one release ahead.
