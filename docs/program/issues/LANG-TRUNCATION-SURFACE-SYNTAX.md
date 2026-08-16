---
id: LANG-TRUNCATION-SURFACE-SYNTAX
title: "Give propositional truncation a surface spelling and an elaboration rule -- the kernel already types Trunc and TruncProj, and no .ken file can reach them"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-16, on the merge of V3-FO-CHECKER-SOUNDNESS D0 (PR #2424). D0 hard-stopped on part (2): truncation is unwritable in .ken. Architect evt_38f22rwkq90ry re-diagnosed the stop as a missing spelling rather than a missing capability and asked for the unblock to be filed separately -- explicitly not a D0 recut. Every coordinate below re-measured by the Steward against origin/main bfff4290d, including two the Architect's census did not name. Steward-filed per COORDINATION section 2."
---

## Why this exists

`23 §4.3` defines `Derives(s) : Omega := ‖ Derivation(s) ‖` and
`classically_valid` through it, and `23 §4.4` gates route FO's `proved` verdict
on `checker_soundness`, whose conclusion is `classically_valid q`.
[[V3-FO-CHECKER-SOUNDNESS]] `D0` tried to write that one line in a `.ken` file
and could not.

**The kernel is not the problem. It has had propositional truncation all
along** — `‖A‖` is a `[K2]` former, specified at `16 §6`, typed and computed
by the kernel. What is missing is a way to say it.

> **`Term::Trunc` exists and is kernel-typed; no surface syntax or elaboration
> rule reaches it.**

This node writes the spelling. It is the whole of the work: no kernel change, no
new former, no trusted-base entry.

## Fixed inputs, measured at `origin/main` `bfff4290d`

**Treat every anchor here as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around it.**
Line numbers are anchors to re-find, never values to check.

### The kernel side is complete. All three judgments already exist.

| judgment | where | what it says |
|---|---|---|
| **formation** | `ken-kernel/src/check.rs`, the `Term::Trunc(a)` arm of `infer` | `‖A‖ : Ω_l` for `A : Type l` |
| **introduction** | `check.rs`, the `Term::TruncProj(a)` arm of `check` | `|a| : ‖A‖` iff `a : A` |
| **elimination** | `check.rs`, `infer_quot_elim` — its scrutinee match admits `Term::Trunc(a) => (*a, None)` | `QuotElim` at a `Trunc` scrutinee, **Ω target only** |

`Term::Trunc(Box<Term>)` and `Term::TruncProj(Box<Term>)` are declared adjacent
in `ken-kernel/src/term.rs`, doc-commented `‖ A ‖` and `|t|`, and their `Debug`
renders those exact glyphs. Conversion normalizes through both
(`conv.rs`), and `subst.rs` rebuilds both in all four substitution walks.

### Two corrections to the received account. Both change what to build.

**1. `TruncProj` is the INTRODUCTION, not the elimination.** The Architect's
approval asks whether *"`TruncProj`'s elimination is adequate for `D1`'s
needs"*; `TruncProj` is what injects `a : A` into `‖A‖`. **There is no
`TruncElim` node in the kernel at all** — elimination is `QuotElim` with a
`Trunc` scrutinee. Anyone grepping for a truncation eliminator finds nothing and
concludes the kernel is incomplete. It is not.

**2. The adequacy question is CLOSED, and the answer is yes.** `infer_quot_elim`
refuses a `Type` target on a `Trunc` scrutinee — *"quotient-elim Type target
requires a Quot (not Trunc)"* — and permits an Ω target. **That restriction is
exactly propositional truncation's defining property, and Ω is exactly where
`classically_valid : Form -> Omega` lands.** The kernel permits precisely the
elimination `checker_soundness` needs. This was measured here rather than
assumed; the Architect stated it as unverified.

### The elaborator side: three construction sites, no surface path

| site | what it does |
|---|---|
| `ken-elaborator/src/prelude.rs`, in the `perm_body` construction | builds `Trunc` of the `Perm` relation from raw Rust |
| `ken-elaborator/src/fo_kripke.rs`, inside `denote` | builds `Trunc` of a two-constructor sum — this is `or` |
| `ken-elaborator/src/fo_kripke.rs`, the `or_term` construction | the same, for the slice signature |

Every other elaborator reference is a structural traversal — `zonk`,
`subst`/`generalize`, the depth walk, the checked-core encode, the trust-surface
walk in `foreign.rs`.

⇒ **The former is already load-bearing in route FO's own Rust half.** `denote`
constructs truncations and `fo_kripke.rs` reads them back as the canonical `or`.
The gap is purely that a `.ken` author cannot write what the Rust already
builds.

> **The Architect's census named one construction site. There are three.** The
> two extra ones are in route FO itself, which is the caller this node exists to
> unblock — so the count is not incidental to the framing.

### The lexer already speaks this dialect. This is one more arm.

**21 single-character non-ASCII token arms already exist** in
`ken-elaborator/src/lexer.rs`, in one contiguous block — among them
`λ → ↦ ≡ ≤ ≥ ≠ ∧ ∨ ⊑ ⊔ ⊓ ×`. Each is four lines: match the char, `advance`,
return a `Token`.

**And one of them is the precedent for the ASCII question.** `Ω` does not lex to
a symbol token at all; it lexes to `Token::ConId("Omega")`. **The language
already gives one former a Unicode glyph and an ASCII name for the same thing**,
and every catalog `.ken` uses the ASCII one — `Omega` appears, `Ω` does not.

## The design, stated as a guess to attack rather than a survey

Per the framing rule: this is the a-priori best guess, and discovery is expected
to happen inside the attempt. **If building it shows a choice below is wrong,
that is a result — report it with what you found.**

**Formation: `‖A‖`, one new token `‖` (U+2016 DOUBLE VERTICAL LINE).** Matches
`16 §6`, matches the kernel's own `Debug`, and drops into the existing block as
a twenty-second arm.

**Also give it an ASCII spelling**, on the `Ω`/`Omega` precedent, because the
catalog is written in ASCII and `FoKripke.ken` will be its first caller. **Name
it so it cannot collide with a user-defined constructor** — this is the one
place the `Ω` precedent does not transfer cleanly, since `Ω` resolves to a
prelude name that already exists and a truncation spelling would be a new
reserved word.

**Introduction is the hard one, and `|a|` is not available.** `|` is already an
arm separator, so the kernel's `Debug` spelling is ambiguous in surface
position. **Compounding it: `TruncProj` cannot be inferred** — `check.rs` lists
it among the introduction forms that need an expected type or an ascription. So
whatever the spelling, it is only writable in a checked position. Say so in the
error when it is not.

**Elimination: expose `16 §6`'s `elim_trunc P f t` and elaborate it to
`QuotElim`.** The spec names the form and its computation rule
(`elim_trunc P f |a| ≡ f a`); the kernel implements it through the quotient
eliminator. **Bridging that is exactly what an elaborator is for** — do not
propagate `QuotElim` into the surface, and do not add a kernel node to make the
names match.

## Deliverables

**`D1` — the token and the formation rule.** `‖A‖` lexes, parses, and
elaborates to `Term::Trunc`, with its ASCII spelling. A `.ken` file declares
something of type `Omega` using it.

**`D2` — introduction and elimination.** The intro form in a checked position,
and `elim_trunc` elaborating to `QuotElim` at a `Trunc` scrutinee.

**`D3` — the caller.** `Derives(s) : Omega := ‖ FokDerivation s ‖` written in a
real `.ken` file and kernel-checked. **This is the line `D0` could not write and
the reason this node exists.**

## Acceptance criteria

**`AC-1`. Zero new entries in `trusted_base()`, pinned before and after.** The
entire premise is that the former is already admitted. `D0`'s probe pins
`trusted_base()` around its declarations — reuse that shape. **If this node
grows the trusted base, its central claim was wrong and it routes back rather
than landing.**

**`AC-2`. No kernel change.** Not `term.rs`, not `check.rs`, not `conv.rs`, not
`subst.rs`. The three judgments exist. **If elaboration genuinely cannot be
built without touching the kernel, that is a finding and it escalates** — it
would mean the received diagnosis was wrong and the owner is Kernel, not
Language.

**`AC-3`. The surface form produces the SAME core term the Rust builds,
established by a CONTROL.** Elaborate the surface spelling and compare the
resulting `Term` against one constructed directly, as `denote` does. A reading
of the elaboration rule is not this AC. **Mutate the elaboration rule and show
the comparison reds.**

**`AC-4`. The Ω-only restriction is PRESERVED, shown by a negative control.**
Eliminating a `‖A‖` into `Type` must still be refused with the kernel's existing
error. **A surface form that widens the restriction is unsound**, and a passing
positive test cannot tell the difference. Write the refusal as a test.

**And the other refusal must NAME its remedy.** An introduction form written
where no type is expected must say that it needs an expected type or an
ascription — not `unresolved identifier`, not a bare parse error. **`D0` spent
an increment discovering by three successive failures that the surface admitted
nothing**, and each failure message pointed at the layer that refused rather
than at what to write instead. Assert the message text in a test.

**`AC-5`. `Derives(s) : Omega := ‖ FokDerivation s ‖` elaborates and
kernel-checks in a `.ken` file.** Not a paraphrase and not a smaller analogue —
the exact shape [[V3-FO-CHECKER-SOUNDNESS]] `D1b` needs.

**`AC-6`. Name every corpus-wide oracle the new token must satisfy, and check
each.** A new token has to survive the formatter round-trip at minimum
(`ken_fmt`), and formatting gates are collected by glob in crates this node does
not touch, so targeted validation cannot see them. **Enumerate them by grepping
for the collectors rather than naming the one you remember** — and note that
`FoKripke.ken` is in **zero** formatting gates today, so its being green proves
nothing about the file that will carry the new syntax.

**`AC-7`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only — `-p ken-elaborator`, never `--workspace`.

## Banned scope

- **Any kernel edit.** See `AC-2`.
- **Proving anything.** `checker_soundness` is [[V3-FO-CHECKER-SOUNDNESS]]'s.
  This node authors syntax and stops.
- **`FoKripke.ken`'s rule set, `fok_check_cert`, the Rust reference checker, or
  `attempt_fo`.** `D3` adds one definition to a `.ken` file; it changes no
  existing one.
- **Emitting `proved` for FO.**
- **Quotient surface syntax.** `Quot`/`QuotClass` have the same missing-spelling
  shape and are out of scope. If the work naturally covers them, say so and let
  the Steward decide whether to widen — do not widen unilaterally.
- **Redesigning `16 §6`.** If the spec's `elim_trunc` shape turns out not to fit
  what the kernel implements, that is a finding for the enclave.

## Sequencing

**`ready` at filing. `depends_on: []` — nothing gates it.** The kernel formers
are landed and typed, the spec section is written, and `D0` has already measured
exactly which layers refuse.

**`blocks` names [[V3-FO-CHECKER-SOUNDNESS]], and `depends_on` on that node
deliberately does NOT name this one.** The dependency is real but partial: only
`D1b` — `fok_derives` and `fok_classically_valid` — needs the spelling.
`D1a` and `D2` over there are dispatchable today. **Adding the operative edge
would mark a node with live work as gated**, and a blocked-looking node with an
idle ring is the failure this asymmetry exists to avoid. The relationship is
carried in prose on both nodes for that reason.

**This is lane 2 under the operator's 2026-08-15 two-lane directive** — it is
the FO Kripke embedding's blocker, not a third lane. It touches
`ken-elaborator`, contending with neither lane 1 (runtime) nor
[[V3-Z3-EMISSION-CONTROL]].

**Sizing note: this is `M` because the lexer precedent is 21 arms deep and the
kernel work is zero.** If the introduction-form spelling turns into a parser
ambiguity fight, that is the part to hand back on rather than push through.
