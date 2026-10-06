//! CAT-TRANSPORT-COMBINATOR-CONSOLIDATION public transport and reuse controls.
//!
//! Promise class: durable invariants. Checked transport is publicly selectable,
//! a reverse-direction use is rejected at the typed equality boundary, and
//! the eight consumers retain the provider's identities without local copies.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, KernelError, Term};

const TRANSPORT: &str = "Core.Logic.Transport";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

const SUBST_CLIENT: &str = "import Core.Logic.Transport (subst)\n\
    data IndexedNat : Nat → Type where {\n\
      IndexedZero : IndexedNat Zero;\n\
      IndexedSuc : (n : Nat) → IndexedNat n → IndexedNat (Suc n)\n\
    }\n\
    fn carry_index (x : Nat) (y : Nat) (p : Eq Nat x y) (value : IndexedNat x)\n\
      : IndexedNat y = subst Nat x y IndexedNat p value";

/// MEASURED: a fresh roots-loaded Transport plus a client importing only
/// `subst` kernel-check an open Nat-indexed family and add no trusted globals.
/// CLAIMED: the existing `J` combinator is a public, checked family transport.
/// THE GAP: the swapped-direction sibling must fail by type, not visibility.
#[test]
fn public_subst_transports_an_open_nat_indexed_family_without_new_trust() {
    let mut env = ElabEnv::new().expect("fresh environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], TRANSPORT)
        .expect("Transport must roots-load");
    env.elaborate_file(SUBST_CLIENT)
        .expect("public subst must transport an open Nat-indexed family");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Transport and its client must add no trust");
}

/// MEASURED: the one-axis sibling uses `p : Eq Nat x y` to pass `IndexedNat y`
/// as the source and demand `IndexedNat x`, while the call is unchanged.
/// CLAIMED: public `subst` transports only from the left to the right endpoint.
/// THE GAP: the rejection must reach KernelRejected(TypeMismatch), not syntax
/// or name resolution; the positive test above checks those earlier layers.
#[test]
fn public_subst_rejects_swapped_endpoints_at_the_typed_boundary() {
    let source = "value : IndexedNat x)";
    let result = ": IndexedNat y = subst";
    assert_eq!(SUBST_CLIENT.matches(source).count(), 1);
    assert_eq!(SUBST_CLIENT.matches(result).count(), 1);
    let mutant = SUBST_CLIENT
        .replacen(source, "value : IndexedNat y)", 1)
        .replacen(result, ": IndexedNat x = subst", 1);
    let mut env = ElabEnv::new().expect("fresh environment");
    env.elaborate_module_from_roots(&[catalog_root()], TRANSPORT)
        .expect("Transport must roots-load");
    match env.elaborate_file(&mutant) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        Err(other) => panic!("reversed transport must fail at typed equality: {other:?}"),
        Ok(_) => panic!("a forward equality must not transport backwards"),
    }
}

/// MEASURED: after the real roots loader closes each package, none of the
/// fourteen retired combinator copies or Posix's unused cong0 has an owned
/// kernel artifact. CLAIMED: no migrated package retains those private names.
/// THE GAP: provider-GlobalId occurrence controls separately check that
/// retained consumers use Transport rather than merely dropping local names.
#[test]
fn all_eight_packages_retire_the_fifteen_local_combinator_artifacts() {
    let mut env = ElabEnv::new().expect("fresh environment");
    for (module, retired) in [
        ("Algorithm.Numeric.Gcd", &["subst"][..]),
        (
            "Algorithm.Searching.OrderedSearch",
            &["search_sym", "search_trans"],
        ),
        (
            "Capability.Filesystem.Path.Posix",
            &[
                "path_equal_sym",
                "path_equal_trans",
                "path_equal_cong",
                "path_equal_cong0",
            ],
        ),
        (
            "Application.Configuration.Decoder",
            &["env_config_sym", "env_config_trans"],
        ),
        ("Application.Input.Schema", &["schema_sym"]),
        ("Capability.Parsing.Decoder", &["decoder_equal_chain"]),
        ("Data.Collections.Deque", &["deque_cong"]),
        (
            "Tooling.Verification.FoKripke",
            &["fok_cong", "fok_eq_sym", "fok_eq_trans"],
        ),
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], module)
            .unwrap_or_else(|err| panic!("{module} must roots-load: {err:?}"));
        for name in retired {
            assert!(
                !env.globals.contains_key(&format!("{module}.{name}")),
                "{module} must not mint its retired local {name}"
            );
        }
    }
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions(child, target)),
    }
}

/// MEASURED: preloaded Transport `GlobalId`s remain fixed after eight real
/// roots loads, and a retained checked consumer in each package refers to
/// each newly selected combinator's exact `GlobalId` in its transparent body.
/// CLAIMED: the surviving proofs reuse Transport, not renamed private copies.
/// THE GAP: the absence sibling covers the retired declaration identities;
/// this body walk covers direct canonical use at each retained consumer seam.
#[test]
fn migrated_proof_bodies_use_preloaded_transport_global_identities() {
    let mut env = ElabEnv::new().expect("fresh environment");
    env.elaborate_module_from_roots(&[catalog_root()], TRANSPORT)
        .expect("Transport provider must roots-load before its consumers");
    let providers = ["subst", "cong", "sym", "trans"]
        .into_iter()
        .map(|name| (name, env.globals[&format!("{TRANSPORT}.{name}")]))
        .collect::<std::collections::BTreeMap<_, _>>();

    for (module, witness_bodies) in [
        ("Algorithm.Numeric.Gcd", &[("subst_divides", "subst")][..]),
        (
            "Algorithm.Searching.OrderedSearch",
            &[
                ("boolean_contradiction", "sym"),
                ("boolean_contradiction", "trans"),
            ],
        ),
        (
            "Capability.Filesystem.Path.Posix",
            &[
                ("path_split_no_slash_end", "cong"),
                ("path_split_no_slash_end", "sym"),
                ("path_split_no_slash_end", "trans"),
            ],
        ),
        (
            "Application.Configuration.Decoder",
            &[
                ("env_config_required_check_lookup", "sym"),
                ("env_config_required_values", "trans"),
                ("env_config_required_values", "cong"),
            ],
        ),
        (
            "Application.Input.Schema",
            &[("schema_validate_fields::accepted_tail_invalid", "sym")],
        ),
        (
            "Capability.Parsing.Decoder",
            &[("decoder_many_decoded_result_matches", "trans")],
        ),
        (
            "Data.Collections.Deque",
            &[("deque_append_snoc_assoc", "cong")],
        ),
        (
            "Tooling.Verification.FoKripke",
            &[
                ("fok_nat_eq_sound", "cong"),
                ("fok_absurd_right_intro", "sym"),
                ("fok_absurd_right_intro", "trans"),
            ],
        ),
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], module)
            .unwrap_or_else(|err| panic!("{module} must roots-load: {err:?}"));
        for (body_name, combinator) in witness_bodies {
            let provider_name = format!("{TRANSPORT}.{combinator}");
            let provider = providers[combinator];
            assert_eq!(
                env.globals[&provider_name], provider,
                "{module} must not replace preloaded Transport.{combinator}"
            );
            let consumer_name = format!("{module}.{body_name}");
            let consumer = *env
                .globals
                .get(&consumer_name)
                .unwrap_or_else(|| panic!("missing checked consumer {consumer_name}"));
            let (_, body) = env
                .env
                .transparent_body(consumer)
                .unwrap_or_else(|| panic!("{consumer_name} must remain a checked definition"));
            assert!(
                term_mentions(&body, provider),
                "{consumer_name} must use canonical Transport.{combinator}"
            );
        }
    }
}
