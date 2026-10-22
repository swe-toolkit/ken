//! Obligation closure carries the path of the call or partial primitive.
//! Promise class: durable invariant (spec/20-verification/22 §3).

use ken_elaborator::{ElabEnv, ElabResult, Obligation, ObligationKind};
use ken_kernel::{check::check as kernel_check, subst::weaken, Context, Decl, GlobalId, Term};
use std::collections::BTreeSet;

fn result<'a>(results: &'a [ElabResult], name: &str) -> &'a ElabResult {
    results
        .iter()
        .find(|result| result.name == name)
        .expect(name)
}

fn single<'a>(env: &ElabEnv, result: &'a ElabResult, kind: ObligationKind) -> &'a Obligation {
    let [obligation] = result.obligations.as_slice() else {
        panic!(
            "expected exactly one reported obligation for {}",
            result.name
        );
    };
    assert!(std::mem::discriminant(&obligation.kind) == std::mem::discriminant(&kind));
    assert!(
        matches!(env.env.lookup(obligation.hole_id), Some(Decl::Opaque { ty, .. }) if *ty == obligation.goal_closed)
    );
    obligation
}

fn assert_reported_trust_delta(env: &ElabEnv, before: &BTreeSet<GlobalId>, results: &[ElabResult]) {
    let delta = env
        .env
        .trusted_base()
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect::<BTreeSet<_>>();
    let reported = results
        .iter()
        .flat_map(|result| result.obligations.iter().map(|o| o.hole_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        delta, reported,
        "no path retry may leak an unreported trusted hole"
    );
}

fn eq_bool(env: &ElabEnv, b: usize, ctor: &str) -> Term {
    Term::Eq(
        Box::new(Term::indformer(env.globals["Bool"], vec![])),
        Box::new(Term::var(b)),
        Box::new(Term::constructor(env.globals[ctor], vec![])),
    )
}

fn written_equal(env: &ElabEnv, carrier: Term, left: Term, right: Term) -> Term {
    Term::app(
        Term::app(
            Term::app(Term::const_(env.globals["Equal"], vec![]), carrier),
            left,
        ),
        right,
    )
}

fn peel_pis(mut goal: &Term) -> (Vec<&Term>, &Term) {
    let mut domains = vec![];
    while let Term::Pi(domain, codomain) = goal {
        domains.push(domain.as_ref());
        goal = codomain;
    }
    (domains, goal)
}

/// MEASURED: the inferred local's Eq binder follows Γ in the one NoOvf hole.
/// CLAIMED: partial addition carries its current path without inventing core
/// proof arguments. THE GAP: a path-free control and an if-arm sibling separately
/// check that the added domain belongs to the actual split.
#[test]
fn fixed_width_addition_closes_let_equation_and_if_branch() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let source = "fn plain (c : Bool) (a : UInt8) (b : UInt8) : UInt8 = a + b\n\
                  fn local (a : UInt8) (b : UInt8) : UInt8 = let x : UInt8 = a in x + b\n\
                  fn branch (c : Bool) (a : UInt8) (b : UInt8) : UInt8 = if c then a + b else a";
    let results = env
        .elaborate_file_v1(source)
        .expect("fixed-width additions elaborate");
    let plain = single(&env, result(&results, "plain"), ObligationKind::PartialPrim);
    let local = single(&env, result(&results, "local"), ObligationKind::PartialPrim);
    let branch = single(
        &env,
        result(&results, "branch"),
        ObligationKind::PartialPrim,
    );
    let (plain_pis, plain_body) = peel_pis(&plain.goal_closed);
    let byte = Term::const_(env.globals["UInt8"], vec![]);
    let bool_ty = Term::indformer(env.globals["Bool"], vec![]);
    let novf = Term::const_(env.globals["NoOvfAddUInt8"], vec![]);
    assert_eq!(plain_pis, vec![&bool_ty, &byte, &byte]);
    assert_eq!(
        plain_body,
        &Term::app(Term::app(novf.clone(), Term::var(1)), Term::var(0))
    );
    let (local_pis, local_body) = peel_pis(&local.goal_closed);
    assert_eq!(
        local_pis,
        vec![
            &byte,
            &byte,
            &byte,
            &Term::Eq(
                Box::new(byte.clone()),
                Box::new(Term::var(0)),
                Box::new(Term::var(2)),
            )
        ]
    );
    assert_eq!(
        local_body,
        &Term::app(Term::app(novf, Term::var(1)), Term::var(2))
    );
    let (branch_pis, branch_body) = peel_pis(&branch.goal_closed);
    assert_eq!(branch_pis.len(), 4);
    assert_eq!(branch_pis[3], &eq_bool(&env, 2, "True"));
    assert_eq!(branch_body, &weaken(plain_body, 1));
    assert_reported_trust_delta(&env, &before, &results);
}

/// MEASURED: True and False arms gain their distinct Eq Bool path domains.
/// CLAIMED: the Int nonzero obligation is attached at its operation site.
/// THE GAP: the path-free `/` control proves there is no unconditional domain.
#[test]
fn division_closes_each_if_arm_without_changing_path_free_goal() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let results = env
        .elaborate_file_v1(
            "fn plain (c : Bool) (n : Int) (d : Int) : Int = n / d\n\
         fn yes (c : Bool) (n : Int) (d : Int) : Int = if c then n / d else n\n\
         fn no (c : Bool) (n : Int) (d : Int) : Int = if c then n else n / d\n\
         fn remainder (c : Bool) (n : Int) (d : Int) : Int = if c then n % d else n\n\
         fn by_match (c : Bool) (n : Int) (d : Int) : Int = match c { True |-> n / d ; False |-> n }",
        )
        .expect("partial divisions elaborate");
    let plain = single(&env, result(&results, "plain"), ObligationKind::PartialPrim);
    let (plain_pis, plain_body) = peel_pis(&plain.goal_closed);
    assert_eq!(plain_pis.len(), 3);
    for (name, constructor) in [
        ("yes", "True"),
        ("no", "False"),
        ("remainder", "True"),
        ("by_match", "True"),
    ] {
        let obligation = single(&env, result(&results, name), ObligationKind::PartialPrim);
        let (pis, body) = peel_pis(&obligation.goal_closed);
        assert_eq!(pis.len(), 4, "{name}: Γ and exactly one path binder");
        assert_eq!(pis[3], &eq_bool(&env, 2, constructor));
        assert_eq!(body, &weaken(plain_body, 1));
    }
    assert_reported_trust_delta(&env, &before, &results);
}

/// An impossible True arm of `if False` carries Eq Bool False True. That
/// equation reduces to Bottom, so this is a checked proof of the partial
/// operation's otherwise-opaque proposition, not a test-only postulate. A
/// certificate missing the equation would have no final `Var(0)` witness.
fn check_equation_dependent_certificate(env: &ElabEnv, obligation: &Obligation) {
    let (domains, proposition) = peel_pis(&obligation.goal_closed);
    let equation = Term::Eq(
        Box::new(Term::indformer(env.globals["Bool"], vec![])),
        Box::new(Term::constructor(env.globals["False"], vec![])),
        Box::new(Term::constructor(env.globals["True"], vec![])),
    );
    assert_eq!(domains.last().copied(), Some(&equation));
    let mut certificate = Term::Absurd(Box::new(proposition.clone()), Box::new(Term::var(0)));
    for domain in domains.iter().rev() {
        certificate = Term::lam((*domain).clone(), certificate);
    }
    kernel_check(
        &env.env,
        &Context::new(),
        &certificate,
        &obligation.goal_closed,
    )
    .expect("kernel checks a certificate needing the closed path equation");
}

/// MEASURED: each partial caller accepts a checked certificate using only an
/// impossible arm's Eq Bool False True. CLAIMED: the added domain is a usable
/// kernel hypothesis, not merely printed structure. THE GAP: the proposition
/// remains an open obligation on real reachable paths; this is an impossible
/// branch witness only, not an overflow or nonzero solver.
#[test]
fn partial_primitive_holes_accept_certificates_needing_their_path_equation() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let results = env
        .elaborate_file_v1(
            "fn impossible_add (a : UInt8) (b : UInt8) : UInt8 = if False then a + b else a\n\
         fn impossible_div (n : Int) (d : Int) : Int = if False then n / d else n",
        )
        .expect("impossible arms still report their partial primitive obligations");
    for name in ["impossible_add", "impossible_div"] {
        let obligation = single(&env, result(&results, name), ObligationKind::PartialPrim);
        check_equation_dependent_certificate(&env, obligation);
    }
    assert_reported_trust_delta(&env, &before, &results);
}

fn requires_call_argument<'a>(body: &'a Term, callee: ken_kernel::GlobalId) -> Option<&'a Term> {
    if let Term::App(function, argument) = body {
        if matches!(function.as_ref(), Term::App(head, _) if matches!(head.as_ref(), Term::Const { id, .. } if *id == callee))
        {
            return Some(argument);
        }
    }
    body.children()
        .into_iter()
        .find_map(|child| requires_call_argument(child, callee))
}

/// MEASURED: a checked if convoy supplies its Eq binder directly as the
/// requires argument, without minting a hole; the related non-convertible
/// requirement mints a hole whose Γ includes that binder. CLAIMED: applied
/// holes never gain an unapplied proposition-only domain. THE GAP: a missing
/// convoy must redden the direct row's hole count and the related row's type.
/// Known boundary: a non-convoyable unbound frame keeps the stronger old
/// obligation rather than refusing; the indexed-match Requires acceptance
/// in `lang_match_result_refinement_identity` pins that non-refusal separately.
#[test]
fn requires_argument_is_checked_binder_or_hole_applied_to_checked_binder() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let results = env
        .elaborate_file_v1(
            "fn direct (b : Bool) : Bool requires Equal Bool b True = b\n\
         fn direct_plain (b : Bool) : Bool = direct b\n\
         fn direct_branch (b : Bool) : Bool = if b then direct b else b\n\
         fn indirect (b : Bool) : Bool requires Not (Equal Bool b False) = b\n\
         fn indirect_plain (b : Bool) : Bool = indirect b\n\
         fn indirect_branch (b : Bool) : Bool = if b then indirect b else b",
        )
        .expect("requires call inside checked if retries with checked Γ binder");
    let direct_plain = single(
        &env,
        result(&results, "direct_plain"),
        ObligationKind::Requires,
    );
    let direct = result(&results, "direct_branch");
    assert!(
        direct.obligations.is_empty(),
        "directly equal premise is checked evidence, not a hole"
    );
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(direct.def_id) else {
        panic!("checked direct function");
    };
    assert!(
        matches!(
            requires_call_argument(body, result(&results, "direct").def_id),
            Some(Term::Var(_))
        ),
        "core passes a bound equation, not a hole constant"
    );
    let indirect_plain = single(
        &env,
        result(&results, "indirect_plain"),
        ObligationKind::Requires,
    );
    let indirect = single(
        &env,
        result(&results, "indirect_branch"),
        ObligationKind::Requires,
    );
    let (plain_domains, _) = peel_pis(&direct_plain.goal_closed);
    assert_eq!(
        plain_domains.len(),
        1,
        "path-free direct control stays Π Bool. Eq"
    );
    let (plain_domains, plain_body) = peel_pis(&indirect_plain.goal_closed);
    assert_eq!(
        plain_domains.len(),
        1,
        "path-free indirect control stays Π Bool. Not Eq"
    );
    assert_eq!(
        indirect.goal_closed,
        Term::pi(
            Term::indformer(env.globals["Bool"], vec![]),
            Term::pi(eq_bool(&env, 0, "True"), weaken(plain_body, 1)),
        )
    );
    let Some(Decl::Transparent { body, .. }) =
        env.env.lookup(result(&results, "indirect_branch").def_id)
    else {
        panic!("checked indirect function");
    };
    let argument = requires_call_argument(body, result(&results, "indirect").def_id)
        .expect("indirect call retains its premise argument");
    assert!(
        matches!(argument, Term::App(function, eq) if matches!(function.as_ref(), Term::App(head, b)
            if matches!(head.as_ref(), Term::Const { id, .. } if *id == indirect.hole_id)
                && *b.as_ref() == Term::var(1)) && *eq.as_ref() == Term::var(0)),
        "reported hole is applied to Γ including the checked Eq binder"
    );
    assert_reported_trust_delta(&env, &before, &results);
}

/// MEASURED: the related but non-convertible Requires goal keeps exactly one
/// open hole under Γ, Eq Bool b True. CLAIMED: R2 convoy demand cannot be
/// removed without weakening that reported hole's logical context. THE GAP:
/// this row is independently mutation-red even if the direct-equality row
/// aborts first when R2 is removed.
#[test]
fn related_requires_hole_independently_needs_the_checked_if_equation() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let results = env
        .elaborate_file_v1(
            "fn indirect (b : Bool) : Bool requires Not (Equal Bool b False) = b\n\
         fn outside (b : Bool) : Bool = indirect b\n\
         fn inside (b : Bool) : Bool = if b then indirect b else b",
        )
        .expect("related Requires call admits with an open checked-convoy hole");
    let plain = single(&env, result(&results, "outside"), ObligationKind::Requires);
    let (_, body) = peel_pis(&plain.goal_closed);
    let inside = single(&env, result(&results, "inside"), ObligationKind::Requires);
    assert_eq!(
        inside.goal_closed,
        Term::pi(
            Term::indformer(env.globals["Bool"], vec![]),
            Term::pi(eq_bool(&env, 0, "True"), weaken(body, 1)),
        )
    );
    assert_reported_trust_delta(&env, &before, &results);
}

/// MEASURED: the recursive Nat `Suc k` arm's Requires hole has the exact
/// constructor equation in its checked Γ, after its existing Requires proof
/// and field binders. CLAIMED: the R2 retry carries match equations into
/// call-site obligations without bypassing SCT. THE GAP: an obligation-count
/// check alone stays green if the entire Eq domain disappears.
#[test]
fn recursive_match_requires_hole_pins_constructor_equation() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let declaration = env
        .elaborate_decl_v1(
            "fn r (n : Nat) : Nat requires Equal Nat n n = \
         match n { Zero ↦ Zero; Suc k ↦ r k }",
        )
        .expect("structural recursive call keeps one Requires hole");
    let hole = single(&env, &declaration, ObligationKind::Requires);
    let nat = Term::indformer(env.globals["Nat"], vec![]);
    let expected = Term::pi(
        nat.clone(),
        Term::pi(
            written_equal(&env, nat.clone(), Term::var(0), Term::var(0)),
            Term::pi(
                nat.clone(),
                Term::pi(
                    Term::Eq(
                        Box::new(nat.clone()),
                        Box::new(Term::var(2)),
                        Box::new(Term::app(
                            Term::constructor(env.globals["Suc"], vec![]),
                            Term::var(0),
                        )),
                    ),
                    written_equal(&env, nat, Term::var(1), Term::var(1)),
                ),
            ),
        ),
    );
    assert_eq!(hole.goal_closed, expected);
    assert!(
        env.env.transparent_body(declaration.def_id).is_some(),
        "SCT checked the recursive body"
    );
    assert_reported_trust_delta(&env, &before, std::slice::from_ref(&declaration));
}

/// MEASURED: an informative `let x = a` closes a Requires call with both x
/// and `Eq Int x a` as binders, not only the outer a. CLAIMED: a missing R2
/// retry cannot preserve the stated call-site obligation. THE GAP: the
/// outside-let control is checked against the original one-binder type.
#[test]
fn let_body_requires_hole_pins_local_equation() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect();
    let results = env
        .elaborate_file_v1(
            "fn need (x : Int) : Int requires Equal Int x 5 = x\n\
         fn outside (a : Int) : Int = need a\n\
         fn inside (a : Int) : Int = let x : Int = a in need x",
        )
        .expect("informative let body checks an applied Requires hole");
    let plain = single(&env, result(&results, "outside"), ObligationKind::Requires);
    let inside = single(&env, result(&results, "inside"), ObligationKind::Requires);
    let int = Term::const_(env.globals["Int"], vec![]);
    assert_eq!(
        plain.goal_closed,
        Term::pi(
            int.clone(),
            written_equal(&env, int.clone(), Term::var(0), Term::IntLit(5.into()))
        )
    );
    assert_eq!(
        inside.goal_closed,
        Term::pi(
            int.clone(),
            Term::pi(
                int.clone(),
                Term::pi(
                    Term::Eq(
                        Box::new(int.clone()),
                        Box::new(Term::var(0)),
                        Box::new(Term::var(1))
                    ),
                    written_equal(&env, int, Term::var(1), Term::IntLit(5.into())),
                ),
            ),
        )
    );
    assert_reported_trust_delta(&env, &before, &results);
}
