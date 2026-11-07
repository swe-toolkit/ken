//! One checked in-place settlement sequence shared by both Construct arms.
//! `perform_response_at_source` supplies the existing host effect; the ledger
//! binds it to the authenticated S and exit ports. The settlement returns the
//! lexical K for its caller to lower: the expression arm lowers it directly,
//! and the producer arm hands it to the producer trampoline, so K is never
//! lowered under the producer stack or nested inside the settlement's frames.

use cranelift_frontend::FunctionBuilder;

use super::super::planning::StaticOriginId;
use super::{
    BracketSettlement, BracketSettlementMode, Lowering, LoweringEnvironmentBinding, LoweringOperand,
};
use crate::cranelift_backend::surface::{backend_module, CraneliftBackendError, LoweringCause};
use crate::RuntimeExpr;

impl Lowering<'_> {
    pub(super) fn settle_bracket_at_source(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        region: &BracketSettlement,
        lowered_args: &[LoweringOperand],
    ) -> Result<(StaticOriginId, Vec<LoweringEnvironmentBinding>), CraneliftBackendError> {
        let row = region.row().cloned().ok_or_else(|| {
            Self::bracket_error("an InPlace bracket settlement has no validated response row")
        })?;
        if region.mode() != BracketSettlementMode::InPlace
            || row.vis_origin() != region.settlement()
            || lowered_args.len() != 2
        {
            return Err(Self::bracket_error(
                "a bracket settlement has no exact binary Construct and row",
            ));
        }
        if self
            .static_transition_plan
            .response_disposition_at_operation_root(region.settlement())
            .is_some()
        {
            return Err(Self::bracket_error(
                "a marked S acquired an ordinary operation-root disposition",
            ));
        }
        // The effect-only authority bind is proved for this checked shape,
        // not for a future effect that could lower an owner-sensitive input.
        let effect = self
            .retained_body_occurrence(row.effect_origin())?
            .expr
            .clone();
        let swept = matches!(&effect, RuntimeExpr::Effect { operation, capability: None, args, .. }
            if *operation == ken_host::HostOpV1::ResourceRelease
                && args.iter().all(|arg| matches!(arg, RuntimeExpr::Var(_))));
        if row.operation() != ken_host::HostOpV1::ResourceRelease || !swept {
            return Err(Self::bracket_error(
                "an InPlace S host effect is outside the R8-swept ResourceRelease shape",
            ));
        }
        self.bracket_ledger.consume_settlement(
            region.id(),
            region.mode(),
            region.body_exit(),
            region.settlement(),
        )?;
        if self
            .bracket_effect_claim
            .replace((region.id(), row.effect_origin()))
            .is_some()
        {
            return Err(Self::bracket_error(
                "an InPlace S effect claim is already open",
            ));
        }
        let settled = self.perform_response_at_source(
            builder,
            region.settlement(),
            lowered_args,
            row.operation_root_origin(),
            row.effect_origin(),
            row.operation(),
            Some((row.base_owner(), row.effect_source_owner())),
        );
        let unclaimed = self.bracket_effect_claim.take().is_some();
        let settled = settled?;
        if unclaimed {
            return Err(Self::bracket_error(
                "an InPlace S effect claim was not consumed by its effect",
            ));
        }
        self.bracket_ledger
            .consume_outcome_merge(region.id(), region.outcome_merge())?;
        self.bracket_ledger
            .consume_resume(region.id(), region.resume())?;
        Ok(settled)
    }

    fn bracket_error(message: &str) -> CraneliftBackendError {
        backend_module(LoweringCause::PlannerInvariant, message.to_string())
    }
}
