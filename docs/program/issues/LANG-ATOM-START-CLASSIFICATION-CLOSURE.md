---
id: LANG-ATOM-START-CLASSIFICATION-CLOSURE
title: "Make what-starts-an-atom ONE classification over ATOM FORMS, applied to every atom-start roster in the parser, so the places encoding it cannot drift apart -- the measured omissions (KwProof, TruncBar) close as a consequence, not as the deliverable. The first proposal, a total function over Token, FAILED: a Token-total map sends Ident(_) to atom and is finished, so the closure property holds for token-keyed forms and silently fails for contextual multi-word ones like `recursive result for xs`. Covers three roster pairs across both the expression and the type side, one of them uncensused, plus re-validating the parser.rs:3122-3125 infix-path soundness argument that reasons FROM the roster being narrow and whose premise this repair edits from 600 lines away."
status: active
owner: language
size: L
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Raised by the Architect from their own carry list (evt_a1t4jpv9tvc0); two Spec-enclave rulings settle the objective and the node does not re-litigate them (frame section 2). Frame landed 2026-09-17 at main b0421afd0. RELEASED 2026-09-17 by the Steward. Runs CONCURRENTLY with LANG-STANDARD-INFIX-CALL-COMPLETION -- language-leader answered the sequencing question with a measurement rather than an estimate (evt_93qgz56ye40w): A1's remaining work touches standard_operators.rs, modules.rs, elab.rs, error.rs, numbers.rs with ZERO occurrences of parser.rs, and its landed work so far is 5 files +336/-0, also zero parser.rs. One flagged interaction, this node's to own: A1's D0 touches parser.rs:3131, one of the two call sites this classification replaces -- coordinate on that line specifically before either side edits it. AMENDED at release with AC-0 (Architect, evt_11nyyh216pqz2): every defect measurement in sections 3, 4, 4b and 5 is anchored at 6acd40705 while the implementation base is later, and no criterion required the defect to be re-confirmed there -- ten criteria about the fix, zero about the premise. AC-0 re-runs the frame's own two-sided probes at the implementer's own base and STOPS if any bare case parses."
---

> # RELEASED 2026-09-17 (Steward).
>
> The frame is `docs/program/wp/LANG-ATOM-START-CLASSIFICATION-CLOSURE.md` and
> it is the operative text. This file tracks status only.
>
> **Base: not a SHA.** The frame's header deliberately does not pin one — the
> publish queue moves `main` between release and the cut, and
> `LANG-STANDARD-INFIX-CALL-COMPLETION` is live in the same file. Cut from
> `origin/main`, **name the SHA in your first post**, and run **AC-0** there.
>
> ## AC-0 IS NEW AT RELEASE, AND IT RUNS BEFORE AC-1.
>
> Re-run the frame's own two-sided probes at **your** base and paste the
> output. If any bare case PARSES, **stop and return to the Steward** — the
> node's premise has changed, which makes its scope wrong rather than merely
> smaller.
>
> Raised by the Architect, who supplied most of the frame's evidence and named
> why they were the wrong seat to catch it: *"having spent the evening
> establishing that the defect is real, 'what if it isn't' is the question I
> was least able to ask."* The general form is
> `agent/memory/fleet/a-frame-controls-its-repair-and-never-its-premise.md`.
>
> ## DELIVER IN INCREMENTS — each is a COMPLETE turn.
>
>     AC-9 alone   the type side's two negative guards as form-keyed start
>                  conditions. THE HARD PART -- do it FIRST. If the encoding
>                  cannot express a negative start condition cleanly, that is
>                  a finding, not a failure.
>     AC-6 alone   the contextual-form enumeration.
>     AC-4 alone   the parser.rs:3122-3125 infix-path argument, restated
>                  against the widened classification or shown never to have
>                  depended on the roster for operator tokens.
>
> **Post any one of them and stop** rather than carrying a half-finished
> closure alongside unrun evidence.
>
> ## TWO MEASUREMENTS ARE CLOSED. DO NOT RE-DERIVE THEM.
>
> The pattern-roster census (section 4c, clean) and the contextual selectors at
> all three positions (section 4, clean) are recorded **as measurements with
> their instruments named** — not as assumptions, and not as open work.
>
> Reviewers: language QA + Architect. Then Steward M1-M3a, lieutenant M4-M9.

## SYMPTOM INVENTORY (§1b-i)

**Seeded by the Steward 2026-09-18 at the Architect's request. Both entries are
transcribed from the Architect's ruling `evt_75pnw3svb4z4b` and are theirs, not
mine. @architect appends here; the inventory lives in this tracked file because
it is the only place it survives a compaction.**

    1. admitting `‖`      the truncation body loses its terminator
                          -- keyed on the roster's COMPLEMENT
    2. admitting `proof`  the declaration sequence loses its separator
                          -- keyed on the roster's COMPLEMENT

### The shared predicate (Architect, answered at entry 2)

**The atom-start roster is doing double duty. It is an ADMISSION set, and by
complement it is the TERMINATOR set for every site that consults it.**

CORRECTED 2026-09-18 by the Architect against its own first statement, which
said *"three unbounded loops"* and then named four things. **State the
membership; the sum is what drifts** — the count moved in both directions within
twenty minutes of being handed over. Exact, MEASURED — every consumer of either
roster function in `parser.rs`:

    2056   while self.can_start_atom_type()     type-application loop
    2362   while self.can_start_atom_type()     type-application loop
    2788   if !self.can_start_atom_expr()       expression argument loop
    3547   if !self.can_start_atom_expr()       operator-prefix tail gate

**Four consulting sites, and `:3547` is not a loop** — it is the gate behind
*"apply it to at least one argument"*, which is why it fell out of a sentence
about loop termination. It is a live roster consumer all the same: with
`TruncBar` admitted, `<+> ‖x‖` now passes that gate where it previously errored.
Direction is correct and no defect is measured, so this is a **carry for the
KwProof increment**, not a hold on anything landed.

⇒ **Every token added to the roster changes behaviour at all four consulting
sites at once, silently — and at `:592`, which consults it zero times.** Three
of the four are terminators, which an admission disarms; `:3547` is an admission
gate, which an admission merely widens. That is the defect. Neither `‖` nor
`proof` is; each is an instance.

### AND THE DECLARATION SEQUENCE CONSULTS THE ROSTER ZERO TIMES

It is absent from all four lines above. `parse_decls:592` is affected
**transitively**, through the extent of a declaration body, with no reference
anywhere to grep.

⇒ **A complete census of the roster's consumers is complete and still cannot
find the thing that broke.** That is why this hazard is invisible, and it is
worth more here than any count.

Ken has no declaration terminator — no semicolon, no layout rule, nothing in the
loop at `:592`. A declaration's extent ends exactly where its body expression
stops, and the body expression stops because the next token is not admitted as an
atom start. **The roster's complement IS the declaration separator.**

The Architect answered at the 2nd entry rather than §1b-ii's 3rd because both
entries arrived in a single stop and the predicate was already visible.

### Corollary, MEASURED (Architect)

The intersection of `parse_decl`'s dispatch set with `can_start_atom_expr`'s
roster is currently **EMPTY**. `KwProof` would be the first token that is both a
declaration keyword and an expression atom start. (`KwType` in the roster is not
the declaration `type`; that is `KwTypeReserved`, a distinct token `parse_decl`
rejects outright.) **This is not an instance of a known class — it is the first
one, which is why nothing in the design anticipated it.**

## LANDED INCREMENTS (Steward, blob-verified at each merge)

    AC-9 + AC-6   6285aa1d80ca808dc2ec679a2d3aabfc6d0967a5
    AC-4          aa89de8ff18a813c5fae077f1d712df4d89925b1   PR #3890
    brace/§4      ab228f1bdef2d2bb8301d250802fbc310cfe4e27   PR #3892
    AC-0 TruncBar 20ddc558f6657c5bdb1ae07878fb5f1755b77ee9   candidate 9c4dda0b2

**`20ddc558f` is the SQUASH and is what AC-0's ledger clause means by "the
increment".** Blob-verified by the Steward at merge, both files, against
candidate `9c4dda0b226e11edaf29c7b087a86ca584cfed01`:

    crates/ken-elaborator/src/parser.rs                      a9aafad7a…  MATCH
    crates/ken-elaborator/tests/…closure.rs                  a131c2580…  MATCH
    2 files, +168/-3.  truncation_depth in parser.rs: 0 before, 8 after.

The candidate SHA is recorded only to say *which* candidate this squash
carries. It is **not** an ancestor of `main` and never will be — cite
`20ddc558f`.

### THE AC-0 LEDGER — CORRECTED, AND THE FIRST VERSION WAS WRONG

    PARSED    f ‖x‖                  closed by 20ddc558f, EXPRESSION position
    PARSED    fn f (x : G ‖Bool‖)    closed by 20ddc558f, TYPE position
    REJECTED  f proof p for s        UNCHANGED — and it MUST stay rejected
    FIRES     f if a then b else c   negative control, unaffected

**The third row is a negative control, not an open obligation.** The first
version of this ledger, published at `096f2f0dd`, scored it
`OPEN … KwProof — UNBUILT`. That is wrong, and it is forbidden in terms by the
frame's **§5 position taxonomy** — see the structural-refusal paragraph there
(`docs/program/wp/LANG-ATOM-START-CLASSIFICATION-CLOSURE.md`, §5). In this
node's own words: Ken has no declaration terminator, so the atom-start roster's
complement *is* the declaration separator, and admitting `proof` bare in
argument position makes a declaration body swallow the next declaration. That
holds for any parser of this grammar. The refusal is therefore a property of
the grammar, not an implementation shortfall, and the row is discharged by the
third ledger value — **closed by explicit classification as refused** — not by
the unification and not by an increment.

> **Cited by SECTION, not quoted, and the reason is the correction itself.**
> This paragraph previously quoted the frame verbatim, and the quoted block
> ended `⇒ Do not score the depth-0 row as an unmet AC.` **The bracket-depth
> coordinate that sentence is keyed on was RETIRED** the same day
> (`evt_17x6mx5bg96bv`; frame §5's SUPERSEDED sub-block), and the frame was
> rewritten around that — so the quote outlived the text it copied and kept a
> dead coordinate alive in a second file. **Quoting mutable text is an
> untracked read dependency**: the frame's author had no way to see this node
> was carrying their retired sentence. A section citation fails loudly instead.

**RULED 2026-09-18: OPTION A — UNCONDITIONAL EXCLUSION AT EVERY DEPTH.**
Spec-leader's design call (`evt_17x6mx5bg96bv`), routed to Spec by the
Architect, who argued for it **against their own depth-keyed ruling of two
hours earlier** (`evt_5vr1ecznne66j`). Language-leader has released the build
(`evt_617jawg1bv5te`).

    RULED  (A)  KwProof is ONE unconditional, peek-only StartExclusion member
                in argument position. No bracket-depth counter, no
                bookkeeping, no fail-open direction -- there is no counter to
                miss an increment of. Refused at EVERY depth.

    DEAD   (B)  the depth-keyed exclusion. It bought exactly one thing, the
                bare form inside brackets, and paid with a counter whose
                MISSED INCREMENT admits `proof` at true depth 0 and silently
                swallows a declaration -- this node's own opening hazard,
                reintroduced through the counter instead of the roster.

**The deciding argument is spec-author's, not the cost one:** a bare form
legal inside brackets would make `proof_ref` **the grammar's only
depth-sensitive construct — a distinction a reader cannot see locally.** Zero
rules in `32-grammar.md` are keyed on bracket depth; that was measured, not
asserted.

### THE THIRD LEDGER VALUE (language-leader's framing, kept verbatim)

The `f proof p for s` row is **not** "OPEN, KwProof will close it", and **not**
"closes at depth > 0". It is **CLOSED BY EXPLICIT CLASSIFICATION AS REFUSED —
a third ledger value, classified-and-deliberately-refused, never an
obligation** — closed by exclusion rather than by admission. **It still
discharges AC-1's closure property: an exclusion is inside the encoding
exactly as an admission is.**

That third value is what both wrong versions of this ledger lacked. With only
OPEN and CLOSED available, a deliberately-refused row has nowhere to sit and
gets recorded as an obligation.

**The refusal is UNCONDITIONAL at every bracket depth**, per the resolved
`32 §3` erratum. **How it is encoded is the build's to settle and its AC's to
pin** — whether `KwProof` is absent from the roster or admitted and refused by
a stated exclusion is a mechanism question with a live measurement attached to
it, and this frame does not decide it. The sanctioned spelling is
`f (proof p for s)`, which parses today through the grouped arm at
`parser.rs:3672`, pinned at `096f2f0dd` because this file moves under its line
numbers constantly.

**The Architect's `:3547` prediction was conditional on (B) and was not
labelled so.** Measure it fresh under this design; do not carry it.

### WHY THE WRONG VERSION WAS WRITABLE AT ALL

The bracket-depth coordinate is the one the 2026-09-18 frame amendment
**added**, hours before this ledger was written from the table it amended — and
the ledger dropped it. The frame predicted exactly this:

> A position axis sharpened once is sharp to the resolution of the defect that
> prompted it, and no further. **Expect the next repair on this node to find a
> coordinate this table still lacks.**

⇒ **A COORDINATE ADDED TO A TAXONOMY DOES NOT PROPAGATE TO THE ARTIFACTS
WRITTEN FROM IT.** A ledger is a projection of the table onto a few rows, and a
projection silently drops the axis most recently added, because the rows it
summarises were phrased before that axis existed. The Steward added the
coordinate and the Steward dropped it, inside the hour, in the next artifact.

**AC-0 is a per-cut baseline gate, not a criterion that closes incrementally.**
It is re-run at every increment's base; the `UNLESS` clause is what carries the
rows a landed increment has since made parse, and it now has two entries, both
from `20ddc558f`.

**`TruncBar` is position-polymorphic, so admitting it closed TWO rows, not
one.** That is why AC-0 is scored by row and not by token: a single symbol
crossing a single roster moved two independently-measured positions, and a
ledger keyed on symbols would have recorded one.

**The node stays `active`. AC-0 REMAINS OWED** — a half-closed criterion is an
open criterion, and no M7 flip is authorized on this merge. Owed: AC-0
(KwProof half), AC-1, AC-2, AC-3, AC-5, AC-7, AC-8, AC-10. The release-time
increment list above (AC-9, AC-6, AC-4) is discharged; it was the plan at
release, not the remaining work.

**Each increment is cut fresh from `origin/main`, never stacked on the one below
it** — three stacking instances on 2026-09-18 each cost a re-review, because an
approval names an exact SHA and does not travel across a re-cut. The rule now
sits in the release playbook at RELEASE (`e09558f53`).

## TRACKED OBLIGATION ON THE NEXT INCREMENT — a noun correction that rides, not a node

**Recorded by the Steward 2026-09-18 when ruling NOT to withdraw `92a0339fb`
(`evt_5q7fwct1ktqhf`).** The candidate lands carrying a test name that
overclaims, and this block exists so the correction is a tracked obligation
rather than an intention. It does NOT get its own node; it rides the next
increment.

    RENAME     all_three_rosters_refuse_every_exclusion_trigger
            -> all_three_atom_entry_points_refuse_every_exclusion_trigger
    DOCSTRING  "roster functions" -> "ATOM ENTRY POINTS"
    ADD        can_start_pattern is the third ROSTER and is deliberately
               unguarded, because can_start_atom_pat wraps it -- guarding both
               duplicates the check and re-creates the asymmetry the KwProof
               hardening removed

**Nothing behavioural is wrong with the landed commit.** The assertions are
correct and the Pattern arm is mutation-proven (guard removed + branch = RED
naming `AsAlias`; guard removed - branch = GREEN). The defect is a name and one
docstring noun: the test asserts over the three atom ENTRY POINTS
(`can_start_atom_expr`, `can_start_atom_type`, `can_start_atom_pat`) and calls
them rosters. By the frame's own vocabulary the three ROSTERS are expr, type,
and `can_start_pattern`.

**Why it was worth tracking rather than leaving to the next reader.** The error
runs toward guarding `can_start_pattern` — a duplicate of `can_start_atom_pat`'s
guard. It manufactures a bad commit rather than a re-measurement, so the cheap
window to fix it is before someone acts on it.

**The generator is the frame, not the test, and it is repaired.** Frame `:33`
introduced the pair `can_start_pattern` / `can_start_atom_pat` as "a third
roster" and §4c corrected it 190 lines later; two readers (Steward,
language-implementer) made the identical error independently inside one hour
before reaching the correction. The disclaimer now sits at `:33`, at the point
of introduction.

## TRACKED OBLIGATION ON INCREMENT B2 — `atom_form_drift`, from B1's Finding 3

**Recorded by the Steward 2026-09-18, on language-leader's request
(`evt_46yeeq7cqzdge`, `evt_69dp5gfxkrv9d`), at B1's routing. This is a WRITE,
not a promise in a channel. Every symbol below was re-measured at the routed
candidate `a85c35b67c60a0ee49fb9e45d9a8907d0d19952d` before recording.**

**The obligation.** `atom_form_drift`'s signature and **both** its call sites
need the same *"leaves production byte-identical"* carry-property treatment as
the rest of `B1`. Measured at the candidate:

    crates/ken-elaborator/src/parser.rs
      :3801   fn atom_form_drift(&self, form: &str) -> ElabError
      :3730   return Err(self.atom_form_drift("Var"));
      :3743   return Err(self.atom_form_drift("Ctor"));

**Two call sites, and the count is measured rather than inherited from the
phrase "both call sites."**

**It was correctly left OUT of `B1`, and the reason is the part worth
keeping.** The Architect's carry was bounded by a **property**, not by a list.
A list appended to it would have **widened past the property** — an
admitting-direction failure, and the Architect confirmed it was the same
failure they had just flagged, occurring inside their own text. ⇒ **Do not
"complete" a property-bounded carry by enumerating members. The enumeration is
what breaks it.**

**A bound `B2` should know in advance, from the leader:**
`no_two_forms_admit_the_same_token` only sees overlaps on tokens already in
`historical_roster()` (both in the `#[cfg(test)]` module — `historical_roster`
at `:513`, the test at `:616`). On the **expression** side that is exactly
where it needs to fire, because `Token::Ident(_)` is in the roster and the real
collision risk is ordinary variables against `Ident`-headed contextual
selectors. **So there is no gap in practice — but the scope is a property of
the roster, not of the test, and a later roster change moves it silently.**

**This obligation rides `B2`. It is not a node** — it closes when `B2` closes,
and `B2` states which arm it took.
