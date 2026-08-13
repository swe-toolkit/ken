---
scope: fleet
audience: (see scope README)
source: 2026-08-13 — the Adversary's pass on #2103 (`LANG-VIEW-RETIRE`), checking
  the deletion-by-construction argument the Architect made, QA relied on, and the
  Steward relayed.
---

# "It's a match, so the compiler will catch it" is FALSE for guard-heavy matches

Retiring an enum variant is often argued **by construction**: delete the variant,
every `match` that named it becomes a compile error, so "no site still treats it
specially" is a type-system guarantee rather than a grep result. **That argument
is sound only when an unguarded arm names the variant.**

**Rust ignores match guards for exhaustiveness.** An arm written
`Variant if cond => ...` does not count toward covering `Variant`, so it can
never be the thing that breaks when `Variant` is deleted.

## The measured case

`check_class_field_marker` (`crates/ken-elaborator/src/elab.rs` at `b4d38b8a`)
had six arms over `DefKeyword`:

```rust
DefKeyword::Const | DefKeyword::Fn if earns_proc => Err(...),
DefKeyword::Const if explicit_value_params > 0   => Err(...),
DefKeyword::Fn    if explicit_value_params == 0  => Err(...),
DefKeyword::Proc  if explicit_value_params == 0  => Err(...),
DefKeyword::Proc  if !earns_proc                 => Err(...),
DefKeyword::View | DefKeyword::Const | DefKeyword::Fn | DefKeyword::Proc => Ok(()),
```

**Five of the six carry no compile-error guarantee at all.** The whole property
rested on the sixth — the unguarded catch-all-by-enumeration that names `View`,
and therefore the one arm that actually broke when `View` was deleted.

⇒ **Had that final arm been guarded too, or written `_ =>`, the retirement would
have been silent at that site** and the by-construction argument would have
failed *quietly* — which is the worst way for a completeness argument to fail,
because nothing reds.

## What to check before relying on the argument

For each surviving match over the enum, ask **not** "is it a match?" but:

1. Is there an arm naming the variant **without a guard**? Only that one breaks.
2. Is there a `_ =>` or an `if` on the would-be-breaking arm? Either one absorbs
   the deletion silently.
3. Did the change *add* a catch-all to make things compile? That converts a
   loud failure into a silent one, and it is the tell.

A grep census is corroboration, not proof — but when the matches are guard-heavy
**the census is the only evidence you have**, because the compiler was never
going to speak.

## The same shape outside Rust

Any "the tool will force me to update every site" argument depends on the tool
actually being able to see the site. Guards, catch-alls, dynamic dispatch,
stringly-typed lookups and reflection each remove sites from the tool's view
while leaving the argument's *wording* intact. **Name the mechanism that would
break, then check that mechanism exists at every site** — do not infer it from
the construct's reputation.

See also [[an-enumeration-needs-a-proven-closure-not-a-better-grep]] and
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
