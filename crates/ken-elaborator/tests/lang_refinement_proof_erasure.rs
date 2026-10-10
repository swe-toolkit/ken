//! Typed Ω erasure reaches the emitted package, the native runtime IR and the
//! interpreter through one kernel-classified plan.

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::checked_core::{
    canonical_decl_bytes, emit_checked_core_package, CheckedCoreArtifactInputs, CheckedCorePackage,
    CheckedCorePackageError, CheckedCorePackageHeader, CheckedCoreSemanticInputs,
    LowerabilityStatus, StableSymbol, StableSymbolTable, SymbolNamespace,
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

    // Projection of that checked subset pair is the identity, not a runtime
    // field read; the deferred source proof remains an admitted assumption.
    let projection = Term::proj1(Term::const_(pair_id, vec![]));
    let projection_id = declare_def(
        &mut elaborated.env,
        vec![],
        int_ty.clone(),
        projection.clone(),
    )
    .expect("checked first projection");
    let projected = crate::package(
        &elaborated.env,
        &[
            (int_id, "Int"),
            (hole, "hole"),
            (pair_id, "pair"),
            (projection_id, "project"),
        ],
        &[pair_id, projection_id],
    );
    let selected = vec![sym("project"), sym("pair")];
    assert_eq!(
        observed(&native_body(&projected, &sym("project"), &selected)),
        RuntimeObservation::Returned(RuntimeGroundValue::Int(7.into()))
    );
    assert!(matches!(
        eval_checked(&projection, &int_ty, &elaborated.env, &mut store)
            .expect("interpreter first projection"),
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

    // AC-5 structural refusal, one arm at a time. The emitter recomputes
    // both hashes for each mutated plan, so a stale hash cannot mask the
    // intended range, tag, or maximal-subtree validator arm.
    for (label, variant, reason) in [
        ("subterm_range", 0, "erased_subterms id 999 outside"),
        ("binder_range", 1, "erased_binders id 999 outside"),
        ("sigma_range", 2, "collapsed_sigmas id 999 outside"),
        ("binder_tag", 3, "erased binder id 1 has tag"),
        ("sigma_tag", 4, "collapsed Σ id 1 has tag"),
        ("not_maximal", 5, "inside maximal erased subtree"),
    ] {
        let mut invalid = package.clone();
        let plan = invalid
            .artifact
            .semantic
            .omega_erasure_plans
            .get_mut(&target)
            .unwrap();
        match variant {
            0 => {
                plan.erased_subterms.insert(999);
            }
            1 => {
                plan.erased_binders.insert(999);
            }
            2 => {
                plan.collapsed_sigmas.insert(999);
            }
            3 => {
                plan.erased_binders.insert(1);
            }
            4 => {
                plan.collapsed_sigmas.insert(1);
            }
            5 => {
                plan.erased_subterms.insert(0);
            }
            _ => unreachable!(),
        }
        let err = emit_checked_core_package(invalid.header, invalid.artifact)
            .expect_err("malformed plan must not validate");
        assert!(
            matches!(&err, CheckedCorePackageError::MalformedOmegaErasurePlan { symbol, reason: why }
                if symbol == &target && why.contains(reason)),
            "{label}: {err:?}"
        );
    }
}

#[test]
fn omega_let_binder_is_erased_without_a_runtime_binding() {
    // Promise class: durable invariant (42 §3.2, 46 §4, 47 §1).
    // MEASURED: the kernel-checked Let proof binder is in erased_binders;
    // native IR contains no Let binding and both evaluators return Int 9.
    // CLAIMED: Ω classification, not the spelling of the proof, removes the
    // bound value from runtime computation. THE GAP: this checks a closed
    // proof-bound Let; open uses of its de Bruijn variable need separate pins.
    let mut elaborated = ElabEnv::new().expect("prelude admits");
    let int_id = elaborated.globals["Int"];
    let int_ty = Term::const_(int_id, vec![]);
    let proof_ty = Term::Eq(Box::new(int_ty.clone()), Box::new(int(9)), Box::new(int(9)));
    let hole = declare_postulate(
        &mut elaborated.env,
        "opaque_let_proof".into(),
        vec![],
        proof_ty.clone(),
    )
    .expect("typed opaque proof admits");
    let body = Term::Let {
        ty: Box::new(proof_ty),
        val: Box::new(Term::const_(hole, vec![])),
        body: Box::new(int(9)),
    };
    ken_kernel::check(&elaborated.env, &Context::new(), &body, &int_ty)
        .expect("kernel checks proof-bound Let");
    let let_id = declare_def(&mut elaborated.env, vec![], int_ty.clone(), body.clone())
        .expect("checked Let definition admits");
    let package = package(
        &elaborated.env,
        &[(int_id, "Int"), (hole, "hole"), (let_id, "proof_let")],
        &[let_id],
    );
    let target = sym("proof_let");
    let plan = &package.artifact.semantic.omega_erasure_plans[&target];
    assert_eq!(
        plan.erased_binders,
        BTreeSet::from([0]),
        "kernel-Ω-classified Let binder is erased"
    );
    assert_eq!(plan.erased_subterms, BTreeSet::from([5]));
    let lowered = native_body(&package, &target, &[target.clone()]);
    assert!(
        !matches!(lowered, RuntimeExpr::Let { .. }),
        "erased Let binder leaves no runtime binding"
    );
    assert_eq!(
        observed(&lowered),
        RuntimeObservation::Returned(RuntimeGroundValue::Int(9.into()))
    );
    let mut store = EvalStore::new();
    assert!(matches!(
        eval_checked(&body, &int_ty, &elaborated.env, &mut store)
            .expect("interpreter evaluates the checked Let"),
        EvalVal::Int(9)
    ));
}

#[test]
fn omega_let_erasure_preserves_outer_runtime_variable() {
    // Promise class: durable invariant (42 §3.2, 46 §4, 47 §1).
    // MEASURED: within λ x:Int, erasing the proof-bound Let removes its
    // binder while the body's outer x remains a live runtime variable.
    // CLAIMED: de Bruijn remapping skips the erased binder, not x.
    // THE GAP: this covers one outer binder; nested outer contexts and
    // other runtime-expression constructors need their own controls.
    let mut elaborated = ElabEnv::new().expect("prelude admits");
    let int_id = elaborated.globals["Int"];
    let int_ty = Term::const_(int_id, vec![]);
    let proof_ty = Term::Eq(
        Box::new(int_ty.clone()),
        Box::new(Term::var(0)),
        Box::new(Term::var(0)),
    );
    let proof_let = Term::Let {
        ty: Box::new(proof_ty),
        val: Box::new(Term::Refl(Box::new(Term::var(0)))),
        body: Box::new(Term::var(1)),
    };
    let function_ty = Term::pi(int_ty.clone(), int_ty.clone());
    let function = Term::lam(int_ty.clone(), proof_let);
    ken_kernel::check(&elaborated.env, &Context::new(), &function, &function_ty)
        .expect("kernel checks proof-bound Let under an outer runtime binder");
    let function_id = declare_def(&mut elaborated.env, vec![], function_ty, function)
        .expect("checked function admits");
    let call = Term::app(Term::const_(function_id, vec![]), int(7));
    let target_id = declare_def(&mut elaborated.env, vec![], int_ty.clone(), call.clone())
        .expect("checked call admits");
    let package = package(
        &elaborated.env,
        &[
            (int_id, "Int"),
            (function_id, "proof_let_function"),
            (target_id, "proof_let_call"),
        ],
        &[function_id, target_id],
    );
    let function_symbol = sym("proof_let_function");
    let plan = &package.artifact.semantic.omega_erasure_plans[&function_symbol];
    assert_eq!(
        plan.erased_binders,
        BTreeSet::from([2]),
        "only the proof Let binder is erased, not the outer λ"
    );
    let target = sym("proof_let_call");
    let lowered = native_body(&package, &target, &[target.clone(), function_symbol]);
    assert_eq!(
        observed(&lowered),
        RuntimeObservation::Returned(RuntimeGroundValue::Int(7.into())),
        "outer x survives native Let-binder remapping"
    );
    let mut store = EvalStore::new();
    assert!(matches!(
        eval_checked(&call, &int_ty, &elaborated.env, &mut store)
            .expect("interpreter evaluates the checked open-context Let"),
        EvalVal::Int(7)
    ));
}

#[test]
fn relevant_sigma_pair_keeps_both_components() {
    // Promise class: durable invariant (42 §3.2, 47 §1).
    // MEASURED: Σ Int Int remains a pair on both runtime paths; a proof-first
    // Σ with computational codomain remains a pair in the plan and interpreter.
    // CLAIMED: only the codomain decides Σ collapse, not the first domain.
    // THE GAP: native has no representation for an erased first field in a
    // retained pair; its fail-closed boundary is pinned below.
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

    // The first domain is itself Ω-classified, but the codomain remains Int.
    // Changing Σ-collapse classification to the first domain would collapse
    // this pair even though its second component is computational.
    let proof_ty = Term::Eq(
        Box::new(Term::const_(int_id, vec![])),
        Box::new(int(7)),
        Box::new(int(7)),
    );
    let hole = declare_postulate(
        &mut elaborated.env,
        "relevant_first_proof".into(),
        vec![],
        proof_ty.clone(),
    )
    .expect("open proof admits");
    let proof_sigma = Term::sigma(proof_ty, Term::const_(int_id, vec![]));
    let proof_pair = Term::pair(Term::const_(hole, vec![]), int(8));
    ken_kernel::check(&elaborated.env, &Context::new(), &proof_pair, &proof_sigma)
        .expect("kernel checks proof-first relevant pair");
    let proof_pair_id = declare_def(
        &mut elaborated.env,
        vec![],
        proof_sigma.clone(),
        proof_pair.clone(),
    )
    .expect("proof-first pair admits");
    let proof_package = crate::package(
        &elaborated.env,
        &[
            (int_id, "Int"),
            (hole, "proof"),
            (proof_pair_id, "proof_pair"),
        ],
        &[proof_pair_id],
    );
    let target = sym("proof_pair");
    let lowered = erase_checked_core_package_for_target(&proof_package, [&target]);
    let plan = &proof_package.artifact.semantic.omega_erasure_plans[&target];
    assert!(
        plan.collapsed_sigmas.is_empty(),
        "a computational codomain retains its Σ pair"
    );
    assert_eq!(plan.erased_subterms, BTreeSet::from([1]));
    let mut store = EvalStore::new();
    let result = eval_checked(&proof_pair, &proof_sigma, &elaborated.env, &mut store)
        .expect("interpreter retains proof-first relevant pair");
    assert!(
        matches!(&result, EvalVal::Pair { fst, snd, .. }
        if matches!(&**fst, EvalVal::Neutral)
            && matches!(&**snd, EvalVal::Int(8))),
        "observed: {result:?}"
    );
    // MEASURED: native lowering refuses a relevant Σ whose first field is
    // an erased Ω subterm at the exact erased-field lane.
    // CLAIMED: fail-closed, never a collapsed or partial native value.
    // THE GAP: a retained Σ slot has no native erased-field representation;
    // successor LANG-NATIVE-SIGMA-ERASED-FIELD owns that upgrade.
    assert!(matches!(&lowered,
        Err(ErasureError::ExpressionLowering { symbol, lane, .. })
            if symbol == &target && *lane == "erased_omega_subterm_reached_computation"));
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

#[test]
fn interpreter_projection_paths_use_erased_transparent_bodies() {
    // Promise class: durable invariant (42 §3.2, 46 §4).
    // MEASURED: a nested checked subset carrier survives both a transparent
    // accessor through an identity call and a direct projection of a global.
    // CLAIMED: projection fast paths inspect the same cached erased bodies as
    // ordinary Const unfolding. THE GAP: the caller and the callee must both
    // use the same checked environment; the two arms are pinned separately.
    let mut elaborated = ElabEnv::new().expect("prelude admits");
    let int_ty = Term::const_(elaborated.globals["Int"], vec![]);
    let proof_ty = Term::Eq(Box::new(int_ty.clone()), Box::new(int(7)), Box::new(int(7)));
    let hole = declare_postulate(&mut elaborated.env, "nested_proof".into(), vec![], proof_ty)
        .expect("open proof is checked");
    let subset = Term::sigma(
        int_ty.clone(),
        Term::Eq(
            Box::new(int_ty.clone()),
            Box::new(Term::var(0)),
            Box::new(Term::var(0)),
        ),
    );
    let source_ty = Term::sigma(int_ty.clone(), subset);
    let source = declare_def(
        &mut elaborated.env,
        vec![],
        source_ty.clone(),
        Term::pair(int(1), Term::pair(int(7), Term::const_(hole, vec![]))),
    )
    .expect("nested subset source admits");
    let identity = declare_def(
        &mut elaborated.env,
        vec![],
        Term::pi(source_ty.clone(), source_ty.clone()),
        Term::lam(source_ty.clone(), Term::var(0)),
    )
    .expect("checked wrapper admits");
    let accessor = declare_def(
        &mut elaborated.env,
        vec![],
        Term::pi(source_ty.clone(), int_ty.clone()),
        Term::lam(source_ty, Term::proj1(Term::proj2(Term::var(0)))),
    )
    .expect("nested carrier accessor admits");
    let source_ref = Term::const_(source, vec![]);
    let wrapped = Term::app(
        Term::const_(accessor, vec![]),
        Term::app(Term::const_(identity, vec![]), source_ref.clone()),
    );
    let directly_projected = Term::proj1(Term::proj2(source_ref));
    let mut store = EvalStore::new();
    assert!(matches!(
        eval_checked(&wrapped, &int_ty, &elaborated.env, &mut store)
            .expect("checked accessor through wrapper"),
        EvalVal::Int(7)
    ));
    assert!(matches!(
        eval_checked(&directly_projected, &int_ty, &elaborated.env, &mut store)
            .expect("checked projection of global"),
        EvalVal::Int(7)
    ));
}
