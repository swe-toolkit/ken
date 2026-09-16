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

> ## CORRECTED after landing — read §2d, §2e, §2f and §4 before starting
>
> Architect ruling `evt_3n1q5324gsqjv` closed §4's open question **and reversed
> the patch shape this frame implied.** Four changes, all folded here so the
> frame is the durable record:
>
> 1. **§4 is RULED, not open.** Candidate A, and it is *entailed* by
>    `32-grammar.md:264`/`:270` rather than chosen — candidate B has no
>    derivation. **No Spec escalation is owed.**
> 2. **§2d's "relocation of the loop" was the Steward's and was wrong.** The
>    head position already conforms on `main`; only the argument position is
>    defective.
> 3. **§2e is new and is the implementable shape** — one loop, two alternatives.
>    The relocation §2d implied regresses `box.value keep` and `f a.b c` from
>    correct to **unparseable**.
> 4. **§2f: do not sweep the third caller** of `parse_atom_expr` (`:2987`,
>    `old`). New AC-OLD-UNCHANGED makes that checkable.
>
> The Architect reviewed the D0 walk plan and would not change it. Nothing here
> blocks D0; it bears on the parser edit that follows it.

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

### 2d. The projection divergence is in the ARGUMENT position ONLY

> **CORRECTED, and the withdrawn sentence was the Steward's.** This section used
> to end *"This is a relocation of the loop, not a deletion of it."* The
> Architect measured that reading and it is **wrong in a way that breaks working
> code** — see §2e. Relocating the postfix loop to fire after the whole
> `head args*` run regresses two expressions that parse correctly today. The
> corrected shape is §2e; do not implement this section without it.

    crates/ken-elaborator/src/parser.rs:2813   fn parse_atom_expr
                                               = parse_atom_expr_base, then a
                                                 postfix `.field` / `.1` / `.2`
                                                 loop building EProj/EPosProj
    crates/ken-elaborator/src/parser.rs:2842   fn parse_atom_expr_base
                                               the atom proper, no projection

`parse_app_expr` calls `parse_atom_expr` for **both** the head (`:2291`) and each
argument (`:2314`). The two positions behave differently, and only one of them
is defective:

    box.value keep   TODAY   head -> EProj(box, value); the arg loop then takes
                             `keep` -> EApp(EProj(box,value), keep)   CORRECT
    keep box.value   TODAY   head -> keep; the argument absorbs its own `.field`
                             -> EApp(keep, EProj(box,value))    VIOLATES THE PIN

⇒ **The head position already conforms on `main`. The defect is confined to the
argument position**, where an argument absorbs a `.field` that the pin says
belongs to the application spine.

### 2e. The correct shape: ONE loop, TWO alternatives — not a relocated loop

Architect ruling `evt_3n1q5324gsqjv`, measured at `origin/main`
`7bac1de18356901f207e5585f4ac55ad8c8f8cc0` and re-verified by the Steward at
`d7596ac68301edbf9ec190c97408cd908aed1fc6` (no parser change between them).

**Why the obvious patch is wrong.** The tempting site is
`atom (atom)* ("." field)*` — postfix after the argument run. Traced:

    box.value keep   head `box`; the arg loop tests can_start_atom_expr on
                     Token::Dot -> FALSE, so the arg run is EMPTY; postfix then
                     gives EProj(box,value) and `keep` is never consumed
    f a.b c          head `f`, arg `a`; `.` ends the arg run; postfix gives
                     EProj(EApp(f,a),b) and `c` falls out the same way

A trailing argument after a projection has no site left to be consumed at. That
patch fixes `keep box.value` and **regresses `box.value keep` from correct to
unparseable** — the very expression §4's question was about.

**The shape that works.** Application and projection are the same
left-associative postfix loop at the same level, so both arms fold into one
accumulator and source order is preserved by construction:

```rust
// parse_app_expr, the `_` arm at :2290 — the head now takes the BASE
let mut f = self.parse_atom_expr_base()?;
loop {
    // projection: same spine, same level. Guard copied verbatim from :2815-2816.
    if matches!(self.peek(), Token::Dot)
        && matches!(self.lookahead(1), Token::Ident(_) | Token::Nat(1 | 2))
    {
        f = self.parse_projection_suffix(f)?;   // body lifted from :2817-2838
        continue;
    }
    if self.is_contextual_ident("eqn") && matches!(self.lookahead(1), Token::Colon) { break; }
    if self.is_contextual_ident("visits") && matches!(self.lookahead(1), Token::LBracket) { break; }
    if self.brace_starts_match_arms() { break; }
    if !self.can_start_atom_expr() { break; }
    let arg = self.parse_atom_expr_base()?;     // :2314 — BASE, so `.` never rides an argument
    let span = Span::merge(f.span(), arg.span());
    f = Expr::EApp(Box::new(f), Box::new(arg), span);
}
Ok(f)
```

Three edits: `:2291` and `:2314` move to `parse_atom_expr_base`, and the dot arm
is added at the top of the loop. **Lift `:2817-2838` verbatim** into
`parse_projection_suffix(&mut self, e: Expr)` rather than re-deriving it — the
`.1`/`.2` arm with its `Nat(1 | 2)` guard and the
`Span::new(e.span().start, projection_span.end)` construction must come along,
or positional projection silently leaves the spine.

**`can_start_atom_expr` stays unchanged, and `Token::Dot` must remain a
non-member.** The dot arm is tested *before* the break guards, so the loop
continues on `.` without Dot ever needing to look like an atom start.

### 2f. The third consumer of `parse_atom_expr` — do NOT sweep it

`parse_atom_expr` has exactly three call sites, verified at `d7596ac68`:

    :2291   parse_app_expr, head       -> becomes parse_atom_expr_base
    :2314   parse_app_expr, argument   -> becomes parse_atom_expr_base
    :2987   `old` (Token::KwOld, spec 21 §6.4)   -> UNCHANGED

After the two edits `parse_atom_expr` has one caller left, and the tempting
cleanup is to inline or delete it. **Do not.** `old x.f` is `EOld(EProj(x,f))`
today; whether it should be `EProj(EOld(x),f)` is a different question that
`32 §3`'s pin does not reach, because `old` is not an `application_atom`.

⇒ **Leave `:2813` byte-identical** so the diff cannot move `old`'s binding, and
let the one-caller smell stand. If that reading wants settling it is a Spec
question on `21 §6.4` and its own node — the Architect has flagged it to
spec-leader — not a rider on this one.

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

## 4. RULED — projection at the HEAD position. Closed; no Spec action owed.

This section was open when the frame landed. It is now **ruled**, and the ruling
is folded here because an in-thread ruling is not a durable deliverable and the
frame is the artifact the implementer meets first.

**Architect `evt_3n1q5324gsqjv`: candidate A — `box.value keep` is
`A(Proj(box, value), keep)`.** And it is **entailed by the pin, not chosen over
an alternative**, so there is no spec gap and nothing to escalate to Spec.

**The derivation.** Both forms are left-recursive postfix productions over
`expr`, `spec/30-surface/32-grammar.md:264` and `:270`:

    | expr application_atom                      -- application (left assoc)
    | expr "." ident | expr ".1" | expr ".2"     -- field / projection

`box.value keep` derives as `expr application_atom` with `expr = box.value`,
itself `expr "." ident`.

⇒ **Candidate B was never the losing side of a fork — it has no derivation at
all.** The right operand of `.` is the terminal `ident`, not an `expr`, so
`Proj(box, A(value, keep))` is unexpressible. A fork with one well-formed member
is not a fork, which is why this could not have been the real question.

`application_atom` (`:278-288`) does not list projection, so `.field` is never an
argument and is always a suffix on the spine. **One rule yields both facts:
application and projection are the same left-associative postfix loop at the
same level, and source order alone decides.**

    keep box.value   =  Proj(A(keep, box), value)
    box.value keep   =  A(Proj(box, value), keep)

**What this cost.** The question was worth asking and the answer was cheap; what
it bought was §2e, which is not cheap. The frame's original instruction was not
to infer the reading from the easiest patch. **The inference that actually
slipped was the converse — that the easiest patch implements the reading.**

## 5. Acceptance

**AC-IF-REJECTS. An ungrouped `if` after an application head rejects at its
leading token.** `keep if c then a else b` is a parse error whose span is the
`if`. **Control:** `keep (if c then a else b)` still parses, and a leading
`if c then a else b` still parses. A candidate that makes `if` unparseable
everywhere has satisfied the first half and broken the language.

**AC-PROJ-NESTS. THREE separate assertions, and they must be separate
criteria.** Asserted on the AST, not on a round-tripped string — a printer that
re-emits the same source cannot distinguish the two trees, which is the whole
defect.

    keep box.value   ->  EProj(EApp(keep, box), value)     the pin's own case
    box.value keep   ->  EApp(EProj(box, value), keep)     CORRECT TODAY
    f a.b c          ->  EApp(EProj(EApp(f, a), b), c)     the interleaving case

**Why all three, and why not one criterion.** The second parses correctly on
`main` already, so it is a **regression guard, not a new behaviour** — and §2e
records a patch shape that satisfies the first while breaking the second. A
suite carrying only the pin's own example cannot fail for that. The third is the
case no single-alternation grammar reaches: it needs the one-loop-two-alternatives
shape, and it discriminates that shape from any sequential arrangement of the two
loops.

**Control:** `(keep box).value` produces the same tree as the first row, and
`keep (box.value)` produces the legacy one. Both must be stated, because a
candidate that collapses all of them has removed the distinction rather than
fixed it.

**AC-OLD-UNCHANGED. `old x.f` still parses as `EOld(EProj(x, f))`, and
`parser.rs:2813` `fn parse_atom_expr` is byte-identical in the diff.** §2f: the
third caller at `:2987` is out of scope, and leaving the function untouched is
what makes that checkable rather than argued. A candidate that inlines or
deletes `parse_atom_expr` has moved `old`'s binding as a side effect of a
cleanup.

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
