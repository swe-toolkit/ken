# WP frame: CONF-FMT8-LEVELTOK

**Node:** `docs/program/issues/CONF-FMT8-LEVELTOK.md`
**Owner:** spec-enclave
**Size:** S
**Measurement base:** `88667c204` (`origin/main`, 2026-08-14)
**Candidate base:** whatever `origin/main` is when you cut. Re-measure the
fixed inputs below if it has moved.

## Objective

Repair the FMT8 row whose fixture cannot be constructed, and census the rest of
the formatting seed for the same class: a `RED-UNTIL-BUILT` row that is not
waiting on work but on something the landed lexer will never produce.

**The row is not deleted.** Its discriminator is real — it separates a
raw-byte over-firing canonicalizer from a correct one — and the repair is to
re-express that discrimination in operands the lexer can actually produce.

## The design judgment, front-loaded

**1. The defect is the word "token", and the spec you cite says so itself.**
The row demands *"a genuine level-token fixture using the canonical level
role"*. `spec/30-surface/31-lexical.md:82` puts the level role in the table as
*"role supplied by parser context"*, and the lexer has **no** `Level` or `Label`
token kind — a `Token`-scoped grep over `crates/ken-elaborator/src/lexer.rs`
returns nothing. There is no token to fixture. A row that waits for one waits
forever, and under the ruled endpoint (b) the absence **is** the ruling.

**2. The three spellings are one identifier, and that is the replacement
discriminator.** Measured, not inferred:

| source | site | token |
|---|---|---|
| `ℓ` | `lexer.rs:885` | `Token::Ident("level")` |
| `l` | `lexer.rs:1043` | `Token::Ident("level")` |
| `level` | ordinary identifier path | `Token::Ident("level")` |

That is exactly `31 §1d` as written at `31-lexical.md:89`: *"source `ℓ`, `l`,
and `level` all produce semantic `Ident("level")`"*, with `:159` requiring that
`ℓ` still **remains `ℓ` after formatting while resolving to the same stored
name**. **One binding, three source lexemes, each round-tripping to its own
spelling** — measurable today, and it fails against precisely the buggy
canonicalizer the original row was built to catch.

**Read `31 §1d` before you write the fixture.** The property is a conjunction:
same stored name **and** distinct preserved lexemes. A fixture asserting only
the first is satisfied by a canonicalizer that rewrites all three to one
spelling, which is the bug.

**3. The "unproducible vs pending" marker already exists in this corpus. Reuse
it; do not mint a vocabulary.** In
`conformance/behavioral/buffer-io/seed-buffer-io.md`, line 727 carries:

```
- status: **PARTIAL — interpreter GREEN; native
  BLOCKED-ON-NATIVE-REACHABILITY ([[RT-NATIVE-FNSPLIT]])**
```

A `BLOCKED-ON-<reason> ([[node]])` status, naming the thing that must exist
first. That shape is landed, greppable, and already reviewed. Any row this WP
finds unproducible takes that shape, with the reason and the blocking node
named.

## Fixed inputs, measured at `88667c204`

| object | location | fact |
|---|---|---|
| the FMT8 row | `conformance/surface/formatting/seed-canonical-format.md:304` | `surface/formatting/l-identifier-is-not-a-level-token (ambiguity)` |
| its `given` | same block | `fn keep_l (l : Nat) : Nat = l` beside "a genuine level-token fixture using the canonical level role" |
| its `expect` | same block | `RED-UNTIL-BUILT (B2/B3/C)` |
| level role | `spec/30-surface/31-lexical.md:82` | "role supplied by **parser context**" — not a token kind |
| the alias rule | `31-lexical.md:89` | all three spellings produce semantic `Ident("level")` |
| the preservation rule | `31-lexical.md:159` | `ℓ` remains `ℓ` after formatting |
| `ℓ` lexing | `crates/ken-elaborator/src/lexer.rs:885` | `Token::Ident("level")` |
| `l` lexing | `lexer.rs:1043` | `Token::Ident("level")` |
| `Level`/`Label` token kind | `lexer.rs` | **absent** |
| census population | `seed-canonical-format.md` | **20** `RED-UNTIL-BUILT` occurrences |
| out-of-scope population | elsewhere under `conformance/` | **7** more — 5 in `stdlib/collections/seed-cat3-collection-laws.md`, 1 in `seed-cat4-maps-sets-relations.md`, 1 in `surface/bytes-io/seed-bytes-io.md` |

## Deliverables

**D1.** Repair the FMT8 row: re-express the discriminator over the three
producible spellings, so it asserts one stored name **and** three preserved
lexemes. Keep the row's id and its `why`'s intent.

**D2.** Census the formatting seed's **20** `RED-UNTIL-BUILT` rows: for each,
state whether the landed lexer and the landed formatter surface can produce the
fixture its `given` names. Report the verdict per row, with the fact it turns
on — not a bare yes/no column.

**D3.** For every row `D2` finds unproducible, apply the
`BLOCKED-ON-<reason> ([[node]])` status shape from `seed-buffer-io.md`, naming
what must exist first. If no node exists to name, say so in the row rather than
inventing one — filing tracked work is the Steward's, per `COORDINATION §2`.

**D4.** Record the 7 rows outside the formatting seed as **not swept, and why**.
They turn on collections and bytes-io machinery, not on the lexer, and a census
that silently omits them reads as corpus-wide when it is not.

## Acceptance criteria

**AC-1.** The repaired FMT8 row names no level *token*. **Control:** the
phrase "level token" and any restatement of it is absent from the row.

**AC-2.** The repaired row asserts both halves of `31 §1d` — one stored
`Ident("level")` and three distinct preserved source lexemes. **Control:** a
canonicalizer that rewrote all three to one spelling would fail the row as
written. State that counterfactual in the `why`.

**AC-3.** `D2` covers all 20 rows. **Control:** the count of rows adjudicated
equals the count of `RED-UNTIL-BUILT` occurrences in the file at your base. If
your base's count differs from 20, report the new number rather than the one
here.

**AC-4.** Every unproducible row carries the `BLOCKED-ON-` shape and names its
blocker. **Control:** grep `BLOCKED-ON-` in the seed and require one hit per
row `D2` marked unproducible, and zero for rows it marked producible.

**AC-5.** No row is deleted, and no row's id changes. Repairs are in place.

**AC-6.** `crates/` is byte-identical to the candidate base. **Control:** blob
identity, not a report.

## Banned scope

- **Do not edit `crates/`.** The lexer's behavior is the measurement, not the
  deliverable. If the census concludes the lexer is wrong, that is a finding
  routed to the Steward, not a repair taken here.
- **Do not add a conformance currency checker.** None exists and building one
  is not grounded in this node.
- **Do not reopen endpoint (b).** The absence of a distinct level token is the
  landed ruling; this WP works within it.
- **Do not sweep the other seeds.** `D4` records them as out; widening is a
  re-cut, not an overrun.

## Contention

None. The spec enclave is the only owner of `conformance/surface/formatting/`,
and no other node in the tracker names that path. `SPEC-IDENT-BLESSED`, the
`depends_on` edge, is `merged` — the scheduling dependency it recorded has
been discharged.

The `crates/ken-elaborator` ban keeps this clear of Language's in-flight
`LANG-FOREIGN-CTOR-ARM-REJECT`, which edits `elab.rs`.

## Why this node is worth its slot

A `RED-UNTIL-BUILT` row whose fixture is unproducible is byte-identical, to any
reader, to a row that has not been built yet. It sits red forever and reads as
pending, and nothing in the corpus distinguishes *waiting on work* from
*waiting on something that will never exist*. `SEC1-IFC-R3`'s synthetic
`Disproved` verdicts are the same class.

**The single row is the occasion; `D2` is the deliverable.**
