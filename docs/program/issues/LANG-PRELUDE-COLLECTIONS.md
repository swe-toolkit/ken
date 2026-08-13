---
id: LANG-PRELUDE-COLLECTIONS
title: "`37 §9` requires the List combinators delivered in the prelude and they are declared inside a test file instead -- `prelude.rs` supplies `data List` and no operation over it, so a program that imports the prelude has a list type and no way to map over it; and `filter` was deferred on the ground that `Bool` is an opaque non-matchable primitive, which is no longer true, with the promised follow-on never filed"
status: ready
owner: language
size: S-M
gate: none
depends_on: []
blocks: []
github: null
origin: Steward measurement 2026-08-13 at origin/main=7f9eabbb, taken running the stay-one-release-ahead check after LANG-COMMENT-CLASSIFIER-SHARED merged and left Language with no successor. Five other candidate surface gaps were measured first and all five were already delivered; this one is real.
---

## What this is

Frame: `docs/program/wp/LANG-PRELUDE-COLLECTIONS.md`.

`spec/30-surface/37-strings-collections.md §9` requires WS-L to deliver the
`map`/`filter`/`fold`/`zip` combinators *"in the surface/elaborator + prelude"*.
`map`, `fold` and `zip` are declared in `tests/l3a_acceptance.rs:26-45`, inside
a test helper. `prelude.rs:428` supplies `data List a = Nil | Cons a (List a)`
and no operation over it.

That split is stated in the test file's own header — the combinators are
declared there *"driving the recursive-view-through-SCT wiring"*, with the
prelude supplying *"the types + Ω constants."* The mechanism landed; the
delivery did not.

## `filter`'s deferral reason is measurably gone

The same header defers `filter` because *"`Bool` is an opaque primitive (not
`data Bool = True | False`), so it is not pattern-matchable, and a CBV `if`
primitive would double-evaluate a recursive branch — a separate change (tracked
follow-on)."*

**`Bool` is matchable today**, measured at
`tests/case_eq_dependent_match_sugar.rs:106`, which matches a `Bool` binder
directly in `match b eqn: h { True |-> h ; False |-> h }`. `LANG-SURFACE-IF`
(merged) is what moved it. So `filter` is an ordinary `match` in `map`'s shape,
needing no `if` primitive and raising no double-evaluation question.

**The promised follow-on was never filed** — no node in
`docs/program/issues/` mentions it. Same shape as the finding that produced
`LANG-FOREIGN-NAME-FORMAT-CHARS`: an obligation recorded in prose, in a file
nobody re-reads, owned by nobody.

## Why `sort` is excluded, and why that is not a sizing preference

`l3a_acceptance.rs` AC6 has `sort` emit a conjoined `is_sorted ∧ Perm`
obligation. An open obligation is admitted as a kernel postulate and counts in
`trusted_base()` (`elab.rs:53-57`). Putting `sort` in the prelude would put an
undischarged obligation into every program that loads it — **a trusted-base
change, which is the operator's call and is live with them.** `AC-5` pins
`trusted_base()` unchanged so the exclusion is enforced rather than merely
stated.

`unfoldUpTo`, `Array`, `DecEq`/`Ord`, and the combinator laws-as-propositions
are all `37 §9` and all separate. The laws need a discharge route in Verify's
lane, not a prelude edit.

## What was measured and found already delivered

Recorded so the next sweep does not re-hunt them. All at `7f9eabbb`:

- `34 §4.1`/`§4.2` exhaustiveness and reachability, with a named unmatched
  witness — `ElabError::ExhaustivenessError` / `ReachabilityError`.
- `34 §4.3` the type-possible/index-impossible split — proven by
  `ds5b_dependent_match_refinement_acceptance.rs`, whose `tail` omits the
  index-impossible `VNil` arm and elaborates.
- `34 §2` the explicit `data ... : ... where` form, constructor signatures, and
  index-aware elaboration — `Decl::ExplicitDataDecl`, `parse_explicit_data_decl`.
  The four `SURF-gadt-*` WPs named in `34 §8` were built inside other nodes and
  have no tracker presence.
- `36 §4.3` `old` / pre-state binding — `LANG-SPACE-PRESTATE-BIND` (#1848). The
  surviving `OldPreStateUnsupported` reach is the intended fail-closed for
  contexts with no cell environment, pinned in `v1_acceptance.rs`.
- `35 §3` fixed-width overflow obligations (`ObligationKind::PartialPrim`) and
  `§5` conversions (`conversions.rs`).

## Amended mid-flight 2026-08-13: `AC-7`, a one-character rider

Adversary finding at `evt_7bt65xjjfsgr0`, verified by the Steward at
`424ab5da`. `error.rs:256-260` formats a bare `char` with `Display` where its
sibling `ForeignNameControlCharacter` (`:264-271`) correctly uses `{:?}` seven
lines below. It is the only site in the file that prints a bare `char` with
`{}`.

**The variant's population is exactly the invisible characters** — any non-ASCII
character in an identifier, including `Cf` format characters and bidi
overrides. So a zero-width character prints as `''` with nothing between the
quotes, and U+202E reorders the remainder of the terminal line including the
span numbers.

This is independent of the `Cf` threat-model question holding
`LANG-FOREIGN-NAME-FORMAT-CHARS`: that one is about deceiving a reader of
source, this is the compiler's own output. The Contention and "Not this node"
sections were swept to admit `error.rs` for this change and nothing else.

## Not this node

- `sort`, `unfoldUpTo`, `Array`, `DecEq`/`Ord` instances, the combinator laws.
- Anything in `error.rs` beyond `AC-7`'s single format specifier.
- Changing any combinator's meaning, argument order, or binder structure. The
  L3a declaration text is the pinned shape and moves unchanged.
- Extending `Bool`. The measurement is that it is already matchable.
