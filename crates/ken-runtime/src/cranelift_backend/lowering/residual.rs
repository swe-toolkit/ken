//! Private representation of an issued recursive-position residual. Only this
//! module can read its SSA word or turn it into an ordinary boundary value.

use super::*;

/// The slot, not a dynamically selected member of its closed flow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RecursiveCarrierSlotKey {
    eliminator: StaticOriginId,
    constructor: ConstructorIdentity,
    position: u32,
}

impl RecursiveCarrierSlotKey {
    pub(super) fn new(
        eliminator: StaticOriginId,
        constructor: ConstructorIdentity,
        position: u32,
    ) -> Self {
        Self { eliminator, constructor, position }
    }

    pub(super) fn of(slot: &RecursiveCarrierSlot) -> Self {
        Self::new(slot.eliminator, slot.constructor, slot.position)
    }
}

/// An R record and its planner-issued source slot. The key exists only in
/// lowering; the emitted word contains no variant or static provenance.
#[derive(Clone, Copy, Debug)]
pub(super) struct CarriedResidualWord {
    word: cranelift_codegen::ir::Value,
    slot: RecursiveCarrierSlotKey,
}

impl CarriedResidualWord {
    pub(super) fn issue(word: cranelift_codegen::ir::Value, slot: RecursiveCarrierSlotKey) -> Self {
        Self { word, slot }
    }

    pub(super) fn slot(self) -> RecursiveCarrierSlotKey {
        self.slot
    }

    /// I-0 only: an explicitly counted crossing of an untyped ABI seat.
    /// I-1 deletes this method when every such seat has a planner-issued kind.
    pub(super) fn residual_across_untyped_abi_transitional(self) -> CarriedBoundaryWord {
        #[cfg(any(test, feature = "px8-ds-test-support"))]
        super::record_residual_transitional_escape();
        CarriedBoundaryWord { word: self.word }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Representation {
    Residual(RecursiveCarrierSlotKey),
    Value,
}

pub(super) enum CarriedBlockParam {
    Residual(CarriedResidualWord),
    Value(CarriedBoundaryWord),
}

pub(super) fn append_carried_block_param(
    builder: &mut FunctionBuilder<'_>,
    block: cranelift_codegen::ir::Block,
    representation: Representation,
) -> CarriedBlockParam {
    let word = builder.append_block_param(block, types::I64);
    match representation {
        Representation::Value => CarriedBlockParam::Value(CarriedBoundaryWord { word }),
        Representation::Residual(slot) => CarriedBlockParam::Residual(CarriedResidualWord::issue(word, slot)),
    }
}

impl<'a> Lowering<'a> {
    /// Store an issued R into a constructor field without presenting it to
    /// the ordinary K-field store API. Caller validates the exact slot edge.
    pub(super) fn store_residual_field_passthrough(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
        position: usize,
        residual: CarriedResidualWord,
    ) -> Result<(), CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let index = Self::carrier_position_immediate(builder, position)?;
        let call = builder.ins().call(
            refs.store_field, &[arena, target.word, index, residual.word],
        );
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(())
    }

    /// Inspect the issued discriminant without converting R to a value.
    pub(super) fn residual_label_ordinal(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        residual: CarriedResidualWord,
        slot: &RecursiveCarrierSlot,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        if residual.slot != RecursiveCarrierSlotKey::of(slot) || slot.flow.len() < 2 {
            return Err(unsupported("RecursiveResidual", "label reader has no matching issued slot"));
        }
        let first = *slot.flow.first().ok_or_else(|| unsupported(
            "RecursiveResidual", "the labelled slot has no member",
        ))?;
        let label_index = slot.variant(first)?.role_index(RecursiveCarrierRole::Label)?;
        let wrapped = CarriedBoundaryWord { word: residual.word };
        let class = self.emit_carrier_class(builder, wrapped)?;
        Self::require_i64(builder, class, BoundaryClass::Record as i64);
        let tag = self.emit_carrier_tag(builder, wrapped)?;
        Self::require_i64(builder, tag, 1);
        let label = self.emit_carrier_field(builder, wrapped, label_index)?;
        Ok(Self::emit_carrier_label_ordinal(builder, label))
    }

    /// W/C readers know their specialization; they must also agree on the
    /// source slot carried by R before looking up that variant's roles.
    pub(super) fn assert_recursive_carrier_variant(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        residual: CarriedResidualWord,
        disposition: &RecursiveResidualDisposition,
    ) -> Result<RecursiveCarrierVariant, CraneliftBackendError> {
        if !disposition.wrapped() {
            return Err(unsupported("RecursiveResidual", "the bound residual has no issued sum schema"));
        }
        let slot = self.static_transition_plan
            .recursive_carrier_for_specialization(disposition.specialization)?
            .ok_or_else(|| unsupported("RecursiveResidual", "the bound residual has no planner slot"))?;
        if residual.slot != RecursiveCarrierSlotKey::of(slot) {
            return Err(unsupported("RecursiveResidual", "R has a foreign slot key at W/C consumption"));
        }
        let variant = slot.variant(disposition.specialization)?.clone();
        if disposition.record != Some(variant.record)
            || disposition.label != variant.label
            || disposition.position != slot.position
            || disposition.constructor != slot.constructor
            || variant.roles.len() != disposition.field_count()
        {
            return Err(unsupported("RecursiveResidual", "the bound residual differs from its issued slot variant"));
        }
        let wrapped = CarriedBoundaryWord { word: residual.word };
        let class = self.emit_carrier_class(builder, wrapped)?;
        Self::require_i64(builder, class, BoundaryClass::Record as i64);
        let tag = self.emit_carrier_tag(builder, wrapped)?;
        Self::require_i64(builder, tag, 1);
        let count = self.emit_carrier_field_count(builder, wrapped)?;
        Self::require_i64(builder, count, variant.roles.len() as i64);
        if let Some(expected_label) = variant.label {
            let label = self.emit_carrier_field(
                builder, wrapped, variant.role_index(RecursiveCarrierRole::Label)?,
            )?;
            let ordinal = Self::emit_carrier_label_ordinal(builder, label);
            Self::require_i64(builder, ordinal, i64::from(expected_label));
        }
        Ok(variant)
    }

    pub(super) fn decode_recursive_residual(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        residual: CarriedResidualWord,
        disposition: &RecursiveResidualDisposition,
    ) -> Result<(CarriedBoundaryWord, Vec<LoweringOperand>, Vec<LoweringOperand>), CraneliftBackendError> {
        let variant = self.assert_recursive_carrier_variant(builder, residual, disposition)?;
        let forwarded = self.decode_residual_child(builder, residual)?;
        let wrapped = CarriedBoundaryWord { word: residual.word };
        let seat = variant.roles.iter().find_map(|role| match role {
            RecursiveCarrierRole::WorkerCapture { seat, .. } => Some(*seat),
            _ => None,
        });
        let mut worker = Vec::with_capacity(disposition.worker_captures as usize);
        for ordinal in 0..disposition.worker_captures {
            let seat = seat.ok_or_else(|| unsupported(
                "RecursiveResidual", "a residual worker role has no issued capture seat",
            ))?;
            let index = variant.role_index(RecursiveCarrierRole::WorkerCapture { seat, ordinal })?;
            worker.push(LoweringOperand::Carried(self.emit_carrier_field(builder, wrapped, index)?));
        }
        let mut context = Vec::with_capacity(disposition.context_captures as usize);
        for ordinal in 0..disposition.context_captures {
            let index = variant.role_index(RecursiveCarrierRole::ContinuationInput { ordinal })?;
            context.push(LoweringOperand::Carried(self.emit_carrier_field(builder, wrapped, index)?));
        }
        Ok((forwarded, worker, context))
    }

    /// The only ordinary-value exit. The producer supplies a slot key, and
    /// its label selects a member of that slot's planner-issued flow.
    pub(super) fn decode_residual_child(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        residual: CarriedResidualWord,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let key = residual.slot();
        let slot = self.static_transition_plan.recursive_carrier_slot(
            key.eliminator, key.constructor, key.position,
        )?.ok_or_else(|| unsupported("RecursiveResidual", "residual names no issued slot"))?.clone();
        let wrapped = CarriedBoundaryWord { word: residual.word };
        let class = self.emit_carrier_class(builder, wrapped)?;
        Self::require_i64(builder, class, BoundaryClass::Record as i64);
        let tag = self.emit_carrier_tag(builder, wrapped)?;
        Self::require_i64(builder, tag, 1);
        let labelled = slot.flow.len() > 1;
        let ordinal = if labelled {
            let first = *slot.flow.first().ok_or_else(|| unsupported(
                "RecursiveResidual", "a residual slot has no flow",
            ))?;
            let label_index = slot.variant(first)?.role_index(RecursiveCarrierRole::Label)?;
            let label = self.emit_carrier_field(builder, wrapped, label_index)?;
            Some(Self::emit_carrier_label_ordinal(builder, label))
        } else {
            None
        };
        let join = labelled.then(|| builder.create_block());
        let joined_child = join.map(|block| match append_carried_block_param(
            builder, block, Representation::Value,
        ) {
            CarriedBlockParam::Value(word) => word,
            CarriedBlockParam::Residual(_) => unreachable!("Child joins in the value plane"),
        });
        for (index, specialization) in slot.flow.iter().copied().enumerate() {
            let variant = slot.variant(specialization)?;
            let next = if let Some(ordinal) = ordinal {
                let selected = builder.create_block();
                let next = builder.create_block();
                let chosen = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    ordinal,
                    i64::try_from(index).map_err(|_| unsupported(
                        "RecursiveResidual", "a residual variant index exceeds its label ABI",
                    ))?,
                );
                builder.ins().brif(chosen, selected, &[], next, &[]);
                builder.switch_to_block(selected);
                Some(next)
            } else {
                None
            };
            let child = self.decode_residual_variant_child(builder, residual, variant)?;
            if let (Some(join), Some(next)) = (join, next) {
                builder.ins().jump(join, &[child.word.into()]);
                builder.switch_to_block(next);
            } else {
                return Ok(child);
            }
        }
        let impossible = builder.ins().iconst(types::I64, 0);
        Self::require_i64(builder, impossible, 1);
        // `require_i64` leaves a syntactic success block even for a constant
        // false comparison. Seal that unreachable block before the value join.
        let refused = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[refused]);
        let join = join.ok_or_else(|| unsupported(
            "RecursiveResidual", "a singleton residual has no issued member",
        ))?;
        builder.switch_to_block(join);
        Ok(joined_child.expect("labelled slot has a value parameter"))
    }

    fn decode_residual_variant_child(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        residual: CarriedResidualWord,
        variant: &RecursiveCarrierVariant,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let schema = self.static_transition_plan.aggregate_record_view(variant.record)?;
        if schema.shape() != PlannedAggregateShape::Record {
            return Err(unsupported("RecursiveResidual", "the issued variant is not a Record"));
        }
        let wrapped = CarriedBoundaryWord { word: residual.word };
        let count = self.emit_carrier_field_count(builder, wrapped)?;
        Self::require_i64(builder, count, variant.roles.len() as i64);
        if let Some(expected_label) = variant.label {
            let label = self.emit_carrier_field(
                builder, wrapped, variant.role_index(RecursiveCarrierRole::Label)?,
            )?;
            let ordinal = Self::emit_carrier_label_ordinal(builder, label);
            Self::require_i64(builder, ordinal, i64::from(expected_label));
        }
        let child = self.emit_carrier_field(
            builder, wrapped, variant.role_index(RecursiveCarrierRole::Child)?,
        )?;
        let class = self.emit_carrier_class(builder, child)?;
        Self::require_i64(builder, class, BoundaryClass::Constructor as i64);
        let worker_captures = variant.roles.iter().filter(|role|
            matches!(role, RecursiveCarrierRole::WorkerCapture { .. })).count();
        let child_count = self.emit_carrier_field_count(builder, child)?;
        Self::require_i64(builder, child_count, worker_captures as i64);
        Ok(child)
    }
}
