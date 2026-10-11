//! Spec 33 §3.3, §8.1: a prop intro and authorized module export with the
//! same written path are ambiguous; neither declaration order breaks the tie.
//! Promise class: durable checked-reference invariant.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

fn elaborate(source: &str) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(source)?;
    Ok(env)
}

fn assert_ambiguous(source: &str, path: &str) {
    let err = match elaborate(source) {
        Err(err) => err,
        Ok(_) => panic!("expected ambiguous term reference: {source}"),
    };
    match err {
        ElabError::AmbiguousReference { name, sources, .. } => {
            assert_eq!(name, path);
            assert_eq!(
                sources,
                vec![format!("module {path}"), format!("type {path}")]
            );
        }
        other => panic!("expected ambiguous {path}: {other:?}; source: {source}"),
    }
}

/// MEASURED: term references to P.pi reject with both sources in both
/// declaration orders. CLAIMED: neither an export's late arrival nor a prop
/// intro's late arrival silently selects one meaning. THE GAP: this only
/// exercises direct same-unit visibility, not imported or hidden selectors.
#[test]
fn top_level_prop_intro_and_module_export_are_ambiguous_in_both_orders() {
    for source in [
        "prop P : Omega where { pi : P } module P { pub const pi : Int = 0 } theorem t : P = P.pi",
        "module P { pub const pi : Int = 0 } prop P : Omega where { pi : P } theorem t : P = P.pi",
    ] {
        assert_ambiguous(source, "P.pi");
    }
}

/// MEASURED: a nested written M.P.pi rejects in both declaration orders.
/// CLAIMED: the qualified path remains ambiguous when the owner is nested.
/// THE GAP: the distinct-owner file import identity case is an AC-2 control.
#[test]
fn nested_prop_intro_and_module_export_are_ambiguous_in_both_orders() {
    for inner in [
        "pub prop P : Omega where { pi : P } module P { pub const pi : Int = 0 }",
        "module P { pub const pi : Int = 0 } pub prop P : Omega where { pi : P }",
    ] {
        let source = format!("module M {{ {inner} }} theorem t : M.P = M.P.pi");
        assert_ambiguous(&source, "M.P.pi");
    }
}

/// MEASURED: P.pi inside its enclosing module rejects despite the module's
/// canonical M.P prefix being longer than the locally written family path.
/// CLAIMED: lexical qualification cannot hide the two authorized meanings.
/// THE GAP: this remains a local term use, not an imported selector.
#[test]
fn nested_local_prop_intro_and_export_reject_both_orders() {
    for inner in [
        "prop P : Omega where { pi : P } module P { pub const pi : Int = 0 }",
        "module P { pub const pi : Int = 0 } prop P : Omega where { pi : P }",
    ] {
        let source = format!("module M {{ {inner} theorem t : P = P.pi }}");
        assert_ambiguous(&source, "P.pi");
    }
}

/// MEASURED: a computational consumer cannot silently select the Int export
/// over the prop intro in either declaration order. CLAIMED: ambiguity is a
/// surface reference error, not an eventual kernel type mismatch. THE GAP:
/// this direct-use row does not cover imports or private member lookup.
#[test]
fn computational_reference_does_not_choose_export_over_intro() {
    for source in [
        "module P { pub const pi : Int = 0 } prop P : Omega where { pi : P } const z : Int = P.pi",
        "prop P : Omega where { pi : P } module P { pub const pi : Int = 0 } const z : Int = P.pi",
    ] {
        assert_ambiguous(source, "P.pi");
    }
}

/// MEASURED: a dotted in-scope export item cannot pick the module member
/// over the same written prop introduction, in either declaration order or
/// when both declarations are nested. CLAIMED: export items share the term
/// reference's dual-meaning verdict. THE GAP: facade and import selectors
/// have different authorization gates and are outside this direct-use row.
#[test]
fn dotted_prop_intro_export_item_rejects_both_orders_at_both_depths() {
    for inner in [
        "prop P : Omega where { pi : P } module P { pub const pi : Int = 0 }",
        "module P { pub const pi : Int = 0 } prop P : Omega where { pi : P }",
    ] {
        assert_ambiguous(&format!("{inner} export P.pi"), "P.pi");
        assert_ambiguous(&format!("module M {{ {inner} export P.pi }}"), "P.pi");
    }
}

/// MEASURED: the pre-existing data constructor/type collision is also
/// rejected by a written dotted export, regardless of source order.
/// CLAIMED: one verdict guards both checked family-member kinds. THE GAP:
/// the positive export controls distinguish this from blanket dotted refusal.
#[test]
fn dotted_data_constructor_export_item_rejects_both_orders() {
    for inner in [
        "data D = C module D { pub const C : Int = 0 }",
        "module D { pub const C : Int = 0 } data D = C",
    ] {
        assert_ambiguous(&format!("{inner} export D.C"), "D.C");
    }
}

/// MEASURED: unopposed intro and module exports are accepted through the
/// identical dotted export-item syntax. CLAIMED: the ambiguity gate rejects
/// actual dual meanings only. THE GAP: this exercises local public selection,
/// not imported or private members.
#[test]
fn unopposed_dotted_export_items_still_check() {
    elaborate("prop P : Omega where { pi : P } export P.pi").expect("sole checked intro exports");
    elaborate("module P { pub const other : Int = 0 } export P.other")
        .expect("sole module member exports");
}

/// MEASURED: a sole prop intro remains available by its checked family path,
/// independently of an ordinary module binding with a different leaf.
/// CLAIMED: ambiguity is conditional on the same authorized export path.
/// THE GAP: this is a bounded positive witness, not imported visibility.
#[test]
fn unambiguous_prop_intro_stays_addressable() {
    let source = "prop P : Omega where { pi : P } module P { pub const other : Int = 0 } theorem t : P = P.pi";
    let env = elaborate(source).expect("unique prop intro checks");
    let intro = env.globals["P.pi"];
    let theorem = env.globals["t"];
    let (_, body) = env
        .env
        .transparent_body(theorem)
        .expect("checked theorem body");
    assert!(matches!(body, Term::Const { id, .. } if id == intro));
    assert!(matches!(
        env.env.lookup(intro),
        Some(Decl::Transparent { .. })
    ));
}

/// MEASURED: the existing checked data constructor versus module-export
/// ambiguity retains the same module/type sources. CLAIMED: the intro branch
/// extends, rather than displaces, the existing constructor tie-break.
/// THE GAP: this is a data family control, not proof-intro membership.
#[test]
fn data_constructor_module_export_ambiguity_control() {
    assert_ambiguous(
        "data D = C module D { pub const C : Int = 0 } const z : Int = D.C",
        "D.C",
    );
}
