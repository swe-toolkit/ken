---
id: RT-CHECKED-IH-CAPTURED-ENV-SCHEMA
title: "Admit the checked-IH's non-empty captured environment as a value-domain Record with a planner-issued schema — extend the UnitBoundaryEnvironment role from empty captures to n declared children whose per-field identities are SOURCED from each capture's own occurrence in the checked plan (not fabricated), so the escaping checked functional IH has an admitted environment to cross the effect-seat boundary. The predecessor M6 (RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION) lowering-proper depends on. Tiers 1 (elaborator emit of the captured environment as a semantic object with per-capture occurrences, a COORDINATION section 9a language spillover) + 2 (runtime planner extends UnitBoundaryEnvironment to issue an occurrence with those declared children, mirroring the Constructor branch)."
status: draft
owner: runtime
size: L
gate: none
depends_on: []
blocks: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
github: null
origin: "Steward, 2026-08-22, cutting the Case-C reach-fork predecessor from the Architect's ruling (evt_2e11sk1jvp8mv) on RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION (M6). The M6 build measured the escaped IH environment NON-EMPTY (StaticWorker declared_arity=1, captures=9) and the existing admitted-env concept bounded to empty captures by construction (implementer evt_svjnmypxr4j5, leader evt_4jg3k6r75mhvz). Design shape ruled by the Architect; BUILD gated on the enclave denotational-faithfulness confirmation (AC-ENTRY). Steward-filed per COORDINATION section 2. HS 5."
---

> # BUILD-GATED (draft) — do NOT release until the enclave confirms AC-ENTRY
>
> The DESIGN SHAPE is ruled (Architect evt_2e11sk1jvp8mv), so this node is cut and
> fully framed. Its BUILD does not start until the spec enclave answers the
> denotational-faithfulness question (AC-ENTRY below). The Steward frames + releases
> it once the enclave confirms; a "some captures have no value-domain denotation"
> answer is a deeper representability fork back to the enclave + Architect, not a
> build.

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

- **AC-ENTRY (BUILD GATE — enclave, clean-room semantics).** The spec enclave
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
