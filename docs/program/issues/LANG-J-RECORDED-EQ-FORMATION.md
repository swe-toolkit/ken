---
id: LANG-J-RECORDED-EQ-FORMATION
title: "Surface J refuses evidence whose Eq carrier is a Pi or Sigma type, with J's third argument must have an Eq type: infer_j whnf's the evidence type before matching Term::Eq, and observational whnf reduces Eq at a Pi carrier away. Read the endpoints from the recorded Eq formation first, and ascribe the evidence so the kernel reads the same recording"
status: ready
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-EQ-SUGAR-ENDPOINT-SORT]
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
  - `infer_j` reads `(A, a, b)` from the zonked `eq_ty` through the
    helper first, and falls back to today's whnf match.
  - When the endpoints came from the formation and `eq_core` is not
    already an `Ascript`, emit `Term::Ascript(eq_core, eq_ty)`. The kernel
    checks the ascription, so nothing is trusted.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The helper export in `crates/ken-kernel` (visibility only) and the
`infer_j` change in `crates/ken-elaborator/src/elab.rs`, with the rows
below as a test target.

## Acceptance

- **AC-1.** `jf`, J over `h : Eq (Nat → Nat) f g`, and its ascribed
  variant elaborate and check. A Σ-carrier twin does too.
- **AC-2.** Control `jn` at a `Nat` carrier is unchanged. Evidence whose
  type is not an Eq formation is still refused. A motive mistyped at the Π
  carrier is refused by the kernel. `trusted_base()` is unchanged.
- **AC-3 (sweep).** Classify each elab.rs site that whnf's before
  matching `Term::Eq` (~:2707, :6692, :6882, :12868, :13370, :15558,
  :15654) as reading a recording or not. Repair only those that the same
  refusal reaches, and name any other as a candidate for a separate node.
- **AC-4 (mutation, QA).** Removing the helper read returns `jf` to the
  refusal while `jn` stays green.

## Stop conditions

- Any kernel change beyond the visibility export, including any change to
  kernel checking: stop to the operator.
- A swept site that needs a different mechanism: report it rather than
  extending this node.
