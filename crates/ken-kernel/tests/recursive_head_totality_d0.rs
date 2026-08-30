use std::process::Command;

use ken_kernel::env::Context;
use ken_kernel::term::{GlobalId, Level, LevelVar, Term};
use ken_kernel::{
    convert, convert_type, declare_def, declare_inductive, declare_recursive_group, infer, CtorSpec,
    GlobalEnv, InductiveSpec,
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

fn map_body_gen(
    list: GlobalId,
    nil: GlobalId,
    cons: GlobalId,
    rec_target: GlobalId,
    dup: bool,
) -> Term {
    let type_u = Term::Type(level_u());
    let list_a = list_at(list, Term::var(3));
    let list_b_under_motive = list_at(list, Term::var(3));
    let motive = Term::Ascript(
        Box::new(Term::lam(list_a.clone(), list_b_under_motive)),
        Box::new(Term::pi(list_a, type_u.clone())),
    );
    let nil_b = Term::app(Term::constructor(nil, vec![level_u()]), Term::var(2));
    let head_image = Term::app(Term::var(4), Term::var(2));
    let rec_call = Term::app(
        Term::app(
            Term::app(Term::app(cref(rec_target), Term::var(6)), Term::var(5)),
            Term::var(4),
        ),
        Term::var(1),
    );
    let one_cons = |tail: Term| {
        Term::app(
            Term::app(
                Term::app(Term::constructor(cons, vec![level_u()]), Term::var(5)),
                head_image.clone(),
            ),
            tail,
        )
    };
    let cons_body = if dup {
        one_cons(one_cons(rec_call))
    } else {
        one_cons(rec_call)
    };
    let cons_method = Term::lam(
        Term::var(3),
        Term::lam(
            list_at(list, Term::var(4)),
            Term::lam(list_at(list, Term::var(4)), cons_body),
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
        vec![map_body_gen(list, nil, cons, ids[0], false)]
    })
    .expect("recursive map must be SCT-admitted")[0]
}

// Control 1: a genuinely different self-recursive function (emits each element
// twice) — SCT-admitted, distinct from `map`, open comparison must be false.
fn declare_map_twice(env: &mut GlobalEnv, list: GlobalId, nil: GlobalId, cons: GlobalId) -> GlobalId {
    let ty = map_type(list);
    declare_recursive_group(env, vec![(vec![U], ty)], |ids| {
        vec![map_body_gen(list, nil, cons, ids[0], true)]
    })
    .expect("double-image map must be SCT-admitted")[0]
}

// Control 2: a map whose recursive edge references `target` rather than itself,
// so its unfolded body equals `target`'s and the open comparison must be true.
fn declare_map_delegating(
    env: &mut GlobalEnv,
    list: GlobalId,
    nil: GlobalId,
    cons: GlobalId,
    target: GlobalId,
) -> GlobalId {
    let ty = map_type(list);
    declare_def(env, vec![U], ty, map_body_gen(list, nil, cons, target, false))
        .expect("delegating map admission")
}

fn cref_at(id: GlobalId, level: Level) -> Term {
    Term::Const {
        id,
        level_args: vec![level],
    }
}
fn cref0(id: GlobalId) -> Term {
    Term::Const {
        id,
        level_args: vec![],
    }
}
fn type0() -> Term {
    Term::Type(Level::zero())
}
// Well-typed Type-0 carriers/values (the seed's `Bool`/`not`). `Type 0` itself
// inhabits `Type (suc 0)`, so it cannot be a `map` carrier; `Bool : Type 0`
// with `true`/`false` and `not : Bool -> Bool` are the normative inputs.
fn declare_bool(env: &mut GlobalEnv) -> (GlobalId, GlobalId, GlobalId) {
    let b = declare_inductive(env, |_b| InductiveSpec {
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
    .expect("Bool admission");
    let ctors = &env.inductive(b).expect("Bool lookup").constructors;
    (b, ctors[0].id, ctors[1].id)
}
fn bool_ty(bool_id: GlobalId) -> Term {
    Term::indformer(bool_id, vec![])
}
fn bool_ctor(id: GlobalId) -> Term {
    Term::constructor(id, vec![])
}
fn declare_not(env: &mut GlobalEnv, bool_id: GlobalId, false_id: GlobalId, true_id: GlobalId) -> GlobalId {
    let bt = bool_ty(bool_id);
    let motive = Term::Ascript(
        Box::new(Term::lam(bt.clone(), bt.clone())),
        Box::new(Term::pi(bt.clone(), Term::Type(Level::zero()))),
    );
    let body = Term::lam(
        bt.clone(),
        Term::Elim {
            fam: bool_id,
            level_args: vec![],
            params: vec![],
            motive: Box::new(motive),
            methods: vec![bool_ctor(true_id), bool_ctor(false_id)],
            indices: vec![],
            scrut: Box::new(Term::var(0)),
        },
    );
    declare_def(env, vec![], Term::pi(bt.clone(), bt), body).expect("not admission")
}
/// Positive typing observation: the fixture is well-typed before conversion,
/// so a green verdict is over a normative input (conversion accepts raw terms).
fn assert_typed(env: &GlobalEnv, ctx: &Context, t: &Term) {
    assert!(
        infer(env, ctx, t).is_ok(),
        "fixture must be well-typed before conversion; infer error: {:?}",
        infer(env, ctx, t).err()
    );
}
fn nil_val(nil: GlobalId, ty: Term) -> Term {
    Term::app(Term::constructor(nil, vec![Level::zero()]), ty)
}
fn cons_val(cons: GlobalId, ty: Term, head: Term, tail: Term) -> Term {
    Term::app(
        Term::app(Term::app(Term::constructor(cons, vec![Level::zero()]), ty), head),
        tail,
    )
}
fn map_apply(self_id: GlobalId, a: Term, b: Term, f: Term, xs: Term) -> Term {
    Term::app(
        Term::app(Term::app(Term::app(cref_at(self_id, Level::zero()), a), b), f),
        xs,
    )
}

/// The whole recursive-head-totality matrix, exercised inside the fixed 2 MiB
/// worker so that every case is proven to run under a bounded stack (the
/// divergence would otherwise exhaust it). Black-box behavioural twin of the
/// `conv::tests` counter matrix: four framed cases plus the closed Nil and
/// two-Cons positives, all reached only through the public conversion API.
///
/// Traceability. This is the durable executable twin of
/// `conformance/kernel/conversion/seed-conversion.md`,
/// `delta-distinct-recursive-heads-stuck` (promise class: durable invariant),
/// under that fixture's fixed 2 MiB `CONVERSION_STACK_BYTES` normal-exit
/// instrument. Governing spec: `17 §3.5` (distinct recursive-identity boundary)
/// and `17 §5` obligation 3 (cross-identity symbolic retry is not SCT-certified;
/// SCT is `17 §4` and is explicitly NOT this rule). Carriers/values are the
/// seed's well-typed `Bool`/`not`/`true`/`false` (`Type 0` inhabitants), each
/// checked well-typed by `assert_typed` before conversion. Authored
/// traceability, not an executable scan of repository text.
fn run_query() {
    let mut env = GlobalEnv::new();
    let (list, nil, cons) = declare_list(&mut env);
    let map_f = declare_map(&mut env, list, nil, cons);
    let map_g = declare_map(&mut env, list, nil, cons);
    // Well-typed level-0 carriers/values (the seed's Bool/not). `Bool : Type 0`.
    let (bool_id, false_id, true_id) = declare_bool(&mut env);
    let not = declare_not(&mut env, bool_id, false_id, true_id);
    let bt = bool_ty(bool_id);
    let list_bool = || Term::app(Term::indformer(list, vec![Level::zero()]), bt.clone());
    let ty = map_type(list);
    let ctx = Context::new();

    // Framed case 1 — the type of `map` converts with itself (syntactic, zero δ).
    let types_convert = convert_type(&env, &ctx, &ty, &ty);
    eprintln!("D0_TYPE_VERDICT={types_convert}");
    assert!(types_convert);

    // Framed case 2 (zero δ unfold) — same `map_f`/`not` spine, differing only
    // in an ASCRIBED list argument (convertible via the ascription arm, no δ),
    // both well-typed. Closes via the spine fast path without unfolding a head.
    let case2_a = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt.clone()));
    let ascribed_nil = Term::Ascript(Box::new(nil_val(nil, bt.clone())), Box::new(list_bool()));
    let case2_b = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), ascribed_nil);
    assert_typed(&env, &ctx, &case2_a);
    assert_typed(&env, &ctx, &case2_b);
    let case2 = convert_type(&env, &ctx, &case2_a, &case2_b);
    eprintln!("D0_CASE2_SAME_CONST_SPINE={case2}");
    assert!(case2);

    // Framed case 3 (finite δ retry, true) — distinct convertible constants:
    // `f := Type0`, `h := f` (well-typed at `Type1`). Captures, converges, never
    // refuses. (Here `Type0` is a valid inhabitant of the constant's `Type1`.)
    let type1 = Term::Type(Level::suc(Level::zero()));
    let f = declare_def(&mut env, vec![], type1.clone(), type0()).expect("f admission");
    let h = declare_def(&mut env, vec![], type1, cref0(f)).expect("h admission");
    assert_typed(&env, &ctx, &cref0(f));
    assert_typed(&env, &ctx, &cref0(h));
    let case3 = convert_type(&env, &ctx, &cref0(f), &cref0(h));
    eprintln!("D0_CASE3_DISTINCT_ALIAS={case3}");
    assert!(case3);

    // Framed case 4 (open recursive, false) — the D0 divergence: two distinct
    // SOURCE-ISOMORPHIC recursive maps at a neutral scrutinee. Terminates with
    // `false` via the divergence refusal (not a body difference).
    assert_typed(&env, &ctx, &cref(map_f));
    assert_typed(&env, &ctx, &cref(map_g));
    let value = convert(&env, &ctx, &ty, &cref(map_f), &cref(map_g));
    eprintln!("D0_VALUE_VERDICT={value}");
    assert!(!value);

    // Control 1 (frame AC-MATRIX case 4) — a SEPARATELY admitted recursive
    // function whose constructor equation genuinely differs (emits each element
    // twice). The open comparison is false for a SEMANTIC reason (the bodies
    // diverge structurally), distinct from the source-isomorphic false above.
    let map_twice = declare_map_twice(&mut env, list, nil, cons);
    assert_typed(&env, &ctx, &cref(map_twice));
    let control_diff = convert(&env, &ctx, &ty, &cref(map_f), &cref(map_twice));
    eprintln!("D0_CONTROL_DIFFERENT_FUNCTION={control_diff}");
    assert!(!control_diff);

    // Refusal-impossible witness that map_twice is a genuinely different
    // FUNCTION (not merely a distinct constant): on a CLOSED list the divergence
    // guard cannot arm (ι drives both applications to closed normal forms), so
    // `map_f L ≢ map_twice L` being false is a value/structural inequality, NOT a
    // ledger refusal. With f = not: map_f [true,false] = [false,true];
    // map_twice [true,false] = [false,false,true,true]. The counter twin
    // (`refusals() == 0`) is asserted in
    // `conv::tests::control_genuinely_different_recursive_function_is_false_without_refusal`.
    let closed_ab = cons_val(
        cons,
        bt.clone(),
        bool_ctor(true_id),
        cons_val(cons, bt.clone(), bool_ctor(false_id), nil_val(nil, bt.clone())),
    );
    let diff_lhs = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), closed_ab.clone());
    let diff_rhs = map_apply(map_twice, bt.clone(), bt.clone(), cref0(not), closed_ab);
    assert_typed(&env, &ctx, &diff_lhs);
    assert_typed(&env, &ctx, &diff_rhs);
    let control_diff_closed = convert_type(&env, &ctx, &diff_lhs, &diff_rhs);
    eprintln!("D0_CONTROL_DIFFERENT_CLOSED={control_diff_closed}");
    assert!(!control_diff_closed);

    // Control 2 (seed one-axis positive) — a separately declared map whose
    // recursive edge references `map_f`, so its unfolded body equals `map_f`'s.
    // The open comparison must FLIP to true.
    let map_g_deleg = declare_map_delegating(&mut env, list, nil, cons, map_f);
    assert_typed(&env, &ctx, &cref(map_g_deleg));
    let control_deleg = convert(&env, &ctx, &ty, &cref(map_f), &cref(map_g_deleg));
    eprintln!("D0_CONTROL_DELEGATING_TRUE={control_deleg}");
    assert!(control_deleg);

    // Closed Nil positive — `map_f Nil ≡ map_g Nil` reduces by ι to `Nil B`.
    let nil_lhs = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt.clone()));
    let nil_rhs = map_apply(map_g, bt.clone(), bt.clone(), cref0(not), nil_val(nil, bt.clone()));
    assert_typed(&env, &ctx, &nil_lhs);
    assert_typed(&env, &ctx, &nil_rhs);
    let nil_pos = convert_type(&env, &ctx, &nil_lhs, &nil_rhs);
    eprintln!("D0_CLOSED_NIL_POSITIVE={nil_pos}");
    assert!(nil_pos);

    // Closed two-Cons positive — `map_f L ≡ map_g L` for a closed 2-element
    // list; ι-discharge keeps the lock-step descent refusal-free.
    let l = cons_val(
        cons,
        bt.clone(),
        bool_ctor(true_id),
        cons_val(cons, bt.clone(), bool_ctor(false_id), nil_val(nil, bt.clone())),
    );
    let cons_lhs = map_apply(map_f, bt.clone(), bt.clone(), cref0(not), l.clone());
    let cons_rhs = map_apply(map_g, bt.clone(), bt.clone(), cref0(not), l);
    assert_typed(&env, &ctx, &cons_lhs);
    assert_typed(&env, &ctx, &cons_rhs);
    let cons_pos = convert_type(&env, &ctx, &cons_lhs, &cons_rhs);
    eprintln!("D0_CLOSED_TWO_CONS_POSITIVE={cons_pos}");
    assert!(cons_pos);
}

#[test]
#[ignore = "fixed-2MiB-stack worker; run via d0_..._requires_normal_child_exit, \
            which asserts it now exits normally (it stack-overflowed pre-repair)"]
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
