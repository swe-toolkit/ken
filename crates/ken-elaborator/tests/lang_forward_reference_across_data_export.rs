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
