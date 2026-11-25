//! The closed representation vocabulary shared by native match consumers.

use super::*;
use cranelift_codegen::ir::Value;

pub(super) enum MatchScrutinee {
    Carried(CarriedBoundaryWord),
    Residual(CarriedResidualWord),
    RecursiveBackedge,
    Bool { value: Value, known: Option<bool> },
    HostResult {
        success: Value,
        error: Box<Lowered>,
        ok: Box<Lowered>,
        err_constructor: String,
        ok_constructor: String,
    },
    BorrowedOption { present: Value, value: Value, none: String, some: String },
    BorrowedNativeValue { pointer: Value },
    DynamicConstructor(DynamicConstructorV1),
    BoundedNat(BoundedNatV1),
    StructuralNat(StructuralNatV1),
    ProcessExitStatus { value: Value },
    Constructor {
        constructor: String,
        synthesized_identity: Option<ConstructorIdentity>,
        occurrence: Option<AggregateOccurrenceId>,
        args: Vec<ConstructorField>,
    },
    /// A representation that no consumer destructures, carried unchanged.
    Opaque(Lowered),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum ScrutineeKind {
    Carried,
    Residual,
    RecursiveBackedge,
    Bool,
    HostResult,
    BorrowedOption,
    BorrowedNativeValue,
    DynamicConstructor,
    BoundedNat,
    StructuralNat,
    ProcessExitStatus,
    Constructor,
    Opaque(LoweredVariant),
}

impl MatchScrutinee {
    #[deny(unreachable_patterns)]
    pub(super) fn classify(operand: LoweringOperand) -> Self {
        let value = match operand {
            LoweringOperand::Carried(word) => return Self::Carried(word),
            LoweringOperand::Residual(word) => return Self::Residual(word),
            LoweringOperand::Specialized(value) => value,
        };
        let variant = value.variant();
        let result = match value {
            Lowered::RecursiveBackedge => Self::RecursiveBackedge,
            Lowered::Bool { value, known } => Self::Bool { value, known },
            Lowered::HostResult { success, error, ok, err_constructor, ok_constructor } => {
                Self::HostResult { success, error, ok, err_constructor, ok_constructor }
            }
            Lowered::BorrowedOption { present, value, none, some } => {
                Self::BorrowedOption { present, value, none, some }
            }
            Lowered::BorrowedNativeValue { pointer } => Self::BorrowedNativeValue { pointer },
            Lowered::DynamicConstructor(value) => Self::DynamicConstructor(value),
            Lowered::BoundedNat(value) => Self::BoundedNat(value),
            Lowered::StructuralNat(value) => Self::StructuralNat(value),
            Lowered::ProcessExitStatus { value } => Self::ProcessExitStatus { value },
            Lowered::Constructor { constructor, synthesized_identity, occurrence, args } => {
                Self::Constructor { constructor, synthesized_identity, occurrence, args }
            }
            value @ (Lowered::Int { .. }
            | Lowered::CapabilityToken { .. }
            | Lowered::ResourceToken { .. }
            | Lowered::ResponseBytes(_)
            | Lowered::StaticResponseDeferred { .. }
            | Lowered::Bytes(_)
            | Lowered::String(_)
            | Lowered::Record { .. }
            | Lowered::Closure { .. }
            | Lowered::DeclarationClosure { .. }
            | Lowered::ComputationalRecursorClosure { .. }
            | Lowered::Trap(_)) => Self::Opaque(value),
        };
        debug_assert_eq!(result.kind(), ScrutineeKind::of_variant(variant));
        result
    }

    #[deny(unreachable_patterns)]
    pub(super) fn into_operand(self) -> LoweringOperand {
        let specialized = match self {
            Self::Carried(word) => return LoweringOperand::Carried(word),
            Self::Residual(word) => return LoweringOperand::Residual(word),
            Self::RecursiveBackedge => Lowered::RecursiveBackedge,
            Self::Bool { value, known } => Lowered::Bool { value, known },
            Self::HostResult { success, error, ok, err_constructor, ok_constructor } => {
                Lowered::HostResult { success, error, ok, err_constructor, ok_constructor }
            }
            Self::BorrowedOption { present, value, none, some } => {
                Lowered::BorrowedOption { present, value, none, some }
            }
            Self::BorrowedNativeValue { pointer } => Lowered::BorrowedNativeValue { pointer },
            Self::DynamicConstructor(value) => Lowered::DynamicConstructor(value),
            Self::BoundedNat(value) => Lowered::BoundedNat(value),
            Self::StructuralNat(value) => Lowered::StructuralNat(value),
            Self::ProcessExitStatus { value } => Lowered::ProcessExitStatus { value },
            Self::Constructor { constructor, synthesized_identity, occurrence, args } => {
                Lowered::Constructor { constructor, synthesized_identity, occurrence, args }
            }
            Self::Opaque(value) => value,
        };
        LoweringOperand::Specialized(specialized)
    }

    #[cfg(any(test, feature = "checked-ih-realization-observation"))]
    pub(super) fn observation_kind(&self) -> &'static str {
        match self {
            Self::Carried(_) => "Carried",
            Self::Residual(_) => "Residual",
            Self::RecursiveBackedge => "RecursiveBackedge",
            Self::Bool { .. } => "Bool",
            Self::HostResult { .. } => "HostResult",
            Self::BorrowedOption { .. } => "BorrowedOption",
            Self::BorrowedNativeValue { .. } => "BorrowedNativeValue",
            Self::DynamicConstructor(_) => "DynamicConstructor",
            Self::BoundedNat(_) => "BoundedNat",
            Self::StructuralNat(_) => "StructuralNat",
            Self::ProcessExitStatus { .. } => "ProcessExitStatus",
            Self::Constructor { .. } => "Constructor",
            Self::Opaque(value) => lowered_value_kind(value),
        }
    }

    #[deny(unreachable_patterns)]
    pub(super) fn kind(&self) -> ScrutineeKind {
        match self {
            Self::Carried(_) => ScrutineeKind::Carried,
            Self::Residual(_) => ScrutineeKind::Residual,
            Self::RecursiveBackedge => ScrutineeKind::RecursiveBackedge,
            Self::Bool { .. } => ScrutineeKind::Bool,
            Self::HostResult { .. } => ScrutineeKind::HostResult,
            Self::BorrowedOption { .. } => ScrutineeKind::BorrowedOption,
            Self::BorrowedNativeValue { .. } => ScrutineeKind::BorrowedNativeValue,
            Self::DynamicConstructor(_) => ScrutineeKind::DynamicConstructor,
            Self::BoundedNat(_) => ScrutineeKind::BoundedNat,
            Self::StructuralNat(_) => ScrutineeKind::StructuralNat,
            Self::ProcessExitStatus { .. } => ScrutineeKind::ProcessExitStatus,
            Self::Constructor { .. } => ScrutineeKind::Constructor,
            Self::Opaque(value) => ScrutineeKind::Opaque(value.variant()),
        }
    }
}

impl ScrutineeKind {
    #[deny(unreachable_patterns)]
    pub(super) fn of_variant(variant: LoweredVariant) -> Self {
        match variant {
            LoweredVariant::RecursiveBackedge => Self::RecursiveBackedge,
            LoweredVariant::Bool => Self::Bool,
            LoweredVariant::HostResult => Self::HostResult,
            LoweredVariant::BorrowedOption => Self::BorrowedOption,
            LoweredVariant::BorrowedNativeValue => Self::BorrowedNativeValue,
            LoweredVariant::DynamicConstructor => Self::DynamicConstructor,
            LoweredVariant::BoundedNat => Self::BoundedNat,
            LoweredVariant::StructuralNat => Self::StructuralNat,
            LoweredVariant::ProcessExitStatus => Self::ProcessExitStatus,
            LoweredVariant::Constructor => Self::Constructor,
            LoweredVariant::Int
            | LoweredVariant::CapabilityToken
            | LoweredVariant::ResourceToken
            | LoweredVariant::ResponseBytes
            | LoweredVariant::StaticResponseDeferred
            | LoweredVariant::Bytes
            | LoweredVariant::String
            | LoweredVariant::Record
            | LoweredVariant::Closure
            | LoweredVariant::DeclarationClosure
            | LoweredVariant::ComputationalRecursorClosure
            | LoweredVariant::Trap => Self::Opaque(variant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LoweredVariant as V, ScrutineeKind as K};

    #[test]
    fn scrutinee_partition_covers_every_lowered_variant() {
        let expected = [
            (V::Int, K::Opaque(V::Int)),
            (V::Bool, K::Bool),
            (V::ProcessExitStatus, K::ProcessExitStatus),
            (V::CapabilityToken, K::Opaque(V::CapabilityToken)),
            (V::ResourceToken, K::Opaque(V::ResourceToken)),
            (V::BoundedNat, K::BoundedNat),
            (V::StructuralNat, K::StructuralNat),
            (V::ResponseBytes, K::Opaque(V::ResponseBytes)),
            (V::HostResult, K::HostResult),
            (V::StaticResponseDeferred, K::Opaque(V::StaticResponseDeferred)),
            (V::DynamicConstructor, K::DynamicConstructor),
            (V::Bytes, K::Opaque(V::Bytes)),
            (V::BorrowedNativeValue, K::BorrowedNativeValue),
            (V::BorrowedOption, K::BorrowedOption),
            (V::String, K::Opaque(V::String)),
            (V::Constructor, K::Constructor),
            (V::Record, K::Opaque(V::Record)),
            (V::Closure, K::Opaque(V::Closure)),
            (V::DeclarationClosure, K::Opaque(V::DeclarationClosure)),
            (V::ComputationalRecursorClosure, K::Opaque(V::ComputationalRecursorClosure)),
            (V::RecursiveBackedge, K::RecursiveBackedge),
            (V::Trap, K::Opaque(V::Trap)),
        ];
        assert_eq!(V::ALL.map(|variant| (variant, K::of_variant(variant))), expected);
    }
}
