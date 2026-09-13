---
id: SPEC-MEMBERSHIP-CLASS-CONTRACT
title: "the L2 membership-operator typeclass contract: specify a standard Membership class and the standard binding of the (already glyph-admitted) `∈` to it -- a unary `class Membership (container) { Query : Type; member : Query -> container -> Bool }` with `member_holds := IsTrue (member ..)` as the Prop view, dispatched at use-site through the SAME class resolver A1 factors (no second operator dispatcher), over nominal witness-bound carrier VIEWS (a list view, one ordered-key view serving Map and Set, a distinct relation-edge view) so distinct roles do not collide under today's outer-head-name registry key and the comparator/validity witness lives in the view value not an implicit lookup; a shared `SameMembers` observational law plus a per-provider adapter-fidelity obligation, with container-specific laws kept container-specific; NO multi-parameter class machinery, NO `Membership Tree` instance, NO Prop-to-Bool elimination, NO new trust-root/TCB"
status: draft
owner: spec
size: M
gate: none
depends_on: [SPEC-RESERVED-INFIX-NAMES]
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
tier: T1
origin: "Steward cut 2026-09-13 on the operator directive (Pat, this session): 'Frame the L2 binop typeclass work to support membership ... a spec-enclave elaboration task after the frame.' This is the B track of the reserved-infix-glyph objective (Pat, 2026-09-12: 'membership seems categorically different since it involves some typeclass machinery not required by the others ... dig into typeclass dispatch for member'), which Pat set to run in PARALLEL with the A-track glyph fix. Grounded in the membership class/carrier Research advisory (research evt_10xwbzfz9vz77 + evt_62nsr14rm3zd0, thr_60s5rhqdh4ht8), itself recommended by the Architect (evt_5dar8wmwpxj8x). The Architect is the design authority for the class/carrier ruling and a required reviewer; that ruling is the first design step of the enclave elaboration. Advisory anchors were measured at main 5fb21c419; re-measure at the cut (main is now 5d1347aaf). IN-LANE, spec-enclave-owned: bounded normative surface, no new language/kernel mechanism (the dispatch reuses the existing dictionary-passing resolver that A1 factors), no new trust-root, no TCB -- same posture as SPEC-RESERVED-INFIX-NAMES / SPEC-STANDARD-INFIX-BINDING; no operator sign-off beyond the directive that authorized the work."
---

> # B-TRACK SPEC CONTRACT for the membership operator. Spec-enclave elaboration
> # (operator directive, Pat, this session). Design authority: the Architect's
> # class/carrier ruling on the Research advisory (thr_60s5rhqdh4ht8), which is
> # the first design step here. `depends_on` SPEC-RESERVED-INFIX-NAMES (the `∈`
> # name it binds); the class/carrier/law design may be authored in parallel
> # with the A-track and only the `∈`-surface-binding section finalizes once the
> # name is admitted. Re-measure every cited anchor at the cut (main 5d1347aaf).

## What this is

`∈` is admitted as an ordinary glyph-only symbolic name by
[[SPEC-RESERVED-INFIX-NAMES]] (A0's spec prereq), but with NO standard meaning
(that node and [[SPEC-STANDARD-INFIX-BINDING]] both explicitly defer membership
to this B track). This node specifies the STANDARD meaning of `∈`: a
`Membership` typeclass and the use-site completion that dispatches `q ∈ c` to
its `member` method over the existing collection views. It is the membership
counterpart of `SPEC-STANDARD-INFIX-BINDING` (which binds `∧ ∨ ≤ ≥ ≠`), split
out because membership needs class machinery those five do not.

Membership is categorically a typeclass feature (operator ruling): the three
existing collection views (list, key/Map+Set, relation edge) are distinct named
Bool workers with distinct witness requirements, and there is no unifying class
today. This contract is what makes `∈` a coherent standard operator without
inventing a witness or laundering a comparator.

## What the contract must settle

The Research advisory recommends the smallest sound shape below; the Architect
rules whether to adopt it. State the design QUESTIONS the contract answers, then
the normative text.

1. **The class shape: unary, with an associated query type.** A one-parameter
   structure class over the CONTAINER, with the query type an associated field:

   ```text
   class Membership (container : Type) {
     Query : Type
     member : Query -> container -> Bool
   }
   ```

   `Query` precedes `member` in the dependent telescope (a structure class whose
   later fields may refer to earlier ones -- already expressible by Ken's class
   representation). Results are `Bool`. This is NOT a multi-parameter class:
   genuine `Membership query container` dispatch is not present machinery (the
   registry keys on ONE outermost head) and buys no semantic win here (Lean's
   `outParam` model is behaviorally this unary associated-Query shape). The
   contract must rule unary-associated-Query in, or justify the far larger
   multi-parameter mechanism against the advisory.

2. **The proposition view is a definition over Bool, never an elimination from
   Prop.** `member_holds d q c := IsTrue (d.member q c) : Prop`. Bool is
   primary; there is no law recovering evidence from `Prop`, and `rel_member`'s
   underlying CHECKED Bool computation is reused (not derived by eliminating
   `rel_member : Prop`).

3. **Carrier semantics live in nominal, witness-bound VIEWS, not in raw heads.**
   Because the instance registry keys on the outermost head name, distinct roles
   over the same raw head collide. So the standard providers are distinct
   nominal view heads, each owning its dictionary/comparator and validity
   witness in the VALUE (illustrative shapes; the ruled API names are the
   Architect's / spec-author's):
   - a **list view** owning `(d : Ord a, xs : List a)`, `Query = a`, calling the
     existing `elem a d` (no sortedness proof; the proof-carrying `search` stays
     a separate operation);
   - one **ordered-key view** owning `(d : Ord k, tree : Tree k v,
     ordered : Ordered d.leq tree)`, `Query = k`, serving Map key membership --
     and the SAME view at `v = Unit` serving Set (this mints no `Set` carrier or
     head);
   - a distinct **relation-edge view** owning `d`, the adjacency tree, the outer
     `Ordered`, and evidence every stored successor tree is `Ordered` under that
     same `d`, `Query = Pair k k`, calling the Bool underlying `rel_member`.

   The contract must FORBID installing `Membership Tree` (three canonical Tree
   instances collide), and must forbid an implicit resolver choosing a fresh
   `Ord` at each `member` call (a raw `Tree` binds no comparator in its type, so
   a use-site canonical `Ord` can silently disagree with the tree's valid order;
   the witness must be the one the view value was validated with).

4. **The `∈` standard binding + use-site completion.** `q ∈ c` completes at the
   use site by inferring the RHS carrier FIRST, resolving its one canonical
   `Membership carrier` dictionary through the class resolver, projecting
   `d.Query`, then checking the LHS against it -- it must NOT use the LHS to
   guess among carrier meanings. This is the same use-site dictionary-completion
   mechanism that `SPEC-STANDARD-INFIX-BINDING` specifies for `≤`/`≥` and that
   [[LANG-STANDARD-INFIX-CALL-COMPLETION]] (A1) factors: membership REUSES that
   scoped/coherent resolver, it does not add a second operator dispatcher. The
   binding is to the defining class-method GlobalId + checked telescope, not to
   the `∈` glyph text; a missing / ambiguous / unadmitted-provider case is a
   normal instance error. Standard fixity candidate: `∈ infix 4` (comparison
   band, aligning with `≤ ≥ ≠`); the spec rules it.

5. **The shared law model, and where it stops.** The nonvacuous SHARED law is an
   observational quotient, kept OUTSIDE the minimal dictionary:

   ```text
   SameMembers d x y := (q : d.Query) -> Equal Bool (d.member q x) (d.member q y)
   ```

   Its equivalence laws follow from Bool equality and are reusable across
   providers. Each standard provider additionally carries a nonvacuous
   adapter-fidelity obligation: its class `member` equals the existing explicit
   worker under the exact stored witness (a wrong comparator, swapped relation
   endpoint, value-pair Map query, or worker duplication must flip it). The
   contract must record that there is NO honest nonvacuous algebraic law inside
   the minimal two-field class (empty/insert/lookup laws cannot even be stated
   without more structure; an added `Belongs` field proven equal to
   `IsTrue(member ..)` is vacuous). Container-specific laws stay
   container-specific (List Cons/Nil + permutation; Map member-iff-lookup-Some
   under `Ordered`/`Distinct`; Set membership-extensional Boolean algebra, with
   raw `Tree` equality still refused for union laws per 58 §5; relation
   edge-extensional + converse/composition/reachability under the shared
   ordering premises).

## Deliverables

1. The bounded normative amendment placing the `Membership` class, the standard
   `∈` binding + fixity + use-site completion policy, the standard provider
   views, and the shared/`per-provider law model into the appropriate
   `spec/30-surface/` and `spec/50-stdlib/` sections, without a broad rewrite.
2. A reaching conformance seed: a discriminating implementation resolves `∈`
   over the list, Map, Set, and relation-edge views to the correct Bool, keeps
   the comparator/validity witness bound (a foreign `Ord` or a non-`Ordered`
   tree is refused, not silently accepted), and refuses a missing/ambiguous
   provider; a non-conforming one (LHS-first carrier inference, a `Membership
   Tree` instance, a Prop-to-Bool elimination, or an implicit fresh-`Ord`
   choice) reds. Seed expected observations stated independently of the
   implementation under test.

## Acceptance criteria

- The contract specifies exactly the unary associated-Query `Membership` class
  (or an Architect-ruled alternative with its full added machinery justified),
  with `member : Query -> container -> Bool` and `member_holds := IsTrue (member
  ..)`; no Prop-to-Bool elimination.
- The three standard providers are distinct nominal witness-bound views (list;
  ordered-key serving Map and Set; relation-edge), each carrying its own
  dictionary/comparator + validity witness; no `Membership Tree`, no `Set` head,
  no implicit use-site `Ord` re-choice.
- `∈`'s standard binding completes carrier-first via the SAME resolver A1
  factors (no second dispatcher), binds to the class-method GlobalId + telescope
  not the glyph, and treats missing/ambiguous/unadmitted providers as normal
  instance errors; standard fixity is stated.
- The shared `SameMembers` law and the per-provider adapter-fidelity obligation
  are specified; the contract states plainly that the minimal class has no
  nonvacuous internal algebraic law, and keeps container-specific laws
  container-specific.
- No new class-telescope/registry mechanism, trust-root, or TCB entry is
  introduced (dispatch reuses existing machinery); if the ruled class/carrier
  design would require a new kernel primitive or trust-root, STOP and escalate
  -- that would exceed this node's in-lane posture.
- A reaching seed exists whose expected values are independent of the
  implementation under test.

## Not this node

- `∈`'s name/fixity-target admission -- [[SPEC-RESERVED-INFIX-NAMES]] (A0's
  prereq; this node's `depends_on`).
- The `∧ ∨ ≤ ≥ ≠` standard meanings + the shared completion mechanism --
  [[SPEC-STANDARD-INFIX-BINDING]] (A1's prereq; membership reuses that
  mechanism, does not respecify it).
- The elaborator + catalog implementation (the `Membership` class, the provider
  views/instances, the `∈` dispatch wiring) -- [[LANG-MEMBERSHIP-OPERATOR-SURFACE]]
  (the build, re-cut against this contract + A0 + A1).

## Sizing / tier

**Size M, tier T1.** Larger than the A-track spec nodes: a new class plus three
carrier views plus a two-tier law model plus the `∈` completion policy, and T1
because the coherence (no-overlap under the outer-head key, witness-bound
comparator validity, Bool-primary-no-Prop-elimination) is where the soundness
review turns. Spec-enclave-owned; the Architect is the design authority for the
class/carrier ruling and a required reviewer.

## Contention

Spec enclave, `spec/30-surface/` + `spec/50-stdlib/`. No cross-lane contention
(L1 runtime on `crates/`; L3 foundation on `catalog/`; this is spec-only). It
runs as the PARALLEL B-track design work Pat authorized alongside the A-track
glyph fix: the class/carrier/law design may proceed now, sequenced by the
enclave behind [[SPEC-RESERVED-INFIX-NAMES]] (the `∈` name) and pipelined with
[[SPEC-STANDARD-INFIX-BINDING]] (the shared completion mechanism); only the
`∈`-surface-binding section finalizes once the name admission lands. Re-measure
every cited anchor at the cut.

## Grounded anchors (verified at main 5d1347aaf; re-measure at the cut)

Substrate the contract binds, so the enclave does not re-derive it:

- Class surface is unary: `ClassDecl { param: Option<String>, param_kind:
  Option<Type> }` at `crates/ken-elaborator/src/ast.rs:381-390`; parser
  `parse_class_decl` at `parser.rs:1131-1188`.
- Instance registry keys on `(class_name, head_type_name)`:
  `classes.rs:202` (map) / `:330-333` (`instance_search`); single-`RType`
  `resolve_instance_dictionary` at `elab.rs:9598-9623`. This is why distinct
  roles over one raw head collide and need distinct nominal view heads.
- Use-site implicit dictionary/argument completion is now spelled **`33 §5.4`**
  (NOT "33.5.4"): `spec/30-surface/33-declarations.md:567` (`where C A` -> an
  implicit instance argument); structure-class-as-record `§5.2` at `:453-521`
  (later fields depend on earlier); provider / orphan / floor arms `§5.3-5.5` at
  `:539-799`. Elaboration implicit-arg insertion at
  `spec/30-surface/39-elaboration.md:33,:450`.
- The three workers (spelled as explicit `match` / `Equal Bool .. True`, not
  `is_some` / `IsTrue`, but semantically those): `elem` at
  `catalog/packages/Algorithm/Searching/OrderedSearch.ken.md:32`; Map `member`
  at `Data/Collections/Map.ken.md:142-146`, `set_member` (`v:=Unit`) at
  `:169-170`; `rel_member` (`= Equal Bool (set_member .. (succ ..)) True`) at
  `:15162-15165`.
- `Ordered`/validity premises: `spec/50-stdlib/52-map.md` §2/§2.1 `:90-157`,
  §5.1 `:306-316`; `spec/50-stdlib/58-maps-sets-relations.md` §5 raw-`Tree`-equality
  refusal at **`:281-323`** (relations §7 `:344-514`).
- `IsTrue`: `pub fn IsTrue (b : Bool) : Prop = Equal Bool b True` at
  `catalog/packages/Core/Classes/LawfulClasses.ken.md:54`; spec notation
  `spec/50-stdlib/51-lawful-classes.md:41` and
  `spec/30-surface/37-strings-collections.md:641`.
