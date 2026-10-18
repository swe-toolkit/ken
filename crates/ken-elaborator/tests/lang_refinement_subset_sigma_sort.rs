//! AC-S: type-position level links through checked subset-Σ domains.
//! Non-cumulative universes are governed by spec 12 §3 and 39 §2.

use ken_elaborator::ElabEnv;
use ken_kernel::{Context, Decl, Term};

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("const Big : Type 2 = Type 1")
        .expect("transparent type-universe alias");
    env
}

fn contains_pair(term: &Term) -> bool {
    matches!(term, Term::Pair(..)) || term.children().iter().any(|child| contains_pair(child))
}

fn admits_with_checked_core(source: &str, obligations: usize, needs_pair: bool) {
    // The measured prelude peak is 1,982,464 bytes; an 8 MiB thread leaves
    // over 6 MiB for the elaboration and kernel-checking telescope.
    let source = source.to_owned();
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let mut env = env();
            let before = env.env.trusted_base();
            let result = env
                .elaborate_decl_v1(&source)
                .unwrap_or_else(|error| panic!("{source}: {error:?}"));
            assert_eq!(result.obligations.len(), obligations, "{source}");
            assert!(
                result
                    .obligations
                    .iter()
                    .all(|obligation| !env.is_open_hole(obligation.hole_id)),
                "a Top proof must not add a trusted assumption"
            );
            assert_eq!(env.env.trusted_base(), before, "{source}: no trust delta");
            let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
                panic!("{source}: checked function definition")
            };
            assert_eq!(
                contains_pair(ty),
                needs_pair,
                "a carrier-typed argument requires an emitted checked pair: {ty:?}"
            );
            ken_kernel::check(&env.env, &Context::new(), body, ty)
                .expect("complete function remains kernel-checkable");
        })
        .expect("stated type-position stack")
        .join()
        .expect("type-position thread");
}

/// Promise class: durable introduction and sort-link invariant.
/// MEASURED: the Big carrier is one universe above the bare `Type` binder;
/// the type application checks only when it emits the subset pair and links
/// the carrier's sort before defaulting the argument's level meta.
/// CLAIMED: the written application is checked, not an erased type argument.
/// THE GAP: this row alone does not reach the Σ/Σ sort walk; the next row does.
#[test]
fn carrier_typed_argument_links_subset_domain_and_emits_pair() {
    admits_with_checked_core(
        "fn k (F : ({x : Big | Top}) -> Type) (a : Type) (o : F a) (n : Int) : Int = n",
        1,
        true,
    );
}

/// Promise class: durable same-shape subset level-link invariant.
/// MEASURED: both domain and argument have Σ heads, and the Type carrier's
/// level is open until the nested sort walk links it to Big's Type 1.
/// CLAIMED: a Σ/Σ walk does not fall through to the nonrecursive unifier.
/// THE GAP: a bare carrier uses the separate introduction row above.
#[test]
fn already_refined_argument_links_nested_sigma_carriers() {
    admits_with_checked_core(
        "fn k (F : ({x : Big | Top}) -> Type) (a : {x : Type | Top}) (o : F a) (n : Int) : Int = n",
        0,
        false,
    );
}

#[test]
fn explicit_big_carrier_admits_without_inferred_level() {
    admits_with_checked_core(
        "fn k (F : ({x : Big | Top}) -> Type) (a : Type 1) (o : F a) (n : Int) : Int = n",
        1,
        true,
    );
}

/// Promise class: durable non-cumulativity refusal (spec 12 §3).
/// A fixed Type 0 may not silently inhabit the Type 1 carrier, even when
/// the carrier's predicate is Top and a surface pair could be constructed.
#[test]
fn fixed_zero_cannot_enter_big_subset() {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let mut env = env();
            let before = env.env.next_global_id();
            let source =
                "fn k (F : ({x : Big | Top}) -> Type) (a : Type 0) (o : F a) (n : Int) : Int = n";
            env.elaborate_decl_v1(source)
                .expect_err("non-cumulative fixed Type 0 must refuse");
            assert_eq!(env.env.next_global_id(), before, "refusal must roll back");
            assert!(
                !env.globals.contains_key("k"),
                "refused declaration is unpublished"
            );
        })
        .expect("stated type-position stack")
        .join()
        .expect("type-position thread");
}

#[test]
fn plain_former_remains_admitted_without_subset() {
    admits_with_checked_core(
        "fn k (F : Big -> Type) (a : Type) (o : F a) (n : Int) : Int = n",
        0,
        false,
    );
}
