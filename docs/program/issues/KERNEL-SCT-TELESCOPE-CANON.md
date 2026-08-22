---
id: KERNEL-SCT-TELESCOPE-CANON
title: "The SCT gate derives each group member's arity by counting leading Lam binders in the elaborated body (count_params, sct.rs:623), so once the c-elab result-refinement transport wraps a mutual-recursion body the leading-Lam count diverges from the declared arity, skip_lams skips the wrong number, the size-change matrices are mis-dimensioned, and a terminating transport-carrying clique wrongly reds NotTerminating -- repair the arity to the DECLARED Pi telescope (route A, telescope-canonicalization) with admit==analyze on the same eta-long body, the SCT-pass enabler LANG-INDEXED full admission and V3-FO-CHECKER-SOUNDNESS D3 are blocked on"
status: ready
owner: kernel
size: M
gate: operator
depends_on: []
blocks: [V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]
github: null
origin: "Steward, 2026-08-22, on the operator's authorization (\"tcb change authorized. proceed.\") of the route-A kernel SCT successor that LANG-INDEXED-RECURSIVE-IH-DISCHARGE and V3-FO-CHECKER-SOUNDNESS D3 name as their SCT-pass gate. The c-elab result-refinement transport landed (LANG-INDEXED accepted partial, squash 93d82a398): the narrowed AC-7 -- held-D3 bodies ELABORATE + pass kernel_check -- is met, but FULL admission is kernel_check AND SCT-pass, and the transport-carrying mutual-recursion clique reds the SCT gate. Route A (SCT arity from the declared Pi telescope, not the deep-lambda body heuristic) was specified in-thread by the Architect + research and is captured durably at D0 (an in-thread ruling is not a durable deliverable). TCB change: it modifies the trusted size-change termination gate (crates/ken-kernel/src/sct.rs). gate: operator, and the operator's authorization above satisfies it. Steward-filed per COORDINATION section 2. Estimated capability tier: T1 (soundness-bearing termination-gate change; the arity widening must not admit a nonterminating recursion -- the negative control below is mandatory)."
---

# FINAL RULING -- 2026-08-22 (Architect, evt_1gtmndpzh3xda): route A is correct, this node CLOSES on a SYNTHETIC consumer, the real FoKripke clique moves to the successor

The D1 hard-stop (HS=1) ran the one discriminating measurement on the exact
`AC-CONSUMER` clique (`a84d71005`), and the Architect converted his conditional
into a final ruling. Three firm conclusions govern this node now:

1. **Route A is correct, necessary, and COMPLETE FOR THE ARITY DEFECT.** With
   `WIP 27a84fcc5a94` the matrices are correctly dimensioned and the descent is
   read as real bare-`Var` matched-field structural descent (`0.p6 ↓→ 8.p0` =
   `@2 child`). That WIP is the deliverable value; the strict-decrease gate was
   never weakened.
2. **The Cast/J transport-opacity arm is REFUTED.** The measurement found ZERO
   of the 50 descending call arguments are `Term::Cast` or `Term::J` (the `J`s
   surround call *results*, not recursion arguments). So the conditionally-scoped
   successor `KERNEL-SCT-TRANSPORT-TRANSPARENT` does **not** fire and was **not**
   cut.
3. **The real `AC-CONSUMER` (FoKripke) has a SECOND, independent blocker that is
   neither arity nor Cast/J -- and the Architect's CORRECTION (evt_134z6mr80ymqp,
   amending FIRM 3's cause + destination; FIRM 1 and FIRM 2 stand) names it:**
   the SURVIVING descent thread is bare-`Var` matched-field structural descent
   (`0.p6 ↓→ 8.p0` = `@2 child`), and it fails by ROTATION -- it arrives at
   member-0 `p0`/`p1` while the outgoing `0→8` edge decreases only from `p6`, so
   no single thread survives a lap (one-lap `0→0` product has `Down` only
   off-diagonal at `[6][0]`/`[6][1]`, squares to all-`Unknown`). The
   helper-return edges (`g625`/`g631`) are OFF that binding thread; tracing them
   would not close it. The three shapes seen on this consumer (Cast/J opacity
   refuted, helper-return opacity, rotation) are ONE predicate, now MEASURED
   authoritatively: **the FoKripke clique's real termination is not a single
   structural size-change thread on its declared parameters under the current
   `size_rel` abstraction.**

   The kernel-lexicographic/permutation-aware arm is RULED OUT: SCT's
   idempotent-closure strict-diagonal criterion is already sound-and-complete for
   the size-change abstraction (Lee-Jones-Ben-Amram, POPL 2001), including
   permuting descents -- a genuine rotating descent expressible in the
   size-change graphs WOULD produce a strict-diagonal idempotent matrix, and the
   closure has none. The gap is the GRAPHS (the `size_rel` abstraction does not
   capture the clique's real decreasing measure), not the closure criterion.

**Consequence for this node.** Its founding premise -- that FoKripke's SCT
failure was the arity defect ALONE -- is refuted. So this node's decisive
buildability gate becomes a **SYNTHETIC arity-isolation consumer** (three
criteria in `AC-CONSUMER` below); the **real FoKripke clique is preserved -- not
lost -- as the close gate of `V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY`**, the
language/spec enclave node that owns the rotation fork (upstream re-elaboration
preferred; a narrow `size_rel` completeness fix or a richer measure are
operator-gated conditional arms). The real `AC-CONSUMER` and
`V3-FO-CHECKER-SOUNDNESS`'s `depends_on` re-point there -- NOT onto the
helper-return node and NOT onto a kernel node. `blocks` re-points to that
successor (this node's arity fix is still needed before the real clique can pass,
so it stays a predecessor of it). This node closes on its arity ACs + the
synthetic consumer; `kernel-implementer` is building the synthetic consumer now
(evt_x4nhgwcnr3yj).

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

**`D0` -- RESOLVED. Route-A durable capture (Architect, `evt_67fm72hkpa3ej`,
grounded and locus-pinned).** The Architect + research specified route A
in-thread; an in-thread ruling is not a durable deliverable, so it is
transcribed here verbatim in substance. **Self-contained -- D1 needs no further
lookups.** The four controls below are FIXED inputs, not re-opened.

Grounding coordinates confirmed by direct read at `586530f89`:
`sct.rs` `count_params:623`, `skip_lams:634`, `initial_prov:650`,
`sct_check:665`; `env.const_type:460`; `inductive.peel_pi:97`;
`subst.weaken:129`; `conv.convert` Pi-eta arm `:349-361`; `conv.normalize:225`.
Both `sct_check` call sites (`check.rs:1101` single-member, `:1173` group) pass a
populated `env` in which every group member is pre-admitted opaque, so
`const_type(id)` resolves at check time and delta never unfolds a group member.

*(1) Declared-arity source, and the hidden return-Pi.* The arity for each group
member is derived from its DECLARED TYPE, not from the body's leading-`Lam`
count. Source: `env.const_type(id)` returns `(level_params, ty)`; the arity `n`
is the length of `ty`'s Pi telescope. Concretely: iterate -- `whnf` the type; if
it is `Term::Pi(A, B)`, that is one parameter, recurse into `B`; stop at the
first non-Pi head. This is exactly the peel `inductive::peel_pi` performs (use it
directly when the elaborated type is Pi-headed at each level; use the whnf-guided
peel to be robust to a reducible/delta codomain head -- it mirrors `convert`'s
own Pi-eta, which whnf's the type before matching Pi). **The telescope is
MAXIMAL: a Pi in the RETURN type is counted as a parameter** (it is a Pi in the
type). That is deliberate and is the case `AC-NEG` guards -- see (3): a return-Pi
eta parameter is provably incapable of manufacturing a strict descent, so
counting it is sound; under-counting it is what a naive body-reading would do.

*(2) Canonical eta-long form + exact locus.* The body analysed and the arity are
ONE canonical form: the member's elaborated body `b`, eta-expanded
(type-directed, driven by the declared telescope) to exactly `n` leading
parameters, with those `n` parameters beta-realigned so recursive-call arguments
are expressed against them. Reference construction, mirroring `convert`'s Pi-eta
de Bruijn convention (`conv.rs:349-361`) exactly:

```
let doms = first n domains of the peeled declared telescope;   // A_0 outermost .. A_{n-1} innermost
let mut app = weaken(&b, n);                                   // b↑n, lift free vars past the n new binders
for k in (0..n).rev() { app = Term::app(app, Term::var(k)); }  // apply Var(n-1), Var(n-2), ..., Var(0)
// eta-long body = doms.rev().fold(app, |acc,a| Term::lam(a, acc));  == λA_0..λA_{n-1}. (b↑n @ (n-1) @ .. @ 0)
```

The inner term `collect_calls` analyses is `app` after firing exactly the `n`
eta beta-redexes (NOT a full `normalize` -- see `AC-NO-REGRESSION`). The de
Bruijn convention is unchanged from today: `Var(0)` = innermost = param `(n-1)`,
so `initial_prov(n)`/`initial_recon(n)` are untouched.

EXACT LOCUS in `sct_check` (`sct.rs:665`), the group-construction and edge loop
-- two substitutions and nothing else:

- `group[caller_idx].1`: replace `count_params(body)` with
  `declared_arity(env, id)` (the telescope peel of (1)).
- `inner`: replace `skip_lams(body, n)` with `canonical_inner(env, id, body, n)`
  (the construction of (2), firing the `n` eta-redexes).
- `initial_prov(n)`, `initial_recon(n)`, `collect_calls(inner, caller_idx, n,
  &group, ...)` UNCHANGED -- they already key off `n`.

Helpers are all pre-existing (`const_type`, `peel_pi`, `weaken`,
`Term::var`/`app`/`lam`); no new kernel machinery, no new file. `count_params` is
retained (unused by `sct_check`) so `AC-TELESCOPE`'s control can exhibit the
pre-fix reading vs the declared arity.

The reduction discipline (fire the `n` eta-redexes; do NOT deep-normalize the
body) is bounded by three properties -- the D0 envelope; the exact loop is D1's:
(i) def-equal to `b`; (ii) exposes exactly `n` parameters as the outer binding
structure; (iii) for a canonical body (already `n` leading lambdas) it reduces to
today's `skip_lams` inner verbatim (`AC-NO-REGRESSION`). A full `normalize` would
over-reduce the body's interior (e.g. fire a match at the head) versus today's
un-normalized `skip_lams` analysis and risk `AC-NO-REGRESSION` -- so canonicalise
the eta head only, not the interior.

*(3) Soundness -- strictly more terminating cliques, ZERO additional
nonterminating.*

- **Admission theorem intact** (why route A is sound where the deep-lambda
  heuristic Agda retired is not): the canonical form is beta-eta-delta-equal to
  `b` under the kernel's OWN conversion (eta is literally `convert`'s Pi-eta
  step; beta fires only the eta-redexes; delta touches no group member -- they
  are opaque). Admit and analyse run on this one def-equal form
  (`AC-ADMIT-EQ-ANALYZE`) -- there is no analysis surrogate distinct from the
  admitted body, so an untrusted producer cannot certify one body while a
  different one is analysed.
- **Zero additional nonterminating accepted:** eta to the declared arity adds
  only type-mandated Pi parameters. At every recursive call the eta rule applies
  each added parameter to the very variable that binds it, so its size relation
  to the corresponding callee parameter is `DownEq` (equal), never `Down`
  (strict). A column that is `DownEq`/`Unknown` everywhere contributes no strict
  down-arrow to any idempotent self-loop diagonal, and the added columns do not
  alter the real parameters' size relations or their composition closure. Hence
  the widened arity cannot supply the `>=1` strict-down-arrow acceptance requires
  where the real parameters do not already supply it: a clique the pre-fix gate
  rejects for genuine nontermination stays rejected. `AC-NEG` pins exactly this
  with a return-Pi-carrying nonterminating group.
- **Strictly more terminating accepted:** the only behavioural change is (a) the
  transport case -- a wrong arity/offset (leading-`Lam` count diverged from the
  declared arity) is corrected, so the real descent that makes the clique
  terminate is measured against the right parameter set and the false
  `NotTerminating` clears; and (b) never removing a strict-down-arrow that
  existed, since the real parameters' relations are unchanged. The accept set
  grows by exactly the wrongly-rejected terminating cliques and nothing else.

*Control -> failure-mode map (for the Adversary hunt and the conformance seed).*

- `AC-TELESCOPE` defends "arity from type, not body" -- a revert-to-`count_params`
  mutation reds.
- `AC-ADMIT-EQ-ANALYZE` defends "one canonicalisation for dimension `n` and
  analysed body" -- a two-normalisation split (this defect's own shape) reds.
- `AC-NEG` (MANDATORY, gates close) defends the over-accept direction -- a
  nonterminating return-Pi group admitted by a mis-count reds; this is the
  arity-widening soundness hole.
- `AC-CONSUMER` decisive buildability -- REVISED by the final ruling (this D0
  capture predates the measurement): the real LANG-INDEXED clique proved to
  carry a SECOND blocker (helper-return invisibility, not arity), so it moved to
  the successor and this node closes on the synthetic arity-isolation consumer.
  See the FINAL RULING banner and the revised `AC-CONSUMER` below.
- `AC-NO-REGRESSION` `canonical_inner` == today's inner for canonical bodies, so
  `sct_completeness_repro`, `sct_reconstruction_descent`,
  `sct-reconstruction-descent` units keep their verdicts.

D0 is design capture + locus pin, measured against the tree, not a re-opening of
the route. D0 complete; the ring's first build move is D1.

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

**`AC-CONSUMER` (the decisive buildability gate -- SYNTHETIC arity-isolation
consumer, per the final ruling).** The measurement refuted the premise that the
real FoKripke clique is blocked by arity alone (see the FINAL RULING banner), so
the real clique moves to the successor and this node closes on a synthetic
consumer that isolates the arity dimension. The Architect's three criteria
(evt_4qgm2hatmwcgy, evt_1gtmndpzh3xda) all bind, and criterion 2 is
load-bearing:

1. **SHAPE.** A mutual-recursion clique whose ELABORATED leading-`Lam` count
   diverges from its declared-telescope arity -- the exact defect this node
   repairs. A canonical body that already has `n` leading lambdas exercises
   nothing.
2. **ISOLATION (load-bearing).** Its termination is size-change-visible at the
   CORRECT declared arity with NO transport/cast/`J` and NO non-recursive helper
   on its descending-argument path -- one persistent bare-`Var` descent thread
   through a SINGLE parameter across laps (no rotation). So the arity fix alone
   is necessary AND sufficient to flip it. A descent running through a coercion
   or a helper return would carry the SECOND defect, could never green in this
   node, and would wrongly block the arity landing -- the exact conflation the
   real `AC-CONSUMER` suffered.
3. **DISCRIMINATION.** RED with the arity reverted to `count_params` (arity read
   off the body), GREEN with `27a84fcc5a94`. A consumer that greens without the
   arity fix is vacuous.

This is not "synthetic green stands in for real coverage": the two-defect gate
is correctly split into its independently-landable halves, and the real FoKripke
clique remains the close gate of `V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY` (full
admission = `kernel_check` AND `sct_check`), not lost.

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
(`93d82a398`) is already landed.
`blocks: [V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]`: the D1
measurement (see the FINAL RULING banner) refuted the premise that this arity
fix is the whole SCT-pass gate for FoKripke. The real clique carries a second,
independent residual (rotation -- a `size_rel`-abstraction gap, not arity), owned
upstream by the language/spec enclave node `V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY`,
which is where `V3-FO-CHECKER-SOUNDNESS`'s `depends_on` now points. This node's
arity fix is still a predecessor of that node (the real clique cannot pass until
the arity is correctly dimensioned), so it blocks it.

**Owner: kernel ring.** Author = `kernel-implementer` (T1 -- soundness-bearing
termination-gate work). Independent soundness review = Architect. Adversary
hunt + conformance seed per `AC-REVIEW`. First move is **D0** -- the Architect's
durable capture of route A, grounded, with the locus pinned by measurement.

**Bookkeeping.** This node delivers the ARITY correction to the SCT gate --
one of the fixes the mutual-recursion result-refinement chain needs, and the
only one it was authorized for under Q3. `LANG-INDEXED` delivered the c-elab
transport (`kernel_check` half of full admission); this node corrects the arity
so the size-change matrices are correctly dimensioned. The D1 measurement then
showed FoKripke's full admission needs a THIRD thing beyond both -- an upstream
resolution of the rotation / `size_rel`-abstraction gap -- carried by
`V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY`. This node's own close is its arity ACs +
the synthetic arity-isolation consumer; it does not, by itself, close the full
admission `V3-FO-CHECKER-SOUNDNESS` D3 needs.
