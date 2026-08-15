---
id: CONF-BLOCKER-MARKER-RECONCILE
title: "Three landed `BLOCKED-ON-` markers say 'no blocker node exists' and two of them are now false, while seven other blocked rows name their blockers in prose that no grep can find -- reconcile the corpus against the tracker and adjudicate the one blocker still unowned"
status: active
owner: spec-enclave
size: S
gate: none
depends_on: [CONF-FMT8-LEVELTOK]
blocks: []
github: null
origin: "Direct consequence of CONF-FMT8-LEVELTOK (merged 2026-08-15 at 2ed8bbfd8). Its census correctly wrote `(no blocker node exists)` rather than inventing nodes; the Steward then filed LANG-BYTES-HEX-LIST-LITERAL and LANG-FIXITY-DECL-SURFACE per COORDINATION §2, which makes two of those three parentheticals stale in a landed artifact. Steward-filed and framed 2026-08-15, measuring every marker at the base below."
---

> # THE SEED WAS RIGHT WHEN IT LANDED AND IS WRONG NOW. THAT IS THE WHOLE NODE.
>
> `CONF-FMT8-LEVELTOK` was told not to mint nodes it had no authority to file,
> and it correctly wrote `(no blocker node exists)` instead. **That was true at
> `2ed8bbfd8` and I made two-thirds of it false within the hour** by filing the
> nodes the census called for.
>
> **This is not a defect in that work.** It is the expected second half of a
> two-step that `COORDINATION §2` splits across two seats: the census names the
> missing surface, the Steward files it, and the corpus then has to be told.
> **The failure mode is skipping the telling** — a landed seed that asserts no
> node exists, next to a tracker where it does, is worse than either state
> alone, because a reader who greps the seed stops there.

## Fixed inputs, measured at `6275bbc35`

**The three `(no blocker node exists)` markers**, all in
`conformance/surface/formatting/seed-canonical-format.md`:

| line | marker | still true? |
|---|---|---|
| `:203` | `BLOCKED-ON-HEX-BYTE-LIST-SURFACE` | **FALSE** — [[LANG-BYTES-HEX-LIST-LITERAL]] exists, `ready` |
| `:414` | `BLOCKED-ON-USER-FIXITY-SURFACE` | **FALSE** — [[LANG-FIXITY-DECL-SURFACE]] exists, `draft` |
| `:387` | `BLOCKED-ON-MEMBERSHIP-ASCII-ROLE` | **FALSE** — `D3` answered; [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] filed |
| `:52` | FMT1 aggregate, naming all three, `(no blocker nodes exist)` | **FALSE in all three** |

**The landed convention** is `BLOCKED-ON-<reason> ([[node]])`, used four times
in `conformance/behavioral/buffer-io/seed-buffer-io.md` (`:684`, `:728`, `:763`,
`:797`) and documented at `conformance/README.md:439`.

**The seven prose-only blockers** — every `RED-UNTIL-BUILT` occurrence outside
the formatting seed — **are OUT OF SCOPE and the reason recorded here was
wrong.** Retained rather than deleted, because the correction is the useful
part:

> **What this section originally claimed:** *"These name real, buildable,
> tracked work, so they are legitimately pending — not the never-producible
> class."* **False, verified at `e2c2e6e78`.** The producers are **landed** —
> `prelude.rs:495` carries `fn filter`; the CAT-3 and bytes-CP0 producers are
> present and ancestral. **Those rows assert pending against work that already
> exists**, which is the mirror defect, not the benign case.
>
> **The evidence I used was the rows' own prose**, which is the weakest
> available source and the one thing a stale row is guaranteed to get wrong.
> **Adjudicating a disposition requires reading the producer, not the row.**

⇒ **They move to [[CONF-STALE-RED-DISPOSITIONS]]**, with the four
`seed-buffer-io.md` matrices. **Do not touch them here** — see `D4`.

## Deliverables

**`D1` — correct the THREE now-false markers**, at `:203`, `:387`, and `:414`,
to the landed `BLOCKED-ON-<reason> ([[node]])` shape naming the filed node.
Update FMT1's aggregate at `:52` to name all three nodes.

> **All three constituents are now owned**, so **no `(no blocker node exists)`
> parenthetical should survive in this seed.** `:387` became the third when
> `D3` was answered — see the block below it.

**`D2` — re-adjudicate `all-literal-lexemes-are-verbatim` and the fixity row
for *disposition only*.** Their fixtures remain unproducible **today** — the
nodes are filed, not landed — so the marker stays and the row stays red. **Do
not flip either to producible.** State explicitly that a filed blocker is not a
landed one; that distinction is the entire value of the marker.

> **`D3` IS ANSWERED — by the ring, with the citation `AC-4` demanded.**
> `31-lexical.md:79` puts ASCII `in` in the notation table as the **membership**
> spelling, and `:105-112` requires a glyph and its ASCII transliteration to lex
> to the **identical** token, with ASCII accepted forever. Measured against
> that: `lexer.rs:997` maps `"in"` to `KwIn`, and there is no membership arm in
> either spelling. ⇒ **An unowned surface gap, not a keyword-role decision.**
> The endpoint-(b) reading is **refuted by citation**, not by preference.
> Filed as [[LANG-MEMBERSHIP-OPERATOR-SURFACE]].
>
> ⇒ **`D1` therefore corrects THREE markers plus FMT1's aggregate, and no
> `(no blocker node exists)` parenthetical survives in the formatting seed.**

**`D3` — adjudicate the membership blocker, and report rather than decide.**
The row wants membership written with an accepted ASCII alias. Measured: the
lexer maps ASCII `in` to `KwIn` only, maps source `∈` to `Member`, and **the
parser has no membership-expression arm at all.** So there are two distinct
absences — no ASCII alias, and no parse of the operator in either spelling.

Determine from `spec/30-surface/` **which of these the spec actually requires**:

- If the spec grounds a membership operator and an ASCII alias, **that is a
  surface gap needing a node**, and it routes to the Steward with the citation.
  `COORDINATION §2` — do not file it yourself, and do not write a node name into
  the seed that does not exist.
- If ASCII `in` is committed to the keyword role by ruling, **the row is blocked
  by a decision rather than by missing work**, and the marker should say so.
  That is the endpoint-(b) shape `CONF-FMT8-LEVELTOK` was filed to make visible,
  reached from the other direction.

**`D4` — REMOVED 2026-08-15 by Steward ruling `evt_bgat447r9s6w`. Do not touch
the seven rows.** Not to link them, not to correct them.

**Its premise was false and the ring caught it before any edit.** `D4` asserted
the seven prose-only blockers "name real, buildable, tracked work, so they are
legitimately pending". Verified at `e2c2e6e78`: **the producers are landed** —
`prelude.rs:495` carries `fn filter`, the CAT-3 and bytes-CP0 producers are
present and ancestral, and the base contains CAT-4's named Map operations and
proofs. **So those rows assert pending against work that already exists**, and
linking them as pending would contradict their own producers while correcting
them would contradict `AC-3`.

⇒ **They move to [[CONF-STALE-RED-DISPOSITIONS]]**, together with the four
`seed-buffer-io.md` matrices, which the conformance-validator showed are the
same population: their links resolve to `merged` [[RT-NATIVE-FNSPLIT]], whose
closure records no residual build work and whose contract requires those
matrices to flip. **Eleven sites asserting pending against landed producers.**

## Acceptance criteria

**`AC-1`.** No occurrence of `no blocker node exists` survives for a surface
that now has a node. **Control:** grep the phrase; every remaining hit is
justified in the handback by name.

**`AC-2` — NARROWED by the same ruling.** Every `BLOCKED-ON-` marker either
carries a `[[node]]` link or an explicit statement that none exists.
**Control:** grep `BLOCKED-ON-` corpus-wide and show one or the other for each
hit. **Syntactic resolution only.**

> **The original wording told you to verify the four `seed-buffer-io.md`
> markers "still resolve", on the stated basis that they are the landed
> precedent this convention comes from. They resolve — and the dispositions
> behind them are stale.** A frame cannot ground a convention on an example and
> then keep the instruction when the example fails. **Cite
> `conformance/README.md:439`, which states the convention directly, not those
> four rows.** Their staleness is a delivered finding and belongs to
> [[CONF-STALE-RED-DISPOSITIONS]].

**`AC-3`.** No row is deleted, no row id changes, and no row's
producible/unproducible disposition flips. **This node changes what the corpus
says about ownership, not what it says about production.**

**`AC-4`.** `D3` returns a citation, not an opinion. A verdict of "spec grounds
it" names the file and line; a verdict of "ruled endpoint" names the ruling.
**"Appears to be" satisfies neither.**

**`AC-5`.** `crates/` byte-identical to the candidate base. **Control:** blob
identity, not a report.

## Banned scope

- **Do not edit `crates/`.** Every gap here is measured, not repaired.
- **Do not create tracker nodes.** `COORDINATION §2`. Findings route to the
  Steward; `D3` is expected to produce at most one.
- **Do not sweep other seeds for new unproducible fixtures.** That is a
  different node than this one, and widening is a re-cut rather than an overrun.
- **Do not reopen the level-token endpoint or the FMT8 repair.** Both landed.

## Why this is worth a slot rather than a footnote

**A blocked row is only useful if a reader can get from it to the work that
unblocks it.** Right now the corpus has three shapes for that — a linked node, a
parenthetical saying none exists, and prose a grep cannot see — and two of the
parentheticals are false. **The marker's whole purpose is to distinguish
"waiting on work" from "waiting on nothing", and a stale marker collapses that
distinction back to the state `CONF-FMT8-LEVELTOK` was filed to escape.**
