//! Standard-operator roles — layer 1 of `33 §6.1`'s standard-operator surface.
//!
//! **The compiler owns ROLES, never MEANINGS.** A role names a glyph position
//! in the surface grammar. The meaning behind it is an ordinary checked catalog
//! binding, acquired at elaboration through the standard-operator home's own
//! re-export table (layer 2) and certified against a shape contract (layer 3).
//! Nothing here names a module path, a qualified identifier, or a `GlobalId`.
//!
//! That division is not tidiness. `33 §6.1` is explicit that none of these
//! meanings is a built-in, a kernel rule, or a primitive, and `§6.3` states the
//! consequence for the one that is still to come: a builtin *"would have no
//! `GlobalId` to key on and would falsify the precondition the whole completion
//! policy is built on."* A role is the most the compiler may know.

use crate::ast::{Fixity, FixityAssoc};

/// The standard operator roles of `33 §6.1`'s fixity table.
///
/// **CLOSED AT FIVE, and `∈` is deliberately absent.** `§6.1`'s table names
/// six glyph-fixity pairs; this realises the five whose bindings exist.
///
/// The tempting reading — *"the table names `∈`, so the vocabulary must carry
/// it"* — does not survive contact with how the fixities are installed. **A
/// role's fixity attaches to the `GlobalId` the required-roles check
/// certified.** `∈`'s meaning (`membership_member_at`, `§6.3`) is not authored
/// yet, so it has no identity, so **its fixity cannot be installed either.** A
/// sixth variant here would be an arm nothing produces: no binding, no
/// required-role entry, no completion path, no fixity. A dead arm reads as
/// coverage to every later reader, and a vocabulary carrying a role it never
/// requires is a population that overstates itself.
///
/// **The membership track adds the variant together with its binding**, and
/// because this type is crate-internal that widening is an ordinary edit
/// inside `ken-elaborator` rather than a change to anything A1 publishes —
/// A1's published surface is the facade's export table and the completion
/// behaviour, and neither moves. Exhaustive matching then makes the widening a
/// compiler-generated checklist rather than a tax.
///
/// **Crate-internal on purpose.** Nothing outside this crate names a role, so
/// adding the row that fills `Member` is an ordinary internal edit rather than
/// a change to a published surface. What A1 publishes is the facade's export
/// table and the completion behaviour; neither moves when the required list
/// grows.
///
/// Exhaustively matched with no `_ =>` arm at every consumer (`COORDINATION
/// §7`), so a seventh role is a compile error at each site rather than a
/// silent admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum StandardOperatorRole {
    /// `∧` — `bool_and`.
    And,
    /// `∨` — `bool_or`.
    Or,
    /// `≤` — `ord_leq_at`.
    Leq,
    /// `≥` — `ord_geq_at`, which reverses its two ALREADY-EVALUATED argument
    /// values inside its own body (`§6.1`), never the operand expressions.
    Geq,
    /// `≠` — the negation of the comparator the `==` path selects for the
    /// operand carrier (`§6.2`). Unlike the other five this names no single
    /// binding: it is a carrier-directed selection.
    Neq,
}

impl StandardOperatorRole {
    /// The role vocabulary — every role this compiler can name.
    pub(crate) const ALL: [Self; 5] =
        [Self::And, Self::Or, Self::Leq, Self::Geq, Self::Neq];

    /// The roles a program must actually supply.
    ///
    /// **Today this is the whole vocabulary, so the separation is not
    /// load-bearing yet** — and it is written as a distinct question anyway
    /// rather than as a second copy of the list, so it cannot drift from it.
    /// It earns its keep when the two sets diverge, which is precisely the
    /// membership track's edit: `Member` entering the vocabulary and entering
    /// the required list are two separate acts, and a design that cannot say
    /// them separately forces them to happen together.
    pub(crate) fn required() -> &'static [Self] {
        &Self::ALL
    }

    /// The canonical glyph spelling, as `31 §1c` admits it.
    ///
    /// This is the role's NAME, not a key completion matches on — `39 §6.9`
    /// binds to the defining `GlobalId` and never to glyph text. It exists so
    /// a diagnostic can name the role a reader actually wrote.
    pub(crate) const fn glyph(self) -> &'static str {
        match self {
            Self::And => "∧",
            Self::Or => "∨",
            Self::Leq => "≤",
            Self::Geq => "≥",
            Self::Neq => "≠",
        }
    }

    /// The role's standard fixity (`33 §6.1`).
    ///
    /// Declared of the BINDING and therefore travelling with import and
    /// re-export (`§6`), so this is applied to the identity layer 3 certifies,
    /// never to the glyph.
    pub(crate) const fn fixity(self) -> Fixity {
        match self {
            Self::And => Fixity {
                associativity: FixityAssoc::Right,
                precedence: 3,
            },
            Self::Or => Fixity {
                associativity: FixityAssoc::Right,
                precedence: 2,
            },
            Self::Leq | Self::Geq | Self::Neq => Fixity {
                associativity: FixityAssoc::NonAssociative,
                precedence: 4,
            },
        }
    }
}
