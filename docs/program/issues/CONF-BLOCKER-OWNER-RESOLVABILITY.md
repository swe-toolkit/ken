---
id: CONF-BLOCKER-OWNER-RESOLVABILITY
title: "72 of 77 conformance blocker markers name a condition with no resolvable owner, so nothing can ever re-examine them when the work lands -- the wikilinked five are the only ones that were findable at all"
status: ready
owner: spec-enclave
size: M
gate: none
depends_on: [CONF-STALE-RED-DISPOSITIONS]
blocks: []
github: null
origin: "Steward census at f7459ff9d while running the section 4e successor check on CONF-STALE-RED-DISPOSITIONS. That node's D5 established the mechanism -- nothing re-examines a BLOCKED-ON marker when the node it names flips to merged -- and the census establishes the population it applies to. Steward-filed per COORDINATION §2."
---

> # THE PREDECESSOR FIXED THE SITES IT COULD FIND. THIS NODE IS ABOUT WHY IT COULD ONLY FIND FIVE.
>
> [[CONF-STALE-RED-DISPOSITIONS]] corrected the `RT-NATIVE-FNSPLIT` population
> and established the mechanism in its `D5`: **nothing re-examines a
> `BLOCKED-ON-<x>` marker when the work it waits on lands, and a closure clause
> about someone else's artifact is discharged by nobody by default.**
>
> **The census says that mechanism has almost nothing to bite on.** Of 77
> marker occurrences, **5 name a tracker node resolvably. The other 72 name a
> condition** — `RED-UNTIL-BUILT`, `RED-UNTIL-PX8-R`, `RED-UNTIL-I-5`,
> `RED-UNTIL-K3` — with **no owner a script or a reader can follow.**
>
> ⇒ **The FNSPLIT sites were the GOOD shape.** They went stale *and were
> findable*. The other 72 can go stale without any instrument, including this
> one, ever noticing.

## Fixed inputs, measured at `f7459ff9d`

**Re-measure at your base and report the new numbers if they differ.** The tree
moves under this corpus — that is the standing subject of this lane.

| marker | occurrences | suffix names a tracker node? |
|---|---|---|
| `RED-UNTIL-BUILT` | 27 | no — a condition |
| `RED-UNTIL-PX8-R` | 18 | **no** — no `PX8-R` node exists |
| `RED-UNTIL-I-5` | 10 | **no** |
| `RED-UNTIL-K3` | 6 | **no** |
| `BLOCKED-ON-NATIVE-REACHABILITY` | 5 | no, but **all 5 carry `[[RT-NATIVE-FNSPLIT]]`** |
| `BLOCKED-ON-USER-FIXITY-SURFACE` | 2 | no — see the split below |
| `BLOCKED-ON-MEMBERSHIP-ASCII-ROLE` | 2 | no |
| `BLOCKED-ON-HEX-BYTE-LIST-SURFACE` | 2 | no |
| `RED-UNTIL-REMAINING-PR-C-ARMS` | 1 | no |
| `RED-UNTIL-PX8-V` | 1 | **no** |
| `RED-UNTIL-PX8-F-CAP-41` | 1 | **yes** — `PX8-F-CAP-41`, `draft` |
| `RED-UNTIL-EXTERNAL-WARD-MONITOR-CONSUMER` | 1 | no |
| `RED-UNTIL-ABI-REVOKE` | 1 | **yes** — `ABI-REVOKE`, `draft` |

**77 occurrences, 13 conditions, 2 suffixes that are node ids, 5 occurrences
carrying a `[[wikilink]]`.**

> ### THE SPLIT THAT PROVES A NAME-KEYED CENSUS IS NOT ENOUGH
>
> `BLOCKED-ON-USER-FIXITY-SURFACE` appears twice in
> `surface/formatting/seed-canonical-format.md`. **Line 58 carries
> `[[LANG-FIXITY-DECL-SURFACE]]`. Line 423 — the fixture line — carries the bare
> condition.** Same condition, same file, one resolvable and one not.
>
> ⇒ **A census keyed on `[[wikilink]]` finds the first and misses the second**,
> and reports a clean-looking number either way. **My first cut of this census
> did exactly that and returned 5.** The second cut, keyed on the marker
> vocabulary instead of the link syntax, returned 77. **Run both shapes and
> reconcile them** — a single-shape census here is evidence about a syntax, not
> about the corpus.

## Deliverables

**`D1` — complete the census in both shapes, and reconcile.** Every occurrence,
its file and line, its condition, and whether an owner is resolvable. **Report
the two shapes' counts separately before merging them**, so a future re-run can
tell which instrument drifted.

**`D2` — for each of the 13 conditions, name the owner if one exists.** A
tracker node, a WP id in `docs/program/03-program-of-work.md`, or **nothing**.
`PX8-R`, `PX8-V`, `I-5` and `K3` are the interesting ones: they look like ids
and resolve to no node. **Say what they are** — a retired id, a WP-catalog
entry, a phase name, or a coinage that never had a referent.

**`D3` — adjudicate ONLY the conditions whose owner is resolvable AND landed.**
That is the FNSPLIT class and it is the one this node can actually decide.
`PX8-F-CAP-41` and `ABI-REVOKE` are both `draft`, so their markers are
**correct** and stay — record them as verified-current, which is a real result.

**`D4` — report what would make the other 72 re-examinable, and STOP.** Do not
change the marker vocabulary, do not add wikilinks corpus-wide, and do not
retro-fit owners you inferred. **This is a convention question and it is the
Steward's to rule** — `COORDINATION §9` and `§4c`. Give me the options and their
costs; I will decide and frame the change if one is warranted.

## Acceptance criteria

**`AC-1`.** The census is reconciled across both shapes, with the counts stated
separately. **A single number with no shape attribution fails this.**

**`AC-2`.** Every disposition change cites a run with its result. Inherited from
the predecessor and it is still the likeliest way to get this wrong: **"the
condition looks satisfied" is not evidence.**

**`AC-3`.** No row is deleted and no row id changes.

**`AC-4`.** **Conditions whose owner is unresolvable are REPORTED, not
adjudicated.** Guessing an owner is worse than recording that none exists — it
produces a marker that looks maintained and is not.

**`AC-5`.** `crates/` byte-identical to the candidate base. Every gap here is
measured; none is repaired.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Do not edit `crates/`.** A failing fixture is a finding routed to me.
- **Do not create tracker nodes.** `COORDINATION §2`.
- **Do not touch the three sites [[CONF-STALE-RED-DISPOSITIONS]] owns** — the
  two `seed-buffer-io.md` sites and `conformance/README.md:439`. The
  `depends_on` records the ordering; if that node has merged, they are already
  correct and are `D1` census input, not work.
- **Do not change the marker vocabulary.** `D4` reports; it does not implement.

## Why this earns a slot

**The predecessor found eleven stale sites by hand and the corpus has 77
markers.** The ratio is the point: there is no reason to believe the eleven were
the stale ones rather than the *findable* ones.

**And the cheap fix is not obviously the right one.** "Wikilink every marker"
would make them censusable, but it also couples the conformance corpus to
tracker ids that get retired and renamed — `PX8-R` may already be an instance of
exactly that. **That is why `D4` reports to me instead of acting.**
