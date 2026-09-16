# WP frame — `LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE`

**Owner:** Team Language · **Size:** M · **Risk:** medium (one parser predicate
and one postfix loop, against a catalog that compiles today) · **Tier:** T1 ·
**Gate:** none ·
**Deps:** [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]] — **MERGED**, verified
by node status at `origin/main`

**Origin:** the Architect's D0 reversal (`evt_5sf71fnjxpzmb`), which vacated the
earlier withdraw ruling (`evt_64avxs9ashqqk`). The divergences were measured by
spec-author under the `SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION` hard stop
(`evt_7p78v6qb429rz`) and read against the spec by the Architect.

**Size moves L to M, on the census.** The `L` was a placeholder over an
unmeasured catalog migration. It is measured (§3): the migration is of order
twenty grouping edits with more than 90% of them in one file, and the `if` row
has no migration tail at all.

## 1. Objective

Make the parser conform to the application-atom contract pin in
`spec/30-surface/32-grammar.md §3`, and migrate the catalog source written
against the current non-conforming parse.

**The pin is on `main` and is not in question.** This node does not decide
whether the contraction is wanted — the spec decided that. It closes the gap
between the spec and the implementation. Narrowing the pin is not a way to
satisfy this node; it removes the grounding instead.

## 2. Fixed inputs

Measured at `origin/main` `a631e4fb27351074bb7d4fcbc719c2fb6673df69`.

**Re-measure any coordinate before you use it.** `Expr::EProj` was cited as
`ast.rs:650` in the source material for this node and is **`ast.rs:736`** at
this SHA. The A0 landing shifted it. The same applies to every line number
below if this node is picked up after another parser change lands.

### 2a. The spec pin, verbatim

`spec/30-surface/32-grammar.md`, the application-arm paragraph:

> Both application arms — `expr application_atom` and `operator_prefix` with its
> `application_atom+` tail — intentionally restrict bare arguments to atoms. An
> expression not admitted by `application_atom` — including an ungrouped lambda,
> `let`, `if`, `match`, temporal form, arrow, or projection — must be grouped
> before it is used as an application argument. Consequently the five leading
> forms — lambda and `let`, `if`, `match`, and `temporal` — reject at their
> leading token when ungrouped after an application head, whereas arrow and
> projection remain well-formed with that application nested in their left
> `expr`, as `(keep Nat) -> Nat` and `(keep box).value` respectively. This
> removes the former ambiguous bare `expr expr` shape and is part of §3's
> contract pin; an implementation must not restore a second unrestricted
> application production.

And the bare-operator paragraph:

> An ungrouped `operator_name` is not a general `application_atom`. It is
> admitted only as the head of `operator_prefix`, with at least one following
> atom; grouping `(operator_name)` makes it an ordinary atom that may appear
> anywhere. [...] This restriction applies equally to generic and reserved
> operator names; it does not reclassify `+`, `*`, `==`, or any generic symbolic
> run as a bare general atom.

### 2b. The four rows, and which two diverge

| row | `32 §3` requires | parser today | verdict |
|---|---|---|---|
| lambda / `let` / `match` | reject ungrouped | rejects | CONFORMS |
| `if` | reject ungrouped | ACCEPTS `A(keep, If(...))` | **DIVERGES** |
| arrow | `(keep Nat) -> Nat` | same | CONFORMS |
| projection | `A(keep, Proj(box, value))` is wrong | as stated | **DIVERGES** |

The bare-operator row is a fifth question and its extent is **unmeasured** —
see §3c. Treat it as open, not as clean.

### 2c. The `if` divergence is ONE ENTRY IN ONE PREDICATE

    crates/ken-elaborator/src/parser.rs:2350   fn can_start_atom_expr

That predicate is what `parse_app_expr`'s argument loop consults before taking
another argument (`:2313`, `if !self.can_start_atom_expr() { break; }`), and its
`matches!` list contains **`Token::KwIf`**. The other four leading forms are
absent from it, which is exactly why they already conform.

**The head position is a different site and must not change.**
`parse_app_expr` (`:2284`) dispatches `Token::KwIf => self.parse_if_expr()` in
its leading `match`, before any argument loop runs. A leading `if` is
well-formed and stays so. **The pin is about the argument position only.**

⇒ **Check every other caller of `can_start_atom_expr` before editing it.** It is
a shared predicate; the AC is about the argument position, and a caller that
uses it to mean something else would be changed as a side effect.

### 2d. The projection divergence is WHERE the postfix loop lives

    crates/ken-elaborator/src/parser.rs:2813   fn parse_atom_expr
                                               = parse_atom_expr_base, then a
                                                 postfix `.field` / `.1` / `.2`
                                                 loop building EProj/EPosProj
    crates/ken-elaborator/src/parser.rs:2842   fn parse_atom_expr_base
                                               the atom proper, no projection

`parse_app_expr` calls `parse_atom_expr` for **both** the head and each
argument. So an argument absorbs its own `.field` and yields
`A(keep, Proj(box, value))`. The pin wants the application built first and the
projection applied to it: `Proj(A(keep, box), value)`.

**This is a relocation of the loop, not a deletion of it.** Projection stays;
what changes is the expression it attaches to.

## 3. D0 — the migration ledger. Run it FIRST; it is small.

The Steward's census (below) is a **candidate set**, sized well enough to frame
against and not precise enough to migrate against.

### 3a. What is already established

    class                                   candidate sites   distribution
    ungrouped `if` as an application arg              0        none, either root
    ungrouped PROJECTION as an application arg       16        15 in one file, 1 elsewhere
    bare ungrouped operator_name as an atom           ?        unmeasured

    catalog/packages/Core/Classes/LawfulClasses.ken.md      15
    catalog/packages/Capability/Parsing/Parsing.ken.md       1

Two confirmed in context, so the class is not a regex artifact:

    LawfulClasses.ken.md:753   compare_raw a d.leq x y
    Parsing.ken.md:90          bytes_nat_length s.source_bytes_field

Both are exactly `keep box.value`, and each becomes `(d.leq)` /
`(s.source_bytes_field)`.

**Method, because the number is meaningless without it.** Catalog source lives
under **two** roots — `library/` (4 files) and `catalog/` (53) — and a sweep of
either alone is a clean wrong answer. Only ```` ```ken ```` blocks are compiled:
`literate.rs:345` classifies the openers (`ken`, `ken ignore`, `ken reject`,
`ken example`) and `modules.rs:700-712` blanks prose before the parser sees it,
so a whole-file grep counts English sentences as catalog sites.

### 3b. Why 16 is NOT the number to migrate against

An earlier scan of the same class returned **24**. Both are correct about their
own extraction and differ in case requirements and comment stripping. **Neither
is the migration count**, and a candidate that discharges against either has
discharged against an artifact of its instrument.

⇒ **D0 is the AST walk, and language-implementer has already built it:**
`EApp(head, EProj(base, field))` over the parsed catalog, with `Expr::EProj` at
`crates/ken-elaborator/src/ast.rs:736`. It separates `d.leq`-as-projection from
`Module.name`-as-qualified-path, which no source-level pattern can. Its output
is the ledger every later AC discharges against.

**What both scans agree on is all the sizing needed:** order twenty, more than
90% in one file. That is why this is M and why it does not decompose.

### 3c. The operator-name row is UNMEASURED

**A probe returning nothing is not a measurement.**

A source scan cannot separate `x + y` (ordinary infix, fine) from an operator
passed bare as an argument (the defect). The Steward's probe returned zero
matches of any shape inside `ken` fences — **a reading on the probe, not a
measurement of the catalog.**

**Do not record this row as zero.** The same AST walk covers it: an operator
name occurring as an `application_atom` outside `operator_prefix` head position.
If D0 returns zero for it, that is a result; the current absence of a number
is not.

## 4. Open, and NOT for the implementer to settle

**Projection at the HEAD position.** The pin fixes `keep box.value` as
`Proj(A(keep, box), value)`. It does not say what `box.value keep` is —
`A(Proj(box, value), keep)` (projection binds tighter at the head) or
`Proj(box, A(value, keep))` (which is not well-formed). The natural reading is
the first, and the natural implementation of §2d produces it, but **the pin does
not state it.**

⇒ **Route this to the Architect as a spec-reading question before writing the
postfix relocation**, not after. If the answer is the natural one, it costs a
sentence; if it is not, it changes where the loop goes. Do not infer it from
what the easiest patch happens to do.

## 5. Acceptance

**AC-IF-REJECTS. An ungrouped `if` after an application head rejects at its
leading token.** `keep if c then a else b` is a parse error whose span is the
`if`. **Control:** `keep (if c then a else b)` still parses, and a leading
`if c then a else b` still parses. A candidate that makes `if` unparseable
everywhere has satisfied the first half and broken the language.

**AC-PROJ-NESTS. `keep box.value` parses as `Proj(A(keep, box), value)`.**
Asserted on the AST, not on a round-tripped string — a printer that re-emits the
same source cannot distinguish the two trees, which is the whole defect.
**Control:** `(keep box).value` produces the same tree, and `keep (box.value)`
produces the old one. Both must be stated, because a candidate that collapses
all three has removed the distinction rather than fixed it.

**AC-MIGRATION-LEDGER. Every site the D0 walk returns is migrated, and the diff
touches no site outside it.** The ledger is the population; the AC is over named
sites, not over a count. A count holds under a delete-one-add-one substitution
and this AC exists to catch exactly that.

**AC-CATALOG-COMPILES. The catalog compiles and its controls pass, including
`modules::namespace_effect_tests::priority_queue_actual_export_table_is_exactly_the_six_name_api`.**
That control is the one that failed candidate-only at the A0 checkpoint with
`TypeMismatch ... projection base's type is not a named-field owner`, and
restoring the legacy parse returned it to 1/1. **It is the discriminator for
this whole node** — it goes red under a parser fix without the migration, and
green with both.

**AC-PIN-UNCHANGED. `spec/30-surface/32-grammar.md §3` is byte-unchanged.**
`git diff` over the file shows nothing. The pin is the authority this node
answers to; a candidate that edits it has changed the question.

**AC-SEED-ROWS. The `RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE` rows
of `seed-reserved-infix-names.md` are re-measured and their disposition
stated.** Whether they go green as a consequence or need their own act is
**not known** and is a deliverable of this node, not an assumption in it. State
which, with the measurement.

**AC-NO-REGRESSION. Workspace-green in CI**, never a local `--workspace` run.
Local verification is targeted only, through `scripts/ken-cargo`:
`-p ken-elaborator`.

## 6. Contention

`crates/ken-elaborator/src/parser.rs` and catalog source under **both** roots.

**Both release preconditions that gated this node are now clear**, measured at
`a631e4fb2`:

    SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION   status: merged
    LANG-RESERVED-INFIX-NAMES                     code LANDED (blob-identical),
                                                  wp/ branch deleted at origin

⇒ **No live A0 candidate holds `parser.rs`.** Re-check both at `origin/main` by
node status and by blob before release — never by branch ref, and never by
ancestry, since the publisher squashes.

## 7. What this node is NOT

- **Not a change to `32 §3`.** See AC-PIN-UNCHANGED.
- **Not the six-name admission of [[LANG-RESERVED-INFIX-NAMES]]**, which is
  landed and was bounded to unchanged syntax.
- **Not the temporal rows** (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`), a
  separate gate.
- **Not the lambda / `let` / `match` or arrow rows**, which conform today.
  Touching them is scope this node does not carry.
