# `Standard` — the standard operator surface

The public path at which Ken's standard infix operators reach the ordinary
checked functions that give them meaning.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

`31 §1c` admits `≤`/`<=`, `≥`/`>=`, `≠`/`/=`, `∧`/`/\`, `∨`/`\/` and `∈` as
ordinary symbolic names. Admission fixes names, not meanings. `33 §6.1` fixes
the meanings, and every binding-backed one is an ordinary public function.
`bool_and`, `bool_or`, `ord_leq_at` and `ord_geq_at` live in
`Core.Classes.LawfulClasses`, beside the
`Ord` class whose vocabulary they belong to. `membership_member_at` lives in
`Core.Classes.Membership`, beside its provider class.

This entry is the **path**, not the definitions. It re-exports those five
identities under the operator spellings, so a reader writing `x ≤ y` and a
reader writing `ord_leq_at a d x y` are naming the same binding by two surface
routes.

Re-export is the whole mechanism and it is load-bearing. `33 §4.3` states that
an `export` republishes the existing `GlobalId` and never mints another,
**including through renaming**. A second definition here would give one meaning
two identities, and `39 §6.9`'s completion policy keys on the defining
`GlobalId` rather than on the glyph — so a duplicate would make its own stated
consequence, *a renamed standard binding still completes*, ill-defined. The
facade exists precisely so that no such duplicate is needed.

## 2. Definition

The operator surface. Each line republishes one already-defined identity under
the spelling `33 §6.1` assigns it:

```ken
export Core.Classes.LawfulClasses
  (bool_and as ∧, bool_or as ∨, ord_leq_at as ≤, ord_geq_at as ≥)

export Core.Classes.Membership (membership_member_at as ∈)
```

There is no `import` above that `export`, and that is deliberate rather than an
omission. `33 §3.2`'s facade form publishes the selected names to this module's
clients **without** installing them in this module's own body scope, which is
exactly right for a path that defines nothing and consumes nothing.

## 3. Using it

A client imports the operator spellings from here and the ordinary names from
the defining module, and both reach one identity:

```ken ignore
import Core.Operators.Standard (≤, ∧)

import Core.Classes.LawfulClasses (Ord, ord_leq_at)

fn both_routes_agree (a : Type) (d : Ord a) (x : a) (y : a) : Bool =
  bool_and (ord_leq_at a d x y) (ord_leq_at a d x y)
```

## 4. Laws & proofs

This entry states no laws and inhabits no proof terms. It defines nothing, so
there is nothing here for a law to range over. The laws of the operators'
meanings are stated where those meanings are defined — `Ord`'s `refl`,
`antisym`, `trans` and `total` in `Core.Classes.LawfulClasses`, and the
`bool_and`/`bool_or` introduction and elimination proofs beside them.

That is the intended consequence of re-export preserving identity: a law proved
of `ord_leq_at` is a law of `≤`, because they are one binding.

## 5. Design notes

**Why `≥` is not a class field.** `class Ord a` declares `leq`, `refl`,
`antisym`, `trans` and `total`, and gains no `geq`. `ord_geq_at` is derived over
`leq` and **reverses its two already-evaluated argument values inside its own
body**. The distinction is observable and `33 §6.1` fixes it: operand evaluation
at a `≥` occurrence stays single and left to right, exactly as for any other
application. A definition that rewrote `x ≥ y` into `y ≤ x` at the surface would
change evaluation order and would not be this binding.

**Why `∧` and `∨` do not short-circuit.** Both operands are evaluated, left to
right. `18a §5.4`'s `Bool` eliminator arm laziness is easy to misread as a
claim about operands; it is a property of the **body's arms**. Under
call-by-value the operands are already evaluated when `bool_and`'s body runs,
and its `match` then ranges over values. Non-forcing of an untaken arm yields no
operand short-circuiting at a use site.

The compiler certifies these re-exported identities against their dependent
function shapes and installs the standard fixities from `33 §6.1`. `∈` is
non-associative at precedence 4. Its completion infers the right-hand carrier,
resolves that carrier's `Membership` dictionary, projects the dictionary's
`Query`, and only then checks the left operand.

`≠` remains distinct: its meaning is the negation of the comparator the `==`
path selects (`33 §6.2`) rather than one binding to re-export.

**Why this is a separate entry and not part of `LawfulClasses`.** A module
cannot re-export what it defines — `33 §4.3` makes `export foo` on a local name
equivalent to `pub foo`, with no second identity — so the defining module cannot
be the re-exporting home. And `33 §6.3` puts `∈` in this same home, whose
provider is `Membership` rather than a lawful class; hosting the operator
surface here keeps a non-class provider out of the classes module when that
lands.

## 6. References

- `spec/30-surface/31-lexical.md §1c` — admission of the operator spellings.
- `spec/30-surface/33-modules-and-names.md §3.2`, `§4.3` — the facade export
  form, and re-export preserving identity through renaming.
- `spec/30-surface/33-modules-and-names.md §6`, `§6.1`, `§6.2` — fixity as a
  property of identity; the standard operator bindings; the `≠` carrier
  inventory.
- `spec/30-surface/39-elaboration.md §6.9` — standard-operator call completion,
  keyed on the defining `GlobalId`.
- `catalog/packages/Data/Numeric/Nat/Order.ken.md` — the in-tree facade
  precedent: re-export the generic, define the specific.

## 7. Trust & derivation

**`trusted_base()` delta: zero, and structurally so.** This entry declares
nothing. It contains one `export`, which `33 §6` states is
surface-and-elaboration only — fixity guides parsing into the same core term the
kernel re-checks regardless of which path named the operator. A module with no
definitions cannot add an `Axiom`, a postulate, or an opaque constant.

**Derivation path.** The Boolean and ordering identities derive through
`Core.Classes.LawfulClasses`: `bool_and` and `bool_or` are match-based `Bool`
functions, while `ord_leq_at` and `ord_geq_at` project `leq` from an `Ord`
dictionary. `membership_member_at` derives through `Core.Classes.Membership`
and projects `member` from a `Membership` dictionary. None is a built-in, a
kernel rule, or a primitive.

**Consumers.** None yet at the operator spellings. `ord_leq_at` itself is
heavily consumed under its ordinary name — `Data.Collections.PriorityQueue`
alone applies it in the partially-applied form `ord_leq_at k d`, which stays a
valid two-argument application and is not rewritten into a four-argument call.
