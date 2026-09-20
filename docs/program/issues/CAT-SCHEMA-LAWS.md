---
id: CAT-SCHEMA-LAWS
title: "Application/Input/Schema.ken.md owns the accumulation shape for BOTH of its clients and has zero proofs: prove the Valid-case indexed coverage law, the first-rejection head law that pins issue ORDER, and the law that an accepting field never masks a later rejection -- over the existing schema_validate_fields traversal, with the help traversal deliberately out of scope"
status: ready
owner: foundation
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 / PRINCIPLES #16. Selected and framed by the Steward 2026-09-20 as CAT-PROPERTY-LAWS was routed. Chosen by measuring the packages, not by reading the survey's one-line recommendations. CAT-JSON-LAWS was REJECTED as next: the decoder the survey's recommendation names is NOT in Data/Serialization/Json.ken.md, and of what IS there the four scalar json_size equations are refl, while the JsonArray and JsonObject equations cannot be stated at all without a companion fold -- 'recursive result for' is the nested induction hypothesis over List Json, not a self-call, so there is no existing term to equate against and a new function would be required. Schema was chosen instead because it is the shared precursor UNDER two other unframed follow-ons: both Application/Configuration/Decoder.ken.md and Application/CommandLine/ArgParse.ken.md import this traversal and neither can state its own accumulation law until this one exists. All current-code facts measured at origin/main 3f9243c2ae7870392e0713163fff9243f26657d2."
---

# Schema validation traversal laws

`Application/Input/Schema.ken.md` declares zero `theorem` and zero `proof`.
Every guarantee about the traversal that both clients run is external. Give
that traversal its general laws, over the representation the package already
ships.

Treat anchors as perishable. If a fixed input is false on the landed base, stop
and report the mismatch; do not build around it.

## Settled inputs -- measured at `3f9243c2a`. Do not re-derive.

**The traversal is one fold and one four-case combiner.**
`schema_validate_fields` (`:141`) recurses over `List SchemaField`, and every
decision about what the result carries is taken in `schema_validation_cons`
(`:110`), which is the only place in the package where issue order is decided:

```ken
match head {
  SchemaFieldAccepted accepted ↦
    match tail {
      Valid rest ↦ Valid ... (Cons value accepted rest);
      Invalid issues ↦ Invalid ... issues
    };
  SchemaFieldRejected issue ↦
    match tail {
      Valid rest ↦ Invalid ... (nonempty_cons ... issue (Nil ...));
      Invalid issues ↦
        Invalid ... (nonempty_append ... (nonempty_cons ... issue (Nil ...)) issues)
    }
}
```

**The structural equations of this traversal are VACUOUS -- do not deliver
them.** `schema_validate_fields inspect Nil = Valid (Nil value)` and the `Cons`
equation restating `schema_validation_cons` are both `refl`: they are the
definition. A law is only worth checking here if it quantifies over the field
list. This is the exact failure mode that got `CAT-CONSOLE-TEXT-LAWS` rejected
as a node, and it is why the three deliverables below are stated as
quantified characterizations and not as constructor equations.

**The precursors are landed, which is why this node is selected now.**
`nth::some_below_length` and `nth::at_or_beyond_is_none` are `pub proof`s on
`pub fn nth` (`Data/Collections/Derived.ken.md:91,177,191`), merged with
`CAT-COLLECTIONS-NTH-LAWS`. `nonempty_append` carries `proof assoc`
(`NonEmpty.ken.md:95`), and the survey classifies both `Data/Collections/
NonEmpty.ken.md` and `Data/Sums/Validation.ken.md` as fully proven.

**Two import widenings are AUTHORIZED, and nothing else is.** Both widen an
edge this package already declares; neither adds a module edge.

- `import Data.Collections.Derived (list_append)` (`:14`) to
  `(list_append, nth)`.
- `import Data.Collections.NonEmpty (NonEmpty, nonempty_append, nonempty_cons)`
  (`:16`) to add `nonempty_head`.

**The export posture here is NOT the one CAT-PROPERTY-LAWS used, and the
difference is load-bearing.** `Tooling/Testing/Property.ken.md` has no export
list at all, so its laws were correctly private. This package has zero `pub`
keywords but DOES carry an explicit `export` list (`:236`) that names
`schema_validate_fields` and `schema_validate`. Under the Architect's ruling of
2026-09-20 (`evt_798bkcv51sbv9`, on `spec/30-surface/33-declarations.md` §4.1
and §8.2) a module interface is its `pub` declarations plus its explicit
`export` declarations, and a `pub proof` is exportable when its subject is
exported. **These subjects are exported, so state the laws as public attached
proofs**, matching `pub proof round_trip for process_arguments`. Do not infer
the posture from the absence of the `pub` keyword; if the elaborator refuses a
`pub proof` on an `export`-listed subject, that is a stop condition below, not
a licence to quietly fall back to private.

**The external evidence being replaced** lives in
`crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs`, in
`two_missing_fields_accumulate_exact_environment_origins` (`:438`) and
`config_failures_keep_config_key_origins_distinct_from_environment` (`:473`).
Those tests stay; they stop being the guarantee.

**The census row for this package holds exactly 14 ambient names** in
`crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs:496`: `And`,
`Bottom`, `Equal`, `Prop`, `Proved`, `Top`, `Unit`, `and_fst`, `and_intro`,
`and_snd`, `eqChar`, `is_sorted`, `leqChar`, `map`.

**The help traversal is excluded, and the reason is measured, not editorial.**
`schema_help` and every `schema_*_chars` helper route through
`string_to_list_char`, which the survey classifies as a kernel-`Neutral`
primitive whose only source-level evidence is the retraction axiom in
`Data/Text/StringBijection.ken.md`. A claim about what help RENDERS is a claim
about that primitive. Naming the complement so the next framer does not read
this omission as an oversight: the structural half of help -- that
`schema_fields_help_chars` decomposes over `list_append` treating each
`string_to_list_char "..."` application as an opaque atom -- IS provable, and
belongs to `CAT-ARGPARSE-LAWS`, which is the node that actually needs it.

## Deliverable

Public attached proofs on `schema_validate_fields` stating three quantified
laws, over the existing traversal and carriers.

- **coverage** -- if `schema_validate_fields inspect fields = Valid values`,
  then for every index `i`, `nth SchemaField i fields = Some field` implies
  there is a `v` with `nth value i values = Some v` and
  `inspect field = SchemaFieldAccepted v`. Positional, not merely a count.
- **first rejection** -- if some field rejects, the result is
  `Invalid issues` and `nonempty_head` of `issues` is the issue of the
  EARLIEST rejecting field: recover an index `i`, the exact
  `nth SchemaField i fields = Some field`, `inspect field =
  SchemaFieldRejected issue`, `nonempty_head ... issues = issue`, and the
  implication that every strictly earlier index accepts.
- **no masking** -- an accepting field never turns an `Invalid` tail into a
  `Valid` result: if the tail is `Invalid issues`, then
  `schema_validation_cons head tail = Invalid issues` for either shape of
  `head`.

No new function, carrier, import edge, module, instance, primitive,
postulate, `Axiom`, or trusted entry. `trusted_base()` delta stays zero, and
no client package is touched.

## Acceptance criteria

**AC-1 -- coverage is positional, and an off-by-one reds.** A checked witness
instantiates the law on a three-field schema whose fields carry distinct
accepted values, recovering the value at index 1 through the law rather than by
computation. Mutation, restored byte-exact: change the law's stated conclusion
to read the value at `Suc i` instead of `i`, and it must RED. A law that pins
only the LENGTH of `values` passes a fixture and must not pass this.

**AC-2 -- the ORDER of accumulated issues must be SHOWN to be load-bearing.**
In `schema_validation_cons`, swap the two arguments of the `nonempty_append`
call in the `SchemaFieldRejected` + `Invalid` branch, so the tail's issues
precede the head's, and re-check. The first-rejection law MUST RED. Restore
byte-exact. If it still checks with the arguments swapped, the law is not
characterizing the FIRST rejection -- it is stating the far weaker "some field
rejected" -- and that is a FAILURE of this node, not a pass. Report the
diagnostic.

**AC-3 -- the no-masking law reaches the accept-over-Invalid arm.** Alter that
one arm of `schema_validation_cons` to return `Valid` with the accepted value
consed onto an empty list, and show the no-masking proof reds there
specifically. Restore byte-exact and report which arm was mutated.

**AC-4 -- the census row does not move.** Re-run
`catalog_ambient_passthrough_migration_census` and report the exact added and
removed names for `Application.Input.Schema` against the 14 named above. That
row MAY move -- report its exact delta and why. Any OTHER row moving is a
Steward stop.

## Stop condition

Hand back rather than work around if any holds:

- **The elaborator refuses `pub proof` on an `export`-listed subject that
  carries no `pub` keyword.** Report the exact diagnostic and stop. Do not
  silently deliver private proofs instead: the posture question is the
  deliverable's interface claim, and a quiet downgrade would restate the method
  error that CAT-PROPERTY-LAWS had to have corrected by the Architect.
- **A law cannot be stated without a new function.** In particular, "the issue
  list is exactly the rejected fields' issues in field order" needs a filter or
  map over the field list that this package does not have. That is why the
  deliverable asks for the HEAD and the earlier-accepts implication instead. If
  even that needs a new term, stop and report; do not add the function.
- **The measurement implicates a client** rather than this traversal. Report
  it; that is a different node.

## Not this node

- `schema_help` and the `schema_*_chars` helpers. See the settled input above.
- `CAT-CONFIGURATION-DECODER-LAWS` and `CAT-ARGPARSE-LAWS`, both of which
  consume this traversal and are framed separately after it lands.
- Any edit to `Application/Configuration/Decoder.ken.md`,
  `Application/CommandLine/ArgParse.ken.md`, or `Data/Collections/NonEmpty.ken.md`.
- Duplicate-key, schema-wellformedness, or field-uniqueness policy. The
  traversal does not claim it.
- Machine-checked complexity bounds.
