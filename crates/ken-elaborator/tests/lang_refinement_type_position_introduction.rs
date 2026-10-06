//! Durable behavior pins for spec 34 §5 / 22 §2.1: written refinements
//! either produce a call-site introduction obligation or refuse at their
//! unsupported type position. The examples are fixed-fixture literals, not a
//! census of today's repository text.

use ken_elaborator::{ElabEnv, ElabError, ElabResult, ObligationKind};
use ken_kernel::Term;

fn open_refinements(env: &ElabEnv, result: &ElabResult) -> usize {
    result
        .obligations
        .iter()
        .filter(|obligation| {
            matches!(obligation.kind, ObligationKind::RefinementIntroduction)
                && env.is_open_hole(obligation.hole_id)
        })
        .count()
}

#[test]
fn nested_literal_type_argument_refuses_for_both_values() {
    const REASON: &str = "a refinement nested inside a type is not supported yet: only a binder's, field's or result's own annotation may be refined";
    for value in [5, 6] {
        let mut env = ElabEnv::new().expect("prelude");
        let source =
            format!("const xs : List ({{ x : Int | Equal Int x 5 }}) = Cons Int {value} (Nil Int)");
        let error = env
            .elaborate_decl_v1(&source)
            .expect_err("nested literal must not erase");
        assert!(
            matches!(error, ElabError::TypeMismatch { ref reason, .. } if reason == REASON),
            "{value}: {error:?}"
        );
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
        let error = env
            .elaborate_decl_v1(source)
            .expect_err("unrecorded refinements may not pass inductive conversion");
        assert!(
            matches!(error, ElabError::TypeMismatch { ref reason, .. } if reason == REASON),
            "{source}: {error:?}"
        );
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
    let literal = env
        .elaborate_decl_v1("theorem lit : Equal Int (use_lit six) six = Proved")
        .expect("type-position literal argument");
    let named = env
        .elaborate_decl_v1("theorem named : Equal Int (use_five six) six = Proved")
        .expect("type-position named argument");
    assert_eq!(
        (
            open_refinements(&env, &literal),
            open_refinements(&env, &named)
        ),
        (1, 1)
    );
    assert_eq!(
        literal.obligations[0].goal_closed, named.obligations[0].goal_closed,
        "both routes must instantiate the same false predicate at six"
    );

    let plain = env
        .elaborate_decl_v1("theorem unrefined : Equal Int (plain six) six = Proved")
        .expect("an unrefined type-position application");
    assert_eq!(
        plain.obligations.len(),
        0,
        "ordinary type applications stay free"
    );
}

#[test]
fn anonymous_arrow_refinement_template_lifts_over_unnamed_domain() {
    let mut env = ElabEnv::new().expect("prelude");
    let function = env
        .elaborate_decl_v1("fn g (n : Int) : Bool → ({x : Int | Equal Int x n}) → Int = λb. λx. x")
        .expect("named binder and anonymous arrow in one checked signature");
    assert_eq!(function.obligations.len(), 0);
    let call = env
        .elaborate_decl_v1("const c : Int = g 5 True 6")
        .expect("anonymous-arrow call must emit rather than mis-scope");
    assert_eq!(open_refinements(&env, &call), 1);
    let equal = Term::const_(env.globals["Equal"], vec![]);
    let int = Term::const_(env.globals["Int"], vec![]);
    let goal = Term::app(
        Term::app(Term::app(equal, int), Term::IntLit(6.into())),
        Term::IntLit(5.into()),
    );
    assert_eq!(
        call.obligations[0].goal_closed, goal,
        "the exact obligation is Equal Int 6 5, not a shifted variable"
    );
}

#[test]
fn anonymous_before_and_after_named_binder_preserves_predicate_scope() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl_v1(
        "fn g (u : Bool) : Bool → (n : Int) → Bool → \
         ({x : Int | Equal Int x n}) → Int = λa. λn. λb. λx. x",
    )
    .expect("the first anonymous binder precedes the predicate's named n");
    let call = env
        .elaborate_decl_v1("const c : Int = g True True 5 False 6")
        .expect("one introduction through two anonymous arrows");
    assert_eq!(open_refinements(&env, &call), 1);
    let goal = Term::app(
        Term::app(
            Term::app(
                Term::const_(env.globals["Equal"], vec![]),
                Term::const_(env.globals["Int"], vec![]),
            ),
            Term::IntLit(6.into()),
        ),
        Term::IntLit(5.into()),
    );
    assert_eq!(
        call.obligations[0].goal_closed, goal,
        "anonymous binders on both sides of n cannot change Equal Int 6 5"
    );
}

fn type_position_box_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "const six : Int = 6",
        "def Five = { v : Int | Equal Int v 5 }",
        "fn Box (y : {v : Int | Equal Int v 5}) : Type = Int",
        "fn NBox (y : Five) : Type = Int",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
    }
    env
}

#[test]
fn function_parameter_prepasses_defer_type_position_introduction() {
    let mut env = type_position_box_env();
    for (source, owner) in [
        (
            "fn h_lit (b : Bool) (t : Box six) : Int = 0",
            "fn domain / literal",
        ),
        (
            "fn h_named (b : Bool) (t : NBox six) : Int = 0",
            "fn domain / named",
        ),
    ] {
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{owner}: {error:?}"));
        assert_eq!(
            open_refinements(&env, &result),
            1,
            "{owner}: no prepass refusal"
        );
        assert_eq!(result.obligations.len(), 1, "{owner}: no prepass emission");
    }
}

#[test]
fn anonymous_theorem_signature_domain_emits_type_position_argument_once() {
    let mut env = type_position_box_env();
    for (source, owner) in [
        (
            "theorem p_lit : Bool → (Box six) → Top = λb. λt. Proved",
            "anonymous theorem / literal",
        ),
        (
            "theorem p_named : Bool → (NBox six) → Top = λb. λt. Proved",
            "anonymous theorem / named",
        ),
    ] {
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{owner}: {error:?}"));
        assert_eq!(
            open_refinements(&env, &result),
            1,
            "{owner}: one introduction"
        );
        assert_eq!(result.obligations.len(), 1, "{owner}: no duplicate emitter");
    }
}

fn data_with_field(explicit: bool, field: &str) -> String {
    if explicit {
        format!("data D : Type where {{ Mk : (x : {field}) → D }}")
    } else {
        format!("data D = Mk ({field})")
    }
}

fn check_constructor_refusal_and_rollback(explicit: bool) {
    let mut env = type_position_box_env();
    let source = data_with_field(explicit, "Box six");
    let error = env
        .elaborate_decl_v1(&source)
        .expect_err("a constructor's field type must check Box six");
    let ElabError::ObligationWithoutChannel { span } = error else {
        panic!("{source}: expected an undischarged refinement, found {error:?}");
    };
    let start = source.find("Box six").expect("fixture has the field type");
    assert!(
        span.start <= start && span.end >= start + "Box six".len(),
        "refusal belongs to the constructor field application: {span:?}"
    );
    assert!(!env.globals.contains_key("D"), "data type must roll back");
    assert!(
        !env.globals.contains_key("Mk"),
        "constructor must roll back"
    );
    env.elaborate_decl("const five : Int = 5")
        .expect("retry value");
    let retry = data_with_field(explicit, "Box five");
    let accepted = env
        .elaborate_decl_v1(&retry)
        .unwrap_or_else(|e| panic!("rollback must leave D/Mk reusable: {e:?}"));
    assert!(
        accepted.obligations.is_empty(),
        "discharged retry needs no hole"
    );
    assert!(env.globals.contains_key("D") && env.globals.contains_key("Mk"));
}

#[test]
fn explicit_constructor_type_applications_refuse_and_rollback() {
    check_constructor_refusal_and_rollback(true);
}

#[test]
fn legacy_constructor_type_applications_refuse_and_rollback() {
    check_constructor_refusal_and_rollback(false);
}

#[test]
fn constructor_type_applications_admit_discharged_literal_formals() {
    for explicit in [false, true] {
        let mut env = type_position_box_env();
        env.elaborate_decl("const five : Int = 5")
            .expect("closed argument");
        let source = data_with_field(explicit, "Box five");
        let accepted = env
            .elaborate_decl_v1(&source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert!(
            accepted.obligations.is_empty(),
            "closed Equal Int 5 5 discharges"
        );
    }
}

#[test]
fn constructor_type_applications_check_named_formals_and_reuse_refined_arguments() {
    for explicit in [false, true] {
        let mut bad_env = type_position_box_env();
        let bad = data_with_field(explicit, "NBox six");
        assert!(
            matches!(
                bad_env.elaborate_decl_v1(&bad),
                Err(ElabError::ObligationWithoutChannel { .. })
            ),
            "{bad}: a named root cannot erase an ordinary Int argument"
        );

        let mut good_env = type_position_box_env();
        let value = good_env
            .elaborate_decl_v1("const five : Five = 5")
            .expect("the source can carry the checked refined type");
        let source = data_with_field(explicit, "NBox five");
        let accepted = good_env
            .elaborate_decl_v1(&source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert!(
            accepted.obligations.is_empty(),
            "same-root reuse must not emit a second obligation"
        );
        assert_eq!(
            value.obligations.len(),
            1,
            "any distinct introduction belongs to the earlier five declaration"
        );
    }
}

fn check_family_telescope_refusal_and_rollback(parameter: bool) {
    let mut env = type_position_box_env();
    let source = if parameter {
        "data D (t : Box six) : Type where { Mk : D t }"
    } else {
        "data D : (t : Box six) → Type where { Mk : D six }"
    };
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("a family telescope cannot silently erase Box six");
    let ElabError::ObligationWithoutChannel { span } = error else {
        panic!("{source}: expected an undischarged refinement, got {error:?}");
    };
    let start = source
        .find("Box six")
        .expect("the family carries the refined application");
    assert!(
        span.start <= start && span.end >= start + "Box six".len(),
        "refusal belongs to the family telescope entry: {span:?}"
    );
    assert!(
        !env.globals.contains_key("D") && !env.globals.contains_key("Mk"),
        "postadmission failure must roll the entire data family back"
    );
}

#[test]
fn explicit_data_parameter_checks_refined_type_application() {
    check_family_telescope_refusal_and_rollback(true);
}

#[test]
fn explicit_data_index_checks_refined_type_application() {
    check_family_telescope_refusal_and_rollback(false);
}

#[test]
fn explicit_data_telescope_discharge_accepts_closed_argument() {
    for parameter in [false, true] {
        let mut env = type_position_box_env();
        env.elaborate_decl("const five : Int = 5").expect("value");
        let source = if parameter {
            "data D (t : Box five) : Type where { Mk : D t }"
        } else {
            "data D : (t : Box five) → Type where { Mk : D five }"
        };
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert!(
            result.obligations.is_empty(),
            "a discharged family-telescope refinement needs no hole"
        );
    }
}

fn indexed_five_family_env() -> ElabEnv {
    let mut env = type_position_box_env();
    let value = env
        .elaborate_decl_v1("const five : Five = 5")
        .expect("named refined value");
    assert_eq!(value.obligations.len(), 1, "five owns its own introduction");
    let family = env
        .elaborate_decl_v1("data E : (t : Five) → Type where { MkE : E five }")
        .expect("reusing an already refined value in the family result");
    assert!(
        family.obligations.is_empty(),
        "reuse adds no family obligation"
    );
    env
}

#[test]
fn explicit_data_result_checks_named_indformer_argument() {
    let mut env = type_position_box_env();
    let error = env
        .elaborate_decl_v1("data E : (t : Five) → Type where { MkE : E six }")
        .expect_err("a constructor result must check E six against Five");
    assert!(
        matches!(error, ElabError::ObligationWithoutChannel { .. }),
        "expected no-channel refusal at the constructor result: {error:?}"
    );
    assert!(
        !env.globals.contains_key("E") && !env.globals.contains_key("MkE"),
        "result-position refusal rolls back the family and its constructor"
    );
    let accepted = indexed_five_family_env();
    assert!(accepted.globals.contains_key("E") && accepted.globals.contains_key("MkE"));
}

#[test]
fn function_domain_checks_named_indformer_argument() {
    let mut env = indexed_five_family_env();
    let result = env
        .elaborate_decl_v1("fn k (x : E six) : Int = 0")
        .expect("a written type on a reported route emits an obligation");
    assert_eq!(open_refinements(&env, &result), 1);
    assert_eq!(
        result.obligations.len(),
        1,
        "the same E six is introduced once"
    );
    let goal = Term::app(
        Term::app(
            Term::app(
                Term::const_(env.globals["Equal"], vec![]),
                Term::const_(env.globals["Int"], vec![]),
            ),
            Term::const_(env.globals["six"], vec![]),
        ),
        Term::IntLit(5.into()),
    );
    assert_eq!(
        result.obligations[0].goal_closed, goal,
        "the goal is Equal Int six 5, with six declared as 6"
    );
    assert_eq!(
        ken_kernel::whnf(
            &env.env,
            &ken_kernel::Context::new(),
            &Term::const_(env.globals["six"], vec![])
        ),
        Term::IntLit(6.into()),
        "the checked six used by that exact goal reduces to 6"
    );
}

fn assert_closed_local_head_goal(env: &ElabEnv, result: &ElabResult, head_type: Term) {
    assert_eq!(
        open_refinements(env, result),
        1,
        "one open local-head introduction"
    );
    assert_eq!(
        result.obligations.len(),
        1,
        "no duplicate local-head emission"
    );
    let int = Term::const_(env.globals["Int"], vec![]);
    let equal = Term::const_(env.globals["Equal"], vec![]);
    let predicate = Term::app(
        Term::app(Term::app(equal, int.clone()), Term::var(0)),
        Term::IntLit(5.into()),
    );
    let expected = Term::pi(head_type, Term::pi(int, predicate));
    assert_eq!(
        result.obligations[0].goal_closed, expected,
        "closed goal must quantify the head, then n, and state Equal Int n 5"
    );
}

#[test]
fn local_head_named_domain_emits_and_plain_domain_does_not() {
    let mut refined = ElabEnv::new().expect("prelude");
    refined
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let result = refined
        .elaborate_decl_v1("fn h (f : Five → Type 0) (n : Int) (x : f n) : Int = 0")
        .expect("the aligned local head type has a named refined domain");
    let head = Term::pi(
        Term::const_(refined.globals["Five"], vec![]),
        Term::ty(ken_kernel::Level::Zero),
    );
    assert_closed_local_head_goal(&refined, &result, head);

    let mut plain = ElabEnv::new().expect("prelude");
    plain
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let result = plain
        .elaborate_decl_v1("fn h (f : Int → Type 0) (n : Int) (x : f n) : Int = 0")
        .expect("unrefined local head");
    assert!(
        result.obligations.is_empty(),
        "the plain control stays obligation-free"
    );
}

#[test]
fn local_head_previous_argument_preserves_refinement_position() {
    let mut refined = ElabEnv::new().expect("prelude");
    refined
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    refined
        .elaborate_decl("const zero : Int = 0")
        .expect("type-position value");
    let result = refined
        .elaborate_decl_v1("fn h2 (g : Int → Five → Type 0) (n : Int) (x : g zero n) : Int = 0")
        .expect("the earlier argument must not hide the refined second domain");
    let int = Term::const_(refined.globals["Int"], vec![]);
    let five = Term::const_(refined.globals["Five"], vec![]);
    assert_closed_local_head_goal(
        &refined,
        &result,
        Term::pi(int, Term::pi(five, Term::ty(ken_kernel::Level::Zero))),
    );

    let mut plain = ElabEnv::new().expect("prelude");
    plain
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    plain
        .elaborate_decl("const zero : Int = 0")
        .expect("type-position value");
    let result = plain
        .elaborate_decl_v1("fn h2 (g : Int → Int → Type 0) (n : Int) (x : g zero n) : Int = 0")
        .expect("plain two-argument local head");
    assert!(
        result.obligations.is_empty(),
        "plain second domain stays free"
    );
}

#[test]
fn local_head_named_domain_reuses_refined_argument_root() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let result = env
        .elaborate_decl_v1("fn h3 (f : Five → Type 0) (m : Five) (x : f m) : Int = 0")
        .expect("the local m already carries Five's refinement root");
    assert!(
        result.obligations.is_empty(),
        "same-root reuse needs no new hole"
    );
}

#[test]
fn local_head_data_field_refuses_false_and_accepts_plain() {
    let mut refined = ElabEnv::new().expect("prelude");
    refined
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let error = refined
        .elaborate_decl_v1(
            "data D (f : Five → Type 0) (n : Int) : Type where { Mk : (x : f n) → D f n }",
        )
        .expect_err("a refined application inside a constructor field has no channel");
    assert!(
        matches!(error, ElabError::ObligationWithoutChannel { .. }),
        "expected the false predicate's no-channel refusal: {error:?}"
    );
    assert!(
        !refined.globals.contains_key("D") && !refined.globals.contains_key("Mk"),
        "failed postadmission check rolls back the data family"
    );

    let mut plain = ElabEnv::new().expect("prelude");
    plain
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let result = plain
        .elaborate_decl_v1(
            "data D (f : Int → Type 0) (n : Int) : Type where { Mk : (x : f n) → D f n }",
        )
        .expect("ordinary local-head field type");
    assert!(
        result.obligations.is_empty(),
        "plain data declaration stays free"
    );
}

#[test]
fn unannotated_higher_order_head_is_fail_closed_without_changing_plain_type() {
    // F-F: a separate, baseline universe-level hole-closure gap. This test
    // pins fail-closed handling, not a claimed successful introduction.
    let mut refined = ElabEnv::new().expect("prelude");
    refined
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let error = refined
        .elaborate_decl_v1("fn h (f : Five → Type) (n : Int) (x : f n) : Int = 0")
        .expect_err("an unzonked universe in an obligation must not erase it");
    assert!(
        matches!(error, ElabError::KernelRejected { .. }),
        "F-F remains a separate fail-closed kernel rejection: {error:?}"
    );

    let mut plain = ElabEnv::new().expect("prelude");
    plain
        .elaborate_decl("def Five = {v : Int | Equal Int v 5}")
        .expect("alias");
    let result = plain
        .elaborate_decl_v1("fn h (f : Int → Type) (n : Int) (x : f n) : Int = 0")
        .expect("unannotated but unrefined type remains allowed");
    assert!(result.obligations.is_empty());
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
    let recursion = env
        .elaborate_decl_v1(
            "fn rec (n : Nat) (p : { x : Int | Equal Int x 5 }) : Int = \
         match n { Zero |-> p ; Suc m |-> rec m p }",
        )
        .expect("recursive view with a literal-refined parameter");
    assert!(
        recursion
            .obligations
            .iter()
            .all(|obligation| matches!(obligation.kind, ObligationKind::RefinementIntroduction)),
        "an in-group self-call can emit only refinement introductions here"
    );
    let result = env
        .elaborate_decl_v1("const result : Int = rec Zero 6")
        .expect("recursive call with incorrect literal argument");
    assert_eq!(
        open_refinements(&env, &result),
        1,
        "the recursive-view declaration must store its parameter template"
    );
}

#[test]
fn refined_result_contract_preserves_its_parameter_template() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl_v1(
        "fn with_result (p : { x : Int | Equal Int x 5 }) : \
         { y : Int | Equal Int y p } = p",
    )
    .expect("the result refinement is realized by the contract path");
    let call = env
        .elaborate_decl_v1("const result : Int = with_result 6")
        .expect("literal parameter persists through the contract route");
    assert_eq!(open_refinements(&env, &call), 1);
}

#[test]
fn type_position_requires_callee_is_not_silently_accepted() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("const five : Int = 5").expect("value");
    env.elaborate_decl("fn need (x : Int) : Int requires Equal Int x 5 = x")
        .expect("requires callee");
    let error = env
        .elaborate_decl_v1("theorem no_missing_requires : Equal Int (need five) five = Proved")
        .expect_err("a missing requires argument must fail closed");
    assert!(
        matches!(error, ElabError::KernelRejected { .. }),
        "{error:?}"
    );
}
