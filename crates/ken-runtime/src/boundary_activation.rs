//! **`RT-FNSPLIT-C3-ACTIVATION` `D3` — the Rust-owned activation and its
//! lifecycle.**
//!
//! ⭐ **The lifecycle verbs already existed; the OWNER did not.**
//! `reserve` / `bind_*` / `publish` / `seal_persistent` / `publish_persistent` /
//! `adopt` are landed definitions with **no production caller** — measured, and
//! it is why this node exists. ⇒ What this module supplies is the **caller and
//! its lifetime**, which is the part a definition cannot express: ordering,
//! ownership and teardown.
//!
//! ## The ruled order, across TWO scopes
//!
//! ⭐ The order spans two lifetimes, and splitting it that way is the thing this
//! module got wrong first — see [`BoundaryStoreBindingV1`].
//!
//! **Once per store** ([`BoundaryStoreBindingV1::open`]):
//!
//! 1. **reserve** the persistent image from the profile's persistent limits;
//! 2. **publish** the persistent image.
//!
//! **Once per activation** ([`BoundaryActivationV1::begin`]):
//!
//! 3. **bind** the published persistent base through [`ARENA_PERSISTENT`];
//! 4. **bind** this invocation's native-`Int` arena through
//!    [`ARENA_NATIVE_INT`] — ⭐ the *same* pointer the services record carries,
//!    so ordinary native lowering and boundary-`Int` decoding share one native
//!    authority while no code mistakes it for the boundary header;
//! 5. **reserve** invocation storage from the profile's invocation limits;
//! 6. **publish** the arena and put that base in the services record.
//!
//! ⛔ **No reservation or materialization after either published pointer
//! exists** — growth moves a region's tables under a pointer generated code
//! already holds. That is enforced structurally: each reservation happens inside
//! the one constructor for its scope, and neither type exposes a reserve
//! afterwards.
//!
//! ## ⛔ Why the owned objects are boxed
//!
//! ⚠ [`NativeIntArenaV1`] is a plain `#[repr(C)]` struct, so a pointer to it
//! points **into** the value. Handing that pointer out and then *moving* the
//! activation would leave generated code holding a dangling pointer, and nothing
//! would look wrong. ⭐ Boxing gives a heap address that survives every move of
//! the handle — which matters because the handle is a value until the C ABI
//! pins it. The same reasoning applies to the services record.
//!
//! ⚠ The boundary region's own published header is a `Vec<u64>`, so its base is
//! already heap-stable; ⛔ that is a property of that type, not a general one,
//! and is the reason the two cases are treated differently rather than
//! uniformly.
//!
//! ## ⛔ Not an artifact-static arena
//!
//! `§3b` closes that fork: a process-lifetime arena is shared by repeated,
//! concurrent and re-entrant activations, and its published table pointers and
//! counts are **mutable and invocation-specific**, so two executions would
//! **alias storage**. ⇒ The arena is invocation-owned, and
//! `two_activations_do_not_share_mutable_arena_state` is the control that would
//! have caught the alternative.

use std::ffi::c_void;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

// The deployment's first admitted epoch ceiling wins, including zero. This
// authority is distinct from the no-wrap counter and never resets with a store.
static INVOCATION_EPOCH_CEILING: OnceLock<u64> = OnceLock::new();
static LAST_INVOCATION_EPOCH: AtomicU64 = AtomicU64::new(0);

/// An attempted store would change the process-wide epoch budget. This is a
/// profile disagreement, not a request for an exhausted activation epoch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvocationEpochProfileMismatchV2 {
    pub authorized: u64,
    pub attempted: u64,
}

fn admit_epoch_ceiling(limit: u64) -> Result<(), InvocationEpochProfileMismatchV2> {
    let authorized = *INVOCATION_EPOCH_CEILING.get_or_init(|| limit);
    if authorized == limit {
        Ok(())
    } else {
        Err(InvocationEpochProfileMismatchV2 {
            authorized,
            attempted: limit,
        })
    }
}

fn requested_epoch_after(last: u64) -> u128 {
    u128::from(last) + 1
}

fn mint_invocation_epoch(limit: u64) -> Result<u64, ken_host::CapacityExhaustedV1> {
    loop {
        let last = LAST_INVOCATION_EPOCH.load(Ordering::Acquire);
        let requested = requested_epoch_after(last);
        if requested > u128::from(limit) || requested > u128::from(u64::MAX) {
            return Err(ken_host::CapacityExhaustedV1 {
                scope: ken_host::CapacityScopeV1::Runtime,
                resource: ken_host::CapacityResourceV1::InvocationEpochs,
                limit: u128::from(limit),
                requested,
            });
        }
        let next = requested as u64;
        if LAST_INVOCATION_EPOCH
            .compare_exchange(last, next, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Ok(next);
        }
    }
}

use crate::activation_services::GeneratedActivationServicesV1;
use crate::boundary_resource_profile::{
    BoundaryCapacityExhaustedV1, BoundaryResource, BoundaryResourceProfileV3, BoundaryResourceScope,
};
use crate::boundary_value::{
    ARENA_DATA_CAPACITY, ARENA_LIMB_CAPACITY, ARENA_NATIVE_INT, ARENA_NODE_CAPACITY,
    ARENA_PERSISTENT, ARENA_WORD_CAPACITY, BoundaryArenaBuilder, BoundaryArenaV1,
    BoundaryReservationFailureV1, BoundaryValueStore, BoundaryWord,
};

/// A store either rejects the first process epoch authority or refuses to
/// publish its persistent backing. Both are pre-publication failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryStoreOpenErrorV2 {
    ProfileMismatch(InvocationEpochProfileMismatchV2),
    CapacityExhausted(ken_host::CapacityExhaustedV1),
}

fn region_reservation_fault(
    scope: ken_host::CapacityScopeV1,
    failure: BoundaryReservationFailureV1,
) -> ken_host::CapacityExhaustedV1 {
    let resource = match failure.resource {
        BoundaryResource::Nodes => ken_host::CapacityResourceV1::Nodes,
        BoundaryResource::Words => ken_host::CapacityResourceV1::Words,
        BoundaryResource::DataBytes => ken_host::CapacityResourceV1::DataBytes,
        BoundaryResource::NativeIntLimbs => ken_host::CapacityResourceV1::NativeIntLimbs,
    };
    ken_host::CapacityExhaustedV1 {
        scope, resource, limit: failure.limit, requested: failure.requested,
    }
}
use crate::native_int::NativeIntArenaV1;
use crate::invocation_tickets::InvocationTicketIssuerV1;

/// **One activation: the per-invocation arenas, the services record, and the
/// published bases that generated code is given.**
///
/// ⛔ The `BoundaryValueStore` is **not** owned here — it outlives the
/// activation and may back several in sequence. It is passed to the operations
/// that need it, which keeps *"the store owns the persistent image for as long
/// as any adopted result may live"* a property of the caller's scope rather
/// than of this struct's drop order.
pub struct BoundaryActivationV1 {
    profile: BoundaryResourceProfileV3,
    epoch: u64,
    /// ⛔ Boxed for address stability — see the module doc.
    native_int_arena: Box<NativeIntArenaV1>,
    arena: Box<BoundaryArenaV1>,
    /// Fixed backing and boxed owner are reserved before publishing services.
    call_events: Box<InvocationTicketIssuerV1>,
    /// ⛔ Boxed for the same reason: generated code receives its address.
    services: Box<GeneratedActivationServicesV1>,
    /// The base [`BoundaryArenaV1::publish`] returned, remembered so the
    /// services record can be checked against it without dereferencing
    /// anything.
    published_boundary_base: *mut u64,
    /// The base [`BoundaryValueStore::publish_persistent`] returned.
    published_persistent_base: *mut u64,
    /// The Rust-owned generated-root frame, once bound. ⛔ Boxed: generated
    /// code receives its address.
    frame: Option<Box<GeneratedRootIngressV1>>,
    finished: bool,
}

/// **The store-lifetime half of the ruled order: the persistent image is
/// reserved and published ONCE PER STORE, not once per activation.**
///
/// ⛔⛔ **This split is not stylistic — the first cut of this module got it
/// wrong and the landed guard caught it.** Reserving the persistent image
/// inside each activation panics on the second one with *"reserve before
/// publish: growing a table moves it under the pointer"*, which is
/// `BoundaryRegion::reserve` refusing exactly the thing `§3` forbids.
///
/// ⭐ The correct reading of `§3a`: the **store** owns the persistent image for
/// as long as any adopted persistent result may live, while **each invocation**
/// owns its `NativeIntArenaV1`, `BoundaryArenaV1` and services record. ⇒ Two
/// scopes, two lifetimes, and the profile spans both.
///
/// ⚠ It also makes *"an activation cannot widen its own limits"* structural:
/// the profile lives here, and [`BoundaryActivationV1::begin`] takes it from
/// this binding rather than accepting one from its caller.
pub struct BoundaryStoreBindingV1 {
    profile: BoundaryResourceProfileV3,
    published_persistent_base: *mut u64,
}

impl BoundaryStoreBindingV1 {
    /// Reserve and publish the store's persistent image from the authorized
    /// persistent limits. ⛔ Once per store.
    pub fn open(
        store: &mut BoundaryValueStore,
        profile: BoundaryResourceProfileV3,
    ) -> Result<Self, BoundaryStoreOpenErrorV2> {
        // Admit before publication: a refused profile must not alter the store.
        admit_epoch_ceiling(profile.runtime.invocation_epochs)
            .map_err(BoundaryStoreOpenErrorV2::ProfileMismatch)?;
        // Routed through `as_reserve_arguments` so the named->positional
        // mapping is spelled once in the whole crate.
        let (nodes, words, data, limbs) = profile.persistent.as_reserve_arguments();
        store.reserve_persistent(nodes, words, data, limbs)
            .map_err(|failure| BoundaryStoreOpenErrorV2::CapacityExhausted(
                region_reservation_fault(ken_host::CapacityScopeV1::Persistent, failure)))?;
        let published_persistent_base = store.publish_persistent();
        Ok(BoundaryStoreBindingV1 {
            profile,
            published_persistent_base,
        })
    }

    /// The authorized profile. ⛔ Read-only.
    pub fn profile(&self) -> BoundaryResourceProfileV3 {
        self.profile
    }

    /// The published persistent-image base every activation binds.
    pub fn published_persistent_base(&self) -> *mut u64 {
        self.published_persistent_base
    }
}

impl BoundaryActivationV1 {
    /// **Begin an activation: bind, reserve this invocation's storage, publish,
    /// in the ruled order.**
    ///
    /// ⛔ Every *invocation* reservation happens here and nowhere else. There is
    /// no public reserve on the returned value, so *"no post-publication
    /// reservation"* is a property of the type rather than a rule someone has to
    /// remember. ⭐ The persistent half was already reserved and published by
    /// [`BoundaryStoreBindingV1::open`].
    pub fn begin(binding: &BoundaryStoreBindingV1) -> Result<Self, ken_host::CapacityExhaustedV1> {
        let profile = binding.profile;
        let published_persistent_base = binding.published_persistent_base;

        let mut native_int_arena = Box::new(NativeIntArenaV1::default());
        let mut arena = Box::new(BoundaryArenaBuilder::new().finish());
        arena.bind_persistent(Some(published_persistent_base as *const u64));
        // 2 — bind the native arena. ⭐ The pointer bound here and the pointer
        //     the services record carries are THE SAME, by construction: it is
        //     read once into a local and used twice.
        let native_base: *mut u64 = (&mut *native_int_arena as *mut NativeIntArenaV1).cast();
        arena.bind_native_int(Some(native_base as *const u64));
        // 3 — reserve invocation storage from the AUTHORIZED invocation limits.
        let (nodes, words, data, limbs) = profile.invocation.as_reserve_arguments();
        arena.reserve(nodes, words, data, limbs)
            .map_err(|failure| region_reservation_fault(
                ken_host::CapacityScopeV1::Invocation, failure))?;
        let mut call_events = Box::new(InvocationTicketIssuerV1::reserve(
            0, profile.call_events,
        )?);
        // Consume the process-wide epoch only after all storage has been
        // reserved and before publishing a services pointer or running code.
        let epoch = mint_invocation_epoch(profile.runtime.invocation_epochs)?;
        call_events.bind_epoch_before_publication(epoch);
        // 4 — publish, and only now build the services record.
        let published_boundary_base = arena.publish();
        let services = Box::new(GeneratedActivationServicesV1::new(
            native_base,
            published_boundary_base,
            (&mut *call_events as *mut InvocationTicketIssuerV1).cast(),
        ));

        Ok(BoundaryActivationV1 {
            profile,
            epoch,
            native_int_arena,
            arena,
            call_events,
            services,
            published_boundary_base,
            published_persistent_base,
            frame: None,
            finished: false,
        })
        }

    /// Process-wide identity, minted before this activation was published.
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn call_event_issuer(&mut self) -> Option<&mut InvocationTicketIssuerV1> {
        (!self.finished && self.is_published()).then_some(&mut self.call_events)
    }
    pub fn owned_call_event_address(&self) -> usize {
        (&*self.call_events as *const InvocationTicketIssuerV1) as usize
    }

    /// The `services_ptr` generated code receives as its second parameter.
    ///
    /// ⛔ `None` once the activation is finished: the persistent image has been
    /// sealed and handing generated code a pointer into a sealed world would be
    /// a write path the seal exists to close.
    pub fn services_ptr(&self) -> Option<*const c_void> {
        if self.finished || !self.is_published() {
            return None;
        }
        Some(self.services.as_ptr())
    }

    /// Whether both regions were actually published.
    ///
    /// ⛔⛔ **This exists because a mutation taught it.** Bypassing
    /// `BoundaryArenaV1::publish` left a null base, and every accessor that read
    /// the published header dereferenced it — so `AC-3`(b)'s substitution
    /// produced a **SIGSEGV that took the whole test binary with it**, not a
    /// red. ⚠ A crash is not a loud failure: it destroys the other tests'
    /// results in the same shard and names nothing.
    ///
    /// ⇒ Publication is now a checked state, the header accessors return
    /// `Option`, and an unpublished activation simply has no services pointer to
    /// give — ⭐ which is the permitted fail-closed shape: refusal **before**
    /// generated code is called, ⛔ never a launcher that runs and then quietly
    /// skips it.
    pub fn is_published(&self) -> bool {
        !self.published_boundary_base.is_null() && !self.published_persistent_base.is_null()
    }

    /// The published boundary-arena base, as the owner recorded it at publish
    /// time.
    ///
    /// ⭐ Exposed so the services record can be checked **by pointer equality**
    /// against the value [`BoundaryArenaV1::publish`] actually returned —
    /// ⛔ no dereference, so substituting a foreign pointer is caught without
    /// reading through it.
    pub fn published_boundary_base(&self) -> *mut u64 {
        self.published_boundary_base
    }

    /// The published persistent-image base.
    pub fn published_persistent_base(&self) -> *mut u64 {
        self.published_persistent_base
    }

    /// The native-`Int` arena pointer this activation bound and published, as
    /// the **services record** carries it.
    pub fn native_int_arena_ptr(&self) -> *mut u64 {
        self.services.native_int_arena
    }

    /// The address of the native-`Int` arena this activation **owns**, read
    /// from the box rather than from the services record.
    ///
    /// ⭐⭐ **A second, independent surface on purpose.** A `dead_code` warning
    /// said the owned box was never read — and it was right: every check went
    /// through the services record, so the record could have agreed with itself
    /// while pointing at something the activation does not own. ⇒ Comparing
    /// this against [`Self::native_int_arena_ptr`] is two surfaces agreeing,
    /// ⛔ not one surface read twice.
    ///
    /// ⚠ An **address**, deliberately, not a usable pointer: its only purpose
    /// is identity comparison, and returning `*mut` would invite a caller to
    /// dereference a second alias of the arena.
    pub fn owned_native_arena_address(&self) -> usize {
        (&*self.native_int_arena as *const NativeIntArenaV1) as usize
    }

    /// The profile this activation was authorized with. ⛔ Read-only: an
    /// activation cannot widen its own limits.
    pub fn profile(&self) -> BoundaryResourceProfileV3 {
        self.profile
    }

    /// Read-only view of the invocation arena, for inspecting what generated
    /// code constructed.
    pub fn arena(&self) -> &BoundaryArenaV1 {
        &self.arena
    }

    /// The four capacity ceilings **as generated code will read them**, from
    /// the published header rather than from a Rust-side mirror.
    ///
    /// ⭐ This is deliberately the same read the emitted allocator performs, so
    /// the pin over it measures what generated code will see. ⚠ A Rust-side
    /// accessor would measure a parallel copy and could agree while the header
    /// disagreed.
    ///
    /// Order: nodes, words, data bytes, limbs — the same order as
    /// `reserve`, so it can be compared against `as_reserve_arguments` directly.
    ///
    /// # Safety
    ///
    /// Reads the header this activation itself published and still owns. The
    /// allocation is alive for `&self`, and the offsets are the region's own
    /// derived field offsets.
    pub fn published_capacities(&self) -> Option<(u64, u64, u64, u64)> {
        let base = self.published_boundary_base;
        if base.is_null() {
            return None;
        }
        unsafe {
            Some((
                header_word(base, ARENA_NODE_CAPACITY),
                header_word(base, ARENA_WORD_CAPACITY),
                header_word(base, ARENA_DATA_CAPACITY),
                header_word(base, ARENA_LIMB_CAPACITY),
            ))
        }
    }

    /// The two pointers the published boundary header carries — the persistent
    /// image and the native-`Int` arena.
    ///
    /// # Safety
    ///
    /// As [`Self::published_capacities`].
    pub fn published_bindings(&self) -> Option<(u64, u64)> {
        let base = self.published_boundary_base;
        if base.is_null() {
            return None;
        }
        unsafe {
            Some((
                header_word(base, ARENA_PERSISTENT),
                header_word(base, ARENA_NATIVE_INT),
            ))
        }
    }

    /// **`AC-4` — name WHICH of the eight authorized limits a capacity refusal
    /// was about.**
    ///
    /// ⛔⛔ **Without this, `AC-4` is not dischargeable at all.** Emitted code
    /// answers a single [`crate::boundary_value::BOUNDARY_ERR_CAPACITY`] for
    /// every exhausted table, in either region — ⚠ so a control that asserts
    /// only *"the status was `ERR_CAPACITY`"* is **one control claiming to be
    /// eight**, and cannot tell a persistent data-byte ceiling from an
    /// invocation node ceiling.
    ///
    /// ⭐ The attribution is a comparison between two independent things: the
    /// region's **live count**, which emitted code bumped, and the **authorized
    /// limit** the deployment wrote. ⛔ Not a re-reading of the status.
    ///
    /// Returns the `(scope, resource)` whose live count has reached its
    /// authorized ceiling, or `None` if none has — ⚠ which is itself
    /// informative: a capacity refusal with nothing at its ceiling means the
    /// refusal came from somewhere other than these eight limits, and a test
    /// that ignored `None` would call that a pass.
    ///
    /// ⚠ **It reports the FIRST resource at its ceiling** in inventory order.
    /// That is unambiguous only when the other seven have room, which is why
    /// each control grants exactly one tight limit — ⛔ a fixture that tightened
    /// two would get a well-defined but arbitrary answer.
    pub fn attribute_capacity_exhaustion(
        &self,
        store: &BoundaryValueStore,
    ) -> Option<BoundaryCapacityExhaustedV1> {
        for scope in BoundaryResourceScope::ALL {
            for resource in BoundaryResource::ALL {
                let limit = self.profile.limit(scope, resource);
                let live = self.live_count(store, scope, resource);
                if live >= limit {
                    return Some(BoundaryCapacityExhaustedV1 {
                        scope,
                        resource,
                        limit,
                        requested: live + 1,
                    });
                }
            }
        }
        None
    }

    /// The live count of one resource in one region.
    ///
    /// ⛔ Read from the region itself — what emitted code actually bumped —
    /// never from a Rust-side mirror of what it was expected to bump.
    fn live_count(
        &self,
        store: &BoundaryValueStore,
        scope: BoundaryResourceScope,
        resource: BoundaryResource,
    ) -> usize {
        let region = match scope {
            BoundaryResourceScope::Invocation => &self.arena.0,
            BoundaryResourceScope::Persistent => &store.image().0,
        };
        match resource {
            BoundaryResource::Nodes => region.node_count(),
            BoundaryResource::Words => region.word_count(),
            BoundaryResource::DataBytes => region.data_count(),
            BoundaryResource::NativeIntLimbs => region.limb_count(),
        }
    }

    /// **Finish the activation: seal the persistent image, then adopt an
    /// escaping result.**
    ///
    /// ⛔ Seal first. Adoption absorbs the published counts, validates the
    /// reachable graph, mints identity and publishes — every step reading a
    /// snapshot it assumes is stable. A writer that can still run makes that
    /// snapshot a fiction.
    ///
    /// ⛔ After this the services pointer is withdrawn.
    pub fn finish(
        &mut self,
        store: &mut BoundaryValueStore,
        escaping: Option<BoundaryWord>,
    ) -> Result<Option<BoundaryWord>, i64> {
        self.finished = true;
        store.seal_persistent();
        match escaping {
            None => Ok(None),
            Some(word) => store.adopt(word).map(Some),
        }
    }

    /// Whether [`Self::finish`] has run.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// **Bind this activation's process ingress and hand back the `frame_ptr`
    /// generated code receives as its FIRST parameter.**
    ///
    /// The launch ingress is Rust-owned and C-opaque. It carries only the
    /// process source pair and the direct host-dispatch context; runtime
    /// services travel through the separate services pointer.
    ///
    /// ⛔ Boxed, for the same address-stability reason as the other owned
    /// objects: generated code receives this pointer.
    ///
    /// ⛔ `None` once finished or before publication, exactly as
    /// [`Self::services_ptr`] — a caller must not be able to obtain half of a
    /// withdrawn pair.
    pub fn bind_process_frame(
        &mut self,
        process_input: *const c_void,
        host_context: *mut c_void,
        capability: u64,
    ) -> Option<*const c_void> {
        if self.finished || !self.is_published() {
            return None;
        }
        let frame = Box::new(GeneratedRootIngressV1 {
            process_input,
            host_dispatch_context: host_context,
            capability,
        });
        let pointer = (&*frame as *const GeneratedRootIngressV1).cast::<c_void>();
        self.frame = Some(frame);
        Some(pointer)
    }

    /// The opaque launch pointer for a non-process root.
    ///
    /// The root adapter ignores this pointer in value mode; the native arena is
    /// obtained from the services record. Keeping the owner-provided pointer
    /// preserves one launch surface for C without exposing a layout.
    ///
    /// ⛔ Withdrawn on the same condition as the services view.
    pub fn native_frame_ptr(&self) -> Option<*const c_void> {
        if self.finished || !self.is_published() {
            return None;
        }
        Some(self.services.native_int_arena.cast::<c_void>())
    }

    /// This activation's native-`Int` arena, for reading back the final export.
    ///
    /// ⭐ The reason the C stub no longer needs the layout: it asks the owner
    /// rather than reading fields it declared itself.
    pub fn native_int_arena(&self) -> &NativeIntArenaV1 {
        &self.native_int_arena
    }

    /// The `frame_ptr` previously bound, if any.
    pub fn frame_ptr(&self) -> Option<*const c_void> {
        if self.finished || !self.is_published() {
            return None;
        }
        self.frame
            .as_ref()
            .map(|frame| (&**frame as *const GeneratedRootIngressV1).cast::<c_void>())
    }
}

/// Every field of the process-root launch ingress, in layout order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootIngressField {
    ProcessInput,
    HostDispatchContext,
    Capability,
}

impl RootIngressField {
    pub const ALL: [Self; 3] = [
        Self::ProcessInput,
        Self::HostDispatchContext,
        Self::Capability,
    ];

    pub const fn offset(self) -> i32 {
        (self as i32) * 8
    }
}

pub const ROOT_INGRESS_PROCESS_INPUT: i32 = RootIngressField::ProcessInput.offset();
pub const ROOT_INGRESS_HOST_DISPATCH_CONTEXT: i32 = RootIngressField::HostDispatchContext.offset();
pub const ROOT_INGRESS_CAPABILITY: i32 = RootIngressField::Capability.offset();
pub const ROOT_INGRESS_BYTES: i32 = (RootIngressField::ALL.len() * 8) as i32;

/// The generated public adapter's process launch ingress, owned by Rust.
#[repr(C)]
pub struct GeneratedRootIngressV1 {
    /// The borrowed process-input value the launcher built.
    pub process_input: *const c_void,
    /// The host effect context from `ken_host_invocation_v1_init`.
    pub host_dispatch_context: *mut c_void,
    /// The capability token that init issued.
    pub capability: u64,
}

/// One word of a published region header.
///
/// # Safety
///
/// `base` must be a header published by this crate and still alive, and
/// `offset` must be one of the region's derived field offsets.
unsafe fn header_word(base: *mut u64, offset: i32) -> u64 {
    unsafe { *base.byte_offset(offset as isize) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary_resource_profile::{
        BoundaryRegionLimitsV1, BoundaryResource, BoundaryResourceScope, RuntimeResourceLimitsV2,
    };

    /// Eight **distinct** limits, so a transposition anywhere in `begin` is
    /// visible. ⛔ Equal limits would let every assertion below pass on a
    /// crossed wiring.
    fn distinct_profile() -> BoundaryResourceProfileV3 {
        BoundaryResourceProfileV3 {
            runtime: RuntimeResourceLimitsV2 {
                invocation_epochs: u64::MAX,
            },
            call_events: crate::boundary_resource_profile::InvocationCallLimitsV3 {
                event_generations: 29,
                live_pending_slots: 13,
            },
            invocation: BoundaryRegionLimitsV1 {
                nodes: 12,
                words: 24,
                data_bytes: 36,
                native_int_limbs: 48,
            },
            persistent: BoundaryRegionLimitsV1 {
                nodes: 60,
                words: 72,
                data_bytes: 84,
                native_int_limbs: 96,
            },
        }
    }

    fn refusal_from_real_reservation(
        persistent: bool,
        resource: BoundaryResource,
        requested: usize,
    ) -> ken_host::CapacityExhaustedV1 {
        let mut profile = distinct_profile();
        let limits = if persistent { &mut profile.persistent } else { &mut profile.invocation };
        match resource {
            BoundaryResource::Nodes => limits.nodes = requested,
            BoundaryResource::Words => limits.words = requested,
            BoundaryResource::DataBytes => limits.data_bytes = requested,
            BoundaryResource::NativeIntLimbs => limits.native_int_limbs = requested,
        }
        let mut store = BoundaryValueStore::new();
        if persistent {
            match BoundaryStoreBindingV1::open(&mut store, profile).err().expect("open refuses") {
                BoundaryStoreOpenErrorV2::CapacityExhausted(fault) => fault,
                BoundaryStoreOpenErrorV2::ProfileMismatch(_) => panic!("wrong failure source"),
            }
        } else {
            let binding = BoundaryStoreBindingV1::open(&mut store, profile).expect("open succeeds");
            BoundaryActivationV1::begin(&binding).err().expect("begin refuses")
        }
    }

    /// Durable capacity invariant. Both base-red paths at exact landed D1
    /// e49cdc9b panicked at `(live_nodes + nodes) * NODE_WORDS` for each owner.
    #[test]
    fn unrepresentable_node_grants_do_not_panic_at_open_or_begin() {
        let requested = usize::MAX / (crate::boundary_value::BOUNDARY_NODE_STRIDE as usize / 8) + 1;
        let actual_limit = (isize::MAX as usize / 8)
            / (crate::boundary_value::BOUNDARY_NODE_STRIDE as usize / 8);
        for persistent in [false, true] {
            let fault = refusal_from_real_reservation(persistent, BoundaryResource::Nodes, requested);
            assert_eq!(fault.scope, if persistent { ken_host::CapacityScopeV1::Persistent } else { ken_host::CapacityScopeV1::Invocation });
            assert_eq!(fault.resource, ken_host::CapacityResourceV1::Nodes);
            assert_eq!(fault.limit, actual_limit as u128);
            assert_eq!(fault.requested, requested as u128);
        }
    }

    /// Durable capacity invariant. Both base-red paths at exact landed D1
    /// e49cdc9b panicked at `Vec::resize` for the max child-word grant.
    #[test]
    fn max_word_grants_do_not_panic_at_open_or_begin() {
        let actual_limit = isize::MAX as usize / 8;
        for persistent in [false, true] {
            let fault = refusal_from_real_reservation(persistent, BoundaryResource::Words, usize::MAX);
            assert_eq!(fault.scope, if persistent { ken_host::CapacityScopeV1::Persistent } else { ken_host::CapacityScopeV1::Invocation });
            assert_eq!(fault.resource, ken_host::CapacityResourceV1::Words);
            assert_eq!(fault.limit, actual_limit as u128);
            assert_eq!(fault.requested, usize::MAX as u128);
        }
    }

    /// Independent positive baseline: a materially larger but reservable
    /// deployment profile must remain usable at both real publication owners.
    #[test]
    fn reservable_large_grants_publish_at_open_and_begin() {
        let mut profile = distinct_profile();
        profile.invocation.nodes = 4_096;
        profile.invocation.words = 8_192;
        profile.persistent.nodes = 4_096;
        profile.persistent.words = 8_192;
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, profile).expect("large open");
        let activation = BoundaryActivationV1::begin(&binding).expect("large begin");
        assert!(activation.is_published());
        assert_eq!(activation.published_capacities().unwrap().0, 4_096);
    }

    /// The separately metered live-call backing also refuses an unreservable
    /// declared grant before the activation publishes a services pointer.
    #[test]
    fn impossible_live_slot_grant_refuses_before_publication() {
        let mut profile = distinct_profile();
        profile.call_events.live_pending_slots = usize::MAX;
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, profile)
            .expect("persistent region alone has ordinary limits");
        let fault = BoundaryActivationV1::begin(&binding)
            .err().expect("unrepresentable call-event backing cannot publish");
        assert_eq!(fault.scope, ken_host::CapacityScopeV1::Invocation);
        assert_eq!(fault.resource, ken_host::CapacityResourceV1::LivePendingSlots);
        assert_eq!(fault.requested, usize::MAX as u128);
        assert!(fault.limit < fault.requested);
    }

    #[test]
    fn one_past_max_epoch_has_an_exact_nonwrapping_request() {
        assert_eq!(requested_epoch_after(u64::MAX), u128::from(u64::MAX) + 1);
    }

    /// Promise: durable resource invariant. The parent launches a fresh test
    /// process so the process-wide counter really begins at zero; resetting a
    /// shared atomic in this test would permit epoch reuse during other tests.
    #[test]
    fn bounded_epochs_have_exact_at_limit_and_one_past_across_stores() {
        let binary = std::env::current_exe().expect("test executable");
        let result = std::process::Command::new(binary)
            .arg("--exact")
            .arg("boundary_activation::tests::bounded_epoch_child")
            .env("KEN_EPOCH_ISOLATED_CHILD", "1")
            .output()
            .expect("spawn isolated epoch process");
        assert!(result.status.success(), "isolated child: {result:?}");
        let output = String::from_utf8_lossy(&result.stdout);
        assert!(
            output.contains("1 passed; 0 failed"),
            "test must run: {output}"
        );
    }

    #[test]
    fn bounded_epoch_child() {
        if std::env::var_os("KEN_EPOCH_ISOLATED_CHILD").is_none() {
            return;
        }
        let mut profile = distinct_profile();
        profile.runtime.invocation_epochs = 2;
        let mut first_store = BoundaryValueStore::new();
        let first_binding = BoundaryStoreBindingV1::open(&mut first_store, profile).expect("matching process epoch ceiling");
        let first = BoundaryActivationV1::begin(&first_binding).expect("epoch 1");
        assert_eq!(first.epoch(), 1);
        drop(first);
        drop(first_binding);
        drop(first_store);
        let mut next_store = BoundaryValueStore::new();
        let next_binding = BoundaryStoreBindingV1::open(&mut next_store, profile).expect("matching process epoch ceiling");
        let second = BoundaryActivationV1::begin(&next_binding).expect("epoch 2");
        assert_eq!(second.epoch(), 2);
        drop(second);
        let err = BoundaryActivationV1::begin(&next_binding)
            .err()
            .expect("epoch 3 refuses");
        assert_eq!(
            err,
            ken_host::CapacityExhaustedV1 {
                scope: ken_host::CapacityScopeV1::Runtime,
                resource: ken_host::CapacityResourceV1::InvocationEpochs,
                limit: 2,
                requested: 3,
            }
        );
        // The fault is produced by the real third begin, not constructed by
        // this test. The same typed value must survive the linked wire intact.
        let linked = ken_host::LinkedEffectTrace {
            plan_hash: 1,
            target_abi_hash: ken_host::TARGET_ABI_MANIFEST_HASH,
            host_effect_abi_hash: ken_host::HOST_EFFECT_ABI_V1_HASH,
            terminal_value: ken_host::CAPACITY_EXHAUSTED_STATUS_V1,
            terminal_error: Some(ken_host::TerminalErrorV1::CapacityExhausted(err)),
            effect_trace: Vec::new(),
            terminal_exit: ken_host::TerminalExitClass::ControlledTrap,
        };
        let encoded = ken_host::encode_linked_effect_trace(&linked).expect("real fault encodes");
        assert_eq!(ken_host::decode_linked_effect_trace(&encoded), Ok(linked));
    }

    fn run_epoch_child(test_name: &str) {
        let result = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .arg("--exact")
            .arg(format!("boundary_activation::tests::{test_name}"))
            .env("KEN_EPOCH_ISOLATED_CHILD", "1")
            .output()
            .expect("spawn isolated epoch process");
        assert!(result.status.success(), "isolated child: {result:?}");
        let output = String::from_utf8_lossy(&result.stdout);
        assert!(output.contains("1 passed; 0 failed"), "test did not run: {output}");
    }

    #[test]
    fn later_stores_may_change_only_their_own_region_limits() {
        let first = distinct_profile();
        let mut next = first;
        next.persistent.nodes += 1;
        next.invocation.words += 1;
        let mut store_a = BoundaryValueStore::new();
        let mut store_b = BoundaryValueStore::new();
        let binding_a = BoundaryStoreBindingV1::open(&mut store_a, first).unwrap();
        let binding_b = BoundaryStoreBindingV1::open(&mut store_b, next).unwrap();
        assert_eq!(binding_a.profile(), first);
        assert_eq!(binding_b.profile(), next);
        assert_eq!(store_b.image().0.node_capacity(), next.persistent.nodes);
    }

    #[test]
    fn a_first_cap_of_one_cannot_be_raised_to_two_after_use() {
        run_epoch_child("cap_one_cannot_be_raised_child");
    }

    #[test]
    fn cap_one_cannot_be_raised_child() {
        if std::env::var_os("KEN_EPOCH_ISOLATED_CHILD").is_none() {
            return;
        }
        let mut profile = distinct_profile();
        profile.runtime.invocation_epochs = 1;
        let mut first_store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut first_store, profile).unwrap();
        assert_eq!(BoundaryActivationV1::begin(&binding).unwrap().epoch(), 1);
        profile.runtime.invocation_epochs = 2;
        let mut next_store = BoundaryValueStore::new();
        assert_eq!(
            BoundaryStoreBindingV1::open(&mut next_store, profile).err(),
            Some(BoundaryStoreOpenErrorV2::ProfileMismatch(InvocationEpochProfileMismatchV2 { authorized: 1, attempted: 2 }))
        );
        assert_eq!(next_store.image().0.node_capacity(), 0, "mismatch published storage");
        assert_eq!(BoundaryActivationV1::begin(&binding).err().unwrap().requested, 2);
    }

    #[test]
    fn a_first_cap_of_zero_cannot_be_raised_before_any_activation() {
        run_epoch_child("cap_zero_cannot_be_raised_child");
    }

    #[test]
    fn cap_zero_cannot_be_raised_child() {
        if std::env::var_os("KEN_EPOCH_ISOLATED_CHILD").is_none() {
            return;
        }
        let mut profile = distinct_profile();
        profile.runtime.invocation_epochs = 0;
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, profile).unwrap();
        assert_eq!(BoundaryActivationV1::begin(&binding).err().unwrap(),
            ken_host::CapacityExhaustedV1 {
                scope: ken_host::CapacityScopeV1::Runtime,
                resource: ken_host::CapacityResourceV1::InvocationEpochs,
                limit: 0, requested: 1,
            });
        profile.runtime.invocation_epochs = 5;
        let mut later = BoundaryValueStore::new();
        assert_eq!(BoundaryStoreBindingV1::open(&mut later, profile).err(),
            Some(BoundaryStoreOpenErrorV2::ProfileMismatch(InvocationEpochProfileMismatchV2 { authorized: 0, attempted: 5 })));
    }

    #[test]
    fn concurrent_begins_issue_unique_epochs_without_overrunning_one_cap() {
        run_epoch_child("concurrent_epoch_child");
    }

    #[test]
    fn concurrent_epoch_child() {
        if std::env::var_os("KEN_EPOCH_ISOLATED_CHILD").is_none() {
            return;
        }
        let mut profile = distinct_profile();
        profile.runtime.invocation_epochs = 12;
        let handles = (0..20).map(|_| std::thread::spawn(move || {
            let mut store = BoundaryValueStore::new();
            let binding = BoundaryStoreBindingV1::open(&mut store, profile).unwrap();
            BoundaryActivationV1::begin(&binding).map(|activation| activation.epoch())
        })).collect::<Vec<_>>();
        let mut epochs = Vec::new();
        let mut refused = Vec::new();
        for handle in handles {
            match handle.join().expect("worker must not panic") {
                Ok(epoch) => epochs.push(epoch),
                Err(fault) => refused.push(fault),
            }
        }
        epochs.sort_unstable();
        assert_eq!(epochs, (1..=12).collect::<Vec<_>>());
        assert_eq!(refused.len(), 8);
        assert!(refused.iter().all(|fault| *fault == ken_host::CapacityExhaustedV1 {
            scope: ken_host::CapacityScopeV1::Runtime,
            resource: ken_host::CapacityResourceV1::InvocationEpochs,
            limit: 12, requested: 13,
        }));
    }

    /// ⭐⭐ **`AC-3`(c) — Finding 8's exact defect, caught by pointer identity
    /// and ⛔ without dereferencing anything.**
    ///
    /// **MEASURED:** the services record's `boundary_arena` is the pointer
    /// `BoundaryArenaV1::publish` returned, and its `native_int_arena` is the
    /// pointer bound into the header at `ARENA_NATIVE_INT` — and the two are
    /// **different**.
    /// **CLAIMED:** generated code loading `SERVICES_BOUNDARY_ARENA` reaches the
    /// boundary arena and not the native one.
    /// **THE GAP:** ⛔ that generated code loads that field at all. The
    /// per-function binder is held on `B2F`; ⚠ this is the owner's half of the
    /// contract, not the emitter's.
    ///
    /// ⚠ **Why equality and not a header read:** substituting the native arena
    /// makes every header offset past 64 bytes an out-of-bounds read, so a
    /// "check the header looks wrong" control would be undefined behaviour
    /// rather than a red. Pointer identity fails cleanly.
    #[test]
    fn the_services_record_carries_the_boundary_base_and_not_the_native_one() {
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, distinct_profile()).expect("matching process epoch ceiling");
        let activation = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");

        let services = activation.services_ptr().expect("live activation");
        assert!(!services.is_null());

        assert_eq!(
            activation.services.boundary_arena,
            activation.published_boundary_base(),
            "AC-3(c): the services record's boundary arena is not the pointer \
             `publish` returned"
        );
        assert_eq!(
            activation.services.native_int_arena,
            activation.native_int_arena_ptr()
        );
        assert_ne!(
            activation.services.boundary_arena, activation.services.native_int_arena,
            "AC-3(c): the boundary and native arenas are the same pointer — this \
             is Finding 8 and it must be impossible, not merely absent"
        );

        // The header agrees with the record about the native arena, which is
        // the ruling's "one native authority" property.
        let (persistent, native) = activation
            .published_bindings()
            .expect("AC-3(b): a published activation must expose its header bindings");
        assert_eq!(native, activation.services.native_int_arena as u64);
        assert_eq!(persistent, activation.published_persistent_base() as u64);
        assert_ne!(persistent, 0, "ARENA_PERSISTENT was left unbound");
        assert_ne!(native, 0, "ARENA_NATIVE_INT was left unbound");
        let owner = activation.owned_call_event_address();
        assert_eq!(activation.services.call_events as usize, owner);
        let moved = Box::new(activation);
        assert_eq!(moved.services.call_events as usize, moved.owned_call_event_address());
        assert_eq!(moved.owned_call_event_address(), owner,
            "moving the activation cannot move its published issuer owner");
    }

    /// ⭐⭐ **`AC-2` — two activations get distinct mutable arena state.**
    ///
    /// ⚠ **This is the control that would have caught the artifact-static
    /// fork**, and it is written to fail if storage is aliased. ⛔ A fixture
    /// that runs one activation twice and checks it did not crash is *not*
    /// this: the two activations are alive **simultaneously** and their
    /// published bases, native arenas and services records are all required to
    /// be pairwise distinct.
    ///
    /// ⭐ They share exactly one thing, and it is the one the ruling permits:
    /// the store-owned persistent image.
    #[test]
    fn two_activations_do_not_share_mutable_arena_state() {
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, distinct_profile()).expect("matching process epoch ceiling");
        let first = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");
        let second = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");

        assert_ne!(
            first.published_boundary_base(),
            second.published_boundary_base(),
            "AC-2: two activations published the SAME arena base — their \
             mutable node/word counts alias"
        );
        assert_ne!(
            first.native_int_arena_ptr(),
            second.native_int_arena_ptr(),
            "AC-2: two activations share one native-Int arena"
        );
        assert_ne!(
            first.owned_native_arena_address(),
            second.owned_native_arena_address(),
            "AC-2: the two OWNED native arenas are one allocation, so the \
             records above differ while the storage behind them does not"
        );
        // The record and the owned box agree, per activation — two surfaces,
        // not one read twice.
        assert_eq!(
            first.native_int_arena_ptr() as usize,
            first.owned_native_arena_address()
        );
        assert_eq!(
            second.native_int_arena_ptr() as usize,
            second.owned_native_arena_address()
        );
        assert_ne!(
            first.services_ptr().expect("live"),
            second.services_ptr().expect("live"),
            "AC-2: two activations share one services record"
        );

        // ⭐ And what they DO share is exactly the permitted thing.
        assert_eq!(
            first.published_persistent_base(),
            second.published_persistent_base(),
            "the store-owned persistent image is explicitly shared; if this \
             diverges the store is no longer the single persistent authority"
        );
    }

    /// ⛔ **Moving the activation does not move the pointers generated code
    /// holds.**
    ///
    /// ⚠ The failure this prevents is silent: `NativeIntArenaV1` is a plain
    /// `#[repr(C)]` struct, so an unboxed field would hand out a pointer *into*
    /// the value, and moving the handle — returning it, pushing it into a
    /// `Vec`, storing it in a C-owned box — would dangle it with nothing
    /// looking wrong.
    #[test]
    fn moving_the_activation_does_not_move_what_generated_code_was_given() {
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, distinct_profile()).expect("matching process epoch ceiling");
        let activation = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");
        let (services, native, boundary) = (
            activation.services_ptr().expect("live"),
            activation.native_int_arena_ptr(),
            activation.published_boundary_base(),
        );

        // Move it twice, through a container, exactly as the C ABI will.
        let boxed = Box::new(activation);
        let mut moved = vec![*boxed];
        let activation = moved.pop().expect("one activation");

        assert_eq!(activation.services_ptr().expect("live"), services);
        assert_eq!(activation.native_int_arena_ptr(), native);
        assert_eq!(activation.published_boundary_base(), boundary);
    }

    /// ⭐ **`AC-4`'s WIRING half — each of the eight authorized limits reaches
    /// its own ceiling, read as generated code reads it.**
    ///
    /// **MEASURED:** the four capacity words in the *published header* equal the
    /// profile's four invocation limits, in order; and the persistent image's
    /// node ceiling equals the profile's persistent node limit.
    /// **CLAIMED:** each limit governs its named region and resource.
    /// **THE GAP:** ⛔⛔ **at-limit-plus-one is NOT measured here.** `AC-4`
    /// requires a request one past each ceiling to fail loudly naming that
    /// exact scope, and the requester is *generated code*, which this node does
    /// not make live. ⇒ That half is `S4`'s, against a real linked run.
    /// ⛔ Do not read this test as `AC-4` discharged.
    ///
    /// ⚠ Not circular: it crosses from the **profile** the deployment wrote to
    /// the **published header** generated code will read — two different
    /// objects — so a transposition inside `begin` reddens it.
    #[test]
    fn the_eight_authorized_limits_reach_their_own_published_ceilings() {
        let profile = distinct_profile();
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, profile).expect("matching process epoch ceiling");
        let activation = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");

        let (nodes, words, data, limbs) = activation
            .published_capacities()
            .expect("AC-3(b): a published activation must expose its ceilings");
        let expected = profile.invocation.as_reserve_arguments();
        assert_eq!(
            (
                nodes as usize,
                words as usize,
                data as usize,
                limbs as usize
            ),
            expected,
            "AC-4 wiring: the invocation ceilings in the published header are \
             not the four authorized invocation limits, in order"
        );

        // The persistent side, through the store's own accessor.
        assert_eq!(
            store.image().0.node_capacity(),
            profile.persistent.nodes,
            "AC-4 wiring: the persistent node ceiling is not the authorized one"
        );

        // Non-vacuity: the eight authorized numbers are pairwise distinct, so
        // agreeing with them is a real constraint rather than a coincidence.
        let mut seen = std::collections::BTreeSet::new();
        for scope in BoundaryResourceScope::ALL {
            for resource in BoundaryResource::ALL {
                assert!(seen.insert(profile.limit(scope, resource)));
            }
        }
        assert_eq!(seen.len(), 8);
    }

    /// ⭐⭐ The launch ingress carries only the ruled source pair and direct
    /// host context; the native arena remains exclusively in services.
    ///
    /// **MEASURED:** the fourth field of the Rust-owned invocation record is the
    /// same pointer the services record carries and the same allocation the
    /// activation owns; two activations get two frames pointing at two arenas;
    /// and the frame is withdrawn together with the services pointer.
    /// **CLAIMED:** removing the C stub's own `KenNativeIntArenaV1` and its
    /// stack construction loses nothing generated code needs.
    /// **THE GAP:** ⛔ that the stub actually stops declaring them. That is a
    /// build/link fact and it is `S4b`'s, together with `AC-5`.
    #[test]
    fn the_generated_root_ingress_excludes_the_native_arena() {
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, distinct_profile()).expect("matching process epoch ceiling");
        let mut first = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");
        let mut second = BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");

        let a = first
            .bind_process_frame(std::ptr::null(), std::ptr::null_mut(), 7)
            .expect("a published, unfinished activation binds a frame");
        let b = second
            .bind_process_frame(std::ptr::null(), std::ptr::null_mut(), 9)
            .expect("and so does the second");
        assert_ne!(a, b, "two activations share one generated-root frame");

        let frame = unsafe { &*(a as *const GeneratedRootIngressV1) };
        assert_eq!(frame.capability, 7);
        assert!(frame.process_input.is_null());
        assert!(frame.host_dispatch_context.is_null());
        let other = unsafe { &*(b as *const GeneratedRootIngressV1) };
        assert_eq!(other.capability, 9);
        assert_eq!(
            std::mem::size_of::<GeneratedRootIngressV1>(),
            3 * std::mem::size_of::<usize>()
        );

        // Withdrawn together with the services pointer: a caller must not be
        // able to obtain half of a withdrawn pair.
        assert!(first.frame_ptr().is_some());
        first
            .finish(&mut store, None)
            .expect("finishing with nothing escaping cannot fail");
        assert!(first.frame_ptr().is_none());
        assert!(first.services_ptr().is_none());
    }

    /// ⛔ **`AC-3`(b) — the services pointer is withdrawn once the activation is
    /// finished**, so a caller cannot hand generated code a way into a sealed
    /// world.
    #[test]
    fn finishing_withdraws_the_services_pointer_and_seals_the_image() {
        let mut store = BoundaryValueStore::new();
        let binding = BoundaryStoreBindingV1::open(&mut store, distinct_profile()).expect("matching process epoch ceiling");
        let mut activation =
            BoundaryActivationV1::begin(&binding).expect("explicit test epoch budget");
        assert!(activation.services_ptr().is_some());
        assert!(!store.is_persistent_sealed());

        activation
            .finish(&mut store, None)
            .expect("finishing with nothing escaping cannot fail");

        assert!(activation.is_finished());
        assert!(activation.services_ptr().is_none());
        assert!(
            store.is_persistent_sealed(),
            "AC-3(b): finishing did not seal the persistent image, so adoption \
             would read a snapshot writers can still change"
        );
    }
}
