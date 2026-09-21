# WP frame — `LANG-R-LAYER-EXPORT-RETRACTION`

    owner   language       tier   T2        size   S
    depends none
    node    docs/program/issues/LANG-R-LAYER-EXPORT-RETRACTION.md
    review  Architect NOT required -- the design fork is already ruled.
            Language QA exact-SHA. Code merge → full CI + M8 Adversary.

## 1. Objective

Retract `ken-elaborator`'s R layer from the crate's externally nameable
surface. It is an internal IR that was never meant to be public, and it has
zero consumers outside the crate.

## 2. Fixed inputs

Measurement anchor: `3de9a5030a91ab7133817157617c2705d4e1cfbd`. Re-ground every
coordinate at your own base before editing; this node's coordinates have moved
under it before.

- `lib.rs:50` is `pub mod resolve;` and `lib.rs:114` is
  `pub use resolve::{RDecl, RDeclKind, RExpr, RType}` -- **four** names, not 26.
- **Zero consumers outside `ken-elaborator`.** No other crate, bench, example or
  binary names `ken_elaborator::resolve`, `RExpr` or `RDeclKind`.
- **Eight integration test files consume the R layer**, and they are external to
  the lib target, so `pub(crate)` breaks them. They use exactly **three
  functions** -- `resolve_decl`, `resolve_decls`, `resolve_expr_standalone` --
  and **five types** -- `RDecl`, `RDeclKind`, `RExpr`, `RType`, and
  `RInfixOperator`, which the `pub use` does not name:

      acceptance.rs  effects.rs  kenfmt_b3_layout.rs  kenfmt_b4_splicing.rs
      kenfmt_let_layout.rs  lang_fixity_decl_surface.rs
      lang_structural_result_elab.rs  let4_multi_binding.rs

  `lang_roots_loader_local_instance_dict_scope.rs:190` is a **false hit** -- the
  word "resolve" in an assertion message. Do not relocate it.
- **The node's own "six of the nine functions" is anchored to the abandoned
  branch `a8f0873b0`, not to `main`.** The census above supersedes it. The node's
  "the retraction compiles" is from that same branch and is not a claim about
  your base.
- **The sibling `LANG-INSTANCE-REGISTRY-IDENTITY-KEY` has LANDED** (`83f30f5e8`)
  and took Architect arm (b), **not** the registry re-key: `classes.rs:202` is
  still `HashMap<(String, String), InstanceInfo>`. So this node runs second and
  re-grounds its `classes.rs` coordinates, but the key type it expected to find
  changed is unchanged.

## 3. The unit of retraction, and the trap

**The unit is the item's visibility, not the module declaration and not the
re-export.** `pub(crate) enum RExpr` inside `pub mod resolve;` is not nameable
outside the crate: a public enclosing module does not re-export a crate-private
item. The `pub use` deletion is a **consequence** of narrowing the items, not
the mechanism -- you cannot re-export an item you just made crate-private.

**The trap, stated so it cannot be walked into:** `pub mod resolve;` plus a
`pub` item means `ken_elaborator::resolve::RExpr` resolves with or without the
re-export. A candidate that deletes the re-export, re-runs the consumer census,
gets zero and reports the surface closed would be **green and wrong**. The
census returns zero in both worlds -- the one where the path is closed and the
one where nobody was using it anyway -- so it cannot be the evidence. AC-1 is
written to separate them.

Do not restructure the 34 `pub mod` declarations. That is a crate-wide change
for a retraction reachable as `pub(crate)` on roughly thirty items in two files.

## 4. Deliverable

Narrow the R layer's items so nothing in it is nameable from outside the crate,
and relocate the eight integration test files' R-layer usage into the crate as
unit tests.

**Architect ruling, already made and not reopenable here: move those tests
in-crate.** A `#[doc(hidden)] pub` seam is **explicitly rejected** -- it is a
public surface asked not to look like one, the same failure shape as the export
it would cover up, and it removes the evidence of the defect while leaving the
defect.

**Narrow the four roots first, then measure what is still reachable.** Several of
the field-carrying types are public only *because* a public field reaches them;
they may fall out automatically. Do not hand-edit a list of declarations you
have not re-derived. `pub(crate)` is not presumed to be the right answer for
every item -- some may want to be fully private.

**No TCB claim.** The R layer sits outside the trust root; it produces
certificates the kernel re-checks. Retracting these exports does not shrink the
TCB, and no acceptance criterion or commit message here may say it does.

## 5. Acceptance

**AC-1 — the closure is proved by compilation, not by a census.** A control that
names `ken_elaborator::resolve::RExpr` from outside the crate must **fail to
compile** while `pub mod resolve;` is still present in `lib.rs`. The same control
must **compile** against the base. A candidate whose only evidence is a
zero-consumer census fails this criterion, because that census reads zero in
both worlds.

**AC-2 — the relocation preserves the tests, and adds no seam.** Every assertion
moved in-crate still runs and still asserts the same thing; the total `#[test]`
count across the crate is unchanged or higher, and `#[ignore]` count is
unchanged. The candidate's diff contains **no `#[doc(hidden)]`**, and
`lang_roots_loader_local_instance_dict_scope.rs` is untouched.

**AC-3 — the narrowed set is measured, not assumed.** Name the instrument that
lists the crate's externally nameable items, and show the R layer present in it
at the base and absent after. State which items you narrowed and which fell out
as a consequence of narrowing the four roots, as two separate lists. An item
left `pub` must be named with the reason it is still reachable.

## 6. Stop condition

Hand back, rather than working around, on any of:

- An R-layer item turning out to have a consumer outside `ken-elaborator` after
  all. That changes the node's premise and is the Steward's to route.
- A test whose assertion cannot be made in-crate without weakening it. Report
  which and why; do not keep a public seam to preserve it, and do not delete the
  assertion.
- Any fixed input above measuring false at your base.
