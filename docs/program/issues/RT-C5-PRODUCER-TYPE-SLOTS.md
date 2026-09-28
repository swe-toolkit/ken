---
id: RT-C5-PRODUCER-TYPE-SLOTS
title: "C5 regularity for values built by the interpreter's own producers: a closed, checked, reflexive cast whose index is a list produced by string_to_list_char, bytes_to_list, a directory listing or a bytes slice evaluates to its value instead of Unknown, because those producers fill the constructor type-parameter slot with the same comparable value source-built lists carry"
status: ready
owner: runtime
size: S
gate: architect
tier: T1
depends_on: [RT-C5-PRIMITIVE-TYPE-ARGUMENTS]
blocks: []
github: null
origin: "Adversary finding 2026-09-28 on M8 235b8cefe (evt_3xsyh4fxeqnmw), a measured pre-existing defect inside the class RT-C5-PRIMITIVE-TYPE-ARGUMENTS closed for source-built values. Spec 40-runtime/42-evaluation.md §3.6. Steward-filed per COORDINATION section 2."
---

# C5 on producer-built type slots

## Objective

A closed, well-typed, ground cast `cast A A refl a` evaluates to `a` when an
index of `A` is a value built by an interpreter producer, not only when it is
built from source, as spec 42 §3.6 requires.

## Settled inputs (Adversary `evt_3xsyh4fxeqnmw`, at `235b8cefe`)

- **Two representations of one slot.** Source-built `List Char` carries
  `OpaquePrimType` in its type-parameter slot after `235b8cefe`. The
  producers still fill that slot with `EvalVal::Unknown` or `Neutral`:
  - `build_list_char` (`crates/ken-interp/src/eval.rs:2849`, `:2858`);
  - `build_list_uint8` (`:2899`, `:2903`);
  - the fs directory listing (`:6196`, `:6210`);
  - `bytes_at` and `bytes_slice` results, through `type_args` (`:2561`).
- `eq_type_eq` (`:1230`) has no arm for either filler, so `cast_reduce`
  (`:1165`) returns `Unknown`. The comment at `:2825` ("carried as
  EvalVal::Unknown fillers") is now false for source-built values.
- **Repro.** It uses `data CI (xs : List Char)` with `MkCI : CI xs`, and
  `viewIdx = string_to_list_char "a"`. `cast (CI viewIdx) (CI viewIdx) refl
  MkCI` is kernel-checked and evaluates to `Unknown`. The same row with
  `srcIdx = Cons Char 97 (Nil Char)` reduces. The `List UInt8` row over
  `bytes_to_list (bytes_encode "a")` also gives `Unknown`.
- **No false positive was found**: `Unknown` and `Neutral` never compare
  equal.

## Deliverable

Every producer that fills a constructor type-parameter slot fills it with
the value a source-built term of the same type evaluates to. The five
Adversary rows then reduce, including view to source. An open index still
fails closed.

## Acceptance

- **AC-0 (probe, then ruling).**
  - Re-run the five rows at the landed base (Check 4).
  - Sweep every writer of a constructor type-parameter slot in
    `crates/ken-interp/src` (Check 3), and name any writer beyond the four
    above.
  - Propose the fix, either at the producers or as an `eq_type_eq` arm. Name
    what each fix can equate wrongly. The Architect rules before any build.
- **AC-1.** All five rows evaluate to `MkCI`. A differing-index negative over
  a producer-built index stays `Unknown`.
- **AC-2 (control).** Reverting the repair returns the producer rows to
  `Unknown`. `neutral_inductive_type_app_index_does_not_cast` stays green and
  unedited.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A repair that makes `Unknown` or `Neutral` compare equal to anything is an
  Architect stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
