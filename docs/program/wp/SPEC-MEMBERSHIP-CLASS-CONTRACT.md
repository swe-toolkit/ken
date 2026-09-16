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
shape?** The node states it as the recommended smallest sound shape and says the
Architect rules. **This is the first design step and it is not the spec-author's
to assume.** Route it before drafting §1 of the contract; a contract written
against the wrong class shape is a full rewrite, not an amendment.

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
git diff "$BASE" HEAD -- crates/        # MUST be empty
```

**A moving ref re-reads as a different claim every time `main` advances**, so
`origin/main` here would red this AC on someone else's `crates/` work — and
`...` collapses the base onto `HEAD` and passes vacuously. The worked form,
with the guard that catches a degenerate range, is `§4a-pin` and `§4b` of
`docs/program/wp/ABI-S6-HS18-MAIN-BASED-CLOSURE.md`.

**This AC shipped as a bare `<base>` placeholder in this frame's first cut.** A
placeholder is not neutral — it invites `origin/main`, which is the one
specifically wrong answer.

**AC-METHOD-NOT-KEYED.** The normative text keys completion on
`membership_member_at`'s `GlobalId`, never on a class method or a
`(type_id, field index)` pair. **Control:** the text must name a binding that
`git grep` can find as a top-level `pub fn`; text naming *"the `member` method
of `Membership`"* fails, and it fails while reading correctly — which is why
§2c states the alternative and its rejection rather than only the answer.

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
