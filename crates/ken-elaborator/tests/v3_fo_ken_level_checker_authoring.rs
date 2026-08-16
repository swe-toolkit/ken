//! `V3-FO-KEN-LEVEL-CHECKER-AUTHORING`, increment 1 (`D0`+`D1`): a
//! differential control between the Ken-authored `fok_embed`
//! (`catalog/packages/Tooling/Verification/FoKripke.ken`) and the Rust
//! reference `ken_elaborator::fo_kripke::embed`, on the same quoted `IForm`
//! inputs. The Rust side is the REFERENCE for shape only (`23 §4.3-§4.5`);
//! the Ken source was authored independently and is checked here for
//! agreement, never copied from Rust behavior.
//!
//! Scope: `D0` (the quoted-syntax types) and `D1` (`embed`) only. No
//! `check_cert`, no differential over certificates, no FO `Proved`.

use std::collections::BTreeSet;

use ken_elaborator::fo_kripke::{embed, Form, IForm, IVar, QTerm};
use ken_elaborator::ElabEnv;
use ken_interp::{eval, EvalStore, EvalVal};
use ken_kernel::GlobalId;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

struct FokIds {
    fbottom: GlobalId,
    faccess: GlobalId,
    fdomain_a: GlobalId,
    fforcing_p: GlobalId,
    fand: GlobalId,
    for_: GlobalId,
    fimp: GlobalId,
    fforall_world: GlobalId,
    fforall_obj: GlobalId,
    qbound: GlobalId,
    qparameter: GlobalId,
    zero: GlobalId,
    suc: GlobalId,
}

impl FokIds {
    fn resolve(env: &ElabEnv) -> Self {
        let id = |name: &str| {
            *env.globals
                .get(name)
                .unwrap_or_else(|| panic!("missing global {name}"))
        };
        FokIds {
            fbottom: id("FokBottom"),
            faccess: id("FokAccess"),
            fdomain_a: id("FokDomainA"),
            fforcing_p: id("FokForcingP"),
            fand: id("FokAnd"),
            for_: id("FokOr"),
            fimp: id("FokImp"),
            fforall_world: id("FokForallWorld"),
            fforall_obj: id("FokForallObj"),
            qbound: id("FokQBound"),
            qparameter: id("FokQParameter"),
            zero: id("Zero"),
            suc: id("Suc"),
        }
    }
}

fn decode_nat(ids: &FokIds, v: &EvalVal) -> usize {
    match v {
        EvalVal::Ctor { id, args, .. } if *id == ids.zero => {
            assert!(args.is_empty(), "Zero must carry no args");
            0
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.suc => {
            assert_eq!(args.len(), 1, "Suc must carry exactly one arg");
            1 + decode_nat(ids, &args[0])
        }
        other => panic!("expected a Nat Ctor (Zero/Suc), got {other:?}"),
    }
}

fn decode_qterm(ids: &FokIds, v: &EvalVal) -> QTerm {
    match v {
        EvalVal::Ctor { id, args, .. } if *id == ids.qbound => {
            QTerm::Bound(decode_nat(ids, &args[0]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.qparameter => {
            QTerm::Parameter(decode_nat(ids, &args[0]))
        }
        other => panic!("expected a FokQTerm Ctor, got {other:?}"),
    }
}

fn decode_form(ids: &FokIds, v: &EvalVal) -> Form {
    match v {
        EvalVal::Ctor { id, .. } if *id == ids.fbottom => Form::Bottom,
        EvalVal::Ctor { id, args, .. } if *id == ids.faccess => {
            Form::Access(decode_qterm(ids, &args[0]), decode_qterm(ids, &args[1]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fdomain_a => {
            Form::DomainA(decode_qterm(ids, &args[0]), decode_qterm(ids, &args[1]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fforcing_p => {
            Form::ForcingP(decode_qterm(ids, &args[0]), decode_qterm(ids, &args[1]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fand => Form::And(
            Box::new(decode_form(ids, &args[0])),
            Box::new(decode_form(ids, &args[1])),
        ),
        EvalVal::Ctor { id, args, .. } if *id == ids.for_ => Form::Or(
            Box::new(decode_form(ids, &args[0])),
            Box::new(decode_form(ids, &args[1])),
        ),
        EvalVal::Ctor { id, args, .. } if *id == ids.fimp => Form::Imp(
            Box::new(decode_form(ids, &args[0])),
            Box::new(decode_form(ids, &args[1])),
        ),
        EvalVal::Ctor { id, args, .. } if *id == ids.fforall_world => {
            Form::ForallWorld(Box::new(decode_form(ids, &args[0])))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fforall_obj => {
            Form::ForallObj(Box::new(decode_form(ids, &args[0])))
        }
        other => panic!("expected a FokForm Ctor, got {other:?}"),
    }
}

/// Renders `n` as a `Nat` literal, e.g. `nat_source(2)` = `"(Suc (Suc
/// Zero))"`. The result is always a single argument-safe token: bare `Zero`
/// for `0`, fully parenthesized otherwise.
fn nat_source(n: usize) -> String {
    if n == 0 {
        "Zero".to_string()
    } else {
        format!("(Suc {})", nat_source(n - 1))
    }
}

/// Renders an `IForm` as Ken source text over the `FokIForm` constructors,
/// independently of `embed`/`fok_embed` -- this function only serializes the
/// INPUT, never computes the translation under test.
fn iform_source(f: &IForm) -> String {
    match f {
        IForm::Bottom => "FokIBottom".to_string(),
        IForm::Atom(IVar(k)) => format!("(FokIAtom (FokMkIVar {}))", nat_source(*k)),
        IForm::Or(p, q) => format!("(FokIOr {} {})", iform_source(p), iform_source(q)),
        IForm::Imp(p, q) => format!("(FokIImp {} {})", iform_source(p), iform_source(q)),
        IForm::Forall(p) => format!("(FokIForall {})", iform_source(p)),
    }
}

/// Elaborates `f` as a Ken `FokIForm` constant, evaluates `fok_embed` on it,
/// and decodes the result back to the Rust `Form` type for comparison.
fn ken_embed(env: &mut ElabEnv, ids: &FokIds, case: &str, f: &IForm) -> Form {
    let input_name = format!("fok_case_{case}_input");
    let output_name = format!("fok_case_{case}_output");
    env.elaborate_decl(&format!(
        "const {input_name} : FokIForm = {}",
        iform_source(f)
    ))
    .unwrap_or_else(|e| panic!("case {case}: input elaboration failed: {e}"));
    let output_id = env
        .elaborate_decl(&format!(
            "const {output_name} : FokForm = fok_embed {input_name}"
        ))
        .unwrap_or_else(|e| panic!("case {case}: fok_embed elaboration failed: {e}"));
    let (_, body) = env
        .env
        .transparent_body(output_id)
        .unwrap_or_else(|| panic!("case {case}: output is not a transparent definition"));
    let mut store = EvalStore::new();
    let value = eval(&[], &body, &env.env, &mut store);
    decode_form(ids, &value)
}

/// The five `IForm` constructors, individually and combined. `IForm::Atom`
/// is only well-scoped under an enclosing `IForm::Forall` (its `IVar` must
/// name a bound object variable) -- exactly the invariant `quote_iform`
/// enforces on its callers (`fo_kripke.rs`'s doc comment above `IVar`), so
/// every case below wraps its atoms accordingly; a bare top-level
/// `IForm::Atom` is not a valid slice input and `embed` panics on it
/// (confirmed empirically: the Rust reference indexes an empty
/// `object_env`). The two doubly-nested cases exercise `IVar` indices above
/// `0`, so `fok_nth_nat_default`'s `Suc` arm and the world/object shift
/// bookkeeping in both `w_forces` and `fok_w_forces` are exercised under
/// real nesting, not only at the trivial depth-zero index.
fn cases() -> Vec<(&'static str, IForm)> {
    vec![
        ("bottom", IForm::Bottom),
        (
            "atom_under_forall",
            IForm::Forall(Box::new(IForm::Atom(IVar(0)))),
        ),
        (
            "or",
            IForm::Forall(Box::new(IForm::Or(
                Box::new(IForm::Bottom),
                Box::new(IForm::Atom(IVar(0))),
            ))),
        ),
        (
            "imp",
            IForm::Forall(Box::new(IForm::Imp(
                Box::new(IForm::Atom(IVar(0))),
                Box::new(IForm::Bottom),
            ))),
        ),
        (
            "forall_nested_inner_ref",
            IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Atom(IVar(0)))))),
        ),
        (
            "forall_nested_outer_ref",
            IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Atom(IVar(1)))))),
        ),
        (
            "nested_mixed",
            IForm::Forall(Box::new(IForm::Imp(
                Box::new(IForm::Atom(IVar(0))),
                Box::new(IForm::Or(
                    Box::new(IForm::Atom(IVar(0))),
                    Box::new(IForm::Bottom),
                )),
            ))),
        ),
    ]
}

#[test]
fn fok_embed_agrees_with_rust_embed_on_quoted_inputs() {
    let mut env = ElabEnv::new().expect("prelude construction");
    let before_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let after_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before_trust, after_trust,
        "loading FoKripke.ken must add no trusted_base entry (AC-2 of the node)"
    );

    let ids = FokIds::resolve(&env);

    for (case, f) in cases() {
        let expected = embed(&f);
        let actual = ken_embed(&mut env, &ids, case, &f);
        assert_eq!(
            actual, expected,
            "case {case}: Ken fok_embed disagrees with Rust embed on {f:?}"
        );
    }
}

#[test]
fn fok_iform_has_exactly_five_slice_constructors() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ind = env
        .env
        .inductive(*env.globals.get("FokIForm").expect("FokIForm registered"))
        .expect("FokIForm is an inductive");
    assert_eq!(
        ind.constructors.len(),
        5,
        "FokIForm must carry exactly the five slice constructors (23 §4.5)"
    );
}

#[test]
fn fok_rule_has_exactly_three_slice_variants() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ind = env
        .env
        .inductive(*env.globals.get("FokRule").expect("FokRule registered"))
        .expect("FokRule is an inductive");
    assert_eq!(
        ind.constructors.len(),
        3,
        "FokRule must carry exactly the three slice variants, against the general \
         23 §4.3 Rule's ~20 (init, imp-right, forall-right only)"
    );
}
