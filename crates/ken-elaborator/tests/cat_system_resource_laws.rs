//! Checked resource bracket equations at the prelude's real hidden-helper seam.

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, GlobalId, Term};

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

fn expose_private_source_for_client(env: &mut ElabEnv, private: &str, owner: &str, clean: bool) {
    let ty = law_type(env, owner);
    let (left, _) = equal_sides(&ty, clean);
    let id = head_id(left);
    assert!(env.env.transparent_body(id).is_some());
    assert!(
        !env.globals.values().any(|visible| *visible == id),
        "the implementation helper `{private}` must remain hidden"
    );
    env.globals.insert(private.to_owned(), id);
}

fn expose_release_continuation_for_client(env: &mut ElabEnv) {
    let ty = law_type(env, "resource_release_settles");
    let (_, right) = equal_sides(&ty, false);
    let Term::App(_, continuation) = right else {
        panic!("the released Vis must have a checked continuation")
    };
    let id = head_id(continuation);
    assert!(env.env.transparent_body(id).is_some());
    assert!(!env.globals.values().any(|visible| *visible == id));
    env.globals
        .insert("private_resource_release_result".to_owned(), id);
}

fn assert_exact_client(env: &mut ElabEnv, name: &str, client: &str) {
    let expected = law_type(env, name);
    let client_name = format!("client_{name}");
    env.elaborate_decl(client)
        .unwrap_or_else(|error| panic!("independent client for `{name}` must check: {error:?}"));
    let checked = law_type(env, &client_name);
    assert_eq!(
        checked, expected,
        "`{name}` must state exactly the independently written client proposition"
    );
}

/// Promise class: normative compatibility vector for the Resource bracket.
/// MEASURED: each named prelude theorem is a transparent checked GlobalId, and
/// its type equals a separately elaborated client proposition. CLAIMED: the
/// four settlement outcomes and all three brackets' sequencing remain exact,
/// without a new trusted declaration or a newly exposed implementation helper.
/// THE GAP: a helper alias is recovered from a checked theorem type in this
/// disposable environment; the separate production mutations demonstrate that
/// each settlement and sequencing law still reaches its real prelude definition.
#[test]
fn resource_laws_have_exact_independent_checked_clients() {
    let mut env = ElabEnv::empty().expect("default-stack prelude with resource laws");
    let trusted_before_clients = env.env.trusted_base();
    for private in ["resource_refl", "private_resource_release_result"] {
        assert!(
            !env.globals.contains_key(private),
            "the test must start with `{private}` hidden"
        );
    }
    expose_private_source_for_client(
        &mut env,
        "resource_settle_result_for",
        "resource_settle_body_ok_clean",
        true,
    );
    expose_private_source_for_client(
        &mut env,
        "private_with_resource_after_open",
        "resource_after_open_error",
        false,
    );
    expose_private_source_for_client(
        &mut env,
        "private_with_buffer_after_allocate",
        "buffer_after_allocate_error",
        false,
    );
    expose_private_source_for_client(
        &mut env,
        "private_with_mapping_after_allocate",
        "mapping_after_allocate_error",
        false,
    );
    expose_private_source_for_client(
        &mut env,
        "release_if_live",
        "resource_release_settles",
        false,
    );
    expose_release_continuation_for_client(&mut env);
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
            "PrivateBufferHandle",
            env.prelude_env.private_buffer_handle_id,
        ),
    ] {
        assert!(!env.globals.contains_key(name));
        assert!(env.env.constructor(id).is_some());
        env.globals.insert(name.to_owned(), id);
    }
    let mapping_handle_id = env.globals["MappingHandle"];
    let constructors = &env
        .env
        .inductive(mapping_handle_id)
        .expect("checked MappingHandle")
        .constructors;
    assert_eq!(constructors.len(), 1);
    let private_mapping_handle = constructors[0].id;
    assert!(!env.globals.values().any(|id| *id == private_mapping_handle));
    env.globals
        .insert("PrivateMappingHandle".to_owned(), private_mapping_handle);

    assert_exact_client(
        &mut env,
        "resource_settle_body_ok_clean",
        r#"theorem client_resource_settle_body_ok_clean
          (o : Type) (e : Type) (r : Type) (value : r)
          : And
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyOk e r value)
                (Ok ResourceError Unit MkUnit))
              (Ok o (ResourceBracketResult e r) (ResourceBracketOk e r value)))
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyOk e r value)
                (Err ResourceError Unit Closed))
              (Ok o (ResourceBracketResult e r) (ResourceBracketOk e r value))) =
          resource_settle_body_ok_clean o e r value"#,
    );
    assert_exact_client(
        &mut env,
        "resource_settle_body_error_clean",
        r#"theorem client_resource_settle_body_error_clean
          (o : Type) (e : Type) (r : Type) (body_error : e)
          : And
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyErr e r body_error)
                (Ok ResourceError Unit MkUnit))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketBodyError e r body_error)))
            (Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyErr e r body_error)
                (Err ResourceError Unit Closed))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketBodyError e r body_error))) =
          resource_settle_body_error_clean o e r body_error"#,
    );
    assert_exact_client(
        &mut env,
        "resource_settle_body_ok_release_error",
        r#"theorem client_resource_settle_body_ok_release_error
          (o : Type) (e : Type) (r : Type) (value : r) (release_error : ResourceError)
          : Not (Equal ResourceError release_error Closed)
            → Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyOk e r value)
                (Err ResourceError Unit release_error))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketReleaseError e r release_error)) =
          resource_settle_body_ok_release_error o e r value release_error"#,
    );
    assert_exact_client(
        &mut env,
        "resource_settle_body_error_release_error",
        r#"theorem client_resource_settle_body_error_release_error
          (o : Type) (e : Type) (r : Type) (body_error : e)
          (release_error : ResourceError)
          : Not (Equal ResourceError release_error Closed)
            → Equal (Result o (ResourceBracketResult e r))
              (resource_settle_result_for o e r (ResourceBodyErr e r body_error)
                (Err ResourceError Unit release_error))
              (Ok o (ResourceBracketResult e r)
                (ResourceBracketBodyAndReleaseError e r body_error release_error)) =
          resource_settle_body_error_release_error o e r body_error release_error"#,
    );

    assert_exact_client(
        &mut env,
        "resource_with_acquire",
        r#"theorem client_resource_with_acquire
          (a : Auth) (e : Type) (r : Type) (cap : Cap a)
          (path : Bytes) (mode : ResourceOpenMode)
          (body : Resource ResourceKind.FsHandle -> HostIO a (ResourceBodyResult e r))
          : Equal (HostIO a (Result FileError (ResourceBracketResult e r)))
            (withResource a e r cap path mode body)
            (Vis (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result FileError (ResourceBracketResult e r))
              (InL (FSOp a) AmbientOp (PrivateFsOpen a cap path mode))
              (private_with_resource_after_open a e r body)) =
          resource_with_acquire a e r cap path mode body"#,
    );
    assert_exact_client(
        &mut env,
        "resource_after_open_error",
        r#"theorem client_resource_after_open_error
          (a : Auth) (e : Type) (r : Type)
          (body : Resource ResourceKind.FsHandle -> HostIO a (ResourceBodyResult e r))
          (open_error : FileError)
          : Equal (HostIO a (Result FileError (ResourceBracketResult e r)))
            (private_with_resource_after_open a e r body
              (Err FileError (Resource ResourceKind.FsHandle) open_error))
            (Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result FileError (ResourceBracketResult e r))
              (Err FileError (ResourceBracketResult e r) open_error)) =
          resource_after_open_error a e r body open_error"#,
    );
    assert_exact_client(
        &mut env,
        "resource_after_open_success",
        r#"theorem client_resource_after_open_success
          (a : Auth) (e : Type) (r : Type)
          (body : Resource ResourceKind.FsHandle -> HostIO a (ResourceBodyResult e r))
          (resource : Resource ResourceKind.FsHandle)
          : Equal (HostIO a (Result FileError (ResourceBracketResult e r)))
            (private_with_resource_after_open a e r body
              (Ok FileError (Resource ResourceKind.FsHandle) resource))
            (bind (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (ResourceBodyResult e r) (Result FileError (ResourceBracketResult e r))
              (body resource)
              (release_if_live a FileError e r ResourceKind.FsHandle resource)) =
          resource_after_open_success a e r body resource"#,
    );
    assert_exact_client(
        &mut env,
        "buffer_with_acquire",
        r#"theorem client_buffer_with_acquire
          (a : Auth) (e : Type) (r : Type) (capacity : Int)
          (body : BufferHandle -> HostIO a (ResourceBodyResult e r))
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (withBuffer a e r capacity body)
            (Vis (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError (ResourceBracketResult e r))
              (InL (FSOp a) AmbientOp (PrivateBufferAllocate a capacity))
              (private_with_buffer_after_allocate a e r capacity body)) =
          buffer_with_acquire a e r capacity body"#,
    );
    assert_exact_client(
        &mut env,
        "buffer_after_allocate_error",
        r#"theorem client_buffer_after_allocate_error
          (a : Auth) (e : Type) (r : Type) (capacity : Int)
          (body : BufferHandle -> HostIO a (ResourceBodyResult e r))
          (allocate_error : ResourceError)
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (private_with_buffer_after_allocate a e r capacity body
              (Err ResourceError (Resource ResourceKind.Buffer) allocate_error))
            (Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError (ResourceBracketResult e r))
              (Err ResourceError (ResourceBracketResult e r) allocate_error)) =
          buffer_after_allocate_error a e r capacity body allocate_error"#,
    );
    assert_exact_client(
        &mut env,
        "buffer_after_allocate_success",
        r#"theorem client_buffer_after_allocate_success
          (a : Auth) (e : Type) (r : Type) (capacity : Int)
          (body : BufferHandle -> HostIO a (ResourceBodyResult e r))
          (resource : Resource ResourceKind.Buffer)
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (private_with_buffer_after_allocate a e r capacity body
              (Ok ResourceError (Resource ResourceKind.Buffer) resource))
            (bind (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (ResourceBodyResult e r) (Result ResourceError (ResourceBracketResult e r))
              (body (PrivateBufferHandle resource capacity))
              (release_if_live a ResourceError e r ResourceKind.Buffer resource)) =
          buffer_after_allocate_success a e r capacity body resource"#,
    );
    assert_exact_client(
        &mut env,
        "mapping_with_anonymous_acquire",
        r#"theorem client_mapping_with_anonymous_acquire
          (a : Auth) (e : Type) (r : Type) (length : Int)
          (protection : MappingProt)
          (body : MappingHandle -> HostIO a (ResourceBodyResult e r))
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (withMapping a e r (Anonymous length) protection body)
            (Vis (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError (ResourceBracketResult e r))
              (InL (FSOp a) AmbientOp
                (PrivateMappingAllocate a length protection))
              (private_with_mapping_after_allocate a e r length body)) =
          mapping_with_anonymous_acquire a e r length protection body"#,
    );
    assert_exact_client(
        &mut env,
        "mapping_with_file_acquire",
        r#"theorem client_mapping_with_file_acquire
          (a : Auth) (e : Type) (r : Type)
          (file : Resource ResourceKind.FsHandle) (length : Int)
          (protection : MappingProt)
          (body : MappingHandle -> HostIO a (ResourceBodyResult e r))
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (withMapping a e r (FileBacked file length) protection body)
            (Vis (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError (ResourceBracketResult e r))
              (InL (FSOp a) AmbientOp
                (PrivateMappingAcquireFile a file length protection))
              (private_with_mapping_after_allocate a e r length body)) =
          mapping_with_file_acquire a e r file length protection body"#,
    );
    assert_exact_client(
        &mut env,
        "mapping_after_allocate_error",
        r#"theorem client_mapping_after_allocate_error
          (a : Auth) (e : Type) (r : Type) (length : Int)
          (body : MappingHandle -> HostIO a (ResourceBodyResult e r))
          (allocate_error : ResourceError)
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (private_with_mapping_after_allocate a e r length body
              (Err ResourceError (Resource ResourceKind.Mapping) allocate_error))
            (Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError (ResourceBracketResult e r))
              (Err ResourceError (ResourceBracketResult e r) allocate_error)) =
          mapping_after_allocate_error a e r length body allocate_error"#,
    );
    assert_exact_client(
        &mut env,
        "mapping_after_allocate_success",
        r#"theorem client_mapping_after_allocate_success
          (a : Auth) (e : Type) (r : Type) (length : Int)
          (body : MappingHandle -> HostIO a (ResourceBodyResult e r))
          (resource : Resource ResourceKind.Mapping)
          : Equal (HostIO a (Result ResourceError (ResourceBracketResult e r)))
            (private_with_mapping_after_allocate a e r length body
              (Ok ResourceError (Resource ResourceKind.Mapping) resource))
            (bind (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (ResourceBodyResult e r) (Result ResourceError (ResourceBracketResult e r))
              (body (PrivateMappingHandle resource (MkMappingExtent length)))
              (release_if_live a ResourceError e r ResourceKind.Mapping resource)) =
          mapping_after_allocate_success a e r length body resource"#,
    );
    assert_exact_client(
        &mut env,
        "resource_release_settles",
        r#"theorem client_resource_release_settles
          (a : Auth) (o : Type) (e : Type) (r : Type)
          (kind : ResourceKind) (resource : Resource kind)
          (body_result : ResourceBodyResult e r)
          : Equal (HostIO a (Result o (ResourceBracketResult e r)))
            (release_if_live a o e r kind resource body_result)
            (Vis (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result o (ResourceBracketResult e r))
              (InL (FSOp a) AmbientOp (PrivateResourceRelease a kind resource))
              (private_resource_release_result a o e r body_result)) =
          resource_release_settles a o e r kind resource body_result"#,
    );
    assert_eq!(
        env.env.trusted_base(),
        trusted_before_clients,
        "independently checked clients must not introduce an axiom or primitive"
    );
}
