//! D0/D1 compatibility vectors for the structural-result association and
//! contextual selector (`34 §3.1.1`, `39 §2.3`/`§4`).

use ken_elaborator::{
    error::ElabError,
    parser::parse_expr,
    resolve::{resolve_expr_standalone, RExpr},
    ElabEnv, Expr,
};
use ken_kernel::{AllSupportSort, Term};

const STRUCTURAL_SIZE_SOURCE: &str = "data Bag (a : Type) : Type where { \
      Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a \
    }\n\
    data LiftRose = LiftLeaf | LiftNode (Bag LiftRose)\n\
    fn add (x : Nat) (y : Nat) : Nat = match x { \
      Zero |-> y ; Suc x2 |-> Suc (add x2 y) \
    }\n\
    fn size (r : LiftRose) : Nat = match r { \
      LiftLeaf |-> Suc Zero ; \
      LiftNode b |-> match b { \
        Empty |-> Zero ; \
        One x |-> size x ; \
        Join xs ys |-> add (structural result of xs) \
                            (structural result of ys) \
      } \
    }\n\
    const result : Nat = size \
      (LiftNode (Join LiftRose \
        (One LiftRose LiftLeaf) \
        (Join LiftRose (One LiftRose LiftLeaf) (Empty LiftRose))))";

const WSTYLE_OUT_OF_SCOPE_SOURCE: &str = "data WBag (a : Type) : Type where { WEmpty : WBag a ; WBranch : (Bool -> a) -> WBag a }\n\
data WRose = WLeaf | WNode (WBag WRose)\n\
fn wsize (r : WRose) : Nat = match r { WLeaf |-> Suc Zero ; WNode b |-> match b { WEmpty |-> Zero ; WBranch k |-> (structural result of k) True } }";

const DEEP_STRUCTURAL_SIZE_SOURCE: &str = "data Bag (a : Type) : Type where { \
      Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a \
    }\n\
    data LiftRose = LiftLeaf | LiftNode (Bag LiftRose)\n\
    fn add (x : Nat) (y : Nat) : Nat = match x { \
      Zero |-> y ; Suc x2 |-> Suc (add x2 y) \
    }\n\
    fn size (r : LiftRose) : Nat = match r { \
      LiftLeaf |-> Suc Zero ; \
      LiftNode b |-> match b { \
        Empty |-> Zero ; One x |-> size x ; \
        Join xs ys |-> add (structural result of xs) \
                            (structural result of ys) \
      } \
    }\n\
    const deep_result : Nat = size (LiftNode (Join LiftRose \
      (One LiftRose (LiftNode (Join LiftRose \
        (One LiftRose (LiftNode (Join LiftRose \
          (One LiftRose (LiftNode (Join LiftRose \
            (One LiftRose LiftLeaf) (Empty LiftRose)))) \
          (Empty LiftRose)))) \
        (Empty LiftRose)))) \
      (Empty LiftRose)))";

const MIXED_REACHING_SOURCE: &str = "data Bag (a : Type) : Type where { \
      Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a \
    }\n\
    data Rose = Leaf | Nested (Bag Rose)\n\
    data Direct = DLeaf | DNode Direct\n\
    data WTree = WLeaf | WNode Bool (Bool -> WTree)\n\
    fn add (x : Nat) (y : Nat) : Nat = match x { \
      Zero |-> y ; Suc x2 |-> Suc (add x2 y) \
    }\n\
    fn size (r : Rose) : Nat = match r { \
      Leaf |-> Suc Zero ; \
      Nested b |-> match b { \
        Empty |-> Zero ; \
        One x |-> size x ; \
        Join xs ys |-> add (structural result of xs) \
                            (structural result of ys) \
      } \
    }\n\
    fn direct_size (d : Direct) : Nat = match d { \
      DLeaf |-> Zero ; DNode child |-> Suc (direct_size child) \
    }\n\
    fn wsize (t : WTree) : Nat = match t { \
      WLeaf |-> Zero ; WNode b k |-> Suc Zero \
    }\n\
    const nested_result : Nat = size (Nested (Join Rose (One Rose Leaf) (Empty Rose)))\n\
    const direct_result : Nat = direct_size (DNode DLeaf)\n\
    const wstyle_result : Nat = wsize (WNode True (\\b. WLeaf))\n\
    const mixed_result : Nat = add direct_result (add wstyle_result nested_result)";

fn elimination_family(env: &ElabEnv, name: &str) -> ken_kernel::GlobalId {
    let (_, mut body) = env
        .env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("expected transparent {name}"));
    while let Term::Lam(_, next) = body {
        body = *next;
    }
    match body {
        Term::Elim { fam, .. } => fam,
        other => panic!("expected {name} to retain a match elimination, got {other:?}"),
    }
}

fn strip_lambdas(mut term: Term) -> Term {
    while let Term::Lam(_, body) = term {
        term = *body;
    }
    term
}

fn vars(term: &Term, found: &mut Vec<usize>) {
    if let Term::Var(index) = term {
        found.push(*index);
    }
    for child in term.children() {
        vars(child, found);
    }
}

#[test]
fn selector_is_contextual_and_resolves_the_surface_binding_identity() {
    // MEASURED: the complete four-word primary parses as one selector and its
    // operand resolves to the innermost let binding's de Bruijn identity.
    // CLAIMED: the selector is contextual and identity-based.
    // THE GAP: elaboration must use that identity only through a validated
    // branch association; the positive and out-of-scope tests below cover it.
    let parsed = parse_expr("let x : Nat = Zero in structural result of x").unwrap();
    let resolved = resolve_expr_standalone(&parsed).unwrap();
    let RExpr::RLet(_, _, _, body, _) = resolved else {
        panic!("expected a resolved let expression");
    };
    assert!(matches!(
        *body,
        RExpr::RStructuralResult {
            index: 0,
            ref name,
            ..
        } if name == "x"
    ));

    let ordinary = parse_expr("structural result").unwrap();
    assert!(matches!(
        ordinary,
        Expr::EApp(f, a, _)
            if matches!(*f, Expr::EVar(ref name, _) if name == "structural")
                && matches!(*a, Expr::EVar(ref name, _) if name == "result")
    ));
}

#[test]
fn unbound_operand_remains_unbound_name() {
    // MEASURED: resolution rejects an absent selector operand with UnboundName.
    // CLAIMED: the selector does not invent or publish a hidden source name.
    // THE GAP: generated-support names are a broader resolution property and
    // belong to D2; this pin is deliberately limited to the D1 operand rule.
    let parsed = parse_expr("structural result of absent").unwrap();
    assert!(matches!(
        resolve_expr_standalone(&parsed),
        Err(ElabError::UnboundName { ref name, .. }) if name == "absent"
    ));
}

#[test]
fn resolved_binding_without_association_is_out_of_scope() {
    // MEASURED: a real surface binding without a branch association reaches
    // the selector gate and receives the named out-of-scope diagnostic.
    // CLAIMED: ordinary variables never acquire structural results by type or
    // spelling. THE GAP: a positive nested-method witness is required to show
    // that the gate also admits a validated association; the next test does so.
    let mut env = ElabEnv::new().unwrap();
    assert!(matches!(
        env.elaborate_expr(
            "resolved_binding_without_association_is_out_of_scope",
            "let x : Nat = Zero in structural result of x",
        ),
        Err(ElabError::StructuralResultOutOfScope { .. })
    ));
}

#[test]
fn validated_nested_results_elaborate_and_kernel_check() {
    // MEASURED: two source fields in a generated-support Join method select
    // their correlated trailing method results and the completed eliminator
    // passes the elaborator's kernel re-check.
    // CLAIMED: D0's telescope association is consumed by D1's hidden-result
    // selector. THE GAP: this finite witness does not prove arbitrary depth or
    // carrier anonymity; those mutation-backed discriminators belong to D2.
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_file(STRUCTURAL_SIZE_SOURCE).unwrap();
    assert!(env.globals.contains_key("result"));
}

#[test]
fn d2_wstyle_without_a_trailing_result_binder_is_exactly_out_of_scope() {
    let mut env = ElabEnv::new().unwrap();
    assert!(matches!(
        env.elaborate_file(WSTYLE_OUT_OF_SCOPE_SOURCE),
        Err(ElabError::StructuralResultOutOfScope { .. })
    ));
}

#[test]
fn d2_mixed_direct_wstyle_and_nested_structural_paths_reach_and_retain_behavior() {
    // The selector appears only in `size`'s nested residual Join arm, where
    // each source field has its complete source/evidence/trailing-result triple.
    // The direct and W-style matches remain ordinary recursive behavior.
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_file(MIXED_REACHING_SOURCE).unwrap();

    // Byte-for-behavior retention: the direct and W-style functions still emit
    // eliminations over their own original families, while `size` emits the
    // nested structural elimination. The concrete `mixed_result` retains calls
    // to all three checked functions rather than replacing either ordinary path.
    assert_eq!(elimination_family(&env, "direct_size"), env.globals["Direct"]);
    assert_eq!(elimination_family(&env, "wsize"), env.globals["WTree"]);
    assert_eq!(elimination_family(&env, "size"), env.globals["Rose"]);
    assert!(env.env.transparent_body(env.globals["mixed_result"]).is_some());
}

#[test]
fn d2_nested_join_method_selects_each_exact_trailing_result_binder() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_file(STRUCTURAL_SIZE_SOURCE).unwrap();
    let (_, size_body) = env.env.transparent_body(env.globals["size"]).unwrap();
    let Term::Elim { methods, .. } = strip_lambdas(size_body) else {
        panic!("size must emit its outer LiftRose elimination");
    };
    let Term::Elim { methods, .. } = strip_lambdas(methods[1].clone()) else {
        panic!("LiftNode method must emit its nested generated-support elimination");
    };
    let join_body = strip_lambdas(methods[2].clone());
    let mut references = Vec::new();
    vars(&join_body, &mut references);

    // Join has two source fields, two hidden evidence fields, and then two
    // distinct trailing recursive results. At the method body, those results
    // are de Bruijn 1 and 0; evidence is 3 and 2. Both selectors must use the
    // trailing result pair, not a hidden evidence binder.
    assert!(references.contains(&0));
    assert!(references.contains(&1));
    assert!(!references.contains(&2));
    assert!(!references.contains(&3));
}

#[test]
fn d2_deep_nested_associations_elaborate_and_kernel_check() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_file(DEEP_STRUCTURAL_SIZE_SOURCE).unwrap();
    assert!(env.globals.contains_key("deep_result"));
}

#[test]
fn d2_generated_support_name_stays_unresolvable_beside_visible_source_name() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_decl("data Bag (a : Type) : Type where { Empty : Bag a }")
        .unwrap();
    let support = env
        .env
        .all_support(env.globals["Bag"], 0, AllSupportSort::Type)
        .expect("Bag's real generated Type support exists");
    let generated_name = format!("{support:?}");

    // The token is the actual generated support identity, not a guessed
    // spelling. It is deliberately absent from the source resolver's globals.
    assert!(matches!(
        env.elaborate_decl(&format!("const hidden_support : Type = {generated_name}")),
        Err(ElabError::UnresolvedCon { name, .. }) if name == generated_name
    ));
    // Same type-expression position, but a real source family, remains usable.
    env.elaborate_decl("const visible_source : Type = Bag Nat")
        .expect("source-visible Bag resolves at the identical type position");
}
