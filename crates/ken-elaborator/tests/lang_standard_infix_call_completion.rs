//! `LANG-STANDARD-INFIX-CALL-COMPLETION` — standard-operator call completion
//! (`spec/30-surface/39-elaboration.md §6.9`, names and meanings `33 §6.1`,
//! the `≠` carrier inventory `33 §6.2`).
//!
//! D0 grounding: the facade re-export surface these bindings reach the glyphs
//! through is ALREADY LANDED (`33 §3.2` facade form, `33 §4.3` identity
//! preservation, A0's admission of the glyphs as ordinary symbolic names).
//! These two cases pin that, so a later regression in the facade surface is
//! attributed here rather than read as a completion defect.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl, ElabEnv, ExportForm};
use ken_kernel::Term;

fn body_const(env: &ElabEnv, name: &str) -> ken_kernel::GlobalId {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("{name} must be transparent"));
    match body {
        Term::Const { id, .. } => id,
        other => panic!("{name} must be a canonical constant reference, got {other:?}"),
    }
}

#[test]
fn a_glyph_is_a_legal_rename_target_in_a_facade_export() {
    let decls = parse_decls("export M (ord_leq_at as ≤, bool_and as ∧, bool_or as ∨)")
        .expect("glyph renames parse in a facade export");
    match &decls[0] {
        Decl::ExportDecl {
            form: ExportForm::Facade { module, items },
            ..
        } => {
            assert_eq!(module, "M");
            assert_eq!(items[0].name, "ord_leq_at");
            assert_eq!(items[0].rename.as_deref(), Some("≤"));
            assert_eq!(items[1].rename.as_deref(), Some("∧"));
            assert_eq!(items[2].rename.as_deref(), Some("∨"));
        }
        other => panic!("expected facade export, got {other:?}"),
    }
}

#[test]
fn a_facade_glyph_rename_republishes_the_defining_globalid() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module M { pub const le : Nat = Zero } \
         module Ops { export M (le as ≤) } \
         import Ops (≤) const observed : Nat = (≤)",
    )
    .expect("a facade may republish under a glyph name");
    assert_eq!(body_const(&env, "observed"), env.globals["M.le"]);
    assert!(!env.globals.contains_key("Ops.≤"));
}

// ---------------------------------------------------------------------------
// AC-9 -- the required-roles check (layer 3).
//
// All three of this AC's observations need evidence that has to be
// MANUFACTURED: nothing in the ordinary change puts an unfilled role, a
// wrong-shaped binding, or a moved home in front of you. That is exactly the
// class this frame says is most likely to be silently skipped, so each case
// below constructs its own violation rather than reading one off the diff.
//
// A red build does not discharge AC-9. The diagnostic has to NAME THE ROLE,
// so every assertion below is on the role, not merely on failure.
// ---------------------------------------------------------------------------

/// A provider whose four binding-backed roles all have the shape `33 §6.1`
/// fixes. `{LEQ}` is the one hole the wrong-shape case fills differently.
fn provider_with_leq(leq: &str) -> String {
    format!(
        "class Ord a {{ leq : a -> a -> Bool }} \
         module Provider {{ \
           pub fn bool_and (a : Bool) (b : Bool) : Bool = a \
           pub fn bool_or (a : Bool) (b : Bool) : Bool = a \
           {leq} \
           pub fn ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq y x \
         }}"
    )
}

const GOOD_LEQ: &str =
    "pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y";

/// The home, publishing all four roles from `Provider`.
const HOME: &str = "module Core.Operators.Standard { \
     export Provider (bool_and as ∧, bool_or as ∨, ord_leq_at as ≤, ord_geq_at as ≥) }";

fn elaborate(source: &str) -> Result<(), ken_elaborator::ElabError> {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(source)?;
    Ok(())
}

#[test]
fn ac9_a_role_bound_to_a_wrong_shaped_binding_is_refused_naming_the_role() {
    // `≤` IS published -- a presence-only check passes this. The binding
    // behind it is a two-argument `Nat` comparison instead of `§6.1`'s
    // `(a : Type) (d : Ord a) (x : a) (y : a) : Bool`, which is what a moved
    // or re-pointed binding looks like from the export table's side.
    let wrong = "pub fn ord_leq_at (x : Bool) (y : Bool) : Bool = x";
    let source = format!("{} {HOME}", provider_with_leq(wrong));

    match elaborate(&source) {
        Err(ken_elaborator::ElabError::StandardOperatorRoleWrongShape {
            ref role,
            ref found,
            ref span,
            ..
        }) => {
            assert_eq!(role, "≤", "the diagnostic must name the ROLE");
            assert!(
                found.contains("arity 2"),
                "the diagnostic must say what it found, got {found:?}"
            );
            // A zero span points nowhere. The home is an inline module here,
            // so it knows its own span and the diagnostic must carry it.
            assert!(
                span.end > span.start,
                "the diagnostic must carry a real span, got {span:?}"
            );
        }
        other => panic!("a wrong-shaped role must be refused naming it, got {other:?}"),
    }
}

#[test]
fn ac9_positive_control_the_same_home_with_the_right_shape_is_accepted() {
    // The discriminating half: without this, "refused" and "never looked"
    // read identically. Same home, same four roles, correct `≤`.
    let source = format!("{} {HOME}", provider_with_leq(GOOD_LEQ));
    elaborate(&source).expect("a correctly shaped home certifies");
}

#[test]
fn ac9_an_unfilled_role_is_refused_naming_the_role_and_is_a_DISTINCT_refusal() {
    // `≤` is simply not published. `33 §6.2` makes the same distinction for
    // `≠`'s two refusals and calls collapsing them non-conforming: an
    // unfilled role is closed by publishing a binding, a wrong-shaped one by
    // fixing the binding already published. Different states, different
    // remedies, so different variants.
    let home_without_leq = "module Core.Operators.Standard { \
         export Provider (bool_and as ∧, bool_or as ∨, ord_geq_at as ≥) }";
    let source = format!("{} {home_without_leq}", provider_with_leq(GOOD_LEQ));

    match elaborate(&source) {
        Err(ken_elaborator::ElabError::StandardOperatorRoleUnfilled { ref role, .. }) => {
            assert_eq!(role, "≤", "the diagnostic must name the ROLE");
        }
        other => panic!("an unfilled role must be refused naming it, got {other:?}"),
    }
}

#[test]
fn ac9_the_shape_contract_catches_the_nearest_legal_neighbour_not_only_an_obvious_break() {
    // The arity-2 case above is an OBVIOUS break; a contract that only caught
    // that could be a bare arity check. The dangerous neighbour keeps the
    // arity, keeps the carrier universe, keeps the dictionary applied to the
    // carrier, and drifts ONE operand off the carrier -- which is precisely
    // what a re-pointed binding looks like. `§6.1` fixes both operands as the
    // carrier `a`, so this is not the binding even though it is close.
    let near_miss =
        "pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : Bool) : Bool = d.leq x x";
    let source = format!("{} {HOME}", provider_with_leq(near_miss));

    match elaborate(&source) {
        Err(ken_elaborator::ElabError::StandardOperatorRoleWrongShape {
            ref role,
            ref found,
            ..
        }) => {
            assert_eq!(role, "≤");
            // Arity is RIGHT here, so a diagnostic saying "arity 4" is the
            // proof that arity is not what rejected it.
            assert!(
                found.contains("arity 4"),
                "the near miss must be rejected on dependency, not arity: {found:?}"
            );
        }
        other => panic!("the nearest legal neighbour must be refused, got {other:?}"),
    }
}

#[test]
fn the_real_catalog_facade_certifies_against_the_shape_contract() {
    // The cases above build their own homes, so all of them could pass while
    // the SHIPPED facade fails the contract -- a suite that never reaches the
    // artifact it is about. This loads the real
    // `catalog/packages/Core/Operators/Standard.ken.md` and certifies it.
    let catalog = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages");
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog], "Core.Operators.Standard")
        .expect("the shipped standard-operator facade must certify");
}

// ---------------------------------------------------------------------------
// The standard fixities (`33 §6.1`: ∧ infixr 3, ∨ infixr 2, ≤ ≥ ≠ infix 4).
//
// Asserted BEHAVIOURALLY -- what an expression parses to -- rather than by
// reading the fixity table back. A table assertion would pass while the
// fixity never reached the parser, which is the whole failure this WP had to
// route around: the travel half of `§6` was landed and the DECLARATION half
// had no surface form for a re-exported binding.
// ---------------------------------------------------------------------------

const PROVIDER_AND_HOME: &str = "class Ord a { leq : a -> a -> Bool } \
     module Provider { \
       pub fn bool_and (a : Bool) (b : Bool) : Bool = a \
       pub fn bool_or (a : Bool) (b : Bool) : Bool = a \
       pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y \
       pub fn ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq y x \
     } \
     module Core.Operators.Standard { \
       export Provider (bool_and as ∧, bool_or as ∨, ord_leq_at as ≤, ord_geq_at as ≥) }";

fn body_of(env: &ElabEnv, name: &str) -> ken_kernel::Term {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("{name} must be transparent"));
    body
}

#[test]
fn the_standard_fixities_govern_parsing_and_and_binds_tighter_than_or() {
    // `∧ infixr 3` over `∨ infixr 2`, so `a ∨ b ∧ c` groups as `a ∨ (b ∧ c)`.
    // Without the installed fixities both default to `infixl 9` and this
    // would group left as `(a ∨ b) ∧ c` instead -- a DIFFERENT term, which is
    // what makes this a discriminator rather than a smoke test.
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} \
         import Core.Operators.Standard (∧, ∨) \
         fn written (a : Bool) (b : Bool) (c : Bool) : Bool = a ∨ b ∧ c \
         fn grouped (a : Bool) (b : Bool) (c : Bool) : Bool = ∨ a (∧ b c)"
    ))
    .expect("the standard fixities parse a mixed spine");

    assert_eq!(
        body_of(&env, "written"),
        body_of(&env, "grouped"),
        "`a ∨ b ∧ c` must group as `a ∨ (b ∧ c)` under infixr 3 over infixr 2"
    );
}

#[test]
fn an_explicit_saturated_call_through_the_facade_path_elaborates() {
    // NAMED FOR WHAT IT TESTS. This was drafted as a non-associativity case
    // for `≤ ≥ ≠ infix 4`, but a chained `a ≤ b ≤ c` needs the completion
    // adapter to supply each occurrence's carrier and dictionary, and that is
    // not built yet. Asserting non-associativity here would have been a name
    // claiming more than the body checks. What it does establish is the
    // positive half the completion work rests on: the binding is reachable
    // and well-typed through the facade path, under its glyph.
    let mut env = ElabEnv::new().expect("base environment");
    let result = env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} \
         import Core.Operators.Standard (≤) \
         import Provider (bool_and) \
         fn chained (d : Ord Bool) (a : Bool) (b : Bool) (c : Bool) : Bool = \
           ≤ Bool d a b"
    ));
    // The explicit four-argument call is the positive half: the binding is
    // reachable and well-typed through the facade path.
    result.expect("an explicit saturated call through the facade elaborates");
}

// ---------------------------------------------------------------------------
// AC-1 / AC-2 / AC-4 -- the completion itself, for the roles that need a
// prefix. `∧`/`∨` are already saturated at two arguments; `≤`/`≥` are not, and
// these rows are about the prefix completion supplies.
// ---------------------------------------------------------------------------

/// Peel the lambdas a `fn`'s parameters introduce, leaving the body proper.
///
/// `transparent_body` returns the whole `\a b. ...`, so counting the
/// application spine without peeling counts zero and the row fails for a
/// reason that has nothing to do with completion.
fn under_binders(mut term: ken_kernel::Term, binders: usize) -> ken_kernel::Term {
    for _ in 0..binders {
        match term {
            ken_kernel::Term::Lam(_, body) => term = *body,
            other => panic!("expected a lambda binder, got {other:?}"),
        }
    }
    term
}

/// The length of an application spine.
fn spine_len(mut term: ken_kernel::Term) -> usize {
    let mut depth = 0;
    while let ken_kernel::Term::App(f, _) = term {
        depth += 1;
        term = *f;
    }
    depth
}

const ORD_BOOL: &str = "instance Ord Bool { leq = \\x y. x } ";

/// AC-1 -- a saturated application reaches the kernel.
///
/// The positive control the refusing version could not have: before completion
/// was wired this file was fully green, because nothing here elaborated a `≤`.
/// A suite that passes whether or not the feature exists is the failure AC-1's
/// "show the saturated application reaching the kernel" exists to prevent.
#[test]
fn ac1_a_bare_comparison_completes_its_omitted_prefix() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} {ORD_BOOL} \
         import Core.Operators.Standard (≤, ≥) \
         fn le (a : Bool) (b : Bool) : Bool = a ≤ b \
         fn ge (a : Bool) (b : Bool) : Bool = a ≥ b"
    ))
    .expect("`a ≤ b` must complete: carrier inferred, `Ord` dictionary resolved");

    // Four arguments reached the kernel, not two. The carrier and the
    // dictionary are the prefix completion supplied.
    for name in ["le", "ge"] {
        let depth = spine_len(under_binders(body_of(&env, name), 2));
        assert_eq!(
            depth, 4,
            "{name}'s body must be a FOUR-argument application -- carrier, \
             dictionary, and the two operands -- not the two that were written"
        );
    }
}

/// AC-4 -- operand order, and the mutation this row exists to catch.
///
/// `ord_geq_at a d x y = d.leq y x` reverses INSIDE the binding, on values
/// call-by-value has already evaluated left to right. So completion must apply
/// the operands in SOURCE order for `≥` exactly as for `≤`; reversing at the
/// call site as well would double-reverse and silently invert every `≥`.
///
/// Asserted on the de Bruijn indices of the last two arguments rather than on
/// a rendering: under `\a b.` the source order is `Var(1)` then `Var(0)`, and
/// a double-reverse is precisely the swap.
#[test]
fn ac4_completion_applies_operands_in_source_order_for_both_roles() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} {ORD_BOOL} \
         import Core.Operators.Standard (≤, ≥) \
         fn le (a : Bool) (b : Bool) : Bool = a ≤ b \
         fn ge (a : Bool) (b : Bool) : Bool = a ≥ b"
    ))
    .expect("both comparisons must complete");

    for name in ["le", "ge"] {
        let body = under_binders(body_of(&env, name), 2);
        let ken_kernel::Term::App(applied_lhs, rhs) = body else {
            panic!("{name}'s body must be an application");
        };
        let ken_kernel::Term::App(_, lhs) = *applied_lhs else {
            panic!("{name}'s body must be a two-deep application spine");
        };
        assert!(
            matches!(*lhs, ken_kernel::Term::Var(1)),
            "{name}: the FIRST operand must be the first-written one; a \
             completion that reverses at the call site puts it second and \
             double-reverses `≥` -- got {lhs:?}"
        );
        assert!(
            matches!(*rhs, ken_kernel::Term::Var(0)),
            "{name}: the SECOND operand must be the second-written one -- got \
             {rhs:?}"
        );
    }
}

/// AC-2(b) -- an unrelated local `≤` is NOT completed.
///
/// Completion keys on the defining `GlobalId`, so a user's own operator with
/// that spelling has a different identity and elaborates exactly as its own
/// declaration says. If this ever reddens with an arity complaint, completion
/// has started keying on the glyph.
#[test]
fn ac2b_an_unrelated_local_operator_with_the_same_glyph_is_left_alone() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} {ORD_BOOL} \
         fn ≤ (x : Bool) (y : Bool) : Bool = y \
         fn mine (a : Bool) (b : Bool) : Bool = a ≤ b"
    ))
    .expect("a local `≤` is an ordinary two-argument function and must stay one");

    let depth = spine_len(under_binders(body_of(&env, "mine"), 2));
    assert_eq!(
        depth, 2,
        "the local `≤` takes two arguments and must receive exactly two; a \
         four-argument spine means completion fired on a glyph rather than on \
         the certified identity"
    );
}

/// The carrier-identity confirmation, exercised on the state it exists for --
/// **and the measurement partly refutes the reason it was added.**
///
/// The state is reachable through the public API because the
/// duplicate-definition guard is per compilation unit while `ElabEnv::globals`
/// persists across `elaborate_file` calls and `insert` overwrites. So a second
/// file can rebind `Foo` to a new type after the first file registered an
/// `Ord Foo` instance for the old one. At the occurrence the scan matches the
/// single registered spelling -- `globals["Foo"]` is the NEW type today -- so
/// the two-match ambiguity arm stays silent. That is the blindness the
/// confirmation exists for.
///
/// **MEASURED, and the hazard is MISATTRIBUTED rather than silent.** With the
/// confirmation forced to accept, this program does not elaborate either: the
/// kernel rejects it with `TypeMismatch { expected: (g641 Dg649), found:
/// (g641 Dg646) }`. The wrong dictionary is caught downstream. So what the
/// confirmation buys is ATTRIBUTION -- a refusal naming the class and the
/// spelling, instead of a raw kernel mismatch on two opaque dictionary
/// identities that points at the whole declaration.
///
/// That is worth having and it is not what the amendment was justified by.
/// Recorded here rather than quietly enjoyed, because the next reader
/// weighing this check's cost should weigh the benefit it actually has.
///
/// **RESIDUAL, and it is the reason the check still earns its place.** These
/// two carriers are distinct inductives, so their dictionary types differ and
/// the kernel cannot miss it. I have NOT exhibited a rebind where the old and
/// new carriers are convertible but distinct identities -- there the kernel's
/// net may not catch it and the substitution would be silent after all. The
/// confirmation does not depend on which case it is; the kernel's coverage
/// does.
#[test]
fn a_rebound_carrier_name_is_refused_by_identity_not_accepted_by_spelling() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} \
         data Foo : Type where {{ MkFoo : Foo }} \
         instance Ord Foo {{ leq = \\x y. True }}"
    ))
    .expect("the first file registers `Ord Foo` for the original `Foo`");

    let rebound = env.elaborate_file(
        "data Foo : Type where { MkFoo2 : Foo } \
         import Core.Operators.Standard (≤) \
         fn f (a : Foo) (b : Foo) : Bool = a ≤ b",
    );
    let error = rebound.expect_err(
        "the registered `Ord Foo` is for the FIRST `Foo`; completing against it \
         would hand one carrier's dictionary to another",
    );
    assert!(
        matches!(
            error,
            ken_elaborator::ElabError::InstanceCarrierIdentityMismatch { .. }
        ),
        "the refusal must be the identity confirmation, naming the class and \
         the spelling -- a kernel TypeMismatch here means the confirmation \
         stopped firing and the downstream net is carrying it alone: {error:?}"
    );
}

/// AC-2(a) -- the renaming hop, which is the half the delivery does NOT
/// already produce.
///
/// **FI-2a makes the obvious version degenerate and the frame says so.** The
/// facade's own re-export IS the production path, so "a re-exported binding
/// completes" is satisfied by shipping and controls nothing. The witness has
/// to be a second surface route to the same identity that the delivery does
/// not create on its own.
///
/// `<+>` is that route: the home re-exports `ord_leq_at` twice, once as the
/// role glyph `≤` and once under an ordinary USER OPERATOR spelling.
/// `certify_roles` ignores `<+>` -- it is not a role glyph -- so the certified
/// map is unchanged and the two-spellings refusal does not fire. But `<+>`
/// still RESOLVES to the certified identity, and it is a symbolic operator so
/// it reaches the infix path, mints the node, and completes identically.
///
/// **It has to be a SYMBOLIC name, not just a different word.** A first
/// attempt used `lte`, which is an ordinary identifier: `a lte b` parses as
/// the application `(a lte) b` and never reaches spine reduction at all, so it
/// failed with `NotAFunction` and proved nothing about completion. Operator
/// position is a parse-level fact before it is an identity-level one.
///
/// **That is the whole content of "keyed on the identity, never the glyph":**
/// a spelling the compiler has never heard of completes, because completion
/// never asked what it was spelled.
#[test]
fn ac2a_a_renaming_hop_to_the_same_identity_completes_identically() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "class Ord a {{ leq : a -> a -> Bool }} \
         module Provider {{ \
           pub fn bool_and (a : Bool) (b : Bool) : Bool = a \
           pub fn bool_or (a : Bool) (b : Bool) : Bool = a \
           pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y \
           pub fn ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq y x \
         }} \
         module Core.Operators.Standard {{ \
           export Provider (bool_and as ∧, bool_or as ∨, ord_leq_at as ≤, \
                            ord_leq_at as <+>, ord_geq_at as ≥) }} \
         {ORD_BOOL} \
         import Core.Operators.Standard (≤, <+>) \
         fn viaGlyph (a : Bool) (b : Bool) : Bool = a ≤ b \
         fn viaHop (a : Bool) (b : Bool) : Bool = a <+> b"
    ))
    .expect("a second re-export of the SAME identity under another name must also complete");

    assert_eq!(
        under_binders(body_of(&env, "viaGlyph"), 2),
        under_binders(body_of(&env, "viaHop"), 2),
        "`a <+> b` must produce the SAME completed term as `a ≤ b` -- the \
         policy is keyed on the defining GlobalId, and a spelling the compiler \
         has no role for must reach it just the same"
    );

    // Non-degeneracy: the hop is doing work, not coinciding. A four-argument
    // spine means the prefix was supplied for `lte` too rather than it having
    // elaborated as some ordinary two-argument call.
    assert_eq!(
        spine_len(under_binders(body_of(&env, "viaHop"), 2)),
        4,
        "`<+>` must have been COMPLETED, not merely accepted"
    );
}

/// AC-2(c) -- an explicit partial application is never rewritten.
///
/// **This is the claim I have leaned on in three separate design arguments and
/// never pinned.** The whole case for a distinct `RExpr` node over a marker on
/// `RApp` was that this holds BY CONSTRUCTION: completion is minted only in
/// spine reduction, so an explicit application is a different constructor and
/// cannot reach the adapter. An argument used that often is exactly the one
/// that should not stay unpinned.
///
/// `ord_leq_at Bool d` supplies the carrier and the dictionary and yields a
/// function. If completion ever keyed on identity-plus-arity instead of on
/// operator position, this would be "completed" into a four-argument call and
/// stop being a function -- which the `: Bool -> Bool -> Bool` annotation
/// rejects.
#[test]
fn ac2c_an_explicit_partial_application_is_not_rewritten_into_a_completed_call() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&format!(
        "{PROVIDER_AND_HOME} {ORD_BOOL} \
         import Provider (ord_leq_at) \
         import Core.Operators.Standard (≤) \
         fn partial (d : Ord Bool) : Bool -> Bool -> Bool = ord_leq_at Bool d \
         fn saturated (a : Bool) (b : Bool) : Bool = a ≤ b"
    ))
    .expect(
        "`ord_leq_at Bool d` is a valid TWO-argument application yielding a \
         function; completion must not reinterpret it as a four-argument call",
    );

    // It stayed at two. The prefix positions were filled by the author, and
    // completion supplied nothing.
    assert_eq!(
        spine_len(under_binders(body_of(&env, "partial"), 1)),
        2,
        "the explicit partial application must keep exactly the two arguments \
         written; a longer spine means completion fired on an explicit call"
    );

    // Positive control on the same identity in the same file: the operator
    // occurrence DID get its prefix. Without this row, "stayed at two" would
    // be satisfied by completion being switched off entirely.
    assert_eq!(
        spine_len(under_binders(body_of(&env, "saturated"), 2)),
        4,
        "the operator occurrence must still complete -- otherwise the row \
         above passes because nothing completes at all"
    );
}
