//! The conservative, syntactic occurrence mask for checked data families.
//! Constructor result parameters merely forward the family's parameters;
//! only result indices can add an occurrence beyond its fields.

use crate::resolve::{RDecl, RDeclKind, RExpr, RType};

enum Site<'a> {
    Ty(&'a RType),
    Expr(&'a RExpr),
}

/// Count names, including shadowed names, at every type and embedded-expression
/// depth. This is deliberately not the resolver's free-name dependency walk:
/// an RVarTy/RVar is the occurrence that makes a parameter charged.
fn contains_parameter(ty: &RType, name: &str) -> bool {
    let mut pending = vec![Site::Ty(ty)];
    while let Some(site) = pending.pop() {
        match site {
            Site::Ty(ty) => match ty {
                RType::RVarTy(_, n, _) | RType::RPatternAliasTy(_, n, _) if n == name => {
                    return true;
                }
                RType::RPi(_, a, b, _)
                | RType::RSigma(_, a, b, _)
                | RType::RArr(a, b, _)
                | RType::REffectArr(a, _, b, _)
                | RType::RApp(a, b, _) => {
                    pending.push(Site::Ty(a));
                    pending.push(Site::Ty(b));
                }
                RType::RRefine(_, carrier, predicate, _) => {
                    pending.push(Site::Ty(carrier));
                    pending.push(Site::Expr(predicate));
                }
                RType::RTrunc(inner, _) => pending.push(Site::Ty(inner)),
                RType::RProj(base, _, _) => pending.push(Site::Expr(base)),
                RType::RUniv(..)
                | RType::RCon(..)
                | RType::RCheckedGlobal { .. }
                | RType::RVarTy(..)
                | RType::RPatternAliasTy(..) => {}
            },
            Site::Expr(expr) => match expr {
                RExpr::RVar(_, n, _) | RExpr::RPatternAlias(_, n, _) if n == name => {
                    return true;
                }
                RExpr::RApp(a, b, _)
                | RExpr::RArrow(a, b, _)
                | RExpr::RBinOp(_, a, b, _)
                | RExpr::RStandardOp { lhs: a, rhs: b, .. } => {
                    pending.push(Site::Expr(a));
                    pending.push(Site::Expr(b));
                }
                RExpr::RLam(_, body, _)
                | RExpr::ROld(body, _)
                | RExpr::RBecomes(_, _, body, _)
                | RExpr::RProj(body, _, _)
                | RExpr::RPosProj(body, _, _)
                | RExpr::RTrunc(body, _) => pending.push(Site::Expr(body)),
                RExpr::RLet(_, ty, rhs, body, _) => {
                    if let Some(ty) = ty {
                        pending.push(Site::Ty(ty));
                    }
                    pending.push(Site::Expr(rhs));
                    pending.push(Site::Expr(body));
                }
                RExpr::RAsc(body, ty, _) => {
                    pending.push(Site::Expr(body));
                    pending.push(Site::Ty(ty));
                }
                RExpr::RPi(_, domain, body, _) | RExpr::RSigma(_, domain, body, _) => {
                    pending.push(Site::Ty(domain));
                    pending.push(Site::Expr(body));
                }
                RExpr::RMatch { scrut, arms, .. } => {
                    pending.push(Site::Expr(scrut));
                    for arm in arms {
                        if let Some(guard) = &arm.guard {
                            pending.push(Site::Expr(guard));
                        }
                        pending.push(Site::Expr(&arm.body));
                    }
                }
                RExpr::RIf {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    pending.push(Site::Expr(condition));
                    pending.push(Site::Expr(then_branch));
                    pending.push(Site::Expr(else_branch));
                }
                RExpr::RPair(parts, _) => pending.extend(parts.iter().map(Site::Expr)),
                RExpr::RRecord { base, fields, .. } => {
                    if let Some(base) = base {
                        pending.push(Site::Expr(base));
                    }
                    pending.extend(fields.iter().map(|(_, value, _)| Site::Expr(value)));
                }
                RExpr::RInfixSpine { operands, .. } => {
                    pending.extend(operands.iter().map(Site::Expr));
                }
                RExpr::RVar(..)
                | RExpr::RPatternAlias(..)
                | RExpr::RCon(..)
                | RExpr::RCheckedGlobal { .. }
                | RExpr::RUniv(..)
                | RExpr::RCell(..)
                | RExpr::RRecursiveResult { .. }
                | RExpr::RNumLit(..)
                | RExpr::RStr(..)
                | RExpr::RCharLit(..)
                | RExpr::RByteStr(..)
                | RExpr::RAttachedProofRef { .. }
                | RExpr::RSlotLocal { .. } => {}
            },
        }
    }
    false
}

/// One checked-family mask. `None` denotes a non-family (charge everything).
/// The caller checks the mask's arity against the checked former's Pi telescope
/// before registering it under either identity or spelling.
pub(crate) fn family_charged_positions(rdecl: &RDecl) -> Option<Vec<bool>> {
    match &rdecl.kind {
        RDeclKind::DataDecl { type_params, ctors } => Some(
            type_params
                .iter()
                .map(|name| {
                    ctors
                        .iter()
                        .any(|ctor| ctor.args.iter().any(|ty| contains_parameter(ty, name)))
                })
                .collect(),
        ),
        RDeclKind::ExplicitDataDecl {
            params,
            indices,
            ctors,
            ..
        } => {
            let result_indices = |result: &RType| {
                let mut args = Vec::new();
                let mut head = result;
                while let RType::RApp(f, arg, _) = head {
                    args.push(arg.as_ref());
                    head = f;
                }
                if args.len() != params.len() + indices.len() {
                    // Unrecognized result shape: conservatively charge on the
                    // whole expression, never discard a possible occurrence.
                    return vec![result];
                }
                args.reverse();
                args.into_iter().skip(params.len()).collect::<Vec<_>>()
            };
            let mut charged = Vec::with_capacity(params.len() + indices.len());
            for (i, param) in params.iter().enumerate() {
                let Some(name) = &param.name else {
                    charged.push(true);
                    continue;
                };
                charged.push(
                    params
                        .iter()
                        .enumerate()
                        .any(|(j, entry)| j != i && contains_parameter(&entry.ty, name))
                        || indices
                            .iter()
                            .any(|entry| contains_parameter(&entry.ty, name))
                        || ctors.iter().any(|ctor| {
                            ctor.args
                                .iter()
                                .any(|entry| contains_parameter(&entry.ty, name))
                                || ctor.result.as_ref().is_some_and(|result| {
                                    result_indices(result)
                                        .into_iter()
                                        .any(|ty| contains_parameter(ty, name))
                                })
                        }),
                );
            }
            charged.extend(std::iter::repeat_n(true, indices.len()));
            Some(charged)
        }
        _ => None,
    }
}
