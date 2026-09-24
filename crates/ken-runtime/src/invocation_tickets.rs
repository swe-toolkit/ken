//! Activation-owned, fixed-backing authority for a selected one-use call.
//! A ticket is data, not a callable: the consuming gate validates it before
//! any caller may read its borrowed operands or invoke the selected body.

use crate::boundary_resource_profile::InvocationCallLimitsV3;
use ken_host::{CapacityExhaustedV1, CapacityResourceV1, CapacityScopeV1};

/// An exact runtime target, rather than the source text that named it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SelectedCallTargetV1 {
    pub body: u64,
    pub callee: u64,
}

/// Only `InvocationTicketIssuerV1::issue` creates a live ticket. Copies must
/// consult the owner; carrying this value alone grants no calling authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SelectedCallTicketV1 {
    epoch: u64,
    generation: u64,
    slot: u64,
    target: SelectedCallTargetV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectedCallIntegrityFaultV1 {
    WrongActivation,
    InvalidSlot,
    StaleGeneration,
    Spent,
    WrongTarget,
}

#[derive(Clone, Copy, Debug)]
struct LiveSlot {
    generation: u64,
    target: SelectedCallTargetV1,
    live: bool,
}

/// Moving the activation does not move this box's storage. The generation
/// counter is monotonic and never replenished by consuming a slot; only a live
/// slot is replenished, after its one authorized call.
pub struct InvocationTicketIssuerV1 {
    epoch: u64,
    generation: u64,
    generation_limit: u64,
    slots: Box<[LiveSlot]>,
}

impl InvocationTicketIssuerV1 {
    pub fn reserve(epoch: u64, limits: InvocationCallLimitsV3) -> Result<Self, CapacityExhaustedV1> {
        // A Vec with a fixed final length is boxed before the services pointer
        // is published. No issuance path may grow it after publication.
        let mut slots = Vec::new();
        slots.try_reserve_exact(limits.live_pending_slots).map_err(|_| CapacityExhaustedV1 {
            scope: CapacityScopeV1::Invocation,
            resource: CapacityResourceV1::LivePendingSlots,
            limit: slots.capacity() as u128,
            requested: limits.live_pending_slots as u128,
        })?;
        slots.resize(limits.live_pending_slots, LiveSlot {
            generation: 0,
            target: SelectedCallTargetV1 { body: 0, callee: 0 },
            live: false,
        });
        Ok(Self { epoch, generation: 0, generation_limit: limits.event_generations,
            slots: slots.into_boxed_slice() })
    }

    pub fn issue(&mut self, target: SelectedCallTargetV1) -> Result<SelectedCallTicketV1, CapacityExhaustedV1> {
        let slot_limit = self.slots.len() as u128;
        let (slot, entry) = self.slots.iter_mut().enumerate().find(|(_, entry)| !entry.live)
            .ok_or(CapacityExhaustedV1 {
                scope: CapacityScopeV1::Invocation,
                resource: CapacityResourceV1::LivePendingSlots,
                limit: slot_limit,
                requested: slot_limit + 1,
            })?;
        let requested = self.generation as u128 + 1;
        if requested > self.generation_limit as u128 || requested > u64::MAX as u128 {
            return Err(CapacityExhaustedV1 {
                scope: CapacityScopeV1::Invocation,
                resource: CapacityResourceV1::EventGenerations,
                limit: self.generation_limit as u128,
                requested,
            });
        }
        self.generation = requested as u64;
        entry.generation = self.generation;
        entry.target = target;
        entry.live = true;
        Ok(SelectedCallTicketV1 {
            epoch: self.epoch, generation: self.generation, slot: slot as u64, target,
        })
    }

    /// Must precede operand loads and the call. On any integrity fault no
    /// slot changes state; successful consumption alone marks it spent.
    pub fn consume(
        &mut self,
        ticket: SelectedCallTicketV1,
        expected: SelectedCallTargetV1,
    ) -> Result<(), SelectedCallIntegrityFaultV1> {
        use SelectedCallIntegrityFaultV1 as Fault;
        if ticket.epoch != self.epoch { return Err(Fault::WrongActivation); }
        let index = usize::try_from(ticket.slot).map_err(|_| Fault::InvalidSlot)?;
        let entry = self.slots.get_mut(index).ok_or(Fault::InvalidSlot)?;
        if ticket.generation != entry.generation { return Err(Fault::StaleGeneration); }
        if !entry.live { return Err(Fault::Spent); }
        if ticket.target != expected || entry.target != expected { return Err(Fault::WrongTarget); }
        entry.live = false;
        Ok(())
    }

    pub(crate) fn bind_epoch_before_publication(&mut self, epoch: u64) {
        assert_eq!(self.epoch, 0, "issuer epoch binds once before any ticket can exist");
        assert_ne!(epoch, 0, "process epoch zero is never minted");
        self.epoch = epoch;
    }

    pub fn backing_address(&self) -> usize { self.slots.as_ptr() as usize }
    pub fn epoch(&self) -> u64 { self.epoch }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn target(body: u64) -> SelectedCallTargetV1 { SelectedCallTargetV1 { body, callee: body + 11 } }
    fn issuer(epoch: u64, generations: u64, slots: usize) -> InvocationTicketIssuerV1 {
        InvocationTicketIssuerV1::reserve(epoch, InvocationCallLimitsV3 {
            event_generations: generations, live_pending_slots: slots,
        }).expect("small explicit profile")
    }

    /// Durable one-event property. The positive callback runs only after the
    /// exact slot and selected target are consumed, never before issuance.
    #[test]
    fn selected_call_is_one_use_and_wrong_target_cannot_invoke() {
        let mut owner = issuer(42, 3, 1);
        let address = owner.backing_address();
        let ticket = owner.issue(target(7)).expect("first selected issue");
        let wrong = owner.consume(ticket, target(8));
        assert_eq!(wrong, Err(SelectedCallIntegrityFaultV1::WrongTarget));
        let mut effects = 0;
        owner.consume(ticket, target(7)).expect("exact selected gate");
        effects += 1;
        assert_eq!(effects, 1);
        assert_eq!(owner.consume(ticket, target(7)), Err(SelectedCallIntegrityFaultV1::Spent));
        assert_eq!(owner.backing_address(), address);
    }

    #[test]
    fn slot_reuse_generation_and_cross_activation_epoch_prevent_aba() {
        let mut old = issuer(17, 2, 1);
        let first = old.issue(target(1)).unwrap();
        old.consume(first, target(1)).unwrap();
        let second = old.issue(target(1)).unwrap();
        assert_eq!(first.slot, second.slot);
        assert_ne!(first.generation, second.generation);
        assert_eq!(old.consume(first, target(1)), Err(SelectedCallIntegrityFaultV1::StaleGeneration));
        let mut next_activation = issuer(18, 2, 1);
        let next = next_activation.issue(target(1)).unwrap();
        assert_eq!(next.slot, first.slot);
        assert_eq!(next.generation, first.generation);
        assert_eq!(next_activation.consume(first, target(1)), Err(SelectedCallIntegrityFaultV1::WrongActivation));
        next_activation.consume(next, target(1)).unwrap();
    }

    #[test]
    fn exact_slot_and_generation_limits_refuse_before_half_issuance() {
        let mut zero = issuer(1, 0, 1);
        let generation = zero.issue(target(1)).unwrap_err();
        assert_eq!((generation.scope, generation.resource, generation.limit, generation.requested),
            (CapacityScopeV1::Invocation, CapacityResourceV1::EventGenerations, 0, 1));
        let mut zero_slots = issuer(2, 2, 0);
        let slots = zero_slots.issue(target(1)).unwrap_err();
        assert_eq!((slots.scope, slots.resource, slots.limit, slots.requested),
            (CapacityScopeV1::Invocation, CapacityResourceV1::LivePendingSlots, 0, 1));
        let mut one = issuer(3, 1, 1);
        let ticket = one.issue(target(1)).expect("at-limit generation");
        let slots = one.issue(target(1)).unwrap_err();
        assert_eq!((slots.resource, slots.limit, slots.requested), (CapacityResourceV1::LivePendingSlots, 1, 2));
        one.consume(ticket, target(1)).unwrap();
        let generation = one.issue(target(1)).unwrap_err();
        assert_eq!((generation.resource, generation.limit, generation.requested), (CapacityResourceV1::EventGenerations, 1, 2));
    }
}
