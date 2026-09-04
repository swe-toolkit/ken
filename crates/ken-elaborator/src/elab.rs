//! Bidirectional elaboration to kernel core terms (`39 §5.4`, `§5.7`, `21 §6.3`).
//!
//! V1 additions: `requires`/`ensures` clause processing, obligation holes via
//! `declare_postulate`, honesty guard via `GlobalEnv::trusted_base()`, refinement
//! lowering to carrier, `prove`/`law` declaration elaboration, `old` elaboration.

use std::collections::{HashMap, HashSet};

use ken_kernel::{
    check as kernel_check, convert, convert_type, declare_def, declare_postulate,
    declare_primitive, declare_recursive_group,
    env::PrimReduction,
    inductive::{
        all_support_evidence_positions, method_type, peel_app, peel_pi, recursive_shapes,
        RecursiveArgumentShape,
    },
    infer as kernel_infer,
    sct::sct_check,
    subst::{subst0, subst_levels, subst_outer, subst_tel, weaken},
    whnf, ConstructorDecl, Context, Decl, GlobalEnv, GlobalId, InductiveDecl, Level, LevelVar,
    Term,
};

use crate::ast::{BinOp, DefKeyword, NumLit, RecursiveResultSelector};
use crate::classes::{ClassEnv, ClassInfo, ClassKind, InstanceConstraintInfo, InstanceInfo};
use crate::data;
use crate::error::{ArmDeadCause, ElabError, MissingPatternWitness, RecursiveResultSort, Span};
use crate::numbers::{AddEntry, BinOpEntry, NumericEnv, NumericLitVal};
use crate::resolve::{
    RClassField, RDecl, RDeclKind, RExpr, RInstanceConstraint, RMatchArm, RPatKind, RPattern,
    RPropIntro, RRecordField, RSpaceDecl, RType, SUGAR_ABSURD, SUGAR_AXIOM, SUGAR_ELIM_TRUNC,
    SUGAR_EQ, SUGAR_J, SUGAR_REFL, SUGAR_TRUNC_INTRO,
};

// ----- obligation model -----

/// Source clause kind for a V1 obligation hole (`22 §1`, §2).
#[derive(Debug, Clone)]
pub enum ObligationKind {
    /// From an `ensures ψ` clause or an implicit return-type refinement (`22 §2.2`/§2.1).
    Ensures,
    /// From a `prove name : φ` declaration (`22 §2.4`).
    Prove,
    /// From a `law Name { field : φ }` field (`22 §2.4`).
    LawField(String),
    /// From a bare fixed-width arithmetic op (`35 §3`, `43 §2`).
    PartialPrim,
    /// A `foreign` boundary contract that is statically unprovable → lowered
    /// to a runtime-checked assertion (`21 §5.2`, `38 §3.3`).
    FfiRuntimeCheck,
}

/// A single open obligation hole (`21 §6.5`).
///
/// The hole is admitted as a postulate in the kernel (`trusted_base()` membership
/// = `unknown` status). Discharging it via `ElabEnv::discharge_hole` retires the
/// postulate and moves it to `proved`.
#[derive(Debug, Clone)]
pub struct Obligation {
    /// Sequential id within this elaboration session.
    pub id: u32,
    /// The postulate `GlobalId` registered for this hole (opaque, in `trusted_base()`).
    pub hole_id: GlobalId,
    /// The goal in closed form (abstracted over the local context at the obligation
    /// site). For a goal `φ` in context `[x:A]`, closed = `Pi(A, φ)`.
    pub goal_closed: Term,
    /// The span of the originating clause.
    pub span: Span,
    /// The source clause kind (for V2 provenance and stable ids).
    pub kind: ObligationKind,
}

/// Result of a V1 declaration elaboration.
#[derive(Debug)]
pub struct ElabResult {
    /// Declaration name — used by V2 for stable obligation ids (`22 §1`).
    pub name: String,
    /// The definition's `GlobalId` (or, for `prove`, the hole's postulate id).
    pub def_id: GlobalId,
    /// Open obligation holes emitted during elaboration.
    pub obligations: Vec<Obligation>,
    /// For `foreign` declarations: the full binding record (AC1/AC5 tests).
    /// `None` for all other declaration kinds.
    pub foreign_binding: Option<crate::foreign::ForeignBinding>,
    /// Delegated `Temporal` obligations from `temporal{}` blocks (`72 §4`).
    /// These are **not** kernel holes — a delegated property is exported, not
    /// assumed (`21 §5.2`); they never enter `trusted_base()`. Their sole
    /// projection is the B1 `T`/`delegated` channel (TE-E).
    pub temporal_obligations: Vec<crate::temporal::TemporalObligation>,
    /// Checked surface effect row for `view ... visits [...]` declarations.
    /// Present only when the real const elaboration path consumed the parsed
    /// row annotation and ran the row-poly escape check.
    pub effect_row_type: Option<crate::effects::RowType>,
}

impl ElabResult {
    /// Build [`TEntry`]s from the delegated `Temporal` obligations — the B2
    /// body of the B1 `T` channel (`72 §5`). Each entry carries the elaborated
    /// `Temporal` value with status `delegated` (the constant, pinned at
    /// source).
    pub fn temporal_tentries(&self) -> Vec<crate::export::TEntry> {
        self.temporal_obligations
            .iter()
            .map(|o| crate::export::TEntry {
                obligation_id: o.id.clone(),
                formula: o.formula.clone(),
            })
            .collect()
    }
}

// ----- level meta context -----

#[derive(Default)]
struct MetaCtx {
    metas: Vec<Option<Level>>,
}

impl MetaCtx {
    fn fresh(&mut self) -> Level {
        let id = self.metas.len() as u32;
        self.metas.push(None);
        Level::Var(LevelVar(id))
    }

    fn zonk_level(&self, l: &Level) -> Level {
        match l {
            Level::Zero => Level::Zero,
            Level::Suc(inner) => Level::Suc(Box::new(self.zonk_level(inner))),
            Level::Max(a, b) => {
                Level::Max(Box::new(self.zonk_level(a)), Box::new(self.zonk_level(b)))
            }
            Level::Var(LevelVar(m)) => match &self.metas[*m as usize] {
                Some(sol) => self.zonk_level(sol),
                None => Level::Zero,
            },
        }
    }

    #[allow(dead_code)]
    fn solve(&mut self, m: u32, val: Level) {
        if self.metas[m as usize].is_none() {
            self.metas[m as usize] = Some(val);
        }
    }

    fn zonk_term(&self, t: &Term) -> Term {
        match t {
            Term::Type(l) => Term::ty(self.zonk_level(l)),
            Term::Omega(l) => Term::omega(self.zonk_level(l)),
            Term::Var(i) => Term::var(*i),
            Term::IntLit(n) => Term::IntLit(n.clone()),
            Term::Pi(a, b) => Term::pi(self.zonk_term(a), self.zonk_term(b)),
            Term::Lam(a, body) => Term::lam(self.zonk_term(a), self.zonk_term(body)),
            Term::App(f, a) => Term::app(self.zonk_term(f), self.zonk_term(a)),
            Term::Let { ty, val, body } => Term::Let {
                ty: Box::new(self.zonk_term(ty)),
                val: Box::new(self.zonk_term(val)),
                body: Box::new(self.zonk_term(body)),
            },
            Term::Const { id, level_args } => {
                Term::const_(*id, level_args.iter().map(|l| self.zonk_level(l)).collect())
            }
            Term::IndFormer { id, level_args } => {
                Term::indformer(*id, level_args.iter().map(|l| self.zonk_level(l)).collect())
            }
            Term::Constructor { id, level_args } => {
                Term::constructor(*id, level_args.iter().map(|l| self.zonk_level(l)).collect())
            }
            Term::Sigma(a, b) => Term::sigma(self.zonk_term(a), self.zonk_term(b)),
            Term::Pair(a, b) => Term::pair(self.zonk_term(a), self.zonk_term(b)),
            Term::Proj1(p) => Term::proj1(self.zonk_term(p)),
            Term::Proj2(p) => Term::proj2(self.zonk_term(p)),
            Term::Ascript(t, a) => {
                Term::Ascript(Box::new(self.zonk_term(t)), Box::new(self.zonk_term(a)))
            }
            // `[K2]`-reserved formers — `J`/`Eq`/`Cast`/`Ascript` are exactly
            // the new surface-transport constructs; recursing here closes a
            // pre-existing `zonk_term` completeness gap that nothing built
            // before now exercised (the same "gate-widening exposes latent
            // bugs" shape as `check_match_dependent`'s
            // `subst_var`/unzonked-metavariable fixes —
            // [[gate-widening-exposes-latent-bugs-in-newly-reachable-code]]):
            // any elaborator-built term embedding a level metavariable
            // reaches the raw kernel unresolved unless EVERY structural
            // variant that can carry one recurses.
            Term::Eq(a, x, y) => Term::Eq(
                Box::new(self.zonk_term(a)),
                Box::new(self.zonk_term(x)),
                Box::new(self.zonk_term(y)),
            ),
            Term::Refl(t) => Term::Refl(Box::new(self.zonk_term(t))),
            Term::Cast(a, b, e, t) => Term::Cast(
                Box::new(self.zonk_term(a)),
                Box::new(self.zonk_term(b)),
                Box::new(self.zonk_term(e)),
                Box::new(self.zonk_term(t)),
            ),
            Term::J(m, d, e) => Term::J(
                Box::new(self.zonk_term(m)),
                Box::new(self.zonk_term(d)),
                Box::new(self.zonk_term(e)),
            ),
            Term::Quot(a, r) => {
                Term::Quot(Box::new(self.zonk_term(a)), Box::new(self.zonk_term(r)))
            }
            Term::QuotClass(t) => Term::QuotClass(Box::new(self.zonk_term(t))),
            Term::QuotElim {
                motive,
                method,
                respect,
                scrut,
            } => Term::QuotElim {
                motive: Box::new(self.zonk_term(motive)),
                method: Box::new(self.zonk_term(method)),
                respect: Box::new(self.zonk_term(respect)),
                scrut: Box::new(self.zonk_term(scrut)),
            },
            Term::Trunc(t) => Term::Trunc(Box::new(self.zonk_term(t))),
            Term::TruncProj(t) => Term::TruncProj(Box::new(self.zonk_term(t))),
            Term::Absurd(c, p) => {
                Term::Absurd(Box::new(self.zonk_term(c)), Box::new(self.zonk_term(p)))
            }
            Term::Elim {
                fam,
                level_args,
                params,
                motive,
                methods,
                indices,
                scrut,
            } => Term::Elim {
                fam: *fam,
                level_args: level_args.iter().map(|l| self.zonk_level(l)).collect(),
                params: params.iter().map(|p| self.zonk_term(p)).collect(),
                motive: Box::new(self.zonk_term(motive)),
                methods: methods.iter().map(|m| self.zonk_term(m)).collect(),
                indices: indices.iter().map(|i| self.zonk_term(i)).collect(),
                scrut: Box::new(self.zonk_term(scrut)),
            },
        }
    }
}

// ----- level unification -----

/// IMPORTANT: check raw `Level::Var` BEFORE `zonk_level` — zonking maps `None`
/// metas to `Level::Zero`, masking unsolved metas as concrete zeros.
fn unify_levels(metas: &mut MetaCtx, l1: &Level, l2: &Level) {
    match (l1, l2) {
        (Level::Var(LevelVar(m)), _) if metas.metas[*m as usize].is_none() => {
            let val = metas.zonk_level(l2);
            metas.metas[*m as usize] = Some(val);
        }
        (_, Level::Var(LevelVar(m))) if metas.metas[*m as usize].is_none() => {
            let val = metas.zonk_level(l1);
            metas.metas[*m as usize] = Some(val);
        }
        _ => {}
    }
}

fn unify_types(metas: &mut MetaCtx, t1: &Term, t2: &Term) {
    match (t1, t2) {
        (Term::Type(l1), Term::Type(l2)) => unify_levels(metas, l1, l2),
        (Term::Var(a), Term::Var(b)) if a == b => {}
        (Term::Pi(a1, b1), Term::Pi(a2, b2)) => {
            unify_types(metas, a1, a2);
            unify_types(metas, b1, b2);
        }
        (Term::App(f1, a1), Term::App(f2, a2)) => {
            unify_types(metas, f1, f2);
            unify_types(metas, a1, a2);
        }
        (Term::Lam(a1, b1), Term::Lam(a2, b2)) => {
            unify_types(metas, a1, a2);
            unify_types(metas, b1, b2);
        }
        (
            Term::Const {
                id: id1,
                level_args: la1,
            },
            Term::Const {
                id: id2,
                level_args: la2,
            },
        ) if id1 == id2 => {
            for (l1, l2) in la1.iter().zip(la2.iter()) {
                unify_levels(metas, l1, l2);
            }
        }
        _ => {}
    }
}

// ----- level helpers -----

fn level_from_nat(n: u32) -> Level {
    let mut l = Level::Zero;
    for _ in 0..n {
        l = Level::Suc(Box::new(l));
    }
    l
}

// ----- elaboration context -----

struct ElabCtx<'e> {
    env: &'e mut GlobalEnv,
    /// Required semantic-owner label for every checking-mode `Axiom` minted
    /// through this context. The non-optional type makes missing attribution
    /// unrepresentable; labels are provenance and may legitimately repeat.
    owner_label: String,
    /// Globals admitted as one recursive SCC. Empty outside a mutual group.
    /// This stays elaborator-internal: it recognizes sibling calls without
    /// exposing the generated recursion/refinement encoding at the surface.
    recursive_group: HashSet<GlobalId>,
    ctx: Context,
    metas: MetaCtx,
    globals: &'e HashMap<String, GlobalId>,
    num_values: &'e mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &'e NumericEnv,
    obligations: Vec<Obligation>,
    obl_counter: u32,
    /// The typeclass registry, when available — needed only for `.field`
    /// Σ-record projection (`RExpr::RProj`, `33 §5.2` η). `None` in every
    /// elaboration path that predates class support and never projects
    /// (prove/law/typealias/foreign/temporal/derive/data, recursive views,
    /// the match compiler); wired via `.with_classes` in the view/let path
    /// so a `where C a`-constrained body can project its resolved
    /// dictionary's fields.
    class_env: Option<&'e ClassEnv>,
    /// Fully applied dictionaries introduced by a declaration's `where`
    /// clause.  They are elaborator-local terms, never synthetic globals.
    local_dicts: HashMap<String, (Term, Term, usize)>,
    /// Per-branch index refinements for dependent match (constructor
    /// injectivity + sibling convoy, `check_match_dependent`). Keyed by the
    /// variable's stable bottom-relative context position (`ctx.len()-1-i`
    /// at install time, invariant under later growth — mirrors how
    /// `Context::lookup` itself is position-relative). Value is
    /// `(raw_term, raw_ty, install_depth)`, both stored exactly as built at
    /// `install_depth` (`ctx.len()` at insertion); a later read at a deeper
    /// `ctx.len()` weakens by the difference, the same convention the `RVar`
    /// `weaken(_, i+1)` call already uses for ordinary bindings.
    /// Elaborator-only bookkeeping: the kernel's own `Context` (raw types)
    /// is never touched, so a variable's real (kernel-checked) type never
    /// changes — only which TERM an `RVar` reference resolves to for one
    /// branch's body. Inference selects this outer-refined alias; checking may
    /// instead select the raw context binding when an expected Pi domain asks
    /// for its constructor-local type, preserving both lawful views.
    /// Capability 1 resolves it to a `Cast`-wrapped alias; the
    /// context-telescope convoy resolves it to an index-refinement SENTINEL
    /// `Var` (`INDEX_REFINEMENT_SENTINEL_BASE + slot`) that `finalize_refined_
    /// body` later relocates to the convoy binder — so the resolved term is a
    /// bare `Var` in that case, not a `Cast`.
    var_refinements: HashMap<usize, (Term, Term, usize)>,
    /// Bottom-relative `[start, end)` ranges of `cx.ctx` positions bound as
    /// constructor fields by a match arm currently under elaboration,
    /// innermost last. Capability 2 (sibling convoy) must skip these: a field
    /// bound by an ENCLOSING match is not a genuine outer binder, and refining
    /// it re-points it at the inner match's peeled index while its sibling
    /// references keep the enclosing one (`LANG-CONVOY-MATCH-FIELD-
    /// PROVENANCE`). Provenance, not position — a floor keyed on depth alone
    /// would misclassify a genuine outer binder pushed after the enclosing
    /// match's fields (e.g. a `let` between the outer arm and a nested match).
    match_field_regions: Vec<std::ops::Range<usize>>,
    /// Elaborator-internal method binders are absent from resolved surface
    /// de Bruijn indices. Positions are stable bottom-relative context slots;
    /// `surface_var` skips them when translating an `RVar`.
    hidden_positions: Vec<usize>,
    /// Source values paired with kernel-generated lifted evidence. A support
    /// id marks residual `All`; `None` marks a directly consumable motive leaf.
    lift_bindings: HashMap<usize, LiftBinding>,
    /// Branch-local propositional equalities that relate a concrete matched
    /// constructor to the outer scrutinee. A recursive-group call may use one
    /// to transport its concrete indexed result to the refined expected index.
    /// The proof itself is an index-refinement sentinel until the completed
    /// method is wrapped by `finalize_refined_body`.
    result_refinements: Vec<ResultRefinement>,
    /// The stable bottom-relative position of the state binder plus the
    /// declared cell types while elaborating one space-operation continuation.
    space_state: Option<(usize, Vec<Term>)>,
    /// The stable bottom-relative pre-state position plus cell types while
    /// elaborating a block-space contract. Absent for modifier `space proc`.
    space_pre_state: Option<(usize, Vec<Term>)>,
}

impl<'e> ElabCtx<'e> {
    fn new(
        env: &'e mut GlobalEnv,
        globals: &'e HashMap<String, GlobalId>,
        num_values: &'e mut HashMap<GlobalId, NumericLitVal>,
        numeric_env: &'e NumericEnv,
        owner_label: impl Into<String>,
    ) -> Self {
        Self {
            env,
            owner_label: owner_label.into(),
            recursive_group: HashSet::new(),
            ctx: Context::new(),
            metas: MetaCtx::default(),
            globals,
            num_values,
            numeric_env,
            obligations: Vec::new(),
            obl_counter: 0,
            class_env: None,
            local_dicts: HashMap::new(),
            var_refinements: HashMap::new(),
            match_field_regions: Vec::new(),
            hidden_positions: Vec::new(),
            lift_bindings: HashMap::new(),
            result_refinements: Vec::new(),
            space_state: None,
            space_pre_state: None,
        }
    }

    fn surface_var(&self, index: usize) -> Option<(usize, usize)> {
        let mut remaining = index;
        for position in (0..self.ctx.len()).rev() {
            if self.hidden_positions.contains(&position) {
                continue;
            }
            if remaining == 0 {
                return Some((position, self.ctx.len() - 1 - position));
            }
            remaining -= 1;
        }
        None
    }

    fn binding_term(&self, position: usize) -> Option<(Term, Term)> {
        let index = self.ctx.len().checked_sub(1 + position)?;
        let stored = self.ctx.lookup(index)?;
        Some((Term::var(index), weaken(stored, (index + 1) as i64)))
    }

    /// Resolve a surface binding identity only through the nested-result
    /// association gate. Unlike `surface_var`, this cannot return an ordinary
    /// source term or expose an arbitrary hidden method binder.
    fn selected_recursive_result(&self, index: usize) -> Option<(Term, Term)> {
        let mut remaining = index;
        for position in (0..self.ctx.len()).rev() {
            if self.hidden_positions.contains(&position) {
                continue;
            }
            if remaining == 0 {
                let result_position = self
                    .lift_bindings
                    .get(&position)?
                    .recursive_result_position?;
                return self.binding_term(result_position);
            }
            remaining -= 1;
        }
        None
    }

    fn with_classes(mut self, class_env: &'e ClassEnv) -> Self {
        self.class_env = Some(class_env);
        self
    }

    fn with_recursive_group(mut self, recursive_group: &HashSet<GlobalId>) -> Self {
        self.recursive_group = recursive_group.clone();
        self
    }

    fn with_local_dicts(mut self, local_dicts: &HashMap<String, (Term, Term, usize)>) -> Self {
        self.local_dicts = local_dicts.clone();
        self
    }

    fn install_space_state(&mut self, cell_types: &[Term]) {
        self.space_state = Some((self.ctx.len() - 1, cell_types.to_vec()));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LiftBinding {
    evidence_position: usize,
    recursive_result_position: Option<usize>,
    support: Option<GlobalId>,
}

#[derive(Clone, Debug)]
struct ResultRefinement {
    index_ty: Term,
    concrete_index: Term,
    refined_index: Term,
    premise_slot: usize,
    sentinel_region: usize,
    install_depth: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LiftAssociationFailure {
    Missing {
        source: usize,
    },
    Duplicate {
        sources: Vec<usize>,
    },
    Swapped {
        first: usize,
        second: usize,
    },
    Foreign {
        source: usize,
        expected: Option<GlobalId>,
        actual: Option<GlobalId>,
    },
}

fn validate_lift_associations(
    installed: &HashMap<usize, LiftBinding>,
    expected: &[(usize, LiftBinding)],
) -> Result<(), LiftAssociationFailure> {
    let mut installed_entries = installed.iter().collect::<Vec<_>>();
    installed_entries.sort_by_key(|(source, _)| **source);
    for (index, (source, binding)) in installed_entries.iter().enumerate() {
        for (other_source, other) in installed_entries.iter().skip(index + 1) {
            let duplicate_result = binding
                .recursive_result_position
                .zip(other.recursive_result_position)
                .is_some_and(|(left, right)| left == right);
            if binding.evidence_position == other.evidence_position || duplicate_result {
                return Err(LiftAssociationFailure::Duplicate {
                    sources: vec![**source, **other_source],
                });
            }
        }
    }
    for (source, binding) in expected {
        match installed.get(source) {
            None => return Err(LiftAssociationFailure::Missing { source: *source }),
            Some(actual) if actual.support != binding.support => {
                return Err(LiftAssociationFailure::Foreign {
                    source: *source,
                    expected: binding.support,
                    actual: actual.support,
                })
            }
            Some(actual)
                if actual.evidence_position != binding.evidence_position
                    || actual.recursive_result_position != binding.recursive_result_position =>
            {
                let second = expected
                    .iter()
                    .find(|(candidate_source, candidate)| {
                        candidate_source != source
                            && (candidate.evidence_position == actual.evidence_position
                                || candidate
                                    .recursive_result_position
                                    .zip(actual.recursive_result_position)
                                    .is_some_and(|(left, right)| left == right))
                    })
                    .map(|(candidate_source, _)| *candidate_source)
                    .unwrap_or(*source);
                return Err(LiftAssociationFailure::Swapped {
                    first: *source,
                    second,
                });
            }
            Some(_) => {}
        }
    }
    Ok(())
}

fn lift_association_error(
    failure: LiftAssociationFailure,
    match_span: &Span,
    field_spans: &[(usize, Span)],
) -> ElabError {
    let field_span = |source: usize| {
        field_spans
            .iter()
            .find(|(candidate, _)| *candidate == source)
            .map(|(_, span)| span.clone())
            .unwrap_or_else(|| match_span.clone())
    };
    match failure {
        LiftAssociationFailure::Missing { source } => {
            ElabError::StructuralResultAssociationMissing {
                match_span: match_span.clone(),
                field_span: field_span(source),
            }
        }
        LiftAssociationFailure::Duplicate { sources } => {
            ElabError::StructuralResultAssociationDuplicate {
                match_span: match_span.clone(),
                field_spans: sources.into_iter().map(field_span).collect(),
            }
        }
        LiftAssociationFailure::Swapped { first, second } => {
            ElabError::StructuralResultAssociationSwapped {
                match_span: match_span.clone(),
                first_field_span: field_span(first),
                second_field_span: field_span(second),
            }
        }
        LiftAssociationFailure::Foreign {
            source,
            expected,
            actual,
        } => ElabError::StructuralResultAssociationForeign {
            match_span: match_span.clone(),
            field_span: field_span(source),
            expected_support: expected,
            actual_support: actual,
        },
    }
}

// ----- type elaboration -----

fn elab_type(cx: &mut ElabCtx, ty: &RType) -> Result<Term, ElabError> {
    match ty {
        RType::RUniv(None, _) => {
            let l = cx.metas.fresh();
            Ok(Term::ty(l))
        }
        RType::RUniv(Some(n), _) => Ok(Term::ty(level_from_nat(*n))),

        RType::RCon(name, span) => {
            if name == "Omega" {
                return Ok(Term::omega(Level::Zero));
            }
            let id = cx
                .globals
                .get(name)
                .copied()
                .ok_or_else(|| ElabError::UnresolvedCon {
                    name: name.clone(),
                    span: span.clone(),
                })?;
            // Inductive type formers must be Term::IndFormer, and
            // CONSTRUCTORS must be Term::Constructor, so the kernel's
            // eliminator / conversion rules treat them correctly — a
            // constructor value (e.g. `True`) embedded in a TYPE position
            // (a law-field return-type annotation like
            // `Equal Bool (bool_or True False) True`, ES4-classes) that
            // silently became a bare `Term::Const` would never match
            // `whnf`'s ι-reduction head check (`if let
            // Term::Constructor{..} = head`), permanently stalling
            // reduction on an otherwise-concrete scrutinee.
            if let Some(_) = cx.env.constructor(id) {
                Ok(Term::Constructor {
                    id,
                    level_args: vec![],
                })
            } else if cx.env.inductive(id).is_some() {
                Ok(Term::IndFormer {
                    id,
                    level_args: vec![],
                })
            } else {
                Ok(Term::const_(id, vec![]))
            }
        }

        // `Eq A a b` — the kernel's native equality TYPE, spelled directly
        // (`34 §3.4`, `50-stdlib/53-transport.md §2`, whose combinator
        // listing writes every signature over `Eq`, not the level-fixed
        // `Equal` alias). This is surface PLUMBING for the `J` former's own
        // argument types, not a new eliminator: `Term::Eq` already exists and
        // is already in `trusted_base()` (`term.rs`); `Equal := λA x y. Eq A
        // x y` (`prelude.rs`) is a `declare_def` MONOMORPHIC at `Type0`
        // (`level_params: vec![]`), which cannot spell `cast`'s `Eq Type A
        // B` (an equality of TWO TYPES — the carrier is `Type` itself, one
        // level up). `elab_type` is a raw, UNCHECKED structural builder (the
        // whole declaration is type/kernel-checked later), so building
        // `Term::Eq` directly here — instead of an applied `Const` alias —
        // needs no level parameter at all: the level is read off `A`'s own
        // classification when the surrounding declaration is later checked,
        // exactly as `check.rs`'s own `Term::Eq` inference arm already does.
        RType::RApp(..) if peel_named_rtype_app(ty, SUGAR_EQ, 3).is_some() => {
            let args = peel_named_rtype_app(ty, SUGAR_EQ, 3).expect("checked by guard");
            let a_ty_k = elab_type(cx, args[0])?;
            let a_k = elab_type(cx, args[1])?;
            let b_k = elab_type(cx, args[2])?;
            Ok(Term::Eq(Box::new(a_ty_k), Box::new(a_k), Box::new(b_k)))
        }

        RType::RApp(f, a, _) => {
            let f_k = elab_type(cx, f)?;
            let a_k = elab_type(cx, a)?;
            Ok(Term::app(f_k, a_k))
        }

        RType::RVarTy(i, _, _) => Ok(Term::var(*i)),

        RType::RArr(a, b, _) | RType::REffectArr(a, _, b, _) => {
            let a_core = elab_type(cx, a)?;
            let b_core = elab_type(cx, b)?;
            Ok(Term::pi(a_core, weaken(&b_core, 1)))
        }

        RType::RPi(_, a, b, _) => {
            let a_core = elab_type(cx, a)?;
            cx.ctx.push(a_core.clone());
            let b_core = elab_type(cx, b)?;
            cx.ctx.pop();
            Ok(Term::pi(a_core, b_core))
        }

        RType::RSigma(_, a, b, _) => {
            let a_core = elab_type(cx, a)?;
            cx.ctx.push(a_core.clone());
            let b_core = elab_type(cx, b)?;
            cx.ctx.pop();
            Ok(Term::sigma(a_core, b_core))
        }

        // Refinement lowers to the carrier type (`21 §6.3`): `{x:A|φ}` → `A`.
        // The predicate φ is tracked separately; obligation emitted at introduction.
        RType::RRefine(_, carrier, _phi, _) => elab_type(cx, carrier),
    }
}

// ----- bidirectional elaboration -----

fn prepare_let_rhs(
    cx: &mut ElabCtx,
    ty_opt: &Option<RType>,
    rhs: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    match ty_opt {
        Some(ty) => {
            let ty_core = elab_type(cx, ty)?;
            let rhs_core = check(cx, rhs, &ty_core, span)?;
            Ok((rhs_core, ty_core))
        }
        None => infer(cx, rhs),
    }
}

fn elaborate_if_condition(cx: &mut ElabCtx<'_>, condition: &RExpr) -> Result<Term, ElabError> {
    let (condition_core, _) = infer(cx, condition)?;
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    kernel_check(cx.env, &cx.ctx, &condition_core, &bool_ty).map_err(|_| {
        ElabError::IfConditionNotBool {
            span: condition.span().clone(),
        }
    })?;
    Ok(condition_core)
}

fn make_if_elim(
    cx: &mut ElabCtx<'_>,
    condition: Term,
    then_branch: Term,
    else_branch: Term,
    result_ty: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let classifier =
        kernel_infer(cx.env, &cx.ctx, result_ty).map_err(|error| ElabError::KernelRejected {
            error,
            span: span.clone(),
        })?;
    let motive_sort = match whnf(cx.env, &cx.ctx, &classifier) {
        Term::Type(level) => Term::ty(level),
        Term::Omega(level) => Term::omega(level),
        _ => {
            return Err(ElabError::Internal(
                "conditional result type is not classified by a universe".into(),
            ))
        }
    };
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    let motive = Term::Ascript(
        Box::new(Term::lam(bool_ty.clone(), weaken(result_ty, 1))),
        Box::new(Term::pi(bool_ty, weaken(&motive_sort, 1))),
    );
    let bool_decl = cx
        .env
        .inductive(cx.numeric_env.bool_id)
        .ok_or_else(|| ElabError::Internal("preregistered Bool is missing".into()))?;
    let methods = bool_decl
        .constructors
        .iter()
        .map(|constructor| {
            if constructor.id == cx.numeric_env.bool_true_id {
                Ok(then_branch.clone())
            } else if constructor.id == cx.numeric_env.bool_false_id {
                Ok(else_branch.clone())
            } else {
                Err(ElabError::Internal(
                    "preregistered Bool has an unknown constructor identity".into(),
                ))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methods.len() != 2 {
        return Err(ElabError::Internal(
            "preregistered Bool does not have exactly two constructors".into(),
        ));
    }
    Ok(Term::Elim {
        fam: cx.numeric_env.bool_id,
        level_args: vec![],
        params: vec![],
        motive: Box::new(motive),
        methods,
        indices: vec![],
        scrut: Box::new(condition),
    })
}

fn check_if(
    cx: &mut ElabCtx<'_>,
    condition: &RExpr,
    then_branch: &RExpr,
    else_branch: &RExpr,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let condition_core = elaborate_if_condition(cx, condition)?;
    let then_core = check(cx, then_branch, expected, then_branch.span())?;
    let else_core = check(cx, else_branch, expected, else_branch.span())?;
    make_if_elim(cx, condition_core, then_core, else_core, expected, span)
}

fn infer_if(
    cx: &mut ElabCtx<'_>,
    condition: &RExpr,
    then_branch: &RExpr,
    else_branch: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let condition_core = elaborate_if_condition(cx, condition)?;
    let (then_core, result_ty) = infer(cx, then_branch)?;
    let else_core = check(cx, else_branch, &result_ty, else_branch.span())?;
    let core = make_if_elim(cx, condition_core, then_core, else_core, &result_ty, span)?;
    Ok((core, result_ty))
}

fn check_pair(
    cx: &mut ElabCtx<'_>,
    components: &[RExpr],
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    debug_assert!(components.len() >= 2);
    let expected_wh = whnf(cx.env, &cx.ctx, expected);
    let Term::Sigma(domain, codomain) = expected_wh else {
        return Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "pair literal requires an expected pair type".into(),
        });
    };
    let first = check(cx, &components[0], &domain, components[0].span())?;
    let tail_expected = subst0(&codomain, &first);
    let tail = if components.len() == 2 {
        check(cx, &components[1], &tail_expected, components[1].span())?
    } else {
        check_pair(cx, &components[1..], &tail_expected, span)?
    };
    Ok(Term::pair(first, tail))
}

fn check_record(
    cx: &mut ElabCtx<'_>,
    base: Option<&RExpr>,
    fields: &[(String, RExpr, Span)],
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let (owner_id, head_arg) = match expected {
        Term::App(f, a) => match f.as_ref() {
            Term::Const { id, .. } => (*id, Some((**a).clone())),
            _ => {
                return Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "record literal requires an expected named-field owner type".into(),
                })
            }
        },
        Term::Const { id, .. } => (*id, None),
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "record literal requires an expected named-field owner type".into(),
            })
        }
    };
    let class_env = cx.class_env.ok_or_else(|| ElabError::TypeMismatch {
        span: span.clone(),
        reason: "record literal is unavailable in this elaboration context".into(),
    })?;
    let projection = class_env
        .projection_by_type_id(owner_id)
        .ok_or_else(|| ElabError::TypeMismatch {
            span: span.clone(),
            reason: "record literal expected type is not a known named-field owner".into(),
        })?;
    let owner_name = projection.owner_name.to_string();
    let field_names = projection.field_names.to_vec();
    let field_types = projection.field_types.to_vec();
    let record_nil_val_id = class_env.record_nil_val_id;
    let mut seen = HashMap::<String, Span>::new();
    for (name, _, name_span) in fields {
        if seen.insert(name.clone(), name_span.clone()).is_some() {
            return Err(ElabError::TypeMismatch {
                span: name_span.clone(),
                reason: format!("duplicate field '{name}' for record '{owner_name}'"),
            });
        }
        if !field_names.iter().any(|declared| declared == name) {
            return Err(ElabError::UnresolvedCon {
                name: format!("{owner_name}.{name}"),
                span: name_span.clone(),
            });
        }
    }
    if base.is_none() {
        if let Some(missing) = field_names.iter().find(|name| !seen.contains_key(*name)) {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: format!("record '{owner_name}' literal is missing field '{missing}'"),
            });
        }
    }
    let base_core = base
        .map(|base| check(cx, base, expected, base.span()))
        .transpose()?;
    let mut values = Vec::new();
    for (index, name) in field_names.iter().enumerate() {
        let mut args = Vec::new();
        if let Some(head) = &head_arg {
            args.push(head.clone());
        }
        args.extend(values.iter().cloned());
        let field_expected = subst_tel(&field_types[index], &args);
        let value = if let Some((_, expr, _)) = fields.iter().find(|(given, _, _)| given == name) {
            check(cx, expr, &field_expected, expr.span())?
        } else {
            let mut projected = base_core.clone().expect("update base established");
            for _ in 0..index {
                projected = Term::proj2(projected);
            }
            Term::proj1(projected)
        };
        values.push(value);
    }
    Ok(build_pair_chain(&values, record_nil_val_id))
}

fn check_positional_named_record(
    cx: &mut ElabCtx<'_>,
    components: &[RExpr],
    expected: &Term,
    span: &Span,
) -> Option<Result<Term, ElabError>> {
    let owner_id = match expected {
        Term::App(f, _) => match f.as_ref() {
            Term::Const { id, .. } => *id,
            _ => return None,
        },
        Term::Const { id, .. } => *id,
        _ => return None,
    };
    let names = cx
        .class_env?
        .projection_by_type_id(owner_id)?
        .field_names
        .to_vec();
    if names.len() != components.len() {
        return None;
    }
    let fields = names
        .into_iter()
        .zip(components.iter().cloned())
        .map(|(name, value)| (name, value, span.clone()))
        .collect::<Vec<_>>();
    Some(check_record(cx, None, &fields, expected, span))
}

/// A record literal carries no inferable type on its own (`§37`'s
/// named-field-owner types have no canonical "the" record type to project
/// a synthesized answer from) — pulled out of `infer`'s body for the same
/// per-arm-footprint reason as `check_pair_or_record` above.
#[inline(never)]
fn infer_record_requires_expected_type(span: &Span) -> Result<(Term, Term), ElabError> {
    Err(ElabError::TypeMismatch {
        span: span.clone(),
        reason: "cannot infer record literal type without an expected named record type".into(),
    })
}

fn infer_pair(
    cx: &mut ElabCtx<'_>,
    components: &[RExpr],
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    debug_assert!(components.len() >= 2);
    let (first, first_ty) = infer(cx, &components[0])?;
    let (tail, tail_ty) = if components.len() == 2 {
        infer(cx, &components[1])?
    } else {
        infer_pair(cx, &components[1..], span)?
    };
    let ty = Term::sigma(first_ty, weaken(&tail_ty, 1));
    // A pair is an introduction form, so its inferred type must travel in the
    // core term when the pair is later consumed by an inference-only form such
    // as projection. The kernel erases `Ascript` after checking it.
    let pair = Term::pair(first, tail);
    Ok((Term::Ascript(Box::new(pair), Box::new(ty.clone())), ty))
}

/// Dispatch glue for `RExpr::RPair`, pulled out of `check`'s own body so its
/// two-callee branch (and the `Option<Result<_>>` it inspects) does not sit
/// in `check`'s stack frame. `check` is a single giant match reached by
/// every checked expression in every compile, including ones that use no
/// record syntax at all; in an unoptimized build, locals a match arm
/// introduces are paid by every call regardless of which arm actually runs,
/// so a wide dispatcher's own frame size is worth keeping minimal per arm.
#[inline(never)]
fn check_pair_or_record(
    cx: &mut ElabCtx<'_>,
    components: &[RExpr],
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    match check_positional_named_record(cx, components, expected, span) {
        Some(result) => result,
        None => check_pair(cx, components, expected, span),
    }
}

/// Check a source variable while an indexed-match branch has installed an
/// outer-refined alias for it. `infer` deliberately keeps returning that alias:
/// branch-source reuse needs the view transported from the constructor target
/// index to the outer scrutinee index. Checking mode has more information. A
/// local helper's Pi domain can demand the field's original constructor-local
/// type, which remains the binding's real type in `cx.ctx`; select that raw view
/// only when the refined view does not already satisfy the expected type.
///
/// This makes the refinement genuinely dual-view without changing the kernel
/// context or inventing an equality: the refined view is the existing
/// kernel-checked `try_reindex_cast` term, and the local view is the original
/// kernel binding. If neither view is definitionally suitable, preserve the
/// former behavior (return the refined alias and let ordinary meta unification
/// plus the final kernel re-check decide the term).
#[inline(never)]
fn check_variable_with_index_views(
    cx: &mut ElabCtx,
    surface_index: usize,
    expected: &Term,
) -> Result<Term, ElabError> {
    let (position, actual_index) = cx
        .surface_var(surface_index)
        .ok_or_else(|| ElabError::Internal(format!("Var({surface_index}) out of range")))?;
    let stored_ty = cx
        .ctx
        .lookup(actual_index)
        .ok_or_else(|| ElabError::Internal(format!("Var({surface_index}) out of range")))?;
    let local_ty = weaken(stored_ty, (actual_index as i64) + 1);
    let local_term = Term::var(actual_index);

    let Some((raw_refined_term, raw_refined_ty, install_depth)) =
        cx.var_refinements.get(&position)
    else {
        unify_types(&mut cx.metas, expected, &local_ty);
        return Ok(local_term);
    };
    let growth = cx.ctx.len().checked_sub(*install_depth).ok_or_else(|| {
        ElabError::Internal("index-refined variable escaped its branch context".into())
    })? as i64;
    let refined_term = weaken(raw_refined_term, growth);
    let refined_ty = weaken(raw_refined_ty, growth);
    let expected_zonked = cx.metas.zonk_term(expected);
    let refined_ty_zonked = cx.metas.zonk_term(&refined_ty);
    if convert_type(cx.env, &cx.ctx, &refined_ty_zonked, &expected_zonked) {
        unify_types(&mut cx.metas, expected, &refined_ty);
        return Ok(refined_term);
    }

    let local_ty_zonked = cx.metas.zonk_term(&local_ty);
    if convert_type(cx.env, &cx.ctx, &local_ty_zonked, &expected_zonked) {
        unify_types(&mut cx.metas, expected, &local_ty);
        return Ok(local_term);
    }

    unify_types(&mut cx.metas, expected, &refined_ty);
    Ok(refined_term)
}

#[inline(never)]
fn check_inferred_without_group_transport(
    cx: &mut ElabCtx,
    expr: &RExpr,
    expected: &Term,
) -> Result<Term, ElabError> {
    let (core, inferred_ty) = infer(cx, expr)?;
    unify_types(&mut cx.metas, expected, &inferred_ty);
    Ok(core)
}

#[inline(never)]
fn check_inferred_with_group_transport(
    cx: &mut ElabCtx,
    expr: &RExpr,
    expected: &Term,
) -> Result<Term, ElabError> {
    let (core, inferred_ty) = infer(cx, expr)?;
    if let Some((transported, _)) = transport_recursive_group_call_result(
        cx,
        expr,
        core.clone(),
        inferred_ty.clone(),
        expected,
    )? {
        return Ok(transported);
    }
    unify_types(&mut cx.metas, expected, &inferred_ty);
    Ok(core)
}

fn check(cx: &mut ElabCtx, expr: &RExpr, expected: &Term, _span: &Span) -> Result<Term, ElabError> {
    // FRAME BUDGET: this match is reached by every checked expression in
    // every compile, and in an unoptimized build a new arm's locals are paid
    // by every call regardless of which arm runs (LANG-RECORD-STACK-OVERFLOW,
    // b4d38b8a). A wide, fixed-depth recursion elsewhere (register_decimal_char's
    // 31-level match cascade, unrelated to any arm here) sits close to the
    // guard page as a result: ~115 KiB of headroom out of a 2 MiB thread
    // stack remained at the deepest call after that node's repair -- cleared
    // by inches, not a mile. Keep new arm bodies as a call to a separate
    // (ideally #[inline(never)]) function; don't build locals inline here.
    //
    // REQUIRED MINIMUM (LANG-PRELUDE-ELABORATION-DEPTH D1/D4, measured at
    // 2ca91a3a): a caller of `ElabEnv::new` + `elaborate_file` -- what `ken
    // check`/`ken run` actually do -- must provision at least 4 MiB of
    // stack: more than double the measured unoptimized peak (worst observed
    // 1,982,464 bytes, ~1.98 MiB; a margin of over 2 MiB above it) and never
    // the 2 MiB spawned-thread figure named below as the failure
    // configuration, because that peak was measured only on the shallowest
    // possible input -- a bare-prelude four-line program that calls no
    // prelude combinator, with source nesting depth held constant and never
    // varied -- so it is a floor beneath a deeper program's real cost, not a
    // sufficient bound on it. Optimized peak measured the same way: ~280 KiB.
    //
    // LANG-REFINED-FALLBACK-COLDNESS-CLAIM D2: that gap -- "only ever
    // measured on the shallowest input" -- is now closed with one deep-source
    // data point, not resolved by revising the figure above.
    // LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT D3 process-level-bisected the
    // minimum viable stack for `two_vis_nodes_resume_once_in_source_order`
    // (a real two-node program driving `register_decimal_char`'s 31-level
    // cascade, not the four-line bare-prelude input above) at 1,998,848
    // bytes after that node's fix. That is a process-level MINIMUM-VIABLE-
    // STACK figure -- the smallest allocation that still passes -- not a
    // PEAK-USAGE one, so it is not the same quantity as the 1,982,464
    // figure above and must not be swapped in for it. Whether the headline
    // peak-usage figure itself needs revising still requires a peak-usage
    // run on a deep source, which nobody has done.
    //
    // This is why Rust's 2 MiB spawned-thread default must not be treated as
    // adequate -- it is `r3_c2_source_mixed_branch.rs`'s `r3_4b` worker's
    // exact failure configuration (`#2144`), not a safe answer -- while
    // `ken-cli`'s own 8 MiB main thread has wide headroom today (~6.1 MiB
    // unoptimized free); that margin is this box's, not a guarantee for
    // every future caller.
    match expr {
        // An indexed-match branch may give one source field two lawful types:
        // the outer-refined alias installed by capability 1, and the field's
        // original constructor-local type retained in the kernel context. The
        // expected type selects the view at checking boundaries (notably local
        // helper arguments); inference remains outer-refined by default.
        RExpr::RVar(index, _, _) if !cx.var_refinements.is_empty() => {
            check_variable_with_index_views(cx, *index, expected)
        }
        RExpr::RPair(components, span) => check_pair_or_record(cx, components, expected, span),
        RExpr::RRecord { base, fields, span } => {
            check_record(cx, base.as_deref(), fields, expected, span)
        }
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            span,
        } => check_if(cx, condition, then_branch, else_branch, expected, span),
        RExpr::RNumLit(lit, num_span) => elab_num_lit_checked(cx, lit, expected, num_span),
        RExpr::RStr(s, span) => elab_str_lit(cx, s, Some(expected), span).map(|(t, _)| t),
        // `Refl` — reflexivity, checked (never inferred): the expected goal
        // must originate as a kernel `Eq A t u` / prelude `Equal A t u` with
        // `t`/`u` CONVERTIBLE. If observational equality reduces that equality
        // to Sigma-shaped component obligations before checking sees the Eq
        // head, synthesize the corresponding component proof. This remains
        // gated to an equality-origin target; it is not a general Sigma/And
        // proof search or coercion.
        // Surface sugar only: `Refl` is a bare `ConId` the resolver emits as
        // an `RCon` on scope miss (never registered as a real global), so
        // this must be checked BEFORE the generic `RCon` global lookup.
        RExpr::RCon(name, rspan) if name == SUGAR_REFL => {
            let exp_wh = whnf(cx.env, &cx.ctx, expected);
            if matches!(exp_wh, Term::Eq(..))
                || (matches!(exp_wh, Term::Sigma(..))
                    && refl_goal_originates_in_equality(cx, expected))
            {
                synth_refl_proof(cx.env, &cx.ctx, expected, rspan)
            } else {
                Err(ElabError::TypeMismatch {
                    span: rspan.clone(),
                    reason: "Refl expects an `Eq`-shaped goal".into(),
                })
            }
        }
        // `Axiom` — an EXPLICIT, visible postulate of the expected type
        // (`declare_postulate`, `Decl::Opaque`). The honest surface spelling
        // for an audited-delta law field (`51 §6` erratum's non-zero-delta
        // posture): the resulting `trusted_base()` entry is a real,
        // grep-able `Opaque` — never a silent/implicit assumption. Checked
        // (not inferred), same discipline as `Refl`.
        RExpr::RCon(name, rspan) if name == SUGAR_AXIOM => {
            let id = declare_postulate(cx.env, cx.owner_label.clone(), vec![], expected.clone())
                .map_err(|e| ElabError::KernelRejected {
                    error: e,
                    span: rspan.clone(),
                })?;
            Ok(Term::const_(id, vec![]))
        }
        // `absurd h` — Bottom-elimination (K5, `16 §1.4`): from `h : Bottom`
        // (a hypothesis that has observationally collapsed to `Bottom`, e.g.
        // `Equal D c₁ c₂` for a different-constructor pair), discharge ANY
        // Ω-classified goal — the ascribed `expected` type becomes the
        // eliminator's explicit motive. Surface sugar only: `absurd` is a
        // bare lowercase identifier the resolver emits as an `RCon` on scope
        // miss. Checked (not inferred) so the motive comes from the goal,
        // mirroring `Refl`/`Axiom`/`Proved`.
        //
        // **Reserved-sugar identifiers (FR-2, `docs/program/wp/
        // ds-1-findings-remediation.md`, Architect-corrected).** The five
        // names matched by literal string below (`Refl`/`Axiom`/`absurd`/
        // `J`/`Eq`, `resolve::SUGAR_*` — the shared constants this file and
        // `resolve.rs` both read) do NOT all reserve the same way:
        //
        // - `Refl`/`Axiom` are a bare `RCon` — TOTAL intercept at any arity.
        //   `resolve::RESERVED_SUGAR` rejects a declaration under either
        //   name outright (a resolve-time hard error): it would be wholly
        //   unreachable, full stop.
        // - `absurd` is `RApp(RCon("absurd"), arg)` — arity-**1** only, and
        //   also in `RESERVED_SUGAR` (this is the *originating* FR-2
        //   footgun, DS-1's `absurdEmpty` rename; a value named `absurd` has
        //   no other meaningful arity to coexist at).
        // - `J`/`Eq` are `peel_named_app(_, name, 3)` — arity-**3** only,
        //   BY DESIGN so a lower-arity type-former/class of the same name
        //   coexists (the landed `class Eq a`, `51-lawful-classes.md §2.1`,
        //   is arity-1 and never collides with the arity-3 `Eq A a b`
        //   equality sugar). `J`/`Eq` are deliberately NOT in
        //   `RESERVED_SUGAR` — a declaration-time name reject would break
        //   every legitimate lower-arity `Eq`/`J` use, including most of the
        //   catalog (`DecEq`/`map`/`EmptyDec.ken.md` all pull in `class
        //   Eq`). A user-declared arity-3 type-former literally named
        //   `Eq`/`J` remains a real but deliberately out-of-scope
        //   reservation, not a bug this guard closes.
        RExpr::RApp(f, arg, rspan) if matches!(f.as_ref(), RExpr::RCon(n, _) if n == SUGAR_ABSURD) =>
        {
            let bottom = Term::const_(cx.env.bottom_id(), vec![]);
            let proof_core = check(cx, arg, &bottom, rspan)?;
            Ok(Term::Absurd(
                Box::new(expected.clone()),
                Box::new(proof_core),
            ))
        }
        // `trunc_intro a` — propositional-truncation INTRODUCTION (`16 §6`,
        // LANG-TRUNCATION-SURFACE-SYNTAX D2). The kernel's own `Debug`
        // spelling `|a|` is unavailable (`|` is already `Pipe`, the match-arm
        // separator), so this is a checked-mode arity-1 sugar identifier —
        // same shape as `absurd` above, including its `RESERVED_SUGAR`
        // membership (`resolve.rs`) so a user-declared `trunc_intro` is a
        // hard collision error rather than a silent shadow. Checked, never
        // inferred (`TruncProj` is one of `check.rs::infer`'s explicitly
        // non-inferable introduction forms, `16 §6`) — the expected type
        // supplies `A`; see the dedicated `infer`-mode arm above for the
        // actionable remedy when no expected type is available.
        RExpr::RApp(f, arg, rspan) if matches!(f.as_ref(), RExpr::RCon(n, _) if n == SUGAR_TRUNC_INTRO) => {
            check_trunc_intro(cx, arg, expected, rspan)
        }
        RExpr::RLam(_, body, lam_span) => {
            let exp_wh = whnf(cx.env, &cx.ctx, expected);
            match exp_wh {
                Term::Pi(dom, cod) => {
                    cx.ctx.push(*dom.clone());
                    let body_core = check(cx, body, &cod, lam_span)?;
                    cx.ctx.pop();
                    Ok(Term::lam(*dom, body_core))
                }
                _ => Err(ElabError::LambdaVsNonFunction {
                    span: lam_span.clone(),
                }),
            }
        }
        RExpr::RLet(_name, ty_opt, rhs, body, span) => {
            let (rhs_core, rhs_ty) = prepare_let_rhs(cx, ty_opt, rhs, span)?;
            cx.ctx.push(rhs_ty.clone());
            let body_result = check(cx, body, &weaken(expected, 1), span);
            cx.ctx.pop();
            let body_core = body_result?;
            Ok(Term::Let {
                ty: Box::new(rhs_ty),
                val: Box::new(rhs_core),
                body: Box::new(body_core),
            })
        }
        RExpr::ROld(inner, span) => {
            let Some(pre_state) = cx.space_pre_state.clone() else {
                return Err(ElabError::OldPreStateUnsupported { span: span.clone() });
            };
            let post_state = cx.space_state.replace(pre_state);
            let result = check(cx, inner, expected, span);
            cx.space_state = post_state;
            result
        }
        // `match` against a KNOWN expected type: build the motive from the
        // ascribed goal (`λd. expected[d/scrut]`), not inferred from the
        // first arm's body (ES4-lawproofs AC4). This is what lets a
        // per-branch-varying `Ω`-goal (a structure-class law, `refl :
        // (x:a)->IsTrue (leq x x)`) be proved by case-split at all — the
        // pre-existing `infer_match`/`compile_match_matrix` path (used by
        // `is_sorted`/`Perm`, untouched by this) only ever built a CONSTANT
        // motive derived from arm0's inferred type, which cannot express a
        // goal that differs per constructor.
        RExpr::RMatch {
            scrut,
            equation,
            arms,
            span,
        } => {
            // Gate on PATTERN SHAPE, not goal-dependence: `check_match_
            // dependent` is correct whenever every arm's pattern is FLAT
            // (a constructor with only `Var`/`Wild` sub-patterns) —
            // whether or not `expected` actually mentions the scrutinee.
            // A goal that doesn't mention it (`is_sorted`/`Perm`/`sort`'s
            // `Prop`/carrier-typed returns) just yields a genuinely
            // constant motive (still correctly built and checked — no
            // special-casing needed, verified against `is_sorted`). A goal
            // that mentions a DIFFERENT bound variable than the immediate
            // scrutinee (a hypothesis-driven case-split, e.g. `trans`'s
            // `match y {...}` where the CONCLUSION mentions `x`/`z` but
            // not `y`) is exactly why goal-dependence was the wrong test
            // — the per-arm substitution still correctly threads `x`/`z`
            // through regardless of whether `y` itself appears. Nested
            // constructor sub-patterns (`Suc (Suc m)`) are NOT supported
            // by the flat-pattern builder, so those keep using the
            // existing general `infer_match`/`compile_match_matrix`
            // nested-pattern compiler unchanged.
            let flat = arms.iter().all(|a| match &a.pat.kind {
                RPatKind::Ctor(_, subs) => subs
                    .iter()
                    .all(|s| matches!(s.kind, RPatKind::Var(_) | RPatKind::Wild)),
                _ => false,
            });
            // Flat checked matches use the dependent-match producer path. For
            // indexed families this path emits an equality-premise motive and
            // can synthesize omitted index-impossible methods; nested patterns
            // stay on the existing general `infer_match`/`compile_match_matrix`
            // compiler unchanged.
            let dependent_eligible = flat && {
                let (_, probe_ty) = infer(cx, scrut)?;
                let probe_ty_wh = whnf(cx.env, &cx.ctx, &probe_ty);
                let (head, _) = peel_app(&probe_ty_wh);
                matches!(head, Term::IndFormer { .. })
            };
            if dependent_eligible {
                check_match_dependent(cx, scrut, equation.as_deref(), arms, expected, span)
            } else if equation.is_some() {
                Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "`match ... eqn:` requires a finite enum scrutinee with flat arms"
                        .into(),
                })
            } else {
                let (core, inferred_ty) = infer_match(cx, scrut, arms, span)?;
                unify_types(&mut cx.metas, expected, &inferred_ty);
                Ok(core)
            }
        }
        _ if cx.recursive_group.is_empty() => {
            check_inferred_without_group_transport(cx, expr, expected)
        }
        _ => check_inferred_with_group_transport(cx, expr, expected),
    }
}

fn refl_goal_originates_in_equality(cx: &ElabCtx, expected: &Term) -> bool {
    if matches!(expected, Term::Eq(..)) {
        return true;
    }
    let (head, _) = peel_app(expected);
    matches!(
        head,
        Term::Const { id, .. } if cx.globals.get("Equal").copied() == Some(id)
    )
}

fn synth_refl_proof(
    env: &GlobalEnv,
    ctx: &Context,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    match whnf(env, ctx, expected) {
        Term::Eq(a_ty, t, u) => {
            if convert(env, ctx, &a_ty, &t, &u) {
                Ok(Term::Refl(t))
            } else {
                Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "Refl: the two sides of the goal are not convertible".into(),
                })
            }
        }
        Term::Sigma(dom, cod) => {
            let fst = synth_generated_index_evidence(env, ctx, &dom, span)?;
            let snd_ty = subst0(&cod, &fst);
            let snd = synth_generated_index_evidence(env, ctx, &snd_ty, span)?;
            Ok(Term::pair(fst, snd))
        }
        _ => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "Refl expects an `Eq`-shaped goal".into(),
        }),
    }
}

fn synth_refl_method_from_type(
    env: &GlobalEnv,
    ctx: &mut Context,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    match whnf(env, ctx, expected) {
        Term::Pi(domain, codomain) => {
            let domain = *domain;
            ctx.push(domain.clone());
            let body = synth_refl_method_from_type(env, ctx, &codomain, span);
            ctx.pop();
            Ok(Term::lam(domain, body?))
        }
        _ => synth_refl_proof(env, ctx, expected, span),
    }
}

fn synth_generated_index_evidence(
    env: &GlobalEnv,
    ctx: &Context,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    match whnf(env, ctx, expected) {
        Term::Const { id, .. } if id == env.top_id() => Ok(Term::Const {
            id: env.tt_id(),
            level_args: vec![],
        }),
        _ => synth_refl_proof(env, ctx, expected, span),
    }
}

#[derive(Clone)]
struct IndexEqualityLeaf {
    index_ty: Term,
    target: Term,
    scrutinee: Term,
    proof: Term,
}

/// The complete inventory of data carried by a coherent dependent-match
/// motive into a constructor-refined frame. Keep the dispatch below exhaustive:
/// adding a fourth class must fail compilation until its transport is designed.
enum CoherentFrameCarriedDatum {
    EqualityLeaves(Vec<IndexEqualityLeaf>),
    IndexEquation(Term),
    MotiveReturnTelescopeArguments(Vec<(Term, Term)>),
}

#[derive(Default)]
struct CoherentFrameCarriedData {
    equality_leaves: Vec<IndexEqualityLeaf>,
    index_equation: Option<Term>,
    telescope_argument_substitutions: Vec<(Term, Term)>,
}

impl CoherentFrameCarriedDatum {
    fn carry_into(self, carried: &mut CoherentFrameCarriedData) {
        match self {
            CoherentFrameCarriedDatum::EqualityLeaves(leaves) => {
                carried.equality_leaves = leaves;
            }
            CoherentFrameCarriedDatum::IndexEquation(equation) => {
                debug_assert!(carried.index_equation.is_none());
                carried.index_equation = Some(equation);
            }
            CoherentFrameCarriedDatum::MotiveReturnTelescopeArguments(substitutions) => {
                carried.telescope_argument_substitutions = substitutions;
            }
        }
    }
}

fn carry_coherent_frame_data(
    leaves: Vec<IndexEqualityLeaf>,
    index_equation: Term,
    telescope_argument_substitutions: Vec<(Term, Term)>,
) -> CoherentFrameCarriedData {
    let mut carried = CoherentFrameCarriedData::default();
    for datum in [
        CoherentFrameCarriedDatum::EqualityLeaves(leaves),
        CoherentFrameCarriedDatum::IndexEquation(index_equation),
        CoherentFrameCarriedDatum::MotiveReturnTelescopeArguments(
            telescope_argument_substitutions,
        ),
    ] {
        datum.carry_into(&mut carried);
    }
    carried
}

/// Project one generated dependent-match equality premise into the `Eq` leaves
/// that can lawfully drive `J`-based refinement. The producer keeps one raw
/// `Eq` premise per declared index; observational equality may expose that one
/// premise as a nested `Sigma`, so the proof for each leaf is a projection from
/// the original sentinel rather than a new method argument.
///
/// The walk is atomic: callers receive no leaves unless the complete evidence
/// shape is in the generated `{Eq, Sigma, Top}` vocabulary. In particular, an
/// `Omega` component exposes `Pi` evidence and rejects the whole plan rather
/// than applying only the earlier supported leaves.
fn project_generated_index_equality_leaves(
    env: &GlobalEnv,
    ctx: &Context,
    evidence_ty: &Term,
    proof: Term,
) -> Result<Vec<IndexEqualityLeaf>, ElabError> {
    fn visit(
        env: &GlobalEnv,
        ctx: &Context,
        evidence_ty: &Term,
        proof: Term,
        leaves: &mut Vec<IndexEqualityLeaf>,
    ) -> Result<(), ElabError> {
        match whnf(env, ctx, evidence_ty) {
            Term::Eq(index_ty, target, scrutinee) => {
                leaves.push(IndexEqualityLeaf {
                    index_ty: *index_ty,
                    target: *target,
                    scrutinee: *scrutinee,
                    proof,
                });
                Ok(())
            }
            Term::Sigma(domain, codomain) => {
                let first = Term::proj1(proof.clone());
                visit(env, ctx, &domain, first.clone(), leaves)?;
                let second_ty = subst0(&codomain, &first);
                visit(env, ctx, &second_ty, Term::proj2(proof), leaves)
            }
            Term::Const { id, .. } if id == env.top_id() => Ok(()),
            other => Err(ElabError::Internal(format!(
                "index refinement: unsupported generated equality evidence shape {other:?}"
            ))),
        }
    }

    let mut leaves = Vec::new();
    visit(env, ctx, evidence_ty, proof, &mut leaves)?;
    Ok(leaves)
}

/// Replace an occurrence of `target` with `u` while preserving the surrounding
/// context exactly. Under binders both `target` and `u` are weakened, so the
/// match is against the same outer term as seen from the deeper scope. A thin
/// wrapper over the parallel `subst_term_generalize_many`.
fn subst_term_generalize(term: &Term, target: &Term, u: &Term) -> Term {
    subst_term_generalize_many(term, &[(target.clone(), u.clone())])
}

/// PARALLEL generalization: replace each `target` in `subs` with its paired `u`,
/// simultaneously — no replacement's output is re-scanned for another target.
/// Under binders every target and every replacement is weakened together, so
/// each match is against the same outer term as seen from the deeper scope.
///
/// This is the Architect's simultaneous rebasing primitive
/// (LANG-DEPENDENT-MATCH-MOTIVE-REBASE): when a dependent match generalizes its
/// scrutinee, the scrutinee's OWN indices must be rebased in lockstep with it,
/// or a goal that couples the two (`fin_to_nat nn i`, `i : FokFin nn`) keeps the
/// outer actual index `nn` while the abstracted scrutinee has the local index —
/// an index-family de Bruijn mismatch. Doing them one at a time is unsound here:
/// a later substitution could re-hit an earlier replacement's output. The
/// single-target `subst_term_generalize` is the degenerate one-pair case and is
/// behaviourally identical to before.
fn subst_term_generalize_many(term: &Term, subs: &[(Term, Term)]) -> Term {
    for (target, u) in subs {
        if term == target {
            return u.clone();
        }
    }

    let under = |subs: &[(Term, Term)]| -> Vec<(Term, Term)> {
        subs.iter()
            .map(|(t, u)| (weaken(t, 1), weaken(u, 1)))
            .collect()
    };
    match term {
        Term::Pi(a, b) => Term::pi(
            subst_term_generalize_many(a, subs),
            subst_term_generalize_many(b, &under(subs)),
        ),
        Term::Lam(a, t) => Term::lam(
            subst_term_generalize_many(a, subs),
            subst_term_generalize_many(t, &under(subs)),
        ),
        Term::Sigma(a, b) => Term::sigma(
            subst_term_generalize_many(a, subs),
            subst_term_generalize_many(b, &under(subs)),
        ),
        Term::Let { ty, val, body } => Term::Let {
            ty: Box::new(subst_term_generalize_many(ty, subs)),
            val: Box::new(subst_term_generalize_many(val, subs)),
            body: Box::new(subst_term_generalize_many(body, &under(subs))),
        },
        Term::App(f, a) => Term::app(
            subst_term_generalize_many(f, subs),
            subst_term_generalize_many(a, subs),
        ),
        Term::Pair(a, b) => Term::pair(
            subst_term_generalize_many(a, subs),
            subst_term_generalize_many(b, subs),
        ),
        Term::Proj1(p) => Term::proj1(subst_term_generalize_many(p, subs)),
        Term::Proj2(p) => Term::proj2(subst_term_generalize_many(p, subs)),
        Term::Ascript(t, a) => Term::Ascript(
            Box::new(subst_term_generalize_many(t, subs)),
            Box::new(subst_term_generalize_many(a, subs)),
        ),
        Term::Eq(a, t, u2) => Term::Eq(
            Box::new(subst_term_generalize_many(a, subs)),
            Box::new(subst_term_generalize_many(t, subs)),
            Box::new(subst_term_generalize_many(u2, subs)),
        ),
        Term::Cast(a, b, e, t) => Term::Cast(
            Box::new(subst_term_generalize_many(a, subs)),
            Box::new(subst_term_generalize_many(b, subs)),
            Box::new(subst_term_generalize_many(e, subs)),
            Box::new(subst_term_generalize_many(t, subs)),
        ),
        Term::J(ml, d2, e) => Term::J(
            Box::new(subst_term_generalize_many(ml, subs)),
            Box::new(subst_term_generalize_many(d2, subs)),
            Box::new(subst_term_generalize_many(e, subs)),
        ),
        Term::Quot(a, r) => Term::Quot(
            Box::new(subst_term_generalize_many(a, subs)),
            Box::new(subst_term_generalize_many(r, subs)),
        ),
        Term::QuotClass(t) => Term::QuotClass(Box::new(subst_term_generalize_many(t, subs))),
        Term::Trunc(a) => Term::Trunc(Box::new(subst_term_generalize_many(a, subs))),
        Term::TruncProj(t) => Term::TruncProj(Box::new(subst_term_generalize_many(t, subs))),
        Term::Refl(t) => Term::Refl(Box::new(subst_term_generalize_many(t, subs))),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => Term::QuotElim {
            motive: Box::new(subst_term_generalize_many(motive, subs)),
            method: Box::new(subst_term_generalize_many(method, subs)),
            respect: Box::new(subst_term_generalize_many(respect, subs)),
            scrut: Box::new(subst_term_generalize_many(scrut, subs)),
        },
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => Term::Elim {
            fam: *fam,
            level_args: level_args.clone(),
            params: params
                .iter()
                .map(|p| subst_term_generalize_many(p, subs))
                .collect(),
            motive: Box::new(subst_term_generalize_many(motive, subs)),
            methods: methods
                .iter()
                .map(|m| subst_term_generalize_many(m, subs))
                .collect(),
            indices: indices
                .iter()
                .map(|i| subst_term_generalize_many(i, subs))
                .collect(),
            scrut: Box::new(subst_term_generalize_many(scrut, subs)),
        },
        Term::Absurd(motive, proof) => Term::Absurd(
            Box::new(subst_term_generalize_many(motive, subs)),
            Box::new(subst_term_generalize_many(proof, subs)),
        ),
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => term.clone(),
    }
}

/// Whether `scrut_core` occurs as a subterm of `term`, via an exhaustive
/// structural traversal. Under binders `scrut_core` is weakened with the
/// traversed body. Used to
/// GATE index rebasing: the scrutinee's own indices only need rebasing when the
/// scrutinee is actually coupled to them in the goal (it appears there). When it
/// does not appear — a result-type index unrelated to the scrutinee, e.g. Vec
/// `map`/`zip_with`'s `Vec b n`, or a whole-index motive that mentions only the
/// index — rebasing the index term would over-generalize a coincidentally-equal
/// occurrence and corrupt the goal.
fn scrut_occurs(term: &Term, scrut_core: &Term) -> bool {
    if term == scrut_core {
        return true;
    }
    let under = |t: &Term| -> Term { weaken(t, 1) };
    match term {
        Term::Pi(a, b) | Term::Sigma(a, b) => {
            scrut_occurs(a, scrut_core) || scrut_occurs(b, &under(scrut_core))
        }
        Term::Lam(a, t) => scrut_occurs(a, scrut_core) || scrut_occurs(t, &under(scrut_core)),
        Term::Let { ty, val, body } => {
            scrut_occurs(ty, scrut_core)
                || scrut_occurs(val, scrut_core)
                || scrut_occurs(body, &under(scrut_core))
        }
        Term::App(f, a) | Term::Pair(f, a) => {
            scrut_occurs(f, scrut_core) || scrut_occurs(a, scrut_core)
        }
        Term::Proj1(p) | Term::Proj2(p) | Term::QuotClass(p) | Term::Trunc(p)
        | Term::TruncProj(p) | Term::Refl(p) => scrut_occurs(p, scrut_core),
        Term::Ascript(t, a) | Term::Quot(t, a) | Term::Absurd(t, a) => {
            scrut_occurs(t, scrut_core) || scrut_occurs(a, scrut_core)
        }
        Term::Eq(a, t, u) | Term::J(a, t, u) => {
            scrut_occurs(a, scrut_core)
                || scrut_occurs(t, scrut_core)
                || scrut_occurs(u, scrut_core)
        }
        Term::Cast(a, b, e, t) => {
            scrut_occurs(a, scrut_core)
                || scrut_occurs(b, scrut_core)
                || scrut_occurs(e, scrut_core)
                || scrut_occurs(t, scrut_core)
        }
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => {
            scrut_occurs(motive, scrut_core)
                || scrut_occurs(method, scrut_core)
                || scrut_occurs(respect, scrut_core)
                || scrut_occurs(scrut, scrut_core)
        }
        Term::Elim {
            params,
            motive,
            methods,
            indices,
            scrut,
            ..
        } => {
            params.iter().any(|p| scrut_occurs(p, scrut_core))
                || scrut_occurs(motive, scrut_core)
                || methods.iter().any(|mth| scrut_occurs(mth, scrut_core))
                || indices.iter().any(|i| scrut_occurs(i, scrut_core))
                || scrut_occurs(scrut, scrut_core)
        }
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => false,
    }
}

/// Build the parallel rebase pairs shared by all three dependent-match sites
/// (motive construction, constructor `expected_here`, direct-recursive IH): the
/// scrutinee paired with its LOCAL form, and — ONLY when the scrutinee is
/// coupled to its indices in `expected` — each actual scrutinee index paired
/// with its LOCAL index. Targets are weakened by `depth` (the binders `expected`
/// is being lifted under at that site); the local replacements are already
/// stated at that depth. Applying these TOGETHER via `subst_term_generalize_many`
/// is the Architect invariant (LANG-DEPENDENT-MATCH-MOTIVE-REBASE):
/// `P(actual_indices, outer_scrutinee)` becomes `P(local_indices,
/// local_scrutinee)` in one parallel pass, so a goal coupling the scrutinee to
/// its own index (`fin_to_nat nn i`) stays well-typed once the scrutinee is
/// abstracted, while an uncoupled goal keeps its original scrutinee-only pass.
fn dependent_rebase_subs(
    scrut_core: &Term,
    scrut_indices: &[Term],
    depth: i64,
    local_indices: &[Term],
    local_scrut: &Term,
    expected: &Term,
    // Force the index rebase even when the scrutinee itself does not occur in the
    // goal. A non-empty context-telescope convoy couples the scrutinee's index to
    // the goal THROUGH a captured binder (`Equal (Env n) xs xs`, where `xs` is
    // convoyed): the index must then rebase to the local binder in lockstep with
    // the convoy, or the goal's own `n` and the convoy binder's index diverge.
    // When the convoy is empty this stays the scrutinee-occurrence gate that keeps
    // uncoupled goals (Vec `map`/`zip_with`) on their exact original pass.
    force: bool,
) -> Vec<(Term, Term)> {
    debug_assert_eq!(scrut_indices.len(), local_indices.len());
    let mut subs: Vec<(Term, Term)> = Vec::with_capacity(scrut_indices.len() + 1);
    if force || scrut_occurs(expected, scrut_core) {
        subs.extend(
            scrut_indices
                .iter()
                .zip(local_indices)
                .map(|(actual, local)| (weaken(actual, depth), local.clone())),
        );
    }
    subs.push((weaken(scrut_core, depth), local_scrut.clone()));
    subs
}

/// One ambient-context convoy entry (LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-
/// REBASE): a captured ambient binding whose type is changed by the root
/// index/scrutinee rebasing (or by an earlier convoy binder), so it must travel
/// with the index refinement as part of the motive-codomain telescope rather
/// than be re-typed in place.
#[derive(Clone)]
struct ConvoyEntry {
    /// de Bruijn index of the ambient binding at the ambient depth (0 = innermost).
    var: usize,
    /// The binding's declared type, weakened to the ambient depth (valid there).
    ambient_ty: Term,
}

/// One method of an already-elaborated indexed eliminator that is valid in the
/// ambient goal but ceases to be valid when that eliminator's actual index is
/// generalized into an enclosing match motive. The method travels as an
/// explicit motive companion: at the original index the completed outer
/// eliminator supplies `ambient_value`; at constructor-local indices the method
/// is a genuine hypothesis. This keeps an impossible-branch discharge and the
/// generated reflexivity witness in the same convoy instead of rewriting only
/// one of them.
#[derive(Clone)]
struct EmbeddedMethodConvoy {
    sentinel_region: usize,
    elim_ordinal: usize,
    method_ordinal: usize,
    ambient_value: Term,
    ambient_ty: Term,
    motive_ty: Term,
}

#[derive(Clone)]
struct EmbeddedElimSnapshot {
    fam: GlobalId,
    level_args: Vec<Level>,
    params: Vec<Term>,
    motive: Term,
    methods: Vec<Term>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RecursiveFieldIndexPath {
    /// The recursive field carries its own constructor-declared index. The
    /// ordinary eliminator applies the motive directly at that index.
    PlainDeclared,
    /// A sibling scrutinee or forced index shares the refinement frame. Keep
    /// the existing equality/convoy path for those genuinely coupled shapes.
    CoupledRefinement,
}

struct CoherentFrameMotivePlan {
    expected: Term,
    context_convoy: Vec<ConvoyEntry>,
    embedded_method_convoy: Vec<EmbeddedMethodConvoy>,
    embedded_method_repairs: Vec<(usize, usize)>,
    motive_user_body: Term,
    equation_convoy: bool,
    recursive_field_index_path: RecursiveFieldIndexPath,
}

/// Collect only the outermost eliminators in each term position. Once an
/// eliminator is captured, its methods belong to that eliminator's own frame;
/// any nested elimination in a method is reached after that method is selected
/// and is not a sibling companion of the enclosing goal occurrence.
#[inline(never)]
fn collect_outer_embedded_elims(term: &Term, out: &mut Vec<EmbeddedElimSnapshot>) {
    if let Term::Elim {
        fam,
        level_args,
        params,
        motive,
        methods,
        ..
    } = term
    {
        out.push(EmbeddedElimSnapshot {
            fam: *fam,
            level_args: level_args.clone(),
            params: params.clone(),
            motive: (**motive).clone(),
            methods: methods.clone(),
        });
        return;
    }
    for child in term.children() {
        collect_outer_embedded_elims(child, out);
    }
}

fn term_contains_absurd(term: &Term) -> bool {
    matches!(term, Term::Absurd(_, _)) || term.children().into_iter().any(term_contains_absurd)
}

/// Rebuild an invalid structurally recursive method from the recursive-IH
/// binder already required by the kernel method telescope. The surface method
/// may have captured a concrete-index cast; after convoying the actual index,
/// its recursive IH is the canonical constructor-local computation and carries
/// the exact generalized result type.
fn repair_embedded_method_from_ih(
    env: &GlobalEnv,
    ctx: &Context,
    method: &Term,
    expected: &Term,
) -> Option<Term> {
    fn go(env: &GlobalEnv, ctx: &Context, method: &Term, expected: &Term) -> Option<Term> {
        match (method, whnf(env, ctx, expected)) {
            (Term::Lam(_, body), Term::Pi(domain, codomain)) => {
                let mut body_ctx = ctx.clone();
                body_ctx.push((*domain).clone());
                let body = go(env, &body_ctx, body, &codomain)?;
                Some(Term::lam(*domain, body))
            }
            _ if kernel_check(env, ctx, method, expected).is_ok() => Some(method.clone()),
            _ => {
                for index in 0..ctx.len() {
                    let mut candidate = Term::var(index);
                    let mut candidate_ty = kernel_infer(env, ctx, &candidate).ok()?;
                    let mut consumed = false;
                    loop {
                        if convert_type(env, ctx, &candidate_ty, expected) && consumed {
                            return Some(candidate);
                        }
                        let Term::Pi(domain, codomain) = whnf(env, ctx, &candidate_ty) else {
                            break;
                        };
                        let Some(argument) = (0..ctx.len()).find_map(|argument_index| {
                            let argument = Term::var(argument_index);
                            let argument_ty = kernel_infer(env, ctx, &argument).ok()?;
                            convert_type(env, ctx, &argument_ty, &domain).then_some(argument)
                        }) else {
                            break;
                        };
                        candidate = Term::app(candidate, argument.clone());
                        candidate_ty = subst0(&codomain, &argument);
                        consumed = true;
                    }
                }
                None
            }
        }
    }
    go(env, ctx, method, expected)
}

/// Find exactly those embedded methods whose ambient proof was specialized to
/// the old actual index. A method is convoyed only when its original form
/// kernel-checks and the same structural rebase fails against the generalized
/// method type; already-parametric methods stay in place.
#[inline(never)]
fn plan_embedded_method_convoy(
    env: &GlobalEnv,
    ambient_ctx: &Context,
    motive_ctx: &Context,
    ambient_expected: &Term,
    rebased_expected: &Term,
    sentinel_region: usize,
) -> Result<(Vec<EmbeddedMethodConvoy>, Vec<(usize, usize)>), ElabError> {
    let mut ambient_elims = Vec::new();
    let mut rebased_elims = Vec::new();
    collect_outer_embedded_elims(ambient_expected, &mut ambient_elims);
    collect_outer_embedded_elims(rebased_expected, &mut rebased_elims);
    if ambient_elims.len() != rebased_elims.len() {
        return Err(ElabError::Internal(format!(
            "coherent-frame convoy changed the embedded-eliminator population: {} -> {}",
            ambient_elims.len(),
            rebased_elims.len()
        )));
    }

    let mut plan = Vec::new();
    let mut repairs = Vec::new();
    for (elim_ordinal, (ambient, rebased)) in ambient_elims.iter().zip(&rebased_elims).enumerate() {
        if ambient.fam != rebased.fam
            || ambient.level_args != rebased.level_args
            || ambient.methods.len() != rebased.methods.len()
        {
            return Err(ElabError::Internal(
                "coherent-frame convoy changed an embedded eliminator's identity".into(),
            ));
        }
        let ind = env.inductive(ambient.fam).ok_or_else(|| {
            ElabError::Internal(format!(
                "coherent-frame convoy could not find embedded family {:?}",
                ambient.fam
            ))
        })?;
        for method_ordinal in 0..ambient.methods.len() {
            let ambient_ty = method_type(
                env,
                ind,
                method_ordinal,
                &ambient.motive,
                &ambient.params,
                &ambient.level_args,
            )
            .map_err(|error| {
                ElabError::Internal(format!(
                    "coherent-frame convoy could not classify ambient method: {error:?}"
                ))
            })?;
            kernel_check(
                env,
                ambient_ctx,
                &ambient.methods[method_ordinal],
                &ambient_ty,
            )
            .map_err(|error| {
                ElabError::Internal(format!(
                    "coherent-frame convoy found an ill-typed ambient method: {error:?}"
                ))
            })?;

            let motive_ty = method_type(
                env,
                ind,
                method_ordinal,
                &rebased.motive,
                &rebased.params,
                &rebased.level_args,
            )
            .map_err(|error| {
                ElabError::Internal(format!(
                    "coherent-frame convoy could not classify rebased method: {error:?}"
                ))
            })?;
            let rebased_check = kernel_check(
                env,
                motive_ctx,
                &rebased.methods[method_ordinal],
                &motive_ty,
            );
            if rebased_check.is_err() {
                if term_contains_absurd(&ambient.methods[method_ordinal]) {
                    plan.push(EmbeddedMethodConvoy {
                        sentinel_region,
                        elim_ordinal,
                        method_ordinal,
                        ambient_value: ambient.methods[method_ordinal].clone(),
                        ambient_ty,
                        motive_ty,
                    });
                } else if repair_embedded_method_from_ih(
                    env,
                    motive_ctx,
                    &rebased.methods[method_ordinal],
                    &motive_ty,
                )
                .is_some()
                {
                    repairs.push((elim_ordinal, method_ordinal));
                } else {
                    return Err(ElabError::Internal(format!(
                        "coherent-frame convoy cannot repair embedded method \
                         {elim_ordinal}:{method_ordinal}"
                    )));
                }
            }
        }
    }
    Ok((plan, repairs))
}

/// Replace the planned methods by their companion-binder sentinels. Traversal
/// order is structural and fail-closed: every planned coordinate must be seen
/// exactly once, so two identical method terms in distinct eliminators cannot
/// be conflated by a term-wide substitution.
#[inline(never)]
fn install_embedded_method_sentinels(
    env: &GlobalEnv,
    ctx: &Context,
    term: &Term,
    plan: &[EmbeddedMethodConvoy],
    repairs: &[(usize, usize)],
    slot_base: usize,
) -> Result<Term, ElabError> {
    fn go(
        env: &GlobalEnv,
        ctx: &Context,
        term: &Term,
        plan: &[EmbeddedMethodConvoy],
        repairs: &[(usize, usize)],
        slot_base: usize,
        next_elim: &mut usize,
        seen: &mut [bool],
    ) -> Term {
        let recur = |term: &Term, next_elim: &mut usize, seen: &mut [bool]| {
            go(env, ctx, term, plan, repairs, slot_base, next_elim, seen)
        };
        match term {
            Term::Elim {
                fam,
                level_args,
                params,
                motive,
                methods,
                indices,
                scrut,
            } => {
                let ordinal = *next_elim;
                *next_elim += 1;
                let params: Vec<Term> = params
                    .iter()
                    .map(|term| recur(term, next_elim, seen))
                    .collect();
                let motive = Box::new(recur(motive, next_elim, seen));
                let methods = methods
                    .iter()
                    .enumerate()
                    .map(|(method_ordinal, method)| {
                        let rewritten = recur(method, next_elim, seen);
                        if let Some((plan_index, entry)) =
                            plan.iter().enumerate().find(|(_, entry)| {
                                entry.elim_ordinal == ordinal
                                    && entry.method_ordinal == method_ordinal
                            })
                        {
                            seen[plan_index] = true;
                            index_refinement_sentinel(entry.sentinel_region, slot_base + plan_index)
                        } else if repairs.contains(&(ordinal, method_ordinal)) {
                            let repaired_ty = env.inductive(*fam).and_then(|ind| {
                                method_type(env, ind, method_ordinal, &motive, &params, level_args)
                                    .ok()
                            });
                            repaired_ty
                                .as_ref()
                                .and_then(|expected| {
                                    repair_embedded_method_from_ih(env, ctx, &rewritten, expected)
                                })
                                .unwrap_or(rewritten)
                        } else {
                            rewritten
                        }
                    })
                    .collect();
                let indices = indices
                    .iter()
                    .map(|term| recur(term, next_elim, seen))
                    .collect();
                let scrut = Box::new(recur(scrut, next_elim, seen));
                Term::Elim {
                    fam: *fam,
                    level_args: level_args.clone(),
                    params,
                    motive,
                    methods,
                    indices,
                    scrut,
                }
            }
            Term::Pi(a, b) => Term::pi(recur(a, next_elim, seen), recur(b, next_elim, seen)),
            Term::Lam(a, b) => Term::lam(recur(a, next_elim, seen), recur(b, next_elim, seen)),
            Term::Sigma(a, b) => Term::sigma(recur(a, next_elim, seen), recur(b, next_elim, seen)),
            Term::Let { ty, val, body } => Term::Let {
                ty: Box::new(recur(ty, next_elim, seen)),
                val: Box::new(recur(val, next_elim, seen)),
                body: Box::new(recur(body, next_elim, seen)),
            },
            Term::App(f, a) => Term::app(recur(f, next_elim, seen), recur(a, next_elim, seen)),
            Term::Pair(a, b) => Term::pair(recur(a, next_elim, seen), recur(b, next_elim, seen)),
            Term::Proj1(p) => Term::proj1(recur(p, next_elim, seen)),
            Term::Proj2(p) => Term::proj2(recur(p, next_elim, seen)),
            Term::Ascript(t, a) => Term::Ascript(
                Box::new(recur(t, next_elim, seen)),
                Box::new(recur(a, next_elim, seen)),
            ),
            Term::Eq(a, t, u) => Term::Eq(
                Box::new(recur(a, next_elim, seen)),
                Box::new(recur(t, next_elim, seen)),
                Box::new(recur(u, next_elim, seen)),
            ),
            Term::Cast(a, b, e, t) => Term::Cast(
                Box::new(recur(a, next_elim, seen)),
                Box::new(recur(b, next_elim, seen)),
                Box::new(recur(e, next_elim, seen)),
                Box::new(recur(t, next_elim, seen)),
            ),
            Term::J(m, d, e) => Term::J(
                Box::new(recur(m, next_elim, seen)),
                Box::new(recur(d, next_elim, seen)),
                Box::new(recur(e, next_elim, seen)),
            ),
            Term::Quot(a, r) => Term::Quot(
                Box::new(recur(a, next_elim, seen)),
                Box::new(recur(r, next_elim, seen)),
            ),
            Term::QuotClass(t) => Term::QuotClass(Box::new(recur(t, next_elim, seen))),
            Term::Trunc(a) => Term::Trunc(Box::new(recur(a, next_elim, seen))),
            Term::TruncProj(t) => Term::TruncProj(Box::new(recur(t, next_elim, seen))),
            Term::Refl(t) => Term::Refl(Box::new(recur(t, next_elim, seen))),
            Term::QuotElim {
                motive,
                method,
                respect,
                scrut,
            } => Term::QuotElim {
                motive: Box::new(recur(motive, next_elim, seen)),
                method: Box::new(recur(method, next_elim, seen)),
                respect: Box::new(recur(respect, next_elim, seen)),
                scrut: Box::new(recur(scrut, next_elim, seen)),
            },
            Term::Absurd(motive, proof) => Term::Absurd(
                Box::new(recur(motive, next_elim, seen)),
                Box::new(recur(proof, next_elim, seen)),
            ),
            Term::Type(_)
            | Term::Omega(_)
            | Term::Var(_)
            | Term::Const { .. }
            | Term::IndFormer { .. }
            | Term::Constructor { .. }
            | Term::IntLit(_) => term.clone(),
        }
    }

    let mut next_elim = 0;
    let mut seen = vec![false; plan.len()];
    let rewritten = go(
        env,
        ctx,
        term,
        plan,
        repairs,
        slot_base,
        &mut next_elim,
        &mut seen,
    );
    if let Some(missing) = seen.iter().position(|seen| !seen) {
        return Err(ElabError::Internal(format!(
            "coherent-frame convoy did not reach planned embedded method {missing}"
        )));
    }
    Ok(rewritten)
}

/// Re-synthesize the generated top equality spine of every completed embedded
/// eliminator after its actual indices have moved into the convoy frame. This is
/// paired with `install_embedded_method_sentinels`: refreshing only this witness
/// is the partial-convoy bug, while refreshing it together with every invalid
/// method companion yields a well-typed eliminator at the generalized index.
#[inline(never)]
fn refresh_embedded_elim_evidence(
    env: &GlobalEnv,
    ctx: &Context,
    term: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let go = |term: &Term, ctx: &Context| refresh_embedded_elim_evidence(env, ctx, term, span);
    match term {
        Term::App(f, a) => {
            let rebuilt = Term::app(go(f, ctx)?, go(a, ctx)?);
            let (head, mut args) = peel_app(&rebuilt);
            let Term::Elim {
                fam,
                params,
                indices,
                ..
            } = &head
            else {
                return Ok(rebuilt);
            };
            let ind = env.inductive(*fam).ok_or_else(|| {
                ElabError::Internal(format!(
                    "coherent-frame convoy could not find embedded family {fam:?}"
                ))
            })?;
            let premises = method_index_premises(ind, params, indices, indices, 0);
            if args.len() < premises.len() {
                return Ok(rebuilt);
            }
            for (argument, premise) in args.iter_mut().zip(&premises) {
                *argument = synth_generated_index_evidence(env, ctx, premise, span)?;
            }
            Ok(args.into_iter().fold(head, Term::app))
        }
        Term::Pi(a, b) => {
            let a = go(a, ctx)?;
            let mut body_ctx = ctx.clone();
            body_ctx.push(a.clone());
            Ok(Term::pi(a, go(b, &body_ctx)?))
        }
        Term::Lam(a, b) => {
            let a = go(a, ctx)?;
            let mut body_ctx = ctx.clone();
            body_ctx.push(a.clone());
            Ok(Term::lam(a, go(b, &body_ctx)?))
        }
        Term::Sigma(a, b) => {
            let a = go(a, ctx)?;
            let mut body_ctx = ctx.clone();
            body_ctx.push(a.clone());
            Ok(Term::sigma(a, go(b, &body_ctx)?))
        }
        Term::Let { ty, val, body } => {
            let ty = go(ty, ctx)?;
            let val = go(val, ctx)?;
            let mut body_ctx = ctx.clone();
            body_ctx.push(ty.clone());
            Ok(Term::Let {
                ty: Box::new(ty),
                val: Box::new(val),
                body: Box::new(go(body, &body_ctx)?),
            })
        }
        Term::Pair(a, b) => Ok(Term::pair(go(a, ctx)?, go(b, ctx)?)),
        Term::Proj1(p) => Ok(Term::proj1(go(p, ctx)?)),
        Term::Proj2(p) => Ok(Term::proj2(go(p, ctx)?)),
        Term::Ascript(t, a) => Ok(Term::Ascript(Box::new(go(t, ctx)?), Box::new(go(a, ctx)?))),
        Term::Eq(a, t, u) => Ok(Term::Eq(
            Box::new(go(a, ctx)?),
            Box::new(go(t, ctx)?),
            Box::new(go(u, ctx)?),
        )),
        Term::Cast(a, b, e, t) => Ok(Term::Cast(
            Box::new(go(a, ctx)?),
            Box::new(go(b, ctx)?),
            Box::new(go(e, ctx)?),
            Box::new(go(t, ctx)?),
        )),
        Term::J(m, d, e) => Ok(Term::J(
            Box::new(go(m, ctx)?),
            Box::new(go(d, ctx)?),
            Box::new(go(e, ctx)?),
        )),
        Term::Quot(a, r) => Ok(Term::Quot(Box::new(go(a, ctx)?), Box::new(go(r, ctx)?))),
        Term::QuotClass(t) => Ok(Term::QuotClass(Box::new(go(t, ctx)?))),
        Term::Trunc(a) => Ok(Term::Trunc(Box::new(go(a, ctx)?))),
        Term::TruncProj(t) => Ok(Term::TruncProj(Box::new(go(t, ctx)?))),
        Term::Refl(t) => Ok(Term::Refl(Box::new(go(t, ctx)?))),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => Ok(Term::QuotElim {
            motive: Box::new(go(motive, ctx)?),
            method: Box::new(go(method, ctx)?),
            respect: Box::new(go(respect, ctx)?),
            scrut: Box::new(go(scrut, ctx)?),
        }),
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => Ok(Term::Elim {
            fam: *fam,
            level_args: level_args.clone(),
            params: params
                .iter()
                .map(|term| go(term, ctx))
                .collect::<Result<Vec<_>, _>>()?,
            motive: Box::new(go(motive, ctx)?),
            methods: methods
                .iter()
                .map(|term| go(term, ctx))
                .collect::<Result<Vec<_>, _>>()?,
            indices: indices
                .iter()
                .map(|term| go(term, ctx))
                .collect::<Result<Vec<_>, _>>()?,
            scrut: Box::new(go(scrut, ctx)?),
        }),
        Term::Absurd(motive, proof) => Ok(Term::Absurd(
            Box::new(go(motive, ctx)?),
            Box::new(go(proof, ctx)?),
        )),
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => Ok(term.clone()),
    }
}

/// Does a return telescope contain an argument type coupled to `target`?
/// Only Pi domains count: an occurrence solely in the terminal goal belongs to
/// the existing equality/index-equation path, not the telescope-argument class.
fn motive_return_telescope_argument_occurs(
    env: &GlobalEnv,
    ctx: &Context,
    term: &Term,
    target: &Term,
) -> bool {
    match whnf(env, ctx, term) {
        Term::Pi(domain, codomain) => {
            if scrut_occurs(&domain, target) {
                return true;
            }
            let mut codomain_ctx = ctx.clone();
            codomain_ctx.push((*domain).clone());
            motive_return_telescope_argument_occurs(
                env,
                &codomain_ctx,
                &codomain,
                &weaken(target, 1),
            )
        }
        _ => false,
    }
}

/// Build a branch-specific coherent-frame motive by large-eliminating the one
/// generalized index before introducing the matched-family value. Conflict
/// constructors compute to a J-free `Bottom -> Top`; compatible constructors
/// compute to the usual generated equality premise followed by a transport
/// built only from that branch's already-peeled equality leaves. A forced
/// scrutinee index may additionally rebase coupled return-telescope arguments
/// into the constructor's predecessor frame before method construction.
fn build_index_equation_convoy_body(
    env: &GlobalEnv,
    outer_ctx: &Context,
    motive_ctx: &Context,
    ind: &InductiveDecl,
    params: &[Term],
    scrut_core: &Term,
    scrut_indices: &[Term],
    original_expected: &Term,
    motive_base_depth: usize,
    context_convoy: &[ConvoyEntry],
    sentinel_region: usize,
) -> Result<Option<Term>, ElabError> {
    if ind.indices.len() != 1 || scrut_indices.len() != 1 {
        return Ok(None);
    }
    let goal_head = whnf(env, outer_ctx, original_expected);
    let (return_telescope, goal_level) = match goal_head {
        Term::Eq(goal_ty, _, _) => {
            let Term::Type(goal_level) = whnf(
                env,
                outer_ctx,
                &kernel_infer(env, outer_ctx, &goal_ty).map_err(|error| {
                    ElabError::Internal(format!(
                        "large index convoy could not classify its equality carrier: {error:?}"
                    ))
                })?,
            ) else {
                return Ok(None);
            };
            (false, goal_level)
        }
        Term::Pi(_, _) => {
            let Term::Omega(goal_level) = whnf(
                env,
                outer_ctx,
                &kernel_infer(env, outer_ctx, original_expected).map_err(|error| {
                    ElabError::Internal(format!(
                        "large index convoy could not classify its return telescope: {error:?}"
                    ))
                })?,
            ) else {
                return Ok(None);
            };
            (true, goal_level)
        }
        _ => return Ok(None),
    };
    let goal_sort = Term::Omega(goal_level);

    let index_ty = subst_outer(&ind.indices[0], ind.params.len(), params, 0);
    let (index_head, index_params) = peel_app(&whnf(env, outer_ctx, &index_ty));
    let Term::IndFormer {
        id: index_family,
        level_args: index_level_args,
    } = index_head
    else {
        return Ok(None);
    };
    let index_ind = env
        .inductive(index_family)
        .ok_or_else(|| ElabError::Internal("large index convoy lost its index family".into()))?;
    if !index_ind.indices.is_empty() {
        return Ok(None);
    }

    let matched_at = |depth: usize, index: Term| {
        let mut family = Term::IndFormer {
            id: ind.id,
            level_args: Vec::new(),
        };
        for param in params {
            family = Term::app(family, weaken(param, depth as i64));
        }
        Term::app(family, index)
    };

    // N(r) := D params r -> goal-sort. Eliminating r computes a predicate
    // over the eventual D-value, so no value of D p is captured while the
    // index eliminator itself refines p to a constructor.
    let mut index_binder_ctx = outer_ctx.clone();
    index_binder_ctx.push(index_ty.clone());
    let predicate_body = Term::pi(matched_at(1, Term::var(0)), weaken(&goal_sort, 1));
    let predicate_sort =
        kernel_infer(env, &index_binder_ctx, &predicate_body).map_err(|error| {
            ElabError::Internal(format!(
                "large index convoy predicate is ill-typed: {error:?}"
            ))
        })?;
    let selector_motive = Term::Ascript(
        Box::new(Term::lam(index_ty.clone(), predicate_body)),
        Box::new(Term::pi(index_ty.clone(), predicate_sort)),
    );

    let mut selector_methods = Vec::with_capacity(index_ind.constructors.len());
    for (constructor_ordinal, constructor) in index_ind.constructors.iter().enumerate() {
        let index_method_ty = method_type(
            env,
            index_ind,
            constructor_ordinal,
            &selector_motive,
            &index_params,
            &index_level_args,
        )
        .map_err(|error| {
            ElabError::Internal(format!(
                "large index convoy could not form an index method: {error:?}"
            ))
        })?;
        let recursive = recursive_shapes(env, constructor, index_family, index_ind.params.len())
            .map_err(|error| {
                ElabError::Internal(format!(
                    "large index convoy could not classify index recursion: {error:?}"
                ))
            })?;
        let method_binder_count = constructor.args.len() + recursive.len();
        let mut method_ctx = outer_ctx.clone();
        let mut cursor = index_method_ty.clone();
        let mut method_domains = Vec::with_capacity(method_binder_count);
        for _ in 0..method_binder_count {
            let Term::Pi(domain, codomain) = whnf(env, &method_ctx, &cursor) else {
                return Err(ElabError::Internal(
                    "large index convoy index method lost its binder telescope".into(),
                ));
            };
            let domain = *domain;
            method_ctx.push(domain.clone());
            method_domains.push(domain);
            cursor = *codomain;
        }
        let Term::Pi(value_domain, _) = whnf(env, &method_ctx, &cursor) else {
            return Err(ElabError::Internal(
                "large index convoy index method did not return a predicate".into(),
            ));
        };
        let value_domain = *value_domain;
        method_ctx.push(value_domain.clone());

        let mut target = Term::Constructor {
            id: constructor.id,
            level_args: index_level_args.clone(),
        };
        for param in &index_params {
            target = Term::app(target, weaken(param, method_binder_count as i64));
        }
        for argument in 0..constructor.args.len() {
            target = Term::app(
                target,
                Term::var(recursive.len() + constructor.args.len() - 1 - argument),
            );
        }
        let raw_evidence = Term::Eq(
            Box::new(weaken(&index_ty, (method_binder_count + 1) as i64)),
            Box::new(weaken(&target, 1)),
            Box::new(weaken(&scrut_indices[0], (method_binder_count + 1) as i64)),
        );
        let evidence_shape = whnf(env, &method_ctx, &raw_evidence);
        let conflicting_equation = matches!(
            evidence_shape,
            Term::Const { id, .. } if id == env.bottom_id()
        );
        let mut evidence_ctx = method_ctx.clone();
        let leaves = if conflicting_equation {
            Vec::new()
        } else {
            evidence_ctx.push(raw_evidence.clone());
            project_generated_index_equality_leaves(
                env,
                &evidence_ctx,
                &weaken(&raw_evidence, 1),
                Term::var(0),
            )?
        };
        let depth = method_binder_count + 2;
        let expected_at_depth = weaken(original_expected, depth as i64);
        let telescope_argument_substitutions = if conflicting_equation {
            Vec::new()
        } else {
            leaves
                .iter()
                .filter(|leaf| {
                    motive_return_telescope_argument_occurs(
                        env,
                        &evidence_ctx,
                        &expected_at_depth,
                        &leaf.scrutinee,
                    )
                })
                .map(|leaf| (leaf.scrutinee.clone(), leaf.target.clone()))
                .collect()
        };
        let carried = carry_coherent_frame_data(
            leaves,
            raw_evidence,
            telescope_argument_substitutions,
        );
        let raw_evidence = carried.index_equation.ok_or_else(|| {
            ElabError::Internal("coherent-frame index equation was not carried".into())
        })?;
        let branch_result = if conflicting_equation {
            let impossible_result = if return_telescope {
                expected_at_depth.clone()
            } else {
                Term::Const {
                    id: env.top_id(),
                    level_args: Vec::new(),
                }
            };
            Term::pi(raw_evidence, impossible_result)
        } else {
            let goal = if return_telescope {
                if carried.telescope_argument_substitutions.is_empty() {
                    return Ok(None);
                }
                let telescope_substitutions = carried.telescope_argument_substitutions;
                let mut goal_substitutions = telescope_substitutions.clone();
                goal_substitutions.push((weaken(scrut_core, depth as i64), Term::var(1)));
                let mut goal =
                    subst_term_generalize_many(&expected_at_depth, &goal_substitutions);

                // Ambient context-convoy entries are the already-bound half of
                // the same telescope class. Rebind them inside the selected
                // proposition, refine their types with the same equality-leaf
                // substitutions, and redirect every goal occurrence to the
                // new binders. Constructor extraction later peels these hidden
                // Pis back into the method premise telescope.
                if !context_convoy.is_empty() {
                    let target_index = subst_term_generalize_many(
                        &weaken(&scrut_indices[0], depth as i64),
                        &telescope_substitutions,
                    );
                    let (types_inner_first, sentinels) = convoy_binder_types(
                        context_convoy,
                        scrut_indices,
                        scrut_core,
                        &[target_index],
                        &Term::var(1),
                        depth,
                        0,
                        sentinel_region,
                    );
                    let types_inner_first: Vec<Term> = types_inner_first
                        .iter()
                        .map(|ty| subst_term_generalize_many(ty, &telescope_substitutions))
                        .collect();
                    goal = redirect_convoy_body(context_convoy, depth, &sentinels, goal);
                    let mut premises = Vec::with_capacity(context_convoy.len());
                    for index in (0..context_convoy.len()).rev() {
                        premises.push(types_inner_first[index].clone());
                    }
                    goal = wrap_premise_pis_finalized(goal, &premises, sentinel_region);
                }
                goal
            } else {
                let mut value = Term::var(1);
                let mut value_ty = weaken(&value_domain, 2);
                let mut changed = false;
                for leaf in carried.equality_leaves {
                    let Term::Type(index_level) = whnf(
                        env,
                        &evidence_ctx,
                        &kernel_infer(env, &evidence_ctx, &leaf.index_ty).map_err(|error| {
                            ElabError::Internal(format!(
                                "large index convoy could not classify a peeled index: {error:?}"
                            ))
                        })?,
                    ) else {
                        return Ok(None);
                    };
                    let reverse = build_sym(
                        env,
                        &evidence_ctx,
                        &leaf.index_ty,
                        index_level.clone(),
                        &leaf.target,
                        leaf.proof,
                    );
                    let stable_forward = build_sym(
                        env,
                        &evidence_ctx,
                        &leaf.index_ty,
                        index_level,
                        &leaf.scrutinee,
                        reverse,
                    );
                    if let Some((cast, cast_ty)) = try_reindex_cast(
                        env,
                        &evidence_ctx,
                        &leaf.index_ty,
                        &leaf.target,
                        &leaf.scrutinee,
                        &value_ty,
                        value.clone(),
                        stable_forward,
                    )? {
                        value = cast;
                        value_ty = cast_ty;
                        changed = true;
                    }
                }
                if !changed {
                    return Ok(None);
                }
                subst_term_generalize(
                    &expected_at_depth,
                    &weaken(scrut_core, depth as i64),
                    &value,
                )
            };
            kernel_infer(env, &evidence_ctx, &goal).map_err(|error| {
                ElabError::Internal(format!(
                    "large index convoy successor goal is ill-typed: {error:?}"
                ))
            })?;
            Term::pi(raw_evidence, goal)
        };

        let mut method = Term::lam(value_domain, branch_result);
        for domain in method_domains.into_iter().rev() {
            method = Term::lam(domain, method);
        }
        kernel_check(env, outer_ctx, &method, &index_method_ty).map_err(|error| {
            ElabError::Internal(format!(
                "large index convoy constructed an ill-typed index method: {error:?}"
            ))
        })?;
        selector_methods.push(method);
    }

    let selector_at_index = Term::Elim {
        fam: index_family,
        level_args: index_level_args,
        params: index_params.iter().map(|param| weaken(param, 1)).collect(),
        motive: Box::new(weaken(&selector_motive, 1)),
        methods: selector_methods
            .iter()
            .map(|method| weaken(method, 1))
            .collect(),
        indices: Vec::new(),
        scrut: Box::new(Term::var(0)),
    };
    let selector_ty = Term::pi(
        index_ty.clone(),
        Term::pi(matched_at(1, Term::var(0)), weaken(&goal_sort, 1)),
    );
    let selector = Term::Ascript(
        Box::new(Term::lam(index_ty, selector_at_index)),
        Box::new(selector_ty),
    );
    let body = Term::app(
        Term::app(weaken(&selector, motive_base_depth as i64), Term::var(1)),
        Term::var(0),
    );
    kernel_infer(env, motive_ctx, &body).map_err(|error| {
        ElabError::Internal(format!(
            "large index convoy constructed an ill-typed shared motive: {error:?}"
        ))
    })?;
    Ok(Some(body))
}

fn has_nat_shaped_index(
    env: &GlobalEnv,
    ctx: &Context,
    ind: &InductiveDecl,
    params: &[Term],
) -> bool {
    if ind.indices.len() != 1 {
        return false;
    }
    let index_ty = subst_outer(&ind.indices[0], ind.params.len(), params, 0);
    let (head, index_params) = peel_app(&whnf(env, ctx, &index_ty));
    let Term::IndFormer { id, .. } = head else {
        return false;
    };
    let Some(index_ind) = env.inductive(id) else {
        return false;
    };
    if !index_params.is_empty()
        || !index_ind.params.is_empty()
        || !index_ind.indices.is_empty()
        || index_ind.constructors.len() != 2
    {
        return false;
    }
    let mut has_zero = false;
    let mut has_successor = false;
    for ctor in &index_ind.constructors {
        let Ok(recursive) = recursive_shapes(env, ctor, id, 0) else {
            return false;
        };
        has_zero |= ctor.args.is_empty() && recursive.is_empty();
        has_successor |= ctor.args.len() == 1 && recursive.len() == 1;
    }
    has_zero && has_successor
}

/// Classify where a recursive field's index comes from before motive planning.
///
/// When a direct recursive field is declared at an index different from the
/// constructor result and there is no captured sibling sharing that index, the
/// ordinary eliminator supplies its IH by applying the motive at the field's
/// declared index. A sibling convoy, a forced/non-variable actual index, or any
/// other shape stays on the established coupled-refinement path.
fn recursive_field_index_path(
    cx: &ElabCtx,
    ind: &InductiveDecl,
    params: &[Term],
    level_args: &[Level],
    scrut_core: &Term,
    scrut_indices: &[Term],
) -> Result<RecursiveFieldIndexPath, ElabError> {
    if ind.indices.len() != 1
        || scrut_indices.len() != 1
        || !matches!(scrut_core, Term::Var(_))
        || !matches!(scrut_indices[0], Term::Var(_))
        || !compute_context_convoy(
            &cx.ctx,
            scrut_core,
            scrut_indices,
            &cx.match_field_regions,
        )
        .is_empty()
    {
        return Ok(RecursiveFieldIndexPath::CoupledRefinement);
    }

    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|term| cx.metas.zonk_term(term)).collect(),
    };
    let index_ty = cx.metas.zonk_term(&subst_levels(
        &subst_outer(&ind.indices[0], ind.params.len(), params, 0),
        &ind.level_params,
        level_args,
    ));
    for ctor in &ind.constructors {
        let field_count = ctor.args.len();
        let mut branch_ctx = zonked_ctx.clone();
        for field in 0..field_count {
            branch_ctx.push(cx.metas.zonk_term(&subst_levels(
                &subst_outer(&ctor.args[field], ind.params.len(), params, field),
                &ind.level_params,
                level_args,
            )));
        }
        let target_indices = ctor_target_indices(
            ctor,
            ind,
            params,
            level_args,
            field_count,
        )
        .into_iter()
        .map(|term| cx.metas.zonk_term(&term))
        .collect::<Vec<_>>();
        let shapes = recursive_shapes(cx.env, ctor, ind.id, ind.params.len()).map_err(|error| {
            ElabError::Internal(format!(
                "declared-index recursive-field classification failed: {error:?}"
            ))
        })?;
        for shape in shapes {
            let Some((branching_telescope, declared_indices)) = shape.shape.as_legacy() else {
                continue;
            };
            if !branching_telescope.is_empty() || declared_indices.len() != 1 {
                continue;
            }
            let declared_index = cx.metas.zonk_term(&ken_kernel::subst::shift(
                &subst_levels(
                    &subst_outer(
                        &declared_indices[0],
                        ind.params.len(),
                        params,
                        shape.position,
                    ),
                    &ind.level_params,
                    level_args,
                ),
                (field_count - shape.position) as i64,
                0,
            ));
            let branch_index_ty = weaken(&index_ty, field_count as i64);
            if !convert(
                cx.env,
                &branch_ctx,
                &branch_index_ty,
                &declared_index,
                &target_indices[0],
            ) {
                return Ok(RecursiveFieldIndexPath::PlainDeclared);
            }
        }
    }
    Ok(RecursiveFieldIndexPath::CoupledRefinement)
}

#[allow(clippy::too_many_arguments)]
fn ordinary_coherent_frame_plan(
    cx: &ElabCtx,
    scrut_core: &Term,
    scrut_indices: &[Term],
    original_expected: &Term,
    motive_base_depth: usize,
    motive_local_indices: &[Term],
    recursive_field_index_path: RecursiveFieldIndexPath,
) -> Box<CoherentFrameMotivePlan> {
    let context_convoy = compute_context_convoy(
        &cx.ctx,
        scrut_core,
        scrut_indices,
        &cx.match_field_regions,
    );
    let motive_rebase = dependent_rebase_subs(
        scrut_core,
        scrut_indices,
        motive_base_depth as i64,
        motive_local_indices,
        &Term::var(0),
        original_expected,
        recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared
            || context_convoy
                .iter()
                .any(|entry| scrut_occurs(original_expected, &Term::var(entry.var))),
    );
    Box::new(CoherentFrameMotivePlan {
        expected: original_expected.clone(),
        context_convoy,
        embedded_method_convoy: Vec::new(),
        embedded_method_repairs: Vec::new(),
        motive_user_body: subst_term_generalize_many(
            &weaken(original_expected, motive_base_depth as i64),
            &motive_rebase,
        ),
        equation_convoy: false,
        recursive_field_index_path,
    })
}

#[inline(never)]
fn boxed_simplify_branch_goal(env: &GlobalEnv, ctx: &Context, term: &Term) -> Box<Term> {
    Box::new(simplify_branch_goal(env, ctx, term))
}

#[inline(never)]
fn plan_coherent_frame_motive(
    cx: &ElabCtx,
    ind: &InductiveDecl,
    params_terms: &[Term],
    scrut_core: &Term,
    scrut_indices: &[Term],
    original_expected: &Term,
    motive_base_depth: usize,
    motive_local_indices: &[Term],
    recursive_field_index_path: RecursiveFieldIndexPath,
    sentinel_region: usize,
    defer_coupled_expansion: bool,
    span: &Span,
) -> Result<Box<CoherentFrameMotivePlan>, ElabError> {
    let zonked_ctx = Context {
        types: cx
            .ctx
            .types
            .iter()
            .map(|term| cx.metas.zonk_term(term))
            .collect(),
    };
    let motive_ctx = motive_context(&zonked_ctx, ind, params_terms);
    if recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared
        || !has_nat_shaped_index(cx.env, &zonked_ctx, ind, params_terms)
    {
        return Ok(ordinary_coherent_frame_plan(
            cx,
            scrut_core,
            scrut_indices,
            original_expected,
            motive_base_depth,
            motive_local_indices,
            recursive_field_index_path,
        ));
    }
    let expanded_expected = boxed_simplify_branch_goal(cx.env, &zonked_ctx, original_expected);
    let has_nontrivial_coupled_sides = matches!(original_expected, Term::Eq(_, _, _))
        || matches!(expanded_expected.as_ref(), Term::Eq(_, _, _));
    let probe_context_convoy =
        compute_context_convoy(&cx.ctx, scrut_core, scrut_indices, &cx.match_field_regions);

    // A nested covering is already inside a constructor-refined outer frame.
    // Its motive convoys the generated index equation and transports the inner
    // scrutinee back to that concrete outer index. No concrete sibling is ever
    // placed at the generalized index. The outer match itself stays opaque and
    // uses its ordinary context telescope so the nested covering is reached
    // only after the outer constructor has been selected.
    let forced_telescope_convoy = matches!(original_expected, Term::Pi(_, _))
        && scrut_indices
            .iter()
            .any(|index| !matches!(index, Term::Var(_)));
    let equation_convoy = (!cx.match_field_regions.is_empty()
        && has_nontrivial_coupled_sides)
        || forced_telescope_convoy;
    if equation_convoy {
        if !probe_context_convoy.is_empty() && !forced_telescope_convoy {
            return Err(ElabError::Internal(
                "index-equation convoy unexpectedly overlaps an ambient context convoy".into(),
            ));
        }
        if let Some(motive_user_body) = build_index_equation_convoy_body(
            cx.env,
            &zonked_ctx,
            &motive_ctx,
            ind,
            params_terms,
            scrut_core,
            scrut_indices,
            original_expected,
            motive_base_depth,
            &probe_context_convoy,
            sentinel_region,
        )? {
            return Ok(Box::new(CoherentFrameMotivePlan {
                expected: original_expected.clone(),
                context_convoy: probe_context_convoy.clone(),
                embedded_method_convoy: Vec::new(),
                embedded_method_repairs: Vec::new(),
                motive_user_body,
                equation_convoy: true,
                recursive_field_index_path,
            }));
        }
    }

    let planning_expected = if defer_coupled_expansion && has_nontrivial_coupled_sides {
        original_expected
    } else {
        &expanded_expected
    };
    let probe_rebase = dependent_rebase_subs(
        scrut_core,
        scrut_indices,
        motive_base_depth as i64,
        motive_local_indices,
        &Term::var(0),
        planning_expected,
        probe_context_convoy
            .iter()
            .any(|entry| scrut_occurs(planning_expected, &Term::var(entry.var))),
    );
    let probe_body = subst_term_generalize_many(
        &weaken(planning_expected, motive_base_depth as i64),
        &probe_rebase,
    );
    let (embedded_method_convoy, embedded_method_repairs) = if !(defer_coupled_expansion
        && has_nontrivial_coupled_sides)
        && has_nontrivial_coupled_sides
        && probe_context_convoy.is_empty()
    {
        plan_embedded_method_convoy(
            cx.env,
            &zonked_ctx,
            &motive_ctx,
            planning_expected,
            &probe_body,
            sentinel_region,
        )?
    } else {
        (Vec::new(), Vec::new())
    };
    let expected = if embedded_method_convoy.is_empty() && embedded_method_repairs.is_empty() {
        original_expected.clone()
    } else {
        planning_expected.clone()
    };
    let context_convoy = if embedded_method_convoy.is_empty() && embedded_method_repairs.is_empty()
    {
        compute_context_convoy(&cx.ctx, scrut_core, scrut_indices, &cx.match_field_regions)
    } else {
        probe_context_convoy
    };
    let motive_rebase = dependent_rebase_subs(
        scrut_core,
        scrut_indices,
        motive_base_depth as i64,
        motive_local_indices,
        &Term::var(0),
        &expected,
        context_convoy
            .iter()
            .any(|entry| scrut_occurs(&expected, &Term::var(entry.var))),
    );
    let mut motive_user_body =
        subst_term_generalize_many(&weaken(&expected, motive_base_depth as i64), &motive_rebase);
    if !embedded_method_convoy.is_empty() || !embedded_method_repairs.is_empty() {
        motive_user_body = install_embedded_method_sentinels(
            cx.env,
            &motive_ctx,
            &motive_user_body,
            &embedded_method_convoy,
            &embedded_method_repairs,
            context_convoy.len(),
        )?;
        motive_user_body =
            refresh_embedded_elim_evidence(cx.env, &motive_ctx, &motive_user_body, span)?;
    }
    Ok(Box::new(CoherentFrameMotivePlan {
        expected,
        context_convoy,
        embedded_method_convoy,
        embedded_method_repairs,
        motive_user_body,
        equation_convoy: false,
        recursive_field_index_path,
    }))
}

/// Compute the ordered ambient-context convoy for a dependent match: the
/// transitive forward-dependency closure of genuine ambient bindings whose type
/// the root substitution (or an already-included convoy binder) changes.
/// Returned innermost-first (ascending de Bruijn `var`); the motive telescope
/// wraps them outermost-first. `scrut_core` and the scrutinee's own binder are
/// excluded (the motive already abstracts the scrutinee). Bindings inside an
/// enclosing match's field region are excluded (they are that match's fields,
/// not outer captured binders).
fn compute_context_convoy(
    ctx: &Context,
    scrut_core: &Term,
    scrut_indices: &[Term],
    match_field_regions: &[std::ops::Range<usize>],
) -> Vec<ConvoyEntry> {
    let depth = ctx.len();
    // The dependency seed: the free variables occurring in the scrutinee's
    // actual indices, as de Bruijn indices at the ambient depth. A binding is
    // convoyed when its type mentions one of these (or a var added below).
    let mut dep_vars: Vec<usize> = Vec::new();
    for idx in scrut_indices {
        for v in 0..depth {
            if scrut_occurs(idx, &Term::var(v)) && !dep_vars.contains(&v) {
                dep_vars.push(v);
            }
        }
    }
    let scrut_var = match scrut_core {
        Term::Var(v) => Some(*v),
        _ => None,
    };
    let mut convoy: Vec<ConvoyEntry> = Vec::new();
    // Walk bindings OUTERMOST (highest var) to innermost. A binding's type can
    // only mention OUTER bindings (higher var, bound earlier), so processing
    // outer-first guarantees every dependency a binding could name has ALREADY
    // entered `dep_vars` before that binding is tested — that is what makes this
    // the transitive forward-dependency closure, in ONE pass. Innermost-first is
    // WRONG: an inner `h : p xs` mentions the captured `xs` WITHOUT mentioning
    // the index `n`, so inner-first would test `h` before `xs` joined `dep_vars`
    // and drop it. Collected outermost-first here, then reversed to the
    // innermost-first representation the motive/method telescope consumes.
    for var in (0..depth).rev() {
        // Skip the scrutinee itself — the motive abstracts it directly.
        if scrut_var == Some(var) {
            continue;
        }
        // Skip an enclosing match's field: a bottom-relative position in a
        // recorded field region is not a genuine outer captured binder.
        let bottom_pos = depth - 1 - var;
        if match_field_regions.iter().any(|r| r.contains(&bottom_pos)) {
            continue;
        }
        // The binding's stored type, weakened to be valid at the ambient depth.
        let raw_ty = match ctx.lookup(var) {
            Some(t) => t.clone(),
            None => continue,
        };
        let ambient_ty = weaken(&raw_ty, (var as i64) + 1);
        let mentions_dep = dep_vars
            .iter()
            .any(|d| scrut_occurs(&ambient_ty, &Term::var(*d)));
        if mentions_dep {
            dep_vars.push(var);
            convoy.push(ConvoyEntry { var, ambient_ty });
        }
    }
    // Collected outermost-first (descending var); the telescope representation
    // is innermost-first (ascending var), so reverse. de Bruijn position is
    // itself a topological order, so this single reversed pass — no fixpoint —
    // yields dependency-consistent innermost-first entries.
    convoy.reverse();
    convoy
}

/// Reduce transparent branch goals enough to expose constructor-local matches
/// in proposition/type positions without recursively normalizing stuck
/// eliminator methods. Full normalisation can chase recursive transparent defs
/// indefinitely under neutral scrutinees; this deliberately stops at stuck
/// `Elim` nodes.
fn simplify_branch_goal(env: &GlobalEnv, ctx: &Context, term: &Term) -> Term {
    match whnf(env, ctx, term) {
        Term::Pi(a, b) => {
            let a_s = simplify_branch_goal(env, ctx, &a);
            let mut ctx2 = ctx.clone();
            ctx2.push(a_s.clone());
            Term::pi(a_s, simplify_branch_goal(env, &ctx2, &b))
        }
        Term::Sigma(a, b) => {
            let a_s = simplify_branch_goal(env, ctx, &a);
            let mut ctx2 = ctx.clone();
            ctx2.push(a_s.clone());
            Term::sigma(a_s, simplify_branch_goal(env, &ctx2, &b))
        }
        Term::Eq(a, x, y) => Term::Eq(
            Box::new(simplify_branch_goal(env, ctx, &a)),
            Box::new(simplify_branch_goal(env, ctx, &x)),
            Box::new(simplify_branch_goal(env, ctx, &y)),
        ),
        Term::App(f, a) => Term::app(
            simplify_branch_goal(env, ctx, &f),
            simplify_branch_goal(env, ctx, &a),
        ),
        Term::Ascript(t, a) => Term::Ascript(
            Box::new(simplify_branch_goal(env, ctx, &t)),
            Box::new(simplify_branch_goal(env, ctx, &a)),
        ),
        Term::Cast(a, b, e, t) => Term::Cast(
            Box::new(simplify_branch_goal(env, ctx, &a)),
            Box::new(simplify_branch_goal(env, ctx, &b)),
            Box::new(simplify_branch_goal(env, ctx, &e)),
            Box::new(simplify_branch_goal(env, ctx, &t)),
        ),
        Term::J(motive, base, eq) => Term::J(
            Box::new(simplify_branch_goal(env, ctx, &motive)),
            Box::new(simplify_branch_goal(env, ctx, &base)),
            Box::new(simplify_branch_goal(env, ctx, &eq)),
        ),
        Term::Absurd(motive, proof) => Term::Absurd(
            Box::new(simplify_branch_goal(env, ctx, &motive)),
            Box::new(simplify_branch_goal(env, ctx, &proof)),
        ),
        other => other,
    }
}

fn support_head(env: &GlobalEnv, ctx: &Context, ty: &Term) -> Option<GlobalId> {
    let normalized = whnf(env, ctx, ty);
    let (head, _) = peel_app(&normalized);
    let Term::IndFormer { id, .. } = head else {
        return None;
    };
    env.all_support_origin(id).is_some().then_some(id)
}

/// Specialize the generated recursive-IH evidence before source arguments.
///
/// D0 normalized the held D3 mismatch from
/// `((λ index. λ _ : index = index. FokDerivation index) expected) Refl`
/// versus `FokDerivation concrete_child_sequent` to, respectively,
/// `FokDerivation expected` and `FokDerivation concrete_child_sequent`.
/// Reduction already fired; the source's concrete sequent had been consumed at
/// the generated equality position instead of the first ordinary-argument
/// position. This helper closes that elaborator-side ordering gap.
///
/// It applies only leading equality premises whose endpoints convert, and only
/// until the result type converts to the declared source-call result. A neutral
/// or non-reflexive premise returns the original evidence unchanged so the
/// ordinary path retains the explicit equality/transport rather than inventing
/// `Refl` or erasing a Pi.
fn discharge_reflexive_recursive_ih_evidence(
    env: &GlobalEnv,
    ctx: &Context,
    evidence: Term,
    evidence_ty: Term,
    source_result_ty: &Term,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    if convert_type(env, ctx, &evidence_ty, source_result_ty) {
        return Ok((evidence, evidence_ty));
    }

    let original = (evidence.clone(), evidence_ty.clone());
    let mut specialized = evidence;
    let mut specialized_ty = evidence_ty;
    loop {
        let Term::Pi(domain, codomain) = whnf(env, ctx, &specialized_ty) else {
            return Ok(original);
        };
        let Term::Eq(eq_ty, left, right) = domain.as_ref() else {
            return Ok(original);
        };
        if !convert(env, ctx, eq_ty, left, right) {
            return Ok(original);
        }
        let proof = synth_generated_index_evidence(env, ctx, &domain, span)?;
        specialized_ty = subst0(&codomain, &proof);
        specialized = Term::app(specialized, proof);
        if convert_type(env, ctx, &specialized_ty, source_result_ty) {
            return Ok((specialized, specialized_ty));
        }
    }
}

fn term_mentions_family_indexed_by(
    env: &GlobalEnv,
    ctx: &Context,
    term: &Term,
    index_ty: &Term,
) -> bool {
    let (head, _) = peel_app(term);
    if let Term::IndFormer { id, .. } = head {
        if env.inductive(id).is_some_and(|ind| {
            ind.indices
                .iter()
                .any(|candidate| convert_type(env, ctx, candidate, index_ty))
        }) {
            return true;
        }
    }
    term.children()
        .into_iter()
        .any(|child| term_mentions_family_indexed_by(env, ctx, child, index_ty))
}

fn expression_mentions_recursive_group(cx: &ElabCtx, expr: &RExpr) -> bool {
    cx.globals
        .iter()
        .any(|(name, id)| cx.recursive_group.contains(id) && rexpr_mentions_name(expr, name))
}

fn recursive_group_call_id(cx: &ElabCtx, expr: &RExpr) -> Option<GlobalId> {
    let mut head = expr;
    while let RExpr::RApp(function, _, _) = head {
        head = function.as_ref();
    }
    let RExpr::RCon(name, _) = head else {
        return None;
    };
    let id = cx.globals.get(name).copied()?;
    cx.recursive_group.contains(&id).then_some(id)
}

/// Transport a recursive-group sibling call from the concrete constructor
/// index produced by its declared result to the refined index expected inside
/// the current dependent-match method. The bridge is the method's own hidden
/// propositional equality premise, represented by a sentinel until method
/// finalization. No `Refl` is synthesized here: `try_reindex_cast` builds a
/// genuine equality-of-types proof by `J`, and the kernel re-checks the `Cast`.
fn transport_recursive_group_call_result(
    cx: &ElabCtx,
    expr: &RExpr,
    core: Term,
    inferred_ty: Term,
    expected: &Term,
) -> Result<Option<(Term, Term)>, ElabError> {
    if recursive_group_call_id(cx, expr).is_none()
        || convert_type(cx.env, &cx.ctx, &inferred_ty, expected)
    {
        return Ok(None);
    }

    let mut transported = core;
    let mut transported_ty = inferred_ty;
    let mut changed = false;
    for refinement in cx.result_refinements.iter().rev() {
        let growth = cx
            .ctx
            .len()
            .checked_sub(refinement.install_depth)
            .ok_or_else(|| {
                ElabError::Internal("result refinement escaped its branch context".into())
            })? as i64;
        let index_ty = weaken(&refinement.index_ty, growth);
        let concrete_index = weaken(&refinement.concrete_index, growth);
        let refined_index = weaken(&refinement.refined_index, growth);
        // The placeholder is embedded at the current source-binder depth so
        // `finalize_refined_body` can recover its canonical premise slot.
        let proof = index_refinement_sentinel(
            refinement.sentinel_region,
            refinement.premise_slot + growth as usize,
        );
        if let Some((cast, cast_ty)) = try_reindex_cast(
            cx.env,
            &cx.ctx,
            &index_ty,
            &concrete_index,
            &refined_index,
            &transported_ty,
            transported.clone(),
            proof,
        )? {
            transported = cast;
            transported_ty = cast_ty;
            changed = true;
            if convert_type(cx.env, &cx.ctx, &transported_ty, expected) {
                return Ok(Some((transported, transported_ty)));
            }
        }
    }

    if changed && convert_type(cx.env, &cx.ctx, &transported_ty, expected) {
        Ok(Some((transported, transported_ty)))
    } else {
        Ok(None)
    }
}

#[inline(never)]
fn infer_reflexive_recursive_self_call(
    cx: &mut ElabCtx,
    expr: &RExpr,
    span: &Span,
) -> Result<Option<(Term, Term)>, ElabError> {
    let mut head = expr;
    let mut arguments = Vec::new();
    while let RExpr::RApp(function, argument, _) = head {
        arguments.push(argument.as_ref());
        head = function.as_ref();
    }
    arguments.reverse();
    let (RExpr::RCon(name, _), Some(RExpr::RVar(index, _, _))) = (head, arguments.first().copied())
    else {
        return Ok(None);
    };
    if name != &cx.owner_label {
        return Ok(None);
    }
    let Some((position, _)) = cx.surface_var(*index) else {
        return Ok(None);
    };
    let Some(binding) = cx.lift_bindings.get(&position) else {
        return Ok(None);
    };
    if binding.support.is_some() {
        return Ok(None);
    }
    let Some((evidence, evidence_ty)) = cx.binding_term(binding.evidence_position) else {
        return Ok(None);
    };
    let owner_id = cx
        .globals
        .get(name)
        .copied()
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: name.clone(),
            span: span.clone(),
        })?;
    let (_, owner_ty) = cx
        .env
        .const_type(owner_id)
        .ok_or_else(|| ElabError::Internal(format!("no type for recursive owner '{name}'")))?;
    let Term::Pi(child_domain, owner_codomain) = whnf(cx.env, &cx.ctx, &owner_ty) else {
        return Err(ElabError::NotAFunction { span: span.clone() });
    };
    let child_core = check(cx, arguments[0], &child_domain, span)?;
    let source_result_ty = subst0(&owner_codomain, &child_core);
    let (mut call, mut call_ty) = discharge_reflexive_recursive_ih_evidence(
        cx.env,
        &cx.ctx,
        evidence,
        evidence_ty,
        &source_result_ty,
        span,
    )?;
    if convert_type(cx.env, &cx.ctx, &call_ty, &source_result_ty) {
        for argument in arguments.iter().skip(1) {
            let Term::Pi(domain, codomain) = whnf(cx.env, &cx.ctx, &call_ty) else {
                return Err(ElabError::NotAFunction { span: span.clone() });
            };
            let argument_core = check(cx, argument, &domain, span)?;
            call_ty = subst0(&codomain, &argument_core);
            call = Term::app(call, argument_core);
        }
        return Ok(Some((call, call_ty)));
    }
    if arguments.len() == 1 {
        return Ok(Some((call, call_ty)));
    }
    Ok(None)
}

fn install_lift_binding(
    cx: &mut ElabCtx,
    source_position: usize,
    evidence_position: usize,
    recursive_result_position: Option<usize>,
) -> Result<LiftBinding, ElabError> {
    let (_, evidence_ty) = cx.binding_term(evidence_position).ok_or_else(|| {
        ElabError::Internal("generated lift evidence escaped its method context".into())
    })?;
    let support = support_head(cx.env, &cx.ctx, &evidence_ty);
    let binding = LiftBinding {
        evidence_position,
        recursive_result_position,
        support,
    };
    cx.lift_bindings.insert(source_position, binding);
    Ok(binding)
}

/// Compile a source match whose scrutinee is paired with residual generated
/// `All` evidence. The support constructors are aligned with the host
/// constructors; their leading fields are the source fields and their trailing
/// fields are the exact lifted evidence selected by the kernel producer.
#[allow(clippy::too_many_arguments)]
fn check_match_with_lift(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    expected: &Term,
    span: &Span,
    scrut_core: &Term,
    host: &InductiveDecl,
    host_level_args: &[Level],
    host_params: &[Term],
    binding: LiftBinding,
) -> Result<Term, ElabError> {
    let support = binding
        .support
        .ok_or_else(|| ElabError::Internal("nested match lost residual All evidence".into()))?;
    let (origin, _, _) = cx.env.all_support_origin(support).ok_or_else(|| {
        ElabError::Internal("nested match received foreign generated evidence".into())
    })?;
    if origin != host.id {
        return Err(ElabError::Internal(
            "nested match evidence provenance does not match its source family".into(),
        ));
    }
    let (evidence, evidence_ty) = cx.binding_term(binding.evidence_position).ok_or_else(|| {
        ElabError::Internal("nested match evidence is outside the current context".into())
    })?;
    let evidence_ty = whnf(cx.env, &cx.ctx, &evidence_ty);
    let (support_head_term, support_args) = peel_app(&evidence_ty);
    let (support_id, level_args) = match support_head_term {
        Term::IndFormer { id, level_args } if id == support => (id, level_args),
        _ => {
            return Err(ElabError::Internal(
                "nested match evidence type lost its generated support head".into(),
            ))
        }
    };
    let support_decl = cx
        .env
        .inductive(support_id)
        .ok_or_else(|| ElabError::Internal("generated support declaration is absent".into()))?
        .clone();
    if support_decl.constructors.len() != host.constructors.len()
        || support_args.len() != support_decl.params.len() + support_decl.indices.len()
    {
        return Err(ElabError::Internal(
            "generated support is not aligned with its recorded host".into(),
        ));
    }
    let support_params = support_args[..support_decl.params.len()].to_vec();
    let support_indices = support_args[support_decl.params.len()..].to_vec();

    // The final support index is the literal host source. Generalize the
    // surface goal over that index; the support value itself is motive-irrelevant.
    let motive_depth = support_decl.indices.len() + 1;
    let source_index = Term::var(1);
    let motive_body = subst_term_generalize(
        &weaken(expected, motive_depth as i64),
        &weaken(scrut_core, motive_depth as i64),
        &source_index,
    );
    let motive_ctx = motive_context_at(&cx.ctx, &support_decl, &support_params, &level_args);
    let motive_sort = kernel_infer(cx.env, &motive_ctx, &motive_body).map_err(|error| {
        ElabError::KernelRejected {
            error,
            span: span.clone(),
        }
    })?;
    let motive_ty = motive_type_at(
        &support_decl,
        support_id,
        &support_params,
        &motive_sort,
        &level_args,
    );
    let motive = Term::Ascript(
        Box::new(wrap_motive_lambdas_at(
            &support_decl,
            support_id,
            &support_params,
            motive_body,
            &level_args,
        )),
        Box::new(motive_ty),
    );

    let mut methods = Vec::with_capacity(support_decl.constructors.len());
    let mut arm_used = vec![false; arms.len()];
    let mut subsumed_by: Vec<Option<usize>> = vec![None; arms.len()];
    for (ordinal, support_ctor) in support_decl.constructors.iter().enumerate() {
        let host_ctor = &host.constructors[ordinal];
        let (arm_index, arm) = arms
            .iter()
            .enumerate()
            .find(|(_, arm)| {
                matches!(&arm.pat.kind, RPatKind::Ctor(name, _) if cx.globals.get(name).copied() == Some(host_ctor.id))
            })
            .ok_or_else(|| ElabError::ExhaustivenessError {
                missing: missing_pattern_witness(cx, host_ctor.id),
                span: span.clone(),
            })?;
        arm_used[arm_index] = true;
        mark_shared_ctor_subsumption(cx, arms, host_ctor.id, arm_index, &mut subsumed_by);
        let sub_pats = match &arm.pat.kind {
            RPatKind::Ctor(_, fields) => fields,
            _ => unreachable!("arm selected by constructor guard"),
        };
        if sub_pats.len() != host_ctor.args.len()
            || sub_pats
                .iter()
                .any(|pat| !matches!(pat.kind, RPatKind::Var(_) | RPatKind::Wild))
        {
            return Err(ElabError::Internal(
                "lifted dependent match requires flat source constructor fields".into(),
            ));
        }

        let method_ty = method_type(
            cx.env,
            &support_decl,
            ordinal,
            &motive,
            &support_params,
            &level_args,
        )
        .map_err(|error| ElabError::KernelRejected {
            error,
            span: arm.span.clone(),
        })?;
        let (raw_domains, _) = peel_pi(&method_ty);
        let support_shapes = recursive_shapes(
            cx.env,
            support_ctor,
            support_decl.id,
            support_decl.params.len(),
        )
        .map_err(|error| ElabError::KernelRejected {
            error,
            span: arm.span.clone(),
        })?;
        if raw_domains.len() != support_ctor.args.len() + support_shapes.len() {
            return Err(ElabError::StructuralResultAssociationMissing {
                match_span: span.clone(),
                field_span: arm.pat.span.clone(),
            });
        }
        let base = cx.ctx.len();
        let mut domains = Vec::with_capacity(raw_domains.len());
        for (position, raw_domain) in raw_domains.iter().enumerate() {
            let domain = whnf(cx.env, &cx.ctx, raw_domain);
            cx.ctx.push(domain.clone());
            domains.push(domain);
            if position >= host_ctor.args.len() {
                cx.hidden_positions.push(base + position);
            }
        }
        let evidence_positions =
            all_support_evidence_positions(cx.env, support, ordinal).map_err(|error| {
                ElabError::KernelRejected {
                    error,
                    span: arm.span.clone(),
                }
            })?;
        if support_ctor.args.len() != host_ctor.args.len() + evidence_positions.len()
            || evidence_positions
                .iter()
                .any(|source_field| *source_field >= host_ctor.args.len())
        {
            return Err(ElabError::StructuralResultAssociationMissing {
                match_span: span.clone(),
                field_span: arm.pat.span.clone(),
            });
        }
        if support_shapes.iter().any(|shape| {
            shape.position < host_ctor.args.len()
                || shape.position >= host_ctor.args.len() + evidence_positions.len()
        }) {
            return Err(ElabError::StructuralResultAssociationForeign {
                match_span: span.clone(),
                field_span: arm.pat.span.clone(),
                expected_support: Some(support),
                actual_support: None,
            });
        }
        let mut expected_bindings = Vec::with_capacity(evidence_positions.len());
        for (evidence_ordinal, source_field) in evidence_positions.iter().enumerate() {
            let source_position = base + source_field;
            let evidence_argument = host_ctor.args.len() + evidence_ordinal;
            let result_ordinal = support_shapes
                .iter()
                .position(|shape| shape.position == evidence_argument);
            let installed = install_lift_binding(
                cx,
                source_position,
                base + evidence_argument,
                result_ordinal.map(|ordinal| base + support_ctor.args.len() + ordinal),
            )?;
            expected_bindings.push((source_position, installed));
        }
        let field_spans = sub_pats
            .iter()
            .enumerate()
            .map(|(source_field, pattern)| (base + source_field, pattern.span.clone()))
            .collect::<Vec<_>>();
        validate_lift_associations(&cx.lift_bindings, &expected_bindings)
            .map_err(|failure| lift_association_error(failure, span, &field_spans))?;

        let total = domains.len();
        let mut concrete = Term::Constructor {
            id: host_ctor.id,
            level_args: host_level_args.to_vec(),
        };
        for param in host_params {
            concrete = Term::app(concrete, weaken(param, total as i64));
        }
        for position in 0..host_ctor.args.len() {
            concrete = Term::app(concrete, Term::var(total - 1 - position));
        }
        let expected_here = simplify_branch_goal(
            cx.env,
            &cx.ctx,
            &subst_term_generalize(
                &weaken(expected, total as i64),
                &weaken(scrut_core, total as i64),
                &concrete,
            ),
        );
        let mut method = check(cx, &arm.body, &expected_here, &arm.span)?;

        for source_field in evidence_positions {
            cx.lift_bindings.remove(&(base + source_field));
        }
        cx.hidden_positions.retain(|position| *position < base);
        for _ in 0..total {
            cx.ctx.pop();
        }
        for domain in domains.iter().rev() {
            method = Term::lam(domain.clone(), method);
        }
        let zonked_method = cx.metas.zonk_term(&method);
        let zonked_method_ty = cx.metas.zonk_term(&method_ty);
        kernel_check(cx.env, &cx.ctx, &zonked_method, &zonked_method_ty).map_err(|error| {
            ElabError::Internal(format!(
                "generated All method failed kernel re-check: {error}"
            ))
        })?;
        methods.push(method);
    }
    for (i, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[i] {
                Some(claimant) => ArmDeadCause::Subsumed {
                    first: arms[claimant].span.clone(),
                    rest: Vec::new(),
                },
                None => ArmDeadCause::NoInhabitants,
            };
            return Err(ElabError::ReachabilityError {
                span: arms[i].span.clone(),
                cause,
            });
        }
    }

    let elim = Term::Elim {
        fam: support_id,
        level_args,
        params: support_params,
        motive: Box::new(motive),
        methods,
        indices: support_indices,
        scrut: Box::new(evidence),
    };
    let zonked = cx.metas.zonk_term(&elim);
    kernel_infer(cx.env, &cx.ctx, &zonked).map_err(|error| {
        ElabError::Internal(format!(
            "completed generated All eliminator failed kernel re-check: {error}"
        ))
    })?;
    Ok(elim)
}

#[allow(clippy::too_many_arguments)]
fn check_structured_constructor_method(
    cx: &mut ElabCtx,
    ind: &InductiveDecl,
    ordinal: usize,
    arm: &RMatchArm,
    expected: &Term,
    scrut_core: &Term,
    params: &[Term],
    motive: &Term,
    level_args: &[Level],
    shapes: &[RecursiveArgumentShape],
) -> Result<Term, ElabError> {
    let constructor = &ind.constructors[ordinal];
    if !ind.indices.is_empty() {
        return Err(ElabError::Internal(
            "nested lifted methods for indexed hosts are not yet surface-supported".into(),
        ));
    }
    let method_ty =
        method_type(cx.env, ind, ordinal, motive, params, level_args).map_err(|error| {
            ElabError::KernelRejected {
                error,
                span: arm.span.clone(),
            }
        })?;
    let (raw_domains, _) = peel_pi(&method_ty);
    let field_count = constructor.args.len();
    if raw_domains.len() != field_count + shapes.len() {
        return Err(ElabError::StructuralResultAssociationMissing {
            match_span: arm.span.clone(),
            field_span: arm.pat.span.clone(),
        });
    }
    let base = cx.ctx.len();
    let mut domains = Vec::with_capacity(raw_domains.len());
    for (position, raw_domain) in raw_domains.iter().enumerate() {
        let domain = whnf(cx.env, &cx.ctx, raw_domain);
        cx.ctx.push(domain.clone());
        domains.push(domain);
        if position >= field_count {
            cx.hidden_positions.push(base + position);
        }
    }
    let mut expected_bindings = Vec::with_capacity(shapes.len());
    for (evidence_ordinal, shape) in shapes.iter().enumerate() {
        let source_position = base + shape.position;
        let installed = install_lift_binding(
            cx,
            source_position,
            base + field_count + evidence_ordinal,
            None,
        )?;
        expected_bindings.push((source_position, installed));
    }
    let field_spans = match &arm.pat.kind {
        RPatKind::Ctor(_, fields) => fields
            .iter()
            .enumerate()
            .map(|(source_field, pattern)| (base + source_field, pattern.span.clone()))
            .collect::<Vec<_>>(),
        _ => (0..field_count)
            .map(|source_field| (base + source_field, arm.pat.span.clone()))
            .collect(),
    };
    validate_lift_associations(&cx.lift_bindings, &expected_bindings)
        .map_err(|failure| lift_association_error(failure, &arm.span, &field_spans))?;

    let total = domains.len();
    let mut concrete = Term::Constructor {
        id: constructor.id,
        level_args: level_args.to_vec(),
    };
    for param in params {
        concrete = Term::app(concrete, weaken(param, total as i64));
    }
    for position in 0..field_count {
        concrete = Term::app(concrete, Term::var(total - 1 - position));
    }
    let expected_here = simplify_branch_goal(
        cx.env,
        &cx.ctx,
        &subst_term_generalize(
            &weaken(expected, total as i64),
            &weaken(scrut_core, total as i64),
            &concrete,
        ),
    );
    let checked = check(cx, &arm.body, &expected_here, &arm.span);

    for shape in shapes {
        cx.lift_bindings.remove(&(base + shape.position));
    }
    cx.hidden_positions.retain(|position| *position < base);
    for _ in 0..total {
        cx.ctx.pop();
    }
    let mut method = checked?;
    for domain in domains.iter().rev() {
        method = Term::lam(domain.clone(), method);
    }
    let zonked_method = cx.metas.zonk_term(&method);
    let zonked_ty = cx.metas.zonk_term(&method_ty);
    kernel_check(cx.env, &cx.ctx, &zonked_method, &zonked_ty).map_err(|error| {
        ElabError::Internal(format!(
            "structured host method failed kernel re-check: {error}"
        ))
    })?;
    Ok(method)
}

/// LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT D2: `check_match_dependent`'s
/// capability-3 goal-refinement retry -- the arm taken only when the cheap
/// unrefined attempt fails -- pulled into its own `#[inline(never)]` frame.
/// Same reasoning as `LANG-RECORD-STACK-OVERFLOW`'s `check_pair_or_record`/
/// `infer_record_requires_expected_type`: in an unoptimized build a
/// function's frame is sized for every local declared anywhere in its body,
/// paid on every call regardless of which branch runs. Unlike that node's
/// extraction targets, this one matters here for a *narrower* reason worth
/// recording: `check_match_dependent` is on the `check` <-> `check_match_
/// dependent` mutual-recursion cycle that `decimal_char.rs`'s fixed
/// 31-level `decimalPow10` cascade drives to full depth on every compile
/// (`decimal_char.rs:62-75`), and every arm of that cascade is a bare
/// literal or a two-way `eq_int` match that the cheap unrefined attempt
/// always resolves -- so this fallback's own frame is declared (and paid
/// for by its caller) on every level, but never otherwise entered on this
/// path. Moving it out removes dead weight from
/// the caller's always-paid frame without adding a frame that this
/// particular recursion ever pays for. Unchanged control flow and
/// semantics -- same `refine_branch_goal`/`simplify_branch_goal`/`check`/
/// `Term::Cast` calls in the same order, only their host frame moved.
///
/// This coldness claim is about *which programs*, and it was measured on
/// one: the `decimalPow10` cascade above. A recursion that DOES enter this
/// fallback at every level pays the extracted frame's own call overhead at
/// every level, on top of (not instead of) the reduced caller frame. The
/// inversion mode for such a recursion is a SIGSEGV on the guard page, not
/// a red test in this suite: nothing here would notice before it happened
/// on that recursion's own deployment.
///
/// `LANG-REFINED-FALLBACK-COLDNESS-CLAIM` `D4`, measured, not assumed:
/// `objdump`'s prologue for this function gives `0x1000 + 0x48 = 4168`
/// bytes/call -- well under the `+6472` measured on the reverted mirror
/// extraction above. A recursion that entered this fallback at every level
/// would pay `30232 + 4168 = 34400` bytes/call, `+1048` against the
/// pre-extraction `33352` baseline, not the `+3352`/level a `6472`-sized
/// frame would cost. Over 31 levels that is `+32488` bytes (~31.7 KiB), well
/// inside the ~96 KiB this change bought, not the ~104 KiB that would
/// exceed it.
///
/// `LANG-STACK-ARC-EVIDENCE-USABILITY` `D2`: the `4168` figure above is
/// reproducible from a named artifact, not a one-off read. Build
/// `ken-cli`'s test binaries (`scripts/ken-cargo build -p ken-cli --tests`,
/// default/dev profile), resolve this function's mangled symbol in the
/// `px4b_native_production` binary
/// (`target/debug/deps/px4b_native_production-<hash>`) with
/// `nm <binary> | grep refined_fallback`, then read `0x1000 + 0x48` off
/// `objdump -d --disassemble='<mangled-name>' <binary>`'s prologue -- the
/// two `sub $imm,%rsp` immediates, excluding the `push` register-save
/// bytes.
#[inline(never)]
fn check_match_dependent_refined_fallback(
    cx: &mut ElabCtx,
    arm: &RMatchArm,
    ind: &InductiveDecl,
    params_terms: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    n: usize,
    expected_here: &Term,
    preserve_goal: bool,
) -> Result<Term, ElabError> {
    let (goal_refined, goal_restorations) = refine_branch_goal(
        cx,
        ind,
        params_terms,
        target_indices,
        scrut_indices,
        n,
        expected_here,
    )?;
    let expected_here_refined = if preserve_goal || matches!(arm.body, RExpr::RLam(_, _, _)) {
        goal_refined
    } else {
            simplify_branch_goal(cx.env, &cx.ctx, &goal_refined)
        };
    let body_core_checked = check(cx, &arm.body, &expected_here_refined, &arm.span)?;
    let mut body_core = body_core_checked;
    for restoration in goal_restorations.into_iter().rev() {
        body_core = restoration.apply(body_core);
    }
    Ok(body_core)
}

/// A branch-goal restoration is classified once, where the refined goal is
/// produced. Replay consumes this tag without re-classifying the goal.
enum BranchGoalRestoration {
    TypeCast {
        source_type: Term,
        target_type: Term,
        equality: Term,
    },
    OmegaJ {
        index_type: Term,
        old_index: Term,
        new_index: Term,
        source_type: Term,
        omega_level: Level,
        equality: Term,
    },
}

impl BranchGoalRestoration {
    fn apply(self, value: Term) -> Term {
        match self {
            Self::TypeCast {
                source_type,
                target_type,
                equality,
            } => Term::Cast(
                Box::new(source_type),
                Box::new(target_type),
                Box::new(equality),
                Box::new(value),
            ),
            Self::OmegaJ {
                index_type,
                old_index,
                new_index,
                source_type,
                omega_level,
                equality,
            } => {
                let (transported, _) = build_index_omega_transport(
                    &index_type,
                    &old_index,
                    &new_index,
                    &source_type,
                    omega_level,
                    value,
                    equality,
                );
                transported
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn classify_branch_goal_restoration(
    env: &GlobalEnv,
    ctx: &Context,
    index_type: &Term,
    old_index: &Term,
    new_index: &Term,
    source_type: &Term,
    classifier: Term,
    equality: Term,
) -> Result<BranchGoalRestoration, ElabError> {
    match classifier {
        Term::Type(level) => {
            let (type_equality, target_type) = build_index_type_cong(
                env,
                ctx,
                index_type,
                old_index,
                new_index,
                source_type,
                level,
                equality,
            );
            Ok(BranchGoalRestoration::TypeCast {
                source_type: source_type.clone(),
                target_type,
                equality: type_equality,
            })
        }
        Term::Omega(omega_level) => Ok(BranchGoalRestoration::OmegaJ {
            index_type: index_type.clone(),
            old_index: old_index.clone(),
            new_index: new_index.clone(),
            source_type: source_type.clone(),
            omega_level,
            equality,
        }),
        other => Err(ElabError::Internal(format!(
            "index refinement: branch goal is classified by neither Type nor Omega, found {other:?}"
        ))),
    }
}

/// Finish a dependent method after its branch body has returned. This owns the
/// IH-domain and field-lambda construction in a non-recursive host frame: the
/// caller recursively invokes `check` while elaborating the body, so keeping
/// these later-only temporaries in that caller charges them at every source
/// recursion level even though none is live across the recursive call.
#[inline(never)]
fn wrap_dependent_method_ihs(
    env: &GlobalEnv,
    outer_ctx: &Context,
    span: &Span,
    shapes: &[RecursiveArgumentShape],
    n: usize,
    expected: &Term,
    exact_motive: Option<&Term>,
    scrut_core: &Term,
    ind: &InductiveDecl,
    params_terms: &[Term],
    scrut_indices: &[Term],
    m: usize,
    ctor: &ConstructorDecl,
    sentinel_region: usize,
    context_convoy: &[ConvoyEntry],
    embedded_method_convoy: &[EmbeddedMethodConvoy],
    embedded_method_repairs: &[(usize, usize)],
    method: Term,
) -> Result<Term, ElabError> {
    // IH-slot emission (`dependent-match-nonnullary`, Map Gap B): the
    // kernel's `method_type` requires `Π(fields) Π(ih₁…ih_p). M t̄ (Cₖ …)`.
    let mut method_ctx = outer_ctx.clone();
    for j in 0..n {
        method_ctx.push(subst_outer(&ctor.args[j], m, params_terms, j));
    }
    let rec = shapes
        .iter()
        .map(|argument| {
            let (domains, indices) = argument
                .shape
                .as_legacy()
                .expect("structured shapes returned through the dedicated method path");
            (argument.position, domains, indices)
        })
        .collect::<Vec<_>>();
    let mut method = method;
    for (pos, branching_tel, idxs) in rec.iter().rev() {
        let nb = branching_tel.len();
        let ih_ty = if nb == 0 {
            let field_var = Term::var(n - 1 - pos);
            let ih_indices: Vec<Term> = idxs
                .iter()
                .map(|t| {
                    ken_kernel::subst::shift(
                        &subst_outer(t, m, params_terms, *pos),
                        (n - pos) as i64,
                        0,
                    )
                })
                .collect();
            if let Some(motive) = exact_motive {
                let mut ih_ty = weaken(motive, n as i64);
                for index in &ih_indices {
                    ih_ty = Term::app(ih_ty, index.clone());
                }
                Term::app(ih_ty, field_var)
            } else {
                let ih_body = subst_term_generalize_many(
                    &weaken(expected, n as i64),
                    &dependent_rebase_subs(
                        scrut_core,
                        scrut_indices,
                        n as i64,
                        &ih_indices,
                        &field_var,
                        expected,
                        context_convoy
                            .iter()
                            .any(|e| scrut_occurs(expected, &Term::var(e.var))),
                    ),
                );
                // The direct IH is `motive` applied to the recursive field at
                // `ih_indices`, so it carries the same context-telescope convoy the
                // motive codomain generalizes. Build it through the shared helper
                // (degenerates to the old `wrap_premise_pis` when the convoy is
                // empty).
                build_convoy_refined_type(
                    env,
                    &method_ctx,
                    span,
                    ind,
                    params_terms,
                    &ih_indices,
                    scrut_indices,
                    scrut_core,
                    &field_var,
                    n,
                    sentinel_region,
                    context_convoy,
                    embedded_method_convoy,
                    embedded_method_repairs,
                    ih_body,
                )?
            }
        } else {
            let mut scrut_body = Term::var(n - 1 - pos + nb);
            for bk in 0..nb {
                scrut_body = Term::app(scrut_body, Term::var(nb - 1 - bk));
            }
            let mut ih_ty = subst_term_generalize(
                &weaken(expected, (n + nb) as i64),
                &weaken(scrut_core, (n + nb) as i64),
                &scrut_body,
            );
            for bk in (0..nb).rev() {
                let b_dom = ken_kernel::subst::shift(
                    &subst_outer(&branching_tel[bk], m, params_terms, pos + bk),
                    (n - pos) as i64,
                    bk,
                );
                ih_ty = Term::pi(b_dom, ih_ty);
            }
            ih_ty
        };
        method = Term::lam(ih_ty, weaken(&method, 1));
    }
    for j in (0..n).rev() {
        method = Term::lam(subst_outer(&ctor.args[j], m, params_terms, j), method);
    }
    Ok(method)
}

#[inline(never)]
fn finish_dependent_elim(
    cx: &mut ElabCtx,
    d_id: GlobalId,
    ind: &InductiveDecl,
    params_terms: Vec<Term>,
    motive: Term,
    methods: Vec<Term>,
    scrut_indices: &[Term],
    scrut_ty: &Term,
    scrut_core: &Term,
    context_convoy: &[ConvoyEntry],
    embedded_method_convoy: &[EmbeddedMethodConvoy],
    add_hidden_equation: bool,
    recursive_field_index_path: RecursiveFieldIndexPath,
    span: &Span,
) -> Result<Term, ElabError> {
    let top_premises = if recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared {
        Vec::new()
    } else {
        method_index_premises(ind, &params_terms, scrut_indices, scrut_indices, 0)
    };
    let mut elim = Term::Elim {
        fam: d_id,
        level_args: vec![],
        params: params_terms,
        motive: Box::new(motive),
        methods,
        indices: scrut_indices.to_vec(),
        scrut: Box::new(scrut_core.clone()),
    };
    for premise in &top_premises {
        let proof = synth_generated_index_evidence(cx.env, &cx.ctx, premise, span)?;
        elim = Term::app(elim, proof);
    }
    // Apply the motive-codomain convoy telescope to the ORIGINAL ambient values,
    // outermost binder first (reverse of the innermost-first convoy), so the
    // completed eliminator's type reconstructs the caller's `expected` at the
    // actual indices (LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE step 5).
    for entry in context_convoy.iter().rev() {
        elim = Term::app(elim, Term::var(entry.var));
    }
    for entry in embedded_method_convoy {
        elim = Term::app(elim, entry.ambient_value.clone());
    }
    if add_hidden_equation {
        let hidden_equation = Term::Eq(
            Box::new(scrut_ty.clone()),
            Box::new(scrut_core.clone()),
            Box::new(scrut_core.clone()),
        );
        let proof = synth_generated_index_evidence(cx.env, &cx.ctx, &hidden_equation, span)?;
        elim = Term::app(elim, proof);
        let zonked_ctx = Context {
            types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
        };
        let zonked_elim = cx.metas.zonk_term(&elim);
        kernel_infer(cx.env, &zonked_ctx, &zonked_elim).map_err(|error| {
            ElabError::KernelRejected {
                error,
                span: span.clone(),
            }
        })?;
    }
    Ok(elim)
}

/// Check a recursive source arm for a large-index convoy in the frame where
/// the enclosing recursive IH is defined.  The successor method's peeled
/// evidence is `k = m`, while the IH is naturally an `m`-frame proof.  Abstract
/// each current constructor field that depends on `k`, build the reflexive
/// `m`-frame base without casts, then transport the whole goal once along
/// `sym(e) : m = k`.  The value cast in the large-selector codomain is thereby
/// an image in this single goal-level J motive, never a separately composed
/// branch refinement.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn check_large_convoy_recursive_arm(
    cx: &mut ElabCtx,
    arm: &RMatchArm,
    ind: &InductiveDecl,
    params: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    n: usize,
    expected_here: &Term,
    sentinel_region: usize,
) -> Result<Option<Term>, ElabError> {
    if !expression_mentions_recursive_group(cx, &arm.body) || ind.indices.len() != 1 {
        return Ok(None);
    }
    let pairs = method_index_premise_pairs(ind, params, target_indices, scrut_indices, n);
    if pairs.len() != 1 {
        return Ok(None);
    }
    let (index_ty, _, _) = &pairs[0];
    let raw_evidence = Term::Eq(
        Box::new(index_ty.clone()),
        Box::new(target_indices[0].clone()),
        Box::new(weaken(&scrut_indices[0], n as i64)),
    );
    let leaves = project_generated_index_equality_leaves(
        cx.env,
        &cx.ctx,
        &raw_evidence,
        index_refinement_sentinel(sentinel_region, 0),
    )?;
    let [leaf] = leaves.as_slice() else {
        return Ok(None);
    };
    let Term::Type(index_level) = whnf(
        cx.env,
        &cx.ctx,
        &kernel_infer(cx.env, &cx.ctx, &leaf.index_ty).map_err(|error| {
            ElabError::Internal(format!(
                "large index convoy could not classify its peeled equality: {error:?}"
            ))
        })?,
    ) else {
        return Ok(None);
    };

    // The large-selector codomain deliberately uses this stable, double-sym
    // representative of the forward equality.  With the goal-J oriented over
    // `sym(e)`, its result image contains the same representative, while its
    // reflexive base reduces all three symmetry/J layers away.
    let reverse = build_sym(
        cx.env,
        &cx.ctx,
        &leaf.index_ty,
        index_level.clone(),
        &leaf.target,
        leaf.proof.clone(),
    );
    let stable_forward = build_sym(
        cx.env,
        &cx.ctx,
        &leaf.index_ty,
        index_level.clone(),
        &leaf.scrutinee,
        reverse.clone(),
    );
    if !scrut_occurs(expected_here, &stable_forward) {
        return Ok(None);
    }

    // Abstract precisely the current constructor fields whose types move from
    // the matched predecessor `k` to the IH predecessor `m`.  For DualEnv this
    // is `rest`; the enclosing DualFin sibling remains an ordinary m-frame
    // argument of the IH.  This is the single-J replacement for capability 1,
    // not an additional cast layered beneath it.
    let mut moving_fields = Vec::new();
    for field_j in 0..n {
        let field_index = n - 1 - field_j;
        let field_term = Term::var(field_index);
        let field_ty = weaken(
            cx.ctx
                .lookup(field_index)
                .expect("constructor field in range"),
            (field_index + 1) as i64,
        );
        let source_ty = subst_term_generalize(&field_ty, &leaf.target, &leaf.scrutinee);
        if source_ty != field_ty && scrut_occurs(expected_here, &field_term) {
            let position = cx.ctx.len() - 1 - field_index;
            moving_fields.push((position, field_index, field_term, field_ty, source_ty));
        }
    }
    if moving_fields.is_empty() {
        return Ok(None);
    }

    let mut base_substitutions = vec![
        (leaf.target.clone(), leaf.scrutinee.clone()),
        (
            stable_forward.clone(),
            Term::Refl(Box::new(refl_base_arg(
                cx.env,
                &cx.ctx,
                &leaf.index_ty,
                &leaf.scrutinee,
            ))),
        ),
    ];
    let mut source_domains = Vec::with_capacity(moving_fields.len());
    for (slot, (_, _, field_term, _, source_ty)) in moving_fields.iter().enumerate() {
        base_substitutions.push((
            field_term.clone(),
            index_refinement_sentinel(sentinel_region, slot),
        ));
        source_domains.push(source_ty.clone());
    }
    let base_goal = subst_term_generalize_many(expected_here, &base_substitutions);

    let refinement_snapshot = cx.var_refinements.clone();
    for (slot, (position, _, _, _, source_ty)) in moving_fields.iter().enumerate() {
        cx.var_refinements.insert(
            *position,
            (
                index_refinement_sentinel(sentinel_region, slot),
                source_ty.clone(),
                cx.ctx.len(),
            ),
        );
    }
    let checked_base = (|| {
        let (core, inferred) = infer(cx, &arm.body)?;
        unify_types(&mut cx.metas, &base_goal, &inferred);
        let base = wrap_premise_lams_finalized(core, &source_domains, sentinel_region);
        let base_ty =
            wrap_premise_pis_finalized(base_goal.clone(), &source_domains, sentinel_region);
        let zonked_base = cx.metas.zonk_term(&base);
        let zonked_ty = cx.metas.zonk_term(&base_ty);
        let zonked_ctx = Context {
            types: cx
                .ctx
                .types
                .iter()
                .map(|term| cx.metas.zonk_term(term))
                .collect(),
        };
        kernel_check(cx.env, &zonked_ctx, &zonked_base, &zonked_ty).map_err(|error| {
            ElabError::KernelRejected {
                error,
                span: arm.span.clone(),
            }
        })?;
        Ok(zonked_base)
    })();
    cx.var_refinements = refinement_snapshot;
    let base = checked_base?;

    // Build P(x,h) from the exact large-selector codomain.  Here
    // h : m = x, so sym(h) : x = m is exactly the equality image used by the
    // selector's value cast.  At x=m,h=Refl the goal and every moving field are
    // definitionally the uncast recursive base.
    let mut motive_ctx = cx.ctx.clone();
    motive_ctx.push(leaf.index_ty.clone());
    let motive_evidence_domain = Term::Eq(
        Box::new(weaken(&leaf.index_ty, 1)),
        Box::new(weaken(&leaf.scrutinee, 1)),
        Box::new(Term::var(0)),
    );
    motive_ctx.push(motive_evidence_domain.clone());
    let motive_reverse = build_sym(
        cx.env,
        &motive_ctx,
        &weaken(&leaf.index_ty, 2),
        index_level.clone(),
        &weaken(&leaf.scrutinee, 2),
        Term::var(0),
    );
    let mut motive_substitutions = vec![
        (weaken(&leaf.target, 2), Term::var(1)),
        (weaken(&stable_forward, 2), motive_reverse),
    ];
    let mut motive_domains = Vec::with_capacity(moving_fields.len());
    for (slot, (_, _, field_term, field_ty, _)) in moving_fields.iter().enumerate() {
        motive_substitutions.push((
            weaken(field_term, 2),
            index_refinement_sentinel(sentinel_region, slot),
        ));
        motive_domains.push(subst_term_generalize(
            &weaken(field_ty, 2),
            &weaken(&leaf.target, 2),
            &Term::var(1),
        ));
    }
    let motive_goal = subst_term_generalize_many(&weaken(expected_here, 2), &motive_substitutions);
    let motive_result = wrap_premise_pis_finalized(motive_goal, &motive_domains, sentinel_region);

    let Term::Eq(goal_carrier, _, _) = whnf(cx.env, &cx.ctx, expected_here) else {
        return Ok(None);
    };
    let Term::Type(goal_level) = whnf(
        cx.env,
        &cx.ctx,
        &kernel_infer(cx.env, &cx.ctx, &goal_carrier).map_err(|error| {
            ElabError::Internal(format!(
                "large index convoy could not classify its recursive goal: {error:?}"
            ))
        })?,
    ) else {
        return Ok(None);
    };
    let motive_body = Term::lam(
        leaf.index_ty.clone(),
        Term::lam(motive_evidence_domain.clone(), motive_result),
    );
    let motive_ty = Term::pi(
        leaf.index_ty.clone(),
        Term::pi(motive_evidence_domain, Term::Omega(goal_level)),
    );
    let motive = Term::Ascript(Box::new(motive_body), Box::new(motive_ty));
    let symmetric_evidence = build_sym(
        cx.env,
        &cx.ctx,
        &leaf.index_ty,
        index_level,
        &leaf.target,
        leaf.proof.clone(),
    );
    let mut transported = Term::J(
        Box::new(motive),
        Box::new(base),
        Box::new(symmetric_evidence),
    );
    for (_, field_index, _, _, _) in &moving_fields {
        transported = Term::app(transported, Term::var(*field_index));
    }
    Ok(Some(transported))
}

struct DependentConstructorFrame {
    concrete: Box<Term>,
    target_indices: Vec<Term>,
    premise_domains: Vec<Term>,
    expected_here: Term,
    convoy_refinements: Vec<(usize, Term, Term)>,
    hidden_result_premise_slot: Option<usize>,
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn build_dependent_constructor_frame(
    cx: &ElabCtx,
    ind: &InductiveDecl,
    ctor: &ConstructorDecl,
    params: &[Term],
    level_args: &[Level],
    scrut_indices: &[Term],
    scrut_ty: &Term,
    scrut_core: &Term,
    expected: &Term,
    motive: &Term,
    field_count: usize,
    sentinel_region: usize,
    context_convoy: &[ConvoyEntry],
    embedded_method_convoy: &[EmbeddedMethodConvoy],
    embedded_method_repairs: &[(usize, usize)],
    hidden_group_result_refinement: bool,
    equation_convoy: bool,
    recursive_field_index_path: RecursiveFieldIndexPath,
    span: &Span,
) -> Result<Box<DependentConstructorFrame>, ElabError> {
    let concrete = build_dependent_concrete(ctor, params, level_args, field_count);
    let target_indices = ctor_target_indices(ctor, ind, params, level_args, field_count);
    let plain_declared = recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared;
    if plain_declared
        && (!context_convoy.is_empty()
            || !embedded_method_convoy.is_empty()
            || !embedded_method_repairs.is_empty()
            || equation_convoy)
    {
        return Err(ElabError::Internal(
            "plain declared-index path overlapped coupled convoy state".into(),
        ));
    }
    let mut premise_domains = if plain_declared {
        Vec::new()
    } else {
        method_index_premises(
            ind,
            params,
            &target_indices,
            scrut_indices,
            field_count,
        )
    };
    let mut expected_here = if plain_declared {
        let mut specialized = weaken(motive, field_count as i64);
        for index in &target_indices {
            specialized = Term::app(specialized, index.clone());
        }
        Term::app(specialized, concrete.as_ref().clone())
    } else if equation_convoy {
        if premise_domains.len() != 1 {
            return Err(ElabError::Internal(
                "large index convoy expected exactly one generated premise".into(),
            ));
        }
        large_convoy_branch_goal(
            cx.env,
            &cx.ctx,
            motive,
            &target_indices,
            &concrete,
            field_count,
            context_convoy.len(),
            sentinel_region,
        )?
    } else {
        subst_term_generalize_many(
            &weaken(expected, field_count as i64),
            &dependent_rebase_subs(
                scrut_core,
                scrut_indices,
                field_count as i64,
                &target_indices,
                &concrete,
                expected,
                context_convoy
                    .iter()
                    .any(|entry| scrut_occurs(expected, &Term::var(entry.var))),
            ),
        )
    };

    let index_premise_count = premise_domains.len();
    let (mut types_inner_first, sentinels) = convoy_binder_types(
        context_convoy,
        scrut_indices,
        scrut_core,
        &target_indices,
        &concrete,
        field_count,
        index_premise_count,
        sentinel_region,
    );
    if equation_convoy && !context_convoy.is_empty() {
        let index_equation = premise_domains[0].clone();
        let leaves = project_generated_index_equality_leaves(
            cx.env,
            &cx.ctx,
            &index_equation,
            index_refinement_sentinel(sentinel_region, 0),
        )?;
        let telescope_argument_substitutions: Vec<(Term, Term)> = leaves
            .iter()
            .filter(|leaf| {
                types_inner_first
                    .iter()
                    .any(|ty| scrut_occurs(ty, &leaf.scrutinee))
            })
            .map(|leaf| (leaf.scrutinee.clone(), leaf.target.clone()))
            .collect();
        let carried = carry_coherent_frame_data(
            leaves,
            index_equation,
            telescope_argument_substitutions,
        );
        for ty in &mut types_inner_first {
            *ty = subst_term_generalize_many(
                ty,
                &carried.telescope_argument_substitutions,
            );
        }
    }
    expected_here = redirect_convoy_body(
        context_convoy,
        field_count,
        &sentinels,
        expected_here,
    );
    let mut convoy_refinements = Vec::with_capacity(context_convoy.len());
    for (index, entry) in context_convoy.iter().enumerate() {
        let bottom_position = cx.ctx.len() - 1 - (entry.var + field_count);
        convoy_refinements.push((
            bottom_position,
            sentinels[index].clone(),
            types_inner_first[index].clone(),
        ));
    }
    for index in (0..context_convoy.len()).rev() {
        premise_domains.push(types_inner_first[index].clone());
    }

    if !embedded_method_convoy.is_empty() || !embedded_method_repairs.is_empty() {
        let embedded_slot_base = premise_domains.len();
        expected_here = install_embedded_method_sentinels(
            cx.env,
            &cx.ctx,
            &expected_here,
            embedded_method_convoy,
            embedded_method_repairs,
            embedded_slot_base,
        )?;
        expected_here =
            refresh_embedded_elim_evidence(cx.env, &cx.ctx, &expected_here, span)?;
        let branch_rebase = dependent_rebase_subs(
            scrut_core,
            scrut_indices,
            field_count as i64,
            &target_indices,
            &concrete,
            expected,
            context_convoy
                .iter()
                .any(|entry| scrut_occurs(expected, &Term::var(entry.var))),
        );
        for entry in embedded_method_convoy {
            let branch_ty = subst_term_generalize_many(
                &weaken(&entry.ambient_ty, field_count as i64),
                &branch_rebase,
            );
            premise_domains.push(redirect_convoy_body(
                context_convoy,
                field_count,
                &sentinels,
                branch_ty,
            ));
        }
    }

    let hidden_result_premise_slot = if hidden_group_result_refinement {
        let slot = premise_domains.len();
        premise_domains.push(Term::Eq(
            Box::new(weaken(scrut_ty, field_count as i64)),
            Box::new(concrete.as_ref().clone()),
            Box::new(weaken(scrut_core, field_count as i64)),
        ));
        Some(slot)
    } else {
        None
    };
    Ok(Box::new(DependentConstructorFrame {
        concrete,
        target_indices,
        premise_domains,
        expected_here,
        convoy_refinements,
        hidden_result_premise_slot,
    }))
}

#[inline(never)]
fn build_dependent_concrete(
    ctor: &ConstructorDecl,
    params: &[Term],
    level_args: &[Level],
    field_count: usize,
) -> Box<Term> {
    let mut concrete = Term::Constructor {
        id: ctor.id,
        level_args: level_args.to_vec(),
    };
    for param in params {
        concrete = Term::app(concrete, weaken(param, field_count as i64));
    }
    for field in (0..field_count).rev() {
        concrete = Term::app(concrete, Term::var(field));
    }
    Box::new(concrete)
}

#[inline(never)]
fn large_convoy_branch_goal(
    env: &GlobalEnv,
    ctx: &Context,
    motive: &Term,
    target_indices: &[Term],
    concrete: &Term,
    field_count: usize,
    context_convoy_len: usize,
    sentinel_region: usize,
) -> Result<Term, ElabError> {
    let mut motive_args = target_indices.to_vec();
    motive_args.push(concrete.clone());
    let specialized = motive_args
        .into_iter()
        .fold(weaken(motive, field_count as i64), Term::app);
    let Term::Pi(_, codomain) = whnf(env, ctx, &specialized) else {
        return Err(ElabError::Internal(
            "large index convoy branch did not compute to its evidence premise".into(),
        ));
    };
    let mut goal = subst0(
        &codomain,
        &index_refinement_sentinel(sentinel_region, 0),
    );
    for convoy_slot in 0..context_convoy_len {
        let Term::Pi(_, codomain) = whnf(env, ctx, &goal) else {
            return Err(ElabError::Internal(
                "large index convoy branch lost a context-telescope argument".into(),
            ));
        };
        goal = subst0(
            &codomain,
            &index_refinement_sentinel(sentinel_region, 1 + convoy_slot),
        );
    }
    Ok(goal)
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn build_large_convoy_recursive_method(
    cx: &mut ElabCtx,
    arm: &RMatchArm,
    ind: &InductiveDecl,
    params: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    field_count: usize,
    expected_here: &Term,
    sentinel_region: usize,
    premise_domains: &[Term],
) -> Result<Term, ElabError> {
    let goal_j = check_large_convoy_recursive_arm(
        cx,
        arm,
        ind,
        params,
        target_indices,
        scrut_indices,
        field_count,
        expected_here,
        sentinel_region,
    )?
    .ok_or_else(|| {
        ElabError::Internal(
            "large index convoy could not construct its recursive goal transport".into(),
        )
    })?;
    Ok(wrap_premise_lams_finalized(
        goal_j,
        premise_domains,
        sentinel_region,
    ))
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn finalize_large_convoy_refl_method(
    env: &GlobalEnv,
    ctx: &Context,
    equation_convoy: bool,
    arm_index: Option<usize>,
    arms: &[RMatchArm],
    ind: &InductiveDecl,
    constructor_ordinal: usize,
    motive: &Term,
    params: &[Term],
    fallback: Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let Some(arm_index) = arm_index.filter(|arm_index| {
        equation_convoy
            && matches!(&arms[*arm_index].body, RExpr::RCon(name, _) if name == SUGAR_REFL)
    }) else {
        return Ok(fallback);
    };
    let exact_ty =
        method_type(env, ind, constructor_ordinal, motive, params, &[]).map_err(|error| {
            ElabError::KernelRejected {
                error,
                span: span.clone(),
            }
        })?;
    synth_refl_method_from_type(env, &mut ctx.clone(), &exact_ty, &arms[arm_index].span)
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn finish_dependent_constructor_method(
    cx: &mut ElabCtx,
    span: &Span,
    shapes: &[RecursiveArgumentShape],
    field_count: usize,
    expected: &Term,
    equation_convoy: bool,
    recursive_field_index_path: RecursiveFieldIndexPath,
    scrut_core: &Term,
    ind: &InductiveDecl,
    params: &[Term],
    scrut_indices: &[Term],
    param_count: usize,
    ctor: &ConstructorDecl,
    sentinel_region: usize,
    context_convoy: &[ConvoyEntry],
    embedded_method_convoy: &[EmbeddedMethodConvoy],
    embedded_method_repairs: &[(usize, usize)],
    method: Term,
    arm_index: Option<usize>,
    arms: &[RMatchArm],
    constructor_ordinal: usize,
    motive: &Term,
) -> Result<Term, ElabError> {
    let wrapped = wrap_dependent_method_ihs(
        cx.env,
        &cx.ctx,
        span,
        shapes,
        field_count,
        expected,
        (equation_convoy
            || recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared)
            .then_some(motive),
        scrut_core,
        ind,
        params,
        scrut_indices,
        param_count,
        ctor,
        sentinel_region,
        context_convoy,
        embedded_method_convoy,
        embedded_method_repairs,
        method,
    )?;
    finalize_large_convoy_refl_method(
        cx.env,
        &cx.ctx,
        equation_convoy,
        arm_index,
        arms,
        ind,
        constructor_ordinal,
        motive,
        params,
        wrapped,
        span,
    )
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn build_checked_dependent_motive(
    env: &GlobalEnv,
    motive_ctx: &Context,
    ind: &InductiveDecl,
    family: GlobalId,
    params: &[Term],
    scrut_indices: &[Term],
    motive_user_body: Term,
    equation_convoy: bool,
    recursive_field_index_path: RecursiveFieldIndexPath,
    span: &Span,
) -> Result<Box<Term>, ElabError> {
    let motive_premises = motive_index_premises(ind, params, scrut_indices);
    let motive_body = if equation_convoy
        || recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared
    {
        motive_user_body
    } else {
        wrap_premise_pis(motive_user_body, &motive_premises)
    };
    let motive_sort =
        kernel_infer(env, motive_ctx, &motive_body).map_err(|error| ElabError::KernelRejected {
            error,
            span: span.clone(),
        })?;
    let motive_ty = motive_type(ind, family, params, &motive_sort);
    Ok(Box::new(Term::Ascript(
        Box::new(wrap_motive_lambdas(ind, family, params, motive_body)),
        Box::new(motive_ty),
    )))
}

/// Install the plain-eliminator view of the actual index and scrutinee
/// variables. The match motive has abstracted both ambient names, so inside a
/// constructor method they denote the constructor result index and value
/// directly. No equality premise or transport is needed; the kernel context
/// still carries the real constructor fields and validates every use against
/// their declared types.
fn install_plain_declared_index_aliases(
    cx: &mut ElabCtx,
    ind: &InductiveDecl,
    params: &[Term],
    level_args: &[Level],
    target_indices: &[Term],
    scrut_indices: &[Term],
    scrut_core: &Term,
    concrete: &Term,
    field_count: usize,
) -> Result<(), ElabError> {
    if ind.indices.len() != target_indices.len() || scrut_indices.len() != target_indices.len() {
        return Err(ElabError::Internal(
            "plain declared-index path lost its index telescope".into(),
        ));
    }
    // Surface `(a : Type)` may still leave a universe metavariable in the
    // elaborator context even though the admitted family parameter has already
    // zonked it. Every raw-kernel query below must see the same zonked context,
    // target indices, and level-instantiated constructor.
    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|term| cx.metas.zonk_term(term)).collect(),
    };
    for (ordinal, (actual, target)) in scrut_indices.iter().zip(target_indices).enumerate() {
        let Term::Var(actual_index) = actual else {
            return Err(ElabError::Internal(
                "plain declared-index path received a non-variable actual index".into(),
            ));
        };
        let actual_index = actual_index + field_count;
        let position = cx
            .ctx
            .len()
            .checked_sub(1 + actual_index)
            .ok_or_else(|| {
                ElabError::Internal(
                    "plain declared-index variable escaped its constructor context".into(),
                )
            })?;
        let index_ty = cx.metas.zonk_term(&weaken(
            &subst_levels(
                &subst_outer(&ind.indices[ordinal], ind.params.len(), params, ordinal),
                &ind.level_params,
                level_args,
            ),
            field_count as i64,
        ));
        let target = cx.metas.zonk_term(target);
        let target_ty = kernel_infer(cx.env, &zonked_ctx, &target).map_err(|error| {
            ElabError::Internal(format!(
                "plain declared-index target is ill-typed: {error:?}"
            ))
        })?;
        if !convert_type(cx.env, &zonked_ctx, &target_ty, &index_ty) {
            return Err(ElabError::Internal(
                "plain declared-index target has the wrong index type".into(),
            ));
        }
        cx.var_refinements
            .insert(position, (target, index_ty, cx.ctx.len()));
    }

    let Term::Var(scrut_index) = scrut_core else {
        return Err(ElabError::Internal(
            "plain declared-index path received a non-variable scrutinee".into(),
        ));
    };
    let scrut_index = scrut_index + field_count;
    let scrut_position = cx
        .ctx
        .len()
        .checked_sub(1 + scrut_index)
        .ok_or_else(|| {
            ElabError::Internal(
                "plain declared-index scrutinee escaped its constructor context".into(),
            )
        })?;
    let concrete = cx.metas.zonk_term(concrete);
    let concrete_ty = kernel_infer(cx.env, &zonked_ctx, &concrete).map_err(|error| {
        ElabError::Internal(format!(
            "plain declared-index constructor is ill-typed: {error:?}"
        ))
    })?;
    cx.var_refinements
        .insert(scrut_position, (concrete, concrete_ty, cx.ctx.len()));
    Ok(())
}

/// Check an ordinary dependent-match arm outside the wide method-construction
/// dispatcher. Keeping this stateful fallback in a cold frame is load-bearing:
/// the elaborator's unoptimized recursive checker runs near the spawned-thread
/// stack limit even for matches that never use a coherent-frame convoy.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn check_dependent_branch_body(
    cx: &mut ElabCtx,
    arm: &RMatchArm,
    ind: &InductiveDecl,
    params: &[Term],
    level_args: &[Level],
    target_indices: &[Term],
    scrut_indices: &[Term],
    n: usize,
    expected_here: &Term,
    premise_domains: &[Term],
    hidden_result_premise_slot: Option<usize>,
    scrut_ty: &Term,
    concrete: &Term,
    scrut_core: &Term,
    sentinel_region: usize,
    convoy_refinements: &[(usize, Term, Term)],
    equation_convoy: bool,
    recursive_field_index_path: RecursiveFieldIndexPath,
) -> Result<Term, ElabError> {
    let outer_scope_depth = cx.ctx.len() - n;
    let var_refinement_snapshot = cx.var_refinements.clone();
    let result_refinement_base = cx.result_refinements.len();
    cx.match_field_regions.push(outer_scope_depth..cx.ctx.len());

    let outcome = (|| {
        if recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared {
            install_plain_declared_index_aliases(
                cx,
                ind,
                params,
                level_args,
                target_indices,
                scrut_indices,
                scrut_core,
                concrete,
                n,
            )?;
        } else {
            install_index_refinements(
                cx,
                ind,
                params,
                target_indices,
                scrut_indices,
                n,
                outer_scope_depth,
            )?;
        }
        if let Some(premise_slot) = hidden_result_premise_slot {
            install_hidden_result_variable_refinements(
                cx,
                &weaken(scrut_ty, n as i64),
                concrete,
                &weaken(scrut_core, n as i64),
                premise_slot,
                outer_scope_depth,
            )?;
        }
        for (bottom_pos, sentinel, ctor_ty) in convoy_refinements {
            cx.var_refinements.insert(
                *bottom_pos,
                (sentinel.clone(), ctor_ty.clone(), cx.ctx.len()),
            );
        }

        let preserve_nested_goal =
            !ind.indices.is_empty() && matches!(arm.body, RExpr::RMatch { .. });
        let expected_unrefined =
            if preserve_nested_goal || matches!(arm.body, RExpr::RLam(_, _, _)) {
                expected_here.clone()
            } else {
                simplify_branch_goal(cx.env, &cx.ctx, expected_here)
            };
        if let Some(premise_slot) = hidden_result_premise_slot {
            cx.result_refinements.push(ResultRefinement {
                index_ty: weaken(scrut_ty, n as i64),
                concrete_index: concrete.clone(),
                refined_index: weaken(scrut_core, n as i64),
                premise_slot,
                sentinel_region,
                install_depth: cx.ctx.len(),
            });
        }
        let obligation_base = cx.obligations.len();
        let attempt = check(cx, &arm.body, &expected_unrefined, &arm.span)
        .and_then(|checked| {
            let wrapped =
                wrap_premise_lams_finalized(checked.clone(), premise_domains, sentinel_region);
            let wrapped_ty = wrap_premise_pis_finalized(
                expected_unrefined.clone(),
                premise_domains,
                sentinel_region,
            );
            let zonked_wrapped = cx.metas.zonk_term(&wrapped);
            let zonked_ty = cx.metas.zonk_term(&wrapped_ty);
            let zonked_ctx = Context {
                types: cx
                    .ctx
                    .types
                    .iter()
                    .map(|term| cx.metas.zonk_term(term))
                    .collect(),
            };
            kernel_check(cx.env, &zonked_ctx, &zonked_wrapped, &zonked_ty)
                .map(|()| checked)
                .map_err(|error| ElabError::KernelRejected {
                    error,
                    span: arm.span.clone(),
                })
        });
        let checked = match attempt {
            Ok(checked) => checked,
            Err(error)
                if recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared =>
            {
                cx.obligations.truncate(obligation_base);
                return Err(error);
            }
            Err(_) => {
                cx.obligations.truncate(obligation_base);
                check_match_dependent_refined_fallback(
                    cx,
                    arm,
                    ind,
                    params,
                    target_indices,
                    scrut_indices,
                    n,
                    expected_here,
                    equation_convoy || preserve_nested_goal,
                )?
            }
        };
        Ok(wrap_premise_lams_finalized(
            checked,
            premise_domains,
            sentinel_region,
        ))
    })();

    cx.result_refinements.truncate(result_refinement_base);
    cx.var_refinements = var_refinement_snapshot;
    cx.match_field_regions.pop();
    outcome
}

#[inline(never)]
fn dependent_inductive(env: &GlobalEnv, family: GlobalId) -> Result<Box<InductiveDecl>, ElabError> {
    env.inductive(family)
        .cloned()
        .map(Box::new)
        .ok_or_else(|| ElabError::Internal(format!("inductive {:?} not found", family)))
}

#[inline(never)]
fn dependent_scrutinee_family(
    scrutinee_type: &Term,
    span: &Span,
) -> Result<(GlobalId, Vec<Level>, Vec<Term>), ElabError> {
    let (head, arguments) = peel_app(scrutinee_type);
    let Term::IndFormer { id, level_args } = head else {
        return Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "match scrutinee must have an inductive type".into(),
        });
    };
    Ok((id, level_args, arguments))
}

#[inline(never)]
fn zonked_dependent_expected(cx: &ElabCtx, expected: &Term) -> Box<Term> {
    Box::new(cx.metas.zonk_term(expected))
}

#[inline(never)]
fn infer_dependent_match_scrutinee(
    cx: &mut ElabCtx,
    scrutinee: &RExpr,
) -> Result<(Box<Term>, Box<Term>), ElabError> {
    let (core, inferred) = infer(cx, scrutinee)?;
    let inferred = whnf(cx.env, &cx.ctx, &inferred);
    Ok((Box::new(core), Box::new(inferred)))
}

/// Check `match scrut { C₁ p… => e₁ ; … }` against a KNOWN `expected` goal
/// that may reference the scrutinee (a per-branch-varying `Ω`- or `Type`-
/// motive) — the K4/AC4 dependent-elimination path. Only FLAT constructor
/// patterns are supported (no nested constructor sub-patterns), deliberately
/// narrower than `infer_match`'s general nested-pattern compiler.
#[inline(never)]
fn check_match_dependent(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    equation: Option<&str>,
    arms: &[RMatchArm],
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let hidden_group_result_refinement = equation.is_none()
        && !cx.recursive_group.is_empty()
        && arms
            .iter()
            .any(|arm| expression_mentions_recursive_group(cx, &arm.body));
    if hidden_group_result_refinement {
        check_match_dependent_mode::<true>(cx, scrut, equation, arms, expected, span)
    } else {
        check_match_dependent_mode::<false>(cx, scrut, equation, arms, expected, span)
    }
}

#[inline(never)]
fn check_match_dependent_mode<const MAY_REFINE_GROUP_RESULT: bool>(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    equation: Option<&str>,
    arms: &[RMatchArm],
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    for arm in arms {
        ensure_pattern_constructors_resolve(cx, &arm.pat)?;
    }
    // Zonk `expected` up front: a bare surface `(a : Type)` parameter's own
    // TYPE may still carry an unresolved universe metavariable at this point
    // (pinned to `Type 0` only once something concrete unifies against it,
    // which can happen LATER in the body than this function runs) — the
    // kernel has no notion of elaborator metavariables (`Level::Var` is just
    // an opaque, non-zero level to it), so an unzonked `expected` applying
    // that parameter to a family whose own param is concretely `Type 0`
    // (every surface `data`, `data.rs`) surfaces as a spurious `TypeMismatch
    // {expected: Type 0, found: Type <meta>}` the moment `kernel_infer` (or
    // any downstream kernel check) looks at it or its shape shows up inside
    // a reconstructed method/motive. This was always latent here — masked
    // before because only NULLARY families reached `check_match_dependent`,
    // and none of those goals closed over a still-unresolved generic type
    // parameter this early.
    let expected_zonked = zonked_dependent_expected(cx, expected);
    let original_expected = expected_zonked.as_ref();
    let (scrut_core, scrut_ty) = infer_dependent_match_scrutinee(cx, scrut)?;

    let (d_id, family_level_args, scrut_args) =
        dependent_scrutinee_family(&scrut_ty, span)?;
    let ind = dependent_inductive(cx.env, d_id)?;
    ensure_arm_ctors_belong_to_family(cx, arms, &ind, d_id)?;
    if equation.is_some()
        && (ind.indices.len() != 0 || ind.constructors.iter().any(|ctor| !ctor.args.is_empty()))
    {
        return Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "`match ... eqn:` only supports finite enums with nullary constructors".into(),
        });
    }
    let m = ind.params.len();
    let n_i = ind.indices.len();
    if scrut_args.len() != m + n_i {
        return Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "match scrutinee has the wrong number of family arguments".into(),
        });
    }
    let mut params_terms = Box::new(scrut_args);
    let scrut_indices = Box::new(params_terms.split_off(m));
    let recursive_field_index_path = recursive_field_index_path(
        cx,
        &ind,
        &params_terms,
        &family_level_args,
        &scrut_core,
        &scrut_indices,
    )?;

    // A source field paired with residual generated `All` evidence is
    // eliminated through that evidence. Its recorded source index keeps the
    // host constructor and support constructor aligned without exposing the
    // generated family at the surface.
    if equation.is_none() {
        if let Term::Var(index) = scrut_core.as_ref() {
            if let Some(position) = cx.ctx.len().checked_sub(1 + *index) {
                if let Some(binding) = cx.lift_bindings.get(&position).copied() {
                    if binding.support.is_some() {
                        return check_match_with_lift(
                            cx,
                            arms,
                            expected,
                            span,
                            &Term::var(*index),
                            &ind,
                            &family_level_args,
                            &params_terms,
                            binding,
                        );
                    }
                }
            }
        }
    }

    // The motive: `expected` with the elaborated scrutinee abstracted to the
    // final `D p̄ ī` binder. Genuinely coupled indexed families additionally
    // return a telescope of branch-local equalities
    // `Eq I_j i_j i0_j -> ...`; the completed elim is applied to generated
    // reflexivity evidence at the actual indices. A constructor-declared
    // shifting recursive field instead takes the plain eliminator path: its
    // motive abstracts the index directly and carries no equality telescope.
    let motive_base_depth = n_i + 1;
    let sentinel_region = cx.match_field_regions.len();
    let motive_local_indices: Vec<Term> = (0..n_i).map(|j| Term::var(n_i - j)).collect();
    let mut motive_plan = plan_coherent_frame_motive(
        cx,
        &ind,
        &params_terms,
        &scrut_core,
        &scrut_indices,
        original_expected,
        motive_base_depth,
        &motive_local_indices,
        recursive_field_index_path,
        sentinel_region,
        !ind.indices.is_empty()
            && arms
                .iter()
                .any(|arm| matches!(arm.body, RExpr::RMatch { .. })),
        span,
    )?;
    let mut motive_user_body =
        std::mem::replace(&mut motive_plan.motive_user_body, Term::Type(Level::Zero));
    let expected = &motive_plan.expected;
    let context_convoy = motive_plan.context_convoy.as_slice();
    let embedded_method_convoy = motive_plan.embedded_method_convoy.as_slice();
    let embedded_method_repairs = motive_plan.embedded_method_repairs.as_slice();
    let equation_convoy = motive_plan.equation_convoy;
    let recursive_field_index_path = motive_plan.recursive_field_index_path;
    let zonked_ctx = Context {
        types: cx
            .ctx
            .types
            .iter()
            .map(|term| cx.metas.zonk_term(term))
            .collect(),
    };
    let motive_ctx = motive_context(&zonked_ctx, &ind, &params_terms);
    // Context-telescope convoy (LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE):
    // the ordered forward-dependency closure of ambient bindings whose type the
    // index refinement changes (`context_convoy`, computed above) travels WITH the
    // scrutinee as a motive-codomain telescope, so a captured `xs : Env n` follows
    // the constructor's index in every method and is reconstructed at the actual
    // indices after the Elim. This REPLACES capability 2 (the sibling-outer-binding
    // scan) of install_index_refinements. Empty when nothing captures the index
    // (uncoupled Vec `map`, whose scrutinee IS the env) — then the old
    // scrutinee-only shape.
    // Generalize the convoy into the motive codomain as one telescope, ambient
    // order preserved (`xs'` outer, `h' : P xs'` inner). Each binder type is
    // rebased by the root pairs (scrutinee index -> LOCAL index at the base motive
    // frame, scrutinee -> the scrutinee binder) and threads every OTHER convoy
    // binder through its sentinel; the goal's ambient reference becomes its
    // binder's sentinel. `wrap_premise_pis_finalized` relocates every sentinel to
    // its real de Bruijn position in both the goal (codomain) and each binder type
    // (a transitive `h : Wit n xs` naming the outer `xs`), and shifts each type's
    // free references by its telescope position. A single depth-fixed
    // `subst_term_generalize` per binder would misplace an ambient reference that
    // an earlier-built binder type nests at a different depth. Only the Elim
    // itself, kernel-validated, certifies the assembled shape.
    let convoy_count = context_convoy.len();
    let context_convoy_embedded_in_large_selector = equation_convoy && convoy_count > 0;
    if context_convoy_embedded_in_large_selector {
        debug_assert!(embedded_method_convoy.is_empty());
    } else if convoy_count > 0 || !embedded_method_convoy.is_empty() {
        // Same ONE plan as the methods/IH: local indices `Var(n_i - j)` and the
        // scrutinee binder `Var(0)` are this frame's targets. Embedded methods
        // are appended after the ambient convoy, so their sentinels and types
        // share the same finalized telescope rather than being wrapped by an
        // independent pass.
        let (cv_types_inner_first, sentinels) = convoy_binder_types(
            &context_convoy,
            &scrut_indices,
            &scrut_core,
            &motive_local_indices,
            &Term::var(0),
            motive_base_depth,
            0,
            sentinel_region,
        );
        motive_user_body = redirect_convoy_body(
            &context_convoy,
            motive_base_depth,
            &sentinels,
            motive_user_body,
        );
        let mut convoy_premises: Vec<Term> =
            Vec::with_capacity(convoy_count + embedded_method_convoy.len());
        for i in (0..convoy_count).rev() {
            convoy_premises.push(cv_types_inner_first[i].clone());
        }
        for entry in embedded_method_convoy {
            convoy_premises.push(redirect_convoy_body(
                &context_convoy,
                motive_base_depth,
                &sentinels,
                entry.motive_ty.clone(),
            ));
        }
        motive_user_body =
            wrap_premise_pis_finalized(motive_user_body, &convoy_premises, sentinel_region);
    }
    let hidden_group_result_refinement = MAY_REFINE_GROUP_RESULT
        && ind.indices.is_empty()
        // The matched carrier must itself be an index domain of the result
        // family. This excludes unrelated control matches (for example Bool
        // guards around a FokSequent-indexed derivation), preserving their
        // original definitional computation and SCT presentation.
        && term_mentions_family_indexed_by(cx.env, &cx.ctx, expected, &scrut_ty)
        // A whole-scrutinee equation is a lawful hidden result refinement only
        // for non-recursive matches. On a recursive family it would also alter
        // each IH from `M child` to `child = parent -> M child`, an unusable and
        // false premise. Recursive IH slots retain their existing index-only
        // refinement machinery.
        && ind.constructors.iter().all(|ctor| {
            recursive_shapes(cx.env, ctor, d_id, m).is_ok_and(|shapes| shapes.is_empty())
        });
    if equation.is_some() {
        // The eliminator returns a function over the branch equation.  Its
        // methods can therefore bind the surface `eqn:` name, while applying
        // the completed eliminator to `Refl` recovers the author's goal.
        let eq_dom = Term::Eq(
            Box::new(weaken(&scrut_ty, 1)),
            Box::new(weaken(&scrut_core, 1)),
            Box::new(Term::var(0)),
        );
        motive_user_body = Term::pi(eq_dom, weaken(&motive_user_body, 1));
    } else if hidden_group_result_refinement {
        // Keep the matched scrutinee's propositional refinement internal. The
        // method receives `concrete = outer`; recursive-group calls may use its
        // symmetric direction to transport a concrete indexed result back to
        // the outer refined index. The completed eliminator supplies `Refl`.
        let eq_dom = Term::Eq(
            Box::new(weaken(&scrut_ty, motive_base_depth as i64)),
            Box::new(Term::var(0)),
            Box::new(weaken(&scrut_core, motive_base_depth as i64)),
        );
        motive_user_body = Term::pi(eq_dom, weaken(&motive_user_body, 1));
    }
    let motive = build_checked_dependent_motive(
        cx.env,
        &motive_ctx,
        &ind,
        d_id,
        &params_terms,
        &scrut_indices,
        motive_user_body,
        equation_convoy,
        recursive_field_index_path,
        span,
    )?;

    let mut methods = Box::new(vec![None; ind.constructors.len()]);
    let mut arm_used = Box::new(vec![false; arms.len()]);
    let mut subsumed_by = Box::new(vec![None; arms.len()]);
    for (k, ctor) in ind.constructors.iter().enumerate() {
        let arm_idx = arms
            .iter()
            .position(|a| matches!(&a.pat.kind, RPatKind::Ctor(name, _) if cx.globals.get(name).copied() == Some(ctor.id)));
        let n = ctor.args.len();
        if let Some(arm_idx) = arm_idx {
            arm_used[arm_idx] = true;
            mark_shared_ctor_subsumption(cx, arms, ctor.id, arm_idx, &mut subsumed_by);
            let arm = &arms[arm_idx];
            let sub_pats = match &arm.pat.kind {
                RPatKind::Ctor(_, subs) => subs.clone(),
                _ => unreachable!("guarded by the position() match above"),
            };
            if sub_pats.len() != n {
                return Err(ElabError::Internal(
                    "dependent match (AC4): constructor arity mismatch".into(),
                ));
            }
            for sp in &sub_pats {
                if !matches!(sp.kind, RPatKind::Var(_) | RPatKind::Wild) {
                    return Err(ElabError::Internal(
                        "dependent match (AC4): nested constructor sub-patterns are not \
                         yet supported here"
                            .into(),
                    ));
                }
            }
        }
        let shapes = Box::new(
            recursive_shapes(cx.env, ctor, d_id, m).map_err(|error| {
                ElabError::KernelRejected {
                    error,
                    span: span.clone(),
                }
            })?,
        );
        if shapes
            .iter()
            .any(|argument| argument.shape.as_legacy().is_none())
        {
            if equation.is_some() {
                return Err(ElabError::Internal(
                    "`match ... eqn:` does not support nested lifted methods".into(),
                ));
            }
            let arm_idx = arm_idx.ok_or_else(|| ElabError::ExhaustivenessError {
                missing: missing_pattern_witness(cx, ctor.id),
                span: span.clone(),
            })?;
            methods[k] = Some(check_structured_constructor_method(
                cx,
                &ind,
                k,
                &arms[arm_idx],
                expected,
                &scrut_core,
                &params_terms,
                &motive,
                &family_level_args,
                &shapes,
            )?);
            continue;
        }
        for j in 0..n {
            let raw_ty = subst_levels(
                &subst_outer(&ctor.args[j], m, &params_terms, j),
                &ind.level_params,
                &family_level_args,
            );
            cx.ctx.push(raw_ty);
        }
        let constructor_frame = build_dependent_constructor_frame(
            cx,
            &ind,
            ctor,
            &params_terms,
            &family_level_args,
            &scrut_indices,
            &scrut_ty,
            &scrut_core,
            expected,
            &motive,
            n,
            sentinel_region,
            context_convoy,
            embedded_method_convoy,
            embedded_method_repairs,
            hidden_group_result_refinement,
            equation_convoy,
            recursive_field_index_path,
            span,
        )?;
        let concrete = constructor_frame.concrete.as_ref();
        let target_indices = constructor_frame.target_indices.as_slice();
        let premise_domains = constructor_frame.premise_domains.as_slice();
        let expected_here = &constructor_frame.expected_here;
        let convoy_refinements = constructor_frame.convoy_refinements.as_slice();
        let hidden_result_premise_slot = constructor_frame.hidden_result_premise_slot;
        let method = if let Some(arm_idx) = arm_idx {
            let arm = &arms[arm_idx];
            if equation_convoy && matches!(&arm.body, RExpr::RCon(name, _) if name == SUGAR_REFL) {
                Term::Const {
                    id: cx.env.tt_id(),
                    level_args: Vec::new(),
                }
            } else if equation_convoy && expression_mentions_recursive_group(cx, &arm.body) {
                build_large_convoy_recursive_method(
                    cx,
                    arm,
                    &ind,
                    &params_terms,
                    &target_indices,
                    &scrut_indices,
                    n,
                    &expected_here,
                    sentinel_region,
                    &premise_domains,
                )?
            } else if equation.is_some() {
                let eq_dom = Term::Eq(
                    Box::new(weaken(&scrut_ty, n as i64)),
                    Box::new(weaken(&scrut_core, n as i64)),
                    Box::new(concrete.clone()),
                );
                cx.ctx.push(eq_dom.clone());
                let body = check(cx, &arm.body, &weaken(&expected_here, 1), &arm.span)?;
                cx.ctx.pop();
                Term::lam(eq_dom, body)
            } else {
                check_dependent_branch_body(
                    cx,
                    arm,
                    &ind,
                    &params_terms,
                    &family_level_args,
                    &target_indices,
                    &scrut_indices,
                    n,
                    &expected_here,
                    &premise_domains,
                    hidden_result_premise_slot,
                    &scrut_ty,
                    &concrete,
                    &scrut_core,
                    sentinel_region,
                    &convoy_refinements,
                    equation_convoy,
                    recursive_field_index_path,
                )?
            }
        } else {
            let expected_here = simplify_branch_goal(cx.env, &cx.ctx, &expected_here);
            let missing = missing_pattern_witness(cx, ctor.id);
            synthesize_omitted_index_method(
                cx,
                &premise_domains,
                &expected_here,
                sentinel_region,
                missing,
                span,
            )?
        };
        for _ in 0..n {
            cx.ctx.pop();
        }

        methods[k] = Some(finish_dependent_constructor_method(
            cx,
            span,
            &shapes,
            n,
            expected,
            equation_convoy,
            recursive_field_index_path,
            &scrut_core,
            &ind,
            &params_terms,
            &scrut_indices,
            m,
            ctor,
            sentinel_region,
            context_convoy,
            embedded_method_convoy,
            embedded_method_repairs,
            method,
            arm_idx,
            arms,
            k,
            &motive,
        )?);
    }
    for (i, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[i] {
                Some(claimant) => ArmDeadCause::Subsumed {
                    first: arms[claimant].span.clone(),
                    rest: Vec::new(),
                },
                None => ArmDeadCause::NoInhabitants,
            };
            return Err(ElabError::ReachabilityError {
                span: arms[i].span.clone(),
                cause,
            });
        }
    }
    let methods: Vec<Term> = (*methods)
        .into_iter()
        .map(|m| m.expect("every ctor bucket filled above"))
        .collect();
    finish_dependent_elim(
        cx,
        d_id,
        &ind,
        *params_terms,
        *motive,
        methods,
        &scrut_indices,
        &scrut_ty,
        &scrut_core,
        context_convoy,
        embedded_method_convoy,
        equation.is_some() || hidden_group_result_refinement,
        recursive_field_index_path,
        span,
    )
}

fn motive_context(outer: &Context, ind: &InductiveDecl, params: &[Term]) -> Context {
    let mut ctx = outer.clone();
    for j in 0..ind.indices.len() {
        ctx.push(subst_outer(&ind.indices[j], ind.params.len(), params, j));
    }
    ctx.push(indexed_scrutinee_type(ind, ind.id, params));
    ctx
}

fn motive_context_at(
    outer: &Context,
    ind: &InductiveDecl,
    params: &[Term],
    level_args: &[Level],
) -> Context {
    let mut ctx = outer.clone();
    for j in 0..ind.indices.len() {
        ctx.push(subst_levels(
            &subst_outer(&ind.indices[j], ind.params.len(), params, j),
            &ind.level_params,
            level_args,
        ));
    }
    ctx.push(indexed_scrutinee_type_at(ind, ind.id, params, level_args));
    ctx
}

fn motive_type(ind: &InductiveDecl, d_id: GlobalId, params: &[Term], motive_sort: &Term) -> Term {
    let mut ty = Term::pi(
        indexed_scrutinee_type(ind, d_id, params),
        motive_sort.clone(),
    );
    for j in (0..ind.indices.len()).rev() {
        ty = Term::pi(
            subst_outer(&ind.indices[j], ind.params.len(), params, j),
            ty,
        );
    }
    ty
}

fn wrap_motive_lambdas(ind: &InductiveDecl, d_id: GlobalId, params: &[Term], body: Term) -> Term {
    let mut term = Term::lam(indexed_scrutinee_type(ind, d_id, params), body);
    for j in (0..ind.indices.len()).rev() {
        term = Term::lam(
            subst_outer(&ind.indices[j], ind.params.len(), params, j),
            term,
        );
    }
    term
}

fn motive_type_at(
    ind: &InductiveDecl,
    d_id: GlobalId,
    params: &[Term],
    motive_sort: &Term,
    level_args: &[Level],
) -> Term {
    let mut ty = Term::pi(
        indexed_scrutinee_type_at(ind, d_id, params, level_args),
        motive_sort.clone(),
    );
    for j in (0..ind.indices.len()).rev() {
        ty = Term::pi(
            subst_levels(
                &subst_outer(&ind.indices[j], ind.params.len(), params, j),
                &ind.level_params,
                level_args,
            ),
            ty,
        );
    }
    ty
}

fn wrap_motive_lambdas_at(
    ind: &InductiveDecl,
    d_id: GlobalId,
    params: &[Term],
    body: Term,
    level_args: &[Level],
) -> Term {
    let mut term = Term::lam(
        indexed_scrutinee_type_at(ind, d_id, params, level_args),
        body,
    );
    for j in (0..ind.indices.len()).rev() {
        term = Term::lam(
            subst_levels(
                &subst_outer(&ind.indices[j], ind.params.len(), params, j),
                &ind.level_params,
                level_args,
            ),
            term,
        );
    }
    term
}

fn indexed_scrutinee_type(ind: &InductiveDecl, d_id: GlobalId, params: &[Term]) -> Term {
    indexed_scrutinee_type_at(ind, d_id, params, &[])
}

fn indexed_scrutinee_type_at(
    ind: &InductiveDecl,
    d_id: GlobalId,
    params: &[Term],
    level_args: &[Level],
) -> Term {
    let n_i = ind.indices.len();
    let mut d_app = Term::IndFormer {
        id: d_id,
        level_args: level_args.to_vec(),
    };
    for p in params {
        d_app = Term::app(d_app, weaken(p, n_i as i64));
    }
    for j in 0..n_i {
        d_app = Term::app(d_app, Term::var(n_i - 1 - j));
    }
    d_app
}

fn motive_index_premises(
    ind: &InductiveDecl,
    params: &[Term],
    scrut_indices: &[Term],
) -> Vec<Term> {
    let n_i = ind.indices.len();
    (0..n_i)
        .filter_map(|j| {
            let raw_index_ty = subst_outer(&ind.indices[j], ind.params.len(), params, j);
            // Later dependent index domains would require heterogeneous
            // transport through earlier equality premises; do not emit an
            // ill-typed ordinary Eq premise for those cases.
            if index_domain_mentions_prior_index(&raw_index_ty, j) {
                return None;
            }
            let index_ty = ken_kernel::subst::shift(&raw_index_ty, (n_i - j + 1) as i64, 0);
            let abstract_index = Term::var(n_i - j);
            let actual_index = weaken(&scrut_indices[j], (n_i + 1) as i64);
            Some(Term::Eq(
                Box::new(index_ty),
                Box::new(abstract_index),
                Box::new(actual_index),
            ))
        })
        .collect()
}

fn ctor_target_indices(
    ctor: &ConstructorDecl,
    ind: &InductiveDecl,
    params: &[Term],
    level_args: &[Level],
    field_count: usize,
) -> Vec<Term> {
    ctor.target_indices
        .iter()
        .map(|term| {
            subst_levels(
                &subst_outer(term, ind.params.len(), params, field_count),
                &ind.level_params,
                level_args,
            )
        })
        .collect()
}

/// Per-index `(index_ty, target_index, actual_index)` triples for a
/// constructor method — the same filter `method_index_premises` uses to
/// build `Eq` premises, exposed separately so the injectivity / convoy pass
/// can inspect each index's own endpoints rather than only the wrapped `Eq`
/// term.
fn method_index_premise_pairs(
    ind: &InductiveDecl,
    params: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    field_count: usize,
) -> Vec<(Term, Term, Term)> {
    (0..ind.indices.len())
        .filter_map(|j| {
            let raw_index_ty = subst_outer(&ind.indices[j], ind.params.len(), params, j);
            // Keep constructor/top premise arity aligned with the motive
            // premises above.
            if index_domain_mentions_prior_index(&raw_index_ty, j) {
                return None;
            }
            let index_ty_with_fields =
                ken_kernel::subst::shift(&raw_index_ty, field_count as i64, j);
            let index_ty = subst_tel(&index_ty_with_fields, &target_indices[..j]);
            let actual_index = weaken(&scrut_indices[j], field_count as i64);
            Some((index_ty, target_indices[j].clone(), actual_index))
        })
        .collect()
}

fn method_index_premises(
    ind: &InductiveDecl,
    params: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    field_count: usize,
) -> Vec<Term> {
    method_index_premise_pairs(ind, params, target_indices, scrut_indices, field_count)
        .into_iter()
        .map(|(ty, a, b)| Term::Eq(Box::new(ty), Box::new(a), Box::new(b)))
        .collect()
}

/// `refl`'s argument for the base case of a `J` at `Eq ty a a` — the
/// WHNF-peeled form of `a`, matching what `Eq ty a a` itself reduces to
/// (e.g. `Eq Nat (Suc m) (Suc m)` peels one constructor layer to
/// `Eq Nat m m`, via the kernel's own same-constructor `eq_at_inductive`
/// case — so the witness must be `refl m`, not `refl (Suc m)`).
/// `check`'s `Term::Refl` rule WHNFs the *expected type* but never the
/// *supplied witness* (`ken-kernel/check.rs`), so handing it the unpeeled
/// `a` is a silent, arity-invisible mismatch — caught only by the kernel's
/// own recheck, never by elaboration itself (isolated via a direct
/// `kernel_check` probe run in isolation on this witness). `x == y` always holds
/// here (both sides are literally `a`), so extracting either endpoint is
/// safe regardless of how deep the peel goes.
fn refl_base_arg(env: &GlobalEnv, ctx: &Context, ty: &Term, a: &Term) -> Term {
    match whnf(
        env,
        ctx,
        &Term::Eq(
            Box::new(ty.clone()),
            Box::new(a.clone()),
            Box::new(a.clone()),
        ),
    ) {
        Term::Eq(_, x, _) => *x,
        _ => a.clone(),
    }
}

/// `h : Eq idx_ty a b` ⇒ `sym h : Eq idx_ty b a`, derived via `J` — never
/// postulated. Motive `λ(y:idx_ty)(_:Eq idx_ty a y). Eq idx_ty y a`,
/// based at `a` (`base = refl a`); `J` gives the result at `y = b`.
fn build_sym(
    env: &GlobalEnv,
    ctx: &Context,
    idx_ty: &Term,
    idx_level: Level,
    a: &Term,
    h: Term,
) -> Term {
    let dom2 = Term::Eq(
        Box::new(weaken(idx_ty, 1)),
        Box::new(weaken(a, 1)),
        Box::new(Term::var(0)),
    );
    let cod = Term::Eq(
        Box::new(weaken(idx_ty, 2)),
        Box::new(Term::var(1)),
        Box::new(weaken(a, 2)),
    );
    let motive_body = Term::lam(idx_ty.clone(), Term::lam(dom2.clone(), cod));
    // `J`'s motive is an introduction form (`Lam`) — `infer` can never
    // accept one without an ascription, even under `check` (`infer_j` calls
    // `infer(motive)` directly), so every motive we hand-build must be
    // wrapped (`hand-built-elim-motive-and-method-gotchas`).
    let motive_ty = Term::pi(idx_ty.clone(), Term::pi(dom2, Term::omega(idx_level)));
    let motive = Term::Ascript(Box::new(motive_body), Box::new(motive_ty));
    let base = Term::Refl(Box::new(refl_base_arg(env, ctx, idx_ty, a)));
    Term::J(Box::new(motive), Box::new(base), Box::new(h))
}

/// Build `e : Eq Type cur_ty new_ty` where `new_ty = cur_ty[new_idx/old_idx]`,
/// given `h : Eq idx_ty old_idx new_idx` — the type-level congruence a
/// constructor-index equation licenses (index-refinement injectivity /
/// convoy re-typing). Derived via `J`, never postulated: motive
/// `λ(y:idx_ty)(_:Eq idx_ty old_idx y). Eq Type cur_ty cur_ty[y/old_idx]`,
/// based at `old_idx` (`base = refl cur_ty`); `J` gives the result at
/// `y = new_idx`. Returns `(e, new_ty)`.
fn build_index_type_cong(
    env: &GlobalEnv,
    ctx: &Context,
    idx_ty: &Term,
    old_idx: &Term,
    new_idx: &Term,
    cur_ty: &Term,
    type_level: Level,
    h: Term,
) -> (Term, Term) {
    let new_ty = subst_term_generalize(cur_ty, old_idx, new_idx);
    // Motive body, under the two new binders (`y`, `_ : Eq idx_ty old_idx y`):
    // `cur_ty` with `old_idx` abstracted to `y` (= `Var(1)` at this depth).
    let cur_ty_at_y = subst_term_generalize(&weaken(cur_ty, 2), &weaken(old_idx, 2), &Term::var(1));
    let dom2 = Term::Eq(
        Box::new(weaken(idx_ty, 1)),
        Box::new(weaken(old_idx, 1)),
        Box::new(Term::var(0)),
    );
    let cod = Term::Eq(
        Box::new(Term::Type(type_level.clone())),
        Box::new(weaken(cur_ty, 2)),
        Box::new(cur_ty_at_y),
    );
    let motive_body = Term::lam(idx_ty.clone(), Term::lam(dom2.clone(), cod));
    // Ascribed for the same reason `build_sym` is — `J`'s motive is a bare
    // `Lam`, never inferrable on its own. Its own classifier is one level
    // up: `Eq (Type l) _ _ : Omega (suc l)`.
    let motive_ty = Term::pi(
        idx_ty.clone(),
        Term::pi(dom2, Term::omega(type_level.clone().suc())),
    );
    let motive = Term::Ascript(Box::new(motive_body), Box::new(motive_ty));
    let base = Term::Refl(Box::new(refl_base_arg(
        env,
        ctx,
        &Term::Type(type_level),
        cur_ty,
    )));
    let e = Term::J(Box::new(motive), Box::new(base), Box::new(h));
    (e, new_ty)
}

/// Transport `value : cur_ty` directly to `cur_ty[new_idx/old_idx]` when
/// `cur_ty : Ω level`, using `h : Eq idx_ty old_idx new_idx`. The motive is
/// `λ y (_ : Eq idx_ty old_idx y). cur_ty[y/old_idx]`; unlike the Type arm,
/// this transports the inhabitant itself and emits no `Cast`.
fn build_index_omega_transport(
    idx_ty: &Term,
    old_idx: &Term,
    new_idx: &Term,
    cur_ty: &Term,
    omega_level: Level,
    value: Term,
    h: Term,
) -> (Term, Term) {
    let new_ty = subst_term_generalize(cur_ty, old_idx, new_idx);
    let cur_ty_at_y = subst_term_generalize(&weaken(cur_ty, 2), &weaken(old_idx, 2), &Term::var(1));
    let dom2 = Term::Eq(
        Box::new(weaken(idx_ty, 1)),
        Box::new(weaken(old_idx, 1)),
        Box::new(Term::var(0)),
    );
    let motive_body = Term::lam(idx_ty.clone(), Term::lam(dom2.clone(), cur_ty_at_y));
    // `infer_j` infers its motive directly, so the bare lambda needs the
    // ruled explicit sort ascription.
    let motive_ty = Term::pi(idx_ty.clone(), Term::pi(dom2, Term::omega(omega_level)));
    let motive = Term::Ascript(Box::new(motive_body), Box::new(motive_ty));
    let transported = Term::J(Box::new(motive), Box::new(value), Box::new(h));
    (transported, new_ty)
}

/// If `cur_ty` (a type at the branch's current context depth) literally
/// mentions `old_idx`, re-type `value : cur_ty` at
/// `cur_ty[new_idx/old_idx]` using `h : Eq idx_ty old_idx new_idx`. Type-
/// classified positions retain the existing equality-of-types plus `Cast`;
/// Ω-classified positions use direct `J` transport. Returns `None` — never a
/// spurious refinement (AC8) — if `cur_ty` does not depend on `old_idx` at all.
fn try_reindex_cast(
    env: &GlobalEnv,
    ctx: &Context,
    idx_ty: &Term,
    old_idx: &Term,
    new_idx: &Term,
    cur_ty: &Term,
    value: Term,
    h: Term,
) -> Result<Option<(Term, Term)>, ElabError> {
    let candidate_new_ty = subst_term_generalize(cur_ty, old_idx, new_idx);
    if &candidate_new_ty == cur_ty {
        return Ok(None);
    }
    let level_ty = kernel_infer(env, ctx, cur_ty).map_err(|e| {
        ElabError::Internal(format!(
            "index refinement: could not classify a re-indexed position's type: {e:?}"
        ))
    })?;
    match whnf(env, ctx, &level_ty) {
        Term::Type(level) => {
            let (e, new_ty) =
                build_index_type_cong(env, ctx, idx_ty, old_idx, new_idx, cur_ty, level, h);
            let cast = Term::Cast(
                Box::new(cur_ty.clone()),
                Box::new(new_ty.clone()),
                Box::new(e),
                Box::new(value),
            );
            Ok(Some((cast, new_ty)))
        }
        Term::Omega(level) => {
            let (transported, new_ty) = build_index_omega_transport(
                idx_ty, old_idx, new_idx, cur_ty, level, value, h,
            );
            Ok(Some((transported, new_ty)))
        }
        other => Err(ElabError::Internal(format!(
            "index refinement: re-indexed position is classified by neither Type nor Omega, found {other:?}"
        ))),
    }
}

/// Capability 3: does the branch's own CHECKING GOAL (not a context
/// variable) depend on the scrutinee's un-refined outer index? A branch
/// that constructs a FRESH value (e.g. `VNil Nat` against goal `Vec Nat n`,
/// `zip`'s base case) needs the goal itself refined (each index's outer
/// value substituted for the constructor's own target value) before
/// `check` can succeed at all — capability 1/2 only re-type EXISTING
/// context variables, never the goal `check` runs against, so a branch
/// whose body never re-uses an existing field/sibling (like `tail`'s or
/// `firstIsSecond`'s) never exercises this gap. Returns the (possibly
/// more-refined) goal to check the body against, plus a producer-classified
/// restoration plan needed to bring the CHECKED result back up to the original
/// `expected_here`, to be applied in REVERSE order (innermost/most-refined
/// first). Replay dispatches on the tag and never re-classifies a goal.
fn refine_branch_goal(
    cx: &ElabCtx,
    ind: &InductiveDecl,
    params: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    n: usize,
    expected_here: &Term,
) -> Result<(Term, Vec<BranchGoalRestoration>), ElabError> {
    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let pairs = method_index_premise_pairs(ind, params, target_indices, scrut_indices, n);
    let sentinel_region = cx.match_field_regions.len().saturating_sub(1);
    // Complete every evidence walk before refining the goal. An unsupported
    // child therefore rejects the whole plan, even when an earlier declared
    // index or Sigma child had usable Eq leaves.
    let mut leaves = Vec::new();
    for (slot, (idx_ty, target, scrut)) in pairs.iter().enumerate() {
        let raw_eq = Term::Eq(
            Box::new(cx.metas.zonk_term(idx_ty)),
            Box::new(cx.metas.zonk_term(target)),
            Box::new(cx.metas.zonk_term(scrut)),
        );
        leaves.extend(project_generated_index_equality_leaves(
            cx.env,
            &zonked_ctx,
            &raw_eq,
            index_refinement_sentinel(sentinel_region, slot),
        )?);
    }

    let mut goal = expected_here.clone();
    let mut restorations = Vec::new();
    for leaf in leaves {
        let candidate = subst_term_generalize(&goal, &leaf.scrutinee, &leaf.target);
        if candidate == goal {
            continue;
        }
        let level_ty = kernel_infer(cx.env, &zonked_ctx, &candidate).map_err(|e| {
            ElabError::Internal(format!(
                "index refinement: could not classify the branch goal: {e:?}"
            ))
        })?;
        let classifier = whnf(cx.env, &zonked_ctx, &level_ty);
        let restoration = classify_branch_goal_restoration(
            cx.env,
            &zonked_ctx,
            &leaf.index_ty,
            &leaf.target,
            &leaf.scrutinee,
            &candidate,
            classifier,
            leaf.proof,
        )?;
        restorations.push(restoration);
        goal = candidate;
    }
    Ok((goal, restorations))
}

/// Install `var_refinements` for one branch of a dependent match —
/// constructor injectivity on the branch's own peeled recursive fields
/// (capability 1), and sibling convoy on any outer binder sharing the
/// refined index (capability 2). `cx.ctx` holds exactly this branch's `n`
/// constructor fields (unchanged — see the caller's comment on why the
/// premises themselves are never pushed); returns the installed
/// bottom-relative positions so the caller can remove them once the
/// branch body has been checked. Each proof embedded into a `var_refinements`
/// entry references its premise via an `INDEX_REFINEMENT_SENTINEL_BASE`-
/// tagged placeholder that `finalize_refined_body` resolves afterward.
fn install_hidden_result_variable_refinements(
    cx: &mut ElabCtx,
    index_ty: &Term,
    concrete_index: &Term,
    refined_index: &Term,
    premise_slot: usize,
    outer_scope_depth: usize,
) -> Result<Vec<usize>, ElabError> {
    if outer_scope_depth == 0 {
        return Ok(Vec::new());
    }
    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let index_ty = cx.metas.zonk_term(index_ty);
    let concrete_index = cx.metas.zonk_term(concrete_index);
    let refined_index = cx.metas.zonk_term(refined_index);
    let index_level_ty = kernel_infer(cx.env, &zonked_ctx, &index_ty).map_err(|error| {
        ElabError::Internal(format!(
            "result refinement: could not classify the matched type: {error:?}"
        ))
    })?;
    match whnf(cx.env, &zonked_ctx, &index_level_ty) {
        Term::Type(_) => {}
        other => {
            return Err(ElabError::Internal(format!(
                "result refinement: matched type is not classified by Type, found {other:?}"
            )))
        }
    }

    let sentinel_region = cx.match_field_regions.len().saturating_sub(1);
    let raw_eq = Term::Eq(
        Box::new(index_ty),
        Box::new(concrete_index),
        Box::new(refined_index),
    );
    let leaves = project_generated_index_equality_leaves(
        cx.env,
        &zonked_ctx,
        &raw_eq,
        index_refinement_sentinel(sentinel_region, premise_slot),
    )?;
    // The hidden premise is oriented `concrete = refined`, while outer
    // bindings are retyped from the refined scrutinee back to the constructor.
    // Build one symmetric proof per projected Eq leaf, never one J over the
    // whole observational Sigma.
    let mut symmetric_leaves = Vec::with_capacity(leaves.len());
    for leaf in leaves {
        let level_ty = kernel_infer(cx.env, &zonked_ctx, &leaf.index_ty).map_err(|error| {
            ElabError::Internal(format!(
                "result refinement: could not classify a projected index type: {error:?}"
            ))
        })?;
        let level = match whnf(cx.env, &zonked_ctx, &level_ty) {
            Term::Type(level) => level,
            other => {
                return Err(ElabError::Internal(format!(
                    "result refinement: projected index is not classified by Type, found {other:?}"
                )))
            }
        };
        let proof_sym = build_sym(
            cx.env,
            &zonked_ctx,
            &leaf.index_ty,
            level,
            &leaf.target,
            leaf.proof.clone(),
        );
        symmetric_leaves.push((leaf, proof_sym));
    }

    let mut installed = Vec::new();
    for position in 0..outer_scope_depth {
        if cx
            .match_field_regions
            .iter()
            .any(|region| region.contains(&position))
            || cx.var_refinements.contains_key(&position)
        {
            continue;
        }
        let index = cx.ctx.len() - 1 - position;
        let outer_ty = cx.metas.zonk_term(&weaken(
            cx.ctx
                .lookup(index)
                .expect("outer result-refinement position in range"),
            (index + 1) as i64,
        ));
        let outer_classifier = whnf(
            cx.env,
            &zonked_ctx,
            &kernel_infer(cx.env, &zonked_ctx, &outer_ty).map_err(|error| {
                ElabError::Internal(format!(
                    "result refinement: could not classify an outer binding: {error:?}"
                ))
            })?,
        );
        match outer_classifier {
            Term::Type(_) | Term::Omega(_) => {}
            // Admitted declaration binders are classified by Type or Omega,
            // but an expression-position dependent Pi temporarily extends the
            // context before kernel inference validates the completed Pi. A
            // dependent match nested there can therefore observe an inferable
            // non-sort domain. Preserve the existing silent skip; final Pi
            // admission rejects the malformed domain.
            other => {
                let _defensive_non_sort = other;
                continue;
            }
        }

        let mut value = Term::var(index);
        let mut value_ty = outer_ty;
        let mut changed = false;
        for (leaf, proof_sym) in &symmetric_leaves {
            if let Some((cast, cast_ty)) = try_reindex_cast(
                cx.env,
                &zonked_ctx,
                &leaf.index_ty,
                &leaf.scrutinee,
                &leaf.target,
                &value_ty,
                value.clone(),
                proof_sym.clone(),
            )? {
                value = cast;
                value_ty = cast_ty;
                changed = true;
            }
        }
        if changed {
            cx.var_refinements
                .insert(position, (value, value_ty, cx.ctx.len()));
            installed.push(position);
        }
    }
    Ok(installed)
}

fn install_index_refinements(
    cx: &mut ElabCtx,
    ind: &InductiveDecl,
    params: &[Term],
    target_indices: &[Term],
    scrut_indices: &[Term],
    n: usize,
    outer_scope_depth: usize,
) -> Result<Vec<usize>, ElabError> {
    // Zonk a throwaway copy of the context (and every term this function
    // hands to the raw kernel): a bare surface `(a:Type)` parameter's own
    // type may still carry an unresolved elaborator level metavariable here.
    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let pairs = method_index_premise_pairs(ind, params, target_indices, scrut_indices, n);
    let sentinel_region = cx.match_field_regions.len().saturating_sub(1);

    // Build the complete leaf plan before mutating `var_refinements`. This is
    // atomic across every declared-index premise: an unsupported child cannot
    // leave earlier Eq components installed in the live elaboration context.
    let mut leaves = Vec::new();
    for (slot, (idx_ty, target, scrut)) in pairs.iter().enumerate() {
        let raw_eq = Term::Eq(
            Box::new(cx.metas.zonk_term(idx_ty)),
            Box::new(cx.metas.zonk_term(target)),
            Box::new(cx.metas.zonk_term(scrut)),
        );
        leaves.extend(project_generated_index_equality_leaves(
            cx.env,
            &zonked_ctx,
            &raw_eq,
            index_refinement_sentinel(sentinel_region, slot),
        )?);
    }

    // Capability 2's per-binding sibling-Cast retyping is replaced by the
    // context-telescope convoy. What remains here is its decision-5 fail-closed
    // guard, now applied to every projected index type rather than to an
    // unreduced record type.
    if outer_scope_depth > 0 {
        for leaf in &leaves {
            let level_ty = kernel_infer(cx.env, &zonked_ctx, &leaf.index_ty).map_err(|e| {
                ElabError::Internal(format!(
                    "index refinement: could not classify an index type: {e:?}"
                ))
            })?;
            match whnf(cx.env, &zonked_ctx, &level_ty) {
                Term::Type(_) => {}
                other => {
                    return Err(ElabError::Internal(format!(
                        "index refinement: index type is not classified by a Type universe, found {other:?}"
                    )))
                }
            }
        }
    }

    // Capability 1: chain every projected component refinement through each
    // constructor field. A field may depend on several components of the same
    // record, so each successful transport becomes the next leaf's input.
    let mut installed = Vec::new();
    for field_j in 0..n {
        let field_pos = n - 1 - field_j;
        let field_ty = cx.metas.zonk_term(&weaken(
            cx.ctx
                .lookup(field_pos)
                .expect("field position just pushed"),
            (field_pos as i64) + 1,
        ));
        let mut value = Term::var(field_pos);
        let mut value_ty = field_ty;
        let mut changed = false;
        for leaf in &leaves {
            if let Some((cast, new_ty)) = try_reindex_cast(
                cx.env,
                &zonked_ctx,
                &leaf.index_ty,
                &leaf.target,
                &leaf.scrutinee,
                &value_ty,
                value.clone(),
                leaf.proof.clone(),
            )? {
                value = cast;
                value_ty = new_ty;
                changed = true;
            }
        }
        if changed {
            let bottom_pos = cx.ctx.len() - 1 - field_pos;
            cx.var_refinements
                .insert(bottom_pos, (value, value_ty, cx.ctx.len()));
            installed.push(bottom_pos);
        }
    }

    Ok(installed)
}

/// A `Var` index in this range is an index-refinement sentinel — a
/// placeholder for a premise binder that does not exist yet at the point
/// it's embedded (`install_index_refinements` runs before the branch's
/// premises become real λs). `INDEX_REFINEMENT_SENTINEL_BASE + slot` is
/// astronomically larger than any real nesting depth a Ken program could
/// reach, so it can never collide with a genuine `Var`.
const INDEX_REFINEMENT_SENTINEL_BASE: usize = 1 << 40;
const INDEX_REFINEMENT_SENTINEL_STRIDE: usize = 1 << 20;

fn index_refinement_sentinel(region: usize, slot: usize) -> Term {
    Term::var(INDEX_REFINEMENT_SENTINEL_BASE + region * INDEX_REFINEMENT_SENTINEL_STRIDE + slot)
}

/// Resolve a checked branch body's index-refinement sentinels to their true
/// index and shift every other free variable by `premise_count` — together
/// replicating, in one binder-aware pass, what the (now premise-aware)
/// `wrap_premise_lams_from_full` needs `body` to already satisfy. `depth`
/// counts binders traversed so far from `body`'s own root (start at `0`);
/// a `Var(v)` with `v < depth` is bound within the term itself (untouched);
/// otherwise, if `v - depth` is a sentinel for slot `s`, replace it with
/// `depth + premise_count - 1 - s` (a premise's true wrap-relative index,
/// counted from wherever it's referenced); otherwise it is ordinary
/// pre-existing content (a field/outer reference) and gets the standard
/// `+premise_count` shift `wrap_premise_lams_from_full`'s callers used to
/// apply via `weaken`. Exhaustive over every `Term` variant — no catch-all
/// — so a future variant forces this traversal to be extended too.
fn finalize_refined_body(
    term: &Term,
    depth: usize,
    premise_count: usize,
    sentinel_region: usize,
) -> Term {
    let go = |t: &Term, d: usize| finalize_refined_body(t, d, premise_count, sentinel_region);
    match term {
        Term::Var(v) => {
            if *v < depth {
                Term::Var(*v)
            } else {
                let canonical = *v - depth;
                let region_start = INDEX_REFINEMENT_SENTINEL_BASE
                    + sentinel_region * INDEX_REFINEMENT_SENTINEL_STRIDE;
                if canonical >= region_start
                    && canonical < region_start + INDEX_REFINEMENT_SENTINEL_STRIDE
                {
                    let slot = canonical - region_start;
                    let resolved = premise_count
                        .checked_sub(1 + slot)
                        .expect("refinement sentinel slot exceeds its own premise telescope");
                    Term::var(depth + resolved)
                } else {
                    Term::var(*v + premise_count)
                }
            }
        }
        Term::Pi(a, b) => Term::pi(go(a, depth), go(b, depth + 1)),
        Term::Lam(a, t) => Term::lam(go(a, depth), go(t, depth + 1)),
        Term::Sigma(a, b) => Term::sigma(go(a, depth), go(b, depth + 1)),
        Term::Let { ty, val, body } => Term::Let {
            ty: Box::new(go(ty, depth)),
            val: Box::new(go(val, depth)),
            body: Box::new(go(body, depth + 1)),
        },
        Term::App(f, a) => Term::app(go(f, depth), go(a, depth)),
        Term::Pair(a, b) => Term::pair(go(a, depth), go(b, depth)),
        Term::Proj1(p) => Term::proj1(go(p, depth)),
        Term::Proj2(p) => Term::proj2(go(p, depth)),
        Term::Ascript(t, a) => Term::Ascript(Box::new(go(t, depth)), Box::new(go(a, depth))),
        Term::Eq(a, t, u) => Term::Eq(
            Box::new(go(a, depth)),
            Box::new(go(t, depth)),
            Box::new(go(u, depth)),
        ),
        Term::Cast(a, b, e, t) => Term::Cast(
            Box::new(go(a, depth)),
            Box::new(go(b, depth)),
            Box::new(go(e, depth)),
            Box::new(go(t, depth)),
        ),
        Term::J(ml, d2, e) => Term::J(
            Box::new(go(ml, depth)),
            Box::new(go(d2, depth)),
            Box::new(go(e, depth)),
        ),
        Term::Quot(a, r) => Term::Quot(Box::new(go(a, depth)), Box::new(go(r, depth))),
        Term::QuotClass(t) => Term::QuotClass(Box::new(go(t, depth))),
        Term::Trunc(a) => Term::Trunc(Box::new(go(a, depth))),
        Term::TruncProj(t) => Term::TruncProj(Box::new(go(t, depth))),
        Term::Refl(t) => Term::Refl(Box::new(go(t, depth))),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => Term::QuotElim {
            motive: Box::new(go(motive, depth)),
            method: Box::new(go(method, depth)),
            respect: Box::new(go(respect, depth)),
            scrut: Box::new(go(scrut, depth)),
        },
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => Term::Elim {
            fam: *fam,
            level_args: level_args.clone(),
            params: params.iter().map(|p| go(p, depth)).collect(),
            motive: Box::new(go(motive, depth)),
            methods: methods.iter().map(|m| go(m, depth)).collect(),
            indices: indices.iter().map(|i| go(i, depth)).collect(),
            scrut: Box::new(go(scrut, depth)),
        },
        Term::Absurd(motive, proof) => {
            Term::Absurd(Box::new(go(motive, depth)), Box::new(go(proof, depth)))
        }
        Term::Type(_)
        | Term::Omega(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => term.clone(),
    }
}

fn index_domain_mentions_prior_index(term: &Term, prior_count: usize) -> bool {
    match term {
        Term::Var(i) => *i < prior_count,
        Term::Pi(dom, cod) | Term::Lam(dom, cod) | Term::Sigma(dom, cod) => {
            index_domain_mentions_prior_index(dom, prior_count)
                || index_domain_mentions_prior_index(cod, prior_count + 1)
        }
        Term::Let { ty, val, body } => {
            index_domain_mentions_prior_index(ty, prior_count)
                || index_domain_mentions_prior_index(val, prior_count)
                || index_domain_mentions_prior_index(body, prior_count + 1)
        }
        _ => term
            .children()
            .iter()
            .any(|child| index_domain_mentions_prior_index(child, prior_count)),
    }
}

fn wrap_premise_pis(body: Term, premises: &[Term]) -> Term {
    let mut term = weaken(&body, premises.len() as i64);
    for i in (0..premises.len()).rev() {
        term = Term::pi(weaken(&premises[i], i as i64), term);
    }
    term
}

fn wrap_premise_lams_from_full(body: Term, premises: &[Term]) -> Term {
    let mut term = body;
    for i in (0..premises.len()).rev() {
        term = Term::lam(weaken(&premises[i], i as i64), term);
    }
    term
}

/// Wrap `body` in the premise telescope, relocating index-refinement sentinels
/// throughout via `finalize_refined_body` — in the codomain AND in each premise
/// DOMAIN. A transitive context-telescope convoy premise's type references an
/// EARLIER convoy binder (`h : Wit n xs` names the `xs` binder), emitted as a
/// sentinel; a premise domain at position `i` sits under `i` binders, so
/// `finalize_refined_body(premise[i], 0, i)` relocates its sentinels to the real
/// binders and shifts its field references by `i` — exactly the shift
/// `weaken(_, i)` performs for the sentinel-free premises, so this degenerates
/// to `wrap_premise_{pis,lams}_from_full` when no premise carries a sentinel.
fn wrap_premise_lams_finalized(body: Term, premises: &[Term], sentinel_region: usize) -> Term {
    let total = premises.len();
    let mut term = finalize_refined_body(&body, 0, total, sentinel_region);
    for i in (0..total).rev() {
        term = Term::lam(
            finalize_refined_body(&premises[i], 0, i, sentinel_region),
            term,
        );
    }
    term
}

fn wrap_premise_pis_finalized(body: Term, premises: &[Term], sentinel_region: usize) -> Term {
    let total = premises.len();
    let mut term = finalize_refined_body(&body, 0, total, sentinel_region);
    for i in (0..total).rev() {
        term = Term::pi(
            finalize_refined_body(&premises[i], 0, i, sentinel_region),
            term,
        );
    }
    term
}

/// Build the constructor-refined motive-application type
/// `Π(index-premises). Π(context-telescope convoy). base_body[amb := binder]`,
/// as a self-contained type at the `n`-deep field context. This is the shared
/// shape of the constructor `expected_here` goal AND the direct IH slot in
/// `wrap_dependent_method_ihs`: both are `motive` applied to a value of the
/// family at `refined_indices`, so both must carry the same convoy telescope the
/// motive codomain now generalizes over (LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-
/// REBASE). `refined_core` is the value the scrutinee is refined to (the ctor at
/// its fields, or a recursive field variable for the IH). Convoy binder
/// references are emitted as index-refinement sentinels and relocated to their
/// real de Bruijn positions by `finalize_refined_body`.
/// The context-telescope convoy's binder types (innermost-first) and their
/// sentinels, DERIVED from the ONE ordered plan `context_convoy` for one
/// consumer frame, so the motive, each constructor method, the direct IH, and
/// (through the sentinels) the final Elim application cannot diverge. Rebases
/// `scrut_indices[j] -> index_targets[j]` and `scrut_core -> core_target`, and
/// threads every OTHER convoy binder through its sentinel (all-but-self,
/// order-independent — an entry's type names only outer convoy binders, never
/// itself; fixes the indirect `h : p xs` where `h` is innermost yet depends on
/// the outer `xs`). `frame_depth` = binders between the ambient context and
/// where these types are stated (`motive_base_depth` for the motive; `n` for a
/// method/IH). `slot_base` = premises preceding the convoy in the enclosing
/// telescope (0 for the motive; the index-premise count for a method/IH).
/// `finalize_refined_body` later relocates each sentinel to its real binder.
fn convoy_binder_types(
    context_convoy: &[ConvoyEntry],
    scrut_indices: &[Term],
    scrut_core: &Term,
    index_targets: &[Term],
    core_target: &Term,
    frame_depth: usize,
    slot_base: usize,
    sentinel_region: usize,
) -> (Vec<Term>, Vec<Term>) {
    let c = context_convoy.len();
    let sentinels: Vec<Term> = (0..c)
        .map(|i| index_refinement_sentinel(sentinel_region, slot_base + (c - 1 - i)))
        .collect();
    let mut types: Vec<Term> = Vec::with_capacity(c);
    for (i, entry) in context_convoy.iter().enumerate() {
        let mut subs: Vec<(Term, Term)> = Vec::with_capacity(scrut_indices.len() + c);
        for (j, actual) in scrut_indices.iter().enumerate() {
            subs.push((weaken(actual, frame_depth as i64), index_targets[j].clone()));
        }
        subs.push((weaken(scrut_core, frame_depth as i64), core_target.clone()));
        for (k, e2) in context_convoy.iter().enumerate() {
            if k != i {
                subs.push((Term::var(e2.var + frame_depth), sentinels[k].clone()));
            }
        }
        types.push(subst_term_generalize_many(
            &weaken(&entry.ambient_ty, frame_depth as i64),
            &subs,
        ));
    }
    (types, sentinels)
}

/// Redirect a body's ambient convoy references to the convoy binder sentinels,
/// using the SAME plan and frame as `convoy_binder_types`.
fn redirect_convoy_body(
    context_convoy: &[ConvoyEntry],
    frame_depth: usize,
    sentinels: &[Term],
    body: Term,
) -> Term {
    let mut body = body;
    for (i, entry) in context_convoy.iter().enumerate() {
        body = subst_term_generalize(&body, &Term::var(entry.var + frame_depth), &sentinels[i]);
    }
    body
}

fn build_convoy_refined_type(
    env: &GlobalEnv,
    ctx: &Context,
    span: &Span,
    ind: &InductiveDecl,
    params_terms: &[Term],
    refined_indices: &[Term],
    scrut_indices: &[Term],
    scrut_core: &Term,
    refined_core: &Term,
    n: usize,
    sentinel_region: usize,
    context_convoy: &[ConvoyEntry],
    embedded_method_convoy: &[EmbeddedMethodConvoy],
    embedded_method_repairs: &[(usize, usize)],
    base_body: Term,
) -> Result<Term, ElabError> {
    let mut premises = method_index_premises(ind, params_terms, refined_indices, scrut_indices, n);
    let n_idx = premises.len();
    let (types_inner_first, sentinels) = convoy_binder_types(
        context_convoy,
        scrut_indices,
        scrut_core,
        refined_indices,
        refined_core,
        n,
        n_idx,
        sentinel_region,
    );
    let mut body = redirect_convoy_body(context_convoy, n, &sentinels, base_body);
    // Append the convoy binder types outermost-first (reverse of innermost-first).
    for i in (0..context_convoy.len()).rev() {
        premises.push(types_inner_first[i].clone());
    }
    if !embedded_method_convoy.is_empty() || !embedded_method_repairs.is_empty() {
        let embedded_slot_base = premises.len();
        body = install_embedded_method_sentinels(
            env,
            ctx,
            &body,
            embedded_method_convoy,
            embedded_method_repairs,
            embedded_slot_base,
        )?;
        body = refresh_embedded_elim_evidence(env, ctx, &body, span)?;
        let branch_rebase = dependent_rebase_subs(
            scrut_core,
            scrut_indices,
            n as i64,
            refined_indices,
            refined_core,
            &body,
            true,
        );
        for entry in embedded_method_convoy {
            let branch_ty =
                subst_term_generalize_many(&weaken(&entry.ambient_ty, n as i64), &branch_rebase);
            premises.push(redirect_convoy_body(
                context_convoy,
                n,
                &sentinels,
                branch_ty,
            ));
        }
    }
    Ok(wrap_premise_pis_finalized(body, &premises, sentinel_region))
}

fn synthesize_omitted_index_method(
    cx: &ElabCtx,
    premise_domains: &[Term],
    expected_here: &Term,
    sentinel_region: usize,
    missing: MissingPatternWitness,
    span: &Span,
) -> Result<Term, ElabError> {
    let bottom = Term::const_(cx.env.bottom_id(), vec![]);
    let impossible_idx = premise_domains
        .iter()
        .enumerate()
        .find_map(|(i, premise)| {
            let mut premise_ctx = cx.ctx.clone();
            premise_ctx.push(premise.clone());
            kernel_check(cx.env, &premise_ctx, &Term::var(0), &bottom)
                .is_ok()
                .then_some(i)
        })
        .ok_or_else(|| ElabError::ExhaustivenessError {
            missing,
            span: span.clone(),
        })?;
    let proof = index_refinement_sentinel(sentinel_region, impossible_idx);
    let body = Term::Absurd(Box::new(expected_here.clone()), Box::new(proof));
    Ok(wrap_premise_lams_finalized(
        body,
        premise_domains,
        sentinel_region,
    ))
}

fn ctor_name(cx: &ElabCtx, id: GlobalId) -> String {
    cx.globals
        .iter()
        .find(|(_, &candidate)| candidate == id)
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| format!("<ctor_{:?}>", id))
}

/// The `34 §4.1` unmatched-pattern witness for an omitted constructor: its
/// name and the arity of its own declaration, both derived from `id` so the
/// two cannot disagree -- there is no caller-suppliable arity to mismatch.
///
/// `id` is asserted to resolve in the kernel's own constructor index: every
/// call site derives it from the kernel's own enumeration of the scrutinee's
/// inductive family, never from a caller-supplied or surface-resolved value,
/// so a miss here is an elaborator-internal invariant violation, not a
/// user-triggerable case. `ctor_name`'s own fallback for an id absent from
/// `cx.globals` is real -- `cx.globals` is deliberately pruned for some
/// constructors while their kernel declaration survives, e.g. `PrivateFsOpen`
/// after `prelude.rs`'s private-op pruning -- and it IS reached from this
/// call for exactly that population: an id the kernel still resolves but
/// `cx.globals` has pruned degrades to `ctor_name`'s fallback spelling here,
/// which the `H1` control below measures directly. What never reaches this
/// call is the OTHER direction -- an id `cx.globals` would still resolve but
/// the kernel cannot -- because the `.expect()` below panics first on any id
/// the kernel does not resolve, and every call site's id is constructed from
/// the kernel's own enumeration, so that direction is unconstructible by
/// this function's own callers rather than independently measured
/// (`LANG-WITNESS-DIAGNOSTIC-STRICTNESS` H1).
fn missing_pattern_witness(cx: &ElabCtx, id: GlobalId) -> MissingPatternWitness {
    let (ind, ordinal) = cx.env.constructor(id).expect(
        "the constructor id passed here always names a constructor of an \
         already-admitted inductive resolved from this same env",
    );
    let ctor = ind.constructors.get(ordinal).expect(
        "ordinal is populated by GlobalEnv::add_decl from the exact same \
         ind.constructors.iter().enumerate() that ctor_index later returns \
         it from, and Decl::Inductive is never mutated in place after \
         insertion -- only replaced wholesale via remove_last, which purges \
         ctor_index for the same family atomically -- so ordinal is always a \
         valid index into ind.constructors for any pair GlobalEnv::constructor \
         returns (H3)",
    );
    MissingPatternWitness {
        constructor: ctor_name(cx, id),
        arity: ctor.args.len(),
    }
}

/// For the single-column, top-level-constructor-identity matching shape
/// shared by `check_match_with_lift` and `check_match_dependent`: every OTHER
/// arm in `arms` whose top-level pattern names `ctor_id` is subsumed by the
/// arm that already claimed it (`claiming_idx`). At most one arm is ever
/// marked as the claimant of a given constructor at these sites (the
/// enclosing loop's `.find()`/`.position()` returns the lowest-index match),
/// so `get_or_insert` never overwrites a prior claim.
fn mark_shared_ctor_subsumption(
    cx: &ElabCtx,
    arms: &[RMatchArm],
    ctor_id: GlobalId,
    claiming_idx: usize,
    subsumed_by: &mut [Option<usize>],
) {
    for (idx, arm) in arms.iter().enumerate() {
        if idx == claiming_idx {
            continue;
        }
        if matches!(&arm.pat.kind, RPatKind::Ctor(name, _) if cx.globals.get(name).copied() == Some(ctor_id))
        {
            subsumed_by[idx].get_or_insert(claiming_idx);
        }
    }
}

fn infer(cx: &mut ElabCtx, expr: &RExpr) -> Result<(Term, Term), ElabError> {
    match expr {
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            span,
        } => infer_if(cx, condition, then_branch, else_branch, span),
        RExpr::RRecursiveResult {
            selector,
            index,
            name: _,
            binding_span,
            span,
        } => {
            let (result, result_type) = cx.selected_recursive_result(*index).ok_or_else(|| {
                ElabError::StructuralResultOutOfScope {
                    selector_span: span.clone(),
                    binding_span: binding_span.clone(),
                }
            })?;
            let classifier = kernel_infer(cx.env, &cx.ctx, &result_type).map_err(|error| {
                ElabError::KernelRejected {
                    error,
                    span: span.clone(),
                }
            })?;
            let classifier = whnf(cx.env, &cx.ctx, &classifier);
            let (actual, required) = match classifier {
                Term::Type(level) => (
                    RecursiveResultSort::Type(level),
                    RecursiveResultSelector::RecursiveResult,
                ),
                Term::Omega(level) => (
                    RecursiveResultSort::Omega(level),
                    RecursiveResultSelector::InductionHypothesis,
                ),
                _ => {
                    return Err(ElabError::RecursiveResultClassifierNotUniverse {
                        selector_span: span.clone(),
                        binding_span: binding_span.clone(),
                    })
                }
            };
            if *selector != required {
                return Err(ElabError::RecursiveResultSortMismatch {
                    selector_span: span.clone(),
                    binding_span: binding_span.clone(),
                    actual_classifier: actual,
                    required_spelling: required.spelling(),
                });
            }
            Ok((result, result_type))
        }
        RExpr::RVar(i, _, _) => {
            // An installed index refinement (constructor injectivity
            // / sibling convoy) replaces the bare `Var` with its `Cast`-
            // wrapped alias for the duration of one branch's body — see
            // `ElabCtx::var_refinements`.
            let (pos, actual_index) = cx
                .surface_var(*i)
                .ok_or_else(|| ElabError::Internal(format!("Var({}) out of range", i)))?;
            if let Some((raw_term, raw_ty, install_depth)) = cx.var_refinements.get(&pos) {
                let growth = (cx.ctx.len() - install_depth) as i64;
                return Ok((weaken(raw_term, growth), weaken(raw_ty, growth)));
            }
            let ty_stored = cx
                .ctx
                .lookup(actual_index)
                .ok_or_else(|| ElabError::Internal(format!("Var({}) out of range", i)))?;
            let ty = weaken(ty_stored, (actual_index as i64) + 1);
            Ok((Term::var(actual_index), ty))
        }

        RExpr::RCell(index, _, span) => {
            let Some((state_position, cell_types)) = &cx.space_state else {
                return Err(ElabError::MutationOutsideSpace {
                    construct: "cell read".to_string(),
                    span: span.clone(),
                });
            };
            let cell_ty = cell_types.get(*index).cloned().ok_or_else(|| {
                ElabError::Internal(format!("space cell index {index} out of range"))
            })?;
            let state_index = cx.ctx.len() - 1 - state_position;
            Ok((
                project_space_cell(Term::var(state_index), *index, cell_types.len()),
                cell_ty,
            ))
        }

        RExpr::RBecomes(_, _, _, span) => Err(ElabError::MutationOutsideSpace {
            construct: "becomes".to_string(),
            span: span.clone(),
        }),

        RExpr::RCon(name, span) => {
            if let Some((term, ty, install_depth)) = cx.local_dicts.get(name) {
                let growth = cx.ctx.len().checked_sub(*install_depth).ok_or_else(|| {
                    ElabError::Internal(format!(
                        "dictionary '{name}' used outside its declaration context"
                    ))
                })? as i64;
                return Ok((weaken(term, growth), weaken(ty, growth)));
            }
            let id = cx
                .globals
                .get(name)
                .copied()
                .ok_or_else(|| ElabError::UnresolvedCon {
                    name: name.clone(),
                    span: span.clone(),
                })?;
            // Constructor: Term::Constructor with the ctor's declared type.
            let ctor_ty = cx
                .env
                .constructor(id)
                .map(|(ind, k)| ind.constructors[k].type_.clone());
            if let Some(ty) = ctor_ty {
                return Ok((
                    Term::Constructor {
                        id,
                        level_args: vec![],
                    },
                    ty,
                ));
            }
            // Inductive type former: Term::IndFormer.
            let ind_ty = cx.env.inductive(id).map(|ind| ind.former_type.clone());
            if let Some(ty) = ind_ty {
                return Ok((
                    Term::IndFormer {
                        id,
                        level_args: vec![],
                    },
                    ty,
                ));
            }
            // Regular constant (postulate/def/primitive).
            let (_, decl_ty) = cx
                .env
                .const_type(id)
                .ok_or_else(|| ElabError::Internal(format!("no type for global '{}'", name)))?;
            Ok((Term::const_(id, vec![]), decl_ty.clone()))
        }

        RExpr::RUniv(None, _) => {
            let l = cx.metas.fresh();
            let ty = Term::ty(Level::Suc(Box::new(l.clone())));
            Ok((Term::ty(l), ty))
        }
        RExpr::RUniv(Some(n), _) => {
            let l = level_from_nat(*n);
            let ty = Term::ty(Level::Suc(Box::new(l.clone())));
            Ok((Term::ty(l), ty))
        }

        // `J motive base eq` — the identity eliminator (`34 §3.4`), surfaced
        // as an INFER-mode former mirroring the existing checked-sugar idiom
        // (`Refl`/`absurd`/`Axiom` above are `RCon`/`RApp` special forms over
        // a resolver-emitted `RCon` on scope miss; `J` is the 3-argument,
        // infer-mode sibling — its motive is user-written, not recovered
        // from a checked goal). Detected BEFORE the generic application arm
        // below via a full application-spine peel (`absurd` only needed one
        // level; `J` needs three).
        RExpr::RApp(..) if peel_named_app(expr, SUGAR_J, 3).is_some() => {
            let args = peel_named_app(expr, SUGAR_J, 3).expect("checked by guard");
            infer_j(cx, args[0], args[1], args[2], expr.span())
        }

        // `Eq A a b` at EXPRESSION position (e.g. inside a `J` motive's body,
        // `\b' _. Eq B (P a) (P b')` — `cong`'s motive, `50-stdlib/53-
        // transport.md §2`). Same plumbing as the `elab_type` arm above
        // (`peel_named_rtype_app`), needed because a motive body is
        // elaborated via `infer`/`check`, not `elab_type`.
        RExpr::RApp(..) if peel_named_app(expr, SUGAR_EQ, 3).is_some() => {
            let args = peel_named_app(expr, SUGAR_EQ, 3).expect("checked by guard");
            infer_eq(cx, args[0], args[1], args[2], expr.span())
        }

        // `elim_trunc P f t` — the `16 §6` truncation eliminator
        // (LANG-TRUNCATION-SURFACE-SYNTAX D2), elaborated directly to a
        // `Term::QuotElim` over a `Trunc` scrutinee. Infer-mode, arity-3,
        // mirroring `J`'s shape exactly (`peel_named_app`, not the
        // single-arg checked-sugar idiom `absurd`/`trunc_intro` use): the
        // target proposition `P` is user-written, not recovered from a
        // checked goal.
        RExpr::RApp(..) if peel_named_app(expr, SUGAR_ELIM_TRUNC, 3).is_some() => {
            let args = peel_named_app(expr, SUGAR_ELIM_TRUNC, 3).expect("checked by guard");
            infer_elim_trunc(cx, args[0], args[1], args[2], expr.span())
        }

        // `trunc_intro a` reached in INFER position (no expected type in
        // scope) — the introduction form is checked-only (see the `check`
        // arm below); give the actionable remedy rather than letting this
        // fall through to the generic `RApp` arm and report an unrelated
        // "unresolved identifier 'trunc_intro'" (AC-4).
        RExpr::RApp(f, _, rspan) if matches!(f.as_ref(), RExpr::RCon(n, _) if n == SUGAR_TRUNC_INTRO) => {
            Err(ElabError::TypeMismatch {
                span: rspan.clone(),
                reason: "trunc_intro (‖A‖ introduction) cannot be inferred — it needs an \
                         expected type; add an ascription `(trunc_intro a : ‖A‖)` or place \
                         it where the expected type is already known (e.g. a declaration's \
                         declared type)"
                    .into(),
            })
        }

        RExpr::RApp(f, a, span) => {
            // Peel and inspect the whole source spine in a separate frame.
            // On the overwhelmingly-common decline path that frame (including
            // its argument vector) is gone before generic application recurses.
            if let Some(call) = infer_reflexive_recursive_self_call(cx, expr, span)? {
                return Ok(call);
            }
            let (f_core, f_ty) = infer(cx, f)?;
            let f_ty_wh = whnf(cx.env, &cx.ctx, &f_ty);
            match f_ty_wh {
                Term::Pi(dom, cod) => {
                    let a_core = check(cx, a, &dom, span)?;
                    let result_ty = subst0(&cod, &a_core);
                    Ok((Term::app(f_core, a_core), result_ty))
                }
                _ => Err(ElabError::NotAFunction { span: span.clone() }),
            }
        }

        RExpr::RAsc(e, ty, _) => {
            let ty_core = elab_type(cx, ty)?;
            let e_core = check(cx, e, &ty_core, e.span())?;
            Ok((e_core, ty_core))
        }

        RExpr::RLam(_, _, span) => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "cannot infer type of lambda without annotation".into(),
        }),

        RExpr::RLet(_x, ty_opt, rhs, body, span) => {
            let (rhs_core, rhs_ty) = prepare_let_rhs(cx, ty_opt, rhs, span)?;
            cx.ctx.push(rhs_ty.clone());
            let body_result = infer(cx, body);
            cx.ctx.pop();
            let (body_core, body_ty) = body_result?;
            let result_ty = subst0(&body_ty, &rhs_core);
            Ok((
                Term::Let {
                    ty: Box::new(rhs_ty),
                    val: Box::new(rhs_core),
                    body: Box::new(body_core),
                },
                result_ty,
            ))
        }

        RExpr::ROld(inner, span) => {
            let Some(pre_state) = cx.space_pre_state.clone() else {
                return Err(ElabError::OldPreStateUnsupported { span: span.clone() });
            };
            let post_state = cx.space_state.replace(pre_state);
            let result = infer(cx, inner);
            cx.space_state = post_state;
            result
        }

        RExpr::RNumLit(lit, span) => elab_num_lit_infer(cx, lit, span),

        RExpr::RStr(s, span) => elab_str_lit(cx, s, None, span),

        RExpr::RCharLit(c, span) => elab_char_lit(cx, *c, span),

        RExpr::RByteStr(bytes, span) => elab_bytes_lit(cx, bytes, span),

        RExpr::RBinOp(op, lhs, rhs, span) => elab_binop(cx, op, lhs, rhs, span),

        RExpr::RMatch {
            scrut: _,
            equation: Some(_),
            span,
            ..
        } => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "`match ... eqn:` requires a declared expected type".into(),
        }),

        RExpr::RMatch {
            scrut,
            equation: None,
            arms,
            span,
        } => infer_match(cx, scrut, arms, span),

        RExpr::RProj(base, field, span) => infer_proj(cx, base, field, span),

        RExpr::RPosProj(base, projection, span) => {
            infer_positional_proj(cx, base, *projection, span)
        }

        RExpr::RPair(components, span) => infer_pair(cx, components, span),
        RExpr::RRecord { span, .. } => infer_record_requires_expected_type(span),

        RExpr::RPi(_, a, b, span) => infer_pi(cx, a, b, span),

        RExpr::RArrow(a, b, span) => infer_arrow(cx, a, b, span),

        RExpr::RTrunc(inner, span) => infer_trunc(cx, inner, span),

        RExpr::RAttachedProofRef {
            subject,
            proof_name,
            span,
            // This variant has no expression child to receive a checking
            // goal: it is a leaf lookup for the qualified proof name. The
            // ordinary infer-then-unify checking fallback therefore loses no
            // bidirectional information here.
        } => infer(
            cx,
            &RExpr::RCon(format!("{subject}::{proof_name}"), span.clone()),
        ),
    }
}

/// Peel a left-nested application spine, returning its arguments in surface
/// (left-to-right) order iff the spine is headed by `RCon(name)` applied to
/// EXACTLY `arity` arguments (generalizes the single-arg `absurd` match in
/// `check` above to `J`'s 3 arguments: motive, base, eq).
fn peel_named_app<'a>(expr: &'a RExpr, name: &str, arity: usize) -> Option<Vec<&'a RExpr>> {
    let mut args: Vec<&RExpr> = Vec::new();
    let mut cur = expr;
    loop {
        match cur {
            RExpr::RApp(f, a, _) => {
                args.push(a.as_ref());
                cur = f.as_ref();
            }
            RExpr::RCon(n, _) if n == name && args.len() == arity => {
                args.reverse();
                return Some(args);
            }
            _ => return None,
        }
    }
}

/// `Eq A a b` at expression position — elaborates directly to the kernel's
/// existing `Term::Eq` (see the `elab_type` companion arm above for the
/// type-position spelling and the full rationale). `A` is inferred (so a
/// bare `Type` argument, needed for `cast`'s `Eq Type A B`, gets its own
/// fresh level via the ordinary `RUniv(None)` path), then `a`/`b` are
/// CHECKED against it — mirroring `check.rs`'s own `Term::Eq` inference arm
/// (`synth_type(a_ty)`; `check(x,a_ty)`; `check(y,a_ty)`) exactly, with a
/// final `kernel_infer` re-derivation as the soundness net (never trusting
/// this function's own bookkeeping, same discipline as `infer_j`).
fn infer_eq(
    cx: &mut ElabCtx,
    a_ty_expr: &RExpr,
    a_expr: &RExpr,
    b_expr: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let (a_ty_core, _a_ty_ty) = infer(cx, a_ty_expr)?;
    let a_ty_core = cx.metas.zonk_term(&a_ty_core);
    let a_core = check(cx, a_expr, &a_ty_core, span)?;
    let b_core = check(cx, b_expr, &a_ty_core, span)?;
    let eq_term = Term::Eq(Box::new(a_ty_core), Box::new(a_core), Box::new(b_core));

    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let zonked_eq = cx.metas.zonk_term(&eq_term);
    let ty =
        kernel_infer(cx.env, &zonked_ctx, &zonked_eq).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    Ok((eq_term, ty))
}

/// `(x : A) -> B` — dependent function type in expr position (VAL2 #4,
/// `32 §3`). Domain `A` is a `type` (mirrors the type-position `Pi`,
/// `elab_type`'s `RType::RPi` arm); codomain `B` is an expr, elaborated in
/// a context extended by `A` so `x`'s references resolve. Elaborates to
/// the existing kernel `Term::Pi` — no new kernel variant (types are
/// terms, `11 §1`); the kernel's own `kernel_infer` classifies the result
/// (`Type ℓ` or `Ω`, whichever the domain/codomain sorts license) rather
/// than this function guessing a sort.
fn infer_pi(
    cx: &mut ElabCtx,
    a: &RType,
    b: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let a_core = elab_type(cx, a)?;
    let a_core = cx.metas.zonk_term(&a_core);
    cx.ctx.push(a_core.clone());
    let b_result = infer(cx, b);
    cx.ctx.pop();
    let (b_core, _b_ty) = b_result?;
    let b_core = cx.metas.zonk_term(&b_core);
    let pi = Term::pi(a_core, b_core);

    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let zonked_pi = cx.metas.zonk_term(&pi);
    let sort =
        kernel_infer(cx.env, &zonked_ctx, &zonked_pi).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    Ok((pi, sort))
}

/// `A -> B` — non-dependent function type in expr position (VAL2 #4,
/// `32 §3`). BOTH `A` and `B` are exprs (types are terms, `11 §1` — the
/// same "`ConId`/`Type` already stand in expr position" precedent this
/// closes the gap for), each elaborated via ordinary `infer` — a plain
/// `Int`/`List Int`-style type-valued expression infers fine today, no new
/// machinery needed. `B` doesn't reference the (unused, non-dependent)
/// bound variable, so it's `weaken`ed by 1 to sit correctly under the
/// implicit `Pi` binder (the exact same construction `elab_type`'s
/// `RType::RArr` arm already uses for the type-position non-dependent
/// arrow). Elaborates to the existing kernel `Term::Pi` — no new variant.
fn infer_arrow(
    cx: &mut ElabCtx,
    a: &RExpr,
    b: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let (a_core, _a_ty) = infer(cx, a)?;
    let (b_core, _b_ty) = infer(cx, b)?;
    let a_core = cx.metas.zonk_term(&a_core);
    let b_core = cx.metas.zonk_term(&b_core);
    let pi = Term::pi(a_core, weaken(&b_core, 1));

    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let zonked_pi = cx.metas.zonk_term(&pi);
    let sort =
        kernel_infer(cx.env, &zonked_ctx, &zonked_pi).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    Ok((pi, sort))
}

/// `trunc_intro a` — propositional-truncation INTRODUCTION, checked mode
/// (`16 §6`, LANG-TRUNCATION-SURFACE-SYNTAX D2). Split out of `check`'s own
/// match per that function's FRAME BUDGET note (a new arm's locals are paid
/// by every checked expression in every compile; keep new arm bodies a call
/// to a separate, `#[inline(never)]` function).
#[inline(never)]
fn check_trunc_intro(
    cx: &mut ElabCtx,
    arg: &RExpr,
    expected: &Term,
    rspan: &Span,
) -> Result<Term, ElabError> {
    match whnf(cx.env, &cx.ctx, expected) {
        Term::Trunc(a_ty) => {
            let a_core = check(cx, arg, &a_ty, rspan)?;
            Ok(Term::TruncProj(Box::new(a_core)))
        }
        _ => Err(ElabError::TypeMismatch {
            span: rspan.clone(),
            reason: "trunc_intro expects a ‖A‖-shaped goal".into(),
        }),
    }
}

/// `‖A‖` / `||A||` — propositional-truncation FORMATION (`16 §6`,
/// LANG-TRUNCATION-SURFACE-SYNTAX D1). `A` is an ordinary expr (types are
/// terms, `11 §1` — the same precedent `infer_arrow` above already relies
/// on); elaborates directly to the kernel's existing `Term::Trunc`, already
/// kernel-typed (`check.rs`'s `Term::Trunc(a)` infer arm: `‖A‖ : Ω_l` for
/// `A : Type l`). No new kernel node — `kernel_infer` is the sole authority
/// on the resulting sort, mirroring `infer_arrow`/`infer_pi` exactly rather
/// than re-deriving the level here.
fn infer_trunc(cx: &mut ElabCtx, inner: &RExpr, span: &Span) -> Result<(Term, Term), ElabError> {
    let (a_core, _a_ty) = infer(cx, inner)?;
    let a_core = cx.metas.zonk_term(&a_core);
    let trunc = Term::Trunc(Box::new(a_core));

    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let zonked_trunc = cx.metas.zonk_term(&trunc);
    let sort = kernel_infer(cx.env, &zonked_ctx, &zonked_trunc).map_err(|e| {
        ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        }
    })?;
    Ok((trunc, sort))
}

/// `elim_trunc P f t` — the `16 §6` truncation ELIMINATOR
/// (LANG-TRUNCATION-SURFACE-SYNTAX D2). There is no `TruncElim` kernel
/// node — the kernel implements this through the EXISTING quotient
/// eliminator, `Term::QuotElim`, whose scrutinee match already admits a
/// `Trunc` (`check.rs::infer_quot_elim`, the `Term::Trunc(a) => (*a, None)`
/// arm — no relation, so the Type-target respect schema is refused via its
/// own `opt_rel` check and only an Ω target is admitted, exactly `16 §6`'s
/// defining restriction — preserved here, not re-implemented). `P` doesn't
/// depend on the truncated element (the spec's `elim_trunc` signature is
/// non-dependent), so the motive is the CONSTANT function `λ_:‖A‖. P` —
/// ascripted at `(‖A‖) -> Ω_l` so it is itself kernel-INFERABLE (a bare
/// `Term::Lam` is check-only; the same `Ascript`-wrapped-motive technique
/// `infer_j` above and `ken-kernel/src/inductive.rs`'s `host_motive` both
/// already use, so this is a precedented shape, not a new one). `respect`
/// is unused by both reduction and admission at an Ω target
/// (`check.rs::infer_quot_elim`: `raw_well_formed` only, no schema check) —
/// `P` itself is a convenient, always well-scoped placeholder.
fn infer_elim_trunc(
    cx: &mut ElabCtx,
    p_expr: &RExpr,
    f_expr: &RExpr,
    t_expr: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    // t : ‖A‖ — recover A and build the scrutinee core term.
    //
    // `trunc_intro a` is checked-only (see the `check` arm above), so it
    // cannot be `infer`'d by the general fallback below. Special-case it
    // exactly the way the spec states truncation's OWN computation rule
    // (`elim_trunc P f |a| ≡ f a`, `16 §6`) -- this is the only way a
    // FRESHLY built truncation reaches an eliminator within this WP's
    // delivered surface, since `D1`-`D3` add no type-annotation-position
    // spelling for `‖A‖` (out of scope), so no OTHER surface path can
    // produce a named value whose OWN inferred type is structurally
    // `Trunc(_)`. An already-bound `‖A‖`-typed value (e.g. a parameter of
    // some future caller) still works through the ordinary `infer`
    // fallback, unchanged.
    let (a_ty, t_core) = match t_expr {
        RExpr::RApp(head, arg, _) if matches!(head.as_ref(), RExpr::RCon(n, _) if n == SUGAR_TRUNC_INTRO) =>
        {
            let (arg_core, arg_ty) = infer(cx, arg)?;
            let arg_ty = cx.metas.zonk_term(&arg_ty);
            let proj = Term::TruncProj(Box::new(cx.metas.zonk_term(&arg_core)));
            // `TruncProj` is check-only at the KERNEL level too
            // (`check.rs::infer`'s explicit non-inferable list) -- the
            // final whole-result `kernel_check` re-verifies this term
            // independently via `infer_quot_elim`, whose first step
            // `infer`s the scrutinee, so a bare `TruncProj` here would pass
            // elaboration and then fail admission. `Ascript` makes it
            // inferable, exactly like `motive` below.
            let ascripted = Term::Ascript(
                Box::new(proj),
                Box::new(Term::Trunc(Box::new(arg_ty.clone()))),
            );
            (arg_ty, ascripted)
        }
        _ => {
            let (t_core, t_ty) = infer(cx, t_expr)?;
            let t_core = cx.metas.zonk_term(&t_core);
            let t_ty = cx.metas.zonk_term(&t_ty);
            match whnf(cx.env, &cx.ctx, &t_ty) {
                Term::Trunc(a) => (*a, t_core),
                other => {
                    return Err(ElabError::TypeMismatch {
                        span: t_expr.span().clone(),
                        reason: format!(
                            "elim_trunc's third argument must have a truncation (‖A‖) type, \
                             found {other:?}"
                        ),
                    })
                }
            }
        }
    };

    // P : Omega_l — recover l.
    let (p_core, p_ty) = infer(cx, p_expr)?;
    let p_core = cx.metas.zonk_term(&p_core);
    let p_ty = cx.metas.zonk_term(&p_ty);
    let level = match whnf(cx.env, &cx.ctx, &p_ty) {
        Term::Omega(l) => l,
        other => {
            return Err(ElabError::TypeMismatch {
                span: p_expr.span().clone(),
                reason: format!(
                    "elim_trunc's first argument must be an Ω-classified proposition, \
                     found a term of type {other:?}"
                ),
            })
        }
    };

    // f : A -> P.
    let f_expected_ty = Term::pi(a_ty.clone(), weaken(&p_core, 1));
    let f_core = check(cx, f_expr, &f_expected_ty, span)?;

    // motive := (λ_:‖A‖. P) ascripted at (‖A‖) -> Ω_l — constant, ignores
    // the scrutinee, matching the spec's non-dependent `elim_trunc`.
    let scrut_ty = Term::Trunc(Box::new(a_ty));
    let motive_lam = Term::lam(scrut_ty.clone(), weaken(&p_core, 1));
    let motive_ty = Term::pi(scrut_ty, Term::Omega(level));
    let motive_core = Term::Ascript(Box::new(motive_lam), Box::new(motive_ty));

    let elim = Term::QuotElim {
        motive: Box::new(motive_core),
        method: Box::new(f_core),
        respect: Box::new(p_core.clone()),
        scrut: Box::new(t_core),
    };
    Ok((elim, p_core))
}

/// `J motive base eq` — elaborates directly to the kernel's existing
/// `Term::J` (`34 §3.4`; kernel target `check.rs::infer_j`, already in
/// `trusted_base()`). Unlike `Refl`/`absurd`/`Proved` (checked-mode, the motive
/// comes from the ascribed goal), `J`'s motive is USER-WRITTEN and cannot be
/// `infer`'d as a bare lambda (`RExpr::RLam` has no domain annotation — see
/// the unconditional error in `infer`'s own `RLam` arm above). So the motive
/// is elaborated BIDIRECTIONALLY here: recover `A`/`a`/`b` from `eq`'s
/// inferred type, peel the motive's own two binders, bind them at their
/// rule-mandated types (`A` and `Eq A a b'`), and `infer` (not `check`) the
/// motive's BODY — its inferred type IS the codomain sort `s` the kernel's
/// rule leaves unconstrained (`Type ℓ` or `Ω`; e.g. an `Eq`-valued body
/// naturally infers to `Omega(l)`, licensing `cong`'s `Ω`-motive). `base` is
/// checked against `motive a (refl a)` built the same UNREDUCED-application
/// way `check.rs::infer_j` itself does (`Term::app` twice, no manual
/// substitution — `check`'s own whnf handles the redex).
fn infer_j(
    cx: &mut ElabCtx,
    motive_expr: &RExpr,
    base_expr: &RExpr,
    eq_expr: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    // eq : Eq A a b — recover A, a, b. Zonked before the whnf match: a bare
    // surface `(a:Type)` parameter can still carry an unresolved universe
    // metavariable this far into elaboration (the same latent trap fixed for
    // `check_match_dependent` — [[gate-widening-exposes-latent-bugs-in-newly-reachable-code]]).
    let (eq_core, eq_ty) = infer(cx, eq_expr)?;
    let eq_ty = cx.metas.zonk_term(&eq_ty);
    let eq_ty_wh = whnf(cx.env, &cx.ctx, &eq_ty);
    let (a_ty, a, b) = match eq_ty_wh {
        Term::Eq(at, x, y) => (*at, *x, *y),
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "J's third argument must have an `Eq` type".into(),
            })
        }
    };

    let motive_body_expr = match motive_expr {
        RExpr::RLam(_, inner, _) => match inner.as_ref() {
            RExpr::RLam(_, body, _) => body.as_ref(),
            _ => {
                return Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "J's motive must be a 2-argument lambda `\\b' e'. G[b']`".into(),
                })
            }
        },
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "J's motive must be a 2-argument lambda `\\b' e'. G[b']`".into(),
            })
        }
    };

    // Bind b':A, e':Eq A a b' and INFER the motive's body — its type is
    // whatever sort `s` the body computes.
    let eq_dom_ty = Term::Eq(
        Box::new(weaken(&a_ty, 1)),
        Box::new(weaken(&a, 1)),
        Box::new(Term::var(0)),
    );
    cx.ctx.push(a_ty.clone());
    cx.ctx.push(eq_dom_ty.clone());
    let body_result = infer(cx, motive_body_expr);
    cx.ctx.pop();
    cx.ctx.pop();
    let (body_core, body_ty) = body_result?;

    let motive_lam = Term::lam(a_ty.clone(), Term::lam(eq_dom_ty.clone(), body_core));
    let motive_ty = Term::pi(a_ty, Term::pi(eq_dom_ty, body_ty));
    let motive_core = Term::Ascript(Box::new(motive_lam.clone()), Box::new(motive_ty));

    let base_expected_ty = Term::app(
        Term::app(motive_lam.clone(), a.clone()),
        Term::Refl(Box::new(a)),
    );
    let base_core = check(cx, base_expr, &base_expected_ty, span)?;

    let result_ty = Term::app(Term::app(motive_lam, b), eq_core.clone());
    let term_j = Term::J(
        Box::new(motive_core),
        Box::new(base_core),
        Box::new(eq_core),
    );

    // Whole-result admission (`declare_def`, or standalone
    // `elaborate_rexpr`'s final `kernel_check`) is the sole soundness net.
    // Eagerly rechecking this subterm in an assumption-only local `Context`
    // is incomplete for definitional `let` aliases: the neutral binder cannot
    // zeta-reduce to its definition until the enclosing `Term::Let` is checked
    // as a whole.

    Ok((term_j, result_ty))
}

/// `e.field` — Σ-record field projection (`33 §5.2` η). Infers `e`'s type,
/// identifies which registered named-field owner produced its transparent
/// type identity, finds `field`'s
/// declared position, and builds `proj1(proj2^k(e))` — the field's
/// expected type is `field_types[k]` with the class param (this
/// dictionary's concrete head type) and every EARLIER field substituted by
/// its own self-projection off the SAME base (works whether `base` is a
/// concrete instance value or an opaque bound variable like a `where`-
/// supplied dictionary).
fn infer_proj(
    cx: &mut ElabCtx,
    base: &RExpr,
    field: &str,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let (base_core, base_ty) = infer(cx, base)?;
    // Deliberately inspect `base_ty` AS ELABORATED (never `whnf`'d): a named
    // owner type is itself `Decl::Transparent`, so `whnf` would eagerly unfold
    // it straight through into the raw Sigma chain — losing exactly the owner
    // identity this lookup needs.
    // The surface-elaborated shape (`App(Const(owner_id), head)` or bare
    // `Const(owner_id)` for an unparameterized owner) is always already in
    // this un-unfolded form immediately after `infer`/`env.const_type`.
    let (class_type_id, head_arg) = match &base_ty {
        Term::App(f, a) => match f.as_ref() {
            Term::Const { id, .. } => (*id, Some((**a).clone())),
            _ => {
                return Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "projection base's type is not a named-field owner".into(),
                })
            }
        },
        Term::Const { id, .. } => (*id, None),
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "projection base's type is not a named-field owner".into(),
            })
        }
    };
    let class_env = cx.class_env.ok_or_else(|| ElabError::TypeMismatch {
        span: span.clone(),
        reason: "`.field` projection is unavailable in this elaboration context".into(),
    })?;
    let projection = class_env
        .projection_by_type_id(class_type_id)
        .ok_or_else(|| ElabError::TypeMismatch {
            span: span.clone(),
            reason: "projection base's type is not a known named-field owner".into(),
        })?;
    let idx = projection
        .field_names
        .iter()
        .position(|n| n == field)
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: field.to_string(),
            span: span.clone(),
        })?;

    // Build proj1(proj2^idx(base_core)) — field `idx`'s value. Each
    // earlier field's self-projection (proj1(proj2^j(base_core)), j<idx)
    // is built off the SAME base, cloned before consuming it below.
    let mut args: Vec<Term> = Vec::new();
    if let Some(h) = head_arg {
        args.push(h);
    }
    args.extend((0..idx).map(|j| {
        let mut v = base_core.clone();
        for _ in 0..j {
            v = Term::proj2(v);
        }
        Term::proj1(v)
    }));

    let mut val = base_core;
    for _ in 0..idx {
        val = Term::proj2(val);
    }
    let val = Term::proj1(val);

    let expected_ty = ken_kernel::subst::subst_tel(&projection.field_types[idx], &args);
    Ok((val, expected_ty))
}

fn infer_positional_proj(
    cx: &mut ElabCtx<'_>,
    base: &RExpr,
    projection: u8,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let (base_core, base_ty) = infer(cx, base)?;
    let base_ty_wh = whnf(cx.env, &cx.ctx, &base_ty);
    let Term::Sigma(domain, codomain) = base_ty_wh else {
        return Err(ElabError::PositionalProjectionNotPair {
            projection,
            span: span.clone(),
        });
    };
    // Projection is inference-only in the kernel. Preserve the elaborator's
    // inferred type on an introduction-form base (including an explicitly
    // ascribed pair) so the kernel can infer through the projection.
    let base_core = Term::Ascript(Box::new(base_core), Box::new(base_ty));
    match projection {
        1 => Ok((Term::proj1(base_core), *domain)),
        2 => {
            let first = Term::proj1(base_core.clone());
            Ok((Term::proj2(base_core), subst0(&codomain, &first)))
        }
        _ => Err(ElabError::Internal(format!(
            "unsupported positional projection .{projection}"
        ))),
    }
}

// ----- numeric literal helpers -----

/// Elaborate a numeric literal with its default type (no expected type).
fn elab_num_lit_infer(
    cx: &mut ElabCtx,
    lit: &NumLit,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    // `Int` (arbitrary-precision) emits a kernel-native `Term::IntLit`
    // directly (`docs/adr/0013-int-decidable-equality-kernel-posture.md`
    // Layer 2) — no opaque postulate, no `num_values` entry; the value
    // lives in the term itself, which is what makes the kernel's
    // `Eq`-at-registered-literal reduction surface-reachable. Fixed-width
    // int / Float / Decimal / Float32 are untouched, below.
    if let NumLit::Int(n) = lit {
        let ty_term = Term::const_(cx.numeric_env.int_id, vec![]);
        return Ok((Term::IntLit(n.clone()), ty_term));
    }

    let (val, type_id) = num_lit_default_type(lit, cx.numeric_env);
    let ty_term = Term::const_(type_id, vec![]);
    // A literal's value comes from checked surface syntax and is stored in the
    // elaborator side table for evaluation; it is not a primitive operation or
    // an assumed axiom in trust accounting.
    let postulate_id = declare_primitive(cx.env, vec![], ty_term.clone(), PrimReduction::Literal)
        .map_err(|e| ElabError::KernelRejected {
        error: e,
        span: span.clone(),
    })?;
    cx.num_values.insert(postulate_id, val);
    Ok((Term::const_(postulate_id, vec![]), ty_term))
}

/// Elaborate a numeric literal with a known expected type.
///
/// If the expected type is a numeric type that accepts this literal form, use it.
/// Otherwise infer the default type and unify (may yield a type error).
fn elab_num_lit_checked(
    cx: &mut ElabCtx,
    lit: &NumLit,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let nenv = cx.numeric_env;
    let exp_wh = whnf(cx.env, &cx.ctx, expected);

    // Try type-directed dispatch: if expected type is a numeric Const (or,
    // for `Decimal := DecimalPair`, the `IndFormer` `whnf` unfolds the
    // transparent alias to — `18a §5.6.1`), use it.
    let const_or_indformer_id = match &exp_wh {
        Term::Const { id, .. } => Some(*id),
        Term::IndFormer { id, .. } => Some(*id),
        _ => None,
    };
    if let Some(id) = const_or_indformer_id {
        let ty_id = id;

        // `Int` (arbitrary-precision) emits `Term::IntLit` directly — same
        // rewiring as `elab_num_lit_infer`, bypassing the shared postulate
        // + `num_values` path below entirely. Fixed-width integer types
        // (`Int8`..`UInt64`) are unaffected: the certificate/`IntLit`
        // mechanism is registered for `Int` only.
        if let NumLit::Int(n) = lit {
            if ty_id == nenv.int_id {
                return Ok(Term::IntLit(n.clone()));
            }
        }

        let val_opt: Option<NumericLitVal> = match lit {
            NumLit::Int(n) => {
                if let Some((target, min, max)) = nenv.fixed_int_literal_descriptor(ty_id) {
                    if n < min || n > max {
                        return Err(ElabError::FixedWidthLiteralOutOfRange {
                            literal: n.clone(),
                            target,
                            min: min.clone(),
                            max: max.clone(),
                            span: span.clone(),
                        });
                    }
                    Some(crate::numbers::int_lit_val(n))
                } else {
                    None
                }
            }
            NumLit::Float(f) if ty_id == nenv.float_id => Some(NumericLitVal::Float(*f)),
            NumLit::Decimal(c, e) if ty_id == nenv.decimal_id || ty_id == nenv.decimalpair_id => {
                Some(NumericLitVal::Decimal {
                    coeff: c.clone(),
                    exp: *e,
                })
            }
            NumLit::Float32(f) if ty_id == nenv.float32_id => Some(NumericLitVal::Float32(*f)),
            _ => None,
        };
        if let Some(val) = val_opt {
            // Checked numeric literals are accounting-neutral values; see
            // `elab_num_lit_infer`.
            let postulate_id =
                declare_primitive(cx.env, vec![], exp_wh.clone(), PrimReduction::Literal).map_err(
                    |e| ElabError::KernelRejected {
                        error: e,
                        span: span.clone(),
                    },
                )?;
            cx.num_values.insert(postulate_id, val);
            return Ok(Term::const_(postulate_id, vec![]));
        }
    }

    // Fall through: infer default type, then unify with expected.
    let (core, inferred_ty) = elab_num_lit_infer(cx, lit, span)?;
    unify_types(&mut cx.metas, expected, &inferred_ty);
    Ok(core)
}

// ----- string literal helper -----

/// Elaborate a string literal (`37 §2.1`, VAL1-surface).
///
/// `expected` is `Some(ty)` in the check path, `None` in the infer path.
/// Always resolves to `String` type; if an expected type is provided the
/// caller is responsible for unifying (or delegating to `check`).
fn elab_str_lit(
    cx: &mut ElabCtx,
    s: &str,
    expected: Option<&Term>,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let str_id = cx
        .globals
        .get("String")
        .copied()
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: "String".to_owned(),
            span: span.clone(),
        })?;
    let str_ty = Term::const_(str_id, vec![]);
    if let Some(exp) = expected {
        unify_types(&mut cx.metas, exp, &str_ty);
    }
    // Checked string literals are accounting-neutral values; see
    // `elab_num_lit_infer`.
    let lit_id = declare_primitive(cx.env, vec![], str_ty.clone(), PrimReduction::Literal)
        .map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    cx.num_values
        .insert(lit_id, NumericLitVal::Str(crate::NfcString::new(s)));
    Ok((Term::const_(lit_id, vec![]), str_ty))
}

/// Elaborate a character literal (`31 §3`) -- one decoded Unicode scalar,
/// already validated by the lexer's cardinality check. `Char` is `{c : Int |
/// isScalar c}` (`decimal_char.rs`), so the literal's value is its codepoint
/// stored as an ordinary `NumericLitVal::Int` -- every existing Int-consuming
/// bridge (evaluation, native codegen) sees it unchanged; only the surface
/// type is `Char`.
fn elab_char_lit(cx: &mut ElabCtx, c: char, span: &Span) -> Result<(Term, Term), ElabError> {
    let char_id = cx
        .globals
        .get("Char")
        .copied()
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: "Char".to_owned(),
            span: span.clone(),
        })?;
    let char_ty = Term::const_(char_id, vec![]);
    let lit_id = declare_primitive(cx.env, vec![], char_ty.clone(), PrimReduction::Literal)
        .map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    cx.num_values
        .insert(lit_id, NumericLitVal::Int(num_bigint::BigInt::from(c as u32)));
    Ok((Term::const_(lit_id, vec![]), char_ty))
}

/// Elaborate a byte-string literal (`31 §3`), mirroring `elab_str_lit`
/// exactly -- `Bytes` is `declare_primitive(OpaqueType)`, same family as
/// `String`, so an opaque literal typed directly at `Bytes` is the same
/// mechanism, carrying the decoded byte vector instead of an `NfcString`.
fn elab_bytes_lit(cx: &mut ElabCtx, bytes: &[u8], span: &Span) -> Result<(Term, Term), ElabError> {
    let bytes_id = cx
        .globals
        .get("Bytes")
        .copied()
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: "Bytes".to_owned(),
            span: span.clone(),
        })?;
    let bytes_ty = Term::const_(bytes_id, vec![]);
    let lit_id = declare_primitive(cx.env, vec![], bytes_ty.clone(), PrimReduction::Literal)
        .map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    cx.num_values
        .insert(lit_id, NumericLitVal::Bytes(bytes.to_vec()));
    Ok((Term::const_(lit_id, vec![]), bytes_ty))
}

/// Returns the default (Val, TypeId) for a literal without an expected type.
fn num_lit_default_type(lit: &NumLit, nenv: &NumericEnv) -> (NumericLitVal, GlobalId) {
    match lit {
        NumLit::Int(n) => (NumericLitVal::Int(n.clone()), nenv.int_id),
        NumLit::Float(f) => (NumericLitVal::Float(*f), nenv.float_id),
        NumLit::Decimal(c, e) => (
            NumericLitVal::Decimal {
                coeff: c.clone(),
                exp: *e,
            },
            nenv.decimal_id,
        ),
        NumLit::Float32(f) => (NumericLitVal::Float32(*f), nenv.float32_id),
    }
}

/// Elaborate a type-directed binary operator.
///
/// Infers the LHS type, dispatches to the right op, and emits an obligation for
/// fixed-width addition (`35 §3`, `43 §2`).
fn elab_binop(
    cx: &mut ElabCtx,
    op: &BinOp,
    lhs: &RExpr,
    rhs: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let (lhs_core, lhs_ty) = infer(cx, lhs)?;
    let lhs_ty_wh = whnf(cx.env, &cx.ctx, &lhs_ty);

    match op {
        BinOp::Add | BinOp::WrappingAdd => {
            let entry: &AddEntry =
                cx.numeric_env
                    .classify_add(&lhs_ty_wh)
                    .ok_or_else(|| ElabError::TypeMismatch {
                        span: span.clone(),
                        reason: format!("'+' / '+%' not supported on this type"),
                    })?;
            let result_ty = Term::const_(entry.result_id, vec![]);
            let rhs_core = check(cx, rhs, &result_ty, span)?;
            let op_id = if matches!(op, BinOp::WrappingAdd) {
                entry.wrapping_id.ok_or_else(|| ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: format!("'+%' wrapping not available on this type"),
                })?
            } else {
                entry.op_id
            };
            let op_term = Term::const_(op_id, vec![]);
            let applied = Term::app(Term::app(op_term, lhs_core.clone()), rhs_core.clone());

            // Emit no-overflow obligation for bare '+' on fixed-width types.
            if matches!(op, BinOp::Add) {
                if let Some(novf_id) = entry.no_ovf_id {
                    // phi = NoOvf a b : Ω₀
                    let phi = Term::app(
                        Term::app(Term::const_(novf_id, vec![]), lhs_core.clone()),
                        rhs_core.clone(),
                    );
                    let closed = close_goal(&cx.ctx, phi);
                    let hole_id =
                        declare_postulate(cx.env, cx.owner_label.clone(), vec![], closed.clone())
                            .map_err(|e| ElabError::KernelRejected {
                            error: e,
                            span: span.clone(),
                        })?;
                    let obl_id = cx.obl_counter;
                    cx.obl_counter += 1;
                    cx.obligations.push(Obligation {
                        id: obl_id,
                        hole_id,
                        goal_closed: closed,
                        span: span.clone(),
                        kind: ObligationKind::PartialPrim,
                    });
                }
            }

            Ok((applied, result_ty))
        }

        BinOp::Sub => {
            let entry: &BinOpEntry =
                cx.numeric_env
                    .classify_sub(&lhs_ty_wh)
                    .ok_or_else(|| ElabError::TypeMismatch {
                        span: span.clone(),
                        reason: format!("'-' not supported on this type"),
                    })?;
            let result_ty = Term::const_(entry.result_id, vec![]);
            let rhs_core = check(cx, rhs, &result_ty, span)?;
            let op_term = Term::const_(entry.op_id, vec![]);
            let applied = Term::app(Term::app(op_term, lhs_core), rhs_core);
            Ok((applied, result_ty))
        }

        BinOp::Mul => {
            let entry: &BinOpEntry =
                cx.numeric_env
                    .classify_mul(&lhs_ty_wh)
                    .ok_or_else(|| ElabError::TypeMismatch {
                        span: span.clone(),
                        reason: format!("'*' not supported on this type"),
                    })?;
            let result_ty = Term::const_(entry.result_id, vec![]);
            let rhs_core = check(cx, rhs, &result_ty, span)?;
            let op_term = Term::const_(entry.op_id, vec![]);
            let applied = Term::app(Term::app(op_term, lhs_core), rhs_core);
            Ok((applied, result_ty))
        }

        BinOp::EqEq => {
            let eq_entry =
                cx.numeric_env
                    .classify_eq(&lhs_ty_wh)
                    .ok_or_else(|| ElabError::TypeMismatch {
                        span: span.clone(),
                        reason: format!("'==' not supported on this type"),
                    })?;
            let rhs_core = check(cx, rhs, &lhs_ty_wh, span)?;
            let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
            let op_term = Term::const_(eq_entry.op_id, vec![]);
            let applied = Term::app(Term::app(op_term, lhs_core), rhs_core);
            Ok((applied, bool_ty))
        }
    }
}

// ----- goal closing -----

/// Close an open goal over the local context.
///
/// Given `goal` valid in `ctx` (depth = n), builds `Pi(T_{n-1}, ..., Pi(T_0, goal))`
/// — the universally quantified form suitable for `declare_postulate`.
///
/// Limitation (V1): works correctly for independent parameter types (no mutual
/// de Bruijn references between stored types). Sufficient for all V1 conformance
/// cases.
fn close_goal(ctx: &Context, goal: Term) -> Term {
    let n = ctx.types.len();
    let mut result = goal;
    // Wrap from innermost (Var(0)) to outermost (Var(n-1))
    for i in 0..n {
        // types[n-1-i] = stored type of Var(i) (innermost-first indexing)
        let stored_ty = ctx.types[n - 1 - i].clone();
        result = Term::pi(stored_ty, result);
    }
    result
}

// ----- declaration elaboration -----

/// V0-compatible elaboration (no spec clauses).
pub fn elaborate_rdecl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    rdecl: &RDecl,
) -> Result<GlobalId, ElabError> {
    let class_dependent = match &rdecl.kind {
        RDeclKind::RecordDecl { .. }
        | RDeclKind::ClassDecl { .. }
        | RDeclKind::InstanceDecl { .. }
        | RDeclKind::DeriveDecl { .. } => true,
        RDeclKind::View { constraints, .. } => !constraints.is_empty(),
        _ => false,
    };
    if class_dependent {
        return Err(ElabError::TypeMismatch {
            span: rdecl.span.clone(),
            reason: "class-dependent declarations require `elaborate_rdecl_v1` with an initialized `ClassEnv`"
                .into(),
        });
    }
    let mut sentinel = ClassEnv::sentinel();
    let result = elaborate_rdecl_v1(env, globals, num_values, numeric_env, &mut sentinel, rdecl)?;
    Ok(result.def_id)
}

/// Peel a left-nested `RType` application spine headed by `RCon(name)`
/// applied to exactly `arity` arguments (the `RType`-side sibling of
/// `peel_named_app`, used for the `Eq A a b` type-position spelling).
fn peel_named_rtype_app<'a>(ty: &'a RType, name: &str, arity: usize) -> Option<Vec<&'a RType>> {
    let mut args: Vec<&RType> = Vec::new();
    let mut cur = ty;
    loop {
        match cur {
            RType::RApp(f, a, _) => {
                args.push(a.as_ref());
                cur = f.as_ref();
            }
            RType::RCon(n, _) if n == name && args.len() == arity => {
                args.reverse();
                return Some(args);
            }
            _ => return None,
        }
    }
}

/// Extract the outermost constructor name from a resolved type for
/// `instance_search` key lookup (`37 §6`, L3b).
fn rtype_head_name(ty: &RType) -> String {
    match ty {
        RType::RCon(name, _) => name.clone(),
        RType::RApp(f, _, _) => rtype_head_name(f),
        RType::RVarTy(_, name, _) => name.clone(),
        _ => String::new(),
    }
}

fn instantiate_instance_rtype(ty: &RType, args: &[RType], param_count: usize) -> RType {
    match ty {
        RType::RVarTy(index, _, _) if *index < param_count => args[param_count - 1 - index].clone(),
        RType::RApp(f, a, span) => RType::RApp(
            Box::new(instantiate_instance_rtype(f, args, param_count)),
            Box::new(instantiate_instance_rtype(a, args, param_count)),
            span.clone(),
        ),
        RType::RArr(a, b, span) => RType::RArr(
            Box::new(instantiate_instance_rtype(a, args, param_count)),
            Box::new(instantiate_instance_rtype(b, args, param_count)),
            span.clone(),
        ),
        RType::REffectArr(a, row, b, span) => RType::REffectArr(
            Box::new(instantiate_instance_rtype(a, args, param_count)),
            row.clone(),
            Box::new(instantiate_instance_rtype(b, args, param_count)),
            span.clone(),
        ),
        RType::RRefine(name, carrier, prop, span) => RType::RRefine(
            name.clone(),
            Box::new(instantiate_instance_rtype(carrier, args, param_count)),
            prop.clone(),
            span.clone(),
        ),
        _ => ty.clone(),
    }
}

fn rtypes_match(left: &RType, right: &RType) -> bool {
    match (left, right) {
        (RType::RCon(left, _), RType::RCon(right, _)) => left == right,
        (RType::RVarTy(left, _, _), RType::RVarTy(right, _, _)) => left == right,
        (RType::RUniv(left, _), RType::RUniv(right, _)) => left == right,
        (RType::RApp(left_f, left_a, _), RType::RApp(right_f, right_a, _)) => {
            rtypes_match(left_f, right_f) && rtypes_match(left_a, right_a)
        }
        (RType::RArr(left_a, left_b, _), RType::RArr(right_a, right_b, _)) => {
            rtypes_match(left_a, right_a) && rtypes_match(left_b, right_b)
        }
        (
            RType::REffectArr(left_a, left_row, left_b, _),
            RType::REffectArr(right_a, right_row, right_b, _),
        ) => {
            left_row == right_row && rtypes_match(left_a, right_a) && rtypes_match(left_b, right_b)
        }
        _ => false,
    }
}

fn match_instance_head(
    pattern: &RType,
    requested: &RType,
    param_count: usize,
    args: &mut [Option<RType>],
) -> bool {
    match pattern {
        RType::RVarTy(index, _, _) if *index < param_count => {
            let slot = param_count - 1 - index;
            match &args[slot] {
                Some(previous) => rtypes_match(previous, requested),
                None => {
                    args[slot] = Some(requested.clone());
                    true
                }
            }
        }
        RType::RCon(name, _) => matches!(requested, RType::RCon(other, _) if name == other),
        RType::RApp(pattern_f, pattern_a, _) => match requested {
            RType::RApp(requested_f, requested_a, _) => {
                match_instance_head(pattern_f, requested_f, param_count, args)
                    && match_instance_head(pattern_a, requested_a, param_count, args)
            }
            _ => false,
        },
        _ => false,
    }
}

/// Resolve an instance and recursively apply every prerequisite dictionary.
/// The returned candidate is immediately kernel-inferred, so an elaborator
/// wiring error fails closed before it can become a local dictionary binding.
fn resolve_instance_dictionary(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    ctx: &Context,
    class_name: &str,
    requested: &RType,
    span: &Span,
    owner_label: &str,
) -> Result<(Term, Term), ElabError> {
    resolve_instance_dictionary_inner(
        env,
        globals,
        num_values,
        numeric_env,
        class_env,
        ctx,
        class_name,
        requested,
        span,
        owner_label,
        true,
    )
}

#[allow(clippy::too_many_arguments)]
fn resolve_instance_dictionary_inner(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    ctx: &Context,
    class_name: &str,
    requested: &RType,
    span: &Span,
    owner_label: &str,
    enforce_direct_use: bool,
) -> Result<(Term, Term), ElabError> {
    let head_name = rtype_head_name(requested);
    let info = class_env
        .instances
        .get(&(class_name.to_string(), head_name.clone()))
        .cloned()
        .ok_or_else(|| ElabError::NoInstance {
            class: class_name.to_string(),
            ty: head_name.clone(),
            span: span.clone(),
        })?;
    if enforce_direct_use {
        if let Some(admitted) = &class_env.direct_use_packages {
            let self_admitted =
                class_env.current_package.as_deref() == Some(info.defining_package.as_str());
            let sole_implicit_provider = class_env.implicit_single_provider
                && class_env.source_instance_packages.len() == 1
                && class_env
                    .source_instance_packages
                    .contains(&info.defining_package);
            if !self_admitted
                && !sole_implicit_provider
                && !admitted.contains(&info.defining_package)
                && !class_env.direct_use_instances.contains(&info.instance_id)
            {
                return Err(ElabError::UnadmittedInstance {
                    defining_package: info.defining_package.clone(),
                    class: class_name.to_string(),
                    head_type: head_name.clone(),
                    instance_id: info.instance_id,
                    span: span.clone(),
                });
            }
        }
    }
    let type_args = if info.head_param_count == 0 {
        Vec::new()
    } else if let Some(pattern) = &info.head_type {
        let mut matched = vec![None; info.head_param_count];
        if !match_instance_head(pattern, requested, info.head_param_count, &mut matched) {
            return Err(ElabError::NoInstance {
                class: class_name.to_string(),
                ty: rtype_head_name(requested),
                span: span.clone(),
            });
        }
        matched
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| ElabError::NoInstance {
                class: class_name.to_string(),
                ty: rtype_head_name(requested),
                span: span.clone(),
            })?
    } else {
        return Err(ElabError::NoInstance {
            class: class_name.to_string(),
            ty: rtype_head_name(requested),
            span: span.clone(),
        });
    };
    let core_args = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, owner_label);
        for ty in &ctx.types {
            cx.ctx.push(ty.clone());
        }
        let mut args = Vec::with_capacity(type_args.len());
        for arg in &type_args {
            let core = elab_type(&mut cx, arg)?;
            args.push(cx.metas.zonk_term(&core));
        }
        args
    };
    let mut candidate =
        ken_kernel::subst::apply_args(Term::const_(info.instance_id, vec![]), &core_args);
    for constraint in &info.constraints {
        let required_head =
            instantiate_instance_rtype(&constraint.head_type, &type_args, info.head_param_count);
        let (dictionary, _) = resolve_instance_dictionary_inner(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            ctx,
            &constraint.class_name,
            &required_head,
            span,
            owner_label,
            false,
        )?;
        candidate = Term::app(candidate, dictionary);
    }
    let ty = kernel_infer(env, ctx, &candidate).map_err(|error| ElabError::KernelRejected {
        error,
        span: span.clone(),
    })?;
    if enforce_direct_use {
        class_env
            .resolution_provenance
            .push(crate::classes::InstanceResolution {
                instance_id: info.instance_id,
                class_name: class_name.to_string(),
                head_type: head_name,
                defining_package: info.defining_package.clone(),
            });
    }
    Ok((candidate, ty))
}

fn check_view_visits_row(rdecl: &RDecl) -> Result<Option<crate::effects::RowType>, ElabError> {
    let visits = match &rdecl.kind {
        RDeclKind::View {
            visits: Some(row), ..
        } => row,
        _ => return Ok(None),
    };

    // D1 fail-closed rule: row variables are bound by a higher-order latent-row
    // occurrence in the declaration type, then referenced again in `visits`.
    // Until production latent-row extraction is wired, this map is empty, so
    // `[e]` / `[E | e]` reject instead of minting a fresh row variable here.
    let row_vars = crate::effects::row_var_map(&[]);
    let mut decl = crate::effects::EffectDecl::new(&rdecl.name);

    let declared =
        crate::effects::surface_row_to_row_type(visits, &row_vars).map_err(|reason| {
            ElabError::TypeMismatch {
                span: visits.span.clone(),
                reason,
            }
        })?;
    decl = decl.with_declared_row_type(declared.clone());

    let rows = crate::effects::infer_all_poly(&HashMap::new(), &[decl.clone()]);
    let inferred = rows.get(&rdecl.name).ok_or_else(|| {
        ElabError::Internal(format!("effect row inference omitted '{}'", rdecl.name))
    })?;
    crate::effects::check_decl_poly(&decl, inferred, &crate::effects::EffectRow::empty()).map_err(
        |err| ElabError::TypeMismatch {
            span: visits.span.clone(),
            reason: err.to_string(),
        },
    )?;

    Ok(Some(declared))
}

pub fn surface_declared_row_type(
    rdecl: &RDecl,
) -> Result<Option<crate::effects::RowType>, ElabError> {
    let visits = match &rdecl.kind {
        RDeclKind::View {
            visits: Some(row), ..
        } => row,
        _ => return Ok(None),
    };
    let row_vars = crate::effects::row_var_map(&[]);
    crate::effects::surface_row_to_row_type(visits, &row_vars)
        .map(Some)
        .map_err(|reason| ElabError::TypeMismatch {
            span: visits.span.clone(),
            reason,
        })
}

fn is_empty_closed_row(row: &crate::effects::RowType) -> bool {
    row.concrete_effects().is_empty() && row.row_vars().is_empty()
}

fn explicit_value_param_count_from_type(ty: &RType) -> usize {
    match ty {
        RType::RPi(_, domain, codomain, _) => {
            let domain_is_type_param = matches!(&**domain, RType::RUniv(_, _));
            usize::from(!domain_is_type_param) + explicit_value_param_count_from_type(codomain)
        }
        _ => 0,
    }
}

fn explicit_value_param_count_from_field_type(ty: &RType) -> usize {
    match ty {
        RType::RPi(_, domain, codomain, _) => {
            let domain_is_type_param = matches!(&**domain, RType::RUniv(_, _));
            usize::from(!domain_is_type_param)
                + explicit_value_param_count_from_field_type(codomain)
        }
        RType::RArr(_, codomain, _) | RType::REffectArr(_, _, codomain, _) => {
            1 + explicit_value_param_count_from_field_type(codomain)
        }
        _ => 0,
    }
}

fn type_contains_effect_row(ty: &RType) -> bool {
    match ty {
        RType::REffectArr(_, _, _, _) => true,
        RType::RPi(_, domain, codomain, _)
        | RType::RSigma(_, domain, codomain, _)
        | RType::RArr(domain, codomain, _) => {
            type_contains_effect_row(domain) || type_contains_effect_row(codomain)
        }
        RType::RApp(f, a, _) => type_contains_effect_row(f) || type_contains_effect_row(a),
        RType::RRefine(_, carrier, _, _) => type_contains_effect_row(carrier),
        RType::RUniv(_, _) | RType::RCon(_, _) | RType::RVarTy(_, _, _) => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RTypeHead {
    Con(String),
    Var(usize, String),
}

fn rtype_heads_match(a: &RTypeHead, b: &RTypeHead) -> bool {
    match (a, b) {
        (RTypeHead::Con(a), RTypeHead::Con(b)) => a == b,
        (RTypeHead::Var(_, a), RTypeHead::Var(_, b)) => a == b,
        (RTypeHead::Con(a), RTypeHead::Var(_, b)) | (RTypeHead::Var(_, a), RTypeHead::Con(b)) => {
            a == b
        }
    }
}

fn rtype_app_head(ty: &RType) -> Option<RTypeHead> {
    match ty {
        RType::RApp(f, _, _) => rtype_app_head(f),
        RType::RCon(name, _) => Some(RTypeHead::Con(name.clone())),
        RType::RVarTy(index, name, _) => Some(RTypeHead::Var(*index, name.clone())),
        _ => None,
    }
}

fn rtype_is_app_headed_by(ty: &RType, head: &RTypeHead) -> bool {
    matches!(ty, RType::RApp(_, _, _))
        && rtype_app_head(ty)
            .as_ref()
            .is_some_and(|candidate| rtype_heads_match(candidate, head))
}

fn type_is_applicative_dict_for_head(ty: &RType, head: &RTypeHead) -> bool {
    match ty {
        RType::RApp(f, arg, _) => {
            matches!(&**f, RType::RCon(name, _) if name == "Applicative")
                && rtype_app_head(arg)
                    .as_ref()
                    .is_some_and(|candidate| rtype_heads_match(candidate, head))
        }
        _ => false,
    }
}

fn callback_result_head(ty: &RType) -> Option<RTypeHead> {
    match ty {
        RType::RArr(_, codomain, _) | RType::REffectArr(_, _, codomain, _) => {
            let head = rtype_app_head(codomain)?;
            rtype_is_app_headed_by(codomain, &head).then_some(head)
        }
        _ => None,
    }
}

fn collect_field_arrow_chain<'a>(ty: &'a RType, args: &mut Vec<&'a RType>) -> &'a RType {
    match ty {
        RType::RPi(_, domain, codomain, _) => {
            args.push(domain);
            collect_field_arrow_chain(codomain, args)
        }
        RType::RArr(domain, codomain, _) | RType::REffectArr(domain, _, codomain, _) => {
            args.push(domain);
            collect_field_arrow_chain(codomain, args)
        }
        _ => ty,
    }
}

fn type_has_applicative_row_polymorphic_contract(ty: &RType) -> bool {
    let mut args = Vec::new();
    let result = collect_field_arrow_chain(ty, &mut args);
    for arg in &args {
        let Some(head) = callback_result_head(arg) else {
            continue;
        };
        if rtype_is_app_headed_by(result, &head)
            && args
                .iter()
                .any(|candidate| type_is_applicative_dict_for_head(candidate, &head))
        {
            return true;
        }
    }
    false
}

fn field_type_earns_proc(ty: &RType) -> bool {
    type_contains_effect_row(ty) || type_has_applicative_row_polymorphic_contract(ty)
}

fn class_field_declared_row(keyword: DefKeyword, field_name: &str) -> crate::effects::RowType {
    match keyword {
        DefKeyword::Proc => {
            crate::effects::RowType::singleton(format!("proc class field `{}`", field_name))
        }
        DefKeyword::Const | DefKeyword::Fn => crate::effects::RowType::empty(),
    }
}

fn check_class_field_marker(
    keyword: DefKeyword,
    field_name: &str,
    ty: &RType,
    span: &Span,
) -> Result<(), ElabError> {
    let explicit_value_params = explicit_value_param_count_from_field_type(ty);
    let earns_proc = field_type_earns_proc(ty);
    match keyword {
        DefKeyword::Const | DefKeyword::Fn if earns_proc => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "`{:?}` class field `{}` declares a latent or row-polymorphic effect; use `proc`",
                keyword, field_name
            ),
        }),
        DefKeyword::Const if explicit_value_params > 0 => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "`const` class field `{}` has {} explicit value parameter(s); use `fn`",
                field_name, explicit_value_params
            ),
        }),
        DefKeyword::Fn if explicit_value_params == 0 => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "`fn` class field `{}` has zero explicit value parameters; use `const`",
                field_name
            ),
        }),
        DefKeyword::Proc if explicit_value_params == 0 => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "`proc` class field `{}` has zero explicit value parameters; use `const`",
                field_name
            ),
        }),
        DefKeyword::Proc if !earns_proc => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "`proc` class field `{}` declares no latent or row-polymorphic effect; use `fn`/`const` for pure fields",
                field_name
            ),
        }),
        DefKeyword::Const | DefKeyword::Fn | DefKeyword::Proc => Ok(()),
    }
}

fn leading_lambda_count(expr: &RExpr) -> usize {
    match expr {
        RExpr::RLam(_, body, _) => 1 + leading_lambda_count(body),
        _ => 0,
    }
}

fn explicit_value_param_count(rdecl: &RDecl) -> usize {
    rdecl
        .ty
        .as_ref()
        .map(explicit_value_param_count_from_type)
        .unwrap_or_else(|| leading_lambda_count(&rdecl.body))
}

fn decl_eval_body(expr: &RExpr) -> &RExpr {
    match expr {
        RExpr::RLam(_, body, _) => decl_eval_body(body),
        _ => expr,
    }
}

struct ProjectionPurityCtx<'a> {
    globals: &'a HashMap<String, GlobalId>,
    class_env: &'a ClassEnv,
    local_constraints: &'a [RInstanceConstraint],
    bound_dict_classes: &'a [(String, String)],
}

fn instance_class_for_global<'a>(
    class_env: &'a ClassEnv,
    instance_id: GlobalId,
) -> Option<&'a str> {
    class_env
        .instances
        .values()
        .find(|inst| inst.instance_id == instance_id)
        .map(|inst| inst.class_name.as_str())
}

fn projected_instance_id(base: &RExpr, ctx: &ProjectionPurityCtx<'_>) -> Option<GlobalId> {
    match base {
        RExpr::RCon(name, _)
            if ctx.local_constraints.len() == 1
                && (name == "d" || name == &ctx.local_constraints[0].binder) =>
        {
            let constraint = &ctx.local_constraints[0];
            ctx.class_env.instance_search(
                &constraint.class_name,
                &rtype_head_name(&constraint.head_type),
            )
        }
        RExpr::RCon(name, _) => {
            if let Some(constraint) = ctx
                .local_constraints
                .iter()
                .find(|constraint| constraint.binder == *name)
            {
                ctx.class_env.instance_search(
                    &constraint.class_name,
                    &rtype_head_name(&constraint.head_type),
                )
            } else {
                ctx.globals.get(name).copied()
            }
        }
        _ => None,
    }
}

fn projected_field_row_type(
    base: &RExpr,
    field: &str,
    ctx: Option<&ProjectionPurityCtx<'_>>,
) -> crate::effects::RowType {
    let Some(ctx) = ctx else {
        return crate::effects::RowType::empty();
    };
    if let RExpr::RVar(_, name, _) = base {
        if let Some((_, class_name)) = ctx.bound_dict_classes.iter().find(|(n, _)| n == name) {
            return projected_class_field_row_type(ctx.class_env, class_name, field);
        }
    }
    let Some(instance_id) = projected_instance_id(base, ctx) else {
        return crate::effects::RowType::empty();
    };
    let Some(class_name) = instance_class_for_global(ctx.class_env, instance_id) else {
        return crate::effects::RowType::empty();
    };
    let Some(class_info) = ctx.class_env.class(class_name) else {
        return crate::effects::RowType::empty();
    };
    let Some(idx) = class_info
        .projection
        .field_names
        .iter()
        .position(|n| n == field)
    else {
        return crate::effects::RowType::empty();
    };
    if let Some(row) = ctx
        .class_env
        .instances
        .values()
        .find(|inst| inst.instance_id == instance_id)
        .and_then(|inst| inst.field_effect_rows.get(idx))
        .filter(|row| !is_empty_closed_row(row))
    {
        return row.clone();
    }
    match class_info.field_purities.get(idx).copied().flatten() {
        Some(DefKeyword::Proc) => crate::effects::RowType::singleton(format!(
            "projected proc class field `{}.{}`",
            class_name, field
        )),
        _ => crate::effects::RowType::empty(),
    }
}

fn projected_class_field_row_type(
    class_env: &ClassEnv,
    class_name: &str,
    field: &str,
) -> crate::effects::RowType {
    let Some(class_info) = class_env.class(class_name) else {
        return crate::effects::RowType::empty();
    };
    let Some(idx) = class_info
        .projection
        .field_names
        .iter()
        .position(|n| n == field)
    else {
        return crate::effects::RowType::empty();
    };
    match class_info.field_purities.get(idx).copied().flatten() {
        Some(DefKeyword::Proc) => crate::effects::RowType::singleton(format!(
            "projected proc class field `{}.{}`",
            class_name, field
        )),
        _ => crate::effects::RowType::empty(),
    }
}

fn class_name_for_dictionary_type(class_env: &ClassEnv, ty: &RType) -> Option<String> {
    let head = rtype_head_name(ty);
    class_env.class(&head).is_some().then_some(head)
}

fn collect_bound_dictionary_params(
    ty: Option<&RType>,
    class_env: &ClassEnv,
) -> Vec<(String, String)> {
    let mut dicts = Vec::new();
    let mut cur = ty;
    while let Some(RType::RPi(name, domain, codomain, _)) = cur {
        if let Some(class_name) = class_name_for_dictionary_type(class_env, domain) {
            dicts.push((name.clone(), class_name));
        }
        cur = Some(codomain);
    }
    dicts
}

fn infer_expr_row_type(
    expr: &RExpr,
    effect_rows: &HashMap<String, crate::effects::RowType>,
    projection_ctx: Option<&ProjectionPurityCtx<'_>>,
) -> crate::effects::RowType {
    match expr {
        RExpr::RCon(name, _) => effect_rows
            .get(name)
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RVar(_, _, _)
        | RExpr::RCell(_, _, _)
        | RExpr::RRecursiveResult { .. }
        | RExpr::RUniv(_, _)
        | RExpr::RNumLit(_, _)
        | RExpr::RStr(_, _)
        | RExpr::RCharLit(_, _)
        | RExpr::RByteStr(_, _) => crate::effects::RowType::empty(),
        RExpr::RApp(f, a, _) => infer_expr_row_type(f, effect_rows, projection_ctx)
            .join(infer_expr_row_type(a, effect_rows, projection_ctx)),
        RExpr::RLam(_, _, _)
        | RExpr::RPi(_, _, _, _)
        | RExpr::RArrow(_, _, _)
        | RExpr::RTrunc(_, _) => crate::effects::RowType::empty(),
        RExpr::RAttachedProofRef {
            subject,
            proof_name,
            ..
        } => effect_rows
            .get(&format!("{subject}::{proof_name}"))
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RLet(_, _, val, body, _) => infer_expr_row_type(val, effect_rows, projection_ctx)
            .join(infer_expr_row_type(body, effect_rows, projection_ctx)),
        RExpr::RAsc(e, _, _) | RExpr::ROld(e, _) | RExpr::RBecomes(_, _, e, _) => {
            infer_expr_row_type(e, effect_rows, projection_ctx)
        }
        RExpr::RPair(components, _) => components
            .iter()
            .fold(crate::effects::RowType::empty(), |row, component| {
                row.join(infer_expr_row_type(component, effect_rows, projection_ctx))
            }),
        RExpr::RRecord { base, fields, .. } => {
            let mut row = base
                .as_deref()
                .map(|base| infer_expr_row_type(base, effect_rows, projection_ctx))
                .unwrap_or_else(crate::effects::RowType::empty);
            for (_, value, _) in fields {
                row = row.join(infer_expr_row_type(value, effect_rows, projection_ctx));
            }
            row
        }
        RExpr::RPosProj(e, _, _) => infer_expr_row_type(e, effect_rows, projection_ctx),
        RExpr::RProj(e, field, _) => infer_expr_row_type(e, effect_rows, projection_ctx)
            .join(projected_field_row_type(e, field, projection_ctx)),
        RExpr::RBinOp(_, l, r, _) => infer_expr_row_type(l, effect_rows, projection_ctx)
            .join(infer_expr_row_type(r, effect_rows, projection_ctx)),
        RExpr::RMatch { scrut, arms, .. } => {
            let mut row = infer_expr_row_type(scrut, effect_rows, projection_ctx);
            for arm in arms {
                row = row.join(infer_expr_row_type(&arm.body, effect_rows, projection_ctx));
            }
            row
        }
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            ..
        } => infer_expr_row_type(condition, effect_rows, projection_ctx)
            .join(infer_expr_row_type(
                then_branch,
                effect_rows,
                projection_ctx,
            ))
            .join(infer_expr_row_type(
                else_branch,
                effect_rows,
                projection_ctx,
            )),
    }
}

/// SURF-1 D2 purity-keyword check (`36 §1.6`) over the current production
/// declaration path. Legacy `view` stays unchecked until the D3/D4 migration.
pub fn check_surface_purity(
    rdecl: &RDecl,
    effect_rows: &HashMap<String, crate::effects::RowType>,
    globals: &HashMap<String, GlobalId>,
    class_env: &ClassEnv,
) -> Result<(), ElabError> {
    let (keyword, is_space_op, visits, constraints) = match &rdecl.kind {
        RDeclKind::View {
            keyword,
            is_space_op,
            visits,
            constraints,
        } => (*keyword, *is_space_op, visits, constraints.as_slice()),
        _ => return Ok(()),
    };

    let declared = surface_declared_row_type(rdecl)?.unwrap_or_else(crate::effects::RowType::empty);
    let bound_dict_classes = collect_bound_dictionary_params(rdecl.ty.as_ref(), class_env);
    let projection_ctx = ProjectionPurityCtx {
        globals,
        class_env,
        local_constraints: constraints,
        bound_dict_classes: &bound_dict_classes,
    };
    let inferred = infer_expr_row_type(
        decl_eval_body(&rdecl.body),
        effect_rows,
        Some(&projection_ctx),
    );
    let decl =
        crate::effects::EffectDecl::new(&rdecl.name).with_declared_row_type(declared.clone());
    crate::effects::check_decl_poly(&decl, &inferred, &crate::effects::EffectRow::empty())
        .map_err(|err| ElabError::TypeMismatch {
            span: rdecl.span.clone(),
            reason: format!("false purity or effect escape in `{}`: {}", rdecl.name, err),
        })?;

    let has_impure_decl = !is_empty_closed_row(&declared) || is_space_op;
    let explicit_value_params = explicit_value_param_count(rdecl);

    match keyword {
        DefKeyword::Const => {
            if explicit_value_params > 0 {
                return Err(ElabError::TypeMismatch {
                    span: rdecl.span.clone(),
                    reason: format!(
                        "`const {}` has {} explicit value parameter(s); use `fn` for a pure function",
                        rdecl.name, explicit_value_params
                    ),
                });
            }
            if has_impure_decl {
                return Err(ElabError::TypeMismatch {
                    span: rdecl.span.clone(),
                    reason: format!(
                        "`const {}` declares an effect row or space operation; use `proc`",
                        rdecl.name
                    ),
                });
            }
        }
        DefKeyword::Fn => {
            if explicit_value_params == 0 {
                return Err(ElabError::TypeMismatch {
                    span: rdecl.span.clone(),
                    reason: format!(
                        "`fn {}` has zero explicit value parameters; use `const`",
                        rdecl.name
                    ),
                });
            }
            if has_impure_decl {
                return Err(ElabError::TypeMismatch {
                    span: rdecl.span.clone(),
                    reason: format!(
                        "`fn {}` declares an effect row or space operation; use `proc`",
                        rdecl.name
                    ),
                });
            }
        }
        DefKeyword::Proc => {
            if !has_impure_decl {
                let expected = if explicit_value_params == 0 {
                    "const"
                } else {
                    "fn"
                };
                return Err(ElabError::TypeMismatch {
                    span: rdecl.span.clone(),
                    reason: format!(
                        "`proc {}` is provably pure with an empty declared row; use `{}`",
                        rdecl.name, expected
                    ),
                });
            }
        }
    }

    if !matches!(keyword, DefKeyword::Proc) && visits.is_some() {
        return Err(ElabError::TypeMismatch {
            span: rdecl.span.clone(),
            reason: "`visits` is only valid on `proc` definitions".to_string(),
        });
    }

    Ok(())
}

/// V1 elaboration: returns the definition id plus any emitted obligation holes.
pub fn elaborate_rdecl_v1(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    elaborate_rdecl_v1_with_effect_rows(
        env,
        globals,
        num_values,
        numeric_env,
        class_env,
        &HashMap::new(),
        rdecl,
    )
}

/// Rebuild just a declaration's explicit parameter context. Constraint terms
/// are installed at this depth, so generic dictionaries can mention the same
/// type/value parameters as the declaration without changing its telescope.
fn declaration_param_context(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    rdecl: &RDecl,
) -> Result<Context, ElabError> {
    let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
    let mut current = rdecl.ty.as_ref();
    while let Some(RType::RPi(_, domain, codomain, _)) = current {
        let domain_core = elab_type(&mut cx, domain)?;
        cx.ctx.push(cx.metas.zonk_term(&domain_core));
        current = Some(codomain);
    }
    Ok(cx.ctx)
}

pub fn elaborate_rdecl_v1_with_effect_rows(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    effect_rows: &HashMap<String, crate::effects::RowType>,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    if matches!(
        rdecl.kind,
        RDeclKind::View {
            keyword: DefKeyword::Fn | DefKeyword::Const,
            ..
        }
    ) {
        if let Some(ty) = &rdecl.ty {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
            let ty = elab_type(&mut cx, ty)?;
            let ty_core = cx.metas.zonk_term(&ty);
            ensure_not_omega_type(cx.env, &Context::new(), &ty_core, &rdecl.span)?;
        }
    }
    match &rdecl.kind {
        RDeclKind::View { constraints, .. } => {
            let effect_row_type = check_view_visits_row(rdecl)?;
            let dictionary_ctx =
                declaration_param_context(env, globals, num_values, numeric_env, rdecl)?;
            // Resolve each constraint into its fully applied dictionary term.
            // A generic instance is not a bare global: its type arguments and
            // recursively-required dictionaries must be applied at this use
            // site before the kernel checks the candidate.
            let mut local_dicts = HashMap::new();
            for constraint in constraints {
                let dictionary = resolve_instance_dictionary(
                    env,
                    globals,
                    num_values,
                    numeric_env,
                    class_env,
                    &dictionary_ctx,
                    &constraint.class_name,
                    &constraint.head_type,
                    &rdecl.span,
                    &rdecl.name,
                )?;
                local_dicts.insert(
                    constraint.binder.clone(),
                    (dictionary.0, dictionary.1, dictionary_ctx.len()),
                );
            }
            // The shared naming rule retains `d` as the sole-constraint alias.
            if constraints.len() == 1 && constraints[0].binder != "d" {
                let dictionary = local_dicts
                    .get(&constraints[0].binder)
                    .cloned()
                    .expect("resolved sole constraint must have its binder");
                local_dicts.insert("d".to_string(), dictionary);
            }
            let mut result = elaborate_view_or_let(
                env,
                globals,
                num_values,
                numeric_env,
                class_env,
                rdecl,
                &local_dicts,
            );
            if let Ok(result) = &mut result {
                result.effect_row_type = effect_row_type;
            }
            result
        }
        RDeclKind::Let => elaborate_view_or_let(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            &HashMap::new(),
        ),
        RDeclKind::Prove => elaborate_prove(env, globals, num_values, numeric_env, rdecl),
        RDeclKind::Prop { intros } => {
            elaborate_prop_decl(env, globals, num_values, numeric_env, rdecl, intros)
        }
        RDeclKind::Theorem => elaborate_checked_theorem(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            None,
        ),
        RDeclKind::AttachedProof { subject, .. } => elaborate_checked_theorem(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            Some(subject),
        ),
        RDeclKind::Law { param, fields } => elaborate_law(
            env,
            globals,
            num_values,
            numeric_env,
            rdecl,
            param.clone(),
            fields.clone(),
        ),
        RDeclKind::DataDecl { type_params, ctors } => {
            let d_id =
                data::elab_data_decl(env, globals, &rdecl.name, type_params, ctors, &rdecl.span)?;
            // Register data type in the module map for orphan check (`33 §5.3`).
            class_env
                .global_modules
                .insert(d_id, class_env.current_module);
            Ok(ElabResult {
                name: rdecl.name.clone(),
                def_id: d_id,
                obligations: vec![],
                foreign_binding: None,
                temporal_obligations: vec![],
                effect_row_type: None,
            })
        }
        RDeclKind::ExplicitDataDecl {
            params,
            indices,
            level,
            ctors,
        } => {
            let d_id = data::elab_explicit_data_decl(
                env,
                globals,
                &rdecl.name,
                params,
                indices,
                *level,
                ctors,
                &rdecl.span,
            )?;
            class_env
                .global_modules
                .insert(d_id, class_env.current_module);
            Ok(ElabResult {
                name: rdecl.name.clone(),
                def_id: d_id,
                obligations: vec![],
                foreign_binding: None,
                temporal_obligations: vec![],
                effect_row_type: None,
            })
        }
        RDeclKind::TypeAlias { ty } => {
            // A definition `def T = A` declares T as a transparent definition
            // of type `Type 0` whose body is A (`34 §2`).
            let (alias_body, alias_id) = {
                let mut cx =
                    ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
                let body = elab_type(&mut cx, ty)?;
                let body_z = cx.metas.zonk_term(&body);
                (body_z, ())
            };
            let _ = alias_id;
            let alias_ty = Term::ty(Level::Zero);
            let id = declare_def(env, vec![], alias_ty, alias_body).map_err(|e| {
                ElabError::KernelRejected {
                    error: e,
                    span: rdecl.span.clone(),
                }
            })?;
            globals.insert(rdecl.name.clone(), id);
            Ok(ElabResult {
                name: rdecl.name.clone(),
                def_id: id,
                obligations: vec![],
                foreign_binding: None,
                temporal_obligations: vec![],
                effect_row_type: None,
            })
        }
        RDeclKind::Foreign {
            symbol,
            library,
            is_pure,
            visits,
        } => elaborate_foreign_decl(
            env,
            globals,
            num_values,
            numeric_env,
            rdecl,
            symbol,
            library,
            *is_pure,
            visits,
        ),
        RDeclKind::Temporal { formula, source } => {
            elaborate_temporal(env, globals, rdecl, formula, source)
        }
        RDeclKind::RecordDecl { fields } => elab_record_decl(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            fields,
        ),
        RDeclKind::ClassDecl {
            param,
            param_kind,
            fields,
        } => elab_class_decl(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            param,
            param_kind.as_ref(),
            fields,
        ),
        RDeclKind::InstanceDecl {
            head_params,
            head_type,
            constraints,
            fields,
        } => elab_instance_decl(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            effect_rows,
            &rdecl.name.clone(),
            head_params,
            head_type,
            constraints,
            fields,
        ),
        RDeclKind::DeriveDecl { data_name } => elab_derive(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            &rdecl.name.clone(),
            data_name,
        ),
    }
}

/// Initialize the typeclass environment, pre-declaring `RecordNil` and
/// `record_nil_val` as structural postulates (`33 §5`).
pub fn init_class_env(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
) -> Result<ClassEnv, ElabError> {
    // RecordNil : Omega 0 — the Σ-chain prop terminator.
    let record_nil_id = declare_postulate(
        env,
        "RecordNil".to_string(),
        vec![],
        Term::omega(Level::Zero),
    )
    .map_err(|e| ElabError::Internal(format!("RecordNil postulate: {}", e)))?;
    globals.insert("RecordNil".to_string(), record_nil_id);
    // record_nil_val : RecordNil — the unique inhabitant.
    let record_nil_val_id = declare_postulate(
        env,
        "record_nil_val".to_string(),
        vec![],
        Term::const_(record_nil_id, vec![]),
    )
    .map_err(|e| ElabError::Internal(format!("record_nil_val postulate: {}", e)))?;
    globals.insert("record_nil_val".to_string(), record_nil_val_id);
    Ok(ClassEnv::initialized(record_nil_id, record_nil_val_id))
}

// ---- typeclass elaboration (`33 §5`, `39 §6`) --------------------------------

/// Sigma chain type for field types `[T1, T2, …, Tn]`.
///
/// Chain: `Sigma(T1, Sigma(T2, …Sigma(Tn, RecordNil)…))`. Each `Ti` MUST
/// already be elaborated in the correct nested context — `T0` in `[a?]`,
/// `T1` in `[a?, T0]`, …, `Ti` in `[a?, T0, …, T_{i-1}]` (a real Σ-telescope,
/// `33 §5.2`: a later field's type may reference an earlier field's VALUE
/// as `Var(0)`, e.g. `refl : (x:a) -> IsTrue (eq x x)`). No `weaken` is
/// needed here — placing `Ti` as the head of `Sigma(Ti, rest)` is *exactly*
/// what "one more binder than `rest`'s context" requires, and that's
/// precisely the context `Ti` was elaborated in.
fn build_sigma_chain(field_types: &[Term], record_nil_id: GlobalId) -> Term {
    let mut acc = Term::const_(record_nil_id, vec![]);
    for t in field_types.iter().rev() {
        acc = Term::sigma(t.clone(), acc);
    }
    acc
}

/// Pair chain value for field values `[v1, v2, …, vn]`.
/// Chain: `Pair(v1, Pair(v2, …Pair(vn, record_nil_val)…))`.
fn build_pair_chain(field_vals: &[Term], record_nil_val_id: GlobalId) -> Term {
    let mut acc = Term::const_(record_nil_val_id, vec![]);
    for v in field_vals.iter().rev() {
        acc = Term::pair(v.clone(), acc);
    }
    acc
}

/// Elaborate a named-field record declaration to the existing transparent
/// right-nested Sigma encoding and register only its shared projection facts.
fn elab_record_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    rdecl: &RDecl,
    fields: &[RRecordField],
) -> Result<ElabResult, ElabError> {
    let field_types = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        let mut types = Vec::new();
        for field in fields {
            let ty = elab_type(&mut cx, &field.ty)?;
            let ty = cx.metas.zonk_term(&ty);
            cx.ctx.push(ty.clone());
            types.push(ty);
        }
        types
    };

    let sigma_chain = build_sigma_chain(&field_types, class_env.record_nil_id);
    let record_sort = kernel_infer(env, &Context::new(), &sigma_chain).map_err(|error| {
        ElabError::KernelRejected {
            error,
            span: rdecl.span.clone(),
        }
    })?;
    let id = declare_def(env, vec![], record_sort, sigma_chain).map_err(|error| {
        ElabError::KernelRejected {
            error,
            span: rdecl.span.clone(),
        }
    })?;
    globals.insert(rdecl.name.clone(), id);
    class_env
        .global_modules
        .insert(id, class_env.current_module);
    class_env.register_record(
        rdecl.name.clone(),
        id,
        fields.iter().map(|field| field.name.clone()).collect(),
        field_types,
    );

    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: id,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

/// Extract the outermost type constructor name from a resolved type.
fn head_type_name(ty: &RType) -> String {
    match ty {
        RType::RCon(s, _) | RType::RVarTy(_, s, _) => s.clone(),
        RType::RApp(f, _, _) => head_type_name(f),
        RType::RUniv(_, _) => "Type".to_string(),
        RType::RArr(_, _, _) | RType::REffectArr(_, _, _, _) | RType::RPi(_, _, _, _) => {
            "->".to_string()
        }
        RType::RSigma(_, _, _, _) => "×".to_string(),
        RType::RRefine(_, inner, _, _) => head_type_name(inner),
    }
}

/// Elaborate `class C A { f1 : T1 ; … }` → Σ-record type (`33 §5`).
///
/// The Σ-chain sort (via `sort_sigma`, `check.rs:192`) determines whether the
/// class is a property class (Ω, coherence-free) or structure class (Type,
/// canonical-instance policy). The class type is admitted via `declare_def`
/// (kernel re-check at `check.rs:944`).
fn elab_class_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    rdecl: &RDecl,
    param: &Option<String>,
    param_kind: Option<&RType>,
    fields: &[RClassField],
) -> Result<ElabResult, ElabError> {
    let span = &rdecl.span;
    let has_param = param.is_some();
    let param_kind_core = if has_param {
        if let Some(kind) = param_kind {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
            let kind_core = elab_type(&mut cx, kind)?;
            cx.metas.zonk_term(&kind_core)
        } else {
            Term::ty(Level::Zero)
        }
    } else {
        Term::ty(Level::Zero)
    };

    // Elaborate each field type incrementally: a real Σ-telescope (`33
    // §5.2`) where a later field's type may reference an EARLIER field's
    // value (a law like `refl : (x:a) -> IsTrue (eq x x)` refers to the
    // `eq` op field). Push each field's OWN elaborated type onto `cx.ctx`
    // before elaborating the next, so `resolve.rs`'s bound `RVarTy`
    // reference for that field name lines up with the real kernel depth.
    let field_types: Vec<Term> = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        if has_param {
            cx.ctx.push(param_kind_core.clone());
        }
        let mut tys = Vec::new();
        for field in fields {
            if let Some(keyword) = field.purity {
                check_class_field_marker(keyword, &field.name, &field.ty, span)?;
            }
            let t = elab_type(&mut cx, &field.ty)?;
            let t = cx.metas.zonk_term(&t);
            cx.ctx.push(t.clone());
            tys.push(t);
        }
        tys
    };

    // Build Σ-chain (under the param binder if present).
    let sigma_chain = build_sigma_chain(&field_types, class_env.record_nil_id);

    // Determine the sort of the Σ-chain by calling kernel infer on it.
    // Sigma inference is supported (`check.rs:276`). We need a context for A.
    let chain_sort = {
        let mut ctx_a = Context::new();
        if has_param {
            ctx_a.push(param_kind_core.clone());
        }
        kernel_infer(env, &ctx_a, &sigma_chain).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?
    };

    // Classify: Ω = property class, Type = structure class.
    let kind = match &chain_sort {
        Term::Omega(_) => ClassKind::Property,
        _ => ClassKind::Structure,
    };

    // Build class type and body.
    let (class_ty, class_body) = if has_param {
        let pi_ty = Term::pi(param_kind_core.clone(), weaken(&chain_sort, 1));
        let lam_body = Term::lam(param_kind_core.clone(), sigma_chain);
        (pi_ty, lam_body)
    } else {
        (chain_sort, sigma_chain)
    };

    let id =
        declare_def(env, vec![], class_ty, class_body).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        })?;
    globals.insert(rdecl.name.clone(), id);
    class_env
        .global_modules
        .insert(id, class_env.current_module);
    class_env.register_class(
        rdecl.name.clone(),
        ClassInfo {
            param: param.clone(),
            param_kind: has_param.then_some(param_kind_core),
            field_names: fields.iter().map(|f| f.name.clone()).collect(),
            field_types: field_types.clone(),
            field_purities: fields.iter().map(|f| f.purity).collect(),
            type_id: id,
            kind,
            module_id: class_env.current_module,
        },
    );

    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: id,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

/// Compute an instance's field VALUES, in class-declaration order, each
/// **checked** (not blindly inferred) against its properly-substituted
/// expected type (`33 §5.3` Σ-Intro re-check) — the load-bearing mechanism
/// for AC3 (`ES4-classes`): a law field's declared type (e.g.
/// `refl : (x:a) -> IsTrue (eq x x)`) is a Σ-telescope term referencing the
/// class param and every EARLIER field by position (`ClassInfo::field_types`,
/// `elab_class_decl`). For THIS instance, substitute the concrete head type
/// for the param and every ALREADY-COMPUTED field value for its slot
/// (`ken_kernel::subst::subst_tel`, outermost-first) to get field `i`'s
/// concrete expected type, then `check` the provided expression against it.
/// A postulated/holed/wrong-shaped proof fails right here (kernel re-check),
/// never silently accepted — the whole "laws PROVED, not postulated" gate.
fn compute_ordered_field_values(
    cx: &mut ElabCtx,
    class_env: &ClassEnv,
    class_name: &str,
    head_name: &str,
    head_core: &Term,
    fields: &[(String, RExpr)],
    effect_rows: &HashMap<String, crate::effects::RowType>,
    span: &Span,
) -> Result<(Vec<Term>, Vec<crate::effects::RowType>), ElabError> {
    let (field_names, field_types, field_purities, has_param) = {
        let ci = class_env
            .class(class_name)
            .ok_or_else(|| ElabError::UnresolvedCon {
                name: class_name.to_string(),
                span: span.clone(),
            })?;
        (
            ci.projection.field_names.to_vec(),
            ci.projection.field_types.to_vec(),
            ci.field_purities.to_vec(),
            ci.projection.head_param.is_some(),
        )
    };
    let mut values: Vec<Term> = Vec::new();
    let mut field_rows: Vec<crate::effects::RowType> = Vec::new();
    for (i, fname) in field_names.iter().enumerate() {
        cx.owner_label = format!("{class_name}.{head_name}.{fname}");
        let pos = fields
            .iter()
            .position(|(n, _)| n == fname)
            .ok_or_else(|| ElabError::Internal(format!("instance missing field '{}'", fname)))?;
        let mut args: Vec<Term> = Vec::new();
        if has_param {
            args.push(head_core.clone());
        }
        args.extend(values.iter().cloned());
        let expected = ken_kernel::subst::subst_tel(&field_types[i], &args);
        let projection_ctx = ProjectionPurityCtx {
            globals: cx.globals,
            class_env,
            local_constraints: &[],
            bound_dict_classes: &[],
        };
        let field_row = infer_expr_row_type(&fields[pos].1, effect_rows, Some(&projection_ctx));
        if let Some(keyword) = field_purities[i] {
            check_instance_field_purity(
                keyword,
                class_name,
                fname,
                &fields[pos].1,
                effect_rows,
                cx.globals,
                class_env,
                span,
            )?;
        }
        let v = check(cx, &fields[pos].1, &expected, span)?;
        values.push(cx.metas.zonk_term(&v));
        field_rows.push(field_row);
    }
    Ok((values, field_rows))
}

fn check_instance_field_purity(
    keyword: DefKeyword,
    class_name: &str,
    field_name: &str,
    expr: &RExpr,
    effect_rows: &HashMap<String, crate::effects::RowType>,
    globals: &HashMap<String, GlobalId>,
    class_env: &ClassEnv,
    span: &Span,
) -> Result<(), ElabError> {
    let projection_ctx = ProjectionPurityCtx {
        globals,
        class_env,
        local_constraints: &[],
        bound_dict_classes: &[],
    };
    let inferred = infer_expr_row_type(expr, effect_rows, Some(&projection_ctx));
    let impure = !is_empty_closed_row(&inferred);
    match keyword {
        // DS-8b (`docs/program/wp/ds-8b-pure-into-proc-widening.md`):
        // covariant subsumption `∅ ⊆ open row` (SURF-1 §1.6 do-not-optimize)
        // — a `proc` field's contract is "may be effectful," and a pure
        // (`∅`-row) witness is a valid, more precise inhabitant of that
        // contract. There used to be a `DefKeyword::Proc if !impure => Err`
        // arm here (an exact-match gate that rejected every pure witness for
        // a `proc` field, leaving e.g. `class Traversable`'s row-polymorphic
        // `proc traverse` field with NO possible lawful instance — every
        // real witness like `list_traverse` is genuinely pure). Removed:
        // a pure witness for a `proc` field now falls through to the
        // catch-all `Ok(())` below, same as it already did for an impure
        // witness. The DANGEROUS direction is untouched — see the next arm.
        DefKeyword::Const | DefKeyword::Fn if impure => {
            let declared = class_field_declared_row(keyword, field_name);
            let decl = crate::effects::EffectDecl::new(&format!("{}.{}", class_name, field_name))
                .with_declared_row_type(declared);
            crate::effects::check_decl_poly(&decl, &inferred, &crate::effects::EffectRow::empty())
                .map_err(|err| ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: format!(
                        "class field `{}.{}` requires `{:?}` but instance implementation is effectful: {}",
                        class_name, field_name, keyword, err
                    ),
                })
        }
        _ => Ok(()),
    }
}

fn push_type0_params(cx: &mut ElabCtx, count: usize) {
    for _ in 0..count {
        cx.ctx.push(Term::ty(Level::Zero));
    }
}

fn close_type0_pis(mut ty: Term, count: usize) -> Term {
    for _ in 0..count {
        ty = Term::pi(Term::ty(Level::Zero), ty);
    }
    ty
}

fn close_type0_lams(mut body: Term, count: usize) -> Term {
    for _ in 0..count {
        body = Term::lam(Term::ty(Level::Zero), body);
    }
    body
}

/// Elaborate `instance C HeadType [where C1 T1 ; …] { f1 = e1 ; … }`.
///
/// Enforces the orphan check (`33 §5.3`) and overlap check (`39 §6.1`),
/// builds the Σ-chain value, and admits it through `declare_def` (kernel
/// re-check).  For constraint-carrying instances, uses
/// `declare_recursive_group` so that `sct_check` can reject non-terminating
/// resolution chains at admission time (`39 §6.4`).
fn elab_instance_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    rdecl: &RDecl,
    effect_rows: &HashMap<String, crate::effects::RowType>,
    class_name: &str,
    head_params: &[String],
    head_type: &RType,
    constraints: &[RInstanceConstraint],
    fields: &[(String, RExpr)],
) -> Result<ElabResult, ElabError> {
    let span = &rdecl.span;

    // ---- look up class ---------------------------------------------------
    let (class_module, class_type_id, class_kind) = {
        let ci = class_env
            .class(class_name)
            .ok_or_else(|| ElabError::UnresolvedCon {
                name: class_name.to_string(),
                span: span.clone(),
            })?;
        (ci.module_id, ci.projection.type_id, ci.kind.clone())
    };

    let head_name = head_type_name(head_type);
    let instance_key = (class_name.to_string(), head_name.clone());

    // ---- orphan check (`33 §5.3`) ----------------------------------------
    let in_class_module = class_module == class_env.current_module;
    let in_head_module = globals
        .get(&head_name)
        .and_then(|id| class_env.global_modules.get(id))
        .map(|m| *m == class_env.current_module)
        .unwrap_or(false);
    if !in_class_module && !in_head_module {
        return Err(ElabError::OrphanInstance {
            class: class_name.to_string(),
            head_type: head_name.clone(),
            span: span.clone(),
        });
    }

    // ---- overlap check (`39 §6.1`) — skip for property classes (Ω-PI) ---
    if class_kind == ClassKind::Structure && class_env.instances.contains_key(&instance_key) {
        let first_span = class_env.instances[&instance_key].declaration_span.clone();
        return Err(ElabError::OverlappingInstances {
            class: class_name.to_string(),
            head_type: head_name.clone(),
            first_span,
            second_span: span.clone(),
        });
    }

    // ---- elaborate head type --------------------------------------------
    let head_core = {
        let mut cx = ElabCtx::new(
            env,
            globals,
            num_values,
            numeric_env,
            format!("{class_name}.{head_name}"),
        );
        push_type0_params(&mut cx, head_params.len());
        let h = elab_type(&mut cx, head_type)?;
        cx.metas.zonk_term(&h)
    };

    // ---- build instance type --------------------------------------------
    // App(class_type, head) if parameterized, else class_type directly.
    let instance_ty = if class_env
        .class(class_name)
        .map(|ci| ci.projection.head_param.is_some())
        .unwrap_or(false)
    {
        Term::app(Term::const_(class_type_id, vec![]), head_core.clone())
    } else {
        Term::const_(class_type_id, vec![])
    };
    let constraint_core_types = {
        let mut cx = ElabCtx::new(
            env,
            globals,
            num_values,
            numeric_env,
            format!("{class_name}.{head_name}"),
        );
        push_type0_params(&mut cx, head_params.len());
        constraints
            .iter()
            .map(|constraint| {
                let class = class_env
                    .class(&constraint.class_name)
                    .ok_or_else(|| ElabError::UnresolvedCon {
                        name: constraint.class_name.clone(),
                        span: span.clone(),
                    })?;
                let head = elab_type(&mut cx, &constraint.head_type)?;
                Ok(if class.projection.head_param.is_some() {
                    Term::app(Term::const_(class.projection.type_id, vec![]), head)
                } else {
                    Term::const_(class.projection.type_id, vec![])
                })
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    let closed_instance_ty = close_type0_pis(
        wrap_premise_pis(instance_ty.clone(), &constraint_core_types),
        head_params.len(),
    );

    // ---- direct-self-reference detection (`39 §6.4`, scope-limited) -------
    //
    // This check detects DIRECT self-reference: a constraint whose (class, head)
    // is identical to the instance being declared. It does NOT detect mutual or
    // indirect cycles (e.g. `instance C (F a) where C (G a)` +
    // `instance C (G a) where C (F a)` — each admits as zero-edge, but resolution
    // loops at runtime).
    //
    // [tracked follow-on: Lc-mutual-cycle-termination]
    // Faithful reification (§6.4: one group node per sub-goal, one edge per
    // dischargeSubConstraints call, head-type metric for descent) would require
    // gathering ALL transitively-constrained instances into one
    // declare_recursive_group and threading the head-type metric through the edges.
    // This is deferred — the current slice covers direct-self-ref rejection only.
    // There is NO search-side backstop (no resolution-depth bound or occurs-check);
    // faithful reification is the sole net for mutual-cycle termination.
    let has_self_ref = constraints.iter().any(|constraint| {
        let chead = head_type_name(&constraint.head_type);
        (constraint.class_name.as_str(), chead.as_str()) == (class_name, head_name.as_str())
    });

    // ---- admit the instance ----------------------------------------------
    let (instance_id, field_effect_rows) = if has_self_ref {
        // Direct self-referential constraint: encode as a fixpoint-arrow so
        // sct_check sees the self-loop in App position and rejects (`39 §6.4`).
        //
        // Type  = Pi(T, T)   where T = instance_ty.
        // Body  = Lam(T, App(Const(own_id), Var(0)))
        //
        // collect_calls sees App(Const(own_id), Var(0)) → edge with M=[[?]]
        // (Var(0) = the parameter, not strictly decreasing) → SCT rejects.
        let t = closed_instance_ty.clone();
        let fixpoint_ty = Term::pi(t.clone(), t.clone());
        let ids = declare_recursive_group(env, vec![(vec![], fixpoint_ty)], |ids| {
            let own_id = ids[0];
            let body = Term::lam(
                t.clone(),
                Term::app(Term::const_(own_id, vec![]), Term::var(0)),
            );
            vec![body]
        })
        .map_err(|_| ElabError::NonTerminatingInstances { span: span.clone() })?;
        (ids[0], vec![])
    } else if !constraints.is_empty() {
        // Non-self-ref constrained instance: elaborate fields, then route through
        // declare_recursive_group so sct_check runs on the group (`39 §6.4`).
        // Body has no App(Const(own_id), ...) → edges.is_empty() → sct_check
        // accepts. Mutual/indirect cycles are not detected here (see above).
        let (ordered_vals, field_effect_rows): (Vec<Term>, Vec<crate::effects::RowType>) = {
            let mut cx = ElabCtx::new(
                env,
                globals,
                num_values,
                numeric_env,
                format!("{class_name}.{head_name}"),
            )
            .with_classes(&*class_env);
            push_type0_params(&mut cx, head_params.len());
            for (index, constraint_ty) in constraint_core_types.iter().enumerate() {
                cx.ctx.push(weaken(constraint_ty, index as i64));
            }
            compute_ordered_field_values(
                &mut cx,
                class_env,
                class_name,
                &head_name,
                &head_core,
                fields,
                effect_rows,
                span,
            )?
        };
        let pair_chain = close_type0_lams(
            wrap_premise_lams_from_full(
                build_pair_chain(&ordered_vals, class_env.record_nil_val_id),
                &constraint_core_types,
            ),
            head_params.len(),
        );
        let inst_ty = closed_instance_ty.clone();
        let ids = declare_recursive_group(env, vec![(vec![], inst_ty)], |_ids| vec![pair_chain])
            .map_err(|e| ElabError::KernelRejected {
                error: e,
                span: span.clone(),
            })?;
        (ids[0], field_effect_rows)
    } else {
        // No constraints: declare_def path (no recursion possible, SCT not needed).
        let (ordered_vals, field_effect_rows): (Vec<Term>, Vec<crate::effects::RowType>) = {
            let mut cx = ElabCtx::new(
                env,
                globals,
                num_values,
                numeric_env,
                format!("{class_name}.{head_name}"),
            )
            .with_classes(&*class_env);
            push_type0_params(&mut cx, head_params.len());
            compute_ordered_field_values(
                &mut cx,
                class_env,
                class_name,
                &head_name,
                &head_core,
                fields,
                effect_rows,
                span,
            )?
        };
        let pair_chain = close_type0_lams(
            build_pair_chain(&ordered_vals, class_env.record_nil_val_id),
            head_params.len(),
        );
        let id = declare_def(env, vec![], closed_instance_ty, pair_chain).map_err(|e| {
            ElabError::KernelRejected {
                error: e,
                span: span.clone(),
            }
        })?;
        (id, field_effect_rows)
    };

    // ---- register instance ----------------------------------------------
    let inst_name = format!("{}_instance_{}", class_name, head_name);
    globals.insert(inst_name, instance_id);
    class_env
        .global_modules
        .insert(instance_id, class_env.current_module);
    // For property classes, allow multiple registrations (Ω-PI means they're
    // all definitionally equal; the key is occupied but we don't error).
    class_env.instances.insert(
        instance_key,
        InstanceInfo {
            instance_id,
            class_name: class_name.to_string(),
            field_effect_rows,
            module_id: class_env.current_module,
            head_param_count: head_params.len(),
            head_type: Some(head_type.clone()),
            constraints: constraints
                .iter()
                .zip(&constraint_core_types)
                .map(|(constraint, core_type)| InstanceConstraintInfo {
                    class_name: constraint.class_name.clone(),
                    head_type: constraint.head_type.clone(),
                    core_type: core_type.clone(),
                })
                .collect(),
            defining_package: class_env
                .current_package
                .clone()
                .unwrap_or_else(|| "<local>".to_string()),
            declaration_span: span.clone(),
        },
    );
    if let Some(package) = &class_env.current_package {
        class_env.source_instance_packages.insert(package.clone());
    }

    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: instance_id,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

/// Elaborate `derive ClassName for DataName` (`33 §5.6`, `39 §6.6`).
///
/// Generates a candidate instance through the real `declare_def` re-check
/// (untrusted generation — the kernel re-verifies). For the current build:
/// the candidate for nullary/prop-only classes is `record_nil_val` directly;
/// the kernel rejects malformed candidates.
fn elab_derive(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    _num_values: &mut HashMap<GlobalId, NumericLitVal>,
    _numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    rdecl: &RDecl,
    class_name: &str,
    data_name: &str,
) -> Result<ElabResult, ElabError> {
    let span = &rdecl.span;

    let (class_type_id, has_param) = {
        let ci = class_env
            .class(class_name)
            .ok_or_else(|| ElabError::UnresolvedCon {
                name: class_name.to_string(),
                span: span.clone(),
            })?;
        (ci.projection.type_id, ci.projection.head_param.is_some())
    };

    let data_id = globals
        .get(data_name)
        .copied()
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: data_name.to_string(),
            span: span.clone(),
        })?;

    let data_term = if env.inductive(data_id).is_some() {
        Term::indformer(data_id, vec![])
    } else {
        Term::const_(data_id, vec![])
    };

    let instance_ty = if has_param {
        Term::app(Term::const_(class_type_id, vec![]), data_term)
    } else {
        Term::const_(class_type_id, vec![])
    };

    // Generate candidate: record_nil_val (minimal inhabitant of a prop-only
    // class Σ-chain). The kernel's declare_def re-checks: a malformed candidate
    // (wrong type) is rejected here.
    let candidate = Term::const_(class_env.record_nil_val_id, vec![]);
    let instance_id = declare_def(env, vec![], instance_ty, candidate).map_err(|e| {
        ElabError::KernelRejected {
            error: e,
            span: span.clone(),
        }
    })?;

    let head_name = data_name.to_string();
    let inst_name = format!("{}_instance_{}", class_name, head_name);
    globals.insert(inst_name, instance_id);
    class_env
        .global_modules
        .insert(instance_id, class_env.current_module);
    class_env.instances.insert(
        (class_name.to_string(), head_name),
        InstanceInfo {
            instance_id,
            class_name: class_name.to_string(),
            field_effect_rows: vec![],
            module_id: class_env.current_module,
            head_param_count: 0,
            head_type: None,
            constraints: vec![],
            defining_package: class_env
                .current_package
                .clone()
                .unwrap_or_else(|| "<local>".to_string()),
            declaration_span: span.clone(),
        },
    );
    if let Some(package) = &class_env.current_package {
        class_env.source_instance_packages.insert(package.clone());
    }

    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: instance_id,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

/// Elaborate a `foreign` declaration (`38 §2`, L7).
fn elaborate_foreign_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    rdecl: &RDecl,
    symbol: &str,
    library: &str,
    is_pure: bool,
    visits: &[String],
) -> Result<ElabResult, ElabError> {
    use crate::foreign::elaborate_foreign;

    let ty_core = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal("foreign decl must have a type annotation".into())
        })?;
        let ty_c = elab_type(&mut cx, ty)?;
        cx.metas.zonk_term(&ty_c)
    };

    let bytes_id = globals
        .get("Bytes")
        .copied()
        .ok_or_else(|| ElabError::Internal("Bytes not registered before foreign layer".into()))?;

    // Foreign ensures → runtime check obligations (AC4).
    let ensures_strs: Vec<String> = rdecl.ensures.iter().map(|e| format!("{:?}", e)).collect();

    let binding = elaborate_foreign(
        env,
        globals,
        bytes_id,
        &rdecl.name,
        ty_core,
        symbol,
        library,
        is_pure,
        visits,
        &ensures_strs,
        &rdecl.span,
    )?;

    let def_id = binding.postulate_id;

    let obligations: Vec<Obligation> = binding
        .runtime_checks
        .iter()
        .enumerate()
        .map(|(i, rc)| Obligation {
            id: i as u32,
            hole_id: rc.hole_id,
            goal_closed: Term::omega(Level::Zero),
            span: rdecl.span.clone(),
            kind: ObligationKind::FfiRuntimeCheck,
        })
        .collect();

    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id,
        obligations,
        foreign_binding: Some(binding),
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

fn elaborate_view_or_let(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    rdecl: &RDecl,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
) -> Result<ElabResult, ElabError> {
    // Check for implicit ensures from a return-type refinement (`22 §2.1`).
    let has_refine_return = rdecl
        .ty
        .as_ref()
        .and_then(|ty| innermost_refine_pred(ty))
        .is_some();
    if rdecl.requires.is_empty() && rdecl.ensures.is_empty() && !has_refine_return {
        // V0 path: no spec clauses and no return-type refinement
        return elaborate_v0(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            local_dicts,
        );
    }
    // V1 path: has requires/ensures or implicit return-type refinement obligation
    elaborate_view_with_spec(
        env,
        globals,
        num_values,
        numeric_env,
        class_env,
        rdecl,
        local_dicts,
    )
}

fn apply_space_args(head: Term, args: &[Term]) -> Term {
    args.iter().fold(head, |function, argument| {
        Term::app(function, argument.clone())
    })
}

fn build_space_state_type(cell_types: &[Term]) -> Term {
    let mut state = cell_types
        .last()
        .cloned()
        .expect("a parsed space has at least one cell");
    for cell_type in cell_types[..cell_types.len() - 1].iter().rev() {
        state = Term::sigma(cell_type.clone(), weaken(&state, 1));
    }
    state
}

fn build_space_state_value(cell_values: &[Term]) -> Term {
    let mut state = cell_values
        .last()
        .cloned()
        .expect("a parsed space has at least one cell");
    for cell_value in cell_values[..cell_values.len() - 1].iter().rev() {
        state = Term::pair(cell_value.clone(), state);
    }
    state
}

fn project_space_cell(mut state: Term, index: usize, cell_count: usize) -> Term {
    for _ in 0..index {
        state = Term::proj2(state);
    }
    if index + 1 == cell_count {
        state
    } else {
        Term::proj1(state)
    }
}

fn update_space_cell(state: Term, index: usize, cell_count: usize, value: Term) -> Term {
    if cell_count == 1 {
        return value;
    }
    if index == 0 {
        return Term::pair(value, Term::proj2(state));
    }
    Term::pair(
        Term::proj1(state.clone()),
        update_space_cell(Term::proj2(state), index - 1, cell_count - 1, value),
    )
}

fn elab_space_contract_prop(cx: &mut ElabCtx<'_>, expr: &RExpr) -> Result<Term, ElabError> {
    let (core, ty) = infer(cx, expr)?;
    let core = cx.metas.zonk_term(&core);
    let ty = cx.metas.zonk_term(&ty);
    if !matches!(ty, Term::Omega(_)) {
        return Err(ElabError::TypeMismatch {
            span: expr.span().clone(),
            reason: "space-operation contract clause must have type Omega".to_string(),
        });
    }
    Ok(core)
}

fn space_transformer_payload(
    cx: &ElabCtx<'_>,
    run: Term,
    ret_id: GlobalId,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let normalized = whnf(cx.env, &cx.ctx, &run);
    let mut head = &normalized;
    let mut args = Vec::new();
    while let Term::App(function, argument) = head {
        args.push(argument.as_ref());
        head = function.as_ref();
    }
    let is_ret = matches!(head, Term::Constructor { id, .. } if *id == ret_id);
    match (is_ret, args.first().copied()) {
        (true, Some(Term::Pair(result, post_state))) => {
            Ok((result.as_ref().clone(), post_state.as_ref().clone()))
        }
        _ => Err(ElabError::Internal(format!(
            "space contract transformer did not reduce to Ret (result, post-state) at {:?}: {:?}",
            span, normalized
        ))),
    }
}

/// Elaborate a `space` surface block onto the already-built `State` effect.
///
/// The emitted kernel objects are ordinary transparent definitions:
/// `S : Type := T₁ × … × Tₘ`, a private initial-state definition, and one
/// qualified operation `S.op : Π params. ITree (State S ⊕ Empty) ... R`.
pub(crate) fn elaborate_space_decl(
    elab: &mut crate::ElabEnv,
    space: &RSpaceDecl,
) -> Result<Vec<ElabResult>, ElabError> {
    let mut cell_types = Vec::with_capacity(space.cells.len());
    let mut cell_values = Vec::with_capacity(space.cells.len());
    for cell in &space.cells {
        let mut cx = ElabCtx::new(
            &mut elab.env,
            &elab.globals,
            &mut elab.num_values,
            &elab.numeric_env,
            format!("{}.initial", space.name),
        );
        let cell_type = elab_type(&mut cx, &cell.ty)?;
        let cell_value = check(&mut cx, &cell.init, &cell_type, &cell.span)?;
        cell_types.push(cx.metas.zonk_term(&cell_type));
        cell_values.push(cx.metas.zonk_term(&cell_value));
    }

    let state_body = build_space_state_type(&cell_types);
    let state_sort = kernel_infer(&elab.env, &Context::new(), &state_body).map_err(|error| {
        ElabError::KernelRejected {
            error,
            span: space.span.clone(),
        }
    })?;
    let state_id = declare_def(&mut elab.env, vec![], state_sort, state_body).map_err(|error| {
        ElabError::KernelRejected {
            error,
            span: space.span.clone(),
        }
    })?;
    elab.globals.insert(space.name.clone(), state_id);
    let mut results = vec![ElabResult {
        name: space.name.clone(),
        def_id: state_id,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    }];

    let state_type = Term::const_(state_id, vec![]);
    let initial_id = declare_def(
        &mut elab.env,
        vec![],
        state_type.clone(),
        build_space_state_value(&cell_values),
    )
    .map_err(|error| ElabError::KernelRejected {
        error,
        span: space.span.clone(),
    })?;
    elab.space_metadata
        .initial_states
        .insert(space.name.clone(), initial_id);

    let prelude = elab.prelude_env.clone();
    let empty_id = *elab.globals.get("Empty").ok_or_else(|| {
        ElabError::Internal("space desugaring requires the prelude Empty type".to_string())
    })?;
    let empty_type = Term::indformer(empty_id, vec![]);
    let unit_type = Term::indformer(prelude.unit_id, vec![]);
    let resp_empty = Term::lam(empty_type.clone(), unit_type.clone());

    for operation in &space.operations {
        let visits = operation
            .visits
            .as_ref()
            .ok_or_else(|| ElabError::TypeMismatch {
                span: operation.span.clone(),
                reason: format!(
                    "false purity or effect escape in `{}.{}`: cell access requires visits [{}]",
                    space.name, operation.name, space.name
                ),
            })?;
        let row_vars = crate::effects::row_var_map(&[]);
        let declared_row =
            crate::effects::surface_row_to_row_type(visits, &row_vars).map_err(|reason| {
                ElabError::TypeMismatch {
                    span: visits.span.clone(),
                    reason,
                }
            })?;
        if !declared_row.concrete_effects().contains(&space.name) {
            return Err(ElabError::TypeMismatch {
                span: visits.span.clone(),
                reason: format!(
                    "space operation `{}.{}` must include visits [{}]",
                    space.name, operation.name, space.name
                ),
            });
        }

        let qualified_name = format!("{}.{}", space.name, operation.name);
        let inferred_row = infer_expr_row_type(&operation.body, &elab.effect_rows, None)
            .join(crate::effects::RowType::singleton(space.name.clone()));
        let effect_decl = crate::effects::EffectDecl::new(&qualified_name)
            .with_declared_row_type(declared_row.clone());
        crate::effects::check_decl_poly(
            &effect_decl,
            &inferred_row,
            &crate::effects::EffectRow::empty(),
        )
        .map_err(|err| ElabError::TypeMismatch {
            span: operation.span.clone(),
            reason: format!("false purity or effect escape in `{qualified_name}`: {err}"),
        })?;
        let mut cx = ElabCtx::new(
            &mut elab.env,
            &elab.globals,
            &mut elab.num_values,
            &elab.numeric_env,
            qualified_name.clone(),
        )
        .with_classes(&elab.class_env);
        let mut parameter_domains = Vec::with_capacity(operation.params.len());
        for (_, parameter_type) in &operation.params {
            let domain = elab_type(&mut cx, parameter_type)?;
            let domain = cx.metas.zonk_term(&domain);
            cx.ctx.push(domain.clone());
            parameter_domains.push(domain);
        }
        let return_type = elab_type(&mut cx, &operation.ret_ty)?;
        let return_type = cx.metas.zonk_term(&return_type);

        let op_type = apply_space_args(
            Term::indformer(prelude.coproduct_id, vec![]),
            &[
                Term::app(
                    Term::indformer(prelude.state_op_id, vec![]),
                    state_type.clone(),
                ),
                empty_type.clone(),
            ],
        );
        let response_type = apply_space_args(
            Term::const_(prelude.resp_coproduct_id, vec![]),
            &[
                Term::app(
                    Term::indformer(prelude.state_op_id, vec![]),
                    state_type.clone(),
                ),
                empty_type.clone(),
                Term::app(
                    Term::const_(prelude.resp_state_id, vec![]),
                    state_type.clone(),
                ),
                resp_empty.clone(),
            ],
        );
        let computation_type = apply_space_args(
            Term::indformer(prelude.itree_id, vec![]),
            &[op_type.clone(), response_type.clone(), return_type.clone()],
        );
        let get_call = apply_space_args(
            Term::const_(prelude.get_fn_id, vec![]),
            &[
                state_type.clone(),
                empty_type.clone(),
                resp_empty.clone(),
                Term::constructor(prelude.mkunit_id, vec![]),
            ],
        );

        cx.ctx.push(state_type.clone());
        cx.install_space_state(&cell_types);
        cx.hidden_positions.push(cx.ctx.len() - 1);
        let continuation_body = match &operation.body {
            RExpr::RBecomes(index, _, value, span) => {
                kernel_check(
                    cx.env,
                    &cx.ctx,
                    &Term::constructor(prelude.mkunit_id, vec![]),
                    &weaken(&return_type, 1),
                )
                .map_err(|_| ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "`becomes` produces Unit; the space operation must return Unit"
                        .to_string(),
                })?;
                let target_type = cell_types.get(*index).ok_or_else(|| {
                    ElabError::Internal(format!("space cell index {index} out of range"))
                })?;
                let value = check(&mut cx, value, target_type, span)?;
                let updated = update_space_cell(Term::var(0), *index, cell_types.len(), value);
                apply_space_args(
                    Term::const_(prelude.put_fn_id, vec![]),
                    &[
                        state_type.clone(),
                        empty_type.clone(),
                        resp_empty.clone(),
                        updated,
                    ],
                )
            }
            body => {
                let value = check(&mut cx, body, &weaken(&return_type, 1), &operation.span)?;
                apply_space_args(
                    Term::constructor(prelude.ret_id, vec![]),
                    &[
                        op_type.clone(),
                        response_type.clone(),
                        weaken(&return_type, 1),
                        value,
                    ],
                )
            }
        };
        cx.space_state = None;
        cx.hidden_positions.pop();
        cx.ctx.pop();
        let continuation = Term::lam(state_type.clone(), continuation_body);
        let body = apply_space_args(
            Term::const_(prelude.bind_id, vec![]),
            &[
                op_type.clone(),
                response_type.clone(),
                state_type.clone(),
                return_type.clone(),
                get_call,
                continuation,
            ],
        );

        let mut contract_obligations = Vec::new();
        if !operation.requires.is_empty() || !operation.ensures.is_empty() {
            let contract_base_depth = cx.ctx.len();
            cx.ctx.push(state_type.clone());
            let pre_state_position = cx.ctx.len() - 1;
            cx.hidden_positions.push(pre_state_position);
            cx.install_space_state(&cell_types);
            cx.space_pre_state = Some((pre_state_position, cell_types.clone()));

            let mut requirement_count = 0usize;
            for requirement in &operation.requires {
                let requirement = elab_space_contract_prop(&mut cx, requirement)?;
                cx.ctx.push(requirement);
                cx.hidden_positions.push(cx.ctx.len() - 1);
                requirement_count += 1;
            }

            let pre_state_index = cx.ctx.len() - 1 - pre_state_position;
            let run = apply_space_args(
                Term::const_(prelude.run_state_id, vec![]),
                &[
                    state_type.clone(),
                    empty_type.clone(),
                    resp_empty.clone(),
                    weaken(&return_type, (1 + requirement_count) as i64),
                    Term::var(pre_state_index),
                    weaken(&body, (1 + requirement_count) as i64),
                ],
            );
            let (result_value, post_state_value) =
                space_transformer_payload(&cx, run, prelude.ret_id, &operation.span)?;

            for ensures in &operation.ensures {
                cx.ctx
                    .push(weaken(&return_type, (1 + requirement_count) as i64));
                let result_position = cx.ctx.len() - 1;
                cx.ctx.push(state_type.clone());
                let post_state_position = cx.ctx.len() - 1;
                cx.hidden_positions.push(post_state_position);
                cx.space_state = Some((post_state_position, cell_types.clone()));

                let predicate = elab_space_contract_prop(&mut cx, ensures)?;
                let goal = subst0(
                    &subst0(&predicate, &weaken(&post_state_value, 1)),
                    &result_value,
                );

                cx.space_state = Some((pre_state_position, cell_types.clone()));
                cx.hidden_positions.pop();
                cx.ctx.pop();
                debug_assert_eq!(cx.ctx.len() - 1, result_position);
                cx.ctx.pop();

                let closed = close_goal(&cx.ctx, goal);
                let hole_id =
                    declare_postulate(cx.env, qualified_name.clone(), vec![], closed.clone())
                        .map_err(|error| ElabError::KernelRejected {
                            error,
                            span: ensures.span().clone(),
                        })?;
                contract_obligations.push(Obligation {
                    id: contract_obligations.len() as u32,
                    hole_id,
                    goal_closed: closed,
                    span: ensures.span().clone(),
                    kind: ObligationKind::Ensures,
                });
            }

            cx.space_state = None;
            cx.space_pre_state = None;
            for _ in 0..requirement_count {
                cx.hidden_positions.pop();
                cx.ctx.pop();
            }
            cx.hidden_positions.pop();
            cx.ctx.pop();
            debug_assert_eq!(cx.ctx.len(), contract_base_depth);
        }

        let mut body = body;
        let mut operation_type = computation_type;
        for domain in parameter_domains.iter().rev() {
            body = Term::lam(domain.clone(), body);
            operation_type = Term::pi(domain.clone(), operation_type);
        }
        let operation_id = declare_def(cx.env, vec![], operation_type, body).map_err(|error| {
            ElabError::KernelRejected {
                error,
                span: operation.span.clone(),
            }
        })?;
        drop(cx);
        elab.globals.insert(qualified_name.clone(), operation_id);
        elab.effect_rows
            .insert(qualified_name.clone(), declared_row.clone());
        results.push(ElabResult {
            name: qualified_name,
            def_id: operation_id,
            obligations: contract_obligations,
            foreign_binding: None,
            temporal_obligations: vec![],
            effect_row_type: Some(declared_row),
        });
    }
    Ok(results)
}

/// Extract the predicate from the innermost refinement in a resolved type.
///
/// `{ k : A | φ }` at the end of a Pi-chain → `Some(φ)`. Used by V2 to
/// emit a refinement-introduction obligation for the return type (`22 §2.1`).
pub(crate) fn innermost_refine_pred(ty: &RType) -> Option<&RExpr> {
    match ty {
        RType::RPi(_, _, cod, _) | RType::RArr(_, cod, _) | RType::REffectArr(_, _, cod, _) => {
            innermost_refine_pred(cod)
        }
        RType::RRefine(_, _, phi, _) => Some(phi),
        _ => None,
    }
}

fn elaborate_v0(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    rdecl: &RDecl,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
) -> Result<ElabResult, ElabError> {
    // A self-recursive view/let (body mentions its own name) must be admitted
    // through the SCT gate with the name pre-bound, so the body's self-call
    // resolves — `declare_def` allocates the id only after the body is built,
    // which is too late for a self-reference. Route to the recursive path.
    if rexpr_mentions_name(&rdecl.body, &rdecl.name) {
        return elaborate_recursive_view(env, globals, num_values, numeric_env, class_env, rdecl);
    }
    let (ty_core, body_core, body_obligations) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
            .with_classes(class_env)
            .with_local_dicts(local_dicts);
        let (body_raw, ty_raw) = if let Some(ty) = &rdecl.ty {
            let ty_c = elab_type(&mut cx, ty)?;
            let body_c = check(&mut cx, &rdecl.body, &ty_c, &rdecl.span)?;
            (body_c, ty_c)
        } else {
            let (body_c, ty_c) = infer(&mut cx, &rdecl.body)?;
            (body_c, ty_c)
        };
        let obligations = std::mem::take(&mut cx.obligations);
        (
            cx.metas.zonk_term(&ty_raw),
            cx.metas.zonk_term(&body_raw),
            obligations,
        )
    };
    if rdecl.ty.is_none()
        && matches!(
            rdecl.kind,
            RDeclKind::View {
                keyword: DefKeyword::Fn | DefKeyword::Const,
                ..
            }
        )
    {
        ensure_not_omega_type(env, &Context::new(), &ty_core, &rdecl.span)?;
    }
    let id =
        declare_def(env, vec![], ty_core, body_core).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        })?;
    globals.insert(rdecl.name.clone(), id);
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: id,
        obligations: body_obligations,
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

/// Elaborate a self-recursive `view`/`let` through the SCT gate (Approach A).
///
/// The kernel's `declare_def` already pre-admits an opaque, kernel-checks the
/// body, runs `sct_check`, and upgrades to transparent — but it allocates the
/// id *after* the body is built. A recursive def's body references its own id
/// during elaboration (the resolver emits `RCon(name)` on a scope miss,
/// `c3a3f1d`; the elaborator resolves it against `globals`), so the id must be
/// visible *before* the body is elaborated. This function splits the sequence
/// the kernel performs atomically in `declare_def`:
///
///   1. Elaborate the declared type → `ty_core`.
///   2. Pre-admit the name as `Opaque` with that type and insert it into
///      `globals`, so the body's self-reference resolves to this id.
///   3. Elaborate the body checked against `ty_core` (self-calls see the
///      opaque's type; the kernel `check` sees the opaque too).
///   4. Kernel-check the closed body against `ty_core`, then `sct_check` the
///      singleton recursive group.
///   5. On SCT acceptance, `upgrade_to_transparent` (δ-unfoldable, leaves
///      `trusted_base`); on rejection, roll back the pre-admission — the opaque
///      plus any literal postulates body elaboration added after it — and
///      unbind the name from `globals`.
///
/// **Contained vs deferred (K2c).** This is a contained elaborator-side wiring
/// of an *existing* kernel capability (`sct_check` + `upgrade_to_transparent`);
/// the soundness-critical part — verifying structural descent — already lives
/// in the kernel. The deferred sibling is **K2c general recursive δ** (`11
/// §4`): arbitrary recursive δ-unfolding in conversion. Here the recursive call
/// is to an *opaque* (δ blocks during checking); only after SCT acceptance does
/// it become transparent, and termination is by structural descent on an
/// inductive sub-term (SCT's `↓`) — not general δ. A recursive fn carrying
/// `requires` clauses (so the full type ≠ the carrier Pi-chain) is a tracked
/// follow-on; L3a's recursive views (`map`/`filter`/`fold`/`zip`/`unfoldUpTo`/
/// `sort`/`insert`) carry none.
fn elaborate_recursive_view(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    // 1. Elaborate the declared type (recursive views are annotated).
    let ty_core = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal("recursive declaration requires a type annotation".into())
        })?;
        let ty_c = elab_type(&mut cx, ty)?;
        cx.metas.zonk_term(&ty_c)
    };

    // 2. Pre-admit as Opaque so the body can self-reference.
    let id = env.fresh_id();
    env.add_decl(Decl::Opaque {
        id,
        name: rdecl.name.clone(),
        level_params: vec![],
        ty: ty_core.clone(),
    });
    globals.insert(rdecl.name.clone(), id);

    // 3. Elaborate the body (self-ref resolves to `id` via globals).
    let (body_core, body_obligations) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
            .with_classes(class_env);
        let body_c = check(&mut cx, &rdecl.body, &ty_core, &rdecl.span)?;
        let obligations = std::mem::take(&mut cx.obligations);
        (cx.metas.zonk_term(&body_c), obligations)
    };

    // 4. Kernel type-check + SCT gate (singleton recursive group).
    let admit_result = kernel_check(env, &Context::new(), &body_core, &ty_core)
        .and_then(|_| sct_check(env, &[(id, body_core.clone())]));

    match admit_result {
        Ok(()) => {
            // 5. SCT accepted → upgrade opaque to transparent (δ-unfoldable).
            env.upgrade_to_transparent(id, body_core);
            Ok(ElabResult {
                name: rdecl.name.clone(),
                def_id: id,
                obligations: body_obligations,
                foreign_binding: None,
                temporal_obligations: vec![],
                effect_row_type: None,
            })
        }
        Err(e) => {
            // Roll back: remove the pre-admitted opaque and any literal
            // postulates body elaboration added after it (remove_last until we
            // hit our opaque), then unbind the name.
            while let Some(d) = env.remove_last() {
                if d.id() == id {
                    break;
                }
            }
            globals.remove(&rdecl.name);
            Err(ElabError::KernelRejected {
                error: e,
                span: rdecl.span.clone(),
            })
        }
    }
}

/// Elaborate a genuinely mutually-recursive group of `view`/`let` decls
/// (VAL2 #3) — `members.len() >= 2`, already confirmed to form one strongly-
/// connected call-graph component (`modules.rs`'s SCC pre-pass). Generalizes
/// `elaborate_recursive_view`'s singleton pattern (pre-admit as `Opaque`,
/// elaborate the body against that name-in-scope, kernel-check, `sct_check`,
/// upgrade-or-rollback) to the whole group at once, so the WHOLE GROUP is one
/// `sct_check` call — no member escapes the termination check
/// (`[[sct-unapplied-self-reference-over-accepts]]`).
///
/// Each member requires an explicit type annotation (mirrors the existing
/// singleton recursive-const rule — a mutual group's forward references need
/// every member's *type* resolvable before any body is elaborated).
pub fn elaborate_mutual_group(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    members: &[RDecl],
) -> Result<Vec<ElabResult>, ElabError> {
    // 1. Elaborate every member's declared type FIRST (the signature
    // pre-pass) — none of these need a sibling's id, only their own params.
    let mut ty_cores: Vec<Term> = Vec::with_capacity(members.len());
    for rdecl in members {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal(format!(
                "mutually-recursive '{}' requires a type annotation",
                rdecl.name
            ))
        })?;
        let ty_c = elab_type(&mut cx, ty)?;
        ty_cores.push(cx.metas.zonk_term(&ty_c));
    }

    // 2. Pre-admit ALL members as Opaque, binding every name in `globals`
    // BEFORE any body is elaborated — this is what lets a forward/mutual
    // reference to any sibling resolve, exactly as the singleton case
    // pre-admits its own single name.
    let mut ids: Vec<GlobalId> = Vec::with_capacity(members.len());
    for (rdecl, ty_core) in members.iter().zip(&ty_cores) {
        let id = env.fresh_id();
        env.add_decl(Decl::Opaque {
            id,
            name: rdecl.name.clone(),
            level_params: vec![],
            ty: ty_core.clone(),
        });
        globals.insert(rdecl.name.clone(), id);
        ids.push(id);
    }

    // Proof declarations share the same signature-first admission path as
    // computations, but retain their existing Ω and attached-subject guards
    // before any recursive body is checked.
    let proof_validation = (|| -> Result<(), ElabError> {
        for ((rdecl, ty_core), id) in members.iter().zip(&ty_cores).zip(&ids) {
            match &rdecl.kind {
                RDeclKind::View {
                    keyword: DefKeyword::Fn | DefKeyword::Const,
                    ..
                } => ensure_not_omega_type(env, &Context::new(), ty_core, &rdecl.span)?,
                RDeclKind::Theorem => {
                    ensure_omega_type(env, &Context::new(), ty_core, &rdecl.span)?
                }
                RDeclKind::AttachedProof { subject, .. } => {
                    ensure_omega_type(env, &Context::new(), ty_core, &rdecl.span)?;
                    validate_attached_subject_occurs_applied(
                        env,
                        globals,
                        subject,
                        ty_core,
                        &rdecl.span,
                    )?;
                    debug_assert_eq!(globals.get(&rdecl.name), Some(id));
                }
                _ => {}
            }
        }
        Ok(())
    })();
    if let Err(e) = proof_validation {
        for id in ids.iter().rev() {
            while let Some(decl) = env.remove_last() {
                if decl.id() == *id {
                    break;
                }
            }
        }
        for rdecl in members {
            globals.remove(&rdecl.name);
        }
        return Err(e);
    }

    // 3. Elaborate each body checked against its own type (every sibling
    // name, including self, already resolves via `globals` from step 2).
    let recursive_group = ids.iter().copied().collect::<HashSet<_>>();
    let mut bodies: Vec<Term> = Vec::with_capacity(members.len());
    let mut all_obligations: Vec<Vec<Obligation>> = Vec::with_capacity(members.len());
    let elab_err = (|| -> Result<(), ElabError> {
        for (rdecl, ty_core) in members.iter().zip(&ty_cores) {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
                .with_classes(class_env)
                .with_recursive_group(&recursive_group);
            let body_c = check(&mut cx, &rdecl.body, ty_core, &rdecl.span)?;
            let obligations = std::mem::take(&mut cx.obligations);
            bodies.push(cx.metas.zonk_term(&body_c));
            all_obligations.push(obligations);
        }
        Ok(())
    })();

    // Roll back ALL pre-admitted members on ANY elaboration failure (not
    // just the SCT gate below) — a partially-elaborated group must leave no
    // trace, same discipline as the singleton path's rollback.
    if let Err(e) = elab_err {
        for id in ids.iter().rev() {
            while let Some(d) = env.remove_last() {
                if d.id() == *id {
                    break;
                }
            }
        }
        for rdecl in members {
            globals.remove(&rdecl.name);
        }
        return Err(e);
    }

    // 4. Kernel-check every body against its own declared type, THEN run
    // `sct_check` on the WHOLE GROUP as ONE termination problem — the whole
    // point of a mutual group is that no member's descent is checked in
    // isolation (a member could look non-terminating alone but be fine via
    // the group's cross-cycle measure, or vice versa look terminating alone
    // while the CYCLE diverges).
    let group_bodies: Vec<(GlobalId, Term)> =
        ids.iter().cloned().zip(bodies.iter().cloned()).collect();
    let admit_result: Result<(), ken_kernel::KernelError> = (|| {
        for (body, ty_core) in bodies.iter().zip(&ty_cores) {
            kernel_check(env, &Context::new(), body, ty_core)?;
        }
        sct_check(env, &group_bodies)
    })();

    match admit_result {
        Ok(()) => {
            for (id, body) in ids.iter().zip(bodies) {
                env.upgrade_to_transparent(*id, body);
            }
            Ok(members
                .iter()
                .zip(ids)
                .zip(all_obligations)
                .map(|((rdecl, id), obligations)| ElabResult {
                    name: rdecl.name.clone(),
                    def_id: id,
                    obligations,
                    foreign_binding: None,
                    temporal_obligations: vec![],
                    effect_row_type: None,
                })
                .collect())
        }
        Err(e) => {
            // Roll back every pre-admitted member (reverse order) — a
            // rejected group leaves zero trace, exactly like the singleton
            // rollback, just for every member instead of one.
            for id in ids.iter().rev() {
                while let Some(d) = env.remove_last() {
                    if d.id() == *id {
                        break;
                    }
                }
            }
            for rdecl in members {
                globals.remove(&rdecl.name);
            }
            Err(ElabError::KernelRejected {
                error: e,
                span: members[0].span.clone(),
            })
        }
    }
}

/// Does `expr` mention the global name `name` (as an `RCon`)? Used to detect
/// whether a view/let definition is self-recursive — the body references its
/// own name, which the resolver emits as `RCon(name)` on a scope miss. Pattern
/// positions are not scanned: a def name is a view/function, never a
/// constructor, so it cannot appear in a pattern.
pub(crate) fn rexpr_mentions_name(expr: &RExpr, name: &str) -> bool {
    match expr {
        RExpr::RCon(n, _) => n == name,
        RExpr::RVar(_, _, _)
        | RExpr::RCell(_, _, _)
        | RExpr::RRecursiveResult { .. }
        | RExpr::RUniv(_, _)
        | RExpr::RNumLit(_, _)
        | RExpr::RStr(_, _)
        | RExpr::RCharLit(_, _)
        | RExpr::RByteStr(_, _) => false,
        RExpr::RApp(f, a, _) => rexpr_mentions_name(f, name) || rexpr_mentions_name(a, name),
        RExpr::RLam(_, b, _) => rexpr_mentions_name(b, name),
        RExpr::RLet(_, _, rhs, body, _) => {
            rexpr_mentions_name(rhs, name) || rexpr_mentions_name(body, name)
        }
        RExpr::RAsc(e, _, _) => rexpr_mentions_name(e, name),
        RExpr::ROld(e, _) => rexpr_mentions_name(e, name),
        RExpr::RBecomes(_, _, e, _) => rexpr_mentions_name(e, name),
        RExpr::RBinOp(_, l, r, _) => rexpr_mentions_name(l, name) || rexpr_mentions_name(r, name),
        RExpr::RMatch { scrut, arms, .. } => {
            rexpr_mentions_name(scrut, name)
                || arms.iter().any(|a| rexpr_mentions_name(&a.body, name))
        }
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            rexpr_mentions_name(condition, name)
                || rexpr_mentions_name(then_branch, name)
                || rexpr_mentions_name(else_branch, name)
        }
        RExpr::RPair(components, _) => components
            .iter()
            .any(|component| rexpr_mentions_name(component, name)),
        RExpr::RRecord { base, fields, .. } => {
            base.as_deref()
                .is_some_and(|base| rexpr_mentions_name(base, name))
                || fields
                    .iter()
                    .any(|(_, value, _)| rexpr_mentions_name(value, name))
        }
        RExpr::RPosProj(e, _, _) => rexpr_mentions_name(e, name),
        RExpr::RProj(e, _, _) => rexpr_mentions_name(e, name),
        // The domain is a `type`, not an `RExpr` — a mutual-recursion call
        // graph only cares about VALUE-level (expr) references, so only
        // the codomain (an `RExpr`) is scanned.
        RExpr::RPi(_, _, b, _) => rexpr_mentions_name(b, name),
        RExpr::RArrow(a, b, _) => rexpr_mentions_name(a, name) || rexpr_mentions_name(b, name),
        RExpr::RAttachedProofRef { .. } => false,
        RExpr::RTrunc(e, _) => rexpr_mentions_name(e, name),
    }
}

/// Type-side counterpart to [`rexpr_mentions_name`].  Scope dependency order
/// must account for a declaration used only in another declaration's theorem
/// or result type, not merely direct calls in bodies.
pub(crate) fn rtype_mentions_name(ty: &RType, name: &str) -> bool {
    match ty {
        RType::RCon(n, _) => n == name,
        RType::RPi(_, domain, codomain, _)
        | RType::RSigma(_, domain, codomain, _)
        | RType::RArr(domain, codomain, _)
        | RType::REffectArr(domain, _, codomain, _)
        | RType::RApp(domain, codomain, _) => {
            rtype_mentions_name(domain, name) || rtype_mentions_name(codomain, name)
        }
        RType::RRefine(_, carrier, predicate, _) => {
            rtype_mentions_name(carrier, name) || rexpr_mentions_name(predicate, name)
        }
        RType::RUniv(_, _) | RType::RVarTy(_, _, _) => false,
    }
}

/// Elaborate a `view` with `requires`/`ensures` clauses (`21 §6.3`).
fn elaborate_view_with_spec(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    rdecl: &RDecl,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
) -> Result<ElabResult, ElabError> {
    let omega = Term::omega(Level::Zero);

    // Phase 1: elaborate the declared type (carrier) and body.
    //
    // A self-recursive spec'd view (e.g. `sort`) must have its name pre-admitted
    // as Opaque before the body is elaborated, so the body's self-call resolves
    // (Approach A; see `elaborate_recursive_view`). The non-recursive path keeps
    // type+body in one context so their level metas unify.
    let is_recursive = rexpr_mentions_name(&rdecl.body, &rdecl.name);

    let (body_raw, carrier_ty_raw, pre_admit_id): (Term, Term, Option<GlobalId>) = if is_recursive {
        // Recursive: elab the carrier type, pre-admit, then elab the body.
        let carrier_ty = {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
                .with_classes(class_env)
                .with_local_dicts(local_dicts);
            let ty = rdecl.ty.as_ref().ok_or_else(|| {
                ElabError::Internal(
                    "recursive const with spec clauses requires a type annotation".into(),
                )
            })?;
            let ty_c = elab_type(&mut cx, ty)?;
            cx.metas.zonk_term(&ty_c)
        };
        let id = env.fresh_id();
        env.add_decl(Decl::Opaque {
            id,
            name: rdecl.name.clone(),
            level_params: vec![],
            ty: carrier_ty.clone(),
        });
        globals.insert(rdecl.name.clone(), id);
        let body = {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
                .with_classes(class_env)
                .with_local_dicts(local_dicts);
            let body_c = check(&mut cx, &rdecl.body, &carrier_ty, &rdecl.span)?;
            cx.metas.zonk_term(&body_c)
        };
        (body, carrier_ty, Some(id))
    } else {
        // Non-recursive: original one-context flow.
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
            .with_classes(class_env)
            .with_local_dicts(local_dicts);
        if let Some(ty) = &rdecl.ty {
            let ty_c = elab_type(&mut cx, ty)?;
            let body_c = check(&mut cx, &rdecl.body, &ty_c, &rdecl.span)?;
            (cx.metas.zonk_term(&body_c), cx.metas.zonk_term(&ty_c), None)
        } else {
            let (body_c, ty_c) = infer(&mut cx, &rdecl.body)?;
            (cx.metas.zonk_term(&body_c), cx.metas.zonk_term(&ty_c), None)
        }
    };

    // Build the param context from the Pi-chain of the carrier type.
    let param_types = unwrap_pi_chain(&carrier_ty_raw);
    let carrier_b = innermost_codomain(&carrier_ty_raw);
    let mut param_ctx = Context::new();
    for pt in &param_types {
        param_ctx.push(pt.clone());
    }

    // Phase 2: process `requires` clauses.
    let mut req_cores: Vec<Term> = Vec::new();
    for req in &rdecl.requires {
        let phi_core = elab_in_ctx_at_omega(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            local_dicts,
            &param_ctx,
            req,
            &omega,
            &rdecl.span,
            &rdecl.name,
        )?;
        req_cores.push(phi_core);
    }

    // Phase 3: process `ensures` clauses.
    // ensures context = param_ctx + [result : carrier_b]
    let mut ens_ctx = param_ctx.clone();
    ens_ctx.push(carrier_b.clone());

    // body_inner = the inner body term (past all param lambdas)
    let body_inner = unwrap_lam(&body_raw, param_types.len());

    // Collect ensures: explicit clauses + implicit from return-type refinement (`22 §2.1`).
    // A `{ x : A | φ }` return type is a refinement introduction at the body site;
    // its predicate φ is an implicit ensures with the same ψ[body/result] structure.
    let mut all_ensures: Vec<&RExpr> = rdecl.ensures.iter().collect();
    if let Some(phi) = rdecl.ty.as_ref().and_then(|ty| innermost_refine_pred(ty)) {
        all_ensures.push(phi);
    }

    let mut ens_obligations: Vec<Obligation> = Vec::new();
    let mut obl_counter = 0u32;
    for ens in &all_ensures {
        let psi_core = elab_in_ctx_at_omega(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            local_dicts,
            &ens_ctx,
            ens,
            &omega,
            &rdecl.span,
            &rdecl.name,
        )?;
        // goal = ψ[body_inner/result]: result = Var(0) in ens_ctx, substitute body
        let goal_open = subst0(&psi_core, &body_inner);
        let closed = close_goal(&param_ctx, goal_open);
        let hole_id =
            declare_postulate(env, rdecl.name.clone(), vec![], closed.clone()).map_err(|e| {
                ElabError::KernelRejected {
                    error: e,
                    span: rdecl.span.clone(),
                }
            })?;
        ens_obligations.push(Obligation {
            id: obl_counter,
            hole_id,
            goal_closed: closed,
            span: rdecl.span.clone(),
            kind: ObligationKind::Ensures,
        });
        obl_counter += 1;
    }

    // Phase 4: build the full type and body.
    // full_ty = Pi(params..., Pi(req..., carrier_b))
    let mut full_ty = carrier_b.clone();
    for req in req_cores.iter().rev() {
        full_ty = Term::pi(req.clone(), weaken(&full_ty, 1));
    }
    for pt in param_types.iter().rev() {
        full_ty = Term::pi(pt.clone(), full_ty);
    }
    // full_body = Lam(params..., Lam(req..., body_inner))
    // body_inner has free variables indexed relative to param_ctx (depth n_params).
    // The req lambdas are inserted BETWEEN the param lambdas and the body, so each
    // param variable in body_inner shifts up by req_cores.len() to skip the req binders.
    let mut full_body = weaken(&body_inner, req_cores.len() as i64);
    for req in req_cores.iter().rev() {
        full_body = Term::lam(req.clone(), full_body);
    }
    for pt in param_types.iter().rev() {
        full_body = Term::lam(pt.clone(), full_body);
    }

    let id = if let Some(pre_id) = pre_admit_id {
        // Recursive: the opaque was pre-admitted with the carrier Pi-chain. For
        // L3a's recursive views (no `requires`), `full_ty` == the carrier
        // Pi-chain, so the opaque's type is already `full_ty`. Kernel-check +
        // SCT-gate the singleton group, then upgrade. (A recursive fn WITH
        // `requires` — `full_ty` ≠ carrier — is a tracked follow-on; see
        // `elaborate_recursive_view`'s K2c note.)
        let result = kernel_check(env, &Context::new(), &full_body, &full_ty)
            .and_then(|_| sct_check(env, &[(pre_id, full_body.clone())]));
        match result {
            Ok(()) => {
                env.upgrade_to_transparent(pre_id, full_body);
                pre_id
            }
            Err(e) => {
                // Roll back the pre-admission + any obligation holes / literal
                // postulates added after it (ensures holes from Phase 3, etc.).
                while let Some(d) = env.remove_last() {
                    if d.id() == pre_id {
                        break;
                    }
                }
                globals.remove(&rdecl.name);
                return Err(ElabError::KernelRejected {
                    error: e,
                    span: rdecl.span.clone(),
                });
            }
        }
    } else {
        let id = declare_def(env, vec![], full_ty, full_body).map_err(|e| {
            ElabError::KernelRejected {
                error: e,
                span: rdecl.span.clone(),
            }
        })?;
        globals.insert(rdecl.name.clone(), id);
        id
    };
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: id,
        obligations: ens_obligations,
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

/// Elaborate `prove name : φ` (`21 §6.3`, §3).
///
/// Declares `name` as a postulate of `φ`, emitting one obligation hole.
fn elaborate_prove(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    let phi_core = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        let omega = Term::omega(Level::Zero);
        let (phi_raw, phi_ty_raw) = infer(&mut cx, &rdecl.body)?;
        // Check φ is Ω-typed
        unify_types(&mut cx.metas, &omega, &phi_ty_raw);
        cx.metas.zonk_term(&phi_raw)
    };
    // Declare as postulate (the hole)
    let hole_id =
        declare_postulate(env, rdecl.name.clone(), vec![], phi_core.clone()).map_err(|e| {
            ElabError::KernelRejected {
                error: e,
                span: rdecl.span.clone(),
            }
        })?;
    globals.insert(rdecl.name.clone(), hole_id);
    let obl = Obligation {
        id: 0,
        hole_id,
        goal_closed: phi_core,
        span: rdecl.span.clone(),
        kind: ObligationKind::Prove,
    };
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: hole_id,
        obligations: vec![obl],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

fn elaborate_prop_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    rdecl: &RDecl,
    intros: &[RPropIntro],
) -> Result<ElabResult, ElabError> {
    let prop_ty = rdecl.ty.as_ref().ok_or_else(|| {
        ElabError::Internal(format!(
            "prop '{}' reached elaboration without a type",
            rdecl.name
        ))
    })?;
    validate_seed_prop_shape(prop_ty, &rdecl.name, intros, &rdecl.span)?;

    let (ty_core, body_core) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone());
        let ty = elab_type(&mut cx, prop_ty)?;
        let ty = cx.metas.zonk_term(&ty);
        let body = top_body_for_prop_type(env, &ty, &rdecl.span)?;
        (ty, body)
    };

    let id = declare_def(env, vec![], ty_core.clone(), body_core).map_err(|e| {
        ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        }
    })?;
    globals.insert(rdecl.name.clone(), id);

    let mut produced = ElabResult {
        name: rdecl.name.clone(),
        def_id: id,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    };

    for intro in intros {
        let helper_ty = prepend_prop_params(prop_ty, &intro.ty)?;
        let helper_name = format!("{}.{}", rdecl.name, intro.name);
        let helper_rdecl = RDecl {
            name: helper_name,
            ty: Some(helper_ty),
            body: top_intro_body(prop_ty, &intro.span)?,
            requires: vec![],
            ensures: vec![],
            span: intro.span.clone(),
            kind: RDeclKind::Theorem,
        };
        let helper = elaborate_checked_theorem(
            env,
            globals,
            num_values,
            numeric_env,
            &ClassEnv::sentinel(),
            &helper_rdecl,
            None,
        )?;
        produced.def_id = id;
        produced.obligations.extend(helper.obligations);
    }

    Ok(produced)
}

fn elaborate_checked_theorem(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    rdecl: &RDecl,
    attached_subject: Option<&str>,
) -> Result<ElabResult, ElabError> {
    if globals.contains_key(&rdecl.name) {
        return Err(ElabError::TypeMismatch {
            span: rdecl.span.clone(),
            reason: format!("duplicate proof name '{}'", rdecl.name),
        });
    }

    let (ty_core, body_core, body_obligations) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, rdecl.name.clone())
            .with_classes(class_env);
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal(format!("checked theorem '{}' has no type", rdecl.name))
        })?;
        let ty_core = elab_type(&mut cx, ty)?;
        let ty_core = cx.metas.zonk_term(&ty_core);
        ensure_omega_type(cx.env, &Context::new(), &ty_core, &rdecl.span)?;
        if let Some(subject) = attached_subject {
            validate_attached_subject_occurs_applied(
                cx.env,
                cx.globals,
                subject,
                &ty_core,
                &rdecl.span,
            )?;
        }
        let body_core = check(&mut cx, &rdecl.body, &ty_core, &rdecl.span)?;
        let obligations = std::mem::take(&mut cx.obligations);
        (
            cx.metas.zonk_term(&ty_core),
            cx.metas.zonk_term(&body_core),
            obligations,
        )
    };
    let id =
        declare_def(env, vec![], ty_core, body_core).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        })?;
    globals.insert(rdecl.name.clone(), id);
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: id,
        obligations: body_obligations,
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

fn ensure_omega_type(
    env: &GlobalEnv,
    ctx: &Context,
    ty: &Term,
    span: &Span,
) -> Result<(), ElabError> {
    let sort = kernel_infer(env, ctx, ty).map_err(|e| ElabError::KernelRejected {
        error: e,
        span: span.clone(),
    })?;
    match whnf(env, ctx, &sort) {
        Term::Omega(_) => Ok(()),
        other => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!("proof claim type must classify at Omega, found {:?}", other),
        }),
    }
}

/// `fn` and `const` are computational definitions. Their result type must
/// classify at `Type`, leaving Ω-valued definitions to `theorem` and `proof`.
fn ensure_not_omega_type(
    env: &GlobalEnv,
    ctx: &Context,
    ty: &Term,
    span: &Span,
) -> Result<(), ElabError> {
    let sort = kernel_infer(env, ctx, ty).map_err(|e| ElabError::KernelRejected {
        error: e,
        span: span.clone(),
    })?;
    match whnf(env, ctx, &sort) {
        Term::Type(_) => Ok(()),
        Term::Omega(_) => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "`fn`/`const` compute; use `theorem`/`proof` for an Ω-valued definition"
                .to_string(),
        }),
        other => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "`fn`/`const` result must classify at Type, found {:?}",
                other
            ),
        }),
    }
}

fn validate_attached_subject_occurs_applied(
    env: &GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    subject: &str,
    proof_ty: &Term,
    span: &Span,
) -> Result<(), ElabError> {
    let subject_id = globals
        .get(subject)
        .copied()
        .ok_or_else(|| ElabError::UnboundName {
            name: subject.to_string(),
            span: span.clone(),
        })?;
    env.const_type(subject_id)
        .ok_or_else(|| ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!("attached proof subject '{}' is not a definition", subject),
        })?;
    if term_contains_applied_global(proof_ty, subject_id) {
        Ok(())
    } else {
        Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "attached proof for '{}' must mention that subject applied in its claim",
                subject
            ),
        })
    }
}

fn term_contains_applied_global(term: &Term, target: GlobalId) -> bool {
    if let Term::App(fun, _) = term {
        let mut head = fun.as_ref();
        while let Term::App(next, _) = head {
            head = next;
        }
        if matches!(head, Term::Const { id, .. } if *id == target) {
            return true;
        }
    }
    term.children()
        .into_iter()
        .any(|child| term_contains_applied_global(child, target))
}

fn top_body_for_prop_type(env: &GlobalEnv, ty: &Term, span: &Span) -> Result<Term, ElabError> {
    match ty {
        Term::Pi(dom, cod) => Ok(Term::lam(
            *dom.clone(),
            top_body_for_prop_type(env, cod, span)?,
        )),
        _ => {
            match whnf(env, &Context::new(), ty) {
                Term::Omega(_) => {}
                other => {
                    return Err(ElabError::TypeMismatch {
                        span: span.clone(),
                        reason: format!("prop family result must be Omega, found {:?}", other),
                    })
                }
            }
            Ok(Term::const_(env.top_id(), vec![]))
        }
    }
}

fn top_intro_body(prop_ty: &RType, span: &Span) -> Result<RExpr, ElabError> {
    match prop_ty {
        RType::RPi(name, _, cod, _) => {
            let body = top_intro_body(cod, span)?;
            Ok(RExpr::RLam(name.clone(), Box::new(body), span.clone()))
        }
        _ => Ok(RExpr::RCon("Proved".to_string(), span.clone())),
    }
}

fn prepend_prop_params(prop_ty: &RType, result: &RType) -> Result<RType, ElabError> {
    match prop_ty {
        RType::RPi(name, dom, cod, span) => Ok(RType::RPi(
            name.clone(),
            dom.clone(),
            Box::new(prepend_prop_params(cod, result)?),
            span.clone(),
        )),
        _ => Ok(result.clone()),
    }
}

fn validate_seed_prop_shape(
    prop_ty: &RType,
    prop_name: &str,
    intros: &[RPropIntro],
    span: &Span,
) -> Result<(), ElabError> {
    let mut param_count = 0;
    let mut cur = prop_ty;
    while let RType::RPi(_, _, cod, _) = cur {
        param_count += 1;
        cur = cod;
    }
    match cur {
        RType::RCon(name, _) if name == "Omega" || name == "Prop" => {}
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "prop family result must be Omega".to_string(),
            })
        }
    }
    for intro in intros {
        let args = peel_rtype_app(&intro.ty, prop_name).ok_or_else(|| ElabError::TypeMismatch {
            span: intro.span.clone(),
            reason: format!(
                "prop intro '{}' must return the declared family '{}'",
                intro.name, prop_name
            ),
        })?;
        if args.len() != param_count {
            return Err(ElabError::TypeMismatch {
                span: intro.span.clone(),
                reason: format!(
                    "prop intro '{}' must apply '{}' to exactly its parameters",
                    intro.name, prop_name
                ),
            });
        }
        for (i, arg) in args.iter().enumerate() {
            let expected = param_count - 1 - i;
            match arg {
                RType::RVarTy(idx, _, _) if *idx == expected => {}
                _ => {
                    return Err(ElabError::TypeMismatch {
                        span: intro.span.clone(),
                        reason: format!(
                            "prop intro '{}' is outside the v0 Omega-clean seed shape",
                            intro.name
                        ),
                    })
                }
            }
        }
    }
    Ok(())
}

fn peel_rtype_app<'a>(ty: &'a RType, head_name: &str) -> Option<Vec<&'a RType>> {
    let mut args = Vec::new();
    let mut cur = ty;
    loop {
        match cur {
            RType::RApp(f, a, _) => {
                args.push(a.as_ref());
                cur = f.as_ref();
            }
            RType::RCon(name, _) if name == head_name => {
                args.reverse();
                return Some(args);
            }
            _ => return None,
        }
    }
}

/// Elaborate `temporal name { φ }` — a delegated temporal/behavioral
/// obligation (`72 §4`).
///
/// The surface formula elaborates to a [`Temporal`] value (the §3
/// constructors, derived ops expanded) and is recorded as a **delegated**
/// obligation — **not** a kernel hole. A delegated property is exported, not
/// assumed (`21 §5.2`): it never enters `trusted_base()` (it is not
/// `unknown`) and is never kernel-proved (not `proved`/`Q`). Its sole
/// projection is the B1 `T`/`delegated` channel (TE-E). The verbatim `source`
/// is carried for human-visibility (`72 §4`).
fn elaborate_temporal(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    rdecl: &RDecl,
    formula: &crate::temporal::TemporalExpr,
    source: &str,
) -> Result<ElabResult, ElabError> {
    use crate::temporal::{elaborate_temporal_expr, TemporalObligation};

    let temporal_value = elaborate_temporal_expr(formula);
    // Stable obligation id (`22 §1`): one obligation per `temporal{}` block.
    let id = format!("{}.temporal.0", rdecl.name);
    let obl = TemporalObligation {
        id,
        formula: temporal_value,
        source: source.to_string(),
    };

    // Delegated ≠ unknown: allocate a placeholder `def_id` that is NOT
    // committed to the kernel env, so the obligation never enters
    // `trusted_base()`. Reserve the name in `globals`.
    let placeholder = env.fresh_id();
    globals.insert(rdecl.name.clone(), placeholder);

    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: placeholder,
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![obl],
        effect_row_type: None,
    })
}

/// Elaborate `law Name (param) { f : φ ; … }` (`21 §3`).
///
/// Each field φ is checked at Ω; one obligation hole per field.
fn elaborate_law(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    rdecl: &RDecl,
    _param: String,
    fields: Vec<(String, RExpr)>,
) -> Result<ElabResult, ElabError> {
    let omega = Term::omega(Level::Zero);
    let mut obligations: Vec<Obligation> = Vec::new();

    // The param is pre-declared by the resolver; for each field φ, check at Ω
    // and emit an obligation hole.
    for (i, (field_name, field_phi)) in fields.iter().enumerate() {
        let phi_core = {
            let mut cx = ElabCtx::new(
                env,
                globals,
                num_values,
                numeric_env,
                format!("{}.{}", rdecl.name, field_name),
            );
            // param is the law's `param` argument — it's in scope (resolver pushed it)
            // For elaboration, we need the param in scope. Since the resolver resolved
            // field_phi with param in scope at Var(0), we replicate that:
            // Note: we DON'T have a declared type for the param here. For V1, the param
            // is just a term variable whose type must be inferrable from the field props.
            // For test cases, params will always be globally declared.
            let (phi_raw, phi_ty_raw) = infer(&mut cx, field_phi)?;
            unify_types(&mut cx.metas, &omega, &phi_ty_raw);
            cx.metas.zonk_term(&phi_raw)
        };
        let hole_id = declare_postulate(
            env,
            format!("{}.{}", rdecl.name, field_name),
            vec![],
            phi_core.clone(),
        )
        .map_err(|e| ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        })?;
        let law_field_name = format!("{}_{}", rdecl.name, field_name);
        globals.insert(law_field_name, hole_id);
        obligations.push(Obligation {
            id: i as u32,
            hole_id,
            goal_closed: phi_core,
            span: rdecl.span.clone(),
            kind: ObligationKind::LawField(field_name.clone()),
        });
    }

    // The law itself: declare a postulate of the conjunction type.
    // For V1, law_id is a fresh postulate (placeholder — full Σ-of-Ω is V3+).
    let law_ty = Term::omega(Level::Zero);
    let law_id = declare_postulate(env, rdecl.name.clone(), vec![], law_ty).map_err(|e| {
        ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        }
    })?;
    globals.insert(rdecl.name.clone(), law_id);

    // Return: def_id = law_id (the law postulate), obligations = per-field holes
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: law_id,
        obligations,
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

// ----- helpers -----

/// Elaborate `expr` checked at Ω in `ctx`, returning the core term.
///
/// Used for requires/ensures proposition bodies.
fn elab_in_ctx_at_omega(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
    ctx: &Context,
    expr: &RExpr,
    omega: &Term,
    span: &Span,
    owner_label: &str,
) -> Result<Term, ElabError> {
    let mut cx = ElabCtx::new(
        env,
        globals,
        num_values,
        numeric_env,
        owner_label.to_string(),
    )
    .with_classes(class_env)
    .with_local_dicts(local_dicts);
    // Populate cx.ctx from the snapshot
    for ty in &ctx.types {
        cx.ctx.push(ty.clone());
    }
    let (core_raw, ty_raw) = infer(&mut cx, expr)?;
    // Unify inferred type with Ω — if the proposition is non-Ω, this will
    // be caught by the kernel on the next kernel_check call.
    // For the surface error, check that ty is Ω-shaped.
    let ty_zonked = cx.metas.zonk_term(&ty_raw);
    let core_zonked = cx.metas.zonk_term(&core_raw);
    // Surface-level Ω check: if the type is not Omega(_), error
    match &ty_zonked {
        Term::Omega(_) => {}
        _ => {
            // Check if the kernel will accept it as Ω — check core at omega
            // If not, surface error
            kernel_check(env, ctx, &core_zonked, omega).map_err(|_| ElabError::TypeMismatch {
                span: span.clone(),
                reason: format!("spec proposition must have type Ω, found non-proposition"),
            })?;
        }
    }
    Ok(core_zonked)
}

/// Unwrap the outermost `n` Pi binders, collecting domain types.
///
/// `Pi(A, Pi(B, C))` with n=2 → `[A, B]` (A = outermost, B = innermost param).
fn unwrap_pi_chain(ty: &Term) -> Vec<Term> {
    let mut result = Vec::new();
    let mut cur = ty;
    loop {
        match cur {
            Term::Pi(dom, cod) => {
                result.push(*dom.clone());
                cur = cod;
            }
            _ => break,
        }
    }
    result
}

/// Return the innermost codomain of a Pi-chain.
fn innermost_codomain(ty: &Term) -> Term {
    let mut cur = ty;
    loop {
        match cur {
            Term::Pi(_, cod) => cur = cod,
            other => return other.clone(),
        }
    }
}

/// Unwrap the outermost `n` Lam binders, returning the inner body.
fn unwrap_lam(term: &Term, n: usize) -> Term {
    let mut cur = term;
    for _ in 0..n {
        match cur {
            Term::Lam(_, body) => cur = body,
            _ => break,
        }
    }
    cur.clone()
}

// ----- match elaboration -----

/// Elaborate `match scrut { C₁ x₁… => body₁ ; … }` (`34 §3`).
///
/// Compiles to `Term::Elim` with one method per constructor in declaration order.
/// Constant-motive variant: return type inferred from the first arm, checked
/// consistent across all arms by kernel type-checking the Elim.
/// A pending column in the pattern-matrix compiler (`34-data-match.md §3.1`):
/// either a genuine surface column (tracked per-row in `RowState::real_pats`)
/// or a synthetic induction-hypothesis slot the eliminator's method type
/// requires but no surface pattern ever names.
/// `Ih(remaining)`: `remaining` is how many MORE `Ih` columns immediately
/// following this one belong to the *same* constructor bucket (the same
/// `build_ctor_buckets` call that produced this one) — 0 for the last (or
/// only) `Ih` in its own batch. This lets `compile_match_matrix` tell "my
/// own sibling Ih, from the ctor I was just built for" (skip over — its own
/// type is flat, computed independently) apart from "a genuinely enclosing
/// split's pending tail" (fold via `tail_codomain`, as that tail's owed
/// type is not flat).
#[derive(Clone, Copy, PartialEq, Eq)]
enum ColKind {
    Real,
    Ih(usize),
}

/// One row of the pattern matrix: the still-unconsumed `Real` column
/// patterns for one arm, plus which top-level arm it came from (for
/// reachability bookkeeping across wildcard-row expansion, `§4.2`).
struct RowState {
    real_pats: Vec<RPattern>,
    arm_idx: usize,
}

/// The type every raw method built from `col_types`/`col_kinds` (a suffix of
/// still-pending columns) ultimately has, as a Pi-chain ending in `ret_ty`:
/// each `Real` column contributes one arrow (regardless of whether it is
/// later bound flatly or split further — that happens *inside* the arrow's
/// codomain, never changing the arrow's own presence), each `Ih` column
/// contributes an arrow of type `ret_ty` weakened to its own position. This
/// is exactly what a split's motive must compute once applied to a
/// scrutinee value — a nested `elim_D` still owes whatever the tail owes.
fn tail_codomain(
    tail_col_types: &[Term],
    tail_col_kinds: &[ColKind],
    ret_ty_base: &Term,
    depth_before_tail: usize,
) -> Term {
    if tail_col_types.is_empty() {
        return weaken(ret_ty_base, depth_before_tail as i64);
    }
    match tail_col_kinds[0] {
        ColKind::Ih(_) => {
            let ih_ty = weaken(ret_ty_base, depth_before_tail as i64);
            let rest = tail_codomain(
                &tail_col_types[1..],
                &tail_col_kinds[1..],
                ret_ty_base,
                depth_before_tail,
            );
            Term::pi(ih_ty, weaken(&rest, 1))
        }
        ColKind::Real => {
            let rest = tail_codomain(
                &tail_col_types[1..],
                &tail_col_kinds[1..],
                ret_ty_base,
                depth_before_tail + 1,
            );
            Term::pi(tail_col_types[0].clone(), rest)
        }
    }
}

/// Compile the pattern matrix `col_types`/`col_kinds` (aligned; `Real`
/// columns are matched against `rows[_].real_pats`, `Ih` columns are
/// synthetic and never touch row patterns) down to a nested-`elim_D` method
/// term, per the standard column-by-column algorithm.
///
/// `real_depth_so_far` counts only genuine (`Real`, non-split) `cx.ctx`
/// pushes made along the current path — it lines up with what `resolve.rs`
/// counted when flattening pattern-bound names, so `infer`'s raw
/// `Term::var(i)` passthrough resolves correctly. Columns that need
/// splitting (a `Ctor` sub-pattern present) or `Ih` slots are *never* pushed
/// onto `cx.ctx` — they are woven in afterward via `weaken`, exactly as the
/// pre-existing single-level code already did for induction hypotheses.
fn compile_match_matrix(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    col_types: &[Term],
    col_kinds: &[ColKind],
    rows: Vec<RowState>,
    real_depth_so_far: usize,
    top_span: &Span,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Term, ElabError> {
    if col_types.is_empty() {
        // Leaf: the first row in preserved (first-match-wins) order claims
        // this path; any others are shadowed here (possibly still reachable
        // via a different expansion elsewhere — checked globally by the
        // caller via `arm_used`). Record each shadowed row's winner at THIS
        // leaf into `subsumed_by` -- the same information the caller's final
        // `arm_used` sweep needs, captured where it is already in hand
        // rather than re-derived by a second walk. A wildcard/variable row
        // can reach more than one leaf (expanded into every constructor
        // bucket in `build_ctor_buckets`), so a dead arm can accumulate more
        // than one distinct subsuming winner across the whole compile;
        // dedup so repeats across leaves don't inflate the reported set.
        let winner = rows[0].arm_idx;
        arm_used[winner] = true;
        for shadowed in &rows[1..] {
            let entry = &mut subsumed_by[shadowed.arm_idx];
            if !entry.contains(&winner) {
                entry.push(winner);
            }
        }
        let arm = &arms[winner];
        let (body_core, body_ty_ctx) = infer(cx, &arm.body)?;
        if ret_ty_slot.is_none() {
            let zonked = cx.metas.zonk_term(&body_ty_ctx);
            let lowered = lower_by(&zonked, real_depth_so_far).unwrap_or(zonked);
            *ret_ty_slot = Some(lowered);
        }
        return Ok(body_core);
    }

    match col_kinds[0] {
        ColKind::Ih(remaining) => {
            // A synthetic induction-hypothesis slot: never resolver-counted,
            // so it is woven in via weaken-then-wrap rather than a real push.
            //
            // The IH's own type is `M` applied to its field, where `M` is
            // the motive of the elim THIS Ih belongs to (constant, so `M x`
            // is just some fixed type) — but that fixed type is not always
            // the bare `ret_ty`: it is `ret_ty` only when there is no
            // genuinely-enclosing split still owed beyond this Ih's own
            // ctor batch. `remaining` siblings immediately follow from the
            // SAME `build_ctor_buckets` call (the same ctor's own other
            // recursive fields) — those are invisible to THIS Ih's type,
            // since each sibling gets its own independent binder via the
            // recursive call below. Skip past them, then fold whatever
            // comes after via `tail_codomain` — if that's empty (no
            // enclosing split), the fold degenerates to flat `ret_ty`
            // exactly like the sibling case; if non-empty (this Ih sits
            // inside a nested split's method, e.g. matching a sub-pattern
            // one recursive field deep), the enclosing split's own pending
            // continuation (its constant motive's codomain) is genuinely
            // owed and must be folded in.
            let ret_ty = ret_ty_slot
                .as_ref()
                .expect("IH column reached before return type known")
                .clone();
            let ih_ty = tail_codomain(
                &col_types[remaining + 1..],
                &col_kinds[remaining + 1..],
                &ret_ty,
                real_depth_so_far,
            );
            let inner = compile_match_matrix(
                cx,
                arms,
                &col_types[1..],
                &col_kinds[1..],
                rows,
                real_depth_so_far,
                top_span,
                ret_ty_slot,
                arm_used,
                subsumed_by,
            )?;
            Ok(Term::lam(ih_ty, weaken(&inner, 1)))
        }
        ColKind::Real => {
            let all_flat = rows
                .iter()
                .all(|r| matches!(r.real_pats[0].kind, RPatKind::Wild | RPatKind::Var(_)));
            if all_flat {
                // No constructor pattern in this column across any row: bind
                // it flatly (a real `cx.ctx` push), matching the resolver's
                // count exactly, and move on.
                cx.ctx.push(col_types[0].clone());
                let new_rows: Vec<RowState> = rows
                    .into_iter()
                    .map(|r| RowState {
                        real_pats: r.real_pats[1..].to_vec(),
                        arm_idx: r.arm_idx,
                    })
                    .collect();
                let inner = compile_match_matrix(
                    cx,
                    arms,
                    &col_types[1..],
                    &col_kinds[1..],
                    new_rows,
                    real_depth_so_far + 1,
                    top_span,
                    ret_ty_slot,
                    arm_used,
                    subsumed_by,
                );
                cx.ctx.pop();
                return Ok(Term::lam(col_types[0].clone(), inner?));
            }

            // At least one row has a constructor pattern here: split.
            let ty0 = whnf(cx.env, &cx.ctx, &col_types[0]);
            let (head, params0) = peel_app(&ty0);
            let d_id0 = match head {
                Term::IndFormer { id, .. } => id,
                _ => {
                    return Err(ElabError::TypeMismatch {
                        span: top_span.clone(),
                        reason: "match scrutinee must have an inductive type".into(),
                    })
                }
            };
            let ind0 = cx
                .env
                .inductive(d_id0)
                .ok_or_else(|| ElabError::Internal(format!("inductive {:?} not found", d_id0)))?
                .clone();
            let m0 = ind0.params.len();

            let raw_methods = build_ctor_buckets(
                cx,
                arms,
                &ind0,
                d_id0,
                m0,
                &params0,
                rows,
                &col_types[1..],
                &col_kinds[1..],
                real_depth_so_far,
                top_span,
                ret_ty_slot,
                arm_used,
                subsumed_by,
            )?;

            // The split column itself is a fresh binder no surface pattern
            // named — resolver never counted it, so (like the IH slots
            // above) it is woven in via weaken-then-wrap, never a real push.
            //
            // The motive's codomain is NOT bare `ret_ty`: any columns still
            // pending after this split (a sibling field, or an enclosing
            // constructor's own IH slot carried in via `tail_col_kinds`)
            // still owe a value, so each raw method's real type is
            // `(tail columns) -> ret_ty`, and the motive must match.
            let ret_ty_base = ret_ty_slot
                .as_ref()
                .expect("split column reached before return type known")
                .clone();
            let codomain = tail_codomain(
                &col_types[1..],
                &col_kinds[1..],
                &ret_ty_base,
                real_depth_so_far + 1,
            );
            let ret_level = match kernel_infer(cx.env, &cx.ctx, &codomain) {
                Ok(Term::Type(l)) => l,
                _ => Level::Zero,
            };
            let motive_ty = Term::pi(col_types[0].clone(), Term::ty(ret_level));
            let motive = Term::Ascript(
                Box::new(Term::lam(col_types[0].clone(), codomain)),
                Box::new(motive_ty),
            );
            let methods: Vec<Term> = raw_methods.iter().map(|m| weaken(m, 1)).collect();
            let elim = Term::Elim {
                fam: d_id0,
                level_args: vec![],
                params: params0.iter().map(|p| weaken(p, 1)).collect(),
                motive: Box::new(motive),
                methods,
                indices: vec![],
                scrut: Box::new(Term::var(0)),
            };
            Ok(Term::lam(col_types[0].clone(), elim))
        }
    }
}

/// Group `rows` (whose `real_pats[0]` matches the inductive `ind0`) into one
/// bucket per constructor — expanding a `Wild`/`Var` row into every
/// constructor (it matches all of them) — and recurse to build each
/// constructor's raw method term: `λ(fields). λ(IHs). <continuation>`,
/// where `<continuation>` threads through `tail_col_types`/`tail_col_kinds`
/// (the columns after this one). Each returned method is valid at
/// `real_depth_so_far` — i.e. as if the split column's own binder does not
/// yet exist; the caller (top-level `infer_match`, or a nested nested split
/// in `compile_match_matrix`) wraps accordingly.
#[allow(clippy::too_many_arguments)]
fn build_ctor_buckets(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    ind0: &InductiveDecl,
    d_id0: GlobalId,
    m0: usize,
    params0: &[Term],
    rows: Vec<RowState>,
    tail_col_types: &[Term],
    tail_col_kinds: &[ColKind],
    real_depth_so_far: usize,
    top_span: &Span,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Vec<Term>, ElabError> {
    let mut methods: Vec<Option<Term>> = vec![None; ind0.constructors.len()];

    for (k0, c0) in ind0.constructors.iter().enumerate() {
        let mut bucket: Vec<RowState> = Vec::new();
        for r in &rows {
            match &r.real_pats[0].kind {
                RPatKind::Ctor(name, subs) => {
                    if cx.globals.get(name).copied() == Some(c0.id) {
                        let mut new_pats = subs.clone();
                        new_pats.extend_from_slice(&r.real_pats[1..]);
                        bucket.push(RowState {
                            real_pats: new_pats,
                            arm_idx: r.arm_idx,
                        });
                    }
                }
                RPatKind::Wild | RPatKind::Var(_) => {
                    let span = r.real_pats[0].span.clone();
                    let mut new_pats: Vec<RPattern> = (0..c0.args.len())
                        .map(|_| RPattern {
                            kind: RPatKind::Wild,
                            span: span.clone(),
                        })
                        .collect();
                    new_pats.extend_from_slice(&r.real_pats[1..]);
                    bucket.push(RowState {
                        real_pats: new_pats,
                        arm_idx: r.arm_idx,
                    });
                }
            }
        }

        let n_args0 = c0.args.len();
        if bucket.is_empty() {
            return Err(ElabError::ExhaustivenessError {
                missing: missing_pattern_witness(cx, c0.id),
                span: top_span.clone(),
            });
        }

        let field_types0: Vec<Term> = (0..n_args0)
            .map(|j| subst_outer(&c0.args[j], m0, params0, j))
            .collect();
        let p_ihs0 = recursive_shapes(cx.env, c0, d_id0, m0)
            .map_err(|error| ElabError::KernelRejected {
                error,
                span: top_span.clone(),
            })?
            .len();

        // `col_types`/`col_kinds` stay index-aligned; an `Ih` slot's own type
        // entry is never read (its lambda domain is computed from `ret_ty`
        // instead) but must still occupy a position.
        let mut new_col_types = field_types0;
        new_col_types.extend(std::iter::repeat(Term::ty(Level::Zero)).take(p_ihs0));
        new_col_types.extend_from_slice(tail_col_types);
        let mut new_col_kinds: Vec<ColKind> = vec![ColKind::Real; n_args0];
        new_col_kinds.extend((0..p_ihs0).map(|i| ColKind::Ih(p_ihs0 - 1 - i)));
        new_col_kinds.extend_from_slice(tail_col_kinds);

        let inner = compile_match_matrix(
            cx,
            arms,
            &new_col_types,
            &new_col_kinds,
            bucket,
            real_depth_so_far,
            top_span,
            ret_ty_slot,
            arm_used,
            subsumed_by,
        )?;
        methods[k0] = Some(inner);
    }

    Ok(methods.into_iter().map(|m| m.unwrap()).collect())
}

fn infer_match(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    for arm in arms {
        ensure_pattern_constructors_resolve(cx, &arm.pat)?;
    }
    // 1. Infer scrutinee.
    let (scrut_core, scrut_ty_raw) = infer(cx, scrut)?;
    let scrut_ty = whnf(cx.env, &cx.ctx, &scrut_ty_raw);

    // 2. Peel the type-former application: D p₀ … pₘ₋₁.
    let (head, params_terms) = peel_app(&scrut_ty);
    let d_id = match &head {
        Term::IndFormer { id, .. } => *id,
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "match scrutinee must have an inductive type".into(),
            })
        }
    };

    // 3. Clone the InductiveDecl so we can release the &env borrow before
    //    mutating cx.ctx inside the recursive matrix compiler.
    let ind = cx
        .env
        .inductive(d_id)
        .ok_or_else(|| ElabError::Internal(format!("inductive {:?} not found", d_id)))?
        .clone();
    ensure_arm_ctors_belong_to_family(cx, arms, &ind, d_id)?;
    let m = ind.params.len();

    // 4. Every arm must open with a constructor pattern (no top-level
    //    wildcard/var scrutinee-binding yet); nested sub-patterns may be
    //    arbitrary (`Ctor`, `Var`, `Wild`, recursively).
    for arm in arms {
        if let RPatKind::Wild | RPatKind::Var(_) = arm.pat.kind {
            return Err(ElabError::Internal(
                "non-constructor pattern in match (wildcard/var not yet supported \
                 at top level; use constructor patterns)"
                    .into(),
            ));
        }
    }

    // 5. Build the initial one-column matrix (the scrutinee itself) and
    //    compile it via the pattern-matrix algorithm (`34-data-match.md
    //    §3.1`): column-by-column, splitting on constructors, recursing on
    //    the residual matrix under each constructor's freshly-bound fields.
    let rows: Vec<RowState> = arms
        .iter()
        .enumerate()
        .map(|(i, arm)| RowState {
            real_pats: vec![arm.pat.clone()],
            arm_idx: i,
        })
        .collect();

    let mut ret_ty_slot: Option<Term> = None;
    let mut arm_used = vec![false; arms.len()];
    let mut subsumed_by: Vec<Vec<usize>> = vec![Vec::new(); arms.len()];

    let raw_methods = build_ctor_buckets(
        cx,
        arms,
        &ind,
        d_id,
        m,
        &params_terms,
        rows,
        &[],
        &[],
        0,
        span,
        &mut ret_ty_slot,
        &mut arm_used,
        &mut subsumed_by,
    )?;

    // 6. AC4: reachability — an arm that never won at any leaf (including any
    //    it was expanded into via a wildcard row) is dead code.
    for (i, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[i].split_first() {
                Some((&first, rest)) => ArmDeadCause::Subsumed {
                    first: arms[first].span.clone(),
                    rest: rest.iter().map(|&w| arms[w].span.clone()).collect(),
                },
                None => ArmDeadCause::NoInhabitants,
            };
            return Err(ElabError::ReachabilityError {
                span: arms[i].span.clone(),
                cause,
            });
        }
    }

    let ret_ty = ret_ty_slot.unwrap_or_else(|| Term::ty(Level::Zero));

    // 7. Build the constant motive: Ascript(λ(x: D). R, D → Type ℓ)
    //    The kernel can't infer the type of a bare lambda, so we annotate.
    //    Determine ℓ from the return type's own type.
    let ret_level = {
        match kernel_infer(cx.env, &cx.ctx, &ret_ty) {
            Ok(Term::Type(l)) => l,
            _ => Level::Zero, // fallback: level 0
        }
    };
    let motive_ty = Term::pi(scrut_ty.clone(), Term::ty(ret_level));
    let motive = Term::Ascript(
        Box::new(Term::lam(scrut_ty.clone(), weaken(&ret_ty, 1))),
        Box::new(motive_ty),
    );

    // 8. Build Term::Elim (non-indexed: indices = []). The top-level
    //    scrutinee is already a concrete elaborated value (`scrut_core`), so
    //    — unlike a nested split — no extra binder/weaken is needed here.
    let elim = Term::Elim {
        fam: d_id,
        level_args: vec![],
        params: params_terms,
        motive: Box::new(motive),
        methods: raw_methods,
        indices: vec![],
        scrut: Box::new(scrut_core),
    };

    Ok((elim, ret_ty))
}

fn ensure_pattern_constructors_resolve(
    cx: &ElabCtx<'_>,
    pattern: &RPattern,
) -> Result<(), ElabError> {
    if let RPatKind::Ctor(name, fields) = &pattern.kind {
        if !cx.globals.contains_key(name) {
            return Err(ElabError::UnresolvedCon {
                name: name.clone(),
                span: pattern.span.clone(),
            });
        }
        for field in fields {
            ensure_pattern_constructors_resolve(cx, field)?;
        }
    }
    Ok(())
}

/// `LANG-FOREIGN-CTOR-ARM-REJECT`: every arm's top-level constructor pattern
/// must name a constructor of the scrutinee's own inductive family `ind`.
/// `ensure_pattern_constructors_resolve` only proves the name resolves to
/// SOME declared constructor; this proves it resolves to one of THIS type's.
/// Checked once, before match compilation begins, so a foreign constructor
/// (a real constructor of a *different* family) is rejected as the mismatch
/// it is instead of reaching the reachability sweep, where it would silently
/// read as `NoInhabitants` -- a true statement about inhabitants that says
/// nothing about the mismatch actually present. Scoped to the arm's own head
/// pattern only, never nested sub-patterns (the checked path this feeds
/// permits only flat `Var`/`Wild` sub-patterns; the general path's nested
/// sub-patterns each have their own, different, per-position expected type
/// and are out of this node's one-shape scope).
fn ensure_arm_ctors_belong_to_family(
    cx: &ElabCtx,
    arms: &[RMatchArm],
    ind: &InductiveDecl,
    d_id: GlobalId,
) -> Result<(), ElabError> {
    for arm in arms {
        if let RPatKind::Ctor(name, _) = &arm.pat.kind {
            let ctor_id = *cx.globals.get(name).expect(
                "ensure_pattern_constructors_resolve already validated every top-level \
                 arm pattern name resolves in cx.globals before this function runs",
            );
            if !ind.constructors.iter().any(|c| c.id == ctor_id) {
                return Err(ElabError::TypeMismatch {
                    span: arm.pat.span.clone(),
                    reason: format!(
                        "constructor '{}' is not a constructor of type '{}'",
                        name,
                        type_name(cx, d_id)
                    ),
                });
            }
        }
    }
    Ok(())
}

/// The elaborator's own surface name for an inductive family's `GlobalId`,
/// resolved the same way `ctor_name` resolves a constructor's: the kernel
/// records no name for a `Decl::Inductive` any more than it does for a
/// `ConstructorDecl` (names live only in `cx.globals`), so this is the same
/// inverse scan with the same render-something-over-panic fallback
/// (`prelude.rs`'s `combinator_actual_delta` precedent: "a diagnostic that
/// can fail to render is worse than the bare id it replaces").
fn type_name(cx: &ElabCtx, id: GlobalId) -> String {
    cx.globals
        .iter()
        .find(|(_, &candidate)| candidate == id)
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| format!("<type_{:?}>", id))
}

/// Shift a term's free variables DOWN by `k`, stopping with `None` if any
/// variable at index `i` (outer context) satisfies `0 ≤ i < k` (it references
/// a ctor-arg binder that doesn't exist in the outer scope).
///
/// Used to extract the return type from a match arm body type (which was
/// inferred in a context extended by k ctor-arg binders) back into the outer
/// context.  Closed types (Int, Bool, Color, …) pass through unchanged.
fn lower_by(term: &Term, k: usize) -> Option<Term> {
    if k == 0 {
        return Some(term.clone());
    }
    lower_by_inner(term, k, 0)
}

fn lower_by_inner(term: &Term, k: usize, cutoff: usize) -> Option<Term> {
    match term {
        Term::Var(i) => {
            if *i < cutoff {
                Some(Term::var(*i)) // bound under a local binder — keep as is
            } else if *i < cutoff + k {
                None // refers to a ctor-arg var — can't project to outer scope
            } else {
                Some(Term::var(*i - k)) // outer context var — shift down
            }
        }
        Term::Type(l) => Some(Term::ty(l.clone())),
        Term::Omega(l) => Some(Term::omega(l.clone())),
        Term::Pi(a, b) => Some(Term::pi(
            lower_by_inner(a, k, cutoff)?,
            lower_by_inner(b, k, cutoff + 1)?,
        )),
        Term::Lam(a, body) => Some(Term::lam(
            lower_by_inner(a, k, cutoff)?,
            lower_by_inner(body, k, cutoff + 1)?,
        )),
        Term::App(f, a) => Some(Term::app(
            lower_by_inner(f, k, cutoff)?,
            lower_by_inner(a, k, cutoff)?,
        )),
        Term::Const { id, level_args } => Some(Term::const_(*id, level_args.clone())),
        Term::IndFormer { id, level_args } => Some(Term::IndFormer {
            id: *id,
            level_args: level_args.clone(),
        }),
        Term::Constructor { id, level_args } => Some(Term::Constructor {
            id: *id,
            level_args: level_args.clone(),
        }),
        other => Some(other.clone()),
    }
}

// ----- standalone expression elaboration -----

pub fn elaborate_rexpr(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    owner_label: impl Into<String>,
    rexpr: &RExpr,
) -> Result<(Term, Term), ElabError> {
    let (core, ty, expr_span) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, owner_label);
        let (core_raw, ty_raw) = infer(&mut cx, rexpr)?;
        let c = cx.metas.zonk_term(&core_raw);
        let t = cx.metas.zonk_term(&ty_raw);
        (c, t, rexpr.span().clone())
    };
    kernel_check(env, &Context::new(), &core, &ty).map_err(|e| ElabError::KernelRejected {
        error: e,
        span: expr_span,
    })?;
    Ok((core, ty))
}

#[cfg(test)]
mod omega_index_refinement_tests {
    use crate::{ElabEnv, ElabError};
    use ken_kernel::{
        infer as kernel_infer, whnf, ConstructorDecl, Context, GlobalId, InductiveDecl, Level, Term,
    };

    use super::{
        build_index_omega_transport, check_variable_with_index_views,
        classify_branch_goal_restoration, install_hidden_result_variable_refinements,
        install_index_refinements, subst_term_generalize, try_reindex_cast, weaken, ElabCtx,
    };

    #[test]
    fn checked_index_refined_variable_selects_both_expected_views() {
        // Promise class: durable invariant. MEASURED: the checking-mode
        // selector returns the raw context binding for its constructor-local
        // type and the installed alias for its outer-refined type. CLAIMED:
        // capability 1 is dual-view at expected-type boundaries. THE GAP: the
        // integration grid independently proves a real dependent-match field
        // installs and consumes these views through the production producer.
        let mut env = ElabEnv::new().expect("base environment");
        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let top = Term::const_(env.env.top_id(), vec![]);
        let proved = Term::const_(env.env.tt_id(), vec![]);
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "dual-view-selector-control",
        );
        cx.ctx.push(nat.clone());
        cx.var_refinements
            .insert(0, (proved.clone(), top.clone(), cx.ctx.len()));

        assert_eq!(
            check_variable_with_index_views(&mut cx, 0, &nat)
                .expect("the constructor-local view must remain selectable"),
            Term::var(0)
        );
        assert_eq!(
            check_variable_with_index_views(&mut cx, 0, &top)
                .expect("the outer-refined view must remain selectable"),
            proved
        );
    }

    #[test]
    fn direct_omega_transport_builds_the_ruled_j_motive_exactly() {
        // Promise class: durable invariant. Intended extensions may add more
        // refinement callers or sorts; changing the ruled J motive, base,
        // scrutinee, or old-to-new orientation must make this control red.
        // MEASURED: the private production constructor's complete Term tree.
        // CLAIMED: decision 1 emits the ruled direct-J transport. THE GAP:
        // arm reachability and kernel admission are exercised by integration
        // tests over real dependent matches.
        let idx_ty = Term::ty(Level::Zero);
        let old_idx = Term::var(3);
        let new_idx = Term::var(4);
        let cur_ty = Term::Eq(
            Box::new(idx_ty.clone()),
            Box::new(old_idx.clone()),
            Box::new(old_idx.clone()),
        );
        let value = Term::var(5);
        let h = Term::var(6);

        let (transported, new_ty) = build_index_omega_transport(
            &idx_ty,
            &old_idx,
            &new_idx,
            &cur_ty,
            Level::Zero,
            value.clone(),
            h.clone(),
        );

        let expected_new_ty = subst_term_generalize(&cur_ty, &old_idx, &new_idx);
        let expected_at_y =
            subst_term_generalize(&weaken(&cur_ty, 2), &weaken(&old_idx, 2), &Term::var(1));
        let expected_eq_domain = Term::Eq(
            Box::new(weaken(&idx_ty, 1)),
            Box::new(weaken(&old_idx, 1)),
            Box::new(Term::var(0)),
        );
        let expected_motive_body = Term::lam(
            idx_ty.clone(),
            Term::lam(expected_eq_domain.clone(), expected_at_y),
        );
        let expected_motive_type = Term::pi(
            idx_ty,
            Term::pi(expected_eq_domain, Term::omega(Level::Zero)),
        );
        let expected = Term::J(
            Box::new(Term::Ascript(
                Box::new(expected_motive_body),
                Box::new(expected_motive_type),
            )),
            Box::new(value),
            Box::new(h),
        );

        assert_eq!(new_ty, expected_new_ty);
        assert_eq!(transported, expected);
    }

    #[test]
    fn omega_branch_goal_plan_reuses_the_pinned_direct_j_constructor() {
        // Promise class: durable invariant. Producer classification may gain
        // more sorts, but an Omega-tagged branch-goal plan must replay through
        // the same exact direct-J result D1 pins above, never a separate motive.
        // MEASURED: applying the D2 plan is Term-identical to the independently
        // pinned D1 constructor on the same complete input tuple. CLAIMED: D2
        // reuses that constructor. THE GAP: the real producer and kernel
        // admission are exercised by the D2 integration fixture.
        let index_type = Term::ty(Level::Zero);
        let old_index = Term::var(3);
        let new_index = Term::var(4);
        let source_type = Term::Eq(
            Box::new(index_type.clone()),
            Box::new(old_index.clone()),
            Box::new(old_index.clone()),
        );
        let value = Term::var(5);
        let equality = Term::var(6);
        let env = ElabEnv::new().expect("base environment");
        let plan = classify_branch_goal_restoration(
            &env.env,
            &Context::new(),
            &index_type,
            &old_index,
            &new_index,
            &source_type,
            Term::omega(Level::Zero),
            equality.clone(),
        )
        .expect("decision 2 must produce the Omega-J plan");
        let (pinned, _) = build_index_omega_transport(
            &index_type,
            &old_index,
            &new_index,
            &source_type,
            Level::Zero,
            value.clone(),
            equality,
        );

        assert_eq!(plan.apply(value), pinned);
    }

    #[test]
    fn branch_goal_classifier_outside_type_or_omega_fails_closed() {
        // Promise class: durable invariant. Decision 2 may gain explicitly
        // ruled sorts only; an inferred non-sort classifier must be rejected
        // with its actual identity rather than admitted by a catch-all.
        // MEASURED: the production classifier seam's exact error. CLAIMED:
        // decision 2 fails closed outside Type | Omega. THE GAP: integration
        // independently reaches both admitted tags through dependent matches.
        let env = ElabEnv::new().expect("base environment");
        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let index_type = Term::ty(Level::Zero);
        let old_index = Term::var(1);
        let new_index = Term::var(2);
        let source_type = Term::Eq(
            Box::new(index_type.clone()),
            Box::new(old_index.clone()),
            Box::new(old_index.clone()),
        );
        let error = match classify_branch_goal_restoration(
            &env.env,
            &Context::new(),
            &index_type,
            &old_index,
            &new_index,
            &source_type,
            nat.clone(),
            Term::var(3),
        ) {
            Err(error) => error,
            Ok(_) => panic!("a non-sort branch-goal classifier must fail closed"),
        };

        match error {
            ElabError::Internal(message) => {
                assert!(message.contains("classified by neither Type nor Omega"));
                assert!(message.contains(&format!("found {nat:?}")));
            }
            other => panic!("expected the decision-2 Internal error, got {other:?}"),
        }
    }

    #[test]
    fn hidden_result_prefilter_delegates_a_lawful_omega_outer_binding() {
        // Promise class: durable invariant. This is the private production
        // seam for decision 3: a well-formed `n : Nat, h : Eq Nat n n`
        // context supplies an Omega-classified outer binding that depends on
        // the refined index. The prefilter must delegate it to decision 1 and
        // install the direct-J refinement, rather than silently skipping it.
        // MEASURED: the returned installed position and its stored J term.
        // CLAIMED: decision 3 delegates lawful Omega bindings. THE GAP: the
        // position is private state; real matches exercise decision 1 itself.
        let mut env = ElabEnv::new().expect("base environment");
        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let zero = Term::Constructor {
            id: env.globals["Zero"],
            level_args: vec![],
        };
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "omega-prefilter-control",
        );
        cx.ctx.push(nat.clone());
        cx.ctx.push(Term::Eq(
            Box::new(nat.clone()),
            Box::new(Term::var(0)),
            Box::new(Term::var(0)),
        ));

        let installed =
            install_hidden_result_variable_refinements(&mut cx, &nat, &zero, &Term::var(1), 0, 2)
                .expect("the lawful Omega outer binding must be delegated");
        assert_eq!(installed, vec![1]);
        let (refined, refined_ty, install_depth) = cx
            .var_refinements
            .get(&1)
            .expect("the Omega binding's bottom-relative position must be installed");
        assert!(matches!(refined, Term::J(_, _, _)));
        assert_eq!(
            refined_ty,
            &Term::Eq(Box::new(nat), Box::new(zero.clone()), Box::new(zero),)
        );
        assert_eq!(*install_depth, 2);
    }

    #[test]
    fn hidden_result_matched_type_classifier_remains_type_only() {
        // Promise class: durable invariant. Decision 4 classifies the matched
        // type itself, not a re-indexed position, and must reject an Omega-
        // classified proposition before installing any outer refinement.
        // MEASURED: the exact decision-4 Internal diagnostic. CLAIMED: the
        // matched-type classifier stays Type-only. THE GAP: none; the test
        // invokes the production decision before later refinement work.
        let mut env = ElabEnv::new().expect("base environment");
        let top = Term::const_(env.env.top_id(), vec![]);
        let proved = Term::const_(env.globals["Proved"], vec![]);
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "omega-index-type-control",
        );
        cx.ctx.push(Term::ty(Level::Zero));

        let error =
            install_hidden_result_variable_refinements(&mut cx, &top, &proved, &proved, 0, 1)
                .expect_err("decision 4 must remain Type-only");
        match error {
            ElabError::Internal(message) => assert!(
                message.contains(
                    "result refinement: matched type is not classified by Type, found Ω0"
                ),
                "the decision-4 Type-only classifier moved: {message:?}"
            ),
            other => panic!("expected the decision-4 Internal error, got {other:?}"),
        }
    }

    #[test]
    fn sibling_convoy_index_type_classifier_remains_type_only() {
        // Promise class: durable invariant. Decision 5 classifies an index
        // type, so even though proposition domains are legal binder types, it
        // must reject an Omega-classified index before convoying any sibling.
        // MEASURED: the exact decision-5 Internal diagnostic. CLAIMED: the
        // convoy index classifier stays Type-only. THE GAP: the fake family is
        // required because admitted surface family indices are already Type.
        let mut env = ElabEnv::new().expect("base environment");
        let top = Term::const_(env.env.top_id(), vec![]);
        let proved = Term::const_(env.globals["Proved"], vec![]);
        let fake_family = InductiveDecl {
            id: GlobalId(u32::MAX - 1),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![top],
            level: Level::Zero,
            constructors: vec![ConstructorDecl {
                id: GlobalId(u32::MAX),
                args: vec![],
                target_indices: vec![proved.clone()],
                type_: Term::ty(Level::Zero),
                recursive_positions: vec![],
            }],
            former_type: Term::ty(Level::Zero),
        };
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "omega-convoy-index-control",
        );
        cx.ctx.push(Term::ty(Level::Zero));

        let error =
            install_index_refinements(&mut cx, &fake_family, &[], &[proved], &[Term::var(0)], 0, 1)
                .expect_err("decision 5 must remain Type-only");
        match error {
            ElabError::Internal(message) => assert!(
                message.contains(
                    "index refinement: index type is not classified by a Type universe, found Ω0"
                ),
                "the decision-5 Type-only classifier moved: {message:?}"
            ),
            other => panic!("expected the decision-5 Internal error, got {other:?}"),
        }
    }

    #[test]
    fn reindex_classifier_outside_type_or_omega_fails_closed() {
        // Promise class: durable invariant. A malformed intermediate may infer
        // successfully while its classifier is a non-sort; decision 1 must
        // reject it and report that actual classifier.
        // MEASURED: the error names the independently inferred classifier.
        // CLAIMED: decision 1 fails closed outside Type | Omega. THE GAP: the
        // malformed `cur_ty` is injected at the private production seam.
        let env = ElabEnv::new().expect("base environment");
        let idx_ty = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let old_idx = Term::Constructor {
            id: env.globals["Zero"],
            level_args: vec![],
        };
        let new_idx = Term::app(
            Term::Constructor {
                id: env.globals["Suc"],
                level_args: vec![],
            },
            old_idx.clone(),
        );
        // Deliberately malformed as a type: `Zero` infers successfully, but
        // its classifier is `Nat`, neither Type nor Omega.
        let cur_ty = old_idx.clone();
        let ctx = Context::new();
        let actual_classifier = whnf(
            &env.env,
            &ctx,
            &kernel_infer(&env.env, &ctx, &cur_ty).expect("Zero infers at Nat"),
        );
        let error = try_reindex_cast(
            &env.env,
            &ctx,
            &idx_ty,
            &old_idx,
            &new_idx,
            &cur_ty,
            old_idx.clone(),
            Term::Refl(Box::new(old_idx.clone())),
        )
        .expect_err("a non-sort classifier must fail closed");

        match error {
            ElabError::Internal(message) => {
                assert!(message.contains("classified by neither Type nor Omega"));
                assert!(message.contains(&format!("found {actual_classifier:?}")));
            }
            other => panic!("expected the decision-1 Internal error, got {other:?}"),
        }
    }
}

#[cfg(test)]
mod nested_lift_association_tests {
    use crate::{
        error::RecursiveResultSort,
        parser::parse_expr,
        resolve::{resolve_expr_standalone, RExpr},
        ElabEnv,
    };
    use ken_kernel::{
        check as kernel_check, declare_postulate, infer as kernel_infer, Context, KernelError,
        Level, Term,
    };

    use super::{
        discharge_reflexive_recursive_ih_evidence, infer, lift_association_error,
        validate_lift_associations, ElabCtx, ElabError, GlobalId, HashMap, LiftAssociationFailure,
        LiftBinding, Span,
    };

    fn binding(
        evidence_position: usize,
        recursive_result_position: Option<usize>,
        support: Option<GlobalId>,
    ) -> LiftBinding {
        LiftBinding {
            evidence_position,
            recursive_result_position,
            support,
        }
    }

    // LANG-INDEXED-RECURSIVE-IH-DISCHARGE AC-1..AC-6 controls.
    // Promise class: durable invariant. Intended extensions may add indexed
    // families, constructors, and ordinary arguments while preserving the
    // reflexive-only discharge boundary; changing equality orientation,
    // evidence order, or neutral transport must make these controls red.
    fn nat_terms(env: &ElabEnv) -> (Term, Term, Term) {
        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let zero = Term::Constructor {
            id: env.globals["Zero"],
            level_args: vec![],
        };
        let suc_zero = Term::app(
            Term::Constructor {
                id: env.globals["Suc"],
                level_args: vec![],
            },
            zero.clone(),
        );
        (nat, zero, suc_zero)
    }

    #[test]
    fn recursive_ih_reflexive_index_evidence_is_applied_before_source_arguments() {
        // MEASURED: one reflexive leading equality is consumed and the emitted
        // term kernel-infers at the declared source-result type. CLAIMED: a
        // generated reflexive premise is discharged rather than leaked. THE
        // GAP: application-spine ordering is exercised separately below.
        let env = ElabEnv::new().unwrap();
        let (nat, zero, _) = nat_terms(&env);
        let equality = Term::Eq(
            Box::new(nat.clone()),
            Box::new(zero.clone()),
            Box::new(zero),
        );
        let evidence_ty = Term::pi(equality, nat.clone());
        let mut ctx = Context::new();
        ctx.push(evidence_ty.clone());

        let (specialized, specialized_ty) = discharge_reflexive_recursive_ih_evidence(
            &env.env,
            &ctx,
            Term::var(0),
            evidence_ty,
            &nat,
            &Span::new(0, 0),
        )
        .unwrap();

        assert_eq!(specialized_ty, nat);
        assert!(matches!(specialized, Term::App(_, _)));
        assert_eq!(
            kernel_infer(&env.env, &ctx, &specialized).unwrap(),
            specialized_ty
        );
    }

    #[test]
    fn recursive_ih_record_sigma_evidence_is_applied_before_source_arguments() {
        // Promise class: durable invariant.
        // MEASURED: a private direct call specializes one reflexive record Eq
        // premise whose observational evidence is Sigma(Eq, Top), and the
        // resulting application kernel-infers at Nat.
        // CLAIMED: recursive-IH discharge delegates whole generated premises to
        // the same Sigma-aware evidence synthesizer. THE GAP: record fields are
        // closed here; the integration fixture separately reaches a recursive
        // constructor field with an open component equality.
        let mut env = ElabEnv::new().unwrap();
        env.elaborate_decl("data RecursiveIx = RecursiveMkIx Nat Bool")
            .unwrap();
        let (nat, zero, _) = nat_terms(&env);
        let record_ty = Term::IndFormer {
            id: env.globals["RecursiveIx"],
            level_args: vec![],
        };
        let record_value = Term::app(
            Term::app(
                Term::Constructor {
                    id: env.globals["RecursiveMkIx"],
                    level_args: vec![],
                },
                zero,
            ),
            Term::Constructor {
                id: env.globals["True"],
                level_args: vec![],
            },
        );
        let equality = Term::Eq(
            Box::new(record_ty),
            Box::new(record_value.clone()),
            Box::new(record_value),
        );
        let evidence_ty = Term::pi(equality, nat.clone());
        let mut ctx = Context::new();
        ctx.push(evidence_ty.clone());

        let (specialized, specialized_ty) = discharge_reflexive_recursive_ih_evidence(
            &env.env,
            &ctx,
            Term::var(0),
            evidence_ty,
            &nat,
            &Span::new(0, 0),
        )
        .unwrap();

        assert_eq!(specialized_ty, nat);
        assert!(matches!(specialized, Term::App(_, _)));
        assert_eq!(
            kernel_infer(&env.env, &ctx, &specialized).unwrap(),
            specialized_ty
        );
    }

    #[test]
    fn recursive_ih_multiple_index_evidence_is_applied_once_in_telescope_order() {
        // MEASURED: two leading premises produce exactly two nested
        // applications in telescope order. CLAIMED: every generated index
        // premise is discharged once. THE GAP: equal-looking premises do not
        // distinguish semantic index identities; kernel typing and the
        // source-result conversion gate enforce their positional order.
        let env = ElabEnv::new().unwrap();
        let (nat, zero, _) = nat_terms(&env);
        let equality = Term::Eq(
            Box::new(nat.clone()),
            Box::new(zero.clone()),
            Box::new(zero),
        );
        let evidence_ty = Term::pi(equality.clone(), Term::pi(equality, nat.clone()));
        let mut ctx = Context::new();
        ctx.push(evidence_ty.clone());

        let (specialized, specialized_ty) = discharge_reflexive_recursive_ih_evidence(
            &env.env,
            &ctx,
            Term::var(0),
            evidence_ty,
            &nat,
            &Span::new(0, 0),
        )
        .unwrap();

        assert_eq!(specialized_ty, nat);
        let Term::App(first_application, _) = specialized else {
            panic!("second generated equality must be applied");
        };
        assert!(matches!(*first_application, Term::App(_, _)));
    }

    fn assert_recursive_self_call_spine_discharge(recursive_result_position: Option<usize>) {
        let mut env = ElabEnv::new().unwrap();
        let (nat, zero, _) = nat_terms(&env);
        let owner_ty = Term::pi(nat.clone(), Term::pi(nat.clone(), nat.clone()));
        let owner_id = declare_postulate(
            &mut env.env,
            "recursive-ih-spine-owner".to_string(),
            vec![],
            owner_ty,
        )
        .unwrap();
        env.globals.insert("recursiveOwner".into(), owner_id);

        let equality = Term::Eq(
            Box::new(nat.clone()),
            Box::new(zero.clone()),
            Box::new(zero.clone()),
        );
        let evidence_ty = Term::pi(equality, Term::pi(nat.clone(), nat.clone()));
        let span = Span::new(0, 0);
        let expression = RExpr::RApp(
            Box::new(RExpr::RApp(
                Box::new(RExpr::RCon("recursiveOwner".into(), span.clone())),
                Box::new(RExpr::RVar(0, "child".into(), span.clone())),
                span.clone(),
            )),
            Box::new(RExpr::RCon("Zero".into(), span.clone())),
            span.clone(),
        );

        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "recursiveOwner",
        );
        cx.ctx.push(nat.clone());
        cx.ctx.push(evidence_ty.clone());
        cx.hidden_positions.push(1);
        cx.lift_bindings
            .insert(0, binding(1, recursive_result_position, None));

        let (core, ty) = infer(&mut cx, &expression).unwrap();
        assert_eq!(ty, nat);
        assert_eq!(kernel_infer(cx.env, &cx.ctx, &core).unwrap(), ty);
        let Term::App(specialized, ordinary_argument) = core else {
            panic!("ordinary source argument must remain the outer application");
        };
        assert_eq!(*ordinary_argument, zero.clone());
        let Term::App(raw_evidence, _) = *specialized else {
            panic!("generated equality evidence must be applied exactly once first");
        };
        assert_eq!(*raw_evidence, Term::var(0));

        assert!(
            kernel_infer(cx.env, &cx.ctx, &Term::app(Term::var(0), zero),).is_err(),
            "leaving the equality Pi unapplied must reproduce the wrong-position mismatch"
        );
    }

    #[test]
    fn direct_recursive_ih_self_call_discharges_before_ordinary_arguments() {
        // MEASURED: the emitted spine is evidence, generated proof, then the
        // ordinary source argument; the un-specialized mutation is rejected.
        // CLAIMED: source arguments never occupy generated equality slots.
        // THE GAP: this synthetic direct association does not itself traverse
        // a generated nested-All support; the next control uses that metadata.
        assert_recursive_self_call_spine_discharge(None);
    }

    #[test]
    fn nested_all_leaf_self_call_uses_the_same_specialized_contract() {
        // MEASURED: a binding carrying a nested recursive-result position emits
        // the same checked application spine as the direct binding. CLAIMED:
        // direct and nested-All leaves share one specialized contract. THE GAP:
        // support-family construction remains covered by the structural-result
        // integration suite, while this pin isolates leaf presentation.
        assert_recursive_self_call_spine_discharge(Some(2));
    }

    #[test]
    fn recursive_ih_neutral_or_unequal_index_evidence_is_preserved() {
        // MEASURED: unequal evidence remains a Pi, an explicit proof applies,
        // and an invented Refl is kernel-rejected. CLAIMED: discharge never
        // makes neutral or non-reflexive transport definitional. THE GAP: this
        // pin supplies an assumed local proof rather than constructing a J;
        // existing convoy tests cover J/cast construction itself.
        let env = ElabEnv::new().unwrap();
        let (nat, zero, suc_zero) = nat_terms(&env);
        let unequal = Term::Eq(
            Box::new(nat.clone()),
            Box::new(zero.clone()),
            Box::new(suc_zero.clone()),
        );
        let evidence_ty = Term::pi(unequal.clone(), nat.clone());
        let mut ctx = Context::new();
        ctx.push(evidence_ty.clone());

        let original = Term::var(0);
        let result = discharge_reflexive_recursive_ih_evidence(
            &env.env,
            &ctx,
            original.clone(),
            evidence_ty.clone(),
            &nat,
            &Span::new(0, 0),
        )
        .unwrap();
        assert_eq!(result, (original, evidence_ty.clone()));

        let mut explicit_ctx = Context::new();
        explicit_ctx.push(unequal.clone());
        explicit_ctx.push(evidence_ty);
        let explicitly_transported = Term::app(Term::var(0), Term::var(1));
        assert_eq!(
            kernel_infer(&env.env, &explicit_ctx, &explicitly_transported).unwrap(),
            nat,
            "neutral evidence must remain an explicit argument rather than becoming definitional",
        );
        assert!(
            kernel_check(
                &env.env,
                &Context::new(),
                &Term::Refl(Box::new(zero)),
                &unequal,
            )
            .is_err(),
            "Refl at unequal endpoints must remain kernel-rejected"
        );
    }

    #[test]
    fn missing_lift_association_mutation_rejects() {
        let installed = HashMap::new();
        assert_eq!(
            validate_lift_associations(&installed, &[(3, binding(7, Some(9), None))]),
            Err(LiftAssociationFailure::Missing { source: 3 })
        );
    }

    #[test]
    fn swapped_lift_association_mutation_rejects() {
        let installed = HashMap::from([
            (3, binding(8, Some(10), None)),
            (4, binding(7, Some(9), None)),
        ]);
        assert_eq!(
            validate_lift_associations(
                &installed,
                &[
                    (3, binding(7, Some(9), None)),
                    (4, binding(8, Some(10), None)),
                ],
            ),
            Err(LiftAssociationFailure::Swapped {
                first: 3,
                second: 4,
            })
        );
    }

    #[test]
    fn duplicate_lift_association_reverse_injectivity_rejects() {
        let duplicate = binding(7, Some(9), Some(GlobalId(42)));
        let installed = HashMap::from([(3, duplicate), (4, duplicate)]);
        assert_eq!(
            validate_lift_associations(&installed, &[(3, duplicate), (4, duplicate)],),
            Err(LiftAssociationFailure::Duplicate {
                sources: vec![3, 4],
            })
        );
    }

    #[test]
    fn foreign_lift_association_mutation_rejects() {
        let installed = HashMap::from([(3, binding(7, Some(9), Some(GlobalId(99))))]);
        assert_eq!(
            validate_lift_associations(&installed, &[(3, binding(7, Some(9), Some(GlobalId(42))))],),
            Err(LiftAssociationFailure::Foreign {
                source: 3,
                expected: Some(GlobalId(42)),
                actual: Some(GlobalId(99)),
            })
        );
    }

    #[test]
    fn anonymous_association_requires_a_distinct_result_and_rejects_when_deleted() {
        // No carrier/name/constructor special case: `None` support is the
        // anonymous association class.  The exact source/evidence/result
        // triple is admissible, while deleting it is the named fail-closed
        // Missing arm.
        let association = binding(3, Some(5), None);
        let installed = HashMap::from([(1, association)]);
        assert_eq!(
            validate_lift_associations(&installed, &[(1, association)]),
            Ok(())
        );
        assert_eq!(
            validate_lift_associations(&HashMap::new(), &[(1, association)]),
            Err(LiftAssociationFailure::Missing { source: 1 })
        );
    }

    #[test]
    fn validator_failures_map_to_named_structural_diagnostics_with_field_spans() {
        let match_span = Span::new(100, 200);
        let first = Span::new(10, 11);
        let second = Span::new(20, 21);
        let fields = vec![(3, first.clone()), (4, second.clone())];

        assert!(matches!(
            lift_association_error(
                LiftAssociationFailure::Missing { source: 3 },
                &match_span,
                &fields,
            ),
            ElabError::StructuralResultAssociationMissing { match_span: got_match, field_span }
                if got_match == match_span && field_span == first
        ));
        assert!(matches!(
            lift_association_error(
                LiftAssociationFailure::Duplicate { sources: vec![3, 4] },
                &match_span,
                &fields,
            ),
            ElabError::StructuralResultAssociationDuplicate { match_span: got_match, field_spans }
                if got_match == match_span && field_spans == vec![first.clone(), second.clone()]
        ));
        assert!(matches!(
            lift_association_error(
                LiftAssociationFailure::Swapped { first: 3, second: 4 },
                &match_span,
                &fields,
            ),
            ElabError::StructuralResultAssociationSwapped {
                match_span: got_match,
                first_field_span,
                second_field_span,
            } if got_match == match_span && first_field_span == first && second_field_span == second
        ));
        assert!(matches!(
            lift_association_error(
                LiftAssociationFailure::Foreign {
                    source: 4,
                    expected: Some(GlobalId(42)),
                    actual: Some(GlobalId(99)),
                },
                &match_span,
                &fields,
            ),
            ElabError::StructuralResultAssociationForeign {
                match_span: got_match,
                field_span,
                expected_support: Some(GlobalId(42)),
                actual_support: Some(GlobalId(99)),
            } if got_match == match_span && field_span == second
        ));
    }

    #[test]
    fn anonymous_u_to_r_association_selects_r_and_rejects_when_deleted() {
        // Resolve the literal surface selector first. Its spelling is arbitrary:
        // the binding identity is index 0, not a carrier/ctor/field/motive name.
        let parsed = parse_expr("let u : Type = Type in recursive result for u").unwrap();
        let RExpr::RLet(_, _, _, selector, _) = resolve_expr_standalone(&parsed).unwrap() else {
            panic!("expected the source selector under its ordinary let binding");
        };
        let RExpr::RRecursiveResult {
            index,
            binding_span,
            span,
            ..
        } = &*selector
        else {
            panic!("expected resolved recursive-result selector");
        };
        assert_eq!(*index, 0);

        let mut env = ElabEnv::new().unwrap();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "anonymous-u-to-r",
        );
        // Branch telescope: source u, hidden evidence, distinct hidden result r.
        // `None` support is the anonymous class: no carrier/constructor name is
        // available to the association or selector gate.
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        selected.hidden_positions.extend([1, 2]);
        selected.lift_bindings.insert(0, binding(1, Some(2), None));

        let (term, _) = infer(&mut selected, &selector).unwrap();
        assert_eq!(
            term,
            Term::var(0),
            "selector must choose trailing r, not evidence"
        );

        selected.lift_bindings.clear();
        assert!(matches!(
            infer(&mut selected, &selector),
            Err(ElabError::StructuralResultOutOfScope {
                selector_span,
                binding_span: got_binding,
            }) if selector_span == *span && got_binding == *binding_span
        ));
    }

    fn resolved_selector(source: &str) -> RExpr {
        let parsed = parse_expr(source).unwrap();
        let RExpr::RLet(_, _, _, selector, _) = resolve_expr_standalone(&parsed).unwrap() else {
            panic!("expected selector beneath its source binding");
        };
        *selector
    }

    #[test]
    fn selected_result_sort_accepts_type_spelling_and_rejects_inverse() {
        // Promise class: durable invariant. The Type classifier and its
        // inverse spelling must remain a discriminating pair.
        let recursive = resolved_selector("let u : Type = Type in recursive result for u");
        let inverse = resolved_selector("let u : Type = Type in induction hypothesis for u");
        let mut env = ElabEnv::new().unwrap();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "type-result-selector",
        );
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        selected.hidden_positions.extend([1, 2]);
        selected.lift_bindings.insert(0, binding(1, Some(2), None));

        assert!(infer(&mut selected, &recursive).is_ok());
        assert!(matches!(
            infer(&mut selected, &inverse),
            Err(ElabError::RecursiveResultSortMismatch {
                actual_classifier: RecursiveResultSort::Type(_),
                required_spelling: "recursive result for",
                ..
            })
        ));
    }

    #[test]
    fn type_resident_support_does_not_override_omega_result_sort() {
        // Promise class: durable invariant. Support residence never selects
        // the spelling; only the selected result classifier does.
        let induction = resolved_selector("let u : Type = Type in induction hypothesis for u");
        let inverse = resolved_selector("let u : Type = Type in recursive result for u");
        let mut env = ElabEnv::new().unwrap();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "omega-result-selector",
        );
        // u : Type; S : Type; support : S; P : Omega; result : P.
        // The evidence is Type-resident, while only the selected result type P
        // is Omega-classified. The selector must inspect P, never S.
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::var(0));
        selected.ctx.push(Term::Omega(Level::Zero));
        selected.ctx.push(Term::var(0));
        selected.hidden_positions.extend([1, 2, 3, 4]);
        selected
            .lift_bindings
            .insert(0, binding(2, Some(4), Some(GlobalId(42))));

        assert!(infer(&mut selected, &induction).is_ok());
        assert!(matches!(
            infer(&mut selected, &inverse),
            Err(ElabError::RecursiveResultSortMismatch {
                actual_classifier: RecursiveResultSort::Omega(_),
                required_spelling: "induction hypothesis for",
                ..
            })
        ));
    }

    #[test]
    fn proof_relevant_type_result_uses_recursive_result_spelling() {
        // Promise class: durable invariant. Programmer intent cannot override
        // the selected result's Type classifier.
        let selector = resolved_selector("let u : Type = Type in recursive result for u");
        let mut env = ElabEnv::new().unwrap();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "proof-relevant-type-result",
        );
        // u : Type; Witness : Type; result : Witness. The suggestive name and
        // use are irrelevant: Witness is Type-classified.
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::var(0));
        selected.hidden_positions.extend([1, 2]);
        selected.lift_bindings.insert(0, binding(1, Some(2), None));
        assert!(infer(&mut selected, &selector).is_ok());
    }

    #[test]
    fn selector_distinguishes_nonuniverse_classifier_from_kernel_failure() {
        // Promise class: durable invariant. A successful kernel inference with
        // a non-universe classifier is the elaborator's refusal, while a real
        // kernel inference failure on the identical selector remains kernel-
        // attributed. Both selector spellings refuse the former, so neither
        // can become a default.
        let recursive = resolved_selector("let u : Type = Type in recursive result for u");
        let induction = resolved_selector("let u : Type = Type in induction hypothesis for u");
        let mut env = ElabEnv::new().unwrap();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            "unclassifiable-result-selector",
        );
        selected.ctx.push(Term::Type(Level::Zero));
        selected.ctx.push(Term::Type(Level::Zero));
        // This is a well-typed Int value, not a fabricated malformed term.
        // Its successful inference produces a non-universe classifier; putting
        // the value in a result-type slot models the defensive invariant arm.
        selected.ctx.push(Term::IntLit(num_bigint::BigInt::from(0)));
        selected.hidden_positions.extend([1, 2]);
        selected.lift_bindings.insert(0, binding(1, Some(2), None));

        for selector in [&recursive, &induction] {
            let RExpr::RRecursiveResult {
                binding_span: expected_binding_span,
                ..
            } = selector
            else {
                panic!("expected resolved recursive-result selector");
            };
            assert!(matches!(
                infer(&mut selected, selector),
                Err(ElabError::RecursiveResultClassifierNotUniverse {
                    selector_span,
                    binding_span,
                }) if selector_span == *selector.span()
                    && binding_span == *expected_binding_span
            ));
        }

        // Change only the selected slot, retaining the exact resolved selector
        // and association shape. This raw out-of-scope variable makes
        // kernel_infer itself fail before the classifier arm.
        selected.ctx.types[2] = Term::var(99);
        assert!(matches!(
            infer(&mut selected, &recursive),
            Err(ElabError::KernelRejected {
                error: KernelError::VarOutOfScope { index: 100, depth: 3 },
                span,
            }) if span == *recursive.span()
        ));
    }
}

/// `LANG-WITNESS-DIAGNOSTIC-STRICTNESS` H1 control. Requires white-box access
/// to `ElabCtx`/`cx.globals` to reproduce the asymmetric-prune shape
/// `prelude.rs` exercises for real (`PrivateFsOpen` et al.), so it lives here
/// rather than in the `tests/l2_acceptance.rs` integration suite, which can
/// only drive `missing_pattern_witness` indirectly through source elaboration.
#[cfg(test)]
mod missing_pattern_witness_diagnostic_strictness_tests {
    use super::{missing_pattern_witness, ElabCtx};
    use crate::ElabEnv;

    #[test]
    fn globals_pruned_kernel_resolvable_id_degrades_instead_of_panicking() {
        // H1: `cx.globals` is deliberately pruned for some constructors while
        // their kernel declaration survives (the same discipline `prelude.rs`
        // applies to `PrivateFsOpen` et al.) -- reproduced locally so this
        // control does not depend on prelude wiring. The id still resolves
        // via `cx.env.constructor` (the kernel's own `ctor_index`), so
        // `missing_pattern_witness`'s `.expect()` must not fire; only
        // `ctor_name`'s independent fallback degrades the rendered name.
        let mut elab = ElabEnv::new().expect("fresh env");
        elab.elaborate_decl("data T2 = A | B")
            .expect("local data decl elaborates");
        let b_id = elab.globals["B"];
        elab.globals.remove("B");

        let cx = ElabCtx::new(
            &mut elab.env,
            &elab.globals,
            &mut elab.num_values,
            &elab.numeric_env,
            "missing-pattern-witness-globals-prune",
        );
        let witness = missing_pattern_witness(&cx, b_id);

        assert_eq!(
            witness.constructor,
            format!("<ctor_{:?}>", b_id),
            "a constructor id present in the kernel's ctor_index but absent \
             from cx.globals must degrade to ctor_name's own fallback \
             spelling, never panic"
        );
        assert_eq!(
            witness.arity, 0,
            "B's declared arity is still correctly derived from the kernel \
             lookup even though the name lookup missed -- the two lookups \
             are independent, not coupled"
        );
    }
}
