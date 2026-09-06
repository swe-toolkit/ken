---
id: CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION
title: "Tier-C staging P3 (flip-only): make {NonEmpty, Validation} strict abstract importable modules and add the import edges into their now-ambient clients. Logic-free once P1 added the smart constructors and P2 removed the last external raw-ctor consumer; this node only hides the raw constructor and wires imports. Full Architect soundness pass required (abstraction boundary + loading semantics)."
status: active
owner: foundation
size: M
gate: none
tier: T1
depends_on: [CAT-MIGRATE-LF-SEMIGROUP-PUBLISH, CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS, LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE, LANG-ABSTRACT-EXPORT-PARAM-ELAB, CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP]
blocks: []
github: null
origin: "Steward, 2026-09-03, split out of [[CAT-MIGRATE-TIER-C-DATA-VALUE]] on the confirmed D0 census (foundation evt_19kq7r92attpy, Architect confirmation evt_4hp6qxkdaqgbz). RECUT 2026-09-06 to the flip-only step P3 of the Architect's 3-step additive staging DAG (evt_5fxtzhk104q96): P1 (CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS, merged 0e439f1e5) added the smart constructors; P2 (CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP, merged e75ce9b58) swapped all five external raw-ctor call sites to nonempty_cons, so the external raw-NonEmptyCons census is now ZERO. RELEASED 2026-09-06 by the Steward once P2 landed: all five depends_on are merged (LF-SEMIGROUP-PUBLISH, EC-APPLICATIVE-PROVIDERS, LANG-ROOTS-LOADER, LANG-ABSTRACT-EXPORT-PARAM-ELAB, the P2 callsite-swap). This node does ONLY the logic-free flip; the code motion (smart ctors + call-site swap) already landed ahead of it, which is what makes the abstract flip safe (no window where a live consumer names a hidden ctor)."
---

> # RELEASED 2026-09-06 to the foundation ring (lane-3), step P3 (flip-only) of
> # the Architect's 3-step additive staging DAG (evt_5fxtzhk104q96). All five
> # depends_on are MERGED. This is the FLIP: {NonEmpty, Validation} become strict
> # abstract importable modules (raw ctor hidden), and their now-ambient clients
> # gain the import edges they need. The prior monolithic scope (candidate
> # 94b061de, which bundled smart ctors + abstract flip and reddened cc7/cc8) is
> # SUPERSEDED and fully retired -- P1/P2 moved that code motion ahead of this
> # flip. Base = current origin/main (e4f355b12 at release; re-measure at cut,
> # lines drift). Architect is the REQUIRED reviewer and gives this its FULL
> # soundness pass (abstraction boundary + loading semantics); this is not an
> # additive/mechanical review like P1/P2. Seat: check the tier at kick -- the
> # mechanics are small but the loading-semantics/standalone-green work
> # hard-stopped twice in the monolithic scope, so it is provisioned T1.

## What this is

The final step of the Tier-C {NonEmpty, Validation} scaffold retirement. With the
smart constructors landed (P1) and every external raw-`NonEmptyCons` call site
swapped to `nonempty_cons` (P2), no source outside NonEmpty's defining module
names the raw constructor. This node flips {NonEmpty, Validation} to strict
abstract importable modules -- the raw constructor becomes internal, hidden from
clients -- and adds the selective imports the now-ambient clients need to keep
resolving. It is logic-free: it introduces no new definition and changes no
denotation; it only hides the raw ctor and wires imports.

## Fixed inputs (D0-measured at e4f355b12; RE-MEASURE at your cut -- lines drift)

- **NonEmpty** defining module: `catalog/packages/Data/Collections/NonEmpty.ken.md`.
  Public API line (§7) currently lists `NonEmpty`/`NonEmptyCons`,
  `nonempty_singleton`, `nonempty_cons`, `nonempty_head`, `nonempty_tail`,
  `nonempty_to_list`, `nonempty_map`, `nonempty_append`, and
  `Semigroup_instance_NonEmpty`.
- **External raw-`NonEmptyCons` census = ZERO** (P2's AC-NO-RAW-CONSUMER holds;
  re-confirm at cut). The only `NonEmptyCons` references left in the tree are
  inside the defining module (definition, accessors, laws, the local example).
- **Clients that must gain an import edge at the flip** (measured post-P2):
  - `Data/Sums/Validation.ken.md` -- itself made strict/abstract by this node;
    consumes from NonEmpty: the `NonEmpty` type, `nonempty_cons` (:77/:86),
    `nonempty_append` (:125), and `Semigroup_instance_NonEmpty` (:113, the
    synthesized dict -- imported-head carry ONLY, never a minted `pub instance`).
  - `Application/CommandLine/ArgParse.ken.md` -- `NonEmpty` type + `nonempty_cons`
    (:252).
  - `Application/Input/Schema.ken.md` -- `NonEmpty` type + `nonempty_cons`
    (:123/:130).
  - `Application/Configuration/Decoder.ken.md` -- names the Validation/NonEmpty
    TYPES only; type imports at the flip.
  - `catalog/examples/CommandLine/Forge.ken.md` -- names the Validation/NonEmpty
    TYPES only; type imports at the flip.
  Re-run the census at cut; a client that names a symbol only through ambient
  reach today needs a selective import the moment the producer goes strict.
- **Harness legs**: cc7/cc8 (and cc1 if still ambient) move to the roots loader;
  extend the loader inventory to cover the newly-strict modules.

## Deliverables

- **D0 -- re-measure at the release SHA.** Re-run the external raw-ctor census
  (expect zero), the per-client symbol-use sets, and the exact NonEmpty §7 Public
  API and Validation export surfaces. NonEmpty before Validation (the intra-tier
  edge). Any drift from the Fixed inputs is D0's to correct before authoring.
- **D1 NonEmpty -- flip to strict abstract importable.** Hide the raw
  `NonEmptyCons` constructor: drop `NonEmptyCons` from the §7 Public API list;
  keep the smart constructors (`nonempty_singleton`, `nonempty_cons`) and the
  accessors (`nonempty_head`/`nonempty_tail`/`nonempty_to_list`/`nonempty_map`/
  `nonempty_append`) and `Semigroup_instance_NonEmpty` public. The raw
  constructor stays internal to the defining module; the `nonempty_append` law
  (uses the raw ctor inside the defining module) stays internal and valid -- the
  LANG-ABSTRACT-EXPORT-PARAM-ELAB fix keeps the constructor visible to the
  defining module. Add the selective imports of `nonempty_cons` (and any other
  now-non-ambient symbol) into ArgParse, Schema, and Validation.
- **D2 Validation -- flip to strict abstract importable**, after D1. Publish the
  measured Validation export surface; add selective imports from NonEmpty (type,
  `nonempty_cons`, `nonempty_append`, `Semigroup_instance_NonEmpty`) and from the
  published lower tiers; retire ambient reach; add type imports into
  Configuration.Decoder and Forge. Extend the roots-loader inventory; standalone
  exit 0. The `Semigroup_instance_NonEmpty` example resolves via imported-head
  carry ONLY.
- **D3 -- honest reach.** cc7/cc8 (+cc1 if still ambient) run through the roots
  loader over the newly-strict modules; standalone-green each module.

## Architect bounded notes to fold (z3570/z3660 -- carry into the candidate)

- **(a) Validation law witnesses that are `pub proof`/`pub theorem`.** When
  Validation goes abstract, a law witness exported as a public proof/theorem must
  either be PRIVATIZED (the `option_map` precedent) or explicitly listed in the
  §7 Public API as part of the intended abstract surface. Do not leave a public
  proof dangling across the abstraction boundary. Measure which witnesses are
  currently `pub` and decide per witness; the Architect confirms.
- **(b) The removed no-Monad-registry guard needs a BEHAVIORAL replacement**, not
  just deletion -- assert the property the guard protected as a positive check the
  node carries.

## Acceptance criteria

- **AC-EXPORTED** (per published symbol): each symbol the flip publishes is
  loader-resolved from a client, with a still-private-sibling control proving the
  boundary is real (a sibling NOT exported is NOT resolvable).
- **AC-EXACT-INVENTORY** (per module): a per-symbol reddening mutation -- removing
  any one exported symbol reddens exactly the clients that use it; the loader
  inventory covers exactly the newly-strict modules (extra-ctor mutation nets).
- **AC-STANDALONE-GREEN**: removing an added import line restores the exact prior
  standalone failure for that client (the import is load-bearing, not decorative).
- **AC-ABSTRACT** (Architect evt_4s9y6bpyzgaet): a CLIENT module cannot construct
  or match `NonEmpty`/`Validation` via the raw constructor (opaque view) -- it
  constructs only through the smart constructors and eliminates only through the
  accessors. Control: the raw-constructor reference that compiles INSIDE the
  defining module is REJECTED (UnboundName / abstract-view rejection) from a
  client, at the exact name.
- **AC-VISIBILITY-ONLY**: no new definition and no denotation change. Pub-widening
  of existing symbols is a byte-unchanged body; hiding the raw ctor removes it
  from the client-visible surface only. NO second class/instance, NO invented
  `pub instance`, NO smart-ctor body change (they landed in P1).
- **AC-LAW-WITNESS** (folds note (a)): every Validation law witness is either
  privatized or listed in the §7 Public API; no public proof/theorem crosses the
  boundary unaccounted.
- **AC-GUARD-REPLACEMENT** (folds note (b)): the property the removed
  no-Monad-registry guard protected is asserted by a positive behavioral check.
- **AC-NO-REGRESSION**: full `-p ken-elaborator` green in CI over the complete
  affected-target closure (every target that loads any module whose loading this
  flip changes, diff-touched or not); cc1/cc7/cc8 green. Targeted via
  `scripts/ken-cargo`, never `--workspace` -- green in CI is the workspace verdict.

## Gate, reviewer, sequencing

`gate: none`. On the candidate: **Architect** (REQUIRED -- full soundness pass:
abstraction boundary + loading semantics + the imported-head carry for the
synthesized dict) + **Foundation QA** + **CV** on the exact SHA, then Steward
M1-M4 -> lieutenant M5-M9. Final step of the 3-step DAG; depends_on all merged.
NonEmpty (D1) before Validation (D2); may land as accepted partial per increment.
