---
id: CAT-COLLECTIONS-NTH-LAWS
title: "Land the nth/length pair in Data/Collections/Derived.ken.md: nth returning Some bounds the index below the length, and an index at or beyond the length returns None -- the general List facts CAT-PARSING-CURSOR-LAWS reaches in the base case of its first law and cannot discharge in-package."
status: ready
owner: foundation
size: M
gate: none
tier: T2
depends_on: []
blocks: [CAT-PARSING-CURSOR-LAWS]
github: null
origin: "Cut by Architect ruling evt_1391bs7gcxara (2026-09-19) after the Steward framed CAT-PARSING-CURSOR-LAWS with the nth gap as a stop condition. The Architect ruled a stop condition is the wrong instrument: the gap is reached with certainty in the base case of CursorPeekHasRemaining, so the stop would be a scheduled turn that ends in this node being cut anyway. Not one of the seventeen proof-backfill follow-ons in docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md -- it is a newly measured predecessor to one of them."
---

# Proof-backfill predecessor: the `nth`/`length` pair

`Data/Collections/Derived.ken.md` defines `nth` (`:91`) and `length` (`:169`)
and relates them nowhere. `CAT-PARSING-CURSOR-LAWS` needs both directions and
has no cursor structure left to induct on at the point it needs them.

# Frame

Releasable now. Nothing blocks it: the facts it needs already exist.

## Why this is a node and not a stop condition

Architect ruling `evt_1391bs7gcxara`, derived at the object rather than
assumed. `arg_cursor_peek` is two `nth` calls at two different layers — `nth
Bytes index args` and `nth UInt8 offset (bytes_to_list arg)`. Induction on
`index`/`args` discharges the outer layer with no lemma, and lands in
`index = Zero`, `args = Cons arg rest`. With `rest = Nil` — a one-argument
cursor, the common case — the goal reduces to `offset < arg_length arg`, whose
only hypothesis is `nth UInt8 offset (bytes_to_list arg) = Some value`. Since
`arg_length` is `bytes_nat_length` is definitionally
`length UInt8 (bytes_to_list arg)`, the residual obligation is exactly this
node's first direction, over a bare `List UInt8`.

The escape was checked and is closed: `arg_cursor_normalize` does test
`cursor_nat_lt offset (arg_length arg)`, so a *normalized* cursor carries the
invariant by construction — but the three laws at `Cursor.ken.md:222-258` all
open `(cur : c) →` over the bare carrier with no normalization or reachability
precondition, so the normalizer's check is not available as a hypothesis.

## Settled inputs

Measured. Do not re-derive.

**1. No new package edge, so there is no cycle to check.** `Derived.ken.md:66`
already imports `Core.Classes.LawfulClasses (bool_and, bool_leq)`; widen that
list to add `IsTrue` and `leq_nat`. Widening an import list on an edge that
already exists cannot introduce a cycle — the edge is the thing a cycle would
be made of, and it is already there. (Corroborating but not the argument:
`LawfulClasses` imports only `Core.Logic.*` and `Data.Text.StringBijection`,
not `Data.Collections.Derived`.)

**2. Nothing in the catalog already states this.** `nth` appears in ken-fenced
code in exactly four files — `Parsing/Cursor`, `Parsing/Parsing`,
`Process/Arguments`, `Collections/Derived` — and zero declarations relate it to
`length`. Swept catalog-wide per declaration with whitespace collapsed, so a
wrapped statement cannot hide.

**3. Scrutinee order.** `nth a n xs` matches `xs` first, then `n` (`:91`).
`length a xs` matches `xs` (`:169`). `leq_nat m n` matches `m` first, then `n`
(`LawfulClasses.ken.md:489`). Neither `nth` nor `leq_nat` reduces until both
scrutinees are in WHNF; an induction that splits only one leaves the other
stuck. That reads like a wall and is a missing case split.

**4. `IsTrue` is an alias.** `IsTrue (b : Bool) : Prop = Equal Bool b True`
(`LawfulClasses.ken.md:54`) — non-recursive and single-clause, so it unfolds on
any argument without needing its scrutinee in WHNF. The two spellings are one
proposition. Note that `LawfulClasses`' own `leq_nat` proofs use the
`Equal Bool ... True` spelling (`:499`), so the sibling convention in that file
differs from what `AC-2` requires here — see `AC-2` for why the consumer wins.

**5. Widening this import breaks one Rosetta consumer, by design, and the
repair layer is settled.** `crates/ken-cli/tests/rosetta.rs:147` pins the
string `import Core.Classes.LawfulClasses (bool_and, bool_leq)` exactly, and
`remove_flattened_import` (`:66-76`) panics at `:70` when it does not occur.
`ken run` takes one flat source unit, so this compatibility runner flattens
`Derived`'s providers and subtracts the now-redundant import edges; an import
that gains names is a miss, and the panic is the runner **working**. Settled
input 1 requires the widening and `AC-2` requires the spelling that forces it,
so this is not an avoidable collision — the candidate cannot satisfy this frame
without it.

**The same collision one package over was already repaired and cleared.**
`47b811be4` (`CAT-NAT-ORDER-LAWS` D1) hit it when D1's new `Order` exports
leaked into flat source as `UnresolvedCon leq_nat`, breaking 4/16 examples. Its
repair replaced subtractive surgery over `Nat.Order` with an additive
`["pub fn min", "pub fn sub"]` allowlist, 16/16 green, and the Adversary
reviewed that repair and found NO DEFECT. **The direction of that repair is the
part to carry: it moved a fail-open construct to a fail-closed one.**

## Deliverable

Two exported declarations in `Data/Collections/Derived.ken.md`, for a general
element type `a`:

- `nth a n xs = Some a v` implies `Suc n ≤ length a xs`;
- `length a xs ≤ n` implies `nth a n xs = None a`.

Names and binder order are the implementer's. The **statement form is not** —
see `AC-2`.

## Stop condition

**If either direction needs a `Nat` fact that `Data/Numeric/Nat/Order.ken.md`
and `Data/Numeric/Nat/Arithmetic.ken.md` do not already carry, stop and report.
Do not land a `Nat` fact inside `Data/Collections`.** That is the same rule
that produced this node, one layer out, and it is the direction that fails
closed.

## Acceptance criteria

**`AC-1` — both directions are inhabited and neither is degenerate.**
*Control, both halves required:* the package elaborates with both terms
present; **and** replacing either direction's term with `Refl` makes the
package go RED, restored byte-exact afterwards. A positive check alone passes
for a statement that is accidentally trivial.

**`AC-2` — the hypothesis is spelled `IsTrue` over `leq_nat`.** Carry it as
`IsTrue (leq_nat (Suc n) (length a xs))`, not `Equal Bool (...) True`.

*Control:* each added statement's hypothesis is grepped and matches
`IsTrue (leq_nat`, with `Equal Bool` absent from both hypothesis positions.
**This half is textual on purpose.** The two spellings are definitionally the
same proposition, so no elaboration, no mutation, and no test can separate
them — `AC-1`'s control is invariant under this choice and cannot discharge it.

*Why it is required at all, given that nothing breaks inside Ken either way:*
the consumer
is `CAT-PARSING-CURSOR-LAWS`, reaching these through `Order`'s `suc_decreases`,
which is in `IsTrue` form. The argument is legibility at the use site, not
provability and not term size — there is no conversion to avoid. It needs a
criterion because the nearest model of a `leq_nat` fact,
`pub proof refl for leq_nat ... : Equal Bool (leq_nat x x) True`
(`LawfulClasses.ken.md:499`), uses the **other** spelling, so an implementer
copying the neighbour diverges from the consumer and nothing reds.

*One report, not a deliverable:* if Ken's conversion checker turns out to treat
`IsTrue` as **opaque** at these use sites rather than unfolding it, say so
rather than quietly relying on the `IsTrue` form working. Nobody has verified
it; the definition makes it free in principle. If it is opaque, the spelling
stops being legibility and becomes load-bearing, this `AC` changes character,
and its control above becomes the least interesting thing about it. The node
goes green either way, which is what makes it cheap to miss.

**`AC-3` — `nth` and `length` are unchanged.** This node relates the two
functions the package already ships; it does not redefine either to make the
relation provable. *Control:* `nth` (`:91`) and `length` (`:169`) are
byte-identical to their pre-candidate text, extracted and compared
programmatically, not by eye.

**`AC-4` — no new trust, and the diff reaches product source nowhere.**
*Control, both halves:* the added lines contain no `Axiom`, postulate,
primitive, `Omega` carrier, or kernel/TCB surface; **and** the diff touches
`catalog/packages/Data/Collections/Derived.ken.md` plus, under `crates/`,
**only test harnesses that mechanically reconstruct a consumer's view of a
catalog package.** Any path under `crates/**/src/**`, or any other non-test
path, is a hard stop and a report, not a scope extension.

**This `AC` has been wrong TWICE, both times mine, and both times in the same
direction — so this revision changes its SHAPE and not its bound.** It first
read *"under `crates/`, nothing at all"*, which forbade the only repair `AC-2`
makes necessary; the candidate went CI-red *because* it complied. It then read
*"at most `crates/ken-cli/tests/rosetta.rs`"*, naming the one such harness I
had found by grep — and `crates/ken-elaborator/tests/cat_derived_pub_export.rs`
is a second one, which surfaced only once the Rosetta panic stopped masking it.

**The defect was not the bound. It was enumerating a population I had only
sampled.** I measured the consumers I could find and wrote the census into the
`AC` as if finding them completed it, so each new member arrives as a frame
failure. **A predicate can report being incomplete; a list cannot.** Hence the
clause above is now a property — *reconstructs a consumer's view of a catalog
package* — and the hard stop lives where it was always supposed to: at
**product source**, which is what "no scope extension" was protecting.

Both hard stops the ring returned under the old wording were **correct** and
neither widened scope on its own. That is the clause working.

**`AC-5` — the Rosetta repair restores the consumer without weakening its
guard.** The runner's exact-string, exact-cardinality matching is a fail-closed
roster: it is *supposed* to panic when a provider edge changes shape, so that a
catalog change cannot silently stop being flattened. *Control, both halves:*

- **Rosetta is 16/16** on the repaired object, by the same command as
  `47b811be4`'s discriminator.
- **The pin stays exact.** No substring, prefix, regex, "contains", or
  tolerate-extra-names matching, and no removal of a cardinality assertion.
  Update the pinned text to what `Derived` now carries; do not teach the pin to
  accept a class of strings. A pin that tolerates a widened import also
  tolerates the next one silently, which converts the one construct here that
  fails closed into one that fails open — the exact direction `47b811be4`
  moved away from.

*Two hazards this frame can name, both measured; the repair shape is the
ring's:* updating the pinned string alone subtracts the import edge while
nothing supplies `IsTrue` or `leq_nat` to the flat unit — `collections_prelude`
allowlists only `["pub fn bool_leq", "pub fn bool_and"]` from `LawfulClasses`
(`rosetta.rs:115`) — which is the `UnresolvedCon` shape D1 hit. And
`pub fn IsTrue` (`LawfulClasses.ken.md:54`) is a single-line braceless
declaration, so `flattened_braced_declaration` does not panic on it: it scans
forward to the next `{` in the file and returns a range over an unrelated
declaration. **That helper fails open on exactly this input**, so extending the
allowlist to `IsTrue` needs more than adding a string to it.

*If a repair cannot stay inside a consumer-view harness, stop and report rather
than reaching further.* Product source is a different node, not a wider one.

**`AC-6` — the export-surface harness publishes the two new proofs, and its
fail-closed arms are untouched.**
`crates/ken-elaborator/tests/cat_derived_pub_export.rs`
synthesizes, per attached proof parsed out of `Derived.ken.md`, a probe file
that imports **only the subject** and then restates the theorem
(`:177-189`). That construction assumes **every attached proof's statement
mentions nothing but its subject and base-environment names** — true of the
three `list_append` monoid laws it was written against, false of a proof whose
hypothesis is `IsTrue (leq_nat ...)`. The probe therefore cannot state the
theorem, and `UnresolvedCon { name: "leq_nat" }` is that assumption expiring.
Measured; this is the same class as settled input 5, one harness over.

*Control, all four halves:*

- **Both proofs are repaired, not the one the panic names.** Queries are sorted
  by surface (`:192`), so `nth::at_or_beyond_is_none` panics first and
  `nth::some_below_length` never runs. Their statements carry the same two
  foreign names. A repair keyed on the reported name leaves the second live.
- **The literal contract set gains exactly these two surfaces**, so `:229-241`
  goes from eleven names to thirteen. *This assertion is the node's fail-closed
  backstop and the reason the harness may be touched at all:* it refuses to let
  `Derived`'s loader-visible surface grow without someone saying so in a
  literal. **Control on the control:** deleting either new name from that set
  must make the test RED, restored byte-exact. If it stays green, the
  `assert_eq!` is no longer discriminating and that is a hard stop.
- **The error classification is not widened.** `UnboundName` is the one class
  read as "not published" (`:211-218`); every other class panics (`:219`).
  **Do not move `UnresolvedCon` into the graceful arm.** That would reclassify
  a genuine resolution failure as a legitimate absence, shrink the expected set
  instead of growing it, and convert this harness's one fail-closed construct
  into a fail-open one — the same direction `AC-5` forbids and `47b811be4`
  moved away from.
- **The two prose enumerations move with the literal.** The module doc comment
  (`:1-9`) and the `assert_eq!` message (`:242-244`) both say *"the eight
  operations plus the three migrated `list_append` monoid-law proofs"*. Leaving
  that while the set says thirteen is exactly the prose-versus-surface drift
  `CAT-NAT-ORDER-LAWS` `AC-D2-3` was written for.

*The spelling is not a degree of freedom.* The cheapest way to make this red go
away is to restate the two theorems using only base-environment names, and
**that is forbidden** — `AC-2` requires `IsTrue (leq_nat ...)` and wins here. If
the only available repair is a statement change, **stop and report**: that
promotes `AC-2`'s "one report, not a deliverable" clause into a live finding
about what a catalog statement may mention, which is a real result and not a
workaround.

*Noted for the successor, not a criterion:* a consumer citing these proofs must
import `IsTrue` and `leq_nat` alongside `nth`. That is ordinary module
discipline rather than a defect, and `CAT-PARSING-CURSOR-LAWS` needs those names
regardless — `Order`'s `suc_decreases` is already in `IsTrue` form. Recorded so
the successor's frame does not rediscover it as a surprise.
