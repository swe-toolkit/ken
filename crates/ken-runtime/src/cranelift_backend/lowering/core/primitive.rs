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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum NativePrimitive {
    AddInt,
    SubInt,
    MulInt,
    EqInt,
    LeqInt,
    Uint8ToInt,
    IntToUint8Raw,
    IntToUint64Raw,
    NotBool,
    AndBool,
    OrBool,
    BytesLength,
    BytesAt,
    BytesSlice,
    BytesConcat,
    BytesEncode,
    BytesDecode,
    BytesToList,
    ListToBytes,
    ListCharToString,
    ByteLength,
    CharLength,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ScalarOperand { Int, Bool }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OperandRoute {
    /// The symbol consumes the raw operand list itself.
    Emitted,
    /// A carried scalar is projected; a borrowed native leaf aborts.
    Scalar(ScalarOperand),
    /// One bytes operand; a carried one is viewed as a span.
    CarriedBytes,
    /// A bytes operand, then an ImmediateInt index.
    BytesAndIndex,
    /// A carried operand refuses at specialized_operands_at.
    SpecializedOnly,
}

impl NativePrimitive {
    pub(super) const ALL: [Self; 22] = [
        Self::AddInt,
        Self::SubInt,
        Self::MulInt,
        Self::EqInt,
        Self::LeqInt,
        Self::Uint8ToInt,
        Self::IntToUint8Raw,
        Self::IntToUint64Raw,
        Self::NotBool,
        Self::AndBool,
        Self::OrBool,
        Self::BytesLength,
        Self::BytesAt,
        Self::BytesSlice,
        Self::BytesConcat,
        Self::BytesEncode,
        Self::BytesDecode,
        Self::BytesToList,
        Self::ListToBytes,
        Self::ListCharToString,
        Self::ByteLength,
        Self::CharLength,
    ];

    #[deny(unreachable_patterns)]
    pub(super) fn symbol(self) -> &'static str {
        match self {
            Self::AddInt => "add_int",
            Self::SubInt => "sub_int",
            Self::MulInt => "mul_int",
            Self::EqInt => "eq_int",
            Self::LeqInt => "leq_int",
            Self::Uint8ToInt => "uint8_to_int",
            Self::IntToUint8Raw => "int_to_uint8_raw",
            Self::IntToUint64Raw => "int_to_uint64_raw",
            Self::NotBool => "not_bool",
            Self::AndBool => "and_bool",
            Self::OrBool => "or_bool",
            Self::BytesLength => "bytes_length",
            Self::BytesAt => "bytes_at",
            Self::BytesSlice => "bytes_slice",
            Self::BytesConcat => "bytes_concat",
            Self::BytesEncode => "bytes_encode",
            Self::BytesDecode => "bytes_decode",
            Self::BytesToList => "bytes_to_list",
            Self::ListToBytes => "list_to_bytes",
            Self::ListCharToString => "list_char_to_string",
            Self::ByteLength => "byte_length",
            Self::CharLength => "char_length",
        }
    }

    pub(super) fn from_symbol(symbol: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.symbol() == symbol)
    }

    #[deny(unreachable_patterns)]
    pub(super) fn operand_route(self) -> OperandRoute {
        match self {
            Self::AddInt => OperandRoute::Scalar(ScalarOperand::Int),
            Self::SubInt => OperandRoute::Scalar(ScalarOperand::Int),
            Self::MulInt => OperandRoute::Scalar(ScalarOperand::Int),
            Self::EqInt => OperandRoute::Scalar(ScalarOperand::Int),
            Self::LeqInt => OperandRoute::Scalar(ScalarOperand::Int),
            Self::Uint8ToInt => OperandRoute::Scalar(ScalarOperand::Int),
            Self::IntToUint8Raw => OperandRoute::Scalar(ScalarOperand::Int),
            Self::IntToUint64Raw => OperandRoute::SpecializedOnly,
            Self::NotBool => OperandRoute::Scalar(ScalarOperand::Bool),
            Self::AndBool => OperandRoute::Scalar(ScalarOperand::Bool),
            Self::OrBool => OperandRoute::Scalar(ScalarOperand::Bool),
            Self::BytesLength => OperandRoute::CarriedBytes,
            Self::BytesAt => OperandRoute::BytesAndIndex,
            Self::BytesSlice => OperandRoute::SpecializedOnly,
            Self::BytesConcat => OperandRoute::Emitted,
            Self::BytesEncode => OperandRoute::SpecializedOnly,
            Self::BytesDecode => OperandRoute::SpecializedOnly,
            Self::BytesToList => OperandRoute::Emitted,
            Self::ListToBytes => OperandRoute::Emitted,
            Self::ListCharToString => OperandRoute::SpecializedOnly,
            Self::ByteLength => OperandRoute::SpecializedOnly,
            Self::CharLength => OperandRoute::SpecializedOnly,
        }
    }
}

#[cfg(test)]
mod operand_table_tests {
    use super::*;

    #[test]
    fn native_primitive_spellings_round_trip() {
        let expected = [
            "add_int",
            "sub_int",
            "mul_int",
            "eq_int",
            "leq_int",
            "uint8_to_int",
            "int_to_uint8_raw",
            "int_to_uint64_raw",
            "not_bool",
            "and_bool",
            "or_bool",
            "bytes_length",
            "bytes_at",
            "bytes_slice",
            "bytes_concat",
            "bytes_encode",
            "bytes_decode",
            "bytes_to_list",
            "list_to_bytes",
            "list_char_to_string",
            "byte_length",
            "char_length",
        ];
        for symbol in expected {
            assert_eq!(NativePrimitive::from_symbol(symbol).map(NativePrimitive::symbol), Some(symbol));
        }
        assert_eq!(NativePrimitive::ALL.into_iter().collect::<BTreeSet<_>>().len(), expected.len());
    }

    #[test]
    fn native_primitive_operand_routes_match_the_closed_table() {
        let expected = [
            (NativePrimitive::AddInt, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::SubInt, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::MulInt, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::EqInt, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::LeqInt, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::Uint8ToInt, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::IntToUint8Raw, OperandRoute::Scalar(ScalarOperand::Int)),
            (NativePrimitive::IntToUint64Raw, OperandRoute::SpecializedOnly),
            (NativePrimitive::NotBool, OperandRoute::Scalar(ScalarOperand::Bool)),
            (NativePrimitive::AndBool, OperandRoute::Scalar(ScalarOperand::Bool)),
            (NativePrimitive::OrBool, OperandRoute::Scalar(ScalarOperand::Bool)),
            (NativePrimitive::BytesLength, OperandRoute::CarriedBytes),
            (NativePrimitive::BytesAt, OperandRoute::BytesAndIndex),
            (NativePrimitive::BytesSlice, OperandRoute::SpecializedOnly),
            (NativePrimitive::BytesConcat, OperandRoute::Emitted),
            (NativePrimitive::BytesEncode, OperandRoute::SpecializedOnly),
            (NativePrimitive::BytesDecode, OperandRoute::SpecializedOnly),
            (NativePrimitive::BytesToList, OperandRoute::Emitted),
            (NativePrimitive::ListToBytes, OperandRoute::Emitted),
            (NativePrimitive::ListCharToString, OperandRoute::SpecializedOnly),
            (NativePrimitive::ByteLength, OperandRoute::SpecializedOnly),
            (NativePrimitive::CharLength, OperandRoute::SpecializedOnly),
        ];
        for (primitive, route) in expected {
            assert_eq!(primitive.operand_route(), route, "{}", primitive.symbol());
        }
    }
}

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

        let native = NativePrimitive::from_symbol(&primitive.symbol);
        let route = native.map(NativePrimitive::operand_route);
        if let Some(OperandRoute::Emitted) = route {
            if native == Some(NativePrimitive::ListToBytes) {
                let [operand] = lowered_args.as_slice() else {
                    return Err(unsupported(
                        LoweringCause::PlannerInvariant,
                        "PrimitiveCall",
                        "list_to_bytes requires exactly one list operand",
                    ));
                };
                return self.lower_list_to_bytes(builder, static_origin, operand);
            }
            if native == Some(NativePrimitive::BytesToList) {
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
            debug_assert_eq!(native, Some(NativePrimitive::BytesConcat));
            // Two literals still use the specialized fold below.
            if !matches!(
                lowered_args.as_slice(),
                [LoweringOperand::Specialized(Lowered::Bytes(_)), LoweringOperand::Specialized(Lowered::Bytes(_))],
            ) {
                return self.lower_dynamic_bytes_concat(builder, &lowered_args).map(LoweringOperand::Carried);
            }
        }
        let lowered_args = match route {
            Some(OperandRoute::CarriedBytes) => {
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
            }
            Some(OperandRoute::BytesAndIndex) => {
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
            }
            Some(OperandRoute::Scalar(kind)) => {
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
                            ScalarOperand::Int => self.lower_dynamic_small_int(builder, zero),
                            ScalarOperand::Bool => Lowered::Bool { value: zero, known: None },
                        })
                    }
                    LoweringOperand::Specialized(value) => Ok(value),
                    LoweringOperand::Carried(word) => {
                        Ok(match kind {
                            ScalarOperand::Int => self.decode_carried_int_operand(builder, word)?,
                            ScalarOperand::Bool => {
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
                        })
                    }
                    LoweringOperand::Residual(_) => Err(
                        CraneliftBackendError::ResidualRepresentationRequired {
                            site: "a scalar primitive requires an ordinary value",
                        },
                    ),
                })
                .collect::<Result<Vec<_>, CraneliftBackendError>>()?
            }
            Some(OperandRoute::SpecializedOnly | OperandRoute::Emitted) | None => {
                specialized_operands_at(&lowered_args, "a primitive-call operand")?
            }
        };
        let Some(native) = native else {
            return Err(unsupported(LoweringCause::Fragment,
                "PrimitiveCall",
                format!("primitive {} is not in the supported native set", primitive.symbol),
            ));
        };
        let lowered = match native {
            NativePrimitive::AddInt => self.lower_int_binop(builder, native.symbol(), lowered_args, |lhs, rhs| {
                lhs.checked_add(rhs)
            }),
            NativePrimitive::SubInt => self.lower_int_binop(builder, native.symbol(), lowered_args, |lhs, rhs| {
                lhs.checked_sub(rhs)
            }),
            NativePrimitive::MulInt => self.lower_int_binop(builder, native.symbol(), lowered_args, |lhs, rhs| {
                lhs.checked_mul(rhs)
            }),
            NativePrimitive::EqInt => self.lower_int_cmp(
                builder,
                native.symbol(),
                lowered_args,
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                |lhs, rhs| lhs == rhs,
            ),
            NativePrimitive::LeqInt => self.lower_int_cmp(
                builder,
                native.symbol(),
                lowered_args,
                cranelift_codegen::ir::condcodes::IntCC::SignedLessThanOrEqual,
                |lhs, rhs| lhs <= rhs,
            ),
            NativePrimitive::Uint8ToInt | NativePrimitive::IntToUint8Raw | NativePrimitive::IntToUint64Raw => {
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
            NativePrimitive::NotBool => self.lower_bool_not(builder, lowered_args),
            NativePrimitive::AndBool => self.lower_bool_binop(
                builder,
                native.symbol(),
                lowered_args,
                |builder, lhs, rhs| builder.ins().band(lhs, rhs),
                |lhs, rhs| lhs && rhs,
            ),
            NativePrimitive::OrBool => self.lower_bool_binop(
                builder,
                native.symbol(),
                lowered_args,
                |builder, lhs, rhs| builder.ins().bor(lhs, rhs),
                |lhs, rhs| lhs || rhs,
            ),
            NativePrimitive::BytesLength => self.lower_bytes_length(builder, lowered_args),
            NativePrimitive::BytesAt => self.lower_bytes_at(builder, lowered_args, &primitive.partiality),
            NativePrimitive::BytesSlice => self.lower_bytes_slice(lowered_args, &primitive.partiality),
            NativePrimitive::BytesConcat => self.lower_bytes_concat(lowered_args),
            NativePrimitive::BytesEncode => self.lower_bytes_encode(lowered_args),
            NativePrimitive::BytesDecode => self.lower_bytes_decode(lowered_args, &primitive.partiality),
            NativePrimitive::ListCharToString => {
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
            NativePrimitive::ByteLength => self.lower_string_byte_length(builder, lowered_args),
            NativePrimitive::CharLength => self.lower_string_char_length(builder, lowered_args),
            NativePrimitive::BytesToList => unreachable!("the emitted bytes_to_list returned above"),
            NativePrimitive::ListToBytes => unreachable!("the emitted list_to_bytes returned above"),
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
            match NativePrimitive::from_symbol(symbol) {
                Some(NativePrimitive::AddInt) => 0,
                Some(NativePrimitive::SubInt) => 1,
                Some(NativePrimitive::MulInt) => 2,
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
            match NativePrimitive::from_symbol(symbol) {
                Some(NativePrimitive::EqInt) => 0,
                Some(NativePrimitive::LeqInt) => 1,
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
        let (bytes, index) = expect_two_args(NativePrimitive::BytesAt.symbol(), args)?;
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

    /// Walk only the compiler's list template, not a list in the runtime heap.
    fn lower_list_to_bytes(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: StaticOriginId,
        operand: &LoweringOperand,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let wrong_shape = || {
            unsupported(
                LoweringCause::Fragment,
                "PrimitiveCall",
                "list_to_bytes requires a statically shaped List UInt8 in native lowering",
            )
        };
        let mut node = match operand {
            LoweringOperand::Specialized(node @ Lowered::Constructor { .. }) => node,
            LoweringOperand::Specialized(_) => return Err(wrong_shape()),
            LoweringOperand::Carried(word) => {
                return self.lower_carried_list_to_bytes(builder, seat, *word);
            }
            LoweringOperand::Residual(_) => {
                return Err(CraneliftBackendError::ResidualRepresentationRequired {
                    site: "list_to_bytes requires an ordinary list value",
                });
            }
        };
        let mut elements = Vec::new();
        loop {
            let Lowered::Constructor {
                constructor, args, ..
            } = node
            else {
                return Err(wrong_shape());
            };
            match args.as_slice() {
                [] if constructor == &self.process_symbols.list_nil => break,
                [head, tail] if constructor == &self.process_symbols.list_cons => {
                    let Lowered::Int { value, known } =
                        head.specialized_at("a list_to_bytes element")?
                    else {
                        return Err(wrong_shape());
                    };
                    elements.push((*value, *known));
                    node = tail.specialized_at("a list_to_bytes tail")?;
                }
                _ => return Err(wrong_shape()),
            }
        }
        if elements.iter().all(|(_, known)| known.is_some()) {
            let bytes = elements
                .into_iter()
                .map(|(_, known)| {
                    u8::try_from(known.expect("all elements are known")).map_err(|_| {
                        unsupported(
                            LoweringCause::PlannerInvariant,
                            "PrimitiveCall",
                            "a list_to_bytes element is outside UInt8",
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(LoweringOperand::Specialized(Lowered::Bytes(bytes)));
        }
        let length = i64::try_from(elements.len()).map_err(|_| {
            unsupported(
                LoweringCause::Capacity,
                "PrimitiveCall",
                "a list_to_bytes spine is longer than the ABI can name",
            )
        })?;
        let length = builder.ins().iconst(types::I64, length);
        let (tag, class) = Self::carrier_handle_disposition(&Lowered::Bytes(Vec::new()))?;
        let word = self.emit_carrier_bytes_claim(builder, tag, class, length)?;
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        for (index, (value, known)) in elements.into_iter().enumerate() {
            // Every index fits in the checked length above.
            let position = builder.ins().iconst(types::I64, index as i64);
            let value = match known {
                Some(value) => {
                    let byte = u8::try_from(value).map_err(|_| {
                        unsupported(
                            LoweringCause::PlannerInvariant,
                            "PrimitiveCall",
                            "a list_to_bytes element is outside UInt8",
                        )
                    })?;
                    builder.ins().iconst(types::I64, i64::from(byte))
                }
                None => {
                    // The Small/Big distinction is in the tag, not the payload:
                    // a Big payload is a slot, so its magnitude proves nothing.
                    let tag = self.native_int_tag(builder, value, None)?;
                    Self::require_i64(builder, tag, crate::NATIVE_INT_SMALL_TAG_V1 as i64);
                    Self::require_octet(builder, value);
                    value
                }
            };
            let write = builder
                .ins()
                .call(refs.store_byte, &[arena, word.word, position, value]);
            Self::require_i64(builder, builder.inst_results(write)[0], BOUNDARY_OK);
        }
        Ok(LoweringOperand::Carried(word))
    }

    /// Fail closed unless `value` is an unsigned octet.
    fn require_octet(builder: &mut FunctionBuilder<'_>, value: cranelift_codegen::ir::Value) {
        use cranelift_codegen::ir::condcodes::IntCC;
        let octet = builder.ins().icmp_imm(IntCC::UnsignedLessThan, value, 256);
        let octet = builder.ins().uextend(types::I64, octet);
        Self::require_i64(builder, octet, 1);
    }

    /// Fail closed unless `node` is a canonical constructor handle: a
    /// Constructor admits only PersistentGround in the persistent region and
    /// InvocationAggregate in the invocation region, so after this guard the
    /// whole word is the node's identity. It runs before any read through the
    /// word.
    fn require_canonical_list_node(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        node: CarriedBoundaryWord,
    ) -> Result<(), CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let tag = builder
            .ins()
            .band_imm(node.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
        let persistent =
            builder.ins().icmp_imm(IntCC::Equal, tag, BoundaryTag::PersistentGround as i64);
        let invocation =
            builder.ins().icmp_imm(IntCC::Equal, tag, BoundaryTag::InvocationAggregate as i64);
        let canonical = builder.ins().bor(persistent, invocation);
        let canonical = builder.ins().uextend(types::I64, canonical);
        Self::require_i64(builder, canonical, 1);
        let class = self.emit_carrier_class(builder, node)?;
        Self::require_i64(builder, class, BoundaryClass::Constructor as i64);
        Ok(())
    }

    /// Read a run-time List UInt8 word into fresh Bytes.
    ///
    /// Pass 1 counts the spine with a Floyd walk, because the store admits a
    /// cycle and adoption, which refuses one, is not on this path. Pass 2
    /// re-reads the spine from the word after the claim: words are handles, so
    /// no pointer crosses the claim. Every malformed node fails closed through
    /// `require_i64` at run time; nothing here is a build-time shape refusal.
    fn lower_carried_list_to_bytes(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: StaticOriginId,
        word: CarriedBoundaryWord,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let nil = self.synthesized_fixed_identity(Role::ListNil)?.tag_abi_word()?;
        let cons = self.synthesized_fixed_identity(Role::ListCons)?.tag_abi_word()?;
        #[cfg(test)]
        let word = self.test_list_to_bytes_carried_seam(builder, seat, word)?;
        #[cfg(not(test))]
        let _ = seat;

        // Pass 1: (hare, tortoise, count, parity), starting at (word, word, 0, 0).
        let visit = builder.create_block();
        for _ in 0..4 {
            builder.append_block_param(visit, types::I64);
        }
        let nil_node = builder.create_block();
        let cons_node = builder.create_block();
        let advance_tortoise = builder.create_block();
        let step = builder.create_block();
        builder.append_block_param(step, types::I64);
        let counted = builder.create_block();
        builder.append_block_param(counted, types::I64);
        let zero = builder.ins().iconst(types::I64, 0);
        builder
            .ins()
            .jump(visit, &[word.word.into(), word.word.into(), zero.into(), zero.into()]);

        builder.switch_to_block(visit);
        let hare = CarriedBoundaryWord { word: builder.block_params(visit)[0] };
        let tortoise = CarriedBoundaryWord { word: builder.block_params(visit)[1] };
        let count = builder.block_params(visit)[2];
        let parity = builder.block_params(visit)[3];
        self.require_canonical_list_node(builder, hare)?;
        let tag = self.emit_carrier_tag(builder, hare)?;
        let fields = self.emit_carrier_field_count(builder, hare)?;
        let nil_tag = Self::carrier_identity_immediate(builder, nil);
        let is_nil = builder.ins().icmp(IntCC::Equal, tag, nil_tag);
        builder.ins().brif(is_nil, nil_node, &[], cons_node, &[]);

        builder.switch_to_block(nil_node);
        Self::require_i64(builder, fields, 0);
        builder.ins().jump(counted, &[count.into()]);

        builder.switch_to_block(cons_node);
        let cons_tag = Self::carrier_identity_immediate(builder, cons);
        let is_cons = builder.ins().icmp(IntCC::Equal, tag, cons_tag);
        let is_cons = builder.ins().uextend(types::I64, is_cons);
        Self::require_i64(builder, is_cons, 1);
        Self::require_i64(builder, fields, 2);
        let next_hare = self.emit_carrier_field(builder, hare, 1)?;
        let next_count = builder.ins().iadd_imm(count, 1);
        let slow = builder.ins().icmp_imm(IntCC::Equal, parity, 1);
        builder
            .ins()
            .brif(slow, advance_tortoise, &[], step, &[tortoise.word.into()]);

        builder.switch_to_block(advance_tortoise);
        let next_tortoise = self.emit_carrier_field(builder, tortoise, 1)?;
        builder.ins().jump(step, &[next_tortoise.word.into()]);

        builder.switch_to_block(step);
        let next_tortoise = builder.block_params(step)[0];
        let next_parity = builder.ins().bxor_imm(parity, 1);
        let same = builder.ins().icmp(IntCC::Equal, next_hare.word, next_tortoise);
        let same = builder.ins().uextend(types::I64, same);
        Self::require_i64(builder, same, 0);
        builder.ins().jump(
            visit,
            &[
                next_hare.word.into(),
                next_tortoise.into(),
                next_count.into(),
                next_parity.into(),
            ],
        );

        builder.switch_to_block(counted);
        let count = builder.block_params(counted)[0];
        let (tag, class) = Self::carrier_handle_disposition(&Lowered::Bytes(Vec::new()))?;
        let out = self.emit_carrier_bytes_claim(builder, tag, class, count)?;

        // Pass 2: (node, index) from (word, 0), stopping at index == count.
        let store = builder.create_block();
        builder.append_block_param(store, types::I64);
        builder.append_block_param(store, types::I64);
        let body = builder.create_block();
        let stored = builder.create_block();
        let zero = builder.ins().iconst(types::I64, 0);
        builder.ins().jump(store, &[word.word.into(), zero.into()]);

        builder.switch_to_block(store);
        let node = CarriedBoundaryWord { word: builder.block_params(store)[0] };
        let index = builder.block_params(store)[1];
        let finished = builder.ins().icmp(IntCC::Equal, index, count);
        builder.ins().brif(finished, stored, &[], body, &[]);

        builder.switch_to_block(body);
        self.require_canonical_list_node(builder, node)?;
        let tag = self.emit_carrier_tag(builder, node)?;
        let cons_tag = Self::carrier_identity_immediate(builder, cons);
        let is_cons = builder.ins().icmp(IntCC::Equal, tag, cons_tag);
        let is_cons = builder.ins().uextend(types::I64, is_cons);
        Self::require_i64(builder, is_cons, 1);
        let fields = self.emit_carrier_field_count(builder, node)?;
        Self::require_i64(builder, fields, 2);
        let head = self.emit_carrier_field(builder, node, 0)?;
        // Every UInt8 is in 0..=255, so an immediate is its only lawful
        // carried form; a boxed head is never decoded and its slot never read.
        let head_tag = builder
            .ins()
            .band_imm(head.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
        Self::require_i64(builder, head_tag, BoundaryTag::ImmediateInt as i64);
        let value = self.emit_carrier_scalar(builder, head)?;
        Self::require_octet(builder, value);
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let write = builder
            .ins()
            .call(refs.store_byte, &[arena, out.word, index, value]);
        Self::require_i64(builder, builder.inst_results(write)[0], BOUNDARY_OK);
        let tail = self.emit_carrier_field(builder, node, 1)?;
        let next = builder.ins().iadd_imm(index, 1);
        builder.ins().jump(store, &[tail.word.into(), next.into()]);

        builder.switch_to_block(stored);
        Ok(LoweringOperand::Carried(out))
    }

    fn lower_bytes_concat(&mut self, args: Vec<Lowered>) -> Result<Lowered, CraneliftBackendError> {
        let (lhs, rhs) = expect_two_args(NativePrimitive::BytesConcat.symbol(), args)?;
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

/// Test-only corruptions of a carried `list_to_bytes` operand, applied at the
/// carried arm's entry before pass 1. Each writes through the ordinary carrier
/// ABI; the seam's own failures return [`LIST_TO_BYTES_SEAM_FAILED`], never the
/// arm's `-1`.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ListToBytesSeam {
    /// Store the first node as the tail of the last Cons.
    Cycle,
    /// Replace head 0 with a carried Big Int (2^64) word.
    BoxedHead,
    /// Rebuild the first two Cons nodes in the invocation region, with the
    /// first node's tail naming the second under the non-canonical
    /// InvocationBorrowed tag, and lower from the rebuilt first node.
    AliasTail,
    /// Build the AliasTail prefix, then return `n0_tag << 8 | alias_tag`
    /// instead of lowering, to report that the fixture reached the
    /// invocation region.
    ReportTailTag,
}

#[cfg(test)]
pub(super) const LIST_TO_BYTES_SEAM_FAILED: i64 = -77;

#[cfg(test)]
std::thread_local! {
    static LIST_TO_BYTES_SEAM: std::cell::Cell<Option<ListToBytesSeam>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(super) fn with_list_to_bytes_seam<T>(seam: ListToBytesSeam, body: impl FnOnce() -> T) -> T {
    let old = LIST_TO_BYTES_SEAM.with(|cell| cell.replace(Some(seam)));
    let result = body();
    LIST_TO_BYTES_SEAM.with(|cell| cell.set(old));
    result
}

#[cfg(test)]
impl<'a> Lowering<'a> {
    fn seam_require_ok(builder: &mut FunctionBuilder<'_>, status: cranelift_codegen::ir::Value) {
        let ok = builder.create_block();
        let failed = builder.create_block();
        let matches = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            status,
            BOUNDARY_OK,
        );
        builder.ins().brif(matches, ok, &[], failed, &[]);
        builder.switch_to_block(failed);
        let marker = builder.ins().iconst(types::I64, LIST_TO_BYTES_SEAM_FAILED);
        builder.ins().return_(&[marker]);
        builder.switch_to_block(ok);
    }

    fn seam_field(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        node: cranelift_codegen::ir::Value,
        position: i64,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer);
        let index = builder.ins().iconst(types::I64, position);
        let call = builder.ins().call(refs.field, &[arena, node, index, out]);
        Self::seam_require_ok(builder, builder.inst_results(call)[0]);
        Ok(builder.ins().stack_load(types::I64, slot, 0))
    }

    fn seam_store_field(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        node: cranelift_codegen::ir::Value,
        position: i64,
        child: cranelift_codegen::ir::Value,
    ) -> Result<(), CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let index = builder.ins().iconst(types::I64, position);
        let call = builder.ins().call(refs.store_field, &[arena, node, index, child]);
        Self::seam_require_ok(builder, builder.inst_results(call)[0]);
        Ok(())
    }

    fn seam_require_one(builder: &mut FunctionBuilder<'_>, value: cranelift_codegen::ir::Value) {
        let ok = builder.create_block();
        let failed = builder.create_block();
        let matches = builder.ins().icmp_imm(cranelift_codegen::ir::condcodes::IntCC::Equal, value, 1);
        builder.ins().brif(matches, ok, &[], failed, &[]);
        builder.switch_to_block(failed);
        let marker = builder.ins().iconst(types::I64, LIST_TO_BYTES_SEAM_FAILED);
        builder.ins().return_(&[marker]);
        builder.switch_to_block(ok);
    }

    /// A fresh invocation-region ListCons node, `[field0, field1]`.
    fn seam_invocation_cons(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        field0: cranelift_codegen::ir::Value,
        field1: cranelift_codegen::ir::Value,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer = builder.func.dfg.value_type(arena);
        let cons = self.synthesized_fixed_identity(Role::ListCons)?.tag_abi_word()?;
        let (slot, out) = Self::carrier_out_slot(builder, pointer);
        let tag = builder.ins().iconst(types::I64, BoundaryTag::InvocationAggregate as i64);
        let class = builder.ins().iconst(types::I64, BoundaryClass::Constructor as i64);
        let two = builder.ins().iconst(types::I64, 2);
        let alloc = builder.ins().call(refs.alloc, &[arena, tag, class, two, out]);
        Self::seam_require_ok(builder, builder.inst_results(alloc)[0]);
        let node = builder.ins().stack_load(types::I64, slot, 0);
        let identity = Self::carrier_identity_immediate(builder, cons);
        let call = builder.ins().call(refs.store_tag_id, &[arena, node, identity]);
        Self::seam_require_ok(builder, builder.inst_results(call)[0]);
        self.seam_store_field(builder, node, 0, field0)?;
        self.seam_store_field(builder, node, 1, field1)?;
        Ok(node)
    }

    /// Rebuild the first two Cons nodes in the invocation region, with n0's
    /// tail naming n1 under the non-canonical InvocationBorrowed tag. Returns
    /// `(n0, alias)`.
    fn seam_invocation_alias_prefix(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        word: CarriedBoundaryWord,
    ) -> Result<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Value), CraneliftBackendError> {
        let h0 = self.seam_field(builder, word.word, 0)?;
        let w1 = self.seam_field(builder, word.word, 1)?;
        let h1 = self.seam_field(builder, w1, 0)?;
        let w2 = self.seam_field(builder, w1, 1)?;
        let n1 = self.seam_invocation_cons(builder, h1, w2)?;
        let index_bits = builder
            .ins()
            .band_imm(n1, !(crate::boundary_value::BOUNDARY_TAG_MASK as i64));
        let alias = builder
            .ins()
            .bor_imm(index_bits, BoundaryTag::InvocationBorrowed as i64);
        let n0 = self.seam_invocation_cons(builder, h0, alias)?;
        Ok((n0, alias))
    }

    fn test_list_to_bytes_carried_seam(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: StaticOriginId,
        word: CarriedBoundaryWord,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let Some(seam) = LIST_TO_BYTES_SEAM.with(std::cell::Cell::get) else {
            return Ok(word);
        };
        match seam {
            ListToBytesSeam::Cycle => {
                let refs = self.carrier_refs()?;
                let arena = self.carrier_arena()?;
                let pointer = builder.func.dfg.value_type(arena);
                let walk = builder.create_block();
                builder.append_block_param(walk, types::I64);
                let close = builder.create_block();
                let next = builder.create_block();
                let done = builder.create_block();
                builder.ins().jump(walk, &[word.word.into()]);
                builder.switch_to_block(walk);
                let node = builder.block_params(walk)[0];
                let tail = self.seam_field(builder, node, 1)?;
                let (slot, out) = Self::carrier_out_slot(builder, pointer);
                let call = builder.ins().call(refs.field_count, &[arena, tail, out]);
                Self::seam_require_ok(builder, builder.inst_results(call)[0]);
                let count = builder.ins().stack_load(types::I64, slot, 0);
                let last = builder.ins().icmp_imm(IntCC::Equal, count, 0);
                builder.ins().brif(last, close, &[], next, &[]);
                builder.switch_to_block(next);
                builder.ins().jump(walk, &[tail.into()]);
                builder.switch_to_block(close);
                self.seam_store_field(builder, node, 1, word.word)?;
                builder.ins().jump(done, &[]);
                builder.switch_to_block(done);
            }
            ListToBytesSeam::BoxedHead => {
                // A Big Int head for 2^64: limbs little-endian (0, 1), sign 0,
                // interned through the function-local NativeIntV1 helper.
                let native_arena = self.function_local.native_int_arena.ok_or_else(|| {
                    unsupported(LoweringCause::PlannerInvariant, "PrimitiveCall",
                        "the boxed-head seam has no native Int arena")
                })?;
                let intern = self.function_local.native_int_intern.ok_or_else(|| {
                    unsupported(LoweringCause::PlannerInvariant, "PrimitiveCall",
                        "the boxed-head seam has no native Int intern helper")
                })?;
                let pointer = builder.func.dfg.value_type(native_arena);
                let limbs =
                    builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
                let low = builder.ins().iconst(types::I64, 0);
                let high = builder.ins().iconst(types::I64, 1);
                builder.ins().stack_store(low, limbs, 0);
                builder.ins().stack_store(high, limbs, 8);
                let limbs = builder.ins().stack_addr(pointer, limbs, 0);
                let output =
                    builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
                let output_pointer = builder.ins().stack_addr(pointer, output, 0);
                let sign = builder.ins().iconst(types::I64, 0);
                let len = builder.ins().iconst(types::I64, 2);
                let call = builder
                    .ins()
                    .call(intern, &[native_arena, sign, limbs, len, output_pointer]);
                let interned = builder.inst_results(call)[0];
                let interned = builder.ins().icmp_imm(IntCC::Equal, interned, 0);
                let interned = builder.ins().uextend(types::I64, interned);
                Self::seam_require_one(builder, interned);
                let tag = builder.ins().stack_load(types::I64, output, 0);
                let payload = builder.ins().stack_load(types::I64, output, 8);
                let big = builder
                    .ins()
                    .icmp_imm(IntCC::Equal, tag, crate::NATIVE_INT_BIG_TAG_V1 as i64);
                let big = builder.ins().uextend(types::I64, big);
                Self::seam_require_one(builder, big);
                let value = self.lowered_from_scalar_pair(
                    ScalarMergeKind::Int,
                    NativeScalarPairV1 { tag, payload },
                );
                let boxed = self.emit_carrier_transfer(builder, seat, &value)?;
                self.seam_store_field(builder, word.word, 0, boxed.word)?;
            }
            ListToBytesSeam::AliasTail => {
                let (n0, _alias) = self.seam_invocation_alias_prefix(builder, word)?;
                return Ok(CarriedBoundaryWord { word: n0 });
            }
            ListToBytesSeam::ReportTailTag => {
                let (n0, alias) = self.seam_invocation_alias_prefix(builder, word)?;
                let mask = crate::boundary_value::BOUNDARY_TAG_MASK as i64;
                let n0_tag = builder.ins().band_imm(n0, mask);
                let alias_tag = builder.ins().band_imm(alias, mask);
                let n0_tag = builder.ins().ishl_imm(n0_tag, 8);
                let tags = builder.ins().bor(n0_tag, alias_tag);
                builder.ins().return_(&[tags]);
                let unreachable = builder.create_block();
                builder.switch_to_block(unreachable);
            }
        }
        Ok(word)
    }
}
