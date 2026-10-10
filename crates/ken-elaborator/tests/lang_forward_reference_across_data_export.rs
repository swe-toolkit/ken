//! Spec 33 §8.4 forward references across declarations within one segment.
//! Promise class: durable invariant. The observed boundary is the checked
//! canonical identity and rejection of genuine cycles/unbound names, not a
//! snapshot of elaboration order or source text.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

fn mentions_global(term: &Term, id: GlobalId) -> bool {
    match term {
        Term::Const { id: actual, .. }
        | Term::Constructor { id: actual, .. }
        | Term::IndFormer { id: actual, .. }
            if *actual == id =>
        {
            true
        }
        _ => term
            .children()
            .into_iter()
            .any(|child| mentions_global(child, id)),
    }
}

fn checked(source: &str) -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    env
}

fn transparent_body<'a>(env: &'a ElabEnv, name: &str) -> Term {
    env.env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("{name} should have a checked body"))
        .1
}

#[test]
fn function_ref_across_data_uses_later_sibling_checked_identity() {
    let env =
        checked("module M { fn a (x : Int) : Int = b x data D = MkD fn b (x : Int) : Int = x }");
    let body = transparent_body(&env, "M.a");
    assert!(mentions_global(&body, env.globals["M.b"]));
}

#[test]
fn function_ref_across_export_uses_later_sibling_checked_identity() {
    let env = checked("module M { fn a (x : Int) : Int = b x export a fn b (x : Int) : Int = x }");
    let body = transparent_body(&env, "M.a");
    assert!(mentions_global(&body, env.globals["M.b"]));
}

#[test]
fn earlier_type_names_later_data_family() {
    let env = checked("module M { fn id_d (x : D) : D = x data D = MkD }");
    let Some(Decl::Transparent { ty, .. }) = env.env.lookup(env.globals["M.id_d"]) else {
        panic!("checked definition must be transparent");
    };
    assert!(mentions_global(ty, env.globals["M.D"]));
}

#[test]
fn export_before_later_definition_publishes_checked_identity() {
    let env = checked("module M { fn a (x : Int) : Int = b x export b fn b (x : Int) : Int = x } import M (b) fn consumer (x : Int) : Int = b x");
    let earlier = transparent_body(&env, "M.a");
    let client = transparent_body(&env, "consumer");
    assert!(mentions_global(&earlier, env.globals["M.b"]));
    assert!(mentions_global(&client, env.globals["M.b"]));
}

#[test]
fn earlier_definition_uses_constructor_owned_by_later_data() {
    let env = checked("module M { const a : D = C data D = C }");
    assert!(
        matches!(transparent_body(&env, "M.a"), Term::Constructor { id, .. } if id == env.globals["M.C"])
    );
}

/// Promise class: durable invariant (spec 33 §8.4).
/// MEASURED: a sole qualified constructor selector resolves to the later
/// data family's checked constructor identity at both module and file root.
/// CLAIMED: this selector creates a dependency edge to its own family.
/// THE GAP: other qualified import and privacy routes have their own gates.
#[test]
fn earlier_definition_names_later_family_only_by_qualified_selector() {
    for (source, f, red) in [
        (
            "module M { fn f (u : Nat) : Nat = match Colour.Red { Colour.Red |-> Zero ; Colour.Blue |-> u } data Colour = Red | Blue }",
            "M.f",
            "M.Red",
        ),
        (
            "fn f (u : Nat) : Nat = match Colour.Red { Colour.Red |-> Zero ; Colour.Blue |-> u }\ndata Colour = Red | Blue",
            "f",
            "Red",
        ),
    ] {
        let env = checked(source);
        assert!(mentions_global(&transparent_body(&env, f), env.globals[red]));
    }
}

#[test]
fn earlier_let_annotation_names_later_data() {
    checked("module M { fn a (x : Int) : Int = let f : D -> Int = λd. x in x data D = MkD }");
}

#[test]
fn earlier_ascription_names_later_data() {
    checked("module M { fn a (x : Int) : Int = let f = ((λd. x) : D -> Int) in x data D = MkD }");
}

#[test]
fn earlier_pi_domain_names_later_data() {
    let env = checked("module M { const T : Type = (d : D) -> Int data D = MkD }");
    assert!(mentions_global(&transparent_body(&env, "M.T"), env.globals["M.D"]));
}

/// Promise class: durable invariant (spec 33 §8.4). Both a separated and an
/// adjacent later prop are checked before a theorem that names their intros.
/// MEASURED: the theorem body contains the two kernel-checked intro IDs, each
/// with its own checked prop family as its type. CLAIMED: P.pi orders t after P
/// without relying on textual order or mistaking Q.qi for P.pi. THE GAP:
/// imported and hidden intro paths have independent namespace gates.
#[test]
fn earlier_theorem_uses_later_prop_checked_intro_identity() {
    for data_sibling in ["data D = MkD", ""] {
        let source = format!(
            "module M {{ prop Q : Omega where {{ qi : Q }} \
             theorem t : Q = let h = P.pi in Q.qi \
             {data_sibling} prop P : Omega where {{ pi : P }} }}"
        );
        let env = checked(&source);
        let body = transparent_body(&env, "M.t");
        let p = env.globals["M.P"];
        let q = env.globals["M.Q"];
        let pi = env.globals["M.P.pi"];
        let qi = env.globals["M.Q.qi"];
        assert_ne!(p, q);
        assert_ne!(pi, qi);
        for (family, intro) in [(p, pi), (q, qi)] {
            assert!(matches!(
                env.env.lookup(intro),
                Some(Decl::Transparent { .. })
            ));
            assert_eq!(
                env.env.const_type(intro).unwrap().1,
                Term::const_(family, vec![])
            );
            assert!(
                mentions_global(&body, intro),
                "theorem must use its checked intro"
            );
        }
    }
}

/// Promise class: durable invariant. An ordinary `pi` binding and the prop
/// intro `P.pi` have separate dependency edges and separate checked IDs.
/// MEASURED: a forward consumer selects the checked ordinary pi, whose body
/// still selects its own later dependency; t selects both prop intros.
/// CLAIMED: a prop intro never steals the bare module binding or its edges.
/// THE GAP: other source declarations and imports have separate scope gates.
#[test]
fn prop_intro_keeps_an_ordinary_pi_nodes_dependency_edges() {
    let env = checked(
        "module M { const consumer : Int = pi \
         const pi : Int = later \
         theorem t : Q = let h = P.pi in Q.qi \
         prop Q : Omega where { qi : Q } \
         prop P : Omega where { pi : P } \
         const later : Int = 0 }",
    );
    let pi = env.globals["M.pi"];
    let p_intro = env.globals["M.P.pi"];
    assert_ne!(pi, p_intro);
    assert!(mentions_global(&transparent_body(&env, "M.consumer"), pi));
    assert!(mentions_global(
        &transparent_body(&env, "M.pi"),
        env.globals["M.later"]
    ));
    assert!(mentions_global(&transparent_body(&env, "M.t"), p_intro));
}

/// Promise class: durable invariant. The neighboring dependency routes are
/// unchanged by making a prop's family-qualified intro available to the graph.
#[test]
fn prop_intro_forward_edge_preserves_neighboring_forward_routes() {
    for source in [
        "module M { prop Q : Omega where { qi : Q } prop P : Omega where { pi : P } theorem t : Q = let h = P.pi in Q.qi data D = MkD }",
        "module M { const a : Int = let h = C in 0 data E = MkE data D = C }",
        "module M { prop Q : Omega where { qi : Q } theorem t (p : P) : Q = Q.qi prop P : Omega where { pi : P } }",
        "module M { fn f (x : Int) : Int = x theorem t (x : Int) : Equal Int (f x) x = f::p x proof p for f (x : Int) : Equal Int (f x) x = Refl }",
        "module M { fn f (x : A) : A = x def A = Int }",
    ] {
        checked(source);
    }
    let dotted = checked(
        "module M { pub fn f (x : Int) : Int = x } \
         proof p for M.f (x : Int) : Equal Int (M.f x) x = Refl",
    );
    assert!(dotted.env.lookup(dotted.globals["M.f::p"]).is_some());
}

#[test]
fn type_declaration_cannot_share_cycle_with_a_definition() {
    let mut env = ElabEnv::new().expect("base environment");
    let error = env
        .elaborate_file("module M { data D = MkD T const T : Type = D }")
        .expect_err("type and definition cycle must reject at the SCC gate");
    match error {
        ElabError::TypeMismatch { reason, .. } => assert_eq!(
            reason,
            "a type declaration cannot share a dependency cycle with another declaration"
        ),
        other => panic!("expected type-declaration cycle refusal, got {other:?}"),
    }
}

#[test]
fn ordinary_data_self_recursion_is_not_a_mixed_cycle() {
    checked("module M { data Chain = End | Next Chain }");
}

#[test]
fn existing_cycle_and_missing_name_fences_remain() {
    let mut env = ElabEnv::new().expect("base environment");
    let cycle = env
        .elaborate_file(
            "module M { fn a (x : Int) : Int = b x data D = MkD fn b (x : Int) : Int = a x }",
        )
        .expect_err("a non-decreasing cycle must fail SCT");
    assert!(
        matches!(
            cycle,
            ElabError::KernelRejected {
                error: ken_kernel::KernelError::NotTerminating(_),
                ..
            }
        ),
        "the cycle must reach SCT, not an unrelated earlier error: {cycle:?}"
    );
    let mut env = ElabEnv::new().expect("base environment");
    let unbound = env
        .elaborate_file("module M { fn a (x : Int) : Int = b x data D = MkD }")
        .expect_err("an absent sibling must not resolve");
    assert!(matches!(unbound, ElabError::UnresolvedCon { name, .. } if name == "b"));
    checked("module M { fn a (x : Int) : Int = b x fn b (x : Int) : Int = x }");
}
