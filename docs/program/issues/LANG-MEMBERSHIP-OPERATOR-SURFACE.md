---
id: LANG-MEMBERSHIP-OPERATOR-SURFACE
title: "the membership BUILD (re-cut 2026-09-13): define the Membership class and its three provider views (list; one ordered-key view serving Map and Set; relation-edge) in catalog and wire glyph-only ∈ elaboration to member-dispatch through A1's shared use-site resolver; ∈ is glyph-only so there is no let ... in ASCII collision and no standard in alias; HELD on SPEC-MEMBERSHIP-CLASS-CONTRACT + LANG-RESERVED-INFIX-NAMES + LANG-STANDARD-INFIX-CALL-COMPLETION, then the Steward frames full ACs and releases to the language ring"
status: ready
owner: language
size: L
gate: none
depends_on: [SPEC-MEMBERSHIP-CLASS-CONTRACT, LANG-RESERVED-INFIX-NAMES, LANG-STANDARD-INFIX-CALL-COMPLETION, LANG-TYPE-PROJECTION-SURFACE-FORM]
blocks: []
github: null
tier: T1
origin: "CONF-BLOCKER-MARKER-RECONCILE's D3, answered by the spec enclave with the citation its AC-4 demanded and corroborated independently by the conformance-validator (31-lexical.md:33-35, :79, :101-113). Steward ruling evt_bgat447r9s6w: this is an unowned surface gap, not a keyword-role decision -- the endpoint-(b) reading is refuted by citation. Steward-filed per COORDINATION §2. Supplies the blocker for seed-canonical-format.md:387 and FMT1's aggregate at :52. RECUT 2026-09-06 (Steward) on the Architect hard-stop ruling evt_356e6vfg2hrs6: the node is NOT buildable as framed -- AC-1's parse+elaborate has no honest semantic target -- so it is split, deferred, and returned to draft. RE-CUT AGAIN 2026-09-13 (Steward) on the operator directive (Pat, this session): the deferred split-(B) trigger has fired -- the typeclass-method-dispatch capability is now being cut for real demand (A1's factored resolver LANG-STANDARD-INFIX-CALL-COMPLETION + the SPEC-MEMBERSHIP-CLASS-CONTRACT contract), so this node is re-cut as the membership BUILD against them. Stays draft, HELD until its three deps land; then the Steward frames the full ACs and releases. See the 2026-09-13 banner."
---

> # RELEASED 2026-09-19. ALL FOUR DEPENDENCIES LANDED; `§5a` IS PINNED.
> # Frame: `docs/program/wp/LANG-MEMBERSHIP-OPERATOR-SURFACE.md`.
>
> **The hold was [[LANG-STANDARD-INFIX-CALL-COMPLETION]] (A1). It landed at
> `origin/main` `e2e40e2b404d9775b3cd1fee049b3ecaab481bba`**, verified in the
> tree on the `modules.rs` blob rather than read off a status field.
>
> **`§5a`'s four items are now pinned against A1's landed shape** — facade home
> `Core.Operators.Standard`, the `StandardOperatorRole` vocabulary, the
> `BINDING_BACKED` required-roles list with `expected_shape`/`shape_matches`,
> and `certify_roles` as the entry point. Read them there; they were measured
> out of the tree at that SHA, not off A1's approval posts.
>
> **Two things `§5a` settles that change what this node owes.** `D0-1`'s verify
> condition is enforced by `#![deny(private_interfaces)]`, so it is a compile
> error rather than an audit — the obligation is to not weaken it. And `D0-2` is
> largely answered in A1's landed code: `expected_shape`'s doc states the
> membership shape extends **without widening**, leaving this node the narrower
> measurement of whether `d.Query` is representable where the check runs.
>
> **The other three dependencies were verified landed at `4bc5f0eee`:** `58b`
> with `membership_member_at`, `Token::Member` in `lexer.rs` and `parser.rs`,
> and `LANG-TYPE-PROJECTION-SURFACE-FORM` at `294cb5e28`.
> `SPEC-MEMBERSHIP-CLASS-CONTRACT`'s `active` was an unflipped M7 and is
> corrected in this same commit, not a residual.
>
> **THE FRAME'S TWO INTERFACE QUESTIONS ARE ANSWERED, 2026-09-17, and they are
> answers rather than open D0s.** Raised to the language ring while A1 was still
> open, on the ground that they are cheap now and expensive afterwards; both
> ruled with no hard stop to A1's `§1a` chain.
>
> - **`D0-1` — A1's role vocabulary closes at FIVE; `∈` arrives with its
>   binding, and this node adds the variant.** Architect `evt_xx1v1qvksqke`,
>   **reversing** `evt_4d6jab0wj9mjb`. A1's fixities attach to the `GlobalId`s
>   its layer 3 certifies, so `∈` — having no binding — could not carry a fixity
>   in A1 either. **The Steward's premise for raising it was wrong and is
>   struck:** widening is crate-internal, an ordinary edit, and `COORDINATION §7`
>   makes it a compiler-generated checklist handed to this node. **One condition
>   this node must VERIFY at D0, because this node is where a failure surfaces:**
>   A1's role type must not leak through any `pub` signature.
> - **`D0-2` — `∈` satisfies `D1c`, whose shape contract is specified over the
>   ELABORATED TELESCOPE with back-references.** Ruled `evt_4d6jab0wj9mjb`,
>   confirmed `evt_xx1v1qvksqke` on the implementer's stronger measurement:
>   `ord_leq_at` already has three of four domains as de Bruijn references, so
>   **the contract is forced by `≤` before `∈` is mentioned.** What remains is a
>   **measurement** this node makes — whether `d.Query` is representable at the
>   point the check runs. Not representable ⇒ name it and stop.
>
> **DO NOT re-frame against A1's working tree.** A1 took three amendments on
> 2026-09-17 alone (FI-2a, FI-2b, FI-5), each moving where an identity lives or
> how the elaborator acquires it. A frame written against a surface that moved
> three times in one day is a frame written against a guess.
>
> **Two things the frame settles that this body does not:** `∈` must not become
> an elaborator builtin — `AC-5` gives that a structural control, because no
> behavioural test on this node would catch it — and carrier-first completion
> needs a fixture that *discriminates*, because an LHS-first implementation
> passes every case where the query type is unambiguous.
>
> # FOURTH DEPENDENCY, ADDED 2026-09-16 AND **DISCHARGED 2026-09-17**:
> # [[LANG-TYPE-PROJECTION-SURFACE-FORM]] IS ON `main`. THE SIGNATURES ARE NOW
> # SPELLABLE, AND THE BLOCKING PREMISE BELOW IS RETAINED ONLY AS HISTORY.
>
> **Architect `evt_4tmt0n7era4w3`, ruling `SPEC-MEMBERSHIP-CLASS-CONTRACT`'s
> D0-1.** The `∈` binding is `membership_member_at (c : Type) (d : Membership c)
> (q : d.Query) (x : c) : Bool` — **a parameter typed by a projection from an
> earlier parameter.** The kernel admits the telescope (`Proj1(d)` is closed by
> `d`) and the spec contract specifies it.
>
> **AS MEASURED AT `24e9ce039`, AND NO LONGER TRUE:** *"Ken's surface type
> grammar has no projection form — `RType` has eleven variants and none of them
> is a projection (`RType::RProj`, 0 hits; control `RExpr::RProj`, 4 files)."*
> **That measurement was correct when taken and is now false in every part.**
>
> ⇒ **`LANG-TYPE-PROJECTION-SURFACE-FORM` landed at `294cb5e28843dff8e2edeae9945e1cdc20b0318f`**
> (routed `ecd8fa875`, 9 files `+1052/-15`, blob-verified). Re-measured by the
> Steward at that SHA:
>
> ```text
>                          24e9ce039 / 03f2e65d0        origin/main 294cb5e28
>   RType variants         11, no projection            12, RProj PRESENT
>   RType::RProj           0 hits                       12 hits across 4 files
>                                                       (data 1, elab 7,
>                                                        modules 2, resolve 2)
>   control RExpr::RProj   4 files                      4 files  (unmoved)
> ```
>
> **THE CLOSING CLAIM — *"the catalog bindings this node must author are
> currently UNSPELLABLE"* — IS FALSE.** It was this node's blocking premise, so a
> seat picking the node up reads it as a live obstacle. It is not one.
>
> **A NEAR-MISS ON THE SUBJECT, worth keeping because the two enums are
> deliberately parallel.** `spec-author` flagged this premise (`evt_2ng4xxdcc62wx`)
> on the reading that *"the node says eleven and today's count is eleven, so a
> re-measure agrees and stops."* **That is true of `Type` in `ast.rs` (10 → 11,
> gaining `TProj`) and NOT of the enum this node names.** The premise is about
> `RType` in `resolve.rs`, which went **11 → 12**. The conclusion they reached is
> right and the route to it measured a different enum.
>
> ⇒ **The practical difference is the direction of the failure.** Their reading
> would make this a premise that survives a re-measure — the dangerous kind. On
> the actual subject it fails **loudly**: 11 against 12, and 0 hits against 12.
> **Anyone who re-measures the thing the sentence names disagrees with it
> immediately.** Recorded so the next reader does not inherit the quieter
> diagnosis and go looking for a silent-staleness problem that is not here.
>
> ## CORRECTED 2026-09-16 (Steward): THIS NODE AUTHORS **THREE** PROJECTION-TYPED
> ## BINDINGS, NOT ONE. The banner above said "the `∈` binding" and "its own
> ## signature", singular, and that was written before the contract landed.
>
> `SPEC-MEMBERSHIP-CLASS-CONTRACT` merged as `origin/main`
> `a63eebe3e7582c71a2c59624786a39705cf9a0e0` (candidate `9b9586dc0`, 6/6 blobs
> verified). `spec/50-stdlib/58b-membership.md:148-150` says it in the spec's own
> words: **three** projection-typed bindings — `membership_member_at`,
> `member_holds` and `same_members`.
>
>     membership_member_at (c) (d : Membership c) (q : d.Query) (x : c) : Bool
>     member_holds         (c) (d : Membership c) (q : d.Query) (x : c) : Ω
>     same_members         (c) (d : Membership c) (x : c) (y : c)       : Ω
>       same_members c d x y := (q : d.Query) -> Equal Bool (d.member q x)
>                                                           (d.member q y)
>
> ⇒ **`LANG-TYPE-PROJECTION-SURFACE-FORM` gates strictly more than this node
> previously said**, and a frame that satisfies only `membership_member_at`
> leaves two of three bindings unwritable.
>
> **`same_members` puts its projection in a different PLACE, and I checked
> whether that is a second grammar requirement. It is not.** Its signature
> carries no projection; the `d.Query` sits in the **body**, as the domain of a
> `Π` in an expression-position definition. That could have meant the surface
> fix needed two productions rather than one, and a fix scoped to parameter
> telescopes would then have satisfied two of three bindings and left this one
> unspellable. Measured at `a63eebe3e`, it is one production:
>
>     RType::RPi (String, Box<RType>, Box<RType>, Span)    domain is RType
>     RExpr::RPi (String, Box<RType>, Box<RExpr>, Span)    domain is RType
>
> **Both Π forms annotate their domain with an `RType`.** So all three bindings
> are unblocked by the same single change — a projection variant on `RType` —
> and the spec's *"writes `d.Query` in type position, exactly as `§2`'s
> `member_holds` does"* (`58b §4`) is exact rather than approximate.
>
> **This is recorded because it is the kind of thing that gets half-done.** The
> count (three, not one) is the correction; the one-production finding is what
> stops the correction from being read as "three separate surface problems".
> Neither is the language ring's to rediscover, and neither should be taken from
> this banner without re-measuring — `RType`'s shape is perishable.
>
> **And the obvious way around it is CLOSED:** `∈` must not become an elaborator
> builtin. `33 §6.1` requires standard meanings to be ordinary top-level
> bindings, and a builtin would falsify the precondition A-track's completion
> policy rests on (`39 §6.9`, `SPEC-STANDARD-INFIX-BINDING §2f`) — **in a way no
> test on this node would catch.** When this node is framed, that is a criterion
> with a control, not a note.

> # RE-CUT 2026-09-13 -- THE DEFERRED (B) TRIGGER HAS FIRED (operator directive,
> # Pat, this session: "Frame the L2 binop typeclass work to support membership").
> # This banner supersedes the 2026-09-06 defer banner below; that banner and the
> # original body are retained for history.
> #
> # The 2026-09-06 recut deferred split-(B) -- the membership VALUE operator --
> # "GATED on a membership/typeclass-dispatch capability that does not exist ...
> # When a typeclass-method-dispatch capability is cut for real demand, this node
> # is re-cut against it." That capability is now being cut, with operator demand:
> #  - the DISPATCH: A1 [[LANG-STANDARD-INFIX-CALL-COMPLETION]] factors a
> #    scoped/coherent use-site dictionary resolver "so later membership reuses the
> #    SAME resolver" -- membership is a CONSUMER of it, never a second dispatcher;
> #  - the CONTRACT: [[SPEC-MEMBERSHIP-CLASS-CONTRACT]] specifies the Membership
> #    class, the nominal witness-bound carrier views, the `∈` standard binding +
> #    completion, and the law model (grounded in the Research advisory
> #    thr_60s5rhqdh4ht8; Architect rules class/carrier);
> #  - the `∈` PARSE: A0 [[LANG-RESERVED-INFIX-NAMES]] admits `∈` as a name.
> #
> # SO THIS NODE IS RE-CUT as the membership BUILD: define the Membership class +
> # the three provider views (list; ordered-key serving Map and Set; relation-edge)
> # in catalog, and wire `∈` elaboration to member-dispatch through A1's resolver
> # (carrier inferred from the RHS first; witness/comparator bound in the view
> # value; no `Membership Tree`; no Prop-to-Bool elimination; missing/ambiguous
> # provider = normal instance error). Bool result; `member_holds := IsTrue(member)`.
> # depends_on the three nodes above; stays `draft`, HELD until all three land,
> # then the Steward frames the full ACs from the landed contract + A1 surface and
> # releases to the language ring. language-leader owner; Architect required
> # reviewer. CODE merge (catalog + crates/ken-elaborator) -> full CI + M8/M8a
> # Adversary on its candidate. The A0/A1 ASCII-role material below is SUBSUMED:
> # `∈` is glyph-only (A0), so there is no `let ... in` collision to resolve here.
> #
> # --- 2026-09-06 defer banner + original body retained below for history ---
>
> # RECUT 2026-09-06 -- NOT BUILDABLE AS FRAMED (Architect evt_356e6vfg2hrs6).
> # Hard stop #1 on this WP. The node is split, deferred, and returned to draft.
> # This banner supersedes the Deliverables and Acceptance criteria below; the
> # original body is retained for history.
> #
> # WHY IT IS NOT BUILDABLE. The Architect grounded this at origin/main 072dc688d
> # (the WP base) and verified: (1) the infix-operator surface is CLOSED -- BinOp
> # = {Add, WrappingAdd, Sub, Mul, EqEq} lowered through a closed numeric/structural
> # registry, with NO open, user-extensible typeclass method-dispatch for operators;
> # (2) membership is NOT one operation -- it is four distinct NAMED functions each
> # needing an explicit witness (elem needs Ord a; set_member needs leq; member;
> # rel_member), and there is NO unifying `class Membership` in the catalog;
> # (3) `∈` has ZERO uses in the entire .ken corpus -- a reserved dead token with no
> # consumer and no demand; (4) ASCII `in` is UNCONDITIONALLY the let-separator
> # keyword. So AC-1 ("`∈` parses AND elaborates") has nothing honest to elaborate
> # TO: a fixed lowering fabricates a witness out of nowhere (dishonest), and a
> # correct general `x ∈ s` requires type-directed class-method dispatch + witness
> # synthesis -- a MAJOR new language capability that does not exist. A parse-only
> # arm that always errors at elaboration is strictly worse than the current clean
> # dead-token state. Do NOT build it (honesty-about-the-boundary, docs/PRINCIPLES).
> #
> # THE SPLIT (Architect route §4; recut is the Steward's):
> #
> # (A) THE REAL DRIVER IS THE FORMATTER, and it is SPEC's, not this node's. The
> #     only actual demand is seed-canonical-format.md:387 BLOCKED-ON-MEMBERSHIP-
> #     ASCII-ROLE + FMT1:52 -- the formatter needs a COHERENT §1b notation row,
> #     NOT a working value operator. D2 resolved to frame option (2): the §1b:79
> #     row (`| ∈ | in | membership |`) is internally incoherent -- §1c/BL3's
> #     "ASCII accepted forever, identical token" guarantee cannot be satisfied for
> #     a glyph whose only natural ASCII form is a reserved keyword word, and the
> #     landed lexer already chose Token::Member DISTINCT from KwIn. The Architect
> #     recommends (spec-leader authors, evt_356e6vfg2hrs6): declare `∈` GLYPH-ONLY
> #     with a narrow documented exception to the total-ASCII-transliteration
> #     guarantee. That matches the landed lexer and §1a P4 exactly and discharges
> #     the formatter blocker with ZERO parser/elaborator work. This is now a SPEC
> #     obligation (spec-leader, in flight per the Architect's @mention) plus the
> #     already-planned CONF-BLOCKER-MARKER-RECONCILE marker reconciliation. It is
> #     NOT owned by this language node.
> #
> # (B) THE MEMBERSHIP VALUE OPERATOR (parse `∈` + elaborate) is what THIS node is
> #     recut to, and it is DEFERRED: 0 corpus demand, no semantic target, GATED on
> #     a membership/typeclass-dispatch capability that does not exist. No such
> #     capability node exists yet and none is minted speculatively for zero-demand
> #     work (ken-steward §4c) -- this node stays `draft` as the parking place for
> #     the obligation. When a typeclass-method-dispatch capability is cut for real
> #     demand, this node is re-cut against it; language-leader remains owner and
> #     the Architect remains required reviewer. Do NOT release it before then.
> #
> # LANGUAGE STOOD DOWN CORRECTLY (language-leader evt_wyj2aqmqs316): no source
> # edit, no implementer dispatch, nothing landed on the language side. The honest
> # increment is Spec's row amendment, then -- if demand ever appears -- a future
> # dispatch-capability node ahead of a re-cut (B).
> #
> # --- original body retained below for history (superseded by this banner) ---
>
> # THE ENDPOINT READING IS REFUTED BY CITATION, NOT BY PREFERENCE.
>
> The natural conclusion from the tree alone is that ASCII `in` is simply
> committed to the keyword role and membership is glyph-only — a settled
> endpoint, like the level token in [[CONF-FMT8-LEVELTOK]]. **The spec says
> otherwise, in the general rule and in the specific table row**, and the
> enclave produced both citations rather than an opinion. Do not re-litigate
> the endpoint; measure against the cited requirement.

## The citations, which are the premise

`spec/30-surface/31-lexical.md:79` — the notation table:

| glyph | ASCII | role |
|---|---|---|
| `∈` | `in` | membership |

`31-lexical.md:105-112` — the general rule, stated as a **lexer capability**:

> *"A curated Unicode glyph and its ASCII transliteration ... lex to the
> **identical** token ... So the glyph carries **zero** extra information and
> **ASCII spellings remain accepted forever** (no program ever requires a
> special keyboard). This is genuinely a **lexer** capability, not only a
> convention."*

## The measurement

| fact | site |
|---|---|
| ASCII `in` maps to the keyword token | `lexer.rs:997` — `"in" => Token::KwIn` |
| source `∈` maps to `Member` | the glyph arm |
| **membership expression arm in the parser** | **absent — in either spelling** |

⇒ **Two distinct absences, and the second is the larger one.** Even the glyph
spelling does not parse as membership: `Member` is lexed and nothing consumes
it. So this is not merely an ASCII-alias gap; **the operator has no parse at
all.**

## Why this one is genuinely harder than its siblings, and is `M` not `S`

[[LANG-BYTES-HEX-LIST-LITERAL]] is a second spelling of an existing token. **This
is not**, because the ASCII bytes are already spoken for:

> **`let x = value in body`.** The same three characters are the `let`-binder's
> keyword. `seed-canonical-format.md:387`'s `why` states the bind exactly:
> *"the same input bytes occupy opposite token roles. Replacing every `in`
> either corrupts the keyword or fails to canonicalize membership."*

**So the lexer cannot decide this on the bytes alone**, and `31 §1b`'s rule —
one token for both spellings — collides with `let … in` as literally stated.
**That collision is the node.** It is why this is not a one-line lexer arm and
why it is filed `M`.

## Deliverables

**`D1` — the parser arm, glyph spelling first.** Make `∈` parse as a membership
expression. **This half has no keyword collision and is the part that is
unambiguously owed** — do it first and report it separately, so that a hard stop
on `D2` still lands a real increment.

**`D2` — determine what `31 §1b` actually requires of ASCII `in` here, and
report before you implement.** The candidates, and none is to be chosen by the
ring:

1. **Context discriminates.** `in` after a `let` binder is the keyword;
   elsewhere in expression position it is membership. Report whether the
   grammar makes that decidable at the point the lexer or parser must decide,
   **with the ambiguous case named if one exists.**
2. **`31 §1b` has an unstated exception** for bytes already bound to a keyword,
   and the table row at `:79` is in error or under-qualified. **Then the spec is
   what changes**, and this is a finding for the enclave, not a repair here.
3. **A different ASCII spelling** is intended for membership. Report whether
   anything in `31` supports one; **do not invent one.**

**`D3` — report which of the three holds, with the evidence, and stop there if
it is (2) or (3).** Those outcomes are spec changes and the Architect rules
them. Only (1) is implementable on this node's authority.

## Acceptance criteria

**`AC-1`.** `∈` parses as membership and elaborates. **Control:** a fixture
using the glyph, asserted on the elaborated form — not "it compiles".

**`AC-2`.** `let x = e in b` is unchanged. **Control:** the existing `let`
tests are named individually in the handback and stay green. **This is the
regression the whole node risks, and a green suite reported as a total is not
evidence for it.**

**`AC-3`.** `D2`'s verdict is reported with citations before any ASCII handling
is implemented. **A candidate that implements option 1 without first reporting
that option 1 holds has skipped the deliverable**, even if the code is right.

**`AC-4`.** If the outcome is (2) or (3), **no ASCII handling is implemented
at all** and the node stops with `D1` landed. That is a good outcome, not a
partial failure.

**`AC-5`.** Direction stated for every behaviour change. This **adds** accepted
programs; nothing currently accepted becomes rejected. If anything does, stop.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`.

## TWO QUEUED COVERAGE OBLIGATIONS THIS NODE INHERITS (Steward, 2026-09-17)

**Neither was raised against this node, and both were deferred ONTO it** by
separate reviewers on separate candidates, each reasoning — correctly and
independently — that this node is the natural consumer. **Written here because
an obligation deferred in a review thread and recorded nowhere else is an
obligation that gets lost**, and two of them, deferred from two threads on the
same evening, is exactly the shape that survives nobody's memory.

| # | the untested property | deferred by | on |
|---|---|---|---|
| Q1 | the **lossless round-trip** for a type-position projection (`(q : d.Query)`) — `collect_type_spans` gained a `TProj` arm and **nothing exercises it**: the new tests are parse+elaborate with no reprint assertion, and no `.ken` file in the corpus uses the form | adversary, `evt_17feaegey3cnd` | `LANG-TYPE-PROJECTION-SURFACE-FORM` |
| Q2 | **conformance seed cases** for the new capability — zero exist for type-position projection | conformance-validator, via `dec_4tb0wtx34mtrb` | `SPEC-TYPE-PROJECTION-SURFACE-NORMATIVE` |

**Both deferrals were the right call and this is not a complaint about either.**
Holding a spec-only or elaborator-only candidate to manufacture fixtures for a
capability with no consumer is worse than waiting for the consumer — and **this
node is that consumer**: it must author `(q : d.Query)` signatures, so it
produces the fixtures as a by-product either way.

⇒ **The cost of picking them up here is close to zero; the cost of NOT recording
them was the whole obligation.** Q1 closes with one round-trip fixture on a
projection-typed binding. Q2 closes with the conformance seed the spec enclave
owns — **coordinate, do not author it** (see *"Do not edit `conformance/`"*
below, which still binds).

**If either turns out to be more than incidental, stop and say so** — that is a
Steward re-cut, not a scope expansion to absorb quietly.

## What this unblocks, and the follow-through that is NOT yours

`seed-canonical-format.md:387` carries
`BLOCKED-ON-MEMBERSHIP-ASCII-ROLE (no blocker node exists)`, and FMT1's
aggregate at `:52` names the same surface. **This node is that blocker.**
[[CONF-BLOCKER-MARKER-RECONCILE]] will name it.

**Do not edit `conformance/`.** That seed is the spec enclave's and is in flight.
Say in the handback that it is now owned.

## Not this node

- **Not the IFC lattice operators.** `31-lexical.md:81`'s `⊑`/`<:` row is a
  different table entry with a different consumer.
- **Not a general notation-table audit.** If you find other glyph/ASCII pairs
  with no parse, **report them and stop** — that is a Steward re-cut, and it is
  a good finding.
