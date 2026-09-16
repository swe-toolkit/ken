---
id: LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE
title: "Bring the parser into conformance with spec/30-surface/32-grammar.md §3's application-atom contract pin -- the five ungrouped leading forms reject at their leading token, and arrow/projection nest the application in their left expr ((keep Nat) -> Nat, (keep box).value) -- and migrate the catalog source written against the current non-conforming parse. Two measured divergences, in opposite directions: `if` is ACCEPTED as a bare application argument where the pin requires rejection, and `keep box.value` parses A(keep, Proj(box, value)) where the pin requires Proj(A(keep, box), value)."
status: draft
owner: language
size: L
gate: none
depends_on: [SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16 at the Architect's naming, on their D0 REVERSAL (evt_5sf71fnjxpzmb), which vacates the earlier WITHDRAW ruling (evt_64avxs9ashqqk). The divergences were measured by spec-author under the SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION hard stop (evt_7p78v6qb429rz) and read against the spec by the Architect. Steward verified the two quoted spec passages verbatim at origin/main e11341c7b9d11cd74879d27d555d2a5729837847: 32-grammar.md :357-363 (ungrouped operator_name is not a general application_atom, 'applies equally to generic and reserved operator names') and :368-378 (the five leading forms reject; arrow and projection nest, as `(keep Nat) -> Nat` and `(keep box).value`; 'part of §3's contract pin; an implementation must not restore a second unrestricted application production'). The catalog-source extent is UNMEASURED -- see Sizing. Not yet framed; the Steward frames and releases it when the language lane reaches it."
---

> # DRAFT -- cut so the seed rows have a real node to point at.
>
> This node exists because [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]]
> re-gates two row groups of `seed-reserved-infix-names.md` onto a named
> successor, and a `RED-UNTIL-<node>` tag naming a node that does not exist is
> not a gate. It is **not released**: it has no frame. Do not start it.
>
> **The census IS RUN (2026-09-16) — it is no longer the blocking unknown.**
> See "The census" below: the migration is of order twenty sites with more than
> 90% of them in one file, the `if` row has zero catalog tail, and the node does
> **not** decompose on size. What remains before a frame is the D0 AST ledger,
> not a measurement of whether this is tractable.
>
> **Two release preconditions, neither of them about the frame.** Its dependency
> [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]] is routed and **not landed**;
> and it shares `crates/ken-elaborator/src/parser.rs` with
> [[LANG-RESERVED-INFIX-NAMES]], whose A0 candidate is **live**. Check both at
> `origin/main`, by node status and by blob — never by branch ref.

# Objective

Make the parser conform to the application-atom contract pin in
`spec/30-surface/32-grammar.md §3`, and migrate the catalog source that was
written against the current non-conforming parse.

The pin is already on `main` and is not in question. This node does not decide
whether the contraction is wanted -- the spec decided that. It closes the gap
between the spec and the implementation.

# The divergence (measured)

Measured by spec-author against the base parser; read against the spec by the
Architect. Two divergences, in opposite directions, out of four rows:

| row | `32 §3` requires | parser today | verdict |
|---|---|---|---|
| lambda / `let` / `match` | reject ungrouped | rejects | CONFORMS |
| `if` | reject ungrouped | ACCEPTS `A(keep, If(...))` | **DIVERGES** |
| arrow | `(keep Nat) -> Nat` | same | CONFORMS |
| projection | `(keep box).value` | `A(keep, Proj(box, value))` | **DIVERGES** |

The bare-operator-value group is pinned by the same section at `:357-363`, for
generic and reserved names alike -- an ungrouped `operator_name` is admitted
only as the head of `operator_prefix`, so `(OP)` grouping is required to use it
as an ordinary atom.

# Why the catalog migration is in scope by construction

The language ring's A0 checkpoint `2c63d409` implemented the seed -- which is
the spec -- and the focused catalog control
`modules::namespace_effect_tests::priority_queue_actual_export_table_is_exactly_the_six_name_api`
failed candidate-only with `TypeMismatch ... projection base's type is not a
named-field owner`. Restoring the legacy parse returned it to 1/1.

That failure is not evidence against the contraction. **Existing catalog source
relies on ungrouped projected arguments, i.e. it is written against the
non-conforming parse.** Conforming the parser therefore necessarily moves the
catalog source with it, and the two cannot be separated -- which is precisely
what made the contraction out of scope for
[[LANG-RESERVED-INFIX-NAMES]] (A0), whose decomposition is bounded to unchanged
syntax.

# Deliverables (indicative -- this node is not framed)

- Parser conformance for the `if` row: an ungrouped `if` after an application
  head rejects at its leading token.
- Parser conformance for the projection row: `keep box.value` parses as
  `Proj(A(keep, box), value)`.
- Bare-operator-value grouping per `:357-363`, generic and reserved alike.
- Catalog source migration for every site that relies on the non-conforming
  parse, with a census establishing the set.
- The re-gated `seed-reserved-infix-names.md` rows go green.

# Not this node

- Any change to `32 §3` itself. The pin is the authority this node answers to;
  narrowing it would remove the grounding rather than satisfy it.
- The six-name admission of [[LANG-RESERVED-INFIX-NAMES]] (A0), which is
  released and bounded to unchanged syntax.
- The temporal rows (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`), a separate gate.
- The lambda/`let`/`match` and arrow rows, which conform today.

# The census — RUN (Steward, 2026-09-16). Read the instrument before the number.

The section below used to say *"the first framing act is that census."* It has
been run. **It does not decompose on size, and the reason is that the migration
is concentrated in a single file.**

**Roots swept — both of them.** Catalog source is literate `.ken.md`, and it
lives under **two** roots, not one: `library/` (4 files) and `catalog/` (53).
A sweep of either alone is a clean wrong answer.

**Only ```` ```ken ```` blocks are compiled.** `crates/ken-elaborator/src/
literate.rs:345` classifies fence openers: bare `ken` is `Source`; `ken ignore`,
`ken reject`, and `ken example` are not. Prose is blanked out by
`extract_ken_md` before the parser sees it (`modules.rs:700-712`), so a
whole-file grep counts English sentences as catalog sites.

## By divergence class

    class                                    candidate sites   distribution
    ungrouped `if` as an application arg              0        none
    ungrouped PROJECTION as an application arg       16        15 in one file, 1 elsewhere
    bare ungrouped operator_name as an atom           ?        instrument produced nothing

**The `if` row costs nothing to migrate.** Zero catalog sites, in either root,
in any `ken` fence. It is a parser fix with no migration tail — which makes it
the cheapest of the three and separable from the rest if anyone wants it early.

**The projection row is the whole migration, and it is one file.**

    catalog/packages/Core/Classes/LawfulClasses.ken.md      15
    catalog/packages/Capability/Parsing/Parsing.ken.md       1

Two confirmed in context, so the class is real and not a regex artifact:

    LawfulClasses.ken.md:753   compare_raw a d.leq x y
    Parsing.ken.md:90          bytes_nat_length s.source_bytes_field

Both are exactly `keep box.value`. Under `32 §3` each re-parses as a projection
of the application — `Proj(A(compare_raw, a, d), leq)` — and must become
`(d.leq)` / `(s.source_bytes_field)`.

## What this census does NOT establish, stated because the numbers look tidy

**16 is a candidate set from a source-level scan, not the population.** An
earlier Steward scan of the same class returned **24**. Both are correct about
their own extraction: the two differ in whether the head, base, and field are
required to be lowercase, and in comment stripping. **Do not quote either as
the migration count.**

⇒ **The authoritative instrument is the AST walk the language-implementer
already built** — `EApp(head, EProj(base, field))` over the parsed catalog,
with `Expr::EProj` at `crates/ken-elaborator/src/ast.rs:650`. It decides
`d.leq`-as-projection against `Module.name`-as-qualified-path, which no regex
can. **That walk is this node's D0, and its result is the migration ledger.**

**What both scans agree on is the only thing the sizing needs:** the population
is of order twenty and **more than 90% of it is in one file.**

**The operator-name row is UNMEASURED and my probe is not evidence it is
empty.** A source scan cannot separate `x + y` (ordinary infix, fine) from an
operator passed bare as an argument (the defect). My probe returned zero
matches of any shape inside `ken` fences, which is a reading on the probe, not
a measurement of the catalog. The AST walk covers this row too; until it runs,
treat this row's extent as unknown rather than as zero.

# Sizing / tier

**Size L, tier T1. The L now stands over a measured quantity, and the answer is
that this node does NOT decompose on size.**

The catalog migration is ~16-24 mechanical grouping edits, 15 of them in
`LawfulClasses.ken.md`. That is not a second deliverable; it is the tail of the
first. **Splitting parser conformance from catalog migration would land a
parser that the catalog cannot compile against** — the A0 checkpoint measured
exactly that (`priority_queue_actual_export_table_is_exactly_the_six_name_api`
failed candidate-only, and restoring the legacy parse returned it to 1/1). The
two are coupled by construction, and the coupling is the reason the contraction
was out of scope for A0.

⇒ **One node, one candidate.** If it decomposes at all it decomposes by ROW, not
by parser-versus-catalog: the `if` row has zero migration tail and could ship
alone. That is an option for the framer, not a recommendation — three rows in
one diff against one pin reviews better than three candidates against the same
pin.

**What the frame still owes**, and none of it is blocked: the D0 ledger from the
AST walk, the operator-name row's extent, and whether the re-gated
`seed-reserved-infix-names.md` rows go green as a consequence or need their own
act.

# Contention

`crates/ken-elaborator/src/parser.rs` and catalog source. Overlaps A0's file
set, so it must not run concurrently with a live A0 candidate -- check
[[LANG-RESERVED-INFIX-NAMES]]'s node status at `origin/main`, not for a branch
ref, before releasing this.
