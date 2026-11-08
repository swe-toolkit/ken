//! Spec 36 §2.4: each performed scrutinee slot has the same checked core
//! body as its explicit sequenced-let twin. Promise class: durable invariant.
//! MEASURED: each of four pairs elaborates both forms in one environment and
//! produces identical body terms, in checked and inferred contexts with nested outer
//! references. CLAIMED: scrutinee-slot elaboration reuses the let denotation.
//! THE GAP: literal occurrences have distinct checked global identities, so
//! the pairs hoist their shared message to one checked const; this does not
//! assert that separate literal occurrences are definitionally identical.

use ken_elaborator::ElabEnv;
use ken_kernel::{convert, Context, Decl};

// Inline Ken input: adding an isolated .ken file would widen the strict
// isolated-source census without changing the property this test measures.
const PRE: &str = r#"program capabilities FS APartial
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data PBox = MkPBox (Unit -> HostIO APartial Unit)
data Flag = On | Off
record Slot { bx : Box }
def Callback = Unit ->[Console] HostIO APartial Unit
data Bag (a : Type) : Type where { Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a }
data Rose = Leaf | Node (Bag Rose)
proc pick (r : Rose) : Box visits [Console] = match r {
  Leaf ↦ MkBox action ;
  Node b ↦ match b {
    Empty ↦ MkBox action ;
    One x ↦ pick x ;
    Join xs ys ↦ MkBox action } }
"#;

fn verdict(pair: &str, decls: &str, slot: &str, twin: &str) -> &'static str {
    let mut env = ElabEnv::new().expect("checked prelude");
    let source = format!("{PRE}{decls}");
    if let Err(error) = env.elaborate_file(&source) {
        println!("D02CORE {pair} ELAB-ERROR {error:?}");
        return "ELAB-ERROR";
    }
    let body = |name: &str| match env.env.lookup(env.globals[name]) {
        Some(Decl::Transparent { ty, body, .. }) => (ty.clone(), body.clone()),
        other => panic!("{pair}: {name} is not a checked definition: {other:?}"),
    };
    let (slot_ty, slot_body) = body(slot);
    let (_, twin_body) = body(twin);
    let verdict = if slot_body == twin_body {
        "IDENTICAL"
    } else if convert(&env.env, &Context::new(), &slot_ty, &slot_body, &twin_body) {
        "CONVERTIBLE"
    } else {
        "DIFFERENT"
    };
    println!("D02CORE {pair} {verdict}");
    if verdict != "IDENTICAL" {
        println!("D02CORE {pair} slot {slot_body:?}");
        println!("D02CORE {pair} twin {twin_body:?}");
    }
    verdict
}

#[test]
fn slot_bodies_against_let_twins() {
    let use_g =
        "match bx { MkBox g ↦ let ignored = g MkUnit in host_program APartial (print_line msg_b) }";
    let pairs = [
        (
            "P1-callee-check",
            "proc usep (r : Rose) : Box visits [Console] = match pick r { MkBox f ↦ MkBox f }\n\
             proc uset (r : Rose) : Box visits [Console] = let s = pick r in match s { MkBox f ↦ MkBox f }\n"
                .to_owned(),
        ),
        (
            "P2-nested-outer-ref",
            "proc pickq (u : Unit) : Box visits [Console] = let p = print_line \"q\" in MkBox quiet\n\
             proc usep (r : Rose) : Box visits [Console] = match pick r { MkBox f ↦ match pickq MkUnit { MkBox g ↦ match r { Leaf ↦ MkBox g ; Node b ↦ MkBox f } } }\n\
             proc uset (r : Rose) : Box visits [Console] = let s = pick r in match s { MkBox f ↦ let t = pickq MkUnit in match t { MkBox g ↦ match r { Leaf ↦ MkBox g ; Node b ↦ MkBox f } } }\n"
                .to_owned(),
        ),
        (
            "P3-main-check",
            "const msg_b : String = \"b\"\n\
             proc usep (u : Unit) : HostIO APartial ExitCode visits [Console] = match pick (Node (One Rose Leaf)) { MkBox f ↦ let ignored = f MkUnit in host_program APartial (print_line msg_b) }\n\
             proc uset (u : Unit) : HostIO APartial ExitCode visits [Console] = let s = pick (Node (One Rose Leaf)) in match s { MkBox f ↦ let ignored = f MkUnit in host_program APartial (print_line msg_b) }\n"
                .to_owned(),
        ),
        (
            "P4-main-infer",
            format!(
                "const msg_b : String = \"b\"\n\
                 proc usep (u : Unit) : HostIO APartial ExitCode visits [Console] = let bx = match pick (Node (One Rose Leaf)) {{ MkBox f ↦ MkBox f }} in {use_g}\n\
                 proc uset (u : Unit) : HostIO APartial ExitCode visits [Console] = let bx = (let s = pick (Node (One Rose Leaf)) in match s {{ MkBox f ↦ MkBox f }}) in {use_g}\n"
            ),
        ),
    ];
    let verdicts: Vec<_> = pairs
        .iter()
        .map(|(pair, decls)| (*pair, verdict(pair, decls, "usep", "uset")))
        .collect();
    assert!(
        verdicts.iter().all(|(_, verdict)| *verdict == "IDENTICAL"),
        "{verdicts:?}"
    );
}
