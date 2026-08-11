---
id: LANG-SURFACE-IF
title: "`if e then t else f` is required by 32-grammar §3 and is wholly absent -- no token, no keyword-map entry, no parser arm, no AST node -- while its stated elaboration target (real matchable `data Bool = True | False`) has been pre-registered since ES2"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Steward measurement 2026-08-11 at origin/main=f4069bf4, from the research surface-gap sweep at 8898c426 (evt_3dsd7j9t4r33a), which ranked conditionals the cleanest next one-turn Language candidate. The intervening main advance is doc-only, so the code measurement carries. The constructor-capture hazard below is a Steward finding on top of that sweep, not part of it.
---

## The gap

`spec/30-surface/32-grammar.md §3` admits the conditional in the `expr`
production and fixes its meaning in the same line:

```
| "if" expr "then" expr "else" expr  -- = match on Bool
```

**Nothing implements it.** Measured at `origin/main = f4069bf4`:

| layer | state |
|---|---|
| `lexer.rs` | no `KwIf`/`KwThen`/`KwElse` token; the keyword map reserves none of the three |
| `parser.rs:1927-1932` | `parse_app_expr` dispatches `Lambda`, `KwLet`, `KwMatch` — there is no `if` arm |
| `ast.rs` | no conditional expression node |
| `resolve.rs`, `elab.rs` | nothing downstream to resolve or elaborate |

**The elaboration target is already built.** `lib.rs:150-180` pre-registers
`Bool` as a real inductive through `data::elab_data_decl` — `data Bool = True |
False`, matchable, with a stable `GlobalId` that `register_numeric_env` reuses.
The ES2 change that demoted the former opaque postulate is what made `match` on
`Bool` work. **This node is surface plus desugaring onto a landed target**, the
same character as `SURF-SPACE-CELLS`, and it is not a design fork.

## The absence is user-visible, and the corpus has two fossils of it

- `tests/surface_transport_acceptance.rs:176-181` writes a hand-rolled `match`
  and says in a comment that it *"stands in for `if leq k k' then … else …`
  (the Map shape)"*.
- `tests/l3a_acceptance.rs:12-13` defers `filter` outright: *"`filter` is
  deferred — it needs Boolean branching."*

**A deferred list function and a stand-in comment in a transport proof are the
same defect seen twice.** `leq` already returns `Bool` and is threaded through
`insert` and `sort` in that suite; the only missing piece is the branch.

> ## THE HAZARD RESEARCH DID NOT SURFACE: `Bool` IS PROTECTED, `True`/`False` ARE NOT
>
> `modules.rs:75-83` installs the unshadowable prelude floor, and it is a closed
> set of **three type names**:
>
> ```rust
> // `30-taxonomy §4` derives this exact closed set from the built-in
> // primitive signatures. Other definitions constructed in prelude.rs
> // are package-level conveniences, not unshadowable prelude members.
> self.prelude_names = ["Bool", "Char", "List"]
> ```
>
> **`True` and `False` are not in it**, and the comment is explicit that
> everything else `prelude.rs` builds is a package-level convenience rather than
> an unshadowable member.
>
> `resolve.rs:1807` carries constructor patterns as
> `RPatKind::Ctor(name.clone(), rsubs)` — **a bare `String`, resolved
> downstream by name.**
>
> ⇒ **A desugaring that expands `if` into surface `match` arms spelled `True`
> and `False` is captured by whatever those two names resolve to at the use
> site.** `Bool` the type cannot be shadowed; its two constructors can.
>
> **This is not hypothetical — three existing test files declare their own
> `data Bool = True | False`** (`l3a_acceptance.rs`, `es2_acceptance.rs`,
> `val1_string_literals.rs`). The shape is already in the corpus.
>
> **The failure is silent and it is the bad direction.** A file that binds its
> own `True` would make every `if` in it mean something else, with no error, no
> red test, and a plausible-looking elaboration.

## What that settles about the cut

Two constraints point at the same answer, so it is front-loaded here rather than
left to the turn:

1. **Capture, above** — the branch selection must be against the pre-registered
   `Bool`'s constructor identities, not the names `True`/`False` in user scope.
2. **Lossless printing.** `lossless.rs` round-trips the surface, and
   `tests/kenfmt_b3_layout.rs` pins layout. **A parser that rewrites `if` into a
   `match` before the AST exists cannot print `if` back** — the formatter would
   silently rewrite user source into a different construct.

⇒ **A real AST node, desugared after resolution against the registered
constructor ids.** Desugaring in the parser is cheaper and fails both
constraints at once.

## Scope

**IN:** the three keyword tokens and their reservations; a conditional AST node;
the `parse_app_expr` arm and its interaction with application and operator
parsing; resolution; elaboration to a `Bool` case analysis against the
registered constructor ids; the lossless-print and layout path; a specific
diagnostic when the scrutinee is not `Bool`.

**OUT:**

- **`if` as a pattern guard.** `32-grammar.md §3` lines 217-223 puts guards on
  `match` arms; that is the pattern-language node and it is separate.
- **`else if` as its own construct.** It is an `else` whose expression is
  another conditional and needs no rule of its own; a chain must work, but not
  by a second production.
- **Making `True`/`False` unshadowable.** The floor is derived from
  `30-taxonomy §4` and changing it is a taxonomy question, not this node's. The
  fix here is that `if` does not go through those names at all.
- **`filter` and the deferred `l3a` library work.** Unblocked by this, not
  delivered by it.
