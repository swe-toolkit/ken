# WP frame — `SPEC-MEMBERSHIP-CLASS-CONTRACT`

**Owner:** Spec enclave · **Size:** M · **Risk:** medium (a new class shape plus
three carrier views; the coherence argument is where review turns) ·
**Tier:** T1 · **Gate:** none · **Deps:** [[SPEC-RESERVED-INFIX-NAMES]] —
**merged**

**Measured at:** `origin/main` `ae890874c391aa394f947a4e58fd3e3f5c5ce2e4`.
A **RECORD** of where §2's coordinates were taken, not a base to cut from.

**Origin:** Steward frames 2026-09-16 at @spec-leader's ask (`evt_7crxbr742ddpm`)
— *"please frame B-track now, in parallel with A-track's authoring, so it's
shovel-ready the moment the ring clears A-track."* Design content is the node's;
this frame supplies verified coordinates, the sequencing, and the D0.

> # THE NODE IS THE DESIGN. THIS FRAME IS ITS COORDINATES.
>
> `docs/program/issues/SPEC-MEMBERSHIP-CLASS-CONTRACT.md` already carries the
> class shape, the three views, the two-tier law model, the completion policy,
> the deliverables and the acceptance criteria — and they are not re-derived
> here. **Read the node first; this frame is what the node itself asks for**
> when it says *"re-measure every cited anchor at the cut."*
>
> **That instruction was not ceremony. Two of its code anchors had moved.**

> # KICKED (`evt_72kh7zbht4pa2`). D0-1 RULED IN FULL — §2c STANDS AS WRITTEN.
>
> **Read this banner before §2c, and read it as the current state. D0-1 flipped
> twice within forty minutes and the frame text below it never moved.**
>
>     D0-1 class shape     ADOPTED    unary, associated `Query`            evt_6y241yhbamnhf
>     D0-1 binding shape   held ...   (superseded -- do not act on it)     evt_6y241yhbamnhf
>       ... then RULED     MINT IT    §2c is RIGHT as originally written   evt_4tmt0n7era4w3
>     SameMembers law      CLEAR      state it as a CLASS FIELD            evt_34fgmaq13mz07
>     universe level       RULED      by the ENCLAVE; NOT open, NOT new    evt_3krx0gyr74t5h
>     three AC errata      STAND      repaired below                       evt_61hkytdx8hwkk
>
> **THE HOLD IS LIFTED. Draft the completion policy.** Key it on
> `membership_member_at`'s `GlobalId`, in the `ord_leq_at` shape, exactly as
> §2c says.
>
> **The reason the hold lifted is worth more than the verdict: UNSPELLABLE IS
> NOT UNCLOSABLE.** The kernel has projections (`term.rs:301` `Proj1`, `:303`
> `Proj2`) and a class is a right-nested Σ (`33 §5.2`), so with `Query` as field
> 0, `d.Query` is just `Proj1(d)` and the telescope
>
>     [ c : Type,  d : Membership c,  Proj1(d),  c ]
>
> is well-formed and kernel-checkable — **`d.Query` is closed by the binding's
> own earlier parameter `d`.** That is ordinary dependency, exactly like
> `(a : Type) (x : a)`.
>
> > **THE TWO CASES ARE DIFFERENT OBSTRUCTIONS, and blurring them is what
> > produced the retracted hold** (Architect, `evt_4tmt0n7era4w3`):
> >
> >     class FIELD as the key   field_types[i] closes only once a dictionary is
> >                              RESOLVED -- and resolution is what completion is
> >                              DOING. CIRCULAR. Stays rejected.
> >     minted BINDING           Proj1(d) is closed by parameter `d`, which the
> >                              binding itself binds. NOT circular. Checkable today.
> >
> > ⇒ **`∈` is the SHARPEST CASE for the mint rule, not a counterexample to
> > it** — minting is precisely what converts a *field-relative* type into a
> > *parameter-relative* one. §2c-i's rejection of the `(type_id, field)` route
> > stands on its original reason.
>
> **What IS missing is surface syntax, and it lands on the BUILD node.** The
> kernel can express the telescope and this contract can state it; `RType` has
> no projection form, so the *catalog* binding cannot be written yet. That is a
> prerequisite of [[LANG-MEMBERSHIP-OPERATOR-SURFACE]], not of this contract —
> B-track specifies a binding, it does not author catalog code. **A node for
> that surface prerequisite is cut: [[LANG-TYPE-PROJECTION-SURFACE-FORM]].**
>
> **Paste-ready clause the contract must carry** (Architect, verbatim):
>
> > `membership_member_at`'s signature binds the query at `d.Query` — a
> > projection from an earlier parameter. The kernel admits this (the telescope
> > is closed by `d`), and this contract specifies it at that level. **Ken's
> > surface type grammar has no projection form** (`RType`, `resolve.rs`), so
> > the catalog binding cannot be written until one exists. That is a
> > prerequisite of `LANG-MEMBERSHIP-OPERATOR-SURFACE`, not of this contract,
> > and it must not be discharged by making `∈` an elaborator builtin:
> > `33 §6.1` requires the standard meanings to be **ordinary top-level
> > bindings**, and a builtin would break the precondition A-track's completion
> > policy rests on (`39 §6.9`, and `SPEC-STANDARD-INFIX-BINDING §2f`).
>
> **That last clause is the load-bearing one.** The tempting escape from a
> missing surface form is to special-case the operator in the elaborator — and
> that would quietly falsify the premise the whole A-track policy rests on, in a
> way **no test on this node would catch.**
>
> **Two questions remain OPEN and a contract silent on either is wrong in a way
> nothing checks:**
>
> - **UNMEASURED, and do not cite the D0-1 ruling as closing it:** whether the
>   instance registry and `resolve_instance_dictionary` handle a class at
>   non-zero level. `type_id: GlobalId` plus `Term::Const{level_args}` suggests
>   yes; nobody has run it.
>
> **THE UNIVERSE LEVEL IS *NOT* OPEN — THE ENCLAVE RULED IT (`evt_3krx0gyr74t5h`).**
> An earlier draft of this banner listed it as open. That was wrong at the time
> it was written, not merely overtaken: the Architect routed the question to the
> enclave (`evt_475nxrksyyxk9`) **and the enclave had already answered it.**
>
>     Membership : Type ℓ → Type (suc ℓ)   is `33 §5.2`'s class former OPERATING,
>                                          NOT new mechanism.
>
> `§5.2` fixes no level, and `§5.1` keys the property/structure discriminant on
> the record's kernel-computed sort **family**, not on a level. What is actually
> wrong is **`33 §5`'s introductory `C : Type → Type`** (`33-declarations.md:404`,
> Steward-verified at `24e9ce039`) — **a motivating description, already exceeded
> on the parameter axis before `Membership` existed.** The Architect's own
> ten-hit sweep found nine parameter-kind statements and exactly one restrictive
> assertion, which is that sentence. Implemented in the author's candidate
> `17f5d22e161fd76171440854f72dde41ddf545f3`: the level stated explicitly with
> the computation inline, plus a bounded clarification to `33 §5`.
>
> ⇒ **A candidate that decides this is NOT over-reaching**, and a reviewer
> reading "open" here would be right to flag it on that text. That is why this
> correction is worth a frame touch rather than a note: **the frame's word
> carries the Steward's authority against the author's own ruling.**
>
> **The Steward's kick presented the mint shape as settled while it was
> momentarily held** (corrected, `evt_61g2pr7pj02rt`), **and that correction is
> itself now superseded by `evt_4tmt0n7era4w3`.** The frame's original §2c text
> was right throughout. Net: author against §2c as written.

---

## 1. Objective

Specify the standard meaning of `∈`: a `Membership` typeclass, its three
standard witness-bound carrier views, the use-site completion that dispatches
`q ∈ c`, and the two-tier law model — reusing the resolver A1 factors rather
than adding a second operator dispatcher.

## 2. The anchors, re-measured. Two moved; every catalog and spec anchor held.

The node's anchors were verified at `5d1347aaf`. At
`ae890874c391aa394f947a4e58fd3e3f5c5ce2e4`:

    anchor                                node says      now        verdict
    ast.rs ClassDecl{param,param_kind}    :381-390       :381,:386  HELD
    parser.rs parse_class_decl            :1131-1188     :1152      MOVED
    classes.rs canonical-instance map     :202           :196       MOVED
    classes.rs instance_search            :330-333       :330-331   HELD
    elab.rs resolve_instance_dictionary   :9598-9623     :9598      HELD
    OrderedSearch.ken.md elem             :32            :32        HELD
    Map.ken.md member                     :142-146       :142       HELD
    Map.ken.md set_member                 :169-170       :169       HELD
    Map.ken.md rel_member                 :15162-15165   :15162     HELD
    LawfulClasses.ken.md IsTrue           :54            :54        HELD
    33-declarations.md §5.4               :567           :567       HELD
    39-elaboration.md implicit insertion  :33, :450      :33, :450  HELD
    52-map.md §2 / §2.1 / §5.1            :90/:118/:306  same       HELD
    58-maps-sets-relations.md §5 / §7     :281 / :344    same       HELD

### 2a. `parser.rs` will move AGAIN before you reach it. Cite by symbol.

`crates/ken-elaborator/src/parser.rs` is **under active edit in lane 2 right
now** — [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] is rewriting its
postfix/argument loop, and [[LANG-BARE-OPERATOR-ATOM-REJECTION]] narrows the
operator-name atom after it. `parse_class_decl` has already drifted 21 lines and
is the anchor most likely to be wrong by the time you read it.

⇒ **Cite by SYMBOL, verify by line.** `git grep -n 'fn parse_class_decl'`
survives every edit above it; `parser.rs:1131` did not survive one day. The same
holds for `classes.rs:202`, which moved six lines for reasons nobody recorded.
Line numbers are a convenience for the reader and **never** the citation.

### 2b. The substrate facts the contract binds, restated because they are the argument

- **The class surface is unary.** `ClassDecl { param: Option<String>,
  param_kind: Option<Type> }` — one parameter, and `param_kind` is the only
  kind slot. A multi-parameter class is not a spelling choice; it is absent
  machinery.
- **The instance registry keys on `(class_name, head_type_name)`** — a `&str`
  pair, `instance_search(&self, class_name: &str, head_name: &str)`. **This one
  fact is the whole reason the three views must be distinct nominal heads**:
  three roles over a raw `Tree` collide on one key, and the collision is at the
  registry, not at elaboration, so no amount of use-site cleverness separates
  them.
- **The workers exist and are Bool-valued.** `elem`, `member`, `set_member`
  (`v := Unit`), `rel_member`. The class `member` adapts them; it does not
  replace them, and `rel_member`'s underlying checked Bool computation is what
  gets reused — not an elimination from its `Prop` view.

### 2c. RULING — mint `membership_member_at`. Node line 102 is STRUCK.

The node says the binding is *"to the defining **class-method GlobalId** +
checked telescope"*. **Strike that clause: it names something that will not
exist.** `member` is a field name in `field_names: Vec<String>`, exactly as
`leq` is; a class method has no `GlobalId` (`classes.rs:36/:44/:49/:149`).

**The ground is better than that framing suggests. `class Membership` does not
exist on `main` at all** (Architect, `evt_6dyqef1q4t6av`; Steward-verified at
`1edc1afea`):

    git grep -l 'class Membership' -- catalog/ spec/    ->  (empty)
    control: pub class Ord                              ->  LawfulClasses.ken.md:119
    git grep -l 'membership_member_at' -- catalog/ spec/ -> (empty)

So B-track is **not** an existing class missing a wrapper. It authors the whole
thing, and the A-track shape is therefore available for free rather than needing
a retrofit.

⇒ **B-track mints `membership_member_at` as a top-level binding in the
`ord_leq_at` shape, and the contract keys on its `GlobalId`.**

> ### THIS RULING STANDS AS WRITTEN. A hold on it was raised and RETRACTED.
>
> **Do not act on `evt_6y241yhbamnhf`'s hold — it is superseded by
> `evt_4tmt0n7era4w3`.** It is preserved here only because the measurement
> underneath it is real and the contract must carry it: **the surface obstruction
> is genuine, and it lands on [[LANG-TYPE-PROJECTION-SURFACE-FORM]] and
> [[LANG-MEMBERSHIP-OPERATOR-SURFACE]], not on this node.** The top banner has
> the full current state and the paste-ready clause.
>
> **The measurement, which survives the retraction.** The rule was derived on
> `≤`, where the class parameter closes every argument type. `∈` is the first
> operator whose query type comes from the **dictionary**:
>
>     ord_leq_at           (a : Type) (d : Ord a)        (x : a)       (y : a)  : Bool
>     membership_member_at (c : Type) (d : Membership c) (q : d.Query) (x : c)  : Bool
>                                                             ^^^^^^^ a PROJECTION
>                                                                     from an earlier
>                                                                     parameter
>
> **`d.Query` cannot be spelled in a type position in Ken today** (Architect,
> measured):
>
>     resolve.rs:390   RType has ELEVEN variants, NO projection
>     git grep 'RType::RProj' -- crates/                          0 hits
>     projection exists only as RExpr::RProj (elab.rs:8547) -- the EXPRESSION category
>     catalog bindings with a parameter typed by a projection from an
>       earlier parameter                                         0
>     catalog classes with a field at the universe `Type`         0
>       (control: the scan reads `class Ord a`'s five field lines correctly, and
>        correctly EXCLUDES `Source`'s `source_utf8_field : IsUtf8 source_bytes_field`,
>        which is dependency on an earlier field's VALUE -- a different thing)
>
> **And one repair does NOT work — keep it, because it is the one an author
> reaches for.** Making the query type an ordinary parameter —
> `(c : Type) (q : Type) (d : Membership c) (x : q) (y : c) : Bool = d.member x y`
> — is well-formed as a signature and then fails **in the body**: it needs
> `q ≡ d.Query` definitionally, and with `d` abstract `d.Query` is **stuck**. It
> checks at every concrete instance, where `d.Query` reduces, and **not at the
> polymorphic binding — which is the only place a `GlobalId` for completion to
> key on can exist.** So the query type must stay at `d.Query`; the answer is
> not to weaken the signature.
>
> ⇒ **WHERE THE OBSTRUCTION STOPS, which is the retraction's whole content:**
>
>     kernel    the telescope is expressible and checkable        AVAILABLE
>     spec      this contract can state it                        AVAILABLE
>     surface   `RType` cannot spell `d.Query`                    MISSING
>
> **Only the third line is a gap, and it is not this node's.** The hold came
> from reading the third line as if it were the first. §1's *"already
> expressible by Ken's class representation"* establishes **dependency, not
> sort** — that observation was correct and is simply about a different
> question.

> ### THE TWO RULINGS ARE NOT IN TENSION. The rule is ONE MEANING, ONE IDENTITY.
>
> Read alone they look opposed — A-track was told **do not define**, B-track is
> told **define**. They are the same rule applied to different ground, and
> stating that here is what stops a later reader from "harmonising" them:
>
>     A-track   the binding EXISTS      -> RE-EXPORT. A second definition would
>                                          give one meaning two identities.
>     B-track   NO binding exists       -> MINT ONE. There is no identity to
>                                          re-export, and projecting a field
>                                          directly leaves nothing to key on.
>
> The governing rule is **not** "always re-export". `33 §4.3` is why the
> A-track half works: `:364` *"republishes the existing `GlobalId` and never
> mints another one"*; `:375` *"A same-shaped source declaration has a different
> identity."* Both lines Steward-verified at `1edc1afea`.

### 2c-i. The rejected alternative — and the obstruction is in the TELESCOPE, not the identity

**State this in the contract.** Keying the completion on the class plus its
field, rather than on a minted binding, is what a careful author reaches for,
and **the obvious objection to it is wrong.**

    (class type_id, field INDEX)   rejected: an index is POSITIONAL -- adding a
                                   field before `member` silently re-points it
    (class type_id, field NAME)    a name is NOT positional. This variant
                                   SURVIVES that objection entirely.

The name variant also survives the next two: the class `type_id` travels under
re-export by `33 §4.3` exactly as a binding's identity does, and a user's own
class with a field named `member` carries a **different** `type_id`, so the pair
discriminates. **The stability argument does not decide this fork** (Architect,
`evt_2yne8tkxk84xf`, correcting the reason given in `evt_6dyqef1q4t6av`).

> ### THE DECIDING REASON — a class field has no CLOSED TELESCOPE.
>
> The completion policy keys on a `GlobalId` **and its checked telescope**.
> Steward-verified at `1edc1afea`, `crates/ken-elaborator/src/classes.rs:37-44`:
>
>     field_types[i] is a kernel Term valid in context
>       [a?, field_types[0], ..., field_types[i-1]]
>     (the class param, if any, then every EARLIER field's type)
>
> **A field's type is meaningful only relative to the class parameter and every
> field before it.** It is not a closed telescope; it becomes one only after a
> dictionary is resolved and substituted. ⇒ **The shape completion must supply
> is not knowable at the point the policy says to supply it.**
>
> Both framings of this fork looked at the **identity** half, where the name
> variant does fine. The obstruction is in the **telescope** half, where it has
> no referent at all.

**The second reason, independently held:** it would make *"completion"* denote
two different acts — supply an omitted **prefix of arguments** on A-track,
synthesize a **projection out of a resolved dictionary** on B-track — for one
policy stated once and implemented once by A1. Subsume-don't-proliferate.

**A boundary, stated rather than hidden:** whether the name variant survives
instance resolution end-to-end was **not** checked. The telescope obstruction
decides the fork without it, so that question is **open, not answered** — do
not cite this frame as having closed it.

> ### THE TELESCOPE ARGUMENT DOES **NOT** CUT BOTH WAYS. The alternative stays rejected.
>
> **A "cuts both ways" reading was raised (`evt_6y241yhbamnhf`) and RETRACTED
> (`evt_4tmt0n7era4w3`); it is recorded here because it is the natural misreading
> and it fooled the ruling's own author for half an hour.** The reading was:
> if no closed telescope can be written for `∈` at all, then *"a class field has
> no closed telescope"* stops discriminating between the two routes.
>
> **It fails because the two are DIFFERENT obstructions:**
>
>     class FIELD as the key   field_types[i] closes only once a dictionary is
>                              RESOLVED -- and resolution is what completion IS.
>                              CIRCULAR. Stays rejected.
>     minted BINDING           Proj1(d) is closed by parameter `d`, which the
>                              binding itself BINDS. Not circular. Checkable today,
>                              at the kernel; only the SURFACE cannot spell it.
>
> ⇒ **Minting is precisely what converts a field-relative type into a
> parameter-relative one, so `∈` is the SHARPEST case for the mint rule.** The
> rejection below stands on its original reason, and the second, independently
> held reason — that the alternative makes "completion" denote two different
> acts — never depended on this at all.

> ### THE DURABLE FORM. It governs every later operator, not just `∈`.
>
> **An operator whose standard meaning is a class method requires a top-level
> binding minted for it.** The completion policy keys on a closed telescope; a
> class field never has one, because its type is well-formed only relative to
> the fields before it.
>
> ⇒ **`ord_leq_at` is not a convenience wrapper around `d.leq` — it is the thing
> that makes `≤` completable at all.** Any `∈`, `⊆`, `≈`, or later operator in
> the same position needs its own. (Architect, `evt_2yne8tkxk84xf`.)

## 3. D0 — answer before writing normative text

**D0-1. Does the Architect adopt the advisory's unary associated-`Query`
shape?** **RULED IN FULL — ADOPTED on the class shape (`evt_6y241yhbamnhf`) and
MINT on the binding shape (`evt_4tmt0n7era4w3`).** Do not re-route it.

**ADOPTED: unary with an associated `Query`, not multi-parameter.** The
Architect's reason is stronger than the node's *"buys no semantic win"*: **the
registry keys on one outermost head, so a second class parameter has nowhere to
live**, and `Query`-as-a-field is what lets the telescope carry it. The kernel
admits the record — `check.rs` `sort_sigma(s1,s2) -> Term::Type(max(s1.level,
s2.level).normalize())`; `Omega` only when **both** components are
propositions, and there is **no rejection path**. A field at the universe
`Type` simply raises the record's level.

> **THE CONSEQUENCE IS THE PART TO BUILD ON, and it collapses two design
> questions into one.** A type-sorted field makes `Membership` a **STRUCTURE
> class** by the elaborator's own rule (`classes.rs:14-24`: *"at least one field
> is Type-sorted"* → canonical-one-per-head). **That one-per-head rule is
> exactly what forces §3's nominal carrier views and forbids `Membership
> Tree`.** The class shape and the carrier design are the **same decision**, not
> two that must be kept consistent.

**RULED on the binding shape too: MINT IT** (`evt_4tmt0n7era4w3`, superseding a
hold in `evt_6y241yhbamnhf`). Author the completion policy normally, keyed on
`membership_member_at`'s `GlobalId` in the `ord_leq_at` shape, and carry the
paste-ready surface-prerequisite clause from the top banner. **Nothing in this
node is held.**

**`SameMembers` is CLEAR — state it as a CLASS FIELD, not as a standalone over a
dictionary** (Architect, `evt_34fgmaq13mz07`). The node writes it as
`SameMembers d x y := (q : d.Query) -> ...`, which names `d.Query` in a type
position at a **use** site and is unspellable for the same reason the binding
is. As a field it is spellable today, because **inside the class body the
field's own name is in scope** — `class Ord a` already shows the shape
(`refl : (x : a) -> IsTrue (leq x x)` names `leq`, a sibling field). **The fix
is where the law LIVES, not what it SAYS.**

**The universe level is RULED, not open** (`evt_3krx0gyr74t5h`, the enclave's
own call, confirmed as theirs by the Architect). `Membership : Type ℓ → Type
(suc ℓ)` is `33 §5.2`'s class former operating, **not new mechanism** — so it
does not disturb this node's "no new mechanism" posture. See the top banner.

**Ruled is not the same as recorded — the contract must still STATE the
level.** `33 §5.1` ends *"the elaborator reads the sort off the emitted record;
it does not carry a separate kind tag"*, so **nothing will contradict a contract
that omits it**, and that is true of a ruled question exactly as it was of an
open one. The author's `17f5d22e161fd76171440854f72dde41ddf545f3` states it with
the computation inline. **That is the bar, not a courtesy.**

**Two first-of-its-kind facts worth stating in the normative text**, both
measured by the Architect: **0 catalog classes have a field at the universe
`Type`**, and **0 catalog bindings take a parameter typed by a projection from
an earlier parameter.** `Membership` would be the first of each.

**D0-2. Is `∈ infix 4` ruled, or merely proposed?** The node offers it as a
candidate aligning with `≤ ≥ ≠`. `SPEC-STANDARD-INFIX-BINDING` fixes that band
for the five A-track operators; **if A-track has landed a different band by the
time you draft, the alignment argument moves with it.** Re-read A-track's
landed §3c rather than this frame's account of it.

## 4. Acceptance

**The node's acceptance criteria are the ACs and are not restated here.** Three
additions, each with a control that can fail:

**AC-ANCHORS-REMEASURED.** Every coordinate the candidate cites resolves at the
candidate's own base. **Control:** re-run §2's table at the cut; a cited line
that does not contain the named symbol fails, and `parse_class_decl` is the row
expected to have moved. *(A citation that resolved when the frame was written is
not evidence it resolves now — that is the defect this AC exists for.)*

**AC-NO-SECOND-DISPATCHER.** Membership completes through the same resolver A1
factors. **Control:** the normative text reaches `resolve_instance_dictionary`'s
path by name; text describing a parallel membership-specific resolution route
fails, however small.

**AC-CARRIER-FIRST-INFERENCE.** `q ∈ c` infers the **RHS** carrier first,
resolves its dictionary, projects `d.Query`, then checks the LHS. **Control:**
the text must decide a case where LHS-first inference would pick a different
carrier — if no such case is stated, the criterion is unmeasured, not met.

**AC-NO-SHIPPED-CODE.** Spec + conformance only. **Control:**

```sh
# SUBSTITUTE THE LITERAL SHA YOU CUT FROM. Never `origin/main`, never `...`.
BASE=<the 40-char sha this candidate's branch was cut from>

# BASE must be a LITERAL sha, not a ref -- this is what makes
# "never origin/main" ENFORCEABLE instead of advisory.
test "$BASE" = "$(git rev-parse --verify --quiet "$BASE^{commit}")" \
  || { echo "BASE is a ref, not a literal sha"; exit 1; }

# GUARD -- a degenerate range passes the real check vacuously.
test "$BASE" != "$(git rev-parse HEAD)" || { echo "BASE==HEAD: vacuous"; exit 1; }
git merge-base --is-ancestor "$BASE" HEAD || { echo "BASE not an ancestor"; exit 1; }
test -n "$(git diff --name-only "$BASE" HEAD)" || { echo "empty range"; exit 1; }

git diff "$BASE" HEAD -- crates/        # MUST be empty
```

**A moving ref re-reads as a different claim every time `main` advances**, so
`origin/main` here would red this AC on someone else's `crates/` work — and
`...` collapses the base onto `HEAD` and passes vacuously.

**The guard is INLINE and not a citation, because the citation was the defect**
(Architect, `evt_6y241yhbamnhf`). It pointed at `§4a-pin`/`§4b` of
`ABI-S6-HS18-MAIN-BASED-CLOSURE.md` — a runtime frame a spec author has no
reason to open — **and the guard is the half that matters.** The warning above
covers `...` and does **not** cover the direct form: `BASE=$(git rev-parse
HEAD)` is neither an ellipsis nor a moving ref, and it passes vacuously.

**And the literal-sha test is what makes this section's OWN headline rule
mechanical** (Architect, `evt_mpjw61te40bm`). With `BASE=origin/main` and main
*behind* `HEAD`, the three range guards all pass and the check still runs
against a ref that re-reads differently every time main advances — **the exact
defect the section opens by naming, left advisory by the guards that follow
it.** Same reasoning that replaced the citation with a guard, applied one line
up.

**This AC shipped as a bare `<base>` placeholder in this frame's first cut.** A
placeholder is not neutral — it invites `origin/main`, which is the one
specifically wrong answer.

**AC-METHOD-NOT-KEYED — LIVE, and it matters MORE now, not less.** With the mint
instruction reinstated (`evt_4tmt0n7era4w3`), **this AC is the gate on the key.**
As it shipped on `main` it bars the field-*index* variant while admitting the
field-*name* one — so it is repaired below and the repaired text is what CV
checks against, whether or not a reader is looking at a patched frame.

> **AC-METHOD-NOT-KEYED.** The normative text designates exactly one construct
> as the completion key, and it is the `GlobalId` of `membership_member_at`. It
> must not designate a class method, or a (class identity, field) pair in any
> spelling — by index or by name — **as the key**.
>
> **Control — find the sentence that states what completion keys on, and judge
> that sentence.** This criterion is about what the text DESIGNATES, not about
> which words appear in it. **Two required statements name a (class, field)
> pair and NEITHER fails this AC:** the class declaration itself (`member :
> Query → container → Bool` is a field of `Membership`), and §2c-i's rejection,
> which this frame instructs the contract to carry. A text that names the pair
> only inside its rejection **passes**; a text missing that rejection fails
> §2c-i, not this AC.
>
> **The field-NAME variant is the one this control exists for.** It is
> positionally stable, travels correctly under re-export, and discriminates by
> `type_id`, so a reviewer checking only for a field *index* will pass it. What
> excludes it is §2c-i's telescope argument, not the key's stability.
>
> **Check this criterion against the CANDIDATE'S TEXT, not against the tree.**

**Three repairs, all the Architect's, and each was a criterion that could not do
its job** (`evt_6y241yhbamnhf`, `evt_mpjw61te40bm`):

1. **It barred the variant nobody proposed and admitted the one that was.** The
   old text excluded a `(type_id, field INDEX)` pair — while §2c-i, twenty lines
   earlier, establishes that the index variant is not the one that matters and
   the **NAME** variant survives every objection but the telescope. **A contract
   keying on `(type_id, field name)` passed the criterion as written.** The
   prose was right and the thing that can fail was keyed on the wrong object.
2. **Its control could not pass.** It demanded a binding `git grep` can find as
   a top-level `pub fn` — while **Not this node** assigns `membership_member_at`'s
   creation to [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] and §6 says spec-only, and
   §2c **measures that grep empty itself**. **The frame contained its own proof
   that the control fails.** Hence "against the text, not the tree."
3. **The first repair REPRODUCED the defect it was repairing** (Architect, on
   review of `350e0de51`). Fixing the criterion's *object* — index → any
   designation — left the *control* keyed on **word presence**, while §2c-i
   instructs the contract to **state** the rejected pair and the class
   declaration **names** the field. **So the contract that best satisfies this
   criterion was the one most certain to trip its control.** Gate on what the
   text CLAIMS, never on which words appear in it.

## 5. Sequencing — SEQUENCED behind A-track, by the enclave's own ruling

@spec-leader, `evt_7crxbr742ddpm`: *"sequence them, don't take together... one
agent active at a time per COORDINATION §1; running two spec WPs in parallel
doesn't buy real throughput, it just splits attention across the same three
seats."*

    A-track  SPEC-STANDARD-INFIX-BINDING     kicked, evt_46jy81vcw530d
    B-track  this node                       kicked when A-track reaches CV

**This frame exists NOW precisely so the enclave does not idle at that
handoff** — which is exactly how A-track came to sit on a met precondition with
nobody watching. Framing ahead is not starting ahead: **do not open a branch for
this node before its kick.**

**The node's `origin:` says Pat authorized B-track to run in PARALLEL with
A-track.** That was about the *design* work being independent of A0's name
admission, and the enclave's sequencing ruling is about *seat contention* — they
do not conflict. If anyone reads them as conflicting, the operator's directive
governs and it is mine to escalate, not the ring's to resolve.

## 6. Contention

`spec/30-surface/` + `spec/50-stdlib/`. **No `crates/` paths and no cross-lane
contention** — lane 1 is on `crates/`, lane 3 on `catalog/`.

The `parser.rs`/`classes.rs` anchors in §2 are **read, not written**. A read is
not a dependency: this node does not wait on lane 2, it only re-measures at its
cut.

## 7. Related

- [[SPEC-STANDARD-INFIX-BINDING]] — A-track; binds `∧ ∨ ≤ ≥ ≠` and specifies
  the shared completion mechanism this node reuses rather than respecifies.
- [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] — the build, held on this contract.
- [[LANG-STANDARD-INFIX-CALL-COMPLETION]] — A1; factors the resolver.
- [[SPEC-RESERVED-INFIX-NAMES]] — merged; admitted `∈` as a glyph-only name
  with no standard meaning, which is the gap this node closes.
