---
id: CAT-PARSING-NUMERIC-LAWS
title: "give parse_digits_at, parse_nat_chars, and parse_int_chars the structural success/failure characterization the package header already promises -- that decimal parsing reports the exact character index of the first non-digit -- stating every law over the guard's Boolean value rather than over opaque Int ordering, and confining the claim to control flow, position, and diagnostic identity because the accumulator is opaque"
status: active
owner: foundation
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 / PRINCIPLES #16. Selected and framed by the Steward 2026-09-20 as L3's successor while CAT-SCHEMA-LAWS is active, and NOT taken from the survey's one-line recommendation: the package was measured first. Chosen over CAT-CONFIGURATION-DECODER-LAWS, which is the better node but whose precursor is CAT-SCHEMA-LAWS itself -- framing it now would ground a frame on an unmerged commit. CAT-CONSOLE-TEXT-LAWS and CAT-JSON-LAWS were measured and rejected earlier (reasons in CAT-SCHEMA-LAWS origin). All current-code facts measured at origin/main dcbb9648f39ac8d9dd8698635018b392e5a58978."
---

# Located decimal parsing laws

`Capability/Parsing/Numeric.ken.md` opens by promising that parsing "reports
the exact character index of the first failure". That promise is carried
entirely by one acceptance test. The format direction of this same package is
already proven structurally; the parse direction is not.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Settled inputs -- measured at `dcbb9648f`. Do not re-derive.

**What is already proven, and is the posture to match.** Three
`numeric_argument_origin_*_faithful` theorems (`:62,70,78`) pin the origin
projections by `Refl`; `proof valid for decimal_digit_to_char` (`:180`) is
attached and public; `theorem format_digits_roundtrip` (`:225`) proves the
structural round trip by induction with `cong`/`trans`. `Core.Logic.Transport`
is therefore already imported and supplies everything the induction needs.

**Every guard must be lifted as a Boolean, never as an `Int` fact.**
`char_to_digit` (`:94`) branches on
`and_bool (leq_int (48 : Int) (charToInt c)) (leq_int (charToInt c) (57 : Int))`,
and `parse_int_chars` (`:133`) branches on `eq_int (charToInt c) (45 : Int)`.
`leq_int` and `eq_int` are kernel-`Neutral`; the survey classifies
`Core/Classes/LawfulClasses.ken.md` as TCB-contract-axiom precisely because
their properties are contracts, not reducible structure. So state each law
with the guard's **value** as a hypothesis -- "given
`char_to_digit c = Some digit`" or "given `eq_int (charToInt c) (45 : Int) =
True`" -- and match on that. A law that tries to derive the digit range from
the ordering is asking Ken induction to prove a primitive, and will hard stop.

**The accumulator is opaque and no law may speak about its value.** The fold
step is `add_int (mul_int accumulator (10 : Int)) digit` (`:113`) over
arbitrary-precision `Int` with no destructor. Laws state the fold's shape as a
definitional equation; they never claim the result equals the decimal value of
the digits. That is the same boundary `show_int` sits behind (`:156`, a named
deferral).

**The `String` entry points are out of reach and the `_chars` forms are not.**
`parse_nat` and `parse_int` (`:144,147`) apply `string_to_list_char`, a
kernel-`Neutral` primitive whose retraction axiom is the separately homed
assumption. State every law on `parse_nat_chars` and `parse_int_chars`.

**A NEW IMPORT REDS FIVE SUITES, and this is measured, not cautionary.**
`Numeric.ken.md` is `include_str!`-loaded by
`cc2_text_codec_numeric_acceptance.rs`, `cc4_diagnostic_core_acceptance.rs`,
`cc7_argparse_acceptance.rs`, `cc8_env_config_decoder_acceptance.rs`, and
`cat_tier_d_parsing_group_import.rs`. **None of the five loads
`Data/Collections/Derived.ken.md`.** So the first-failure law cannot be stated
by indexing with `nth`, and cannot borrow `length` or `add` -- reach for a
locally defined predicate instead, as `CAT-PROPERTY-LAWS` did with
`property_nat_lt`.

**Coverage is automatic and no runtime witness is owed.**
`cc2_text_codec_numeric_acceptance.rs:264` elaborates the whole file
(`.expect("Numeric.ken.md must elaborate")`), so a proof that fails to
typecheck reds it. `cc2_checked_code_has_zero_axiom_and_zero_trusted_base_delta`
(`:230`) is a landed trust-delta gate over this exact file.

## Deliverable

**D0 -- the dispatch and step equations, attached and public.** For
`parse_digits_at`: the `Nil` case equals `Ok accumulator`; a refused head
equals `Err (numeric_diagnostic locate InvalidDigit position)` at exactly the
**current** `position`; an accepted head equals the recursive call at exactly
`Suc position`. For `parse_nat_chars`: empty input refuses with `EmptyInput`
at `Zero`; a non-empty input equals the worker started at `Zero`. For
`parse_int_chars`: empty input refuses with `EmptyInput` at `Zero`; a bare
sign refuses with `EmptyInput` at `Suc Zero`; a signed non-empty input equals
`negate_parsed` of the worker started at `Suc Zero`; an unsigned input equals
the worker started at `Zero`.

**D1 -- the success characterization.** Define a local predicate over
`List Char` stating that every element is accepted by `char_to_digit`, and
prove that `parse_digits_at` returns `Ok` on any list satisfying it. The
predicate is local to this package and adds no trust.

**Two files, and the second is required, not optional.** The package file, plus
`crates/ken-elaborator/tests/cat_tier_d_parsing_group_import.rs`, whose
`parsing_numeric_loader_visible_inventory_is_exact` (`:316`) pins an exact
eight-name `assert_eq!` against `published_module_surfaces` and then builds an
import list from that same set at `:332`. D0's attached members cannot leave
that pin green, so the candidate carries a NAMED-SET delta to it. **A count
re-pin is not acceptable** -- show the delta is exactly the intended names.

Follow the landed precedent rather than inventing a shape:
`crates/ken-elaborator/tests/cc1_nonempty_validation_acceptance.rs` solved this
for `CAT-NONEMPTY-APPEND-HEAD-LEFT` by adding the attached name to the
published-surface expectation while EXCLUDING `::`-bearing names from the
import list, because an attached proof is not a `::`-selectable import token.
Nothing else in the Tier-D file moves: no other assertion, no `#[ignore]`, no
second test.

Beyond those two files: no new function on the parse path, import, module,
instance, primitive, postulate, `Axiom`, or trusted entry. `trusted_base()`
delta stays zero and the format direction is byte-unchanged.

## Acceptance criteria

**AC-1 -- the position arithmetic is pinned, and an off-by-one reds.** The
exact character index is this node's whole subject, so two mutations,
each restored byte-exact: replace `Suc position` with `position` in the
accepted-head equation, and it must RED; replace `Suc Zero` with `Zero` in the
bare-sign equation, and it must RED. A law set that survives either mutation
has stated that parsing terminates somewhere, not that it reports where.

**AC-2 -- the two error kinds are independently load-bearing.** Three separate
refusal witnesses that a single combined negative cannot satisfy: `EmptyInput`
at `Zero` from an empty list, `EmptyInput` at `Suc Zero` from a bare sign, and
`InvalidDigit` at a nonzero position from a digit-prefixed bad character (the
acceptance test's `"12x4"` shape qualifies, stated on the `_chars` form).
Report the position each witness derives. `EmptyInput` never arises from the
worker and `InvalidDigit` never arises from an entry point; witnesses that do
not separate the two do not satisfy this.

**AC-3 -- D1 genuinely inducts.** Remove the inductive appeal from the success
law and it must red locally. Report the diagnostic. A success law discharged
without recursion is pinning the `Nil` case and nothing else.

## Stop condition

Hand back rather than work around if either holds:

- **A law cannot be stated without an `Int` ordering or value fact.** Do not
  add an `Axiom`, do not introduce a bounded digit table, and do not
  restate `leq_int`/`eq_int` behavior. Report which law needed it; under
  PRINCIPLES #17 that is a TCB contract and a separate Architect question.
- **D1 cannot be stated without an import.** The five-loader measurement above
  makes an import a fleet-visible change, not a local one. Report it; the
  frame amendment is the Steward's.
- **An exact public D0 signature needs a currently-private Numeric name.**
  `numeric_diagnostic`, `EmptyInput`/`InvalidDigit` and the named `Int`
  literals are private, and the surface parser rejects inlining them into a
  public proof type. Publishing them, or weakening the public claim while the
  exact equation stays private, changes the package's published surface. That
  is the **Architect's** call, not the ring's and not the Steward's: stop and
  route it. If the chosen arm makes a public law weaker than the private
  equation it names, that asymmetry is stated in the package rather than left
  for a reader to find.

## Not this node

- `show_int`, and any round trip that would need it.
- Any claim about the numeric value the accumulator computes.
- `parse_nat` and `parse_int`, which cross the opaque `String` boundary.
- Changes to the format direction; `format_digits_roundtrip` is landed.
- `Capability/Parsing/Parsing.ken.md` and `Capability/Parsing/Decoder.ken.md`,
  which are separate survey rows.
