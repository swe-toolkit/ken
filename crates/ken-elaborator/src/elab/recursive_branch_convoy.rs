//! Demand-only recursive constructor convoys (22 §3; 34 §3.3).
//!
//! The ordinary SCT-admitted recursive global is not a method hypothesis.
//! A source selector or generated lift may consume a real method hypothesis;
//! those matches keep their original eliminator and never acquire an equation.

use super::*;

pub(super) fn match_consumes_method_ih(
    cx: &ElabCtx<'_>,
    arms: &[RMatchArm],
    scrutinee_position: Option<usize>,
) -> bool {
    scrutinee_position.is_some_and(|position| cx.lift_bindings.contains_key(&position))
        || arms.iter().any(|arm| {
            contains_recursive_result(&arm.body)
                || arm.guard.as_ref().is_some_and(contains_recursive_result)
        })
}

/// Only direct recursive fields use the exact-motive method builder. Pi,
/// Sigma, and Former fields use distinct IH builders, so a match containing
/// any of them must retain its base path even if the source never reads an IH.
pub(super) fn direct_recursive_fields_only(
    env: &GlobalEnv,
    ind: &InductiveDecl,
) -> Result<bool, ken_kernel::KernelError> {
    for ctor in &ind.constructors {
        for argument in recursive_shapes(env, ctor, ind.id, ind.params.len())? {
            if !matches!(
                argument.shape,
                ken_kernel::inductive::RecursiveShape::Direct { .. }
            ) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

fn type_contains_recursive_result(ty: &RType) -> bool {
    match ty {
        RType::RPi(_, domain, codomain, _)
        | RType::RSigma(_, domain, codomain, _)
        | RType::RArr(domain, codomain, _)
        | RType::REffectArr(domain, _, codomain, _)
        | RType::RApp(domain, codomain, _) => {
            type_contains_recursive_result(domain) || type_contains_recursive_result(codomain)
        }
        RType::RRefine(_, carrier, predicate, _) => {
            type_contains_recursive_result(carrier) || contains_recursive_result(predicate)
        }
        RType::RTrunc(inner, _) => type_contains_recursive_result(inner),
        RType::RProj(base, _, _) => contains_recursive_result(base),
        RType::RUniv(_, _)
        | RType::RCon(_, _)
        | RType::RCheckedGlobal { .. }
        | RType::RVarTy(_, _, _)
        | RType::RPatternAliasTy(_, _, _) => false,
    }
}

/// Include nested bodies and annotation-embedded expressions. A new resolved
/// expression constructor must be classified here, not silently skipped.
fn contains_recursive_result(expr: &RExpr) -> bool {
    match expr {
        RExpr::RRecursiveResult { .. } => true,
        RExpr::RApp(function, argument, _)
        | RExpr::RBinOp(_, function, argument, _)
        | RExpr::RArrow(function, argument, _) => {
            contains_recursive_result(function) || contains_recursive_result(argument)
        }
        RExpr::RLam(_, body, _)
        | RExpr::ROld(body, _)
        | RExpr::RBecomes(_, _, body, _)
        | RExpr::RProj(body, _, _)
        | RExpr::RPosProj(body, _, _)
        | RExpr::RTrunc(body, _) => contains_recursive_result(body),
        RExpr::RLet(_, annotation, value, body, _) => {
            annotation
                .as_ref()
                .is_some_and(type_contains_recursive_result)
                || contains_recursive_result(value)
                || contains_recursive_result(body)
        }
        RExpr::RAsc(body, ty, _) => {
            contains_recursive_result(body) || type_contains_recursive_result(ty)
        }
        RExpr::RStandardOp { lhs, rhs, .. } => {
            contains_recursive_result(lhs) || contains_recursive_result(rhs)
        }
        RExpr::RInfixSpine { operands, .. } | RExpr::RPair(operands, _) => {
            operands.iter().any(contains_recursive_result)
        }
        RExpr::RMatch { scrut, arms, .. } => {
            contains_recursive_result(scrut)
                || arms.iter().any(|arm| {
                    contains_recursive_result(&arm.body)
                        || arm.guard.as_ref().is_some_and(contains_recursive_result)
                })
        }
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            contains_recursive_result(condition)
                || contains_recursive_result(then_branch)
                || contains_recursive_result(else_branch)
        }
        RExpr::RRecord { base, fields, .. } => {
            base.as_deref().is_some_and(contains_recursive_result)
                || fields
                    .iter()
                    .any(|(_, value, _)| contains_recursive_result(value))
        }
        RExpr::RPi(_, domain, codomain, _) | RExpr::RSigma(_, domain, codomain, _) => {
            type_contains_recursive_result(domain) || contains_recursive_result(codomain)
        }
        RExpr::RVar(_, _, _)
        | RExpr::RPatternAlias(_, _, _)
        | RExpr::RCon(_, _)
        | RExpr::RCheckedGlobal { .. }
        | RExpr::RUniv(_, _)
        | RExpr::RCell(_, _, _)
        | RExpr::RNumLit(_, _)
        | RExpr::RStr(_, _)
        | RExpr::RCharLit(_, _)
        | RExpr::RByteStr(_, _)
        | RExpr::RAttachedProofRef { .. } => false,
    }
}

/// This is the core-plane backstop to the source classifier. The kernel's
/// method has `field_count` field lambdas, then `ih_count` IH lambdas. Removing
/// any IH binder from the method body must succeed, even below nested lambdas.
pub(super) fn check_case_only_method(
    method: &Term,
    field_count: usize,
    ih_count: usize,
) -> Result<(), ElabError> {
    let mut body = method;
    for _ in 0..field_count + ih_count {
        let Term::Lam(_, inner) = body else {
            return Err(ElabError::Internal(
                "case-only convoy lost its checked field/IH method telescope".into(),
            ));
        };
        body = inner;
    }
    for index in 0..ih_count {
        if lower_binders(body, 1, index).is_err() {
            return Err(ElabError::Internal(
                "case-only convoy consumed a method induction hypothesis".into(),
            ));
        }
    }
    Ok(())
}
