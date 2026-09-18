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

use std::collections::HashMap;

use ken_kernel::env::GlobalEnv;
use ken_kernel::{GlobalId, Term};

use crate::ast::{Fixity, FixityAssoc};
use crate::error::{ElabError, Span};

/// The standard-operator home's module path — **the one string this crate
/// holds about the catalog**, and a deliberate residual rather than an
/// oversight.
///
/// Acquisition itself reads the catalog: this module's own `export` line is
/// the glyph-to-identity declaration, so nothing here names `ord_leq_at` or
/// any other meaning. What remains is the path at which to look, and it
/// **fails closed and loud** — if the home moves or is absent, every
/// binding-backed role is unfilled and a standard-operator occurrence is
/// refused naming the role, rather than silently completing to nothing.
///
/// A fully self-declaring home would remove even this; that is recorded as a
/// deferred, non-blocking improvement and is deliberately not built here.
pub(crate) const STANDARD_OPERATOR_HOME: &str = "Core.Operators.Standard";

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

    /// The roles whose meaning is ONE binding published by the
    /// standard-operator home, and which the required-roles check therefore
    /// certifies against that home's export table.
    ///
    /// **`≠` is absent, and it is absent for a different reason than `∈` is
    /// absent from [`Self::ALL`].** `§6.1`'s table names a binding for four
    /// roles and *describes* the fifth: `≠` is "the negation of the comparator
    /// the `==` path selects", a carrier-directed selection over the registry
    /// `§6.2` closes by construction. It has no single identity for a home to
    /// publish, so an export-table check has nothing to look up for it — and
    /// requiring it there would hard-error on a correct tree.
    ///
    /// **So the vocabulary and the certified set genuinely differ TODAY**, at
    /// five against four, rather than only once the membership track lands.
    pub(crate) const BINDING_BACKED: [Self; 4] =
        [Self::And, Self::Or, Self::Leq, Self::Geq];

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

/// What `33 §6.1` fixes for a binding-backed role, as a description a
/// diagnostic can print.
///
/// Stated over the ELABORATED TELESCOPE rather than as a surface signature
/// template, and that is forced by the five rather than granted to a later
/// role. `ord_leq_at`'s own type is
///
/// ```text
/// Π Type 0. (Π (Ord @0). (Π @1. (Π @2. Bool)))
/// ```
///
/// — three of four domains are de Bruijn back-references to earlier binders.
/// A flat signature match was never sufficient for `≤` itself, so there is no
/// non-dependent version of this contract anyone could have built. A domain
/// that is a PROJECTION from an earlier binder (`d.Query`, `§6.3`) is the same
/// kind of back-reference as one that is an APPLICATION to it, which is why
/// this shape extends to the membership role without widening.
fn expected_shape(role: StandardOperatorRole) -> &'static str {
    match role {
        StandardOperatorRole::And | StandardOperatorRole::Or => {
            "(a : Bool) (b : Bool) : Bool"
        }
        StandardOperatorRole::Leq | StandardOperatorRole::Geq => {
            "(a : Type) (d : <class> a) (x : a) (y : a) : Bool"
        }
        // Not binding-backed; never certified against an export table.
        StandardOperatorRole::Neq => "a comparator selection (`33 §6.2`)",
    }
}

/// Peel a Π-telescope into its domains and its final codomain.
fn telescope(ty: &Term) -> (Vec<&Term>, &Term) {
    let mut domains = Vec::new();
    let mut cur = ty;
    while let Term::Pi(domain, codomain) = cur {
        domains.push(domain.as_ref());
        cur = codomain.as_ref();
    }
    (domains, cur)
}

fn is_bool(term: &Term, bool_id: GlobalId) -> bool {
    matches!(
        term,
        Term::IndFormer { id, .. } | Term::Const { id, .. } if *id == bool_id
    )
}

/// Describe a telescope compactly enough for a diagnostic to be actionable.
fn describe(ty: &Term, bool_id: GlobalId) -> String {
    let (domains, codomain) = telescope(ty);
    let arity = domains.len();
    let result = if is_bool(codomain, bool_id) {
        "Bool".to_string()
    } else {
        format!("{codomain:?}")
    };
    format!("arity {arity} returning {result}")
}

/// Does the published binding have the shape `§6.1` fixes for this role?
///
/// Returns `Ok(())`, or a description of what was found for the diagnostic.
fn shape_matches(role: StandardOperatorRole, ty: &Term, bool_id: GlobalId) -> bool {
    let (domains, codomain) = telescope(ty);
    if !is_bool(codomain, bool_id) {
        return false;
    }
    match role {
        // `(a : Bool) (b : Bool) : Bool` — non-dependent, both operands Bool.
        StandardOperatorRole::And | StandardOperatorRole::Or => {
            domains.len() == 2 && domains.iter().all(|d| is_bool(d, bool_id))
        }
        // `(a : Type) (d : <class> a) (x : a) (y : a) : Bool`.
        //
        // The dictionary's own identity is deliberately NOT pinned: the
        // compiler owns roles, never meanings, so requiring the class to be
        // named `Ord` would put a catalog identity back into this crate. What
        // IS pinned is the dependency structure, which is what a moved or
        // re-pointed binding loses: a carrier universe, a dictionary APPLIED
        // to that carrier, and two operands that are the carrier itself.
        StandardOperatorRole::Leq | StandardOperatorRole::Geq => {
            if domains.len() != 4 {
                return false;
            }
            let carrier_is_universe = matches!(domains[0], Term::Type(_));
            let dictionary_depends_on_carrier = matches!(
                domains[1],
                Term::App(_, ref argument) if matches!(argument.as_ref(), Term::Var(0))
            );
            let operands_are_the_carrier =
                matches!(domains[2], Term::Var(1)) && matches!(domains[3], Term::Var(2));
            carrier_is_universe && dictionary_depends_on_carrier && operands_are_the_carrier
        }
        StandardOperatorRole::Neq => false,
    }
}

/// Layer 3 — certify every binding-backed role against the standard-operator
/// home's own export table, and return the identities completion binds to.
///
/// **Acquisition is layer 2 and it reads the catalog rather than this crate.**
/// The home's `export … (bool_and as ∧, ord_leq_at as ≤, …)` line IS the
/// glyph-to-identity declaration (`33 §4.3` republishes, never mints), so the
/// only thing this compiler holds is the home's module path.
///
/// **Two distinct refusals, and they are not one refusal with two messages.**
/// A role absent from the table is closed by publishing a binding; a role
/// present with the wrong shape is closed by fixing the one already published.
/// `§6.2` makes exactly this distinction for `≠`'s two refusals and calls an
/// implementation that collapses them non-conforming; the same reasoning
/// applies here, so the arms are separate variants rather than one.
pub(crate) fn certify_roles(
    env: &GlobalEnv,
    exports: &HashMap<String, HashMap<String, String>>,
    globals: &HashMap<String, GlobalId>,
    home: &str,
    bool_id: GlobalId,
    span: &Span,
) -> Result<HashMap<StandardOperatorRole, GlobalId>, ElabError> {
    let Some(published) = exports.get(home) else {
        // The home is not in this program at all. Completion then fails at the
        // occurrence instead, naming the role -- see the completion adapter.
        return Ok(HashMap::new());
    };

    let mut certified = HashMap::new();
    for role in StandardOperatorRole::BINDING_BACKED {
        let glyph = role.glyph();
        let canonical =
            published
                .get(glyph)
                .ok_or_else(|| ElabError::StandardOperatorRoleUnfilled {
                    role: glyph.to_string(),
                    home: home.to_string(),
                    span: span.clone(),
                })?;
        let id = globals.get(canonical).copied().ok_or_else(|| {
            ElabError::StandardOperatorRoleUnfilled {
                role: glyph.to_string(),
                home: home.to_string(),
                span: span.clone(),
            }
        })?;
        let ty = env
            .lookup(id)
            .and_then(declared_type)
            .ok_or_else(|| ElabError::StandardOperatorRoleWrongShape {
                role: glyph.to_string(),
                binding: canonical.clone(),
                expected: expected_shape(role).to_string(),
                found: "a declaration with no type".to_string(),
                span: span.clone(),
            })?;
        if !shape_matches(role, &ty, bool_id) {
            return Err(ElabError::StandardOperatorRoleWrongShape {
                role: glyph.to_string(),
                binding: canonical.clone(),
                expected: expected_shape(role).to_string(),
                found: describe(&ty, bool_id),
                span: span.clone(),
            });
        }
        certified.insert(role, id);
    }
    Ok(certified)
}

fn declared_type(decl: &ken_kernel::Decl) -> Option<Term> {
    match decl {
        ken_kernel::Decl::Transparent { ty, .. } | ken_kernel::Decl::Opaque { ty, .. } => {
            Some(ty.clone())
        }
        _ => None,
    }
}
