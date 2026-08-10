//! D0/D1 compatibility vectors for the structural-result association and
//! contextual selector (`34 §3.1.1`, `39 §2.3`/`§4`).

use ken_elaborator::{
    error::ElabError,
    parser::parse_expr,
    resolve::{resolve_expr_standalone, RExpr},
    ElabEnv, Expr,
};

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
