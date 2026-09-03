---
id: CAT-MIGRATE-TIER-A-PROVIDERS
title: "Scaffold-retirement Tier A (primitive providers): publish the two remaining consumer-needed Derived provider values nth and bytes_nat_length as pub, so lower-tier consumers (Cursor, Parsing, Process.Arguments) import the canonical values instead of reaching them through fixture scaffolding. Census folded the tier to one module: Transport/Compare/Arithmetic/Nat.Order are already sufficiently published."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: [CAT-SCAFFOLD-RETIREMENT]
github: null
origin: "Steward, 2026-09-02. Tier A of the scaffold-retirement migration (parent CAT-SCAFFOLD-RETIREMENT; Architect 5-tier DAG decomposition evt_2e0pee5jxzv07). Tier A = primitive-provider export publication, no consumer edits. Foundation-leader STEP-2 census at exact origin/main e485a696c (evt_3qgh6dvwf647v, cross-checked against the STEP-2 dynamic inventory + source): Transport cong/sym/trans already public (pub theorem :53/:71/:78); Compare list_eq already pub fn (:369); Arithmetic add already pub fn (:18), remaining privates not consumer-needed; Nat.Order sub already pub fn (:69), no private value/law consumer-needed. ONLY delta = Data.Collections.Derived nth (:83) + bytes_nat_length (:892), both private fn, consumer-needed by Cursor/Parsing/Process.Arguments/higher; no attached law consumed in the measured import lists, no [higher] hazard measured. Same shape as CAT-DERIVED-PUB-EXPORT / CAT-ORD-LEQ-PUB-EXPORT (a prior pub-export that published Derived's other surface)."
---

> # MERGED (Steward, 2026-09-03) — squash `f1d7d4133` on `origin/main`, full CI
> # green. Blob-verified: `pub fn nth` (:83) + `pub fn bytes_nat_length` (:892)
> # on main; the `cat_derived_pub_export.rs` inventory extended to the 11-name
> # set. Foundation QA `evt_2dmz06114m8hy` + CV `evt_rht7rmk31qdq` approved on
> # exact `6fc9fca04`; Decision `dec_11r8mfh83ma60`; the conditional Architect
> # gate did not trigger (no attached-law hazard). Successor: Tier B
> # (Core.Classes + the operator-ruled DecEq->LC relocation), framed next.
> #
> # Tier A of the scaffold-retirement migration: the primitive-provider export
> # publication, folded by census to two Derived values.
>
> The Architect's bottom-up DAG (evt_2e0pee5jxzv07) places primitive providers
> at the base: publish their export surfaces so every higher tier imports the
> canonical value instead of a scaffolding-reached one. STEP-2 measurement
> (foundation-leader evt_3qgh6dvwf647v) collapsed Tier A's five candidate modules
> to ONE real delta — `Derived::{nth, bytes_nat_length}`. This is the exact shape
> landed repeatedly (CAT-DERIVED-PUB-EXPORT, CAT-BOOL-PUB-EXPORT,
> CAT-ORD-LEQ-PUB-EXPORT): mark the value `pub`, prove the export is real by
> extending the module's loader-visible inventory-equality control, not a `^pub`
> grep.

## Fixed inputs (measured at origin/main `e485a696c`)

- **The two symbols, both PRIVATE (bare `fn`) and consumer-needed.**
  - `catalog/packages/Data/Collections/Derived.ken.md:83`
    `fn nth (a : Type) (n : Nat) (xs : List a) : Option a = ...`
  - `catalog/packages/Data/Collections/Derived.ken.md:892`
    `fn bytes_nat_length (bs : Bytes) : Nat = length UInt8 (bytes_to_list bs)`
  - Re-measure the exact lines at the SHA you build on (a line number decays).
- **No attached law.** The STEP-2 measured import lists consumed neither value's
  attached law (none was measured for these two). Publish the FUNCTIONS only.
- **Current Derived published inventory = 9 loader-visible names**, asserted by
  `crates/ken-elaborator/tests/cat_derived_pub_export.rs`
  (the `assert_eq!` at :223-238): `concat_map, count, eq_from_ord, length,
  list_append, list_append::assoc, list_append::left_unit,
  list_append::right_unit, reverse`. Publishing `nth` + `bytes_nat_length`
  extends this to **exactly 11** — adds those two names, nothing else.
- **Providers-only.** The consumer sites (Cursor, Parsing, Process.Arguments,
  and higher) are NOT touched here — repointing them off scaffolding onto these
  imports is the later-tier consumer work, not this node.
- **The other four Tier-A modules fold out** (census evt_3qgh6dvwf647v): Transport
  (`cong`/`sym`/`trans` already `pub theorem`), Compare (`list_eq` already
  `pub fn`), Arithmetic (`add` already `pub fn`; other privates not
  consumer-needed), Nat.Order (`sub` already `pub fn`; no private value/law
  consumer-needed). No node work for them.

## Deliverable

- **D1 — mark `nth` and `bytes_nat_length` `pub` in Derived and extend the
  loader-visible inventory.** Two `pub` keywords at the two lines above; update
  the Derived equality assertion in `cat_derived_pub_export.rs` to the 11-name
  set; add a required reddening mutation PER value; re-run the affected closure.

## Acceptance criteria, each with its control

- **AC-EXPORTED (positive, per value).** Each of `nth`, `bytes_nat_length` is
  LOADER-VISIBLE from `Data.Collections.Derived`, measured by the loader — a
  selective import `import Data.Collections.Derived (nth)` (resp.
  `bytes_nat_length`) resolves it to the transparent provider `GlobalId`, not by
  a `^pub` grep. Control: the selective-import probe resolves; a probe for a
  still-private Derived name still rejects as `UnboundName`.
- **AC-EXACT-INVENTORY.** The Derived loader-visible inventory equality in
  `cat_derived_pub_export.rs` equals the 11-name set (the 9 above plus `nth`,
  `bytes_nat_length`). Population from the module's own definitions, verdict from
  the loader, **EQUALITY** — exactly these two names flip private to public and
  no other name's visibility changes — never a per-name spot-check, never a
  hand-edited roster.
- **AC-EVASION-REDDENS (per value).** Publishing an excluded name in any
  compile-preserving spelling — including one leading space before `pub`, which
  the roots loader accepts and publishes at the exact provider `GlobalId` — must
  RED this control. Byte-restore afterwards and show the restoration. A
  per-value mutation (remove `nth`; remove `bytes_nat_length`) each reds the
  equality distinctly, proving both additions are independently load-bearing.
- **AC-STANDALONE-GREEN.** Derived still elaborates standalone (exit 0) after the
  pub-marking: no `UnresolvedCon`, no eligibility rejection, no attached-law
  nonlocal-position hazard. The census measured no `[higher]` hazard for these
  two, but do not assume it — measure base standalone-green first, then again
  after the two `pub` keywords. A `[higher]` hazard is a HARD STOP to the
  Architect, not a workaround.
- **AC-NO-COMPUTATIONAL-CHANGE.** `pub` is export-visibility only; a differential
  over function bodies shows byte-unchanged. Only Derived changes; the consumer
  sites are untouched (providers-only).
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  that loads Derived or a module whose closure this changes), scoped by which
  PATHS changed. Targeted via `scripts/ken-cargo`, never `--workspace`;
  whole-suite green is CI's job.

## Gate and sequencing

On the candidate: fresh Foundation QA + CV on the exact SHA, then Steward
M1-M4. The Architect enters only if the attached-law hazard fires (a design
fork), per AC-STANDALONE-GREEN. Size is two symbols plus a test extension — a
sub-hour turn or a genuine hard stop. `gate: none` (no TCB touch).

This is Tier A of [[CAT-SCAFFOLD-RETIREMENT]]. Its successor is Tier B
(Core.Classes + the primitive-instance consolidation, where the operator-ruled
`DecEq` relocation to LC lands); Tier B's consumer sites import these two values,
so it sequences after this. Do not fold any consumer repoint into this node —
this is the providers-only base of the migration.
