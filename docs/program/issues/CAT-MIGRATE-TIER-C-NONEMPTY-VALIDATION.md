---
id: CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION
title: "Tier-C staging P3 (flip-only): make NonEmpty a strict abstract importable module (hide the raw NonEmptyCons ctor) and add the import edges into its now-ambient clients. Logic-free once P1 added the smart constructors and P2 removed the last external raw-ctor consumer; this node only hides the raw constructor and wires imports. Full Architect soundness pass required (abstraction boundary + loading semantics). Validation was SPLIT OUT of this node to CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT -- its abstract flip was refuted and it goes strict-transparent instead."
status: merged
owner: foundation
size: S
gate: none
tier: T1
depends_on: [CAT-MIGRATE-LF-SEMIGROUP-PUBLISH, CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS, LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE, LANG-ABSTRACT-EXPORT-PARAM-ELAB, CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP]
blocks: []
github: null
origin: "Steward, 2026-09-03, split out of [[CAT-MIGRATE-TIER-C-DATA-VALUE]] on the confirmed D0 census (foundation evt_19kq7r92attpy, Architect confirmation evt_4hp6qxkdaqgbz). RECUT 2026-09-06 to the flip-only step P3 of the Architect's 3-step additive staging DAG (evt_5fxtzhk104q96): P1 (CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS, merged 0e439f1e5) added the smart constructors; P2 (CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP, merged e75ce9b58) swapped all five external raw-ctor call sites to nonempty_cons, so the external raw-NonEmptyCons census is now ZERO. RELEASED 2026-09-06 by the Steward once P2 landed: all five depends_on are merged. RESCOPED 2026-09-07 (Steward) to NonEmpty ONLY: Validation's abstract flip was refuted by Architect ruling evt_1wgz67t0nj1ne + Spec confirmation evt_5m18jwangsdf8 (Schema constructs AND pattern-matches raw Valid/Invalid, and Validation carries no representation invariant), so Validation was split out to [[CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT]] where it goes strict-transparent, not abstract. NonEmpty's abstract flip (invariant-justified: hiding NonEmptyCons keeps nonempty_head total by construction) stays here. This node does ONLY the logic-free NonEmpty flip; the code motion (smart ctors + call-site swap) already landed ahead of it, which is what makes the abstract flip safe (no window where a live consumer names a hidden ctor)."
---

> # RESCOPED 2026-09-07 (Steward) to NonEmpty ONLY. The Validation abstract flip
> # that this node used to bundle was REFUTED: Architect ruling evt_1wgz67t0nj1ne
> # (Schema both constructs AND pattern-matches raw Valid/Invalid at
> # Schema.ken.md:113-142, and Validation carries no representation invariant, so
> # abstraction is pure cost) + Spec confirmation evt_5m18jwangsdf8 (no
> # spec/conformance/API intent requires it; no Decision needed). Validation is now
> # CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT (strict-TRANSPARENT flip). NonEmpty
> # stays abstract here because hiding NonEmptyCons protects a real invariant
> # (non-emptiness -> nonempty_head total). This node's scope is now the single
> # NonEmpty abstract flip; it is no longer a two-root node and no longer lands as
> # an "accepted partial" -- the NonEmpty flip IS the node.
>
> # RELEASED 2026-09-06 to the foundation ring (lane-3), step P3 (flip-only) of the
> # Architect's 3-step additive staging DAG (evt_5fxtzhk104q96). All five depends_on
> # are MERGED. Base = current origin/main (re-measure at cut, lines drift).
> # Architect is the REQUIRED reviewer and gives this its FULL soundness pass
> # (abstraction boundary + loading semantics); this is not an additive/mechanical
> # review like P1/P2. Seat: check the tier at kick -- the mechanics are small but
> # the loading-semantics/standalone-green work hard-stopped twice in the monolithic
> # scope, so it is provisioned T1.

## What this is

The final step of the Tier-C NonEmpty scaffold retirement. With the smart
constructors landed (P1) and every external raw-`NonEmptyCons` call site swapped
to `nonempty_cons` (P2), no source outside NonEmpty's defining module names the
raw constructor. This node flips NonEmpty to a strict abstract importable module
-- the raw constructor becomes internal, hidden from clients -- and adds the
selective imports the now-ambient clients need to keep resolving. It is logic-free:
it introduces no new definition and changes no denotation; it only hides the raw
ctor and wires imports.

Validation is NOT in this node's scope: its abstract flip was refuted and it goes
strict-transparent separately in CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT.
Validation still appears below only as a CLIENT of NonEmpty (it imports
`nonempty_cons`/`nonempty_append`/`Semigroup_instance_NonEmpty`), and that import
edge is this node's to add.

## Fixed inputs (D0-measured at e4f355b12; RE-MEASURE at your cut -- lines drift)

- **NonEmpty** defining module: `catalog/packages/Data/Collections/NonEmpty.ken.md`.
  Public API line (§7) currently lists `NonEmpty`/`NonEmptyCons`,
  `nonempty_singleton`, `nonempty_cons`, `nonempty_head`, `nonempty_tail`,
  `nonempty_to_list`, `nonempty_map`, `nonempty_append`, and
  `Semigroup_instance_NonEmpty`.
- **External raw-`NonEmptyCons` census = ZERO** (P2's AC-NO-RAW-CONSUMER holds;
  re-confirm at cut). The only `NonEmptyCons` references left in the tree are
  inside the defining module (definition, accessors, laws, the local example).
- **Clients that must gain a NonEmpty import edge at the flip** (measured post-P2):
  - `Data/Sums/Validation.ken.md` -- a NonEmpty CLIENT (its own strict-transparent
    flip is the separate VALIDATION-STRICT-IMPORT node); consumes from NonEmpty:
    the `NonEmpty` type, `nonempty_cons` (:77/:86), `nonempty_append` (:125), and
    `Semigroup_instance_NonEmpty` (:113, the synthesized dict -- imported-head
    carry ONLY, never a minted `pub instance`).
  - `Application/CommandLine/ArgParse.ken.md` -- `NonEmpty` type + `nonempty_cons`
    (:252).
  - `Application/Input/Schema.ken.md` -- `NonEmpty` type + `nonempty_cons`
    (:123/:130).
  - `Application/Configuration/Decoder.ken.md` -- names the `NonEmpty` TYPE only;
    type import at the flip.
  - `catalog/examples/CommandLine/Forge.ken.md` -- names the `NonEmpty` TYPE only;
    type import at the flip.
  Re-run the census at cut; a client that names a symbol only through ambient
  reach today needs a selective import the moment the producer goes strict.
- **Harness legs**: cc7/cc8 (and cc1 if still ambient) move to the roots loader;
  extend the loader inventory to cover the newly-strict NonEmpty module.

## Deliverables

- **D0 -- re-measure at the release SHA.** Re-run the external raw-ctor census
  (expect zero), the per-client NonEmpty symbol-use sets, and the exact NonEmpty
  §7 Public API. Any drift from the Fixed inputs is D0's to correct before
  authoring.
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
- **D2 -- honest reach.** cc7/cc8 (+cc1 if still ambient) run through the roots
  loader over the newly-strict NonEmpty module; standalone-green the module.

## Acceptance criteria

- **AC-EXPORTED** (per published symbol): each symbol the flip publishes is
  loader-resolved from a client, with a still-private-sibling control proving the
  boundary is real (a sibling NOT exported is NOT resolvable).
- **AC-EXACT-INVENTORY** (per module): a per-symbol reddening mutation -- removing
  any one exported symbol reddens exactly the clients that use it; the loader
  inventory covers exactly the newly-strict NonEmpty module (extra-ctor mutation
  nets).
- **AC-STANDALONE-GREEN**: removing an added import line restores the exact prior
  standalone failure for that client (the import is load-bearing, not decorative).
- **AC-ABSTRACT** (Architect evt_4s9y6bpyzgaet): a CLIENT module cannot construct
  or match `NonEmpty` via the raw `NonEmptyCons` constructor (opaque view) -- it
  constructs only through the smart constructors and eliminates only through the
  accessors. Control: the raw-constructor reference that compiles INSIDE the
  defining module is REJECTED (UnboundName / abstract-view rejection) from a
  client, at the exact name.
- **AC-VISIBILITY-ONLY**: no new definition and no denotation change. Pub-widening
  of existing symbols is a byte-unchanged body; hiding the raw ctor removes it
  from the client-visible surface only. NO second class/instance, NO invented
  `pub instance`, NO smart-ctor body change (they landed in P1).
- **AC-NO-REGRESSION**: full `-p ken-elaborator` green in CI over the complete
  affected-target closure (every target that loads any module whose loading this
  flip changes, diff-touched or not); cc1/cc7/cc8 green. Targeted via
  `scripts/ken-cargo`, never `--workspace` -- green in CI is the workspace verdict.

## Gate, reviewer, sequencing

`gate: none`. On the candidate: **Architect** (REQUIRED -- full soundness pass:
abstraction boundary + loading semantics + the imported-head carry for the
synthesized dict) + **Foundation QA** + **CV** on the exact SHA, then Steward
M1-M4 -> lieutenant M5-M9. This node is now the single NonEmpty abstract flip
(Validation split out to CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT); its
depends_on are all merged, so it is releasable as-is.
