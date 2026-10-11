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

/// Promise class: transition sentinel, retired if the export-item grammar
/// permits a lowercase dotted segment. MEASURED: `export P.pi` fails at the
/// uppercase-only parser rule, before any ambiguity resolution. CLAIMED:
/// prop intros currently cannot reach the dotted in-scope export choke.
/// THE GAP: if this grammar later changes, the reachable prop intro must take
/// the same dual-meaning verdict as the term reference; this refusal no
/// longer proves that boundary.
#[test]
fn lowercase_prop_intro_export_item_is_currently_unspellable() {
    let err = match elaborate("prop P : Omega where { pi : P } export P.pi") {
        Err(error) => error,
        Ok(_) => panic!("lowercase dotted export grammar unexpectedly admitted"),
    };
    assert!(
        matches!(err, ElabError::ParseError { ref msg, .. }
        if msg.contains("expected uppercase constructor name")),
        "wrong export-item boundary: {err:?}"
    );
}

/// MEASURED: an export item spelling D.C refuses when its checked constructor
/// competes with the module export, with data declared first. CLAIMED: dotted
/// export selection takes the same dual-meaning verdict as term references.
/// THE GAP: the reverse order and the module-only positive are separate pins.
#[test]
fn dotted_data_export_item_refuses_data_first() {
    assert_ambiguous(
        "data D = C module D { pub const C : Int = 0 } export D.C",
        "D.C",
    );
}

/// MEASURED: the same written export item refuses when the module is declared
/// first. CLAIMED: export order does not break the constructor/module tie.
/// THE GAP: import and facade exports have distinct selection routes.
#[test]
fn dotted_data_export_item_refuses_module_first() {
    assert_ambiguous(
        "module D { pub const C : Int = 0 } data D = C export D.C",
        "D.C",
    );
}

/// MEASURED: an unopposed dotted module export is still admitted. CLAIMED:
/// the added ambiguity verdict is conditional on a checked family member.
/// THE GAP: a sole prop intro cannot be exported by this grammar today.
#[test]
fn unopposed_dotted_module_export_item_still_checks() {
    elaborate("module P { pub const Other : Int = 0 } export P.Other")
        .expect("sole uppercase module member exports");
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
