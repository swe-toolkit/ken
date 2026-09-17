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
