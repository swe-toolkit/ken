//! Primitive and value lowering.
//!
//! This child owns primitive argument evaluation, partiality, carried
//! projection, representation checks, symbol dispatch, and emission.

use crate::cranelift_backend::lowering::aggregates::CarrierAllocationRequest;
use crate::cranelift_backend::planning::{
    PlannedAggregateShape, SynthesizedAggregateNode, SynthesizedAggregatePath,
    SynthesizedAggregateRoot, SynthesizedConstructorRole, SynthesizedFixedConstructorRole as Role,
};
use crate::cranelift_backend::surface::LoweringCause;
use super::*;

#[derive(Clone, Copy)]
struct BytesListConstructors {
    nil: (AggregateOccurrenceId, u64),
    cons: (AggregateOccurrenceId, u64),
}
use unicode_normalization::UnicodeNormalization;

#[cfg(test)]
mod tests;

fn lowered_char_list(value: &Lowered) -> Option<Vec<char>> {
    let Lowered::Constructor {
        constructor, args, ..
    } = value
    else {
        return None;
    };
    if constructor.ends_with("::Nil") && args.is_empty() {
        return Some(Vec::new());
    }
    if !constructor.ends_with("::Cons") || args.len() != 2 {
        return None;
    }
    // A worker in either field means this is not a statically known char list.
    // `None` is the conservative answer here for the same reason as in
    // `resource_open_mode_tag`: the caller falls back to the general path
    // rather than acting on a decoded literal.
    let Lowered::Int {
        known: Some(head), ..
    } = args[0].specialized_at("a char list head field").ok()?
    else {
        return None;
    };
    let head = u32::try_from(*head).ok().and_then(char::from_u32)?;
    let mut tail = lowered_char_list(args[1].specialized_at("a char list tail field").ok()?)?;
    tail.insert(0, head);
    Some(tail)
}

impl<'a> Lowering<'a> {
    /// `static_origin` is the `PrimitiveCall` occurrence's own origin; argument
    /// *i* is child *i* (a primitive symbol is an atom, not a child).
    pub(super) fn lower_primitive_call(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        primitive: &RuntimePrimitive,
        args: &[RuntimeExpr],
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let lowered_args = args
            .iter()
            .enumerate()
            .map(|(position, arg)| {
                let arg = self.child_occurrence(static_origin, position, arg)?;
                self.lower_expr(builder, arg, env)
            })
            .collect::<Result<Vec<_>, _>>()?;
        if lowered_args.iter().any(|arg| match arg {
            LoweringOperand::Specialized(Lowered::RecursiveBackedge) => true,
            LoweringOperand::Residual(_) => false,
            LoweringOperand::Specialized(_) | LoweringOperand::Carried(_) => false,
        }) {
            return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
        }

        match &primitive.partiality {
            RuntimePartiality::Total => {}
            RuntimePartiality::SafeOption { .. } | RuntimePartiality::SafeResult { .. } => {}
            RuntimePartiality::CheckedTrap { obligation } => {
                self.assumptions.insert(format!(
                    "checked partial obligation {obligation} not discharged"
                ));
                let trap = crate::cranelift_backend::planning::planned_partiality_trap(primitive)
                    .expect("CheckedTrap has one planner-derived trap");
                return Ok(LoweringOperand::Specialized(Lowered::Trap(trap)));
            }
            RuntimePartiality::TrustedTrap { assumption } => {
                self.assumptions.insert(format!(
                    "trusted partial assumption {assumption} remains visible"
                ));
                let trap = crate::cranelift_backend::planning::planned_partiality_trap(primitive)
                    .expect("TrustedTrap has one planner-derived trap");
                return Ok(LoweringOperand::Specialized(Lowered::Trap(trap)));
            }
        }

        if primitive.symbol == "bytes_to_list" {
            let [operand] = lowered_args.as_slice() else {
                return Err(unsupported(
                    LoweringCause::PlannerInvariant,
                    "PrimitiveCall",
                    "bytes_to_list requires exactly one bytes operand",
                ));
            };
            return self.lower_bytes_to_list(builder, static_origin, operand)
                .map(LoweringOperand::Carried);
        }

        // Two compile-time literals still fold in the compiler. Any runtime operand
        // takes the emitted route, whose result is a fresh region Bytes node.
        if primitive.symbol == "bytes_concat"
            && !matches!(
                lowered_args.as_slice(),
                [
                    LoweringOperand::Specialized(Lowered::Bytes(_)),
                    LoweringOperand::Specialized(Lowered::Bytes(_)),
                ]
            )
        {
            return self
                .lower_dynamic_bytes_concat(builder, &lowered_args)
                .map(LoweringOperand::Carried);
        }

        // A primitive's static symbol determines whether its operands are
        // scalar Ints or Bools. A carried word in one of those positions is
        // projected through the emitted scalar helper; no runtime tag chooses
        // which source type the operand is.
        let scalar_kind = match primitive.symbol.as_str() {
            "add_int" | "sub_int" | "mul_int" | "eq_int" | "leq_int" | "uint8_to_int"
            | "int_to_uint8_raw" => Some("Int"),
            "not_bool" | "and_bool" | "or_bool" => Some("Bool"),
            _ => None,
        };
        let lowered_args = if primitive.symbol == "bytes_length" {
            match lowered_args.as_slice() {
                [LoweringOperand::Specialized(_)] => {
                    specialized_operands_at(&lowered_args, "the bytes_length operand")?
                }
                [LoweringOperand::Residual(_)] => return Err(
                    CraneliftBackendError::ResidualRepresentationRequired {
                        site: "bytes_length requires an ordinary bytes value",
                    },
                ),
                [LoweringOperand::Carried(word)] => {
                    vec![self.lower_carried_bytes_operand(builder, *word)?]
                }
                _ => {
                    return Err(unsupported(LoweringCause::Fragment,
                        "PrimitiveCall",
                        "bytes_length requires exactly one bytes operand",
                    ));
                }
            }
        } else if primitive.symbol == "bytes_at" {
            lowered_args
                .into_iter()
                .enumerate()
                .map(|(position, arg)| match (position, arg) {
                    (_, LoweringOperand::Specialized(value)) => Ok(value),
                    (0, LoweringOperand::Carried(word)) => {
                        self.lower_carried_bytes_operand(builder, word)
                    }
                    (1, LoweringOperand::Carried(word)) => {
                        let tag = builder
                            .ins()
                            .band_imm(word.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
                        Self::require_i64(
                            builder,
                            tag,
                            crate::boundary_value::BoundaryTag::ImmediateInt as i64,
                        );
                        let value = self.emit_carrier_scalar(builder, word)?;
                        Ok(self.lower_dynamic_small_int(builder, value))
                    }
                    (_, LoweringOperand::Carried(_)) => Err(unsupported(LoweringCause::Fragment,
                        "PrimitiveCall",
                        "bytes_at received more operands than its closed static signature",
                    )),
                    (_, LoweringOperand::Residual(_)) => Err(
                        CraneliftBackendError::ResidualRepresentationRequired {
                            site: "bytes_at requires an ordinary bytes or index value",
                        },
                    ),
                })
                .collect::<Result<Vec<_>, CraneliftBackendError>>()?
        } else if let Some(kind) = scalar_kind {
            lowered_args
                .into_iter()
                .map(|arg| match arg {
                    LoweringOperand::Specialized(Lowered::BorrowedNativeValue { .. }) => {
                        // Borrowed ingress supplies only Bytes and constructors. A
                        // process-family match can emit a borrowed leaf even for a
                        // list whose checked element type cannot come from ingress.
                        // Fail closed in that leaf rather than rejecting the unit.
                        let failure = builder.ins().iconst(types::I64, -1);
                        builder.ins().return_(&[failure]);
                        self.record_checked_frame_terminal(builder, FrameTerminalKind::Abort)?;
                        let unreachable = builder.create_block();
                        builder.switch_to_block(unreachable);
                        let zero = builder.ins().iconst(types::I64, 0);
                        Ok(match kind {
                            "Int" => self.lower_dynamic_small_int(builder, zero),
                            "Bool" => Lowered::Bool { value: zero, known: None },
                            _ => unreachable!("closed primitive scalar kind"),
                        })
                    }
                    LoweringOperand::Specialized(value) => Ok(value),
                    LoweringOperand::Carried(word) => {
                        Ok(match kind {
                            "Int" => self.decode_carried_int_operand(builder, word)?,
                            "Bool" => {
                                let value = self.emit_carrier_scalar(builder, word)?;
                                let tag = builder.ins().band_imm(
                                    word.word,
                                    crate::boundary_value::BOUNDARY_TAG_MASK as i64,
                                );
                                Self::require_i64(
                                    builder,
                                    tag,
                                    crate::boundary_value::BoundaryTag::ImmediateBool as i64,
                                );
                                Lowered::Bool { value, known: None }
                            }
                            _ => unreachable!("closed primitive scalar kind"),
                        })
                    }
                    LoweringOperand::Residual(_) => Err(
                        CraneliftBackendError::ResidualRepresentationRequired {
                            site: "a scalar primitive requires an ordinary value",
                        },
                    ),
                })
                .collect::<Result<Vec<_>, CraneliftBackendError>>()?
        } else {
            specialized_operands_at(&lowered_args, "a primitive-call operand")?
        };
        let lowered = match primitive.symbol.as_str() {
            "add_int" => self.lower_int_binop(builder, "add_int", lowered_args, |lhs, rhs| {
                lhs.checked_add(rhs)
            }),
            "sub_int" => self.lower_int_binop(builder, "sub_int", lowered_args, |lhs, rhs| {
                lhs.checked_sub(rhs)
            }),
            "mul_int" => self.lower_int_binop(builder, "mul_int", lowered_args, |lhs, rhs| {
                lhs.checked_mul(rhs)
            }),
            "eq_int" => self.lower_int_cmp(
                builder,
                "eq_int",
                lowered_args,
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                |lhs, rhs| lhs == rhs,
            ),
            "leq_int" => self.lower_int_cmp(
                builder,
                "leq_int",
                lowered_args,
                cranelift_codegen::ir::condcodes::IntCC::SignedLessThanOrEqual,
                |lhs, rhs| lhs <= rhs,
            ),
            "uint8_to_int" | "int_to_uint8_raw" | "int_to_uint64_raw" => {
                let [value]: [Lowered; 1] = lowered_args.try_into().map_err(|args: Vec<_>| {
                    unsupported(LoweringCause::PlannerInvariant,
                        "PrimitiveCall",
                        format!(
                            "{} expects one argument, got {}",
                            primitive.symbol,
                            args.len()
                        ),
                    )
                })?;
                let Lowered::Int { .. } = value else {
                    return Err(unsupported(LoweringCause::Fragment,
                        "PrimitiveCall",
                        format!("{} expects an Int-represented value", primitive.symbol),
                    ));
                };
                Ok(value)
            }
            "not_bool" => self.lower_bool_not(builder, lowered_args),
            "and_bool" => self.lower_bool_binop(
                builder,
                "and_bool",
                lowered_args,
                |builder, lhs, rhs| builder.ins().band(lhs, rhs),
                |lhs, rhs| lhs && rhs,
            ),
            "or_bool" => self.lower_bool_binop(
                builder,
                "or_bool",
                lowered_args,
                |builder, lhs, rhs| builder.ins().bor(lhs, rhs),
                |lhs, rhs| lhs || rhs,
            ),
            "bytes_length" => self.lower_bytes_length(builder, lowered_args),
            "bytes_at" => self.lower_bytes_at(builder, lowered_args, &primitive.partiality),
            "bytes_slice" => self.lower_bytes_slice(lowered_args, &primitive.partiality),
            "bytes_concat" => self.lower_bytes_concat(lowered_args),
            "bytes_encode" => self.lower_bytes_encode(lowered_args),
            "bytes_decode" => self.lower_bytes_decode(lowered_args, &primitive.partiality),
            "list_char_to_string" => {
                let [value]: [Lowered; 1] = lowered_args.try_into().map_err(|args: Vec<_>| {
                    unsupported(LoweringCause::PlannerInvariant,
                        "PrimitiveCall",
                        format!(
                            "list_char_to_string expects one argument, got {}",
                            args.len()
                        ),
                    )
                })?;
                let chars = lowered_char_list(&value).ok_or_else(|| {
                    unsupported(LoweringCause::Fragment,
                        "PrimitiveCall",
                        "list_char_to_string requires a closed List Char of Unicode scalars",
                    )
                })?;
                Ok(Lowered::String(chars.into_iter().nfc().collect()))
            }
            "byte_length" => self.lower_string_byte_length(builder, lowered_args),
            "char_length" => self.lower_string_char_length(builder, lowered_args),
            other => Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                format!("primitive {other} is not in the supported native set"),
            )),
        };
        // ⭐ Back onto the spine: a primitive's result is a fresh specialized
        // value re-entering the phase sum.
        lowered.map(LoweringOperand::Specialized)
    }

    fn lower_int_binop(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        symbol: &'static str,
        args: Vec<Lowered>,
        eval: impl FnOnce(i64, i64) -> Option<i64>,
    ) -> Result<Lowered, CraneliftBackendError> {
        let (lhs, rhs) = expect_two_args(symbol, args)?;
        let (
            Lowered::Int {
                value: lhs,
                known: lhs_known,
            },
            Lowered::Int {
                value: rhs,
                known: rhs_known,
            },
        ) = (lhs, rhs)
        else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                format!("{symbol} only supports Int arguments in native lowering"),
            ));
        };
        #[cfg(test)]
        match self.native_int_mutation {
            NativeIntLoweringMutation::Exact => {}
            NativeIntLoweringMutation::Wrapping => {}
            NativeIntLoweringMutation::Trap => {
                return Err(unsupported(LoweringCause::PlannerInvariant,
                    "PrimitiveCall",
                    "PX8-I mutation traps before exact Int support",
                ));
            }
            NativeIntLoweringMutation::SuppressTerminalExport
            | NativeIntLoweringMutation::CorruptTerminalExport => {}
        }
        let lhs_tag = self.native_int_tag(builder, lhs, lhs_known)?;
        let rhs_tag = self.native_int_tag(builder, rhs, rhs_known)?;
        let arena = self.function_local.native_int_arena.ok_or_else(|| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                "exact Int operation has no invocation arena",
            )
        })?;
        let helper = self.function_local.native_int_binop.ok_or_else(|| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                "exact Int operation has no local support function",
            )
        })?;
        let output =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
        let pointer_type = builder.func.dfg.value_type(arena);
        let output_pointer = builder.ins().stack_addr(pointer_type, output, 0);
        let operation = builder.ins().iconst(
            types::I64,
            match symbol {
                "add_int" => 0,
                "sub_int" => 1,
                "mul_int" => 2,
                _ => unreachable!("caller supplies exact Int arithmetic symbol"),
            },
        );
        let call = builder.ins().call(
            helper,
            &[arena, operation, lhs_tag, lhs, rhs_tag, rhs, output_pointer],
        );
        let status = builder.inst_results(call)[0];
        Self::require_i64(builder, status, 0);
        let tag = builder.ins().stack_load(types::I64, output, 0);
        let value = builder.ins().stack_load(types::I64, output, 8);
        Self::require_one_of_i64(
            builder,
            tag,
            &[
                crate::NATIVE_INT_SMALL_TAG_V1 as i64,
                crate::NATIVE_INT_BIG_TAG_V1 as i64,
            ],
        );
        self.function_local.native_int_tags.insert(value, tag);
        let known = lhs_known.and_then(|lhs| rhs_known.and_then(|rhs| eval(lhs, rhs)));
        Ok(Lowered::Int { value, known })
    }

    fn lower_int_cmp(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        symbol: &'static str,
        args: Vec<Lowered>,
        _cc: cranelift_codegen::ir::condcodes::IntCC,
        eval: impl FnOnce(i64, i64) -> bool,
    ) -> Result<Lowered, CraneliftBackendError> {
        let (lhs, rhs) = expect_two_args(symbol, args)?;
        let (
            Lowered::Int {
                value: lhs,
                known: lhs_known,
            },
            Lowered::Int {
                value: rhs,
                known: rhs_known,
            },
        ) = (lhs, rhs)
        else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                format!("{symbol} only supports Int arguments in native lowering"),
            ));
        };
        let lhs_tag = self.native_int_tag(builder, lhs, lhs_known)?;
        let rhs_tag = self.native_int_tag(builder, rhs, rhs_known)?;
        let arena = self.function_local.native_int_arena.ok_or_else(|| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                "exact Int comparison has no invocation arena",
            )
        })?;
        let helper = self.function_local.native_int_compare.ok_or_else(|| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                "exact Int comparison has no local support function",
            )
        })?;
        let operation = builder.ins().iconst(
            types::I64,
            match symbol {
                "eq_int" => 0,
                "leq_int" => 1,
                _ => unreachable!("caller supplies exact Int comparison symbol"),
            },
        );
        let call = builder
            .ins()
            .call(helper, &[arena, operation, lhs_tag, lhs, rhs_tag, rhs]);
        let value = builder.inst_results(call)[0];
        Self::require_one_of_i64(builder, value, &[0, 1]);
        Ok(Lowered::Bool {
            value,
            known: lhs_known.and_then(|lhs| rhs_known.map(|rhs| eval(lhs, rhs))),
        })
    }

    fn lower_bool_not(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        args: Vec<Lowered>,
    ) -> Result<Lowered, CraneliftBackendError> {
        let [arg]: [Lowered; 1] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("not_bool expects 1 arg, got {}", args.len()),
            )
        })?;
        let Lowered::Bool { value, known } = arg else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "not_bool only supports Bool arguments in native lowering",
            ));
        };
        let one = builder.ins().iconst(types::I64, 1);
        Ok(Lowered::Bool {
            value: builder.ins().bxor(value, one),
            known: known.map(|value| !value),
        })
    }

    fn lower_bool_binop(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        symbol: &'static str,
        args: Vec<Lowered>,
        emit: impl FnOnce(
            &mut FunctionBuilder<'_>,
            cranelift_codegen::ir::Value,
            cranelift_codegen::ir::Value,
        ) -> cranelift_codegen::ir::Value,
        eval: impl FnOnce(bool, bool) -> bool,
    ) -> Result<Lowered, CraneliftBackendError> {
        let (lhs, rhs) = expect_two_args(symbol, args)?;
        let (
            Lowered::Bool {
                value: lhs,
                known: lhs_known,
            },
            Lowered::Bool {
                value: rhs,
                known: rhs_known,
            },
        ) = (lhs, rhs)
        else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                format!("{symbol} only supports Bool arguments in native lowering"),
            ));
        };
        Ok(Lowered::Bool {
            value: emit(builder, lhs, rhs),
            known: lhs_known.and_then(|lhs| rhs_known.map(|rhs| eval(lhs, rhs))),
        })
    }

    /// Turn either lawful carried Bytes class into a span consumed by this primitive.
    /// An invocation-borrowed native value has class BorrowedOpaque; a response
    /// payload copied into the region across a generated-unit frame has class
    /// Bytes. Keep the persistent view within the primitive: its address is
    /// stable only until the next materialization.
    fn lower_carried_bytes_operand(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        word: CarriedBoundaryWord,
    ) -> Result<Lowered, CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let class = self.emit_carrier_class(builder, word)?;
        let borrowed = builder.create_block();
        let persistent = builder.create_block();
        let done = builder.create_block();
        builder.append_block_param(done, pointer_type);
        builder.append_block_param(done, types::I64);
        let is_borrowed =
            builder
                .ins()
                .icmp_imm(IntCC::Equal, class, BoundaryClass::BorrowedOpaque as i64);
        builder
            .ins()
            .brif(is_borrowed, borrowed, &[], persistent, &[]);

        builder.switch_to_block(borrowed);
        let pointer = self.emit_carrier_scalar(builder, word)?;
        let (data, len) = Self::borrowed_bytes_view(builder, pointer);
        builder.ins().jump(done, &[data.into(), len.into()]);

        builder.switch_to_block(persistent);
        Self::require_i64(builder, class, BoundaryClass::Bytes as i64);
        let slot =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
        let view = builder.ins().stack_addr(pointer_type, slot, 0);
        let call = builder
            .ins()
            .call(refs.bytes_view, &[arena, word.word, view]);
        let status = builder.inst_results(call)[0];
        Self::require_i64(builder, status, BOUNDARY_OK);
        let data = builder.ins().stack_load(pointer_type, slot, 0);
        let len = builder.ins().stack_load(types::I64, slot, 8);
        builder.ins().jump(done, &[data.into(), len.into()]);

        builder.switch_to_block(done);
        let data = builder.block_params(done)[0];
        let len = builder.block_params(done)[1];
        let valid = builder.ins().iconst(types::I8, 1);
        Ok(Lowered::ResponseBytes(SafeByteSpan::masked_at_producer(
            builder,
            pointer_type,
            data,
            len,
            valid,
        )))
    }

    fn borrowed_bytes_view(
        builder: &mut FunctionBuilder<'_>,
        pointer: cranelift_codegen::ir::Value,
    ) -> (cranelift_codegen::ir::Value, cranelift_codegen::ir::Value) {
        use cranelift_codegen::ir::condcodes::IntCC;
        let pointer_type = builder.func.dfg.value_type(pointer);
        let kind = builder
            .ins()
            .load(types::I64, MemFlags::trusted(), pointer, 0);
        Self::require_i64(builder, kind, 1);
        let data = builder
            .ins()
            .load(pointer_type, MemFlags::trusted(), pointer, 16);
        let len = builder
            .ins()
            .load(types::I64, MemFlags::trusted(), pointer, 24);
        // Preserve the borrowed arm's non-null guard before reading a byte.
        let null = builder.ins().icmp_imm(IntCC::Equal, data, 0);
        let nonempty = builder.ins().icmp_imm(IntCC::NotEqual, len, 0);
        let broken = builder.ins().band(null, nonempty);
        let broken = builder.ins().uextend(types::I64, broken);
        Self::require_i64(builder, broken, 0);
        (data, len)
    }

    fn bytes_list_constructors(
        &self,
        seat: StaticOriginId,
    ) -> Result<BytesListConstructors, CraneliftBackendError> {
        let owner = self.defining_emission_owner.ok_or_else(|| {
            unsupported(LoweringCause::PlannerInvariant, "PrimitiveCall",
                "bytes_to_list is not in a declared emission owner")
        })?;
        let root = SynthesizedAggregatePath::root(SynthesizedAggregateRoot::PrimitiveResult);
        let nil = SynthesizedConstructorRole::Fixed(Role::ListNil);
        let cons = SynthesizedConstructorRole::Fixed(Role::ListCons);
        let plan = &self.static_transition_plan;
        let alternatives = plan.synthesized_dynamic_alternatives(seat, &root)?;
        let nil_children = plan.synthesized_aggregate_children(owner, seat, &root.alternative(0), nil)?;
        let cons_children = plan.synthesized_aggregate_children(owner, seat, &root.alternative(1), cons)?;
        if alternatives != [nil, cons]
            || !nil_children.is_empty()
            || cons_children != [
                SynthesizedAggregateNode::native_int(),
                SynthesizedAggregateNode::RepeatedAggregateLink { class: BoundaryClass::Constructor },
            ]
        {
            return Err(unsupported(LoweringCause::PlannerInvariant, "PrimitiveCall",
                "bytes_to_list requires planned Nil and Cons(Int, constructor link)"));
        }
        Ok(BytesListConstructors {
            nil: (
                plan.synthesized_aggregate_occurrence(owner, seat, &root.alternative(0), nil)?,
                self.synthesized_fixed_identity(Role::ListNil)?.tag_abi_word()?,
            ),
            cons: (
                plan.synthesized_aggregate_occurrence(owner, seat, &root.alternative(1), cons)?,
                self.synthesized_fixed_identity(Role::ListCons)?.tag_abi_word()?,
            ),
        })
    }

    fn emit_bytes_list_node(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        (occurrence, identity): (AggregateOccurrenceId, u64),
        children: &[CarriedBoundaryWord],
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let node = self.emit_carrier_alloc(
            builder,
            CarrierAllocationRequest::PlannedAggregate {
                occurrence,
                shape: PlannedAggregateShape::Constructor,
            },
            BoundaryClass::Constructor,
            children.len(),
        )?;
        self.emit_carrier_store_tag_id(builder, node, identity)?;
        for (position, child) in children.iter().enumerate() {
            self.emit_carrier_store_field(builder, node, position, *child)?;
        }
        Ok(node)
    }

    fn prepend_bytes_list_byte(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: StaticOriginId,
        constructors: BytesListConstructors,
        byte: cranelift_codegen::ir::Value,
        tail: CarriedBoundaryWord,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let byte = builder.ins().uextend(types::I64, byte);
        let byte = self.lower_dynamic_small_int(builder, byte);
        let head = self.emit_carrier_transfer(builder, seat, &byte)?;
        self.emit_bytes_list_node(builder, constructors.cons, &[head, tail])
    }

    fn lower_bytes_to_list(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: StaticOriginId,
        operand: &LoweringOperand,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let constructors = self.bytes_list_constructors(seat)?;
        match operand {
            LoweringOperand::Specialized(Lowered::ResponseBytes(span)) => {
                self.build_bytes_list_loop(builder, seat, constructors, span.len(), |_, _| Ok(span.pointer()))
            }
            LoweringOperand::Specialized(Lowered::BorrowedNativeValue { pointer }) => {
                let (data, len) = Self::borrowed_bytes_view(builder, *pointer);
                self.build_bytes_list_loop(builder, seat, constructors, len, |_, _| Ok(data))
            }
            LoweringOperand::Specialized(Lowered::Bytes(bytes)) => {
                let mut tail = self.emit_bytes_list_node(builder, constructors.nil, &[])?;
                for byte in bytes.iter().rev() {
                    let byte = builder.ins().iconst(types::I8, i64::from(*byte));
                    tail = self.prepend_bytes_list_byte(builder, seat, constructors, byte, tail)?;
                }
                Ok(tail)
            }
            LoweringOperand::Carried(word) => {
                let class = self.emit_carrier_class(builder, *word)?;
                let borrowed = builder.create_block();
                let persistent = builder.create_block();
                let done = builder.create_block();
                builder.append_block_param(done, types::I64);
                let is_borrowed = builder.ins().icmp_imm(IntCC::Equal, class, BoundaryClass::BorrowedOpaque as i64);
                builder.ins().brif(is_borrowed, borrowed, &[], persistent, &[]);

                builder.switch_to_block(borrowed);
                let pointer = self.emit_carrier_scalar(builder, *word)?;
                let (data, len) = Self::borrowed_bytes_view(builder, pointer);
                let list = self.build_bytes_list_loop(builder, seat, constructors, len, |_, _| Ok(data))?;
                builder.ins().jump(done, &[list.word.into()]);

                builder.switch_to_block(persistent);
                Self::require_i64(builder, class, BoundaryClass::Bytes as i64);
                let arena = self.carrier_arena()?;
                let pointer_type = builder.func.dfg.value_type(arena);
                let slot = builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
                let out = builder.ins().stack_addr(pointer_type, slot, 0);
                self.emit_bytes_list_view(builder, *word, out)?;
                let len = builder.ins().stack_load(types::I64, slot, 8);
                let list = self.build_bytes_list_loop(builder, seat, constructors, len, |lowering, builder| {
                    // Only the length survives the previous view. Materializing a
                    // Cons can invalidate the data pointer, so read it afresh.
                    lowering.emit_bytes_list_view(builder, *word, out)?;
                    Ok(builder.ins().stack_load(pointer_type, slot, 0))
                })?;
                builder.ins().jump(done, &[list.word.into()]);

                builder.switch_to_block(done);
                Ok(CarriedBoundaryWord { word: builder.block_params(done)[0] })
            }
            LoweringOperand::Residual(_) => Err(CraneliftBackendError::ResidualRepresentationRequired {
                site: "bytes_to_list requires an ordinary bytes value",
            }),
            LoweringOperand::Specialized(_) => Err(unsupported(
                LoweringCause::Fragment, "PrimitiveCall",
                "bytes_to_list only supports Bytes arguments in native lowering",
            )),
        }
    }

    fn emit_bytes_list_view(
        &self,
        builder: &mut FunctionBuilder<'_>,
        word: CarriedBoundaryWord,
        out: cranelift_codegen::ir::Value,
    ) -> Result<(), CraneliftBackendError> {
        let call = builder.ins().call(self.carrier_refs()?.bytes_view, &[self.carrier_arena()?, word.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(())
    }

    fn build_bytes_list_loop(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: StaticOriginId,
        constructors: BytesListConstructors,
        len: cranelift_codegen::ir::Value,
        data_for_byte: impl FnOnce(&mut Self, &mut FunctionBuilder<'_>) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError>,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let nil = self.emit_bytes_list_node(builder, constructors.nil, &[])?;
        let test = builder.create_block();
        let body = builder.create_block();
        let done = builder.create_block();
        builder.append_block_param(test, types::I64);
        builder.append_block_param(test, types::I64);
        builder.append_block_param(done, types::I64);
        builder.ins().jump(test, &[len.into(), nil.word.into()]);
        builder.switch_to_block(test);
        let remaining = builder.block_params(test)[0];
        let tail = CarriedBoundaryWord { word: builder.block_params(test)[1] };
        builder.ins().brif(remaining, body, &[], done, &[tail.word.into()]);
        builder.switch_to_block(body);
        let index = builder.ins().iadd_imm(remaining, -1);
        let data = data_for_byte(self, builder)?;
        let address = builder.ins().iadd(data, index);
        let byte = builder.ins().load(types::I8, MemFlags::trusted(), address, 0);
        let list = self.prepend_bytes_list_byte(builder, seat, constructors, byte, tail)?;
        builder.ins().jump(test, &[index.into(), list.word.into()]);
        builder.switch_to_block(done);
        Ok(CarriedBoundaryWord { word: builder.block_params(done)[0] })
    }

    fn lower_bytes_length(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        args: Vec<Lowered>,
    ) -> Result<Lowered, CraneliftBackendError> {
        let [arg]: [Lowered; 1] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("bytes_length expects 1 arg, got {}", args.len()),
            )
        })?;
        if let Lowered::ResponseBytes(span) = arg {
            return self.lower_unsigned_u64_int(builder, span.len());
        }
        if let Lowered::BorrowedNativeValue { pointer } = arg {
            let kind = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 0);
            Self::require_i64(builder, kind, 1);
            let len = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 24);
            return self.lower_unsigned_u64_int(builder, len);
        }
        let Lowered::Bytes(bytes) = arg else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_length only supports Bytes arguments in native lowering",
            ));
        };
        let len = i64::try_from(bytes.len()).map_err(|_| {
            unsupported(LoweringCause::Capacity,
                "PrimitiveCall",
                "bytes_length result does not fit the runtime Int representation",
            )
        })?;
        Ok(Lowered::Int {
            value: builder.ins().iconst(types::I64, len),
            known: Some(len),
        })
    }

    fn lower_bytes_at(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        args: Vec<Lowered>,
        partiality: &RuntimePartiality,
    ) -> Result<Lowered, CraneliftBackendError> {
        let RuntimePartiality::SafeOption { none, some, .. } = partiality else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_at requires safe Option result metadata",
            ));
        };
        let (bytes, index) = expect_two_args("bytes_at", args)?;
        let Lowered::Int {
            known: Some(index), ..
        } = index
        else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_at requires a statically known Int index",
            ));
        };
        if let Lowered::ResponseBytes(span) = bytes {
            let (data, len) = (span.pointer(), span.len());
            let index_value = builder.ins().iconst(types::I64, index);
            let present = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThan,
                index_value,
                len,
            );
            let in_bounds = builder.create_block();
            let out_of_bounds = builder.create_block();
            let merge = builder.create_block();
            builder.append_block_param(merge, types::I64);
            builder.append_block_param(merge, types::I64);
            builder
                .ins()
                .brif(present, in_bounds, &[], out_of_bounds, &[]);
            builder.switch_to_block(in_bounds);
            let address = builder.ins().iadd_imm(data, index);
            let byte = builder
                .ins()
                .load(types::I8, MemFlags::trusted(), address, 0);
            let yes = builder.ins().iconst(types::I64, 1);
            let byte = builder.ins().uextend(types::I64, byte);
            builder.ins().jump(merge, &[yes.into(), byte.into()]);
            builder.switch_to_block(out_of_bounds);
            let no = builder.ins().iconst(types::I64, 0);
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().jump(merge, &[no.into(), zero.into()]);
            builder.switch_to_block(merge);
            let value = builder.block_params(merge)[1];
            let tag = builder
                .ins()
                .iconst(types::I64, crate::NATIVE_INT_SMALL_TAG_V1 as i64);
            self.function_local.native_int_tags.insert(value, tag);
            return Ok(Lowered::BorrowedOption {
                present: builder.block_params(merge)[0],
                value,
                none: none.clone(),
                some: some.clone(),
            });
        }
        if let Lowered::BorrowedNativeValue { pointer } = bytes {
            let kind = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 0);
            Self::require_i64(builder, kind, 1);
            let pointer_type = builder.func.dfg.value_type(pointer);
            let data = builder
                .ins()
                .load(pointer_type, MemFlags::trusted(), pointer, 16);
            let len = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 24);
            let index_value = builder.ins().iconst(types::I64, index);
            let present = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThan,
                index_value,
                len,
            );
            let in_bounds = builder.create_block();
            let out_of_bounds = builder.create_block();
            let merge = builder.create_block();
            builder.append_block_param(merge, types::I64);
            builder
                .ins()
                .brif(present, in_bounds, &[], out_of_bounds, &[]);
            builder.switch_to_block(in_bounds);
            Self::require_nonzero(builder, data);
            let address = builder.ins().iadd_imm(data, index);
            let byte = builder
                .ins()
                .load(types::I8, MemFlags::trusted(), address, 0);
            let byte = builder.ins().uextend(types::I64, byte);
            builder.ins().jump(merge, &[byte.into()]);
            builder.switch_to_block(out_of_bounds);
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().jump(merge, &[zero.into()]);
            builder.switch_to_block(merge);
            let value = builder.block_params(merge)[0];
            let tag = builder
                .ins()
                .iconst(types::I64, crate::NATIVE_INT_SMALL_TAG_V1 as i64);
            self.function_local.native_int_tags.insert(value, tag);
            return Ok(Lowered::BorrowedOption {
                present,
                value,
                none: none.clone(),
                some: some.clone(),
            });
        }
        let Lowered::Bytes(bytes) = bytes else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_at requires Bytes in native lowering",
            ));
        };
        let byte = usize::try_from(index)
            .ok()
            .and_then(|index| bytes.get(index).copied());
        Ok(match byte {
            Some(byte) => Lowered::Constructor {
                constructor: some.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: vec![ConstructorField::specialized(Lowered::Int {
                    value: builder.ins().iconst(types::I64, i64::from(byte)),
                    known: Some(i64::from(byte)),
                })],
            },
            None => Lowered::Constructor {
                constructor: none.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: Vec::new(),
            },
        })
    }

    fn lower_bytes_slice(
        &mut self,
        args: Vec<Lowered>,
        partiality: &RuntimePartiality,
    ) -> Result<Lowered, CraneliftBackendError> {
        let RuntimePartiality::SafeOption { none, some, .. } = partiality else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_slice requires safe Option result metadata",
            ));
        };
        let [bytes, start, len]: [Lowered; 3] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("bytes_slice expects 3 args, got {}", args.len()),
            )
        })?;
        let (
            Lowered::Bytes(bytes),
            Lowered::Int {
                known: Some(start), ..
            },
            Lowered::Int {
                known: Some(len), ..
            },
        ) = (bytes, start, len)
        else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_slice requires Bytes and statically known Int bounds",
            ));
        };
        let value = usize::try_from(start)
            .ok()
            .zip(usize::try_from(len).ok())
            .and_then(|(start, len)| {
                start
                    .checked_add(len)
                    .filter(|end| *end <= bytes.len())
                    .map(|end| bytes[start..end].to_vec())
            });
        Ok(match value {
            Some(bytes) => Lowered::Constructor {
                constructor: some.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: vec![ConstructorField::specialized(Lowered::Bytes(bytes))],
            },
            None => Lowered::Constructor {
                constructor: none.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: Vec::new(),
            },
        })
    }

    fn lower_bytes_concat(&mut self, args: Vec<Lowered>) -> Result<Lowered, CraneliftBackendError> {
        let (lhs, rhs) = expect_two_args("bytes_concat", args)?;
        let (Lowered::Bytes(mut lhs), Lowered::Bytes(rhs)) = (lhs, rhs) else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_concat only supports Bytes arguments in native lowering",
            ));
        };
        lhs.extend(rhs);
        Ok(Lowered::Bytes(lhs))
    }

    /// `bytes_concat` with a runtime operand. Lengths are read first, the result
    /// is claimed at their sum, and each source is copied at its offset. A
    /// persistent Bytes source is viewed again after the claim, because a byte
    /// view is stable only until the next reservation of its image (AC-11).
    fn lower_dynamic_bytes_concat(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        args: &[LoweringOperand],
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let [lhs, rhs] = args else {
            return Err(unsupported(
                LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                "bytes_concat requires exactly two bytes operands",
            ));
        };
        let lhs_len = self.bytes_concat_operand_len(builder, lhs)?;
        let rhs_len = self.bytes_concat_operand_len(builder, rhs)?;
        let total = builder.ins().iadd(lhs_len, rhs_len);
        let wrapped = builder.ins().icmp(IntCC::UnsignedLessThan, total, lhs_len);
        let wrapped = builder.ins().uextend(types::I64, wrapped);
        Self::require_i64(builder, wrapped, 0);
        let (tag, class) = Self::carrier_handle_disposition(&Lowered::Bytes(Vec::new()))?;
        let word = self.emit_carrier_bytes_claim(builder, tag, class, total)?;
        self.copy_bytes_concat_operand(builder, word, lhs, lhs_len, None)?;
        self.copy_bytes_concat_operand(builder, word, rhs, rhs_len, Some(lhs_len))?;
        Ok(word)
    }

    fn bytes_concat_operand_len(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        operand: &LoweringOperand,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        match operand {
            LoweringOperand::Specialized(Lowered::Bytes(bytes)) => {
                let len = i64::try_from(bytes.len()).map_err(|_| {
                    unsupported(
                        LoweringCause::Capacity,
                        "PrimitiveCall",
                        "a bytes_concat literal is longer than the ABI can name",
                    )
                })?;
                Ok(builder.ins().iconst(types::I64, len))
            }
            LoweringOperand::Specialized(Lowered::ResponseBytes(span)) => Ok(span.len()),
            LoweringOperand::Specialized(Lowered::BorrowedNativeValue { pointer }) => {
                Ok(Self::borrowed_bytes_view(builder, *pointer).1)
            }
            LoweringOperand::Carried(word) => {
                match self.lower_carried_bytes_operand(builder, *word)? {
                    Lowered::ResponseBytes(span) => Ok(span.len()),
                    _ => Err(unsupported(
                        LoweringCause::PlannerInvariant,
                        "PrimitiveCall",
                        "a carried bytes operand did not normalize to a byte span",
                    )),
                }
            }
            LoweringOperand::Residual(_) => Err(
                CraneliftBackendError::ResidualRepresentationRequired {
                    site: "bytes_concat requires an ordinary bytes value",
                },
            ),
            LoweringOperand::Specialized(_) => Err(unsupported(
                LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_concat only supports Bytes arguments in native lowering",
            )),
        }
    }

    fn copy_bytes_concat_operand(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        word: CarriedBoundaryWord,
        operand: &LoweringOperand,
        len: cranelift_codegen::ir::Value,
        offset: Option<cranelift_codegen::ir::Value>,
    ) -> Result<(), CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        match operand {
            LoweringOperand::Specialized(Lowered::Bytes(bytes)) => {
                let refs = self.carrier_refs()?;
                let arena = self.carrier_arena()?;
                for (index, byte) in bytes.iter().enumerate() {
                    let index = i64::try_from(index).map_err(|_| {
                        unsupported(
                            LoweringCause::Capacity,
                            "PrimitiveCall",
                            "a bytes_concat literal is longer than the ABI can name",
                        )
                    })?;
                    let position = match offset {
                        Some(offset) => builder.ins().iadd_imm(offset, index),
                        None => builder.ins().iconst(types::I64, index),
                    };
                    let byte = builder.ins().iconst(types::I64, i64::from(*byte));
                    let write = builder
                        .ins()
                        .call(refs.store_byte, &[arena, word.word, position, byte]);
                    Self::require_i64(builder, builder.inst_results(write)[0], BOUNDARY_OK);
                }
                Ok(())
            }
            LoweringOperand::Specialized(Lowered::ResponseBytes(span)) => {
                self.emit_carrier_bytes_copy(builder, word, span.pointer(), len, offset)
            }
            LoweringOperand::Specialized(Lowered::BorrowedNativeValue { pointer }) => {
                let (data, _) = Self::borrowed_bytes_view(builder, *pointer);
                self.emit_carrier_bytes_copy(builder, word, data, len, offset)
            }
            LoweringOperand::Carried(source) => {
                // Viewed again after the claim; only the length survives the first view.
                let Lowered::ResponseBytes(span) =
                    self.lower_carried_bytes_operand(builder, *source)?
                else {
                    return Err(unsupported(
                        LoweringCause::PlannerInvariant,
                        "PrimitiveCall",
                        "a carried bytes operand did not normalize to a byte span",
                    ));
                };
                let same = builder.ins().icmp(IntCC::Equal, span.len(), len);
                let same = builder.ins().uextend(types::I64, same);
                Self::require_i64(builder, same, 1);
                self.emit_carrier_bytes_copy(builder, word, span.pointer(), len, offset)
            }
            LoweringOperand::Residual(_) | LoweringOperand::Specialized(_) => {
                Err(unsupported(
                    LoweringCause::PlannerInvariant,
                    "PrimitiveCall",
                    "bytes_concat copied an operand its length pass refused",
                ))
            }
        }
    }

    fn lower_bytes_encode(&mut self, args: Vec<Lowered>) -> Result<Lowered, CraneliftBackendError> {
        let [arg]: [Lowered; 1] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("bytes_encode expects 1 arg, got {}", args.len()),
            )
        })?;
        let Lowered::String(value) = arg else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_encode only supports String arguments in native lowering",
            ));
        };
        Ok(Lowered::Bytes(value.into_bytes()))
    }

    fn lower_bytes_decode(
        &mut self,
        args: Vec<Lowered>,
        partiality: &RuntimePartiality,
    ) -> Result<Lowered, CraneliftBackendError> {
        let RuntimePartiality::SafeResult { err, ok, error } = partiality else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_decode requires safe Result metadata",
            ));
        };
        let [arg]: [Lowered; 1] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("bytes_decode expects 1 arg, got {}", args.len()),
            )
        })?;
        let Lowered::Bytes(value) = arg else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "bytes_decode only supports Bytes arguments in native lowering",
            ));
        };
        Ok(match String::from_utf8(value) {
            Ok(value) => Lowered::Constructor {
                constructor: ok.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: vec![ConstructorField::specialized(Lowered::String(
                    value.chars().nfc().collect(),
                ))],
            },
            Err(_) => Lowered::Constructor {
                constructor: err.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: vec![ConstructorField::specialized(Lowered::Constructor {
                    constructor: error.clone(),
                    synthesized_identity: None,
                    occurrence: None,
                    args: Vec::new(),
                })],
            },
        })
    }

    fn lower_string_byte_length(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        args: Vec<Lowered>,
    ) -> Result<Lowered, CraneliftBackendError> {
        let [arg]: [Lowered; 1] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("byte_length expects 1 arg, got {}", args.len()),
            )
        })?;
        let Lowered::String(value) = arg else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "byte_length only supports String arguments in native lowering",
            ));
        };
        let len = i64::try_from(value.len()).map_err(|_| {
            unsupported(LoweringCause::Capacity,
                "PrimitiveCall",
                "byte_length result does not fit the runtime Int representation",
            )
        })?;
        Ok(Lowered::Int {
            value: builder.ins().iconst(types::I64, len),
            known: Some(len),
        })
    }

    fn lower_string_char_length(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        args: Vec<Lowered>,
    ) -> Result<Lowered, CraneliftBackendError> {
        let [arg]: [Lowered; 1] = args.try_into().map_err(|args: Vec<Lowered>| {
            unsupported(LoweringCause::PlannerInvariant,
                "PrimitiveCall",
                format!("char_length expects 1 arg, got {}", args.len()),
            )
        })?;
        let Lowered::String(value) = arg else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                "char_length only supports String arguments in native lowering",
            ));
        };
        let len = i64::try_from(value.chars().count()).map_err(|_| {
            unsupported(LoweringCause::Capacity,
                "PrimitiveCall",
                "char_length result does not fit the runtime Int representation",
            )
        })?;
        Ok(Lowered::Int {
            value: builder.ins().iconst(types::I64, len),
            known: Some(len),
        })
    }
}
