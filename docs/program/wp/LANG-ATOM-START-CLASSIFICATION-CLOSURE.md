# LANG-ATOM-START-CLASSIFICATION-CLOSURE — work package

**Owner: Team Language. Size M. Tier T1. Gate: none.**
**Implementation base: `origin/main` as of whenever you cut your branch. NAME
IT in your first post — AC-0 requires you to re-measure the defect there.**
**Inputs below measured at `6acd40705709cadc5e9a86cd0a83e835fd9dc9f4`.**

> **The base is deliberately not a SHA here.** A release-time SHA in this line
> decays between release and the cut — the publish queue moves `main`, and
> `LANG-STANDARD-INFIX-CALL-COMPLETION` is live in the same file. A base you
> measured at beats a base someone wrote down for you. **AC-0 is where that
> gets checked.**

Raised by the Architect from their own carry list, `evt_a1t4jpv9tvc0`. Two
spec-authority rulings from the Spec enclave settle the objective; both are
cited in §2.

## 1. Objective

**Make "what starts an atom" one classification over ATOM FORMS, applied to
EVERY atom-start roster in the parser, so the places encoding it cannot drift
apart.** Close the measured omissions as a consequence of that, not as the
deliverable.

**This is a parser repair, not a spec correction.** Both rulings went the same
way and both are on the record (§2). The node does not re-litigate them.

> ### THE REMIT IS ROSTER CLOSURE GENERALLY, NOT ONE PREDICATE. MEASURED.
>
> The defect was found at `can_start_atom_expr`. **It is also present, in
> identical shape, at `can_start_atom_type`** — a different function, a
> different subtraction, the same asymmetry, measured at `6acd40705` (§4b).
> **A third roster pair, `can_start_pattern` / `can_start_atom_pat`, is
> genuinely uncensused** and is AC-9.
>
> **A node scoped to `can_start_atom_expr` fixes two of three occurrences and
> leaves an identical third behind a different function name.**
>
> ⇒ This is the same failure as AC-1's, one level up. There the repair
> inherited the **domain** of the instrument that found the defect. Here it
> would inherit its **location**. **Scope the repair to the population, not to
> wherever the finding happened to surface.**

## 1a. THE JUSTIFICATION IS A MECHANISM, NOT A COUNT. MEASURED 4 OF 4.

**Read this before §4. The census is this node's backlog; this section is its
reason.**

Every commit that added an atom production, diff restricted to
`crates/ken-elaborator/src/parser.rs`, counting lines matching
`can_start_atom` (Architect, `evt_m5txd7g2jmqk`, direct per-commit reads at
`origin/main`):

    69570b265  "WP #29 Lane B -- parse bare proof-selector atom"   +12        0
    6a85d8dec  "SURF named proof claims core"                      +261/-30   0
    83967e3c1  "LANG-TRUNCATION-SURFACE-SYNTAX D1-D3"              +12        0
    15c4ba089  "LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES D1"           +28        0

**Four forms, four commits, zero visits to the roster.**

**`69570b265` is the one to read twice.** Its stated purpose is *parse bare
proof-selector atom* — a WP dedicated to making `proof_ref` a bare atom. It
delivers the production in twelve lines and never touches the predicate
deciding whether a bare atom is admitted **as an argument**. **The WP that
existed to make the form bare is the WP that left it non-bare in one of its
two positions.**

⇒ **The gap is not drift between two artifacts that fell out of sync. Adding
an atom form is a PRODUCTION edit, and the roster is an artifact the
atom-adding workflow does not visit.** The head/argument split is that
workflow's signature — which is why it reproduced identically on the type side
and why it will reproduce on the next atom form.

> ### WHY THIS PARAGRAPH EXISTS AND THE CENSUS IS NOT THE ARGUMENT
>
> **Three instances is evidence that something is wrong. 4-of-4 is a cause.**
> The closure's value is not tidiness — it is that **it makes the omission
> impossible rather than unlikely.**
>
> **A frame justified on "three gaps found" invites a two-line fix that adds
> two roster entries and declares victory. A frame justified on the mechanism
> cannot.** If you find yourself about to close §4's tokens by hand, you are
> answering the backlog and not the node.

**Unmeasured, and named rather than assumed:** whether the roster has *ever*
been widened by some unrelated commit. Two history queries were attempted and
both were unsound — one ran ref-less against a branch 1616 commits behind
main, the other keyed on the function *name*, which a body edit does not
touch. **The 4-of-4 stands without them** because it is a direct read of four
diffs, and a counterexample would not weaken it.

## 2. The two rulings this node stands on

**`proof_ref` — spec is authority** (spec-leader, `evt_4bhb0n2w8btry`).
`32-grammar.md:397-401` is normative, not illustrative: it names the AST node
(`Expr::EAttachedProofRef`), the desugaring target, and works the application
decomposition out longhand. Provenance `f64c788ba`, *"WP #29 Lane A — bare
attached-proof selector atom"*, a dedicated WP whose purpose was making
`proof_ref` a bare selector atom.

**Truncation — spec is authority** (spec-leader, `evt_8v50za9r1nvm`).
`32-grammar.md:276-278`'s *"first-class atomic type, spellable wherever a type
is expected... and, since types are terms (§3), in expression position"* is a
design claim about surface uniformity, unqualified, and it predates the
asymmetry being discovered.

**Its neighbour two clauses over is NOT the same kind of sentence and is NOT
ruled by the above.** *"Expression position is landed now"* is a **claim of
fact about implementation state**. A factual claim cannot be "intended" — it
is true or it is too broad, and this one is too broad. Its repair is narrowing,
and it belongs to spec-author's vocabulary edit, **not to this node**.

> ### A SPEC SENTENCE IS EITHER NORMATIVE OR A CLAIM OF FACT, AND THEY REPAIR
> ### DIFFERENTLY.
>
> A normative claim can be overruled by the implementation's design. A factual
> one can only be corrected or narrowed. **Reading the second as the first
> invents a design fork that does not exist** — the Steward did exactly that
> when routing this node, and it would have cost the enclave a ruling on a
> non-question. Both sentences sit in `32 §2`, two clauses apart.

## 3. Fixed inputs, measured at `6acd40705`

    crates/ken-elaborator/src/parser.rs

    EXPRESSION SIDE
    2449        the APPLICATION-ARGUMENT loop's gate -- call site 1
    2505-2523   can_start_atom_expr   the roster, 14 tokens
    3002        parse_atom_expr_base  the arms, 17 tokens + `other`
    3063-3072   the CONTEXTUAL selectors, inside the `Token::Ident(s)` arm
    3122-3125   an existing soundness argument that reasons FROM the roster
                being narrow -- see AC-4
    3131        the operator-prefix tail's gate -- call site 2

    TYPE SIDE -- the second occurrence, same shape
    2046        can_start_atom_type   the roster, 4 tokens
                parse_atom_type       the arms, 5 tokens

    PATTERN SIDE -- UNCENSUSED, see AC-9
    2783        can_start_pattern
    2801        can_start_atom_pat

    parse_app_expr's leading-form dispatch   Lambda KwLet KwMatch KwIf

**The relation that should hold is `roster == arms MINUS leading-forms`, and no
artifact states it**, so all three drift independently. **The compiler cannot
see the drift:** the roster is a `matches!` over a subset, which stays
well-typed however far behind it falls.

## 4. THE CENSUS IS COMPLETE OVER TOKENS AND INCOMPLETE BY CONSTRUCTION
## OVER EVERYTHING ELSE

Derived mechanically, twice, independently, with the identical result
(Architect `evt_a1t4jpv9tvc0`; spec-author re-derivation):

    arms MINUS roster = { KwIf, KwProof, TruncBar }

    KwIf       DELIBERATE exclusion. Carries its own affirmative rejection
               arm. MUST NOT BE CLOSED. It is the negative control, and it
               fires correctly today.
    KwProof    omission. Close it.
    TruncBar   omission. Close it.

**Two omissions, not three, and the set has not grown since the first post.**
`TruncBar` was found, measured and classified in the Architect's opening
finding; it was later re-derived independently by spec-author from the test
corpus. **Two people finding one object is one object.** Do not go looking for
a third token — there is not one.

> **The census's domain is TOKENS, and that is not the population.** The
> population is ATOM FORMS; tokens are merely how *some* of them are
> recognized. **A form that is not token-keyed is not in the subtraction's
> domain at all**, so it cannot appear in the difference whether it is broken
> or fine. The known contextual forms came back clean — that is a measurement
> (below), not something the census established.

**Measured clean, so the two known contextual forms need no repair**
(Architect, seven fixtures at `6acd40705`):

    PARSED  recursive result for xs           head
    PARSED  f (recursive result for xs)       grouped argument
    PARSED  f recursive result for xs         BARE ARGUMENT -- clean
    PARSED  induction hypothesis for xs       head
    PARSED  f (induction hypothesis for xs)   grouped argument
    PARSED  f induction hypothesis for xs     BARE ARGUMENT -- clean
    control f a b c d                         four-argument spine

They clear the roster because `Token::Ident(_)` is already in it for ordinary
variables — **they ride in by accident, and nothing in the roster knows they
exist.** `32 §3:406` agrees: *"Each four-word sequence is contextual... its
words remain ordinary identifier tokens."*

## 4b. THE SECOND ROSTER — SAME SUBTRACTION, TYPE SIDE, SAME RESULT

Measured with `parse_decls` at `6acd40705` (Architect, `evt_4dzdnaqftvese`):

    parse_atom_type arms      KwType  ConId  Ident  LParen  TruncBar    5
    can_start_atom_type       KwType  ConId  Ident  LParen              4

    IN PARSER, NOT IN ROSTER   TruncBar
    IN ROSTER, NOT IN PARSER   (none)  -- a strict subset here too

    PARSED    fn f (x : ‖ Bool ‖) : Nat = Zero       type HEAD, unicode
    PARSED    fn f (x : ||Bool||) : Nat = Zero       type HEAD, ascii
    PARSED    fn f (x : G (‖ Bool ‖)) : Nat = Zero   arg, GROUPED   control
    REJECTED  fn f (x : G ‖ Bool ‖) : Nat = Zero     arg, BARE
              parse error at 12-15: expected RParen, found TruncBar
    PARSED    fn f (x : G Bool) : Nat = Zero         arg, plain ident  control

**Third occurrence of one shape — parses at the head, rejects as an
application argument, grouped form fine.** Expression side twice (`proof_ref`,
`‖A‖`), type side once (`‖A‖`). **Two rosters, neither knowing about the
other.**

## 4c. THE PATTERN ROSTERS ARE CLEAN. THE REMIT IS TWO ROSTERS, NOT THREE.

Censused by spec-author at `origin/main`, both rosters read **verbatim**
rather than inferred from a count (`evt_1t7wcwex197hv`):

    parse_atom_expr_base / can_start_atom_expr   17 vs 14   {KwIf*, KwProof, TruncBar}
    parse_atom_type      / can_start_atom_type    5 vs  4   {TruncBar}
    parse_atom_pattern   / can_start_pattern     12 vs 12   none, BOTH DIRECTIONS

    * KwIf deliberate

**`can_start_atom_pat` is NOT a third roster.** It is
`can_start_pattern() && !is_contextual_ident("as")` — it inherits rather than
duplicating.

⇒ **Close `can_start_atom_expr` and `can_start_atom_type`. The pattern side is
a WORKED EXAMPLE of the target shape, not a site to repair** — which is the
direction an implementer is rarely told: where not to go.

> ### AMENDED 2026-09-18 (Steward), measured at `b53dd9fcf`: the pattern side
> ### was the worked example on a SECOND axis nobody had named.
>
> This section compares the three rosters on **content** — which tokens each
> admits, 12 vs 12 on the pattern side. There is a second axis, **where the
> exclusion is applied**, and the pattern side was already right on that one
> too. Re-measured in `parser.rs`:
>
>     can_start_atom_expr    SELF-GUARDS   atom_start_exclusion(Expression)
>     can_start_atom_type    SELF-GUARDS   atom_start_exclusion(Type)
>     can_start_atom_pat     SELF-GUARDS   atom_start_exclusion(Pattern)
>     can_start_pattern      NO GUARD, AND SHOULD NOT HAVE ONE
>
> `can_start_atom_expr` acquired its guard from the adversary's hardening; the
> other two had theirs already. **The three ATOM entry points self-guard.**
> `can_start_pattern` is the shared base predicate that `can_start_atom_pat`
> wraps, exactly as §4c says above — it is not a fourth site and not an
> unguarded hole.
>
> **The defect the guard closes is not a missing token; it is a per-CALLER
> obligation.** While `can_start_atom_expr` was a pure token predicate,
> applying the exclusions was every caller's job, and such an obligation is
> discharged by whoever remembers. The bare-operator-name gate did not, so
> admitting `KwProof` to the roster would have made
> `fn f (x : Int) : Int = <=` legal whenever the next token was `proof`.
> Guarding at the entry point covers every consumer by construction, including
> the ones nobody has written yet — the same closure AC-1 states over tokens,
> taken over consumers.
>
> **Correcting my own carried note, because it was wrong in the direction that
> manufactures work:** I had this recorded as *"all three rosters self-guard."*
> They do not. The three that self-guard are the atom entry points, and
> `can_start_pattern` — one of the three ROSTERS this section names — is
> deliberately not among them. A note that conflates the two would have sent an
> implementer to put a guard on the base predicate, duplicating
> `can_start_atom_pat`'s and re-creating the asymmetry the hardening removed.

> **The census's own instrument failed twice, and only one direction mattered.**
> An arm regex missed or-pattern continuations and manufactured seven phantom
> entries — it **invented** a finding. A roster regex over-collected the type
> side's guard tokens and reported 4 as 6 — **over-collecting the roster
> shrinks the forward gap, so that direction HIDES a real omission.** The `4`
> is right and the `6` was the artifact.
>
> **Report both directions of an instrument defect and say which one could
> have concealed something.** A census that over-reports is embarrassing; one
> that over-collects its subtrahend is unsound, and they do not look different
> in the output.

## 5. The position taxonomy — the axis this defect lives on

    position                    proof_ref     ‖A‖
    expression, head            PARSES        PARSES
    expression, app argument    REJECTS       REJECTS
    type / annotation, head     n/a           PARSES     -- measured, §4b
    type, app argument          n/a           REJECTS    -- measured, §4b

**"Expression position" is not one position, and both `32 §2` and `§3` use it
as though it were.** `§3` escaped the trap only because it worked
`f proof p for s` out longhand.

> **A worked example is a specification with its own test attached. A
> positional noun phrase is not.** Treat the former as an oracle; treat the
> latter as a claim needing a position before it can be checked.

> ### AMENDED 2026-09-18 (Steward), THEN SUPERSEDED THE SAME DAY — the
> ### bracket-depth coordinate this heading asks for DOES NOT EXIST. Read to
> ### the sub-block below before acting on anything in this amendment.
>
> **`expression, app argument` is not one cell.** Under the Architect's `KwProof`
> ruling (`evt_75pnw3svb4z4b`) it splits, and the frame's axis cannot express the
> split:
>
>     position                                      proof_ref
>     expression, app argument, bracket depth > 0    CLOSES under the ruling
>     expression, app argument, bracket depth 0      STAYS REFUSED, structurally
>
> > #### SUPERSEDED 2026-09-18T09:38Z — THE DEPTH COORDINATE IS RETIRED.
> >
> > **The `depth > 0` row above is WRONG and the split it introduces does not
> > exist.** Spec ruled (`evt_17x6mx5bg96bv`) that `proof_ref` as an argument
> > must be grouped **at every depth**; the Architect argued for that against
> > their own depth-keyed ruling (`evt_5vr1ecznne66j`), and language-leader
> > released the build on it (`evt_617jawg1bv5te`).
> >
> >     position                                    proof_ref
> >     expression, app argument, ANY bracket depth  REFUSED, unconditionally
> >
> > **The whole bracket-depth coordinate is retired**, not re-valued: no
> > counter, no bookkeeping, no depth-keyed `StartExclusion` member. The
> > deciding argument was that a bare form legal inside brackets would make
> > `proof_ref` **the grammar's only depth-sensitive construct, a distinction a
> > reader cannot see locally** — and that zero rules in `32-grammar.md` are
> > keyed on bracket depth, measured rather than asserted.
> >
> > **This amendment's own prediction came true against itself, one level
> > deeper than predicted.** It said to expect the next repair to find a
> > coordinate this table still lacks. Instead the next repair found that the
> > coordinate this amendment *added* should not exist at all. ⇒ **A taxonomy
> > can be wrong by having too many axes as well as too few, and the axis you
> > just added is the one you will defend rather than re-examine.**
> >
> > Nothing else in this section changes: the depth-0 structural argument is
> > intact and is now simply the argument for the whole row.
>
> **The refusal is not an implementation shortfall.** Ken has no declaration
> terminator, so the atom-start roster's complement *is* the declaration
> separator; admitting `proof` bare makes a declaration body swallow the
> next declaration. **That holds for any parser of this grammar, not just ours.**
> The sanctioned spelling is `f (proof p for s)`, which works today
> through the grouped arm — `parser.rs:3672`, pinned at `5107f4adc`, because
> this file moves under its line numbers constantly.
>
> ⇒ **The objective's omission-closure clause does not reach `proof` at all.**
> The bare form is refused in argument position unconditionally, so there is no
> position where it closes *as a consequence of the unification*. The row is
> discharged by the third ledger value — **CLOSED BY EXPLICIT CLASSIFICATION AS
> REFUSED** — not by the unification and not by an increment. Do not score the
> refused row as an unmet AC, and do not score it as closed by unification
> either.
>
> > #### THE THREE SENTENCES ABOVE WERE CORRECTED 2026-09-18T10:14Z, and the
> > #### reason is worth more than the correction.
> >
> > They sat **after** the SUPERSEDED sub-block that retires the bracket-depth
> > coordinate, and they went on using it — *"the depth-0 row"*, *"at depth 0"*,
> > *"wherever a declaration cannot begin"*, the last of which is a depth-keyed
> > quantifier wearing prose clothes. The retraction landed on the **table** and
> > left the **paragraph that reads from it** untouched, so a reader who got
> > past the marker was handed the retired coordinate as live text.
> >
> > **Deleting that quantifier is where this correction nearly went wrong, and
> > the near-miss is the more useful half.** The first draft struck *"wherever a
> > declaration cannot begin"* and stopped, leaving *"`proof` closes as a
> > consequence of the unification"* — which, with the qualifier gone, asserts
> > closure **everywhere**, the exact opposite of the unconditional refusal that
> > was just ruled. ⇒ **REMOVING A RETIRED QUALIFIER STRENGTHENS THE CLAIM IT
> > QUALIFIED.** A retired coordinate cannot simply be struck out; the sentence
> > has to be re-derived from the new ruling, because the qualifier was the only
> > thing holding the claim down to a range that no longer exists.
> >
> > ⇒ **A SUPERSESSION MARKER BOUNDS A REGION, AND THE PROSE THAT DEPENDS ON
> > THAT REGION IS NOT INSIDE IT.** Marking the table cost one edit and felt
> > complete; the sentences downstream of it are where the retired idea
> > actually survives, because they restate it in words rather than reproduce
> > it in a row.
> >
> > The citation was wrong in the same place and for a different reason:
> > `parser.rs:3637` is inside the **bare** arm the erratum forbids, cited as
> > the arm that sanctions the grouped spelling. Corrected to `:3672` and
> > pinned. This was the second site of that citation; the first was repaired
> > when it landed, which is how a corrected fact ends up disagreeing with
> > itself across two files.
>
> **This is §5's own lesson recurring against §5.** The section exists because
> *"expression position is not one position"* — and the refined taxonomy it
> built is itself too coarse, by exactly one coordinate, discovered the same way
> the first coarseness was. **A position axis sharpened once is sharp to the
> resolution of the defect that prompted it, and no further.** Expect the next
> repair on this node to find a coordinate this table still lacks; that is the
> normal behaviour of a taxonomy, not a defect in this amendment.
>
> **That prediction is NOT standing permission to miss a row** (Architect,
> `evt_3r6xjnk510pmb`). As written the table is **scoring-complete for the rows
> AC-0 enumerates.** A new coordinate is a **frame amendment** — authored,
> stated, reviewed — never a silent re-score of a row already being counted.
>
> **No AC changes.** AC-0 is a baseline measurement — it is satisfied by the bare
> cases being REJECTED, so it never required the bare form and is not
> unsatisfiable. AC-1 is a property of the encoding, which `ProofSelector`
> satisfies the same way the existing three **Expression-position** members do
> (`EffectRowAnnotation`, `MatchEquationBinder`, `BraceOpensMatchArms`).
>
> **The residual is RESOLVED, not outstanding.** It was routed to the Spec
> enclave as a behavioural-contract question and SPEC-32-PROOF-REF-ARGUMENT-
> ERRATA has since landed (`5107f4adc`), correcting the grammar's worked
> example: a bare `proof_ref` argument must be grouped, and `proof_ref` is not
> an `application_atom`. The grammar and this node now agree. Nothing here
> waits on a declaration terminator.
>
> > #### CORRECTED 2026-09-18T10:19Z — THREE MORE SENTENCES, SAME MARKER.
> >
> > The Architect blocked the first pass of this correction for stopping at one
> > paragraph (`evt_k202bnbe4was`). Three further sentences at this same `>`
> > level still asserted the retired coordinate: a *depth-keyed* exclusion
> > member, a contradiction with the grammar, and a depth exclusion to delete
> > if a terminator arrived. **The middle one had gone FALSE forty minutes
> > earlier**, when SPEC-32 landed and the grammar stopped asserting what the
> > sentence said it asserted.
> >
> > ⇒ **MY RETRIEVAL SELECTED THEM OUT, AND I DESCRIBED THE RESULT AS THE
> > WIDER SET.** I found the first paragraph by grepping a stale *citation*, so
> > my working set was *sentences carrying a bad line number* — then I reported
> > it as *the prose downstream of the marker*. These three carry no citation
> > and the instrument could never have reached them. An enumeration inherits
> > the selection of whatever produced it.
> >
> > **The count was never wrong, and my first attempt to fix it introduced the
> > error.** The block reported *"six members, not three"*, read off
> > `StartExclusion::COUNT`. That symbol counts the roster across **all three
> > positions**; this sentence's referent is the **Expression** set, which is
> > three before `ProofSelector` and four after. *"The existing three"* was
> > correct when written and still is. The Architect caught their own number
> > before I committed (`evt_53yrdrsyefhj2`); both of us verified from
> > `applies_in` rather than from the enum.
> >
> > ⇒ **A COUNT IS ONLY CHECKABLE AGAINST ITS REFERENT, AND THE NEAREST SYMBOL
> > IS NOT THE REFERENT.** `COUNT` sits beside the enum and answers a question
> > nobody asked here.
> >
> > **And the repair I reached for first was worse than the count.** I replaced
> > *"the existing three"* with *"every other `StartExclusion` member"* —
> > applying the write-a-predicate-not-a-count rule. But the roster spans three
> > positions and `AsAlias` and `BinderName` are not Expression members, so the
> > quantifier ranges over a **different set than the count counted**.
> > ⇒ **A QUANTIFIER IS NOT A SAFE SUBSTITUTE FOR A COUNT UNLESS IT RANGES OVER
> > THE SET THE COUNT COUNTED.** Generalising looks like strengthening and is
> > silently a re-scoping. The text now names the position *and* the three
> > members, so the referent is on the page and the count is checkable against
> > it.

**Do not retire the argument position into `LANG-TRUNC-INTRO-DIAGNOSTIC-
REMEDIES` D1** — the Architect nearly reported the opposite and flagged the
move explicitly.

> ### THE STAGING PARENTHETICAL IS WRONG IN BOTH DIRECTIONS AT ONCE
>
> `32 §2` carries two position-blind reports of implementation state, and each
> went stale in whichever direction nobody re-measured:
>
>     "expression position is landed now"         OVERSTATES  (head only)
>     "rejects ‖A‖ in a type position until D1"   UNDERSTATES (head annotation
>                                                 position PARSES today, §4b)
>
> **Neither is normative; both are factual.** A single overclaim reads as
> optimism. **A matched pair in opposite directions shows the vocabulary
> cannot express the distinction at all** — which is the case for the position
> axis, and it is stronger than either error alone.
>
> **This node does not fix that sentence and does not re-scope D1.** Both are
> spec-author's and spec-leader's. It is recorded here because D1's actual
> remit is narrower than its sentence implies, and someone reading D1 as the
> home for the argument position would be reading a claim that is already
> false.

## 6. Deliverables

1. **One classification keyed on the ATOM FORM, not on `Token`.** Each entry
   carries its start test and its parse together.
2. **All three consumers read it** — the argument loop's gate, the
   operator-prefix tail's gate, and the atom dispatch.
3. **`KwProof` and `TruncBar` admitted as a consequence**, with `KwIf` still
   refused and still carrying its own message.
4. **The leading-form rejection message DERIVED from the classification**, not
   hand-written per case.
5. **The operator-prefix message repaired.** Today it reads *"group it as
   `(≤)` to use it as a value, or apply it to at least one argument"* — to an
   author who has applied it to two. **A message instructing the reader toward
   the state they are already in costs more than no message.**
6. **AC-6's finding on further contextual forms, reported either way.**

## 7. Acceptance criteria

**AC-0 — THE DEFECT STILL OCCURS AT YOUR BASE. RUN THIS BEFORE AC-1.**
Re-run this frame's own two-sided probes **at your implementation base** — not
at `6acd40705` — and paste the output:

    REJECTED  f proof p for s          PARSED  f (proof p for s)
    REJECTED  f ‖x‖                    PARSED  f (‖x‖)
    REJECTED  fn f (x : G ‖Bool‖)...   PARSED  fn f (x : G (‖Bool‖))...
    FIRES     f if a then b else c     -- the negative control

**If any bare case PARSES at your base, STOP and return to the Steward —
UNLESS a landed increment of THIS node closed it. Name that increment and the
row it closed.** If no increment of this node accounts for the row, the node's
premise has changed, and that makes its scope WRONG, not merely smaller.

**Name the increment as recorded in the issue file's LANDED INCREMENTS table —
the SQUASH.** Every landing is a squash, so a candidate SHA is **not an ancestor
of `main`** and never becomes one: the KwProof increment cutting after
`9c4dda0b2` lands will find that SHA absent from `main`'s history. **If you cite
a candidate SHA, establish landedness by CONTENT — blob identity — never by
ancestry**, or a reader checking your ledger by ancestry gets a false negative on
a true entry. (Architect, `evt_3r6xjnk510pmb`.)

> ### AMENDED 2026-09-18 (Steward). THE UNLESS CLAUSE IS NOT A SOFTENING.
>
> **Without it this AC reports the node's own success as a premise failure, and
> it does so on the very next cut.** The TruncBar increment
> (`9c4dda0b226e11edaf29c7b087a86ca584cfed01`) makes **two** bare rows PARSE —
> that *is* the repair — so the KwProof increment would cut from a `main` where
> bare rows parse and be instructed to stop and declare the scope wrong. Raised
> unprompted by the Architect (`evt_exrk02z1p7gg`), one cut before it would have
> fired.
>
> **The ledger as of that increment, MEASURED by the implementer at
> `9c4dda0b2` (`evt_5tcc70s0t90st`) — it is TWO rows, not one:**
>
>     PARSED    f ‖x‖                  closed by 9c4dda0b2, EXPRESSION position
>     PARSED    fn f (x : G ‖Bool‖)    closed by 9c4dda0b2, TYPE position
>     REJECTED  f proof p for s        unchanged
>     FIRES     f if a then b else c   negative control, unaffected
>
> `TruncBar` is **position-polymorphic** — it has arms in both `parse_atom_type`
> and `parse_atom_expr_base` — so one increment closes a row on each side.
>
> ⇒ **Do not read this worked example as the inventory.** The clause says *"name
> the row it closed"* and accommodates any number; a reader who takes a singular
> example as the list leaves the type row with no accounting increment, and it
> trips the stop. **A count stated in prose beside a clause that does not carry
> the count is where this AC will be misread next.** The singular example was the
> Steward's, caught by the implementer before publication.
>
> **The discrimination the AC exists for is preserved exactly.** Its hazard is a
> SIBLING (`LANG-STANDARD-INFIX-CALL-COMPLETION`, live in the same file) silently
> closing the defect, so you build a repair for something already fixed. The
> observable is identical in both cases and the old predicate saw only the
> observable:
>
>     a bare row parses because a SIBLING closed it   -> STOP, correct
>     a bare row parses because THIS NODE closed it   -> proceed
>
> Naming the accounting increment is what separates them, and it makes the node
> accumulate a ledger of which rows it has closed — which AC-0's *"paste the
> output"* already half-produces. The `FIRES` negative control is unaffected.
>
> **Root cause: a once-per-NODE baseline written as a once-per-INCREMENT gate.**
> AC-0 was authored when this was one increment; it is now six.
>
> ⇒ **A GATE THAT HAS ONLY EVER TAKEN ITS PASSING BRANCH IS UNMEASURED.** The
> first five increments kept every bare row rejected, so AC-0 passed on every run
> and **its stop branch has never once executed** — unmeasured five times while
> reading as a discharged AC each time. A criterion whose own repair invalidates
> it is unsatisfiable, and the tell is available before the failure: ask which
> branch has ever run.
>
> **Whoever writes an increment-gated AC on a multi-increment node owes this
> check at every recut**, not just here: does the node's own progress move the
> gate's input? If it does, the gate needs a clause naming the node's own work.

> ### WHY THIS AC EXISTS, AND WHY IT IS AC-0 RATHER THAN AC-11
>
> **Every other criterion in this section is about the repair or about
> preservation.** Ask the reader-side question — *which AC fails if the defect
> does not exist?* — and the answer for AC-1 through AC-10 is **none**. If
> `f proof p for s` already parsed at your base, you could build the
> classification, pass every control, and close the node without anything
> reporting that there was nothing to fix.
>
> **This frame has a specific reason to care.** Every defect measurement in §3,
> §4, §4b and §5 is anchored at `6acd40705`; the base you work from is a later
> `main`, set at release, with other candidates in the publish queue ahead of
> you — and `LANG-STANDARD-INFIX-CALL-COMPLETION` is live **in the same file**.
> Its implementer reports that layer 3 does not touch the atom roster. That is
> a claim about a design, not about a landed diff, and nobody has measured it.
>
> **The probes are already written, so this costs one build.** Raised by the
> Architect (`evt_11nyyh216pqz2`), who supplied most of §3, §4b and §5 and
> named the reason they were the wrong seat to catch it: *"having spent the
> evening establishing that the defect is real, 'what if it isn't' is the
> question I was least able to ask."*

**AC-1 — the closure property, over BOTH kinds of atom form.** *Adding an atom
form must be impossible without its start test and its parse moving together.*
**State how your encoding enforces that, and demonstrate it for a
token-keyed form AND for a contextual multi-word one.**

> ### THE REPAIR THIS NODE WAS FIRST GIVEN FAILED EXACTLY HERE.
>
> The original proposal was *one total classification over `Token`*. **A
> `Token`-total function maps `Ident(_)` to "atom" and is finished; it cannot
> separate `recursive result for xs` from a variable named `recursive`.** So
> the closure property — *"a new atom form is a compile error until it is
> classified once"* — **holds for token-keyed forms and silently fails for
> contextual ones.**
>
> The Architect's account of how that happened is the reusable part, and it is
> why this AC leads: *"My census was a subtraction over TOKENS... then I wrote
> a repair whose domain was also tokens. **I never chose that, I inherited it
> from the instrument.**"*
>
> ⇒ **THE INSTRUMENT'S DOMAIN SILENTLY BECOMES THE REPAIR'S DOMAIN.** Check
> your repair's domain against the POPULATION, never against the instrument
> that found the defect.

**AC-2 — a control on what the loop now CONSUMES, and it must lead.** Today's
failures are **fail-closed**: valid programs refused, nothing misparsed.
**Widening the roster widens what the application-argument loop consumes,
which is the fail-open direction.** `f ‖x‖ y` is the required case — the
truncation must close and the loop must then continue. **A repair measured
only on its rejections going away cannot see what it started swallowing.**

**AC-3 — `KwIf` still refused, by the derived path.** The negative control
must still fire, and its message must now come from the classification rather
than from the one hand-written case. **If closing the roster silently admits
`if`, the classification has lost a distinction the old code had.**

**AC-4 — RE-VALIDATE THE INFIX-PATH ARGUMENT. BLOCKING, NOT ADVISORY.**
`parser.rs:3122-3125` reasons **from** the roster being narrow:

> *"The infix path is untouched: it consumes its operator directly and never
> routes one through this arm, **because `can_start_atom_expr` does not admit
> an operator token**, so an operator is only ever seen here when an
> expression STARTS with it."*

**This node's repair edits that comment's premise, from 600 lines away, with
no link in either direction.** Either restate the argument against the widened
classification, or show it never depended on the roster for operator tokens
specifically.

> **It is an accurate statement of a true fact that becomes false without
> anyone editing it, and nothing fails when it does.** That is why this is
> blocking: a prose dependency on a predicate's narrowness is invisible to
> every gate we have. **A narrow predicate is a guard for whoever quietly
> relied on it.**

**AC-5 — the acceptance suite varies POSITION, with the grouped form beside
each bare one.** `crates/ken-elaborator/tests/bare_proof_selector_atom.rs` is
the existing coverage for this feature and it is **green**. It varies the
**spelling** — bare, grouped, canonical `s::p` — and holds the **position**
constant at the application head. **The head call site is ungated; the roster
gates the argument loop and the operator-prefix tail. Every fixture routes
around the predicate under test.** Same shape in
`lang_truncation_surface_syntax.rs`, where every non-head fixture is
parenthesized. **Both suites must exercise head, bare argument, and grouped
argument.**

**AC-6 — ENUMERATE the contextual forms; do not assume the two known ones are
all.** `recursive result for` and `induction hypothesis for` are measured
clean (§4). **Whether the tree holds other contextual multi-word atom forms is
UNMEASURED, and no token subtraction can answer it.** Derive the set from the
lookahead sites, report it as derivation output, and say what your instrument
would have missed.

    FOUND, CLEAN      they parse in all three positions. Say so with fixtures.
    FOUND, BROKEN     report it; do not absorb it without coming back to the
                      Steward to re-size.
    NONE FOUND        a claim about your DERIVATION, not about the tree. State
                      how you searched and what shape would have escaped it.

**AC-7 — blast radius over POSITIONS, not token kinds.** Head, bare argument,
grouped argument, annotation. **Token kinds are how this was found; positions
are where it lives.** Elaboration, the formatter, and the catalog corpus are
unmeasured — **"how many real programs this rejects" is OPEN in this frame and
must not be reported as zero without a measurement.**

**AC-8 — every measurement states its reach.** See §8.

**AC-9 — PRESERVE THE TYPE SIDE'S GUARD SEMANTICS. This is where the work is.**

The two rosters are not the same shape, and the difference is the whole
difficulty of keying on the FORM:

    can_start_pattern      a flat `matches!`, twelve tokens, NO GUARDS
    can_start_atom_type    two NEGATIVE LOOKAHEAD GUARDS before its `matches!`
                           -- a `visits[` exclusion, and an `Ident|ConId`
                           followed by `Colon` exclusion

**THE CLOSURE MUST PRESERVE EVERY EXISTING REFUSAL, NOT MERELY EVERY EXISTING
ADMISSION.** `can_start_atom_type` is **not a function of `peek()`**:

    if peek == Ident("visits") && lookahead(1) == LBracket      -> false
    if peek matches Ident|ConId && lookahead(1) == Colon        -> false
    matches!(peek, ConId | Ident | KwType | LParen)

**The second guard is what stops a type-application loop consuming a BINDER
NAME.** `can_start_atom_type` is called from two `while` loops (`:1726`,
`:2032`); without it, `(x : T)` offers `x` to the loop as another atom
argument. **The admitted set and the refusals do two different jobs, and only
the admitted set looks like a roster.**

⇒ **A form's `start` test must express NEGATIVE context, not just a token
match.** State how your encoding does, and give the case each guard exists to
refuse. **A closure that admits what a guard was excluding is a fail-open
regression in type parsing** — the same direction as AC-2, in a second place.

> **Do not generalize from the pattern side.** It is clean *and* guardless,
> which makes it the misleading example: a shape that works there drops both
> type-side guards silently.

**AC-10 — the classification covers every roster, and adding a roster is not
free.** After the repair there must be no way to introduce a new atom-start
predicate that is not derived from the classification. **Two rosters with the
identical defect and neither aware of the other is the condition this node
exists to end** — a repair that fixes both by hand and leaves the next one
open has not done it.

> ### IF YOUR ANSWER IS NOT ON A LIST IN THIS FRAME, THE LIST IS WRONG —
> ### SAY SO AND STOP.
>
> Three Steward outcome menus in this series have been exhaustive over the
> wrong axis, and each time the real answer sat in the gap. **Report the
> outcome you measured, name the branch it does not fit, and hand it back.**

## 8. THE STANDING CONDITION ON EVERY MEASUREMENT THIS NODE PRODUCES

**Four instruments on this node were correct and answered a narrower question
than they were read as answering. All four narrowed along the same axis the
defect lives on.**

    the green acceptance suite   varies SPELLING, holds POSITION at the head
    the truncation test suite    every non-head fixture PARENTHESIZED
    the arms-minus-roster census cannot see a form that is not token-keyed
    the first proposed repair    inherited the census's TOKEN domain

**That is not four coincidences. The instruments and the defect have a common
cause** — spec-author named it: *"the normal shape of a test suite written
from the same understanding as the prose."*

⇒ **On this node, an instrument's clean result is evidence about the
INSTRUMENT until its reach is stated.** Every measurement in the handoff
carries what it ranged over and what shape would have escaped it. **A clean
result with no stated reach is not a result here.**

## 9. What this node is NOT

- **Not the two tokens.** Adding `KwProof` and `TruncBar` to the roster is
  hand enumeration that drifts again at the next atom form. The closure is
  the deliverable; the tokens fall out of it.
- **Not a spec correction.** Both authority questions are ruled (§2).
- **Not the staging-vocabulary edit.** *"Expression position"* needs the
  position axis wherever it appears as a bare noun phrase. That is
  spec-author's, held until this lands, and independent of the parser repair.
- **Not `LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES` D1**, which is the annotation
  position — a third row that does not account for the second (§5).
- **Not folded into `LANG-STANDARD-INFIX-CALL-COMPLETION`.** Separate defect;
  folding it would widen a live WP mid-flight. **A1's layer-3 work must not
  build on the roster as if it were correct** — the operator-prefix tail at
  `:3131` is one of the two gated call sites.

## 10. Contention — RUN CONCURRENTLY, and this was measured

`LANG-STANDARD-INFIX-CALL-COMPLETION` is live in the language lane.
**language-leader answered the sequencing question with a measurement, not an
estimate** (`evt_93qgz56ye40w`): A1's remaining work — layer-3 required-roles,
five fixities, the D1 completion adapter, `≠` — touches
`standard_operators.rs`, `modules.rs`, `elab.rs`, `error.rs`, `numbers.rs`,
with **zero occurrences of `parser.rs`**. Its landed work so far is 5 files,
`+336/-0`, also zero `parser.rs`. Structural rather than lucky: A0 already
landed every surface piece A1 needs.

⇒ **Run concurrently.** Separate branches, separate PRs.

**One flagged interaction, and it is this node's to own:** A1's D0 touches
`parser.rs:3131` — the operator-prefix tail, which is one of the two call
sites this node's classification replaces. **Coordinate on that line
specifically before either side edits it**; everything else in the file is
uncontended.

> ### `LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES` D1 — RAISED, GROUNDED, CLOSED.
> ### NO OVERLAP.
>
> Raised as a possible contention: D1 is framed as *adding* the
> annotation-position production, yet head annotation position already parses
> (§4b). If D1's remaining grammar content were the argument-position gap,
> two live nodes would edit the same two predicates.
>
> **Grounded by spec-author: D1 is MERGED and finished.** Node status
> `merged`; `Type::TTrunc` present on main; `parse_atom_type`'s `TruncBar` arm
> present on main; landing commit `15c4ba089`. **There is nothing left of D1
> to build or frame, so there is nothing to overlap with.**
>
> ⇒ **`§4b`'s measurement explains itself: head annotation position parses
> because D1 LANDED.** The spec sentence *"rejects `‖A‖` in a type position
> until D1"* is stale twice over — wrong about the rejection, and written
> against a future that has already arrived. That is the staging vocabulary's
> defect, not this node's, and it stays with spec-author.

## 11. Estimated tier: T1

**The closure's domain is a design call, and the first proposal got it wrong
in a way that type-checked.** Choosing what the classification ranges over,
and keeping a widened predicate from silently invalidating an existing
soundness argument, is judgment about the parser's structure — not
transcription.

## 12. Sizing note

**L, re-sized upward when the second roster was measured.** It was framed as M
against one predicate. It now covers three roster pairs, one of them
uncensused, plus the structural closure over all of them.

**The classification is a contained refactor. The measurement work is not** —
AC-4's re-validation, AC-6's contextual enumeration, and AC-9's pattern census
are each an independent piece of evidence.

**Deliver in increments; each of these is a COMPLETE turn on its own:**

    AC-9 alone     the type side's two negative guards, expressed as form-keyed
                   start conditions. This is the hard part -- do it FIRST, and
                   if the encoding cannot express a negative start condition
                   cleanly, that is a finding, not a failure.
    AC-6 alone     the contextual-form enumeration.
    AC-4 alone     the infix-path argument, restated or discharged.

**Post any one of them and stop** rather than carrying a half-finished closure
alongside unrun evidence.

**Two measurements that were open when this frame was drafted are now closed,
and neither needs re-running:** the pattern-roster census (§4c, clean) and the
contextual selectors at all three positions (§4, clean). **They are recorded
as measurements with their instruments named — do not re-derive them, and do
not treat them as assumptions either.**
