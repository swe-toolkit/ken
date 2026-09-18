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
