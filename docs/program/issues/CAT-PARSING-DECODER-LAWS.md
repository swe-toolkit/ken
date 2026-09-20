---
id: CAT-PARSING-DECODER-LAWS
title: "inhabit DecoderManyConsumesAllLaw for the existing decoder_many, and give the pure/fail/bind/seq/alt/recursive combinators their general semantic equations -- the next of the seventeen proof-backfill follow-ons, selected after CAT-PARSING-CURSOR-LAWS on a measured absence of dependency rather than a chain"
status: ready
owner: foundation
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (landed ec23d4ab6), under operator ruling 2026-09-13 / PRINCIPLES #16. Selected by the Steward as CAT-PARSING-CURSOR-LAWS entered closeout, and the dependency was MEASURED at origin/main aae38c0b9d018fbb73e344534561b0279934b579 rather than inferred: DecoderManyConsumesAllLaw is stated parametrically over ops : CursorOps, so it does NOT need a CursorLaws inhabitant and depends_on is empty. Filed draft, not released."
---

# Decoder laws

**`DecoderManyConsumesAllLaw` is a `Prop`-valued definition with no inhabitant,
and the combinators have no general semantic equations.** Survey row for
`Capability/Parsing/Decoder.ken.md`: *"no term proves it, and the combinators
have no general semantic equations. The proof suite is absent."* This is an
absent proof, not a named deferral.

## Settled inputs — measured. Do not re-derive.

**1. The law is PARAMETRIC over the cursor dictionary, so there is NO
dependency on `CAT-PARSING-CURSOR-LAWS`.** Measured at `origin/main`
`aae38c0b9d018fbb73e344534561b0279934b579`, `Decoder.ken.md:293`:

    fn DecoderManyConsumesAllLaw (c el loc a : Type)
          (ops : CursorOps c el loc) (step : Decoder c loc a) : Prop =
      And (DecoderProgress ...) (DecoderRejectsOnlyAtEnd ...)
      → DecoderConsumesAll ... (decoder_many c el loc a ops step)

`ops` is a **parameter**, and the strict-decrease fact the proof needs arrives
as the **hypothesis** `DecoderProgress`, not from any `CursorLaws` inhabitant.

> **I ASSUMED THE OPPOSITE AND THE MEASUREMENT REFUTED IT.** Decoder sits in
> the same package family as Cursor, imports it at `:19`, and shares its
> acceptance test file, so "Cursor's progress law must be what feeds
> many-consumes-all" is the natural reading. **It is wrong, and the reason it
> is worth writing down is that every circumstantial signal pointed at a chain
> that the type does not contain.** `depends_on` is empty because the law says
> so, not because nothing was checked.

> **RELEASED 2026-09-20, AND THE SCHEDULING HALF OF INPUT 2 HAS INVERTED.**
> `CAT-PARSING-CURSOR-LAWS` is **parked**, not landing: its candidate
> `b62d091ec` is byte-clean and correct but cannot merge until
> `LANG-IMPORT-IDENTITY-ESCAPE` lands in the language lane. **So the bridge
> lemmas below are KNOWN-ABSENT from `main`, not pending arrival** — measured,
> `cursor_nat_lt_from_leq_suc` / `cursor_nat_lt_to_leq_suc` /
> `cursor_nat_not_lt_to_reverse_leq` appear nowhere under `catalog/` on
> `origin/main`. Input 2's "report a missing bridge, never widen into
> `Cursor.ken.md`" is now the **expected** path rather than the contingency.
> Build the decoder side and report what you needed; do not wait and do not
> reach into Cursor.
>
> **The earlier contention hold is DISCHARGED.** This node was held because
> `b62d091ec` touches `crates/ken-elaborator/tests/cat_tier_d_cursor_import.rs`,
> the file `AC-4` mandates editing, and that candidate looked minutes from
> landing. It is not. **You reach that file first; Cursor rebases onto you.**
>
> **One thing measured in your favour:** `Decoder.ken.md` uses `IsTrue` **zero**
> times today, so you do not inherit Cursor's blocker as written. If your new
> proofs introduce it, you will hit the same wall — say so and stop rather than
> working around it.

**2. `CAT-PARSING-CURSOR-LAWS` is still the right predecessor, for SCHEDULING
and for LEMMAS, neither of which is a gate.** `Decoder.ken.md:19` imports
`cursor_nat_lt` among six names, and Cursor's node landed
`cursor_nat_lt_from_leq_suc` / `cursor_nat_lt_to_leq_suc` as **mutual
inverses**, plus `cursor_nat_not_lt_to_reverse_leq` for the negative side. The
Architect's approval (`evt_4nh9w0qr67mw3`) records that proving both directions
means *"a later consumer can cross in either direction without a new lemma."*
**This node is that later consumer.** If a bridge is missing, that is a report,
not a widening into `Cursor.ken.md`.

**3. The fuel is structural and already bounded by `remaining`.**
`decoder_many` (`:165`) delegates to `decoder_many_fuel` (`:117`) seeded with
`cursor_remaining ... cur` (`:173`). Callers supply no fuel. **Do not introduce
a fuel parameter, and do not prove fuel sufficiency from a length argument** —
see the stop condition.

## Deliverable

**One inhabitant of `DecoderManyConsumesAllLaw` for the existing
`decoder_many`, plus the general semantic equations for `decoder_pure`,
`decoder_fail`, `decoder_bind`, `decoder_seq`, `decoder_alt`, and
`decoder_recursive`**, over the existing decoder and derived-functor values.
**No new trust and no representation change.**

## Acceptance criteria

**`AC-1` — the law has a real inhabitant and it is not vacuous.** *Control:*
the term discharges both conjuncts of the hypothesis and concludes
`DecoderConsumesAll` for `decoder_many`; **and** it does real case analysis and
structural recursion rather than reducing to a one-line `Proved`. A law that
type-checks because the two sides are definitionally the same object has proved
nothing.

**`AC-2` — the equations range over the whole carrier, not the reachable
sub-population.** *Control:* for each combinator equation, either the premise
forces the normalization (state which premise, for which equation) or the
equation is checked on the off-normal inputs — cursor past the end, zero-width
remaining, an immediately-rejecting step. **This is the population audit the
Architect ran on Cursor and it is an AC here because a law-at-a-time read
cannot perform it.**

**`AC-3` — no new trust.** *Control:* the added lines contain no `Axiom`,
postulate, primitive, `Omega` carrier, or kernel/TCB surface; **and** no axiom
is *inherited* — follow each imported proof reference to a real proof term and
say which module declares it. Those are different statements and only the
second discharges this.

**`AC-4` — the harness edit is bounded, and BOTH its inventories move
together.** *Control:* if the diff adds a `pub` export, the export-side literal
set and the **import-side provider roster** are both updated in the same
candidate; roster entries are **canonical loader identities** (`env.globals`
keys), never import spellings. **A facade re-export mints no new identity** —
`Data.Numeric.Nat.Order` re-exports `leq_nat`, whose canonical id is
`Core.Classes.LawfulClasses.leq_nat`.

> **`AC-4` exists because this exact pair cost `CAT-PARSING-CURSOR-LAWS` two
> hard stops.** The export side is an `assert_eq!` against the full expected
> set and is fail-closed; the import side iterates the roster it is handed, so
> **an added import is structurally invisible to it** and only an author who
> knows to look will update it. **Read the sibling rosters in that function
> before adding an entry** — the convention that decides an entry's form is
> already written there.

## Stop condition

**A general fact about `CursorOps`, or any change under
`catalog/packages/Capability/Parsing/Cursor.ken.md`, is a report — never a
widening.** Likewise **any argument that proves fuel sufficiency from a length
or a `remaining` bound**: Cursor's progress law was deliberately built to
survive fuel exhaustion, and an argument that reinstates a sufficiency premise
here would be correct today and fragile against any change to the fuel source.

## Not this node

The import-side fail-closed asymmetry in `cat_tier_d_cursor_import.rs` — that
the provider-consumption loop quantifies over nothing. Named at
`CAT-PARSING-CURSOR-LAWS`'s closeout. `AC-4` above works *around* it; repairing
it is a separate decision and is not authorized here.
