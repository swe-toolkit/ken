---
id: LANG-INSTANCE-SEARCH-SECOND-PATH
title: "`projected_instance_id` (`elab.rs:10446`) misattributes instance rows two ways and both are keyed on a SPELLING. Its first arm fires when the projected base is a constant literally named `d` even when `d` is NOT the constraint binder, so a user global named `d` is attributed the constraint's effect row instead of its own. Both of its `instance_search` calls key on `rtype_head_name(&constraint.head_type)` -- the same carrier-spelling predicate LANG-INSTANCE-REGISTRY-IDENTITY-KEY is closing on the term-producing path, which the Architect explicitly left here. Delete the `d` disjunct; then measure whether this path can be keyed on identity WITHOUT kernel state. The Architect forbade threading kernel state into this function, so a requirement for it is a STOP, not an implementer choice."
status: merged
owner: language
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "language-leader, 2026-09-19 (evt_6mbj5ebssp9ca), routed to the Steward out of LANG-STANDARD-INFIX-CALL-COMPLETION's AC-3 discharge on the Architect's instruction, as pre-existing work that predates A1 and discharges independently of AC-3. Filed with a CORRECTED premise: see the first section. Steward-filed per COORDINATION section 2; constraint interrogated per steward.md section 4c."
---

# FIRST: THE FILING'S PREMISE IS FALSE, AND THE CORRECTION MAKES THE NODE SHARPER

**Filed as:** *"pub, zero production callers, live only in
`tests/lc_acceptance.rs`"* — a second dispatch path **waiting for a caller**.

**Measured by the Steward at `origin/main` `eadba7e82`:**

    crates/ken-elaborator/src/classes.rs:331   pub fn instance_search
    crates/ken-elaborator/src/elab.rs:10090    ctx.class_env.instance_search(..)
    crates/ken-elaborator/src/elab.rs:10101    ctx.class_env.instance_search(..)

Both are inside `fn projected_instance_id` (`elab.rs:10083`), called at
`elab.rs:10126`. All in `src/`, all at module top level.

**The one thing that could have made them test-only was checked:** the nearest
preceding `#[cfg(test)]` is at `:7824` and **closes at `:7829`** — a five-line
block, nowhere near. `ProjectionPurityCtx` occurs only in `elab.rs` and no part
of the chain is test-gated.

⇒ **TWO PRODUCTION CALL SITES, NOT ZERO.** And the other half is wrong in the
opposite direction: test uses span `cat1_constructor_classes`,
`cat1_lawful_functors_package`, `cat_bool_pub_export`, `cat_bytes_keys_closeout`,
`cc1_nonempty_validation_acceptance`, `ds1_empty_dec_acceptance`,
`l3b_acceptance`, `record_decl_form` and `lc_acceptance` — not one file.

> ### WHAT THIS REFUTES, AND WHAT IT SHARPENS.
>
> **Refuted:** "dead code waiting for a caller." There is nothing to wait for.
>
> **NOT refuted — sharpened:** "second dispatch path." A path with live
> production callers is not latent. **Whether that is a defect is exactly what
> `D0` asks**, and it is a better question than the one the node was filed with:
> a dead path is a tidiness item, a live second selector is a soundness-adjacent
> one.

# D0 IS ANSWERED: IT READS. The node is the smaller branch.

**Architect, `evt_14epkeqjam4q2`, classified off the producer at `0b2f327e9`.
The chain is short and single-caller at every hop:**

    elab.rs:10083  projected_instance_id -> Option<GlobalId>   ONE caller
    elab.rs:10126    inside projected_field_row_type (:10113)   ONE caller
    elab.rs:10262      infer_expr_row_type's RProj arm

**What the `GlobalId` is used for, in full:** recover the class name, the
`ClassInfo`, a field index, and then `field_effect_rows[idx]` (falling back to
`field_purities[idx]`) — returning a `crate::effects::RowType`.

⇒ **The id never becomes a `Term`, is never applied, and never enters the
elaborated term.** It is a lookup key for a field's EFFECT ROW.

    TWO live production CALLERS. ZERO live production SELECTIONS.

**So this node is the READS branch: stale-and-false doc comments plus a
visibility question. Size `S` stands and is no longer provisional.**

# THE DOC COMMENTS ARE FALSE, NOT MERELY UNQUALIFIED

**An earlier revision of this node said they "assert it without
qualification."** That undersold it, and `D0`'s answer is what makes them false.

**Steward re-measured independently at `a710759f4`:** `instance_search` has
exactly THREE code occurrences in all of `crates/**/src` — the definition at
`classes.rs:331` and the two calls at `elab.rs:10090`/`:10101`. Every other hit
is a comment. **`elaborate_rdecl_v1` does not call it** (measured: zero
occurrences in its body). The `where`-clause path goes through
`resolve_instance_dictionary` at `elab.rs:9634`.

    resolve.rs:165    "checked against `instance_search` in
                       `elaborate_rdecl_v1`"                      FALSE
    prelude.rs:21/:23 the `where DecEq K` path resolves "via the
                       Lc-landed `instance_search`"               FALSE
    prelude.rs:21     cites `classes.rs:91`; it is at `:331`      STALE

⇒ **They name the wrong mechanism for a path that has a different one**, so a
reader chasing the `where` path is sent to a function it does not call. That is
a bigger doc defect than a missing qualifier, and it is the node's main content.

# TWO FURTHER FINDINGS IN `projected_instance_id`, from the Architect

Both surfaced while classifying `D0` and both belong to this node:

1. **It keys on `rtype_head_name(&constraint.head_type)` — a carrier
   SPELLING**, the same predicate A1 just closed on the completion path. The
   consequence here is a wrong EFFECT ROW rather than a wrong dictionary, **so
   the blast radius is smaller and the defect is the same one in smaller
   clothes.**
2. **Its first arm matches `name == "d"`** — a hardcoded one-letter binder
   spelling — firing whenever the decl has exactly one local constraint and the
   projected base is a constant named literally `d`. **A global constant named
   `d` in such a decl is attributed the constraint's instance row.**
   Misattribution keyed on a name the user chose.

# The original D0, kept for the record

    Classify elab.rs:10090 and :10101 at a NAMED SHA.

    Does `projected_instance_id` RESOLVE a dictionary that reaches
    elaboration, or READ the registry to classify projection purity?

    RESOLVES -> main carries two live selection paths. Defect node,
                size grows, and it is no longer `S`.
    READS    -> the node is three stale doc comments plus a visibility
                question. Smaller than filed.

**Do not guess this from the name.** `projected_instance_id` returns
`Option<GlobalId>` inside a **purity** context, which is consistent with either
reading. The Steward measured call SITES and deliberately did not classify what
they DO — **a census of call sites is not a census of behaviour**, and this
session has already spent two numbers that were complete, correct, and cited for
a question they did not answer.

# The three doc comments are CONFIRMED, and are a finding either way

    prelude.rs:21   "via the Lc-landed `instance_search` (`classes.rs:91`)"
    prelude.rs:23   "checked against `instance_search` before the body
                     elaborates, emitting `NoInstance` on failure"
    resolve.rs:165  "checked against `instance_search` in `elaborate_rdecl_v1`"

All three describe `instance_search` as the mechanism the `where`-clause path
uses, **without qualification**. Whether that is accurate today is `D0`'s
answer; that they assert it unconditionally is true now.

**Note `prelude.rs:21` cites `classes.rs:91`. The function is at `:331`.** That
citation is stale on its face and is the cheapest available evidence that this
prose has not been re-grounded in a long time.

# This node and AC-3 are the SAME MEASUREMENT

`[[LANG-STANDARD-INFIX-CALL-COMPLETION]]`'s AC-3 was amended 2026-09-19 to the
property *"ONE FUNCTION PERFORMS INSTANCE SELECTION; every entry point
contributes a registry KEY and REFUSALS, never a resolution,"* discharged by
enumerating every `ClassEnv::instances` read that yields a dictionary instance
and showing exactly one sits on a production path.

**That census IS `D0`.** Whoever runs either runs both.

> **Which is the argument for the amended AC rather than a coincidence.** The
> struck wording — *"name the single resolver entry point"* — has no vocabulary
> for a second READER that may or may not be a selector, so it could not have
> asked this question at all. The rewrite is what makes `D0` expressible.

**They still do not fold.** AC-3 is an acceptance criterion on a released WP;
this is a pre-existing defect that predates A1 and discharges independently
(Architect, via `evt_6mbj5ebssp9ca`). Running one census satisfies both, and
neither blocks the other.

# The constraint, interrogated

**Grounded, CONDITIONALLY on `D0`.** If two production paths select instances,
`docs/PRINCIPLES.md` §8 applies directly — instance selection decides which
dictionary a term elaborates against, and two selectors that can disagree is
silent divergence. **If `D0` returns READS, the grounding is different and
weaker:** three doc comments asserting a mechanism the code does not use, which
is an honesty-of-the-record issue, real but small.

**NOT grounded, and must not be written in:** not a TCB argument
(`PRINCIPLES.md` §5 puts the elaborator outside the trust root), and not a
safety-of-`main` argument. Nothing is red.

# Why this is `draft`

**QUEUED by priority, and additionally gated on its own `D0`.** L1 (clearing the
ignored tests) is the operator's top priority as of 2026-09-17; the language
lane's objective is `LANG-MODULE-IMPORT-SYSTEM`. Not released until the Steward
releases it. Pre-existing, no contention, and it blocks nothing.

**Size `S` is provisional and is `D0`'s to overturn** — it is sized for the
READS branch. If `D0` returns RESOLVES, re-size before starting rather than
absorbing the growth silently.

# AMENDED 2026-09-20 (Steward): the grounding is INVERTED and the node is `ready`

**What changed.** This node previously declared the three false doc comments
"the node's main content." That is backwards and `steward.md` §1 forbids it: a
documentation correction is not a lane deliverable. The node's content is the
**two measured misattribution defects** the Architect surfaced while
classifying `D0`. They were already in this file, filed under a heading that
read as an aside. The doc comments become a rider on the repair, not its
justification.

**Every coordinate below was re-measured at `origin/main`
`1bd3ad8f15bdfee72e6fb16ce7c187191bb6d415`.** The coordinates this node was
filed with are two revisions stale (`:10083`, `:10090`, `:10101`) and must not
be used. Cite by symbol; `elab.rs` moves under this node again when
`LANG-INSTANCE-REGISTRY-IDENTITY-KEY` lands.

    elab.rs:10446   fn projected_instance_id
    elab.rs:10450     arm 1 guard: local_constraints.len() == 1
                      && (name == "d" || name == &local_constraints[0].binder)
    elab.rs:10453/:10455   instance_search on rtype_head_name(head_type)
    elab.rs:10464/:10466   instance_search on rtype_head_name(head_type)

## The settled inputs. Do not re-derive any of these.

**`D0` is answered and stays answered: this path READS.** The `GlobalId` never
becomes a `Term`, is never applied, and never enters the elaborated term. It is
a lookup key for a field's EFFECT ROW. Today's `LANG-INSTANCE-REGISTRY-IDENTITY-KEY`
`D0` measured the same thing independently at these exact sites: both branches
consume the old instance's stored row in pre-elaboration purity analysis and
refuse before dictionary-term resolution, with provenance empty. Two live
production callers, zero live production selections.

**Arm 1 is subsumed by arm 2 once the `d` disjunct is removed, and this is
measured, not predicted.** Arm 2 (`:10458`) searches `local_constraints` for a
constraint whose `binder == name` and performs the identical
`instance_search`; falling back to `ctx.globals.get(name)`. So with `len() == 1`
and `name == binder`, the two arms compute the same value. The `d` disjunct is
therefore the ONLY behavior arm 1 adds, and that behavior is the defect.

**The defect is decidable in one sentence.** When a declaration has exactly one
local constraint whose binder is not `d`, and the projected base is a global
constant named `d`, arm 1 returns the constraint's instance; the correct answer
is `ctx.globals.get("d")`. Misattribution keyed on a name the user chose.

**The sibling WP is closing the same predicate on the other path, and left this
one deliberately.** The Architect's arm (b) ruling (`evt_2vg7yn39wg0ge`)
instructs: "Do not thread kernel state into `projected_instance_id` or absorb
`elab.rs:10455/10466` here ... their separate consequence remains
LANG-INSTANCE-SEARCH-SECOND-PATH." When that candidate lands, `main` carries the
carrier-spelling predicate REPAIRED on the term-producing path and UNREPAIRED at
these two calls. That asymmetry is this node's subject.

## Deliverables

**`D0` -- delete the `d` disjunct.** One guard. Prove by control, not by
inspection, that a global named `d` is no longer attributed a constraint's row.
If the measurement above holds, arm 1 becomes wholly redundant; removing the
whole arm is permitted but is not required and must not change any other
behavior.

**`D1` -- measure whether these two `instance_search` calls can be keyed on
identity WITHOUT kernel state, then repair or stop.** `rtype_head_name` returns
a spelling. The question is whether the purity context already carries, or can
cheaply derive, an identity-grade key at this point in pre-elaboration. Report
what the context holds before proposing any repair.

**`D2` -- correct the three false doc comments** (`resolve.rs:165`,
`prelude.rs:21`, `prelude.rs:23`). They name `instance_search` as the mechanism
the `where`-clause path uses; that path goes through
`resolve_instance_dictionary`. `prelude.rs:21` also cites `classes.rs:91` for a
function at `classes.rs:331`. This is a rider on `D0`/`D1`, never a substitute
for them, and it does not close this node on its own.

## Acceptance criteria

**AC-1 -- the `d` misattribution has a behavioral control that RED-s on
restoration.** One declaration, exactly one local constraint whose binder is
NOT `d`, a global constant named `d`, and a projection off that constant.
Assert the effect row is the global's own. Then restore the `d` disjunct: the
control MUST go red. A control that passes with the disjunct restored is
testing something else.

**AC-2 -- the binder path is unchanged.** A declaration whose single
constraint's binder IS the projected name must produce the same effect row
before and after `D0`. This is the arm-1-is-subsumed claim, and it must be
executed rather than argued.

**AC-3 -- `D1` reports the context contents before any repair.** State what
identity-grade key material `ProjectionPurityCtx` carries at the call. If a
repair is built, a mutation reverting the key to `rtype_head_name` must RED a
control that distinguishes two head spellings sharing one identity. If no
repair is built, AC-3 is discharged by the report.

## Stop conditions

Hand back rather than work around if either holds:

- **Identity keying requires threading kernel state into
  `projected_instance_id`.** The Architect forbade exactly that in the sibling
  WP. It is not an implementer choice and not a thing to do quietly here; it is
  a frame amendment and it is mine to author. Report what is missing and stop.
- **`D0`'s removal changes any behavior beyond the `d` case.** That refutes the
  subsumption measurement above, which is mine. Report the differing case.

## Not this node

- `ClassEnv::instances` re-keying. Refuted for the sibling WP by the Architect
  on the grounds that a bare `(class GlobalId, head GlobalId)` key cannot
  represent the admitted variable and structural heads; nothing here revives it.
- Any change to `resolve_instance_dictionary`, the A1 confirmation helper, or
  the ambiguity refusal.
- Coherence, orphan, re-export, derive, kernel, trust, or public interface.

## Release note

Hold the release until `LANG-INSTANCE-REGISTRY-IDENTITY-KEY` lands. Both touch
`elab.rs`, that candidate is in review, and there is no reason to create a
same-file intersection while it is in flight. The two repairs are independent;
this is contention avoidance, not a dependency, and `depends_on` stays empty.

## `D0` WITHDRAWN 2026-09-20 — REFUTED by the ring at `evt_zh6jg8q0693f`

**Do not delete the `name == "d"` disjunct. The settled input that authorized
`D0` was mine and it was false in both directions.**

**What I did wrong.** I read both arms of `projected_instance_id`, observed that
arm 2 performs the identical `instance_search` when `binder == name`, and
concluded the `d` disjunct was the only behavior arm 1 adds. I never read
`resolve_instance_constraints`. I wrote a predicate over the name `d` without
reading the producer of its meaning.

**What `d` actually is.** `resolve_instance_constraints` registers `d` as a
local-dictionary alias whenever a declaration has exactly one explicitly named
constraint whose binder is not `d`, and `elaborate_rdecl_v1` installs the same
alias in `local_dicts`. So an unqualified `d` in a sole-constraint declaration
IS that alias. The disjunct is the purity-side counterpart of a real aliasing
rule, not a stray hardcoded spelling.

**Deleting it is an OVER-ACCEPTANCE, measured.** With a pure global
`const d : Quiet Bool` and one `(effect : Effectful Bool)` constraint:

    disjunct present    d.step x  rejects: TypeMismatch, EffectEscapes, FS
    disjunct deleted    d.step x  becomes Ok(g648)
    disjunct restored   both controls green, 2/2

Resolution still binds `d` to the effectful constraint alias while purity falls
through to the pure global, so an effect escape is silently admitted -- in a
node whose entire subject is misattributed effect rows.

**And the witness I named is unreachable.** The alias shadows the global at that
spelling, so the "user global named `d` is misattributed the constraint's row"
case cannot arise through unqualified `d` in a sole-constraint declaration.
There was no defect there to fix.

**Keep the diagnostic checkpoint `3a6f64494f29b081292a14f96101c49181be36b8`.**
Its two behavioral controls pin a real aliasing invariant that nothing else
pins. They should outlive the refuted deliverable.

## The node is now `D1`, and `D1` opens with a measurement

**`D1` is untouched by the refutation and is sharpened by it.** The two
`instance_search` calls still key on `rtype_head_name(&constraint.head_type)`, a
carrier SPELLING, and a wrong hit is a wrong effect row. That this function must
AGREE with `resolve_instance_constraints` about what a name means is precisely
why a spelling key is fragile.

**FIRST, establish what the landed sibling changed at these two exact calls, and
report before proposing any repair.** `LANG-INSTANCE-REGISTRY-IDENTITY-KEY`
landed as `83f30f5e8` and routes term-producing resolution through
`confirm_instance_dictionary_carrier`. Its M8 report asserts all four named D0
consumers including the Ord and Membership projections reach that confirmation,
yet `projected_instance_id` calls `ClassEnv::instance_search` directly rather
than through `resolve_instance_dictionary_inner`. **Which of those holds at
these two calls is unmeasured, and the Steward is not asserting it.** The answer
may shrink this node or close it outright.

**`D2` (the three false doc comments) is unchanged and remains a rider.** AC-1
and AC-2 as originally written are withdrawn with `D0`; AC-3's requirement --
that a mutation reverting the key to `rtype_head_name` must RED a control
distinguishing two head spellings sharing one identity -- survives and applies
to whatever `D1` returns.

**Stop conditions are unchanged**, and the kernel-state one is now the more
likely of the two: threading kernel state into `projected_instance_id` is
forbidden by the Architect's sibling ruling, so a repair needing it is a stop
and a frame amendment that is the Steward's to author.

## `D1` REACHES ITS FRAMED STOP 2026-09-20. The node CLOSES on a capped closeout cut.

**The stop condition this frame named as the more likely one is the one that
fired**, and it fired honestly: the ring measured before proposing, and returned
an absence rather than a workaround.

**What `D1` measured** (implementer `evt_6hpxy4qe1n4nk`, Architect confirming on
exact base `e4df15c48`): identity-grade carrier material sufficient to repair
either purity read is ABSENT from `ProjectionPurityCtx`. The complete material
there is `globals: name -> GlobalId`, the name-keyed `ClassEnv::instances` and
its `InstanceInfo`, the local `(class_name, head_type, binder)` constraints, and
string-only bound dictionary pairs. `RType::RCon` retains a spelling, the
registry key is a spelling, and `InstanceInfo` stores the dictionary
`instance_id` plus an optional surface head pattern but no captured core carrier.

**Why that is insufficient rather than merely awkward.** On the exact rebound
boundary, the requested `RCon("Foo")` and the stale registry key `"Foo"` consult
the SAME current `globals["Foo"]`. Resolving either through that map blesses the
stale row instead of distinguishing it -- **the instrument reinterprets both
operands through the one mapping whose staleness is the question.** The only
honest discriminator is kernel inference of the candidate dictionary type
compared against an independently derived expected carrier, and
`ProjectionPurityCtx` carries neither operand nor the `GlobalEnv` to derive them.

**And the sibling supplies no authority here.**
`LANG-INSTANCE-REGISTRY-IDENTITY-KEY`'s
M8 named four D0 consumers reaching `confirm_instance_dictionary_carrier`. Those
are the term-producing `Ord` and `Membership` sites, which route through
`resolve_instance_dictionary_by_head_id`. `projected_instance_id` holds the only
two direct production calls to `ClassEnv::instance_search` and enters neither the
resolver nor the confirmation helper. **The M8 statement is true and covers
different consumers**; this was measured rather than inferred from the name.

### The residual is RECORDED, NOT CLOSED AS SAFE

**I am not adopting the phrase "fail-closed behind term-producing
confirmation."** The Architect offered it; I decline it because I have not
measured it and the two consumers are distinct by the Architect's own finding.
Confirmation on the term-producing path catches a wrong CARRIER. It does not
obviously catch a wrong EFFECT ROW verdict reached before dictionary-term
resolution, and the reachability of the rebound-carrier case at these two exact
calls is unmeasured.

⇒ **The residual is real in shape, unmeasured in reach, and unrepairable under
the current frame.** A closed node reads as resolved; this one is closed for
lack of an authorized remedy, which is a different thing, and the difference is
recorded here so nobody cites this closure as a safety result.

**No successor node is framed for it.** The Architect forbade the identity-keyed
repair, the re-keying, and the representation change. If the operator's objective
ever demands it, it is a new node and a new frame.

### The closeout cut, capped

Authorized at `evt_4wj4aa400zsh0`. Two items, both already measured, one turn:

    1. The two behavioral controls from diagnostic checkpoint
       3a6f64494f29b081292a14f96101c49181be36b8, landed as ordinary tests.
    2. D2's three false doc comments: resolve.rs, prelude.rs (two sites).

    AC-C1  both controls green on the candidate
    AC-C2  deleting the `name == "d"` disjunct REDs the `d.step` control and
           leaves the binder-named control green -- re-run on the candidate,
           not carried from the checkpoint
    AC-C3  corrected comments cite BY SYMBOL, no line numbers
    AC-C4  the diff over `crates/**/src` is comment lines and test files only

**AC-C3 is the one with a reason behind it.** `prelude.rs` cited
`classes.rs:91` for a function at `:331`, and that stale coordinate is the
cheapest evidence the prose was never re-grounded. Replacing one line number
with another only resets the clock on the same defect.

**Scope stop:** if re-grounding a comment turns up anything beyond the mechanism
name, land the tests and leave that comment alone. A three-comment rider does
not become a prose pass.

**The node flips to closed when this cut lands. Not before, and no further
deliverable is authorized on it.**
