# Membership — the provider class behind `∈` (B-track, Layer 2)

> Status: **DRAFT v0** (`SPEC-MEMBERSHIP-CLASS-CONTRACT`). This chapter is the
> **contract** for the `Membership` class, its standard providers, and its law
> model. It adds no kernel feature: the class is a record (`../30-surface/33
> §5.2`, `../10-kernel/13 §3` Σ+η) and its laws are `Ω` propositions
> (`../10-kernel/16 §1`). The standard `∈` binding, its fixity and the
> use-site completion policy are fixed where the standard operator bindings
> are specified, and are not restated here.
>
> **This is a class chapter sitting in the container block, and that is
> deliberate.** The other class contracts are `51` (structure classes), `55`
> (constructor classes) and `56` (effectful constructor classes); those three
> are split by the shape of the class parameter because the tranches were cut
> that way, and each carries its own tranche banner. `Membership` belongs to
> neither of those tranches, and `51` in particular exists to explain why `Eq`,
> `DecEq` and `Ord` lead ES4 — a fourth class arriving from a later tranche
> would contradict that section rather than extend it. It is numbered beside
> `58`/`58a` because its standard providers draw from exactly those containers.
> A reader scanning `51`-`56` for classes will not find it here, which is why
> the `README` index entry for this chapter is load-bearing rather than tidy.


This section fixes the class, its providers and its law model. The standard
`∈` binding, its fixity and the use-site completion policy are fixed where the
standard operator bindings are specified, and are not restated here.

## 1. The class — unary, with an associated query type

```
class Membership (container : Type) {
  Query  : Type
  member : Query → container → Bool
}
```

**Unary over the container, with the query type an associated field.** It is
not a two-parameter `Membership query container`: instance resolution keys on
**one outermost head** (`39 §6.1`), so a second class parameter has nowhere to
live. The query type is therefore carried *in* the dictionary, and `Query`
precedes `member` in the field telescope so `member` may name it.

**The record lands one universe above an ordinary class, and the contract says
so rather than leaving it to be discovered.** A field whose *type* is `Type ℓ`
contributes a component at sort `Type (suc ℓ)` (`12 §1`'s `Type ℓ : Type
(suc ℓ)`), and the Σ-sort takes the maximum over components (`13 §4`), so:

    Membership : Type ℓ → Type (suc ℓ)

**This is the class former of `33 §5.2` operating, not an exception to it.**
That rule elaborates a class to the right-nested Σ over its field telescope and
fixes no level; the level is whatever the kernel computes. `33 §5.1` keys the
property/structure discriminant on the record's **kernel-computed sort family**
— `Ω` iff every field is `Ω`-valued, `Type` the moment any field is relevant —
so `Membership` is an ordinary **structure class**, with `Query` relevant. Its
level is a consequence of that classification, not a new kind of declaration.

> `33 §5`'s introductory sentence describes a class as `C : Type → Type`. That
> is a description of the common case in a motivating passage, not a formation
> constraint — the same chapter's `class Traversable (f : Type → Type)` already
> exceeds it on the parameter axis. `Membership` exceeds it on the level axis.
> The formation rule is `§5.2`, and the discriminant is `§5.1`.

**`Membership` is the first class in the catalog to carry a field at the
universe `Type`.** That is a measurement, not a turn of phrase: no other
catalog class declares one, while ordinary `Type`-valued *operation* fields are
common — so the zero is about a field whose type **is** a universe, not about
`Type`-valued fields generally. It is worth stating because it means this
chapter is the first place the level consequence above can arise, and a reader
who expects the `C : Type → Type` shape has never yet had cause to doubt it.

Being a structure class, `Membership` is subject to the canonical-one-per-head
resolver convention (`33 §5.5`). That is what forces `§3`'s nominal views.

## 2. The proposition view — `Bool` is primary

```
member_holds (c : Type) (d : Membership c) (q : d.Query) (x : c) : Ω
member_holds c d q x := IsTrue (d.member q x)
```

`IsTrue b := Equal Bool b True : Ω` (`51 §2`), so `member_holds` is an `Ω`
proposition and is proof-irrelevant.

**Its signature is given in full because its third parameter is typed by a
projection**, `d.Query`, exactly as `membership_member_at`'s is (`33 §6.3`).
It is a standalone binding over a dictionary, not a class field, so the class
stays at its two fields — and it therefore carries the **same** surface
prerequisite: Ken's type grammar has no projection form, so this binding cannot
be *written* until one exists, and `LANG-MEMBERSHIP-OPERATOR-SURFACE` authors
**three** projection-typed bindings rather than one — this, `§4`'s
`same_members`, and `membership_member_at`. A defining equation with no
ascription would have hidden that, which is why the type is stated.

**`member_holds` is a definition over the `Bool` result, never an elimination
from `Ω`.** There is no law recovering a `Bool` from a proof of `member_holds`,
and a provider must not be given one: the checked `Bool` computation is the
primary artifact and the proposition is read off it. An implementation that
derives membership by eliminating an `Ω`-valued predicate is non-conforming
even where it computes the same answers.

## 3. Providers are nominal, witness-bound views — never raw heads

Because the resolver is canonical per outermost head, distinct membership roles
over the *same* raw head collide. **Installing `Membership Tree` is therefore
forbidden**: key membership, set membership and relation-edge membership are
three different meanings over one head, and at most one could be canonical.

The standard providers are **distinct nominal view types**, each owning its
operations *and its validity witness in the value*:

    list view            (d : Ord a, xs : List a)               Query = a
    ordered-key view     (d : Ord k, t : Tree k v,
                          ordered : Ordered d.leq t)            Query = k
    relation-edge view   (d, adjacency, the outer Ordered, and
                          evidence every stored successor tree
                          is Ordered under that same d)         Query = Pair k k

The ordered-key view at `v = Unit` serves set membership; this mints no `Set`
carrier and no second head.

**The comparator must be the one the view value was validated with.** A raw
`Tree` binds no comparator in its type, so an implementation that resolves a
canonical `Ord` at each `member` call can silently use an order different from
the one the tree is `Ordered` under, and answer wrongly on a well-typed input.
The witness travels in the view value for exactly this reason; **an implicit
resolver choosing a fresh `Ord` at the use site is non-conforming.**

## 4. The law model, and where it honestly stops

The shared **observational relation** says two containers are indistinguishable
when every query agrees. It is a **definition, not a law** — it states no
obligation and nothing discharges it:

    same_members (c : Type) (d : Membership c) (x : c) (y : c) : Ω
    same_members c d x y := (q : d.Query) → Equal Bool (d.member q x) (d.member q y)

**It is a standalone definition, not a class field**, and `§1`'s two-field
declaration is what settles that. A class elaborates to a right-nested Σ over
its field telescope (`33 §5.2`), so a field is a component an instance must
**supply** — that is, precisely a thing that *is* discharged — and a record
declares fields rather than defining them, leaving nowhere for the `:=`. Read
as a field this would be a slot each provider fills with **any** relation, with
nothing tying it to the observational one.

Being standalone, it takes the dictionary as a parameter and so writes
`d.Query` in type position, exactly as `§2`'s `member_holds` does. **It carries
the same surface prerequisite**, which brings the count in `§2` to **three**
projection-typed bindings: `membership_member_at`, `member_holds` and
`same_members`.

It is `Ω`-clean with no truncation: `Equal Bool _ _ : Ω`, and a `Π` whose
codomain is `Ω` is itself `Ω` (`13 §4`). Its reflexivity, symmetry and
transitivity follow from `Bool` equality and are reusable across providers.

> **It is deliberately NOT an equality obligation.** A definition concluding
> `Equal container x y` from pointwise agreement would be **uninhabitable by
> the first standard provider**: over the list view `[1,2]` and `[2,1]` agree
> on every query and are not `Equal`. That is the same fact recorded below as
> permutation being container-specific — so the conclusion a reader might
> supply is one the chapter already knows to be false.

**Each provider additionally carries an adapter-fidelity obligation:** its
class `member` equals the existing explicit worker under the exact stored
witness. This is the obligation that has to be nonvacuous — a wrong
comparator, a swapped relation endpoint, a value-pair query where a key query
was meant, or a duplicated worker must each falsify it.

**The class's shared law layer is therefore empty, and the contract records
that rather than manufacturing content for it.** `same_members` is a
definition; the whole of the nonvacuous obligation is the per-provider
adapter fidelity above. There is no nonvacuous algebraic law inside the
minimal two-field class either. Empty/insert/lookup
laws cannot even be stated without structure the class does not have, and a
`Belongs` field proven equal to `IsTrue (member ..)` is vacuous — it restates
`§2`'s definition and discharges by `Refl`. Container-specific laws stay
container-specific: `List` `Cons`/`Nil` and permutation; `Map`
member-iff-`lookup`-`Some` under `Ordered`/`Distinct`; set extensionality as a
Boolean algebra, with raw `Tree` equality still refused for union laws
(`58 §5`); relation edge-extensionality with converse, composition and
reachability under the shared ordering premises.

## 5. Why a class field cannot serve as a resolution key

Any mechanism that must select this class's operation **before** a dictionary
is in hand — the use-site completion of `∈` is the case this contract exists
for — cannot key on `member`. The alternative is worth stating rather than
merely avoiding, because it is the one a careful author reaches for and it
survives every objection except the decisive one.

**What the alternative is.** Designate the operation by the pair
*(the class's `type_id`, the field name `member`)*. The class record has a
canonical identity; the field is named rather than positional; the pair
discriminates a user's own class carrying a field of the same name, because
that class has a different `type_id`; and the identity travels under import
and re-export exactly as a binding's does (`33 §4.3`). None of the stability
objections reach it. In particular, designating the field by **name** is not
the same proposal as designating it by **index** — an index is positional, so
inserting a field ahead of `member` would silently re-point it, and a name is
not.

**Why it is rejected anyway.** A class's field types form a Σ-telescope
(`33 §5.2`): a field's type is well-formed only relative to the class parameter
and **every earlier field**. It is therefore not a closed telescope on its own;
it becomes one only once a dictionary has been **resolved and substituted**.
Resolving that dictionary is precisely what the selecting mechanism is doing.
**The key would be closed only by the act it is supposed to key.** That is
circular, and no amount of stability in the designation repairs it.

**What minting changes, stated because the contrast is the whole argument.** A
top-level binding takes the dictionary as its own parameter, so the query type
is reached as a projection from that parameter rather than from a sibling
field. The projection is then closed by an entry the binding itself binds —
ordinary dependency, exactly like `(a : Type) (x : a)`. **Minting is what
converts a field-relative type into a parameter-relative one**, and that
conversion, not convenience, is why an operator whose standard meaning is a
class method needs a binding of its own.

The two obstructions are therefore different, and reading them as one has
already misled once: *unspellable* and *unclosable* are not the same defect.
The field route is **unclosable** at the point of use and stays rejected. The
minted route is closed and kernel-checkable today, and is merely **unspellable**
in Ken's current surface type grammar — a gap in the surface, recorded with the
binding, not a defect in this design.

**No catalog binding today types a parameter by projecting an earlier one**, so
the minted binding would be the first. Again a measurement rather than an
impression, and the control is what makes it one: projection itself is in
ordinary catalog use — a class dictionary's field is projected in binding
**bodies** in several places — so the zero is specifically about projection in
**type** position, not about projection being unavailable. The gap is narrow
and it is exactly where this design meets it.
