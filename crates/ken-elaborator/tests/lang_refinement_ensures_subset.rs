//! Contract results are checked subset Σ values, not detached obligations.

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::{Decl, GlobalId, Level, Term};

/// Promise class: durable result-representation invariant.
/// MEASURED: the admitted function's kernel type has a subset-Σ result and
/// its checked body constructs a Pair, with an open hole visible when false.
/// CLAIMED: a written `ensures` is intrinsic to the checked result type.
/// THE GAP: the inspected pair must inhabit the *declared* type, not an
/// unrelated synthetic term; `Decl::Transparent` supplies that identity.
#[test]
fn ensures_is_a_checked_result_pair_even_when_its_proof_is_open() {
    for (value, open) in [(5, false), (6, true)] {
        let mut env = ElabEnv::new().expect("prelude");
        let source = format!("fn result_subset (n : Int) : Int ensures Equal Int result 5 = {value}");
        let result = env.elaborate_decl_v1(&source).expect("typed result pair");
        let decl = env.env.lookup(result.def_id).expect("admitted definition");
        let Decl::Transparent { ty: Term::Pi(_, cod), body: Term::Lam(_, body), .. } = decl else {
            panic!("function must be admitted at the checked Pi signature: {decl:?}");
        };
        assert!(matches!(cod.as_ref(), Term::Sigma(..)), "ensures is in the kernel result type");
        assert!(matches!(body.as_ref(), Term::Pair(..)), "the function constructs the pair");
        let ensures = result.obligations.iter().filter(|o| matches!(o.kind, ObligationKind::Ensures))
            .collect::<Vec<_>>();
        assert_eq!(ensures.len(), 1, "one clause, one site-local obligation");
        assert_eq!(env.is_open_hole(ensures[0].hole_id), open,
            "proof status must follow the actual returned value");
    }
}

/// Promise class: durable multiple-clause proposition-conjunction invariant.
/// MEASURED: two independent obligations inhabit one subset Σ whose second
/// component is an Ω-valued Sigma; its checked proof is a pair of proofs.
/// CLAIMED: multiple ensures share the result carrier, never nest it twice.
/// THE GAP: the second Sigma must occur *inside the proposition* of the
/// outer result Sigma, not in the function parameter telescope.
#[test]
fn multiple_ensures_share_one_carrier_and_keep_independent_goals() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_decl_v1(
        "fn both (n : Int) : Int ensures Equal Int result 5 ensures Equal Int result 6 = 5",
    ).expect("checked multi-postcondition");
    let Decl::Transparent { ty: Term::Pi(_, cod), body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("admitted function") else { panic!("checked Pi") };
    assert!(matches!(cod.as_ref(), Term::Sigma(_, both) if matches!(both.as_ref(), Term::Sigma(..))),
        "both clauses must be one result subset with proof conjunction");
    assert!(matches!(body.as_ref(), Term::Pair(_, proof) if matches!(proof.as_ref(), Term::Pair(..))),
        "both proofs must be carried in the checked result pair");
    let goals = result.obligations.iter().filter(|o| matches!(o.kind, ObligationKind::Ensures))
        .collect::<Vec<_>>();
    assert_eq!(goals.len(), 2);
    assert!(!env.is_open_hole(goals[0].hole_id));
    assert!(env.is_open_hole(goals[1].hole_id));
}

/// Promise class: durable composition invariant for two independently written
/// return constraints. MEASURED: the explicit result refinement and written
/// ensures both survive in the checked nested subset type and returned pairs.
/// CLAIMED: neither source clause is erased while building the contract.
/// THE GAP: both clauses are provable on `5`, so requiring distinct obligation
/// kinds and checked nested pairs prevents a one-clause success from passing.
#[test]
fn explicit_refined_return_and_ensures_keep_both_checked_proofs() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_decl_v1(
        "fn combined (n : Int) : { x : Int | Equal Int x 5 } ensures Equal Int result 5 = 5",
    ).expect("both return clauses");
    let Decl::Transparent { ty: Term::Pi(_, cod), body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("admitted function") else { panic!("checked Pi") };
    assert!(matches!(cod.as_ref(), Term::Sigma(base, _) if matches!(base.as_ref(), Term::Sigma(..))),
        "both source constraints must survive in the kernel type");
    assert!(matches!(body.as_ref(), Term::Pair(value, _) if matches!(value.as_ref(), Term::Pair(..))),
        "both source constraints have proof-bearing introduction pairs");
    assert_eq!(result.obligations.len(), 2);
    assert!(result.obligations.iter().any(|o| matches!(o.kind, ObligationKind::Ensures)));
    assert!(result.obligations.iter().any(|o| matches!(o.kind, ObligationKind::RefinementIntroduction)));
    assert!(result.obligations.iter().all(|o| !env.is_open_hole(o.hole_id)));
}

/// Promise class: durable recursive-evidence invariant.
/// MEASURED: a structurally recursive function with a closed Zero proof
/// admits both checked per-path result pairs; the Suc path uses the returned
/// subset proof, not a detached proposition in an obligation-only context.
/// CLAIMED: self calls are typed at the full checked result Σ.
/// THE GAP: making the Zero clause false must force a typed refusal under
/// the non-convoyable recursive split, rather than silently admitting it.
#[test]
fn recursive_ensures_reuses_the_checked_self_call_proof() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_decl_v1(
        "fn rec_subset (n : Nat) : Int ensures Equal Int result 5 = match n { Zero |-> 5 ; Suc m |-> rec_subset m }",
    ).expect("recursive body returns a checked subset on both paths");
    let goals = result.obligations.iter().filter(|o| matches!(o.kind, ObligationKind::Ensures))
        .collect::<Vec<_>>();
    assert_eq!(goals.len(), 2);
    assert!(goals.iter().all(|o| !env.is_open_hole(o.hole_id)));
    let Decl::Transparent { ty: Term::Pi(_, cod), .. } =
        env.env.lookup(result.def_id).expect("recursive checked body") else { panic!("checked Pi") };
    assert!(matches!(cod.as_ref(), Term::Sigma(..)));
}

/// Promise class: durable refusal control for non-convoyable recursion.
/// MEASURED: swapping the Zero result from 5 to 6 leaves an open proof
/// beneath the recursive Nat split and returns the typed split refusal.
/// CLAIMED: no recursive path condition is silently closed into a pair hole
/// when the split has no checked convoy equation.
/// THE GAP: the adjacent 5/5 case succeeds with the identical family,
/// signature and recursive arm, isolating the unsolved base proof.
#[test]
fn recursive_open_ensures_without_a_convoy_refuses() {
    let mut env = ElabEnv::new().expect("prelude");
    let start_id = env.env.next_global_id();
    let error = env.elaborate_decl_v1(
        "fn rec_subset (n : Nat) : Int ensures Equal Int result 5 = match n { Zero |-> 6 ; Suc m |-> rec_subset m }",
    ).expect_err("open base case cannot acquire unsupported recursive evidence");
    assert!(matches!(error, ElabError::NonConvoyableRefinementSplit { .. }), "{error:?}");
    assert_eq!(env.env.next_global_id(), start_id, "failed clause rolls back the staged self");
    assert!(!env.globals.contains_key("rec_subset"));
}

/// Promise class: durable conditional result-Σ convoy invariant.
/// MEASURED: each branch of an opaque postcondition has one open checked
/// pair hole, with exactly its own Eq Bool path binder. CLAIMED: a real
/// branch witness, not a proposition-only path, reaches each checked pair.
/// THE GAP: the emitted conditional must apply its motive to `Refl b`;
/// counting obligations alone would not establish that term evidence.
#[test]
fn conditional_result_pairs_bind_one_checked_equation_per_branch() {
    let mut env = ElabEnv::new().expect("prelude");
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int, Term::omega(Level::Zero)))
        .expect("opaque postcondition");
    let result = env.elaborate_decl_v1(
        "fn branch_subset (b : Bool) : Int ensures P result = if b then 5 else 6",
    ).expect("one checked pair in each conditional method");
    let Decl::Transparent { body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("admitted result") else { panic!("checked function") };
    assert!(matches!(body.as_ref(), Term::App(elim, proof)
        if matches!(elim.as_ref(), Term::Elim { .. })
            && matches!(proof.as_ref(), Term::Refl(_))),
        "the checked convoy is applied to its source scrutinee proof");
    assert_eq!(result.obligations.len(), 2);
    for (obligation, constructor) in result.obligations.iter().zip(["True", "False"]) {
        assert!(matches!(obligation.kind, ObligationKind::Ensures));
        assert!(env.is_open_hole(obligation.hole_id));
        let Term::Pi(_, goal) = &obligation.goal_closed else { panic!("parameter binder") };
        let Term::Pi(eq, _) = goal.as_ref() else { panic!("convoy equation binder") };
        assert!(matches!(eq.as_ref(), Term::Eq(_, _, right)
            if right.as_ref() == &Term::constructor(env.globals[constructor], vec![])));
    }
}

/// Promise class: durable let-result-evidence invariant.
/// MEASURED: an opaque postcondition on a let-bound value has one open hole
/// whose path equation is represented once by a checked Γ proof binder.
/// CLAIMED: an introduction under let does not invent a logical-only fact.
/// THE GAP: the emitted let body must apply its proof lambda to Refl(rhs);
/// observing the obligation telescope alone cannot witness that argument.
#[test]
fn let_result_pair_uses_a_checked_rhs_equality() {
    let mut env = ElabEnv::new().expect("prelude");
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int, Term::omega(Level::Zero)))
        .expect("opaque postcondition");
    let result = env.elaborate_decl_v1(
        "fn let_subset (n : Int) : Int ensures P result = let r = n in r",
    ).expect("checked let with equality witness");
    let Decl::Transparent { body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("admitted result") else { panic!("checked function") };
    assert!(matches!(body.as_ref(), Term::Let { body, .. }
        if matches!(body.as_ref(), Term::App(_, proof)
            if matches!(proof.as_ref(), Term::Refl(_)))),
        "the let body applies checked Refl(rhs) to its proof binder");
    let [obligation] = result.obligations.as_slice() else { panic!("one result leaf") };
    assert!(matches!(obligation.kind, ObligationKind::Ensures));
    assert!(env.is_open_hole(obligation.hole_id));
    let Term::Pi(_, rest) = &obligation.goal_closed else { panic!("parameter") };
    let Term::Pi(_, rest) = rest.as_ref() else { panic!("let binder") };
    assert!(matches!(rest.as_ref(), Term::Pi(eq, _)
        if matches!(eq.as_ref(), Term::Eq(..))), "one checked let equation binder");
}

/// Promise class: durable nonrecursive constructor-matrix convoy invariant.
/// MEASURED: a nested Bool pattern beneath a two-constructor wrapper emits
/// three checked result pairs, with open holes consuming both split equations
/// where the nested pattern demands them. CLAIMED: general-matrix splits
/// provide checked evidence at the actual result leaves. THE GAP: inspect
/// each method's final Refl application and the hole's binder telescope;
/// obligation counts alone would pass with an unchecked path assumption.
#[test]
fn nested_constructor_result_pairs_replay_checked_split_equations() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("data Slot = Filled Bool | Vacant")
        .expect("finite, nonrecursive wrapper");
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int, Term::omega(Level::Zero)))
        .expect("opaque result predicate");
    let result = env.elaborate_decl_v1(
        "fn nested_subset (s : Slot) : Int ensures P result = match s { Filled True |-> 5 ; Filled False |-> 6 ; Vacant |-> 7 }",
    ).expect("nested checked result pairs");
    assert_eq!(result.obligations.len(), 3);
    assert!(result.obligations.iter().all(|o|
        matches!(o.kind, ObligationKind::Ensures) && env.is_open_hole(o.hole_id)));
    fn refl_elim_count(term: &Term, family: GlobalId) -> usize {
        let here = if let Term::App(head, witness) = term {
            let mut function = head.as_ref();
            while let Term::App(prior, _) = function { function = prior; }
            usize::from(matches!(function, Term::Elim { fam, .. } if *fam == family)
                && matches!(witness.as_ref(), Term::Refl(_)))
        } else { 0 };
        here + term.children().iter().map(|child| refl_elim_count(child, family)).sum::<usize>()
    }
    fn proof_pair_carries_hole(term: &Term, hole: GlobalId) -> bool {
        fn contains(term: &Term, hole: GlobalId) -> bool {
            matches!(term, Term::Const { id, .. } if *id == hole)
                || term.children().iter().any(|child| contains(child, hole))
        }
        matches!(term, Term::Pair(_, proof) if contains(proof, hole))
            || term.children().iter().any(|child| proof_pair_carries_hole(child, hole))
    }
    let Decl::Transparent { body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("checked nested result") else { panic!("function") };
    assert_eq!(refl_elim_count(body, env.globals["Slot"]), 1,
        "root split consumes its own checked equation");
    assert_eq!(refl_elim_count(body, env.globals["Bool"]), 1,
        "nested split consumes its own checked equation after the root tail");
    for (obligation, split_types) in result.obligations.iter().zip([
        vec![env.globals["Slot"], env.globals["Bool"]],
        vec![env.globals["Slot"], env.globals["Bool"]],
        vec![env.globals["Slot"]],
    ]) {
        let mut actual = Vec::new();
        let mut goal = &obligation.goal_closed;
        while let Term::Pi(domain, body) = goal {
            if let Term::Eq(carrier, _, _) = domain.as_ref() {
                let Term::IndFormer { id, .. } = carrier.as_ref() else {
                    panic!("constructor equation carrier: {carrier:?}");
                };
                actual.push(*id);
            }
            goal = body;
        }
        assert_eq!(actual, split_types, "only the witnessed splits appear in the hole context");
        assert!(proof_pair_carries_hole(body, obligation.hole_id),
            "the exact recorded obligation is the pair's proof");
    }

    let closed = env.elaborate_decl_v1(
        "fn nested_closed (s : Slot) : Int ensures Equal Int result result = match s { Filled True |-> 5 ; Filled False |-> 6 ; Vacant |-> 7 }",
    ).expect("each branch has a closed reflexive certificate");
    let Decl::Transparent { body: Term::Lam(_, body), .. } =
        env.env.lookup(closed.def_id).expect("checked closed result") else { panic!("function") };
    assert_eq!(closed.obligations.len(), 3);
    assert!(closed.obligations.iter().all(|o| !env.is_open_hole(o.hole_id)));
    assert_eq!(refl_elim_count(body, env.globals["Slot"]), 0,
        "closed certificates need no outer convoy");
    assert_eq!(refl_elim_count(body, env.globals["Bool"]), 0,
        "closed certificates need no inner convoy");
}

/// Promise class: transition sentinel for inferred contract carriers.
/// MEASURED: a source without a result annotation refuses at the contract
/// boundary without admitting a bare function or detached obligation.
/// CLAIMED: until return inference can preserve parameter annotations and
/// build a checked result Σ, it cannot falsely advertise an `ensures` proof.
/// THE GAP: the same clause is admitted with an explicit `: Int` above;
/// retire this sentinel when the inference path gains a checked Σ result.
#[test]
fn unannotated_ensures_refuses_instead_of_detaching_its_proof() {
    let mut env = ElabEnv::new().expect("prelude");
    let start = env.env.next_global_id();
    let error = env.elaborate_decl_v1(
        "fn no_result_type (n : Int) ensures Equal Int result n = n",
    ).expect_err("cannot promise a detached postcondition");
    assert!(matches!(error, ElabError::TypeMismatch { reason, .. }
        if reason == "ensures requires an explicit result type for a checked subset Σ"));
    assert_eq!(env.env.next_global_id(), start);
    assert!(!env.globals.contains_key("no_result_type"));
}
