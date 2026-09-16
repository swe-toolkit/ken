---
id: SPEC-32-PROJECTION-PRECEDENCE
title: "amend 32 §3 so projection is a POSTFIX form on `primary` rather than an application arm -- `application_atom ::= primary (\".\" ident | \".1\" | \".2\")*`, making `keep box.value` read `keep (box.value)` with projection binding tighter than application, and strike `and projection` from the five-leading-forms rejection clause it was inherited into by omission; NO change to the five leading forms themselves, NO parser work"
status: merged
owner: spec
size: S
gate: none
depends_on: [SPEC-RESERVED-INFIX-NAMES]
blocks: []
github: null
tier: T1
origin: "Steward filed 2026-09-16, RETROACTIVELY, after the work had already landed. The amendment was authored by spec-author, routed by the Steward as candidate `ed0c02412`, and landed as `f1bda0c5b` -- with no tracker node at any point. Measured at main b34ffd182: ELEVEN of the 12 most recent spec landings carry a node, and this is the ONLY one that does not. (The first pass reported 10 of 12. The second apparent gap, `5c283a88e LANG-TRUNC-SURFACE-spec`, was a FALSE ABSENCE: its node is `LANG-TRUNCATION-SURFACE-SYNTAX.md`, status merged. A key derived from the commit subject cannot match a node filed under a different spelling -- spec-author caught it, and the control for this finding had the very defect the finding is about.) Filed so the tracker records work that is done, and so the two follow-up nodes it spawned have a named origin rather than a commit SHA."
---

> # FILED AFTER THE FACT. The amendment LANDED at `f1bda0c5b`; this node is the
> # tracker's record of it, not a request to do it.
>
> **Nothing here is actionable.** It exists because the work had no node, and
> the Steward routes on node status -- so node-less landed work is invisible to
> the only batch landing instrument the Steward has.

# What landed

Three files, spec only, no `crates/`:

    spec/30-surface/31-lexical.md                       +15
    spec/30-surface/32-grammar.md                       +52 -27 (net)
    .../surface/operators/seed-reserved-infix-names.md  +43

Verified at `origin/main` `b34ffd182ca726ee7920cc7c14a28fd6ef0df1a4` by content,
not by ancestry.

## The production

`spec/30-surface/32-grammar.md:277`:

    application_atom ::= primary ("." ident | ".1" | ".2")*   -- postfix projection chain

and `:336`, in prose: projection is a **postfix form on `primary`** and
therefore binds more tightly than application.

⇒ `keep box.value` is `keep (box.value)`. The superseded reading was
`(keep box).value` -- application first, projection second.

## The clause that was struck

`32 §3` previously listed projection among the forms that reject at their
leading token when ungrouped after an application head. It does not any more;
`:400` now says so explicitly:

> **Projection is not in that set, and this paragraph previously said it was.**

**The five leading forms are unchanged** -- lambda and `let`, `if`, `match`,
`temporal`. That sentence is byte-stable across the amendment, which is why
[[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] was never gated on this landing.

## Why the old text said projection, and why that matters

`:349-352` records it: before `7dea59366` the application arm read
`| expr application_atom` and carried the **same** projection arm. The
application arm was later narrowed; the projection arm was never touched.

⇒ **Projection was inherited by omission rather than introduced by an act.**
That is the finding worth keeping: the superseded reading was never anybody's
decision, so there was no rationale to weigh against changing it.

# What this node caused, and what it did NOT

**Caused.** The parser already conformed to the amended pin -- its existing
grouped reading is now the specified one. So
[[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] was re-cut to its `if` half
alone and its projection half was **inverted, not descoped**: the divergence it
was going to repair stopped being a divergence. `dec_wac7adfhr23c` is VOID and
candidate `7c6dfdfaf` must not be respun.

**Did not.** Two grammar questions the amendment opened are NOT closed by it,
and are filed separately rather than carried:

- [[SPEC-32-PATH-VS-PROJECTION-CHAIN]] -- groundable now.
- [[SPEC-32-QUALIFIED-GLOBAL-REF-VS-PROJECTION]] -- blocked.

**Neither is a defect in this amendment.** A postfix projection chain on
`primary` is the right shape; what it does is put that chain into contact with
two productions written before it existed.

# Related

- [[SPEC-RESERVED-INFIX-NAMES]] -- `7dea59366`, the narrowing that stranded the
  projection arm.
- [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] -- re-cut on this; landed
  `9a1b8247`.
- [[LANG-TYPE-PROJECTION-SURFACE-FORM]] -- projection in TYPE position, a
  separate and still-open gap (`RType` has no projection variant).
