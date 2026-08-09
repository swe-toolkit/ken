use ken_kernel::env::PrimReduction;
use ken_kernel::inductive::{derive_parameter_polarities, iota_reduct, method_type, peel_pi};
use ken_kernel::subst::weaken;
use ken_kernel::{
    check, declare_inductive, declare_primitive, infer, whnf, ConstructorDecl, Context, CtorSpec,
    GlobalEnv, GlobalId, InductiveDecl, InductiveSpec, KernelError, Level, ParameterPolarity, Term,
};

fn ty0() -> Term {
    Term::Type(Level::zero())
}

fn former(id: GlobalId) -> Term {
    Term::indformer(id, vec![])
}

fn constructor(id: GlobalId) -> Term {
    Term::constructor(id, vec![])
}

fn app2(function: Term, first: Term, second: Term) -> Term {
    Term::app(Term::app(function, first), second)
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

fn declare_list(env: &mut GlobalEnv) -> GlobalId {
    declare_inductive(env, |list| InductiveSpec {
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
                args: vec![Term::var(0), Term::app(former(list), Term::var(1))],
                target_indices: vec![],
            },
        ],
    })
    .expect("List")
}

fn declare_pair(env: &mut GlobalEnv) -> GlobalId {
    declare_inductive(env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0(), ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::var(1), Term::var(1)],
            target_indices: vec![],
        }],
    })
    .expect("Pair")
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

fn positivity_error(result: Result<GlobalId, KernelError>) -> String {
    match result {
        Err(KernelError::PositivityViolation(message)) => message,
        Err(other) => panic!("expected PositivityViolation, got {other}"),
        Ok(id) => panic!("expected rejection, admitted {id:?}"),
    }
}

#[test]
fn declared_positive_paths_admit_list_pair_and_fresh_container_nesting() {
    // Durable invariant / normative compatibility vector (AC-K1/AC-K2): the
    // rule follows recorded polarity, including a newly declared former.
    let mut env = GlobalEnv::new();
    let bool_id = declare_bool(&mut env);
    let list = declare_list(&mut env);
    let pair = declare_pair(&mut env);
    let fresh = declare_box(&mut env);
    let composed = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::app(former(fresh), Term::var(0))],
            target_indices: vec![],
        }],
    })
    .expect("positive composition");
    assert_eq!(
        env.inductive(composed).unwrap().parameter_polarities,
        vec![ParameterPolarity::StrictlyPositive]
    );

    let nested = declare_inductive(&mut env, |json| InductiveSpec {
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
                args: vec![Term::app(former(list), former(json))],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::app(
                    former(list),
                    app2(former(pair), former(bool_id), former(json)),
                )],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::app(former(fresh), former(json))],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::app(former(composed), former(json))],
                target_indices: vec![],
            },
        ],
    })
    .expect("all declared-positive paths admit nesting");

    let declaration = env.inductive(nested).expect("nested declaration");
    assert_eq!(declaration.constructors.len(), 5);
}

#[test]
fn nested_negative_unknown_and_non_positive_paths_reject_separately() {
    // Durable invariants (AC-K5/6/7): three different fail-closed boundaries.
    let mut env = GlobalEnv::new();
    let bool_id = declare_bool(&mut env);
    let list = declare_list(&mut env);
    let opaque = declare_primitive(
        &mut env,
        vec![],
        Term::pi(ty0(), ty0()),
        PrimReduction::OpaqueType,
    )
    .expect("opaque unary former");
    let contra = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::pi(Term::var(0), former(bool_id))],
            target_indices: vec![],
        }],
    })
    .expect("contravariant carrier declaration");
    assert_eq!(
        env.inductive(contra).unwrap().parameter_polarities,
        vec![ParameterPolarity::NonPositive]
    );

    let nested_negative = declare_inductive(&mut env, |bad| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::app(
                former(list),
                Term::pi(former(bad), former(bool_id)),
            )],
            target_indices: vec![],
        }],
    });
    assert!(positivity_error(nested_negative).contains("non-strictly-positive"));

    let unknown = declare_inductive(&mut env, |bad| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::app(Term::const_(opaque, vec![]), former(bad))],
            target_indices: vec![],
        }],
    });
    assert!(positivity_error(unknown).contains("non-strictly-positive"));

    let non_positive = declare_inductive(&mut env, |bad| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::app(former(contra), former(bad))],
            target_indices: vec![],
        }],
    });
    assert!(positivity_error(non_positive).contains("non-strictly-positive"));
}

fn polarity_fixture(
    params: Vec<Term>,
    indices: Vec<Term>,
    args: Vec<Term>,
    target_indices: Vec<Term>,
) -> InductiveDecl {
    InductiveDecl {
        id: GlobalId(900),
        level_params: vec![],
        params,
        parameter_polarities: vec![],
        indices,
        level: Level::zero(),
        constructors: vec![ConstructorDecl {
            id: GlobalId(901),
            args,
            target_indices,
            type_: ty0(),
            recursive_positions: vec![],
        }],
        former_type: ty0(),
    }
}

#[test]
fn polarity_producer_covers_all_four_positions_with_independent_mutations() {
    // Durable invariant (D1b/D2 gate): each position independently changes the
    // record. These four mutations are the controls, not four spellings of one.
    let argument = polarity_fixture(vec![ty0()], vec![], vec![Term::var(0)], vec![]);
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &argument),
        vec![ParameterPolarity::StrictlyPositive]
    );
    let mut argument_mutation = argument.clone();
    argument_mutation.constructors[0].args[0] = Term::pi(Term::var(0), ty0());
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &argument_mutation),
        vec![ParameterPolarity::NonPositive]
    );

    let target = polarity_fixture(vec![ty0()], vec![ty0()], vec![], vec![Term::var(0)]);
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &target),
        vec![ParameterPolarity::NonPositive]
    );
    let mut target_mutation = target.clone();
    target_mutation.constructors[0].target_indices.clear();
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &target_mutation),
        vec![ParameterPolarity::StrictlyPositive]
    );

    let index = polarity_fixture(vec![ty0()], vec![Term::var(0)], vec![], vec![]);
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &index),
        vec![ParameterPolarity::NonPositive]
    );
    let mut index_mutation = index.clone();
    index_mutation.indices[0] = ty0();
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &index_mutation),
        vec![ParameterPolarity::StrictlyPositive]
    );

    let dependent_parameter = polarity_fixture(vec![ty0(), Term::var(0)], vec![], vec![], vec![]);
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &dependent_parameter),
        vec![
            ParameterPolarity::NonPositive,
            ParameterPolarity::StrictlyPositive,
        ]
    );
    let mut parameter_mutation = dependent_parameter.clone();
    parameter_mutation.params[1] = ty0();
    assert_eq!(
        derive_parameter_polarities(&GlobalEnv::new(), &parameter_mutation),
        vec![
            ParameterPolarity::StrictlyPositive,
            ParameterPolarity::StrictlyPositive,
        ]
    );
}

#[test]
fn production_nested_lift_is_consumed_and_iota_computes() {
    // Durable invariant (AC-K3/4/14): a production-admitted nested field gets
    // one structured IH. The method consumes that binder in a checked let; if
    // the binder is removed, the same body is ill-typed. Iota supplies it and
    // the complete eliminator computes to Leaf.
    let mut env = GlobalEnv::new();
    let box_id = declare_box(&mut env);
    let family = declare_inductive(&mut env, |tree| InductiveSpec {
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
                args: vec![Term::app(former(box_id), former(tree))],
                target_indices: vec![],
            },
        ],
    })
    .expect("production nested family");
    let declaration = env.inductive(family).expect("nested family declaration");
    let leaf = declaration.constructors[0].id;
    let wrap = declaration.constructors[1].id;
    let box_ctor = env.inductive(box_id).unwrap().constructors[0].id;
    let family_type = former(family);
    let motive = Term::Ascript(
        Box::new(Term::lam(family_type.clone(), family_type.clone())),
        Box::new(Term::pi(family_type.clone(), ty0())),
    );
    let wrap_type =
        method_type(&env, declaration, 1, &motive, &[], &[]).expect("nested method type");
    let (domains, _) = peel_pi(&wrap_type);
    assert_eq!(domains.len(), 2, "field plus one structured IH");
    let consuming_body = Term::Let {
        ty: Box::new(weaken(&domains[1], 1)),
        val: Box::new(Term::var(0)),
        body: Box::new(constructor(leaf)),
    };
    let wrap_method = Term::lam(
        domains[0].clone(),
        Term::lam(domains[1].clone(), consuming_body.clone()),
    );
    check(&env, &Context::new(), &wrap_method, &wrap_type).expect("lift-consuming method checks");
    let missing_lift = Term::lam(domains[0].clone(), consuming_body);
    assert!(check(&env, &Context::new(), &missing_lift, &wrap_type).is_err());

    let leaf_method = constructor(leaf);
    let boxed_leaf = Term::app(
        Term::app(constructor(box_ctor), family_type.clone()),
        constructor(leaf),
    );
    let scrutinee = Term::app(constructor(wrap), boxed_leaf);
    let eliminator = Term::Elim {
        fam: family,
        level_args: vec![],
        params: vec![],
        motive: Box::new(motive),
        methods: vec![leaf_method, wrap_method],
        indices: vec![],
        scrut: Box::new(scrutinee),
    };
    infer(&env, &Context::new(), &eliminator).expect("kernel checks nested eliminator");
    assert!(
        matches!(whnf(&env, &Context::new(), &eliminator), Term::Constructor { id, .. } if id == leaf)
    );
}

#[test]
fn nested_recursive_host_composes_method_lift_and_iota() {
    // Durable invariant (14 §§3.2, 7.8, 8.5): a checked-positive path
    // remains consumable when the admitted host former is itself nested-
    // recursive. This reaches production `method_type` and the matching iota
    // construction; rejecting `RecursiveShape::Former` in host-IH extraction
    // makes the first `method_type` call fail.
    let mut env = GlobalEnv::new();
    let list = declare_list(&mut env);
    let nested = declare_inductive(&mut env, |nested| InductiveSpec {
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
                args: vec![Term::app(
                    former(list),
                    Term::app(former(nested), Term::var(0)),
                )],
                target_indices: vec![],
            },
        ],
    })
    .expect("nested-recursive host former");
    let outer = declare_inductive(&mut env, |outer| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::app(former(nested), former(outer))],
            target_indices: vec![],
        }],
    })
    .expect("outer family through nested-recursive host");

    let outer_decl = env.inductive(outer).expect("outer declaration");
    let wrap = outer_decl.constructors[0].id;
    let nested_decl = env.inductive(nested).expect("nested declaration");
    let nested_leaf = nested_decl.constructors[0].id;
    let outer_type = former(outer);
    let motive = Term::Ascript(
        Box::new(Term::lam(outer_type.clone(), outer_type.clone())),
        Box::new(Term::pi(outer_type.clone(), ty0())),
    );
    let wrap_type = method_type(&env, outer_decl, 0, &motive, &[], &[])
        .expect("method lift composes through nested-recursive host");
    let (domains, _) = peel_pi(&wrap_type);
    assert_eq!(domains.len(), 2, "field plus composed structured IH");
    let wrap_method = Term::lam(
        domains[0].clone(),
        Term::lam(
            domains[1].clone(),
            Term::app(weaken(&constructor(wrap), 2), Term::var(1)),
        ),
    );
    check(&env, &Context::new(), &wrap_method, &wrap_type).expect("composed lifted method checks");

    let nested_outer_leaf = Term::app(constructor(nested_leaf), outer_type.clone());
    let scrutinee = Term::app(constructor(wrap), nested_outer_leaf);
    let eliminator = Term::Elim {
        fam: outer,
        level_args: vec![],
        params: vec![],
        motive: Box::new(motive),
        methods: vec![wrap_method],
        indices: vec![],
        scrut: Box::new(scrutinee.clone()),
    };
    infer(&env, &Context::new(), &eliminator)
        .expect("kernel checks composed nested-former eliminator");
    iota_reduct(
        &env,
        outer_decl,
        0,
        &[],
        &[],
        match &eliminator {
            Term::Elim { motive, .. } => motive,
            _ => unreachable!(),
        },
        match &eliminator {
            Term::Elim { methods, .. } => methods,
            _ => unreachable!(),
        },
        &[Term::app(constructor(nested_leaf), outer_type.clone())],
    )
    .expect("composed nested-former iota term is generated");
    assert_eq!(whnf(&env, &Context::new(), &eliminator), scrutinee);
}
