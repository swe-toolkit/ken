//! Spec 36 §2.4, production checked-core pins for sequenced local lets.
//! Promise class: durable invariant. MEASURED: checked ITree and bind IDs
//! occur at the elaborated term, with one Ret inside an inferred pure-body
//! continuation. CLAIMED: a performed RHS is not lowered as ordinary Let.
//! THE GAP: these declarations do not exhaust all effect rows; CLI tests
//! independently observe execution order. A performing RHS inside a
//! non-tree-result proc with a non-empty row is elaborated by the unmodified
//! ordinary-let fallback and is not sequenced; pre-existing on the exact
//! base; filed separately. An RHS whose effect comes only from a local
//! binder's latent row (for example a callback parameter `f : A ->[E] B`
//! applied as `f a`) is not classified as performing, because the shared
//! row plane (`infer_expr_row_type`) does not yet carry local latent rows
//! (36 §1.2 `latent(typeof(Γ,f))`). Such a let behaves exactly as on the
//! base. The same plane gap makes the effect-escape gate accept those calls;
//! both are closed by `LANG-EFFECT-ROW-LOCAL-LATENT`, not by this WP.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

fn checked_def<'a>(env: &'a ElabEnv, id: ken_kernel::GlobalId) -> (&'a Term, &'a Term) {
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(id) else {
        panic!("the declaration must be a checked transparent definition")
    };
    (ty, body)
}

fn spine(mut term: &Term) -> (&Term, Vec<&Term>) {
    let mut args = Vec::new();
    while let Term::App(function, argument) = term {
        args.push(argument.as_ref());
        term = function;
    }
    args.reverse();
    (term, args)
}

fn assert_bind<'a>(env: &ElabEnv, term: &'a Term) -> Vec<&'a Term> {
    let (head, args) = spine(term);
    assert!(
        matches!(head, Term::Const { id, .. } if *id == env.prelude_env.bind_id),
        "expected the registered bind identity, got {head:?}"
    );
    assert_eq!(args.len(), 6, "bind has (e, resp, a, b, t, f)");
    args
}

#[test]
fn checked_and_inferred_performed_lets_emit_bind_with_ret_only_for_pure_infer_body() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let checked = env
        .elaborate_decl(
            "proc checked : IO (Result IOError Unit) visits [Console] = \
         let r = write ConsoleStream.Stdout (bytes_encode \"a\") in \
         write ConsoleStream.Stdout (bytes_encode \"b\")",
        )
        .expect("checked tree continuation");
    let (_, checked_body) = checked_def(&env, checked);
    let checked_args = assert_bind(&env, checked_body);
    let Term::Lam(_, checked_continuation) = checked_args[5] else {
        panic!("bind's continuation must be a lambda")
    };
    let (checked_head, _) = spine(checked_continuation);
    assert!(
        !matches!(checked_head, Term::Constructor { id, .. } if *id == env.prelude_env.ret_id),
        "an already-tree-typed checked continuation must not get another Ret"
    );

    let inferred = env
        .elaborate_decl(
            "proc inferred visits [Console] = \
         let r = write ConsoleStream.Stdout (bytes_encode \"a\") in MkUnit",
        )
        .expect("infer the result as an ITree with a pure-body Ret");
    let (inferred_ty, inferred_body) = checked_def(&env, inferred);
    let (tree_head, tree_args) = spine(inferred_ty);
    assert!(matches!(tree_head, Term::IndFormer { id, .. } if *id == env.prelude_env.itree_id));
    assert_eq!(tree_args.len(), 3);
    let inferred_args = assert_bind(&env, inferred_body);
    let Term::Lam(_, inferred_continuation) = inferred_args[5] else {
        panic!("bind's continuation must be a lambda")
    };
    let (ret_head, ret_args) = spine(inferred_continuation);
    assert!(matches!(ret_head, Term::Constructor { id, .. } if *id == env.prelude_env.ret_id));
    assert_eq!(
        ret_args.len(),
        4,
        "Ret must wrap the pure body exactly once"
    );
}

/// Promise class: transition sentinel for 36 §2.4 inclusion. A valid
/// different signature is unsupported until `incl` is built.
#[test]
fn valid_inclusion_reports_unsupported_variant() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let error = env
        .elaborate_decl(
            "proc cross : HostIO APartial ExitCode visits [Console] = \
         let x = write ConsoleStream.Stdout (bytes_encode \"a\") in \
           host_program APartial (print_line \"b\")",
        )
        .expect_err("signature inclusion is not built");
    assert!(
        error
            .to_string()
            .starts_with("unsupported: 36 §2.4 inclusion of effect signature"),
        "{error}"
    );
    let ElabError::UnsupportedEffectInclusion { from, to, .. } = error else {
        panic!("a valid cross-signature call is not an ill-typed RHS: {error:?}")
    };
    assert_ne!(from, to, "the signatures really differ");
}

/// Promise class: durable invariant. An annotation naming an action instead
/// of its returned value is ill-typed regardless of future `incl` support.
#[test]
fn annotated_action_type_is_result_type_mismatch() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let error = env
        .elaborate_decl(
            "proc wrong_result : HostIO APartial ExitCode visits [Console] = \
         let p : HostIO APartial (Result IOError Unit) = \
           host_console APartial (Result IOError Unit) \
             (write ConsoleStream.Stdout (bytes_encode \"a\")) in \
           host_program APartial (print_line \"b\")",
        )
        .expect_err("annotation names the action, not its result");
    assert!(matches!(error, ElabError::TypeMismatch { reason, .. }
        if reason.contains("annotated result type differs from the action's result type")));
}

/// Promise class: durable invariant (36 §2.4). The public standalone
/// expression API used by REPL evaluation/type routes must reach the same
/// checked bind identity as declaration elaboration, with the first write
/// occupying bind's action argument rather than an ordinary Let value.
#[test]
fn public_standalone_performed_let_binds_the_first_checked_action() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let (core, ty) = env
        .elaborate_expr(
            "standalone_effectful_let",
            "let r = write ConsoleStream.Stdout (bytes_encode \"a\") in \
         write ConsoleStream.Stdout (bytes_encode \"b\")",
        )
        .expect("standalone action let elaborates");
    let args = assert_bind(&env, &core);
    let (first_head, _) = spine(args[4]);
    assert!(
        matches!(first_head, Term::Const { id, .. } if *id == env.globals["write"]),
        "the first action, not its unevaluated tree value, is passed to bind"
    );
    let (type_head, _) = spine(&ty);
    assert!(matches!(type_head, Term::IndFormer { id, .. } if *id == env.prelude_env.itree_id));
}

/// Promise class: durable invariant. A pure standalone let retains the same
/// ordinary kernel Let, independent of effect-row metadata on the context.
#[test]
fn public_standalone_pure_let_retains_kernel_let() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let (core, _) = env
        .elaborate_expr("standalone_pure_let", "let n = 1 in n")
        .expect("pure standalone let elaborates");
    assert!(matches!(core, Term::Let { .. }));
}

#[test]
fn annotated_non_tree_proc_result_remains_an_ordinary_let() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let direct = env
        .elaborate_decl("proc direct_value : Int visits [Console] = 1")
        .expect("a proc may declare a row while returning a non-tree value");
    let id = env
        .elaborate_decl("proc read : Int visits [Console] = let n : Int = direct_value in n")
        .expect("row-declared non-tree RHS stays on the ordinary annotated path");
    let (_, core) = checked_def(&env, id);
    let Term::Let { ty, val, body } = core else {
        panic!("a non-tree proc result retains the kernel's ordinary Let")
    };
    assert_eq!(ty.as_ref(), &Term::const_(env.globals["Int"], vec![]));
    assert_eq!(val.as_ref(), &Term::const_(direct, vec![]));
    assert_eq!(body.as_ref(), &Term::var(0));
}

#[test]
fn annotated_tree_result_is_bound_by_one_outer_bind() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let producer = env
        .elaborate_decl(
            "proc make_outer : HostIO APartial (HostIO APartial Unit) visits [Console] = \
         host_bind APartial (Result IOError Unit) (HostIO APartial Unit) \
           (host_console APartial (Result IOError Unit) \
             (write ConsoleStream.Stdout (bytes_encode \"a\"))) \
           (\\_. host_pure APartial (HostIO APartial Unit) \
             (host_pure APartial Unit MkUnit))",
        )
        .expect("producer returns a tree as its value");
    let consumer = env
        .elaborate_decl(
            "proc consumer : HostIO APartial ExitCode visits [Console] = \
         let t : HostIO APartial Unit = make_outer in \
           host_bind APartial Unit ExitCode t \
             (\\_. host_pure APartial ExitCode ExitCode.Success)",
        )
        .expect("annotated tree names the result, not the performed action");
    let (_, core) = checked_def(&env, consumer);
    let args = assert_bind(&env, core);
    assert!(
        matches!(args[4], Term::Const { id, .. } if *id == producer),
        "the performed producer occurs once as the outer bind's action"
    );
    let Term::Lam(_, continuation) = args[5] else {
        panic!("the returned action is bound in the continuation")
    };
    let (head, _) = spine(continuation);
    assert!(
        !matches!(head, Term::Const { id, .. } if *id == env.prelude_env.bind_id),
        "only the outer bind is generated around the performing RHS"
    );
}

#[test]
fn both_module_body_routes_and_pure_let_controls_preserve_their_core_shapes() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let plain = env
        .elaborate_decl(
            "proc plain : IO (Result IOError Unit) visits [Console] = \
         let r = write ConsoleStream.Stdout (bytes_encode \"a\") in \
         write ConsoleStream.Stdout (bytes_encode \"b\")",
        )
        .expect("spine-free module path");
    let (_, body) = checked_def(&env, plain);
    assert_bind(&env, body);

    let with_fixity = env
        .elaborate_decl(
            "proc with_fixity : IO (Result IOError Unit) visits [Console] = \
         let n = 1 + 2; r = write ConsoleStream.Stdout (bytes_encode \"a\") in \
         write ConsoleStream.Stdout (bytes_encode \"b\")",
        )
        .expect("fixity-reassociated module path");
    let (_, body) = checked_def(&env, with_fixity);
    let Term::Let { body, .. } = body else {
        panic!("the pure first group member remains an ordinary checked let")
    };
    assert_bind(&env, body);

    let nested = env
        .elaborate_decl("fn pure_nested (x : Int) : Int = let a = x in let b = a in b")
        .expect("nested pure lets");
    let grouped = env
        .elaborate_decl("fn pure_grouped (x : Int) : Int = let a = x; b = a in b")
        .expect("grouped pure lets");
    assert_eq!(
        format!("{:?}", checked_def(&env, nested).1),
        format!("{:?}", checked_def(&env, grouped).1)
    );
    let pure_tree = env
        .elaborate_decl(
            "fn pure_host_tree (x : ExitCode) : HostIO APartial ExitCode = \
         let a = host_pure APartial ExitCode x in a",
        )
        .expect("pure tree value remains an ordinary let");
    let (_, pure_body) = checked_def(&env, pure_tree);
    let Term::Lam(_, inner) = pure_body else {
        panic!("function input remains a lambda")
    };
    assert!(
        matches!(inner.as_ref(), Term::Let { .. }),
        "row-empty tree is bound as a value"
    );
}
