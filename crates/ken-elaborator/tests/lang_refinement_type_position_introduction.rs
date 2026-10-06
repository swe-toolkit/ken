//! Durable behavior pins for spec 34 §5 / 22 §2.1: written refinements
//! either produce a call-site introduction obligation or refuse at their
//! unsupported type position. The examples are fixed-fixture literals, not a
//! census of today's repository text.

use ken_elaborator::{ElabEnv, ElabError, ElabResult, ObligationKind};
use ken_kernel::Term;

fn open_refinements(env: &ElabEnv, result: &ElabResult) -> usize {
    result.obligations.iter().filter(|obligation| {
        matches!(obligation.kind, ObligationKind::RefinementIntroduction)
            && env.is_open_hole(obligation.hole_id)
    }).count()
}

#[test]
fn nested_literal_type_argument_refuses_for_both_values() {
    const REASON: &str = "a refinement nested inside a type is not supported yet: only a binder's, field's or result's own annotation may be refined";
    for value in [5, 6] {
        let mut env = ElabEnv::new().expect("prelude");
        let source = format!(
            "const xs : List ({{ x : Int | Equal Int x 5 }}) = Cons Int {value} (Nil Int)"
        );
        let error = env.elaborate_decl_v1(&source).expect_err("nested literal must not erase");
        assert!(matches!(error, ElabError::TypeMismatch { ref reason, .. } if reason == REASON),
            "{value}: {error:?}");
    }
}

#[test]
fn independent_inductive_converter_refuses_unrecorded_nested_refinements() {
    const REASON: &str = "a refinement nested inside a type is not supported yet: only a binder's, field's or result's own annotation may be refined";
    for source in [
        "data BoxRef = Wrap (List ({x : Int | Equal Int x 5}))",
        "data Indexed (n : {x : Int | Equal Int x 5}) : Type where { Make : Indexed n }",
    ] {
        let mut env = ElabEnv::new().expect("prelude");
        let error = env.elaborate_decl_v1(source)
            .expect_err("unrecorded refinements may not pass inductive conversion");
        assert!(matches!(error, ElabError::TypeMismatch { ref reason, .. } if reason == REASON),
            "{source}: {error:?}");
    }
}

#[test]
fn type_position_literal_and_named_arguments_emit_same_refinement() {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "const six : Int = 6",
        "def Five = { x : Int | Equal Int x 5 }",
        "fn use_lit (x : { y : Int | Equal Int y 5 }) : Int = x",
        "fn use_five (x : Five) : Int = x",
        "fn plain (x : Int) : Int = x",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
    }
    let literal = env.elaborate_decl_v1(
        "theorem lit : Equal Int (use_lit six) six = Proved"
    ).expect("type-position literal argument");
    let named = env.elaborate_decl_v1(
        "theorem named : Equal Int (use_five six) six = Proved"
    ).expect("type-position named argument");
    assert_eq!((open_refinements(&env, &literal), open_refinements(&env, &named)), (1, 1));
    assert_eq!(literal.obligations[0].goal_closed, named.obligations[0].goal_closed,
        "both routes must instantiate the same false predicate at six");

    let plain = env.elaborate_decl_v1(
        "theorem unrefined : Equal Int (plain six) six = Proved"
    ).expect("an unrefined type-position application");
    assert_eq!(plain.obligations.len(), 0, "ordinary type applications stay free");
}

#[test]
fn anonymous_arrow_refinement_template_lifts_over_unnamed_domain() {
    let mut env = ElabEnv::new().expect("prelude");
    let function = env.elaborate_decl_v1(
        "fn g (n : Int) : Bool → ({x : Int | Equal Int x n}) → Int = λb. λx. x"
    ).expect("named binder and anonymous arrow in one checked signature");
    assert_eq!(function.obligations.len(), 0);
    let call = env.elaborate_decl_v1("const c : Int = g 5 True 6")
        .expect("anonymous-arrow call must emit rather than mis-scope");
    assert_eq!(open_refinements(&env, &call), 1);
    let equal = Term::const_(env.globals["Equal"], vec![]);
    let int = Term::const_(env.globals["Int"], vec![]);
    let goal = Term::app(
        Term::app(Term::app(equal, int), Term::IntLit(6.into())),
        Term::IntLit(5.into()),
    );
    assert_eq!(call.obligations[0].goal_closed, goal,
        "the exact obligation is Equal Int 6 5, not a shifted variable");
}

#[test]
fn anonymous_arrows_in_proof_signature_are_not_local_binders() {
    let mut env = ElabEnv::new().expect("prelude");
    let declaration = env.elaborate_decl_v1(
        "axiom ord_shape : (x : Int) → (y : Int) → Equal Int x y → Equal Int y x → Equal Int x y"
    ).expect("the LawfulClasses antisymmetry telescope shape is well-scoped");
    assert!(declaration.obligations.is_empty());
    let next = env.elaborate_decl_v1(
        "axiom or_shape : (b : Bool) → Equal Bool b True → Equal Bool b False → Equal Bool b True"
    ).expect("multiple anonymous arrows must not rebind the earlier b");
    assert!(next.obligations.is_empty());
}

#[test]
fn recursive_view_parameter_records_its_literal_predicate_before_self_calls() {
    let mut env = ElabEnv::new().expect("prelude");
    let recursion = env.elaborate_decl_v1(
        "fn rec (n : Nat) (p : { x : Int | Equal Int x 5 }) : Int = \
         match n { Zero |-> p ; Suc m |-> rec m p }"
    ).expect("recursive view with a literal-refined parameter");
    assert!(recursion.obligations.iter().all(|obligation|
        matches!(obligation.kind, ObligationKind::RefinementIntroduction)),
        "an in-group self-call can emit only refinement introductions here");
    let result = env.elaborate_decl_v1("const result : Int = rec Zero 6")
        .expect("recursive call with incorrect literal argument");
    assert_eq!(open_refinements(&env, &result), 1,
        "the recursive-view declaration must store its parameter template");
}

#[test]
fn refined_result_contract_preserves_its_parameter_template() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl_v1(
        "fn with_result (p : { x : Int | Equal Int x 5 }) : \
         { y : Int | Equal Int y p } = p"
    ).expect("the result refinement is realized by the contract path");
    let call = env.elaborate_decl_v1("const result : Int = with_result 6")
        .expect("literal parameter persists through the contract route");
    assert_eq!(open_refinements(&env, &call), 1);
}

#[test]
fn type_position_requires_callee_is_not_silently_accepted() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("const five : Int = 5").expect("value");
    env.elaborate_decl(
        "fn need (x : Int) : Int requires Equal Int x 5 = x"
    ).expect("requires callee");
    let error = env.elaborate_decl_v1(
        "theorem no_missing_requires : Equal Int (need five) five = Proved"
    ).expect_err("a missing requires argument must fail closed");
    assert!(matches!(error, ElabError::KernelRejected { .. }), "{error:?}");
}
