---
id: KERNEL-SCT-TELESCOPE-CANON
title: "The SCT gate derives each group member's arity by counting leading Lam binders in the elaborated body (count_params, sct.rs:623), so once the c-elab result-refinement transport wraps a mutual-recursion body the leading-Lam count diverges from the declared arity, skip_lams skips the wrong number, the size-change matrices are mis-dimensioned, and a terminating transport-carrying clique wrongly reds NotTerminating -- repair the arity to the DECLARED Pi telescope (route A, telescope-canonicalization) with admit==analyze on the same eta-long body, the SCT-pass enabler LANG-INDEXED full admission and V3-FO-CHECKER-SOUNDNESS D3 are blocked on"
status: ready
owner: kernel
size: M
gate: operator
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-22, on the operator's authorization (\"tcb change authorized. proceed.\") of the route-A kernel SCT successor that LANG-INDEXED-RECURSIVE-IH-DISCHARGE and V3-FO-CHECKER-SOUNDNESS D3 name as their SCT-pass gate. The c-elab result-refinement transport landed (LANG-INDEXED accepted partial, squash 93d82a398): the narrowed AC-7 -- held-D3 bodies ELABORATE + pass kernel_check -- is met, but FULL admission is kernel_check AND SCT-pass, and the transport-carrying mutual-recursion clique reds the SCT gate. Route A (SCT arity from the declared Pi telescope, not the deep-lambda body heuristic) was specified in-thread by the Architect + research and is captured durably at D0 (an in-thread ruling is not a durable deliverable). TCB change: it modifies the trusted size-change termination gate (crates/ken-kernel/src/sct.rs). gate: operator, and the operator's authorization above satisfies it. Steward-filed per COORDINATION section 2. Estimated capability tier: T1 (soundness-bearing termination-gate change; the arity widening must not admit a nonterminating recursion -- the negative control below is mandatory)."
---

# AUTHORIZED and released -- 2026-08-22 (operator, "tcb change authorized. proceed.")

This modifies the trusted SCT termination gate (`crates/ken-kernel/src/sct.rs`).
It was the operator-gated route-A successor named as pending in
`LANG-INDEXED-RECURSIVE-IH-DISCHARGE` (lines 23-32) and in
`V3-FO-CHECKER-SOUNDNESS` D3's hard-stop inventory. The operator authorized the
TCB change on 2026-08-22; `status` flips to `ready` and this node is released to
the kernel ring. `gate: operator` is retained as the record of the requirement;
it is satisfied. The soundness obligation is the one stateable sentence in D1
below (a widened arity must never turn a nonterminating recursion into an
accepted one; `AC-NEG` below is the mandatory guard).

# What is broken

The SCT gate (`sct.rs`, module doc `17 §4`) determines each mutually-recursive
group member's arity by **counting leading `Lam` binders in the elaborated
body**, then analyses size-change edges at that offset:

| step | site | what it does |
|---|---|---|
| arity | `count_params` (`sct.rs:623`) | counts leading `Term::Lam` binders in the body |
| enter body | `skip_lams(body, n)` (`sct.rs:634`) | skips `n` leading `Lam`s, returns the inner term |
| analyse | `sct_check` (`sct.rs:665`) | `group[caller_idx].1 = count_params(body)`; `collect_calls` on the skipped inner body, provenance seeded by `initial_prov(n)` (`sct.rs:650`) |

This is the **deep-lambda heuristic**: arity is read off the *body's* leading
lambdas, not off the member's declared type. It holds as long as the elaborated
body is in the canonical "leading parameter lambdas, then match" shape the
module doc assumes (`sct_check` doc: *"Bodies must include their leading
parameter lambdas"*).

**The c-elab result-refinement transport breaks that assumption.** Once
`LANG-INDEXED-RECURSIVE-IH-DISCHARGE`'s transport (landed, `93d82a398`) inserts a
genuine `J`/cast transport around a mutual-recursion sibling call's refined
result, the transport-carrying body's leading-`Lam` count **diverges from the
declared arity** (the transport wraps / eta-perturbs the body so the recursive
call no longer sits under exactly the declared number of leading lambdas). Then:

- `count_params` returns the wrong `n`,
- `skip_lams` enters the body at the wrong depth,
- the size-change matrices are dimensioned against the wrong parameter set and
  `initial_prov(n)` seeds the wrong diagonal,
- the idempotent self-loop check (`sct.rs`: *"idempotent self-loop has no
  strictly-decreasing parameter"*) fires on a clique that terminates,

and the gate reds `NotTerminating` on a **terminating** transport-carrying
clique. That is why the LANG-INDEXED transport passes `kernel_check` (the
narrowed AC-7 that is met) but not the full admission (`kernel_check` AND
SCT-pass) that `V3-FO-CHECKER-SOUNDNESS` D3 needs.

**This is distinct from `LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN`.** That draft is
a *documentation* node about SCT not tracing a decrease through a non-recursive
helper's return value; its `AC-3` explicitly bans widening SCT and routes any
widening to "a different node with a soundness review." **This node is that
different node with the soundness review** -- a distinct defect (arity source),
authorized as a TCB change.

# The fix -- route A, telescope-canonicalization (Architect + research)

Derive the arity from the member's **declared Pi telescope** (its type, looked
up from `env`), not from the count of leading `Lam` binders in the body; and
**analyse the same eta-long body the arity was taken from** -- eta-expand /
canonicalise the body to the declared arity before `collect_calls`, so admission
and analysis operate on one canonical form. Under that canonicalisation the
transport-carrying body's recursive calls sit at the correct declared parameter
offset and the size-change matrices are dimensioned correctly, so a terminating
transport-carrying clique is accepted -- without weakening what SCT rejects.

# Deliverables

**`D0` -- durable capture of the route-A specification, grounded, by the
Architect.** The Architect + research already specified route A in-thread; an
in-thread ruling is not a durable deliverable, so D0 transcribes it into this
node, grounded at the `sct.rs` coordinates above: (1) the exact source of the
declared arity (the member's Pi-telescope length from `env`, and how a hidden
`Pi` in the RETURN type is treated -- see `AC-NEG`); (2) the exact canonical
eta-long form the body is normalised to and the exact locus that replaces
`count_params`/`skip_lams` in `sct_check`; (3) the soundness argument that the
widened arity accepts strictly more terminating cliques and **zero** additional
nonterminating ones. **The four controls below are FIXED inputs, not open
choices.** D0 is design capture + locus pin, measured against the tree, not a
re-opening of the route.

**`D1` -- the arity repair in `sct.rs`.** Replace the deep-lambda arity with the
declared-telescope arity and make admission and analysis run on the same
eta-long body, at the locus D0 pins. Kernel-only; no elaborator change, no new
kernel file, no new `trusted_base()` entry (a termination-gate arity change adds
no trusted surface). The one stateable soundness sentence: **a widened arity may
turn a wrongly-rejected terminating recursion into an accepted one, and must
never turn a nonterminating recursion into an accepted one.**

# Acceptance criteria

**`AC-TELESCOPE` (control 1 -- arity from the declared telescope).** The arity
used by `sct_check` for each group member is derived from that member's declared
Pi telescope, not from `count_params` on the body. A control: a group member
whose elaborated body's leading-`Lam` count differs from its declared arity
(the transport-carrying shape) is analysed at the declared arity; the pre-fix
`count_params` reading is shown to differ and to be the wrong one.

**`AC-ADMIT-EQ-ANALYZE` (control 2 -- admit==analyze the same eta-long body).**
The body the arity is taken from and the body `collect_calls` analyses are the
one canonical eta-long form. A control that would pass if admission and analysis
used two different normalisations (the divergence this defect is) reds without
the fix and greens with it.

**`AC-NEG` (control 3 -- MANDATORY nonterminating hidden-return-Pi negative
control).** A genuinely nonterminating recursive group whose declared telescope
carries a `Pi` in its RETURN type -- the shape most likely to be mis-counted by
a naive telescope-arity reading -- is REJECTED by the widened gate with
`NotTerminating`. This control is mandatory and gates the close: the arity
widening must not open a soundness hole through a hidden return `Pi`. A mutation
that admits this group reds the control.

**`AC-CONSUMER` (the decisive buildability gate).** The exact LANG-INDEXED
transport consumer -- the held-D3 mutual-recursion clique carrying the c-elab
`J`/cast transport (`a84d71005` rebased onto the landed transport `93d82a398`)
-- passes FULL admission (`kernel_check` AND `sct_check`) through this fix, where
it currently reds `NotTerminating`. Synthetic green does not stand in for
real-family buildability.

**`AC-REVIEW` (control 4 -- adversary + conformance).** The Architect is the
required soundness reviewer (author is not reviewer). The Adversary hunts the
landed code for an admitted-nonterminating over-accept hole (the arity-widening
failure mode). A conformance seed exercises both directions: a terminating
transport-carrying clique ACCEPTED and the `AC-NEG` nonterminating
hidden-return-Pi group REJECTED, verdict-flip discriminating.

**`AC-NO-REGRESSION`.** Every SCT case already on `main` keeps its verdict --
the existing `sct_completeness_repro`, `sct_reconstruction_descent`, and the
`sct-reconstruction-descent` predicate unit tests stay green. Whole-suite green
in CI (`COORDINATION §12`); targeted `-p ken-kernel` locally, never
`--workspace`.

# Banned scope

- **Any change to SCT's acceptance principle** beyond the arity source. The
  criterion stays *"every idempotent self-loop has >=1 down-arrow on the
  diagonal"* (`sct.rs:7`); this node changes **where the arity comes from and on
  what body it is measured**, not what a self-loop must satisfy.
- **Weakening the diagonal / strict-decrease check, `compose_ord`, or the
  idempotent closure.** The over-accept direction is the soundness hole `AC-NEG`
  exists to catch.
- **Any elaborator change, new kernel file, new primitive/postulate, or
  `trusted_base()` entry.** Kernel reduction/admission only.
- **Re-opening the c-elab transport (`LANG-INDEXED`).** Its transport is landed
  and correct; this node makes the SCT gate accept it, it does not change it.
- **Folding in `LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN`.** That is the separate
  helper-return documentation node; its `AC-3` bans the widening this node does,
  and the two defects (helper-return opacity vs. arity source) are independent.

# Sequencing

**`ready`, authorized, released to the kernel ring.** `depends_on` is empty --
the defect exists on `origin/main` today, because the transport that triggers it
(`93d82a398`) is already landed. `blocks: [V3-FO-CHECKER-SOUNDNESS]`: this node
inherits the SCT-pass gate role from `LANG-INDEXED-RECURSIVE-IH-DISCHARGE`, which
is closed as an accepted partial (its transport deliverable done) with
`V3-FO-CHECKER-SOUNDNESS`'s `depends_on` re-pointed here. V3-FO D3 resumes on
this landing against a transport-aware, SCT-admitting IH.

**Owner: kernel ring.** Author = `kernel-implementer` (T1 -- soundness-bearing
termination-gate work). Independent soundness review = Architect. Adversary
hunt + conformance seed per `AC-REVIEW`. First move is **D0** -- the Architect's
durable capture of route A, grounded, with the locus pinned by measurement.

**Bookkeeping.** This is the SCT-pass half of the full-admission gate that the
mutual-recursion result-refinement chain reduces to: `LANG-INDEXED` delivered
the c-elab transport (`kernel_check` half); this node delivers the SCT-pass
half. Together they close the full admission `V3-FO-CHECKER-SOUNDNESS` D3 needs.
