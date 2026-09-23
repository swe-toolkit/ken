---
id: CAT-PARSING-CURSOR-LAWS
title: "Proof-backfill for Capability/Parsing/Cursor.ken.md: construct the missing inhabitant of the package's own CursorLaws proposition for arg_cursor_ops -- the three components (CursorPeekHasRemaining, CursorAdvanceProgress, CursorEndValid) are written as Props with no term proving them for the shipped dictionary."
status: ready
owner: foundation
size: L
gate: none
tier: T1
depends_on: [CAT-PROOF-COMPLETENESS-SURVEY, CAT-NAT-ORDER-LAWS, CAT-COLLECTIONS-NTH-LAWS]
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (landed ec23d4ab6), under operator ruling 2026-09-13 / PRINCIPLES #16. Filed draft, NOT released: the Steward measured its dependency on CAT-NAT-ORDER-LAWS at origin/main b839fd63295e550c12524bd9f4804fdf28be7169 while selecting which of the seventeen to release first -- see the body. The survey lists the seventeen as a flat set and does not record this edge; it is filed here so the measurement is not re-derived."
---

# Proof-backfill: `Capability/Parsing/Cursor.ken.md`

The survey's recommendation, verbatim:

> `CAT-PARSING-CURSOR-LAWS`: prove the three `CursorLaws` components for the
> existing `arg_cursor_ops` and its exact location representation with no new
> trust.

The obligation is already written down as a proposition, which is what makes
this node unusually well-specified: `CursorLaws`
(`catalog/packages/Capability/Parsing/Cursor.ken.md:259-262`) is a conjunction
of `CursorPeekHasRemaining` (`:222`), `CursorAdvanceProgress` (`:235`), and
`CursorEndValid` (`:250`). No term inhabits it for `arg_cursor_ops` (`:204`).
The deliverable is the inhabitant, not the statement.

## WHY THIS WAS `draft` — the measured dependency, now DISCHARGED

**Released 2026-09-19 at `origin/main`
`a86ee0ca54268d7900d686ec95cb1d7cbf7c81b3`.** Both predecessors are on `main`:
`CAT-NAT-ORDER-LAWS` landed `saturates` and `suc_decreases`, and
`CAT-COLLECTIONS-NTH-LAWS` landed the `nth`/`length` pair. The section below is
the original measurement, retained because the survey does not record this edge
and it should not be re-derived. It is history, not an open condition.

Measured at `origin/main` `b839fd63295e550c12524bd9f4804fdf28be7169`.

`arg_cursor_remaining` (`:166`) reduces through `arg_remaining_from` (`:152`)
to `add (sub (arg_length arg) offset) (arg_lengths_sum rest)`. So
`CursorAdvanceProgress` — advancing a peekable cursor strictly reduces the
remaining count — needs two facts about `sub` that **do not exist**:

- **saturation** — `sub len offset` reduces to `Zero` once `offset` reaches
  `len`, which is what makes `arg_cursor_normalize` (`:175`) preserve the
  remaining count while crossing an argument boundary;
- **strict decrease** — `sub len (Suc offset)` is below `sub len offset` while
  `offset < len`.

`catalog/packages/Data/Numeric/Nat/Order.ken.md` carries exactly three proofs
(`:121-125`), all closing without induction, and carries self-subtraction as a
**failing** attempt in a ` ```ken reject ` fence (`:146`).

⇒ **`CAT-NAT-ORDER-LAWS` `D1` is the prerequisite.** Releasing this node first
would force its implementer either to widen scope into `Nat` or to land `Nat`
facts inside `Capability/Parsing`. Release this once `D1` lands.

## One design question to settle at framing time, not now

`cursor_nat_lt` (`:136`) is **package-local to `Cursor`** — it is not
`Order`'s `leq_nat`. So the bridge between the two Booleans belongs in this
package, not in `CAT-NAT-ORDER-LAWS`, and this node's frame must say which
form `Order` chose to carry a `True` hypothesis (`IsTrue` vs
`Equal Bool ... True`) so the bridge matches it. `CAT-NAT-ORDER-LAWS` `D1`
records that choice in its package prose.

# Frame

RELEASED 2026-09-19; both predecessors are on `main`. Sizing raised from
`M`/`T2` to
`L`/`T1`: the work is inductive proof construction over a fuel-recursive
normalizer that crosses argument boundaries, plus a predicate bridge — semantic
invention, not a mechanical port.

## Settled inputs

Measured. Do not re-derive.

**1. The `draft` reason above is discharged.** Both `sub` facts this node was
waiting on are on `main` in
`catalog/packages/Data/Numeric/Nat/Order.ken.md`. **Navigate by symbol; the
line numbers are current at `a86ee0ca5` and will drift again:**

- **saturation** — `pub proof saturates for sub` (`:137`):
  `(b : Nat) → IsTrue (leq_nat a b) → Equal Nat (sub a b) Zero`
- **strict decrease** — `pub proof suc_decreases for sub` (`:155`):
  `IsTrue (leq_nat (Suc b) a) → IsTrue (leq_nat (Suc (sub a (Suc b))) (sub a b))`

Both statements are quoted byte-exact from `main`; it is only the coordinates
that moved. An earlier revision of this frame cited `:91-93` and `:109-113`,
read off the pre-squash `D1` candidate. **Those lines now hold
`zero_left for max` and `right_leq for max` — real proofs, wrong ones.** A
stale coordinate that lands on a different real symbol does not announce
itself the way a miss does.

**2. The deferred design question is answered: the real gap is the predicate,
and the wrapper is free.** `Order` carries its Boolean hypotheses as `IsTrue`
(`Order.ken.md:61`, `:71`, `:93`, `:103`, `:139`, `:158` — six sites, so this
is the package's convention and not one proof's choice); Cursor's three laws
state their conclusions as
`Equal Bool (cursor_nat_lt ...) True` (`:222`, `:235`, `:250`). An earlier
revision of this frame called that a mismatch on two axes. **It is one.**
`IsTrue (b : Bool) : Prop = Equal Bool b True`
(`Core/Classes/LawfulClasses.ken.md:54`) — the two spellings are the same
proposition, so the wrapper closes by unfolding, like `bytes_nat_length` in
input 4. **What is left is the predicate: `cursor_nat_lt` is strict `<`,
`leq_nat` is `≤`, and they are genuinely different functions.**

The bridge between them is **in scope for this node**. It mentions
`cursor_nat_lt`, a `Cursor`-local symbol, so it is not a `Derived` fact and
must not become a third predecessor.

**3. The bridge needs no new package edge.** `Cursor.ken.md:33` already imports
`sub` from `Data.Numeric.Nat.Order`, and `Order.ken.md:39` re-exports `leq_nat`
and `IsTrue`. Widen the existing import list.

> **AMENDED 2026-09-23: IMPORT `IsTrue` ONLY UNDER A PER-ITEM ALIAS. NEVER
> AS BARE `IsTrue`.** The operative import is
> `import Data.Numeric.Nat.Order (IsTrue as NatOrderIsTrue, leq_nat, sub)`.
> The 13 `IsTrue` occurrences in the newly added proofs are spelled
> `NatOrderIsTrue`; the shipped propositions and dictionary are unchanged.
> Architect ruling `evt_71p40x65qww39`.
>
> Why the bare import is refused: base binds a separate `IsTrue`
> (`crates/ken-elaborator/src/decimal_char.rs`, Char's refinement), whose
> `GlobalId` differs from `Core.Classes.LawfulClasses.IsTrue`. `bind_import`
> correctly refuses two identities under one name, directly or through the
> `Order` re-export alike. `§3.2`'s per-name rename is the spec's remedy. The
> directional census (`AC-3c`) needs an explicit provider edge, which the
> aliased import is. This supersedes the 2026-09-19 "do not import `IsTrue`"
> text and the earlier rejection of an alias (`evt_56ngqsbcmtavc`), which
> assumed a same-identity collision.
>
> **Control, owed with the candidate:** an identity assertion, or an
> inspectable resolved term, showing `NatOrderIsTrue` resolves to the
> `LawfulClasses` `GlobalId` and not to base's. Also owed: the untouched
> census green, the targeted Cursor suite, and package check and fmt. No
> sentinel, loader or test-harness edit.

**4. The `Bytes` view closes by unfolding, not by a new fact.**
`bytes_nat_length bs` is *defined* as `length UInt8 (bytes_to_list bs)`
(`Data/Collections/Derived.ken.md:928`), and `arg_length` is `bytes_nat_length`
(`Cursor.ken.md:67`).

**5. `add` facts exist; the `nth` pair HAS LANDED.**
`Data/Numeric/Nat/Arithmetic.ken.md` carries `assoc`, `comm`, `zero_l`,
`zero_r`, `suc_l`, `suc_r`. `Derived.ken.md` carried **no lemma relating `nth`
to `length`**, catalog-wide — until `CAT-COLLECTIONS-NTH-LAWS`, which landed
the pair at `a86ee0ca5`:

- `pub proof some_below_length for nth` (`Derived.ken.md:177`)
- `pub proof at_or_beyond_is_none for nth` (`:191`):
  `IsTrue (leq_nat (length a xs) n) → Equal (Option a) (nth a n xs) (None a)`

It is a discharged `depends_on` of this node, not a wall this node is expected
to hit.

**6. The `Nat` side of the positivity step is already paid — there is no
second predecessor.** `offset < len ⇒ Zero < sub len offset` is not
`saturates`, which is the zero direction. It falls out of `suc_decreases`:
instantiate `b := offset`, `a := len`, case-split on `sub len offset`, and the
`Zero` branch makes the conclusion `IsTrue (leq_nat (Suc _) Zero)`, which is
absurd — so `sub len offset` is a `Suc`, which is what `cursor_nat_lt Zero _`
reduces on. Architect, `evt_1391bs7gcxara`.

**7. Scrutinee order is opposed, and it reads like a wall.** `nth` matches `xs`
first then `n` (`Derived.ken.md:91`); `arg_remaining_from` matches `index`
first then `args` (`Cursor.ken.md:152`). Neither reduces until both scrutinees
are in WHNF, so an induction that splits only on `index` leaves `nth` stuck.
That is a missing case split, not a missing lemma.

## Deliverable

One term inhabiting `CursorLaws ArgCursor UInt8 ArgLocation arg_cursor_ops`,
exported from `Cursor.ken.md`, with whatever package-local bridge and lemmas it
needs. The proposition already exists (`:259-262`); the deliverable is the
inhabitant.

## Stop condition

**If the proof needs a general `List` fact beyond the pair
`CAT-COLLECTIONS-NTH-LAWS` lands, stop and report. Do not widen scope into
`Data/Collections/Derived.ken.md`, and do not land a general `List` fact inside
`Capability/Parsing`.** That is the same rule that produced both predecessors,
and it is the direction that fails closed.

An earlier revision of this frame made the `nth` gap itself the stop condition.
That was the wrong instrument: the gap is reached **with certainty**, in the
base case of `CursorPeekHasRemaining` with a one-argument cursor, so the stop
would have been a scheduled turn ending in the predecessor being cut anyway.
Architect ruling `evt_1391bs7gcxara`; the derivation is recorded in
`CAT-COLLECTIONS-NTH-LAWS`.

Induction on `args`/`index` discharges the outer `nth` layer with no lemma and
is the expected route for that half.

## Acceptance criteria

**`AC-1` — the inhabitant exists and is not degenerate.** *Control, both halves
required:* the package elaborates with the new term present; **and** replacing
the `CursorAdvanceProgress` component of the inhabitant with `Refl` makes the
package go RED, restored byte-exact afterwards. A positive check alone passes
for a proposition that is accidentally trivial.

**`AC-2` — the shipped dictionary and the shipped propositions are unchanged.**
This node proves what the package already ships; it does not adjust the package
until it becomes provable. *Control:* these declarations are byte-identical to
their pre-candidate text — `cursor_nat_lt` (`:136`), `arg_lengths_sum` (`:146`),
`arg_remaining_from` (`:152`), `arg_cursor_remaining` (`:166`),
`arg_cursor_peek` (`:169`), `arg_cursor_normalize` (`:175`),
`arg_cursor_advance` (`:194`), `arg_cursor_locate` (`:201`), `arg_cursor_ops`
(`:204`), and the four `Prop`s at `:222`, `:235`, `:250`, `:259`. Extract and
compare them programmatically, not by eye.

**`AC-3` — no new trust, and the diff goes exactly one place.** *Control:* the
added lines contain no `Axiom`, postulate, primitive, `Omega` carrier, or
kernel/TCB surface; **and** the diff touches
`catalog/packages/Capability/Parsing/Cursor.ken.md` plus, under `crates/`,
**only test harnesses that mechanically reconstruct a consumer's view of a
catalog package.** Any path under `crates/**/src/**`, or any other non-test
path, is a hard stop and a report, not a scope extension.

> **AMENDED 2026-09-19 ON A HARD STOP THAT THIS `AC` PRODUCED CORRECTLY.**
> Foundation stopped at `be70f1f84` rather than edit a harness, which is the
> behaviour the clause was written to get. **The clause was wrong, not the
> stop.** `crates/ken-elaborator/tests/cat_tier_d_cursor_import.rs` holds
> `parsing_cursor_loader_visible_inventory_is_exact`, which asserts
> `published_module_surfaces(...)` equals a 17-name literal set. **A new `pub`
> export necessarily changes that set**, so the harness edit is mechanical and
> forced — refusing it would mean this package can never gain a public export
> without a separate node.
>
> **This `AC` shape has now been wrong three times, all three mine, and this
> time in the OPPOSITE direction from the first two.** On
> `CAT-COLLECTIONS-NTH-LAWS` it was twice too narrow — a path list that had
> only sampled its population — and I reshaped it into exactly the predicate
> above. **One commit later I wrote `crates/`: nothing at all into the
> successor node**, discarding the predicate I had just derived, on a node whose
> whole deliverable is a new `pub` export. Over-tight and under-tight are the
> same defect: an `AC` asserting a population I had not measured.

**`AC-3a` — the harness edit is mechanical, and it is bounded.** *Control, all
three required:* the only change to
`cat_tier_d_cursor_import.rs` is adding the new exported name to the literal
set and updating the "seventeen" prose to match; the assertion stays an
**equality**, never a subset or a `contains`; **and** deleting the added name
makes the harness go RED, restored byte-exact afterwards. **If closing this
needs any harness change beyond those, that is a fresh hard stop** — the
authorisation is for the name, not for the file.

**`AC-3b` — the IMPORT-side provider roster follows the imports this candidate
adds.** *Control:* `provider_modules(PARSING_CURSOR)` lists
`Core.Logic.Transport`; `assert_providers_consumed(PARSING_CURSOR, …)` lists
`Core.Logic.Transport.cong` / `.sym` / `.trans` and
**`Core.Classes.LawfulClasses.leq_nat`**; the doc comment's numeral matches the
roster length. **Exactly those three items and nothing else** — a fold that
grows loses the Architect's pre-clearance (`evt_4nh9w0qr67mw3`) and goes back
for re-review.

> **CORRECTED 2026-09-19 ON A HARD STOP THIS `AC` CAUSED.** `AC-3b` first said
> **`Data.Numeric.Nat.Order.leq_nat`**, and **no such global exists** —
> foundation-leader `evt_2hgcc1ar08dsy`, at WIP `79875f64eabecfcce5007f4382`
> `5a215e8b167f1c`, harness 5/6 with `env.globals[provider]` panicking on that
> key alone while all three Transport entries resolve and pass.
>
> **THE ROSTER IS A LOADER-IDENTITY LIST, NOT AN IMPORT-SPELLING LIST.**
> `assert_providers_consumed` indexes `env.globals[provider]` and then compares
> `GlobalId`s through `term_mentions`; its own assert message says *"must
> consume the canonical provider"*. `Order` **re-exports** `leq_nat` without
> minting an alias, so the name keeps its defining module's identity. This is
> the same identity-versus-spelling distinction that governs instance heads: a
> facade re-export does not mint a new identity.
>
> ⇒ **Cursor's source import stays through `Data.Numeric.Nat.Order`, and the
> roster names `Core.Classes.LawfulClasses`. Both are correct and they describe
> different things.** Do **not** "reconcile" them by changing the catalog
> source to import from `LawfulClasses` — that would edit the package for a
> harness's convenience.
>
> **THE PRECEDENT IS TEN LINES AWAY AND SETTLES IT WITHOUT A JUDGMENT CALL.**
> `FORMATTING_DOC`'s roster in this same function already carries
> `"Core.Classes.LawfulClasses.leq_nat"`, verbatim, and is green today. **The
> convention was established, and I wrote a literal that contradicted it while
> editing the function that contains it.**
>
> **I TRANSCRIBED FOUR LITERALS FROM A REVIEW AND RESOLVED NONE OF THEM.**
> Copying them byte-exact felt like care and stood in for the only check that
> mattered — does the key exist. **A verbatim check is a check on
> transcription.** Read the producer before writing a predicate over its value:
> here the producer is `env.globals`, and one look at the neighbouring roster
> would have shown the right spelling.
>
> **THIS CORRECTION IS NOT GROWTH, so the pre-clearance stands.** Same three
> items, same four identities, same intent; one literal repaired so the entry
> resolves at all. **The one thing that WOULD be growth:** if
> `Core.Classes.LawfulClasses.leq_nat` does not resolve with
> `provider_modules(PARSING_CURSOR)` as the fold leaves it, then adding
> `Core.Classes.LawfulClasses` to that list is a **fourth** item — **stop and
> report; do not add it.** The Steward re-requests the Architect in that case.

> **AC-3a WAS NOT WRONG — IT FIRED CORRECTLY — AND IT STILL BOUNDED A
> POPULATION I HAD NOT MEASURED. That is the fourth instance, and it is mine.**
> `AC-3a` says *"the only change to `cat_tier_d_cursor_import.rs` is…"*. I
> wrote a bound scoped to the **whole file** while reasoning about **one
> function** in it, so the import-side roster — a different function, a
> genuinely different surface — was forbidden by a clause that never
> considered it. The ring hard-stopped rather than edit it, which is `AC-3a`
> working. **The three prior instances were a path list that had sampled its
> population; this one is a FILE that had sampled its FUNCTIONS.** Same defect
> at a finer grain.
>
> **THE DELETION CONTROL IS DELIBERATELY ABSENT HERE, AND THE REASON IS THE
> FINDING.** `AC-3a` can demand that deleting the added name goes RED because
> the export-side inventory is an `assert_eq!` against the full expected set —
> exact equality, fail-closed. **The import side cannot do that.** It iterates
> the roster it is handed and asserts each entry has a consumer, so **nothing
> quantifies over the module's actual import set** and deleting an entry
> merely makes it check less. Demanding a RED here would be an `AC` whose
> evidence cannot be produced, which is the one I silently skip.
>
> ⇒ **`AC-3b` is a CONTENT check, not a gate, and it is written knowing the
> difference.** Its force comes from the four identities having measured
> consumers — `cong` 8, `sym` 9, `trans` 6, `leq_nat` 14 (Architect, same
> event) — so this is a real coverage gap being closed and not a dead import
> being papered over.
>
> **The asymmetry itself is NOT in scope here and must not be chased into this
> candidate.** It predates this node; this is simply the first candidate to
> move Cursor's import set, so it is the first for which the gap has a cost.
> Name it in the closeout. Whether it earns a node is the Steward's call at
> L3's successor.

**`AC-3c` — the consumer-view authorization is a PREDICATE, and it replaces the
enumeration in `AC-3a`/`AC-3b`.** *Control:* for every whole-catalog assertion
that reds causally on this candidate's declared surface, the candidate either
brings the assertion into agreement or changes itself, and the choice is made by
this test and recorded in the candidate:

> **An assertion is MIRRORING when its expected value records what the catalog
> declares. Bring it into agreement.** **An assertion is DIRECTIONAL when its
> expected value records debt to be retired rather than a fact to be mirrored.
> Change the candidate, never the assertion.** Where the assertion states which
> it is, its own doc comment is the authority.

**`AC-3a` and `AC-3b` each authorized a fold against a NAMED harness, and that
is the defect.** An enumeration cannot report being incomplete, so every stop on
this node has been a consumer-view surface the list did not happen to name —
export literals, then the import roster, then the ambient census. **Three stops,
one shape, and the shape is that I kept writing lists.** `AC-3c` is the only
form that closes it: it quantifies over *assertions that fire*, which is a set
the candidate cannot be wrong about because CI enumerates it.

**The fourth stop is what showed the predicate needs two arms rather than one.**
`catalog_ambient_passthrough_migration_census`
(`lang_mod_strict_resolution_d0.rs:369`) failed on `Capability.Parsing.Cursor`
gaining `IsTrue`. Read as a pin it looks exactly like the first three and the
repair looks like refreshing it. **It is directional** — its doc says *"every
remaining name still requires an explicit provider migration"*, so a growing
expected set records a regression rather than a fact. Measured: `IsTrue` is
`pub fn IsTrue` at `Core/Classes/LawfulClasses.ken.md:54`, `Data.Numeric.Nat`
`.Order.ken.md:37` imports it explicitly and uses it eighteen times while
staying **out** of the census, and `IsTrue` appears in **zero** census entries
catalog-wide. So the repair is an explicit import in `Cursor.ken.md`,
aliased per settled input 3, and the sentinel is not touched. Steward ruling `evt_5f4qhhwqk1hye`.

> **A ONE-ARMED PREDICATE WOULD HAVE BEEN WORSE THAN THE LISTS IT REPLACED.**
> "Bring every firing consumer-view assertion into agreement" is the natural
> generalization of the first three stops, it reads as the rigorous fix, and on
> this candidate it authorizes recording new migration debt as routine scope
> hygiene. **The lists at least hard-stopped.** A predicate that generalizes
> from a sampled population inherits the sample's bias and adds authority —
> which is the same defect one level up, and the reason the directional arm is
> written before any further consumer-view surface is met.

## Design note, not a criterion

Name the `cursor_nat_lt`/`leq_nat` bridge as its own top-level declaration
whose type mentions both, rather than inlining the conversion inside each law.
Inlined, the direction of the strict-versus-non-strict step lives nowhere a
reader can check it. Not an AC: the deliverable is the inhabitant, and the
shape is the implementer's.
