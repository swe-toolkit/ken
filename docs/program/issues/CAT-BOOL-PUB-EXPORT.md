---
id: CAT-BOOL-PUB-EXPORT
title: "Mark the three census group-6 boolean providers loader-visible (pub): bool_and and bool_leq in Core.Classes.LawfulClasses, is_some in Data.Sums.Combinators. The provider prerequisite for census group 6 (Boolean computational reuse), shaped on the twice-landed CAT-ORDER-PUB-EXPORT / CAT-DERIVED-PUB-EXPORT pattern."
status: ready
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-29, filed on the CAT-DERIVED-REUSE-CONSUMERS closure (group 4 drained, 3829c1baa) so lane 3 does not idle. Group-6 membership and provider standalone status re-measured at origin/main 3829c1baa: docs/program/cat-reuse-census.md §4.4 item 6 lists three [low] consumer sites (Derived#bool_and, Derived#bool_leq, Map#option_is_some); §3 rows 66/72 show both providers pass standalone [ok] exit 0 with the three names PRIVATE. No prerequisite of its own — the providers already elaborate standalone. Steward-filed per COORDINATION section 2."
---

> # RELEASED — lane 3, the group-6 provider prerequisite. `ready`.
>
> This is the provider half only. It exposes three names; it drains NO consumer
> site. The group-6 consumer drain (Derived#bool_and, Derived#bool_leq,
> Map#option_is_some) is a SEPARATE successor node that will `depends_on` this
> one, shaped on `CAT-DERIVED-REUSE-CONSUMERS`.

## Fixed inputs (re-measured at `origin/main` 3829c1baa)

Census group 6, **"Boolean computational reuse"**, quoted verbatim from
`docs/program/cat-reuse-census.md` §4.4 item 6 — three `[low]` consumer sites,
each needing a now-private provider marked loader-visible:

| consumer site (§4.4) | provider (§3) | provider module | current |
|---|---|---|---|
| `Data/Collections/Derived.ken.md#bool_and` | `LC.bool_and` | `Core/Classes/LawfulClasses.ken.md` | private |
| `Data/Collections/Derived.ken.md#bool_leq` | `LC.bool_leq` | `Core/Classes/LawfulClasses.ken.md` | private |
| `Data/Collections/Map.ken.md#option_is_some` | `SC.is_some` | `Data/Sums/Combinators.ken.md` | private |

Provider standalone status (§3 rows 66, 72): `LC` = `Core.Classes.LawfulClasses`
is `[ok]` standalone exit 0 with `bool_and`/`bool_leq` private; `SC` =
`Data.Sums.Combinators` is `[ok]` standalone exit 0 with `is_some` its only,
private definition. Both providers already elaborate standalone, so this node has
no prerequisite of its own.

## Deliverable

Mark exactly three names loader-visible (`pub`) in their provider modules — two
in `Core/Classes/LawfulClasses.ken.md` (`bool_and`, `bool_leq`), one in
`Data/Sums/Combinators.ken.md` (`is_some`) — and nothing else.

## Acceptance criteria

- **AC-EXPORTED** — the three names are LOADER-VISIBLE from their modules,
  measured by the loader (an actual import resolves them), not by a `^pub` text
  grep. This carries the `CAT-DERIVED-PUB-EXPORT` correction directly: the
  `^pub` census was a PROXY for export visibility resting on an unstated
  column-0 invariant, and a one-leading-space ` pub fn` is loader-published while
  the census stays green.
- **AC-EXACT-INVENTORY** — the control is a loader-visible inventory: population
  from each module's OWN definitions, verdict from the loader, EQUALITY (exactly
  these three names change from private to public and no other name's visibility
  flips), NOT a per-name privacy spot-check.
- **AC-EVASION-REDDENS** — the one-leading-space ` pub` evasion is a required
  reddening mutation: an inventory that stays green when a name is published with
  a column-0 violation has not measured visibility and fails this AC.
- **AC-STANDALONE-GREEN** — both provider modules still elaborate standalone
  (exit 0) after the pub-marking. If exposing `bool_and`/`bool_leq` pulls an
  attached law into a nonlocal position (the `[higher]` hazard the census records
  for boolean items in OTHER consumer contexts), standalone reddens — that is a
  HARD STOP to the Architect, not a workaround.
- **AC-PROVIDERS-ONLY** — this node changes ONLY the two provider modules. The
  three consumer sites are untouched; draining them is the successor node's work.

## Reviewers

Foundation QA (AC controls red/green as specified; AC-EVASION-REDDENS actually
reddens on the column-0 violation) + conformance-validator (the CV rejected
`CAT-DERIVED-PUB-EXPORT`'s proxy census and specified the loader-visible
inventory that replaced it — the CV owns that the inventory here is loader-truth,
not a `^pub` proxy). A pub-marking that turns a provider module non-standalone
HARD-STOPS to the Architect.

## Capability tier

T2 — a mechanical, precedent-shaped catalog visibility change (three names, two
files), reviewed on a loader-visible inventory and standalone-green, not on an
argument. Size S.

## Sequencing

Lane 3 (foundation). Released 2026-08-29 on the group-4 closure so the lane does
not idle. No `depends_on` — both providers pass standalone. Its successor is the
group-6 consumer-drain node (framed after this lands, shaped on
`CAT-DERIVED-REUSE-CONSUMERS`); groups 5 and 7 are not re-measured and are not
framed here (§4c — frame on need, not ahead of it).
