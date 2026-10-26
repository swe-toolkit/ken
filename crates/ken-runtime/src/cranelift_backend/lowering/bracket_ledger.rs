//! Per-lowering compile-time claims for checked bracket settlement edges.
//!
//! Closure of this ledger establishes representation closure, not semantic
//! equivalence. Interpreter/native D0 observes the latter independently.

use std::collections::BTreeMap;

use super::super::planning::{
    BracketSettlementControlView, BracketSettlementId, BracketSettlementMode, StaticOriginId,
    StaticResponseContinuationId,
};
use super::{backend_module, CraneliftBackendError, LoweringCause};

#[derive(Clone, Copy, Debug)]
struct Claim {
    mode: BracketSettlementMode,
    body_exit_origin: StaticOriginId,
    settlement_origin: StaticOriginId,
    outcome_merge_origin: StaticOriginId,
    resume_origin: StaticOriginId,
    response_owner: Option<StaticResponseContinuationId>,
    settlement: u32,
    outcome_merge: u32,
    resume: u32,
}

#[derive(Clone, Debug, Default)]
pub(super) struct BracketSettlementLedger {
    claims: BTreeMap<BracketSettlementId, Claim>,
}

impl BracketSettlementLedger {
    pub(super) fn open(view: &BracketSettlementControlView) -> Self {
        Self {
            claims: view
                .regions()
                .map(|region| {
                    (
                        region.id(),
                        Claim {
                            mode: region.mode(),
                            response_owner: region.response_owner(),
                            body_exit_origin: region.body_exit(),
                            settlement_origin: region.settlement(),
                            outcome_merge_origin: region.outcome_merge(),
                            resume_origin: region.resume(),
                            settlement: 0,
                            outcome_merge: 0,
                            resume: 0,
                        },
                    )
                })
                .collect(),
        }
    }

    pub(super) fn consume_settlement(
        &mut self,
        id: BracketSettlementId,
        mode: BracketSettlementMode,
        body_exit_origin: StaticOriginId,
        settlement_origin: StaticOriginId,
    ) -> Result<(), CraneliftBackendError> {
        let claim = self
            .claims
            .get_mut(&id)
            .ok_or_else(|| Self::error("an unplanned bracket settlement edge was consumed"))?;
        if claim.mode != mode
            || claim.body_exit_origin != body_exit_origin
            || claim.settlement_origin != settlement_origin
        {
            return Err(Self::error(
                "a bracket settlement edge was consumed by the wrong route",
            ));
        }
        claim.settlement = claim
            .settlement
            .checked_add(1)
            .ok_or_else(|| Self::error("a bracket settlement lowering count exhausted"))?;
        Ok(())
    }

    pub(super) fn consume_outcome_merge(
        &mut self,
        id: BracketSettlementId,
        origin: StaticOriginId,
    ) -> Result<(), CraneliftBackendError> {
        let claim = self
            .claims
            .get_mut(&id)
            .ok_or_else(|| Self::error("an unplanned bracket outcome merge was consumed"))?;
        if claim.outcome_merge_origin != origin || claim.outcome_merge >= claim.settlement {
            return Err(Self::error(
                "a bracket outcome merge was consumed out of order",
            ));
        }
        claim.outcome_merge = claim
            .outcome_merge
            .checked_add(1)
            .ok_or_else(|| Self::error("a bracket outcome merge count exhausted"))?;
        Ok(())
    }

    pub(super) fn consume_resume(
        &mut self,
        id: BracketSettlementId,
        origin: StaticOriginId,
    ) -> Result<(), CraneliftBackendError> {
        let claim = self
            .claims
            .get_mut(&id)
            .ok_or_else(|| Self::error("an unplanned bracket Resume edge was consumed"))?;
        if claim.resume_origin != origin || claim.resume >= claim.outcome_merge {
            return Err(Self::error(
                "a bracket Resume edge was consumed out of order",
            ));
        }
        claim.resume = claim
            .resume
            .checked_add(1)
            .ok_or_else(|| Self::error("a bracket Resume lowering count exhausted"))?;
        Ok(())
    }

    pub(super) fn close_owner(
        &self,
        owner: StaticResponseContinuationId,
    ) -> Result<(), CraneliftBackendError> {
        if let Some((id, claim)) = self.claims.iter().find(|(_, claim)| {
            claim.response_owner == Some(owner)
                && claim.mode == BracketSettlementMode::InPlace
                && (claim.settlement == 0
                    || claim.settlement != claim.outcome_merge
                    || claim.outcome_merge != claim.resume)
        }) {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "owner {owner:?} returned bracket {id:?} S {:?} without matched InPlace lowerings \
                 (settlement={}, merge={}, resume={})",
                    claim.settlement_origin, claim.settlement, claim.outcome_merge, claim.resume,
                ),
            ));
        }
        Ok(())
    }

    pub(super) fn close(&self) -> Result<(), CraneliftBackendError> {
        if self.claims.values().any(|claim| {
            claim.settlement == 0
                || claim.settlement != claim.outcome_merge
                || claim.outcome_merge != claim.resume
        }) {
            return Err(Self::error(
                "a checked bracket region has an unconsumed settlement edge",
            ));
        }
        Ok(())
    }

    fn error(message: &str) -> CraneliftBackendError {
        backend_module(LoweringCause::PlannerInvariant, message.to_string())
    }
}
