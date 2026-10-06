use ken_kernel::{
    declare_deceq_certificate, declare_def, declare_inductive, declare_postulate,
    declare_primitive, declare_recursive_group, env_mark, rollback_to_mark, CtorSpec, Decl,
    GlobalEnv, InductiveSpec, KernelError, Level, PrimReduction, Term,
};

fn postulate(env: &mut GlobalEnv, name: &str) -> ken_kernel::GlobalId {
    declare_postulate(env, name.to_owned(), Vec::new(), Term::Type(Level::zero()))
        .expect("well-formed postulate")
}

fn top_const(id: ken_kernel::GlobalId) -> Term {
    Term::Const {
        id,
        level_args: Vec::new(),
    }
}

#[test]
fn rollback_removes_unstaged_tail_and_restores_allocator() {
    let mut env = GlobalEnv::new();
    let before_len = env.declarations().len();
    let before_next = env.next_global_id();
    let mark = env_mark(&env);

    let first = postulate(&mut env, "first after mark");
    let second = postulate(&mut env, "second after mark");
    assert!(env.trusted_base().contains(&first));
    assert!(env.trusted_base().contains(&second));

    let removed = rollback_to_mark(&mut env, mark).expect("same-env rollback");
    assert_eq!(
        removed.iter().map(Decl::id).collect::<Vec<_>>(),
        [second, first]
    );
    assert_eq!(env.declarations().len(), before_len);
    assert_eq!(env.next_global_id(), before_next);
    assert!(!env.trusted_base().contains(&first));
    assert!(!env.trusted_base().contains(&second));

    let reused = postulate(&mut env, "reused after rollback");
    assert_eq!(reused, first, "rollback must rewind GlobalId allocation");
}

#[test]
fn rollback_removes_transparent_declarations_and_reference_indexes() {
    let mut env = GlobalEnv::new();
    let mark = env_mark(&env);
    let top = env.top_id();
    let tt = env.tt_id();
    let first = declare_def(&mut env, Vec::new(), top_const(top), top_const(tt))
        .expect("checked Top definition");
    let second = declare_def(&mut env, Vec::new(), top_const(top), top_const(first))
        .expect("checked definition that refers to the first");

    let removed = rollback_to_mark(&mut env, mark).expect("same-env rollback");
    assert_eq!(
        removed.iter().map(Decl::id).collect::<Vec<_>>(),
        [second, first]
    );
    assert!(env.lookup(first).is_none());
    assert!(env.lookup(second).is_none());

    let group = declare_recursive_group(
        &mut env,
        vec![(Vec::new(), top_const(top)), (Vec::new(), top_const(top))],
        |ids| vec![top_const(ids[1]), top_const(tt)],
    )
    .expect("post-rollback acyclic group");
    assert_eq!(group.len(), 2);
    assert!(
        group.iter().all(|id| !env.is_recursive_transparent(*id)),
        "reused IDs must not inherit old reference edges"
    );
}

#[test]
fn rollback_clears_public_and_certificate_registries() {
    let mut env = GlobalEnv::new();
    let before_len = env.declarations().len();
    let mark = env_mark(&env);
    let bool_family = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: Vec::new(),
        params: Vec::new(),
        indices: Vec::new(),
        level: Level::zero(),
        constructors: vec![
            CtorSpec {
                args: Vec::new(),
                target_indices: Vec::new(),
            },
            CtorSpec {
                args: Vec::new(),
                target_indices: Vec::new(),
            },
        ],
    })
    .expect("Bool family");
    let bool_true = match env.lookup(bool_family) {
        Some(Decl::Inductive(ind)) => ind.constructors[0].id,
        other => panic!("Bool family declaration missing: {other:?}"),
    };
    let primitive_type = declare_primitive(
        &mut env,
        Vec::new(),
        Term::Type(Level::zero()),
        PrimReduction::OpaqueType,
    )
    .expect("opaque primitive type");
    ken_kernel::check::register_checked_int_lit_carrier(&mut env, primitive_type)
        .expect("checked primitive literal carrier");
    let primitive_term = top_const(primitive_type);
    let bool_term = Term::indformer(bool_family, Vec::new());
    let eq_operation = declare_primitive(
        &mut env,
        Vec::new(),
        Term::pi(
            primitive_term.clone(),
            Term::pi(primitive_term.clone(), bool_term),
        ),
        PrimReduction::Op {
            symbol: "env_mark_test_eq",
        },
    )
    .expect("primitive equality operation");
    let certificate = declare_deceq_certificate(
        &mut env,
        primitive_type,
        eq_operation,
        bool_family,
        bool_true,
    )
    .expect("checked equality certificate");
    assert_eq!(env.deceq_cert(primitive_type), Some(&certificate));
    assert_eq!(env.int_lit_type(), Some(primitive_type));

    rollback_to_mark(&mut env, mark).expect("same-env rollback");
    assert_eq!(env.declarations().len(), before_len);
    assert!(env.deceq_cert(primitive_type).is_none());
    assert_eq!(env.int_lit_type(), None);
}

#[test]
fn stale_mark_cannot_rewind_past_replacement_constructor_ids() {
    let mut env = GlobalEnv::new();
    let earlier = env_mark(&env);
    postulate(&mut env, "replaced postulate");
    let stale = env_mark(&env);
    rollback_to_mark(&mut env, earlier).expect("earlier same-env mark");
    declare_inductive(&mut env, |_| InductiveSpec {
        level_params: Vec::new(),
        params: Vec::new(),
        indices: Vec::new(),
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: Vec::new(),
            target_indices: Vec::new(),
        }],
    })
    .expect("replacement singleton inductive");

    let before_stale = env.clone();
    let error = rollback_to_mark(&mut env, stale)
        .expect_err("stale boundary would rewind past a replacement constructor ID");
    assert!(matches!(error, KernelError::IllFormedDecl(_)));
    assert_eq!(env, before_stale);
    assert_eq!(env.next_global_id(), before_stale.next_global_id());
}

#[test]
fn foreign_and_stale_marks_refuse_without_mutating_environment() {
    let owner = GlobalEnv::new();
    let foreign_mark = env_mark(&owner);
    let mut foreign = GlobalEnv::new();
    let foreign_id = postulate(&mut foreign, "foreign");
    let before_foreign = foreign.clone();
    let error = rollback_to_mark(&mut foreign, foreign_mark)
        .expect_err("a mark cannot cross environment instances");
    assert!(matches!(error, KernelError::Msg(message)
        if message == "environment mark belongs to another environment"));
    assert_eq!(foreign, before_foreign);
    assert!(foreign.trusted_base().contains(&foreign_id));

    let mut env = GlobalEnv::new();
    let earlier = env_mark(&env);
    postulate(&mut env, "removed by earlier mark");
    let stale = env_mark(&env);
    rollback_to_mark(&mut env, earlier).expect("earlier same-env mark");
    let before_stale = env.clone();
    let error = rollback_to_mark(&mut env, stale)
        .expect_err("a mark beyond the current declaration boundary is stale");
    assert!(matches!(error, KernelError::IllFormedDecl(_)));
    assert_eq!(env, before_stale);
}
