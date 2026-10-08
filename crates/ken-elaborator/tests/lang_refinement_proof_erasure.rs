//! Typed Ω erasure reaches the emitted package, the native runtime IR and the
//! interpreter through one kernel-classified plan.

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::checked_core::{
    canonical_decl_bytes, emit_checked_core_package, CheckedCoreArtifactInputs, CheckedCorePackage,
    CheckedCorePackageHeader, CheckedCoreSemanticInputs, LowerabilityStatus, StableSymbol,
    StableSymbolTable, SymbolNamespace,
};
use ken_elaborator::erasure::{erase_checked_core_package_for_target, ErasureError};
use ken_elaborator::omega_erasure::omega_erasure_plan;
use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval_checked, EvalStore, EvalVal};
use ken_kernel::{declare_def, declare_postulate, Context, Decl, GlobalEnv, GlobalId, Term};
use ken_runtime::{
    evaluate_runtime_ir_expr, RuntimeDeclarationKind, RuntimeExpr, RuntimeGroundValue,
    RuntimeIrSeedEnvironment, RuntimeObservation,
};
use num_bigint::BigInt;

fn int(n: i64) -> Term {
    Term::IntLit(BigInt::from(n))
}

fn sym(name: &str) -> StableSymbol {
    StableSymbol::declaration("omega_plan", &[], name)
}

fn package(env: &GlobalEnv, named: &[(GlobalId, &str)], bodies: &[GlobalId]) -> CheckedCorePackage {
    let mut table = StableSymbolTable::new();
    let mut semantic = CheckedCoreSemanticInputs::default();
    for (id, name) in named {
        let symbol = sym(name);
        table.insert_global(*id, symbol.clone());
        semantic.symbols.insert(symbol);
    }
    for id in bodies {
        let decl = env.lookup(*id).expect("checked declaration exists");
        let Decl::Transparent { body, ty, .. } = decl else {
            panic!("body must be admitted transparently");
        };
        let symbol = named
            .iter()
            .find(|(candidate, _)| candidate == id)
            .map(|(_, name)| sym(name))
            .expect("stable symbol exists");
        semantic.declarations.insert(
            symbol.clone(),
            canonical_decl_bytes(decl, &table).expect("canonical body"),
        );
        semantic.omega_erasure_plans.insert(
            symbol.clone(),
            omega_erasure_plan(env, body, ty).expect("kernel classifies the checked body"),
        );
        semantic
            .lowerability
            .insert(symbol, LowerabilityStatus::Supported);
    }
    emit_checked_core_package(
        CheckedCorePackageHeader::v0(
            "ken-elaborator:omega-plan-test",
            "ken-kernel:test",
            "spec/40-runtime/46",
            "spec/10-kernel/18a",
            StableSymbol::new(SymbolNamespace::Module, vec!["omega_plan".to_string()]),
        ),
        CheckedCoreArtifactInputs {
            semantic,
            source_identity: BTreeMap::new(),
            annotations: BTreeMap::new(),
        },
    )
    .expect("package emits")
}

fn native_body(
    package: &CheckedCorePackage,
    target: &StableSymbol,
    selected: &[StableSymbol],
) -> RuntimeExpr {
    let program = erase_checked_core_package_for_target(package, selected.iter())
        .expect("typed package lowers");
    let decl = program
        .declarations
        .iter()
        .find(|d| d.symbol == target.to_string())
        .expect("target lowered");
    let RuntimeDeclarationKind::Transparent { body } = &decl.kind else {
        panic!("transparent runtime target required")
    };
    body.clone()
}

fn observed(body: &RuntimeExpr) -> RuntimeObservation {
    evaluate_runtime_ir_expr(body, &RuntimeIrSeedEnvironment::empty())
        .expect("runtime IR evaluates")
}

#[test]
fn subset_pair_with_open_proof_is_only_its_carrier() {
    // Promise class: durable invariant (42 §3.2, 46 §4, 47 §1).
    // MEASURED: the same checked Σ pair returns Int 7 through native IR and
    // interpreter despite an opaque open proof; native never encodes a pair.
    // CLAIMED: a kernel-Ω-classified codomain erases the proof and collapses
    // the pair. THE GAP: the emitted plan must be the one both paths consume.
    let mut elaborated = ElabEnv::new().expect("prelude admits");
    let int_id = elaborated.globals["Int"];
    let int_ty = Term::const_(int_id, vec![]);
    let eq = |x: Term| Term::Eq(Box::new(int_ty.clone()), Box::new(x.clone()), Box::new(x));
    let sigma = Term::sigma(int_ty.clone(), eq(Term::var(0)));
    let hole = declare_postulate(&mut elaborated.env, "open_proof".into(), vec![], eq(int(7)))
        .expect("typed open proof");
    let pair = Term::pair(int(7), Term::const_(hole, vec![]));
    ken_kernel::check(&elaborated.env, &Context::new(), &pair, &sigma)
        .expect("kernel checks subset pair");
    let pair_id = declare_def(&mut elaborated.env, vec![], sigma.clone(), pair.clone())
        .expect("admitted pair");
    let package = package(
        &elaborated.env,
        &[(int_id, "Int"), (hole, "hole"), (pair_id, "pair")],
        &[pair_id],
    );
    let target = sym("pair");
    let plan = &package.artifact.semantic.omega_erasure_plans[&target];
    assert_eq!(plan.collapsed_sigmas, BTreeSet::from([0]));
    assert_eq!(plan.erased_subterms, BTreeSet::from([2]));
    assert_eq!(
        observed(&native_body(&package, &target, &[target.clone()])),
        RuntimeObservation::Returned(RuntimeGroundValue::Int(7.into()))
    );
    let mut store = EvalStore::new();
    assert!(matches!(
        eval_checked(&pair, &sigma, &elaborated.env, &mut store).expect("interpreter classifies"),
        EvalVal::Int(7)
    ));

    let mut missing = package.clone();
    missing
        .artifact
        .semantic
        .omega_erasure_plans
        .remove(&target);
    let missing = emit_checked_core_package(missing.header, missing.artifact)
        .expect("valid older package remains inspectable");
    assert!(
        matches!(erase_checked_core_package_for_target(&missing, [&target]), Err(ErasureError::UnsupportedErasure { symbol, reason }) if symbol == target && reason.contains("missing kernel-classified Ω erasure plan"))
    );
}

#[test]
fn relevant_sigma_pair_keeps_both_components() {
    // Promise class: durable invariant (42 §3.2, 47 §1).
    // MEASURED: two relevant Ints survive as a two-field runtime pair and
    // interpreter pair. CLAIMED: Ω erasure never collapses a relevant Σ.
    // THE GAP: the codomain, not the first component, decides collapse.
    let mut elaborated = ElabEnv::new().expect("prelude admits");
    let int_id = elaborated.globals["Int"];
    let int_ty = Term::const_(int_id, vec![]);
    let sigma = Term::sigma(int_ty.clone(), int_ty);
    let pair = Term::pair(int(7), int(8));
    let pair_id = declare_def(&mut elaborated.env, vec![], sigma.clone(), pair.clone())
        .expect("relevant pair admits");
    let package = package(
        &elaborated.env,
        &[(int_id, "Int"), (pair_id, "pair")],
        &[pair_id],
    );
    let target = sym("pair");
    let plan = &package.artifact.semantic.omega_erasure_plans[&target];
    assert!(plan.collapsed_sigmas.is_empty());
    let native = native_body(&package, &target, &[target.clone()]);
    assert_eq!(
        observed(&native),
        RuntimeObservation::Returned(RuntimeGroundValue::Record {
            fields: vec![
                ("first".into(), RuntimeGroundValue::Int(7.into())),
                ("second".into(), RuntimeGroundValue::Int(8.into()))
            ]
        })
    );
    let mut store = EvalStore::new();
    let result =
        eval_checked(&pair, &sigma, &elaborated.env, &mut store).expect("typed interpreter");
    assert!(
        matches!(result, EvalVal::Pair { fst, snd, .. } if matches!(*fst, EvalVal::Int(7)) && matches!(*snd, EvalVal::Int(8)))
    );
}

#[test]
fn alias_hidden_proof_argument_erases_binder_and_application_slot() {
    // Promise class: durable invariant (42 §3.2, 46 §4).
    // MEASURED: an alias-hidden Eq hole is never evaluated, and the native
    // callee has zero runtime parameters. CLAIMED: erasure is keyed on Ω sort
    // rather than Eq or refl spelling. THE GAP: both target and callee plans
    // are selected from the same checked environment.
    let mut elaborated = ElabEnv::new().expect("prelude admits");
    let int_id = elaborated.globals["Int"];
    let int_ty = Term::const_(int_id, vec![]);
    let eq = Term::Eq(Box::new(int_ty.clone()), Box::new(int(7)), Box::new(int(7)));
    let alias = declare_def(
        &mut elaborated.env,
        vec![],
        Term::Omega(ken_kernel::Level::zero()),
        eq,
    )
    .expect("alias proposition admits");
    let p = Term::const_(alias, vec![]);
    let hole = declare_postulate(
        &mut elaborated.env,
        "opaque_proof".into(),
        vec![],
        p.clone(),
    )
    .expect("typed proof hole");
    // The returned subset proof is the bound variable. If the interpreter
    // unfolds `f` without its plan, this result stays a pair (or Unknown)
    // instead of the carrier; the variable arm is independently observable.
    let subset = Term::sigma(
        int_ty.clone(),
        Term::Eq(
            Box::new(int_ty.clone()),
            Box::new(Term::var(0)),
            Box::new(Term::var(0)),
        ),
    );
    let f_ty = Term::pi(p.clone(), subset.clone());
    let f = declare_def(
        &mut elaborated.env,
        vec![],
        f_ty.clone(),
        Term::lam(p.clone(), Term::pair(int(7), Term::var(0))),
    )
    .expect("proof lambda returning subset pair admits");
    let g = declare_def(
        &mut elaborated.env,
        vec![],
        f_ty,
        Term::lam(p.clone(), Term::app(Term::const_(f, vec![]), Term::var(0))),
    )
    .expect("proof variable passed into another binder admits");
    let call = Term::app(Term::const_(g, vec![]), Term::const_(hole, vec![]));
    let target = declare_def(&mut elaborated.env, vec![], subset.clone(), call.clone())
        .expect("proof application admits");
    let package = package(
        &elaborated.env,
        &[
            (int_id, "Int"),
            (alias, "AliasP"),
            (hole, "hole"),
            (f, "f"),
            (g, "g"),
            (target, "target"),
        ],
        &[alias, f, g, target],
    );
    assert_eq!(
        package.artifact.semantic.omega_erasure_plans[&sym("f")].erased_binders,
        BTreeSet::from([0])
    );
    assert_eq!(
        package.artifact.semantic.omega_erasure_plans[&sym("g")].erased_binders,
        BTreeSet::from([0])
    );
    assert!(package.artifact.semantic.omega_erasure_plans[&sym("g")]
        .erased_subterms
        .contains(&4)); // the variable argument, not `refl`
    assert!(
        package.artifact.semantic.omega_erasure_plans[&sym("target")]
            .erased_subterms
            .contains(&2)
    ); // the open hole at the alias
    let selected = vec![sym("target"), sym("g"), sym("f")];
    let lowered = native_body(&package, &sym("f"), &selected);
    assert!(matches!(lowered, RuntimeExpr::Closure { params, .. } if params.is_empty()));
    assert_eq!(
        observed(&native_body(&package, &sym("target"), &selected)),
        RuntimeObservation::Returned(RuntimeGroundValue::Int(7.into()))
    );
    let mut store = EvalStore::new();
    assert!(matches!(
        eval_checked(&call, &subset, &elaborated.env, &mut store).expect("typed interpreter"),
        EvalVal::Int(7)
    ));
}
