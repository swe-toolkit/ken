# LANG-SURFACE-RECORD-LITERAL — record literals, punning, and functional update

Owner: language. Size: M. Node: [[LANG-SURFACE-RECORD-LITERAL]].
Fixed inputs originally measured at `origin/main` = `a6438b76`, and **re-pinned
2026-08-11 against `e55dc44d`** — the record declaration form. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Depends on [[LANG-SURFACE-RECORD-DECL]], which has LANDED** — the declaration
form, the registry, and the extended `infer_proj` lookup are on `main`. The
dependency is discharged and the record half of this node is unblocked. `D0`
below is independent of it and can still be done first.

> ### The re-pin, and what it changed
>
> This frame's design judgment was measured before the declaration form
> existed. Re-checked against `e55dc44d`: **the judgment survives unchanged and
> every cited coordinate moved by exactly `+30`** — the declaration form added
> thirty lines to `parser.rs` above line 1650. The coordinates in the next
> section are the re-pinned ones. **A stale line range does not error; it
> silently repoints at unrelated real code**, which is why they are restated
> rather than left to the reader.

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if your
diff stays in `crates/`.

## What this deliverable is

The three surface forms `33 §2` names alongside field access:

```
{ x = 1, y = 2 }        -- record literal
{ x, y }                -- punning
{ p | y = 3 }           -- functional update
```

**No new kernel term, and `trusted_base()` does not move.** A record is a
right-nested Σ, `Term::Pair` exists, and [[LANG-SURFACE-PAIR]] landed tuple
introduction. These are surface spellings of a term the elaborator already
builds.

## The design judgment, front-loaded

**The brace fork you were warned about does not exist here, and I measured it
rather than assuming it.** [[LANG-SURFACE-RECORD-DECL]]'s excluded-scope
section said these forms "open a brace fork this node does not need", citing
`31-lexical.md:194`'s pairing of "record/refinement braces". Re-measured at
`e55dc44d`, the SHA that landed the declaration form:

- `parse_atom_expr_base` (`crates/ken-elaborator/src/parser.rs:2276`) has **no
  `Token::LBrace` arm at all**. Expression-position `{` is free. **The
  declaration form did not change this** — `record` is a declaration keyword,
  so it never reached expression position.
- The refinement brace `{ x : A | φ }` is parsed by `parse_type`
  (`parser.rs:1680`, refinement at `:1684-1686`), and `parse_type` and
  `parse_expr` (`:1841`) are **separate entry points**.

⇒ The lexer pairs the two braces in its vocabulary; the parsers never contend
for the same position. **That makes this node smaller than its sibling
expected. It does not make `AC-5` optional** — it changes what the enumeration
finds, not whether you run it.

**One token of lookahead decides all three forms.** All open `{ Ident`; the
fork is the token after:

| next token | form |
|---|---|
| `=` | record literal |
| `,` or `}` | punning |
| `\|` | functional update |

`LL(2)`, no backtracking. **If you find yourself backtracking, stop and tell
me** — it means one of those rows is wrong and I would rather fix the frame
than have you work around it.

**Functional update is the only form with real content.** `{ p | y = 3 }`
projects every field of `p` except the updated ones and rebuilds the Σ. The
other two are field-ordering problems.

> ### PUNNING IS NAME CAPTURE. Treat it as the risky form, not the trivial one.
>
> `{ x, y }` means `{ x = x, y = y }`, where the right-hand `x` resolves in the
> **enclosing scope**. That is the one place in this node where a resolution
> bug produces a **wrong answer rather than an error**: if the field name
> shadows or is shadowed, you silently build a record from the wrong binding.
> `AC-3` is the control, and it is the AC I most expect to catch something.

## Deliverables

**`D0` — the carried tie-direction control.** See the section below. It is
independent of everything else here and does not touch records; do it first so
it cannot be squeezed out at the end.

**`D1` — record literals parse and elaborate.** `{ x = 1, y = 2 }` checked
against a declared record type builds the right-nested Σ value, with fields
placed by **name**, not by the order written. `{ y = 2, x = 1 }` must produce
the same value.

**`D2` — punning.** `{ x, y }` desugars to `{ x = x, y = y }` with the
right-hand occurrences resolved in the enclosing scope.

**`D3` — functional update.** `{ p | y = 3 }` produces a record equal to `p` in
every field but `y`. **Multiple updated fields in one form** (`{ p | x = 1, y =
3 }`) either work or are refused with a span; say which and why.

**`D4` — the refusals.** A field named that the record type does not have; a
field omitted from a literal; a field given twice. Each with a span naming the
field and the record type. **Not a panic, and not a silently short Σ.**

## Acceptance criteria

**AC-1 — the value, not the success.** Assert the elaborated **term** for
`{ x = 1, y = 2 }`, and assert that `{ y = 2, x = 1 }` elaborates to the
**same** term. A success assertion passes for an implementation that builds the
Σ in written order and is wrong for every record whose fields are not written
in declaration order.

**AC-2 — three fields, because two cannot distinguish the bug.** Use a
three-field record for the ordering controls. **With two fields, "placed by
name" and "placed by written order" agree on one of the two permutations and
the reversal is the only witness** — a three-field record has permutations that
are wrong under written-order placement in ways a two-field one cannot express.

**AC-3 — punning resolves in the enclosing scope, under shadowing.** Construct
a program where a local binding named `x` differs from anything the record
could supply by accident, and assert `{ x, y }` picks up the local. **Then the
discriminating half:** a case where field-order placement and scope resolution
would give different answers, so the control fails if punning is implemented as
"fill positionally".

**AC-4 — update is η-respecting.** `{ p | }` or the nearest admissible
empty-update spelling is **definitionally equal to `p`** — assert the equality,
not that it elaborates. If the surface admits no empty update, assert instead
that `{ p | y = y_of_p }` equals `p`, and say in the handback which you did.
This is the AC that proves you rebuilt the Σ rather than constructing a fresh
unrelated value that merely type-checks.

**AC-5 — enumerate the neighbours these forms must not capture, and control
each.** A list with a control per entry, not a sentence.

Minimum set, and note it is **shorter than the sibling's for a measured
reason** — the two parsers are separate:

- the **refinement type** `{ n : Int | n ≥ 0 }` still parses in type position,
  **including in a position adjacent to a record literal in the same program**;
- `class C (A) { … }` and `instance C T { … }` bodies still parse;
- `module M { … }` still parses;
- a **block or record-shaped brace in any other expression position** the
  parser already admits — if you find none, say so explicitly, because "there
  were none" is a finding and "I did not look" is not.

**Why this AC exists.** `LANG-LEX-HEX-FLOAT` took four Architect rejections,
**every one on the scanner**, while its genuinely intricate half was right in
round one. A new surface form is risky in proportion to how many neighbouring
constructs it sits beside. **This node's measured answer is that the number is
small** — but the enumeration is what establishes that, and it was skipped
once already in this arc.

**AC-6 — the declaration path and the projection seam are unchanged.**
`LANG-SURFACE-RECORD-DECL`'s controls still pass; `p.1`, `p.1.2` and `p. 1.2`
behave as [[LANG-SURFACE-PAIR]] and [[LANG-LEX-PROJECTION-ADJACENCY]] left
them; `(1, 2)` checked against a two-field record still constructs.

**AC-7 — the A/B.** Disable the literal branch and show `{ x = 1, y = 2 }`
fails to parse in expression position **while `{ n : Int | n ≥ 0 }` still
parses in type position.** The second half is the informative one: it proves
the branch you added is the one being exercised and that you did not
accidentally route type-position braces through it.

**AC-8 — no `spec/` edit, no new kernel term, `trusted_base()` unchanged.**

## `D0` — the carried tie-direction control, inherited from `LANG-LEX-HEX-FLOAT`

**This is not records work and it is not optional.** It is one assertion in an
existing file, carried here because prose in a merged node is a claim about the
past that no gate reads.

**The finding.** Adversary, measured on `a28a7a33` after
[[LANG-LEX-HEX-FLOAT]] merged. The hex-float control set contains two genuine
ties — `0x100000000000008p-56` (exactly `1 + 2^-53`) and `0x1p-1075` (exactly
half the smallest subnormal) — and **both resolve toward zero, and in both the
even neighbour is the lower one.** So **nearest-ties-even and
nearest-ties-toward-zero produce identical results on every control in the
file.** The pair at lines 12-13 excludes truncation of non-tie values; it says
nothing about tie *direction*.

**The distinguishing case is a tie whose even neighbour is the upper one:**

```
0x100000000000018p-56   -- = 1 + 3*2^-53, halfway between 1+2^-52 and 1+2^-51
```

Ties-even rounds **up**, to `1.0000000000000004`. Ties-toward-zero would give
`1.0000000000000002`.

**AC-D0 — the control discriminates, or you show it cannot exist.** Add the
assertion to `crates/ken-elaborator/tests/lang_lex_hex_float.rs`, in the shape
of the controls already there.

> **The Adversary's own bound rides with this, and it changes what discharges
> the AC.** It read the controls, not the conversion. **If the bit assembly
> makes an up-tie structurally unreachable, this deliverable is discharged by
> demonstrating that** — with the reason, at `file:line` — **not by adding an
> assertion that cannot fail.** A control added to close an empty gap is worse
> than the gap, and this arc has now filed that same shape against itself
> twice.

**This is a control, not a repair.** There is no evidence the implementation is
wrong; nearest-ties-even is very likely exactly what it does. **If the
assertion fails, that is a stop — report it, do not fix the conversion in this
node.**

## Excluded scope

- **Record patterns in `match`** — `34 §3`'s, and `34-data-match.md:272`
  already specifies them.
- **Named-argument application and record-field constructor labels** —
  explicitly deferred by `34-data-match.md:170-173` as `SURF-gadt-field-sugar`.
- **Any change to the record declaration or the registry.** That is
  [[LANG-SURFACE-RECORD-DECL]]'s. If these forms need the registry to expose
  something it does not, that is a stop, not an edit.
- **No implicit binders, no unification work.** The elaborator has no term
  metavariables; that is a foundational node and not this one.

## Stop conditions — return to me, do not decide

- **You need to backtrack** to tell the three forms apart.
- ~~**The registry does not expose field names in declaration order**, so
  name-directed placement cannot be implemented against it.~~ **RESOLVED at the
  re-pin — this stop cannot fire. See the registry fixed input below.**
- **Functional update cannot be built from projections and `Pair`** without a
  new kernel term.
- **The `D0` assertion fails.**
- **`AC-5` turns out not to be cheap** — that would mean a brace ambiguity this
  frame measured away, and the measurement is then wrong.

## Contention

`crates/ken-elaborator/` — `parser.rs` (expression-position brace), `elab.rs`,
`classes.rs`, plus `tests/lang_lex_hex_float.rs` for `D0` only.

**This node follows [[LANG-SURFACE-RECORD-DECL]] in the same lane and touches
the same files.** Do not run them concurrently. **Re-derive the intersection at
candidate time** — a merge-base goes stale without your branch moving.

## Sizing and validation

`scripts/ken-cargo test -p ken-elaborator` plus your focused suite.
**Never `--workspace`**; that is CI's gate.

## The registry fixed input — measured, not deferred

This frame used to tell you to go read `LANG-SURFACE-RECORD-DECL`'s registry
choice before estimating. **It has landed, so the answer is stated here
instead**, measured at `e55dc44d` in `crates/ken-elaborator/src/classes.rs`.

One private `named_field_owners: HashMap<String, NamedFieldInfo>` (`:195`),
**keyed by owner name** — the record or class name, not the field name. Each
entry pairs a `ProjectionInfo` with a closed `NamedFieldKind` of
`Class(ClassOnlyInfo) | Record` (`:133-141`). `class()` and `class_entries()`
return `None` on the `Record` arm; `projection_by_type_id` reaches both.

**The part that decides `AC-1` and kills a stop condition.**
`register_record` (`:263`) takes `field_names: Vec<String>` and
`field_types: Vec<Term>`, and `ProjectionInfo` stores them as ordered vectors.
**Declaration order is preserved.** So given a record's type id,
`projection_by_type_id` yields the ordered field names, and name-directed
placement is implemented by looking up each written field's index in that
vector. **You do not need a registry change, and the stop condition above
cannot fire.**

One thing to notice rather than fix: `projection_by_type_id` (`:255`) is a
linear scan over the map. That is fine at present sizes and **is not this
node's problem** — if a literal-heavy program makes it matter, that is a
finding to report, not an optimization to land here.
