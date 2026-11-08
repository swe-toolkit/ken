//! Spec 36 §2.5, §4.3: a tree-result proc's `ensures` is a separate `AllRet`
//! obligation on its unrefined tree. Promise class: durable invariant.
//! MEASURED: each post mints one Ensures obligation; the true post's goal
//! normalizes to a reflexive equation the kernel proves by `Refl`, through a
//! `Ret` leaf and through a `Vis` response; the opposite post's goal is
//! refused; the proc's own type stays ITree-headed and a caller binds `Int`.
//! CLAIMED: no payload is refined. THE GAP: discharge by the prover routes is
//! not measured here; `ken check` reports these as open obligations.

use ken_elaborator::{ElabEnv, ElabResult, ObligationKind};
use ken_kernel::{check, convert, normalize, whnf, Context, Term};

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

/// `λ binders. Refl lhs` for a goal whose normal form is `Π binders. Eq T lhs rhs`.
fn refl_attempt(env: &ElabEnv, goal: &Term) -> Result<(), String> {
    let mut ctx = Context::new();
    let mut binders = Vec::new();
    let mut body = normalize(&env.env, &ctx, goal);
    while let Term::Pi(domain, codomain) = body {
        ctx.push(domain.as_ref().clone());
        binders.push(domain.as_ref().clone());
        body = *codomain;
    }
    let Term::Eq(ty, lhs, rhs) = &body else {
        return Err(format!("goal leaf is not an equation: {body:?}"));
    };
    if !convert(&env.env, &ctx, ty, lhs, rhs) {
        return Err("leaf equation is not reflexive".into());
    }
    let proof = binders
        .into_iter()
        .rev()
        .fold(Term::Refl(lhs.clone()), |proof, domain| Term::lam(domain, proof));
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
    refl_attempt(&env, &ensures_goal(&results, "one")).expect("AllRet (λ r. r = 1) (Ret 1)");
    let itree = env.globals["ITree"];
    let (_, ty) = env.env.const_type(result(&results, "one").def_id).expect("checked one");
    let mut spine = whnf(&env.env, &Context::new(), &ty);
    while let Term::App(function, _) = spine {
        spine = *function;
    }
    assert!(
        matches!(spine, Term::IndFormer { id, .. } if id == itree),
        "the payload is not refined to a Σ: {spine:?}"
    );
    let (control, results) = elaborate(&changed(2));
    assert!(refl_attempt(&control, &ensures_goal(&results, "one")).is_err());
}

#[test]
fn declared_tree_post_is_all_ret_through_each_response() {
    let (env, results) = elaborate(&declared("False"));
    refl_attempt(&env, &ensures_goal(&results, "tick")).expect("every leaf returns False");
    let (control, results) = elaborate(&declared("True"));
    assert!(refl_attempt(&control, &ensures_goal(&results, "tick")).is_err());
}
