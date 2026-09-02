---
id: CAT-LAWFULFUNCTORS-STANDALONE-IMPORT
title: "Migrate the three orphan list_append proofs (assoc/left_unit/right_unit) from Core/Classes/LawfulFunctors into Data/Collections/Derived, the module that defines list_append, per Architect ruling evt_7khknqydxxd93. An attached proof f::law is part of f's definitional surface and can be soundly owned only by f's module; proving it elsewhere is an orphan attachment that resolves under full-catalog load but goes UnboundName under selective/standalone import. LawfulFunctors keeps its co-located bool_and/list_map/option_map proofs and gains the real standalone imports it owes; EffectfulClasses's prose owner-attribution is re-pointed to Derived. Carries the campaign rule for every future orphan attached proof."
status: merged
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-DERIVED-PUB-EXPORT]
blocks: []
github: null
origin: "Steward RECUT 2026-09-02 of the CAT-LAWFULFUNCTORS-STANDALONE-IMPORT hard stop, per Architect ruling evt_7khknqydxxd93 (thread thr_a8wy366qdepf). The original single-import framing was FALSIFIED by the foundation ring's D0/D1: adding `import Data.Collections.Derived (list_append)` resolves list_append but exposes UnboundName Data.Collections.Derived.list_append::assoc — an attached-proof ownership split. The Architect ruled it an orphan attached proof (the direct analogue of an orphan instance): list_append::{assoc,left_unit,right_unit} are proved IN LawfulFunctors but attach to list_append, which is defined in Derived. Ruling: reject the cross-module attachment-carry mechanism (option b, elaborator/TCB-enlarging, reintroduces the incoherence one level up); adopt option (a) executed as (c) — a migration the Steward recuts moving the three proofs into Derived. The laws are laws-with-the-function (their statements use only List/Nil/Cons, list_append, Equal/Refl/Proved, cong — nothing from Semigroup/Monoid), so Derived is the correct owner. Layering-clean: Derived is the lower layer (imports only Core.Logic.*, never Core.Classes) and its §4 already hosts proofs using Equal/Refl/Proved/cong."
---

> # RECUT: THIS IS AN ATTACHED-PROOF MIGRATION, NOT AN IMPORT REPAIR
>
> The original frame asserted an "ownership axis measured absent / single import
> / sole symbol." **That was FALSIFIED and is void.** The initial measurement
> inherited the elaborator's fail-fast blind spot: elaboration halts at the
> first `UnresolvedCon` (`list_append`) and never reaches the attached-proof
> references behind it, so "list_append is the sole unresolved symbol" was true
> only because the run stopped early. `"not flagged" != "resolved".`
>
> The real defect, ruled by the Architect (`evt_7khknqydxxd93`): the proofs
> `list_append::{assoc,left_unit,right_unit}` are proved in
> `Core.Classes.LawfulFunctors` (`:99/:104/:122` at `4ab87564f`) but attach to
> `list_append`, which is defined in `Data.Collections.Derived` (`:514`). **An
> attached proof `f::law` is part of `f`'s definitional surface and can be
> soundly owned only by the module that defines `f`** — the direct analogue of
> an orphan instance. Its resolution depends on the LOAD SET, not the import
> graph reachable from the reference, which is why it binds under full-catalog
> load and goes `UnboundName` under selective import.

> # CAMPAIGN RULE (carried by this node, governs the whole catalog-reuse effort)
>
> **An attached proof `f::law` MUST be defined in the module that defines `f`.**
> Wherever "standalone-from-declared-imports" meets a module proving laws about a
> function owned elsewhere, MIGRATE the orphan proof to the function's module —
> never build a cross-module attachment-carry. Ownership is fixed by what the
> law's STATEMENT is written in and what it depends on, never by which class
> consumes it:
>
> - A law stated in the function's own vocabulary, needing nothing from the
>   class layer, is a LAW-WITH-THE-FUNCTION: it lives in the function's defining
>   module, and every structure cites the one canonical proof.
> - A law that is a coherence obligation between the function and a specific
>   class/instance — its statement mentions `Monoid.mempty`, `fold_map`, a
>   dictionary — is a LAW-WITH-THE-STRUCTURE and lives with the instance/class.
>
> `list_append::{assoc,left_unit,right_unit}` are laws-with-the-function
> (statements use only `List`/`Nil`/`Cons`, `list_append`, `Equal`/`Refl`/
> `Proved`, `cong`). Derived owns them. The class fields `Semigroup.assoc` /
> `Monoid.assoc` are stated over `op`; the instances discharge them by pointing
> at the function-level proof — the function-level proof is the reusable
> artifact, the class field is only wiring.

## Symptom inventory

Append one line per hard stop; never rewrite history.

- 2026-09-02 HARD STOP (foundation ring, evt_2qmg983kg3j34 / evt_1jam36cz3bwr8),
  routed to Steward + Architect (evt_56d9nsv2yxeze). The single import does NOT
  achieve standalone-clean. D0 (no import): `UnresolvedCon list_append` (4204..4215).
  D1 (+`import Data.Collections.Derived (list_append)`): list_append resolves but a
  NEW `UnboundName Data.Collections.Derived.list_append::assoc` (4723..4752) appears
  — set went {list_append} -> {list_append::assoc}, not -> {}. MECHANISM: the
  FUNCTION list_append is Derived-owned, but its LAWS
  `list_append::assoc/::left_unit/::right_unit` are proved IN this module
  (:99/:104/:122) and consumed cross-module (EffectfulClasses.ken.md). Under
  full-catalog load the attachment binds; under selective import the qualified
  proof name is unbound. An attached-proof ownership split, not a pure import.
  STEWARD FRAME DEFECT: this node's "ownership axis measured absent" and "single
  import / sole symbol" were FALSIFIED — the initial measurement inherited the
  elaborator's fail-fast blind spot (halt at the first `UnresolvedCon` never
  reached the attached-proof references behind it; "not flagged" != "resolved").
- 2026-09-02 RECUT (Steward, per Architect ruling evt_7khknqydxxd93). Reject the
  cross-module attachment-carry (option b); adopt (a)-as-(c): migrate the three
  orphan proofs into Derived. Objective/Deliverables/ACs below are re-authored
  and OPERATIVE as of this recut. The prior single-import Objective/ACs are void.

## Objective

Make `Core/Classes/LawfulFunctors.ken.md` and `Data/Collections/Derived.ken.md`
each elaborate standalone-from-declared-imports at Omega, by relocating the three
orphan `list_append` proofs to their sound owner and supplying LawfulFunctors the
real imports it owes. Corpus stays green under full-catalog load throughout.

## Fixed inputs

Measured by the Steward at `origin/main` `4ab87564f` (the ring's hold SHA and the
Architect's ground-check base). **Line numbers are markdown-source physical
lines; D0 re-establishes every site at the release SHA — reproduce, do not
trust these.**

- `Data.Collections.Derived` defines `fn list_append` at `Derived.ken.md:514`
  (`fn list_append (a : Type) (xs : List a) (ys : List a) : List a = …`). D1
  (foundation-implementer, 4ab87564f) established that
  `import Data.Collections.Derived (list_append)` resolves `list_append` from a
  standalone importer — so the function is reachable by selective import at the
  tip. **D0 confirms reachability at the release SHA and reports whether a
  pub-export edit is required (the census had claimed pub; the current
  declaration carries no `pub` keyword).**
- Derived's §4 "Laws & proofs" (`Derived.ken.md:564`) already hosts proofs using
  `Equal`/`Refl` (`:622`)/`Proved` (`:700`)/`cong` (`:627`) — so the equality and
  transport vocabulary the three proofs need is ALREADY in Derived's scope. D1
  confirms; add an equality-type import only if the build shows one missing.
- The three proofs to move, verbatim, are `LawfulFunctors.ken.md:99-135`
  (`proof left_unit for list_append` :99, `proof assoc for list_append` :104,
  `proof right_unit for list_append` :122). Their bodies use only
  `List`/`Nil`/`Cons`, `list_append`, `Equal`/`Refl`/`Proved`, `cong`.
- LawfulFunctors's two instances that consume them: `instance Semigroup (List
  Nat)` (`:137`, `assoc = proof assoc for list_append Nat`) and `instance Monoid
  (List a)` (`:142`, `assoc`/`left_unit`/`right_unit = proof … for list_append a`).
- LawfulFunctors has **no `import` statement** at the tip
  (`grep -c '^import' <LawfulFunctors>` = 0). Its co-located, CORRECT proofs that
  STAY (attached to functions this module owns): `bool_and` (`:173`), `list_map`
  (`:260`), and its other co-located proofs. D0 enumerates the full stay-set.
- EffectfulClasses consumers of the moved proofs:
  - CODE (unqualified attachment spelling `proof X for list_append`), at
    `:499/:535/:579/:585/:597/:646/:661` — these resolve by attachment/load-set
    and, once the proofs live beside `list_append` in Derived, continue to
    resolve under full-catalog load with NO edit. D0 confirms.
  - PROSE ownership mis-attribution: `:567` names
    `(Core/Classes/LawfulFunctors.ken)` as the owner of `list_append::right_unit`.
    D0 enumerates every such prose attribution; each is re-pointed to Derived.

## Deliverables

- **D0 — measure before changing anything, at the release SHA.** Build ken-cli
  fresh. (1) Reproduce the standalone `LawfulFunctors` failure and record the
  FULL unresolved-symbol evolution as imports are added one at a time — do NOT
  trust a single fail-fast reading; the elaborator halts at the first
  `UnresolvedCon`, so iterate until the set stabilises. (2) Confirm `list_append`
  reachability from a standalone importer and report whether a `pub` export edit
  on `Derived.ken.md:514` is required. (3) Enumerate every EffectfulClasses
  reference to the moved proofs, classified code vs prose. (4) Record the
  full-catalog-green baseline. **A D0 finding that own-module attached proofs do
  NOT travel with a selective `import Data.Collections.Derived (list_append)` —
  i.e. LawfulFunctors's instances cannot cite the moved proofs without an
  explicit attached-name import the surface does not support — is a HARD STOP to
  Steward: that is the open sub-question below, a MODULAR language-surface
  question routed to Spec/Language, not something to invent a workaround for.**
- **D1 — move the three proofs into Derived §4, verbatim.** Relocate
  `proof left_unit/assoc/right_unit for list_append` (LawfulFunctors :99-136)
  into `Data/Collections/Derived.ken.md` §4, co-located with `fn list_append`.
  Change nothing in the proof bodies. Verify Derived elaborates them, producing
  `Data.Collections.Derived.list_append::{left_unit,assoc,right_unit}`. Add an
  equality-type import to Derived ONLY if D0/build shows one missing (its §4
  already hosts such proofs, so expect none).
- **D2 — LawfulFunctors cites the moved proofs and gains its real standalone
  imports.** Re-point the `Semigroup (List Nat)` and `Monoid (List a)` instances
  to the Derived-owned proofs, using the spelling D0 established the surface
  supports (see the open sub-question — this is measured, not guessed). Then add
  the standalone imports LawfulFunctors actually owes for `list_append` and every
  other non-local symbol its remaining content uses (the real owners of `List`/
  `Option`/`Equal`/`Refl`/`Proved`/`cong`, etc., as D0's iterated census names
  them). The co-located `bool_and`/`list_map`/`option_map` proofs STAY untouched.
- **D3 — re-point the EffectfulClasses prose owner-attribution.** For every prose
  reference D0 enumerated that names `Core/Classes/LawfulFunctors.ken` as the
  owner of `list_append::*` (confirmed at `:567`), change the owner to
  `Data.Collections.Derived`. The CODE consumers (`proof X for list_append`) are
  NOT edited — confirm by build they still resolve by attachment under
  full-catalog load.
- **D4 — report the open sub-question's answer.** State, from the D0/D1 build,
  which case holds: (i) `import Data.Collections.Derived (list_append)` brings
  `list_append::assoc` etc. into scope automatically (own-module attachments
  travel with the function ⇒ this migration is pure catalog data movement, no
  language change), or (ii) attached names must be listed explicitly ⇒ a distinct
  modular language-surface question for Spec/Language. This is a REPORT
  deliverable, and case (ii) is the D0 hard stop above.

## Acceptance criteria, each with its control

- **AC-DERIVED-STANDALONE.** `Derived.ken.md` elaborates standalone-from-declared
  -imports at Omega, now including
  `list_append::{left_unit,assoc,right_unit}`. Control: standalone check on
  Derived in isolation; the three proof `GlobalId`s are present in its
  elaboration output and absent before D1.
- **AC-LAWFULFUNCTORS-STANDALONE.** `LawfulFunctors.ken.md` elaborates standalone
  -from-declared-imports at Omega with no `UnresolvedCon` and no `UnboundName`.
  Control: standalone check in isolation; D0's iterated pre-migration census
  (which must show the orphan `list_append::assoc` unbound under a bare
  single-import) is the paired negative — the same check FAILS at D0 and PASSES
  after D2.
- **AC-PROOFS-VERBATIM.** The three moved proof bodies are byte-identical to
  their LawfulFunctors originals (only their module home changed). Control: a
  differential of the moved text against `LawfulFunctors.ken.md:99-136` at the
  base SHA shows zero body change.
- **AC-INSTANCES-CITE-DERIVED.** LawfulFunctors's `Semigroup`/`Monoid` instances
  discharge `assoc`/`left_unit`/`right_unit` by citing the Derived-owned proofs,
  via the surface spelling D0 established. Control: the instances elaborate
  standalone; deleting the citing import reintroduces exactly the
  `list_append::*` `UnboundName`.
- **AC-EFFECTFULCLASSES-GREEN.** EffectfulClasses elaborates unchanged in CODE;
  every prose owner-attribution of `list_append::*` names Derived, none names
  LawfulFunctors. Control: a grep for `LawfulFunctors` as owner of `list_append`
  returns zero; the module's code diff is empty.
- **AC-NO-ORPHAN-REMAINS.** No module proves an attached `f::law` for a function
  it does not define, for `list_append`. Control: after D1, `list_append::*`
  proofs exist ONLY in Derived; a census of `proof … for list_append` definition
  sites returns Derived alone.
- **AC-FULL-CATALOG-GREEN.** Whole-catalog elaboration stays green throughout
  (full-catalog load is the invariant the corpus ships on). Control: the
  full-catalog check is green at the base SHA and green after D3.
- **AC-NO-REGRESSION.** Whole-suite green in CI. Local targeted only, via
  `scripts/ken-cargo -p <crate>`, never `--workspace`.

## What this unblocks, and what it does NOT change

Removes the orphan attached-proof from the catalog: `LawfulFunctors` and
`Derived` both become standalone-from-declared-imports clean for the
`list_append` laws, and the campaign gains its standing fix for every future
orphan attached proof.

- **No kernel/TCB/spec/conformance change is authorized** (Architect,
  evt_7khknqydxxd93). This is catalog data movement plus import wiring.
- **It does NOT do the census row-28 reuse migration** (`list_map→Prelude.map`,
  `list_foldr→Prelude.fold`, `bool_and→LawfulClasses.bool_and`). Those symbols
  are locally defined, cause no unresolved error, and the law-carrying half is
  deferred behind a separate Architect ruling. Do not touch them here.
- **Option (b) — a cross-module attachment-carrying import — is OFF THE TABLE**
  (rejected on coherence + subsume-don't-proliferate). Do not build it, and do
  not queue operator sign-off for it.

## Contention check

Touches `catalog/packages/Data/Collections/Derived.ken.md` (add three proofs),
`catalog/packages/Core/Classes/LawfulFunctors.ken.md` (remove three proofs, add
imports, re-point two instances), and `catalog/packages/Core/Classes/
EffectfulClasses.ken.md` (prose owner-attribution only). No other lane touches
these files. Test-fixture closure only in `crates/`; no `/spec`, no kernel/TCB.
`Derived` is the lower layer (imports only `Core.Logic.*`, never `Core.Classes`),
so moving proofs INTO it introduces no cycle.

## Reviewers

foundation-qa (the three proofs elaborate in Derived, both modules standalone
-clean, instances cite the Derived proofs, proofs byte-verbatim, no orphan
remains, full-catalog green) + conformance-validator (catalog implementation
standard compliance). The open sub-question's answer (D4) routes to Steward; if
it is case (ii), the Steward routes the language-surface question to
Spec/Language — it is NOT a foundation-QA gate.

## Sequencing

Lane-3 (foundation). `active` on this recut: the provider `Derived` is merged and
already hosts §4 proofs, the migration is Architect-specified verbatim movement +
import wiring, and the ring is holding clean at `4ab87564f`. D0 re-measures at the
release SHA and hard-stops to Steward if own-module attachments do not travel with
the function (the language-surface case). Tier T2 — a defined migration with its
design front-loaded by the ruling; the reasoning that earned it is already spent.
