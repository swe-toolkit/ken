use ken_elaborator::ElabEnv;
use ken_kernel::{Level, Term};

fn arbitrary_predicate(env: &mut ElabEnv) {
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int, Term::omega(Level::Zero)))
        .expect("predicate postulate for this fixture");
}

#[test]
fn named_introduction_emits_only_on_new_values() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("def Pos = { x : Int | P x }").expect("alias");
    let introduction = env.elaborate_decl_v1("const five : Pos = 5").expect("intro");
    assert_eq!(introduction.obligations.len(), 1);
    assert!(env.is_open_hole(introduction.obligations[0].hole_id));
    let same = env.elaborate_decl_v1("fn same (p : Pos) : Pos = p").expect("reuse");
    assert!(same.obligations.is_empty(), "a value retaining the same alias is not new");
    let forget = env.elaborate_decl_v1("fn forget (p : Pos) : Int = p").expect("forget");
    assert!(forget.obligations.is_empty(), "forgetting to the carrier is free");
    let open = env.elaborate_decl_v1("fn to_pos (n : Int) : Pos = n").expect("open intro");
    assert_eq!(open.obligations.len(), 1);
    assert!(env.is_open_hole(open.obligations[0].hole_id));
}

#[test]
fn char_scalar_boundary_is_an_obligation_not_a_rejection() {
    let mut env = ElabEnv::new().expect("prelude");
    let valid = env.elaborate_decl_v1("const valid : Char = 55295").expect("valid scalar");
    assert_eq!(valid.obligations.len(), 1);
    assert!(!env.is_open_hole(valid.obligations[0].hole_id), "valid goal: {:?}; whnf: {:?}", valid.obligations[0].goal_closed, ken_kernel::whnf(&env.env, &ken_kernel::Context::new(), &valid.obligations[0].goal_closed));
    let invalid = env.elaborate_decl_v1("const invalid : Char = 55296").expect("open invalid scalar");
    assert_eq!(invalid.obligations.len(), 1);
    assert!(env.is_open_hole(invalid.obligations[0].hole_id));
}

#[test]
fn matched_scalar_introduction_carries_and_uses_its_branch_equation() {
    let mut env = ElabEnv::new().expect("prelude");
    let branch = env.elaborate_decl_v1(
        "fn checked_scalar (n : Int) : Option Char = match (inRangeBool n) { True |-> Some Char n ; False |-> None Char }"
    ).expect("checked branch");
    assert_eq!(branch.obligations.len(), 1, "the Some branch introduces Char once");
    let goal = &branch.obligations[0].goal_closed;
    let Term::Pi(_, under_n) = goal else { panic!("missing n binder: {goal:?}") };
    let Term::Pi(equation, _) = under_n.as_ref() else { panic!("missing case equation: {goal:?}") };
    assert!(matches!(equation.as_ref(), Term::Eq(..)), "the branch binder must be equality: {goal:?}");
    assert!(!env.is_open_hole(branch.obligations[0].hole_id), "branch equation must close the scalar goal: {goal:?}");
}

#[test]
fn literal_annotations_are_site_local_and_parameter_predicates_reach_calls() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    let literal = env.elaborate_decl_v1("const literal : Int = (4 : { x : Int | P x })").expect("literal");
    assert_eq!(literal.obligations.len(), 1);
    let carrier = env.elaborate_decl_v1("const carrier : Int = 4").expect("carrier");
    assert!(carrier.obligations.is_empty(), "a literal must not taint its carrier globally");
    let parameter = env.elaborate_decl_v1("fn take (x : { y : Int | P y }) : Int = x").expect("callee");
    assert!(parameter.obligations.is_empty(), "parameter predicate is an assumption, not a callee burden");
    let caller = env.elaborate_decl_v1("const caller : Int = take 4").expect("caller");
    assert_eq!(caller.obligations.len(), 1);
    assert!(env.is_open_hole(caller.obligations[0].hole_id));
}
