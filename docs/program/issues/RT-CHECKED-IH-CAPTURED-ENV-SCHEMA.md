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
> # TIER-1 IS KEN-RUNTIME, NOT KEN-ELABORATOR 2026-08-23 (Architect
> # evt_6h5ndf9hxf22f, confirming Steward measurement). Supersedes every
> # "elaborator / language-track / §9a spillover / language-QA+language-leader"
> # classification above and below (the SCHEMA UN-FUSED dispatch note, TIERS
> # step 1, and the AC Required-reviewers line).
>
> The dispatch note above assigned tier-1 to the LANGUAGE ring as a
> "ken-elaborator captured-env emit (COORDINATION §9a spillover)." That rests on
> the node's premise "the elaborator built the StaticWorker and knows the free
> variables" — FALSE at the code layer. Measured at origin/main 5a74301f4: the
> emit is entirely ken-runtime. StaticWorker is built ONLY in ken-runtime
> (cranelift_backend/lowering); CaptureSymbol is interned in ken-runtime
> semantic_ir.rs:2870 (from RuntimeExpr::Closure captures); FieldIdentity (:159),
> identity_span (:1657), flatten_allocation_reachable_uses, and the Constructor
> precedent declared_children: Some(semantic_use.children) (aggregates.rs:1312)
> are all ken-runtime; ken-elaborator carries NONE of this capture-semantic
> machinery (its only "capture" hits are capture_canonical_term, an unrelated
> byte-capture of canonical terms). The node's own scope-down ("reuse the
> EXISTING CaptureSymbol atoms; the machinery already exists; NOT a new layer")
> agrees.
>
> CORRECTED OWNERSHIP (Architect evt_6h5ndf9hxf22f): tier-1 is a RUNTIME-ring
> deliverable, NO ken-elaborator diff. The DELIVERABLE SHAPE was further revised
> to POSITIONAL by Architect evt_3tky5wzycfnxb (the capture_field_identity
> producer this banner first named is DROPPED; field_identity is positional
> None) — see the reframed AC-SCHEMA below for the operative deliverable (a new
> positional WorkerCaptureOperand variant + a separate reconcile arm). The
> ownership finding here stands; only the field-shape changed.
>
> REVIEWERS: Architect + runtime-QA + conformance-validator + Adversary (NO
> language-QA/language-leader — there is no ken-elaborator diff to review).
> PACKAGING (Architect): tier-1 + the tier-2 completion (declared_children/
> field_identity None -> Some, built on top of the landed tier-2 partial
> 2d9a96ad7) return to Architect + CV TOGETHER as the AC-SCHEMA / AC-M6-UNBLOCK
> discharge; tier-1 alone does not close the node. There is NO cross-lane
> conflict: the language ring stays on the module/import campaign (ken-elaborator)
> in parallel.
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
- **AC-SCHEMA (REFRAMED POSITIONAL — Architect evt_3tky5wzycfnxb, which
  REVISES the SCHEMA UN-FUSED / z161 ruling; the Architect owns the
  revision).** A DEDICATED checked-IH captured-env record — the distinct
  `CheckedIhCapturedEnvironment` role (role != shape; the distinct role
  preserves the blocker-2 distinct-record soundness guard) — issues a
  POSITIONALLY-identified aggregate carrying:
  - `declared_children: Some(...)` via a NEW positional
    `SynthesizedAggregateNode` variant (e.g. `WorkerCaptureOperand(u32)`;
    exact name at implementer/CV discretion) whose child at position i is
    "the i-th carried continuation-envelope WorkerCapture word." This is
    NOT a reuse of the Constructor `semantic_use.children` const-recipe nor
    the SiteOperand/effect-seat operand vector: the captured env is the
    FIRST synthesized aggregate whose children are neither a compile-time
    recipe nor an effect seat's arguments, so it needs its own kind (the
    original fork's option (a); z161's "reuse Some(semantic_use.children)
    like branch 2" was wrong). STORAGE via const-array-slice-by-arity —
    `const CAPTURE_OPERANDS: &[Node] = &[WorkerCaptureOperand(0)..(N_MAX-1)]`,
    per unit `&CAPTURE_OPERANDS[..arity]` (per-unit content at position i is
    fully determined by i, so per-unit arity is representable by slicing).
    REQUIRES an EXPLICIT arity-bound refusal above N_MAX — never silent
    truncation. RESOLUTION via a NEW `reconcile_declared_children` arm
    resolving `WorkerCaptureOperand(i)` against the continuation-envelope
    WorkerCapture operand vector (the ci<->oi run), with its OWN
    path-identity check, SEPARATE from and NOT touching/weakening the
    existing SiteOperand/effect-seat arm (that separation is what keeps the
    effect-seat path-identity contract every host-result aggregate relies
    on intact).
  - `children[i].origin: Some(oi)` — the capture's checked-plan
    occurrence; branch 4 (`aggregates.rs:1450`) ALREADY sets it.
  - `field_identity: None` is CORRECT and non-fabricated — the captured env
    is POSITIONALLY identified (the ci<->oi ordinal IS the identity). The
    record takes the POSITIONAL downstream path (Lowered::Constructor /
    `record_fields = None`), under which the preflight's field-identity
    comparison (`lowering/aggregates.rs:922-941`) is BYPASSED entirely
    (dispatch at :730-758) — exactly why branch 2's Constructor children
    carry `field_identity: None` and pass. The `capture_field_identity`
    producer is DROPPED; the Var-binder-ParamName route stays rejected (it
    would invent nominal identity). The earlier "no identity: None on this
    path" clause was the error.
  Owners come from `aggregate_child_referent_owners` (aggregates.rs:295).
  LEGITIMACY (CV + Adversary — the invention-in-costume line): the new kind
  is the concept the genuinely-new population REQUIRES, not a widened
  concept for a population it did not model; arity + content trace to the
  ci<->oi run resolved against the real envelope operand vector; the missing
  `&str -> FieldIdentity` path stays missing; the effect-seat reconcile arm
  is untouched. Auditable by: the new reconcile arm reds if pointed at the
  wrong operand vector; the arity bound reds above N_MAX rather than
  truncating; the discriminating pair fires for the checked-IH family and
  NOT for a program `UnitBoundaryEnvironment` already serves.
- **AC-M6-UNBLOCK.** With the slice landed, M6's tier-3 lowering builds the
  captured env as a POSITIONAL aggregate consumed by ordinal projection
  `<v0..vn-1>` (NOT a nominal `Record` / field-name lookup) at
  `core.rs:11674` — the escaped environment now has an admitted
  representation. (Verified by M6, not here.)
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section
  12). Local targeted `-p` only, never `--workspace`.
- **Required reviewers.** Architect (soundness of the environment
  admission — the identities trace to real occurrences, no invention, the
  sealed set is not fractured) + runtime-QA + conformance-validator (tier-1 is
  ken-runtime, NOT an elaborator diff — see the TIER-1 IS KEN-RUNTIME banner;
  Architect evt_6h5ndf9hxf22f) + the Adversary (invention-in-costume: a fabricated
  occurrence, a widened `declared_children` for a population the concept
  did not model, a Constructor repurpose).

# SEQUENCING

Runtime lane-1, ahead of M6's tier-3 (M6 `depends_on` this node). BUILD
gated on AC-ENTRY (enclave, satisfied). Capability tier T1 (soundness-bearing
ken-runtime type work, reviewed on the argument).

SCOPE CORRECTED BACK TO L/XL (Architect evt_3tky5wzycfnxb, REVISING the
earlier evt_497awrccwy20k "S-M / not a new layer" note): tier-1 is real
type work, NOT a reuse of existing machinery — a NEW sealed-vocabulary
`SynthesizedAggregateNode` variant (`WorkerCaptureOperand`) + a NEW
`reconcile_declared_children` arm (separate from the effect-seat arm) + the
arity-bound (N_MAX) machinery. The `capture_field_identity` producer is
DROPPED (field_identity is positional `None`; the ordinal is the identity).
Branch 4 already sources `children[i].origin`; what remains is the new
positional-variant storage + resolution + the positional/distinct-role
shape (Constructor-family / `record_fields = None`). Returns to Architect +
CV + Adversary together as the AC-SCHEMA / AC-M6-UNBLOCK discharge; tier-1
alone does NOT close the node.
