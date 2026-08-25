---
id: LANG-MOD-PAIR-STRICT-BOUNDARY
title: "Durable Spec/Conformance boundary artifact — compiler possession of a non-floor convenience (native Pair) is NOT strict provider availability; pin the strict bare-Pair rejection, same-shape distinct-identity, exact-nine stability, and the deferred/later-closure conformance shape."
status: merged
owner: spec
size: S
gate: none
depends_on: []
blocks: [LANG-MOD-CANONICAL-PAIR-PACKAGE]
github: null
origin: "Spec enclave boundary ruling on Component B hard stop #1 (spec-author evt_6nk4xxkppz3k5, spec-leader evt_w7v4dvvzjr8k, thr_7mdnraw5rrmc2). The ruling directs the Steward to release the durable boundary as its own node. Deductive from the settled floor/package/Strict contracts — no new Decision. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # MERGED 2026-08-25 at squash `eb65d328b` — durable Pair strict boundary landed
>
> PR #2928 merged onto main (origin/main `d616db322` after the subsequent RT
> doc-squash). Candidate `7f8da79cb` (tree `e983a45f`, base `dcfa19210`, 33 paths
> +1231/-250). Exact-SHA APPROVES on `7f8da79cb`: Architect (evt_4mwx48teb96ta),
> CV (evt_5wp11ysmn5t71; preserved-object tests 16/16 + 10/10 + 18/18), spec-leader
> enclave sign-off (evt_7rjrj292azabf); Decision `dec_20b0v2bg9gf7s` resolved.
> BLOB-AUDIT clean: all 33 reviewed paths byte-identical on the landed tree, and
> the Pair squash touched exactly those 33 paths (the 2 `crates/` paths are both
> `tests/`, no production src). Unblocks [[LANG-MOD-CANONICAL-PAIR-PACKAGE]].
>
> # FRAMED 2026-08-25 — enclave boundary ruling; startable now (spec-owned)
>
> The Spec enclave ruled Component B's native-`Pair` contradiction as arm B
> (narrow-and-defer). This node is the durable spec + conformance amendment that
> ruling directs the Steward to release as its own node. It is deductive from the
> settled floor/package/Strict contracts, carries NO Decision, and is
> Pair-INDEPENDENT of Component B's build — so it is startable now, in parallel
> with the recut [[LANG-MOD-CATALOG-COMPLETENESS]]. spec-leader owns recording the
> durable amendment (evt_w7v4dvvzjr8k). It BLOCKS the Pair-package realization
> [[LANG-MOD-CANONICAL-PAIR-PACKAGE]] because it pins the conformance boundary the
> realization must satisfy.

# Objective

Make explicit, in normative spec text plus conformance, the boundary the
Component B hard stop surfaced: the compiler's possession of a non-floor
convenience (native `Pair`, `mk_pair`, `pair_fst`, `pair_snd`) does NOT make it a
strict provider. Under Strict a name resolves only from a defining public
interface; the exact-nine floor is closed; a same-shaped local/package
redeclaration is a distinct identity, not the native one. This codifies existing
behavior — it does not design the Pair package (that is
[[LANG-MOD-CANONICAL-PAIR-PACKAGE]]) and does not change the floor.

# Fixed inputs (the ruling; ground on current main and re-check)

- Spec `30-taxonomy §4` and `33 §3.3` close the strict Ken-defined floor at
  exactly `{Auth, Bool, Char, List, Nat, Option, ResourceKind, Result,
  Utf8Error}`. `Pair` is expressly outside both membership arms.
- `30-taxonomy §5` and `50-stdlib/README` classify `Pair` as an ordinary package
  convenience; such a name enters Strict only by explicit import from a defining
  public interface.
- `39 §2.0` preserves the provider declaration's identity and forbids
  arbitrary-global fallback. The compiler-installed native `Pair` convenience has
  no public provider interface, so its existence does not make it a strict
  dependency.
- A catalog `data Pair` is a NEW declaration and therefore a NEW identity
  (measured: native prelude `Pair = g232` `Decl::Transparent` vs a catalog
  redeclaration `g578` `Decl::Inductive`). It cannot masquerade as the native
  identity or make byte-unchanged native-identity consumers close.

# Deliverables

- D1 — normative clarification. Clarify `30-taxonomy §5`, `33 §3.3`, `39 §2.0`,
  and the `50-stdlib` index that compiler possession of a non-floor convenience is
  NOT provider availability under Strict. No grammar change; no new mechanism; no
  floor change; a proposal to put `Pair` in the floor returns to the operator and
  is explicitly OUT OF SCOPE here.
- D2 — paired conformance controls (the pins below).
- D3 — EXPANDED normative surface (Architect evt_4tq5ad3sstkky + spec-author
  evt_3tgjjqmham8s0). The four taxonomy/module sections are not the whole surface;
  the same spec artifact must reconcile every inherited native/prelude `Pair`
  assumption on current `origin/main`, and normatively specify the future
  package-defined contract (which [[LANG-MOD-CANONICAL-PAIR-PACKAGE]] realizes):
  - Reconcile the native/prelude assumptions: `50-stdlib/README` (calls `Pair` a
    package but points to `30-surface/34`, which supplies no canonical Pair
    contract today), `57 §4.4` (the "landed prelude Σ-pair"), `60` (cites
    `prelude.rs`), the computation contracts in `54` and `58`, `14 §8` (the
    "already-admitted positive Pair"), and every conformance vector that uses bare
    `Pair` — each migrates to the future interface or remains an intentional
    strict-bare rejection.
  - Specify the future contract WITHOUT new syntax beyond the landed surface:
    `Pair : Type → Type → Type` transparently definitionally equal to the
    non-dependent kernel Σ; `mk_pair`/`pair_fst`/`pair_snd` are ordinary checked
    transparent declarations over Σ introduction/projection; fst/snd β and
    reconstruction η remain DEFINITIONAL (current map/view proofs rely on them —
    behavior, not a compatibility alias); each of the four names has exactly one
    defining module and one identity, import/re-export preserving them and
    allocating none; strict UNIMPORTED use still rejects, a same-shaped local
    declaration stays distinct, and removing the compiler globals leaves NO
    Legacy/ambient/alias/mixed-resolution route.
  - `14 §8` nested-inductive positivity must become REPRESENTATION-DERIVED behavior
    of the transparent Σ alias — reached by ordinary reduction — NOT a spelling
    allow-list or ambient identity: positive uses accept structurally, while
    `Pair (Bad → Empty) Unit` still rejects through the ordinary negative
    occurrence WITHOUT recognizing the spelling `Pair`.
  - Prior behavioral rulings survive the recut and are recorded here as still owed
    downstream: the sole `instance Ord Nat` dictionary remains class-owned by
    `Core.Classes.LawfulClasses` (Order may later import/re-export that same
    identity, NOT absorb it); and the settled foreign-attached-proof conversions
    (Component B AC-B8, [[LANG-MOD-ATTACHED-PROOF-OWNERSHIP]]) are neither reversed
    nor discharged by deferring their whole unit.

# Acceptance criteria

- AC-1 — bare `Pair` / `mk_pair` / `pair_fst` / `pair_snd` in Strict rejects as
  unresolved rather than falling back to the compiler global.
- AC-2 — a same-shaped local/package `Pair` receives a DISTINCT identity and
  cannot satisfy an exact-native-identity dependency (spelling / shape is not
  admission).
- AC-3 — the exact-nine floor remains unchanged (a floor-membership assertion for
  `{Auth,Bool,Char,List,Nat,Option,ResourceKind,Result,Utf8Error}`; `Pair`
  absent).
- AC-4 — the deferred cluster's Pair-dependent vectors are marked RED-UNTIL the
  separate canonical-Pair prerequisite lands (they are `deferred`, not `green`;
  aligns with the recut [[LANG-MOD-CATALOG-COMPLETENESS]] AC-B3 disposition).
- AC-5 — the later-closure assertion: AFTER
  [[LANG-MOD-CANONICAL-PAIR-PACKAGE]], explicit import and re-export resolve every
  Pair-dependent cluster reference to the ONE provider identity, with no import
  allocation, competing identity, or trust delta. (This pin is authored here and
  goes green only when the prerequisite lands; it is the seam that flips the
  deferred vectors.)
- AC-6 — zero `trusted_base()` change; flat-Σ invariant preserved (codifies
  existing behavior).
- AC-7 (expanded surface reconciled). No stale native/prelude `Pair` assumption
  remains in `50-stdlib/README`, `57 §4.4`, `60`, `54`, `58`, `14 §8`, or the
  conformance corpus — each is migrated to the future interface or is an
  intentional strict-bare rejection; the `14 §8` nested-inductive positivity pin is
  representation-derived (positive accept, `Pair (Bad → Empty) Unit` reject, no
  spelling recognition).
- AC-8 (contract specified) — the normative text pins `Pair` as transparently
  definitionally equal to non-dependent kernel Σ, `mk_pair`/`pair_fst`/`pair_snd`
  as checked-transparent over Σ intro/projection, and fst/snd β + reconstruction η
  as definitional.
- AC-9 (prior obligations recorded) — the `Ord Nat` class-ownership rule and the
  owed AC-B8 attached-proof conversions are stated as surviving the deferral.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted only, never
  `--workspace`.

# Reviewers

spec-author (normative fidelity: the text codifies the settled
floor/package/Strict boundary exactly, and the same-shape-distinct-identity rule)
+ spec-leader (enclave sign-off; owns recording the durable amendment) +
conformance-validator (the reject / distinct-identity / floor-stable / deferred /
later-closure pins are discriminating). Architect advisory on
mechanism-neutrality (this codifies existing behavior and picks no Pair-package
mechanism). No Decision fork.

# Sequencing

Campaign work under [[LANG-MODULE-IMPORT-SYSTEM]]. Startable now, Pair-independent
of Component B's build; runs in parallel with the recut
[[LANG-MOD-CATALOG-COMPLETENESS]]. BLOCKS [[LANG-MOD-CANONICAL-PAIR-PACKAGE]]
(pins the conformance boundary the realization satisfies). Whole-catalog strict
closure of the deferred cluster returns only after the prerequisite lands.
