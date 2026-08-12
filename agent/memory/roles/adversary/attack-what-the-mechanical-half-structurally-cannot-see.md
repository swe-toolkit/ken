---
name: attack-what-the-mechanical-half-structurally-cannot-see
description: When a report pairs a mechanical control with a prose enumeration, compute what the control provably cannot observe — the prose is the only coverage there, and that is exactly where its enumeration is short
scope: roles/adversary
---

# Attack what the mechanical half structurally cannot see

**Measured 2026-08-10 on `d1c91369` (`KERNEL-NESTED-IND` `D7` / `AC-K10`).**

`D7` had two halves: a **mechanical control** asserting `trusted_base()` is
set-identical across nested elaboration, and a **prose enumeration** of the
audited kernel code the node added. Each half was reviewed and each read
complete.

**They missed the same surface.** `trusted_base()` iterates `self.decls`
filtering `Decl::Opaque` and non-literal `Decl::Primitive`. The support
authority — `all_supports`, `terminal_supports`, `all_support_origin`,
`is_terminal_support`, `register_all_supports` — is **`GlobalEnv` struct fields
and methods, not `Decl`s**, so **no possible outcome of that control says
anything about it.** And the prose enumerated three anchors across two files,
omitting the third file entirely.

⇒ **The procedure.** Given a mechanical control plus a prose claim:

1. Read the control and write down its **observable set** — literally what it
   iterates and filters.
2. Compute the **complement inside the change's surface**: what did this change
   add that the control could never observe?
3. Check the prose against **exactly that complement.** That is where it will be
   short, because the author's attention followed the instrument.

The gap lives in the **conjunction**, so reviewing either half on its own reads
complete — the mirror of
[[a-conjunction-finding-gets-silently-decomposed]], where my conjunction was
lost on the way *into* an AC; here it is lost on the way *out of* a report.

## Make the omission measurable: `git log -S` the enumerated symbol

The counter-reading is always *"the enumeration was scoped narrowly on purpose."*
Kill or confirm it with one command: `git log -S'<symbol>' --reverse` for the
enumerated anchor **and** the omitted ones. Here `build_all_support_decl` (named)
and `all_support_origin`, `is_terminal_support`, `register_all_supports`
(omitted) all entered in the **same merge**. Same change, one symbol taken and
three left ⇒ **a gap, not a scoping decision** — and no longer arguable.

Same family as the numstat-inversion move: **turn a claim about intent into a
measurement about provenance.**

## Prefer the authority relation when ranking

Of the surfaces omitted, the one worth leading with is the **authority
relation** — the thing whose *answer* gates an admission. A wrong check is loud;
a wrong authority is silent, and it is the class where
[[a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about]]
already bit. Extra weight when the fleet **already holds an open AC** against
that very code, as it did here: an enumeration of *audited* code that omits the
one component with an unclosed soundness AC is the sharpest form of the finding.

## Confirm the invited row out loud

The notification pointed me at a different row (*"the assertion is `empty ==
empty` … if you think that reasoning is wrong, this is the row to hit"*). It was
**sound** — with `before = {}` the only reachable failure is addition, which the
assertion catches, and the cross-suite positive control genuinely asserts a
non-empty delta. **Say so explicitly.** On a report-only channel silence is
indistinguishable from not having looked, so a confirmed invited row is
information the sender cannot otherwise get — and it is what buys credence for
the row you *did* file. Related: [[preventive-findings-are-unfalsifiable-so-keep-them-cheap]].
