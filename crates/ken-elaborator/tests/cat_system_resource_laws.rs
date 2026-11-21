//! Checked resource bracket equations at the prelude's real hidden-helper seam.

#[path = "support/resource_law_kernel_terms.rs"]
mod resource_law_kernel_terms;

use ken_elaborator::ElabEnv;
use ken_kernel::{convert_type, infer, Context, Decl, GlobalId, Term};
use resource_law_kernel_terms::Helpers;
use std::path::PathBuf;

fn law_type(env: &ElabEnv, name: &str) -> Term {
    let id = env.globals[name];
    assert!(
        !env.env.trusted_base().contains(&id),
        "the checked law `{name}` must not add trust"
    );
    match env.env.lookup(id) {
        Some(Decl::Transparent { ty, .. }) => ty.clone(),
        other => panic!("`{name}` must be a transparent, kernel-checked theorem: {other:?}"),
    }
}

fn conclusion(mut ty: &Term) -> &Term {
    while let Term::Pi(_, body) = ty {
        ty = body;
    }
    ty
}

fn equal_sides(ty: &Term, clean_pair: bool) -> (&Term, &Term) {
    let mut goal = conclusion(ty);
    if clean_pair {
        let Term::App(and_left, _) = goal else {
            panic!("clean settlement must be a two-equation conjunction: {goal:?}")
        };
        let Term::App(_, first) = and_left.as_ref() else {
            panic!("clean settlement must retain both conjuncts")
        };
        goal = first;
    }
    let Term::App(with_left, right) = goal else {
        panic!("expected an Eq with a right endpoint: {goal:?}")
    };
    let Term::App(_, left) = with_left.as_ref() else {
        panic!("expected an Eq with a left endpoint: {goal:?}")
    };
    (left, right)
}

fn head_id(mut term: &Term) -> GlobalId {
    while let Term::App(function, _) = term {
        term = function;
    }
    match term {
        Term::Const { id, .. } => *id,
        other => panic!("expected a checked global at the expression head, got {other:?}"),
    }
}

// The only law-type recovery for a private helper used by the kernel clients:
// its checked identity is the head of its owner's stated left endpoint.
fn checked_private_helper(env: &ElabEnv, private: &str, owner: &str, clean: bool) -> GlobalId {
    let ty = law_type(env, owner);
    let (left, _) = equal_sides(&ty, clean);
    let id = head_id(left);
    assert!(env.env.transparent_body(id).is_some());
    assert!(
        !env.globals.contains_key(private),
        "{private} must be fresh"
    );
    assert!(
        !env.globals.values().any(|visible| *visible == id),
        "the implementation helper `{private}` must remain hidden"
    );
    id
}

// Retain the pre-existing hiddenness assertion for the release continuation.
// Its checked id is never used to build a kernel client's expected term.
fn assert_release_continuation_hidden(env: &ElabEnv) {
    let ty = law_type(env, "resource_release_settles");
    let (_, right) = equal_sides(&ty, false);
    let Term::App(_, continuation) = right else {
        panic!("the released Vis must have a checked continuation")
    };
    let id = head_id(continuation);
    assert!(env.env.transparent_body(id).is_some());
    assert!(!env.globals.contains_key("private_resource_release_result"));
    assert!(!env.globals.values().any(|visible| *visible == id));
}

// A Not premise is itself a Pi, so the identity witness is found by the
// client's parameter count, not by shape.
fn identity_statement(term: Term, parameters: usize) -> Term {
    match term {
        Term::Pi(domain, body) if parameters > 0 => {
            Term::Pi(domain, Box::new(identity_statement(*body, parameters - 1)))
        }
        Term::Pi(domain, _) => *domain,
        other => panic!("Strict identity theorem must end in a witness: {other:?}"),
    }
}

// The pre-source Not has no source route, so the client abstracts it and the
// helper, and both checked identities are substituted back by the kernel.
fn check_strict_not_client(
    env: &mut ElabEnv,
    name: &str,
    source: &str,
    parameters: usize,
    private_helper: GlobalId,
    pre_source_not: GlobalId,
) {
    let law = law_type(env, name);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let client_ids = env
        .elaborate_isolated_file_strict_v1(
            &[root],
            &format!(
                "import Capability.System.Resource (ResourceBodyResult, ResourceBodyOk, ResourceBodyErr, \
            ResourceBracketResult, ResourceBracketReleaseError, \
            ResourceBracketBodyAndReleaseError)\n{source}"
            ),
        )
        .unwrap_or_else(|error| panic!("Strict Not client for `{name}`: {error:?}"));
    let client_id = client_ids.last().expect("Strict Not theorem").def_id;
    let Some(Decl::Transparent { ty, .. }) = env.env.lookup(client_id) else {
        panic!("Strict Not client must be a checked theorem")
    };
    let Term::Pi(_, rest) = ty else {
        panic!("Strict Not client must abstract the hidden helper")
    };
    let Term::Pi(_, rest) = ken_kernel::subst::subst0(rest, &Term::const_(private_helper, vec![]))
    else {
        panic!("Strict Not client must abstract the pre-source Not")
    };
    let statement = identity_statement(
        ken_kernel::subst::subst0(&rest, &Term::const_(pre_source_not, vec![])),
        parameters,
    );
    assert!(
        convert_type(&env.env, &Context::new(), &statement, &law),
        "the independently authored Not premise must convert to `{name}`"
    );
    assert_eq!(
        statement, law,
        "the Strict Not client must retain its exact independent proposition"
    );
}

fn without_witness(term: Term) -> Term {
    match term {
        Term::Pi(domain, body) => match *body {
            next @ Term::Pi(..) => Term::Pi(domain, Box::new(without_witness(next))),
            _ => *domain,
        },
        other => panic!("Strict identity theorem must end in a witness: {other:?}"),
    }
}

fn with_conjunction_head(ty: &Term, conjunction: GlobalId) -> Term {
    match ty {
        Term::Pi(domain, body) => Term::Pi(
            domain.clone(),
            Box::new(with_conjunction_head(body, conjunction)),
        ),
        Term::App(function, second) => match function.as_ref() {
            Term::App(former, first) => match former.as_ref() {
                Term::Const { level_args, .. } => Term::app(
                    Term::app(
                        Term::const_(conjunction, level_args.clone()),
                        (**first).clone(),
                    ),
                    (**second).clone(),
                ),
                other => panic!("conjunction is not a checked global: {other:?}"),
            },
            other => panic!("expected two independent conjuncts: {other:?}"),
        },
        other => panic!("expected a two-equation conjunction: {other:?}"),
    }
}

fn check_strict_conjunction_client(
    env: &mut ElabEnv,
    name: &str,
    source: &str,
    private_helper: GlobalId,
    conjunction: GlobalId,
) {
    let law = law_type(env, name);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let client_ids = env
        .elaborate_isolated_file_strict_v1(
            &[root],
            &format!(
                "import Core.Logic.Conjunction (And)\n\
          import Capability.System.Resource (ResourceBodyResult, ResourceBodyOk, ResourceBodyErr, \
            ResourceBracketResult, ResourceBracketOk, ResourceBracketBodyError)\n{source}"
            ),
        )
        .unwrap_or_else(|error| panic!("Strict conjunction client for `{name}`: {error:?}"));
    let client_id = client_ids
        .last()
        .expect("Strict conjunction theorem")
        .def_id;
    let Some(Decl::Transparent { ty, .. }) = env.env.lookup(client_id) else {
        panic!("Strict conjunction client must be a checked theorem")
    };
    let Term::Pi(_, rest) = ty else {
        panic!("Strict conjunction client must abstract the hidden helper")
    };
    let statement = without_witness(ken_kernel::subst::subst0(
        rest,
        &Term::const_(private_helper, vec![]),
    ));
    assert!(
        convert_type(&env.env, &Context::new(), &statement, &law),
        "the independently authored conjunction must convert to `{name}`"
    );
    assert_eq!(
        statement,
        with_conjunction_head(&law, conjunction),
        "both exact conjuncts must survive the Strict identity witness"
    );
}

/// Promise class: normative compatibility vector for the Resource bracket.
/// MEASURED: the two clean-settlement and the two Not-premise propositions
/// have independently checked Strict identity-theorem clients, each equal to
/// its law once the abstracted checked identities are substituted. Each of the
/// eleven effect-law types is kernel-convertible to an independently
/// hand-built, well-formed kernel proposition, while its named non-erasable
/// perturbation does not convert.
/// CLAIMED: the four settlement outcomes and three bracket sequences remain
/// exact without exposing implementation helpers or adding trusted claims.
/// THE GAP: a private helper identity is recovered from its owner's checked
/// theorem left head. The pre-source Not has no source route, so its identity
/// is read from the prelude registry, not from a law. The release continuation
/// is authored here as a lambda over the checked settlement helper; its law
/// RHS is inspected only by the old hiddenness assertion, never copied into an
/// expected term. Separate production mutations exercise the laws' reach to
/// real prelude definitions.
/// No expected effect term is copied from a law or elaborated from source text.
#[test]
fn resource_laws_have_exact_independent_checked_clients() {
    let mut env = ElabEnv::empty().expect("Strict Resource laws prelude");
    let trusted_before_clients = env.env.trusted_base();
    // Read before any catalog load can write a bare spelling.
    let pre_source_not = env.globals["Not"];
    for private in ["resource_refl", "private_resource_release_result"] {
        assert!(
            !env.globals.contains_key(private),
            "`{private}` must remain hidden"
        );
    }
    let settle = checked_private_helper(
        &env,
        "resource_settle_result_for",
        "resource_settle_body_ok_clean",
        true,
    );
    let after_open = checked_private_helper(
        &env,
        "private_with_resource_after_open",
        "resource_after_open_error",
        false,
    );
    let after_buffer = checked_private_helper(
        &env,
        "private_with_buffer_after_allocate",
        "buffer_after_allocate_error",
        false,
    );
    let after_mapping = checked_private_helper(
        &env,
        "private_with_mapping_after_allocate",
        "mapping_after_allocate_error",
        false,
    );
    let release_if_live =
        checked_private_helper(&env, "release_if_live", "resource_release_settles", false);
    assert_release_continuation_hidden(&env);
    for (name, id) in [
        ("PrivateFsOpen", env.prelude_env.private_fs_open_id),
        (
            "PrivateBufferAllocate",
            env.prelude_env.private_buffer_allocate_id,
        ),
        (
            "PrivateMappingAllocate",
            env.prelude_env.private_mapping_allocate_id,
        ),
        (
            "PrivateMappingAcquireFile",
            env.prelude_env.private_mapping_acquire_file_id,
        ),
        (
            "PrivateResourceRelease",
            env.prelude_env.private_resource_release_id,
        ),
        (
            "PrivateBracketRelease",
            env.prelude_env.private_bracket_release_id,
        ),
        (
            "PrivateBufferHandle",
            env.prelude_env.private_buffer_handle_id,
        ),
    ] {
        assert!(!env.globals.contains_key(name));
        assert!(env.env.constructor(id).is_some());
        assert!(!env.globals.values().any(|visible| *visible == id));
    }
    let mapping_handle_id = env.globals["MappingHandle"];
    let constructors = &env
        .env
        .inductive(mapping_handle_id)
        .expect("checked MappingHandle")
        .constructors;
    assert_eq!(constructors.len(), 1);
    let private_mapping_handle = constructors[0].id;
    assert!(!env.globals.contains_key("PrivateMappingHandle"));
    assert!(!env.globals.values().any(|id| *id == private_mapping_handle));
    let helpers = Helpers {
        after_open,
        after_buffer,
        after_mapping,
        release_if_live,
        settle_result: settle,
        mapping_handle: private_mapping_handle,
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    for module in [
        "Core.Logic.Conjunction",
        "Capability.System.Resource",
        "Capability.System.Buffer",
        "Capability.System.Mapping",
    ] {
        env.elaborate_module_from_roots_strict(&[root.clone()], module)
            .unwrap_or_else(|error| panic!("checked `{module}` Strict route: {error:?}"));
    }
    let conjunction = env.globals["Core.Logic.Conjunction.and"];
    let identity = |header: &str, proposition: &str| {
        format!("{header}\n(h : {proposition}) : {proposition} = h")
    };
    check_strict_conjunction_client(
        &mut env,
        "resource_settle_body_ok_clean",
        &identity(
            r#"theorem client_resource_settle_body_ok_clean
          (resource_settle_result_for : (o : Type) → (e : Type) → (r : Type)
            → ResourceBodyResult e r → Result ResourceError Unit
            → Result o (ResourceBracketResult e r))
          (o : Type) (e : Type) (r : Type) (value : r)"#,
            r#"And
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyOk e r value)
                (Ok ResourceError Unit MkUnit))
              (Ok o (ResourceBracketResult e r) (ResourceBracketOk e r value)))
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyOk e r value)
                (Err ResourceError Unit ResourceError.Closed))
              (Ok o (ResourceBracketResult e r) (ResourceBracketOk e r value)))"#,
        ),
        settle,
        conjunction,
    );
    check_strict_conjunction_client(
        &mut env,
        "resource_settle_body_error_clean",
        &identity(
            r#"theorem client_resource_settle_body_error_clean
          (resource_settle_result_for : (o : Type) → (e : Type) → (r : Type)
            → ResourceBodyResult e r → Result ResourceError Unit
            → Result o (ResourceBracketResult e r))
          (o : Type) (e : Type) (r : Type) (body_error : e)"#,
            r#"And
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyErr e r body_error)
                (Ok ResourceError Unit MkUnit))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketBodyError e r body_error)))
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyErr e r body_error)
                (Err ResourceError Unit ResourceError.Closed))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketBodyError e r body_error)))"#,
        ),
        settle,
        conjunction,
    );

    let not_header = |theorem: &str, parameters: &str| {
        format!(
            "theorem {theorem}\n\
          (resource_settle_result_for : (o : Type) → (e : Type) → (r : Type)\n\
            → ResourceBodyResult e r → Result ResourceError Unit\n\
            → Result o (ResourceBracketResult e r))\n\
          (Not : Omega → Omega)\n\
          {parameters}"
        )
    };
    check_strict_not_client(
        &mut env,
        "resource_settle_body_ok_release_error",
        &identity(
            &not_header(
                "client_resource_settle_body_ok_release_error",
                "(o : Type) (e : Type) (r : Type) (value : r) (release_error : ResourceError)",
            ),
            r#"Not (Equal ResourceError release_error ResourceError.Closed)
            → Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyOk e r value)
                (Err ResourceError Unit release_error))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketReleaseError e r release_error))"#,
        ),
        5,
        settle,
        pre_source_not,
    );
    check_strict_not_client(
        &mut env,
        "resource_settle_body_error_release_error",
        &identity(
            &not_header(
                "client_resource_settle_body_error_release_error",
                "(o : Type) (e : Type) (r : Type) (body_error : e) (release_error : ResourceError)",
            ),
            r#"Not (Equal ResourceError release_error ResourceError.Closed)
            → Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyErr e r body_error)
                (Err ResourceError Unit release_error))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketBodyAndReleaseError e r body_error release_error))"#,
        ),
        5,
        settle,
        pre_source_not,
    );

    let names = [
        "resource_with_acquire",
        "resource_after_open_error",
        "resource_after_open_success",
        "buffer_with_acquire",
        "buffer_after_allocate_error",
        "buffer_after_allocate_success",
        "mapping_with_anonymous_acquire",
        "mapping_with_file_acquire",
        "mapping_after_allocate_error",
        "mapping_after_allocate_success",
        "resource_release_settles",
    ];
    for name in names {
        let law = law_type(&env, name);
        let (expected, perturbed, perturbation) =
            resource_law_kernel_terms::expected(&env, helpers, name);
        let sort = infer(&env.env, &Context::new(), &expected).unwrap_or_else(|error| {
            panic!("`{name}` independently authored proposition is ill-formed: {error:?}")
        });
        assert!(
            matches!(sort, Term::Type(_) | Term::Omega(_)),
            "`{name}` expected proposition does not infer to a sort: {sort:?}"
        );
        assert!(
            convert_type(&env.env, &Context::new(), &law, &expected),
            "`{name}` must convert to its independently authored proposition"
        );
        eprintln!(
            "kernel client `{name}`: expected == law type: {}",
            expected == law
        );
        if name != "resource_release_settles" {
            assert_eq!(
                expected, law,
                "this hand-built proposition matches the checked law exactly"
            );
        }
        let perturbed_sort = infer(&env.env, &Context::new(), &perturbed).unwrap_or_else(|error| {
            panic!("`{name}` perturbation `{perturbation}` must itself be well-formed: {error:?}")
        });
        assert!(
            matches!(perturbed_sort, Term::Type(_) | Term::Omega(_)),
            "`{name}` perturbation `{perturbation}` does not infer to a sort: {perturbed_sort:?}"
        );
        assert!(
            !convert_type(&env.env, &Context::new(), &law, &perturbed),
            "`{name}` perturbation `{perturbation}` must not convert to its checked law"
        );
    }
    assert_eq!(
        env.env.trusted_base(),
        trusted_before_clients,
        "Strict clients must not introduce an axiom or primitive"
    );
}
