# WP frame — `LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE`

> # RE-CUT 2026-09-16 (Steward). THIS FRAME REPLACES THE PREVIOUS ONE OUTRIGHT.
>
> The prior frame (403 lines) scoped a parser contraction plus a catalog
> migration. **The operator closed PR #3792 and rejected the contraction**; the
> spec enclave is amending `32 §3` instead. Its §2d, §2e, §2f, §3 and §4
> described how to build the rejected design, so they are deleted rather than
> annotated — an instruction that is still readable is still followable.
>
> **Do not respin `7c6dfdfaf1f10190c78f4249002d0ea24ba1d54c`.**
> **`dec_wac7adfhr23c` is VOID** and must not be cited.

Owner: **language**   Size: **S**   Tier: **T1**   Gate: none

## 1. Objective

Make an ungrouped `if` after an application head **reject affirmatively at the
`if`**, with the rejection raised by the argument loop itself rather than by
whatever production later encounters the leftover tokens.

That is the entire deliverable. One parser edit, one test, one closure argument.

## 2. Fixed inputs

Measured at `origin/main`
`a63eebe3e7582c71a2c59624786a39705cf9a0e0`. Re-measure before you start; line
numbers move.

### 2a. The spec pin, verbatim

`spec/30-surface/32-grammar.md:393`:

> **Citation corrected 2026-09-16.** This frame originally cited `:373`,
> measured before `SPEC-32-PROJECTION-PRECEDENCE` (`f1bda0c5b`) landed and
> inserted twenty lines above it. The quoted sentence is unchanged; only its
> coordinate moved. Re-measure rather than trusting either number.

> five leading forms — lambda and `let`, `if`, `match`, and `temporal` — reject
> at their leading token when ungrouped after an application head

**This sentence is byte-stable across the `32 §3` amendment, which has now
LANDED** (`f1bda0c5b`, `SPEC-32-PROJECTION-PRECEDENCE`). The amendment struck
`and projection` from the clause that follows it and left the quoted text alone.

The original form of this paragraph said the node was "not gated on that
amendment landing, and must not wait for it." That was the load-bearing claim
and it held: the amendment landed independently, and nothing in this node's
scope moved when it did.

### 2b. The divergence

    const k : Nat = keep if c then a else b

Required: a parse error at the `if`, raised by the expression parser.
Today: accepted as `A(keep, If(...))`.

Four of the five leading forms already conform. This closes the fifth.

## 3. The mechanism — read this before editing anything

**A continuation predicate can only STOP. It can never REJECT.**

The argument loop:

    if !self.can_start_atom_expr() { break; }

Deleting `Token::KwIf` from `can_start_atom_expr` does **not** make
`keep if c then a else b` reject at the `if`. It makes the loop **break**. The
body parses as `keep`, `if c then a else b` is left unconsumed, and the error is
raised by the **file-level declaration parser** when it tries to begin the next
declaration at a token that cannot start one. That is why the observed message
lists declaration keywords.

⇒ **Removal from the predicate is necessary and cannot be sufficient.** The
required change is an affirmative error inside the loop: distinguish *"no more
arguments"* from *"a form that is not an `application_atom` appears where an
argument may start."*

This is why the node says re-implement, not re-base. The prior edit is not an
incomplete version of the right change; it is the wrong shape for the
requirement.

### 3a. Why the previous candidate's test went 9/9 green

Its `AC-IF-REJECTS` test asserted `span.start == src.find("if")`. **A stray-token
error is reported at the stray token**, so that assertion passes under both the
correct implementation and the defective one. The untouched pre-existing
`lang_surface_if` suite is what caught the defect.

⇒ **A span is a coordinate; the requirement is about authorship.** The test for
this node must discriminate on **which production raised the error**.

## 4. The closure — five loops, four predicates, two spellings

The population is **not** what a grep for `if !self.can_start_*() {` returns;
that spelling finds two of five.

    can_start_atom_type   :1667   while self.can_start_atom_type()    KwIf absent
    can_start_atom_type   :1973   while self.can_start_atom_type()    KwIf absent
    can_start_atom_expr   :2311   if !…  { break }                    KwIf PRESENT
    can_start_pattern     :2540   if !…  { break }                    KwIf absent
    can_start_atom_pat    :2590   while self.can_start_atom_pat()     KwIf absent

**Only `:2311` changes.**

**`:2540` must NOT be given an affirmative rejection.** After a pattern, `if`
legitimately opens a **match-arm guard**. The rule *"a continuation predicate
cannot reject, so make it reject"* applied there rejects every guarded match
arm. **The licence to convert a stop into a rejection is a per-loop fact — it
holds only where the stop position admits no legitimate successor** — not a
property of the predicate class.

**`can_start_atom_pat` is not independent:**

    can_start_atom_pat  ==  can_start_pattern() && !is_contextual_ident("as")

⇒ **Editing `can_start_pattern` silently changes the `:2590` loop as well** —
one edit, two loops, and only one of them appears in the diff a reviewer reads.

## 5. Acceptance

Each criterion names the control that makes it falsifiable.

**AC-IF-REJECTS-AFFIRMATIVELY.** `const k : Nat = keep if c then a else b`
produces a parse error at the `if`, **raised by the argument loop**.
*Control, and this is the load-bearing one:* the test must fail against an
implementation that merely removes `KwIf` from the predicate. Assert the error's
**identity** — which production raised it — not `span.start`. A test that only
checks position passes under the defect and is not evidence.

**AC-GUARD-UNBROKEN.** A guarded match arm (`| p if c => e`) still parses.
*Control:* a fixture that fails if `can_start_pattern` acquired a rejection.
This is the negative control for §4's trap, and it must exist even though
nobody currently intends to touch that loop — `can_start_atom_pat`'s delegation
means the reachable blast radius is wider than the edit.

**AC-LOOP-CLOSURE.** The other four loops and the three non-expression
predicates are **byte-unchanged**.
*Control, mechanical and cheap:* `Token::KwIf` appears in **exactly one**
predicate before the change and **zero** after; the other three predicates'
token lists are byte-identical. State the count both before and after — a
one-sided number cannot show the change was confined.

**AC-INLINE-KEN-CENSUS.** The fixture population includes **inline Ken inside
Rust test files**, not only `.ken` / `.ken.md` sources.
*Control:* the census must return `lang_surface_if.rs:118` — a known member. A
census that misses it is measured blind. Four such files have been found by four
separate CI failures and never by a census; a sweep keyed on source extensions
is structurally incapable of reaching them.

**AC-SURFACE-SUITE-GREEN.** `lang_surface_if` passes, including the pre-existing
`argument_let_and_nested_else_binding_are_reachable` case at `:118`, which today
asserts that shape **elaborates**. It must be re-expressed as a rejection or
grouped. Changing it is in scope; deleting it is not.

**AC-NO-REGRESSION. Workspace-green in CI**, never a local `--workspace` run
(`COORDINATION §12`). Locally, build and test only through `scripts/ken-cargo`
scoped to `-p ken-elaborator`.

## 6. What this node is NOT

- **Not the projection row.** Retired. The parser already conforms to the
  amended pin; its existing grouped reading is now the specified one.
- **Not a catalog migration.** There is none, and the catalog must not be
  touched. It is gated on being a formatter fixed point: byte-splicing parens at
  AST offsets without a formatter pass reddens `ken_fmt`'s frozen-corpus gate
  and `kenfmt_c_capstone`, in two different crates. **If any catalog edit ever
  does become necessary, it owes a formatter pass.**
- **Not a change to `32 §3`.** The amendment is the spec enclave's act, already
  reviewed. This node conforms to the pin; it does not move it.
- **Not the bare-operator-value row** (`:357-363`) — see
  [[LANG-BARE-OPERATOR-ATOM-REJECTION]].
- **Not the temporal rows** (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`).

## 7. Contention

`crates/ken-elaborator/src/parser.rs` only. The catalog and `library/` are out
of scope, so this node no longer contends there. It still overlaps A0's file
set: check [[LANG-RESERVED-INFIX-NAMES]]'s **node status at `origin/main`** —
not for a branch ref — before releasing.

## 8. Why T1 on a small diff

The diff is small; the review is not. The defect this node replaces was a
one-token deletion that looked exactly like the correct edit, passed its own
purpose-built test, and was **approved EXACT** by the Architect. What
distinguishes the right change is an argument about what a continuation
predicate can do — not a diff inspection. **Do not read the small diff as
licence for a mechanical seat.**
