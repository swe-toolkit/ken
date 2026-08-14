---
id: LANG-STACK-ARC-EVIDENCE-USABILITY
title: "The trusted-base guard now localizes the bracket but reports a bare GlobalId, so it names no offender; and both frame-size figures this arc produced cite objdump without naming the artifact, so neither is reproducible by the next reader -- three repairs that make the arc's own evidence usable"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_39t1v4twehspd on squash 6c574cdd, triaged by the Steward and accepted. The hunt FIRED the repaired guard rather than reading it, confirming the D5 readability claim, and the three items below are what firing it turned up. Writability of the inverse lookup re-verified against main 6c574cdd: prelude.rs:498-505 is the guard, and elab.rs carries globals as a HashMap<String, GlobalId>."
---

> # RELEASED 2026-08-14 CARRYING [[LANG-POW10-CASCADE-LITERAL-CLAUSE]].
>
> **The two land as ONE `ken-elaborator` candidate, not two.** `POW10` is an
> `XS` doc-comment repair in `decimal_char.rs` that its own node says should
> ride the next candidate touching this crate rather than spend a ring turn
> alone. This is that candidate.
>
> **They are independent repairs sharing a build, not one deliverable.** If
> either stalls, land the other — do not hold a finished repair for its
> passenger. Read `POW10`'s "why this is not a second
> `LANG-REFINED-FALLBACK-COLDNESS-CLAIM`" section before sizing it; the
> resemblance to this arc is the trap it warns about.

## What this is

**Three repairs that share one property: this arc produced good evidence and
left it in a form the next person cannot use.** None is a defect in what
landed; each is the half a landed repair did not reach.

## First — the `D5` repair WORKS, and this node does not reopen it

**Confirmed by firing, not by reading.** The Adversary forced the guard with a
non-empty `combinator_expected_delta`. Verbatim:

```
base env: Internal("prelude declarations bracketed between combinator_trusted_before
and combinator_trusted_after (LANG-PRELUDE-COMBINATOR-BLOCK-DELTA D2) must contribute
nothing to the trusted base: expected {g999999}, got {}")
```

**Why it works is worth naming, because it is the reusable part:** it anchors on
`combinator_trusted_before` / `combinator_trusted_after`, which are **unique
greppable tokens** landing an author on the bracket in one search — durable in a
way a line range is not, and correct in a way the four spellings were not. Four
of four tests failed with identical text, so the signal survives the wall of
simultaneous failures.

## `D1` — the guard names no offender, which is what a trusted-base guard is for

**The residual the repair did not reach.** In a real failure the message reads
`expected {}, got {g123}`. **`g123` is a bare `GlobalId` with no name.**

⇒ The arc has gone *"names the wrong four"* → *"names none"*. **The strictly
better form names the right one**, and the data is already in the function.

**Writability re-verified at `main` `6c574cdd`:** the guard is
`prelude.rs:498-505`, and `elab.rs` carries `globals: HashMap<String, GlobalId>`.
The lookup is **name → id**, so naming the offender is an **inverse scan** over
that map, not a direct index. Cheap at this size; **say so in the code rather
than leaving the next reader to wonder why it is a scan.**

**If the id resolves to no name, say `<unnamed>` and still print the id.** A
diagnostic that can fail to render is worse than the bare id it replaced.

## `D2` — name the artifact behind each frame-size figure

**Neither number this arc produced is reproducible from what is written.**

`D4` recorded `check_match_dependent_refined_fallback`'s prologue as
`0x1000 + 0x48 = 4168` bytes/call, citing `objdump` **without naming the
artifact** — and the obvious one does not contain the symbol:

- `objdump -d --demangle libken_elaborator-*.rlib | grep -c refined_fallback` → **0**
- `nm libken_elaborator-*.rlib | grep refined_fallback` → **nothing**
- `register_prelude`, a `pub fn`, **is** present in that same artifact.

Plausibly a CGU or test-profile difference for a private `fn`. **The number is
NOT in dispute** — the Architect and QA both saw it, and the arithmetic it feeds
is sound (see below). **What is wrong is that the next person cannot re-derive
it.**

**The sibling figure has the same defect and its fix is already supplied.**
`register_prelude`'s `0x31000` was read from
`target/debug/deps/libken_elaborator-<hash>.rlib` under a default
`scripts/ken-cargo build -p ken-elaborator`. **Record that.**

⇒ **One clause per figure naming the artifact** — which rlib, or which `.o`, or
`--profile test`. That is the whole deliverable.

## `D3` — put the INSTRUMENT in the code, not the number

**`AC-7`'s disposition is CONFIRMED, and the Adversary now argues for it against
its own earlier suggestion.** Recording `~196 KiB` in a code comment would need
a custodian: **the number goes stale at the next unrelated edit to that
function, silently, and nothing reds** — the exact shape this arc has been
filing findings about all week. Keeping the figure in a node and out of the code
is the self-stabilizing side.

**What belongs in the code is the instrument.** One clause at `register_prelude`
saying its frame is large and sits beneath the cascade, so **read its prologue
before adding a local.** That never goes stale, needs no custodian, and gives
the next author the protection the number was meant to.

**Correction carried, because it strengthens `AC-7` rather than weakening it:**
the earlier claim that *"one `objdump` at an older SHA gives the trend"* was too
glib. That is a one-command read for a `pub fn` in a built rlib **and it needs
an old build besides.** **The trend read is not cheap**, which is an argument
for `AC-7`'s disposition, not against it.

## Recorded closed loop — the labelled assumption did its job

The earlier hunt derived a possible `+3352`/level from **assuming** the
fallback's frame was comparable to the mirror extraction's `6472`, and **labelled
the assumption**. `D4` measured `4168`. ⇒ **The real cost is `+1048`/level,
about `31.7 KiB` over 31 levels — comfortably inside the `~96 KiB` margin.**

**The pessimistic derivation is refuted by the read it asked for.** Recorded
because that is what a labelled assumption is supposed to do, and because the
refutation is the reason nobody needs to revisit the extraction choice.

## Acceptance criteria

**`AC-1` — `D1` is verified by FIRING the guard, not by reading it.** Force a
non-empty delta, confirm the message names the offending declaration, and
**report the verbatim text**. This is the same bar the `D5` repair was held to,
and firing is what established that one worked.

**`AC-2` — the unnamed-id path is exercised too.** Confirm the message still
renders when the id resolves to no name. A diagnostic that panics or renders
empty on its fallback path is a worse guard than the bare id.

**`AC-3` — `D2`'s artifact names are the ones actually used**, not reconstructed
from memory. If a figure cannot be reproduced from the named artifact, **say so
in the node rather than adjusting the number to fit** — an unreproducible
measurement is a finding, not a rounding error.

**`AC-4` — nothing landed by `LANG-REFINED-FALLBACK-COLDNESS-CLAIM` or
`LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` is reopened.** Both are merged and
correct. `D4`'s `4168` stands; this node makes it reproducible, it does not
re-measure it.

**`AC-5` — the `~196 KiB` figure stays OUT of the code.** `D3` adds the
instrument, never the number. If a reviewer asks for the number in the comment,
point at this AC and the staleness argument behind it.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run. Build targeted, `-p ken-elaborator`.

## Sizing

**`S`.** `D1` is an inverse lookup and a message change in one function; `D2` and
`D3` are clauses. **`D1` is the only one with a behaviour change**, and it is
confined to an error path.

## Not this node

- **Not a re-measurement of `4168` or `0x31000`.** See `AC-3`/`AC-4`.
- **Not the trend read** on `register_prelude`'s frame. It is not cheap, and
  `AC-7` on the merged node already ruled it out of scope.
- **Not a change to the guard's mechanism or the bracket's population.** The
  guard is fail-closed and correct; this changes only what it *says*.
