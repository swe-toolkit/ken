---
id: LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE
title: "RE-CUT to the `if` half alone: make an ungrouped `if` after an application head reject AFFIRMATIVELY at the `if`, raised by the argument loop itself rather than by whatever parses next. The projection half is GONE -- the operator rejected the contraction (PR #3792 closed) and the 32 §3 amendment retires the pin it existed to enforce, so the parser's existing grouped reading is now the specified one and there is NO catalog migration. Scope is one continuation loop of five: removing a token from a can_start_* predicate makes the loop BREAK, never reject, so this needs re-implementing rather than re-basing."
status: merged
owner: language
size: S
gate: none
depends_on: [SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16 at the Architect's naming, on their D0 REVERSAL (evt_5sf71fnjxpzmb), which vacates the earlier WITHDRAW ruling (evt_64avxs9ashqqk). The divergences were measured by spec-author under the SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION hard stop (evt_7p78v6qb429rz) and read against the spec by the Architect. Steward verified the two quoted spec passages verbatim at origin/main e11341c7b9d11cd74879d27d555d2a5729837847: 32-grammar.md :357-363 (ungrouped operator_name is not a general application_atom, 'applies equally to generic and reserved operator names') and :368-378 (the five leading forms reject; arrow and projection nest, as `(keep Nat) -> Nat` and `(keep box).value`; 'part of §3's contract pin; an implementation must not restore a second unrestricted application production'). The catalog-source extent is UNMEASURED -- see Sizing. Not yet framed; the Steward frames and releases it when the language lane reaches it."
---

> ## MERGED 2026-09-16 at `9a1b8247233b1273487ff22833a985a405fab138`
>
> **Verified by blob, not by ancestry** — the publisher squashes, so a routed
> commit is an ancestor of nothing. All three touched paths
> (`crates/ken-elaborator/src/parser.rs`,
> `crates/ken-elaborator/tests/lang_application_atom_if_rejection.rs`,
> `crates/ken-elaborator/tests/lang_surface_if.rs`) are byte-identical
> between the approved candidate `30f3e19048b7dc5571e30b6e413c19fdb71e9cdc`
> and `main`, landed via PR #3800 with full-mode CI green. Decision
> `dec_1q433nvrj88xc` resolved; Architect approved exact; QA approved.
> This is the if-half re-cut after the projection half was rejected by the
> operator (PR #3792 closed) — see the title for the scope narrowing.

> # READY. Frame:
> `docs/program/wp/LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE.md`.
>
> This node exists because [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]]
> re-gates two row groups of `seed-reserved-infix-names.md` onto a named
> successor, and a `RED-UNTIL-<node>` tag naming a node that does not exist is
> not a gate.
>
> **SUPERSEDED IN PART — read the RE-CUT banner immediately below this one
> first.** The sizing, the migration, and every projection-specific ruling in
> this banner were written for a two-row scope that no longer exists. What
> survives from it is only the precondition check:
>
> **Both release preconditions cleared at `a631e4fb2`, measured not assumed.**
> [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]] is `status: merged`, and
> [[LANG-RESERVED-INFIX-NAMES]]'s A0 landed blob-identical with its `wp/` branch
> deleted at origin — so no live candidate holds
> `crates/ken-elaborator/src/parser.rs`. **Re-measure before releasing** — that
> reading is from `a631e4fb2` and `main` has moved since.
>
> **RETIRED with the projection half:** the head-position ruling
> (`evt_3n1q5324gsqjv`), the one-loop-two-alternatives patch shape, and the
> instruction to read frame §2d-§2f and §4 before touching the parser. Those
> sections describe how to build the contraction the operator rejected. **They
> are not instructions for this node, and following them would rebuild the
> closed candidate.**

> # RE-CUT 2026-09-16 (Steward). THE PROJECTION HALF IS RETIRED; THE `if` HALF
> # SURVIVES AND NEEDS RE-IMPLEMENTING, NOT RE-BASING.
>
> **What changed.** The operator closed PR #3792 and rejected the projection
> contraction as bad language design. The Architect ruled ADOPT on amending
> `32 §3` instead, and the amendment drops projection from the must-be-grouped
> set. So the parser's **existing** grouped reading (`keep box.value` is
> `keep (box.value)`) is now the **specified** reading, and this node's
> projection objective is not merely descoped -- it is **inverted**. There is no
> catalog migration; the 30 paren-only lines that candidate `7c6dfdfaf` added
> were already right before it touched them.
>
> **`dec_wac7adfhr23c` is VOID.** It approved the contraction. Do not cite it,
> and do not respin `7c6dfdfaf`.
>
> **The `if` half never depended on any of that.** Measured at `origin/main`
> `a63eebe3e7582c71a2c59624786a39705cf9a0e0`, `32-grammar.md:373` already says
> the five leading forms reject at their leading token, and the amendment
> preserves that sentence with only `and projection` struck from its second
> clause. **The requirement is byte-stable across the amendment**, so this node
> is releasable now and is not gated on the spec candidate landing.
>
> **Why re-implement rather than re-base.** `7c6dfdfaf` discharged the `if` half
> by deleting `Token::KwIf` from `can_start_atom_expr`. The argument loop reads
> `if !self.can_start_atom_expr() { break; }` -- **a continuation predicate can
> only STOP; it can never REJECT.** The deletion makes the loop break, leaves
> `if c then a else b` unconsumed, and the error is raised by the file-level
> declaration parser hitting a stray token. Removal is necessary and cannot be
> sufficient, so the edit is the wrong SHAPE for the requirement rather than an
> incomplete version of the right one.
>
> **The candidate's own test could not have caught this**, which is why it went
> 9/9 green while the untouched `lang_surface_if` went red: it asserts
> `span.start` equals the offset of `if`, and **a stray-token error is reported
> at the stray token**, so the assertion passes under both the correct and the
> defective implementation. A span is a coordinate; the claim is about
> authorship. **The test must assert which production raised the error.**

# Objective

Make an ungrouped `if` after an application head **reject affirmatively at the
`if`, from the argument loop itself**. That is the whole node.

`spec/30-surface/32-grammar.md:373` on `origin/main` pins it: the five leading
forms -- lambda and `let`, `if`, `match`, and `temporal` -- reject at their
leading token when ungrouped after an application head. Four of the five already
conform. This closes the fifth.

# The divergence (measured, and RE-MEASURED after the recut)

Measured by spec-author against the base parser; read against the spec by the
Architect. The table as originally written found two divergences. **One of them
has since been resolved in the parser's favour by amending the spec**, so only
the `if` row remains live:

| row | `32 §3` requires | parser today | verdict |
|---|---|---|---|
| lambda / `let` / `match` | reject ungrouped | rejects | CONFORMS |
| `if` | reject ungrouped | ACCEPTS `A(keep, If(...))` | **DIVERGES -- this node** |
| arrow | `(keep Nat) -> Nat` | same | CONFORMS |
| projection | *(amended)* `keep (box.value)` | `A(keep, Proj(box, value))` | **CONFORMS -- was the pin that moved** |

The bare-operator-value group is pinned by the same section at `:357-363`, for
generic and reserved names alike -- an ungrouped `operator_name` is admitted
only as the head of `operator_prefix`, so `(OP)` grouping is required to use it
as an ordinary atom. **That group is unaffected by the amendment and is NOT this
node** -- see [[LANG-BARE-OPERATOR-ATOM-REJECTION]].

# There is NO catalog migration. This section previously argued the opposite.

The original argument ran: the A0 checkpoint's catalog control failed with
`TypeMismatch ... projection base's type is not a named-field owner`, therefore
existing catalog source is written against a non-conforming parse, therefore
conforming the parser necessarily moves the catalog with it.

**Every step of that was sound and its premise is now false.** The parse the
catalog was written against is the parse `32 §3` now specifies, so the catalog
source is conforming source and there is nothing to migrate. The 30 paren-only
lines candidate `7c6dfdfaf` added to it were correct before it touched them.

**The migration cost was also never bounded at 30 lines, and this is worth
keeping even though the migration is cancelled.** Each added paren pair is an
independent width decision for the formatter, so re-canonicalising churns
surrounding layout that carries no parens at all: in the one measured instance a
single added pair broke one line into six. A token-level count of a paren
migration is a floor, not a measure.

# Deliverables (indicative -- this node is not framed)

- Parser conformance for the `if` row: an ungrouped `if` after an application
  head rejects **affirmatively at its leading token, raised by the argument
  loop**, not by whatever production runs into the leftover tokens.
- A test that discriminates on **which production raised the error**, not on
  where the error landed.
- A closure argument over the other four continuation loops: they are
  byte-unchanged, and the change cannot reach them.

# Not this node

- Any change to `32 §3` itself. The pin is the authority this node answers to;
  narrowing it would remove the grounding rather than satisfy it. (The
  projection half of the pin is being amended, by the spec enclave, on the
  operator's ruling -- that is a different act by a different ring.)
- **The projection row.** Retired; the parser already conforms to the amended
  pin.
- **Any catalog migration.** There is none. A catalog edit would owe a formatter
  pass -- the catalog is gated on being a formatter fixed point, and byte-
  splicing parens into it at AST offsets reddens `ken_fmt`'s frozen-corpus gate
  and `kenfmt_c_capstone` in two different crates. This node does not touch it.
- The six-name admission of [[LANG-RESERVED-INFIX-NAMES]] (A0), which is
  released and bounded to unchanged syntax.
- The bare-operator-value row (`:357-363`) -- see
  [[LANG-BARE-OPERATOR-ATOM-REJECTION]].
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

> **RE-CUT NOTE.** That last clause is the one that survived. The census was run
> to size a migration that no longer exists, and the row it priced at zero is
> now the only row in the node. **The rest of this section is retained as
> instrument history, not as scope** — in particular the two-scans-disagree
> finding below, which is the reusable part.
>
> **The census also missed a population, and that IS scope.** It enumerated
> `.ken` / `.ken.md` sources. **Inline Ken inside Rust test files is a
> first-class population and is structurally invisible to that sweep** — four
> such files were found by four separate CI failures and never by a census,
> including `lang_surface_if.rs:118`, the fixture this node now turns on. Any
> census this node runs must cover inline Ken in `crates/**/tests/`.

**The projection row WAS the whole migration, and it is now cancelled.** Kept
because the distribution is the evidence for the paragraph above:

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
with `Expr::EProj` at `crates/ken-elaborator/src/ast.rs:736` (this node
originally cited `:650`; the A0 landing shifted it — re-measure before use). It
decides
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

**Size S, tier T1.** Was `M` over a coupled parser-plus-catalog scope; the
catalog half is cancelled and the node is now one parser edit, one test, and a
closure argument.

**S because the diff is small. T1 because the review turns on an argument, not
on the diff.** The defect this node replaces was a one-token deletion that
looked exactly like the right edit, passed its own purpose-built test, and was
approved EXACT by the Architect. What catches the correct version is reasoning
about what a continuation predicate can and cannot do — which is not work a
mechanical tier does reliably. **Do not read the small diff as a cheap seat.**

⇒ **One node, one candidate.** It no longer decomposes.

**What the frame owes:** the affirmative-rejection shape, a test keyed on error
identity, the five-loop closure with its negative controls, and an inline-Ken
census that reaches `crates/**/tests/`.

# Contention

`crates/ken-elaborator/src/parser.rs` only -- the catalog is out of scope, so
this node no longer contends for `catalog/` or `library/`. Still overlaps A0's
file set, so it must not run concurrently with a live A0 candidate -- check
[[LANG-RESERVED-INFIX-NAMES]]'s node status at `origin/main`, not for a branch
ref, before releasing this.
