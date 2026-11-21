//! PX8-P checked Buffer acquisition and canonical settlement in the interpreter.

use std::path::PathBuf;

use ken_interp::{apply, eval, ConsoleIds, CoproductIds, EvalStore, EvalVal, FSIds, PosixHost};
use ken_kernel::{Decl, Term};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> ken_elaborator::ElabEnv {
    let mut env = ken_elaborator::ElabEnv::empty().expect("PX8-P prelude");
    env.set_catalog_roots_v1(&[repo_root().join("catalog/packages")])
        .expect("checked catalog root");
    let trusted_before_clients = env.env.trusted_base();
    env.elaborate_file(
        r#"import Capability.System.Buffer (BufferHandle)
import Capability.System.Resource (ResourceBodyResult, ResourceBodyOk, ResourceBodyErr,
  ResourceBracketResult, ResourceBracketOk, ResourceBracketBodyError,
  ResourceBracketReleaseError, ResourceBracketBodyAndReleaseError, withBuffer)
import Capability.Process.Exit (host_pure, host_bind, host_fs)
import Capability.Filesystem.Authority (FS)
fn px8p_ok_body (_resource : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  host_pure AFull (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

fn px8p_error_body (_resource : BufferHandle)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  host_pure AFull (ResourceBodyResult ResourceError Unit)
    (ResourceBodyErr ResourceError Unit ResourceError.InvalidBounds)

fn px8p_escape_body (resource : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit (BufferHandle)) =
  host_pure AFull (ResourceBodyResult Unit (BufferHandle))
    (ResourceBodyOk Unit (BufferHandle) resource)

proc px8p_after_escape_bracket
  (release : BufferHandle → FS AFull (Result ResourceError Unit))
  (bracket : ResourceBracketResult Unit (BufferHandle))
  : HostIO AFull (Result ResourceError Unit) visits [FS] =
  match bracket {
    ResourceBracketOk resource |->
      host_fs AFull (Result ResourceError Unit) (release resource);
    ResourceBracketBodyError error |-> host_pure AFull (Result ResourceError Unit)
      (Err ResourceError Unit ResourceError.MalformedResource);
    ResourceBracketReleaseError error |-> host_pure AFull (Result ResourceError Unit)
      (Err ResourceError Unit error);
    ResourceBracketBodyAndReleaseError body_error release_error |->
      host_pure AFull (Result ResourceError Unit) (Err ResourceError Unit release_error)
  }

proc px8p_after_escape_outer
  (release : BufferHandle → FS AFull (Result ResourceError Unit))
  (outcome : Result ResourceError
    (ResourceBracketResult Unit (BufferHandle)))
  : HostIO AFull (Result ResourceError Unit) visits [FS] =
  match outcome {
    Err acquire_error |-> host_pure AFull (Result ResourceError Unit)
      (Err ResourceError Unit acquire_error);
    Ok bracket |-> px8p_after_escape_bracket release bracket
  }

proc px8p_escape_then_release
  (release : BufferHandle → FS AFull (Result ResourceError Unit))
  (capacity : Int)
  : HostIO AFull (Result ResourceError Unit)
    visits [FS, BufferAllocate, ResourceRelease] =
  host_bind AFull
    (Result ResourceError (ResourceBracketResult Unit (BufferHandle)))
    (Result ResourceError Unit)
    (withBuffer AFull Unit (BufferHandle) capacity px8p_escape_body)
    (\outcome. px8p_after_escape_outer release outcome)
"#,
    )
    .expect("checked PX8-P interpreter fixtures elaborate");
    assert_eq!(
        env.env.trusted_base(),
        trusted_before_clients,
        "PX8-P client fixtures must not extend trust"
    );
    env
}

// The escape discriminator reaches the same private checked release node used
// by `withBuffer`'s finalizer. That node has no source route, so the release
// action is a kernel term over pre-source ids, and the kernel checks it.
fn private_release(env: &ken_elaborator::ElabEnv) -> Term {
    let release = env.prelude_env.private_resource_release_id;
    assert!(!env.globals.contains_key("PrivateResourceRelease"));
    assert!(!env.globals.values().any(|id| *id == release));
    let handle = env.prelude_env.buffer_handle_resource_id;
    assert!(!env.globals.values().any(|id| *id == handle));
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let op = Term::app(Term::indformer(env.globals["FSOp"], vec![]), afull.clone());
    let resp = Term::app(Term::const_(env.globals["fs_resp"], vec![]), afull.clone());
    let result = Term::app(
        Term::app(
            Term::indformer(env.prelude_env.result_id, vec![]),
            Term::indformer(env.globals["ResourceError"], vec![]),
        ),
        Term::indformer(env.prelude_env.unit_id, vec![]),
    );
    let tree = |constructor| {
        Term::app(
            Term::app(
                Term::app(Term::constructor(constructor, vec![]), op.clone()),
                resp.clone(),
            ),
            result.clone(),
        )
    };
    let label = Term::app(
        Term::app(
            Term::app(Term::constructor(release, vec![]), afull.clone()),
            Term::constructor(env.globals["ResourceKind.Buffer"], vec![]),
        ),
        Term::app(Term::const_(handle, vec![]), Term::var(0)),
    );
    let settled = Term::lam(
        Term::app(resp.clone(), label.clone()),
        Term::app(tree(env.prelude_env.ret_id), Term::var(0)),
    );
    let action = Term::lam(
        Term::indformer(env.globals["BufferHandle"], vec![]),
        Term::app(Term::app(tree(env.prelude_env.vis_id), label), settled),
    );
    let expected = Term::pi(
        Term::indformer(env.globals["BufferHandle"], vec![]),
        Term::app(
            Term::app(Term::const_(env.globals["FS"], vec![]), afull),
            result,
        ),
    );
    ken_kernel::check(&env.env, &ken_kernel::Context::new(), &action, &expected)
        .unwrap_or_else(|error| panic!("the private release action must check: {error:?}"));
    action
}

fn eval_global(env: &ken_elaborator::ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    let id = env.globals[name];
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("`{name}` must be transparent, got {other:?}"),
    }
}

fn type_value(env: &ken_elaborator::ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    eval(
        &[],
        &Term::indformer(env.globals[name], vec![]),
        &env.env,
        store,
    )
}

fn with_buffer(
    env: &ken_elaborator::ElabEnv,
    store: &mut EvalStore,
    error_type: EvalVal,
    result_type: EvalVal,
    capacity: i64,
    body: &str,
) -> EvalVal {
    let mut function = eval_global(env, store, "withBuffer");
    function = apply(
        function,
        eval(
            &[],
            &Term::constructor(env.globals["AFull"], vec![]),
            &env.env,
            store,
        ),
        &env.env,
        store,
    );
    function = apply(function, error_type, &env.env, store);
    function = apply(function, result_type, &env.env, store);
    function = apply(function, EvalVal::Int(capacity), &env.env, store);
    let body = eval_global(env, store, body);
    apply(function, body, &env.env, store)
}

fn drive(env: &ken_elaborator::ElabEnv, store: &mut EvalStore, tree: EvalVal) -> EvalVal {
    let ids = ConsoleIds::from_elab(&env.prelude_env).expect("Console ABI");
    let fs = FSIds::from_elab(&env.prelude_env).expect("FS ABI");
    let coproduct = CoproductIds {
        inl_id: env.globals["Coproduct.InL"],
        inr_id: env.globals["Coproduct.InR"],
    };
    let mut host = PosixHost::new_at(repo_root());
    ken_interp::run_io(
        tree,
        &mut host,
        &ids,
        Some(&fs),
        None,
        Some(&coproduct),
        &env.env,
        store,
    )
    .expect("checked buffer action drives")
}

fn assert_outer_error(env: &ken_elaborator::ElabEnv, value: &EvalVal, error: &str) {
    let EvalVal::Ctor { id, args, .. } = value else {
        panic!("expected Result error, got {value:?}");
    };
    assert_eq!(*id, env.globals["Err"]);
    assert!(matches!(
        &args[2],
        EvalVal::Ctor { id, .. } if *id == env.globals[&format!("ResourceError.{error}")]
    ));
}

#[test]
fn positive_capacity_settles_normally_and_body_error_still_settles() {
    let env = env();

    let mut ok_store = EvalStore::new();
    let unit = type_value(&env, &mut ok_store, "Unit");
    let ok_tree = with_buffer(&env, &mut ok_store, unit.clone(), unit, 8, "px8p_ok_body");
    let ok = drive(&env, &mut ok_store, ok_tree);
    let EvalVal::Ctor { id, args, .. } = ok else {
        panic!("expected outer Result, got {ok:?}");
    };
    assert_eq!(id, env.globals["Ok"]);
    assert!(matches!(
        &args[2],
        EvalVal::Ctor { id, .. } if *id == env.globals["ResourceBracketOk"]
    ));

    let mut error_store = EvalStore::new();
    let error_type = type_value(&env, &mut error_store, "ResourceError");
    let unit = type_value(&env, &mut error_store, "Unit");
    let error_tree = with_buffer(
        &env,
        &mut error_store,
        error_type,
        unit,
        8,
        "px8p_error_body",
    );
    let error = drive(&env, &mut error_store, error_tree);
    let EvalVal::Ctor { id, args, .. } = error else {
        panic!("expected outer Result, got {error:?}");
    };
    assert_eq!(id, env.globals["Ok"]);
    assert!(matches!(
        &args[2],
        EvalVal::Ctor { id, args, .. }
            if *id == env.globals["ResourceBracketBodyError"]
                && matches!(&args[2], EvalVal::Ctor { id, .. }
                    if *id == env.globals["ResourceError.InvalidBounds"])
    ));
}

#[test]
fn malformed_and_policy_invalid_capacities_have_exact_distinct_errors() {
    let env = env();

    let mut malformed_store = EvalStore::new();
    let unit = type_value(&env, &mut malformed_store, "Unit");
    let tree = with_buffer(
        &env,
        &mut malformed_store,
        unit.clone(),
        unit,
        -1,
        "px8p_ok_body",
    );
    let result = drive(&env, &mut malformed_store, tree);
    assert_outer_error(&env, &result, "InvalidBounds");

    for capacity in [0, 1_048_577] {
        let mut store = EvalStore::new();
        let unit = type_value(&env, &mut store, "Unit");
        let tree = with_buffer(
            &env,
            &mut store,
            unit.clone(),
            unit,
            capacity,
            "px8p_ok_body",
        );
        let result = drive(&env, &mut store, tree);
        assert_outer_error(&env, &result, "BufferLimit");
    }
}

#[test]
fn escaped_copy_reaches_exact_closed_after_bracket_settlement() {
    let env = env();
    let mut store = EvalStore::new();
    let release = eval(&[], &private_release(&env), &env.env, &mut store);
    let mut action = eval_global(&env, &mut store, "px8p_escape_then_release");
    action = apply(action, release, &env.env, &mut store);
    action = apply(action, EvalVal::Int(8), &env.env, &mut store);
    let result = drive(&env, &mut store, action);
    assert_outer_error(&env, &result, "Closed");

    // The production namespace remains locked even though the test reached the
    // private checked identity by immutable GlobalId.
    assert!(!env.globals.contains_key("PrivateResourceRelease"));
}
