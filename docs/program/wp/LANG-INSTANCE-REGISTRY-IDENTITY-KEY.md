# WP frame — `LANG-INSTANCE-REGISTRY-IDENTITY-KEY`

    owner   language       tier   T1        size   M
    depends none
    node    docs/program/issues/LANG-INSTANCE-REGISTRY-IDENTITY-KEY.md
    review  Architect REQUIRED. Code merge → full CI + M8 Adversary.

## 1. Objective

First establish whether a surface-program carrier spelling can select an
instance dictionary for a different carrier identity at a consumer that lacks
A1's core confirmation. If it cannot, close the node with the measured negative
result. If it can, route the exhibit to the Architect before implementing either
possible repair.

This message frames only D0. It does not authorize a registry re-key or a
confirmation extension.

## 2. Fixed inputs

Base: `dcbb9648f39ac8d9dd8698635018b392e5a58978`.

At this base, `ClassEnv::instances` is keyed by `(String, String)` at
`crates/ken-elaborator/src/classes.rs:202`; `instance_search` consumes the
class and head spellings at `:327`. `rtype_head_name` returns the spelling of an
`RVarTy` at `crates/ken-elaborator/src/elab.rs:9598`, so an abstract type
parameter can be used as a registry head spelling.

A1's `resolve_instance_dictionary_from_identity` already scans forward and
confirms the resolved dictionary's carrier in core. That path is not the gap.
The D0 population is exactly the four carrier-head consumers with no such
confirmation:

    elab.rs:9811   declaration/surface entry
    elab.rs:10087  constraint head during instance construction
    elab.rs:10455  first projected-instance lookup
    elab.rs:10466  second projected-instance lookup

`elab.rs:10551` is excluded: it keys a class name, not a carrier head.
Reconfirm every coordinate and its enclosing function at the adopted base. A
false coordinate is a stop-and-report condition, not a reason to widen the
population from a grep.

## 3. D0 procedure

Try the `RVarTy` spelling route first at the declaration/surface entry:
`where Ord a` must not be able to use an abstract `a` whose spelling collides
with a registered concrete head and elaborate that concrete dictionary into the
term. The input must be checked Ken source a user can write; directly inserting
a registry entry or using an otherwise-unreachable fixture does not count.

If that path cannot exhibit, investigate the remaining three named consumers,
without treating the two projection sites as an effect-row deliverable. Their
effect-row consequence belongs to `LANG-INSTANCE-SEARCH-SECOND-PATH`; here the
question is solely wrong instance selection by a carrier spelling.

Before each probe, state what observation ends it: either an elaborated term
whose selected dictionary identity differs from the carrier identity, or an
input-specific refusal that makes the candidate mechanism unavailable. Do not
substitute an absence-of-injectivity argument for either observation.

## 4. Acceptance

**AC-D0-1 — surface exhibit or measured closure.** At the named base, either
provide a checked surface source whose elaborated term contains a selected
instance dictionary whose identity is not the occurrence carrier's identity, or
report that no such exhibit is reachable at all four named consumers. A unit
test that writes `ClassEnv::instances` directly is not evidence.

**AC-D0-2 — site attribution pair.** For any exhibit, name its consumer and
selected dictionary identity, and send the same carrier identity through
`resolve_instance_dictionary_from_identity`. The latter must refuse via A1's
core confirmation. This distinguishes a missing confirmation from a genuinely
invalid carrier.

**AC-D0-3 — no detector substitution.** If D0 exhibits and a later repair is
authorized, preserve reachability of A1's
`InstanceHeadSpellingsShareAnIdentity` two-match refusal. The two-names-to-one-id
ambiguity detector is distinct from the two-ids-to-one-name substitution D0
seeks.

## 5. Stop conditions and routing

Stop and hand back if an exhibit requires direct registry editing, is not
surface-writable, or a released coordinate is false. If D0 does not exhibit,
close this node; that is the successful outcome.

If D0 exhibits, route the source, elaborated identity evidence, named consumer,
and A1 comparison to the Architect. The Architect alone chooses between
re-keying on `GlobalId` and extending core confirmation. Do not build either
mechanism before that ruling.

## 6. Boundaries

Do not change A1's landed three-step resolver. Do not fold the projection
sites' effect-row question from `LANG-INSTANCE-SEARCH-SECOND-PATH`, or the
`InstanceInfo` visibility work from `LANG-R-LAYER-EXPORT-RETRACTION`, into this
WP. Do not change coherence, orphan, re-export, or derive behavior while D0 is
running.

Use only targeted `scripts/ken-cargo` runs; CI owns full-workspace validation.

## 7. As-built outcome

D0 exhibited the substitution at the construction prerequisite: a surface
spelling selected an old `Pick Foo` dictionary for a different `Foo` carrier,
while A1 refused the same carrier pair with
`InstanceCarrierIdentityMismatch`. That established a missing carrier
confirmation rather than an invalid carrier.

The landed repair is Architect-authorized arm (b), not a registry re-key. At
squash `83f30f5e8526a357785a0f82681c862dfbff6b77` from candidate
`d7eafb4b463dcddf2b89a5e89f9c8e448de161ba`,
`InstanceHeadRequest` carries an independently derived expected core carrier.
`resolve_instance_dictionary` derives it in the occurrence context, and the
construction-prerequisite path derives it by substituting stored constraint
core types with the selected outer instance's core arguments.

`confirm_instance_dictionary_carrier` is the single common return-path check.
For carrier-parameterized classes it compares the kernel-inferred candidate
carrier with that expected core carrier after `kernel_infer_raw` and before
provenance append or dictionary return. Nullary classes retain their existing
behavior. A1 delegates its STEP 3 to the same check while preserving its
selection and ambiguity refusals.

The rejected re-key remains informative. `ClassEnv::instances` admits named,
variable, and structural heads, including applications, universes, arrows,
Sigma, refinements, and headless projections. A bare `(class GlobalId, head
GlobalId)` key cannot represent that population. Making it honest would require
a closed head-key algebra plus wildcard and overlap rules, changing registry
semantics outside this WP. Surface spelling is therefore a candidate hint; core
carrier identity remains the authority for every term-producing return.
