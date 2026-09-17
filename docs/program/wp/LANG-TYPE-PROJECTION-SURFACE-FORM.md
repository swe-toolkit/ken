# WP frame — `LANG-TYPE-PROJECTION-SURFACE-FORM`

Owner: **language**. Size **M**. Model-capability tier **T1** (`steward.md §4h`)
— the diff is small, but it turns on a resolution argument (which projection
index a field name denotes, and why the telescope is closed) and on one
criterion whose violation no test on the dependent node would catch. Not a
mechanical port.

## 1. Objective

Ken's surface type grammar has no projection form. A parameter typed by a
projection from an **earlier parameter in the same telescope** —
`(d : Membership c) (q : d.Query)` — is unspellable in a `.ken.md` declaration,
even though the kernel admits the telescope and the spec can state it.

Add a projection form to the surface type grammar and resolve it to the
kernel's existing `Term::Proj1` / `Term::Proj2`.

**This is a surface-to-kernel resolution gap, not a new kernel capability.** No
kernel change is in scope. `trusted_base()` delta must be zero.

## 2. Fixed inputs, measured at `origin/main` `ac4b5957120c3da8b63409d161f4290cb72fcc45`

Re-measured for this frame rather than carried from the node, which was written
against `24e9ce039`.

### 2a. The gap is in TWO surface layers, not one

The node says "add a projection form to `RType`". That is the second of two:

    LAYER 1  crates/ken-elaborator/src/ast.rs    pub enum Type
             TPi TSigma TArr TEffectArr TUniv TCon TVar TRefine TApp TTrunc
             TEN variants, NO projection

    LAYER 2  crates/ken-elaborator/src/resolve.rs    pub enum RType
             RPi RSigma RArr REffectArr RUniv RCon RVarTy RPatternAliasTy
             RRefine RApp RTrunc
             ELEVEN variants, NO projection

    TARGET   crates/ken-kernel/src/term.rs    Term::Proj1, Term::Proj2
             plus smart constructors alongside them
             PRESENT, unchanged, not in scope

**Budget for both layers plus the parser production that builds layer 1.** A
frame that names only `RType` understates the work by a parser rule and an AST
variant.

### 2b. The positive control fires — this is a real absence, not a blind grep

    RType::RProj      0 hits across crates/
    RExpr::RProj      lives in FOUR files: classes.rs, elab.rs, modules.rs,
                      resolve.rs
    Expr::EProj       ast.rs, named-field form   `(Box<Expr>, String, Span)`
    Expr::EPosProj    ast.rs, positional form    `(Box<Expr>, u8, Span)`

**Projection exists in the EXPRESSION category only, in both the named and the
positional spelling.** The zero above comes from an instrument whose positive
control fires in four files, so it is a measurement rather than a miss.

#### CORRECTION, 2026-09-17 — the sentence above is right about the CATEGORY and silent on the FIELD-NAME CASE

Raised by `language-implementer` in the D0 answer (`evt_4ycst4cj7t395`) and
**re-measured by the Steward against `parser.rs` and `58b` rather than carried.**
The paragraph above frames the gap on one axis — expression category present,
type category absent — and a reader takes from it that the expression form is a
complete template for the type form. **It is not, and the way it falls short is
exactly where this node's consumers sit.**

The real surface is a 2x2. One fixture per cell:

    d.query    lowercase Ident field    expression: OK        type: ParseError at the dot
    d.Query    Uppercase ConId field    expression: REJECTS   type: ParseError at the dot
    d.1        positional               expression: OK        type: ParseError at the dot

**The expression-side named form is LOWERCASE-ONLY.** `parse_atom_expr`'s
postfix loop guards on `Token::Dot` with `lookahead(1)` in
`Token::Ident(_) | Token::Nat(1 | 2)`. `Query` lexes as a `ConId`, so `.Query`
is never consumed — in either category.

**And all three consumer bindings spell an uppercase field.** `58b §1` declares
`Query : Type`, so `membership_member_at`, `member_holds` and `same_members`
each carry `d.Query` verbatim (`58b:78`, `:136`, `:147`).

⇒ **The type-side production must admit `Ident | ConId` after the dot.** That is
a requirement of the consumer set, not a generalisation: a production copied
faithfully from the expression side would parse `d.query` and still fail on
every binding this node exists to serve. It is also the one place `2c`'s
`TTrunc` precedent does not reach, since `‖A‖` has no field-name position.

**Ambiguity cost: none, and the grammar already pays for it.** The HEAD's case
is the existing disambiguator — `33 §3.2` makes module components `conid`, so a
lowercase head cannot begin a module path. `parse_atom_type`'s `ConId` arm
routes to `parse_dotted`; its `Ident` arm consumes nothing after the identifier.
The discriminating pair, measured:

    (q : Bag.Query)   UnresolvedCon { name: "Bag.Query" }      parsed as a dotted name,
                                                               failed at RESOLUTION
    (q : d.query)     ParseError "expected RParen, found Dot"  never PARSED at all

Attaching the projection production to the lowercase-`Ident` head therefore
collides with nothing already in the grammar.

#### The asymmetry is ONE BUILT HALF AND ONE UNBUILT HALF — do NOT record it as deliberate

**Architect, `evt_6eggjj1k2g314`, concurring with the scope line and correcting
its stated CAUSE.** Steward-verified at BASE rather than carried:

    parse_atom_expr_base   Ident arm    Ok(Expr::EVar(s, span))    consumes NO dots
    parse_atom_expr_base   ConId arm    self.parse_dotted(...)     eats its own dots
    parse_dotted           doc:59       "Only triggered from a ConId start"
                                        (admits Ident | ConId | operator after the dot)

⇒ In expression position `d.Query` is claimed by **no production at all**. It is
not reserved, not ambiguous, and not deferred to a competing reading — the guard
simply never admitted it.

**This matters because the two phrasings are opposite instructions to a future
reader.** *"Deliberate asymmetry"* says preserve this, there was a reason;
*"unbuilt half"* says close it when a consumer appears. Written the first way, a
later reader who wants `f d.Query` in expression position hunts for a reason that
does not exist and either invents one or re-asks a question already answered
here. Write it the second way:

> Type position admits `.ConId`; expression position does not — **because no
> consumer reached it and the widening was scoped out, not because a reading
> competes for that spelling.**

Widening the expression side stays **out of scope** (`§10`): no AC-1 binding
needs it (`d.member` is lowercase) and it is a separate reachable-surface change
with its own review. What changes is the record, not the code.

#### NEITHER GRAMMAR IS A SUBSET OF THE OTHER. Put this table where the productions are.

    spelling      expression      type (after this WP)
    d.query         OK              OK
    d.Query         rejected        OK
    d.1 / d.2       OK              rejected

The uppercase divergence is the half the D0 named; the **positional** divergence
is the other half and had not been said out loud. Not building positional in
type position is correct — reaching-consumer-per-form is the rule and it stands —
but the result is that **no containment holds in either direction**, so every
cell has to be memorised. This table is the only artifact that stops a later
"harmonization" from silently deleting a capability in one direction while adding
one in the other.

#### Two review expectations, stated now so they are cheap. NEITHER IS A NEW AC.

1. **The `Nat` arm in type position is not optional.** The moment the type
   production consumes `Token::Dot`, `(q : d.1)` stops failing at
   `expected RParen, found Dot` and enters it. That arm gets authored either way;
   the only choice is whether it carries a targeted message. Make it say
   positional projection is unavailable in type position — the generic error
   points at the paren, not at the unsupported form.
2. **Make "what may follow the dot in a projection" ONE named predicate called
   from both sites**, with the divergence expressed at its single definition and
   the expression caller keeping its `Nat(1 | 2)` arm explicitly. Two
   hand-written guards over one concept is the shape that drifts. Precedent, and
   it is why this is asked for rather than guessed at: in
   `LANG-BARE-OPERATOR-ATOM-REJECTION` the argument that closed the review was
   that `canonical_operator_name(...).is_some()` is **one predicate in four
   places** — that is what made argument-position unreachability provable rather
   than merely observed.

### 2c. THE WORKED PRECEDENT: `TTrunc` / `RTrunc` is this exact shape, already done

`‖A‖` was added as a **type-annotation-position sibling of an existing
expression form**, which is precisely what this node needs. Its own doc comment
says so:

> `‖A‖` — propositional-truncation formation in annotation position (`16 §6`).
> Resolved from `Type::TTrunc`; elaborates to `Term::Trunc`. The
> expression-position sibling is `RExpr::RTrunc`.

⇒ **Read the `TTrunc` → `RTrunc` → `Term::Trunc` path end to end before
designing anything.** It is the same three-layer traversal with the same shape
of obligation (parser production, AST variant, `RType` variant, resolution arm,
elaboration arm, span plumbing through `impl Type::span` and `impl
RType::span`), and it is already reviewed and landed. **Copy its structure, not
its semantics.**

This is design input, not a deliverable. If the path turns out to differ in a
way that matters, say so in the D0 answer — a refuted precedent is a finding
worth more than a silent divergence.

### 2d. Cite by SYMBOL, never by line

`crates/ken-elaborator/src/parser.rs` and `resolve.rs` move under active work —
the language ring landed in `parser.rs` twice tonight. Every coordinate in this
frame is a **symbol name**, and yours should be too. A line number in your
report will be stale before it is read.

## 3. The substantive design question, stated so it is answered rather than assumed

The surface spells a **field name** (`d.Query`). The kernel target is
**positional** (`Proj1` / `Proj2`). A class is a right-nested Σ (`33 §5.2`), so
a field at index `n` is a `Proj2^n` chain ending in a `Proj1`.

**So resolution must map a name to an index, and that map comes from the class's
field list.** Two things follow, and both are yours to settle in D0:

1. **Where the field list is read from at resolve time**, and whether the
   projected object's class is known there — `d`'s type is `Membership c`, a
   parameter, so the resolver must reach its class declaration.
2. **What happens when it is not known.** A projection off something whose class
   cannot be resolved is a rejection with a located span, not a panic and not a
   silent `Proj1`.

⇒ **Do not guess the arity or the field order from the spec text. Read the
class's own field list from the tree**, and say in your report which structure
you read it out of.

## 4. D0 — answer before writing the production

**D0-1. Does one `RType` variant reach all three consumers?** The node's
measurement says yes — both `RType::RPi` and `RExpr::RPi` annotate their domain
with an `RType`, so a single variant serves the two telescope bindings and the
`Π`-domain-in-a-body case. **Re-measure it; the node itself says the variant
list is perishable and the two-productions worry was live until the enum was
opened.** If it is now false, stop and report — that is a re-scope, not a
work-around.

**D0-2. Named form, positional form, or both?** `Expr` carries `EProj` (named)
and `EPosProj` (positional). The three consumer bindings all spell a name. **The
cheapest correct answer is the named form alone**, with the positional form left
unbuilt until something needs it. Take that unless D0-1's reading refutes it —
and if you add both, the frame requires a reaching consumer for each, not a
symmetry argument.

**D0-3. Is the field list reachable at resolve time (§3)?** If it is not, the
resolution may need to defer to elaboration, which changes the shape of the
work. Answer before building, not during.

**Hard stop, not a defect:** if D0-1 or D0-3 comes back the unfavourable way,
that is a genuine hard stop under `§4b` sizing and the right outcome for the
turn. Report the measurement; do not widen the node to absorb it.

## 5. Deliverables

1. A projection form in the surface type grammar: a parser production, a
   `Type` variant, and its `span()` arm.
2. An `RType` variant, its `span()` arm, and the resolution arm from the `Type`
   variant.
3. Elaboration to `Term::Proj1` / `Term::Proj2`, via the class's field list.
4. **Rejection with a located span** in two cases the node names: the projected
   object is not a record, and the field does not exist. A located span means a
   span that points at the projection, not at the enclosing declaration.
5. Whatever `33` / `34` surface-grammar text must say for the form to be
   **normative rather than an undocumented elaborator affordance**. If that text
   is enclave-owned, say so and file the need — do not author spec text under a
   language WP.

## 6. Acceptance

Each criterion names a command or a call site, never a property. Where a
criterion asserts an absence, its control is named with it — an absence reported
by an instrument that could not have produced a hit is not a measurement.

**AC-1 — the three consumer bindings are writable.** All three from
`SPEC-MEMBERSHIP-CLASS-CONTRACT` `58b §2` / `§4`, each in a fixture, each
checking:

    membership_member_at (q : d.Query) ... : Bool     parameter telescope
    member_holds         (q : d.Query) ... : Ω        parameter telescope
    same_members         ... : Ω, body (q : d.Query) -> ...   Π domain in body

**Exercise all three, not the telescope case alone.** The node's title names
only the first, and a fix serving it alone leaves two of three unwritable —
which is exactly the shape of gap this node exists to prevent downstream.

**THREE BINDINGS ARE TWO PARSE POSITIONS. Say so in the candidate.** Raised by
`language-implementer` against their own AC in the D0 answer
(`evt_4ycst4cj7t395`), and it is a correction to this clause rather than to the
work. `membership_member_at` and `member_holds` are both `Binder.ty` inside a
`params: Vec<Binder>` telescope — they differ in the decl keyword and the result
type, which is the **enclosure**, not the routing axis. Only `same_members` is
structurally distinct (`Expr::EPi` domain). Ship all three fixtures, because
they are the real consumer bindings and this clause names them; but a reviewer
who reads "three fixtures" as "three routes" is over-reading the coverage. The
count of fixtures is not the count of positions, and the gap between them is
where a coverage claim goes wrong.

**AC-2 — both rejections fire, with a span on the projection.** One fixture per
case (non-record object; absent field). Assert the span, not only the rejection:
a rejection with the enclosing declaration's span satisfies "rejects" and fails
the deliverable.

**AC-3 — THE ESCAPE HATCH IS CLOSED, and this is the criterion the Architect
said they cared about most** (`evt_4tmt0n7era4w3`):

> it must not be discharged by making `∈` an elaborator builtin: `33 §6.1`
> requires the standard meanings to be **ordinary top-level bindings**, and a
> builtin would break the precondition A-track's completion policy rests on
> (`39 §6.9`, and `SPEC-STANDARD-INFIX-BINDING §2f`).

**Why this needs a control rather than a promise.** Special-casing the operator
in the elaborator is the tempting way out of a missing surface form, it would
make every dependent test pass, and **no test on the dependent node would catch
it** — the observable behaviour is identical. The criterion must therefore be a
measurement on **how** the bindings resolve, not on whether they work:

- **State the command you used** and its output, in your report. A criterion
  satisfied by "I did not do that" is satisfied by an implementer who did it
  without noticing.
- **The sound direction is positive:** show the three bindings resolving as
  ordinary top-level bindings through the same path any other top-level binding
  takes — not that a builtin-shaped grep came back empty. A name-census over
  the elaborator is the wrong instrument here for the same reason it was wrong
  on the runtime candidate this week: it answers "is the string present", and
  the defect is a resolution path.
- **Name your positive control**: an existing ordinary top-level binding that
  your check reports as ordinary. If the check cannot distinguish that from a
  builtin, it is not a check.

**AC-4 — zero `trusted_base()` delta.** No kernel change is in scope;
`Term::Proj1` / `Proj2` are the target, unchanged.

**AC-5 — no regression.** Green **in CI**, not from a local `--workspace` run,
which is prohibited (`COORDINATION §12`). Locally, build and test only what you
touched: `scripts/ken-cargo test -p ken-elaborator`, plus the named fixture
suite. The box cannot serve a workspace build — 11 consecutive local builds were
OOM-killed on 2026-09-16 and CI is the gate.

**AC-6 — the D0 answers are recorded**, including D0-1's re-measurement and, if
you added a positional form, the reaching consumer that justified it.

## 7. Base pin

Cut from a **literal SHA on `origin/main`**, never from the ref. Record it here
when you cut, and verify at the moment you adopt it:

    BASE=<literal 40-char sha>
    git rev-parse --abbrev-ref HEAD        # cite the branch you are ON
    git diff "$BASE" HEAD -- crates/       # must be EMPTY at adoption
    test -n "$BASE"                        # a degenerate range passes vacuously

`origin/main` moves under you mid-turn because sibling worktrees fetch. A pin
recorded as a ref is not a pin.

## 8. Sequencing

**Independent. Nothing gates this node** — `depends_on: []`, verified at
`ac4b59571`. It gates `LANG-MEMBERSHIP-OPERATOR-SURFACE`, which additionally
waits on `SPEC-MEMBERSHIP-CLASS-CONTRACT` (spec, active),
`LANG-RESERVED-INFIX-NAMES` and `LANG-STANDARD-INFIX-CALL-COMPLETION`.

⇒ **This can lead, and it is the language lane's next work** once
`LANG-BARE-OPERATOR-ATOM-REJECTION` lands.

## 9. Contention

`crates/ken-elaborator/src/` — `ast.rs`, `parser.rs`, `resolve.rs`, `elab.rs`.

**`parser.rs` is hot.** `LANG-BARE-OPERATOR-ATOM-REJECTION` (`ca4ef3472`) is in
the publisher now and touches it. **Check live node status at `origin/main`
before you cut**, not against a branch ref — and cut after that candidate lands,
so your base already contains it.

No contention with the runtime candidate (`93fa1d263`, all under
`crates/ken-runtime/`) or the foundation one (`fefde16e`, docs only).

## 10. Not this node

- The `Membership` class, its views, or its laws — `SPEC-MEMBERSHIP-CLASS-CONTRACT`
  (spec) and `LANG-MEMBERSHIP-OPERATOR-SURFACE` (build).
- `∈`'s elaboration or dispatch — `LANG-MEMBERSHIP-OPERATOR-SURFACE`.
- Projection in expression position — it already exists.
- Any kernel change.
- **Whether the instance registry and `resolve_instance_dictionary` handle a
  class at a non-zero universe level.** This is UNMEASURED (Architect,
  `evt_4tmt0n7era4w3`). It does not gate this node's premise, and **nobody
  should cite the D0-1 ruling as having closed it.** If you trip over it, report
  it; do not absorb it.

## 11. Related

- `LANG-MEMBERSHIP-OPERATOR-SURFACE` — the build this unblocks.
- `SPEC-STANDARD-INFIX-BINDING` — A-track; its `§2f` and `39 §6.9` are what the
  builtin escape hatch would falsify.
- `LANG-STANDARD-INFIX-CALL-COMPLETION` — A1, the shared use-site resolver.
- `SPEC-MEMBERSHIP-CLASS-CONTRACT` — landed at `a63eebe3e`; source of the three
  consumer bindings in AC-1.
