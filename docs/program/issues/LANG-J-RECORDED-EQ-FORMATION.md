---
id: LANG-J-RECORDED-EQ-FORMATION
title: "Surface J refuses evidence whose Eq carrier is a Pi or Sigma type, with J's third argument must have an Eq type: infer_j whnf's the evidence type before matching Term::Eq, and observational whnf reduces Eq at a Pi carrier away. Read the endpoints from the recorded Eq formation first, and ascribe the evidence so the kernel reads the same recording"
status: merged
owner: language
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding F2 on 02b64897f (evt_5vdbykfmgw6v7); Architect ruling evt_4a0d1dbmpbdqt: real, fails closed, one S node on the language ring after LANG-SOLVED-INDEX-SUBSTITUTION and LANG-EQ-SUGAR-ENDPOINT-SORT. Steward-filed per COORDINATION section 2."
---

# Surface J reads the recorded Eq formation

## Objective

`J` applied to evidence at a function or pair carrier, such as
`h : Eq (Nat → Nat) f g`, elaborates and checks, as it already does at a
`Nat` carrier.

## Settled inputs (Architect `evt_4a0d1dbmpbdqt`, on `02b64897f`)

- **The refusal.** `infer_j` (elab.rs ~:12935) zonks the evidence type,
  whnf's it, and demands `Term::Eq`. Observational whnf reduces
  `Eq (Π ..) f g` to its pointwise form, so the match fails with "J's
  third argument must have an `Eq` type". It fails closed.
- **The kernel already reads recordings.** `j_endpoints` (check.rs ~:862)
  takes the endpoints from a checked `Term::Ascript`'s recorded type
  through `eq_formation_head` (check.rs:835), which peels only β, δ, let,
  ascription and function heads, never the Eq head. It falls back to the
  old whnf demand only for an unrecorded proof.
- **The ruled repair.**
  - Export a kernel helper `pub fn recorded_eq_endpoints(env, &ty) ->
    Option<(Term, Term, Term)>` that wraps `eq_formation_head` and returns
    the endpoints of a `Term::Eq` head.
  - `infer_j` takes `(A, a, b)` from today's whnf match first. Only
    where whnf does not yield `Term::Eq` (the Π and Σ carriers base
    refuses) does it read them through the helper (amendment 1,
    Architect `evt_35zvrpyjqj31j`). A program base admits keeps its
    endpoints, ascription and core byte for byte.
  - When the endpoints came from the formation, emit
    `Term::Ascript(eq_core, Term::Eq(A, a, b))` from the recorded
    endpoints, or reuse `eq_core` when it is already an `Ascript`. The raw
    `eq_ty` can be a β-redex the kernel cannot infer in ascription position
    (prelude `write_all_done`). The kernel checks the ascription, so
    nothing is trusted (Architect `evt_7ae8w6d6bgy84`).
- **The second site (Architect `evt_7ae8w6d6bgy84`).** The surface Refl
  gate in `check` (elab.rs ~:2522) and `synth_refl_proof` (~:2730) whnf
  the goal before matching `Term::Eq`, so bare `Refl` at
  `Equal (Nat -> Nat) f f` is refused with "Refl expects an `Eq`-shaped
  goal". Admit a Π whnf when `recorded_eq_endpoints` reads the goal, and
  add a `Term::Pi(..)` arm to `synth_refl_proof` that returns `Refl(t)`
  when the endpoints convert and "Refl: the two sides of the goal are not
  convertible" otherwise. The ruling's text is the code.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The helper export in `crates/ken-kernel` (visibility only), and the
`infer_j` and Refl-gate changes in `crates/ken-elaborator/src/elab.rs`, with the rows
below as a test target.

## Acceptance

- **AC-1.** `jf`, the exact witness
  `J (\b' _. Equal (Nat -> Nat) b' f) Refl h` with a bare `Refl`, and its
  ascribed variant elaborate and check. A Σ-carrier twin does too. `rp`,
  `theorem rp (f : Nat -> Nat) : Equal (Nat -> Nat) f f = Refl`, is
  accepted. `rpd`, with distinct `f g`, is refused with "not
  convertible".
- **AC-2.** Control `jn` at a `Nat` carrier is unchanged. Evidence whose
  type is not an Eq formation is still refused. A motive mistyped at the Π
  carrier is refused by the kernel. `trusted_base()` is unchanged.
- **AC-3 (sweep).** Classify each elab.rs site that whnf's before
  matching `Term::Eq` (~:2707, :6692, :6882, :12868, :13370, :15558,
  :15654) as reading a recording or not. Repair only those that the same
  refusal reaches, and name any other as a candidate for a separate node.
  The Refl gate and `synth_refl_proof` are such a site, repaired here.
- **AC-5 (amendment 1, respin).** `r_layer_tests::cat_tier_d_parsing_group_import`
  is 9/9, including the six publisher-named rows. A nested-J row whose
  evidence whnf's to a different Eq (the Arguments shape:
  `Eq (Option A) (Some a) (Some b)` consumed at `A`) checks, and fails
  with the recorded-first order restored.
- **AC-4 (mutation, QA).** Removing the helper read returns `jf` to the
  refusal while `jn` stays green. Removing the `Term::Pi(..)` gate clause
  returns `jf` and `rp` to "Eq-shaped goal" while `jn` and `js` stay
  green.

## Stop conditions

- Any kernel change beyond the visibility export, including any change to
  kernel checking: stop to the operator.
- A swept site that needs a different mechanism: report it rather than
  extending this node.

## Hard-stop inventory (§1b)

§1a: 2 (Architect `evt_7ae8w6d6bgy84`, `evt_35zvrpyjqj31j`).

1. The ruled witness reached a second whnf-before-Eq-match site (the
   surface Refl gate), and the ruled ascription operand was the unreified
   inferred type. Keyed on the whnf or inferred spelling of the Eq type
   instead of its recorded formation.
2. The ruled recorded-first J read replaced the base whnf endpoints for
   evidence whose whnf already is an Eq (a constructor-injectivity
   reduct, `Some a = Some b` to `a = b`), breaking a landed nested-J
   consumer, `Capability.Process.Arguments`
   `argument_slice_location_complete_at`. Keyed on preferring the
   recording unconditionally, instead of only where the base route
   refuses.
