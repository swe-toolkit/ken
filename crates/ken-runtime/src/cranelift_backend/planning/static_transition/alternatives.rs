//! Closed, runtime-selected alternative transports for a continuation result.

use super::continuations::ContinuationCallIdentity;
use super::occurrences::{occurrence_subtree_contains, StaticOriginId};
use super::{
    planner_error, ContinuationEmissionOwner, CraneliftBackendError, StaticTransitionPlan,
};
use crate::RuntimeExpr;

/// The closed set of producer transports that can reach one result. Members
/// are sorted by target specialization and a member's label is its index. A
/// singleton has no set and keeps its existing unlabelled transport word.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ProducerAlternativeSet {
    pub(in crate::cranelift_backend) emission_owner: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend) result: StaticOriginId,
    pub(in crate::cranelift_backend) members: Vec<ContinuationCallIdentity>,
}

impl ProducerAlternativeSet {
    pub(in crate::cranelift_backend) fn label(
        &self,
        member: &ContinuationCallIdentity,
    ) -> Result<u32, CraneliftBackendError> {
        let index = self
            .members
            .iter()
            .position(|candidate| candidate == member)
            .ok_or_else(|| {
                planner_error("a producer is not a member of its result's alternative set")
            })?;
        u32::try_from(index)
            .map_err(|_| planner_error("an alternative label exceeds the carrier ABI"))
    }
}

impl StaticTransitionPlan<'_> {
    /// Issue a set only for distinct checked-IH sources whose paths from one
    /// result diverge at a source data Match in different arms. Two producers
    /// on the same path and duplicate edges are never alternatives.
    pub(in crate::cranelift_backend) fn producer_alternative_set(
        &self,
        members: &[&ContinuationCallIdentity],
    ) -> Result<Option<ProducerAlternativeSet>, CraneliftBackendError> {
        let [first, _, ..] = members else {
            return Ok(None);
        };
        let result = first.producer_result_origin();
        let emission_owner = first.emission_owner();
        if members.iter().any(|member| {
            member.producer_result_origin() != result
                || member.emission_owner() != emission_owner
                || self
                    .checked_ih_environment_transport_source(member)
                    .is_none()
        }) {
            return Ok(None);
        }
        let paths = members
            .iter()
            .map(|member| self.occurrence_child_path(result, member.producer_construct_origin()))
            .collect::<Result<Vec<_>, _>>()?;
        for (index, left) in paths.iter().enumerate() {
            for right in &paths[index + 1..] {
                let shared = left.iter().zip(right).take_while(|(a, b)| a == b).count();
                let (Some(&(split, left_arm)), Some(&(_, right_arm))) =
                    (left.get(shared), right.get(shared))
                else {
                    return Ok(None);
                };
                if left_arm == right_arm
                    || !matches!(self.source_occurrence(split)?, RuntimeExpr::Match { .. })
                {
                    return Ok(None);
                }
            }
        }
        let mut members = members
            .iter()
            .map(|member| (*member).clone())
            .collect::<Vec<_>>();
        members.sort_by_key(ContinuationCallIdentity::target);
        members.dedup();
        if members.len() != paths.len() {
            return Ok(None);
        }
        Ok(Some(ProducerAlternativeSet {
            emission_owner,
            result,
            members,
        }))
    }

    /// The occurrence/child-position steps from `root` to `needle`.
    fn occurrence_child_path(
        &self,
        root: StaticOriginId,
        needle: StaticOriginId,
    ) -> Result<Vec<(StaticOriginId, usize)>, CraneliftBackendError> {
        let mut path = Vec::new();
        let mut node = root;
        while node != needle {
            let mut next = None;
            for (position, child) in self
                .semantic
                .child_origins(node)?
                .iter()
                .copied()
                .enumerate()
            {
                if occurrence_subtree_contains(self, child, needle)? {
                    next = Some((position, child));
                    break;
                }
            }
            let (position, child) = next
                .ok_or_else(|| planner_error("an alternative producer is not inside its result"))?;
            path.push((node, position));
            node = child;
        }
        Ok(path)
    }
}
