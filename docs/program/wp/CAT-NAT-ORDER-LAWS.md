# WP frame: `CAT-NAT-ORDER-LAWS`

**Owner:** foundation (L3). **Size:** M. **Estimated capability tier: T2.**
**Reviewers:** Foundation QA + Architect (soundness/design) -> Steward M1-M3a
-> lieutenant M4-M9.

**Fixed inputs measured at `origin/main`
`b839fd63295e550c12524bd9f4804fdf28be7169`.** Re-measure before acting: every
coordinate below is a claim about the tree at that SHA, not a standing fact.

## 1. Objective

Land the inductive `sub`/`leq_nat` proof fragment that
`catalog/packages/Data/Numeric/Nat/Order.ken.md` currently defers in prose, so
the package's own stated algebra is discharged by checked terms rather than by
a reader's trust. `D1` is the released increment; `D2` is held.

This is the first of the seventeen proof-backfill nodes named by
`docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md` (landed `ec23d4ab6`), under
the operator's 2026-09-13 ruling and `docs/PRINCIPLES.md` #16.

## 1a. WHY THIS NODE IS FIRST — THE MEASUREMENT, NOT A PREFERENCE

The survey lists its seventeen follow-ons as a flat set. **They are not flat,
and the ordering is measurable.** The Steward measured one downstream node
before selecting this one.

`CAT-PARSING-CURSOR-LAWS` is the survey row whose obligation is already written
down as a `Prop` — `CursorLaws` at
`catalog/packages/Capability/Parsing/Cursor.ken.md:259-262`, a conjunction of
`CursorPeekHasRemaining` (`:222`), `CursorAdvanceProgress` (`:235`), and
`CursorEndValid` (`:250`), with no inhabitant for `arg_cursor_ops` (`:204`). It
therefore looks like the cheapest first cut: nothing to invent, only to inhabit.

**It is not, and the reason is this node.** `arg_cursor_remaining` (`:166`)
reduces through `arg_remaining_from` (`:152`) to

    add (sub (arg_length arg) offset) (arg_lengths_sum rest)

so `CursorAdvanceProgress` — advancing a peekable cursor strictly reduces the
remaining count — is discharged only by two facts about `sub`:

- **saturation**: `offset` past the argument length makes `sub len offset`
  reduce to `Zero`, which is what makes `arg_cursor_normalize` (`:175`)
  preserve the remaining count as it crosses an argument boundary;
- **strict decrease**: `sub len (Suc offset) < sub len offset` while
  `offset < len`.

**Neither exists.** `Order.ken.md:121-125` carries exactly three proofs
(`min::zero_left`, `max::zero_left`, `sub::zero_right`), all of which close
without induction, and `:146` carries the self-subtraction law as a **failing**
attempt inside a ` ```ken reject ` fence.

⇒ Framing `CAT-PARSING-CURSOR-LAWS` first would hand its implementer a node
whose first move is to invent `Nat` lemmas that belong in a different package —
either bloating that node's scope or, worse, landing `Nat` facts in
`Capability/Parsing`. **`Nat`'s `sub`/`leq` fragment is the dependency bottom of
this survey batch.**

**What this claim does NOT assert.** One downstream node was measured, not
seventeen. The ordering fact established is *`CAT-PARSING-CURSOR-LAWS` depends
on this node*, which is enough to select a first release; it is **not** a claim
that the remaining fifteen all do, and nobody should cite it as one. That
dependency is recorded on the cursor node itself
(`docs/program/issues/CAT-PARSING-CURSOR-LAWS.md`, filed `draft`).

## 2. Deliverables

### `D1` — the inductive `sub`/`leq_nat` fragment (THE RELEASED INCREMENT)

Four attached proofs in `catalog/packages/Data/Numeric/Nat/Order.ken.md`, over
the **current** definitions — `sub` at `:69-77`, `compare` at `:79-87`,
`leq_nat` the canonical provider relation.

1. **`sub::self_is_zero`** — `(n : Nat) : Equal Nat (sub n n) Zero`.
   The law the package names at `:139-148` and currently carries only as a
   rejected `Refl`. Replace that ` ```ken reject ` block with the checked
   proof and rewrite the surrounding prose, which asserts the gap.
2. **`sub::zero_left`** — `(b : Nat) : Equal Nat (sub Zero b) Zero`.
3. **`sub::saturates`** — `sub` reduces to `Zero` once the subtrahend reaches
   the minuend: from `leq_nat a b` being `True`, conclude
   `Equal Nat (sub a b) Zero`.
4. **`sub::suc_decreases`** — the strict-decrease law: from `leq_nat (Suc b) a`
   being `True`, conclude that `sub a (Suc b)` is strictly below `sub a b`
   under `leq_nat`.

**Author the exact statements of 3 and 4 in the package's own idiom.** The
shapes above are the semantic content, not a signature to transcribe: how a
`True` Boolean hypothesis is carried (an `IsTrue` premise, as
`Order.ken.md:93` uses, or an `Equal Bool ... True` premise, as
`Cursor.ken.md:222-234` uses) is the author's call. Pick one, use it for both,
and say in the package prose which and why — the downstream consumer has to
match it.

### `D2` — the min/max/compare theory. HELD, NOT IN THIS RELEASE.

`min`/`max` commutativity, associativity, idempotence; `compare`'s agreement
with `leq_nat` on all three results; the `add`/`sub` round trip. Framed when
`D1` lands. **Do not start it.** If `D1` finishes inside the turn, stop and
report, per `steward.md` §4b.

## 3. Acceptance criteria

- **`AC-1` — the four `D1` proofs are checked terms in the package** and the
  package elaborates. The evidence is the elaborator accepting the file, not a
  narrative that the proofs are correct.
- **`AC-2` — NO NEW TRUST.** No `Axiom`, postulate, primitive, `Omega` path
  carrier, TCB change, or kernel change is added.
  *Control:* the diff touches `catalog/packages/Data/Numeric/Nat/Order.ken.md`
  and nothing under `crates/`. A diff that needs a `crates/` change has found
  something this frame did not anticipate — that is a hard stop and a report,
  not a scope extension.
- **`AC-3` — NO DEFINITION CHANGED.** `min`, `max`, `sub`, `compare`, and
  `leq_nat` keep their current bodies. The proofs adapt to the definitions.
  *Control:* the diff shows no edit inside `:60-87`.
- **`AC-4` — THE REJECTED BLOCK IS GONE AND ITS PROSE IS CORRECTED.**
  `:146`'s ` ```ken reject ` fence and the `:139-145` prose asserting the law is
  unproved must both go. *Control:* grep the package for `self_is_zero_wrong`
  and for `deliberately doesn't prove` — both must return nothing.

  **This AC exists because the prose is load-bearing and is not near the
  proof.** A checked `sub::self_is_zero` sitting six lines below a paragraph
  that says the law is deliberately unproved leaves the package asserting both.

## 4. Authoring notes — read these before writing a proof

**The induction idiom already exists, worked, in a sibling package.**
`catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md:30-44` is the template:

    proof zero_r for add (a : Nat) : Equal Nat (add a Zero) a = Refl

    pub proof zero_l for add (a : Nat) : Equal Nat (add Zero a) a =
      match a {
        Zero ↦ Proved;
        Suc a2 ↦ cong Nat Nat (add Zero a2) a2 Suc ((proof zero_l for add) a2)
      }

Induction is a `match` on the structural argument; the induction hypothesis is
the proof's own name applied to the smaller argument —
`(proof zero_l for add) a2`. `cong`, `sym`, and `trans` come from
`Core/Logic/Transport`. `assoc` and `comm` at `:46-65` show the multi-binder
and two-lemma-composition shapes.

**`Proved` vs `Refl`, and the template demonstrates both in adjacent
proofs.** An `Equal` goal whose two sides are CLOSED and reduce to the same
value whnfs to `Top`, so its inhabitant is **`Proved`**, not `Refl` — `Refl`
only inhabits a goal that stays `Eq`-shaped, i.e. with at least one side
neutral (stuck on a parameter). That is exactly why `zero_l`'s `Zero` arm is
`Proved` while `suc_l`'s `Zero` arm (`:41-44`) is `Refl`. A trivial-looking
`Refl` failing with *"Refl expects an `Eq`-shaped goal"* is this, and the fix
is `Proved`. The package's own `:127-138` prose explains the same distinction
for its three existing proofs.

**`recursive result for` is NOT how you write this.** It is the
former-nested-induction-hypothesis construct, for a recursive occurrence buried
under a positive former. Ordinary structural recursion — including every proof
in this node — is a **plain self-call**, as the template shows. Using
`recursive result for` on a direct recursive argument is rejected at
elaboration with `StructuralResultOutOfScope`, upstream of anything else, which
makes it read as a failure of the proof rather than of the idiom.

## 5. Contention check

**Clear.** `catalog/packages/Data/Numeric/Nat/Order.ken.md` last moved well
before the measurement SHA and is not touched by either in-flight lane: L1 is in
`crates/ken-cli` + `crates/ken-elaborator/src/prelude.rs`
(`RT-PLANNER-KRET-GRAFTED-SPINE`), L2 in `crates/ken-elaborator`
(`LANG-REWRITE-DESCENT-FRAME-TAX`). No `wp/` branch holds this path.

**Confirm before branching** (`git fetch` first) — the check is a moment, not a
standing fact, and both lanes are mid-flight as this is released.

## 6. Build discipline

Targeted builds only, through `scripts/ken-cargo`, scoped to the crate you
touch. **Never `--workspace`** (`agent/COORDINATION.md` §12, operator hard
rule). The workspace build and the conformance suite run in CI; a
"no-regression" criterion means green in CI, never a local full run.
