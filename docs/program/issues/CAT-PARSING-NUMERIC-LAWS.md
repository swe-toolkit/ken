---
id: CAT-PARSING-NUMERIC-LAWS
title: "give parse_digits_at, parse_nat_chars, and parse_int_chars the structural success/failure characterization the package header already promises -- that decimal parsing reports the exact character index of the first non-digit -- stating every law over the guard's Boolean value rather than over opaque Int ordering, and confining the claim to control flow, position, and diagnostic identity because the accumulator is opaque"
status: merged
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

**The public-signature arm is RULED.** Architect `evt_2yxag35v29bnp` settled
it; that ruling is operative and this section carries its content, not a
pointer to it. Preserve the exact public equations; do NOT publish the private
error representation and do NOT substitute weaker public shadows.

Keep private: `NumericErrorKind`, `EmptyInput`, `InvalidDigit`,
`numeric_error_code`, `numeric_diagnostic`. Retain the Tier-D private-inventory
assertions naming them.

Publish exactly these six direct names, because the public equations depend on
them: `numeric_empty_input_code` and `numeric_invalid_digit_code` (both
`DiagnosticCode`), `numeric_zero_accumulator`, `numeric_decimal_base` and
`numeric_minus_code` (all `Int`), and `pub fn negate_parsed`.

**One production body changes, and only one.** `numeric_error_code` consumes
the two public code constants instead of rebuilding `MkDiagnosticCode` literals.
This is behavior-preserving and it is load-bearing, not tidying: the Architect
probed the alternative and publishing the constants WITHOUT this redirect reds
`invalid_digit` with `Refl: the two sides of the goal are not convertible`. The
four public error-law types then use the public closed form
(`MkDiagnostic (locate position) numeric_invalid_digit_code`, and the `Zero` /
`Suc Zero` forms with `numeric_empty_input_code`), including the matching
`MkDiagnostic` in the corresponding `J` motive branch. Private proof bodies and
private AC witnesses may keep using private helpers where their own types do
not escape.

**Two files, and the second is required, not optional.** The package file, plus
`crates/ken-elaborator/tests/cat_tier_d_parsing_group_import.rs`, whose
`parsing_numeric_loader_visible_inventory_is_exact` (`:316`) pins an exact
`assert_eq!` against `published_module_surfaces` and then builds an import list
from that same set at `:332`. That pin moves from eight names to **exactly 23**:
14 direct and nine attached. **A count re-pin is not acceptable** -- the delta
is these names and no others.

The six added direct names are `negate_parsed`, `numeric_decimal_base`,
`numeric_empty_input_code`, `numeric_invalid_digit_code`, `numeric_minus_code`,
`numeric_zero_accumulator`, joined to the eight already landed.

The nine attached identities are `parse_digits_at::{accepted_digit, empty,
invalid_digit}`, `parse_int_chars::{bare_sign, empty, signed, unsigned}`, and
`parse_nat_chars::{empty, nonempty}`.

The generated direct-import token list filters only names containing `::`,
leaving the 14 direct names, because an attached law travels with its selected
public subject and is not a literal `subject::proof` import token. This is the
landed pattern from `cc1_nonempty_validation_acceptance.rs`. Nothing else in
the Tier-D file moves: no other assertion, no `#[ignore]`, no second test.

The Architect ran this exact closure against held WIP `b6ba73b85` in a scratch
worktree: the inventory test passed, asserting the 23-name set and importing
the 14 direct names together.

Beyond those two files: no new function on the parse path, import, module,
instance, primitive, postulate, `Axiom`, or trusted entry. `trusted_base()`
delta stays zero and the format direction is byte-unchanged. D1 remains private
and unchanged. Public API and trust prose state the six direct contract names
and the nine attached laws explicitly.

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

Hand back rather than work around if any of these holds. The third is already
ruled and is listed so nobody re-opens it.

- **A law cannot be stated without an `Int` ordering or value fact.** Do not
  add an `Axiom`, do not introduce a bounded digit table, and do not
  restate `leq_int`/`eq_int` behavior. Report which law needed it; under
  PRINCIPLES #17 that is a TCB contract and a separate Architect question.
- **D1 cannot be stated without an import.** The five-loader measurement above
  makes an import a fleet-visible change, not a local one. Report it; the
  frame amendment is the Steward's.
- **An exact public D0 signature needs a currently-private Numeric name.**
  **RULED AND DISCHARGED** by Architect `evt_2yxag35v29bnp`; the Deliverable
  now carries the authorized arm. This is no longer an open stop. Do not
  re-raise it, and do not choose a different arm.

## Hard-stop accounting

Per-chain `§1a` count for `(CAT-PARSING-NUMERIC-LAWS, public attached-D0
signature closure)` is **1**, advanced by `evt_2yxag35v29bnp`. Symptom entry 1
is recorded in exact child `2f829d3d224fc9e571177c6023915fd519dd09a1`: the
exact public laws were keyed on provider-private implementation identities
rather than a closed public contract. The next `§1a` Research trigger and
`§1b` predicate check are both at 3. The WIP audit at `evt_13fhszfz31sqk` was
not a stop and did not count.

## Not this node

- `show_int`, and any round trip that would need it.
- Any claim about the numeric value the accumulator computes.
- `parse_nat` and `parse_int`, which cross the opaque `String` boundary.
- Changes to the format direction; `format_digits_roundtrip` is landed.
- `Capability/Parsing/Parsing.ken.md` and `Capability/Parsing/Decoder.ken.md`,
  which are separate survey rows.
