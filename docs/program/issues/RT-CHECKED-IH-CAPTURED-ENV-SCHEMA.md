---
id: RT-CHECKED-IH-CAPTURED-ENV-SCHEMA
title: "Admit the checked-IH's non-empty captured environment as a value-domain Record with a planner-issued schema — extend the UnitBoundaryEnvironment role from empty captures to n declared children whose per-field identities are SOURCED from each capture's own occurrence in the checked plan (not fabricated), so the escaping checked functional IH has an admitted environment to cross the effect-seat boundary. The predecessor M6 (RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION) lowering-proper depends on. Tiers 1 (elaborator emit of the captured environment as a semantic object with per-capture occurrences, a COORDINATION section 9a language spillover) + 2 (runtime planner extends UnitBoundaryEnvironment to issue an occurrence with those declared children, mirroring the Constructor branch)."
status: ready
owner: runtime
size: L
gate: none
depends_on: []
blocks: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
github: null
origin: "Steward, 2026-08-22, cutting the Case-C reach-fork predecessor from the Architect's ruling (evt_2e11sk1jvp8mv) on RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION (M6). The M6 build measured the escaped IH environment NON-EMPTY (StaticWorker declared_arity=1, captures=9) and the existing admitted-env concept bounded to empty captures by construction (implementer evt_svjnmypxr4j5, leader evt_4jg3k6r75mhvz). Design shape ruled by the Architect; BUILD gated on the enclave denotational-faithfulness confirmation (AC-ENTRY). Steward-filed per COORDINATION section 2. HS 5."
---

> # RELEASED 2026-08-23 — AC-ENTRY satisfied (enclave GO); refined slice below
>
> The enclave ruled AC-ENTRY GO (spec-author evt_46jrmz0ktsg9n, confirmed
> spec-leader evt_6yr2xardwcza1): reifying this population as a record of
> per-capture occurrences HAS a value-domain denotation and preserves it with zero
> new trust, under the live-domain sanction `41-values.md:76-118` — measurement 2
> showed the escape is ActivationOwned / InvocationAggregate (live-domain, NOT the
> durable lane), so the durable-lane prohibition is not tripped. The per-capture
> NEED/seat trace (measurement 1) is NOT an AC prerequisite; it is an
> implementation instrument the build MAY commission from runtime-implementer if
> tier 1 cannot derive + validate the ci<->oi bijection from what it has.
>
> The Architect handed the refined slice shape (evt_710975vkbjqt), settling the
> lifetime call. RELEASED to the runtime ring; the runtime-leader sequences it
> AHEAD of ABI-M1 at the next increment/hard-stop boundary — M6 tier-3 depends on
> it (critical path). Below supersedes the pre-release framing where they differ.
>
> REFINED DESIGN (folds into the tiers + acceptance below):
> - TIER 1 (elaborator): emit the per-capture occurrences AS the exact, order- and
>   multiplicity-preserving bijection ci <-> oi (ci = StaticWorker capture vector
>   c0..cn-1 in binding order; oi = its sourced checked-plan occurrence). The
>   occurrence is FIELD-IDENTITY AUTHORITY ONLY — it neither interprets nor
>   reconstructs the carried word. Tier 1 must DERIVE and MECHANICALLY VALIDATE the
>   bijection at its construction site; FAIL-CLOSED on any capture whose oi cannot
>   be sourced (never a fabricated label).
> - TIER 2 (planner): extend UnitBoundaryEnvironment to issue the occurrence with
>   declared_children = the sourced oi run, plus a VALUE-SOURCED LIFETIME. LIFETIME
>   CALL SETTLED (Architect ruling, confirmed by measurement 2 + the GO): record
>   lifetime (meet + allocation) = ActivationOwned / InvocationAggregate, SOURCED
>   FROM the reified value — NOT the hard-coded Persistent / PersistentGround the
>   empty-captures issuance carries (`aggregates.rs` ~1341). MECHANISM: a LIFETIME
>   PARAMETER on UnitBoundaryEnvironment (value-sourced), NOT a sibling role — role
>   identity unchanged, the empty-captures population keeps its current lifetime, a
>   sibling would fracture the sealed set for one lifetime-parameterized concept. No
>   runtime code-identity tag.
> - TIER 3 (M6 proper, HELD): Record{occurrence, fields=captures}, defunctionalize;
>   the static dispatcher projects the same ordered run <v0..vn-1>.
>
> FOLDED AC (sourced-from-source invariant + the enclave's zero-trust conditions) —
> every property the record carries is ADMITTED FROM its source, never
> hard-coded/invented: dispatch identity = plan template ids; per-field identity =
> capture occurrences via the ci<->oi bijection; lifetime = the reified value's
> meet/allocation. FAIL-CLOSED on any unsourced occurrence (matches the landed
> "None is a REFUSAL", `mod.rs:3226/3397`). NO code/environment identity exposure
> (no code-identity tag; no source-level projection/equality/hash/provenance/
> env-identity; callable edge opaque; only ordinary captured values in an existing
> live Record lane). ZERO TCB/trusted-base growth (semantic object elaborator/
> planner-internal; runtime record uses the existing Record/InvocationAggregate
> class; tier 3 is ordinary backend downstream of kernel checking; correctness
> stays TESTED by the existing native/interpreter checked-family differential
> `42 §3.7` / `45 §4` — a bug yields a wrong value, never a false proof). No /spec
> edit (instance of landed law: `41 §2.1`, `42 §3.1`, `45 §2`/`§4`).
>
> CONTROLS: the exact-occurrence-bijection control + the no-code-identity
> structural control, ALONGSIDE the existing end-to-end checked-family differential.
>
> Architect review on the built predecessor: the ONE sourced-from-source invariant
> (every record property traces to its authority; ci<->oi order+multiplicity
> preserving; fail-closed on unsourced; no code-identity tag; lifetime
> value-sourced). Then M6 tier-3 unblocks; AC-REENUM (both checked-family programs
> green) + Adversary over-accept hunt in parallel.

# WHY THIS NODE EXISTS (the Case-C reach fork)

M6 ([[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]]) defunctionalizes the escaping
checked functional IH: the environment must cross the effect-seat boundary as an
admitted value-domain Record. The M6 build measured that environment at the
nullary_force seam and it is NOT empty:

- `env_binding = StaticWorker(declared_arity=1, captures=9)` — nine captures, the
  free variables the worker (body_origin `StaticOriginId(694)`) closed over.

The existing admitted-environment machinery cannot carry it, BY CONSTRUCTION:

- `unit_boundary_environment_occurrence` issues records with
  `declared_children: Some(&[])` (planning/static_transition/aggregates.rs
  ~1319-1340), documented "Empty captures are the bounded first population: the
  record has no fields, so no compiler-created field-name authority is needed or
  inferred." It withholds per-field identity authority on purpose.
- `LoweredRecordField.identity: None` is documented "a REFUSAL, never a default —
  emitting a name would mean inventing one." So the captures cannot be wrapped in
  an ad-hoc Record with invented field names; that fails closed at preflight.

This is a missing capability (Case C), not an unreached one. Alternatives were
checked and correctly barred (each compiles and each is the barred defect in a
different costume): repurposing the Constructor role needs constructor identity
(invention); carrying captures as the worker's own `StaticWorkerBinding` is the
original defect (compiler metadata that never becomes a value).

# THE RULED DESIGN SHAPE (Architect evt_2e11sk1jvp8mv — not an open fork)

Admit the captured environment as a value-domain Record, extending the
`UnitBoundaryEnvironment` role from empty to n declared children — and each
child's per-field identity is SOURCED FROM THAT CAPTURE'S OWN OCCURRENCE in the
checked plan (the free variable the StaticWorker closed over), NOT fabricated.
This mirrors exactly how the Constructor role already sources its children
(`aggregates.rs` ~1311, `declared_children: Some(semantic_use.children)` — children
come from a semantic use in the program). It is admitted-not-invented precisely
because the identities trace to the captures' occurrences, so it does not breach
the "None is a refusal, emitting a name would invent one" bar — the name is not
invented, it is the capture's own occurrence. Standard defunctionalization /
closure conversion (Reynolds; Minamide/Morrisett/Harper POPL96): the escaping
closure's environment becomes a record whose fields are its captured free
variables. Same role (it IS the unit-boundary environment), extended bound,
sourced identities. A new role would need its own identity authority anyway and
would fracture the sealed set — extend the right role.

# TIERS AND OWNERSHIP (this slice = tiers 1+2; tier 3 is M6 proper)

1. **ELABORATOR (COORDINATION section 9a language spillover, built in this runtime
   slice).** Emit the checked-IH captured environment as a semantic object — the n
   captures with their per-capture occurrences. This IS the field-identity
   authority; it belongs to the elaborator because the semantic/occurrence layer
   that feeds `semantic_use.children` is the elaborator's, and the elaborator built
   the StaticWorker and knows the body's free variables. Required reviewers on the
   elaborator diff: language-QA + language-leader.
2. **RUNTIME PLANNER.** Extend `UnitBoundaryEnvironment` to issue an occurrence
   with `declared_children: Some(the capture occurrences)`, consuming tier 1,
   mirroring the Constructor branch.
3. **RUNTIME LOWERING = M6 proper, NOT in this slice.** Build
   `Record { occurrence, fields = captures }` at `core.rs:11674` and defunctionalize
   with the two-tier dispatcher. Sequenced after this slice lands. (M6's
   identity/dispatch design stands, PR #2802.)

Packaged as ONE slice (tiers 1+2) rather than two: tier 2 consumes exactly what
tier 1 emits, so building and testing them together avoids landing an elaborator
interface the planner then has to adapt to.

# ACCEPTANCE

- **AC-ENTRY (SATISFIED 2026-08-23 — enclave GO, see the release banner above).**
  The spec enclave
  (spec-author -> spec-leader) confirms: the checked-IH's captured environment
  (the StaticWorker's n captures, free variables of the body at
  `StaticOriginId(694)`) has a value-domain denotation such that reifying it as a
  record of per-capture occurrences preserves the checked computation's denotation
  with ZERO new trust, under the same live-domain sanction
  `spec/40-runtime/41-values.md:76-118` gives closure exchange. YES => tier 1 may
  emit; build proceeds. If some captures have NO value-domain denotation
  (genuinely compiler-private state) => STOP, name which, return to enclave +
  Architect (a representability fork), never emit. Whether the confirmation also
  lands as a `41-values.md` erratum or stays an in-slice check is the Steward's
  packaging call once the enclave's answer is seen.
- **AC-SCHEMA.** The extended `UnitBoundaryEnvironment` issues an occurrence with
  `declared_children: Some(...)` whose entries are the captures' own occurrences —
  no fabricated field name, no `identity: None` field on this path. The
  Constructor-branch admitted-children pattern is reused, not duplicated.
- **AC-M6-UNBLOCK.** With the slice landed, M6's tier-3 lowering can build
  `Record { occurrence, fields = captures }` at `core.rs:11674` — i.e. the escaped
  environment now has an admitted representation. (Verified by M6, not here.)
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12). Local
  targeted `-p` only, never `--workspace`.
- **Required reviewers.** Architect (soundness of the environment admission — the
  identities trace to real occurrences, no invention, the sealed set is not
  fractured) + language-QA/language-leader (the tier-1 elaborator diff) + the
  Adversary (invention-in-costume: a fabricated occurrence, a widened
  `declared_children` for a population the concept did not model, a Constructor
  repurpose).

# SEQUENCING

Runtime lane-1, ahead of M6's tier-3 (M6 `depends_on` this node). BUILD gated on
AC-ENTRY (enclave). Capability tier T1 (a new admitted-schema capability + a
soundness-bearing elaborator emit, reviewed on the argument). Size L (may be XL;
the Architect flagged tiers 1+2 as plausibly larger than M6's own L).
