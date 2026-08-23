---
id: RT-CHECKED-IH-CAPTURED-ENV-SCHEMA
title: "Admit the checked-IH's non-empty captured environment as a value-domain Record with a planner-issued schema — issue a DEDICATED checked-IH captured-env record (a sibling of, not an extension of, UnitBoundaryEnvironment) with n declared children whose per-field identities are SOURCED from each capture's own occurrence in the checked plan (not fabricated), so the escaping checked functional IH has an admitted environment to cross the effect-seat boundary. The predecessor M6 (RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION) lowering-proper depends on. Tiers 1 (elaborator emit of the captured environment as a semantic object with per-capture occurrences, a COORDINATION section 9a language spillover) + 2 (runtime planner issues a dedicated checked-IH captured-env occurrence with those declared children, reusing the Constructor-branch child-owner derivation)."
status: ready
owner: runtime
size: L
gate: none
depends_on: []
blocks: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
github: null
origin: "Steward, 2026-08-22, cutting the Case-C reach-fork predecessor from the Architect's ruling (evt_2e11sk1jvp8mv) on RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION (M6). The M6 build measured the escaped IH environment NON-EMPTY (StaticWorker declared_arity=1, captures=9) and the existing admitted-env concept bounded to empty captures by construction (implementer evt_svjnmypxr4j5, leader evt_4jg3k6r75mhvz). Design shape ruled by the Architect; BUILD gated on the enclave denotational-faithfulness confirmation (AC-ENTRY). Steward-filed per COORDINATION section 2. HS 5."
---

> # DESIGN CORRECTED 2026-08-23 — the "extend UnitBoundaryEnvironment"
> # premise is FALSIFIED; tier 2 issues a DISTINCT record. Supersedes the
> # MECHANISM (not the intent) wherever passages below still say "same role /
> # not a sibling".
>
> Runtime-implementer hard-stop (evt_aefgp5acawbw) brought live
> instrumentation that falsifies the "extend the same
> UnitBoundaryEnvironment role" premise this
> node was framed on: the checked-IH population NEVER enters that concept.
> `unit_boundary_environment_fields` (aggregates.rs:1091) issues a field only for
> a Call whose callee is a LexicalClosure with `captures.is_empty()`; the
> checked-IH route calls through a Var->StaticWorker (UBESTAGE calls=30,
> lexical_callee=0 — dies at stage 2; UBECALLEE Var 174 / DeclRef 6 /
> LexicalClosure 0) and carries nine captures (stage 4's empty-captures gate is
> its opposite). So there is NO record to extend — not an empty-children record,
> none.
>
> Architect ruling (evt_613jzw7qc56p7) — OPTION (a):
> - Issue a DEDICATED checked-IH captured-env record, a SIBLING of (not
>   an extension of) UnitBoundaryEnvironment. `captures.is_empty()` is
>   definitional to UBE ("unit boundary" = nothing crosses the boundary);
>   a nine-capture record is a distinct population, and merging the two
>   would DESTROY UBE, not widen it.
> - Reuse the DERIVATION, not the entry condition: key the record on
>   the tier-1 checked-plan coordinates (worker closure_origin + the
>   ci<->oi run) and populate children+owners from
>   `aggregate_child_referent_owners` (aggregates.rs:295), already reused
>   by build_aggregate_ownership_plan for a different population
>   independent of the UBE entry gate. Owners from that derivation, NOT
>   meet/allocation assignments (the delete-meet/allocation direction
>   stands).
> - SOUNDNESS GUARD: keep the record DISTINCT unless a grep proves
>   every UBE consumer is capture-count-agnostic (the empty-captures
>   invariant may be relied on downstream). If the grep proves it, reusing
>   the struct is acceptable — state the grep in the handoff.
> - REJECTED (b) widen the UBE entry condition (backend-wide
>   population change, 174 Var-callees in one run; and stage 4 still
>   excludes the nine-capture closure, so it needs relaxing BOTH gates —
>   which destroys UBE) and (c) re-plumb upstream to present a
>   LexicalClosure callee (making the route lie about its shape; still
>   hits the empty-captures wall).
>
> INTENT UNCHANGED: populate the captured-env schema with n children +
> derived, sourced-not-fabricated owners at zero new trust. Only the
> MECHANISM-NAME is corrected — "extend UnitBoundaryEnvironment (same role,
> not a sibling)" becomes "issue a distinct checked-IH captured-env
> record (a sibling; derivation reused)". This supersedes the title's
> mechanism clause, the refined-slice TIER 2 bullet (including its "NOT a
> sibling role" lifetime-parameter note — the value-sourced lifetime now
> rides the distinct record), the "same role / extend the right role"
> objective, TIERS step 2, and AC-SCHEMA, wherever they differ.
> Tier 1, the zero-trust / sourced-from-source / no-code-identity ACs, and
> AC-ENTRY are unchanged. Steward-transcribed from the Architect ruling; not a
> re-scope of intent. The implementer is already resuming on (a) — this is not a
> hold.
>
> TEST (discriminating pair, Architect): the dedicated issuance MUST
> fire for the checked-IH family (nine children, each a derived owner)
> AND must NOT fire for a program UnitBoundaryEnvironment already serves
> — a non-degenerate pair, not a
> single positive.
>
> # SCHEMA UN-FUSED + SCOPE GAP FOUND 2026-08-23 (Architect evt_497awrccwy20k).
> # The 3rd tier-2 hard-stop is NOT a representability fork — it is a scope gap.
>
> Runtime-implementer hard-stop (evt_2n78cwm2ycqfy) + leader escalation
> (evt_2jj1f4k6ps6w3) raised an a/b/c representability fork: "AC-SCHEMA requires
> declared_children: Some(occurrences) but SynthesizedAggregateNode has no
> occurrence variant." Architect ruling (evt_497awrccwy20k): NOT a/b/c, no new
> type variant. The premise fused two fields. `declared_children` is the STATIC
> SHAPE TREE (kinds); occurrences live in `children[i].origin` (already set by
> branch 4) and per-child identity in `children[i].field_identity`. See the
> un-fused AC-SCHEMA below. The (c) DOMAIN ruling (evt_7arz3220scjs3) stands
> untouched; this is the CONTENTS axis. AC-M6-UNBLOCK is UNCHANGED — satisfied
> exactly when declared_children and field_identity are both Some.
>
> THE SCOPE GAP (Steward-owned dispatch): the slice frames TIERS 1+2, but only
> the runtime tier-2 half was built (5d1531dc). TIER 1 — the ken-elaborator
> captured-environment semantic emit (COORDINATION §9a language spillover) — was
> NEVER dispatched, so tier-2 reached for the planner-internal WorkerCapture
> coordinate run as a proxy and correctly refused to fabricate, leaving
> declared_children/field_identity None (a correct fail-closed, not a defect). The
> runtime "tier 1" at 9b50d63c is the coordinate run (ken-runtime), NOT the
> design's TIER 1. TWO never-assigned pieces to dispatch: (1) the tier-1
> elaborator emit (language-track, reviewers language-QA + language-leader); (2)
> the runtime `capture_field_identity` producer. The runtime-implementer is
> correctly NOT to build the elaborator half. Packaging call (Steward): land the
> implementer's correct-and-incomplete four-gap respin as a tier-2 PARTIAL now
> (declared_children/field_identity stay None, fail-closed, not frozen as final —
> the completed tier-2 will source both from tier-1) OR hold for the single
> complete slice. Dispatch + sequencing pending (cross-lane: this is runtime-lane
> priority needing ken-elaborator work while the language ring is on the
> module/import campaign — an operator sequencing question).
>
> # RELEASED 2026-08-23 — AC-ENTRY satisfied (enclave GO); refined slice below
> # (mechanism now per the DESIGN CORRECTED banner above where they differ)
>
> The enclave ruled AC-ENTRY GO (spec-author evt_46jrmz0ktsg9n,
> confirmed spec-leader evt_6yr2xardwcza1): reifying this population as
> a record of per-capture occurrences HAS a value-domain denotation and
> preserves it with zero new trust, under the live-domain sanction
> `41-values.md:76-118` — measurement 2 showed the escape is
> ActivationOwned / InvocationAggregate (live-domain, NOT the durable
> lane), so the durable-lane prohibition is not tripped. The per-capture
> NEED/seat trace (measurement 1) is NOT an AC prerequisite; it is an
> implementation instrument the build MAY commission from
> runtime-implementer if tier 1 cannot derive + validate the ci<->oi
> bijection from what it has.
>
> The Architect handed the refined slice shape (evt_710975vkbjqt),
> settling the lifetime call. RELEASED to the runtime ring; the
> runtime-leader sequences it AHEAD of ABI-M1 at the next
> increment/hard-stop boundary — M6 tier-3 depends on it (critical
> path). Below supersedes the pre-release framing where they differ.
>
> REFINED DESIGN (folds into the tiers + acceptance below):
> - TIER 1 (elaborator): emit the per-capture occurrences AS the exact,
>   order- and multiplicity-preserving bijection ci <-> oi (ci =
>   StaticWorker capture vector c0..cn-1 in binding order; oi = its
>   sourced checked-plan occurrence). The occurrence is FIELD-IDENTITY
>   AUTHORITY ONLY — it neither interprets nor reconstructs the carried
>   word. Tier 1 must DERIVE and MECHANICALLY VALIDATE the bijection at
>   its construction site; FAIL-CLOSED on any capture whose oi cannot be
>   sourced (never a fabricated label).
> - TIER 2 (planner): extend UnitBoundaryEnvironment to issue the
>   occurrence with declared_children = the sourced oi run, plus a
>   VALUE-SOURCED LIFETIME. LIFETIME CALL SETTLED (Architect ruling,
>   confirmed by measurement 2 + the GO): record lifetime (meet +
>   allocation) = ActivationOwned / InvocationAggregate, SOURCED FROM the
>   reified value — NOT the hard-coded Persistent / PersistentGround the
>   empty-captures issuance carries (`aggregates.rs` ~1341). MECHANISM: a
>   LIFETIME PARAMETER on UnitBoundaryEnvironment (value-sourced), NOT a
>   sibling role — role identity unchanged, the empty-captures population
>   keeps its current lifetime, a sibling would fracture the sealed set
>   for one lifetime-parameterized concept. No runtime code-identity tag.
>   [SUPERSEDED by the DESIGN CORRECTED banner: the record is now a
>   DISTINCT sibling, so the value-sourced lifetime rides that record, not
>   a parameter on UBE. The value-sourced lifetime and no-code-identity
>   requirements are unchanged.]
> - TIER 3 (M6 proper, HELD): Record{occurrence, fields=captures},
>   defunctionalize; the static dispatcher projects the same ordered run
>   <v0..vn-1>.
>
> FOLDED AC (sourced-from-source invariant + the enclave's zero-trust
> conditions) — every property the record carries is ADMITTED FROM its
> source, never hard-coded/invented: dispatch identity = plan template
> ids; per-field identity = capture occurrences via the ci<->oi
> bijection; lifetime = the reified value's meet/allocation. FAIL-CLOSED
> on any unsourced occurrence (matches the landed "None is a REFUSAL",
> `mod.rs:3226/3397`). NO code/environment identity exposure (no
> code-identity tag; no source-level projection/equality/hash/provenance/
> env-identity; callable edge opaque; only ordinary captured values in an
> existing live Record lane). ZERO TCB/trusted-base growth (semantic
> object elaborator/ planner-internal; runtime record uses the existing
> Record/InvocationAggregate class; tier 3 is ordinary backend downstream
> of kernel checking; correctness stays TESTED by the existing
> native/interpreter checked-family differential `42 §3.7` / `45 §4` — a
> bug yields a wrong value, never a false proof). No /spec edit (instance
> of landed law: `41 §2.1`, `42 §3.1`, `45 §2`/`§4`).
>
> CONTROLS: the exact-occurrence-bijection control + the
> no-code-identity structural control, ALONGSIDE the existing end-to-end
> checked-family differential.
>
> Architect review on the built predecessor: the ONE sourced-from-source
> invariant (every record property traces to its authority;
> ci<->oi order+multiplicity preserving; fail-closed on unsourced; no
> code-identity tag; lifetime value-sourced). Then M6 tier-3 unblocks;
> AC-REENUM (both checked-family programs green) + Adversary over-accept
> hunt in parallel.

# WHY THIS NODE EXISTS (the Case-C reach fork)

M6 ([[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]]) defunctionalizes the
escaping checked functional IH: the environment must cross the effect-seat
boundary as an admitted value-domain Record. The M6 build measured that
environment at the nullary_force seam and it is NOT empty:

- `env_binding = StaticWorker(declared_arity=1, captures=9)` — nine
  captures, the free variables the worker (body_origin
  `StaticOriginId(694)`) closed over.

The existing admitted-environment machinery cannot carry it, BY
CONSTRUCTION:

- `unit_boundary_environment_occurrence` issues records with
  `declared_children: Some(&[])` (planning/static_transition/aggregates.rs
  ~1319-1340), documented "Empty captures are the bounded first
  population: the record has no fields, so no compiler-created
  field-name authority is needed or inferred." It withholds per-field
  identity authority on purpose.
- `LoweredRecordField.identity: None` is documented "a REFUSAL, never a
  default — emitting a name would mean inventing one." So the captures
  cannot be wrapped in an ad-hoc Record with invented field names; that
  fails closed at preflight.

This is a missing capability (Case C), not an unreached one. Alternatives
were checked and correctly barred (each compiles and each is the barred
defect in a different costume): repurposing the Constructor role needs
constructor identity (invention); carrying captures as the worker's own
`StaticWorkerBinding` is the original defect (compiler metadata that
never becomes a value).

# THE RULED DESIGN SHAPE (Architect evt_2e11sk1jvp8mv — not an open fork)

Admit the captured environment as a value-domain Record, extending the
`UnitBoundaryEnvironment` role from empty to n declared children — and
each child's per-field identity is SOURCED FROM THAT CAPTURE'S OWN
OCCURRENCE in the checked plan (the free variable the StaticWorker
closed over), NOT fabricated. This mirrors exactly how the Constructor
role already sources its children (`aggregates.rs` ~1311,
`declared_children: Some(semantic_use.children)` — children come from a
semantic use in the program). It is admitted-not-invented precisely
because the identities trace to the captures' occurrences, so it does
not breach the "None is a refusal, emitting a name would invent one"
bar — the name is not invented, it is the capture's own occurrence.
Standard defunctionalization / closure conversion (Reynolds;
Minamide/Morrisett/Harper POPL96): the escaping closure's environment
becomes a record whose fields are its captured free variables. Same
role (it IS the unit-boundary environment), extended bound, sourced
identities. A new role would need its own identity authority anyway and
would fracture the sealed set — extend the right role.

[SUPERSEDED by the DESIGN CORRECTED banner (Architect
evt_613jzw7qc56p7). Live instrumentation falsified "same role": the
checked-IH route never enters the UnitBoundaryEnvironment concept
(Var->StaticWorker callee, nine captures, vs UBE's LexicalClosure-callee
+ `captures.is_empty()` gate). It is a DISTINCT sibling population;
merging them would destroy UBE. What is reused is the child-owner
DERIVATION (`aggregate_child_referent_owners`), not the role. The
sourced-not-invented identities argument in this paragraph is
unchanged.]

# TIERS AND OWNERSHIP (this slice = tiers 1+2; tier 3 is M6 proper)

1. **ELABORATOR (COORDINATION section 9a language spillover, built in
   this runtime slice).** Emit the checked-IH captured environment as a
   semantic object — the n captures with their per-capture occurrences.
   This IS the field-identity authority; it belongs to the elaborator
   because the semantic/occurrence layer that feeds
   `semantic_use.children` is the elaborator's, and the elaborator built
   the StaticWorker and knows the body's free variables. Required
   reviewers on the elaborator diff: language-QA + language-leader.
2. **RUNTIME PLANNER.** Issue a DEDICATED checked-IH captured-env
   occurrence with `declared_children: Some(the capture occurrences)`,
   consuming tier 1, reusing the Constructor-branch child-owner
   DERIVATION (`aggregate_child_referent_owners`). (Corrected from
   "extend UnitBoundaryEnvironment" per the DESIGN CORRECTED banner —
   evt_613jzw7qc56p7; distinct record, not the UBE entry gate.)
3. **RUNTIME LOWERING = M6 proper, NOT in this slice.** Build
   `Record { occurrence, fields = captures }` at `core.rs:11674` and
   defunctionalize with the two-tier dispatcher. Sequenced after this
   slice lands. (M6's identity/dispatch design stands, PR #2802.)

Packaged as ONE slice (tiers 1+2) rather than two: tier 2 consumes
exactly what tier 1 emits, so building and testing them together avoids
landing an elaborator interface the planner then has to adapt to.

# ACCEPTANCE

- **AC-ENTRY (SATISFIED 2026-08-23 — enclave GO, see the release banner
  above).** The spec enclave (spec-author -> spec-leader) confirms: the
  checked-IH's captured environment (the StaticWorker's n captures, free
  variables of the body at `StaticOriginId(694)`) has a value-domain
  denotation such that reifying it as a record of per-capture occurrences
  preserves the checked computation's denotation with ZERO new trust,
  under the same live-domain sanction `spec/40-runtime/41-values.md:76-118`
  gives closure exchange. YES => tier 1 may emit; build proceeds. If some
  captures have NO value-domain denotation (genuinely compiler-private
  state) => STOP, name which, return to enclave + Architect (a
  representability fork), never emit. Whether the confirmation also lands
  as a `41-values.md` erratum or stays an in-slice check is the
  Steward's packaging call once the enclave's answer is seen.
- **AC-SCHEMA (UN-FUSED per the SCHEMA UN-FUSED banner —
  Architect evt_497awrccwy20k).** A DEDICATED checked-IH captured-env
  record (distinct from `UnitBoundaryEnvironment`, unless a grep in the
  handoff proves every UBE consumer is capture-count-agnostic) issues an
  occurrence carrying THREE correctly-located fields — the first ruling's
  "declared_children: Some(occurrences)" was a FUSION defect (Architect
  owns it); occurrences do not live in `declared_children`:
  - `declared_children: Some(semantic_use.children)` — the STATIC SHAPE
    TREE (kinds only), sourced from the TIER-1 elaborator captured-env
    semantic use, byte-for-byte the branch-2 Constructor precedent
    (`aggregates.rs:1388`). NOT the occurrence carrier.
  - `children[i].origin: Some(oi)` — the capture's checked-plan
    occurrence. This is where "entries are the captures' own occurrences"
    actually lives, and branch 4 (`aggregates.rs:1450`) ALREADY sets it.
  - `children[i].field_identity: Some(FieldIdentity)` — per-child
    identity sourced NON-fabricated from the capture's own `CaptureSymbol`
    atom, via a new producer
    `capture_field_identity(origin, position) = FieldIdentity(identity_span(origin, CaptureSymbol, position))`.
    There is deliberately no `&str -> FieldIdentity` path
    (`lowering/aggregates.rs:914-921`), so fabrication is structurally
    impossible.
  Owners still come from `aggregate_child_referent_owners`
  (aggregates.rs:295) — the DERIVATION reused, not the UBE entry gate;
  owners are NOT meet/allocation assignments. Discriminating-pair test:
  the issuance fires for the checked-IH family (nine children, each a
  derived owner) and does NOT fire for a program `UnitBoundaryEnvironment`
  already serves.
- **AC-M6-UNBLOCK.** With the slice landed, M6's tier-3 lowering can
  build `Record { occurrence, fields = captures }` at `core.rs:11674`
  — i.e. the escaped environment now has an admitted representation.
  (Verified by M6, not here.)
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section
  12). Local targeted `-p` only, never `--workspace`.
- **Required reviewers.** Architect (soundness of the environment
  admission — the identities trace to real occurrences, no invention, the
  sealed set is not fractured) + language-QA/language-leader (the tier-1
  elaborator diff) + the Adversary (invention-in-costume: a fabricated
  occurrence, a widened `declared_children` for a population the concept
  did not model, a Constructor repurpose).

# SEQUENCING

Runtime lane-1, ahead of M6's tier-3 (M6 `depends_on` this node). BUILD
gated on AC-ENTRY (enclave). Capability tier T1 (a soundness-bearing
elaborator emit, reviewed on the argument).

SCOPE REVISED DOWN (Architect evt_497awrccwy20k): tier-1 is SMALLER than
the earlier "plausibly XL" — it is NOT a new semantic layer. The
field-identity machinery already exists (`CaptureSymbol` is a
`SemanticAtomKind`; `FieldIdentity` is a transparent newtype over a
named-atom span; `identity_span` exists). Tier-1 is: emit a
captured-environment semantic use whose children reuse the existing
`CaptureSymbol` atoms, plus the one new `capture_field_identity` producer.
The runtime tier-2 planner half already sources `children[i].origin`
(branch 4); what remains is consuming a tier-1 `semantic_use.children` for
`declared_children` and setting `field_identity` from the new producer.
Size S-M for tier-1; the tier-2 completion is a small consume-and-wire on
top of the landed 5d1531dc structure.
