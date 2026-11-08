//! Spec 36 §2.5, §4.3: a tree-result proc's `ensures` is a separate `AllRet`
//! obligation on its unrefined tree. Promise class: durable invariant.
//! MEASURED: each post mints one Ensures obligation; the kernel's observational
//! `Eq` decides the post at each returned literal or constructor, so a true
//! post's goal normalizes to `Top` and the kernel checks `tt` against the
//! unnormalized goal, through a `Ret` leaf and, under one response binder,
//! through a `Vis`; the opposite post normalizes to `Bottom` and `tt` is
//! refused; the proc's own type stays ITree-headed and a caller binds `Int`.
//! CLAIMED: no payload is refined. THE GAP: discharge by the prover routes is
//! not measured here; `ken check` reports these as open obligations.

use ken_elaborator::{ElabEnv, ElabResult, ObligationKind};
use ken_kernel::{check, normalize, whnf, Context, GlobalId, Term};

fn elaborate(source: &str) -> (ElabEnv, Vec<ElabResult>) {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(source)
        .expect("tree-result ensures fixture");
    (env, results)
}

fn result<'a>(results: &'a [ElabResult], name: &str) -> &'a ElabResult {
    results
        .iter()
        .find(|result| result.name == name)
        .unwrap_or_else(|| panic!("missing result {name}"))
}

fn ensures_goal(results: &[ElabResult], name: &str) -> Term {
    let goals: Vec<_> = result(results, name)
        .obligations
        .iter()
        .filter(|obligation| matches!(obligation.kind, ObligationKind::Ensures))
        .map(|obligation| obligation.goal_closed.clone())
        .collect();
    assert_eq!(goals.len(), 1, "{name}: one AllRet obligation");
    goals[0].clone()
}

fn constant(id: GlobalId) -> Term {
    Term::Const {
        id,
        level_args: Vec::new(),
    }
}

/// The goal's normal form split into its `Π` binder domains and its leaf.
fn normal_form(env: &ElabEnv, goal: &Term) -> (Vec<Term>, Term) {
    let mut binders = Vec::new();
    let mut body = normalize(&env.env, &Context::new(), goal);
    while let Term::Pi(domain, codomain) = body {
        binders.push(*domain);
        body = *codomain;
    }
    (binders, body)
}

/// Asserts the goal's normal form is `Π^binders. leaf`, then has the kernel
/// check `λ binders. tt` against the original, unnormalized goal.
fn tt_attempt(env: &ElabEnv, goal: &Term, binders: usize, leaf: GlobalId) -> Result<(), String> {
    let (domains, body) = normal_form(env, goal);
    assert_eq!(domains.len(), binders, "response binders in {body:?}");
    assert_eq!(body, constant(leaf), "the decided post");
    let proof = domains
        .into_iter()
        .rev()
        .fold(constant(env.env.tt_id()), |proof, domain| {
            Term::lam(domain, proof)
        });
    check(&env.env, &Context::new(), &proof, goal).map_err(|error| format!("{error:?}"))
}

fn changed(post: i64) -> String {
    format!(
        "proc one : Int ensures Equal Int result {post} visits [Console] = 1\n\
         proc caller : Int visits [Console] =\n  let n : Int = one in n\n"
    )
}

fn declared(post: &str) -> String {
    format!(
        "proc tick : IO Bool ensures Equal Bool result {post} visits [Console] =\n  \
         let w = print_line \"tick\" in ITree.Ret ConsoleOp console_resp Bool False\n"
    )
}

#[test]
fn changed_proc_post_is_all_ret_on_its_ret_leaf() {
    let (env, results) = elaborate(&changed(1));
    let goal = ensures_goal(&results, "one");
    tt_attempt(&env, &goal, 0, env.env.top_id()).expect("AllRet (λ r. r = 1) (Ret 1)");
    let itree = env.globals["ITree"];
    let (_, ty) = env
        .env
        .const_type(result(&results, "one").def_id)
        .expect("checked one");
    let mut spine = whnf(&env.env, &Context::new(), &ty);
    while let Term::App(function, _) = spine {
        spine = *function;
    }
    assert!(
        matches!(spine, Term::IndFormer { id, .. } if id == itree),
        "the payload is not refined to a Σ: {spine:?}"
    );
    let (control, results) = elaborate(&changed(2));
    let goal = ensures_goal(&results, "one");
    assert!(tt_attempt(&control, &goal, 0, control.env.bottom_id()).is_err());
}

#[test]
fn declared_tree_post_is_all_ret_through_each_response() {
    let (env, results) = elaborate(&declared("False"));
    let goal = ensures_goal(&results, "tick");
    tt_attempt(&env, &goal, 1, env.env.top_id()).expect("every leaf returns False");
    let (control, results) = elaborate(&declared("True"));
    let goal = ensures_goal(&results, "tick");
    assert!(tt_attempt(&control, &goal, 1, control.env.bottom_id()).is_err());
}
