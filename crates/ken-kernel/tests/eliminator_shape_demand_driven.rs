use std::collections::BTreeSet;

use ken_kernel::inductive::{method_type, peel_pi, recursive_shapes, RecursiveShape};
use ken_kernel::{
    declare_inductive, ConstructorDecl, CtorSpec, Decl, GlobalEnv, InductiveDecl, InductiveSpec,
    Level, Term,
};

fn ty0() -> Term {
    Term::Type(Level::zero())
}

fn declare_positive_carrier(env: &mut GlobalEnv) -> ken_kernel::GlobalId {
    declare_inductive(env, |_carrier| InductiveSpec {
        level_params: vec![],
        params: vec![ty0()],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![Term::var(0)],
            target_indices: vec![],
        }],
    })
    .expect("strictly positive carrier")
}

/// Durable invariant: transparent delta closure participates in the recursive
/// shape classifier. A syntactic occurrence check would classify this field as
/// D-free and remove the method's induction-hypothesis binder.
#[test]
fn transparent_recursive_argument_keeps_its_induction_hypothesis() {
    let mut env = GlobalEnv::new();
    let carrier = declare_positive_carrier(&mut env);
    let family = env.fresh_id();
    let constructor = env.fresh_id();
    let wrap = env.fresh_id();
    let wrap_alias = env.fresh_id();

    env.add_decl(Decl::Transparent {
        id: wrap,
        level_params: vec![],
        ty: ty0(),
        body: Term::indformer(family, vec![]),
    });
    env.add_decl(Decl::Transparent {
        id: wrap_alias,
        level_params: vec![],
        ty: ty0(),
        body: Term::const_(wrap, vec![]),
    });

    let mut declaration = InductiveDecl {
        id: family,
        level_params: vec![],
        params: vec![],
        parameter_polarities: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![ConstructorDecl {
            id: constructor,
            args: vec![Term::app(
                Term::indformer(carrier, vec![]),
                Term::const_(wrap_alias, vec![]),
            )],
            target_indices: vec![],
            type_: ty0(),
            recursive_positions: vec![],
        }],
        former_type: ty0(),
    };
    declaration.build_types();
    env.add_decl(Decl::Inductive(declaration));

    let before: BTreeSet<_> = env.trusted_base().into_iter().collect();
    let declaration = env.inductive(family).expect("test family");
    let shapes = recursive_shapes(&env, &declaration.constructors[0], family, 0)
        .expect("delta-hidden recursive shape");
    assert_eq!(
        shapes.len(),
        1,
        "the recursive field must not become D-free"
    );
    assert!(matches!(shapes[0].shape, RecursiveShape::Former { .. }));
    assert_eq!(shapes[0].shape.leaf_count(), 1);

    let family_type = Term::indformer(family, vec![]);
    let motive = Term::Ascript(
        Box::new(Term::lam(family_type.clone(), ty0())),
        Box::new(Term::pi(family_type, Term::Type(Level::zero().suc()))),
    );
    let method = method_type(&env, declaration, 0, &motive, &[], &[])
        .expect("method type with delta-hidden recursive field");
    let (domains, _) = peel_pi(&method);
    assert_eq!(
        domains.len(),
        2,
        "the method must bind both the field and its induction hypothesis"
    );

    let after: BTreeSet<_> = env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "shape derivation must not grow trusted_base");
}
