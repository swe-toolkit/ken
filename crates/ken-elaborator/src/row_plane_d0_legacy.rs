// D0-only exact base 6e1eb2e7 row inference, deleted before candidate.
fn infer_expr_row_type_before_latent(
    expr: &RExpr,
    effect_rows: &CheckedEffectRows<'_>,
    projection_ctx: Option<&ProjectionPurityCtx<'_>>,
) -> crate::effects::RowType {
    match expr {
        RExpr::RCon(name, _) => effect_rows
            .names
            .get(name)
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RCheckedGlobal { id, .. } => effect_rows
            .ids
            .get(id)
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RVar(_, _, _)
        | RExpr::RPatternAlias(_, _, _)
        | RExpr::RCell(_, _, _)
        | RExpr::RRecursiveResult { .. }
        | RExpr::RUniv(_, _)
        | RExpr::RNumLit(_, _)
        | RExpr::RStr(_, _)
        | RExpr::RCharLit(_, _)
        | RExpr::RByteStr(_, _) => crate::effects::RowType::empty(),
        RExpr::RApp(f, a, _) => infer_expr_row_type_before_latent(f, effect_rows, projection_ctx)
            .join(infer_expr_row_type_before_latent(a, effect_rows, projection_ctx)),
        RExpr::RLam(_, _, _)
        | RExpr::RPi(_, _, _, _)
        | RExpr::RSigma(_, _, _, _)
        | RExpr::RArrow(_, _, _)
        | RExpr::RTrunc(_, _) => crate::effects::RowType::empty(),
        RExpr::RAttachedProofRef {
            subject,
            proof_name,
            ..
        } => effect_rows
            .names
            .get(&format!("{subject}::{proof_name}"))
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RLet(_, _, val, body, _) => infer_expr_row_type_before_latent(val, effect_rows, projection_ctx)
            .join(infer_expr_row_type_before_latent(body, effect_rows, projection_ctx)),
        RExpr::RAsc(e, _, _) | RExpr::ROld(e, _) | RExpr::RBecomes(_, _, e, _) => {
            infer_expr_row_type_before_latent(e, effect_rows, projection_ctx)
        }
        RExpr::RPair(components, _) => components
            .iter()
            .fold(crate::effects::RowType::empty(), |row, component| {
                row.join(infer_expr_row_type_before_latent(component, effect_rows, projection_ctx))
            }),
        RExpr::RRecord { base, fields, .. } => {
            let mut row = base
                .as_deref()
                .map(|base| infer_expr_row_type_before_latent(base, effect_rows, projection_ctx))
                .unwrap_or_else(crate::effects::RowType::empty);
            for (_, value, _) in fields {
                row = row.join(infer_expr_row_type_before_latent(value, effect_rows, projection_ctx));
            }
            row
        }
        RExpr::RPosProj(e, _, _) => infer_expr_row_type_before_latent(e, effect_rows, projection_ctx),
        RExpr::RProj(e, field, _) => infer_expr_row_type_before_latent(e, effect_rows, projection_ctx)
            .join(projected_field_row_type(e, field, projection_ctx)),
        // The operands' rows, joined -- identical to `RBinOp` below, because
        // the operator itself is a global binding and contributes no row of
        // its own at this stage.
        RExpr::RStandardOp { lhs, rhs, .. } => {
            let left = infer_expr_row_type_before_latent(lhs, effect_rows, projection_ctx);
            left.join(infer_expr_row_type_before_latent(rhs, effect_rows, projection_ctx))
        }
        RExpr::RBinOp(_, l, r, _) => infer_expr_row_type_before_latent(l, effect_rows, projection_ctx)
            .join(infer_expr_row_type_before_latent(r, effect_rows, projection_ctx)),
        RExpr::RInfixSpine {
            operands,
            operators,
            ..
        } => {
            let mut row = operands
                .iter()
                .fold(crate::effects::RowType::empty(), |row, operand| {
                    row.join(infer_expr_row_type_before_latent(operand, effect_rows, projection_ctx))
                });
            for operator in operators {
                let operator_row = match operator {
                    RInfixOperator::User(name, _) => effect_rows.names.get(name),
                    RInfixOperator::CheckedUser(_, id, _) => effect_rows.ids.get(id),
                    RInfixOperator::Builtin(_, _) => None,
                };
                if let Some(operator_row) = operator_row {
                    row = row.join(operator_row.clone());
                }
            }
            row
        }
        RExpr::RMatch { scrut, arms, .. } => {
            let mut row = infer_expr_row_type_before_latent(scrut, effect_rows, projection_ctx);
            for arm in arms {
                row = row.join(infer_expr_row_type_before_latent(&arm.body, effect_rows, projection_ctx));
            }
            row
        }
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            ..
        } => infer_expr_row_type_before_latent(condition, effect_rows, projection_ctx)
            .join(infer_expr_row_type_before_latent(
                then_branch,
                effect_rows,
                projection_ctx,
            ))
            .join(infer_expr_row_type_before_latent(
                else_branch,
                effect_rows,
                projection_ctx,
            )),
    }
}

