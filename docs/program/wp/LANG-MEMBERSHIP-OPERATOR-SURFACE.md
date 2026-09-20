# WP frame — `LANG-MEMBERSHIP-OPERATOR-SURFACE`

    owner   language       tier   T1        size   L
    depends SPEC-MEMBERSHIP-CLASS-CONTRACT        LANDED
            LANG-RESERVED-INFIX-NAMES             LANDED
            LANG-TYPE-PROJECTION-SURFACE-FORM     LANDED  294cb5e28
            LANG-STANDARD-INFIX-CALL-COMPLETION   IN FLIGHT -- the hold
    node    docs/program/issues/LANG-MEMBERSHIP-OPERATOR-SURFACE.md
    review  Architect REQUIRED. CODE merge -> full CI + M8/M8a Adversary.

## 1. Objective

Build membership: define the `Membership` class and its three provider views in
catalog, author `membership_member_at`, and wire `∈` elaboration to
member-dispatch **through A1's existing resolver** — never a second dispatcher.

**This is the B track of the reserved-infix-glyph objective.** A1
([[LANG-STANDARD-INFIX-CALL-COMPLETION]]) builds the completion adapter and its
first five consumers; this node is the sixth consumer and the first one whose
dictionary is not a lawful class.

## 2. THE HOLD, stated first because it is the whole sequencing story

**Three of four dependencies are landed and verified in the tree, not read off
a status field.** Measured by the Steward at `origin/main` `4bc5f0eee`:

    SPEC-MEMBERSHIP-CLASS-CONTRACT     spec/50-stdlib/58b-membership.md present,
                                       membership_member_at x3.  Its node still
                                       reads `active` -- that is UNFLIPPED M7,
                                       not unlanded work.
    LANG-RESERVED-INFIX-NAMES          Token::Member present in lexer.rs and
                                       parser.rs on main.
    LANG-TYPE-PROJECTION-SURFACE-FORM  merged 294cb5e28.

**`LANG-STANDARD-INFIX-CALL-COMPLETION` (A1) is NOT landed and is being built
right now.** It is the only remaining hold and it is a real one: `§4`'s
deliverables attach to surfaces A1 is still authoring.

> **DO NOT RELEASE THIS NODE UNTIL A1 LANDS, AND DO NOT RE-FRAME `§5` AGAINST
> A1's WORKING TREE.** A1 took **three amendments on 2026-09-17 alone** (FI-2a,
> FI-2b, FI-5), each changing where an identity lives or how the elaborator
> acquires it. **A frame written against a surface that moved three times in one
> day is a frame written against a guess.** The node's own instruction — *"then
> the Steward frames the full ACs from the landed contract + A1 surface"* — is
> obeyed by framing everything A1-independent now and pinning the rest at
> release.

**What is already settled and does NOT wait for A1** (`§3`), and what is
deliberately left open until it lands (`§5a`), are separated below so the
release step is a pin rather than a re-framing.

> # BOTH HOLDS ARE DISCHARGED. RELEASED 2026-09-20.
>
> **There were TWO, and `§2` above only ever knew about the first.** A1 landed
> at `e2e40e2b`, and the release that followed hit a hard stop this frame did
> not anticipate (`evt_4ehtrakx2ftdb`): `elab_standard_operator` took a carrier
> head only from `Term::Const`/`Term::IndFormer`, and
> `resolve_instance_dictionary_inner` refused every `head_param_count > 0`
> instance on the `requested: None` path. **All three mandated views are
> parameterized, so `∈` could not resolve one of its own providers.** That
> produced the second hold, `LANG-CORE-INSTANCE-HEAD-MATCH`.
>
> **It landed as squash `564e9e2cd6096fb9496569214b95dfcf946fb391`**, verified
> by blob rather than by subject: both of its paths — `elab.rs` and
> `lang_standard_infix_call_completion.rs` — are byte-identical between the
> approved candidate `cb33f7fdc6a9fc73e6a744ebcc5baf9ff71eedca` and `main`.
> Adversary returned NO DEFECT on the two named cruxes, the carrier-confirmation
> widening and dispatcher uniqueness.
>
> **`§5` item 6 is the one that changes meaning.** "Wired through A1's `D3`
> resolver" now means the resolver **as extended by `P2`'s core-side instance
> head matcher, inside `resolve_instance_dictionary_inner`** — not beside it.
> Two constraints bound there and they bind here: **match on `GlobalId`
> identity, never spelling**, and **the carrier confirmation survives and
> widens to the full instantiated carrier**. If the extended resolver still
> cannot serve a parameterized view, that is `D0-2` returning and it is a hard
> stop, not a local repair.

## 3. Fixed inputs — measured, all from the LANDED spec

### 3a. The class, `58b §1`. Unary, with an associated query type.

```
class Membership (container : Type) {
  Query  : Type
  member : Query → container → Bool
}
```

**Not `Membership query container`.** Instance resolution keys on **one
outermost head** (`39 §6.1`), so a second class parameter has nowhere to live;
the query type is carried *in* the dictionary. `Query` precedes `member` in the
telescope so `member` may name it.

**It is an ordinary structure class that lands one universe up**, and the
contract says so rather than leaving it to be discovered:

    Membership : Type ℓ → Type (suc ℓ)

A field whose *type* is `Type ℓ` contributes at `Type (suc ℓ)` (`12 §1`) and the
Σ-sort takes the maximum (`13 §4`). **This is `33 §5.2`'s class former
operating, not an exception to it** — do not treat the level as a problem to
engineer around.

### 3b. The three providers, `58b §3`. Nominal, witness-bound views.

    list view          (d : Ord a, xs : List a)                    Query = a
    ordered-key view   (d : Ord k, t : Tree k v,
                        ordered : Ordered d.leq t)                 Query = k
    relation-edge view (d, adjacency, the outer Ordered, and
                        evidence every stored successor tree is
                        Ordered under that same d)                 Query = Pair k k

**The ordered-key view at `v = Unit` serves set membership.** It mints **no
`Set` carrier and no second head.**

**`Membership Tree` is FORBIDDEN.** Key, set and relation-edge membership are
three meanings over one raw head and at most one could be canonical.

**THE COMPARATOR MUST BE THE ONE THE VIEW VALUE WAS VALIDATED WITH.** A raw
`Tree` binds no comparator in its type, so an implementation that resolves a
canonical `Ord` at each `member` call can use an order different from the one
the tree is `Ordered` under **and answer wrongly on a well-typed input**. The
witness travels in the view value for exactly this reason. ⇒ **An implicit
resolver choosing a fresh `Ord` at the use site is NON-CONFORMING** — this is a
criterion, see `AC-4`.

### 3c. The binding, `33 §6.3`. An ordinary top-level binding, not a method.

    membership_member_at (c : Type) (d : Membership c) (q : d.Query) (x : c) : Bool

`∈` takes **`infix 4`**, in the comparison band with `≤ ≥ ≠`.

**It is NOT a class method.** `d.member` is a field projection and has no
`GlobalId` for `39 §6.9`'s policy to key on.

**The query type is reached as a projection from an earlier parameter**, and the
kernel admits it: with `Query` as the provider's first field, the telescope
`[c, d, Proj1(d), c]` is closed by the binding's own parameter `d` — ordinary
dependency, exactly like `(a : Type) (x : a)`.

**The projection prerequisite is DISCHARGED, not outstanding.** `32 §2`'s
`tproj` admits `d.Query` in type position, landed by
`LANG-TYPE-PROJECTION-SURFACE-FORM`. The node's banner recording this as a
blocker is **stale and known stale** — see `§3f`.

### 3d. Carrier-first completion, `39 §6.10`. The one rule specific to `∈`.

**INFER THE RHS CARRIER FIRST.** For `q ∈ c`: elaborate `c`, resolve the one
canonical `Membership` dictionary for its head by the ordinary search of
`§6.2`, project the provider's `Query`, and **only then** check `q` against that
type. **The LHS is never used to guess among carrier meanings.**

> **The order is not a preference and the frame states the reason, because an
> implementation that gets it wrong LOOKS CORRECT.** A provider is keyed on the
> container's head (`33 §5.5`) and the query type is determined *by* the
> provider — so inferring from the LHS would have to guess which carrier was
> meant in order to know what the LHS should be. **An LHS-first implementation
> will appear to work wherever the query type happens to be unambiguous, and
> will diverge exactly where two providers accept the same query type over
> different containers.** That is a test you must construct; it will not arise
> by accident. See `AC-3`.

A missing provider, an ambiguous one, or a container with no admitted provider
is an **ordinary instance-resolution error** at the occurrence (`§6.7`) — never
a fallback to a different meaning, never a silent acceptance.

### 3e. THE BUILTIN ESCAPE HATCH IS CLOSED BY THE SPEC, and no test here catches it

`33 §6.3`, verbatim in substance: **`∈` must not be discharged by making it an
elaborator builtin.** `§6.1` requires every standard meaning to be an ordinary
top-level binding, and `§6.9`'s completion policy rests on that premise — **a
builtin has no `GlobalId` to key on and falsifies the precondition the whole
completion policy is built on, for `∈` and by precedent for every operator
after it.**

⇒ **This is the criterion most likely to be satisfied by accident and the one
with no natural failing test.** A builtin `∈` would parse, elaborate, evaluate,
and pass every behavioural test in this node. **`AC-5` gives it a control.**

### 3f. Two readings already corrected in the node. VERIFY, do not re-derive.

- **`:44` — the closing claim *"the catalog bindings this node must author are
  currently UNSPELLABLE"* is FALSE.** It was this node's blocking premise, so a
  seat picking it up reads it as a live obstacle. It is not one.
- **`:74` — `same_members` is a STANDALONE binding, not a class field.** The
  earlier D0 ruling requiring it as a class field is **SUPERSEDED**
  (`evt_2wmeawkbqqv2x`). Do not inherit it.
- **`:88` — all three bindings are unblocked by ONE production.** `same_members`
  puts its projection in the **body** (the domain of a `Π` in an
  expression-position definition), not the signature — and both `Π` forms
  annotate their domain with an `RType`, so a single projection variant on
  `RType` serves all three. **A fix scoped to parameter telescopes would have
  satisfied two of three and left this one unspellable**; it was checked and it
  is one production.

**`RType`'s shape is perishable** — the node says so itself. Re-measure before
relying on the third bullet; do not take it from the banner.

## 4. D0 — answer before writing production

> **D0-1 AND D0-2 ARE ANSWERED — Architect, 2026-09-17, `evt_4d6jab0wj9mjb`
> then `evt_xx1v1qvksqke`, which REVERSES the first on D0-1. The reversal is
> the operative ruling.** Both were raised while A1 was still open, on the
> ground that they are cheap now and expensive afterwards; both were ruled
> without a hard stop to A1's `§1a` chain. **They are recorded as answers, not
> as questions to re-ask.** Each leaves one item this node must still *do*.

**D0-1. ANSWERED: A1's role vocabulary closes at FIVE, and `∈` arrives with its
binding. This node adds the variant.** Not a gap and not a change to A1's
contract.

**A1's fixities attach to the `GlobalId`s its layer 3 certifies.** `∈` has no
binding, so no `GlobalId`, so **A1 could not install its fixity even if its
vocabulary named it.** A1 ships five roles and five fixities, coherently;
`§6.1`'s table names six glyph-fixity pairs and A1 realises the five whose
bindings exist. **A variant with no binding, no required-role entry and no
completion path is a dead arm, and a dead arm reads as coverage to every later
reader** — so the sixth arrives here, entering the **vocabulary** and the
**required list** as two separate acts.

**The Steward's premise for raising this was wrong and is struck:** *"B must
widen a closed enum, which is a change to A1's contract"* assumed a `pub` enum.
Crate-internal, widening is an **ordinary edit inside `ken-elaborator`**, and
`COORDINATION §7`'s exhaustive-by-construction rule turns it into a
**compiler-generated checklist handed to this node**, not a tax it pays.

> **THE ONE CONDITION, AND THIS NODE IS THE PARTY THAT PAYS IF IT FAILED —
> so VERIFY it at D0 rather than assuming it.** *"Crate-internal has to be a
> property you HOLD, not one you state"* (Architect). If A1's role type leaks
> through any `pub` signature in layers 1 or 3 — a `pub fn` taking or returning
> it, a `pub` field, a re-export — the property **evaporates silently**, and
> **this node is where it would be discovered, at widening time.** Check A1's
> landed surface for such a leak **before** widening. A leak is A1's defect and
> a hard stop back to the Steward, not something to work around here.

**D0-2. ANSWERED: `∈` satisfies `D1c`, because `D1c`'s shape contract is
specified over the ELABORATED TELESCOPE WITH BACK-REFERENCES, not as a signature
template.** Ruled deliberately, in those terms.

**And the contract is forced by `≤` before `∈` is mentioned at all** — the
stronger form, measured by the language implementer rather than argued from
prose:

    ord_leq_at : Π Type 0. (Π (g641 @0). (Π @1. (Π @2. Dg3)))

**Three of four domains are de Bruijn references to earlier binders**, so a flat
signature match was never sufficient for the five either. A domain `d.Query` is
a **projection** Term over `@k`; a domain `Ord a` is an **application** Term over
`@k`; both are Terms carrying a back-reference, and `33 §6.3` asserts these are
the same kind of dependency at the level the contract operates on — *"ordinary
dependency exactly like `(a : Type) (x : a)`"*. **There is no non-dependent
version of `D1c` anyone could accidentally build.**

> **WHAT REMAINS IS A MEASUREMENT, NOT A RULING, and it is this node's to make.**
> Whether `d.Query` is **representable at the point the check runs**. `32 §2`'s
> `tproj` landed with [[LANG-TYPE-PROJECTION-SURFACE-FORM]] (`294cb5e28`) and
> `§6.3` records that the telescope never changed and only the spelling was
> missing — so the machinery is **plausibly** already there. **Measure it; do
> not assume it from the ruling.** If it is not representable at that point, the
> gap is A1's surface: **name it and stop**, unchanged.

**D0-3. Is the `Ordered` evidence for the relation-edge view constructible in
catalog today?** `58b §3` requires *"evidence every stored successor tree is
Ordered under that same `d`"* — a nested invariant. **If the catalog cannot
express or discharge it, say which view is blocked and deliver the other two**;
that is a re-scope for the Steward, not something to fake with an axiom. **A
hard stop here is a good outcome.**

**D0-4. Does the one-universe-up class (`§3a`) elaborate today?** `Membership :
Type ℓ → Type (suc ℓ)` is ordinary per `33 §5.2`, but no catalog class has
needed it. **Check before building on it.** If it does not, this is a
language-capability finding and it precedes everything else in this node.

## 5. Deliverables

1. **The `Membership` class in catalog**, per `§3a`.
2. **The three provider views**, per `§3b`, each carrying its validity witness
   **in the value** — subject to `D0-3`.
3. **`membership_member_at`** per `§3c`, an ordinary checked top-level binding,
   defined exactly once.
4. **`member_holds` and `same_members`** as standalone bindings.
   `member_holds := IsTrue(member)`. **`same_members` is not a class field
   (`§3f`).**
5. **`∈` on the standard-operator facade**, at `infix 4`, re-exporting the
   binding's identity — **not a second definition.**
6. **`∈` wired through A1's `D1a`/`D1b`/`D1c` and its `D3` resolver**, with
   carrier-first order per `§3d`.

**NOT a second dispatcher.** A1's `D3` exists so this reuse needs none; if the
resolver cannot serve `∈` as landed, that is `D0-2` and it comes back.

## 5a. PINNED AT RELEASE — A1 HAS LANDED AND THESE ARE NOW MEASURED

**A1 landed at `origin/main` `e2e40e2b404d9775b3cd1fee049b3ecaab481bba`.** The
four items below were left open at framing because they are A1's landed shape;
they are pinned here, **read out of the tree at that SHA rather than off A1's
approval posts**. Navigate by symbol — every line number is current at that SHA
only.

    facade module PATH and name       A1 D2   Core.Operators.Standard
      catalog  catalog/packages/Core/Operators/Standard.ken.md
      const    standard_operators.rs  STANDARD_OPERATOR_HOME
      test     is_standard_operator_home(module: &str) -> bool

    role vocabulary variant spelling  A1 D1a  StandardOperatorRole
      enum     And, Or, Leq, Geq, Neq          (pub(crate))
      const    StandardOperatorRole::ALL: [Self; 5]
      accessor glyph(self) -> &'static str

    required-roles / shape site       A1 D1c
      const    StandardOperatorRole::BINDING_BACKED: [Self; 4]
                 = [And, Or, Leq, Geq]
      fn       expected_shape(role) -> &'static str
      fn       shape_matches(role, ty, bool_id) -> bool
      error    ElabError::StandardOperatorRoleWrongShape

    resolver exposed entry point      A1 D3
      fn       certify_roles(env, exports, globals, home, bool_id, span)
                 -> Result<HashMap<StandardOperatorRole, GlobalId>, ElabError>
                 (pub(crate))

**THE VOCABULARY AND THE CERTIFIED SET ALREADY DIFFER, FIVE AGAINST FOUR, AND
THAT IS NOT AN ARTEFACT OF `∈` BEING ABSENT.** `Neq` is in `ALL` and not in
`BINDING_BACKED`: `§6.1` names a binding for four roles and *describes* the
fifth, so an export-table check has nothing to look up for `≠` and requiring it
there would hard-error on a correct tree. **So this node adds `Member` to
`ALL`, and whether it also enters `BINDING_BACKED` is a separate act** — A1's
own comment anticipates exactly this and rules that if the two need to diverge
for `Member`, that is a third set which **"should arrive with the caller that
distinguishes them, not before it."** Do not mint that third set speculatively.

**`D0-2` IS ANSWERED IN A1'S LANDED CODE, NOT LEFT TO THIS NODE.**
`expected_shape`'s own doc states that a domain which is a PROJECTION from an
earlier binder (`d.Query`, `§6.3`) is the same kind of back-reference as one
that is an APPLICATION to it, **"which is why this shape extends to the
membership role without widening."** The measurement this node still owes is
narrower than the frame implied: whether `d.Query` is representable at the point
the check runs. Not representable ⇒ name it and stop.

**`D0-1`'s VERIFY CONDITION IS ENFORCED BY THE COMPILER, SO DO NOT DISCHARGE IT
BY INSPECTION.** The condition was that A1's role type must not leak through any
`pub` signature. `StandardOperatorRole` is `pub(crate)`, and
`crates/ken-elaborator/src/lib.rs` carries `#![deny(private_interfaces)]`, which
makes a leak a compile error rather than a review finding. **This node's
obligation is therefore not to re-audit the surface but to not weaken that
attribute** — removing or downgrading it is a hard stop, and adding `Member`
must leave the role type crate-internal.

**Adding a variant is a compiler-generated checklist.** `StandardOperatorRole`
is exhaustively matched with no `_ =>` arm at every consumer (`COORDINATION
§7`), so the new variant reds at each site that must handle it. **A candidate
that reaches green by adding a catch-all arm anywhere has defeated the
checklist** — that is a hard stop, not a style point.

**Nothing else in this frame waits on A1.** `§3` is entirely landed-spec.

**Endorsed, Architect 2026-09-17 (`evt_4d6jab0wj9mjb`): pinning these at release
rather than at framing is *"the difference between a frame and a guess"*** — A1
took three amendments in one day, each moving where an identity lives or how the
elaborator acquires it. **The facade's path is left free deliberately**, not
pending: a self-declaring home would move it without touching any layer. Layer 2
has landed at `48662cbdc`; layers 1 and 3 are being built now, with `AC-9`'s
role-present-wrong-shape case first so the contract cannot quietly degrade into
a presence test.

## 6. Acceptance

**AC-1 — `q ∈ c` elaborates to `membership_member_at` for all three views, and
the result is `Bool`.** Single left-to-right evaluation, no short-circuit.
**Name the provider resolved in each case**, not just that it type-checked.

**AC-2 — the binding is defined EXACTLY ONCE and `∈` republishes that identity.**
Count defining occurrences of `membership_member_at` across the catalog; it is
**one**. The facade `export`s it, and by `33 §4.3` that republishes the existing
`GlobalId` rather than minting a second. **State the count.** `39 §6.9` keys
completion on one defining `GlobalId`, so two definitions silently break
completion for every consumer.

**AC-3 — CARRIER-FIRST IS DEMONSTRATED BY A CASE THAT DISCRIMINATES, and you
must construct it.** Two providers accepting **the same `Query` type over
different containers**, with `q ∈ c` resolving by `c`'s head.

**An LHS-first implementation passes every test where the query type is
unambiguous** (`§3d`), so the ordinary cases are not evidence for this AC and
must not be offered as such. **The discriminating fixture is the AC.** If the
catalog's three views cannot produce two same-`Query` providers, construct the
pair in a test fixture and say that is what you did.

**AC-4 — the comparator is the view's own, and the wrong one is DETECTABLE.**
Per `§3b`: an implementation resolving a fresh canonical `Ord` at the use site
is non-conforming. **The control is a fixture where a second, different `Ord` is
in scope and the answer differs under it** — a tree `Ordered` under one
comparator, queried where another is resolvable. **Passing with only one `Ord`
in scope is vacuous**: both implementations agree there, so that run
distinguishes nothing.

**AC-5 — `∈` IS NOT A BUILTIN, and the evidence is structural rather than
behavioural.** Per `§3e` this cannot be shown by any behavioural test, so:

    (a) membership_member_at resolves to a CATALOG GlobalId at the use site --
        name it, from the elaborated form, not from the source text.
    (b) REMOVING the catalog binding makes the REQUIRED-ROLES CHECK HARD-ERROR,
        NAMING THE UNFILLED ROLE, so `q ∈ c` cannot elaborate. If it still
        elaborates, there is a builtin path and this AC is refuted.
    (c) the elaborator carries NO membership-specific lowering -- `∈` reaches
        the same D1 completion path as the other five roles.

**(b) is the load-bearing one and its evidence must be manufactured.** A green
suite is consistent with both a correct binding and a builtin shadowing it.

**It must fail AT THE REQUIRED-ROLES CHECK, naming the role — not merely fail.**
Architect, 2026-09-17 (`evt_4d6jab0wj9mjb`): under the three-layer ruling the
compiler owns **roles, never meanings**, and acquisition runs through the
catalog's own re-export, so **`AC-5` is discharged structurally by layer 3 and
that is on the record now rather than argued at review.** A bare "it stopped
compiling" is the weaker observation and does not distinguish a structural
discharge from an incidental resolution failure — *"the mechanism has to make it
unrepresentable rather than merely untested."*

**AC-6 — missing/ambiguous provider is an ORDINARY instance error at the
occurrence.** Per `§3d`/`39 §6.7`. Three cases, each with its diagnostic quoted:
no provider; two canonical providers for one head; a container with no admitted
provider. **Never a fallback to a different meaning, never silent acceptance.**

**AC-7 — `Membership Tree` is REFUSED.** Per `§3b`. Installing it is forbidden;
show the refusal and its diagnostic. **Positive control: show that the
ordered-key view over the same underlying `Tree` IS admitted**, so a green AC-7
is distinguishable from a rule that refuses everything.

**AC-8 — no Prop-to-Bool elimination.** `member` is `Bool`-valued and
`member_holds := IsTrue(member)` goes the other way. **Show no `Ω`-to-`Bool`
elimination was introduced.**

**AC-9 — `trusted_base()` delta is ZERO.** The Architect has ruled no new TCB
entry is needed. If this node appears to need one, that is a hard stop.

## 7. Base

Cut from a `main` containing A1's landed candidate. **Pin a literal SHA at the
moment you adopt it, never the ref.**

**PINNED AT RELEASE: `64d8aa755f505098c34a3caa8293922d9f5b0698`.** That tip
contains A1 (`e2e40e2b`) and the second hold's squash (`564e9e2cd`), whose M7
closeout it is. Adopt it or anything later that still contains both; re-pin
literally if you cut later, and **never carry `origin/main` as the base** — it
moves under you, and three lanes are landing into it tonight.

## 8. Contention

**A1 is the live contention AND the dependency, which is why this node is held
rather than coordinated.** It is actively authoring the standard-operator
facade and the `ken-elaborator` completion path — exactly the two surfaces
`§5` items 5 and 6 touch. **Sequencing resolves it; there is nothing to
negotiate.**

Second surface, `catalog/`. **Re-measured at release; the hard-wall sentence
above is SPENT and its date has passed.** Last landed touches at
`64d8aa755`:

    catalog/                 e2e40e2b4   2026-09-19   LANG-STANDARD-INFIX-CALL-COMPLETION
    crates/ken-elaborator/   564e9e2cd   2026-09-20   LANG-CORE-INSTANCE-HEAD-MATCH

> **FOUNDATION IS LIVE ON `catalog/` RIGHT NOW — the opposite of the framing-time
> situation.** L3 is respinning `CAT-PARSING-CURSOR-LAWS` against a CI red. Its
> surface is `catalog/packages/Capability/Parsing/Cursor.ken.md` plus
> `crates/ken-elaborator/tests/cat_tier_d_cursor_import.rs`. **File-disjoint from
> yours** — your class lands beside `LawfulClasses.ken.md` in
> `catalog/packages/Core/Classes/`, and `Membership` appears in no catalog module
> today. Coordinate rather than wait, and **do not touch Parsing.**

> # THE SHARED INVENTORY THAT WILL RED ON YOU, and it is NOT in `§6`.
>
> **Adding any catalog module changes a whole-catalog census, and L3 lost a
> full CI cycle to exactly this an hour before you were released.**
> `catalog_ambient_passthrough_migration_census`
> (`crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs:369`) walks
> every catalog leaf and asserts
>
>     partition(ambient, clean, residual) == discovered
>
> **Your new modules enter `discovered` the moment they exist, so this test reds
> until they are placed in one of the three lists.** That much is a MIRRORING
> update and you are authorized to make it.
>
> **WHICH list is not bookkeeping, and this is the part to get right.** The
> `census` list is **migration debt** — its doc says *"every remaining name
> still requires an explicit provider migration."* **Aim for `clean`**, which
> means importing every name you use explicitly rather than taking it ambiently.
> The concrete trap, because it is the one that just fired: `IsTrue` is
> `pub fn IsTrue` at `Core/Classes/LawfulClasses.ken.md:54` and must be **named
> in an import** — `Data/Numeric/Nat/Order.ken.md:37` does exactly that, uses it
> eighteen times, and stays out of the census. You will use `IsTrue`: `§5` item
> 4 defines `member_holds := IsTrue(member)`.
>
> **If a module of yours cannot reach `clean`, stop and say which name and
> why.** Adding an ambient entry is recording new debt against a migration
> program, and it is a Steward decision, not a test edit. Full reasoning:
> `CAT-PARSING-CURSOR-LAWS` `AC-3c`, Steward ruling `evt_5f4qhhwqk1hye`.

> **A branch scan is the WRONG instrument here and reports merged nodes as
> live.** `git diff origin/main...<branch>` against a squash-merged branch still
> shows that branch's old changes, because the merge-base predates the squash.
> Run at framing time it returned dozens of `wp/` branches "touching" catalog —
> nearly all already landed. **Ask the last landed touch and the live ring, not
> the branch list.**

## 9. Not this node

- **A1's completion adapter itself** — [[LANG-STANDARD-INFIX-CALL-COMPLETION]].
  This node is its sixth consumer, not a second dispatcher.
- **The `∈` name/fixity admission** — [[LANG-RESERVED-INFIX-NAMES]], landed.
- **Any ASCII form of `∈`.** It is **glyph-only** (A0), so there is no
  `let ... in` collision to resolve and no standard `in` alias. **The A0/A1
  ASCII-role material in the node's pre-2026-09-13 body is SUBSUMED** — it
  belongs to the superseded reading and must not be revived.
- **The formatter's `§1b` notation row** — spec's, discharged separately.
- **A `Set` carrier.** The ordered-key view at `v = Unit` serves set membership
  and mints no second head (`§3b`).
- **Any kernel change.**

## 10. Related

- [[LANG-STANDARD-INFIX-CALL-COMPLETION]] — A1, the hold. Its `D1a` reserves
  `∈`'s role, its `D2` facade is the surface `∈` attaches to, and its `D3`
  resolver is what this node reuses. Its `FI-4` states that
  `membership_member_at` is deliberately **not** authored there.
- [[SPEC-MEMBERSHIP-CLASS-CONTRACT]] — `58b`, the class and provider contract.
  Landed; its node's `active` status is unflipped M7.
- [[SPEC-TYPE-PROJECTION-SURFACE-NORMATIVE]] — specifies the projection form
  `d.Query` relies on, and discharges the `unspellable` clause at four sites.
- [[LANG-TYPE-PROJECTION-SURFACE-FORM]] — the landed implementation,
  `294cb5e28843dff8e2edeae9945e1cdc20b0318f`.
- [[LANG-RESERVED-INFIX-NAMES]] — A0, admits `∈` as a name.
