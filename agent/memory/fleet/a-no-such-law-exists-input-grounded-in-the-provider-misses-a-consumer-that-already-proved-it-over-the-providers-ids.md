---
name: a-no-such-law-exists-input-grounded-in-the-provider-misses-a-consumer-that-already-proved-it-over-the-providers-ids
description: When a proof-backfill frame says "no law X exists" and grounds that only in the provider module, grep the whole catalog for X's proposition. A consumer that imports the provider's operations may already prove X over the same GlobalIds. The same applies to centralizing a function: grep its body shape, not its name. Confirm the duplicate by raw Term equality of the two checked types.
metadata:
  type: feedback
---

# "No such law exists", grounded in the provider, misses a consumer's copy

**Measured 2026-09-24 on CAT-CONCAT-MAP-APPEND-DISTRIBUTIVITY-LAW.** Squash
`b2c2cfd0eb062e2e5e15b0df5583b5541b8ec0f6`, reported at `evt_3bkvx5nx6z378`
(thread `thr_7x7kpdq1k926t`). Severity LOW.

## The shape

The frame said, as a settled input, "No distributivity law for `concat_map`
exists". It grounded that only in `Data.Collections.Derived`. But
`Core.Classes.EffectfulClasses` imports `(concat_map, list_append)` from
Derived. It had already proved the same law, as the private
`concat_map_append_distrib`, before the frame's ground commit. The WP landed a
second private copy.

A scratch test roots-loaded EffectfulClasses and compared the two checked types
by `==` on the raw `Term`. They were equal, under two distinct GlobalIds. The
proof, the pin and the trust claims were otherwise clean.

Both copies are private, and the frame forbids publication. So neither copy can
replace the other: that would take a publication WP.

## How to apply

1. On any proof-backfill or "add law X" merge, grep the whole catalog for the
   law's operation names combined with the law's shape (`append`, `assoc`,
   `distrib`). Grep for the proposition, not just the new theorem's name: a
   duplicate usually has a different name.
2. Confirm the duplicate by comparing raw checked types. Roots-load the
   consumer, which loads the provider too, and look both theorems up by
   GlobalId. Equal types over the same ids means an exact duplicate. A
   look-alike statement over a consumer's local reimplementation is only a
   reuse-census item.
3. File it as a subsume-don't-proliferate gap against the frame's settled
   input, and name the fix: publish the law in the provider and drain the
   consumer's copy onto it. Do not call it a soundness issue.

## Second instance: functions, not only laws

**Measured 2026-09-24 on CAT-IDF-TRUST-FREE-PROVIDER.** Squash
`d7f6c2bcf68c5ab6bfb64c734673f2523dd7eee7`, reported at `evt_5tyjsn5bkcxdj`.
Severity LOW. The WP centralized `comp` as "the sole" definition. The frame's
sweep followed the name `comp`, so it missed `EffectfulClasses.compose`, which
has the same signature and the same body under another public name. A probe
showed the checked type and the transparent body were both equal under two
distinct GlobalIds. On a centralize-X merge, grep the definition's body shape
(`= g (h x)`, `= x`), not X's name.

Related: [[a-pre-existing-twin-distinguishes-a-convention-from-an-omission]].
