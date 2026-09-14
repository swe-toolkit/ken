---
id: RT-TRAP-IDENTITY-EMISSION-COORDINATE
title: "Make a trap's identity carry its emission coordinate, so eliminations of one family at distinct source sites are not equal as RuntimeTrap values and do not collapse to one PlannedTrapIdentity -- closing the attribution gap that left HS16-HS21 unable to say WHICH site fired, with a five-occurrence in-tree discrimination criterion that both the catalog-keying fix and a type-keyed fix fail"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Architect ruling evt_2qq9jr1c1e4p9 (2026-09-14), CORRECTED at evt_7eqc0hmbanyzm after runtime-implementer refuted the Architect's own proposed population; mechanism ruled (b) and AC-2 ratified at evt_5kzahxdv9w8d1; recorded durably in ABI-S6 entry 35 (routed f0a735f14). Split out of RT-CHECKED-IH-RESULT-OBLIGATION-REKEY because it is fork-independent and that node's widened D1 is not: see 'Why this is its own node'. Fixed inputs first measured by the Steward at 686ffa8ac, then re-measured on origin/main; re-measure again at D0."
---

> # RELEASED. Architect read and approved at `evt_5xtean9vczn36`.
>
> **The mechanism fork is closed.** The Architect ruled `(b)` at
> `evt_5kzahxdv9w8d1` and ratified `AC-2` as authored here. They then read this
> frame — not a description of it — and approved it for release.
>
> **What this frame changed after the ruling**, by grounding the handed-over
> witness against the tree instead of transcribing it:
>
> 1. `AC-1` is written on the **real five-occurrence fixture**, and splits into
>    `AC-1a` / `AC-1b` because that fixture refutes a **second** plausible fix —
>    keying on the instantiated type — which an authored two-site pair would
>    have passed. The Architect's verdict: a control they did not know to
>    specify, and it exists only because the real fixture was used.
> 2. Two of the three relayed counts for that fixture were wrong (**four**
>    instantiations, not three; occurrence 2 is a `proc`). Corrected in place,
>    with the grep that produces the second error named so it is not re-made.
>    The wrong instantiation count never reached the durable record — ABI-S6
>    entry 35 (blob `c62d2288`) carries no instantiation count — so no erratum
>    is owed there.
>
> **One correction came back and is applied.** The mint-site line numbers do NOT
> drift between `686ffa8ac` and `origin/main`: they are `:2919` / `:6043` at
> both. The Steward's claimed two-line shift was a number carried from this
> frame's own earlier prose rather than re-derived, inside a re-measurement
> undertaken to avoid exactly that. The original citation was right at both refs.

## What this is

Three structurally identical closed-default sites in `func61` (lines 367, 590,
1946) all emit trap code 43. "Trap 43 fired" therefore names a CODE, never a
SITE — and every attribution in HS16 through HS21 that concluded about WHERE
rested on that code. The ruled account of the failure was not refuted by this;
it was left UNATTRIBUTED, which is a state nothing can be ruled from.

This node makes the identity discriminate. It is the instrument the rest of the
arc needs before any conclusion about WHERE can be trusted, and the Architect
ruled it a DELIVERABLE rather than instrumentation that gets reverted.

## The obligation, in the corrected form

**A trap's IDENTITY must carry its EMISSION COORDINATE, so that two eliminations
of one family at two source sites are not equal as `RuntimeTrap` values.** The
catalog keying follows from that; it does not substitute for it.

**The superseded form, and why it is recorded rather than deleted.** The first
ruling named the population as "the trap catalog's entries, keyed by occurrence
rather than value." That fix is NECESSARY AND NOT SUFFICIENT. `intern_trap`
selects with `position(|candidate| candidate == trap)` — a predicate over
VALUES — so re-keying the catalog cannot separate two entries that are equal as
values, because the key ranges over the values. The repair addressed the
collapse at the catalog and left the indistinguishability at the value.

This is kept in the frame because the discarded fix is the thing the acceptance
criterion has to be able to FAIL. See `AC-1`.

## Fixed inputs

> **INVARIANT OVER THIS SECTION: every coordinate below carries the SHA it was
> measured at.** A line number is a claim about one tree; a bare one is worthless
> and, worse, a bare PAIR of them can hide a SET difference by looking like
> drift. This is a structural closure, not a style note — the same
> misreading has now been registered at six sites across this arc, and per-site
> correction stopped being the answer at about the fourth. If you add a
> coordinate here without a SHA, the next reader inherits the defect.

**Every coordinate below names its own SHA. There is no section default.** A
bare coordinate here is a defect, not an inheritance — see "Why there is no
default" at the end of this section. Re-measure every coordinate at `D0`;
`RuntimeTrap` itself lives in `ken-host` and is equally present on `origin/main`.

Where a line says **SAME AT BOTH**, the file is blob-identical at `686ffa8ac`
and at `origin/main` `e3fe32510`, so the coordinate holds at either ref; that
was checked with `git rev-parse <ref>:<path>`, not assumed from the line text.

- **The equality that collapses them**, `crates/ken-host/src/effect_v1.rs:4067`
  at **`686ffa8ac`** — this declaration DRIFTS; the `origin/main` coordinate and
  the size of the gap are in the dedicated bullet below:

      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct RuntimeTrap {
          pub code: RuntimeTrapCode,
          pub message: String,
      }

  Equality is over exactly these two fields. There is **no `serde` derive and no
  `repr`** on `RuntimeTrap` or `RuntimeTrapCode`.

- **The two colliding mint sites, and there are exactly two**:
  `crates/ken-elaborator/src/erasure.rs:2919` and `:6043`. **Measured at BOTH
  `origin/main` `e3fe32510` and `686ffa8ac` and identical at both** (the file is
  blob-identical across the two refs) — carry the SHA beside
  the number, but there is no drift between these refs to correct for. `D0` still
  re-measures at the SHA it builds on. Both build

      code: RuntimeTrapCode::PatternMatchFailure,
      message: format!("no runtime match case selected for {}", view.family_symbol),

  so two eliminations of one family agree on **both** fields. Verified field by
  field, not inferred from the message alone.

- **The other seven `erasure.rs` mints do NOT collide**, and the reason matters:
  `:3215`, `:3995`, `:4024`, `:4044`, `:4064`, `:7526`, `:7942` each carry a
  distinct fixed or differently-interpolated message — **SAME AT BOTH**
  (`erasure.rs` is blob-identical at `686ffa8ac` and `e3fe32510`). **They
  discriminate incidentally, by message content — not by any mechanism.** Nothing
  stops the next family-symbol-derived default from colliding again.

- **WHAT `family_symbol` ACTUALLY INDIVIDUATES, and it is the root of the
  collapse.** It is a `StableSymbol` in the **`Declaration` namespace**, built
  from the declaring package plus the family's dotted name
  (`crates/ken-elaborator/src/compiler_driver.rs:4159-4165` at **`origin/main`
  `e3fe32510`**; at `686ffa8ac` the same `fn declaration_symbol` begins at
  **`:4163`**). **It carries no type arguments and no source coordinate.** Every
  elimination of one family declaration therefore produces a byte-identical
  message, whatever its instantiation and wherever it sits.

  **This coordinate is why the section default had to go.** It was measured on
  `origin/main` while the default asserted `686ffa8ac`, where `:4159` is an
  unrelated `.insert(stable, LowerabilityStatus::Supported);` inside another
  function. The default did not merely weaken the invariant — it was already
  telling a reader the wrong ref for a live coordinate.

- **THE FIVE-OCCURRENCE FIXTURE, ALREADY IN THE TREE AND ALREADY COLLAPSING.**
  `crates/ken-verify/tests/px8f_write_partition.rs`, the `WRITE_ALL_PARTITION`
  program constant beginning at line 15. **Blob-identical at `origin/main`
  `e3fe32510`, at `686ffa8ac`, and at the decided base `187895991`** — all three
  are blob `7e83322db9693502`, so the line numbers in the table below hold at any
  of them and the fixture cannot drift under this node.
  Five `match` sites eliminate the `Result` family, in five distinct enclosing
  declarations:

  | # | line | enclosing declaration | scrutinee instantiation |
  |---|---|---|---|
  | 1 | 18 | `fn body_from_write` (16) | `Result ResourceError Unit` |
  | 2 | 41 | `proc after_read` (37) | `Result ResourceError ReadProgress` |
  | 3 | 71 | `fn buffer_bracket_body` (68) | `Result ResourceError (ResourceBracketResult Unit Unit)` |
  | 4 | 102 | `fn file_bracket_body` (99) | `Result FileError (ResourceBracketResult Unit Unit)` |
  | 5 | 132 | `fn finish` (130) | `Result FileError (ResourceBracketResult Unit Unit)` |

  **Four distinct instantiations across five sites — occurrences 4 and 5 share
  one.** All five share the single `family_symbol`
  `decl:px8f_write_partition::Result`, so all five mint **one** `RuntimeTrap`
  value and `intern_trap` collapses them to **one** `PlannedTrapIdentity`.

  **Two counts here are corrections to the numbers relayed when this witness was
  handed over** ("five matches, five declarations, three instantiations"): the
  instantiation count is **four**, not three, and occurrence 2's enclosing
  declaration is a **`proc`**, not a `fn`. The second correction is the one that
  matters to anyone re-running this: a `^fn ` grep silently attributes
  occurrence 2 to the preceding `fn read_eof_body`, which does not mention
  `Result` at all. Enumerate `fn` and `proc` together.

  The shared instantiation at 4/5 is not an inconvenience in the fixture — it is
  the half that makes `AC-1b` possible, and it is why this fixture is used
  instead of an authored pair.

- **The interning predicate**, in
  `cranelift_backend/planning/static_transition/joins_traps.rs:633` —
  **SAME AT BOTH**: `intern_trap` returns the index of the first
  `position(|candidate| candidate == trap)`, else pushes. N equal-valued sites
  collapse to ONE `PlannedTrapIdentity`.

- **The trap catalog does NOT reach the emitted artifact as a value.** It is an
  in-process `Vec<RuntimeTrap>` (`planning/static_transition.rs:587` at
  **`686ffa8ac`**, which is **`:545`** on `origin/main` `e3fe32510` — this file
  is not blob-identical across the two; `cranelift_backend/compiled.rs:25`,
  **SAME AT BOTH**; surfaced at `cranelift_backend/surface.rs:38`, **SAME AT
  BOTH**). The artifact carries an index/token resolved host-side through
  `root_trap_catalog_index` (`compiled.rs:56`, **SAME AT BOTH**). **This is the
  fact the FENCED assessment rests on** — see "Why this is FENCED".

> #### CORRECTED 2026-09-14 AT `d40d1213e`: THE SURFACE IS **31 ACROSS 10**, AND
> #### THE "INDEPENDENT CORROBORATION" BELOW WAS NOT INDEPENDENT.
>
> **`D0` re-derived per `AC-1` by `runtime-implementer` (`evt_520p8ngmdhdhs`),
> confirmed by the Steward against the object store at `origin/main`
> `d40d1213e`.** Three of the 34 are test-side:
>
>     erasure.rs:7526, :7942     inside `#[cfg(test)] mod px7l_tests`  (attribute :7137, mod :7138)
>     lowering/aggregates.rs:4569 inside `#[cfg(test)] mod tests`      (attribute :4457, mod :4458, closes :6456)
>
> ⇒ **`erasure.rs` is 7 production mints, not 9. `lowering/aggregates.rs` leaves
> the file list entirely. The total is 31 across 10.** The coordinates the frame
> names individually (`erasure.rs:2919`/`:6043`, `compiler_driver.rs:4159`,
> `effect_v1.rs:3989`, the 257 aggregate) all survive the move from `e3fe32510`
> unchanged; no drift correction is owed.
>
> **THE DEFECT CLASS IS THIS FRAME'S OWN, ONE LEVEL IN.** The block below
> corrected the AGGREGATE (257 → 34) precisely because *"it counted in-file
> `#[cfg(test)] mod tests` blocks as production"* — and then did not re-sweep the
> ENUMERATION underneath it. **The correction fixed the number and inherited the
> blind spot in the list.**
>
> **AND THE CORROBORATION BELOW IS SPURIOUS, WHICH IS THE PART WORTH KEEPING.**
> The block argues that the census is independently confirmed because *"this
> frame states `erasure.rs` has 2 colliding mints plus 7 incidental
> discriminators, and 2 + 7 = 9 matches the census's 9 by a different method."*
> **The two methods agreed because they shared one blind spot, not because they
> were independent.** Both counted the same two `px7l_tests` sites as production.
> The corrected identity is `2 + 5 = 7`. **An agreement between two instruments
> is evidence only if their failure modes differ** — which is exactly what the
> implementer's reconciliation did establish: their first instrument returned 37
> across 13 by counting PARENT-DECLARED test modules as production (a single-file
> scan structurally cannot see `#[cfg(test)]` on the `mod` line in the parent),
> and 37 minus those 6 and 34 minus these 3 both land on **31**. Two instruments
> with OPPOSITE blind spots reconciling is the warrant; two with the same one is
> not.
>
> **What survives unchanged:** the argument that the non-colliding `erasure.rs`
> mints discriminate *incidentally, by message content, not by any mechanism* —
> which is what refutes alternative (a). **Five incidental production mints make
> that point as well as seven** (`:3215`, `:3995`, `:4024`, `:4044`, `:4064`).
> The population is smaller than stated and was partly not production at all; the
> conclusion is untouched.
>
> **The table and prose below are left in their original form deliberately**, as
> the record of what was certified and how it was wrong. Read the corrected
> numbers from this block.

- **The construction surface is 34 PRODUCTION SITES ACROSS 11 FILES — not the
  ~76 this frame first estimated.** `D0` certified it (`evt_2kv61375ae400`) and
  the frame's own number was flagged as an approximate grep census; it was, and
  it was wrong in a specific way: **it counted in-file `#[cfg(test)] mod tests`
  blocks as production.** Of 257 `RuntimeTrap {` sites at `686ffa8ac`, 223 are
  test-side.

  | count | file |
  |---|---|
  | 9 | `ken-elaborator/src/erasure.rs` |
  | 6 | `ken-runtime/.../static_transition/joins_traps.rs` |
  | 6 | `ken-runtime/src/ir.rs` |
  | 3 | `ken-runtime/src/object_linker_packaging.rs` |
  | 2 | `ken-runtime/.../planning/static_transition.rs` |
  | 2 | `ken-runtime/src/platform_runtime_support.rs` |
  | 2 | `ken-runtime/src/runtime_ir_evaluator.rs` |
  | 1 | `ken-host/src/effect_v1.rs` |
  | 1 | `ken-runtime/.../lowering/aggregates.rs` |
  | 1 | `ken-runtime/.../lowering/core.rs` |
  | 1 | `ken-runtime/.../lowering/mod.rs` |

  **This roughly HALVES the migration surface**, which matters for a size-M node.

  Two things about how that number was obtained are worth keeping. **The first
  instrument returned 30 and was not reported** — `#[cfg(test)]` in this tree is
  overwhelmingly ITEM-level rather than module-level, and a brace matcher
  treating each as opening a block swallowed following code. A plausible number
  from an instrument that could not discriminate. The replacement was
  hand-validated on five files and agrees on all five, and it is independently
  corroborated here: this frame states `erasure.rs` has 2 colliding mints plus 7
  incidental discriminators, and **2 + 7 = 9 matches the census's 9** by a
  different method.

  **The residual's class is THREE modules and six sites, not one** — the
  Architect ran the sweep (`evt_e8bzq3a95qz5`). Declaration-gated modules
  containing `RuntimeTrap {` sites:

      cranelift_backend/test_objects.rs                    2 sites, gated at cranelift_backend.rs:50
      cranelift_backend/artifact/api/tests.rs              2 sites, gated at artifact/api.rs:12
      cranelift_backend/lowering/core/primitive/tests.rs   2 sites, gated at primitive.rs:8

  Gate coordinates name the **`#[cfg(test)]` attribute line**, not the `mod`
  line below it. Two of them previously named the `mod` line (`api.rs:13`,
  `primitive.rs:9`) while `cranelift_backend.rs:50` named the attribute, so one
  phrase — "gated at" — pointed at two different kinds of line. Corrected to the
  attribute at every row. All three hold at **`686ffa8ac`** and at **`origin/main`
  `e3fe32510`**: `api.rs` and `primitive.rs` are blob-identical across the two,
  and `cranelift_backend.rs` differs elsewhere but carries `#[cfg(test)]` /
  `mod test_objects;` / `#[cfg(test)]` / `mod test_support;` at `:50`-`:53` at
  both refs.

  **The 34-file list contains none of them, so 34 is very likely correct — but
  for a reason not yet established, and `D0` owes that.** The classifier excluded
  two of these; nobody knows whether it did so because it recognises declaration
  gates (**it cannot**) or because they happen to be named `tests.rs`.

  **`cranelift_backend/test_support.rs` is the proof the distinction is live:**
  declaration-gated at `cranelift_backend.rs:52` (attribute line, both refs),
  **not** named `tests.rs`, and carrying zero
  `RuntimeTrap` sites today. It is exactly the shape a filename heuristic misses,
  sitting empty. `test_objects.rs` was found because its name invited it, and
  both other members have inviting names too. **Confirm WHY those two were
  excluded. If it was the filename, the instrument's blind spot is unbounded
  rather than bounded at six.**

- **THE `RuntimeTrap` DECLARATION LINE DRIFTS BETWEEN REFS**, and the header's
  correction does not cover it — that one is about the MINT sites, which genuinely
  do not drift. The struct does:

      effect_v1.rs   derive at :4067  (686ffa8ac)
                     derive at :3989  (origin/main e3fe32510)  -- 78 lines apart

  Anyone building on `origin/main` and going to `:4067` lands 78 lines off.
  `origin/main` is pinned to `e3fe32510` here because a moving ref is not a
  coordinate: the same sentence silently means a different tree tomorrow.

- **The fact that was already in the tree, three hard stops early**,
  `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs:4999`
  at **`686ffa8ac`** (same line at `187895991`; this file is **not**
  blob-identical on `origin/main` `e3fe32510`, where `:4999` is unrelated text —
  locate it there by the string `Hard-stop #18 row 2`), in a comment above a test
  named "Hard-stop #18 row 2":

      erasure.rs derives the case headers from the eliminated family and builds
      the default as format!("no runtime match case selected for
      {family_symbol}"). Two eliminations of one family in one declaration
      therefore agree on every field a header fingerprint can see.

  It was written for a NEIGHBOURING consumer (header fingerprints) and so never
  reached the consumer that needed it (trap identity). Recorded here because it
  is the reason this node exists at all, not as decoration.

### Why there is no default

This section used to open with *"MEASURED BY THE STEWARD at `686ffa8ac` (the
HS-arc WIP tip), except where a line says otherwise."* That line predates the
invariant above it and is the exact negation of it: the invariant works by making
a bare coordinate a **visible violation**, and a section default makes a bare
coordinate **silently valid** by lending it an attribution instead of letting it
lack one. The closure could not fire on the case it exists for. The Architect
found it (`evt_6qwfs0j22sr01`).

**The default was not merely inert — it was already wrong about a live
coordinate.** `compiler_driver.rs:4159-4165` was measured on `origin/main`
`e3fe32510`; under the default it read as `686ffa8ac`, where `:4159` sits inside
an unrelated function. A reader following the stated ref would have found
nothing, with no way to tell whether the frame or the tree was at fault.

`686ffa8ac` was also the wrong thing to privilege: it is the **HS-arc WIP probe
tip**, the ref that is warm because everyone is quoting it, which is never the
same object as the base a deliverable gets built on. Making the warm ref the
silent default is the warm-ref failure written into the document's structure.

⇒ **The general form, worth more than this instance: adding an invariant can
falsify a nearby line that was true before it, and that line will not look
stale.** A stale-value sweep greps for an old value; there is no old value here
to grep for. The defeating text contained no coordinate and no SHA — only the
word "except." When you add a rule, ask what nearby text was relying on the rule
not existing.

## Deliverables

**`D0` — the ruled migration. THE MECHANISM IS NO LONGER A FORK.** The Architect
ruled mechanism **(b)** at `evt_5kzahxdv9w8d1`: **add a required
emission-coordinate field to `RuntimeTrap`**, touching the ~76 production sites
above plus tests. Every future mint must supply a coordinate or it is a compile
error.

**The refuted alternative, recorded because `AC-1` must be able to fail it:**
(a) folding the coordinate into the existing `message` at the colliding mint
sites, about 2 sites. It was refuted on three grounds, and the first is
measured in this frame rather than argued:

  - **It closes the named INSTANCES, not the CLASS.** The fixed inputs above
    record that the other seven `erasure.rs` mints discriminate **incidentally,
    by message content, with no mechanism**. An incidental discrimination is not
    a property — a third family-symbol-derived default added later collides
    again and nothing reds when it does.
  - **It puts a machine identity into user-facing diagnostic text**, making one
    string serve two consumers with different requirements. That is this arc's
    own predicate arriving inside the repair for it.
  - **The compile-error form is actually available here**, which is exactly what
    it is NOT for `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s `D1`. Where the
    structural form is reachable, take it. 76 sites is mechanical, the failures
    are compile errors, and cost does not outrank a class closure whose only net
    is an unsound pass.

**`D0` IS REPORTED AND THE GATE PASSES** (`evt_2kv61375ae400`). All three halves
of the FENCED premise measured, and the third positively rather than as an
absent grep:

  - **No `serde`** anywhere in `effect_v1.rs`, at both refs.
  - **No `repr`** on either type. The file's four `repr(..)` attributes sit on
    `HostOpV1`, `CapabilityTokenV1`, `ResourceTokenV1` and `FdInheritancePolicyV1`.
  - **Nothing serializes it or pins its layout** — zero hits for `RuntimeTrap` in
    any serialize/encode/to_bytes/section context, and zero for
    `size_of::<RuntimeTrap>` / `transmute` / `offset_of`. **Positively: what
    crosses into generated code is an `i64`** — `emit_current_trap` stores
    `identity.abi_word()` or a shifted root token into the trap slot. The struct
    never crosses.

⇒ **FENCED holds. No stop.** The come-back condition remains live for the rest
of the build: if anything later serializes a `RuntimeTrap` or pins its field
layout, that returns to the Steward before continuing.

**L2 CONTENTION: CLEAR, AND MEASURED WITH A POSITIVE CONTROL.** No branch ahead
of `origin/main` touches `erasure.rs` except runtime-lane ones. The control that
makes the zero meaningful: **L2 branches do exist and are active in
`ken-elaborator`** — `LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD`,
`LANG-SURFACE-RECORD-DECL-FORM-RECUT`, `LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES` —
and every one of them avoids `erasure.rs`. The negative is a measurement, not an
artifact of looking somewhere empty. **No sequencing is owed.**

**`D1` — make the identity discriminate**, by the mechanism `D0` rules. Two
distinct source occurrences eliminating the SAME family must produce
`RuntimeTrap` values that are **not equal**, and therefore must receive distinct
`PlannedTrapIdentity` values from `intern_trap`.

**Report the coordinate's source and that it is a static planner or elaborator
fact — not read off a runtime value, and not derived from the order in which
sites happen to be visited.** An ordering-derived coordinate is unstable under
unrelated edits and would make the identity a function of traversal rather than
of the source site.

> #### THE COORDINATE IS CONSTRAINED FROM BOTH SIDES, AND ONE SIDE IS EASY TO MISS
>
> Added 2026-09-14 after two seats read `AC-1` and got five and fourteen. **Both
> readings were competent; the frame did not say which population it meant.** It
> does now, and so does this:
>
>     AC-1   five SOURCE occurrences -> five DISTINCT identities     constrains from ABOVE
>     AC-1b  the shared instantiation at 4/5 must still split        constrains from ABOVE
>     AC-2   many mints of ONE occurrence -> ONE identity            constrains from BELOW
>
> **A candidate coordinate can be too FINE as easily as too coarse, and only
> `AC-2` catches the first.** `(owner, path)` — the lowering path threaded
> through `lower_body_term_with_plans`, extended per branch — individuates
> **lowered positions**, while every criterion here ranges over **source
> occurrences**. Whether it is correct therefore turns on one fact nobody has
> measured: **is lowering injective on source occurrences?** If a single source
> `match` lowers to several mints at distinct paths, `(owner, path)` gives them
> several identities and `AC-2`'s second half fails — the frame's own named
> failure, *"an implementation that simply makes every trap unique"*, in a form
> that does not look like uniqueness.
>
> ⇒ **"The coordinate separates all N mints" is not by itself evidence the
> coordinate is right.** It is evidence only once N is known to be the number of
> source occurrences. Where it exceeds that, separating all N is the FAILURE
> reading. Measure the mint-to-occurrence map before selecting a coordinate, not
> after.
>
> **THE ADMISSIBILITY CONDITION, which is the form the repair has to satisfy**
> (Architect, `evt_2dzwx7r2a7xxj`): a candidate coordinate `C` is admissible
> exactly when the map `mint -> C` **FACTORS THROUGH** `mint -> source
> occurrence` — constant on each occurrence's mint class, distinct across
> classes. `(owner, family)` is too coarse, at 8 classes for 32 mints.
> `(owner, path)` is too fine if lowering is non-injective on source occurrences.
> **Neither more distinct values nor fewer is the goal; the goal is exactly the
> right partition.**
>
> #### `checked_occurrence_path` IS CLOSED. THE NAME IS THE ONLY THING CHECKED
> #### ABOUT IT.
>
> Offered as the one candidate with the right shape — *"a path in the CHECKED
> CORE term is invariant under lowering duplication"* — by the Architect, then
> amplified by the Steward into *"the obvious first place to look"*. **It does
> not have the property its name advertises.** Verified at `d40d1213e`
> (`evt_2gpxqnprtwbd1`):
>
>     :1239   let path = vec![0];                        <- a CONSTANT
>     :1244   checked_occurrence_path: path.clone(),
>     :1240   declaration: root.to_string(),             <- the ROOT, same defect as `owner`
>
>     :1521   checked_occurrence_path: occurrence_path,  <- from the `occurrence_path` PARAM
>     :1005   fn consume_recursive_invocation(.., occurrence_path: &[u64], ..)
>
> and the caller supplies that parameter from the lowering `path` itself, via
> `native_plans.consume_computational_ih_call(&owner, .., path)` in
> `lower_body_term_with_plans`.
>
> ⇒ **It IS the lowering path, stored under a name that says "checked", or a
> literal `vec![0]`, carried beside a `declaration` field holding the root.**
> Same granularity, more reassuring name. **Three seats treated it as a different
> notion on the strength of the name alone** — the one who named it, the one who
> wrote it into this node, and the one who was asked to measure it and correctly
> declined to assume. Recorded closed rather than deleted, because the name is
> the mechanism of the error and deleting it hides that.
>
> #### NO SOURCE COORDINATE CAN BE READ. IT MUST BE CONSTRUCTED.
>
> **The checked core carries no source position anywhere** (Architect,
> `evt_2rs4pyj2dq0ab`; independently measured by `runtime-implementer`,
> `evt_667zt7508v1ny`, both at `d40d1213e`). `CheckedCoreMatchView`
> (`checked_core.rs:309-320`) in full:
>
>     family_symbol, level_args, parameters, motive,
>     indices, scrutinee, branches, computational_recursive_hypotheses
>
> No span, no line, no occurrence id, no origin. A scan of the whole file for
> span/line/origin returns only `all_support_origins` (a `StableSymbol ->
> StableSymbol` map) and `obligation_metadata.origin` (also a symbol).
>
> ⇒ **"Just read the span" is FORECLOSED**, and this is recorded for every
> future reader rather than left to be re-derived. A source-occurrence coordinate
> must be CONSTRUCTED from what the traversal knows, or spans must be added to
> checked core — which crosses the elaborator boundary and is a different
> program, not this node.
>
> **Why the lowering `path` is structurally too fine, from its own construction**
> (`erasure.rs:2607-2611`): descending into an inlined declaration body pushes
> the symbol, copies the path and extends it with the inlining step, then passes
> `root` through UNCHANGED. **The path is an ABSOLUTE descent recording the route
> taken, inlining prefix included** — so one source occurrence inside `D`,
> reached by inlining `D` from two call sites, gets two different paths. Total
> separation at 45/45 is what an absolute path is FOR; it is not accidentally too
> fine.
>
> #### THE `owner` COLLISION — RESOLVED BY MEASUREMENT AGAINST THE DERIVATION
>
> Two reports thirteen seconds apart, neither having read the other:
>
>     DERIVED    (Architect, evt_2rs4pyj2dq0ab, from :2607-2611)
>                `owner` = stack.last() becomes the INLINED DECLARATION, so the
>                five occurrences in five declarations give five distinct owners.
>                Stated explicitly as a check made "without measuring anything".
>
>     MEASURED   (runtime-implementer, evt_667zt7508v1ny, at d40d1213e)
>                13 plans-route Result mints   owner = ["px8f_write_partition","main"]
>                 1 match_view Result mint     owner = ["px8f_write_partition","global_605"]
>                The five eliminations are NOT written in `main` -- they sit in
>                body_from_write, after_read, buffer_bracket_body,
>                file_bracket_body, finish.  `owner` alone gives ONE, not five.
>
> **The candidate `(owner, path[offset_where_owner_was_pushed ..])` rides on the
> same premise as the refuted row.** If `owner` is the lowering root at these
> mints, the offset is recorded at the root's push, the suffix is the full path,
> and the candidate collapses back to `(owner, path)`. The elegant half of the
> mechanism and the contested half are one fact.
>
> **The discriminator, one question:** are the five source eliminations reached
> through an inlining push at all? If yes, `owner` should be the inlinee and the
> measurement needs re-examining (what was read, and when relative to the pop).
> If no, the push never fires for these declarations, `owner` is correctly
> `main`, and **nothing in scope distinguishes the five** — a more serious
> finding than "too fine". The `match_view` mint's `global_605` proves the stack
> does vary somewhere; whether these thirteen sit inside a push is unmeasured.
>
> **MEASURED, and the derivation is refuted** (`runtime-implementer`, four-column
> run at `d40d1213e`; retracted by its author at `evt_2gwyt3y0mjdg6`):
>
>     owner   root   off   suffix-len   full-len
>     main    main   0     32           32
>     main    main   0     24           24        ... all thirteen identical in
>     main    main   0     30           30            owner, root and off
>
> `off = 0` on every one: **no declaration-inlining push occurs anywhere on the
> route to these mints.** The owner stack holds one element throughout and never
> moves, so the suffix equals the full path and `(owner, path[offset..])`
> collapses to `(owner, path)` — the failure it was meant to fix.
>
> **THE STRUCTURAL CAUSE, so this is not read as a bug in `stack`.**
> `erasure.rs:4297` (`lower_transparent_declaration`) and `:4379`
> (`..._with_plans`):
>
>     let declarations = checked_host_declaration_closure(package, &selection, symbol)?;
>     let declaration  = declarations.get(symbol)?;
>     let mut stack    = vec![symbol.clone()];      // SEEDED ONCE, with the TARGET
>     let body = lower_top_level_body(&declaration.body, &declarations, .., &mut stack, symbol)?;
>
> **`stack` holds one element unless an inlining push fires**, and none fires on
> this route. `owner == root == main` is `stack` reporting exactly what it holds:
> the compilation target. **It is an owner-of-LOWERING stack, read by three seats
> as an owner-of-SOURCE stack** — the third instance of the name-versus-population
> predicate, and the only one whose cause is a single line.
>
> Of the eight `declarations.get` sites, the four beside the four `stack.push`
> sites — **`:2535`, `:3276`, `:4702`, `:4816`** — are the consult-then-push
> inlining route. None fired for these thirteen mints.
>
> #### THE CLOSURE AT FAMILY SCOPE. DO NOT PROPOSE A FOURTH `stack` KEY.
>
> Because `owner == root == main` throughout, **every function of `stack` is
> CONSTANT on this route.** The finding is therefore not "these three keys fail"
> but:
>
> > **NO `stack`-DERIVED COORDINATE CAN SATISFY `AC-1` ON THIS ROUTE.** Any of
> > them yields one class where the criterion requires five.
>
> Recorded at family scope deliberately. Three keys were proposed and measured
> dead one at a time — `(owner, path)`, `(owner, path[offset..])`,
> `checked_occurrence_path` — by three different seats within one hour. **The
> family dies in one line from one measured fact, and the line is what stops a
> fourth.** Enumerating instances where a predicate is available is the defect
> this node keeps committing on itself.
>
> **`stack`-derived is NOT the same as in-lowering.** A position in a
> `CheckedCoreDeclarationBodyView.body` is reached by traversal and is not a
> function of `stack`; this closure does not reach it. Stated because reading a
> closure one scope too wide is the same error in the opposite direction.
>
> #### THE TWO REPAIRS ARE **NOT** INDEPENDENT (correction, `evt_2gwyt3y0mjdg6`)
>
> Ruled independent, then measured otherwise. The four `stack.push` sites:
>
>     :2607  lower_body_term_with_plans      path IN SCOPE
>     :3315  lower_checked_host_computation  path IN SCOPE
>     :4738  lower_body_term_inner           NO path parameter
>     :4823  lower_body_term_inner           NO path parameter
>
> **Two of four pushes have no `path` to take an offset into**, so granularity
> depends on propagation at half of them. **Propagation is a PRECONDITION of any
> offset-recording mechanism, not a parallel track** — the reverse of the
> sequencing originally given. The two sites carrying the obstruction are exactly
> the two the ruling had not opened.
>
> Separately, the offset arithmetic is off by two: at `:2607` the extension
> happens after the push, so `path.len()` at push time is the offset of the
> **inlining marker**, not of the owner's body, which starts at `+2`. Constant
> per owner, so it would not have broken factoring — but a mechanism resting on
> the phrase *"where the owner's body began"* has to compute that and did not.

> #### `D1` IS NOT BUILDABLE AS FRAMED. THE SCOPE FORK IS RIPE AND IS QUEUED FOR
> #### THE OPERATOR.
>
> **Status, 2026-09-14 after the deciding read: the fork is no longer "pending
> one read."** The read was taken, `(c)` is refuted and `(a2)` collapsed into
> `(a)`, and what remains is a genuine operator call between an in-scope re-cut
> that knowingly fails an AC and an unpriced repair across a crate and a lane
> boundary. **The unpriced arm is stated as unpriced rather than guessed** — *"we
> located the question and did not price it"* is the honest line, and the operator
> can fund the kernel read as easily as anyone here can ask for it. Details below,
> in the order they were measured.
>
> **Mechanism (b) as ruled — give `RuntimeTrap` a required coordinate field —
> presumes a coordinate exists to put in it.** Four candidates, four closures,
> across three seats in one hour:
>
>     CheckedCoreMatchView fields   no span, no occurrence id, no origin
>     owner / any stack function    names the lowering ROOT; constant on this route
>     motive / parameters / type    cannot separate occurrences 4 and 5 (AC-1b)
>     checked_occurrence_path       IS the lowering path, or a literal vec![0]
>
> The precise statement, which is stronger than "no coordinate is admissible":
> **no coordinate assembled from what is in scope at the mint can be SHOWN
> admissible, because the reference map it must factor through —
> `mint -> source occurrence` — is not observable there.** Anything that passed
> would pass by accident.
>
> **The arms:**
>
>     (a)  add source-occurrence identity to CHECKED CORE, upstream of erasure,
>          and thread it to the mint
>          -> a different and larger program, upstream of this node and this lane
>
>     (b)  re-cut D1 to what IS achievable in-scope -- per-LOWERED-POSITION
>          identity: honest, satisfies AC-1, knowingly FAILS AC-2
>          -> requires AC-2 amended or the deliverable narrowed.  Both Steward calls.
>
>     (c)  CARRY the (declaration, in-body position) pair that checked core
>          ALREADY distinguishes, through erasure's inlining
>          -> the propagation defect one level up; same class as the native_plans
>             repair, not a checked-core change
>
> **(a) was offered as one of two arms on the premise that checked core carries
> no source identity. That premise is false as stated:** `checked_core.rs:278-292`
> gives `CheckedCoreBodyView.declarations: BTreeMap<StableSymbol,
> CheckedCoreDeclarationBodyView>`, each entry carrying its own `symbol` and its
> own `body`. The five eliminations sit at five positions inside five distinct
> declaration bodies. **What is missing is not the identity; it is that erasure
> does not carry it.** `CheckedCoreMatchView` having no origin is true and is a
> statement about one node type, not about the enclosing structure.
>
> **THE ONE READ THAT DECIDES THE FORK** — a dump at the transparent-declaration
> entry, before any lowering. Not a trace: no route to follow, because if no
> inlining push fired then whatever minted was reached by traversing `main`'s own
> body, and the question is simply whether the eliminations are in it.
>
>     1  the KEY SET -- body_from_write, after_read, buffer_bracket_body,
>        file_bracket_body, finish present as SEPARATE entries?
>     2  per declaration body, the COUNT of Result eliminations
>     3  the count of Result eliminations in `main`'s body specifically
>
>     five decls hold their own, main holds ZERO
>         -> the mints are not coming from main's body; an inlining route exists
>            that nobody has found.  Boundary intact.  ARM (c), a thread-a-value
>            repair, framed as ordinary lane work.  No operator item.
>     main holds THIRTEEN, the five hold copies too
>         -> the boundary was destroyed BEFORE erasure saw it.  The per-declaration
>            bodies exist but the lowered terms did not come from them, so there is
>            nothing at the mint to carry.  ARM (a), and the fork goes to the
>            operator with both arms priced.
>
> #### THE READ CAME BACK. THE ANSWER IS NEITHER PREDICTED OUTCOME, AND IT
> #### REFUTES ARM `(c)` — THE ARM THIS SECTION WAS WRITTEN TO FUND.
>
> **The key set has THREE entries, and the five source declarations are not
> among them.** Not "the five hold their own", not "the five hold copies" —
> **absent as keys entirely.** All fourteen `Result` eliminations are already in
> `main`'s body before erasure begins.
>
> ⇒ **`(c)` is dead, and it is dead by the structure it was proposed on.** `(c)`
> was to carry the `(declaration, in-body position)` pair "that checked core
> already distinguishes." Checked core distinguishes it only for declarations
> that are KEYS. The five are not keys, so there is no
> `CheckedCoreDeclarationBodyView` to hold a position within — the pair `(c)`
> would carry does not exist to be carried. **The counter-example that refuted
> `(a)`'s premise was `declarations` existing as a per-symbol map; the thing that
> refutes `(c)` is which symbols are in it.** Same structure, one measurement
> apart, and only the second was checked against the actual key set.
>
> **THE KEY SET'S PRODUCER, which is the cleanest fact in the arc.**
> `erasure.rs:340-368` walks a queue; `collect_checked_body_declaration_refs:371`
> matches exactly `DirectDeclarationCall` and `RecursiveDeclarationCall`.
>
>     the key set = the transitive closure of declarations reached by a
>                   SURVIVING CALL NODE
>
> **The closure inlines nothing and chooses nothing — it reports.** The five are
> absent for exactly one reason: no call to them survives. This is not a defect
> in the closure and there is nothing to repair at `:340-368`.
>
> **AND THE REFERENCES WERE ALREADY GONE UPSTREAM OF THE ELABORATOR.** Read at
> `d40d1213e`:
>
>     checked_core.rs:1453-1456      declarations is populated FROM
>                                    selection.reachable_declarations, nothing else
>     compiler_driver.rs:3477-3492   reachable_declarations is a GRAPH REACHABILITY
>                                    CLOSURE over Term::Const references
>                                    (collect_term_constants, :3494)
>
> ⇒ The key set is *"declarations still referenced by a surviving `Term::Const`"*,
> and the five lost their references **upstream of checked-core construction
> entirely**. There is no elaborator-side inlining step at which a provenance
> symbol could be recorded, because by the time the elaborator builds checked
> core the references are already gone.
>
> #### THE ARMS, RESTATED AFTER THE READ
>
>     (a)   establish source-occurrence identity UPSTREAM.  LOCATED, NOT PRICED:
>           the candidate region is the KERNEL-TERM PRODUCER, not the elaborator.
>           Crosses a crate and a lane boundary.
>
>     (b)   re-cut D1 to per-LOWERED-POSITION identity.  Satisfies AC-1, knowingly
>           FAILS AC-2, and loses the interning the catalog exists for.  IN SCOPE,
>           and the Steward's to decide if the operator declines to fund (a).
>
>     (c)   REFUTED by the key set.  The pair it would carry does not exist.
>
>     (a2)  "have the elaborator record which declaration a term was inlined from,
>           at the point it inlines it" -- NAMES A POINT THAT IS NOT THERE.
>           COLLAPSES INTO (a) rather than standing beside it.
>
>     (d)   preserve the CALL BOUNDARY for these five -- emit them as
>           DirectDeclarationCall so their bodies survive as keys, after which
>           (symbol, position) is derivable downstream with NO new field anywhere.
>           (d) is what would make (c) work rather than a replacement for it.
>           UNPRICED, one upstream read away, ADVOCATED BY NOBODY.
>
> **UNESTABLISHED, and the first decides `(a)` outright:**
>
>     - were the five emitted as Term::Const and substituted away, or NEVER
>       emitted as separate constants at all?
>     - does anything rewrite the STORED body, as distinct from unfolding for
>       conversion?
>
> **`unfold_const` is NOT the answer to the second, and the implementer declined
> to report it as one.** It lives at `ken-kernel/src/conv.rs:37` with five call
> sites, all in `conv.rs` — that is conversion checking. **Conversion unfolds to
> COMPARE terms; it does not thereby rewrite the term that gets stored.** Five
> call sites in `conv.rs` is a fact about conversion, not about how `main`'s body
> came to hold fourteen `Result` matches. Recorded here because it is the exact
> shape this node has lost four candidates to, caught in advance on a find that
> would have been very easy to report as a cause.
>
> #### A STATED LIMIT OF THE FIXTURE: DECLARATION-SYMBOL-ALONE PASSES HERE AND IS
> #### WRONG IN GENERAL
>
> An early wording of `(a2)` keyed on *"provenance symbol only, no spans."*
> **That gives one identity per declaration.**
>
>     AC-1b   occurrences 4 and 5 share the instantiation, and sit in
>             file_bracket_body vs finish -- DIFFERENT declarations
>     =>      declaration-alone separates them, so it passes AC-1b HERE
>     =>      and the case it cannot handle -- two eliminations of one family in
>             ONE declaration -- is not in this fixture at all
>
> That case is not exotic; it is the case this node exists to close, and
> `control.rs`'s own *"Hard-stop #18 row 2"* comment describes it. **A coordinate
> sufficient on this fixture and wrong in general is precisely what `AC-2` exists
> to catch**, and the fixture cannot witness it. **This is the third thing this
> fixture cannot witness**, alongside the interning half of `AC-2` and the
> same-instantiation pair it *can*. Do not key anything on a declaration symbol
> alone: that is now a known-dead shape, not a candidate.
>
> **`off = 0` was consistent with both predicted readings** — no push happens
> either because erasure does no inlining here or because it inlines by a
> non-pushing route. The read settles it in a third way: the elaborator never had
> the boundary to destroy.
>
> **Counts.** `D1` hard stops **2**; `§1a` fires at 3 and the Steward's tracker is
> the count of record. Symptom inventory: **two predicates, four instances** —
> (1) *a coordinate proposed and priced before the population it must individuate
> was established*; (2) *a name was trusted to identify a population that nothing
> measured against it*, instanced by `checked_occurrence_path`, by `owner`, and by
> `stack`. **Seven probes run, seven reverted, no code written, no placeholder
> inserted** — which is `D3` producing the outcome it exists to make reportable.
> **The deliverable of this node is a finding, and the finding is that `D1` is
> not buildable as framed.**

**`D2` — the discrimination control, and it is the load-bearing deliverable.**
A test over the five-occurrence `WRITE_ALL_PARTITION` fixture in which all five
`Result` eliminations receive **five distinct planned identities**, including
the same-instantiation pair at occurrences 4 and 5.

- **MEASURED** = five source sites eliminating one family declaration get five
  distinct `PlannedTrapIdentity` values.
- **CLAIMED** = a trap identity individuates the emission site.
- **GAP** = a test that asserts the catalog's KEYING, or that keys on the
  instantiated type, cannot fail the defect.

**Use the fixture that is already there.** Adding a fresh two-site program
alongside it would test the repair against an example authored after the fix was
known — the existing five-site program predates it and collapses today.

**`D3` — report what the migration required**: how many sites were touched, and
whether any site could not supply a meaningful coordinate. **A site that cannot
name its own emission coordinate is a FINDING
to report, not a hole to fill with a placeholder** — a default value at such a
site silently restores the collapse for exactly that site.

> #### THE `lower_match_view` FORK: THE MECHANISM IS RULED, THE `D3`
> #### CLASSIFICATION IS **OPEN** (Architect, `evt_54rcy9rhdb6k6` as corrected by
> #### `evt_2dzwx7r2a7xxj`, grounded at `d40d1213e`)
>
> Reported by `runtime-implementer` as a `D3` finding arriving before the
> migration: `lower_match_view` has no `path` parameter, so 32 of its mints
> collapse to 8 on `(owner, family)`. **A lowering `path` is available at the
> fork and is DISCARDED rather than absent**, in
> `crates/ken-elaborator/src/erasure.rs` (enclosing functions resolved by RANGE,
> not by nearest-preceding `fn`; the first reading by that method was wrong):
>
>     lower_body_term_with_plans      :2416   params + path, native_plans, parent_oriented_frame
>     lower_checked_host_computation  :2983   params + path, native_plans: OPTION
>     lower_checked_host_value        :2324   params + path      <- FOURTH path-carrier
>     lower_body_term_inner           :4663   params only -- NO path
>     lower_match_view                :5949   params only -- NO path; sole caller is inner at :4920
>
> The fourth path-carrier was absent from the ruling's structure table and found
> by the implementer's instrument. Three of four listed is the same census defect
> this node's own fixed inputs carried at `34 across 11`.
>
> At `:3344-3366` the two arms are **the same call on the same term with the same
> arguments**, differing only in whether position rides along; `path` is a valid
> live binding in both, and the `else` declines to pass it because the callee has
> no parameter to receive it. Same shape at `:3108-3130` and at `:2971`.
>
> **The root defect, one sentence:** `native_plans: Option<&mut
> NativeLoweringPlanCollector>` gates two independent concerns — plan
> COLLECTION, its purpose, and position PROPAGATION, a side effect of which
> overload got called. **Position is lost exactly when no plan collector is
> active, a condition with no relationship to whether the term has a structural
> position.**
>
> **That mechanism stands whatever the coordinate turns out to be, because it is
> a statement about PROPAGATION and not about granularity.** The refusal to
> insert a placeholder is correct on it alone: a placeholder here would encode
> *"no position exists"* at sites where a position demonstrably does.
>
> **`D3`'s APPLICABILITY AT THESE SITES IS OPEN, AND WAS BRIEFLY RULED CLOSED ON
> A PREMISE THIS NODE ITSELF REFUTES.** The ruling *"`D3` does not classify the
> fork sites"* was drawn from path AVAILABILITY — but *"can name a lowering
> path"* and *"can name its source occurrence"* are the same two populations the
> `D1` block above separates. **If `path` is finer than source occurrence, a site
> holding `path` still cannot name its coordinate**, and `D3` classifies it
> exactly as filed. Retracted by its author within the hour (`evt_2dzwx7r2a7xxj`)
> and recorded here rather than deleted, because the shape is this node's
> subject: **a mechanism proved, and a classification drawn one fact too early.**
>
>     D3 applies at the fork sites   IFF  `path` does NOT determine source occurrence
>     D3 does not apply there        IFF  `path` DOES determine source occurrence
>
> Both arms wait on the same origin measurement as the coordinate choice. Until
> it returns, the implementer's filing stands as filed.
>
> **Two corrections of scope that ride with the ruling.** The "27 call sites" is
> a true number on the wrong scope twice — nine are recursive self-calls inside
> `lower_body_term_inner` threading a parameter it already has, the remaining 18
> spread over 11 enclosing functions, and the live-path drops are a small named
> set rather than the whole 27. And the 32 `match_view` mints are **not one
> population**: `lower_body_term_inner` is entered from the dropped-live-path
> forks, from `lower_body_term:4502`, and from `effects_for_targets:8307` — only
> mints descending from the latter two are genuine `D3` findings, a set smaller
> than the reported 24.
>
> **Open, and owed before any repair is priced** — two fields on one re-run of
> the existing instrumentation. **ORIGIN IS THE PRIORITY FIELD; ancestry is the
> cheaper follow-up, not the other way round.**
>
>     ORIGIN    map the 14 Result mints to source occurrence            <- FIRST
>               each -> one of the five fixture sites (18/41/71/102/132), or none
>     ANCESTRY  partition the 32 match_view mints
>               A = live `path` dropped upstream
>               B = no `path` in any ancestor
>
> **Origin decides three things at once:** the coordinate's granularity, `D3`'s
> applicability at the fork sites, and whether the ancestry column matters at
> all — because if `path` does not determine source occurrence, the ancestry
> partition locates where a value is dropped that nothing would have used.
> Ancestry then sizes the residual gap; if `lower_body_term:4502` and
> `effects_for_targets:8307` do not mint `Result` traps, partition B may be empty.
>
> **ANCESTRY MEASURED** (`runtime-implementer`, at `d40d1213e`). Instrument: a
> **Drop-guard** depth counter entered at the four path-carrying functions —
> a Drop guard rather than a manual decrement precisely so the `?` early returns
> that fill this traversal cannot corrupt it. Depth > 0 at the mint means a live
> `path` is on the stack. Probe reverted, tree clean.
>
>     TOTAL match_view mints                      32
>       A  live path dropped upstream (depth>0)   31   recoverable by decoupling
>       B  no path in any ancestor  (depth==0)     1   the genuine no-position gap
>
>     A by family   BufferSpan 9, Nat 8, TransferCount 6, Bool 4,
>                   BufferHandle 2, WriteProgress 1, Result 1
>     B by family   Nat 1
>
>     the match_view Result mint:  depth 26  ->  partition A
>
> ⇒ **The genuine gap is ONE mint, family `Nat`, and no `Result` mint is in it.**
> Partition B is not empty, but it is one. The earlier "24 stay collapsed" is
> superseded by its own author's data and was wrong twice — as a mint count where
> the defect count is over source occurrences, and in the underlying number.
>
> **This closes recoverability, NOT admissibility.** Whether the recovered `path`
> is the right coordinate is the separate question above, and `D3`'s applicability
> at these sites rides on that, not on ancestry.

## Acceptance criteria

**`AC-1` — a DISCRIMINATION test over the five-occurrence in-tree fixture. The
five `Result` eliminations in `WRITE_ALL_PARTITION` receive FIVE distinct
planned identities.**

**`AC-1` RANGES OVER SOURCE OCCURRENCES. NOT OVER LOWERED MINTS, AND NOT OVER
`RuntimeTrap` VALUES.** The five are exactly the five rows of the fixture table
in the fixed inputs — `px8f_write_partition.rs` lines 18, 41, 71, 102 and 132,
blob `7e83322db9693502` — and that table is the criterion's definition of its
own population. `D1` is written on the same population in the same words: *"two
distinct SOURCE OCCURRENCES eliminating the SAME family."*

**A mint count is a correct count of a different thing.** One source occurrence
lowered more than once under specialization mints more than once and may
legitimately intern to a single identity, so a mint total above five is the
expected shape and not a contradiction. Stated because it was not stated:
`runtime-implementer` measured **fourteen** `Result` trap mints in this fixture
against `AC-1`'s five and correctly declined to author the control until the two
were reconciled — *"that is how a control ends up green over the wrong set"*,
which is this node's entire subject. The frame, not the reader, was at fault:
it named a number without naming what it counted.

This is not an authored pair. The fixture is already in the tree and already
collapses, and it is a stronger witness than a two-site construction because it
refutes **two** plausible fixes rather than one.

**`AC-1a` — the superseded catalog-keying fix must FAIL this criterion.** "The
catalog is keyed by occurrence" passes a test written over the KEY while all
five sites still share one identity, because `intern_trap` selects with
`position(|candidate| candidate == trap)` — a predicate over VALUES. An AC that
tests the keying rather than the discrimination cannot fail the defect it exists
to catch.

**`AC-1b` — keying by the INSTANTIATED TYPE must also FAIL this criterion, and
this is the half only the real fixture provides.** Occurrences 4 and 5 carry the
**identical** instantiation `Result FileError (ResourceBracketResult Unit Unit)`
in two different declarations. Any coordinate that is a function of the type
rather than of the source site leaves that pair collapsed. An authored two-site
pair with distinct types would have passed a type-keyed fix and hidden this.

Neither sub-criterion is to be dropped in a restatement: a criterion whose
refuted alternatives are deleted can no longer be shown to discriminate.

**`AC-2` — a positive control on the non-colliding population.** Two
eliminations of **different** families continue to receive different identities,
and two references to the **same** emission site continue to receive the **same**
identity. Without the second half, an implementation that simply makes every
trap unique passes `AC-1` while destroying the interning the catalog exists for.

**"Emission site" here means the SOURCE site**, on the same population as `AC-1`
and `D1` — so the second half says: **all mints descending from one source
occurrence intern to one identity, however many of them there are.** That is the
constraint from below, and it is the only criterion in this node that a
too-fine coordinate can fail. A coordinate keyed on lowered position rather than
source position fails it silently: every assertion about the five still passes,
and the interning the catalog exists for is gone.

**`AC-3` — `func61`'s three sites, named.** After the repair, the three
closed-default sites at `func61` lines 367, 590 and 1946 carry three distinct
identities. **Re-measure those line numbers at `D0`** — they were measured on an
arc WIP and a line number is a claim about one tree.

**Not an acceptance criterion, and deliberately so:** "Trap 43 no longer
appears." The trap still fires; what changes is that it is attributable. An AC
keyed on the trap disappearing would be discharged by a repair that removed the
trap, which is not this node.

## Why this is its own node and not an amendment to the REKEY node

**Because `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s widened `D1` is blocked on a
held fork and this is not.** Folding a fork-independent, releasable repair into
a node whose headline deliverable cannot currently be written would strand this
work behind a design ruling it does not depend on. The two touch disjoint code:
this node is `ken-host` + `erasure.rs` + `joins_traps.rs`; the REKEY node is
`lowering/core.rs`.

The constraint is an Architect ruling, cited: `evt_2qq9jr1c1e4p9` as corrected
by `evt_7eqc0hmbanyzm`. It is not a Steward preference for a tidier graph.

## Why this is FENCED

**Compiler-internal.** It grows no TCB, adds no capability reachable from Ken
source, relocates no emission ownership, and adds no planner authority.

**And it changes no ABI or schema — this is the load-bearing half, and it is
measured rather than assumed.** `RuntimeTrap` carries no `serde` derive and no
`repr`; the trap catalog is host-side state and the artifact carries only an
index resolved through `root_trap_catalog_index`. Adding a discriminating
constituent to the value therefore changes nothing that crosses the ABI or gets
serialized into an artifact. **If `D0` finds that wrong — if any path serializes
a `RuntimeTrap` or pins its field layout — the FENCED assessment does not hold
and this comes back to the Steward before the build continues.**

It is a cross-crate Rust type change (`ken-host` is consumed by `ken-runtime`,
`ken-elaborator`, `ken-interp`, and `ken-cli` tests), which is a real consumer
inventory and is why `D3` exists — but a cross-crate Rust surface is not an ABI.

## Sizing / tier

**Size M, tier T1.** Under the ruled mechanism (b) this is a **wide mechanical
migration with a narrow semantic core** — ~76 sites of bookkeeping around one
design question. The review turns on an argument rather than a byte count: that
the coordinate is a static planner or elaborator fact rather than a traversal
artifact, and that the control actually discriminates five sites rather than
asserting a property of the keying.

**The T1 estimate is for the semantic core and the control, not the 76 sites.**
If the ring wants to split the mechanical migration onto a cheaper seat once the
coordinate's source is settled at `D1`, that is a reasonable cut and it is the
Steward's to make — bring it back rather than absorbing it.

## Contention

Runtime ring. Touches `crates/ken-host`, `crates/ken-elaborator/src/erasure.rs`,
and `crates/ken-runtime/.../joins_traps.rs`, so the candidate is a CODE merge:
full CI, M8/M8a Adversary.

**Cross-lane contention is REAL here and must be checked at `D0`, unlike the
REKEY node.** `erasure.rs` is in `ken-elaborator`, which is L2 language's crate.
L2's current objective is the reserved infix glyph work. Confirm at `D0` that no
L2 candidate is in flight touching `erasure.rs`; if one is, the two need
sequencing and that is the Steward's call, not the ring's.

L3 foundation is on `catalog/` and does not contend.

## Not this node

- The held representation fork — widening the unit-keyed `(actual, demanded)`
  discharge to every emitted hand-off VERSUS making intra-function edges
  structurally unable to carry a mismatched word. **Held by the Architect.**
  Nothing in this node touches it, and nothing in this node may be used to
  argue it either way.
- `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s deliverables, which are retained and
  not reopened here.
- Removing, relocating, or suppressing the trap itself.
- Any change to what the runtime tag means.
