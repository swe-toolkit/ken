//! `LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE` acceptance.
//!
//! Constructor index refinement in a dependent match must transform the
//! transitive forward-dependency closure of the LOCAL CONTEXT — the ordered
//! dependent tail of captured ambient bindings — as one telescope substitution,
//! together with the motive, constructor expected goal, and direct IH. A
//! captured `xs : Env n` must follow the refinement of the `Fin n` index under
//! `match j`, generalized into the motive codomain and applied back to the
//! original ambient values after the eliminator.

use ken_elaborator::ElabEnv;
use ken_kernel::{GlobalId, Term};

/// Peel exactly `n` `Term::Lam` layers.
fn peel_lams(t: &Term, n: usize) -> &Term {
    let mut cur = t;
    for _ in 0..n {
        match cur {
            Term::Lam(_, body) => cur = body,
            other => panic!("expected a Lam layer to peel, got {other:?}"),
        }
    }
    cur
}

/// The `GlobalId` at the head of an application spine, if it is a family former.
fn spine_head_id(t: &Term) -> Option<GlobalId> {
    let mut head = t;
    while let Term::App(f, _) = head {
        head = f.as_ref();
    }
    match head {
        Term::IndFormer { id, .. } => Some(*id),
        _ => None,
    }
}

/// The outermost application's argument (the spine's last-applied term).
fn spine_last_arg(t: &Term) -> Option<&Term> {
    match t {
        Term::App(_, arg) => Some(arg.as_ref()),
        _ => None,
    }
}

/// The family-former `GlobalId` at the head of each `Term::Lam` DOMAIN reachable
/// by peeling `Lam`/`Pi` layers from `t` (used to prove a method reproduces the
/// convoy binders in its own telescope).
fn lam_domain_family_heads(t: &Term) -> Vec<Option<GlobalId>> {
    let mut heads = Vec::new();
    let mut cur = t;
    loop {
        match cur {
            Term::Lam(dom, body) | Term::Pi(dom, body) => {
                heads.push(spine_head_id(dom));
                cur = body.as_ref();
            }
            _ => break,
        }
    }
    heads
}

/// Walk a `Pi` telescope until a domain's spine head is `fam`; return the pair
/// (that domain, the codomain under it). Panics if `fam` never appears.
fn telescope_find<'a>(mut t: &'a Term, fam: GlobalId) -> (&'a Term, &'a Term) {
    loop {
        match t {
            Term::Pi(dom, cod) => {
                if spine_head_id(dom) == Some(fam) {
                    return (dom.as_ref(), cod.as_ref());
                }
                t = cod.as_ref();
            }
            other => panic!("family {fam:?} not found in the telescope; reached {other:?}"),
        }
    }
}

/// Base env with a fresh generic index-refining family `Fin`, a length-indexed
/// `Env`, and a total `elookup` — NO Fok names.
fn fin_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "data Fin : Nat -> Type where { \
           FZ : (n : Nat) -> Fin (Suc n); \
           FS : (n : Nat) -> Fin n -> Fin (Suc n) }",
    )
    .expect("Fin");
    env.elaborate_decl(
        "data Env (a : Type) : Nat -> Type where { \
           ENil : Env a Zero; \
           ECons : (n : Nat) -> a -> Env a n -> Env a (Suc n) }",
    )
    .expect("Env");
    env.elaborate_decl(
        "fn elookup (a : Type) (n : Nat) (xs : Env a n) (i : Fin n) : a = \
         match i { \
           FZ m ↦ match xs { ECons k y rest ↦ y }; \
           FS m j ↦ match xs { ECons k y rest ↦ elookup a m rest j } }",
    )
    .expect("elookup");
    env
}

/// Extends [`fin_env`] with a witness family `Wit a n e` indexed by BOTH the
/// length `n` and the `Env a n` value `e`, so a captured `h : Wit a n xs` depends
/// transitively on the captured `xs : Env a n`.
fn fin_env_wit() -> ElabEnv {
    let mut env = fin_env();
    env.elaborate_decl(
        "data Wit (a : Type) : (n : Nat) -> Env a n -> Type where { \
           MkWit : (n : Nat) -> (e : Env a n) -> Wit a n e }",
    )
    .expect("Wit");
    env
}

/// Extends [`fin_env_wit`] with a third family `Fam a n e w` indexed by the
/// length, the `Env`, AND the `Wit` — so a captured `z : Fam a n xs h` depends
/// on all of the length, the captured `xs`, and the captured `h`.
fn fin_env_fam() -> ElabEnv {
    let mut env = fin_env_wit();
    env.elaborate_decl(
        "data Fam (a : Type) : (n : Nat) -> (e : Env a n) -> Wit a n e -> Type where { \
           MkFam : (n : Nat) -> (e : Env a n) -> (w : Wit a n e) -> Fam a n e w }",
    )
    .expect("Fam");
    env
}

#[test]
fn captured_env_follows_fin_index_refinement_under_match() {
    // THE RED SHAPE (evt_3b9k92cmkn5zh, generic): the captured ambient
    // `xs : Env A n` must follow the `Fin n` refinement of `match j`, so the
    // FS branch's goal `elookup (ECons x xs)(FS n j) = elookup xs j` closes.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem elookup_cons_fs (a : Type) (n : Nat) (x : a) (xs : Env a n) (j : Fin n) \
           : Equal a (elookup a (Suc n) (ECons a n x xs) (FS n j)) (elookup a n xs j) = \
         match j { \
           FZ m ↦ Refl; \
           FS m k ↦ Refl }",
    )
    .expect("the captured-env telescope must follow the Fin refinement");
}

#[test]
fn captured_env_index_couples_goal_type_argument() {
    // The goal's TYPE argument names the index directly (`Equal (Env a n) xs
    // xs`): the captured `xs`'s refined type and the goal's own `n` must refine
    // in lockstep. Without the convoy forcing the goal's index rebase (the
    // scrutinee `j` does not itself occur here), `xs`'s binder index and the
    // goal's `Env a n` diverge and the arm reddens on the convoy class.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem env_refl (a : Type) (n : Nat) (xs : Env a n) (j : Fin n) \
           : Equal (Env a n) xs xs = \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("the captured env's type must follow its own index refinement");
}

#[test]
fn transitive_closure_follows_index_refinement() {
    // THE TRANSITIVE SHAPE: the captured tail is `xs : Env a n`, then
    // `h : Wit a n xs` depending on BOTH the index and the earlier captured
    // `xs`. Both must travel as ONE telescope through the `Fin n` refinement of
    // `match j`, with `h`'s binder type naming `xs`'s binder (not the ambient
    // `xs`). This exercises the inter-convoy threading (c > 1) in the motive,
    // the constructor goal, and the recursive-FS direct IH.
    let mut env = fin_env_wit();
    env.elaborate_decl(
        "theorem wit_refl (a : Type) (n : Nat) (xs : Env a n) (h : Wit a n xs) \
           (j : Fin n) : Equal (Wit a n xs) h h = \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("the transitive Wit/Env telescope must follow the Fin refinement");
}

#[test]
fn three_deep_transitive_closure_follows_index_refinement() {
    // The WP's full transitive shape: the captured tail is `xs : Env a n`, then
    // `h : Wit a n xs`, then `z : Fam a n xs h` — each depending on the index
    // AND every earlier captured binder. All three must travel as ONE ordered
    // telescope through the `Fin n` refinement of `match j`.
    let mut env = fin_env_fam();
    env.elaborate_decl(
        "theorem fam_refl (a : Type) (n : Nat) (xs : Env a n) (h : Wit a n xs) \
           (z : Fam a n xs h) (j : Fin n) : Equal (Fam a n xs h) z z = \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("the three-deep Fam/Wit/Env telescope must follow the Fin refinement");
}

#[test]
fn indirect_dependency_closure_via_predicate_param() {
    // Architect counterexample (evt_1r1jwkm9ctpk8): `h : p xs` depends on the
    // captured `xs` (and `p`), which depend on the index `n`, but h's OWN type
    // names neither `n` nor the scrutinee — only `p` and `xs`. The dependency
    // closure must therefore be computed OUTERMOST-first (so `xs`/`p` enter the
    // set before `h` is tested); an innermost-first walk drops `h` and reddens
    // with `TypeMismatch { expected: (@1 @0), found: (@8 @7) }`.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem param_pred_refl (a : Type) (n : Nat) (p : (e : Env a n) -> Type) \
           (xs : Env a n) (h : p xs) (j : Fin n) : Equal (p xs) h h = \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("an indirect (non-index-repeating) dependency must still convoy");
}

#[test]
fn dependent_let_convoy_member_positive() {
    // Dependent-let boundary (positive): a `let`-bound alias of a captured
    // ambient binding, itself index-dependent, must convoy correctly through the
    // refinement. `ys : Env a n = xs` aliases the captured `xs`, and the goal is
    // stated over `ys`, so the convoy must carry the let-bound member.
    let mut env = fin_env();
    env.elaborate_decl(
        "theorem let_alias_refl (a : Type) (n : Nat) (xs : Env a n) (j : Fin n) \
           : Equal (Env a n) xs xs = \
         let ys : Env a n = xs in \
         match j { FZ m ↦ Refl; FS m k ↦ Refl }",
    )
    .expect("a dependent let-bound convoy member must follow the refinement");
}

#[test]
fn elim_carries_convoy_telescope_in_motive_methods_and_application() {
    // AST inspection of the three-deep convoy across ALL of the elaborated
    // `Term::Elim`: (1) the motive codomain is a Π-telescope xs'/h'/z' with each
    // inner binder naming its outer neighbour (`Var 0`); (2) EVERY method
    // reproduces that xs'/h'/z' telescope; (3) the final application feeds the
    // ambient actuals `xs`, `h`, `z` back in that order. A consistently-applied-
    // but-wrong substitution could pass the kernel while failing any of these.
    let mut env = fin_env_fam();
    let id = env
        .elaborate_decl(
            "theorem fam_refl (a : Type) (n : Nat) (xs : Env a n) (h : Wit a n xs) \
               (z : Fam a n xs h) (j : Fin n) : Equal (Fam a n xs h) z z = \
             match j { FZ m ↦ Refl; FS m k ↦ Refl }",
        )
        .expect("fam_refl elaborates and kernel-checks");
    let body = env
        .env
        .transparent_body(id)
        .expect("fam_refl is transparent")
        .1;
    let env_id = env.globals["Env"];
    let wit_id = env.globals["Wit"];
    let fam_id = env.globals["Fam"];

    // Peel the 6 parameter lambdas (a, n, xs, h, z, j), then collect the
    // eliminator's application spine to reach `Term::Elim`.
    let mut inner = &body;
    while let Term::Lam(_, b) = inner {
        inner = b;
    }
    let mut app_args: Vec<&Term> = Vec::new();
    let mut head = inner;
    while let Term::App(f, a) = head {
        app_args.push(a.as_ref());
        head = f.as_ref();
    }
    app_args.reverse();
    let (motive, methods) = match head {
        Term::Elim { fam, motive, methods, .. } => {
            assert_eq!(*fam, env.globals["Fin"], "must eliminate over Fin");
            (motive.as_ref(), methods)
        }
        other => panic!("the match must lower to a real Term::Elim, got {other:?}"),
    };

    // (3) FINAL APPLICATION ORDER. After the leading index-eq `Refl` premise the
    // eliminator is applied to the ambient convoy actuals in dependency order.
    // Ambient de Bruijn after the 6 param lambdas: xs=3, h=2, z=1.
    let na = app_args.len();
    assert!(na >= 3, "expected the three convoy actuals among the applications");
    assert!(
        matches!(app_args[na - 3], Term::Var(3)),
        "outer convoy actual must be `xs` (@3): {:?}",
        app_args[na - 3]
    );
    assert!(
        matches!(app_args[na - 2], Term::Var(2)),
        "middle convoy actual must be `h` (@2): {:?}",
        app_args[na - 2]
    );
    assert!(
        matches!(app_args[na - 1], Term::Var(1)),
        "inner convoy actual must be `z` (@1): {:?}",
        app_args[na - 1]
    );

    // (1) MOTIVE TELESCOPE. Unwrap the ascription, peel idx+scrut, then find the
    // xs'(Env) binder; h'(Wit) immediately follows and names it (Var 0); z'(Fam)
    // follows h' and names it (Var 0) — the chained one-telescope threading.
    let motive = match motive {
        Term::Ascript(t, _) => t.as_ref(),
        t => t,
    };
    let telescope = peel_lams(motive, 2);
    let (_env_dom, after_env) = telescope_find(telescope, env_id);
    let (wit_dom, after_wit) = match after_env {
        Term::Pi(a, b) => (a.as_ref(), b.as_ref()),
        o => panic!("Env binder must be followed by the Wit binder, got {o:?}"),
    };
    assert_eq!(spine_head_id(wit_dom), Some(wit_id), "second binder is Wit (`h'`)");
    assert!(
        matches!(spine_last_arg(wit_dom), Some(Term::Var(0))),
        "Wit binder must name the outer Env binder (Var 0): {wit_dom:?}"
    );
    let fam_dom = match after_wit {
        Term::Pi(a, _) => a.as_ref(),
        o => panic!("Wit binder must be followed by the Fam binder, got {o:?}"),
    };
    assert_eq!(spine_head_id(fam_dom), Some(fam_id), "third binder is Fam (`z'`)");
    assert!(
        matches!(spine_last_arg(fam_dom), Some(Term::Var(0))),
        "Fam binder must name the outer Wit binder (Var 0): {fam_dom:?}"
    );

    // (2) EVERY METHOD reproduces the xs'/h'/z' convoy telescope in its own
    // binder chain (not just the motive).
    assert_eq!(methods.len(), 2, "Fin has exactly two constructors");
    for (i, method) in methods.iter().enumerate() {
        let heads = lam_domain_family_heads(method);
        for (name, fam) in [("Env", env_id), ("Wit", wit_id), ("Fam", fam_id)] {
            assert!(
                heads.contains(&Some(fam)),
                "method {i} must reproduce the {name} convoy binder; domain heads: {heads:?}"
            );
        }
    }
}
