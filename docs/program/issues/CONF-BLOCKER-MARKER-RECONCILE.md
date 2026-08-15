---
id: CONF-BLOCKER-MARKER-RECONCILE
title: "Three landed `BLOCKED-ON-` markers say 'no blocker node exists' and two of them are now false, while seven other blocked rows name their blockers in prose that no grep can find -- reconcile the corpus against the tracker and adjudicate the one blocker still unowned"
status: ready
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
| `:387` | `BLOCKED-ON-MEMBERSHIP-ASCII-ROLE` | **unadjudicated — this is `D3`** |
| `:52` | FMT1 aggregate, naming all three, `(no blocker nodes exist)` | **FALSE in two of three** |

**The landed convention** is `BLOCKED-ON-<reason> ([[node]])`, used four times
in `conformance/behavioral/buffer-io/seed-buffer-io.md` (`:684`, `:728`, `:763`,
`:797`) and documented at `conformance/README.md:439`.

**The seven prose-only blockers** — every `RED-UNTIL-BUILT` occurrence outside
the formatting seed. **These are a different case and mostly a correct one:**

| file | count | what they say |
|---|---|---|
| `stdlib/collections/seed-cat3-collection-laws.md` | 5 | `length`/`min`, `map`/`length`, `filter`/`mem` unlanded; the `view`/`lens` record |
| `stdlib/collections/seed-cat4-maps-sets-relations.md` | 1 | every CAT-4 op is net-new |
| `surface/bytes-io/seed-bytes-io.md` | 1 | CP0 on the exact base |

⇒ **These name real, buildable, tracked work, so they are legitimately
pending — not the never-producible class.** `filter` is
[[LANG-PRELUDE-COLLECTIONS]]'s. **Do not convert them into `BLOCKED-ON-` rows
as though they were the same defect.** What they lack is only that a grep for
the convention cannot find them.

## Deliverables

**`D1` — correct the two now-false markers**, at `:203` and `:414`, to the
landed `BLOCKED-ON-<reason> ([[node]])` shape naming the filed node. Update
FMT1's aggregate at `:52` so it distinguishes which of its three constituents
are owned and which is not.

**`D2` — re-adjudicate `all-literal-lexemes-are-verbatim` and the fixity row
for *disposition only*.** Their fixtures remain unproducible **today** — the
nodes are filed, not landed — so the marker stays and the row stays red. **Do
not flip either to producible.** State explicitly that a filed blocker is not a
landed one; that distinction is the entire value of the marker.

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

**`D4` — make the seven prose-only blockers greppable without reclassifying
them.** Minimum viable: each names its blocking node as a `[[link]]` where one
exists, so the convention's grep finds them. **If no node exists for one, say
so in the row** — the same honesty the FMT8 census used.

## Acceptance criteria

**`AC-1`.** No occurrence of `no blocker node exists` survives for a surface
that now has a node. **Control:** grep the phrase; every remaining hit is
justified in the handback by name.

**`AC-2`.** Every `BLOCKED-ON-` marker either carries a `[[node]]` link or an
explicit statement that none exists. **Control:** grep `BLOCKED-ON-` corpus-wide
and show one or the other for each hit, including the four pre-existing
`seed-buffer-io.md` ones — **verify those still resolve**, do not assume.

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
