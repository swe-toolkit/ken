# LANG-SURFACE-IF — `if e then t else f`, elaborating to a `Bool` case analysis

Owner: language. Size: M. Node: [[LANG-SURFACE-IF]] (`ready`).

**Released 2026-08-11.** Re-derive your merge-base from `origin/main`; do not
reuse a SHA from this frame. `LANG-SPACE-PRESTATE-BIND` is in the publisher as
this is written and touches `elab.rs`, `parser.rs`, and `lib.rs` — cut after it
lands.

## What you are doing

`spec/30-surface/32-grammar.md §3` admits `"if" expr "then" expr "else" expr`
and fixes its meaning on the same line: **`= match on Bool`**. The target has
been real matchable data since ES2. **Build the surface and the desugaring onto
it. Do not design the semantics.**

## Fixed inputs — measured, do not re-derive

At `origin/main = f4069bf4`. The advance since research's `8898c426` sweep is
doc-only, so these carry.

| layer | state |
|---|---|
| `lexer.rs` | no `KwIf`/`KwThen`/`KwElse`; keyword map reserves none |
| `parser.rs:1927-1932` | `parse_app_expr` dispatches `Lambda`, `KwLet`, `KwMatch` only |
| `ast.rs` | no conditional node |
| `lib.rs:150-180` | `Bool` pre-registered via `data::elab_data_decl` as `data Bool = True \| False`, stable `GlobalId` reused by `register_numeric_env` |
| `modules.rs:75-83` | unshadowable prelude floor is exactly `["Bool", "Char", "List"]` |
| `resolve.rs:1807` | constructor patterns are `RPatKind::Ctor(String, _)` — resolved by bare name |
| `lossless.rs` | round-trips the surface; `tests/kenfmt_b3_layout.rs` pins layout |

**You are not building `Bool`, a branch primitive, or an eliminator.** All three
exist.

## The design call is made, with its reasoning, so evidence can overturn it

**A conditional AST node, desugared after resolution, against the registered
constructor identities.** Not a parser-level rewrite into `match`.

Two independent constraints force it:

1. **Constructor capture.** `Bool` the type is in the unshadowable floor. **Its
   constructors are not**, and the floor's own comment says everything else
   `prelude.rs` builds is *"a package-level convenience, not an unshadowable
   prelude member."* Since patterns resolve by bare name, `if` expanded into
   surface arms spelled `True`/`False` means whatever those names mean at the
   use site.
2. **Lossless printing.** A parser that rewrites `if` before the AST exists
   cannot print `if` back, so `kenfmt` would silently rewrite user source into a
   different construct.

**If you find a third option that satisfies both, take it and say so.** What is
not available is satisfying one and not the other.

## Deliverables

1. **Tokens and reservations** for `if`, `then`, `else`.
2. **A conditional expression node** in `ast.rs`, and its resolved counterpart.
3. **The `parse_app_expr` arm**, correct against application and operator
   parsing.
4. **Elaboration** to a `Bool` case analysis bound to the **registered
   constructor identities**, in both checking and inference mode.
5. **Lossless print and layout** for the new node.
6. **A specific diagnostic** for a non-`Bool` scrutinee.

## Acceptance criteria

**AC-1 — it computes, on both branches.** `if` on a literal `True` and on a
literal `False` each reduce to the corresponding branch. State the reduced term.
An `if` that elaborates without computing is not this AC.

> ### AC-2 — THE CAPTURE CONTROL, AND IT IS THE POINT OF THE NODE
>
> **A file that binds its own `True` and `False` must not change what `if`
> means.**
>
> Write a file that declares its own two-constructor data type spelling its
> constructors `True` and `False`, then uses `if` on an ordinary prelude `Bool`
> scrutinee. **The `if` must still select the prelude `Bool` branches** — or
> fail closed with a diagnostic that names the conflict.
>
> **Silently retargeting to the user's constructors is the failure this AC
> exists to catch, and it is the direction nothing else reds on.** Three test
> files already declare their own `data Bool = True | False`
> (`l3a_acceptance.rs`, `es2_acceptance.rs`, `val1_string_literals.rs`), so this
> is a shape the corpus already contains.
>
> **State which constructor identity each branch resolved to**, not merely that
> the test passed. A pass here is only meaningful with the identity named.

**AC-3 — a non-`Bool` scrutinee gets its own diagnostic.** `if 3 then a else b`
must fail with an error that names the conditional and the expected `Bool`.
**A leaked `match` exhaustiveness or coverage error is a fail** — it would tell
the user about a construct they did not write.

**AC-4 — the lossless round-trip.** A source file containing `if` prints back as
`if`, not as a `match`. Add the layout case alongside the existing
`kenfmt_b3_layout.rs` pins.

**AC-5 — it is an expression.** Cover `if` in argument position, `if` as a
`let` bound value, and a nested chain (`if a then x else if b then y else z`).
For the chain, **state which `if` each `else` binds to** — an associativity that
reads correctly on one example and wrongly on another is the ordinary trap here.

**AC-6 — no trusted-base delta.** `Bool` and its eliminator are already
registered; a surface conditional adds nothing to the TCB. Assert
`trusted_base()` is unchanged. **If a candidate needs a new trusted primitive to
branch, a premise has failed — stop and come back.**

## Excluded scope

- **Pattern guards.** `32-grammar.md §3` lines 217-223 puts guards on `match`
  arms. Separate node.
- **`else if` as its own production.** The chain must work; it must not need a
  second rule.
- **Changing the prelude floor.** Making `True`/`False` unshadowable is a
  `30-taxonomy §4` question. **The fix here is that `if` never goes through
  those names.** If you find yourself editing `install_prelude_floor`, stop —
  that is the wrong layer and it comes back to me.
- **`filter` and the deferred `l3a` library work.** This unblocks it; it does
  not deliver it.
- **Any other `expr` production.** Pairs, records, `forall`, implicit binders,
  and the literal surface are all measured gaps with their own nodes coming.
  **Do not widen into them**, however tempting the adjacency.

## Contention

`crates/ken-elaborator` and its tests. Runtime is on `crates/ken-runtime` under
`RT-LEXICAL-RECURSOR-CONSUMERS`. **No `spec/` or `conformance/` path, so no Spec
vote on the merge Decision** — you are implementing a written spec line, not
amending one.

`LANG-SPACE-PRESTATE-BIND` touches `elab.rs`, `parser.rs`, and `lib.rs`. Cut
after it lands and the intersection is empty by construction.

## Validation

Targeted only. `-p ken-elaborator`, or `--test <name>`, **never `--workspace`**.
A new AST variant makes the floor a full `-p ken-elaborator` test build — a
suite-scoped run cannot observe an exhaustive `match` in a sibling target, and
this change adds a variant that `lossless.rs`, `layout.rs`, and `resolve.rs` all
match on. "No regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. Both are good
outcomes.

**The likely stop is `AC-2`.** If the registered constructor identities are not
reachable at the point where the desugaring has to happen, that is a real
finding and it is worth the turn — report the layer where the identity is lost
rather than falling back to name-spelled arms to get green.
