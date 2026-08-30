use std::process::Command;

use ken_kernel::env::Context;
use ken_kernel::term::{GlobalId, Level, LevelVar, Term};
use ken_kernel::{
    convert, convert_type, declare_inductive, declare_recursive_group, CtorSpec, GlobalEnv,
    InductiveSpec,
};

const CONVERSION_STACK_BYTES: usize = 2 * 1024 * 1024;
const U: LevelVar = LevelVar(0);

fn level_u() -> Level {
    Level::Var(U)
}

fn cref(id: GlobalId) -> Term {
    Term::Const {
        id,
        level_args: vec![level_u()],
    }
}

fn list_at(list: GlobalId, element: Term) -> Term {
    Term::app(Term::indformer(list, vec![level_u()]), element)
}

fn declare_list(env: &mut GlobalEnv) -> (GlobalId, GlobalId, GlobalId) {
    let list = declare_inductive(env, |list| InductiveSpec {
        level_params: vec![U],
        params: vec![Term::Type(level_u())],
        indices: vec![],
        level: level_u(),
        constructors: vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::var(0), list_at(list, Term::var(1))],
                target_indices: vec![],
            },
        ],
    })
    .expect("List admission");
    let constructors = &env.inductive(list).expect("List lookup").constructors;
    (list, constructors[0].id, constructors[1].id)
}

fn map_type(list: GlobalId) -> Term {
    let type_u = Term::Type(level_u());
    Term::pi(
        type_u.clone(),
        Term::pi(
            type_u,
            Term::pi(
                Term::pi(Term::var(1), Term::var(1)),
                Term::pi(list_at(list, Term::var(2)), list_at(list, Term::var(2))),
            ),
        ),
    )
}

fn map_body(list: GlobalId, nil: GlobalId, cons: GlobalId, self_id: GlobalId) -> Term {
    let type_u = Term::Type(level_u());
    let list_a = list_at(list, Term::var(3));
    let list_b_under_motive = list_at(list, Term::var(3));
    let motive = Term::Ascript(
        Box::new(Term::lam(list_a.clone(), list_b_under_motive)),
        Box::new(Term::pi(list_a, type_u.clone())),
    );
    let nil_b = Term::app(Term::constructor(nil, vec![level_u()]), Term::var(2));
    let cons_method = Term::lam(
        Term::var(3),
        Term::lam(
            list_at(list, Term::var(4)),
            Term::lam(
                list_at(list, Term::var(4)),
                Term::app(
                    Term::app(
                        Term::app(Term::constructor(cons, vec![level_u()]), Term::var(5)),
                        Term::app(Term::var(4), Term::var(2)),
                    ),
                    Term::app(
                        Term::app(
                            Term::app(Term::app(cref(self_id), Term::var(6)), Term::var(5)),
                            Term::var(4),
                        ),
                        Term::var(1),
                    ),
                ),
            ),
        ),
    );
    let elim = Term::Elim {
        fam: list,
        level_args: vec![level_u()],
        params: vec![Term::var(3)],
        motive: Box::new(motive),
        methods: vec![nil_b, cons_method],
        indices: vec![],
        scrut: Box::new(Term::var(0)),
    };
    Term::lam(
        type_u.clone(),
        Term::lam(
            type_u,
            Term::lam(
                Term::pi(Term::var(1), Term::var(1)),
                Term::lam(list_at(list, Term::var(2)), elim),
            ),
        ),
    )
}

fn declare_map(env: &mut GlobalEnv, list: GlobalId, nil: GlobalId, cons: GlobalId) -> GlobalId {
    let ty = map_type(list);
    declare_recursive_group(env, vec![(vec![U], ty)], |ids| {
        vec![map_body(list, nil, cons, ids[0])]
    })
    .expect("recursive map must be SCT-admitted")[0]
}

fn run_query() {
    let mut env = GlobalEnv::new();
    let (list, nil, cons) = declare_list(&mut env);
    let map_f = declare_map(&mut env, list, nil, cons);
    let map_g = declare_map(&mut env, list, nil, cons);
    let ty = map_type(list);
    let ctx = Context::new();
    let types_convert = convert_type(&env, &ctx, &ty, &ty);
    eprintln!("D0_TYPE_VERDICT={types_convert}");
    assert!(types_convert);
    let value = convert(&env, &ctx, &ty, &cref(map_f), &cref(map_g));
    eprintln!("D0_VALUE_VERDICT={value}");
    assert!(!value);
}

#[test]
#[ignore = "D0 diagnostic child: expected to stack-overflow before the repair"]
fn d0_distinct_recursive_map_child() {
    std::thread::Builder::new()
        .name("d0-conversion-worker".to_string())
        .stack_size(CONVERSION_STACK_BYTES)
        .spawn(run_query)
        .expect("spawn fixed-stack worker")
        .join()
        .expect("worker must return normally");
}

#[test]
fn d0_distinct_recursive_map_requires_normal_child_exit() {
    let status = Command::new(std::env::current_exe().expect("current test binary"))
        .env_remove("RUST_MIN_STACK")
        .arg("--exact")
        .arg("d0_distinct_recursive_map_child")
        .arg("--ignored")
        .arg("--nocapture")
        .status()
        .expect("launch fixed-stack child");
    assert!(
        status.success(),
        "conversion child must exit normally; status={status}"
    );
}
