use ken_kernel::inductive::{all_support_evidence_positions, peel_app};
use ken_kernel::{
    check, declare_inductive, infer, AllSupportSort, Context, CtorSpec, Decl, GlobalEnv, GlobalId,
    InductiveSpec, KernelError, Level, LevelVar, Term,
};

fn ty0() -> Term {
    Term::Type(Level::zero())
}

fn former(id: GlobalId) -> Term {
    Term::indformer(id, vec![])
}

fn declare_bool(env: &mut GlobalEnv) -> GlobalId {
    declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
        ],
    })
    .expect("Bool")
}

fn declare_box(env: &mut GlobalEnv) -> GlobalId {
    declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::var(0)],
            target_indices: vec![],
        }],
    })
    .expect("Box")
}

fn apply_all(support: GlobalId, carrier: Term, predicate: Term, source: Term) -> Term {
    apply_all_at(support, vec![Level::zero()], carrier, predicate, source)
}

fn apply_all_at(
    support: GlobalId,
    levels: Vec<Level>,
    carrier: Term,
    predicate: Term,
    source: Term,
) -> Term {
    Term::app(
        Term::app(
            Term::app(Term::indformer(support, levels), carrier),
            predicate,
        ),
        source,
    )
}

#[test]
fn generated_all_support_has_the_frozen_first_order_carrier() {
    // Normative compatibility vector: 14 §§1,3.2 and 18 §4.2 fix the
    // published environment carrier, not merely an implementation count.
    let mut env = GlobalEnv::new();
    let before_declarations = env.declarations().len();
    let before_next = env.next_global_id().0;
    let before_trust = env.trusted_base();
    let box_id = declare_box(&mut env);
    let supports = env.all_supports_for(box_id);

    assert_eq!(env.declarations().len() - before_declarations, 3);
    assert_eq!(env.next_global_id().0 - before_next, 6);
    assert_eq!(supports.len(), 2);
    assert!(supports
        .iter()
        .all(|family| env.is_terminal_support(*family)));
    assert!(supports
        .iter()
        .all(|family| env.all_supports_for(*family).is_empty()));
    assert_eq!(env.trusted_base(), before_trust);

    let published = &env.declarations()[before_declarations..];
    assert!(published
        .iter()
        .all(|decl| matches!(decl, Decl::Inductive(_))));
    assert!(published.iter().all(|decl| {
        matches!(decl, Decl::Inductive(inductive) if inductive.constructors.len() == 1)
    }));
    assert_eq!(
        env.all_support(box_id, 0, AllSupportSort::Type),
        Some(supports[0])
    );
    assert_eq!(
        env.all_support(box_id, 0, AllSupportSort::Omega),
        Some(supports[1])
    );
    for sort in [AllSupportSort::Type, AllSupportSort::Omega] {
        let family = env.all_support(box_id, 0, sort).expect("generated support");
        assert_eq!(
            env.all_support_origin(family),
            Some((box_id, 0, sort)),
            "the inverse relation must round-trip the issued support"
        );
    }
}

fn declare_three_way_host(env: &mut GlobalEnv) -> GlobalId {
    declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::var(0)],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::var(0), Term::var(1)],
                target_indices: vec![],
            },
        ],
    })
    .expect("three-way strictly-positive host")
}

#[test]
fn issued_origin_aligns_real_empty_one_join_support_constructors() {
    // Durable invariant: inspect the admitted declarations themselves. The
    // inverse relation, constructor ordinal, source index, and evidence suffix
    // are one issued fact; none may be reconstructed from constructor count.
    let mut env = GlobalEnv::new();
    let host = declare_three_way_host(&mut env);
    let host_declaration = env.inductive(host).expect("host declaration");
    let expected_positions = [vec![], vec![0], vec![0, 1]];

    for sort in [AllSupportSort::Type, AllSupportSort::Omega] {
        let support = env.all_support(host, 0, sort).expect("generated support");
        assert_eq!(
            env.all_support_origin(support),
            Some((host, 0, sort)),
            "forward and inverse issued relations agree"
        );
        let support_declaration = env.inductive(support).expect("support declaration");
        assert_eq!(
            support_declaration.constructors.len(),
            host_declaration.constructors.len()
        );
        assert!(infer(&env, &Context::new(), &support_declaration.former_type).is_ok());

        for (ordinal, (host_constructor, support_constructor)) in host_declaration
            .constructors
            .iter()
            .zip(&support_declaration.constructors)
            .enumerate()
        {
            let positions = all_support_evidence_positions(&env, support, ordinal)
                .expect("issued evidence topology");
            assert_eq!(positions, expected_positions[ordinal]);
            assert_eq!(
                support_constructor.args.len() - host_constructor.args.len(),
                positions.len(),
                "the actual support constructor has one evidence suffix field per position"
            );
            let source = support_constructor
                .target_indices
                .last()
                .expect("source index");
            let (source_head, source_arguments) = peel_app(source);
            assert!(
                matches!(source_head, Term::Constructor { id, .. } if id == host_constructor.id),
                "the real source index is headed by the aligned host constructor"
            );
            assert_eq!(
                source_arguments.len(),
                1 + host_constructor.args.len(),
                "source index contains the carrier parameter and every real host field"
            );
            assert!(infer(&env, &Context::new(), &support_constructor.type_).is_ok());
        }
    }

    let foreign_host = declare_three_way_host(&mut env);
    for sort in [AllSupportSort::Type, AllSupportSort::Omega] {
        let support = env.all_support(host, 0, sort).expect("host support");
        let foreign_support = env
            .all_support(foreign_host, 0, sort)
            .expect("same-cardinality foreign support");
        assert_eq!(
            env.inductive(support).unwrap().constructors.len(),
            env.inductive(foreign_support).unwrap().constructors.len(),
            "the discriminator must survive equal constructor cardinality"
        );
        assert_eq!(
            env.all_support_origin(foreign_support),
            Some((foreign_host, 0, sort))
        );
        assert_ne!(
            env.all_support_origin(support),
            env.all_support_origin(foreign_support)
        );
    }
}

#[test]
fn generated_all_support_is_absent_from_general_nested_lookup() {
    // Soundness control: terminal support retains enough polarity metadata for
    // its private checked construction, but cannot itself become a host edge.
    let mut env = GlobalEnv::new();
    let bool_id = declare_bool(&mut env);
    let box_id = declare_box(&mut env);
    let support = env.all_support(box_id, 0, AllSupportSort::Type).unwrap();
    let before_declarations = env.declarations().to_vec();
    let before_next = env.next_global_id();

    let attempted = declare_inductive(&mut env, |nested| {
        let nested_type = former(nested);
        let bool_type = former(bool_id);
        let source_type = Term::app(former(box_id), bool_type.clone());
        let predicate = Term::lam(bool_type.clone(), nested_type);
        InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![CtorSpec {
                args: vec![
                    source_type,
                    apply_all(support, bool_type, predicate, Term::var(0)),
                ],
                target_indices: vec![],
            }],
        }
    });

    assert!(matches!(
        attempted,
        Err(KernelError::PositivityViolation(_))
    ));
    assert_eq!(env.declarations(), before_declarations);
    assert_eq!(env.next_global_id(), before_next);
    assert!(env.all_supports_for(support).is_empty());
}

#[test]
fn generated_all_type_and_omega_cross_at_the_exact_public_level() {
    // Durable invariant: declared-former topology is Type-valued at max(leaf,
    // host), while the leaf predicate retains its own Type/Omega sort.
    let mut env = GlobalEnv::new();
    let bool_id = declare_bool(&mut env);
    let box_id = declare_box(&mut env);
    let box_ctor = env.inductive(box_id).unwrap().constructors[0].id;
    let bool_type = former(bool_id);
    let source = Term::app(
        Term::app(Term::constructor(box_ctor, vec![]), bool_type.clone()),
        Term::constructor(env.inductive(bool_id).unwrap().constructors[0].id, vec![]),
    );
    let type_predicate = Term::lam(bool_type.clone(), bool_type.clone());
    let omega_predicate = Term::lam(
        bool_type.clone(),
        Term::Const {
            id: env.top_id(),
            level_args: vec![],
        },
    );
    let all_type = apply_all(
        env.all_support(box_id, 0, AllSupportSort::Type).unwrap(),
        bool_type.clone(),
        type_predicate,
        source.clone(),
    );
    let all_omega = apply_all(
        env.all_support(box_id, 0, AllSupportSort::Omega).unwrap(),
        bool_type,
        omega_predicate,
        source,
    );

    assert_eq!(infer(&env, &Context::new(), &all_type).unwrap(), ty0());
    assert_eq!(infer(&env, &Context::new(), &all_omega).unwrap(), ty0());
}

#[test]
fn generated_all_sorts_retain_both_independent_symbolic_levels() {
    // Mutations that drop either leaf or host level, or add a successor, are
    // distinguished while h and l remain unrelated variables.
    const DECLARED_H: LevelVar = LevelVar(0);
    const ACTUAL_H: LevelVar = LevelVar(7);
    const ACTUAL_L: LevelVar = LevelVar(11);
    let mut env = GlobalEnv::new();
    let box_id = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![DECLARED_H],
        params: vec![Term::Type(Level::Var(DECLARED_H))],
        indices: vec![],
        level: Level::Var(DECLARED_H),
        constructors: vec![CtorSpec {
            args: vec![Term::var(0)],
            target_indices: vec![],
        }],
    })
    .expect("level-polymorphic Box");
    let h = Level::Var(ACTUAL_H);
    let l = Level::Var(ACTUAL_L);
    let mut ctx = Context::new();
    ctx.push(Term::Type(h.clone()));
    ctx.push(Term::pi(Term::var(0), Term::Type(l.clone())));
    ctx.push(Term::pi(Term::var(1), Term::Omega(l.clone())));
    ctx.push(Term::app(
        Term::indformer(box_id, vec![h.clone()]),
        Term::var(2),
    ));
    let expected_level = l.clone().max(h.clone()).normalize();
    for (sort, predicate) in [
        (AllSupportSort::Type, Term::var(2)),
        (AllSupportSort::Omega, Term::var(1)),
    ] {
        let support = env.all_support(box_id, 0, sort).unwrap();
        let family = apply_all_at(
            support,
            vec![h.clone(), l.clone()],
            Term::var(3),
            predicate,
            Term::var(0),
        );
        assert_eq!(
            infer(&env, &ctx, &family).unwrap(),
            Term::Type(expected_level.clone())
        );
    }
}

#[test]
fn zero_evidence_constructor_remains_source_aligned() {
    // Durable invariant: a zero-leaf topology has its own aligned constructor;
    // it is not omitted or replaced by a fabricated predicate inhabitant.
    let mut env = GlobalEnv::new();
    let slot = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::var(0)],
                target_indices: vec![],
            },
        ],
    })
    .expect("Slot");
    for sort in [AllSupportSort::Type, AllSupportSort::Omega] {
        let support = env.all_support(slot, 0, sort).unwrap();
        let declaration = env.inductive(support).unwrap();
        assert_eq!(declaration.constructors.len(), 2);
        assert!(declaration.constructors[0].args.is_empty());
        assert_eq!(declaration.constructors[1].args.len(), 2);
        assert_ne!(
            declaration.constructors[0].target_indices,
            declaration.constructors[1].target_indices
        );
    }
}

#[test]
fn generated_support_failure_rolls_back_the_whole_host_transaction() {
    // Durable invariant: an unsupported positive native carrier shape cannot
    // publish a host without its required All families.
    let mut env = GlobalEnv::new();
    let declarations = env.declarations().to_vec();
    let next = env.next_global_id();
    let trust = env.trusted_base();
    let result = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::Eq(
                Box::new(ty0()),
                Box::new(Term::var(0)),
                Box::new(Term::var(0)),
            )],
            target_indices: vec![],
        }],
    });
    assert!(result.is_err());
    assert_eq!(env.declarations(), declarations);
    assert_eq!(env.next_global_id(), next);
    assert_eq!(env.trusted_base(), trust);
    let failed_host = next;
    assert!(env.all_supports_for(failed_host).is_empty());
    for sort in [AllSupportSort::Type, AllSupportSort::Omega] {
        assert_eq!(env.all_support(failed_host, 0, sort), None);
    }
    for raw in next.0..next.0 + 8 {
        let candidate = GlobalId(raw);
        assert!(env.lookup(candidate).is_none());
        assert_eq!(env.all_support_origin(candidate), None);
        assert!(!env.is_terminal_support(candidate));
        assert!(env.all_supports_for(candidate).is_empty());
    }
}

#[test]
fn neutral_all_inhabitant_checks_at_the_literal_family_application() {
    // Durable invariant: same source-indexed family, no method-dependent
    // decoder conversion. The aligned support constructor is a direct witness.
    let mut env = GlobalEnv::new();
    let bool_id = declare_bool(&mut env);
    let box_id = declare_box(&mut env);
    let bool_type = former(bool_id);
    let box_bool = Term::app(former(box_id), bool_type.clone());
    let support = env.all_support(box_id, 0, AllSupportSort::Type).unwrap();
    let support_ctor = env.inductive(support).unwrap().constructors[0].id;
    let predicate = Term::lam(bool_type.clone(), bool_type.clone());
    let mut ctx = Context::new();
    ctx.push(box_bool.clone());
    let expected = apply_all(support, bool_type.clone(), predicate.clone(), Term::var(0));
    let true_term = Term::constructor(env.inductive(bool_id).unwrap().constructors[0].id, vec![]);
    let mut method_body = Term::constructor(support_ctor, vec![Level::zero()]);
    method_body = Term::app(method_body, bool_type.clone());
    method_body = Term::app(method_body, predicate.clone());
    method_body = Term::app(method_body, Term::var(0));
    method_body = Term::app(method_body, true_term);
    let method = Term::lam(bool_type.clone(), method_body);
    let motive = Term::Ascript(
        Box::new(Term::lam(
            box_bool.clone(),
            apply_all(
                support,
                weaken_for_binder(&bool_type),
                weaken_for_binder(&predicate),
                Term::var(0),
            ),
        )),
        Box::new(Term::pi(box_bool, ty0())),
    );
    let witness = Term::Elim {
        fam: box_id,
        level_args: vec![],
        params: vec![bool_type],
        motive: Box::new(motive),
        methods: vec![method],
        indices: vec![],
        scrut: Box::new(Term::var(0)),
    };
    check(&env, &ctx, &witness, &expected)
        .expect("neutral host eliminator inhabits the literal All application");
}

#[test]
fn neutral_method_dependent_decoder_cannot_replace_literal_all() {
    // Controlled mutation from the conformance vector: keep v, Bool, and Box
    // fixed, but replace All P v with a neutral decoder's first projection.
    let mut env = GlobalEnv::new();
    let bool_id = declare_bool(&mut env);
    let box_id = declare_box(&mut env);
    let bool_type = former(bool_id);
    let box_bool = Term::app(former(box_id), bool_type.clone());
    let support = env.all_support(box_id, 0, AllSupportSort::Type).unwrap();
    let support_ctor = env.inductive(support).unwrap().constructors[0].id;
    let predicate = Term::lam(bool_type.clone(), bool_type.clone());
    let true_term = Term::constructor(env.inductive(bool_id).unwrap().constructors[0].id, vec![]);
    let mut ctx = Context::new();
    ctx.push(box_bool.clone());

    let mut all_method_body = Term::constructor(support_ctor, vec![Level::zero()]);
    all_method_body = Term::app(all_method_body, bool_type.clone());
    all_method_body = Term::app(all_method_body, predicate);
    all_method_body = Term::app(all_method_body, Term::var(0));
    all_method_body = Term::app(all_method_body, true_term.clone());
    let all_motive = Term::Ascript(
        Box::new(Term::lam(
            box_bool.clone(),
            apply_all(
                support,
                weaken_for_binder(&bool_type),
                Term::lam(weaken_for_binder(&bool_type), weaken_for_binder(&bool_type)),
                Term::var(0),
            ),
        )),
        Box::new(Term::pi(box_bool.clone(), ty0())),
    );
    let witness = Term::Elim {
        fam: box_id,
        level_args: vec![],
        params: vec![bool_type.clone()],
        motive: Box::new(all_motive),
        methods: vec![Term::lam(bool_type.clone(), all_method_body)],
        indices: vec![],
        scrut: Box::new(Term::var(0)),
    };

    let decoder_motive = Term::Ascript(
        Box::new(Term::lam(
            box_bool.clone(),
            Term::sigma(ty0(), Term::var(0)),
        )),
        Box::new(Term::pi(box_bool, Term::Type(Level::zero().suc()))),
    );
    let decoder = Term::Elim {
        fam: box_id,
        level_args: vec![],
        params: vec![bool_type.clone()],
        motive: Box::new(decoder_motive),
        methods: vec![Term::lam(
            bool_type.clone(),
            Term::pair(bool_type, true_term),
        )],
        indices: vec![],
        scrut: Box::new(Term::var(0)),
    };
    let mutated_binder = Term::proj1(decoder);
    let error = check(&env, &ctx, &witness, &mutated_binder)
        .expect_err("a neutral method-dependent decoder cannot replace All P v");
    assert!(matches!(error, KernelError::TypeMismatch { .. }));
}

fn weaken_for_binder(term: &Term) -> Term {
    ken_kernel::subst::weaken(term, 1)
}
