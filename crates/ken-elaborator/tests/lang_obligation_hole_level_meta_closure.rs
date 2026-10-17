//! Durable admission and goal-contract pins for spec 12 §4, 39 §5.7, 22 §2.1
//! and Architect D0 evt_7vz7ep5vbbaqg. Hole levels are parameters, not early
//! defaults; the final declaration alone chooses explicit level arguments.

use ken_elaborator::{ElabEnv, ElabResult, ObligationKind};
use ken_kernel::{Decl, Level, LevelVar, Term};

const USE5: &str = "fn use5 (x : {v : Int | Equal Int v 5}) : Int = x";
const NEED: &str = "fn need (x : Int) : Int requires Equal Int x 5 = x";

fn env_with(source: &str) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(source).expect("producer");
    env
}

fn one_open<'a>(env: &ElabEnv, result: &'a ElabResult, kind: ObligationKind) -> &'a Term {
    assert_eq!(result.obligations.len(), 1, "one reported obligation");
    let obligation = &result.obligations[0];
    assert!(
        matches!(
            (&obligation.kind, kind),
            (ObligationKind::Requires, ObligationKind::Requires)
                | (ObligationKind::PartialPrim, ObligationKind::PartialPrim)
                | (
                    ObligationKind::RefinementIntroduction,
                    ObligationKind::RefinementIntroduction
                )
        ),
        "wrong obligation kind: {:?}",
        obligation.kind
    );
    assert!(
        env.is_open_hole(obligation.hole_id),
        "the wrong proof was discharged"
    );
    let Some(Decl::Opaque { ty, .. }) = env.env.lookup(obligation.hole_id) else {
        panic!("the reported hole is a kernel postulate")
    };
    assert_eq!(ty, &obligation.goal_closed, "report matches kernel hole");
    &obligation.goal_closed
}

fn hole_params(env: &ElabEnv, result: &ElabResult) -> Vec<LevelVar> {
    env.env
        .lookup(result.obligations[0].hole_id)
        .expect("hole declaration")
        .level_params()
        .to_vec()
}

fn equal_n_5(env: &ElabEnv) -> Term {
    Term::app(
        Term::app(
            Term::app(
                Term::const_(env.globals["Equal"], vec![]),
                Term::const_(env.globals["Int"], vec![]),
            ),
            Term::var(0),
        ),
        Term::IntLit(5.into()),
    )
}

fn uses_hole_at_levels(term: &Term, hole_id: ken_kernel::GlobalId, levels: &[Level]) -> bool {
    matches!(term, Term::Const { id, level_args } if *id == hole_id && level_args == levels)
        || term
            .children()
            .into_iter()
            .any(|child| uses_hole_at_levels(child, hole_id, levels))
}

/// MEASURED: the formerly rejected `use5 n` checks and its one hole is Π over
/// the unannotated Type binder, then Int, then exactly Equal Int n 5. CLAIMED:
/// levels in that context are declared parameters, not undeclared vars. THE
/// GAP: a later nonzero level solve must flow into a use, covered below.
#[test]
fn unsolved_binder_meta_closes_one_refinement_hole_without_changing_the_twin() {
    let mut env = env_with(USE5);
    let result = env
        .elaborate_decl_v1("fn k (A : Type) (n : Int) : Int = use5 n")
        .expect("a hole over a fresh level meta checks");
    let goal = one_open(&env, &result, ObligationKind::RefinementIntroduction);
    let params = hole_params(&env, &result);
    assert_eq!(params.len(), 1);
    let int = Term::const_(env.globals["Int"], vec![]);
    assert_eq!(
        *goal,
        Term::pi(
            Term::ty(Level::Var(params[0])),
            Term::pi(int, equal_n_5(&env))
        )
    );
    // Refinement introductions are free-standing obligations, not proof terms
    // inserted in this function body. Requires below exercises a used hole.
    let mut twin_env = env_with(USE5);
    let twin = twin_env
        .elaborate_decl_v1("fn k (n : Int) : Int = use5 n")
        .expect("twin");
    let twin_goal = one_open(&twin_env, &twin, ObligationKind::RefinementIntroduction);
    assert_eq!(hole_params(&twin_env, &twin), vec![]);
    assert_eq!(
        *twin_goal,
        Term::pi(
            Term::const_(twin_env.globals["Int"], vec![]),
            equal_n_5(&twin_env)
        )
    );
}

/// MEASURED: Requires is a used hole whose unsolved level is later solved to
/// suc 0; a free-standing refinement hole is polymorphic at the same level.
/// CLAIMED: no early default can freeze either hole to Zero. THE GAP: an
/// unsolved level at an unused hole can still be defaulted safely only by the
/// final declaration zonk; the free-standing row checks that separately.
#[test]
fn solved_after_requires_uses_nonzero_level_and_refinement_remains_polymorphic() {
    let mut env = env_with(NEED);
    let result = env
        .elaborate_decl_v1("fn k (a : Type) (n : Int) : Type 1 = let q : Int = need n in a")
        .expect("a later solve must reach the used Requires hole");
    let goal = one_open(&env, &result, ObligationKind::Requires);
    let params = hole_params(&env, &result);
    assert_eq!(params.len(), 1);
    assert!(matches!(goal, Term::Pi(a, _) if **a == Term::ty(Level::Var(params[0]))));
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked body")
    };
    assert!(
        uses_hole_at_levels(
            body,
            result.obligations[0].hole_id,
            &[Level::Suc(Box::new(Level::Zero))]
        ),
        "the final zonk must solve a used hole's argument to suc 0"
    );

    let mut free = env_with(USE5);
    let result = free
        .elaborate_decl_v1("fn k (a : Type) (n : Int) : Type 1 = let q : Int = use5 n in a")
        .expect("free-standing hole is well-typed at the later nonzero level");
    one_open(&free, &result, ObligationKind::RefinementIntroduction);
    assert_eq!(hole_params(&free, &result).len(), 1);
}

/// MEASURED: each partial primitive emits one hole under a bare Type binder,
/// including the distinct no-overflow and nonzero mint sites. CLAIMED: both
/// close over that binder's unsolved level rather than leaking it to the
/// kernel. THE GAP: this checks hole well-formedness and the recorded goal;
/// it does not assert the numeric primitive's computational behavior.
#[test]
fn partial_primitive_holes_close_over_a_type_binder_at_both_mints() {
    for (expr, result_type, parameter_type) in
        [("n + d", "UInt8", "UInt8"), ("n / d", "Int", "Int")]
    {
        let mut env = ElabEnv::new().expect("prelude");
        let source = format!(
            "fn k (A : Type) (n : {parameter_type}) (d : {parameter_type}) : {result_type} = {expr}"
        );
        let result = env
            .elaborate_decl_v1(&source)
            .expect("partial operation with a Type binder");
        let goal = one_open(&env, &result, ObligationKind::PartialPrim);
        let params = hole_params(&env, &result);
        assert_eq!(params.len(), 1, "one open level for {expr}");
        assert!(matches!(goal, Term::Pi(a, _) if **a == Term::ty(Level::Var(params[0]))));
    }
}

/// MEASURED: a meta solved before mint is substituted into the recorded goal,
/// while two unsolved metas become two distinct hole level parameters. CLAIMED:
/// params are origin-keyed, deduplicated and in id order. THE GAP: syntactic
/// repetition is not independently checked here; the same MetaCtx id appears
/// in the goal and in its use-site argument on the Requires row above.
#[test]
fn solved_before_mint_substitutes_and_two_metas_remain_distinct() {
    let mut solved = env_with(USE5);
    let result = solved
        .elaborate_decl_v1(
            "fn k (a : Type) (n : Int) : Type 1 = let b : Type 1 = a in let q : Int = use5 n in b",
        )
        .expect("meta solved before mint");
    let goal = one_open(&solved, &result, ObligationKind::RefinementIntroduction);
    assert_eq!(hole_params(&solved, &result), vec![]);
    assert!(matches!(goal, Term::Pi(a, _) if **a == Term::ty(Level::Suc(Box::new(Level::Zero)))));

    let mut pair = env_with(USE5);
    let result = pair
        .elaborate_decl_v1("fn k (A : Type) (B : Type) (n : Int) : Int = use5 n")
        .expect("two distinct levels close independently");
    let goal = one_open(&pair, &result, ObligationKind::RefinementIntroduction);
    let params = hole_params(&pair, &result);
    assert_eq!(params.len(), 2);
    assert_ne!(params[0], params[1]);
    assert_eq!(
        *goal,
        Term::pi(
            Term::ty(Level::Var(params[0])),
            Term::pi(
                Term::ty(Level::Var(params[1])),
                Term::pi(Term::const_(pair.globals["Int"], vec![]), equal_n_5(&pair))
            )
        )
    );

    let mut used = env_with(NEED);
    let result = used
        .elaborate_decl_v1("fn k (A : Type) (B : Type) (n : Int) : Int = need n")
        .expect("two level metas applied at one used Requires hole");
    one_open(&used, &result, ObligationKind::Requires);
    assert_eq!(hole_params(&used, &result).len(), 2);
    let Some(Decl::Transparent { body, .. }) = used.env.lookup(result.def_id) else {
        panic!("checked two-meta Requires body")
    };
    assert!(
        uses_hole_at_levels(
            body,
            result.obligations[0].hole_id,
            &[Level::Zero, Level::Zero]
        ),
        "both hole arguments are explicitly defaulted at the final zonk"
    );
}

/// MEASURED: a certificate for `use5 5` discharges even after the binder's
/// level is solved to suc 0, matching the explicitly annotated Type 1 twin.
/// CLAIMED: candidate and goal receive the same solved-meta substitution
/// before checking. THE GAP: the production-side deletion mutation must
/// distinguish this row from the twin (one opens, one stays discharged).
#[test]
fn solved_level_certificate_discharge_matches_annotated_twin() {
    let states: Vec<_> = ["a : Type", "a : Type 1"]
        .into_iter()
        .map(|binder| {
            let mut env = env_with(USE5);
            let source = format!(
                "fn k ({binder}) (n : Int) : Type 1 = let b : Type 1 = a in let q : Int = use5 5 in b"
            );
            let result = env
                .elaborate_decl_v1(&source)
                .expect("certificate at solved level");
            assert_eq!(result.obligations.len(), 1, "one matching hole");
            (binder, env.is_open_hole(result.obligations[0].hole_id))
        })
        .collect();
    assert_eq!(states, [("a : Type", false), ("a : Type 1", false)]);
}

/// MEASURED: a space-operation ensures under a bare `Type` parameter is
/// level-closed by the operation's already-zonked signature. CLAIMED: the
/// separate space-ensures mint preserves its one reported postulate and does
/// not invent a level parameter. THE GAP: this input does not produce a live
/// meta at that mint; only the general closure and its identity case run.
#[test]
fn space_ensures_under_unannotated_type_uses_zonked_operation_domain() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(
            "space LevelSpace { mut cell : Int = 0 \
         proc inspect (A : Type) : Int ensures Equal Int result 0 visits [LevelSpace] = 1 }",
        )
        .expect("space operation with a bare Type parameter");
    let result = results
        .iter()
        .find(|r| r.name == "LevelSpace.inspect")
        .expect("operation");
    assert_eq!(result.obligations.len(), 1);
    let hole = &result.obligations[0];
    assert!(matches!(hole.kind, ObligationKind::Ensures));
    assert!(env.is_open_hole(hole.hole_id));
    assert_eq!(hole_params(&env, result), vec![]);
    let Some(Decl::Opaque { ty, .. }) = env.env.lookup(hole.hole_id) else {
        panic!("space ensures registered a checked postulate")
    };
    assert_eq!(*ty, hole.goal_closed);
}
