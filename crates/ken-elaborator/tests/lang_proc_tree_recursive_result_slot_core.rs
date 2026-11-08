//! Spec 36 §2.4, SLOTS Increment 3. Promise class: durable invariant.
//! MEASURED: a selected recursive result in a scrutinee converts to its
//! sequenced-let twin; a recursive call under a scrutinee converts to its let
//! twin. CLAIMED: both slots share the checked left-to-right bind route.
//! THE GAP: two concrete Rose/Box branch shapes, not every recursive carrier;
//! each pair uses a separate fresh environment with the same checked names.

use ken_elaborator::ElabEnv;
use ken_kernel::{convert, Context, Decl, Term};

const PRE: &str = r#"program capabilities FS APartial
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data Bag (a : Type) : Type where { Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a }
data Rose = Leaf | Node (Bag Rose)
"#;

fn pick_body(one: &str, join: &str) -> (ElabEnv, Term, Term) {
    let source = format!(
        "{PRE}proc pick (r : Rose) : Box visits [Console] = match r {{
  Leaf ↦ MkBox action ;
  Node b ↦ match b {{
    Empty ↦ MkBox action ;
    One x ↦ {one} ;
    Join xs ys ↦ {join} }} }}\n"
    );
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_file(&source)
        .unwrap_or_else(|error| panic!("recursive result fixture failed: {error:?}"));
    let (ty, body) = match env.env.lookup(env.globals["pick"]) {
        Some(Decl::Transparent { ty, body, .. }) => (ty.clone(), body.clone()),
        other => panic!("pick not a checked definition: {other:?}"),
    };
    (env, ty, body)
}

fn compare(label: &str, one_direct: &str, one_let: &str, join_direct: &str, join_let: &str) {
    let (env, ty, direct) = pick_body(one_direct, join_direct);
    let (_, let_ty, explicit) = pick_body(one_let, join_let);
    assert_eq!(ty, let_ty, "{label}: checked result types");
    assert!(
        direct == explicit || convert(&env.env, &Context::new(), &ty, &direct, &explicit),
        "{label}: scrutinee and let checked bodies must convert"
    );
}

#[test]
fn recursive_result_scrutinee_converts_to_its_let_twin() {
    compare(
        "s2 versus s3",
        "pick x",
        "pick x",
        "match (recursive result for xs) { MkBox f ↦ MkBox f }",
        "let s = (recursive result for xs) in match s { MkBox f ↦ MkBox f }",
    );
}

#[test]
fn structural_call_scrutinee_converts_to_its_let_twin() {
    compare(
        "s7 versus s6",
        "match pick x { MkBox f ↦ MkBox f }",
        "let s = pick x in match s { MkBox f ↦ MkBox f }",
        "MkBox action",
        "MkBox action",
    );
}
