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

**The node stays `active`.** Owed: AC-0, AC-1, AC-2, AC-3, AC-5, AC-7, AC-8,
AC-10. The release-time increment list above (AC-9, AC-6, AC-4) is discharged;
it was the plan at release, not the remaining work.

**Each increment is cut fresh from `origin/main`, never stacked on the one below
it** — three stacking instances on 2026-09-18 each cost a re-review, because an
approval names an exact SHA and does not travel across a re-cut. The rule now
sits in the release playbook at RELEASE (`e09558f53`).
