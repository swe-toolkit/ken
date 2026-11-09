use super::*;

// Synthetic lowered bodies, not checked-source witnesses. The production queue
// supplies `performing`; a missed performed match must now refuse instead of
// patching a match whose native plans were already collected.
const RET: &str = "ctor:fixture::ITree::Ret";
const VIS: &str = "ctor:fixture::ITree::Vis";
const PERFORMED: &str = "decl:fixture::drain";
const TREE_STYLE: &str = "decl:fixture::helper";

// Promise class: durable invariant. A smoke checked-source preparation must
// lower each performing push in direct style before beginning the tree match.
// MEASURED: main contains Let(Call DeclarationRef) for both push members and
// neither remains the scrutinee of a computational match.
// CLAIMED: the checked smoke runs the single performing emission and binds its
// payload instead of planning its remaining operations as a returned Vis chain.
// THE GAP: this fixture witnesses these two members, not every checked match.
#[test]
fn smoke_performed_push_binds_lower_in_direct_style() {
    const SMOKE: &str = include_str!("../../../ken-cli/tests/fixtures/px10_child_wait_smoke.ken");
    // The same checked smoke preparation measured 3292 KiB resident stack in
    // the native-build census. A 16 MiB worker leaves 13092 KiB of headroom;
    // this provisions its baseline, not a regression in an existing test.
    std::thread::Builder::new()
        .name("px10-b1a-direct-bind".into())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            let source = SMOKE.replacen(
                "program capabilities FS AFull, Process\n",
                "program capabilities FS AFull, Process, RootExecution Allow\n",
                1,
            );
            assert_ne!(source, SMOKE, "the smoke fixture header moved");
            let preparation = crate::compiler_driver::prepare_native_program_sources(
                "px10_f1_scrutinee",
                vec![crate::compiler_driver::CompilerSource::new("src/main.ken", source)],
            )
            .expect("the checked smoke prepares under its host root");
            let ret = preparation.bracket_host_spine_for_test().ret.to_string();
            let main = preparation.runtime_program().declarations.iter().find(|declaration| {
                declaration.symbol.ends_with("::main")
            }).expect("the smoke has an emitted main");
            let RuntimeDeclarationKind::Transparent { body } = &main.kind else {
                panic!("smoke main must have an executable body");
            };
            let mut work = vec![body];
            let members = ["private_process_push_arguments#0", "private_process_push_environment#0"];
            let mut calls = std::collections::BTreeMap::<&str, usize>::new();
            while let Some(expr) = work.pop() {
                match expr {
                    RuntimeExpr::ComputationalMatch { scrutinee, cases, .. } => {
                        let direct = if let RuntimeExpr::Construct { constructor, args } = scrutinee.as_ref() {
                            if constructor == &ret { args.first() } else { None }
                        } else { Some(scrutinee.as_ref()) };
                        if let Some(RuntimeExpr::Call { callee, .. }) = direct {
                            if let RuntimeExpr::DeclarationRef { symbol } = callee.as_ref() {
                                for member in members {
                                    assert!(!symbol.ends_with(&format!("::{member}")),
                                        "performed {member} call must bind directly, not be a computational-match scrutinee");
                                }
                            }
                        }
                        work.push(scrutinee);
                        work.extend(cases.iter().map(|case| &case.body));
                    }
                    RuntimeExpr::Match { scrutinee, cases, .. } => {
                        work.push(scrutinee);
                        work.extend(cases.iter().map(|case| &case.body));
                    }
                    RuntimeExpr::CheckedBracketFinalizer { body, .. }
                    | RuntimeExpr::CheckedJoinSite { body, .. }
                    | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
                    | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
                    | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
                    | RuntimeExpr::CheckedComputationalIHInvocation { body, .. }
                    | RuntimeExpr::Project { record: body, .. }
                    | RuntimeExpr::Closure { body, .. } => work.push(body),
                    RuntimeExpr::LexicalClosure { captures, body, .. } => {
                        work.push(body);
                        work.extend(captures);
                    }
                    RuntimeExpr::Let { value, body } => {
                        if let RuntimeExpr::Call { callee, args } = value.as_ref() {
                            if let RuntimeExpr::DeclarationRef { symbol } = callee.as_ref() {
                                for member in members {
                                    if symbol.ends_with(&format!("::{member}")) {
                                        assert_eq!(args.len(), 3, "{symbol}: one full application");
                                        *calls.entry(member).or_default() += 1;
                                    }
                                }
                            }
                        }
                        work.extend([value.as_ref(), body.as_ref()]);
                    }
                    RuntimeExpr::If { scrutinee, then_expr, else_expr } => {
                        work.extend([scrutinee.as_ref(), then_expr.as_ref(), else_expr.as_ref()]);
                    }
                    RuntimeExpr::Construct { args, .. } | RuntimeExpr::PrimitiveCall { args, .. } => work.extend(args),
                    RuntimeExpr::Call { callee, args } => {
                        work.push(callee);
                        work.extend(args);
                    }
                    RuntimeExpr::Effect { capability, args, .. } => {
                        if let Some(capability) = capability { work.push(&capability.value); }
                        work.extend(args);
                    }
                    RuntimeExpr::Record { fields } => work.extend(fields.iter().map(|(_, field)| field)),
                    RuntimeExpr::Value(_) | RuntimeExpr::Var(_) | RuntimeExpr::DeclarationRef { .. }
                    | RuntimeExpr::ImportedDeclarationRef { .. } | RuntimeExpr::Trap(_) => {}
                }
            }
            for member in members {
                assert!(calls.get(member).is_some_and(|count| *count > 0),
                    "main must bind the performed {member} call directly; found {calls:?}");
            }
        })
        .expect("spawn smoke preparation worker")
        .join()
        .expect("smoke preparation worker");
}

fn owner() -> StableSymbol {
    StableSymbol::declaration("fixture", &[], "main")
}

fn performing() -> BTreeSet<String> {
    BTreeSet::from([PERFORMED.to_string()])
}

fn call(symbol: &str) -> RuntimeExpr {
    RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: symbol.to_string(),
        }),
        args: vec![RuntimeExpr::Var(0)],
    }
}

fn tree_match(scrutinee: RuntimeExpr, vis_body: RuntimeExpr, ret_case: bool) -> RuntimeExpr {
    let mut cases = Vec::new();
    if ret_case {
        cases.push(RuntimeComputationalMatchCase {
            constructor: RET.to_string(),
            argument_binders: 1,
            recursive_positions: Vec::new(),
            body: RuntimeExpr::Var(0),
        });
    }
    cases.push(RuntimeComputationalMatchCase {
        constructor: VIS.to_string(),
        argument_binders: 2,
        recursive_positions: vec![1],
        body: vis_body,
    });
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(scrutinee),
        cases,
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "performed-call fixture default".to_string(),
        },
    }
}

fn scrutinee_of(expression: &RuntimeExpr) -> &RuntimeExpr {
    let RuntimeExpr::ComputationalMatch { scrutinee, .. } = expression else {
        panic!("the fixture is a computational match: {expression:?}");
    };
    scrutinee
}

fn is_call_to(expression: &RuntimeExpr, symbol: &str) -> bool {
    matches!(
        expression,
        RuntimeExpr::Call { callee, .. }
            if matches!(callee.as_ref(), RuntimeExpr::DeclarationRef { symbol: called } if called == symbol)
    )
}

#[test]
fn a_performed_call_scrutinee_is_refused_at_any_depth() {
    let mut expression = RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::Value(RuntimeValue::Int(1.into()))),
        body: Box::new(tree_match(call(PERFORMED), RuntimeExpr::Var(0), true)),
    };
    let error = refuse_performed_call_scrutinees(&owner(), &mut expression, &performing(), RET)
        .expect_err("a performed call must have been consumed before match planning");
    assert!(matches!(&error, ErasureError::ExpressionLowering { lane, .. }
        if *lane == "performed_call_scrutinee_not_direct"), "{error:?}");
}

#[test]
fn only_a_performed_call_in_scrutinee_position_is_refused() {
    // A tree-style member already returns a tree, and a performed call that is
    // a case body is a continuation's result, which is not a scrutinee.
    let mut expression = tree_match(call(TREE_STYLE), call(PERFORMED), true);
    refuse_performed_call_scrutinees(&owner(), &mut expression, &performing(), RET)
        .expect("a pure scrutinee and a performed case body are permitted");
    assert!(
        is_call_to(scrutinee_of(&expression), TREE_STYLE),
        "{expression:?}"
    );
    let RuntimeExpr::ComputationalMatch { cases, .. } = &expression else {
        unreachable!()
    };
    assert!(is_call_to(&cases[1].body, PERFORMED), "{expression:?}");
}

#[test]
fn a_performed_call_scrutinee_without_a_checked_ret_case_refuses() {
    let mut expression = tree_match(call(PERFORMED), RuntimeExpr::Var(0), false);
    let error = refuse_performed_call_scrutinees(&owner(), &mut expression, &performing(), RET)
        .expect_err("a performed call in a match scrutinee is refused");
    assert!(
        matches!(
            &error,
            ErasureError::ExpressionLowering { lane, .. }
                if *lane == "performed_call_scrutinee_not_direct"
        ),
        "{error:?}"
    );
}
