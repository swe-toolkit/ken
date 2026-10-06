//! Bidirectional elaboration to kernel core terms (`39 §5.4`, `§5.7`, `21 §6.3`).
//!
//! V1 additions: `requires`/`ensures` clause processing, obligation holes via
//! `declare_postulate`, honesty guard via `GlobalEnv::trusted_base()`, refinement
//! lowering to carrier, `prove`/`law` declaration elaboration, `old` elaboration.

use std::cell::Cell;
use std::collections::{HashMap, HashSet};

// Raw query aliases are intentionally conspicuous: any active-frame-reachable
// source-derived judgment belongs behind the contextual gateways below.
use ken_kernel::{
    check as kernel_check_raw, convert, convert_type, declare_def, declare_postulate,
    declare_primitive, declare_recursive_group,
    env::PrimReduction,
    inductive::{
        all_support_evidence_positions, method_type, peel_app, peel_pi, recursive_shapes,
        RecursiveArgumentShape,
    },
    infer as kernel_infer_raw,
    subst::{shift, subst0, subst_levels, subst_outer, subst_tel, weaken},
    whnf, ConstructorDecl, Context, Decl, GlobalEnv, GlobalId, InductiveDecl, Level, LevelVar,
    Term,
};

use crate::ast::{
    BinOp, DefKeyword, Fixity, FixityAssoc, LiteralPat, NumLit, RecursiveResultSelector,
};
use crate::classes::{ClassEnv, ClassInfo, ClassKind, InstanceConstraintInfo, InstanceHeadKey, InstanceInfo};
use crate::data;
use crate::error::{ArmDeadCause, ElabError, MissingPatternWitness, RecursiveResultSort, Span};
use crate::numbers::{AddEntry, BinOpEntry, DivEntry, NumericEnv, NumericLitVal};
use crate::standard_operators::StandardOperatorRole;
use crate::resolve::{
    RClassField, RDecl, RDeclKind, RExpr, RInfixOperator, RInstanceConstraint, RMatchArm, RPatKind,
    RPattern, RPropIntro, RRecordField, RRecordPatField, RSpaceDecl, RType, SUGAR_ABSURD,
    SUGAR_AXIOM, SUGAR_ELIM_TRUNC, SUGAR_EQ, SUGAR_J, SUGAR_REFL, SUGAR_TRUNC_INTRO,
};

// ----- obligation model -----

/// Surface refinement facts never enter the kernel's carrier type. An alias
/// identity, not its spelling or its carrier, selects an introduction site.
#[derive(Clone, Default)]
pub(crate) struct RefinementFacts {
    pub refinement_predicates: HashMap<GlobalId, Term>,
    /// Each alias points directly to the predicate-owning refinement identity.
    pub refinement_aliases: HashMap<GlobalId, GlobalId>,
    pub refined_params: HashMap<GlobalId, Vec<Option<Term>>>,
    /// Literal constructor-field clauses survive carrier erasure by the
    /// owning constructor identity and its checked argument position.
    pub constructor_field_predicates: HashMap<GlobalId, Vec<Option<Term>>>,
    /// A record's literal field clauses follow its checked owner identity.
    pub record_field_predicates: HashMap<GlobalId, Vec<Option<Term>>>,
}

impl RefinementFacts {
    pub(crate) fn refinement_root(&self, id: GlobalId) -> Option<GlobalId> {
        if self.refinement_predicates.contains_key(&id) {
            Some(id)
        } else {
            self.refinement_aliases.get(&id).copied()
        }
    }

    pub(crate) fn refinement_predicate(&self, id: GlobalId) -> Option<&Term> {
        self.refinement_root(id)
            .and_then(|root| self.refinement_predicates.get(&root))
    }
}

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
    /// A callee's `requires` premise that is absent in the caller (`22 §2.3`).
    Requires,
    /// From an introduction at a named or site-local refinement (`22 §2.1`).
    RefinementIntroduction,
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
#[derive(Debug, Clone)]
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
    defaulted: Cell<bool>,
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
                None => {
                    self.defaulted.set(true);
                    Level::Zero
                }
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
            Term::Quot(a, r, e) => Term::Quot(
                Box::new(self.zonk_term(a)),
                Box::new(self.zonk_term(r)),
                Box::new(self.zonk_term(e)),
            ),
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

/// A proposition whose proof binder is present at `depth` in `ctx`.
/// Recognition uses kernel conversion against the binder's proposition.
#[derive(Clone)]
struct Assumption {
    prop: Term,
    depth: usize,
}

/// Provenance is written when a binder enters the elaborator context. The
/// stable key is its bottom-relative de Bruijn level, not its current index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MatchBinderOrigin {
    Field,
    Ih,
    Scrutinee,
    ConvoyRebound { original: usize },
    GeneralizedDependent { original: usize },
    GeneratedEquation,
    UserLocal,
}

/// One checked arm's provenance, target and embedding into its enclosing
/// telescope. An absent enclosing level means this binder is new to the arm;
/// it is not permission to reconstruct identity from its type or position.
struct MatchFrame {
    start_level: usize,
    sentinel_region: usize,
    origins: HashMap<usize, MatchBinderOrigin>,
    enclosing_telescope: HashMap<usize, Option<usize>>,
    premise_bindings: HashMap<(usize, usize), usize>,
    convoy_originals: HashSet<usize>,
    refined_target: Option<Term>,
    scrutinee_level: Option<usize>,
    /// Result-position postconditions owned by this match, not its scrutinee.
    result_predicates: Vec<ResultPredicate>,
}

#[inline(never)]
fn boxed_match_frame(
    start_level: usize,
    sentinel_region: usize,
    refined_target: Option<Term>,
    scrutinee_level: Option<usize>,
) -> Box<MatchFrame> {
    Box::new(MatchFrame::new(
        start_level,
        sentinel_region,
        refined_target,
        scrutinee_level,
    ))
}

impl MatchFrame {
    fn new(start_level: usize, sentinel_region: usize, refined_target: Option<Term>, scrutinee_level: Option<usize>) -> Self {
        Self {
            start_level,
            sentinel_region,
            origins: HashMap::new(),
            enclosing_telescope: HashMap::new(),
            premise_bindings: HashMap::new(),
            convoy_originals: HashSet::new(),
            refined_target,
            scrutinee_level,
            result_predicates: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PremiseHoles {
    /// Every elaborator obligation hole is taken into a reported result.
    Reported,
    /// An undischarged elaborator obligation is refused before a hole is
    /// declared, and a discharged one records nothing.
    Refused,
}

/// A postcondition carried through result positions and realized at leaves.
#[derive(Clone)]
struct ResultPredicate {
    /// `lambda result. psi` in the context at `install_depth`.
    predicate: Term,
    install_depth: usize,
    kind: ObligationKind,
    recursive_self: Option<RecursiveSelf>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum RefinementSlot {
    #[default]
    None,
    Outermost,
    Signature { result: bool },
}

#[derive(Clone)]
struct RecursiveSelf {
    id: GlobalId,
    params: usize,
    requires: usize,
    /// The proposition in the parameter, requirement, result telescope.
    psi: Term,
}

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
    assumptions: Vec<Assumption>,
    metas: MetaCtx,
    globals: &'e HashMap<String, GlobalId>,
    /// Contract premise arities, selected only by checked global identity.
    preconditions: HashMap<GlobalId, (usize, usize)>,
    premise_holes: PremiseHoles,
    num_values: &'e mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &'e NumericEnv,
    obligations: Vec<Obligation>,
    obl_counter: u32,
    refinement_facts: &'e RefinementFacts,
    /// Only throwaway type prepasses defer introductions to their admitting
    /// elaboration. The default is to emit or refuse, never silently erase.
    type_introductions: bool,
    /// One-shot permission to erase a source refinement only when its caller
    /// records that predicate. Consumed before visiting any type children.
    refinement_slot: RefinementSlot,
    /// Logical branch equations for refinement obligations; never used as
    /// unchecked evidence in the emitted program.
    path_conditions: Vec<(Term, usize)>,
    /// One-shot result-position channel. Never visible to a subterm check.
    result_predicates: Vec<ResultPredicate>,
    /// The typeclass registry, when available — needed only for `.field`
    /// Σ-record projection (`RExpr::RProj`, `33 §5.2` η). `None` in every
    /// elaboration path that predates class support and never projects
    /// (prove/law/typealias/foreign/temporal/derive/data, recursive views,
    /// the match compiler); wired via `.with_classes` in the view/let path
    /// so a `where C a`-constrained body can project its resolved
    /// dictionary's fields.
    class_env: Option<&'e ClassEnv>,
    /// The standard-operator identities certified by the required-roles check
    /// (`33 §6.1`). `None` on paths that elaborate no user expression body.
    ///
    /// **An occurrence reached with this unset is NOT refused naming the
    /// role, and this comment used to say it was.** Nothing is certified, so
    /// the occurrence takes the non-certified arm in `reduce_resolved_operator`
    /// and is left as an ordinary UNDER-APPLIED application — precisely the
    /// outcome the old sentence offered as the alternative it ruled out. It is
    /// caught downstream by the kernel check. That arm's residual carries the
    /// reachability argument.
    standard_operators: Option<&'e HashMap<StandardOperatorRole, GlobalId>>,
    /// The sink `§6.2` instance search appends its provenance to.
    ///
    /// **Carried beside `class_env`, not inside it.** The registry itself is
    /// borrowed SHARED here -- expression elaboration must not be typed as
    /// able to mutate it, because `§6.2` search is a lookup against a registry
    /// fixed before any body elaborates. The one thing resolution writes is
    /// this append-only log, so it travels as its own `&mut` and the registry
    /// stays immutable.
    provenance: Option<&'e mut Vec<crate::classes::InstanceResolution>>,
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
    /// During a generalized branch-goal check, a premise sentinel resolves to
    /// the temporary Pi binder rather than its original method premise. The
    /// bottom-relative position is stable across deeper local binders.
    /// Generated equality leaves for each active indexed-match branch. The
    /// leaves are stored at `install_depth`; a fresh local binder rebases their
    /// endpoints by later context growth before refining its recorded type.
    /// This is type-at-introduction bookkeeping only: fresh binders are not
    /// added to `var_refinements`.
    active_index_refinements: Vec<ActiveIndexRefinement>,
    /// Bottom-relative `[start, end)` ranges of `cx.ctx` positions bound as
    /// constructor fields by a match arm currently under elaboration,
    /// innermost last. Capability 2 (sibling convoy) must skip these: a field
    /// bound by an ENCLOSING match is not a genuine outer binder, and refining
    /// it re-points it at the inner match's peeled index while its sibling
    /// references keep the enclosing one (`LANG-CONVOY-MATCH-FIELD-
    /// PROVENANCE`). Provenance, not position — a floor keyed on depth alone
    /// would misclassify a genuine outer binder pushed after the enclosing
    /// match's fields (e.g. a `let` between the outer arm and a nested match).
    match_frames: Vec<MatchFrame>,
    next_aux_sentinel_region: usize,
    /// Elaborator-internal method binders are absent from resolved surface
    /// de Bruijn indices. Positions are stable bottom-relative context slots;
    /// `surface_var` skips them when translating an `RVar`.
    hidden_positions: Vec<usize>,
    /// Split pattern columns emit method binders without a `ctx` push. Keep
    /// their source positions so later names still resolve past the split.
    matrix_virtual_surface_positions: Vec<usize>,
    /// Source values paired with kernel-generated lifted evidence. A support
    /// id marks residual `All`; `None` marks a directly consumable motive leaf.
    lift_bindings: HashMap<usize, LiftBinding>,
    /// Branch-local propositional equalities that relate a concrete matched
    /// constructor to the outer scrutinee. A recursive-group call may use one
    /// to transport its concrete indexed result to the refined expected index.
    /// The proof itself is an index-refinement sentinel until the completed
    /// method is wrapped by `finalize_refined_body`.
    result_refinements: Vec<ResultRefinement>,
    /// Premise telescopes owned by dependent-match frames whose arm bodies are
    /// still being checked, outermost first. Immediate kernel queries rebuild a
    /// disposable contextual view containing every active telescope; production
    /// terms remain wrapped only by their owning frame.
    active_index_premise_frames: Vec<ActiveIndexPremiseFrame>,
    /// The stable bottom-relative position of the state binder plus the
    /// declared cell types while elaborating one space-operation continuation.
    space_state: Option<(usize, Vec<Term>)>,
    /// The stable bottom-relative pre-state position plus cell types while
    /// elaborating a block-space contract. Absent for modifier `space proc`.
    space_pre_state: Option<(usize, Vec<Term>)>,
    /// Consumer-only metadata for as-pattern aliases. Matrix traversal records
    /// an alias's type beside the existing per-position occurrence; the leaf
    /// exposes it while elaborating that arm body. Nested matches stack frames.
    pattern_alias_type_frames: Vec<PatternAliasTypeFrame>,
    active_pattern_aliases: Vec<Vec<ActivePatternAlias>>,
    /// Inferred indexed matches may nest inside arm bodies. Only the top
    /// frame's memoized motive types this matrix's root IH columns.
    indexed_match_roots: Vec<IndexedMatchRootFrame>,
    /// A match owns its discovery, single-use first leaf, and literal plans.
    /// Nested matches stack even when they share the indexed-root depth.
    matrix_entries: Vec<MatrixEntry>,
}

#[derive(Clone, Copy)]
enum SurfaceBindingTarget {
    Context(usize),
    Virtual(usize),
}

impl<'e> ElabCtx<'e> {
    fn fresh_aux_sentinel_region(&mut self) -> Result<usize, ElabError> {
        // Method/arm regions are stack-depth keyed for existing motive replay;
        // auxiliary source-field premises live in a disjoint checked namespace.
        if self.match_frames.len() >= AUX_SENTINEL_REGION_BASE
            || self.next_aux_sentinel_region >= AUX_SENTINEL_REGION_END
        {
            return Err(ElabError::Internal("match sentinel-region namespace exhausted".into()));
        }
        let region = self.next_aux_sentinel_region;
        self.next_aux_sentinel_region += 1;
        checked_index_refinement_sentinel(region, 0)?;
        Ok(region)
    }

    fn push_match_binder(&mut self, ty: Term, origin: MatchBinderOrigin) {
        let level = self.ctx.len();
        self.ctx.push(ty);
        if let Some(frame) = self.match_frames.last_mut() {
            let outer = match origin {
                MatchBinderOrigin::ConvoyRebound { original }
                | MatchBinderOrigin::GeneralizedDependent { original } => Some(original),
                MatchBinderOrigin::Field
                | MatchBinderOrigin::Ih
                | MatchBinderOrigin::Scrutinee
                | MatchBinderOrigin::GeneratedEquation
                | MatchBinderOrigin::UserLocal => None,
            };
            frame.origins.insert(level, origin);
            frame.enclosing_telescope.insert(level, outer);
        }
    }

    fn require_constructor_field_ownership(
        &self,
        start_level: usize,
        field_count: usize,
    ) -> Result<(), ElabError> {
        let frame = self.match_frames.last().ok_or_else(|| {
            ElabError::Internal("constructor fields have no owning match frame".into())
        })?;
        if frame.start_level != start_level
            || self.ctx.len().checked_sub(start_level) != Some(field_count)
            || (start_level..self.ctx.len()).any(|level| {
                frame.origins.get(&level) != Some(&MatchBinderOrigin::Field)
                    || frame.enclosing_telescope.get(&level) != Some(&None)
            })
        {
            return Err(ElabError::Internal(
                "constructor fields have no owning match frame".into(),
            ));
        }
        Ok(())
    }

    fn scoped_match_premises(&self) -> Result<HashMap<(usize, usize), usize>, ElabError> {
        let mut aliases = HashMap::new();
        for frame in &self.match_frames {
            for (&key, &level) in &frame.premise_bindings {
                if frame.origins.get(&level) != Some(&MatchBinderOrigin::GeneratedEquation)
                    || !frame.enclosing_telescope.contains_key(&level)
                {
                    return Err(ElabError::Internal(
                        "generalized premise has no recorded equation origin".into(),
                    ));
                }
                aliases.insert(key, level);
            }
        }
        Ok(aliases)
    }

    fn match_binder_origin(&self, level: usize) -> Result<Option<MatchBinderOrigin>, ElabError> {
        // The innermost frame whose start precedes this level owns the binder.
        // A popped local can leave an old entry at the same level in a parent;
        // never use it to fill a missing entry in the actual owner.
        for frame in self.match_frames.iter().rev() {
            if level < frame.start_level {
                continue;
            }
            let origin = *frame.origins.get(&level).ok_or_else(|| {
                ElabError::Internal(format!("match binder level {level} has no recorded origin"))
            })?;
            if !frame.enclosing_telescope.contains_key(&level) {
                return Err(ElabError::Internal(format!(
                    "match binder level {level} lost its enclosing telescope map"
                )));
            }
            return Ok(Some(origin));
        }
        Ok(None)
    }

    fn new(
        env: &'e mut GlobalEnv,
        globals: &'e HashMap<String, GlobalId>,
        num_values: &'e mut HashMap<GlobalId, NumericLitVal>,
        numeric_env: &'e NumericEnv,
        refinement_facts: &'e RefinementFacts,
        owner_label: impl Into<String>,
    ) -> Self {
        Self {
            env,
            owner_label: owner_label.into(),
            recursive_group: HashSet::new(),
            ctx: Context::new(),
            assumptions: Vec::new(),
            metas: MetaCtx::default(),
            globals,
            preconditions: HashMap::new(),
            premise_holes: PremiseHoles::Refused,
            num_values,
            numeric_env,
            obligations: Vec::new(),
            obl_counter: 0,
            refinement_facts,
            type_introductions: true,
            refinement_slot: RefinementSlot::None,
            path_conditions: Vec::new(),
            result_predicates: Vec::new(),
            class_env: None,
            standard_operators: None,
            provenance: None,
            local_dicts: HashMap::new(),
            var_refinements: HashMap::new(),
            active_index_refinements: Vec::new(),
            match_frames: Vec::new(),
            next_aux_sentinel_region: AUX_SENTINEL_REGION_BASE,
            hidden_positions: Vec::new(),
            matrix_virtual_surface_positions: Vec::new(),
            lift_bindings: HashMap::new(),
            result_refinements: Vec::new(),
            active_index_premise_frames: Vec::new(),
            space_state: None,
            space_pre_state: None,
            pattern_alias_type_frames: Vec::new(),
            active_pattern_aliases: Vec::new(),
            indexed_match_roots: Vec::new(),
            matrix_entries: Vec::new(),
        }
    }

    /// A pre-pass whose type result is discarded or re-derived by the admitting
    /// elaboration of the same source type. The admitting elaboration emits.
    fn deferring_type_introductions(mut self) -> Self {
        self.type_introductions = false;
        self
    }

    fn surface_binding_target(&self, index: usize) -> Option<SurfaceBindingTarget> {
        let mut remaining = index;
        for position in (0..self.ctx.len()).rev() {
            // A split occupies a hidden `ctx` position at `split_depth`.
            // Its virtual slot supplies the surface name at that position.
            for (slot, _) in self.matrix_virtual_surface_positions.iter().enumerate()
                .filter(|(_, &split)| split == position + 1)
            {
                if remaining == 0 {
                    return Some(SurfaceBindingTarget::Virtual(slot));
                }
                remaining -= 1;
            }
            if self.hidden_positions.contains(&position) {
                continue;
            }
            if remaining == 0 {
                return Some(SurfaceBindingTarget::Context(position));
            }
            remaining -= 1;
        }
        // A split before the first real binder has position zero, too.
        for (slot, _) in self.matrix_virtual_surface_positions.iter().enumerate()
            .filter(|(_, &split)| split == 0)
        {
            if remaining == 0 {
                return Some(SurfaceBindingTarget::Virtual(slot));
            }
            remaining -= 1;
        }
        None
    }

    fn surface_binding_position(&self, index: usize) -> Option<usize> {
        match self.surface_binding_target(index)? {
            SurfaceBindingTarget::Context(position) => Some(position),
            SurfaceBindingTarget::Virtual(_) => None,
        }
    }

    fn virtual_surface_binding_slot(&self, index: usize) -> Option<usize> {
        match self.surface_binding_target(index)? {
            SurfaceBindingTarget::Context(_) => None,
            SurfaceBindingTarget::Virtual(slot) => Some(slot),
        }
    }

    fn surface_var(&self, index: usize) -> Option<(usize, usize)> {
        let position = self.surface_binding_position(index)?;
        Some((position, self.ctx.len() - 1 - position))
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
        let position = self.surface_binding_position(index)?;
        let result_position = self.lift_bindings.get(&position)?.recursive_result_position?;
        self.binding_term(result_position)
    }

    /// Wire the class registry AND the certified standard-operator identities
    /// together. **One call on purpose**: completing a standard operator needs
    /// the identity to recognise the occurrence and the class registry to
    /// resolve its dictionary, so threading them apart is how one of them ends
    /// up missing on a path nobody enumerated. Taking both makes every call
    /// site a compile error until it supplies both -- the audit is bounded by
    /// the compiler, not by a grep.
    /// **All three together, on purpose.** Taking them in one call makes
    /// every call site a compile error until it supplies all of them, so the
    /// audit is bounded by the compiler rather than by a grep. The provenance
    /// sink belongs in the bundle for the same reason the other two do:
    /// completion needs the identity to recognise an occurrence, the registry
    /// to resolve its dictionary, and the sink to record what it resolved.
    fn with_classes(
        mut self,
        class_env: &'e ClassEnv,
        provenance: &'e mut Vec<crate::classes::InstanceResolution>,
        standard_operators: &'e HashMap<StandardOperatorRole, GlobalId>,
    ) -> Self {
        self.class_env = Some(class_env);
        self.provenance = Some(provenance);
        self.standard_operators = Some(standard_operators);
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

    fn with_preconditions(
        mut self,
        preconditions: &HashMap<GlobalId, (usize, usize)>,
        premise_holes: PremiseHoles,
    ) -> Self {
        self.preconditions.clone_from(preconditions);
        self.premise_holes = premise_holes;
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

#[derive(Clone)]
struct MatrixAliasType {
    name: String,
    ty: Term,
    install_depth: usize,
}

#[derive(Clone)]
struct MatrixVirtualAlias {
    slot: usize,
    name: String,
    occurrence: Term,
    ty: Term,
    install_depth: usize,
}

#[derive(Clone, Debug)]
struct OrBinderTypeMismatch {
    name: String,
    span: Span,
}

fn or_binder_type_error(mismatch: OrBinderTypeMismatch) -> ElabError {
    ElabError::TypeMismatch {
        span: mismatch.span,
        reason: format!(
            "or-pattern binder '{}' must have definitionally equal types in the common pre-branch context; use separate arms",
            mismatch.name
        ),
    }
}

struct PatternAliasTypeFrame {
    aliases: HashMap<(usize, usize), MatrixAliasType>,
    /// Only slots originating inside `RPatKind::Or` require cross-alternative
    /// common-context type equality. Other aliases may be revisited after a
    /// wildcard row expands and retain their existing last-visit behavior.
    or_slots: HashSet<(usize, usize)>,
    /// The context depth at the matrix column where each or-slot's residual
    /// row was duplicated. This is the exact common pre-branch context.
    or_common_depths: HashMap<(usize, usize), usize>,
    type_mismatch: Option<OrBinderTypeMismatch>,
    /// Occurrence-backed variables and anonymous wildcards are lexical aliases,
    /// not core binders. Their per-leaf core positions must be hidden while the
    /// shared arm body is elaborated.
    hidden_slots: HashSet<(usize, usize)>,
}

#[derive(Clone)]
struct ActivePatternAlias {
    slot: usize,
    virtual_slot: Option<usize>,
    name: String,
    occurrence: Term,
    occurrence_depth: usize,
    ty: Term,
    install_depth: usize,
}

#[derive(Clone, Debug)]
struct ActiveIndexPremiseFrame {
    sentinel_region: usize,
    premise_domains: Vec<Term>,
    install_depth: usize,
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

/// Disposable kernel-query context containing the logically live premise
/// binders that owner-local production terms still encode as sentinels.
#[derive(Clone)]
struct ActivePremiseKernelView {
    context: Context,
    embedding: ActivePremiseEmbedding,
}

/// Total bottom-relative correspondence between the live elaborator context
/// and its premise-expanded kernel view.
#[derive(Clone, Debug)]
struct ActivePremiseEmbedding {
    original_len: usize,
    expanded_len: usize,
    original_to_expanded: Vec<usize>,
    expanded_sources: Vec<ExpandedBindingSource>,
    premise_to_expanded: HashMap<(usize, usize), usize>,
    premise_install_depth: HashMap<(usize, usize), usize>,
    scoped_premise_aliases: HashMap<(usize, usize), usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ExpandedBindingSource {
    Original(usize),
    Premise {
        sentinel_region: usize,
        premise_slot: usize,
    },
}

/// Keep malformed view authority distinct from an ordinary kernel rejection so
/// consumers can preserve their existing diagnostics without hiding the former.
#[derive(Debug)]
enum CurrentKernelQueryError {
    View(ElabError),
    Kernel(ken_kernel::KernelError),
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

/// `RVarTy` arm of `elab_type`, split out per `check`'s FRAME BUDGET note:
/// `elab_type` recurses on every nested type, so the alias temporary must
/// not sit in its frame.
#[inline(never)]
fn elab_type_named_variable(
    cx: &mut ElabCtx,
    index: usize,
    name: &str,
    span: &Span,
) -> Result<Term, ElabError> {
    if let Some((term, _)) = infer_virtual_pattern_alias(cx, index, name, span)? {
        Ok(term)
    } else {
        cx.surface_var(index)
            .map(|(_, actual_index)| Term::var(actual_index))
            .ok_or_else(|| {
                ElabError::Internal(format!(
                    "type variable '{}' at {}-{} is out of range",
                    name, span.start, span.end
                ))
            })
    }
}

fn elab_type_in_slot(
    cx: &mut ElabCtx<'_>, ty: &RType, slot: RefinementSlot,
) -> Result<Term, ElabError> {
    cx.refinement_slot = slot;
    let result = elab_type(cx, ty);
    cx.refinement_slot = RefinementSlot::None;
    result
}

fn elab_type(cx: &mut ElabCtx, ty: &RType) -> Result<Term, ElabError> {
    let slot = std::mem::take(&mut cx.refinement_slot);
    match ty {
        RType::RUniv(None, _) => {
            let l = cx.metas.fresh();
            Ok(Term::ty(l))
        }
        RType::RUniv(Some(n), _) => Ok(Term::ty(level_from_nat(*n))),

        RType::RCheckedGlobal { id, .. } => {
            let id = *id;
            if cx.env.constructor(id).is_some() {
                Ok(Term::Constructor { id, level_args: vec![] })
            } else if cx.env.inductive(id).is_some() {
                Ok(Term::IndFormer { id, level_args: vec![] })
            } else {
                Ok(Term::const_(id, vec![]))
            }
        }
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

        RType::RApp(f, a, span) => {
            let f_k = elab_type(cx, f)?;
            let a_k = elab_type(cx, a)?;
            introduce_type_position_argument(cx, &f_k, &a_k, span)?;
            Ok(Term::app(f_k, a_k))
        }

        RType::RVarTy(index, name, span) => elab_type_named_variable(cx, *index, name, span),
        RType::RPatternAliasTy(slot, name, _) => {
            infer_active_pattern_alias(cx, *slot, name).map(|(term, _)| term)
        }

        RType::RArr(a, b, _) | RType::REffectArr(a, _, b, _) => {
            let (domain_slot, codomain_slot) = match slot {
                RefinementSlot::Signature { .. } => (RefinementSlot::Outermost, slot),
                _ => (RefinementSlot::None, RefinementSlot::None),
            };
            let a_core = elab_type_in_slot(cx, a, domain_slot)?;
            let b_core = elab_type_in_slot(cx, b, codomain_slot)?;
            Ok(Term::pi(a_core, weaken(&b_core, 1)))
        }

        RType::RPi(_, a, b, _) => {
            let (domain_slot, codomain_slot) = match slot {
                RefinementSlot::Signature { .. } => (RefinementSlot::Outermost, slot),
                _ => (RefinementSlot::None, RefinementSlot::None),
            };
            let a_core = elab_type_in_slot(cx, a, domain_slot)?;
            cx.push_match_binder(a_core.clone(), MatchBinderOrigin::UserLocal);
            let b_result = elab_type_in_slot(cx, b, codomain_slot);
            cx.ctx.pop();
            Ok(Term::pi(a_core, b_result?))
        }

        RType::RSigma(_, a, b, _) => {
            let a_core = elab_type(cx, a)?;
            cx.push_match_binder(a_core.clone(), MatchBinderOrigin::UserLocal);
            let b_core = elab_type(cx, b)?;
            cx.ctx.pop();
            Ok(Term::sigma(a_core, b_core))
        }

        // Refinement lowers to the carrier type (`21 §6.3`): `{x:A|φ}` → `A`.
        // The predicate φ is tracked separately; obligation emitted at introduction.
        RType::RRefine(_, carrier, _phi, span) => match slot {
            RefinementSlot::Outermost | RefinementSlot::Signature { result: true } => {
                elab_type(cx, carrier)
            }
            _ => Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "a refinement nested inside a type is not supported yet: only a binder's, field's or result's own annotation may be refined".into(),
            }),
        },

        // `‖A‖` — propositional truncation formation in annotation position
        // (`16 §6`, LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES D1). Raw structural
        // build to `Term::Trunc`, mirroring the `Term::Eq` arm above; the
        // kernel's own `Term::Trunc` inference (`‖A‖ : Ω_l` for `A : Type l`)
        // validates it when the surrounding declaration is checked.
        RType::RTrunc(inner, _) => Ok(Term::Trunc(Box::new(elab_type(cx, inner)?))),

        // `d.Query` in type position (`33 §6.3`, `58b §1`). Delegates to the
        // SAME `infer_proj` the expression-position `RExpr::RProj` uses, so the
        // name-to-index map, the owner-identity rule (the base's type is read
        // AS ELABORATED, never `whnf`'d, or a transparent owner unfolds into a
        // raw Sigma chain and the owner identity is lost) and BOTH rejections
        // are shared rather than reimplemented here. A second copy of that
        // lookup is the thing most likely to drift out of agreement with the
        // class's field list.
        //
        // We keep the projected VALUE and discard the field's own type: in type
        // position `d.Query` denotes the field's value, which for a field
        // declared `Query : Type` is itself a type. A field whose value is not
        // a type (`d.member`) is refused downstream by the kernel, where the
        // resulting term is checked against the sort its use demands -- this
        // arm deliberately mints no third rejection of its own, because `AC-2`
        // names two and a third with no reaching fixture would be an
        // unexercised arm.
        RType::RProj(base, field, span) => {
            let (value, _field_type) = infer_proj(cx, base, field, span)?;
            Ok(value)
        }
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
            let ty_core = elab_type_in_slot(cx, ty, RefinementSlot::Outermost)?;
            let predicate = literal_result_predicate(cx, ty, &ty_core)?;
            let rhs_core = if let Some(predicate) = predicate {
                check_result_position(cx, rhs, &ty_core, span, &[predicate])?
            } else {
                check(cx, rhs, &ty_core, span)?
            };
            Ok((rhs_core, ty_core))
        }
        None => infer(cx, rhs),
    }
}

/// Refine a fresh `let` binder by every enclosing indexed-match branch. Each
/// leaf was projected where its branch fields were installed, so weaken both
/// endpoints by the intervening context growth before applying the same
/// `scrutinee -> target` substitution as `refine_branch_goal`. Apply it to the
/// RHS core and its recorded type in lockstep: the fresh binder itself is not
/// in scope in either, and it is not added to `var_refinements`.
#[inline(never)]
fn refine_let_rhs(
    cx: &ElabCtx,
    rhs_core: &mut Term,
    rhs_ty: &mut Term,
) -> Result<(), ElabError> {
    for frame in &cx.active_index_refinements {
        let growth = cx
            .ctx
            .len()
            .checked_sub(frame.install_depth)
            .ok_or_else(|| {
                ElabError::Internal("let type refinement escaped its branch context".into())
            })? as i64;
        for leaf in &frame.leaves {
            let scrutinee = weaken(&leaf.scrutinee, growth);
            let target = weaken(&leaf.target, growth);
            let candidate_ty = subst_term_generalize(rhs_ty, &scrutinee, &target);
            if candidate_ty != *rhs_ty {
                *rhs_core = subst_term_generalize(rhs_core, &scrutinee, &target);
                *rhs_ty = candidate_ty;
            }
        }
    }
    Ok(())
}

/// Spec 22 §3: relate a let binder to its RHS for obligations inside the body.
/// Both the RHS and its type live before the binder, so weaken them once before
/// forming the equation with the new binder (`Var(0)`). Proof terms classify
/// by Ω and need no equation under definitional proof irrelevance. An unknown
/// classifier leaves the context too weak, never too strong. This logical
/// hypothesis does not enter the emitted `Term::Let`.
#[inline(never)]
fn let_equation(cx: &ElabCtx<'_>, rhs_ty: &Term, rhs_core: &Term) -> Option<Term> {
    let ty = cx.metas.zonk_term(rhs_ty);
    let informative = kernel_infer_current(cx, &ty)
        .ok()
        .is_some_and(|sort| matches!(whnf(cx.env, &cx.ctx, &sort), Term::Type(_)));
    informative.then(|| {
        Term::Eq(
            Box::new(weaken(&ty, 1)),
            Box::new(Term::var(0)),
            Box::new(weaken(&cx.metas.zonk_term(rhs_core), 1)),
        )
    })
}

/// Check a local `let` outside `check`'s always-paid recursive frame. The
/// helper remains live only for an actual `RLet`; unrelated deep match trees
/// pay the original single-call dispatch frame.
#[inline(never)]
fn check_let(
    cx: &mut ElabCtx,
    ty_opt: &Option<RType>,
    rhs: &RExpr,
    body: &RExpr,
    expected: &Term,
    span: &Span,
    result_predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    let (mut rhs_core, mut rhs_ty) = prepare_let_rhs(cx, ty_opt, rhs, span)?;
    refine_let_rhs(cx, &mut rhs_core, &mut rhs_ty)?;
    let equation = let_equation(cx, &rhs_ty, &rhs_core);
    cx.push_match_binder(rhs_ty.clone(), MatchBinderOrigin::UserLocal);
    let path_base = cx.path_conditions.len();
    if let Some(equation) = equation {
        cx.path_conditions.push((equation, cx.ctx.len()));
    }
    let body_result = check_result_position(cx, body, &weaken(expected, 1), span, result_predicates);
    cx.path_conditions.truncate(path_base);
    cx.ctx.pop();
    let body_core = body_result?;
    Ok(Term::Let {
        ty: Box::new(rhs_ty),
        val: Box::new(rhs_core),
        body: Box::new(body_core),
    })
}

fn elaborate_if_condition(cx: &mut ElabCtx<'_>, condition: &RExpr) -> Result<Term, ElabError> {
    let (condition_core, _) = infer(cx, condition)?;
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    match kernel_check_current(cx, &condition_core, &bool_ty) {
        Ok(()) => {}
        Err(CurrentKernelQueryError::View(error)) => return Err(error),
        Err(CurrentKernelQueryError::Kernel(_)) => {
            return Err(ElabError::IfConditionNotBool {
                span: condition.span().clone(),
            })
        }
    }
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
    let classifier = kernel_infer_current(cx, result_ty).map_err(|error| match error {
        CurrentKernelQueryError::View(error) => error,
        CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
            error,
            span: span.clone(),
        },
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
    debug_assert!(bool_decl.indices.is_empty());
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
    result_predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    let condition_core = elaborate_if_condition(cx, condition)?;
    let base = cx.path_conditions.len();
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    cx.path_conditions.push((Term::Eq(
        Box::new(bool_ty.clone()), Box::new(condition_core.clone()),
        Box::new(Term::constructor(cx.numeric_env.bool_true_id, vec![])),
    ), cx.ctx.len()));
    let then_result = check_result_position(cx, then_branch, expected, then_branch.span(), result_predicates);
    cx.path_conditions.truncate(base);
    let then_core = then_result?;
    cx.path_conditions.push((Term::Eq(
        Box::new(bool_ty), Box::new(condition_core.clone()),
        Box::new(Term::constructor(cx.numeric_env.bool_false_id, vec![])),
    ), cx.ctx.len()));
    let else_result = check_result_position(cx, else_branch, expected, else_branch.span(), result_predicates);
    cx.path_conditions.truncate(base);
    let else_core = else_result?;
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
    let base = cx.path_conditions.len();
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    cx.path_conditions.push((Term::Eq(
        Box::new(bool_ty.clone()), Box::new(condition_core.clone()),
        Box::new(Term::constructor(cx.numeric_env.bool_true_id, vec![])),
    ), cx.ctx.len()));
    let then_result = infer(cx, then_branch);
    cx.path_conditions.truncate(base);
    let (then_core, result_ty) = then_result?;
    cx.path_conditions.push((Term::Eq(
        Box::new(bool_ty), Box::new(condition_core.clone()),
        Box::new(Term::constructor(cx.numeric_env.bool_false_id, vec![])),
    ), cx.ctx.len()));
    let else_result = check(cx, else_branch, &result_ty, else_branch.span());
    cx.path_conditions.truncate(base);
    let else_core = else_result?;
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
    let projection =
        class_env
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
            let value = check(cx, expr, &field_expected, expr.span())?;
            if let Some(template) = cx.refinement_facts
                .record_field_predicates.get(&owner_id)
                .and_then(|predicates| predicates.get(index))
                .and_then(Option::as_ref)
                .cloned()
            {
                let predicate = subst_outer(&template, args.len(), &args, 0);
                emit_refinement_predicate(cx, predicate, value.clone(), expr.span())?;
            }
            value
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
#[cfg(test)]
fn check_variable_with_index_views(
    cx: &mut ElabCtx,
    surface_index: usize,
    expected: &Term,
) -> Result<Term, ElabError> {
    check_variable_with_index_views_named(cx, surface_index, "", &Span::zero(), expected)
}

#[inline(never)]
fn check_variable_with_index_views_named(
    cx: &mut ElabCtx,
    surface_index: usize,
    name: &str,
    span: &Span,
    expected: &Term,
) -> Result<Term, ElabError> {
    if let Some((term, ty)) = infer_virtual_pattern_alias(cx, surface_index, name, span)? {
        unify_types(&mut cx.metas, expected, &ty);
        return Ok(term);
    }
    let (position, actual_index) = cx
        .surface_var(surface_index)
        .ok_or_else(|| ElabError::Internal(format!("Var({surface_index}) out of range")))?;
    let stored_ty = cx
        .ctx
        .lookup(actual_index)
        .ok_or_else(|| ElabError::Internal(format!("Var({surface_index}) out of range")))?;
    let local_ty = weaken(stored_ty, (actual_index as i64) + 1);
    let local_term = Term::var(actual_index);

    let Some((raw_refined_term, raw_refined_ty, install_depth)) = cx.var_refinements.get(&position)
    else {
        unify_types(&mut cx.metas, expected, &local_ty);
        return Ok(local_term);
    };
    let growth = cx.ctx.len().checked_sub(*install_depth).ok_or_else(|| {
        ElabError::Internal("index-refined variable escaped its branch context".into())
    })? as i64;
    let refined_term = weaken(raw_refined_term, growth);
    let (refined_term, refined_ty) = scoped_premise_binding(cx, &refined_term)?
        .unwrap_or_else(|| (refined_term, weaken(raw_refined_ty, growth)));
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

/// Resolve a generalized premise to ONE ambient binder identity. Inference
/// of a nested-match scrutinee and checking of a body variable must return
/// the same binder and its type, never the original sentinel with the new
/// binder's type (or the new binder with the original premise's type).
fn scoped_premise_binding(
    cx: &ElabCtx<'_>,
    term: &Term,
) -> Result<Option<(Term, Term)>, ElabError> {
    let Term::Var(index) = term else {
        return Ok(None);
    };
    for ((region, slot), position) in cx.scoped_match_premises()?.iter() {
        let frame = cx
            .active_index_premise_frames
            .iter()
            .find(|frame| frame.sentinel_region == *region && *slot < frame.premise_domains.len())
            .ok_or_else(|| {
                ElabError::Internal("generalized premise lost its installing frame".into())
            })?;
        let growth = cx.ctx.len().checked_sub(frame.install_depth).ok_or_else(|| {
            ElabError::Internal("generalized premise escaped its installing context".into())
        })?;
        let effective_slot = slot.checked_add(growth).ok_or_else(|| {
            ElabError::Internal("generalized premise slot overflowed at the nested match".into())
        })?;
        if *index == checked_index_refinement_sentinel(*region, effective_slot)? {
            let binding = cx.binding_term(*position).ok_or_else(|| {
                ElabError::Internal("generalized premise binder escaped its checking scope".into())
            })?;
            return Ok(Some(binding));
        }
    }
    Ok(None)
}

#[inline(never)]
fn check_inferred_without_group_transport(
    cx: &mut ElabCtx,
    expr: &RExpr,
    expected: &Term,
) -> Result<Term, ElabError> {
    let (core, inferred_ty) = infer(cx, expr)?;
    unify_types(&mut cx.metas, expected, &inferred_ty);
    emit_refinement_introduction(cx, expected, &inferred_ty, core, expr.span(), None)
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
        return emit_refinement_introduction(
            cx, expected, &inferred_ty, transported, expr.span(), None,
        );
    }
    unify_types(&mut cx.metas, expected, &inferred_ty);
    emit_refinement_introduction(cx, expected, &inferred_ty, core, expr.span(), None)
}

#[inline(never)]
fn check_refined_index_variable(
    cx: &mut ElabCtx<'_>,
    index: usize,
    name: &str,
    span: &Span,
    expected: &Term,
) -> Result<Term, ElabError> {
    let core = check_variable_with_index_views_named(cx, index, name, span, expected)?;
    let inferred = kernel_infer_current(cx, &core).map_err(|error| match error {
        CurrentKernelQueryError::View(error) => error,
        CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
            error,
            span: span.clone(),
        },
    })?;
    emit_refinement_introduction(cx, expected, &inferred, core, span, None)
}

#[inline(never)]
fn check_refined_number_literal(
    cx: &mut ElabCtx<'_>,
    literal: &NumLit,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let core = elab_num_lit_checked(cx, literal, expected, span)?;
    let inferred = Term::const_(cx.numeric_env.int_id, vec![]);
    emit_refinement_introduction(cx, expected, &inferred, core, span, None)
}

#[inline(never)]
fn check_refined_string_literal(
    cx: &mut ElabCtx<'_>,
    literal: &str,
    expected: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let (core, inferred) = elab_str_lit(cx, literal, Some(expected), span)?;
    emit_refinement_introduction(cx, expected, &inferred, core, span, None)
}

/// An empty channel must not introduce an extra frame on every checked
/// expression in a nested match. The nonempty route is cold and isolated.
#[inline(always)]
fn check_result_position(
    cx: &mut ElabCtx<'_>, expr: &RExpr, expected: &Term, span: &Span,
    predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    if predicates.is_empty() {
        check(cx, expr, expected, span)
    } else {
        check_result_position_with_predicates(cx, expr, expected, span, predicates)
    }
}

#[inline(never)]
fn check_result_position_with_predicates(
    cx: &mut ElabCtx<'_>, expr: &RExpr, expected: &Term, span: &Span,
    predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    if !cx.result_predicates.is_empty() {
        return Err(ElabError::Internal("result predicate escaped its owning check".into()));
    }
    cx.result_predicates.extend_from_slice(predicates);
    let result = check(cx, expr, expected, span);
    if !cx.result_predicates.is_empty() {
        cx.result_predicates.clear();
        return Err(ElabError::Internal("result predicate was not consumed".into()));
    }
    result
}

/// An arm body belongs to the innermost match frame. Synthetic/cloned arms
/// have exactly the same obligation channel as that frame's original arms.
/// Ordinary arms go directly to `check` without another repeating frame.
#[inline(always)]
fn check_match_arm_result(
    cx: &mut ElabCtx<'_>, arm: &RMatchArm, expected: &Term, span: &Span,
) -> Result<Term, ElabError> {
    if cx.match_frames.last().is_none_or(|frame| frame.result_predicates.is_empty()) {
        check(cx, &arm.body, expected, span)
    } else {
        check_match_arm_result_with_predicates(cx, arm, expected, span)
    }
}

#[inline(never)]
fn check_match_arm_result_with_predicates(
    cx: &mut ElabCtx<'_>, arm: &RMatchArm, expected: &Term, span: &Span,
) -> Result<Term, ElabError> {
    let predicates = cx.match_frames.last()
        .expect("result match arm has an owning frame")
        .result_predicates.clone();
    check_result_position_with_predicates(cx, &arm.body, expected, span, &predicates)
}

/// The only entry that forwards a pending predicate to a result-position form.
#[inline(never)]
fn check_with_result_predicates(
    cx: &mut ElabCtx<'_>, expr: &RExpr, expected: &Term, span: &Span,
) -> Result<Term, ElabError> {
    let predicates = std::mem::take(&mut cx.result_predicates);
    match expr {
        RExpr::RIf { condition, then_branch, else_branch, span } =>
            check_if(cx, condition, then_branch, else_branch, expected, span, &predicates),
        RExpr::RLet(_, ty, rhs, body, span) =>
            check_let(cx, ty, rhs, body, expected, span, &predicates),
        RExpr::RMatch { scrut, equation, arms, span } =>
            check_match_result(cx, scrut, equation.as_deref(), arms, expected, span, &predicates),
        _ => {
            let core = check(cx, expr, expected, span)?;
            for predicate in &predicates {
                emit_result_predicate(cx, predicate, &core, expected, span)?;
            }
            Ok(core)
        }
    }
}

fn check(cx: &mut ElabCtx, expr: &RExpr, expected: &Term, _span: &Span) -> Result<Term, ElabError> {
    if !cx.result_predicates.is_empty() {
        return check_with_result_predicates(cx, expr, expected, _span);
    }
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
        RExpr::RVar(index, name, span) if !cx.var_refinements.is_empty() => {
            check_refined_index_variable(cx, *index, name, span, expected)
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
        } => check_if(cx, condition, then_branch, else_branch, expected, span, &[]),
        RExpr::RNumLit(lit, num_span) => {
            check_refined_number_literal(cx, lit, expected, num_span)
        }
        RExpr::RStr(s, span) => check_refined_string_literal(cx, s, expected, span),
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
                    let domain = *dom;
                    let position = cx.ctx.len();
                    let is_proposition = kernel_infer_current(cx, &domain)
                        .ok()
                        .is_some_and(|sort| matches!(whnf(cx.env, &cx.ctx, &sort), Term::Omega(_)));
                    let assumption_base = cx.assumptions.len();
                    if is_proposition {
                        cx.assumptions.push(Assumption {
                            prop: domain.clone(),
                            depth: position,
                        });
                    }
                    cx.push_match_binder(domain.clone(), MatchBinderOrigin::UserLocal);
                    let body_result = check(cx, body, &cod, lam_span);
                    cx.ctx.pop();
                    cx.assumptions.truncate(assumption_base);
                    body_result.map(|body_core| Term::lam(domain, body_core))
                }
                _ => Err(ElabError::LambdaVsNonFunction {
                    span: lam_span.clone(),
                }),
            }
        }
        RExpr::RLet(_name, ty_opt, rhs, body, span) => {
            check_let(cx, ty_opt, rhs, body, expected, span, &[])
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
        } => check_match_result(cx, scrut, equation.as_deref(), arms, expected, span, &[]),
        _ if cx.recursive_group.is_empty() => {
            check_inferred_without_group_transport(cx, expr, expected)
        }
        _ => check_inferred_with_group_transport(cx, expr, expected),
    }
}

/// Match setup and motive construction run with no result predicate. Only
/// checked arm bodies receive it, including seeded general-matrix leaves.
#[inline(never)]
fn check_match_result(
    cx: &mut ElabCtx<'_>, scrut: &RExpr, equation: Option<&str>,
    arms: &[RMatchArm], expected: &Term, span: &Span,
    predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    {
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
                RPatKind::Ctor(_, subs) | RPatKind::CheckedCtor(_, _, subs) => subs
                    .iter()
                    .all(|s| matches!(s.kind, RPatKind::Var(_, _) | RPatKind::Wild)),
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
                check_match_dependent(cx, scrut, equation, arms, expected, span, predicates)
            } else if equation.is_some() {
                Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "`match ... eqn:` requires a finite enum scrutinee with flat arms"
                        .into(),
                })
            } else {
                let (core, inferred_ty) =
                    infer_match_with_predicates(cx, scrut, arms, span, Some(expected), predicates)?;
                unify_types(&mut cx.metas, expected, &inferred_ty);
                // This is an inferred match, not the checked structural path;
                // each branch has already been checked against its result.
                emit_refinement_introduction(cx, expected, &inferred_ty, core, span, None)
            }
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

#[derive(Clone)]
struct ActiveIndexRefinement {
    leaves: Vec<IndexEqualityLeaf>,
    install_depth: usize,
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
        CoherentFrameCarriedDatum::MotiveReturnTelescopeArguments(telescope_argument_substitutions),
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
        Term::Quot(a, r, e) => Term::Quot(
            Box::new(subst_term_generalize_many(a, subs)),
            Box::new(subst_term_generalize_many(r, subs)),
            Box::new(subst_term_generalize_many(e, subs)),
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
        Term::Proj1(p)
        | Term::Proj2(p)
        | Term::QuotClass(p)
        | Term::Trunc(p)
        | Term::TruncProj(p)
        | Term::Refl(p) => scrut_occurs(p, scrut_core),
        Term::Ascript(t, a) | Term::Absurd(t, a) => {
            scrut_occurs(t, scrut_core) || scrut_occurs(a, scrut_core)
        }
        Term::Quot(a, r, e) => {
            scrut_occurs(a, scrut_core)
                || scrut_occurs(r, scrut_core)
                || scrut_occurs(e, scrut_core)
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
    cx: &ElabCtx<'_>,
    ctx: &Context,
    method: &Term,
    expected: &Term,
) -> Result<Option<Term>, ElabError> {
    fn go(
        cx: &ElabCtx<'_>,
        ctx: &Context,
        method: &Term,
        expected: &Term,
    ) -> Result<Option<Term>, ElabError> {
        match (method, whnf(cx.env, ctx, expected)) {
            (Term::Lam(_, body), Term::Pi(domain, codomain)) => {
                let mut body_ctx = ctx.clone();
                body_ctx.push((*domain).clone());
                let Some(body) = go(cx, &body_ctx, body, &codomain)? else {
                    return Ok(None);
                };
                Ok(Some(Term::lam(*domain, body)))
            }
            _ => {
                match kernel_check_in_context_current(cx, ctx, method, expected) {
                    Ok(()) => return Ok(Some(method.clone())),
                    Err(CurrentKernelQueryError::View(error)) => return Err(error),
                    Err(CurrentKernelQueryError::Kernel(_)) => {}
                }
                for index in 0..ctx.len() {
                    let mut candidate = Term::var(index);
                    let mut candidate_ty = match kernel_infer_in_context_current(cx, ctx, &candidate)
                    {
                        Ok(ty) => ty,
                        Err(CurrentKernelQueryError::View(error)) => return Err(error),
                        Err(CurrentKernelQueryError::Kernel(_)) => return Ok(None),
                    };
                    let mut consumed = false;
                    loop {
                        if convert_type(cx.env, ctx, &candidate_ty, expected) && consumed {
                            return Ok(Some(candidate));
                        }
                        let Term::Pi(domain, codomain) = whnf(cx.env, ctx, &candidate_ty) else {
                            break;
                        };
                        let mut selected_argument = None;
                        for argument_index in 0..ctx.len() {
                            let argument = Term::var(argument_index);
                            let argument_ty = match kernel_infer_in_context_current(
                                cx,
                                ctx,
                                &argument,
                            ) {
                                Ok(ty) => ty,
                                Err(CurrentKernelQueryError::View(error)) => return Err(error),
                                Err(CurrentKernelQueryError::Kernel(_)) => return Ok(None),
                            };
                            if convert_type(cx.env, ctx, &argument_ty, &domain) {
                                selected_argument = Some(argument);
                                break;
                            }
                        }
                        let Some(argument) = selected_argument else {
                            break;
                        };
                        candidate = Term::app(candidate, argument.clone());
                        candidate_ty = subst0(&codomain, &argument);
                        consumed = true;
                    }
                }
                Ok(None)
            }
        }
    }
    go(cx, ctx, method, expected)
}

/// Find exactly those embedded methods whose ambient proof was specialized to
/// the old actual index. A method is convoyed only when its original form
/// kernel-checks and the same structural rebase fails against the generalized
/// method type; already-parametric methods stay in place.
#[inline(never)]
fn plan_embedded_method_convoy(
    cx: &ElabCtx<'_>,
    ambient_ctx: &Context,
    motive_ctx: &Context,
    ambient_expected: &Term,
    rebased_expected: &Term,
    sentinel_region: usize,
) -> Result<(Vec<EmbeddedMethodConvoy>, Vec<(usize, usize)>), ElabError> {
    let env: &GlobalEnv = &*cx.env;
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
            kernel_check_in_context_current(
                cx,
                ambient_ctx,
                &ambient.methods[method_ordinal],
                &ambient_ty,
            )
            .map_err(|error| match error {
                CurrentKernelQueryError::View(error) => error,
                CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                    "coherent-frame convoy found an ill-typed ambient method: {error:?}"
                )),
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
            let rebased_check = kernel_check_in_context_current(
                cx,
                motive_ctx,
                &rebased.methods[method_ordinal],
                &motive_ty,
            );
            let rebased_failed = match rebased_check {
                Ok(()) => false,
                Err(CurrentKernelQueryError::View(error)) => return Err(error),
                Err(CurrentKernelQueryError::Kernel(_)) => true,
            };
            if rebased_failed {
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
                    cx,
                    motive_ctx,
                    &rebased.methods[method_ordinal],
                    &motive_ty,
                )?
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
    cx: &ElabCtx<'_>,
    ctx: &Context,
    term: &Term,
    plan: &[EmbeddedMethodConvoy],
    repairs: &[(usize, usize)],
    slot_base: usize,
) -> Result<Term, ElabError> {
    fn go(
        cx: &ElabCtx<'_>,
        ctx: &Context,
        term: &Term,
        plan: &[EmbeddedMethodConvoy],
        repairs: &[(usize, usize)],
        slot_base: usize,
        next_elim: &mut usize,
        seen: &mut [bool],
        repair_error: &std::cell::RefCell<Option<ElabError>>,
    ) -> Term {
        let env: &GlobalEnv = &*cx.env;
        let recur = |term: &Term, next_elim: &mut usize, seen: &mut [bool]| {
            go(
                cx,
                ctx,
                term,
                plan,
                repairs,
                slot_base,
                next_elim,
                seen,
                repair_error,
            )
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
                            match repaired_ty.as_ref().map(|expected| {
                                repair_embedded_method_from_ih(cx, ctx, &rewritten, expected)
                            }) {
                                Some(Ok(Some(repaired))) => repaired,
                                Some(Err(error)) => {
                                    *repair_error.borrow_mut() = Some(error);
                                    rewritten
                                }
                                Some(Ok(None)) | None => rewritten,
                            }
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
            Term::Quot(a, r, e) => Term::Quot(
                Box::new(recur(a, next_elim, seen)),
                Box::new(recur(r, next_elim, seen)),
                Box::new(recur(e, next_elim, seen)),
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
    let repair_error = std::cell::RefCell::new(None);
    let rewritten = go(
        cx,
        ctx,
        term,
        plan,
        repairs,
        slot_base,
        &mut next_elim,
        &mut seen,
        &repair_error,
    );
    if let Some(error) = repair_error.into_inner() {
        return Err(error);
    }
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
        Term::Quot(a, r, e) => Ok(Term::Quot(
            Box::new(go(a, ctx)?),
            Box::new(go(r, ctx)?),
            Box::new(go(e, ctx)?),
        )),
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
    cx: &ElabCtx<'_>,
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
    _split_span: &Span,
) -> Result<Option<Term>, ElabError> {
    let env: &GlobalEnv = &*cx.env;
    if ind.indices.len() != 1 || scrut_indices.len() != 1 {
        return Ok(None);
    }
    let goal_head = whnf(env, outer_ctx, original_expected);
    let (return_telescope, goal_level) = match goal_head {
        Term::Eq(goal_ty, _, _) => {
            let Term::Type(goal_level) = whnf(
                env,
                outer_ctx,
                &kernel_infer_in_context_current(cx, outer_ctx, &goal_ty).map_err(
                    |error| match error {
                        CurrentKernelQueryError::View(error) => error,
                        CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                            "large index convoy could not classify its equality carrier: {error:?}"
                        )),
                    },
                )?,
            ) else {
                return Ok(None);
            };
            (false, goal_level)
        }
        Term::Pi(_, _) => {
            let Term::Omega(goal_level) = whnf(
                env,
                outer_ctx,
                &kernel_infer_in_context_current(cx, outer_ctx, original_expected).map_err(
                    |error| match error {
                        CurrentKernelQueryError::View(error) => error,
                        CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                            "large index convoy could not classify its return telescope: {error:?}"
                        )),
                    },
                )?,
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
    let predicate_sort = kernel_infer_in_context_current(
        cx,
        &index_binder_ctx,
        &predicate_body,
    )
    .map_err(|error| match error {
        CurrentKernelQueryError::View(error) => error,
        CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
            "large index convoy predicate is ill-typed: {error:?}"
        )),
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
        let carried =
            carry_coherent_frame_data(leaves, raw_evidence, telescope_argument_substitutions);
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
                let mut goal = subst_term_generalize_many(&expected_at_depth, &goal_substitutions);

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
                        &kernel_infer_in_context_current(cx, &evidence_ctx, &leaf.index_ty)
                            .map_err(|error| match error {
                                CurrentKernelQueryError::View(error) => error,
                                CurrentKernelQueryError::Kernel(error) => {
                                    ElabError::Internal(format!(
                                        "large index convoy could not classify a peeled index: {error:?}"
                                    ))
                                }
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
                        &leaf.scrutinee,
                        leaf.proof,
                    );
                    let stable_forward = build_sym(
                        env,
                        &evidence_ctx,
                        &leaf.index_ty,
                        index_level,
                        &leaf.scrutinee,
                        &leaf.target,
                        reverse,
                    );
                    if let Some((cast, cast_ty)) = try_reindex_cast(
                        Some(cx),
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
            kernel_infer_in_context_current(cx, &evidence_ctx, &goal).map_err(
                |error| match error {
                    CurrentKernelQueryError::View(error) => error,
                    CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                        "large index convoy successor goal is ill-typed: {error:?}"
                    )),
                },
            )?;
            Term::pi(raw_evidence, goal)
        };

        let mut method = Term::lam(value_domain, branch_result);
        for domain in method_domains.into_iter().rev() {
            method = Term::lam(domain, method);
        }
        kernel_check_in_context_current(cx, outer_ctx, &method, &index_method_ty).map_err(
            |error| match error {
                CurrentKernelQueryError::View(error) => error,
                CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                    "large index convoy constructed an ill-typed index method: {error:?}"
                )),
            },
        )?;
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
    kernel_infer_in_context_current(cx, motive_ctx, &body).map_err(|error| match error {
        CurrentKernelQueryError::View(error) => error,
        CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
            "large index convoy constructed an ill-typed shared motive: {error:?}"
        )),
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
    frame: &MatchFrame,
) -> Result<RecursiveFieldIndexPath, ElabError> {
    if ind.indices.len() != 1
        || scrut_indices.len() != 1
        || !matches!(scrut_core, Term::Var(_))
        || !matches!(scrut_indices[0], Term::Var(_))
        || !compute_context_convoy(cx, frame, scrut_indices)?.is_empty()
    {
        return Ok(RecursiveFieldIndexPath::CoupledRefinement);
    }

    let zonked_ctx = Context {
        types: cx
            .ctx
            .types
            .iter()
            .map(|term| cx.metas.zonk_term(term))
            .collect(),
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
        let target_indices = ctor_target_indices(ctor, ind, params, level_args, field_count)
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
    frame: &MatchFrame,
) -> Result<Box<CoherentFrameMotivePlan>, ElabError> {
    let context_convoy = compute_context_convoy(cx, frame, scrut_indices)?;
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
    Ok(Box::new(CoherentFrameMotivePlan {
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
    }))
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
    frame: &MatchFrame,
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
        return ordinary_coherent_frame_plan(
            cx,
            scrut_core,
            scrut_indices,
            original_expected,
            motive_base_depth,
            motive_local_indices,
            recursive_field_index_path,
            frame,
        );
    }
    let expanded_expected = boxed_simplify_branch_goal(cx.env, &zonked_ctx, original_expected);
    let has_nontrivial_coupled_sides = matches!(original_expected, Term::Eq(_, _, _))
        || matches!(expanded_expected.as_ref(), Term::Eq(_, _, _));
    let probe_context_convoy = compute_context_convoy(cx, frame, scrut_indices)?;

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
    let equation_convoy = (!cx.match_frames.is_empty() && has_nontrivial_coupled_sides)
        || forced_telescope_convoy;
    if equation_convoy {
        if !probe_context_convoy.is_empty() && !forced_telescope_convoy {
            return Err(ElabError::Internal(
                "index-equation convoy unexpectedly overlaps an ambient context convoy".into(),
            ));
        }
        if let Some(motive_user_body) = build_index_equation_convoy_body(
            cx,
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
            span,
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
            cx,
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
        compute_context_convoy(cx, frame, scrut_indices)?
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
            cx,
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
/// wraps them outermost-first. The scrutinee's own binder is excluded (the
/// motive already abstracts it). Binders with a recorded generated/arm origin
/// are excluded; user-local binders remain ambient even within an arm.
fn compute_context_convoy(
    cx: &ElabCtx,
    frame: &MatchFrame,
    scrut_indices: &[Term],
) -> Result<Vec<ConvoyEntry>, ElabError> {
    let ctx = &cx.ctx;
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
        // Skip the scrutinee recorded once at match introduction; the motive
        // abstracts it directly even as later binders move its de Bruijn index.
        if frame.scrutinee_level == Some(depth - 1 - var) {
            continue;
        }
        // A user-local binder remains genuinely ambient, even when pushed
        // between an enclosing arm and this match. Every other origin is
        // owned by its arm; missing provenance inside an active frame is an
        // error, never a positional guess.
        let bottom_pos = depth - 1 - var;
        if let Some(origin) = cx.match_binder_origin(bottom_pos)? {
            if origin != MatchBinderOrigin::UserLocal {
                continue;
            }
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
    Ok(convoy)
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

fn apply_term_spine(head: Term, arguments: &[Term]) -> Term {
    arguments.iter().cloned().fold(head, Term::app)
}

/// J-congruence for the exact family supplied by the caller. `family_at_y`
/// lives under both motive binders: `y = Var(1)` and its equality proof at
/// `Var(0)`. The caller obtains its source and target from the same forward
/// family; this constructor never inverts a substitution by value.
#[allow(clippy::too_many_arguments)]
fn build_family_type_cong(
    env: &GlobalEnv,
    ctx: &Context,
    index_ty: &Term,
    old_index: &Term,
    new_index: &Term,
    source_type: &Term,
    family_at_y: &Term,
    type_level: Level,
    equality: Term,
) -> Term {
    let equality_domain = Term::Eq(
        Box::new(weaken(index_ty, 1)),
        Box::new(weaken(old_index, 1)),
        Box::new(Term::var(0)),
    );
    let motive_body = Term::lam(
        index_ty.clone(),
        Term::lam(
            equality_domain.clone(),
            Term::Eq(
                Box::new(Term::Type(type_level.clone())),
                Box::new(weaken(source_type, 2)),
                Box::new(family_at_y.clone()),
            ),
        ),
    );
    let motive_type = Term::pi(
        index_ty.clone(),
        Term::pi(equality_domain, Term::omega(type_level.clone().suc())),
    );
    let motive = Term::Ascript(Box::new(motive_body), Box::new(motive_type));
    let base = Term::Refl(Box::new(refl_base_arg(
        env, ctx, &Term::Type(type_level), source_type,
    )));
    let proof_ty = Term::Eq(
        Box::new(index_ty.clone()),
        Box::new(old_index.clone()),
        Box::new(new_index.clone()),
    );
    let proof = Term::Ascript(Box::new(equality), Box::new(proof_ty));
    Term::J(Box::new(motive), Box::new(base), Box::new(proof))
}

/// Direct Ω transport through the same caller-supplied family. The target is
/// carried out unchanged, never recovered by reversing the old substitution.
#[allow(clippy::too_many_arguments)]
fn build_family_omega_transport(
    idx_ty: &Term,
    old_idx: &Term,
    new_idx: &Term,
    family_at_y: &Term,
    target_type: &Term,
    omega_level: Level,
    value: Term,
    h: Term,
) -> (Term, Term) {
    let dom2 = Term::Eq(
        Box::new(weaken(idx_ty, 1)),
        Box::new(weaken(old_idx, 1)),
        Box::new(Term::var(0)),
    );
    let motive_body = Term::lam(
        idx_ty.clone(),
        Term::lam(dom2.clone(), family_at_y.clone()),
    );
    let motive_ty = Term::pi(idx_ty.clone(), Term::pi(dom2, Term::omega(omega_level)));
    let motive = Term::Ascript(Box::new(motive_body), Box::new(motive_ty));
    let proof_ty = Term::Eq(
        Box::new(idx_ty.clone()),
        Box::new(old_idx.clone()),
        Box::new(new_idx.clone()),
    );
    let proof = Term::Ascript(Box::new(h), Box::new(proof_ty));
    (
        Term::J(Box::new(motive), Box::new(value), Box::new(proof)),
        target_type.clone(),
    )
}

/// Build equality between two applications of one result family while changing
/// exactly one index argument. Unlike `build_index_type_cong`, this does not
/// replace equal-looking occurrences in sibling indices: the J motive rebuilds
/// the application spine and varies only `argument_position`.
#[allow(clippy::too_many_arguments)]
fn build_result_index_type_cong(
    env: &GlobalEnv,
    ctx: &Context,
    index_ty: &Term,
    old_index: &Term,
    new_index: &Term,
    result_head: &Term,
    result_arguments: &[Term],
    argument_position: usize,
    type_level: Level,
    equality: Term,
) -> (Term, Term) {
    let source_type = apply_term_spine(result_head.clone(), result_arguments);
    let mut target_arguments = result_arguments.to_vec();
    target_arguments[argument_position] =
        subst_term_generalize(&target_arguments[argument_position], old_index, new_index);
    let target_type = apply_term_spine(result_head.clone(), &target_arguments);

    let mut arguments_at_y: Vec<Term> = result_arguments
        .iter()
        .map(|argument| weaken(argument, 2))
        .collect();
    arguments_at_y[argument_position] = subst_term_generalize(
        &arguments_at_y[argument_position],
        &weaken(old_index, 2),
        &Term::var(1),
    );
    let type_at_y = apply_term_spine(weaken(result_head, 2), &arguments_at_y);
    let proof = build_family_type_cong(
        env, ctx, index_ty, old_index, new_index, &source_type, &type_at_y,
        type_level, equality,
    );
    (proof, target_type)
}

/// Transport a recursive-group sibling call from its concrete result-family
/// indices to the indices expected inside the current dependent-match method.
/// The method's hidden match equality is represented by a sentinel until method
/// finalization; the completed eliminator supplies it through
/// `synth_generated_index_evidence`.
///
/// Transport synthesizes one equality per RESULT-family index. Reflexive
/// positions use the shared synthesizer; non-reflexive positions reuse a leaf
/// projected from the hidden match equality. Each synthesized premise is then
/// consumed by `project_generated_index_equality_leaves`. The final J motive
/// varies only that result-index position, so a component repeated inside a
/// sibling whole-record index is not accidentally rewritten there too.
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
    'refinements: for refinement in cx.result_refinements.iter().rev() {
        let growth = cx
            .ctx
            .len()
            .checked_sub(refinement.install_depth)
            .ok_or_else(|| {
                ElabError::Internal("result refinement escaped its branch context".into())
            })? as i64;
        let source_index_ty = weaken(&refinement.index_ty, growth);
        let source_concrete = weaken(&refinement.concrete_index, growth);
        let source_refined = weaken(&refinement.refined_index, growth);
        let source_evidence_ty = Term::Eq(
            Box::new(source_index_ty.clone()),
            Box::new(source_concrete.clone()),
            Box::new(source_refined.clone()),
        );
        let source_proof = index_refinement_sentinel(
            refinement.sentinel_region,
            refinement.premise_slot + growth as usize,
        );
        let (source_head, _) = peel_app(&source_index_ty);
        let scoped_source_projection = match source_head {
            Term::IndFormer { id, .. } => cx.env.inductive(id).is_some_and(|inductive| {
                inductive.indices.is_empty() && inductive.constructors.len() == 1
            }),
            _ => false,
        };
        let source_leaves = if scoped_source_projection {
            project_generated_index_equality_leaves(
                cx.env,
                &cx.ctx,
                &source_evidence_ty,
                source_proof.clone(),
            )
            .unwrap_or_default()
        } else {
            Vec::new()
        };

        'result_indices: {
            let current_whnf = whnf(cx.env, &cx.ctx, &transported_ty);
            let expected_whnf = whnf(cx.env, &cx.ctx, expected);
            let (result_head, mut result_arguments) = peel_app(&current_whnf);
            let (expected_head, expected_arguments) = peel_app(&expected_whnf);
            let (
                Term::IndFormer {
                    id: result_family,
                    level_args: result_levels,
                },
                Term::IndFormer {
                    id: expected_family,
                    level_args: expected_levels,
                },
            ) = (&result_head, &expected_head)
            else {
                break 'result_indices;
            };
            if result_family != expected_family || result_levels != expected_levels {
                break 'result_indices;
            }
            let Some(result_decl) = cx.env.inductive(*result_family) else {
                break 'result_indices;
            };
            let parameter_count = result_decl.params.len();
            let index_count = result_decl.indices.len();
            if result_arguments.len() != parameter_count + index_count
                || expected_arguments.len() != result_arguments.len()
            {
                break 'result_indices;
            }
            let mut parameters_match = true;
            for (actual, wanted) in result_arguments[..parameter_count]
                .iter()
                .zip(&expected_arguments[..parameter_count])
            {
                let parameter_ty = kernel_infer_current(cx, actual).map_err(|error| match error {
                    CurrentKernelQueryError::View(error) => error,
                    CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                        "result refinement: could not classify a result-family parameter: {error:?}"
                    )),
                })?;
                if !convert(cx.env, &cx.ctx, &parameter_ty, actual, wanted) {
                    parameters_match = false;
                    break;
                }
            }
            if !parameters_match {
                break 'result_indices;
            }

            let mut local_transported = transported.clone();
            let mut local_transported_ty = transported_ty.clone();
            let mut local_changed = false;
            let mut completed_plan = true;
            for index_ordinal in 0..index_count {
                let argument_position = parameter_count + index_ordinal;
                let raw_index_ty = subst_levels(
                    &subst_outer(
                        &result_decl.indices[index_ordinal],
                        parameter_count,
                        &result_arguments[..parameter_count],
                        index_ordinal,
                    ),
                    &result_decl.level_params,
                    result_levels,
                );
                let index_ty = subst_tel(
                    &raw_index_ty,
                    &result_arguments[parameter_count..argument_position],
                );
                let raw_expected_index_ty = subst_levels(
                    &subst_outer(
                        &result_decl.indices[index_ordinal],
                        parameter_count,
                        &expected_arguments[..parameter_count],
                        index_ordinal,
                    ),
                    &result_decl.level_params,
                    expected_levels,
                );
                let expected_index_ty = subst_tel(
                    &raw_expected_index_ty,
                    &expected_arguments[parameter_count..argument_position],
                );
                if !convert_type(cx.env, &cx.ctx, &index_ty, &expected_index_ty) {
                    completed_plan = false;
                    break;
                }
                let old_index = result_arguments[argument_position].clone();
                let new_index = expected_arguments[argument_position].clone();
                let result_evidence_ty = Term::Eq(
                    Box::new(index_ty.clone()),
                    Box::new(old_index.clone()),
                    Box::new(new_index.clone()),
                );
                let result_proof = if convert(cx.env, &cx.ctx, &index_ty, &old_index, &new_index) {
                    match synth_generated_index_evidence(
                        cx.env,
                        &cx.ctx,
                        &result_evidence_ty,
                        expr.span(),
                    ) {
                        Ok(proof) => proof,
                        Err(_) => {
                            completed_plan = false;
                            break;
                        }
                    }
                } else if convert_type(cx.env, &cx.ctx, &index_ty, &source_index_ty)
                    && convert(cx.env, &cx.ctx, &index_ty, &old_index, &source_concrete)
                    && convert(cx.env, &cx.ctx, &index_ty, &new_index, &source_refined)
                {
                    source_proof.clone()
                } else if let Some(leaf) = scoped_source_projection
                    .then(|| {
                        source_leaves.iter().find(|leaf| {
                            convert_type(cx.env, &cx.ctx, &index_ty, &leaf.index_ty)
                                && convert(cx.env, &cx.ctx, &index_ty, &old_index, &leaf.target)
                                && convert(cx.env, &cx.ctx, &index_ty, &new_index, &leaf.scrutinee)
                        })
                    })
                    .flatten()
                {
                    leaf.proof.clone()
                } else {
                    // This result index is unrelated to the current match
                    // refinement. A different active refinement may own it.
                    continue;
                };

                let result_leaves = match project_generated_index_equality_leaves(
                    cx.env,
                    &cx.ctx,
                    &result_evidence_ty,
                    result_proof,
                ) {
                    Ok(leaves) => leaves,
                    Err(_) => {
                        completed_plan = false;
                        break;
                    }
                };
                for leaf in result_leaves {
                    let next_argument = subst_term_generalize(
                        &result_arguments[argument_position],
                        &leaf.target,
                        &leaf.scrutinee,
                    );
                    if next_argument == result_arguments[argument_position] {
                        continue;
                    }
                    let classifier = kernel_infer_current(cx, &local_transported_ty).map_err(
                        |error| match error {
                            CurrentKernelQueryError::View(error) => error,
                            CurrentKernelQueryError::Kernel(error) => {
                                ElabError::Internal(format!(
                                    "result refinement: could not classify the recursive result: {error:?}"
                                ))
                            }
                        },
                    )?;
                    let Term::Type(type_level) = whnf(cx.env, &cx.ctx, &classifier) else {
                        completed_plan = false;
                        break;
                    };
                    let (type_equality, next_ty) = build_result_index_type_cong(
                        cx.env,
                        &cx.ctx,
                        &leaf.index_ty,
                        &leaf.target,
                        &leaf.scrutinee,
                        &result_head,
                        &result_arguments,
                        argument_position,
                        type_level,
                        leaf.proof,
                    );
                    local_transported = Term::Cast(
                        Box::new(local_transported_ty),
                        Box::new(next_ty.clone()),
                        Box::new(type_equality),
                        Box::new(local_transported),
                    );
                    local_transported_ty = next_ty;
                    result_arguments[argument_position] = next_argument;
                    local_changed = true;
                }
                if !completed_plan
                    || !convert(
                        cx.env,
                        &cx.ctx,
                        &index_ty,
                        &result_arguments[argument_position],
                        &new_index,
                    )
                {
                    completed_plan = false;
                    break;
                }
            }
            if completed_plan && local_changed {
                transported = local_transported;
                transported_ty = local_transported_ty;
                changed = true;
                if convert_type(cx.env, &cx.ctx, &transported_ty, expected) {
                    return Ok(Some((transported, transported_ty)));
                }
                continue 'refinements;
            }
        }

        // Route-(a) was inapplicable or could not complete atomically. Preserve
        // the former whole-index path on the CURRENT cumulative value/type;
        // a successful row becomes the input to the next active refinement.
        if let Some((fallback, fallback_ty)) = try_reindex_cast(
            Some(cx),
            cx.env,
            &cx.ctx,
            &source_index_ty,
            &source_concrete,
            &source_refined,
            &transported_ty,
            transported.clone(),
            source_proof,
        )? {
            transported = fallback;
            transported_ty = fallback_ty;
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

fn call_application_spine(expr: &RExpr) -> (&RExpr, Vec<&RExpr>) {
    let mut head = expr;
    let mut arguments = Vec::new();
    while let RExpr::RApp(function, argument, _) = head {
        arguments.push(argument.as_ref());
        head = function.as_ref();
    }
    arguments.reverse();
    (head, arguments)
}

fn premise_proof_in_scope(cx: &ElabCtx<'_>, goal: &Term) -> Option<Term> {
    if let Some(proof) = cx.assumptions.iter().rev().find_map(|assumption| {
        let later_binders = cx.ctx.len().checked_sub(assumption.depth)?;
        if assumption.depth >= cx.ctx.len() {
            return None;
        }
        let proposition = weaken(&assumption.prop, later_binders as i64);
        convert_type(cx.env, &cx.ctx, &proposition, goal)
            .then(|| Term::var(cx.ctx.len() - 1 - assumption.depth))
    }) {
        return Some(proof);
    }
    (0..cx.ctx.len()).rev().find_map(|position| {
        let proposition = weaken(&cx.ctx.types[position], (cx.ctx.len() - position) as i64);
        convert_type(cx.env, &cx.ctx, &proposition, goal)
            .then(|| Term::var(cx.ctx.len() - 1 - position))
    })
}

/// The single mint point for obligation holes recorded in an `ElabCtx`'s
/// `cx.obligations`: `requires` premises, partial-primitive side conditions
/// (`+` overflow, `/` and `%` nonzero), refinement introductions, and
/// contract `ensures` realized at result leaves. In a
/// context whose obligations cannot reach a reporter
/// (`PremiseHoles::Refused`) it refuses before `declare_postulate`, so no
/// unreported hole enters the environment.
///
/// Other declaration-level producers (space `ensures`, `prove`, law fields,
/// FFI runtime checks) do not record into `cx.obligations`: each
/// builds its `Obligation` beside the mint and returns it in
/// `ElabResult::obligations` to the declaration elaborator's caller, the same
/// channel every `Reported` context's obligations take. Whether that caller
/// reports or discards an `ElabResult` is the caller's contract, not this
/// gate's; no `ElabCtx` mode applies to these producers, so they do not call
/// this gate. Any new producer that records into `cx.obligations` must mint
/// through this function.
fn declare_obligation_hole(
    cx: &mut ElabCtx<'_>,
    closed: Term,
    span: &Span,
    kind: ObligationKind,
) -> Result<GlobalId, ElabError> {
    if cx.premise_holes == PremiseHoles::Refused {
        return Err(match kind {
            ObligationKind::Requires => {
                ElabError::PremiseWithoutObligationChannel { span: span.clone() }
            }
            ObligationKind::PartialPrim
            | ObligationKind::RefinementIntroduction
            | ObligationKind::Ensures
            | ObligationKind::Prove
            | ObligationKind::LawField(_)
            | ObligationKind::FfiRuntimeCheck => {
                ElabError::ObligationWithoutChannel { span: span.clone() }
            }
        });
    }
    let hole_id = declare_postulate(cx.env, cx.owner_label.clone(), vec![], closed.clone())
        .map_err(|error| ElabError::KernelRejected {
            error,
            span: span.clone(),
        })?;
    cx.obligations.push(Obligation {
        id: cx.obl_counter,
        hole_id,
        goal_closed: closed,
        span: span.clone(),
        kind,
    });
    cx.obl_counter += 1;
    Ok(hole_id)
}

fn precondition_proof(cx: &mut ElabCtx<'_>, goal: Term, span: &Span) -> Result<Term, ElabError> {
    let closed = close_goal(&cx.ctx, &[], goal);
    let hole_id = declare_obligation_hole(cx, closed, span, ObligationKind::Requires)?;
    Ok((0..cx.ctx.len())
        .rev()
        .fold(Term::const_(hole_id, vec![]), |proof, index| {
            Term::app(proof, Term::var(index))
        }))
}

fn append_precondition_proofs(
    cx: &mut ElabCtx<'_>,
    mut call: Term,
    mut ty: Term,
    requires_arity: usize,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    for _ in 0..requires_arity {
        let Term::Pi(domain, codomain) = whnf(cx.env, &cx.ctx, &ty) else {
            return Err(ElabError::NotAFunction { span: span.clone() });
        };
        let proof = match premise_proof_in_scope(cx, &domain) {
            Some(proof) => proof,
            None => precondition_proof(cx, *domain.clone(), span)?,
        };
        ty = subst0(&codomain, &proof);
        call = Term::app(call, proof);
    }
    Ok((call, ty))
}

fn apply_zero_arity_preconditions(
    cx: &mut ElabCtx<'_>,
    id: GlobalId,
    call: Term,
    ty: Term,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    match cx.preconditions.get(&id).copied() {
        Some((0, requires_arity)) => append_precondition_proofs(cx, call, ty, requires_arity, span),
        _ => Ok((call, ty)),
    }
}

/// The checked global at a call head, before zero-arity premise insertion.
/// Probing a call for its precondition arity must not raise a call-site
/// obligation: a declined probe falls through to generic application, which
/// infers the head again and would raise the same premise a second time.
fn infer_unpremised_global_head(
    cx: &mut ElabCtx<'_>,
    head: &RExpr,
) -> Result<Option<(GlobalId, Term, Term)>, ElabError> {
    let (core, ty) = match head {
        RExpr::RCheckedGlobal { name, id, .. } => {
            let id = *id;
            if cx.env.constructor(id).is_some() || cx.env.inductive(id).is_some() {
                return Ok(None);
            }
            let (_, ty) = cx.env.const_type(id).ok_or_else(|| {
                ElabError::Internal(format!(
                    "no checked type for imported global '{name}' {id:?}"
                ))
            })?;
            (Term::const_(id, vec![]), ty.clone())
        }
        // Surface eliminators such as `J` can be unresolved RCons until their
        // dedicated application arm sees the complete arity. Do not infer an
        // arbitrary head while probing for call-site metadata.
        RExpr::RCon(name, span) if cx.globals.contains_key(name) => {
            infer_spelling_global(cx, name, span)?
        }
        _ => return Ok(None),
    };
    let Term::Const { id, .. } = &core else {
        return Ok(None);
    };
    Ok(Some((*id, core, ty)))
}

fn infer_preconditioned_application(
    cx: &mut ElabCtx<'_>,
    expr: &RExpr,
    span: &Span,
) -> Result<Option<(Term, Term)>, ElabError> {
    let (head, arguments) = call_application_spine(expr);
    let Some((id, head_core, mut ty)) = infer_unpremised_global_head(cx, head)? else {
        return Ok(None);
    };
    let Some((parameter_arity, requires_arity)) = cx.preconditions.get(&id).copied() else {
        return Ok(None);
    };
    if arguments.len() != parameter_arity {
        return Ok(None);
    }

    let mut call = head_core;
    for argument in arguments {
        let Term::Pi(domain, codomain) = whnf(cx.env, &cx.ctx, &ty) else {
            return Err(ElabError::NotAFunction { span: span.clone() });
        };
        let argument_core = check(cx, argument, &domain, span)?;
        ty = subst0(&codomain, &argument_core);
        call = Term::app(call, argument_core);
    }
    append_precondition_proofs(cx, call, ty, requires_arity, span).map(Some)
}

fn append_saturated_preconditions(
    cx: &mut ElabCtx<'_>,
    expr: &RExpr,
    call: (Term, Term),
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let (head, arguments) = call_application_spine(expr);
    let Some((id, _, _)) = infer_unpremised_global_head(cx, head)? else {
        return Ok(call);
    };
    let Some((parameter_arity, requires_arity)) = cx.preconditions.get(&id).copied() else {
        return Ok(call);
    };
    if arguments.len() != parameter_arity {
        return Ok(call);
    }
    append_precondition_proofs(cx, call.0, call.1, requires_arity, span)
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
#[cfg(test)]
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
    check_match_with_lift_with_predicates(
        cx, arms, expected, span, scrut_core, host, host_level_args,
        host_params, binding, &[],
    )
}

#[allow(clippy::too_many_arguments)]
fn check_match_with_lift_with_predicates(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    expected: &Term,
    span: &Span,
    scrut_core: &Term,
    host: &InductiveDecl,
    host_level_args: &[Level],
    host_params: &[Term],
    binding: LiftBinding,
    predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    ensure_arm_ctors_belong_to_family(cx, arms, host, host.id)?;
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
    let motive_sort = kernel_infer_in_context_current(cx, &motive_ctx, &motive_body).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        },
    )?;
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
        let (arm, _) =
            guarded_constructor_arm(cx, arms, host_ctor.id, &mut arm_used, &mut subsumed_by)
                .ok_or_else(|| ElabError::ExhaustivenessError {
                    missing: missing_pattern_witness(cx, host_ctor.id),
                    span: span.clone(),
                })?;
        let sub_pats = match &arm.pat.kind {
            RPatKind::Ctor(_, fields) | RPatKind::CheckedCtor(_, _, fields) => fields,
            _ => unreachable!("arm selected by constructor guard"),
        };
        if sub_pats.len() != host_ctor.args.len()
            || sub_pats
                .iter()
                .any(|pat| !matches!(pat.kind, RPatKind::Var(_, _) | RPatKind::Wild))
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
        let mut frame = MatchFrame::new(base, cx.match_frames.len(), Some(expected.clone()), None);
        frame.result_predicates = predicates.to_vec();
        cx.match_frames.push(frame);
        let mut domains = Vec::with_capacity(raw_domains.len());
        for (position, raw_domain) in raw_domains.iter().enumerate() {
            let domain = whnf(cx.env, &cx.ctx, raw_domain);
            let origin = if position < host_ctor.args.len() {
                MatchBinderOrigin::Field
            } else {
                MatchBinderOrigin::Ih
            };
            cx.push_match_binder(domain.clone(), origin);
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
            // A DIRECT recursive field's evidence is a support-family occurrence
            // whose separate trailing method result carries the recursive value;
            // that result is surfaced by `result_ordinal`. A field whose recursive
            // occurrence sits one positive former deeper (`List (Pair String
            // Self)`) has no such trailing support-family result — its evidence
            // BINDER is itself the guest induction hypothesis, the kernel's
            // positivity witness built by build_all_support_decl/carrier_lift_type.
            // Surface that binder as the selectable recursive result so
            // `recursive result for <field>` reaches the former-nested occurrence,
            // consuming the nested All-IH the kernel already builds. A higher-order
            // (Pi) evidence — the W-style `(Bool -> a)` occurrence — is left out of
            // scope exactly as today: its hypothesis is a function, not a value,
            // and applying it is a separate capability. This never manufactures a
            // decrease: an evidence binder exists only for a POSITIVE recursive
            // position (derive_carrier_shape is positivity-gated), so a negative
            // occurrence builds no evidence and remains rejected.
            let recursive_result_position = match result_ordinal {
                Some(ordinal) => Some(base + support_ctor.args.len() + ordinal),
                None => cx.binding_term(base + evidence_argument).and_then(
                    |(_, evidence_field_ty)| {
                        (!matches!(
                            whnf(cx.env, &cx.ctx, &evidence_field_ty),
                            Term::Pi(_, _)
                        ))
                        .then_some(base + evidence_argument)
                    },
                ),
            };
            let installed = install_lift_binding(
                cx,
                source_position,
                base + evidence_argument,
                recursive_result_position,
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
        cx.match_frames.last_mut().expect("lifted arm frame").refined_target = Some(expected_here.clone());
        let mut scrut_ty = Term::indformer(host.id, host_level_args.to_vec());
        for param in host_params {
            scrut_ty = Term::app(scrut_ty, param.clone());
        }
        let path_base = push_branch_path_condition(cx, &scrut_ty, scrut_core, &concrete, total, 0);
        let checked = check_match_arm_result(cx, &arm, &expected_here, &arm.span);
        cx.path_conditions.truncate(path_base);

        for source_field in evidence_positions {
            cx.lift_bindings.remove(&(base + source_field));
        }
        cx.hidden_positions.retain(|position| *position < base);
        for _ in 0..total {
            cx.ctx.pop();
        }
        cx.match_frames.pop();
        let mut method = checked?;
        for domain in domains.iter().rev() {
            method = Term::lam(domain.clone(), method);
        }
        let zonked_method = cx.metas.zonk_term(&method);
        let zonked_method_ty = cx.metas.zonk_term(&method_ty);
        kernel_check_current(cx, &zonked_method, &zonked_method_ty).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "generated All method failed kernel re-check: {error}"
            )),
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
    if cx.active_index_premise_frames.is_empty() {
        let zonked = cx.metas.zonk_term(&elim);
        kernel_infer_current(cx, &zonked).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "completed generated All eliminator failed kernel re-check: {error}"
            )),
        })?;
    } else {
        kernel_check_current(cx, &elim, expected).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "completed generated All eliminator failed kernel re-check: {error}"
            )),
        })?;
    }
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
    predicates: &[ResultPredicate],
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
    let mut frame = MatchFrame::new(base, cx.match_frames.len(), Some(expected.clone()), None);
    frame.result_predicates = predicates.to_vec();
    cx.match_frames.push(frame);
    let mut domains = Vec::with_capacity(raw_domains.len());
    for (position, raw_domain) in raw_domains.iter().enumerate() {
        let domain = whnf(cx.env, &cx.ctx, raw_domain);
        let origin = if position < field_count {
            MatchBinderOrigin::Field
        } else {
            MatchBinderOrigin::Ih
        };
        cx.push_match_binder(domain.clone(), origin);
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
        RPatKind::Ctor(_, fields) | RPatKind::CheckedCtor(_, _, fields) => fields
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
    cx.match_frames.last_mut().expect("structured arm frame").refined_target = Some(expected_here.clone());
    let mut scrut_ty = Term::indformer(ind.id, level_args.to_vec());
    for param in params {
        scrut_ty = Term::app(scrut_ty, param.clone());
    }
    let path_base = push_branch_path_condition(cx, &scrut_ty, scrut_core, &concrete, total, 0);
    let checked = check_match_arm_result(cx, arm, &expected_here, &arm.span);
    cx.path_conditions.truncate(path_base);

    for shape in shapes {
        cx.lift_bindings.remove(&(base + shape.position));
    }
    cx.hidden_positions.retain(|position| *position < base);
    for _ in 0..total {
        cx.ctx.pop();
    }
    cx.match_frames.pop();
    let mut method = checked?;
    for domain in domains.iter().rev() {
        method = Term::lam(domain.clone(), method);
    }
    let zonked_method = cx.metas.zonk_term(&method);
    let zonked_ty = cx.metas.zonk_term(&method_ty);
    kernel_check_current(cx, &zonked_method, &zonked_ty).map_err(|error| match error {
        CurrentKernelQueryError::View(error) => error,
        CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
            "structured host method failed kernel re-check: {error}"
        )),
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
    check_generalized_branch_goal(
        cx,
        arm,
        goal_refined,
        goal_restorations,
        expected_here,
        preserve_goal,
    )
}

#[inline(never)]
fn check_generalized_branch_goal(
    cx: &mut ElabCtx<'_>,
    arm: &RMatchArm,
    goal_refined: Term,
    restorations: Vec<BranchGoalRestoration>,
    expected_here: &Term,
    preserve_goal: bool,
) -> Result<Term, ElabError> {
    // Do not introduce an additional early kernel admission for a branch
    // whose goal needs no dependent binder. In particular, declaration
    // universe metas may still be solved by the enclosing method checker.
    if !restorations.iter().any(|restoration| {
        matches!(restoration, BranchGoalRestoration::Generalized { binders, .. } if !binders.is_empty())
    }) {
        let expected = if preserve_goal || matches!(arm.body, RExpr::RLam(_, _, _)) {
            goal_refined
        } else {
            simplify_branch_goal(cx.env, &cx.ctx, &goal_refined)
        };
        let mut body = check_match_arm_result(cx, arm, &expected, &arm.span)?;
        for restoration in restorations.into_iter().rev() {
            body = restoration.apply(body);
        }
        return Ok(body);
    }
    let original_depth = cx.ctx.len();
    let original_hidden = cx.hidden_positions.len();
    let saved_refinements = cx.var_refinements.clone();
    let (saved_premises, saved_origins, saved_telescope) = {
        let frame = cx.match_frames.last().ok_or_else(|| {
            ElabError::Internal("generalized branch has no owning match frame".into())
        })?;
        (frame.premise_bindings.clone(), frame.origins.clone(),
         frame.enclosing_telescope.clone())
    };
    let attempt = (|| {
        let mut inner_goal = goal_refined;
        let mut domains = Vec::new();
        let mut premise_binders = Vec::new();
        // The last leaf wrapped the outermost Pi. Peel in the same order
        // that `check` enters its hidden, temporary binder telescope.
        for restoration in restorations.iter().rev() {
            let BranchGoalRestoration::Generalized { binders, .. } = restoration else {
                continue;
            };
            for binder in binders {
                let Term::Pi(domain, codomain) = inner_goal else {
                    return Err(ElabError::Internal(
                        "generalized branch goal lost its binder telescope".into(),
                    ));
                };
                let domain = *domain;
                let origin = match binder.source {
                    GeneralizedGoalSource::Original(original) => {
                        if cx.match_frames.last().is_some_and(|frame| {
                            frame.convoy_originals.contains(&original)
                        }) {
                            MatchBinderOrigin::ConvoyRebound { original }
                        } else {
                            MatchBinderOrigin::GeneralizedDependent { original }
                        }
                    }
                    GeneralizedGoalSource::Premise { .. } => MatchBinderOrigin::GeneratedEquation,
                };
                cx.push_match_binder(domain.clone(), origin);
                let position = cx.ctx.len() - 1;
                cx.hidden_positions.push(position);
                match binder.source {
                    GeneralizedGoalSource::Original(original_position) => {
                        cx.var_refinements.insert(
                            original_position,
                            (Term::var(0), weaken(&domain, 1), cx.ctx.len()),
                        );
                    }
                    GeneralizedGoalSource::Premise { region, slot } => {
                        cx.match_frames.last_mut().expect("owning generalized arm").premise_bindings
                            .insert((region, slot), position);
                        premise_binders.push((region, slot, position));
                    }
                }
                domains.push(domain);
                inner_goal = *codomain;
            }
        }
        let expected = if preserve_goal || matches!(arm.body, RExpr::RLam(_, _, _)) {
            inner_goal
        } else {
            simplify_branch_goal(cx.env, &cx.ctx, &inner_goal)
        };
        let mut body = check_match_arm_result(cx, arm, &expected, &arm.span)?;
        kernel_check_current(cx, &body, &expected).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: arm.span.clone(),
            },
        })?;
        // A generated proof can still contain the owner-local sentinel even
        // though immediate kernel queries saw its scoped alias. Redirect it
        // before introducing the Lambda; finalization must not recover the
        // unrefined method premise underneath the generalized binder.
        for (region, slot, position) in premise_binders {
            let frame = cx
                .active_index_premise_frames
                .iter()
                .find(|frame| frame.sentinel_region == region && slot < frame.premise_domains.len())
                .ok_or_else(|| {
                    ElabError::Internal("generalized premise lost its installing frame".into())
                })?;
            let sentinel = checked_index_refinement_sentinel(
                region,
                slot + cx.ctx.len() - frame.install_depth,
            )?;
            body = subst_term_generalize(
                &body,
                &Term::var(sentinel),
                &Term::var(cx.ctx.len() - 1 - position),
            );
        }
        for domain in domains.into_iter().rev() {
            body = Term::lam(domain, body);
        }
        Ok(body)
    })();
    cx.ctx.types.truncate(original_depth);
    cx.hidden_positions.truncate(original_hidden);
    cx.var_refinements = saved_refinements;
    let frame = cx.match_frames.last_mut().ok_or_else(|| {
        ElabError::Internal("generalized branch lost its owning match frame".into())
    })?;
    frame.premise_bindings = saved_premises;
    frame.origins = saved_origins;
    frame.enclosing_telescope = saved_telescope;
    let mut body = attempt?;
    for restoration in restorations.into_iter().rev() {
        body = restoration.apply(body);
    }
    kernel_check_current(cx, &body, expected_here).map_err(|error| match error {
        CurrentKernelQueryError::View(error) => error,
        CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
            error,
            span: arm.span.clone(),
        },
    })?;
    Ok(body)
}

/// A branch-goal restoration is classified once, where the refined goal is
/// produced. Replay consumes this tag without re-classifying the goal.
enum BranchGoalRestoration {
    Generalized {
        whole: Box<BranchGoalRestoration>,
        binders: Vec<GeneralizedGoalBinder>,
    },
    TypeCast {
        source_type: Term,
        target_type: Term,
        equality: Term,
    },
    OmegaJ {
        index_type: Term,
        old_index: Term,
        new_index: Term,
        target_type: Term,
        family_at_y: Term,
        omega_level: Level,
        equality: Term,
    },
}

impl BranchGoalRestoration {
    fn apply(self, value: Term) -> Term {
        match self {
            Self::Generalized { whole, binders } => binders
                .into_iter()
                .fold(whole.apply(value), |function, binder| {
                    Term::app(function, binder.source_value)
                }),
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
                target_type,
                family_at_y,
                omega_level,
                equality,
            } => {
                let (transported, _) = build_family_omega_transport(
                    &index_type, &old_index, &new_index, &family_at_y,
                    &target_type, omega_level, value, equality,
                );
                transported
            }
        }
    }

    fn translate_to_original(self, embedding: &ActivePremiseEmbedding) -> Result<Self, ElabError> {
        let translate = |term: &Term| {
            embedding.translate_to_original(term, embedding.original_len, embedding.expanded_len)
        };
        Ok(match self {
            Self::Generalized { whole, binders } => Self::Generalized {
                whole: Box::new(whole.translate_to_original(embedding)?),
                binders,
            },
            Self::TypeCast {
                source_type,
                target_type,
                equality,
            } => Self::TypeCast {
                source_type: translate(&source_type)?,
                target_type: translate(&target_type)?,
                equality: translate(&equality)?,
            },
            Self::OmegaJ {
                index_type, old_index, new_index, target_type, family_at_y,
                omega_level, equality,
            } => {
                // The stored family has two free *motive-local* variables.
                // Embed it under its actual binders before translating the
                // outer expanded context, then peel them back off. Translating
                // the naked body would mistake Var(1) for a context binding.
                let equality_domain = Term::Eq(
                    Box::new(weaken(&index_type, 1)),
                    Box::new(weaken(&old_index, 1)),
                    Box::new(Term::var(0)),
                );
                let family_motive = Term::lam(
                    index_type.clone(), Term::lam(equality_domain, family_at_y),
                );
                let Term::Lam(_, inner) = translate(&family_motive)? else {
                    unreachable!("the family motive has two binders")
                };
                let Term::Lam(_, translated_family) = *inner else {
                    unreachable!("the family motive has two binders")
                };
                Self::OmegaJ {
                    index_type: translate(&index_type)?,
                    old_index: translate(&old_index)?,
                    new_index: translate(&new_index)?,
                    target_type: translate(&target_type)?,
                    family_at_y: *translated_family,
                    omega_level,
                    equality: translate(&equality)?,
                }
            },
        })
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
    // The pre-image of the caller's forward substitution. Abstract precisely
    // its scrutinee side; never invert `source_type` by replacing its image.
    generalized: &Term,
    classifier: Term,
    equality: Term,
) -> Result<BranchGoalRestoration, ElabError> {
    let family_at_y = subst_term_generalize(
        &weaken(generalized, 2), &weaken(new_index, 2), &Term::var(1),
    );
    match classifier {
        Term::Type(level) => {
            let type_equality = build_family_type_cong(
                env, ctx, index_type, old_index, new_index, source_type,
                &family_at_y, level, equality,
            );
            Ok(BranchGoalRestoration::TypeCast {
                source_type: source_type.clone(),
                target_type: generalized.clone(),
                equality: type_equality,
            })
        }
        Term::Omega(omega_level) => Ok(BranchGoalRestoration::OmegaJ {
            index_type: index_type.clone(),
            old_index: old_index.clone(),
            new_index: new_index.clone(),
            target_type: generalized.clone(),
            family_at_y,
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
    cx: &ElabCtx<'_>,
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
                    cx,
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
    expected: &Term,
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
        if cx.active_index_premise_frames.is_empty() {
            let zonked_ctx = Context {
                types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
            };
            let zonked_elim = cx.metas.zonk_term(&elim);
            kernel_infer_in_zonked_current(cx, &zonked_ctx, &zonked_elim).map_err(
                |error| match error {
                    CurrentKernelQueryError::View(error) => error,
                    CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                        error,
                        span: span.clone(),
                    },
                },
            )?;
        } else {
            kernel_check_current(cx, &elim, expected).map_err(|error| match error {
                CurrentKernelQueryError::View(error) => error,
                CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                    error,
                    span: span.clone(),
                },
            })?;
        }
    }
    Ok(elim)
}

fn validate_large_convoy_base(
    cx: &ElabCtx,
    base: &Term,
    base_ty: &Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let zonked_base = cx.metas.zonk_term(base);
    let zonked_ty = cx.metas.zonk_term(base_ty);
    let zonked_ctx = Context {
        types: cx
            .ctx
            .types
            .iter()
            .map(|term| cx.metas.zonk_term(term))
            .collect(),
    };
    kernel_check_in_context_current(cx, &zonked_ctx, &zonked_base, &zonked_ty).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        },
    )?;
    Ok(zonked_base)
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
        &kernel_infer_current(cx, &leaf.index_ty).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "large index convoy could not classify its peeled equality: {error:?}"
            )),
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
        &leaf.scrutinee,
        leaf.proof.clone(),
    );
    let stable_forward = build_sym(
        cx.env,
        &cx.ctx,
        &leaf.index_ty,
        index_level.clone(),
        &leaf.scrutinee,
        &leaf.target,
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

    let source_region = cx.fresh_aux_sentinel_region()?;
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
            index_refinement_sentinel(source_region, slot),
        ));
        source_domains.push(source_ty.clone());
    }
    let base_goal = subst_term_generalize_many(expected_here, &base_substitutions);
    cx.match_frames.last_mut().ok_or_else(|| {
        ElabError::Internal("large convoy arm has no owning match frame".into())
    })?.refined_target = Some(base_goal.clone());

    let refinement_snapshot = cx.var_refinements.clone();
    for (slot, (position, _, _, _, source_ty)) in moving_fields.iter().enumerate() {
        cx.var_refinements.insert(
            *position,
            (
                index_refinement_sentinel(source_region, slot),
                source_ty.clone(),
                cx.ctx.len(),
            ),
        );
    }
    let premise_frame_base = cx.active_index_premise_frames.len();
    cx.active_index_premise_frames.push(ActiveIndexPremiseFrame {
        sentinel_region: source_region,
        premise_domains: source_domains.clone(),
        install_depth: cx.ctx.len(),
    });
    let checked_base = (|| {
        let refined_target = cx.match_frames.last().and_then(|frame| frame.refined_target.clone())
            .ok_or_else(|| ElabError::Internal("large convoy arm lost its refined target".into()))?;
        let core = check_match_arm_result(cx, arm, &refined_target, &arm.span)?;
        let base = wrap_premise_lams_finalized(core, &source_domains, source_region);
        let base_ty = wrap_premise_pis_finalized(base_goal.clone(), &source_domains, source_region);
        validate_large_convoy_base(cx, &base, &base_ty, &arm.span)
    })();
    cx.active_index_premise_frames.truncate(premise_frame_base);
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
        &Term::var(1),
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
            index_refinement_sentinel(source_region, slot),
        ));
        motive_domains.push(subst_term_generalize(
            &weaken(field_ty, 2),
            &weaken(&leaf.target, 2),
            &Term::var(1),
        ));
    }
    let motive_goal = subst_term_generalize_many(&weaken(expected_here, 2), &motive_substitutions);
    let motive_result = wrap_premise_pis_finalized(motive_goal, &motive_domains, source_region);

    let Term::Eq(goal_carrier, _, _) = whnf(cx.env, &cx.ctx, expected_here) else {
        return Ok(None);
    };
    let Term::Type(goal_level) = whnf(
        cx.env,
        &cx.ctx,
        &kernel_infer_current(cx, &goal_carrier).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "large index convoy could not classify its recursive goal: {error:?}"
            )),
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
        &leaf.scrutinee,
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
        method_index_premises(ind, params, &target_indices, scrut_indices, field_count)
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
        let carried =
            carry_coherent_frame_data(leaves, index_equation, telescope_argument_substitutions);
        for ty in &mut types_inner_first {
            *ty = subst_term_generalize_many(ty, &carried.telescope_argument_substitutions);
        }
    }
    expected_here = redirect_convoy_body(context_convoy, field_count, &sentinels, expected_here);
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
            cx,
            &cx.ctx,
            &expected_here,
            embedded_method_convoy,
            embedded_method_repairs,
            embedded_slot_base,
        )?;
        expected_here = refresh_embedded_elim_evidence(cx.env, &cx.ctx, &expected_here, span)?;
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
    let mut goal = subst0(&codomain, &index_refinement_sentinel(sentinel_region, 0));
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
    let premise_base = cx.active_index_premise_frames.len();
    if !premise_domains.is_empty() {
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region,
                premise_domains: premise_domains.to_vec(),
                install_depth: cx.ctx.len(),
            });
    }
    let goal_result = check_large_convoy_recursive_arm(
        cx,
        arm,
        ind,
        params,
        target_indices,
        scrut_indices,
        field_count,
        expected_here,
        sentinel_region,
    );
    cx.active_index_premise_frames.truncate(premise_base);
    let goal_j = goal_result?.ok_or_else(|| {
        ElabError::Internal(
            "large index convoy could not construct its recursive goal transport".into(),
        )
    })?;
    Ok(wrap_premise_lams_finalized(goal_j, premise_domains, sentinel_region))
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
        cx,
        &cx.ctx,
        span,
        shapes,
        field_count,
        expected,
        (equation_convoy || recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared)
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
    cx: &ElabCtx<'_>,
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
    let motive_sort = kernel_infer_in_context_current(cx, motive_ctx, &motive_body).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        },
    )?;
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
        types: cx
            .ctx
            .types
            .iter()
            .map(|term| cx.metas.zonk_term(term))
            .collect(),
    };
    for (ordinal, (actual, target)) in scrut_indices.iter().zip(target_indices).enumerate() {
        let Term::Var(actual_index) = actual else {
            return Err(ElabError::Internal(
                "plain declared-index path received a non-variable actual index".into(),
            ));
        };
        let actual_index = actual_index + field_count;
        let position = cx.ctx.len().checked_sub(1 + actual_index).ok_or_else(|| {
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
        let target_ty = kernel_infer_in_zonked_current(cx, &zonked_ctx, &target).map_err(
            |error| match error {
                CurrentKernelQueryError::View(error) => error,
                CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                    "plain declared-index target is ill-typed: {error:?}"
                )),
            },
        )?;
        if !convert_type(cx.env, &zonked_ctx, &target_ty, &index_ty) {
            return Err(ElabError::Internal(
                "plain declared-index target has the wrong index type".into(),
            ));
        }
        cx.var_refinements
            .insert(position, (target, index_ty, cx.ctx.len()));
    }

    let scrut_position = cx.match_frames.last().and_then(|frame| frame.scrutinee_level)
        .ok_or_else(|| ElabError::Internal(
            "plain declared-index path lost its recorded scrutinee".into(),
        ))?;
    if scrut_position >= cx.ctx.len().saturating_sub(field_count) {
        return Err(ElabError::Internal(
            "plain declared-index scrutinee escaped its constructor context".into(),
        ));
    }
    let concrete = cx.metas.zonk_term(concrete);
    let concrete_ty = kernel_infer_in_zonked_current(cx, &zonked_ctx, &concrete).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "plain declared-index constructor is ill-typed: {error:?}"
            )),
        },
    )?;
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
    _match_span: &Span,
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
    let active_index_refinement_base = cx.active_index_refinements.len();
    let result_refinement_base = cx.result_refinements.len();
    let active_index_premise_frame_base = cx.active_index_premise_frames.len();
    debug_assert_eq!(cx.match_frames.last().map(|frame| frame.start_level), Some(outer_scope_depth));
    let path_base = push_branch_path_condition(cx, scrut_ty, scrut_core, concrete, n, 0);

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
        let expected_unrefined = if preserve_nested_goal || matches!(arm.body, RExpr::RLam(_, _, _))
        {
                expected_here.clone()
            } else {
                simplify_branch_goal(cx.env, &cx.ctx, expected_here)
            };
        cx.match_frames.last_mut().ok_or_else(|| {
            ElabError::Internal("dependent arm has no owning match frame".into())
        })?.refined_target = Some(expected_unrefined.clone());
        if let Some(premise_slot) = hidden_result_premise_slot {
            if premise_slot >= premise_domains.len() {
                return Err(ElabError::Internal(format!(
                    "hidden result-refinement sentinel slot {premise_slot} exceeds region \
                     {sentinel_region} premise telescope of length {}",
                    premise_domains.len()
                )));
            }
            cx.result_refinements.push(ResultRefinement {
                index_ty: weaken(scrut_ty, n as i64),
                concrete_index: concrete.clone(),
                refined_index: weaken(scrut_core, n as i64),
                premise_slot,
                sentinel_region,
                install_depth: cx.ctx.len(),
            });
        }
        if !premise_domains.is_empty() {
            if cx
                .active_index_premise_frames
                .iter()
                .any(|frame| frame.sentinel_region == sentinel_region)
            {
                return Err(ElabError::Internal(format!(
                    "duplicate active index-premise sentinel region {sentinel_region}"
                )));
            }
            cx.active_index_premise_frames
                .push(ActiveIndexPremiseFrame {
                    sentinel_region,
                    premise_domains: premise_domains.to_vec(),
                    install_depth: cx.ctx.len(),
                });
        }
        let obligation_base = cx.obligations.len();
        // Kernel conversion may expose a named refinement's carrier while
        // shaping the branch motive. Its source identity must survive at the
        // introduction check so each arm leaves its own predicate obligation.
        let source_expected = if names_source_refinement(cx, expected_here) {
            expected_here
        } else {
            &expected_unrefined
        };
        let attempt = check_match_arm_result(cx, arm, source_expected, &arm.span).and_then(|checked| {
            kernel_check_current(cx, &checked, &expected_unrefined)
                .map(|()| checked)
                .map_err(|error| match error {
                    CurrentKernelQueryError::View(error) => error,
                    CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                        error,
                        span: arm.span.clone(),
                    },
                })
        });
        let checked = match attempt {
            Ok(checked) => checked,
            Err(error) if recursive_field_index_path == RecursiveFieldIndexPath::PlainDeclared => {
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
        Ok(wrap_premise_lams_finalized(checked, premise_domains, sentinel_region))
    })();

    cx.active_index_premise_frames
        .truncate(active_index_premise_frame_base);
    cx.path_conditions.truncate(path_base);
    cx.result_refinements.truncate(result_refinement_base);
    cx.active_index_refinements
        .truncate(active_index_refinement_base);
    cx.var_refinements = var_refinement_snapshot;
    outcome
}

/// Install the arm's path equation `scrut = concrete` and return the
/// truncation base. Outlined so the recursive branch frame does not hold the
/// equation's temporaries.
#[inline(never)]
fn push_branch_path_condition(
    cx: &mut ElabCtx,
    scrut_ty: &Term,
    scrut_core: &Term,
    concrete: &Term,
    n: usize,
    future_fields: usize,
) -> usize {
    let path_base = cx.path_conditions.len();
    let path_eq = Term::Eq(
        Box::new(weaken(scrut_ty, n as i64)),
        Box::new(weaken(scrut_core, n as i64)),
        Box::new(concrete.clone()),
    );
    // Matrix buckets install an equation before their method's field binders
    // are entered; the proposition already names those future binders.
    cx.path_conditions.push((path_eq, cx.ctx.len() + future_fields));
    path_base
}

/// Whether the arm's expected type is a named source refinement whose
/// identity the introduction check keeps. Outlined for the same reason.
#[inline(never)]
fn names_source_refinement(cx: &ElabCtx, expected: &Term) -> bool {
    matches!(
        cx.metas.zonk_term(expected),
        Term::Const { id, .. }
            if cx.refinement_facts.refinement_root(id).is_some()
    )
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

// Context-telescope convoy (LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE):
// captured bindings follow the constructor's index into the motive codomain.
// Each binder type and the goal may mention earlier convoy entries at different
// depths; finalize the shared telescope once, rather than independently
// substituting a depth-fixed sentinel into each nested binder type. This work
// precedes recursive arm bodies so its temporaries do not grow their frames.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn wrap_dependent_motive_convoy(
    context_convoy: &[ConvoyEntry],
    embedded_method_convoy: &[EmbeddedMethodConvoy],
    scrut_indices: &[Term],
    scrut_core: &Term,
    motive_local_indices: &[Term],
    motive_base_depth: usize,
    sentinel_region: usize,
    equation_convoy: bool,
    mut motive_user_body: Term,
) -> Term {
    let convoy_count = context_convoy.len();
    if equation_convoy && convoy_count > 0 {
        debug_assert!(embedded_method_convoy.is_empty());
    } else if convoy_count > 0 || !embedded_method_convoy.is_empty() {
        let (cv_types_inner_first, sentinels) = convoy_binder_types(
            context_convoy,
            scrut_indices,
            scrut_core,
            motive_local_indices,
            &Term::var(0),
            motive_base_depth,
            0,
            sentinel_region,
        );
        motive_user_body = redirect_convoy_body(
            context_convoy,
            motive_base_depth,
            &sentinels,
            motive_user_body,
        );
        let mut convoy_premises = Vec::with_capacity(convoy_count + embedded_method_convoy.len());
        for i in (0..convoy_count).rev() {
            convoy_premises.push(cv_types_inner_first[i].clone());
        }
        for entry in embedded_method_convoy {
            convoy_premises.push(redirect_convoy_body(
                context_convoy,
                motive_base_depth,
                &sentinels,
                entry.motive_ty.clone(),
            ));
        }
        motive_user_body =
            wrap_premise_pis_finalized(motive_user_body, &convoy_premises, sentinel_region);
    }
    motive_user_body
}

// Motive construction has no recursive arm body. Outlining its temporary
// context, equality, and convoy terms keeps them off each arm descent.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn finish_checked_dependent_motive<const MAY_REFINE_GROUP_RESULT: bool>(
    cx: &ElabCtx,
    ind: &InductiveDecl,
    family: GlobalId,
    params: &[Term],
    scrut_indices: &[Term],
    scrut_core: &Term,
    scrut_ty: &Term,
    motive_base_depth: usize,
    motive_local_indices: &[Term],
    sentinel_region: usize,
    equation: Option<&str>,
    span: &Span,
    plan: &mut CoherentFrameMotivePlan,
) -> Result<(Box<Term>, bool), ElabError> {
    let mut motive_user_body =
        std::mem::replace(&mut plan.motive_user_body, Term::Type(Level::Zero));
    let zonked_ctx = Context {
        types: cx
            .ctx
            .types
            .iter()
            .map(|term| cx.metas.zonk_term(term))
            .collect(),
    };
    let motive_ctx = motive_context(&zonked_ctx, ind, params);
    // The context convoy and embedded methods share one finalized telescope.
    motive_user_body = wrap_dependent_motive_convoy(
        &plan.context_convoy,
        &plan.embedded_method_convoy,
        scrut_indices,
        scrut_core,
        motive_local_indices,
        motive_base_depth,
        sentinel_region,
        plan.equation_convoy,
        motive_user_body,
    );
    // A hidden whole-scrutinee result equality is lawful only for an index
    // domain of the result family and a non-recursive match. A recursive
    // family would also change each IH into an unusable parent equality.
    let hidden_group_result_refinement = MAY_REFINE_GROUP_RESULT
        && ind.indices.is_empty()
        && term_mentions_family_indexed_by(cx.env, &cx.ctx, &plan.expected, scrut_ty)
        && ind.constructors.iter().all(|ctor| {
            recursive_shapes(cx.env, ctor, family, ind.params.len())
                .is_ok_and(|shapes| shapes.is_empty())
        });
    if equation.is_some() {
        // The author-visible `eqn:` premise is discharged by Refl after Elim.
        let eq_dom = Term::Eq(
            Box::new(weaken(scrut_ty, 1)),
            Box::new(weaken(scrut_core, 1)),
            Box::new(Term::var(0)),
        );
        motive_user_body = Term::pi(eq_dom, weaken(&motive_user_body, 1));
    } else if hidden_group_result_refinement {
        let eq_dom = Term::Eq(
            Box::new(weaken(scrut_ty, motive_base_depth as i64)),
            Box::new(Term::var(0)),
            Box::new(weaken(scrut_core, motive_base_depth as i64)),
        );
        motive_user_body = Term::pi(eq_dom, weaken(&motive_user_body, 1));
    }
    let motive = build_checked_dependent_motive(
        cx,
        &motive_ctx,
        ind,
        family,
        params,
        scrut_indices,
        motive_user_body,
        plan.equation_convoy,
        plan.recursive_field_index_path,
        span,
    )?;
    Ok((motive, hidden_group_result_refinement))
}

// Keep arm-field introduction off the recursive dependent-match stack frame.
// In particular, ownership must be checked before any fallible arm work can
// consume these fields; the caller restores both context and frame on error.
#[inline(never)]
fn open_checked_constructor_arm_frame(
    cx: &mut ElabCtx,
    ctor: &ConstructorDecl,
    params: &[Term],
    level_args: &[Level],
    ind: &InductiveDecl,
    sentinel_region: usize,
    target: &Term,
    scrutinee_level: Option<usize>,
    predicates: &[ResultPredicate],
) -> Result<(), ElabError> {
    let start_level = cx.ctx.len();
    let mut frame = MatchFrame::new(
        start_level, sentinel_region, Some(target.clone()), scrutinee_level,
    );
    frame.result_predicates = predicates.to_vec();
    cx.match_frames.push(frame);
    for (j, domain) in ctor.args.iter().enumerate() {
        let raw_ty = subst_levels(
            &subst_outer(domain, ind.params.len(), params, j),
            &ind.level_params,
            level_args,
        );
        cx.push_match_binder(raw_ty, MatchBinderOrigin::Field);
    }
    cx.require_constructor_field_ownership(start_level, ctor.args.len())
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
    predicates: &[ResultPredicate],
) -> Result<Term, ElabError> {
    let hidden_group_result_refinement = equation.is_none()
        && !cx.recursive_group.is_empty()
        && arms
            .iter()
            .any(|arm| expression_mentions_recursive_group(cx, &arm.body));
    if hidden_group_result_refinement {
        check_match_dependent_mode::<true>(cx, scrut, equation, arms, expected, span, predicates)
    } else {
        check_match_dependent_mode::<false>(cx, scrut, equation, arms, expected, span, predicates)
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
    predicates: &[ResultPredicate],
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

    let (d_id, family_level_args, scrut_args) = dependent_scrutinee_family(&scrut_ty, span)?;
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
    let sentinel_region = cx.match_frames.len();
    let scrutinee_level = match scrut_core.as_ref() {
        Term::Var(index) => cx.ctx.len().checked_sub(index + 1),
        _ => None,
    };
    let frame = boxed_match_frame(
        cx.ctx.len(),
        sentinel_region,
        Some(original_expected.clone()),
        scrutinee_level,
    );
    let recursive_field_index_path = recursive_field_index_path(
        cx,
        &ind,
        &params_terms,
        &family_level_args,
        &scrut_core,
        &scrut_indices,
        &frame,
    )?;

    // A source field paired with residual generated `All` evidence is
    // eliminated through that evidence. Its recorded source index keeps the
    // host constructor and support constructor aligned without exposing the
    // generated family at the surface.
    if equation.is_none() {
        if let Some(position) = frame.scrutinee_level {
            if let Some(binding) = cx.lift_bindings.get(&position).copied() {
                if binding.support.is_some() {
                    return check_match_with_lift_with_predicates(
                        cx, arms, expected, span,
                        &Term::var(cx.ctx.len() - 1 - position), &ind,
                        &family_level_args, &params_terms, binding, predicates,
                    );
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
        &frame,
    )?;
    let (motive, hidden_group_result_refinement) =
        finish_checked_dependent_motive::<MAY_REFINE_GROUP_RESULT>(
            cx,
            &ind,
            d_id,
            &params_terms,
            &scrut_indices,
            &scrut_core,
            &scrut_ty,
            motive_base_depth,
            &motive_local_indices,
            sentinel_region,
            equation,
            span,
            &mut motive_plan,
        )?;
    let expected = &motive_plan.expected;
    let context_convoy = motive_plan.context_convoy.as_slice();
    let embedded_method_convoy = motive_plan.embedded_method_convoy.as_slice();
    let embedded_method_repairs = motive_plan.embedded_method_repairs.as_slice();
    let equation_convoy = motive_plan.equation_convoy;
    let recursive_field_index_path = motive_plan.recursive_field_index_path;

    let mut methods = Box::new(vec![None; ind.constructors.len()]);
    let mut arm_used = Box::new(vec![false; arms.len()]);
    let mut subsumed_by = Box::new(vec![None; arms.len()]);
    for (k, ctor) in ind.constructors.iter().enumerate() {
        let arm_selection =
            guarded_constructor_arm(cx, arms, ctor.id, &mut arm_used, &mut subsumed_by);
        let arm = arm_selection.as_ref().map(|(arm, _)| arm);
        let unchanged_arm_index = arm_selection.as_ref().and_then(|(_, index)| *index);
        let n = ctor.args.len();
        if let Some(arm) = arm {
            let sub_pats = match &arm.pat.kind {
                RPatKind::Ctor(_, subs) | RPatKind::CheckedCtor(_, _, subs) => subs.clone(),
                _ => unreachable!("guarded by the constructor-arm selection above"),
            };
            if sub_pats.len() != n {
                return Err(ElabError::Internal(
                    "dependent match (AC4): constructor arity mismatch".into(),
                ));
            }
            for sp in &sub_pats {
                if !matches!(sp.kind, RPatKind::Var(_, _) | RPatKind::Wild) {
                    return Err(ElabError::Internal(
                        "dependent match (AC4): nested constructor sub-patterns are not \
                         yet supported here"
                            .into(),
                    ));
                }
            }
        }
        let shapes = Box::new(recursive_shapes(cx.env, ctor, d_id, m).map_err(|error| {
            ElabError::KernelRejected {
                error,
                span: span.clone(),
            }
        })?);
        if shapes
            .iter()
            .any(|argument| argument.shape.as_legacy().is_none())
        {
            if equation.is_some() {
                return Err(ElabError::Internal(
                    "`match ... eqn:` does not support nested lifted methods".into(),
                ));
            }
            let arm = arm.ok_or_else(|| ElabError::ExhaustivenessError {
                missing: missing_pattern_witness(cx, ctor.id),
                span: span.clone(),
            })?;
            methods[k] = Some(check_structured_constructor_method(
                cx,
                &ind,
                k,
                arm,
                expected,
                &scrut_core,
                &params_terms,
                &motive,
                &family_level_args,
                &shapes,
                predicates,
            )?);
            continue;
        }
        let constructor_scope_depth = cx.ctx.len();
        // Expand restoration at each fallible site without adding a call frame
        // to recursive dependent-match checking.
        macro_rules! frame_try {
            ($result:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(error) => {
                        cx.ctx.types.truncate(constructor_scope_depth);
                        cx.match_frames.pop();
                        return Err(error);
                    }
                }
            };
        }
        frame_try!(open_checked_constructor_arm_frame(
            cx,
            ctor,
            &params_terms,
            &family_level_args,
            &ind,
            sentinel_region,
            expected,
            frame.scrutinee_level,
            predicates,
        ));
        let constructor_frame = frame_try!(build_dependent_constructor_frame(
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
        ));
        let frame = cx.match_frames.last_mut().ok_or_else(|| {
            ElabError::Internal("constructor frame lost its owning match arm".into())
        })?;
        frame.convoy_originals.extend(
            constructor_frame.convoy_refinements.iter().map(|(position, _, _)| *position),
        );
        let concrete = constructor_frame.concrete.as_ref();
        let target_indices = constructor_frame.target_indices.as_slice();
        let premise_domains = constructor_frame.premise_domains.as_slice();
        let expected_here = &constructor_frame.expected_here;
        let convoy_refinements = constructor_frame.convoy_refinements.as_slice();
        let hidden_result_premise_slot = constructor_frame.hidden_result_premise_slot;
        let method = if let Some(arm) = arm {
            if equation_convoy && matches!(&arm.body, RExpr::RCon(name, _) if name == SUGAR_REFL) {
                Term::Const {
                    id: cx.env.tt_id(),
                    level_args: Vec::new(),
                }
            } else if equation_convoy && expression_mentions_recursive_group(cx, &arm.body) {
                frame_try!(build_large_convoy_recursive_method(
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
                ))
            } else if equation.is_some() {
                let eq_dom = Term::Eq(
                    Box::new(weaken(&scrut_ty, n as i64)),
                    Box::new(weaken(&scrut_core, n as i64)),
                    Box::new(concrete.clone()),
                );
                cx.push_match_binder(eq_dom.clone(), MatchBinderOrigin::GeneratedEquation);
                let body = frame_try!(check_match_arm_result(cx, arm, &weaken(&expected_here, 1), &arm.span));
                cx.ctx.pop();
                Term::lam(eq_dom, body)
            } else {
                frame_try!(check_dependent_branch_body(
                    cx,
                    arm,
                    span,
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
                ))
            }
        } else {
            let expected_here = simplify_branch_goal(cx.env, &cx.ctx, &expected_here);
            let missing = missing_pattern_witness(cx, ctor.id);
            frame_try!(synthesize_omitted_index_method(
                cx,
                &premise_domains,
                &expected_here,
                sentinel_region,
                missing,
                span,
            ))
        };
        for _ in 0..n {
            cx.ctx.pop();
        }
        cx.match_frames.pop();
        debug_assert_eq!(
            cx.ctx.len(),
            constructor_scope_depth,
            "dependent constructor field context must unwind on both success and error",
        );

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
            unchanged_arm_index,
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
        expected,
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
    b: &Term,
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
    let proof_ty = Term::Eq(
        Box::new(idx_ty.clone()),
        Box::new(a.clone()),
        Box::new(b.clone()),
    );
    let proof = Term::Ascript(Box::new(h), Box::new(proof_ty));
    Term::J(Box::new(motive), Box::new(base), Box::new(proof))
}

/// Forward type transport: both its target and its motive come from the
/// caller's source, never by inverting a substituted image.
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
    let family_at_y =
        subst_term_generalize(&weaken(cur_ty, 2), &weaken(old_idx, 2), &Term::var(1));
    let proof = build_family_type_cong(
        env, ctx, idx_ty, old_idx, new_idx, cur_ty, &family_at_y,
        type_level, h,
    );
    (proof, new_ty)
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
    let family_at_y =
        subst_term_generalize(&weaken(cur_ty, 2), &weaken(old_idx, 2), &Term::var(1));
    build_family_omega_transport(
        idx_ty, old_idx, new_idx, &family_at_y, &new_ty,
        omega_level, value, h,
    )
}

/// If `cur_ty` (a type at the branch's current context depth) literally
/// mentions `old_idx`, re-type `value : cur_ty` at
/// `cur_ty[new_idx/old_idx]` using `h : Eq idx_ty old_idx new_idx`. Type-
/// classified positions retain the existing equality-of-types plus `Cast`;
/// Ω-classified positions use direct `J` transport. Returns `None` — never a
/// spurious refinement (AC8) — if `cur_ty` does not depend on `old_idx` at all.
fn try_reindex_cast(
    active_cx: Option<&ElabCtx<'_>>,
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
    let level_ty = if let Some(cx) = active_cx {
        kernel_infer_in_zonked_current(cx, ctx, cur_ty).map_err(|error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "index refinement: could not classify a re-indexed position's type: {error:?}"
            )),
        })?
    } else {
        kernel_infer_raw(env, ctx, cur_ty).map_err(|error| {
            ElabError::Internal(format!(
                "index refinement: could not classify a re-indexed position's type: {error:?}"
            ))
        })?
    };
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

#[derive(Clone)]
enum GeneralizedGoalSource {
    Original(usize),
    Premise { region: usize, slot: usize },
}

#[derive(Clone)]
struct GeneralizedGoalBinder {
    source: GeneralizedGoalSource,
    /// A checked value at the original leaf endpoint. For an Original with
    /// an installed refinement this is the effective alias, not its raw Var.
    source_value: Term,
}

/// Free expanded-context positions, not surface indices. This also visits
/// proof terms hidden inside casts and generated index equalities.
fn expanded_goal_free_positions(
    term: &Term,
    context_len: usize,
) -> Result<HashSet<usize>, ElabError> {
    let mut positions = HashSet::new();
    relocate_active_premise_term(term, 0, &mut |index, depth| {
        let position = context_len.checked_sub(1 + index).ok_or_else(|| {
            ElabError::Internal("branch goal has a free variable outside the expanded view".into())
        })?;
        positions.insert(position);
        Ok(Term::var(depth + index))
    })?;
    Ok(positions)
}

fn generalized_goal_binders(
    cx: &ElabCtx<'_>,
    view: &ActivePremiseKernelView,
    goal: &Term,
    leaf_scrutinee: &Term,
    leaf_target: &Term,
) -> Result<Vec<(Term, Term, GeneralizedGoalBinder)>, ElabError> {
    let len = view.context.len();
    let original_len = view.embedding.original_len;
    let mut needed = expanded_goal_free_positions(goal, len)?;
    // A binder's domain may name another, outer binder. Follow these edges
    // from inner to outer before deciding which goal-referenced types change.
    let mut selected = Vec::new();
    for position in (0..len).rev() {
        let source = &view.embedding.expanded_sources[position];
        let index = len - 1 - position;
        let mut current_type = weaken(
            view.context.lookup(index).expect("expanded position in range"),
            (index + 1) as i64,
        );
        let mut value = Term::var(index);
        if let ExpandedBindingSource::Original(original_position) = source {
            if let Some((alias, ty, install_depth)) = cx.var_refinements.get(original_position) {
                let growth = original_len.checked_sub(*install_depth).ok_or_else(|| {
                    ElabError::Internal("branch goal alias escaped its installing context".into())
                })?;
                let alias = view.embedding.translate_from_original(
                    &cx.metas.zonk_term(&weaken(alias, growth as i64)),
                    original_len,
                    len,
                )?;
                // Only an alias actually occurring in the goal is its
                // abstraction source; a raw Var must keep its raw type.
                if scrut_occurs(goal, &alias) {
                    current_type = view.embedding.translate_from_original(
                        &cx.metas.zonk_term(&weaken(ty, growth as i64)),
                        original_len,
                        len,
                    )?;
                    value = alias;
                }
            }
        }
        if !needed.contains(&position)
            || subst_term_generalize(&current_type, leaf_scrutinee, leaf_target) == current_type
        {
            continue;
        }
        needed.extend(expanded_goal_free_positions(&current_type, len)?);
        let source_original = view.embedding.translate_to_original(&value, original_len, len)?;
        let binder_source = match source {
            ExpandedBindingSource::Original(original_position) => {
                GeneralizedGoalSource::Original(*original_position)
            }
            ExpandedBindingSource::Premise {
                sentinel_region,
                premise_slot,
            } => GeneralizedGoalSource::Premise {
                region: *sentinel_region,
                slot: *premise_slot,
            },
        };
        selected.push((
            current_type,
            value,
            GeneralizedGoalBinder {
                source: binder_source,
                source_value: source_original,
            },
        ));
    }
    selected.reverse();
    Ok(selected)
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
    let sentinel_region = cx.match_frames.last().ok_or_else(|| {
        ElabError::Internal("goal refinement has no owning match frame".into())
    })?.sentinel_region;
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

    // No equality leaf can refine this goal. Preserve the old no-op path:
    // constructing an expanded kernel view here would prematurely classify
    // unrelated context domains (including a malformed expression Pi still
    // awaiting its final kernel admission).
    if leaves.is_empty() {
        return Ok((expected_here.clone(), Vec::new()));
    }
    let view = active_premise_kernel_view_for_context(cx, &zonked_ctx)?;
    let mut goal = expected_here.clone();
    let mut restorations = Vec::new();
    for leaf in leaves {
        let Some(view) = &view else {
            // The leaf's generated proof is a premise sentinel; an absent
            // expanded view cannot justify refining the branch goal.
            return Err(ElabError::Internal(
                "branch-goal refinement has no active premise view".into(),
            ));
        };
        let embedding = &view.embedding;
        let original_len = embedding.original_len;
        let expanded_len = embedding.expanded_len;
        let from_original = |term: &Term| {
            embedding.translate_from_original(
                &cx.metas.zonk_term(term),
                original_len,
                expanded_len,
            )
        };
        let goal_expanded = from_original(&goal)?;
        let index_ty = from_original(&leaf.index_ty)?;
        let target = from_original(&leaf.target)?;
        let scrutinee = from_original(&leaf.scrutinee)?;
        let proof = from_original(&leaf.proof)?;
        let binders = generalized_goal_binders(cx, view, &goal_expanded, &scrutinee, &target)?;
        let mut generalized = goal_expanded.clone();
        // Wrap inside-out to retain the outer-first telescope order; replace
        // the actual binder value before rewriting the index leaf. This is
        // what prevents an existing alias's earlier proof being rewritten.
        for (domain, value, _) in binders.iter().rev() {
            generalized = Term::pi(
                domain.clone(),
                subst_term_generalize(
                    &weaken(&generalized, 1),
                    &weaken(value, 1),
                    &Term::var(0),
                ),
            );
        }
        let candidate = subst_term_generalize(&generalized, &scrutinee, &target);
        if candidate == generalized {
            continue;
        }
        let level_ty = kernel_infer_raw(cx.env, &view.context, &candidate).map_err(|error| {
            ElabError::Internal(format!(
                "index refinement: could not classify the branch goal: {error:?}"
            ))
        })?;
        let classifier = whnf(cx.env, &view.context, &level_ty);
        let restoration = classify_branch_goal_restoration(
            cx.env,
            &view.context,
            &index_ty,
            &target,
            &scrutinee,
            &candidate,
            &generalized,
            classifier,
            proof,
        )?;
        let restoration = restoration.translate_to_original(embedding)?;
        restorations.push(BranchGoalRestoration::Generalized {
            whole: Box::new(restoration),
            binders: binders.into_iter().map(|(_, _, binder)| binder).collect(),
        });
        goal = embedding.translate_to_original(&candidate, original_len, expanded_len)?;
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
    let index_level_ty = kernel_infer_in_zonked_current(cx, &zonked_ctx, &index_ty).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                "result refinement: could not classify the matched type: {error:?}"
            )),
        },
    )?;
    match whnf(cx.env, &zonked_ctx, &index_level_ty) {
        Term::Type(_) => {}
        other => {
            return Err(ElabError::Internal(format!(
                "result refinement: matched type is not classified by Type, found {other:?}"
            )))
        }
    }

    let sentinel_region = cx.match_frames.last().ok_or_else(|| {
        ElabError::Internal("result refinement has no owning match frame".into())
    })?.sentinel_region;
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
        let level_ty = kernel_infer_in_zonked_current(cx, &zonked_ctx, &leaf.index_ty).map_err(
            |error| match error {
                CurrentKernelQueryError::View(error) => error,
                CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                    "result refinement: could not classify a projected index type: {error:?}"
                )),
            },
        )?;
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
            &leaf.scrutinee,
            leaf.proof.clone(),
        );
        symmetric_leaves.push((leaf, proof_sym));
    }

    let mut installed = Vec::new();
    for position in 0..outer_scope_depth {
        if cx.match_binder_origin(position)?.is_some_and(|origin| {
            origin != MatchBinderOrigin::UserLocal
        }) || cx.var_refinements.contains_key(&position) {
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
            &kernel_infer_in_zonked_current(cx, &zonked_ctx, &outer_ty).map_err(
                |error| match error {
                    CurrentKernelQueryError::View(error) => error,
                    CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                        "result refinement: could not classify an outer binding: {error:?}"
                    )),
                },
            )?,
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
                Some(cx),
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
    let sentinel_region = cx.match_frames.last().ok_or_else(|| {
        ElabError::Internal("index refinement has no owning match frame".into())
    })?.sentinel_region;

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
            let level_ty = kernel_infer_in_zonked_current(cx, &zonked_ctx, &leaf.index_ty)
                .map_err(|error| match error {
                    CurrentKernelQueryError::View(error) => error,
                    CurrentKernelQueryError::Kernel(error) => ElabError::Internal(format!(
                        "index refinement: could not classify an index type: {error:?}"
                    )),
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
                Some(cx),
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

    if !leaves.is_empty() {
        cx.active_index_refinements.push(ActiveIndexRefinement {
            leaves,
            install_depth: cx.ctx.len(),
        });
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
// Auxiliary proof-view source fields must never alias a depth-keyed method
// premise. Exhaustion refuses rather than permitting a wrapped sentinel.
const AUX_SENTINEL_REGION_BASE: usize = 1 << 30;
const AUX_SENTINEL_REGION_END: usize = 1 << 31;

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
        Term::Quot(a, r, e) => Term::Quot(
            Box::new(go(a, depth)),
            Box::new(go(r, depth)),
            Box::new(go(e, depth)),
        ),
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

fn checked_index_refinement_sentinel(
    sentinel_region: usize,
    effective_slot: usize,
) -> Result<usize, ElabError> {
    if effective_slot >= INDEX_REFINEMENT_SENTINEL_STRIDE {
        return Err(ElabError::Internal(format!(
            "active premise sentinel offset {effective_slot} exceeds its region stride"
        )));
    }
    sentinel_region
        .checked_mul(INDEX_REFINEMENT_SENTINEL_STRIDE)
        .and_then(|offset| offset.checked_add(effective_slot))
        .and_then(|offset| INDEX_REFINEMENT_SENTINEL_BASE.checked_add(offset))
        .ok_or_else(|| {
            ElabError::Internal(format!(
                "active premise sentinel region {sentinel_region} overflows the variable index"
            ))
        })
}

/// FRAME BUDGET (LANG-ACTIVE-PREMISE-RELOCATION-STACK-FRAME): this walk
/// recurses once per level of the relocated term, and in an unoptimized build
/// every arm's locals are paid by every level. Each arm is therefore a single
/// call into an `#[inline(never)]` helper that holds only that shape's
/// temporaries; keep new arms in that form. Evaluation order is unchanged, so
/// `map_free`'s effects and the first error are the same as before.
fn relocate_active_premise_term<F>(
    term: &Term,
    depth: usize,
    map_free: &mut F,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
{
    match term {
        Term::Var(index) if *index < depth => Ok(Term::var(*index)),
        Term::Var(index) => map_free(*index - depth, depth),
        Term::Pi(domain, codomain) => relocate_binder(domain, codomain, depth, map_free, Term::pi),
        Term::Lam(domain, body) => relocate_binder(domain, body, depth, map_free, Term::lam),
        Term::Sigma(domain, codomain) => {
            relocate_binder(domain, codomain, depth, map_free, Term::sigma)
        }
        Term::Let { ty, val, body } => relocate_let(ty, val, body, depth, map_free),
        Term::App(function, argument) => {
            relocate_two(function, argument, depth, map_free, Term::app)
        }
        Term::Pair(first, second) => relocate_two(first, second, depth, map_free, Term::pair),
        Term::Proj1(pair) => relocate_one(pair, depth, map_free, Term::proj1),
        Term::Proj2(pair) => relocate_one(pair, depth, map_free, Term::proj2),
        Term::Ascript(checked, expected) => {
            relocate_two(checked, expected, depth, map_free, |checked, expected| {
                Term::Ascript(Box::new(checked), Box::new(expected))
            })
        }
        Term::Eq(ty, left, right) => {
            relocate_three(ty, left, right, depth, map_free, |ty, left, right| {
                Term::Eq(Box::new(ty), Box::new(left), Box::new(right))
            })
        }
        Term::Cast(source, target, evidence, value) => relocate_four(
            [source, target, evidence, value],
            depth,
            map_free,
            |[source, target, evidence, value]| {
                Term::Cast(
                    Box::new(source),
                    Box::new(target),
                    Box::new(evidence),
                    Box::new(value),
                )
            },
        ),
        Term::J(motive, base, evidence) => relocate_three(
            motive,
            base,
            evidence,
            depth,
            map_free,
            |motive, base, evidence| Term::J(Box::new(motive), Box::new(base), Box::new(evidence)),
        ),
        Term::Quot(carrier, relation, equivalence) => relocate_three(
            carrier,
            relation,
            equivalence,
            depth,
            map_free,
            |carrier, relation, equivalence| {
                Term::Quot(
                    Box::new(carrier),
                    Box::new(relation),
                    Box::new(equivalence),
                )
            },
        ),
        Term::QuotClass(value) => relocate_one(value, depth, map_free, |value| {
            Term::QuotClass(Box::new(value))
        }),
        Term::Trunc(ty) => relocate_one(ty, depth, map_free, |ty| Term::Trunc(Box::new(ty))),
        Term::TruncProj(value) => relocate_one(value, depth, map_free, |value| {
            Term::TruncProj(Box::new(value))
        }),
        Term::Refl(value) => {
            relocate_one(value, depth, map_free, |value| Term::Refl(Box::new(value)))
        }
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => relocate_four(
            [motive, method, respect, scrut],
            depth,
            map_free,
            |[motive, method, respect, scrut]| Term::QuotElim {
                motive: Box::new(motive),
                method: Box::new(method),
                respect: Box::new(respect),
                scrut: Box::new(scrut),
            },
        ),
        Term::Elim { .. } => relocate_elim(term, depth, map_free),
        Term::Absurd(motive, proof) => {
            relocate_two(motive, proof, depth, map_free, |motive, proof| {
                Term::Absurd(Box::new(motive), Box::new(proof))
            })
        }
        Term::Type(_)
        | Term::Omega(_)
        | Term::Const { .. }
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => Ok(term.clone()),
    }
}

#[inline(never)]
fn relocate_one<F, C>(
    term: &Term,
    depth: usize,
    map_free: &mut F,
    build: C,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
    C: FnOnce(Term) -> Term,
{
    Ok(build(relocate_active_premise_term(term, depth, map_free)?))
}

#[inline(never)]
fn relocate_two<F, C>(
    first: &Term,
    second: &Term,
    depth: usize,
    map_free: &mut F,
    build: C,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
    C: FnOnce(Term, Term) -> Term,
{
    let first = relocate_active_premise_term(first, depth, map_free)?;
    let second = relocate_active_premise_term(second, depth, map_free)?;
    Ok(build(first, second))
}

/// The domain is relocated at `depth`, the body under one more binder.
#[inline(never)]
fn relocate_binder<F, C>(
    domain: &Term,
    body: &Term,
    depth: usize,
    map_free: &mut F,
    build: C,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
    C: FnOnce(Term, Term) -> Term,
{
    let domain = relocate_active_premise_term(domain, depth, map_free)?;
    let body = relocate_active_premise_term(body, depth + 1, map_free)?;
    Ok(build(domain, body))
}

#[inline(never)]
fn relocate_three<F, C>(
    first: &Term,
    second: &Term,
    third: &Term,
    depth: usize,
    map_free: &mut F,
    build: C,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
    C: FnOnce(Term, Term, Term) -> Term,
{
    let first = relocate_active_premise_term(first, depth, map_free)?;
    let second = relocate_active_premise_term(second, depth, map_free)?;
    let third = relocate_active_premise_term(third, depth, map_free)?;
    Ok(build(first, second, third))
}

#[inline(never)]
fn relocate_four<F, C>(
    terms: [&Term; 4],
    depth: usize,
    map_free: &mut F,
    build: C,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
    C: FnOnce([Term; 4]) -> Term,
{
    let [first, second, third, fourth] = terms;
    let first = relocate_active_premise_term(first, depth, map_free)?;
    let second = relocate_active_premise_term(second, depth, map_free)?;
    let third = relocate_active_premise_term(third, depth, map_free)?;
    let fourth = relocate_active_premise_term(fourth, depth, map_free)?;
    Ok(build([first, second, third, fourth]))
}

#[inline(never)]
fn relocate_let<F>(
    ty: &Term,
    val: &Term,
    body: &Term,
    depth: usize,
    map_free: &mut F,
) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
{
    let ty = relocate_active_premise_term(ty, depth, map_free)?;
    let val = relocate_active_premise_term(val, depth, map_free)?;
    let body = relocate_active_premise_term(body, depth + 1, map_free)?;
    Ok(Term::Let {
        ty: Box::new(ty),
        val: Box::new(val),
        body: Box::new(body),
    })
}

/// A plain loop rather than an iterator `collect`, whose adapter chain adds
/// about fifteen frames per eliminator level in an unoptimized build.
#[inline(never)]
fn relocate_all<F>(terms: &[Term], depth: usize, map_free: &mut F) -> Result<Vec<Term>, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
{
    let mut relocated = Vec::with_capacity(terms.len());
    for term in terms {
        relocated.push(relocate_active_premise_term(term, depth, map_free)?);
    }
    Ok(relocated)
}

#[inline(never)]
fn relocate_elim<F>(term: &Term, depth: usize, map_free: &mut F) -> Result<Term, ElabError>
where
    F: FnMut(usize, usize) -> Result<Term, ElabError>,
{
    let Term::Elim {
        fam,
        level_args,
        params,
        motive,
        methods,
        indices,
        scrut,
    } = term
    else {
        unreachable!("relocate_elim is called only on an eliminator");
    };
    let params = relocate_all(params, depth, map_free)?;
    let motive = relocate_active_premise_term(motive, depth, map_free)?;
    let methods = relocate_all(methods, depth, map_free)?;
    let indices = relocate_all(indices, depth, map_free)?;
    let scrut = relocate_active_premise_term(scrut, depth, map_free)?;
    Ok(Term::Elim {
        fam: *fam,
        level_args: level_args.clone(),
        params,
        motive: Box::new(motive),
        methods,
        indices,
        scrut: Box::new(scrut),
    })
}

impl ActivePremiseEmbedding {
    fn sentinel_at_prefix(
        &self,
        sentinel_region: usize,
        premise_slot: usize,
        original_prefix_len: usize,
    ) -> Result<usize, ElabError> {
        let install_depth = self
            .premise_install_depth
            .get(&(sentinel_region, premise_slot))
            .copied()
            .ok_or_else(|| {
                ElabError::Internal(format!(
                    "active premise region {sentinel_region} slot {premise_slot} has no install depth"
                ))
            })?;
        let growth = original_prefix_len.checked_sub(install_depth).ok_or_else(|| {
            ElabError::Internal(format!(
                "active premise region {sentinel_region} slot {premise_slot} is a forward dependency"
            ))
        })?;
        let effective_slot = premise_slot.checked_add(growth).ok_or_else(|| {
            ElabError::Internal(format!(
                "active premise region {sentinel_region} slot {premise_slot} overflows at context growth {growth}"
            ))
        })?;
        checked_index_refinement_sentinel(sentinel_region, effective_slot)
    }

    fn translate_from_original(
        &self,
        term: &Term,
        original_prefix_len: usize,
        expanded_prefix_len: usize,
    ) -> Result<Term, ElabError> {
        if original_prefix_len > self.original_len || expanded_prefix_len > self.expanded_len {
            return Err(ElabError::Internal(
                "active premise translation prefix exceeds its embedding".into(),
            ));
        }
        relocate_active_premise_term(term, 0, &mut |free_index, depth| {
            for (&(sentinel_region, premise_slot), &expanded_position) in
                &self.premise_to_expanded
            {
                let Some(&install_depth) = self
                    .premise_install_depth
                    .get(&(sentinel_region, premise_slot))
                else {
                    return Err(ElabError::Internal(
                        "active premise embedding lost a premise install depth".into(),
                    ));
                };
                let expected = if install_depth <= original_prefix_len {
                    self.sentinel_at_prefix(
                        sentinel_region,
                        premise_slot,
                        original_prefix_len,
                    )?
                } else {
                    checked_index_refinement_sentinel(sentinel_region, premise_slot)?
                };
                if free_index == expected {
                    if install_depth > original_prefix_len
                        || expanded_position >= expanded_prefix_len
                    {
                        return Err(ElabError::Internal(format!(
                            "active premise region {sentinel_region} slot {premise_slot} is a forward dependency"
                        )));
                    }
                    let position = self
                        .scoped_premise_aliases
                        .get(&(sentinel_region, premise_slot))
                        .copied()
                        .filter(|position| *position < original_prefix_len)
                        .map(|position| self.original_to_expanded[position])
                        .unwrap_or(expanded_position);
                    if position >= expanded_prefix_len {
                        return Err(ElabError::Internal(
                            "active premise alias is outside its expanded prefix".into(),
                        ));
                    }
                    return Ok(Term::var(depth + expanded_prefix_len - 1 - position));
                }
            }
            if free_index >= original_prefix_len {
                return Err(ElabError::Internal(format!(
                    "active premise query contains unknown sentinel or ordinary out-of-scope index {free_index} at prefix {original_prefix_len}"
                )));
            }
            let original_position = original_prefix_len - 1 - free_index;
            let expanded_position = self
                .original_to_expanded
                .get(original_position)
                .copied()
                .ok_or_else(|| {
                    ElabError::Internal(format!(
                        "active premise query lost original binding {original_position}"
                    ))
                })?;
            if expanded_position >= expanded_prefix_len {
                return Err(ElabError::Internal(format!(
                    "active premise query references original binding {original_position} beyond the expanded prefix"
                )));
            }
            Ok(Term::var(
                depth + expanded_prefix_len - 1 - expanded_position,
            ))
        })
    }

    fn translate_to_original(
        &self,
        term: &Term,
        original_prefix_len: usize,
        expanded_prefix_len: usize,
    ) -> Result<Term, ElabError> {
        if original_prefix_len > self.original_len || expanded_prefix_len > self.expanded_len {
            return Err(ElabError::Internal(
                "active premise inverse-translation prefix exceeds its embedding".into(),
            ));
        }
        relocate_active_premise_term(term, 0, &mut |free_index, depth| {
            if free_index >= expanded_prefix_len {
                return Err(ElabError::Internal(format!(
                    "active premise inferred output contains expanded out-of-scope index {free_index}"
                )));
            }
            let expanded_position = expanded_prefix_len - 1 - free_index;
            let source = self.expanded_sources.get(expanded_position).ok_or_else(|| {
                ElabError::Internal(format!(
                    "active premise inferred output has no source for expanded binding {expanded_position}"
                ))
            })?;
            match source {
                ExpandedBindingSource::Original(original_position) => {
                    if *original_position >= original_prefix_len {
                        return Err(ElabError::Internal(format!(
                            "active premise inferred output cannot invert original binding {original_position}"
                        )));
                    }
                    Ok(Term::var(
                        depth + original_prefix_len - 1 - original_position,
                    ))
                }
                ExpandedBindingSource::Premise {
                    sentinel_region,
                    premise_slot,
                } => Ok(Term::var(
                    depth
                        + self.sentinel_at_prefix(
                            *sentinel_region,
                            *premise_slot,
                            original_prefix_len,
                        )?,
                )),
            }
        })
    }
}

/// Validate the active-frame plan, construct its total embedding, and rebuild
/// the supplied query context one binding at a time against exact prefixes.
fn active_premise_kernel_view_for_context(
    cx: &ElabCtx<'_>,
    original_context: &Context,
) -> Result<Option<ActivePremiseKernelView>, ElabError> {
    if cx.active_index_premise_frames.is_empty() {
        if let Some(refinement) = cx.result_refinements.first() {
            return Err(ElabError::Internal(format!(
                "result refinement region {} has no active premise frame",
                refinement.sentinel_region
            )));
        }
        return Ok(None);
    }

    let original_len = original_context.len();
    let mut regions = HashMap::new();
    let mut previous_install_depth = None;
    let mut premise_install_depth = HashMap::new();
    let mut premise_domains = HashMap::new();
    let mut expanded_len = original_len;
    for (frame_index, frame) in cx.active_index_premise_frames.iter().enumerate() {
        if regions
            .insert(frame.sentinel_region, frame_index)
            .is_some()
        {
            return Err(ElabError::Internal(format!(
                "duplicate active index-premise sentinel region {}",
                frame.sentinel_region
            )));
        }
        if frame.install_depth > original_len {
            return Err(ElabError::Internal(format!(
                "active index-premise frame region {} escaped its install context",
                frame.sentinel_region
            )));
        }
        if previous_install_depth.is_some_and(|depth| frame.install_depth < depth) {
            return Err(ElabError::Internal(format!(
                "active index-premise frame region {} is not nested by install depth",
                frame.sentinel_region
            )));
        }
        previous_install_depth = Some(frame.install_depth);
        expanded_len = expanded_len
            .checked_add(frame.premise_domains.len())
            .ok_or_else(|| {
                ElabError::Internal("active premise expanded context length overflows".into())
            })?;
        let maximum_growth = original_len - frame.install_depth;
        for (premise_slot, domain) in frame.premise_domains.iter().enumerate() {
            let effective_slot = premise_slot.checked_add(maximum_growth).ok_or_else(|| {
                ElabError::Internal(format!(
                    "active premise region {} slot {premise_slot} overflows at maximum context growth",
                    frame.sentinel_region
                ))
            })?;
            checked_index_refinement_sentinel(frame.sentinel_region, effective_slot)?;
            premise_install_depth.insert(
                (frame.sentinel_region, premise_slot),
                frame.install_depth,
            );
            premise_domains.insert(
                (frame.sentinel_region, premise_slot),
                domain.clone(),
            );
        }
    }

    for refinement in &cx.result_refinements {
        let Some(&frame_index) = regions.get(&refinement.sentinel_region) else {
            return Err(ElabError::Internal(format!(
                "result refinement region {} has no active premise frame",
                refinement.sentinel_region
            )));
        };
        let frame = &cx.active_index_premise_frames[frame_index];
        if refinement.premise_slot >= frame.premise_domains.len() {
            return Err(ElabError::Internal(format!(
                "result-refinement sentinel slot {} exceeds region {} premise telescope of length {}",
                refinement.premise_slot,
                refinement.sentinel_region,
                frame.premise_domains.len()
            )));
        }
        if refinement.install_depth != frame.install_depth {
            return Err(ElabError::Internal(format!(
                "result refinement region {} install depth {} differs from its frame depth {}",
                refinement.sentinel_region,
                refinement.install_depth,
                frame.install_depth
            )));
        }
    }

    let mut original_to_expanded = vec![usize::MAX; original_len];
    let mut expanded_sources = Vec::with_capacity(expanded_len);
    let mut premise_to_expanded = HashMap::new();
    for original_position in 0..=original_len {
        for frame in cx
            .active_index_premise_frames
            .iter()
            .filter(|frame| frame.install_depth == original_position)
        {
            for premise_slot in 0..frame.premise_domains.len() {
                let expanded_position = expanded_sources.len();
                premise_to_expanded.insert(
                    (frame.sentinel_region, premise_slot),
                    expanded_position,
                );
                expanded_sources.push(ExpandedBindingSource::Premise {
                    sentinel_region: frame.sentinel_region,
                    premise_slot,
                });
            }
        }
        if original_position < original_len {
            original_to_expanded[original_position] = expanded_sources.len();
            expanded_sources.push(ExpandedBindingSource::Original(original_position));
        }
    }
    if expanded_sources.len() != expanded_len
        || original_to_expanded.contains(&usize::MAX)
        || premise_to_expanded.len() != premise_install_depth.len()
    {
        return Err(ElabError::Internal(
            "active premise embedding is not total".into(),
        ));
    }

    let embedding = ActivePremiseEmbedding {
        original_len,
        expanded_len,
        original_to_expanded,
        expanded_sources,
        premise_to_expanded,
        premise_install_depth,
        scoped_premise_aliases: cx.scoped_match_premises()?,
    };
    let mut context = Context::new();
    for source in &embedding.expanded_sources {
        let (raw_domain, original_prefix_len) = match source {
            ExpandedBindingSource::Original(original_position) => (
                original_context.types[*original_position].clone(),
                *original_position,
            ),
            ExpandedBindingSource::Premise {
                sentinel_region,
                premise_slot,
            } => (
                premise_domains
                    .get(&(*sentinel_region, *premise_slot))
                    .cloned()
                    .ok_or_else(|| {
                        ElabError::Internal(format!(
                            "active premise region {sentinel_region} slot {premise_slot} lost its domain"
                        ))
                    })?,
                *embedding
                    .premise_install_depth
                    .get(&(*sentinel_region, *premise_slot))
                    .ok_or_else(|| {
                        ElabError::Internal(format!(
                            "active premise region {sentinel_region} slot {premise_slot} lost its install depth"
                        ))
                    })?,
            ),
        };
        let zonked_domain = cx.metas.zonk_term(&raw_domain);
        let relocated_domain = embedding.translate_from_original(
            &zonked_domain,
            original_prefix_len,
            context.len(),
        )?;
        let classifier = kernel_infer_raw(cx.env, &context, &relocated_domain).map_err(|error| {
            ElabError::Internal(format!(
                "active premise expanded context contains an ill-typed domain: {error:?}"
            ))
        })?;
        match whnf(cx.env, &context, &classifier) {
            Term::Type(_) | Term::Omega(_) => {}
            other => {
                return Err(ElabError::Internal(format!(
                    "active premise expanded context domain is classified by neither Type nor Omega, found {other:?}"
                )))
            }
        }
        context.push(relocated_domain);
    }

    Ok(Some(ActivePremiseKernelView { context, embedding }))
}

#[cfg(test)]
fn active_premise_kernel_view(
    cx: &ElabCtx<'_>,
) -> Result<Option<ActivePremiseKernelView>, ElabError> {
    active_premise_kernel_view_for_context(cx, &cx.ctx)
}

/// The ordinary query route has no expanded frame to zonk its own context.
fn zonked_kernel_query_context(cx: &ElabCtx<'_>, context: &Context) -> Context {
    Context {
        types: context
            .types
            .iter()
            .map(|ty| cx.metas.zonk_term(ty))
            .collect(),
    }
}

/// Check original owner-local operands in a disposable premise-expanded view.
/// No translated operand or context entry is returned.
/// Even without a premise frame, the kernel must see resolved level terms in
/// both the local telescope and the operands (`39 §5.7`).
#[inline(never)]
fn kernel_check_in_context_current(
    cx: &ElabCtx<'_>,
    original_context: &Context,
    checked: &Term,
    expected: &Term,
) -> Result<(), CurrentKernelQueryError> {
    let Some(view) = active_premise_kernel_view_for_context(cx, original_context)
        .map_err(CurrentKernelQueryError::View)?
    else {
        let context = zonked_kernel_query_context(cx, original_context);
        let checked = cx.metas.zonk_term(checked);
        let expected = cx.metas.zonk_term(expected);
        return kernel_check_raw(cx.env, &context, &checked, &expected)
            .map_err(CurrentKernelQueryError::Kernel);
    };
    let checked = cx.metas.zonk_term(checked);
    let expected = cx.metas.zonk_term(expected);
    let checked = view
        .embedding
        .translate_from_original(&checked, view.embedding.original_len, view.embedding.expanded_len)
        .map_err(CurrentKernelQueryError::View)?;
    let expected = view
        .embedding
        .translate_from_original(&expected, view.embedding.original_len, view.embedding.expanded_len)
        .map_err(CurrentKernelQueryError::View)?;
    kernel_check_raw(cx.env, &view.context, &checked, &expected)
        .map_err(CurrentKernelQueryError::Kernel)
}

fn kernel_check_current(
    cx: &ElabCtx<'_>,
    checked: &Term,
    expected: &Term,
) -> Result<(), CurrentKernelQueryError> {
    kernel_check_in_context_current(cx, &cx.ctx, checked, expected)
}

fn kernel_infer_in_zonked_current(
    cx: &ElabCtx<'_>,
    zonked_ctx: &Context,
    inferred: &Term,
) -> Result<Term, CurrentKernelQueryError> {
    kernel_infer_in_context_current(cx, zonked_ctx, inferred)
}

/// Infer an original owner-local term in a disposable premise-expanded view,
/// then invert only its inferred type back to original coordinates/sentinels.
/// The no-premise path must apply the same level substitution as the expanded
/// path before a kernel query sees the local telescope or inferred term.
#[inline(never)]
fn kernel_infer_in_context_current(
    cx: &ElabCtx<'_>,
    original_context: &Context,
    inferred: &Term,
) -> Result<Term, CurrentKernelQueryError> {
    let Some(view) = active_premise_kernel_view_for_context(cx, original_context)
        .map_err(CurrentKernelQueryError::View)?
    else {
        let context = zonked_kernel_query_context(cx, original_context);
        let inferred = cx.metas.zonk_term(inferred);
        return kernel_infer_raw(cx.env, &context, &inferred)
            .map_err(CurrentKernelQueryError::Kernel);
    };
    let inferred = cx.metas.zonk_term(inferred);
    let inferred = view
        .embedding
        .translate_from_original(
            &inferred,
            view.embedding.original_len,
            view.embedding.expanded_len,
        )
        .map_err(CurrentKernelQueryError::View)?;
    let inferred_ty =
        kernel_infer_raw(cx.env, &view.context, &inferred).map_err(CurrentKernelQueryError::Kernel)?;
    view.embedding
        .translate_to_original(
            &inferred_ty,
            view.embedding.original_len,
            view.embedding.expanded_len,
        )
        .map_err(CurrentKernelQueryError::View)
}

fn kernel_infer_current(
    cx: &ElabCtx<'_>,
    inferred: &Term,
) -> Result<Term, CurrentKernelQueryError> {
    kernel_infer_in_context_current(cx, &cx.ctx, inferred)
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
fn wrap_premise_lams_finalized(
    body: Term,
    premises: &[Term],
    sentinel_region: usize,
) -> Term {
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

fn wrap_premise_pis_finalized(
    body: Term,
    premises: &[Term],
    sentinel_region: usize,
) -> Term {
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
    cx: &ElabCtx<'_>,
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
    let env: &GlobalEnv = &*cx.env;
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
            cx,
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
    let mut impossible_idx = None;
    for (index, premise) in premise_domains.iter().enumerate() {
        let mut premise_ctx = cx.ctx.clone();
        premise_ctx.push(premise.clone());
        match kernel_check_in_context_current(cx, &premise_ctx, &Term::var(0), &bottom) {
            Ok(()) => {
                impossible_idx = Some(index);
                break;
            }
            Err(CurrentKernelQueryError::View(error)) => return Err(error),
            Err(CurrentKernelQueryError::Kernel(_)) => {}
        }
    }
    let impossible_idx = impossible_idx.ok_or_else(|| ElabError::ExhaustivenessError {
        missing,
        span: span.clone(),
    })?;
    let proof = index_refinement_sentinel(sentinel_region, impossible_idx);
    let body = Term::Absurd(Box::new(expected_here.clone()), Box::new(proof));
    Ok(wrap_premise_lams_finalized(body, premise_domains, sentinel_region))
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

/// Build the one branch expression selected by all source arms for a single
/// top-level constructor. Guarded predecessors become nested conditionals;
/// the first unguarded arm is the covering fallback and alone subsumes later
/// arms. If no unguarded arm exists, return `None`: the caller must treat the
/// constructor as omitted, proving it index-impossible or reporting it missing.
fn pattern_ctor_id(cx: &ElabCtx<'_>, kind: &RPatKind) -> Option<GlobalId> {
    match kind {
        RPatKind::Ctor(name, _) => cx.globals.get(name).copied(),
        RPatKind::CheckedCtor(_, id, _) => Some(*id),
        _ => None,
    }
}

#[inline(never)]
fn guarded_constructor_arm(
    cx: &ElabCtx,
    arms: &[RMatchArm],
    ctor_id: GlobalId,
    arm_used: &mut [bool],
    subsumed_by: &mut [Option<usize>],
) -> Option<(RMatchArm, Option<usize>)> {
    let candidates = arms
        .iter()
        .enumerate()
        .filter(|(_, arm)| {
            pattern_ctor_id(cx, &arm.pat.kind) == Some(ctor_id)
        })
        .collect::<Vec<_>>();
    let fallback = candidates.iter().position(|(_, arm)| arm.guard.is_none())?;
    for (idx, _) in &candidates[..=fallback] {
        arm_used[*idx] = true;
    }
    let fallback_idx = candidates[fallback].0;
    for (idx, _) in &candidates[fallback + 1..] {
        subsumed_by[*idx].get_or_insert(fallback_idx);
    }

    let mut body = candidates[fallback].1.body.clone();
    for (_, arm) in candidates[..fallback].iter().rev() {
        let guard = arm
            .guard
            .clone()
            .expect("every constructor predecessor before the fallback is guarded");
        body = RExpr::RIf {
            condition: Box::new(guard),
            then_branch: Box::new(arm.body.clone()),
            else_branch: Box::new(body),
            span: arm.span.clone(),
        };
    }
    let first = candidates[0];
    let unchanged_arm_index = (fallback == 0).then_some(first.0);
    Some((
        RMatchArm {
            pat: first.1.pat.clone(),
            guard: None,
            body,
            span: first.1.span.clone(),
        },
        unchanged_arm_index,
    ))
}

#[cold]
#[inline(never)]
fn unassociated_infix_error(span: &Span) -> Result<(Term, Term), ElabError> {
    Err(ElabError::Internal(format!(
        "unassociated infix spine reached type-directed elaboration at {}-{}",
        span.start, span.end
    )))
}

// A spelling-routed global is a leaf, but its constructor, inductive, and
// dictionary cases carry temporaries. Keep those out of `infer`'s recursive
// dispatch frame: every nested application pays that frame before reaching
// this leaf, even when no spelling lookup is performed at that depth.
#[inline(never)]
fn infer_spelling_global(
    cx: &mut ElabCtx,
    name: &str,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    if let Some((term, ty, install_depth)) = cx.local_dicts.get(name) {
        let growth = cx.ctx.len().checked_sub(*install_depth).ok_or_else(|| {
            ElabError::Internal(format!("dictionary '{name}' used outside its declaration context"))
        })? as i64;
        return Ok((weaken(term, growth), weaken(ty, growth)));
    }
    let id = cx
        .globals
        .get(name)
        .copied()
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: name.to_string(),
            span: span.clone(),
        })?;
    if let Some((ind, k)) = cx.env.constructor(id) {
        return Ok((
            Term::Constructor {
                id,
                level_args: vec![],
            },
            ind.constructors[k].type_.clone(),
        ));
    }
    if let Some(ind) = cx.env.inductive(id) {
        return Ok((
            Term::IndFormer {
                id,
                level_args: vec![],
            },
            ind.former_type.clone(),
        ));
    }
    let (_, decl_ty) = cx
        .env
        .const_type(id)
        .ok_or_else(|| ElabError::Internal(format!("no type for global '{name}'")))?;
    Ok((Term::const_(id, vec![]), decl_ty.clone()))
}

/// `RVar` arm of `infer`, split out per `check`'s FRAME BUDGET note:
/// `infer` recurses on every expression, so the alias temporary must not
/// sit in its frame.
#[inline(never)]
fn infer_named_local_variable(
    cx: &mut ElabCtx,
    i: &usize,
    name: &str,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    if let Some(alias) = infer_virtual_pattern_alias(cx, *i, name, span)? {
        return Ok(alias);
    }
    // An installed index refinement (constructor injectivity
    // / sibling convoy) replaces the bare `Var` with its `Cast`-
    // wrapped alias for the duration of one branch's body — see
    // `ElabCtx::var_refinements`.
    let (pos, actual_index) = cx
        .surface_var(*i)
        .ok_or_else(|| ElabError::Internal(format!("Var({}) out of range", i)))?;
    if let Some((raw_term, raw_ty, install_depth)) = cx.var_refinements.get(&pos) {
        let growth = (cx.ctx.len() - install_depth) as i64;
        let core = weaken(raw_term, growth);
        return Ok(scoped_premise_binding(cx, &core)?
            .unwrap_or_else(|| (core, weaken(raw_ty, growth))));
    }
    let ty_stored = cx
        .ctx
        .lookup(actual_index)
        .ok_or_else(|| ElabError::Internal(format!("Var({}) out of range", i)))?;
    let ty = weaken(ty_stored, (actual_index as i64) + 1);
    Ok((Term::var(actual_index), ty))
}

/// An application in a written type owes the same refinement introduction as
/// a value call. The already-checked global identity selects literal parameter
/// or constructor-field facts; named domains use the value route's alias-root
/// reuse rule. A missing obligation channel refuses rather than erasing.
#[inline(never)]
fn introduce_type_position_argument(
    cx: &mut ElabCtx<'_>, function: &Term, argument: &Term, span: &Span,
) -> Result<(), ElabError> {
    if !cx.type_introductions { return Ok(()); }
    emit_call_refinements(cx, function, argument, span)?;
    let (head, previous_args) = peel_app(function);
    if !matches!(head, Term::Const { .. } | Term::Constructor { .. }) {
        return Ok(());
    }
    // The head's checked global type does not depend on local binder
    // alignment. Infer a local argument only when the Pi domain actually
    // carries a named refinement; literal parameters were handled above.
    let unchecked = |_| ElabError::TypeMismatch {
        span: span.clone(),
        reason: "cannot check an argument's refinement in type position".into(),
    };
    let mut ty = kernel_infer_current(cx, &head).map_err(unchecked)?;
    for arg in &previous_args {
        let Term::Pi(_, codomain) = whnf(cx.env, &cx.ctx, &ty) else {
            return Ok(());
        };
        ty = subst0(&codomain, arg);
    }
    let Term::Pi(domain, _) = whnf(cx.env, &cx.ctx, &ty) else {
        return Ok(());
    };
    let named = matches!(cx.metas.zonk_term(&domain),
        Term::Const { id, .. } if cx.refinement_facts.refinement_predicate(id).is_some());
    if !named {
        return Ok(());
    }
    let argument_ty = kernel_infer_current(cx, argument).map_err(unchecked)?;
    emit_refinement_introduction(cx, &domain, &argument_ty, argument.clone(), span, None).map(|_| ())
}

/// A direct call can introduce a checked argument at a source-refined
/// parameter or constructor field; the recursive `infer` frame does not carry
/// the predicate lookup's transient application spine.
#[inline(never)]
fn emit_call_refinements(
    cx: &mut ElabCtx<'_>,
    function: &Term,
    argument: &Term,
    span: &Span,
) -> Result<(), ElabError> {
    let (head, previous_args) = peel_app(function);
    let template = match head {
        Term::Const { id, .. } => cx
            .refinement_facts
            .refined_params.get(&id)
            .and_then(|params| params.get(previous_args.len())),
        // A constructor spine starts with its family parameters; the field
        // predicates are indexed by constructor argument position.
        Term::Constructor { id, .. } => {
            let params = cx
                .env
                .constructor(id)
                .map_or(0, |(family, _)| family.params.len());
            previous_args.len().checked_sub(params).and_then(|field| {
                cx.refinement_facts
                    .constructor_field_predicates.get(&id)
                    .and_then(|fields| fields.get(field))
            })
        }
        _ => None,
    }
    .and_then(Option::as_ref)
    .cloned();
    if let Some(template) = template {
        let instantiated = subst_outer(&template, previous_args.len(), &previous_args, 0);
        emit_refinement_predicate(cx, instantiated, argument.clone(), span)?;
    }
    Ok(())
}

/// `(e : T)`. An ascription at a source refinement introduces its predicate
/// obligation. Outlined so the recursive `infer` frame does not hold the
/// kernel query.
#[inline(never)]
fn infer_ascription(
    cx: &mut ElabCtx<'_>,
    e: &RExpr,
    ty: &RType,
    _span: &Span,
) -> Result<(Term, Term), ElabError> {
    let ty_core = elab_type_in_slot(cx, ty, RefinementSlot::Outermost)?;
    let predicate = literal_result_predicate(cx, ty, &ty_core)?;
    let e_core = if let Some(predicate) = predicate {
        check_result_position(cx, e, &ty_core, e.span(), &[predicate])?
    } else {
        check(cx, e, &ty_core, e.span())?
    };
    Ok((e_core, ty_core))
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
            let classifier = kernel_infer_current(cx, &result_type).map_err(|error| match error {
                CurrentKernelQueryError::View(error) => error,
                CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                    error,
                    span: span.clone(),
                },
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
        RExpr::RPatternAlias(slot, name, _span) => infer_active_pattern_alias(cx, *slot, name),
        RExpr::RVar(i, name, span) => infer_named_local_variable(cx, i, name, span),

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

        RExpr::RCheckedGlobal { name, id, span } => {
            let id = *id;
            if let Some((ind, k)) = cx.env.constructor(id) {
                let ty = ind.constructors[k].type_.clone();
                return Ok((Term::Constructor { id, level_args: vec![] }, ty));
            }
            if let Some(ind) = cx.env.inductive(id) {
                return Ok((Term::IndFormer { id, level_args: vec![] }, ind.former_type.clone()));
            }
            let (_, ty) = cx.env.const_type(id).ok_or_else(|| {
                ElabError::Internal(format!("no checked type for imported global '{name}' {id:?}"))
            })?;
            let (core, ty) = (Term::const_(id, vec![]), ty.clone());
            apply_zero_arity_preconditions(cx, id, core, ty, span)
        }
        RExpr::RCon(name, span) => {
            let (core, ty) = infer_spelling_global(cx, name, span)?;
            let Term::Const { id, .. } = core else {
                return Ok((core, ty));
            };
            apply_zero_arity_preconditions(cx, id, core, ty, span)
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
                return append_saturated_preconditions(cx, expr, call, span);
            }
            if let Some(call) = infer_preconditioned_application(cx, expr, span)? {
                return Ok(call);
            }
            let (f_core, f_ty) = infer(cx, f)?;
            let f_ty_wh = whnf(cx.env, &cx.ctx, &f_ty);
            match f_ty_wh {
                Term::Pi(dom, cod) => {
                    let a_core = check(cx, a, &dom, span)?;
                    emit_call_refinements(cx, &f_core, &a_core, span)?;
                    let result_ty = subst0(&cod, &a_core);
                    Ok((Term::app(f_core, a_core), result_ty))
                }
                _ => Err(ElabError::NotAFunction { span: span.clone() }),
            }
        }

        RExpr::RAsc(e, ty, span) => infer_ascription(cx, e, ty, span),

        RExpr::RLam(_, _, span) => Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "cannot infer type of lambda without annotation".into(),
        }),

        RExpr::RLet(_x, ty_opt, rhs, body, span) => {
            let (rhs_core, rhs_ty) = prepare_let_rhs(cx, ty_opt, rhs, span)?;
            let equation = let_equation(cx, &rhs_ty, &rhs_core);
            cx.push_match_binder(rhs_ty.clone(), MatchBinderOrigin::UserLocal);
            let path_base = cx.path_conditions.len();
            if let Some(equation) = equation {
                cx.path_conditions.push((equation, cx.ctx.len()));
            }
            let body_result = infer(cx, body);
            cx.path_conditions.truncate(path_base);
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
        RExpr::RStandardOp {
            op,
            lhs,
            rhs,
            span,
        } => elab_standard_operator(cx, *op, lhs, rhs, span),

        RExpr::RInfixSpine { span, .. } => unassociated_infix_error(span),

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
        } => infer_match(cx, scrut, arms, span, None),

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
    let ty = kernel_infer_in_zonked_current(cx, &zonked_ctx, &zonked_eq).map_err(|error| {
        match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        }
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
    cx.push_match_binder(a_core.clone(), MatchBinderOrigin::UserLocal);
    let b_result = infer(cx, b);
    cx.ctx.pop();
    let (b_core, _b_ty) = b_result?;
    let b_core = cx.metas.zonk_term(&b_core);
    let pi = Term::pi(a_core, b_core);

    let zonked_ctx = Context {
        types: cx.ctx.types.iter().map(|t| cx.metas.zonk_term(t)).collect(),
    };
    let zonked_pi = cx.metas.zonk_term(&pi);
    let sort = kernel_infer_in_zonked_current(cx, &zonked_ctx, &zonked_pi).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        },
    )?;
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
    let sort = kernel_infer_in_zonked_current(cx, &zonked_ctx, &zonked_pi).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        },
    )?;
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
    let sort = kernel_infer_in_zonked_current(cx, &zonked_ctx, &zonked_trunc).map_err(
        |error| match error {
            CurrentKernelQueryError::View(error) => error,
            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                error,
                span: span.clone(),
            },
        },
    )?;
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
    cx.push_match_binder(a_ty.clone(), MatchBinderOrigin::UserLocal);
    cx.push_match_binder(eq_dom_ty.clone(), MatchBinderOrigin::UserLocal);
    let body_result = infer(cx, motive_body_expr);
    cx.ctx.pop();
    cx.ctx.pop();
    let (body_core, body_ty) = body_result?;

    let motive_lam = Term::lam(a_ty.clone(), Term::lam(eq_dom_ty.clone(), body_core));
    let motive_ty = Term::pi(a_ty.clone(), Term::pi(eq_dom_ty, body_ty));
    let motive_core = Term::Ascript(Box::new(motive_lam.clone()), Box::new(motive_ty));

    let base_expected_ty = Term::app(
        Term::app(motive_lam.clone(), a.clone()),
        Term::Refl(Box::new(a.clone())),
    );
    let base_core = check(cx, base_expr, &base_expected_ty, span)?;

    let result_ty = Term::app(Term::app(motive_lam, b.clone()), eq_core.clone());
    // Record the exact Eq formation the surface rule used. The kernel's J
    // reader must not re-derive endpoints from an Eq head's later reduct.
    let eq_recorded = Term::Ascript(
        Box::new(eq_core),
        Box::new(Term::Eq(Box::new(a_ty), Box::new(a), Box::new(b))),
    );
    let term_j = Term::J(
        Box::new(motive_core),
        Box::new(base_core),
        Box::new(eq_recorded),
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
    let lit_id = ken_kernel::check::declare_checked_string_literal(cx.env, s).map_err(|error| {
        ElabError::KernelRejected {
            error,
            span: span.clone(),
        }
    })?;
    let checked = cx.env.checked_literal(lit_id)
        .expect("kernel just registered checked String payload");
    cx.num_values
        .insert(lit_id, NumericLitVal::Str(crate::NfcString::new(checked.as_str())));
    Ok((Term::const_(lit_id, vec![]), str_ty))
}

/// Elaborate a character literal (`31 §3`) -- one decoded Unicode scalar,
/// already validated by the lexer's cardinality check. `Char` is `{c : Int |
/// isScalar c}` (`decimal_char.rs`). The kernel validates the scalar and
/// emits an ordinary, type-checked IntLit at the Int-compatible Char carrier;
/// the immutable core value itself drives conversion, interpreter and native.
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
    let literal = ken_kernel::check::checked_char_literal(cx.env, c as u32)
        .map_err(|error| ElabError::KernelRejected {
            error,
            span: span.clone(),
        })?;
    Ok((literal, char_ty))
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
                    let closed = close_goal(&cx.ctx, &[], phi);
                    declare_obligation_hole(cx, closed, span, ObligationKind::PartialPrim)?;
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

        BinOp::Div | BinOp::Mod => {
            let symbol = if matches!(op, BinOp::Div) { "/" } else { "%" };
            let entry: &DivEntry =
                cx.numeric_env
                    .classify_div(&lhs_ty_wh)
                    .ok_or_else(|| ElabError::TypeMismatch {
                        span: span.clone(),
                        reason: format!("'{symbol}' not supported on this type"),
                    })?;
            let result_ty = Term::const_(entry.result_id, vec![]);
            let rhs_core = check(cx, rhs, &result_ty, span)?;
            let op_id = if matches!(op, BinOp::Div) { entry.div_id } else { entry.mod_id };
            let applied = Term::app(
                Term::app(Term::const_(op_id, vec![]), lhs_core),
                rhs_core.clone(),
            );

            // Recognition is by kernel conversion of a DIRECT assumption,
            // never by spelling or by reasoning from a stronger proposition.
            let goal = Term::app(Term::const_(entry.nonzero_id, vec![]), rhs_core);
            let known = premise_proof_in_scope(cx, &goal).is_some();
            if !known {
                let closed = close_goal(&cx.ctx, &[], goal);
                declare_obligation_hole(cx, closed, span, ObligationKind::PartialPrim)?;
            }
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

/// Close a goal over its context, inserting every available proposition at
/// its recorded binder depth. The kernel's actual body context is unchanged.
/// When an assumption is inserted, shift the still-free outer variables in
/// the inner telescope by one so they continue to refer to their binders.
fn close_goal(ctx: &Context, assumptions: &[Assumption], goal: Term) -> Term {
    let mut result = goal;
    for depth in (0..=ctx.len()).rev() {
        // Reverse at equal depth so discovery order remains the outer-to-inner
        // order of assumptions in the closed Pi telescope.
        for assumption in assumptions.iter().rev().filter(|a| a.depth == depth) {
            result = Term::pi(assumption.prop.clone(), weaken(&result, 1));
        }
        if depth > 0 {
            result = Term::pi(ctx.types[depth - 1].clone(), result);
        }
    }
    result
}

/// Check a requires or ensures proposition in the current context before it
/// becomes an elaborator-only hypothesis or an obligation.
fn elab_prop_at_omega(cx: &mut ElabCtx<'_>, expr: &RExpr, span: &Span) -> Result<Term, ElabError> {
    let (raw, inferred) = infer(cx, expr)?;
    let prop = cx.metas.zonk_term(&raw);
    let ty = cx.metas.zonk_term(&inferred);
    let omega = Term::omega(Level::Zero);
    if !matches!(ty, Term::Omega(_))
        && kernel_check_raw(cx.env, &cx.ctx, &prop, &omega).is_err()
    {
        return Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "spec proposition must have type Ω, found non-proposition".into(),
        });
    }
    Ok(prop)
}

#[cfg(test)]
mod omega_clause_gate_tests {
    use super::*;

    #[test]
    fn nonzero_omega_level_uses_the_omega_shaped_arm_for_requires_and_ensures() {
        for clause in ["requires", "ensures"] {
            let mut env = crate::ElabEnv::new().expect("numeric prelude");
            // The gate sees a well-formed Ω₂ proposition. Full declaration
            // admission has other level restrictions, so test the shared
            // requires/ensures predicate itself rather than hiding those gates.
            let source = format!("fn f (n : Int) : Int {clause} Eq (Type 1) Type Type = n");
            let parsed = crate::parser::parse_decls(&source).expect("clause syntax");
            let rdecl = crate::resolve::resolve_decl(&parsed[0]).expect("resolved clause");
            let expr = if clause == "requires" {
                &rdecl.requires[0]
            } else {
                &rdecl.ensures[0]
            };

            let refinement_facts = super::RefinementFacts::default();
            let mut cx = ElabCtx::new(
                &mut env.env,
                &env.globals,
                &mut env.num_values,
                &env.numeric_env,
                &refinement_facts,
                "omega-clause-gate",
            );
            let prop = elab_prop_at_omega(&mut cx, expr, expr.span())
                .unwrap_or_else(|error| panic!("higher-Ω {clause} gate: {error:?}"));
            let inferred = kernel_infer_raw(cx.env, &cx.ctx, &prop).expect("formed proposition");
            assert!(
                matches!(inferred, Term::Omega(ref level) if level != &Level::Zero),
                "{clause}: control must be Ω at a nonzero level, not Ω₀"
            );
            assert!(
                kernel_check_raw(cx.env, &cx.ctx, &prop, &Term::omega(Level::Zero)).is_err(),
                "{clause}: the kernel-at-Ω₀ fallback cannot make this row pass"
            );

            let bad_source = format!("fn bad (n : Int) : Int {clause} True = n");
            let bad_parsed = crate::parser::parse_decls(&bad_source).expect("Bool syntax");
            let bad = crate::resolve::resolve_decl(&bad_parsed[0]).expect("Bool clause");
            let bad_expr = if clause == "requires" {
                &bad.requires[0]
            } else {
                &bad.ensures[0]
            };
            assert!(
                matches!(
                    elab_prop_at_omega(&mut cx, bad_expr, bad_expr.span()),
                    Err(ElabError::TypeMismatch { .. })
                ),
                "{clause}: Bool still fails both proposition-gate arms"
            );
        }
    }
}

fn build_contract_type(
    carrier_ty: &Term,
    param_count: usize,
    requires: &[Term],
) -> Result<(Vec<Term>, Term, Term), ElabError> {
    let (param_types, carrier_result) = split_params(carrier_ty, param_count).ok_or_else(|| {
        ElabError::Internal("declaration parameter telescope is shorter than its source arity".into())
    })?;
    let mut full_ty = weaken(&carrier_result, requires.len() as i64);
    for requirement in requires.iter().rev() {
        full_ty = Term::pi(requirement.clone(), full_ty);
    }
    for parameter in param_types.iter().rev() {
        full_ty = Term::pi(parameter.clone(), full_ty);
    }
    Ok((param_types, carrier_result, full_ty))
}

fn check_contract_body(
    cx: &mut ElabCtx<'_>,
    body: &RExpr,
    carrier_ty: &Term,
    param_count: usize,
    requires: &[Term],
    span: &Span,
    result_predicates: &[ResultPredicate],
) -> Result<(Term, Term), ElabError> {
    let (param_types, carrier_result) = split_params(carrier_ty, param_count).ok_or_else(|| {
        ElabError::Internal("declaration parameter telescope is shorter than its source arity".into())
    })?;
    let start_depth = cx.ctx.len();
    let hidden_base = cx.hidden_positions.len();
    let assumption_base = cx.assumptions.len();
    let result = (|| {
        let mut current = body;
        for domain in &param_types {
            let RExpr::RLam(_, inner, _) = current else {
                return Err(ElabError::TypeMismatch {
                    span: current.span().clone(),
                    reason: "a declared parameter is missing its body lambda".into(),
                });
            };
            let position = cx.ctx.len();
            let is_proposition = kernel_infer_current(cx, domain)
                .ok()
                .is_some_and(|sort| matches!(whnf(cx.env, &cx.ctx, &sort), Term::Omega(_)));
            if is_proposition {
                cx.assumptions.push(Assumption {
                    prop: domain.clone(),
                    depth: position,
                });
            }
            cx.ctx.push(domain.clone());
            current = inner;
        }
        for requirement in requires {
            let position = cx.ctx.len();
            cx.ctx.push(requirement.clone());
            cx.hidden_positions.push(position);
        }
        let body_ty = weaken(&carrier_result, requires.len() as i64);
        let body_inner = check_result_position(cx, current, &body_ty, span, result_predicates)?;
        let mut full_body = body_inner.clone();
        for requirement in requires.iter().rev() {
            full_body = Term::lam(requirement.clone(), full_body);
        }
        for parameter in param_types.iter().rev() {
            full_body = Term::lam(parameter.clone(), full_body);
        }
        Ok((full_body, body_inner))
    })();
    while cx.ctx.len() > start_depth {
        cx.ctx.pop();
    }
    cx.hidden_positions.truncate(hidden_base);
    cx.assumptions.truncate(assumption_base);
    result
}

/// A spec'd declaration's `requires` are checked before its body and become
/// proof binders at their telescope positions. They are not hole sites.
fn install_requires_assumptions(
    cx: &mut ElabCtx<'_>,
    carrier_ty: &Term,
    param_count: usize,
    requires: &[RExpr],
) -> Result<Vec<Term>, ElabError> {
    let start_depth = cx.ctx.len();
    let hidden_base = cx.hidden_positions.len();
    let assumption_base = cx.assumptions.len();
    let (param_types, _) = split_params(carrier_ty, param_count).ok_or_else(|| {
        ElabError::Internal("requires parameter telescope is shorter than its source arity".into())
    })?;
    for ty in param_types {
        cx.ctx.push(ty);
    }
    let result = (|| {
        let mut cores = Vec::with_capacity(requires.len());
        for req in requires {
            let prop = elab_prop_at_omega(cx, req, req.span())?;
            let depth = cx.ctx.len();
            cx.assumptions.push(Assumption {
                prop: prop.clone(),
                depth,
            });
            cx.ctx.push(prop.clone());
            cx.hidden_positions.push(depth);
            cores.push(prop);
        }
        Ok(cores)
    })();
    while cx.ctx.len() > start_depth {
        cx.ctx.pop();
    }
    cx.hidden_positions.truncate(hidden_base);
    if result.is_err() {
        cx.assumptions.truncate(assumption_base);
    }
    result
}

/// Renumber only at aggregation: no elaboration-site consumer reads an id.
fn absorb_obligations(dst: &mut Vec<Obligation>, src: Vec<Obligation>) {
    for mut obligation in src {
        obligation.id = dst.len() as u32;
        dst.push(obligation);
    }
}

/// Close a logical refinement goal over ordinary binders followed by branch
/// equations and refined-parameter assumptions. The latter are assumptions
/// only in the obligation, never unchecked evidence in an emitted program.
/// Recursive-call contracts and path equations extend the obligation only;
/// neither is evidence introduced into the emitted definition.
#[inline(never)]
fn close_refinement_goal_with(
    cx: &ElabCtx<'_>, goal: Term, hypotheses: &[Term], proof: Option<Term>,
) -> (Term, Option<Term>) {
    let count = cx.path_conditions.len();
    let mut closed = weaken(&goal, (count + hypotheses.len()) as i64);
    let mut certificate = proof;
    for (index, hypothesis) in hypotheses.iter().enumerate().rev() {
        let domain = weaken(hypothesis, (count + index) as i64);
        closed = Term::pi(domain.clone(), closed);
        certificate = certificate.map(|term| Term::lam(domain, term));
    }
    for (index, (condition, install_depth)) in cx.path_conditions.iter().enumerate().rev() {
        let growth = cx.ctx.len().checked_sub(*install_depth)
            .expect("path condition consumed before its field binders were entered");
        let domain = weaken(condition, (growth + index) as i64);
        closed = Term::pi(domain.clone(), closed);
        certificate = certificate.map(|term| Term::lam(domain, term));
    }
    let mut result = closed;
    for stored in cx.ctx.types.iter().rev() {
        result = Term::pi(stored.clone(), result);
        certificate = certificate.map(|term| Term::lam(stored.clone(), term));
    }
    (result, certificate)
}

#[inline(never)]
fn literal_result_predicate(
    cx: &mut ElabCtx<'_>, literal: &RType, expected: &Term,
) -> Result<Option<ResultPredicate>, ElabError> {
    let RType::RRefine(_, _, phi, _) = literal else { return Ok(None) };
    let carrier = cx.metas.zonk_term(expected);
    let install_depth = cx.ctx.len();
    cx.ctx.push(carrier.clone());
    let checked = elab_prop_at_omega(cx, phi, phi.span());
    cx.ctx.pop();
    Ok(Some(ResultPredicate {
        predicate: Term::lam(carrier, checked?), install_depth,
        kind: ObligationKind::RefinementIntroduction,
        recursive_self: None,
    }))
}

#[inline(never)]
fn emit_refinement_introduction(
    cx: &mut ElabCtx<'_>,
    expected: &Term,
    inferred_ty: &Term,
    core: Term,
    span: &Span,
    literal: Option<&RType>,
) -> Result<Term, ElabError> {
    let named = match cx.metas.zonk_term(expected) {
        Term::Const { id, .. } => {
            let facts = cx.refinement_facts;
            let root = facts.refinement_root(id);
            let reused = root.is_some()
                && matches!(cx.metas.zonk_term(inferred_ty),
                    Term::Const { id: source, .. } if facts.refinement_root(source) == root);
            if reused {
                None
            } else {
                facts.refinement_predicate(id).cloned()
            }
        }
        _ => None,
    };
    let predicate = if let Some(literal) = literal {
        literal_result_predicate(cx, literal, expected)?.map(|p| p.predicate)
    } else {
        named
    };
    let Some(predicate) = predicate else { return Ok(core) };
    emit_refinement_predicate(cx, predicate, core.clone(), span)?;
    Ok(core)
}

fn apply_refinement_predicate(predicate: Term, value: Term) -> Term {
    match predicate {
        Term::Lam(_, body) => subst0(&body, &value),
        other => Term::app(other, value),
    }
}

#[inline(never)]
fn emit_refinement_predicate(
    cx: &mut ElabCtx<'_>, predicate: Term, core: Term, span: &Span,
) -> Result<(), ElabError> {
    let goal = apply_refinement_predicate(predicate, core);
    emit_refinement_predicate_with(cx, goal, &[], span, ObligationKind::RefinementIntroduction)
}

#[inline(never)]
fn emit_refinement_predicate_with(
    cx: &mut ElabCtx<'_>, goal: Term, hypotheses: &[Term],
    span: &Span, kind: ObligationKind,
) -> Result<(), ElabError> {
    let (closed, _) = close_refinement_goal_with(cx, goal.clone(), hypotheses, None);
    let proof = (0..hypotheses.len() + cx.path_conditions.len())
        .map(Term::var)
        .chain(std::iter::once(Term::const_(cx.env.tt_id(), vec![])))
        .find_map(|candidate| {
            let (_, certificate) =
                close_refinement_goal_with(cx, goal.clone(), hypotheses, Some(candidate));
            let certificate = certificate?;
            kernel_check_raw(cx.env, &Context::new(), &certificate, &closed)
                .ok().map(|_| certificate)
        });
    if proof.is_some() && cx.premise_holes == PremiseHoles::Refused {
        // Discharged where it is generated; a context with no obligation
        // channel records nothing, so the environment is left unchanged.
        return Ok(());
    }
    let hole_id = declare_obligation_hole(cx, closed, span, kind)?;
    if let Some(proof) = proof {
        ken_kernel::check::admit_bodies(cx.env, &[(hole_id, proof)])
            .map_err(|error| ElabError::KernelRejected { error, span: span.clone() })?;
    }
    Ok(())
}

/// Do not walk into a leaf's own binders: their indices do not belong to the
/// obligation context. A direct, fully-applied staged self is the only IH.
#[inline(never)]
fn recursive_call_hypotheses(rs: &RecursiveSelf, leaf: &Term) -> Vec<Term> {
    fn collect(rs: &RecursiveSelf, term: &Term, out: &mut Vec<Term>) {
        let mut args = Vec::new();
        let mut head = term;
        while let Term::App(f, arg) = head {
            args.push(arg.as_ref().clone());
            head = f;
        }
        if matches!(head, Term::Const { id, .. } if *id == rs.id)
            && args.len() == rs.params + rs.requires
        {
            args.reverse();
            let with_args = subst_outer(&rs.psi, args.len(), &args, 1);
            let hypothesis = subst0(&with_args, term);
            if !out.contains(&hypothesis) {
                out.push(hypothesis);
            }
        }
        match term {
            Term::Pi(dom, _) | Term::Lam(dom, _) | Term::Sigma(dom, _) =>
                collect(rs, dom, out),
            Term::Let { ty, val, .. } => {
                collect(rs, ty, out);
                collect(rs, val, out);
            }
            other => {
                for child in other.children() {
                    collect(rs, child, out);
                }
            }
        }
    }
    let mut hypotheses = Vec::new();
    collect(rs, leaf, &mut hypotheses);
    hypotheses
}

#[inline(never)]
fn emit_result_predicate(
    cx: &mut ElabCtx<'_>, predicate: &ResultPredicate, leaf: &Term,
    result_ty: &Term, span: &Span,
) -> Result<(), ElabError> {
    let growth = cx.ctx.len().checked_sub(predicate.install_depth).ok_or_else(|| {
        ElabError::Internal("result predicate escaped its installation context".into())
    })?;
    let value = if matches!(whnf(cx.env, &cx.ctx, result_ty), Term::Pi(..)) {
        Term::Ascript(Box::new(leaf.clone()), Box::new(result_ty.clone()))
    } else {
        leaf.clone()
    };
    let goal = apply_refinement_predicate(weaken(&predicate.predicate, growth as i64), value);
    let hypotheses = predicate.recursive_self.as_ref()
        .map(|self_info| recursive_call_hypotheses(self_info, leaf))
        .unwrap_or_default();
    emit_refinement_predicate_with(cx, goal, &hypotheses, span, predicate.kind.clone())
}

// ----- declaration elaboration -----

/// V0-compatible elaboration (no spec clauses).
pub(crate) fn elaborate_rdecl(
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
    // A sentinel class environment marks a path that elaborates no user
    // expression body, so it certifies no standard operators either.
    //
    // EMPTY IS NOT A FAIL-CLOSED VALUE, and this comment used to call it one.
    // An empty map certifies nothing, so an occurrence reaching here is left
    // as an ordinary under-applied application and caught downstream by the
    // kernel check -- NOT refused naming the role. See the residual on
    // `reduce_resolved_operator`'s non-certified arm.
    let no_standard_operators = HashMap::new();
    // A LOCAL SINK IS CORRECT HERE, AND IT IS THE ONLY PLACE THAT IS TRUE.
    // Provenance used to live inside `ClassEnv`, so on this path it went into
    // the throwaway `sentinel` above and was dropped with it. A local vector
    // preserves that exactly. Contrast `elaborate_rdecl_v1`, where provenance
    // accumulated into the CALLER's registry and a local would have silently
    // discarded it -- same refactor, opposite right answer, decided by where
    // the old field's owner outlived the call.
    let mut discarded_provenance = Vec::new();
    let result = elaborate_rdecl_v1(
        env,
        globals,
        num_values,
        numeric_env,
        &mut sentinel,
        &mut discarded_provenance,
        &no_standard_operators,
        rdecl,
    )?;
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
        RType::RCon(name, _) | RType::RCheckedGlobal { name, .. } => name.clone(),
        RType::RApp(f, _, _) => rtype_head_name(f),
        RType::RVarTy(_, name, _) => name.clone(),
        // A truncation head, consistent with head_type_name.
        RType::RTrunc(_, _) => "‖‖".to_string(),
        // A PROJECTION HAS NO STATIC HEAD, and the empty string is the right
        // answer rather than an oversight: `d.Query`'s identity is not known
        // until `d`'s dictionary is, which is AFTER instance search rather than
        // before it. Naming a head here would key `instance_search` on
        // something that is not a type constructor. Stated explicitly because
        // the `_` below would give the same value silently.
        RType::RProj(_, _, _) => String::new(),
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
        RType::RTrunc(inner, span) => RType::RTrunc(
            Box::new(instantiate_instance_rtype(inner, args, param_count)),
            span.clone(),
        ),
        // A projection is cloned WITHOUT substituting into its base, exactly as
        // `RRefine` above clones its predicate. This function substitutes types
        // for type parameters and has no expression-side counterpart, so an
        // embedded `RExpr` is carried through untouched. Written out rather
        // than left to the `_` below, because in a substitution function "did
        // not recurse" reads as a bug unless it says why.
        RType::RProj(_, _, _) => ty.clone(),
        _ => ty.clone(),
    }
}

fn rtypes_match(left: &RType, right: &RType, globals: &HashMap<String, GlobalId>) -> bool {
    match (left, right) {
        (RType::RCon(left, _), RType::RCon(right, _)) =>
            globals.get(left).zip(globals.get(right)).is_some_and(|(a, b)| a == b),
        (RType::RCheckedGlobal { id: left, .. }, RType::RCheckedGlobal { id: right, .. }) => left == right,
        (RType::RCon(left, _), RType::RCheckedGlobal { id, .. })
        | (RType::RCheckedGlobal { id, .. }, RType::RCon(left, _)) =>
            globals.get(left) == Some(id),
        (RType::RVarTy(left, left_name, _), RType::RVarTy(right, right_name, _)) =>
            left == right && left_name == right_name,
        (RType::RUniv(left, _), RType::RUniv(right, _)) => left == right,
        (RType::RApp(left_f, left_a, _), RType::RApp(right_f, right_a, _)) => {
            rtypes_match(left_f, right_f, globals) && rtypes_match(left_a, right_a, globals)
        }
        (RType::RArr(left_a, left_b, _), RType::RArr(right_a, right_b, _)) => {
            rtypes_match(left_a, right_a, globals) && rtypes_match(left_b, right_b, globals)
        }
        (
            RType::REffectArr(left_a, left_row, left_b, _),
            RType::REffectArr(right_a, right_row, right_b, _),
        ) => {
            left_row == right_row && rtypes_match(left_a, right_a, globals)
                && rtypes_match(left_b, right_b, globals)
        }
        _ => false,
    }
}

fn match_instance_head(
    pattern: &RType,
    requested: &RType,
    globals: &HashMap<String, GlobalId>,
    param_count: usize,
    args: &mut [Option<RType>],
) -> bool {
    match pattern {
        RType::RVarTy(index, _, _) if *index < param_count => {
            let slot = param_count - 1 - index;
            match &args[slot] {
                Some(previous) => rtypes_match(previous, requested, globals),
                None => {
                    args[slot] = Some(requested.clone());
                    true
                }
            }
        }
        RType::RCon(name, _) => match requested {
            RType::RCon(other, _) => globals.get(name).zip(globals.get(other)).is_some_and(|(a, b)| a == b),
            RType::RCheckedGlobal { id, .. } => globals.get(name) == Some(id),
            _ => false,
        },
        RType::RCheckedGlobal { id, .. } => match requested {
            RType::RCheckedGlobal { id: other, .. } => id == other,
            RType::RCon(name, _) => globals.get(name) == Some(id),
            _ => false,
        },
        RType::RApp(pattern_f, pattern_a, _) => match requested {
            RType::RApp(requested_f, requested_a, _) => {
                match_instance_head(pattern_f, requested_f, globals, param_count, args)
                    && match_instance_head(pattern_a, requested_a, globals, param_count, args)
            }
            _ => false,
        },
        _ => false,
    }
}

/// The identity at the head of a core type application spine.
fn core_type_head_id(ty: &Term) -> Option<GlobalId> {
    match ty {
        Term::App(f, _) => core_type_head_id(f),
        Term::Const { id, .. } | Term::IndFormer { id, .. } => Some(*id),
        _ => None,
    }
}

/// Match a registered surface instance-head pattern against an inferred core
/// carrier. Fixed constructors are compared by resolved identity, never by the
/// surface spelling retained in the registry.
fn match_instance_head_core(
    env: &GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    ctx: &Context,
    pattern: &RType,
    requested: &Term,
    param_count: usize,
    args: &mut [Option<Term>],
) -> bool {
    match pattern {
        RType::RVarTy(index, _, _) if *index < param_count => {
            let slot = param_count - 1 - index;
            match &args[slot] {
                Some(previous) => convert_type(env, ctx, previous, requested),
                None => {
                    args[slot] = Some(requested.clone());
                    true
                }
            }
        }
        RType::RCon(name, _) | RType::RCheckedGlobal { name, .. } => {
            let Some(pattern_id) = (match pattern {
                RType::RCheckedGlobal { id, .. } => Some(*id),
                _ => globals.get(name).copied(),
            }) else {
                return false;
            };
            matches!(
                requested,
                Term::Const { id, .. } | Term::IndFormer { id, .. }
                    if *id == pattern_id
            )
        }
        RType::RApp(pattern_f, pattern_a, _) => match requested {
            Term::App(requested_f, requested_a) => {
                match_instance_head_core(
                    env,
                    globals,
                    ctx,
                    pattern_f,
                    requested_f,
                    param_count,
                    args,
                ) && match_instance_head_core(
                    env,
                    globals,
                    ctx,
                    pattern_a,
                    requested_a,
                    param_count,
                    args,
                )
            }
            _ => false,
        },
        _ => false,
    }
}

#[derive(Clone, Copy)]
enum InstanceHeadRequest<'a> {
    Surface {
        requested: &'a RType,
        expected_carrier: Option<&'a Term>,
    },
    Core {
        expected_carrier: &'a Term,
    },
}

impl<'a> InstanceHeadRequest<'a> {
    fn expected_carrier(self) -> Option<&'a Term> {
        match self {
            Self::Surface {
                expected_carrier, ..
            } => expected_carrier,
            Self::Core { expected_carrier } => Some(expected_carrier),
        }
    }
}

/// Confirm that a selected carrier-parameterized dictionary still names the
/// occurrence carrier in core. Surface names select candidates; conversion of
/// the independently derived carrier is the authority.
fn confirm_instance_dictionary_carrier(
    env: &mut GlobalEnv,
    class_env: &ClassEnv,
    ctx: &Context,
    class_name: &str,
    class_id: GlobalId,
    spelling: &str,
    candidate_type: &Term,
    expected_carrier: Option<&Term>,
    span: &Span,
) -> Result<(), ElabError> {
    let class = class_env.class_by_id(class_id).ok_or_else(|| {
        ElabError::Internal(format!(
            "instance resolution selected an unregistered class `{class_name}`"
        ))
    })?;
    let actual_class = core_type_head_id(candidate_type);
    if actual_class != Some(class_id) {
        return Err(ElabError::InstanceCarrierIdentityMismatch {
            class: class_name.to_string(),
            spelling: spelling.to_string(),
            span: span.clone(),
        });
    }
    if class.projection.head_param.is_none() {
        return Ok(());
    }
    let expected_carrier = expected_carrier.ok_or_else(|| {
        ElabError::Internal(format!(
            "carrier-parameterized class `{class_name}` reached dictionary return without an expected core carrier"
        ))
    })?;
    let confirmed = match candidate_type {
        Term::App(_, resolved_carrier) => {
            convert_type(env, ctx, resolved_carrier.as_ref(), expected_carrier)
        }
        _ => false,
    };
    if !confirmed {
        return Err(ElabError::InstanceCarrierIdentityMismatch {
            class: class_name.to_string(),
            spelling: spelling.to_string(),
            span: span.clone(),
        });
    }
    Ok(())
}

/// Resolve an instance and recursively apply every prerequisite dictionary.
/// The returned candidate is immediately kernel-inferred, so an elaborator
/// wiring error fails closed before it can become a local dictionary binding.
/// **Takes `&ClassEnv` and a separate `&mut` SINK.** The resolution decision is
/// a pure function of the registry; the one thing this writes is an
/// append-only provenance log that used to live inside `ClassEnv` and forced
/// the whole registry to be borrowed mutably. Splitting the sink out is what
/// puts dictionary resolution within reach of an expression site, where
/// `ElabCtx` holds `Option<&ClassEnv>` and must not be granted an authority
/// `33 §6.2` says it does not have.
fn resolve_instance_dictionary(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    ctx: &Context,
    class_name: &str,
    selected_class_id: Option<GlobalId>,
    requested: &RType,
    span: &Span,
    owner_label: &str,
) -> Result<(Term, Term), ElabError> {
    let class_id = checked_class_id(class_env, class_name, selected_class_id, span)
        .map_err(|_| ElabError::NoInstance {
            class: class_name.to_string(),
            ty: rtype_head_name(requested),
            span: span.clone(),
        })?;
    let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, owner_label)
        .with_preconditions(preconditions, PremiseHoles::Refused);
    for ty in &ctx.types {
        cx.ctx.push(ty.clone());
    }
    let carrier = elab_type(&mut cx, requested)?;
    let carrier = cx.metas.zonk_term(&carrier);
    let expected_carrier = class_env.class_by_id(class_id)
        .is_some_and(|class| class.projection.head_param.is_some())
        .then_some(carrier.clone());
    resolve_instance_dictionary_inner(
        env,
        globals,
        preconditions,
        num_values,
        numeric_env,
        refinement_facts,
        class_env,
        provenance,
        ctx,
        class_name,
        class_id,
        instance_head_key(requested, &carrier),
        &rtype_head_name(requested),
        InstanceHeadRequest::Surface {
            requested,
            expected_carrier: expected_carrier.as_ref(),
        },
        span,
        owner_label,
        true,
    )
}

/// Read the dictionary class from a certified standard binding's checked
/// second parameter. A later class with the same spelling cannot redirect
/// completion for an operator whose telescope was already admitted.
fn standard_operator_class_id(
    env: &GlobalEnv,
    operator: GlobalId,
    class_env: &ClassEnv,
    name: &str,
) -> Result<GlobalId, ElabError> {
    let (_, ty) = env.const_type(operator).ok_or_else(|| {
        ElabError::Internal(format!("certified operator {operator:?} has no checked telescope"))
    })?;
    let id = match ty {
        Term::Pi(_, tail) => match tail.as_ref() {
            Term::Pi(domain, _) => core_type_head_id(domain),
            _ => None,
        },
        _ => None,
    };
    id.filter(|id| class_env.class_by_id(*id).is_some()).ok_or_else(|| {
        ElabError::Internal(format!(
            "certified operator {operator:?} has no checked {name} dictionary parameter"
        ))
    })
}

/// Resolve a dictionary when the caller holds the carrier's ID and no surface
/// type. The exact `(class_id, head_id)` selects; mutable names do not.
#[allow(clippy::too_many_arguments)]
fn resolve_instance_dictionary_by_head_id(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    ctx: &Context,
    class_name: &str,
    class_id: GlobalId,
    carrier: &Term,
    head_id: GlobalId,
    span: &Span,
    owner_label: &str,
    enforce_direct_use: bool,
) -> Result<(Term, Term), ElabError> {
    let Some(info) = class_env.instances_by_id.get(&(class_id, InstanceHeadKey::Global(head_id))) else {
        return Err(ElabError::NoInstance {
            class: class_name.to_string(),
            // No registered spelling resolves to this identity, so there is no
            // name to report. The identity is what the occurrence knows.
            ty: format!("{head_id:?}"),
            span: span.clone(),
        });
    };

    let head_name = info.head_type.as_ref().map(rtype_head_name)
        .unwrap_or_else(|| format!("{head_id:?}"));
    // A carrier-parameterized class gives the expected-carrier check a real
    // parameter. A nullary class cannot use this expression-side adapter.
    if !class_env
        .class_by_id(class_id)
        .map(|view| view.projection.head_param.is_some())
        .unwrap_or(false)
    {
        return Err(ElabError::NoInstance {
            class: class_name.to_string(),
            ty: head_name,
            span: span.clone(),
        });
    }

    // The common resolver confirms both class ID and carrier after inference
    // and before recording successful provenance.
    resolve_instance_dictionary_inner(
        env,
        globals,
        preconditions,
        num_values,
        numeric_env,
        refinement_facts,
        class_env,
        provenance,
        ctx,
        class_name,
        class_id,
        InstanceHeadKey::Global(head_id),
        &head_name,
        InstanceHeadRequest::Core {
            expected_carrier: carrier,
        },
        span,
        owner_label,
        enforce_direct_use,
    )
}

#[allow(clippy::too_many_arguments)]
fn resolve_instance_dictionary_inner(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    ctx: &Context,
    class_name: &str,
    class_id: GlobalId,
    head_key: InstanceHeadKey,
    // `head_name` is retained for diagnostics only; `head_key` selects the
    // checked constructor or an explicitly non-global head shape.
    //
    // `requested` preserves the caller's real representation: the declaration
    // path supplies surface syntax, while an inferred expression carrier
    // supplies core. Parameterized matching is selected here, inside the sole
    // registry dispatcher; the identity adapter never selects a dictionary.
    head_name: &str,
    requested: InstanceHeadRequest<'_>,
    span: &Span,
    owner_label: &str,
    enforce_direct_use: bool,
) -> Result<(Term, Term), ElabError> {
    let head_name = head_name.to_string();
    let info = class_env
        .instances_by_id
        .get(&(class_id, head_key))
        .cloned()
        .ok_or_else(|| {
            // The name view is diagnostic only. It can explain a stale
            // same-class, same-spelling head without selecting its dictionary.
            if class_env.instances.get(&(class_name.to_string(), head_name.clone()))
                .is_some_and(|old| old.class_id == class_id)
            {
                ElabError::InstanceCarrierIdentityMismatch {
                    class: class_name.to_string(),
                    spelling: head_name.clone(),
                    span: span.clone(),
                }
            } else {
                ElabError::NoInstance {
                    class: class_name.to_string(),
                    ty: head_name.clone(),
                    span: span.clone(),
                }
            }
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
    let expected_carrier = requested.expected_carrier();
    let (type_args, core_args) = if info.head_param_count == 0 {
        (Some(Vec::new()), Vec::new())
    } else if let Some(pattern) = &info.head_type {
        match requested {
            InstanceHeadRequest::Surface { requested, .. } => {
                let mut matched = vec![None; info.head_param_count];
                if !match_instance_head(pattern, requested, globals, info.head_param_count, &mut matched) {
                    return Err(ElabError::NoInstance {
                        class: class_name.to_string(),
                        ty: head_name.clone(),
                        span: span.clone(),
                    });
                }
                let type_args = matched
                    .into_iter()
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| ElabError::NoInstance {
                        class: class_name.to_string(),
                        ty: head_name.clone(),
                        span: span.clone(),
                    })?;
                let core_args = {
                    let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, owner_label)
                        .with_preconditions(preconditions, PremiseHoles::Refused);
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
                (Some(type_args), core_args)
            }
            InstanceHeadRequest::Core {
                expected_carrier: requested_core,
            } => {
                let mut matched = vec![None; info.head_param_count];
                if !match_instance_head_core(
                    env,
                    globals,
                    ctx,
                    pattern,
                    requested_core,
                    info.head_param_count,
                    &mut matched,
                ) {
                    return Err(ElabError::NoInstance {
                        class: class_name.to_string(),
                        ty: head_name.clone(),
                        span: span.clone(),
                    });
                }
                let core_args = matched
                    .into_iter()
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| ElabError::NoInstance {
                        class: class_name.to_string(),
                        ty: head_name.clone(),
                        span: span.clone(),
                    })?;
                (None, core_args)
            }
        }
    } else {
        return Err(ElabError::NoInstance {
            class: class_name.to_string(),
            ty: head_name.clone(),
            span: span.clone(),
        });
    };
    let mut candidate =
        ken_kernel::subst::apply_args(Term::const_(info.instance_id, vec![]), &core_args);
    for constraint in &info.constraints {
        let required_type = ken_kernel::subst::subst_tel(&constraint.core_type, &core_args);
        let constraint_is_parameterized = class_env
            .class_by_id(constraint.class_id)
            .map(|class| class.projection.head_param.is_some())
            .unwrap_or(false);
        let required_carrier = if constraint_is_parameterized {
            let carrier = match &required_type {
                Term::App(_, carrier) => carrier.as_ref().clone(),
                _ => {
                    return Err(ElabError::NoInstance {
                        class: constraint.class_name.clone(),
                        ty: format!("{required_type:?}"),
                        span: span.clone(),
                    })
                }
            };
            Some(whnf(env, ctx, &carrier))
        } else {
            None
        };
        let (dictionary, _) = if let Some(type_args) = &type_args {
            let required_head =
                instantiate_instance_rtype(&constraint.head_type, type_args, info.head_param_count);
            resolve_instance_dictionary_inner(
                env,
                globals,
                preconditions,
                num_values,
                numeric_env,
                refinement_facts,
                class_env,
                provenance,
                ctx,
                &constraint.class_name,
                constraint.class_id,
                instance_head_key(&required_head, required_carrier.as_ref().unwrap_or(&required_type)),
                &rtype_head_name(&required_head),
                InstanceHeadRequest::Surface {
                    requested: &required_head,
                    expected_carrier: required_carrier.as_ref(),
                },
                span,
                owner_label,
                false,
            )?
        } else {
            let Some(required_carrier) = required_carrier else {
                return Err(ElabError::NoInstance {
                    class: constraint.class_name.clone(),
                    ty: format!("{required_type:?}"),
                    span: span.clone(),
                });
            };
            let Some(required_head_id) = core_type_head_id(&required_carrier) else {
                return Err(ElabError::NoInstance {
                    class: constraint.class_name.clone(),
                    ty: format!("{required_carrier:?}"),
                    span: span.clone(),
                });
            };
            resolve_instance_dictionary_by_head_id(
                env,
                globals,
                preconditions,
                num_values,
                numeric_env,
                refinement_facts,
                class_env,
                provenance,
                ctx,
                &constraint.class_name,
                constraint.class_id,
                &required_carrier,
                required_head_id,
                span,
                owner_label,
                false,
            )?
        };
        candidate = Term::app(candidate, dictionary);
    }
    let ty = kernel_infer_raw(env, ctx, &candidate).map_err(|error| ElabError::KernelRejected {
        error,
        span: span.clone(),
    })?;
    confirm_instance_dictionary_carrier(
        env,
        class_env,
        ctx,
        class_name,
        class_id,
        &head_name,
        &ty,
        expected_carrier,
        span,
    )?;
    if enforce_direct_use {
        provenance.push(crate::classes::InstanceResolution {
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

pub(crate) fn surface_declared_row_type(
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
        RType::RTrunc(inner, _) => type_contains_effect_row(inner),
        // A projection's base is an EXPRESSION and this walk is type-side only,
        // exactly as `RRefine` above inspects its carrier and not its predicate.
        //
        // **`false` here rests on `Type::TProj`'s stated base invariant, NOT on
        // the type forbidding a row.** The base is declared `Box<Expr>`; what
        // rules a row out is that the sole producer emits an `EVar`/`EProj`
        // chain. This answer therefore fails OPEN if that producer set widens —
        // see the invariant paragraph at `Type::TProj`, which names this
        // function as one of its two dependants.
        RType::RProj(_, _, _) => false,
        RType::RUniv(_, _)
        | RType::RCon(_, _)
        | RType::RCheckedGlobal { .. }
        | RType::RVarTy(_, _, _)
        | RType::RPatternAliasTy(_, _, _) => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RTypeHead {
    Con(String, Option<GlobalId>),
    Var(usize, String),
}

fn rtype_heads_match(a: &RTypeHead, b: &RTypeHead) -> bool {
    match (a, b) {
        (RTypeHead::Con(_, Some(a)), RTypeHead::Con(_, Some(b))) => a == b,
        (RTypeHead::Con(a, _), RTypeHead::Con(b, _)) => a == b,
        (RTypeHead::Var(_, a), RTypeHead::Var(_, b)) => a == b,
        (RTypeHead::Con(a, _), RTypeHead::Var(_, b))
        | (RTypeHead::Var(_, a), RTypeHead::Con(b, _)) => a == b,
    }
}

fn rtype_app_head(ty: &RType) -> Option<RTypeHead> {
    match ty {
        RType::RApp(f, _, _) => rtype_app_head(f),
        RType::RCon(name, _) => Some(RTypeHead::Con(name.clone(), None)),
        RType::RCheckedGlobal { name, id, .. } => {
            Some(RTypeHead::Con(name.clone(), Some(*id)))
        }
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
            matches!(&**f,
                RType::RCon(name, _) | RType::RCheckedGlobal { name, .. }
                    if name == "Applicative"
            )
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

fn view_param_count(rdecl: &RDecl) -> usize {
    match &rdecl.kind {
        RDeclKind::View { param_count, .. } => *param_count,
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
    bound_dict_classes: &'a [(String, GlobalId)],
}

fn instance_class_for_global(
    class_env: &ClassEnv,
    instance_id: GlobalId,
) -> Option<&InstanceInfo> {
    class_env.instances_by_id.values().find(|inst| inst.instance_id == instance_id)
}

fn constraint_instance_id(
    constraint: &RInstanceConstraint,
    ctx: &ProjectionPurityCtx<'_>,
) -> Option<GlobalId> {
    let class_id = constraint.class_id.or_else(|| {
        ctx.class_env.class(&constraint.class_name).map(|info| info.projection.type_id)
    })?;
    let head_key = match &constraint.head_type {
        RType::RVarTy(_, name, _) => InstanceHeadKey::Parameter(name.clone()),
        RType::RCheckedGlobal { id, .. } => InstanceHeadKey::Global(*id),
        RType::RCon(name, _) => InstanceHeadKey::Global(*ctx.globals.get(name)?),
        RType::RApp(..) => InstanceHeadKey::Global(
            named_head_id(&constraint.head_type, ctx.globals)?
        ),
        _ => return None, // no inferred core type in this purity-only view
    };
    ctx.class_env.instances_by_id.get(&(class_id, head_key)).map(|info| info.instance_id)
}

fn projected_instance_id(base: &RExpr, ctx: &ProjectionPurityCtx<'_>) -> Option<GlobalId> {
    match base {
        RExpr::RCon(name, _)
            if ctx.local_constraints.len() == 1
                && (name == "d" || name == &ctx.local_constraints[0].binder) =>
        {
            let constraint = &ctx.local_constraints[0];
            constraint_instance_id(constraint, ctx)
        }
        RExpr::RCon(name, _) => {
            if let Some(constraint) = ctx
                .local_constraints
                .iter()
                .find(|constraint| constraint.binder == *name)
            {
                constraint_instance_id(constraint, ctx)
            } else {
                ctx.globals.get(name).copied()
            }
        }
        RExpr::RCheckedGlobal { id, .. } => Some(*id),
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
        if let Some((_, class_id)) = ctx.bound_dict_classes.iter().find(|(n, _)| n == name) {
            return projected_class_field_row_type(ctx.class_env, *class_id, field);
        }
    }
    let Some(instance_id) = projected_instance_id(base, ctx) else {
        return crate::effects::RowType::empty();
    };
    let Some(instance) = instance_class_for_global(ctx.class_env, instance_id) else {
        return crate::effects::RowType::empty();
    };
    let class_name = &instance.class_name;
    let Some(class_info) = ctx.class_env.class_by_id(instance.class_id) else {
        return crate::effects::RowType::singleton("unknown checked class owner");
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
        .instances_by_id
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
    class_id: GlobalId,
    field: &str,
) -> crate::effects::RowType {
    let Some(class_info) = class_env.class_by_id(class_id) else {
        return crate::effects::RowType::singleton("unknown checked class owner");
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
            class_info.projection.owner_name, field
        )),
        _ => crate::effects::RowType::empty(),
    }
}

fn class_id_for_dictionary_type(class_env: &ClassEnv, ty: &RType) -> Option<GlobalId> {
    let head = match ty { RType::RApp(f, _, _) => f.as_ref(), _ => ty };
    match head {
        RType::RCheckedGlobal { id, .. } => {
            // A known record is not a class. An unknown selected owner must
            // not disappear from a purity check as an empty effect row.
            (class_env.class_by_id(*id).is_some()
                || class_env.projection_by_type_id(*id).is_none()).then_some(*id)
        }
        RType::RCon(name, _) => class_env.class(name).map(|info| info.projection.type_id),
        _ => None,
    }
}

fn collect_bound_dictionary_params(
    ty: Option<&RType>,
    class_env: &ClassEnv,
) -> Vec<(String, GlobalId)> {
    let mut dicts = Vec::new();
    let mut cur = ty;
    while let Some(RType::RPi(name, domain, codomain, _)) = cur {
        if let Some(class_id) = class_id_for_dictionary_type(class_env, domain) {
            dicts.push((name.clone(), class_id));
        }
        cur = Some(codomain);
    }
    dicts
}

/// Two indices for one effect assertion: mutable spellings support local
/// forward declarations; checked imported IDs preserve their owner's row
/// after a different provider overwrites the same canonical spelling.
pub(crate) struct CheckedEffectRows<'a> {
    names: &'a HashMap<String, crate::effects::RowType>,
    ids: &'a HashMap<GlobalId, crate::effects::RowType>,
}

impl<'a> CheckedEffectRows<'a> {
    pub(crate) fn new(
        names: &'a HashMap<String, crate::effects::RowType>,
        ids: &'a HashMap<GlobalId, crate::effects::RowType>,
    ) -> Self {
        Self { names, ids }
    }
}

fn infer_expr_row_type(
    expr: &RExpr,
    effect_rows: &CheckedEffectRows<'_>,
    projection_ctx: Option<&ProjectionPurityCtx<'_>>,
) -> crate::effects::RowType {
    match expr {
        RExpr::RCon(name, _) => effect_rows
            .names
            .get(name)
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RCheckedGlobal { id, .. } => effect_rows
            .ids
            .get(id)
            .cloned()
            .unwrap_or_else(crate::effects::RowType::empty),
        RExpr::RVar(_, _, _)
        | RExpr::RPatternAlias(_, _, _)
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
            .names
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
        // The operands' rows, joined -- identical to `RBinOp` below, because
        // the operator itself is a global binding and contributes no row of
        // its own at this stage.
        RExpr::RStandardOp { lhs, rhs, .. } => {
            let left = infer_expr_row_type(lhs, effect_rows, projection_ctx);
            left.join(infer_expr_row_type(rhs, effect_rows, projection_ctx))
        }
        RExpr::RBinOp(_, l, r, _) => infer_expr_row_type(l, effect_rows, projection_ctx)
            .join(infer_expr_row_type(r, effect_rows, projection_ctx)),
        RExpr::RInfixSpine {
            operands,
            operators,
            ..
        } => {
            let mut row = operands
                .iter()
                .fold(crate::effects::RowType::empty(), |row, operand| {
                    row.join(infer_expr_row_type(operand, effect_rows, projection_ctx))
                });
            for operator in operators {
                let operator_row = match operator {
                    RInfixOperator::User(name, _) => effect_rows.names.get(name),
                    RInfixOperator::CheckedUser(_, id, _) => effect_rows.ids.get(id),
                    RInfixOperator::Builtin(_, _) => None,
                };
                if let Some(operator_row) = operator_row {
                    row = row.join(operator_row.clone());
                }
            }
            row
        }
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
pub(crate) fn check_surface_purity(
    rdecl: &RDecl,
    effect_rows: &HashMap<String, crate::effects::RowType>,
    effect_rows_by_id: &HashMap<GlobalId, crate::effects::RowType>,
    globals: &HashMap<String, GlobalId>,
    class_env: &ClassEnv,
) -> Result<(), ElabError> {
    let (keyword, is_space_op, visits, constraints) = match &rdecl.kind {
        RDeclKind::View {
            keyword,
            is_space_op,
            visits,
            constraints,
            ..
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
        &CheckedEffectRows::new(effect_rows, effect_rows_by_id),
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

fn builtin_fixity(operator: BinOp) -> Fixity {
    match operator {
        BinOp::EqEq => Fixity {
            associativity: FixityAssoc::Left,
            precedence: 4,
        },
        BinOp::Add | BinOp::WrappingAdd | BinOp::Sub => Fixity {
            associativity: FixityAssoc::Left,
            precedence: 6,
        },
        BinOp::Mul | BinOp::Div | BinOp::Mod => Fixity {
            associativity: FixityAssoc::Left,
            precedence: 7,
        },
    }
}

fn resolved_operator_fixity(
    operator: &RInfixOperator,
    globals: &HashMap<String, GlobalId>,
    fixities: &HashMap<GlobalId, Fixity>,
) -> Result<(Fixity, String), ElabError> {
    match operator {
        RInfixOperator::Builtin(operator, _) => {
            Ok((builtin_fixity(*operator), format!("{operator:?}")))
        }
        RInfixOperator::User(name, span)
        | RInfixOperator::CheckedUser(name, _, span) => {
            let id = match operator {
                RInfixOperator::CheckedUser(_, id, _) => Some(*id),
                _ => globals.get(name).copied(),
            }
            .ok_or_else(|| ElabError::UnboundName {
                name: name.clone(),
                span: span.clone(),
            })?;
            Ok((
                fixities.get(&id).copied().unwrap_or(Fixity::DEFAULT),
                name.clone(),
            ))
        }
    }
}

/// Reduce one operator against the two operands on top of the value stack.
///
/// **THIS IS THE ONLY PLACE OPERATOR POSITION IS STILL VISIBLE**, and the only
/// place [`RExpr::RStandardOp`] is minted. Its two call sites are both inside
/// `reassociate_rexpr`'s spine arm, so an explicit application never reaches
/// it -- which is what makes AC-2(c) (`ord_leq_at Nat d` stays partial) a
/// property of the tree rather than of a guard.
/// `39 §6.9` standard-operator call completion, at the occurrence.
///
/// **The role is recovered from the IDENTITY, by reverse lookup in the
/// certified map.** That is what makes `§6.9`'s *"binds to the defining
/// `GlobalId`, never to the occurrence's glyph text"* true of the
/// implementation and not just of the node: an alias reaching the same binding
/// completes identically, and the lookup is a function because
/// `certify_roles` refuses a home that binds two roles to one identity.
fn elab_standard_operator(
    cx: &mut ElabCtx,
    op: GlobalId,
    lhs: &RExpr,
    rhs: &RExpr,
    span: &Span,
) -> Result<(Term, Term), ElabError> {
    let role = cx
        .standard_operators
        .and_then(|roles| {
            roles
                .iter()
                .find_map(|(&role, &id)| (id == op).then_some(role))
        })
        .ok_or_else(|| {
            // Minting consulted the same map, so reaching here means the map
            // changed between reduction and elaboration. Fail closed and say
            // which invariant broke rather than guessing a role.
            ElabError::Internal(format!(
                "standard-operator occurrence at {}-{} carries identity {:?}, \
                 which the certified map no longer contains",
                span.start, span.end, op
            ))
        })?;

    match role {
        // `∧` and `∨` bind `bool_and` / `bool_or`, whose telescope is already
        // `Bool -> Bool -> Bool`. THERE IS NO OMITTED PREFIX TO SUPPLY, so
        // completion here is the saturated application itself.
        //
        // **Both operands are CHECKED, which is AC-7's no-short-circuit
        // property holding by construction rather than by a guard.** Under
        // call-by-value both are evaluated before the body runs; `bool_and`'s
        // body matching on its first argument is about the body's ARMS, which
        // `33 §6.1` names as the conflation to avoid.
        StandardOperatorRole::And | StandardOperatorRole::Or => {
            let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
            let lhs_core = check(cx, lhs, &bool_ty, span)?;
            let rhs_core = check(cx, rhs, &bool_ty, span)?;
            let applied = Term::app(Term::app(Term::const_(op, vec![]), lhs_core), rhs_core);
            Ok((applied, bool_ty))
        }
        // `≤` and `≥` bind `ord_leq_at` / `ord_geq_at`, whose telescope is
        // `(a : Type) -> Ord a -> a -> a -> Bool`. Completion must infer the
        // carrier and resolve the `Ord` dictionary by `§6.2`'s ordinary
        // instance search before the saturated application exists.
        StandardOperatorRole::Leq | StandardOperatorRole::Geq => {
            // `§6.9`'s ORDER, exactly: infer the operand carrier, resolve the
            // dictionary by the ordinary `§6.2` search, then check the
            // saturated application.
            let (lhs_core, lhs_ty) = infer(cx, lhs)?;
            let carrier = whnf(cx.env, &cx.ctx, &lhs_ty);

            // THE CARRIER'S HEAD IDENTITY, peeled through every core
            // application. A carrier whose spine has no type constant at its
            // head has no registry key and no instance; refusing here is
            // `§6.9`'s "carrier un-inferable" step failing at the occurrence.
            let Some(head_id) = core_type_head_id(&carrier) else {
                return Err(ElabError::NoInstance {
                    class: "Ord".to_string(),
                    ty: format!("{carrier:?}"),
                    span: span.clone(),
                });
            };

            // The right operand is CHECKED at the carrier, which is what makes
            // `x ≤ y` reject a mismatched pair at the occurrence rather than
            // inside the binding.
            let rhs_core = check(cx, rhs, &carrier, span)?;
            let preconditions = cx.preconditions.clone();

            let (dictionary, _) = {
                // Destructured so the resolver's `&mut` arguments are disjoint
                // FIELD borrows of `cx` rather than several borrows of `cx`
                // itself.
                let ElabCtx {
                    env,
                    globals,
                    num_values,
                    numeric_env,
                    refinement_facts,
                    ctx,
                    class_env,
                    provenance,
                    owner_label,
                    ..
                } = &mut *cx;
                let class_env = class_env.ok_or_else(|| {
                    ElabError::Internal(format!(
                        "standard operator '{}' at {}-{} reached completion with \
                         no class registry; `with_classes` was not applied on \
                         this path",
                        role.glyph(),
                        span.start,
                        span.end
                    ))
                })?;
                let provenance = provenance.as_deref_mut().ok_or_else(|| {
                    ElabError::Internal(format!(
                        "standard operator '{}' at {}-{} reached completion with \
                         no provenance sink; `with_classes` takes the registry \
                         and the sink together so this cannot happen singly",
                        role.glyph(),
                        span.start,
                        span.end
                    ))
                })?;
                resolve_instance_dictionary_by_head_id(
                    env,
                    globals,
                    &preconditions,
                    num_values,
                    numeric_env,
                    refinement_facts,
                    class_env,
                    provenance,
                    ctx,
                    "Ord",
                    standard_operator_class_id(env, op, class_env, "Ord")?,
                    &carrier,
                    head_id,
                    span,
                    owner_label,
                    true,
                )?
            };

            // THE SATURATED APPLICATION, in source operand order for BOTH
            // roles. `≥` does NOT reverse here: `ord_geq_at a d x y = d.leq y x`
            // reverses inside the binding, on values call-by-value has already
            // evaluated left to right. Reversing at the call site too would
            // double-reverse, and AC-4's control is exactly that mutation.
            let applied = Term::app(
                Term::app(
                    Term::app(Term::app(Term::const_(op, vec![]), carrier), dictionary),
                    lhs_core,
                ),
                rhs_core,
            );
            Ok((applied, Term::indformer(cx.numeric_env.bool_id, vec![])))
        }
        // `∈` binds `membership_member_at`, whose telescope is
        // `(c : Type) -> Membership c -> d.Query -> c -> Bool`. The provider
        // is selected from the RIGHT-HAND carrier before the left query is
        // checked; the query never participates in provider selection.
        StandardOperatorRole::Member => {
            let (rhs_core, rhs_ty) = infer(cx, rhs)?;
            let carrier = whnf(cx.env, &cx.ctx, &rhs_ty);
            let Some(head_id) = core_type_head_id(&carrier) else {
                return Err(ElabError::NoInstance {
                    class: "Membership".to_string(),
                    ty: format!("{carrier:?}"),
                    span: span.clone(),
                });
            };

            let preconditions = cx.preconditions.clone();
            let (dictionary, _) = {
                let ElabCtx {
                    env,
                    globals,
                    num_values,
                    numeric_env,
                    refinement_facts,
                    ctx,
                    class_env,
                    provenance,
                    owner_label,
                    ..
                } = &mut *cx;
                let class_env = class_env.ok_or_else(|| {
                    ElabError::Internal(format!(
                        "standard operator '{}' at {}-{} reached completion with \
                         no class registry; `with_classes` was not applied on \
                         this path",
                        role.glyph(),
                        span.start,
                        span.end
                    ))
                })?;
                let provenance = provenance.as_deref_mut().ok_or_else(|| {
                    ElabError::Internal(format!(
                        "standard operator '{}' at {}-{} reached completion with \
                         no provenance sink; `with_classes` takes the registry \
                         and the sink together so this cannot happen singly",
                        role.glyph(),
                        span.start,
                        span.end
                    ))
                })?;
                resolve_instance_dictionary_by_head_id(
                    env,
                    globals,
                    &preconditions,
                    num_values,
                    numeric_env,
                    refinement_facts,
                    class_env,
                    provenance,
                    ctx,
                    "Membership",
                    standard_operator_class_id(env, op, class_env, "Membership")?,
                    &carrier,
                    head_id,
                    span,
                    owner_label,
                    true,
                )?
            };

            let query_ty = whnf(cx.env, &cx.ctx, &Term::proj1(dictionary.clone()));
            let lhs_core = check(cx, lhs, &query_ty, span)?;
            let applied = Term::app(
                Term::app(
                    Term::app(Term::app(Term::const_(op, vec![]), carrier), dictionary),
                    lhs_core,
                ),
                rhs_core,
            );
            Ok((applied, Term::indformer(cx.numeric_env.bool_id, vec![])))
        }
        // `≠` is authored rather than re-exported (D2), so the home does not
        // publish it and `certify_roles` never admits it -- this node is not
        // minted for it. Unreachable via the certified map, and it fails
        // closed rather than pretending otherwise.
        StandardOperatorRole::Neq => Err(ElabError::Internal(format!(
            "standard operator '≠' at {}-{} reached completion, but it is not a \
             binding-backed role and the home cannot certify it",
            span.start, span.end
        ))),
    }
}

fn reduce_resolved_operator(
    values: &mut Vec<RExpr>,
    operator: RInfixOperator,
    globals: &HashMap<String, GlobalId>,
    standard_operators: Option<&HashMap<StandardOperatorRole, GlobalId>>,
) {
    let rhs = values.pop().expect("an infix operator has a right operand");
    let lhs = values.pop().expect("an infix operator has a left operand");
    let span = Span::merge(lhs.span(), rhs.span());
    let imported_id = match &operator {
        RInfixOperator::CheckedUser(_, id, _) => Some(*id),
        _ => None,
    };
    let combined = match operator {
        RInfixOperator::Builtin(operator, _) => {
            RExpr::RBinOp(operator, Box::new(lhs), Box::new(rhs), span)
        }
        RInfixOperator::User(name, operator_span)
        | RInfixOperator::CheckedUser(name, _, operator_span) => {
            let selected_id = imported_id.or_else(|| globals.get(&name).copied());
            // KEYED ON THE RESOLVED IDENTITY, NOT ON `name`. `39 §6.9` binds
            // completion to the defining `GlobalId` "never to the occurrence's
            // glyph text", so this resolves the surface name first and then
            // asks whether that identity is one the home certified. An alias
            // reaching the same binding under a different spelling mints the
            // same node; an unrelated local `≤` resolves elsewhere and does
            // not (AC-2(b)).
            let certified = standard_operators.and_then(|roles| {
                let id = selected_id?;
                roles.values().any(|&certified| certified == id).then_some(id)
            });
            match certified {
                Some(op) => RExpr::RStandardOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span,
                },
                // The ordinary user-operator spine, unchanged.
                //
                // **RESIDUAL: REACHABLE, UNFIXTURED.** A standard occurrence
                // taking this arm is left as a two-argument application of a
                // four-argument binding, caught downstream by the kernel check
                // rather than named as a role. Deciding it here without the
                // map would mean keying on the glyph, which is the thing
                // `§6.9` forbids.
                //
                // AN EARLIER VERSION OF THIS COMMENT CALLED THE CASE
                // UNCONSTRUCTIBLE, and the reason it gave covered only half of
                // it. It reasoned about `standard_operators == None` -- the
                // paths that elaborate no user body -- and concluded the map is
                // absent exactly where no user body elaborates. But an ABSENT
                // HOME yields `Some(empty)`, not `None`: `certify_roles`
                // returns an empty map rather than declining, and an empty map
                // certifies nothing, so this arm is reached WITH a user body.
                //
                // The shape that reaches it: import `ord_leq_at as ≤` directly
                // while `Core.Operators.Standard` is absent from the program.
                // `≤` then resolves to the four-argument binding, nothing is
                // certified, and the occurrence lands here.
                //
                // That construction is stated and NOT BUILT. A named, unbuilt
                // construction is a better thing to hand an attacker than a
                // claim of non-constructibility, which is what this said
                // before and what the paragraph above refutes.
                None => {
                    let head = match selected_id {
                        Some(id) if imported_id.is_some() => {
                            RExpr::RCheckedGlobal { name, id, span: operator_span.clone() }
                        }
                        _ => RExpr::RCon(name, operator_span.clone()),
                    };
                    let first_span = Span::merge(head.span(), lhs.span());
                    let applied = RExpr::RApp(Box::new(head), Box::new(lhs), first_span);
                    RExpr::RApp(Box::new(applied), Box::new(rhs), span)
                }
            }
        }
    };
    values.push(combined);
}

fn should_reduce_before(
    top: &RInfixOperator,
    incoming: &RInfixOperator,
    globals: &HashMap<String, GlobalId>,
    fixities: &HashMap<GlobalId, Fixity>,
) -> Result<bool, ElabError> {
    let (top_fixity, top_name) = resolved_operator_fixity(top, globals, fixities)?;
    let (incoming_fixity, incoming_name) = resolved_operator_fixity(incoming, globals, fixities)?;
    if top_fixity.precedence != incoming_fixity.precedence {
        return Ok(top_fixity.precedence > incoming_fixity.precedence);
    }
    if top_fixity.associativity == FixityAssoc::NonAssociative
        || incoming_fixity.associativity == FixityAssoc::NonAssociative
    {
        let operator = if top_fixity.associativity == FixityAssoc::NonAssociative {
            top_name
        } else {
            incoming_name
        };
        return Err(ElabError::NonAssociativeInfix {
            operator,
            first_span: top.span().clone(),
            second_span: incoming.span().clone(),
        });
    }
    match (top_fixity.associativity, incoming_fixity.associativity) {
        (FixityAssoc::Left, FixityAssoc::Left) => Ok(true),
        (FixityAssoc::Right, FixityAssoc::Right) => Ok(false),
        _ => Err(ElabError::TypeMismatch {
            span: Span::merge(top.span(), incoming.span()),
            reason: format!(
                "operators '{}' and '{}' have conflicting associativity at precedence {}",
                top_name, incoming_name, top_fixity.precedence
            ),
        }),
    }
}

fn reassociate_rexpr(
    expr: RExpr,
    globals: &HashMap<String, GlobalId>,
    fixities: &HashMap<GlobalId, Fixity>,
    standard_operators: Option<&HashMap<StandardOperatorRole, GlobalId>>,
) -> Result<RExpr, ElabError> {
    Ok(match expr {
        RExpr::RApp(function, argument, span) => RExpr::RApp(
            Box::new(reassociate_rexpr(*function, globals, fixities, standard_operators)?),
            Box::new(reassociate_rexpr(*argument, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RLam(name, body, span) => RExpr::RLam(
            name,
            Box::new(reassociate_rexpr(*body, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RLet(name, ty, value, body, span) => RExpr::RLet(
            name,
            ty.map(|ty| reassociate_rtype(ty, globals, fixities, standard_operators))
                .transpose()?,
            Box::new(reassociate_rexpr(*value, globals, fixities, standard_operators)?),
            Box::new(reassociate_rexpr(*body, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RAsc(value, ty, span) => RExpr::RAsc(
            Box::new(reassociate_rexpr(*value, globals, fixities, standard_operators)?),
            Box::new(reassociate_rtype(*ty, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::ROld(value, span) => RExpr::ROld(
            Box::new(reassociate_rexpr(*value, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RBecomes(index, name, value, span) => RExpr::RBecomes(
            index,
            name,
            Box::new(reassociate_rexpr(*value, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RBinOp(operator, lhs, rhs, span) => RExpr::RBinOp(
            operator,
            Box::new(reassociate_rexpr(*lhs, globals, fixities, standard_operators)?),
            Box::new(reassociate_rexpr(*rhs, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RInfixSpine {
            operands,
            operators,
            ..
        } => {
            let mut operands = operands
                .into_iter()
                .map(|operand| reassociate_rexpr(operand, globals, fixities, standard_operators));
            let mut values = vec![operands
                .next()
                .expect("a parsed spine has one more operand")?];
            let mut pending: Vec<RInfixOperator> = Vec::new();
            for (operator, rhs) in operators.into_iter().zip(operands) {
                while let Some(top) = pending.last() {
                    if !should_reduce_before(top, &operator, globals, fixities)? {
                        break;
                    }
                    reduce_resolved_operator(
                        &mut values,
                        pending.pop().expect("pending operator exists"),
                        globals,
                        standard_operators,
                    );
                }
                pending.push(operator);
                values.push(rhs?);
            }
            while let Some(operator) = pending.pop() {
                reduce_resolved_operator(&mut values, operator, globals, standard_operators);
            }
            values.pop().expect("reassociation produces one expression")
        }
        RExpr::RMatch {
            scrut,
            equation,
            arms,
            span,
        } => RExpr::RMatch {
            scrut: Box::new(reassociate_rexpr(*scrut, globals, fixities, standard_operators)?),
            equation,
            arms: arms
                .into_iter()
                .map(|arm| {
                    Ok(RMatchArm {
                        pat: arm.pat,
                        guard: arm
                            .guard
                            .map(|guard| reassociate_rexpr(guard, globals, fixities, standard_operators))
                            .transpose()?,
                        body: reassociate_rexpr(arm.body, globals, fixities, standard_operators)?,
                        span: arm.span,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
            span,
        },
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            span,
        } => RExpr::RIf {
            condition: Box::new(reassociate_rexpr(*condition, globals, fixities, standard_operators)?),
            then_branch: Box::new(reassociate_rexpr(*then_branch, globals, fixities, standard_operators)?),
            else_branch: Box::new(reassociate_rexpr(*else_branch, globals, fixities, standard_operators)?),
            span,
        },
        RExpr::RPair(components, span) => RExpr::RPair(
            components
                .into_iter()
                .map(|component| reassociate_rexpr(component, globals, fixities, standard_operators))
                .collect::<Result<Vec<_>, _>>()?,
            span,
        ),
        RExpr::RRecord { base, fields, span } => RExpr::RRecord {
            base: base
                .map(|base| reassociate_rexpr(*base, globals, fixities, standard_operators).map(Box::new))
                .transpose()?,
            fields: fields
                .into_iter()
                .map(|(name, value, name_span)| {
                    Ok((
                        name,
                        reassociate_rexpr(value, globals, fixities, standard_operators)?,
                        name_span,
                    ))
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
            span,
        },
        RExpr::RProj(value, field, span) => RExpr::RProj(
            Box::new(reassociate_rexpr(*value, globals, fixities, standard_operators)?),
            field,
            span,
        ),
        RExpr::RPosProj(value, index, span) => RExpr::RPosProj(
            Box::new(reassociate_rexpr(*value, globals, fixities, standard_operators)?),
            index,
            span,
        ),
        RExpr::RPi(name, domain, codomain, span) => RExpr::RPi(
            name,
            Box::new(reassociate_rtype(*domain, globals, fixities, standard_operators)?),
            Box::new(reassociate_rexpr(*codomain, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RArrow(domain, codomain, span) => RExpr::RArrow(
            Box::new(reassociate_rexpr(*domain, globals, fixities, standard_operators)?),
            Box::new(reassociate_rexpr(*codomain, globals, fixities, standard_operators)?),
            span,
        ),
        RExpr::RTrunc(inner, span) => RExpr::RTrunc(
            Box::new(reassociate_rexpr(*inner, globals, fixities, standard_operators)?),
            span,
        ),
        // A LEAF HERE, AND THE ARGUMENT IS WHY -- stated because
        // `leaf => leaf` below would have swallowed this variant with no
        // compile error, the same invisible shape as the `modules.rs` remap.
        //
        // This node is MINTED BY THIS PASS, in `reduce_resolved_operator`,
        // from operands the spine arm has already reassociated. So it can only
        // be encountered on a SECOND traversal of an already-reassociated
        // tree, where descending would be a no-op. Leafing is correct and
        // descending would also be correct; what is not correct is leaving
        // which one it is to a catch-all, since the reachability argument is
        // the part that rots when the minting site moves.
        leaf @ RExpr::RStandardOp { .. } => leaf,
        leaf => leaf,
    })
}

fn reassociate_rtype(
    ty: RType,
    globals: &HashMap<String, GlobalId>,
    fixities: &HashMap<GlobalId, Fixity>,
    standard_operators: Option<&HashMap<StandardOperatorRole, GlobalId>>,
) -> Result<RType, ElabError> {
    Ok(match ty {
        RType::RPi(name, domain, codomain, span) => RType::RPi(
            name,
            Box::new(reassociate_rtype(*domain, globals, fixities, standard_operators)?),
            Box::new(reassociate_rtype(*codomain, globals, fixities, standard_operators)?),
            span,
        ),
        RType::RSigma(name, domain, codomain, span) => RType::RSigma(
            name,
            Box::new(reassociate_rtype(*domain, globals, fixities, standard_operators)?),
            Box::new(reassociate_rtype(*codomain, globals, fixities, standard_operators)?),
            span,
        ),
        RType::RArr(domain, codomain, span) => RType::RArr(
            Box::new(reassociate_rtype(*domain, globals, fixities, standard_operators)?),
            Box::new(reassociate_rtype(*codomain, globals, fixities, standard_operators)?),
            span,
        ),
        RType::REffectArr(domain, row, codomain, span) => RType::REffectArr(
            Box::new(reassociate_rtype(*domain, globals, fixities, standard_operators)?),
            row,
            Box::new(reassociate_rtype(*codomain, globals, fixities, standard_operators)?),
            span,
        ),
        RType::RRefine(name, carrier, predicate, span) => RType::RRefine(
            name,
            Box::new(reassociate_rtype(*carrier, globals, fixities, standard_operators)?),
            Box::new(reassociate_rexpr(*predicate, globals, fixities, standard_operators)?),
            span,
        ),
        RType::RApp(function, argument, span) => RType::RApp(
            Box::new(reassociate_rtype(*function, globals, fixities, standard_operators)?),
            Box::new(reassociate_rtype(*argument, globals, fixities, standard_operators)?),
            span,
        ),
        // `‖A‖` — descend into the truncated type. This is REACHING, not
        // defensive: this per-declaration pass runs at DECLARED fixity, and its
        // `RRefine` arm reassociates the predicate (`reassociate_rexpr`). A
        // refinement predicate nested in a `‖…‖` is reachable only through this
        // `RTrunc` -> `RRefine` -> `reassociate_rexpr` path — `resolve` performs
        // no reassociation. The predicate itself is proof-irrelevant and dropped
        // at elaboration, so a mis-GROUPING is unobservable; but the reassociator
        // still runs its fixity-ambiguity check over the truncated predicate, so
        // leafing this arm SILENTLY ACCEPTS an ambiguous-fixity predicate (two
        // operators of conflicting associativity / a non-associative operator in
        // a chain) that a correct descent REJECTS. Mutation-proven by
        // `d1_annotation_trunc_mixed_precedence_predicate_reassociates_under_truncation`.
        RType::RTrunc(inner, span) => {
            RType::RTrunc(Box::new(reassociate_rtype(*inner, globals, fixities, standard_operators)?), span)
        }
        // `d.Query` — descend into the base through the EXPRESSION half, the
        // same split `RRefine` above makes for its predicate.
        //
        // **Descending rather than leafing is compliance with this function's
        // own standard, not a reachability claim.** The parser emits a `TProj`
        // base of `Expr::EVar` wrapped only in `Expr::EProj` (see `Type::TProj`),
        // so no operator spine can occur there today and a leaf would be correct
        // on every input the grammar can currently produce. But this function
        // justifies each arm as reaching or not, its sibling
        // `reassociate_default_type` was given a descending arm in the same
        // change, and the unreachability argument is the part that rots when
        // the base's shape widens. Three lines cost less than the argument.
        RType::RProj(base, field, span) => RType::RProj(
            Box::new(reassociate_rexpr(*base, globals, fixities, standard_operators)?),
            field,
            span,
        ),
        leaf => leaf,
    })
}

fn install_fixity_binding(
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    id: GlobalId,
    operator: &str,
    declared: Option<(Fixity, Span)>,
) -> Result<bool, ElabError> {
    let Some((fixity, span)) = declared else {
        return Ok(false);
    };
    match fixities.get(&id).copied() {
        None => {
            fixities.insert(id, fixity);
            fixity_spans.insert(id, span);
            Ok(true)
        }
        Some(existing) if existing == fixity => Ok(false),
        Some(existing) => Err(ElabError::ConflictingFixity {
            operator: operator.to_string(),
            first: existing,
            second: fixity,
            first_span: fixity_spans.get(&id).cloned().unwrap_or_default(),
            second_span: span,
        }),
    }
}

pub(crate) fn reassociate_space_decl(
    space: &RSpaceDecl,
    globals: &HashMap<String, GlobalId>,
    fixities: &HashMap<GlobalId, Fixity>,
    standard_operators: Option<&HashMap<StandardOperatorRole, GlobalId>>,
) -> Result<Option<Box<RSpaceDecl>>, ElabError> {
    if !space.contains_infix_spine {
        return Ok(None);
    }
    let mut associated = space.clone();
    for cell in &mut associated.cells {
        cell.ty = reassociate_rtype(cell.ty.clone(), globals, fixities, standard_operators)?;
        cell.init = reassociate_rexpr(cell.init.clone(), globals, fixities, standard_operators)?;
    }
    for operation in &mut associated.operations {
        for (_, parameter_type) in &mut operation.params {
            *parameter_type = reassociate_rtype(parameter_type.clone(), globals, fixities, standard_operators)?;
        }
        operation.ret_ty = reassociate_rtype(operation.ret_ty.clone(), globals, fixities, standard_operators)?;
        operation.requires = operation
            .requires
            .clone()
            .into_iter()
            .map(|expr| reassociate_rexpr(expr, globals, fixities, standard_operators))
            .collect::<Result<Vec<_>, _>>()?;
        operation.ensures = operation
            .ensures
            .clone()
            .into_iter()
            .map(|expr| reassociate_rexpr(expr, globals, fixities, standard_operators))
            .collect::<Result<Vec<_>, _>>()?;
        operation.body = reassociate_rexpr(operation.body.clone(), globals, fixities, standard_operators)?;
    }
    Ok(Some(Box::new(associated)))
}

fn reassociate_rdecl(
    rdecl: &RDecl,
    globals: &HashMap<String, GlobalId>,
    fixities: &HashMap<GlobalId, Fixity>,
    standard_operators: Option<&HashMap<StandardOperatorRole, GlobalId>>,
) -> Result<Option<Box<RDecl>>, ElabError> {
    if !rdecl.contains_infix_spine {
        return Ok(None);
    }
    let mut associated = rdecl.clone();
    associated.ty = associated
        .ty
        .map(|ty| reassociate_rtype(ty, globals, fixities, standard_operators))
        .transpose()?;
    associated.body = reassociate_rexpr(associated.body, globals, fixities, standard_operators)?;
    associated.requires = associated
        .requires
        .into_iter()
        .map(|expr| reassociate_rexpr(expr, globals, fixities, standard_operators))
        .collect::<Result<Vec<_>, _>>()?;
    associated.ensures = associated
        .ensures
        .into_iter()
        .map(|expr| reassociate_rexpr(expr, globals, fixities, standard_operators))
        .collect::<Result<Vec<_>, _>>()?;
    match &mut associated.kind {
        RDeclKind::View { constraints, .. } => {
            for constraint in constraints {
                constraint.head_type =
                    reassociate_rtype(constraint.head_type.clone(), globals, fixities, standard_operators)?;
            }
        }
        RDeclKind::Prop { intros } => {
            for intro in intros {
                intro.ty = reassociate_rtype(intro.ty.clone(), globals, fixities, standard_operators)?;
            }
        }
        RDeclKind::Law { fields, .. } => {
            for (_, field) in fields {
                *field = reassociate_rexpr(field.clone(), globals, fixities, standard_operators)?;
            }
        }
        RDeclKind::DataDecl { ctors, .. } => {
            for ctor in ctors {
                ctor.args = ctor
                    .args
                    .clone()
                    .into_iter()
                    .map(|ty| reassociate_rtype(ty, globals, fixities, standard_operators))
                    .collect::<Result<Vec<_>, _>>()?;
            }
        }
        RDeclKind::ExplicitDataDecl {
            params,
            indices,
            ctors,
            ..
        } => {
            for entry in params.iter_mut().chain(indices.iter_mut()) {
                entry.ty = reassociate_rtype(entry.ty.clone(), globals, fixities, standard_operators)?;
            }
            for ctor in ctors {
                for entry in &mut ctor.args {
                    entry.ty = reassociate_rtype(entry.ty.clone(), globals, fixities, standard_operators)?;
                }
                ctor.result = ctor
                    .result
                    .clone()
                    .map(|ty| reassociate_rtype(ty, globals, fixities, standard_operators))
                    .transpose()?;
            }
        }
        RDeclKind::TypeAlias { ty } => {
            *ty = reassociate_rtype(ty.clone(), globals, fixities, standard_operators)?;
        }
        RDeclKind::RecordDecl { fields } => {
            for field in fields {
                field.ty = reassociate_rtype(field.ty.clone(), globals, fixities, standard_operators)?;
            }
        }
        RDeclKind::ClassDecl {
            param_kind, fields, ..
        } => {
            *param_kind = param_kind
                .clone()
                .map(|ty| reassociate_rtype(ty, globals, fixities, standard_operators))
                .transpose()?;
            for field in fields {
                field.ty = reassociate_rtype(field.ty.clone(), globals, fixities, standard_operators)?;
            }
        }
        RDeclKind::InstanceDecl {
            head_type,
            constraints,
            fields,
            ..
        } => {
            *head_type = reassociate_rtype(head_type.clone(), globals, fixities, standard_operators)?;
            for constraint in constraints {
                constraint.head_type =
                    reassociate_rtype(constraint.head_type.clone(), globals, fixities, standard_operators)?;
            }
            for (_, field) in fields {
                *field = reassociate_rexpr(field.clone(), globals, fixities, standard_operators)?;
            }
        }
        RDeclKind::Let
        | RDeclKind::Prove
        | RDeclKind::Theorem
        | RDeclKind::AttachedProof { .. }
        | RDeclKind::Foreign { .. }
        | RDeclKind::Temporal { .. }
        | RDeclKind::DeriveDecl { .. } => {}
    }
    Ok(Some(Box::new(associated)))
}

#[cfg(test)]
mod fixity_reassociation_skip_tests {
    use super::reassociate_rdecl;
    use crate::error::Span;
    use crate::resolve::{RDecl, RDeclKind, RExpr};
    use std::collections::HashMap;

    /// Promise class: durable invariant. MEASURED: resolution's negative spine
    /// marker returns the borrowed-path sentinel without cloning or walking the
    /// body. CLAIMED: legacy declarations do zero reassociation work. THE GAP:
    /// the unchanged Map stated-stack controls separately bind this structural
    /// seam to the full recursive elaboration path.
    #[test]
    fn spine_free_declaration_returns_the_zero_work_sentinel() {
        let span = Span::zero();
        let declaration = RDecl {
            name: "legacy".to_string(),
            ty: None,
            body: RExpr::RUniv(None, span.clone()),
            contains_infix_spine: false,
            requires: vec![],
            ensures: vec![],
            span,
            kind: RDeclKind::Let,
        };
        assert!(
            reassociate_rdecl(&declaration, &HashMap::new(), &HashMap::new(), None)
                .expect("spine-free reassociation cannot fail")
                .is_none(),
            "a spine-free declaration must not allocate or traverse an associated clone"
        );
    }
}

/// V1 elaboration: returns the definition id plus any emitted obligation holes.
pub(crate) fn elaborate_rdecl_v1(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    // Threaded rather than made local. `fixities`, `fixity_spans` and
    // `ctor_decl_spans` below ARE locals on this standalone path because they
    // are scoped to one declaration -- provenance is not. It accumulated into
    // the caller's registry before this change, so making it local here would
    // silently drop it on this path.
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    let mut fixities = HashMap::new();
    let mut fixity_spans = HashMap::new();
    // Standalone (non-module) path: constructor-spelling collisions are tracked
    // only within this single declaration. The persistent cross-declaration
    // registry travels through the module path via `ElabEnv::ctor_decl_spans`.
    let mut ctor_decl_spans = HashMap::new();
    let mut refinement_facts = RefinementFacts::default();
    let no_names = HashMap::new();
    let no_checked_ids = HashMap::new();
    let mut preconditions = HashMap::new();
    elaborate_rdecl_v1_with_effect_rows(
        env,
        globals,
        &mut preconditions,
        num_values,
        numeric_env,
        class_env,
        provenance,
        standard_operators,
        &CheckedEffectRows::new(&no_names, &no_checked_ids),
        &mut fixities,
        &mut fixity_spans,
        &mut ctor_decl_spans,
        &mut refinement_facts,
        None,
        rdecl,
    )
}

/// Rebuild just a declaration's explicit parameter context. Constraint terms
/// are installed at this depth, so generic dictionaries can mention the same
/// type/value parameters as the declaration without changing its telescope.
fn declaration_param_context(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
) -> Result<Context, ElabError> {
    // This walks the declaration's OWN parameter telescope, so it is the FIRST
    // place `(q : d.Query)` is elaborated on the `fn`/`const` path -- ahead of
    // both the `ensure_not_omega_type` pre-check and `elaborate_v0`. It needs
    // the class env for the same reason they do: the name-to-index lookup
    // behind a projection is a `ClassEnv` fact (`33 §6.3`, `58b §1`).
    let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
        .deferring_type_introductions()
        .with_classes(class_env, provenance, standard_operators)
        .with_preconditions(preconditions, PremiseHoles::Refused);
    let mut current = rdecl.ty.as_ref();
    while let Some(RType::RPi(_, domain, codomain, _)) = current {
        // This is a throwaway context pre-pass; the admitting signature
        // collects its predicate. Permit the same recorded domain here.
        let domain_core = elab_type_in_slot(&mut cx, domain, RefinementSlot::Outermost)?;
        cx.ctx.push(cx.metas.zonk_term(&domain_core));
        current = Some(codomain);
    }
    Ok(cx.ctx)
}

pub(crate) fn elaborate_rdecl_v1_with_effect_rows(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &mut HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    effect_rows: &CheckedEffectRows<'_>,
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    ctor_decl_spans: &mut HashMap<String, Span>,
    refinement_facts: &mut RefinementFacts,
    declared_fixity: Option<(Fixity, Span)>,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    let self_reference_waits_for_preadmission = !globals.contains_key(&rdecl.name)
        && rexpr_mentions_name(&rdecl.body, &rdecl.name)
        && matches!(rdecl.kind, RDeclKind::View { .. } | RDeclKind::Let);
    if self_reference_waits_for_preadmission || !rdecl.contains_infix_spine {
        return elaborate_associated_rdecl(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            class_env,
            provenance,
            standard_operators,
            effect_rows,
            fixities,
            fixity_spans,
            ctor_decl_spans,
            refinement_facts,
            declared_fixity,
            rdecl,
        );
    }
    let associated = reassociate_rdecl(rdecl, globals, fixities, Some(standard_operators))?
        .expect("a declaration marked with an infix spine must be reassociated");
    elaborate_associated_rdecl(
        env,
        globals,
        preconditions,
        num_values,
        numeric_env,
        class_env,
        provenance,
        standard_operators,
        effect_rows,
        fixities,
        fixity_spans,
        ctor_decl_spans,
        refinement_facts,
        declared_fixity,
        &associated,
    )
}

#[inline(never)]
fn elaborate_associated_rdecl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &mut HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    effect_rows: &CheckedEffectRows<'_>,
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    ctor_decl_spans: &mut HashMap<String, Span>,
    refinement_facts: &mut RefinementFacts,
    declared_fixity: Option<(Fixity, Span)>,
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
            // `.with_classes` is load-bearing even though this pre-pass throws
            // its result away. It exists only to run `ensure_not_omega_type`,
            // but it ELABORATES the declared type to get there -- so a
            // telescope containing `(q : d.Query)` reaches `infer_proj` HERE,
            // ahead of the real elaboration in `elaborate_v0` (which has had
            // the class env all along). Without it the projection's
            // name-to-index lookup has no field list and a well-formed binding
            // is refused by a sort pre-check.
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, &*refinement_facts, rdecl.name.clone())
                .deferring_type_introductions()
                .with_classes(&*class_env, provenance, standard_operators)
                .with_preconditions(preconditions, PremiseHoles::Refused);
            // This pre-pass checks sort but admits no definition. Match the
            // admitting signature's domain/result permissions and collect its
            // templates rather than rejecting a legitimate refined return.
            let (ty, _) = elab_signature(&mut cx, ty, innermost_refine_pred(ty).is_some())?;
            let ty_core = cx.metas.zonk_term(&ty);
            ensure_not_omega_type(cx.env, &Context::new(), &ty_core, &rdecl.span)?;
        }
    }
    match &rdecl.kind {
        RDeclKind::View { constraints, .. } => {
            let effect_row_type = check_view_visits_row(rdecl)?;
            let dictionary_ctx = declaration_param_context(
                env,
                globals,
                preconditions,
                num_values,
                numeric_env,
                &*refinement_facts,
                class_env,
                provenance,
                standard_operators,
                rdecl,
            )?;
            // Resolve each constraint into its fully applied dictionary term.
            // A generic instance is not a bare global: its type arguments and
            // recursively-required dictionaries must be applied at this use
            // site before the kernel checks the candidate.
            let mut local_dicts = HashMap::new();
            for constraint in constraints {
                let dictionary = resolve_instance_dictionary(
                    env,
                    globals,
                    preconditions,
                    num_values,
                    numeric_env,
                    &*refinement_facts,
                    class_env,
                    provenance,
                    &dictionary_ctx,
                    &constraint.class_name,
                    constraint.class_id,
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
                preconditions,
                num_values,
                numeric_env,
                class_env,
                provenance,
                standard_operators,
                rdecl,
                &local_dicts,
                refinement_facts,
                fixities,
                fixity_spans,
                declared_fixity.clone(),
            );
            if let Ok(result) = &mut result {
                result.effect_row_type = effect_row_type;
            }
            result
        }
        RDeclKind::Let => elaborate_view_or_let(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            class_env,
            provenance,
            standard_operators,
            rdecl,
            &HashMap::new(),
            refinement_facts,
            fixities,
            fixity_spans,
            declared_fixity.clone(),
        ),
        RDeclKind::Prove => {
            elaborate_prove(env, globals, preconditions, num_values, numeric_env, &*refinement_facts, rdecl)
        }
        RDeclKind::Prop { intros } => elaborate_prop_decl(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            refinement_facts,
            class_env,
            provenance,
            standard_operators,
            rdecl,
            intros,
        ),
        RDeclKind::Theorem => elaborate_checked_theorem(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            refinement_facts,
            class_env,
            provenance,
            standard_operators,
            rdecl,
            None,
        ),
        RDeclKind::AttachedProof { subject, .. } => elaborate_checked_theorem(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            refinement_facts,
            class_env,
            provenance,
            standard_operators,
            rdecl,
            Some(subject),
        ),
        RDeclKind::Law { param, fields } => elaborate_law(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            &*refinement_facts,
            rdecl,
            param.clone(),
            fields.clone(),
        ),
        RDeclKind::DataDecl { type_params, ctors } => {
            let d_id = elaborate_data_with_refinements(
                env, globals, num_values, numeric_env, ctor_decl_spans,
                refinement_facts, &rdecl.name, type_params, ctors, &rdecl.span,
            )?;
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
            let d_id = elaborate_explicit_data_with_refinements(
                env, globals, num_values, numeric_env, ctor_decl_spans,
                refinement_facts, &rdecl.name, params, indices, *level, ctors,
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
            if let Some((span, reason)) = alias_nested_refinement(ty) {
                return Err(ElabError::TypeMismatch { span: span.clone(), reason: reason.into() });
            }
            // A named refinement keeps its transparent carrier body; only its
            // GlobalId records the predicate. A literal never mints an alias.
            let (alias_body, predicate) = {
                let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, &*refinement_facts, rdecl.name.clone())
                    .with_preconditions(preconditions, PremiseHoles::Refused);
                let body = elab_type_in_slot(&mut cx, ty, RefinementSlot::Outermost)?;
                let predicate = if let RType::RRefine(_, _, phi, _) = ty {
                    cx.ctx.push(body.clone());
                    let checked = elab_prop_at_omega(&mut cx, phi, phi.span())?;
                    cx.ctx.pop();
                    Some(Term::lam(body.clone(), cx.metas.zonk_term(&checked)))
                } else {
                    None
                };
                (cx.metas.zonk_term(&body), predicate)
            };
            let alias_root = match &alias_body {
                Term::Const { id: target, .. } => refinement_facts.refinement_root(*target),
                _ => None,
            };
            let alias_ty = Term::ty(Level::Zero);
            let id = declare_def(env, vec![], alias_ty, alias_body).map_err(|e| {
                ElabError::KernelRejected {
                    error: e,
                    span: rdecl.span.clone(),
                }
            })?;
            globals.insert(rdecl.name.clone(), id);
            if let Some(predicate) = predicate {
                refinement_facts.refinement_predicates.insert(id, predicate);
            } else if let Some(root) = alias_root {
                refinement_facts.refinement_aliases.insert(id, root);
            }
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
            preconditions,
            num_values,
            numeric_env,
            &*refinement_facts,
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
            preconditions,
            num_values,
            numeric_env,
            class_env,
            refinement_facts,
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
            preconditions,
            num_values,
            numeric_env,
            &*refinement_facts,
            class_env,
            rdecl,
            param,
            param_kind.as_ref(),
            fields,
        ),
        RDeclKind::InstanceDecl {
            class_id,
            head_params,
            head_type,
            constraints,
            fields,
        } => elab_instance_decl(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            &*refinement_facts,
            class_env,
            provenance,
            standard_operators,
            rdecl,
            effect_rows,
            &rdecl.name,
            *class_id,
            head_params,
            head_type,
            constraints,
            fields,
        ),
        RDeclKind::DeriveDecl { class_id, data_name, data_id } => elab_derive(
            env,
            globals,
            num_values,
            numeric_env,
            class_env,
            rdecl,
            &rdecl.name,
            *class_id,
            data_name,
            *data_id,
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

/// Reject a predicate erased inside an alias body before that alias is admitted.
/// The outermost refinement is recorded by the alias arm instead.
pub(crate) fn alias_nested_refinement(ty: &RType) -> Option<(&Span, &'static str)> {
    const FUNCTION: &str = "a refinement under a function-valued return type is not supported yet";
    const OTHER: &str = "a refinement nested inside a named type alias is not supported yet";
    fn walk(ty: &RType, in_function: bool) -> Option<(&Span, &'static str)> {
        match ty {
            RType::RRefine(_, _, _, span) => Some((span, if in_function { FUNCTION } else { OTHER })),
            RType::RPi(_, domain, codomain, _)
            | RType::REffectArr(domain, _, codomain, _) =>
                walk(domain, true).or_else(|| walk(codomain, true)),
            RType::RArr(domain, codomain, _) =>
                walk(domain, true).or_else(|| walk(codomain, true)),
            RType::RSigma(_, first, second, _) | RType::RApp(first, second, _) =>
                walk(first, in_function).or_else(|| walk(second, in_function)),
            RType::RTrunc(inner, _) => walk(inner, in_function),
            RType::RUniv(_, _) | RType::RCon(_, _) | RType::RCheckedGlobal { .. }
            | RType::RVarTy(_, _, _) | RType::RPatternAliasTy(_, _, _)
            | RType::RProj(_, _, _) => None,
        }
    }
    match ty {
        RType::RRefine(_, carrier, _, _) => walk(carrier, false),
        other => walk(other, false),
    }
}

/// Elaborate a named-field record declaration to the existing transparent
/// right-nested Sigma encoding and register only its shared projection facts.
#[inline(never)]
fn elab_record_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    facts: &mut RefinementFacts,
    rdecl: &RDecl,
    fields: &[RRecordField],
) -> Result<ElabResult, ElabError> {
    if !fields.iter().any(|field| matches!(field.ty, RType::RRefine(..))) {
        return elab_record_decl_checked(
            env, globals, preconditions, num_values, numeric_env, class_env, facts, rdecl, fields,
        );
    }
    let snapshot = (env.clone(), globals.clone(), num_values.clone(), facts.clone());
    let result = elab_record_decl_checked(
        env, globals, preconditions, num_values, numeric_env, class_env, facts, rdecl, fields,
    );
    if result.is_err() {
        (*env, *globals, *num_values, *facts) = snapshot;
    }
    result
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn elab_record_decl_checked(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &mut ClassEnv,
    facts: &mut RefinementFacts,
    rdecl: &RDecl,
    fields: &[RRecordField],
) -> Result<ElabResult, ElabError> {
    let (field_types, predicates) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, facts, rdecl.name.clone())
            .with_preconditions(preconditions, PremiseHoles::Refused);
        let mut types = Vec::new();
        let mut predicates = Vec::with_capacity(fields.len());
        for field in fields {
            let ty = elab_type_in_slot(&mut cx, &field.ty, RefinementSlot::Outermost)?;
            let ty = cx.metas.zonk_term(&ty);
            let predicate = if let RType::RRefine(_, _, phi, _) = &field.ty {
                cx.ctx.push(ty.clone());
                let checked = elab_prop_at_omega(&mut cx, phi, phi.span());
                cx.ctx.pop();
                Some(Term::lam(ty.clone(), cx.metas.zonk_term(&checked?)))
            } else {
                None
            };
            predicates.push(predicate);
            cx.ctx.push(ty.clone());
            types.push(ty);
        }
        (types, predicates)
    };

    let sigma_chain = build_sigma_chain(&field_types, class_env.record_nil_id);
    let record_sort = kernel_infer_raw(env, &Context::new(), &sigma_chain).map_err(|error| {
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
    if predicates.iter().any(Option::is_some) {
        facts.record_field_predicates.insert(id, predicates);
    }

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
        RType::RCon(s, _)
        | RType::RCheckedGlobal { name: s, .. }
        | RType::RVarTy(_, s, _)
        | RType::RPatternAliasTy(_, s, _) => s.clone(),
        RType::RApp(f, _, _) => head_type_name(f),
        RType::RUniv(_, _) => "Type".to_string(),
        RType::RArr(_, _, _) | RType::REffectArr(_, _, _, _) | RType::RPi(_, _, _, _) => {
            "->".to_string()
        }
        RType::RSigma(_, _, _, _) => "×".to_string(),
        RType::RRefine(_, inner, _, _) => head_type_name(inner),
        RType::RTrunc(_, _) => "‖‖".to_string(),
        // No static head: `d.Query`'s identity is not known until `d`'s
        // dictionary is, which is AFTER instance search rather than before it.
        // The empty name is what `rtype_head_name` already yields for every
        // headless shape, and both feed instance keying -- so a projection
        // matches no instance and fails closed, which is the correct direction
        // for a key that cannot be computed.
        RType::RProj(_, _, _) => String::new(),
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
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
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
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
                .with_preconditions(preconditions, PremiseHoles::Refused);
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
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_preconditions(preconditions, PremiseHoles::Refused);
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
        kernel_infer_raw(env, &ctx_a, &sigma_chain).map_err(|e| ElabError::KernelRejected {
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
    class_id: GlobalId,
    head_name: &str,
    head_core: &Term,
    fields: &[(String, RExpr)],
    constraints: &[RInstanceConstraint],
    effect_rows: &CheckedEffectRows<'_>,
    span: &Span,
) -> Result<(Vec<Term>, Vec<crate::effects::RowType>), ElabError> {
    let (field_names, field_types, field_purities, has_param) = {
        let ci = class_env
            .class_by_id(class_id)
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
    let mut bound_dict_classes: Vec<(String, GlobalId)> = constraints.iter().map(|constraint| {
        let id = checked_class_id(class_env, &constraint.class_name, constraint.class_id, span)
            .expect("prerequisite checked before instance fields");
        (constraint.binder.clone(), id)
    }).collect();
    if constraints.len() == 1 && constraints[0].binder != "d" {
        bound_dict_classes.push(("d".to_string(), bound_dict_classes[0].1));
    }
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
            local_constraints: constraints,
            bound_dict_classes: &bound_dict_classes,
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
                constraints,
                &bound_dict_classes,
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
    effect_rows: &CheckedEffectRows<'_>,
    globals: &HashMap<String, GlobalId>,
    class_env: &ClassEnv,
    constraints: &[RInstanceConstraint],
    bound_dict_classes: &[(String, GlobalId)],
    span: &Span,
) -> Result<(), ElabError> {
    let projection_ctx = ProjectionPurityCtx {
        globals,
        class_env,
        local_constraints: constraints,
        bound_dict_classes,
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

/// Class references selected by imports are never resolved through the
/// mutable current-name index. A local/legacy reference without a selected ID
/// uses that index only after its declaration has been checked.
fn checked_class_id(
    class_env: &ClassEnv,
    name: &str,
    selected: Option<GlobalId>,
    span: &Span,
) -> Result<GlobalId, ElabError> {
    let id = selected.or_else(|| class_env.class(name).map(|view| view.projection.type_id));
    id.filter(|id| class_env.class_by_id(*id).is_some())
        .ok_or_else(|| ElabError::UnresolvedCon {
            name: name.to_string(),
            span: span.clone(),
        })
}

/// Store a checked identity for every fixed constructor in a saved instance
/// pattern; a later unit may reuse its spelling before a search occurs.
fn freeze_instance_pattern(
    ty: &RType,
    globals: &HashMap<String, GlobalId>,
) -> RType {
    match ty {
        RType::RCon(name, span) => match globals.get(name) {
            Some(id) => RType::RCheckedGlobal {
                name: name.clone(), id: *id, span: span.clone(),
            },
            None => ty.clone(),
        },
        RType::RApp(f, a, span) => RType::RApp(
            Box::new(freeze_instance_pattern(f, globals)),
            Box::new(freeze_instance_pattern(a, globals)), span.clone(),
        ),
        RType::RArr(a, b, span) => RType::RArr(
            Box::new(freeze_instance_pattern(a, globals)),
            Box::new(freeze_instance_pattern(b, globals)), span.clone(),
        ),
        RType::REffectArr(a, row, b, span) => RType::REffectArr(
            Box::new(freeze_instance_pattern(a, globals)), row.clone(),
            Box::new(freeze_instance_pattern(b, globals)), span.clone(),
        ),
        RType::RTrunc(inner, span) => RType::RTrunc(
            Box::new(freeze_instance_pattern(inner, globals)), span.clone(),
        ),
        _ => ty.clone(),
    }
}

fn named_head_id(ty: &RType, globals: &HashMap<String, GlobalId>) -> Option<GlobalId> {
    match ty {
        RType::RCheckedGlobal { id, .. } => Some(*id),
        RType::RCon(name, _) => globals.get(name).copied(),
        RType::RApp(f, _, _) | RType::RRefine(_, f, _, _) => named_head_id(f, globals),
        _ => None,
    }
}

fn instance_head_key(ty: &RType, core: &Term) -> InstanceHeadKey {
    match ty {
        RType::RVarTy(_, name, _) => InstanceHeadKey::Parameter(name.clone()),
        RType::RCheckedGlobal { id, .. } => InstanceHeadKey::Global(*id),
        RType::RApp(f, _, _) if matches!(instance_head_key(f, core), InstanceHeadKey::Global(_)) =>
            instance_head_key(f, core),
        _ => match core_type_head_id(core) {
            Some(id) => InstanceHeadKey::Global(id),
            None => InstanceHeadKey::Structural(core.clone()),
        },
    }
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
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    class_env: &mut ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
    effect_rows: &CheckedEffectRows<'_>,
    class_name: &str,
    selected_class_id: Option<GlobalId>,
    head_params: &[String],
    head_type: &RType,
    constraints: &[RInstanceConstraint],
    fields: &[(String, RExpr)],
) -> Result<ElabResult, ElabError> {
    let span = &rdecl.span;
    let class_type_id = checked_class_id(class_env, class_name, selected_class_id, span)?;
    let (class_module, class_kind) = {
        let ci = class_env.class_by_id(class_type_id).expect("selected class was checked");
        (ci.module_id, ci.kind.clone())
    };
    let head_name = head_type_name(head_type);
    // Orphan ownership is checked at the declaration, even for an unbound
    // head; imported heads carry their selected ID independently of globals.
    let in_class_module = class_module == class_env.current_module;
    let in_head_module = named_head_id(head_type, globals)
        .and_then(|id| class_env.global_modules.get(&id))
        .is_some_and(|module| *module == class_env.current_module);
    if !in_class_module && !in_head_module {
        return Err(ElabError::OrphanInstance {
            class: class_name.to_string(),
            head_type: head_name.clone(),
            span: span.clone(),
        });
    }

    // ---- elaborate head type before identity-keyed overlap test ----------
    let head_core = {
        let mut cx = ElabCtx::new(
            env,
            globals,
            num_values,
            numeric_env,
            refinement_facts,
            format!("{class_name}.{head_name}"),
        )
        .with_preconditions(preconditions, PremiseHoles::Refused);
        push_type0_params(&mut cx, head_params.len());
        let h = elab_type(&mut cx, head_type)?;
        cx.metas.zonk_term(&h)
    };
    let instance_key = (class_type_id, instance_head_key(head_type, &head_core));
    if class_kind == ClassKind::Structure {
        if let Some(existing) = class_env.instances_by_id.get(&instance_key) {
            return Err(ElabError::OverlappingInstances {
                class: class_name.to_string(),
                head_type: head_name.clone(),
                first_span: existing.declaration_span.clone(),
                second_span: span.clone(),
            });
        }
    }

    // ---- build instance type --------------------------------------------
    // App(class_type, head) if parameterized, else class_type directly.
    let instance_ty = if class_env
        .class_by_id(class_type_id)
        .is_some_and(|ci| ci.projection.head_param.is_some())
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
            refinement_facts,
            format!("{class_name}.{head_name}"),
        )
        .with_preconditions(preconditions, PremiseHoles::Refused);
        push_type0_params(&mut cx, head_params.len());
        constraints
            .iter()
            .map(|constraint| {
                let id = checked_class_id(class_env, &constraint.class_name, constraint.class_id, span)?;
                let class = class_env.class_by_id(id).expect("selected constraint was checked");
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
        checked_class_id(class_env, &constraint.class_name, constraint.class_id, span)
            .is_ok_and(|id| id == class_type_id)
            && rtypes_match(&constraint.head_type, head_type, globals)
    });

    // ---- admit the instance ----------------------------------------------
    let (instance_id, field_effect_rows, field_obligations) = if has_self_ref {
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
        (ids[0], vec![], Vec::new())
    } else if !constraints.is_empty() {
        // Non-self-ref constrained instance: elaborate fields, then route through
        // declare_recursive_group so sct_check runs on the group (`39 §6.4`).
        // Body has no App(Const(own_id), ...) → edges.is_empty() → sct_check
        // accepts. Mutual/indirect cycles are not detected here (see above).
        let (ordered_vals, field_effect_rows, obligations) = {
            let mut cx = ElabCtx::new(
                env,
                globals,
                num_values,
                numeric_env,
                refinement_facts,
                format!("{class_name}.{head_name}"),
            )
            .with_classes(&*class_env, provenance, standard_operators)
            .with_preconditions(preconditions, PremiseHoles::Reported);
            push_type0_params(&mut cx, head_params.len());
            for (index, constraint_ty) in constraint_core_types.iter().enumerate() {
                cx.ctx.push(weaken(constraint_ty, index as i64));
            }
            let (ordered_vals, field_effect_rows) = compute_ordered_field_values(
                &mut cx,
                class_env,
                class_name,
                class_type_id,
                &head_name,
                &head_core,
                fields,
                constraints,
                effect_rows,
                span,
            )?;
            let obligations = std::mem::take(&mut cx.obligations);
            (ordered_vals, field_effect_rows, obligations)
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
        (ids[0], field_effect_rows, obligations)
    } else {
        // No constraints: declare_def path (no recursion possible, SCT not needed).
        let (ordered_vals, field_effect_rows, obligations) = {
            let mut cx = ElabCtx::new(
                env,
                globals,
                num_values,
                numeric_env,
                refinement_facts,
                format!("{class_name}.{head_name}"),
            )
            .with_classes(&*class_env, provenance, standard_operators)
            .with_preconditions(preconditions, PremiseHoles::Reported);
            push_type0_params(&mut cx, head_params.len());
            let (ordered_vals, field_effect_rows) = compute_ordered_field_values(
                &mut cx,
                class_env,
                class_name,
                class_type_id,
                &head_name,
                &head_core,
                fields,
                constraints,
                effect_rows,
                span,
            )?;
            let obligations = std::mem::take(&mut cx.obligations);
            (ordered_vals, field_effect_rows, obligations)
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
        (id, field_effect_rows, obligations)
    };

    // ---- register instance ----------------------------------------------
    let inst_name = format!("{}_instance_{}", class_name, head_name);
    globals.insert(inst_name, instance_id);
    class_env
        .global_modules
        .insert(instance_id, class_env.current_module);
    // For property classes, allow multiple registrations (Ω-PI means they're
    // all definitionally equal; the key is occupied but we don't error).
    let info = InstanceInfo {
        instance_id,
        class_name: class_name.to_string(),
        class_id: class_type_id,
        field_effect_rows,
        module_id: class_env.current_module,
        head_param_count: head_params.len(),
        head_type: Some(freeze_instance_pattern(head_type, globals)),
        constraints: constraints
            .iter()
            .zip(&constraint_core_types)
            .map(|(constraint, core_type)| InstanceConstraintInfo {
                class_name: constraint.class_name.clone(),
                class_id: checked_class_id(class_env, &constraint.class_name, constraint.class_id, span)
                    .expect("constraint checked before instance admission"),
                head_type: freeze_instance_pattern(&constraint.head_type, globals),
                core_type: core_type.clone(),
            })
            .collect(),
        defining_package: class_env
            .current_package
            .clone()
            .unwrap_or_else(|| "<local>".to_string()),
        declaration_span: span.clone(),
    };
    class_env.instances.insert((class_name.to_string(), head_name), info.clone());
    class_env.instances_by_id.insert(instance_key, info);
    if let Some(package) = &class_env.current_package {
        class_env.source_instance_packages.insert(package.clone());
    }

    let mut obligations = Vec::new();
    absorb_obligations(&mut obligations, field_obligations);
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: instance_id,
        obligations,
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
    selected_class_id: Option<GlobalId>,
    data_name: &str,
    selected_data_id: Option<GlobalId>,
) -> Result<ElabResult, ElabError> {
    let span = &rdecl.span;

    let class_type_id = checked_class_id(class_env, class_name, selected_class_id, span)?;
    let has_param = class_env.class_by_id(class_type_id)
        .expect("selected class was checked").projection.head_param.is_some();

    let data_id = selected_data_id.or_else(|| globals.get(data_name).copied())
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
    let info = InstanceInfo {
        instance_id,
        class_name: class_name.to_string(),
        class_id: class_type_id,
        field_effect_rows: vec![],
        module_id: class_env.current_module,
        head_param_count: 0,
        head_type: None,
        constraints: vec![],
        defining_package: class_env.current_package.clone().unwrap_or_else(|| "<local>".to_string()),
        declaration_span: span.clone(),
    };
    class_env.instances.insert((class_name.to_string(), head_name), info.clone());
    class_env.instances_by_id.insert(
        (class_type_id, InstanceHeadKey::Global(data_id)), info,
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
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    rdecl: &RDecl,
    symbol: &str,
    library: &str,
    is_pure: bool,
    visits: &[String],
) -> Result<ElabResult, ElabError> {
    use crate::foreign::elaborate_foreign;

    let ty_core = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_preconditions(preconditions, PremiseHoles::Refused);
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

/// A malformed literal field is found after kernel admission, because its
/// predicate is surface-only. Rewind that one declaration and all of its
/// constructor/numeric/side-table registrations on failure.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn elaborate_data_with_refinements(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    ctor_decl_spans: &mut HashMap<String, Span>,
    facts: &mut RefinementFacts,
    name: &str,
    type_params: &[String],
    ctors: &[crate::resolve::RCtorDecl],
    span: &Span,
) -> Result<GlobalId, ElabError> {
    if !ctors.iter().any(|ctor| ctor.args.iter().any(|arg| matches!(arg, RType::RRefine(..)))) {
        return data::elab_data_decl(env, globals, ctor_decl_spans, name, type_params, ctors, span);
    }
    let snapshot = (
        env.clone(), globals.clone(), num_values.clone(),
        ctor_decl_spans.clone(), facts.clone(),
    );
    let result = (|| {
        let id = data::elab_data_decl(env, globals, ctor_decl_spans, name, type_params, ctors, span)?;
        register_legacy_constructor_fields(
            env, globals, num_values, numeric_env, facts,
            type_params.len(), ctors, name,
        )?;
        Ok(id)
    })();
    if result.is_err() {
        (*env, *globals, *num_values, *ctor_decl_spans, *facts) = snapshot;
    }
    result
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn elaborate_explicit_data_with_refinements(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    ctor_decl_spans: &mut HashMap<String, Span>,
    facts: &mut RefinementFacts,
    name: &str,
    params: &[crate::resolve::RTelescopeEntry],
    indices: &[crate::resolve::RTelescopeEntry],
    level: Option<u32>,
    ctors: &[crate::resolve::RExplicitCtorDecl],
    span: &Span,
) -> Result<GlobalId, ElabError> {
    if !ctors.iter().any(|ctor| ctor.args.iter().any(|arg| matches!(arg.ty, RType::RRefine(..)))) {
        return data::elab_explicit_data_decl(
            env, globals, ctor_decl_spans, name, params, indices, level, ctors, span,
        );
    }
    let snapshot = (
        env.clone(), globals.clone(), num_values.clone(),
        ctor_decl_spans.clone(), facts.clone(),
    );
    let result = (|| {
        let id = data::elab_explicit_data_decl(
            env, globals, ctor_decl_spans, name, params, indices, level, ctors, span,
        )?;
        register_explicit_constructor_fields(
            env, globals, num_values, numeric_env, facts, params, ctors, name,
        )?;
        Ok(id)
    })();
    if result.is_err() {
        (*env, *globals, *num_values, *ctor_decl_spans, *facts) = snapshot;
    }
    result
}

#[inline(never)]
fn register_legacy_constructor_fields(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    facts: &mut RefinementFacts,
    param_count: usize,
    ctors: &[crate::resolve::RCtorDecl],
    owner: &str,
) -> Result<(), ElabError> {
    for ctor in ctors {
        if !ctor.args.iter().any(|arg| matches!(arg, RType::RRefine(..))) {
            continue;
        }
        let id = globals[&ctor.name];
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, facts, owner.to_string());
        for _ in 0..param_count {
            cx.ctx.push(Term::ty(Level::Zero));
        }
        let args = ctor.args.iter().collect::<Vec<_>>();
        let predicates = collect_constructor_field_predicates(&mut cx, &args)?;
        facts.constructor_field_predicates.insert(id, predicates);
    }
    Ok(())
}

#[inline(never)]
fn register_explicit_constructor_fields(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    facts: &mut RefinementFacts,
    params: &[crate::resolve::RTelescopeEntry],
    ctors: &[crate::resolve::RExplicitCtorDecl],
    owner: &str,
) -> Result<(), ElabError> {
    for ctor in ctors {
        if !ctor.args.iter().any(|arg| matches!(arg.ty, RType::RRefine(..))) {
            continue;
        }
        let id = globals[&ctor.name];
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, facts, owner.to_string());
        for param in params {
            let ty = elab_type(&mut cx, &param.ty)?;
            cx.ctx.push(ty);
        }
        let args = ctor.args.iter().map(|arg| &arg.ty).collect::<Vec<_>>();
        let predicates = collect_constructor_field_predicates(&mut cx, &args)?;
        facts.constructor_field_predicates.insert(id, predicates);
    }
    Ok(())
}

/// Constructor fields are already bare carriers in the kernel declaration.
/// Keep only literal source predicates indexed by the checked constructor and
/// argument position; the same check-time emitter handles their introduction.
#[inline(never)]
fn collect_constructor_field_predicates(
    cx: &mut ElabCtx<'_>,
    args: &[&RType],
) -> Result<Vec<Option<Term>>, ElabError> {
    let mut result = Vec::with_capacity(args.len());
    for arg in args {
        let carrier = elab_type_in_slot(cx, arg, RefinementSlot::Outermost)?;
        let predicate = if let RType::RRefine(_, _, phi, _) = arg {
            cx.ctx.push(carrier.clone());
            let checked = elab_prop_at_omega(cx, phi, phi.span());
            cx.ctx.pop();
            Some(Term::lam(carrier.clone(), cx.metas.zonk_term(&checked?)))
        } else {
            None
        };
        result.push(predicate);
        cx.ctx.push(carrier);
    }
    Ok(result)
}

fn declared_named_return_id(
    rdecl: &RDecl,
    globals: &HashMap<String, GlobalId>,
    facts: &RefinementFacts,
) -> Option<GlobalId> {
    let mut ty = rdecl.ty.as_ref()?;
    for _ in 0..view_param_count(rdecl) {
        ty = match ty {
            RType::RPi(_, _, result, _)
            | RType::RArr(_, result, _)
            | RType::REffectArr(_, _, result, _) => result,
            _ => return None,
        };
    }
    let id = named_head_id(ty, globals)?;
    facts.refinement_root(id)
}

fn elaborate_view_or_let(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &mut HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
    refinement_facts: &mut RefinementFacts,
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    declared_fixity: Option<(Fixity, Span)>,
) -> Result<ElabResult, ElabError> {
    // Check for implicit ensures from a return-type refinement (`22 §2.1`).
    let has_refine_return = rdecl
        .ty
        .as_ref()
        .and_then(|ty| innermost_refine_pred(ty))
        .is_some()
        || declared_named_return_id(rdecl, globals, refinement_facts).is_some();
    if rdecl.requires.is_empty() && rdecl.ensures.is_empty() && !has_refine_return {
        // V0 path: no spec clauses and no return-type refinement
        return elaborate_v0(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            class_env,
            provenance,
            standard_operators,
            rdecl,
            local_dicts,
            refinement_facts,
            fixities,
            fixity_spans,
            declared_fixity,
        );
    }
    // V1 path: has requires/ensures or implicit return-type refinement obligation
    elaborate_view_with_spec(
        env,
        globals,
        preconditions,
        num_values,
        numeric_env,
        class_env,
        provenance,
        standard_operators,
        rdecl,
        local_dicts,
        refinement_facts,
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
    let mut cell_obligations = Vec::new();
    for cell in &space.cells {
        let mut cx = ElabCtx::new(
            &mut elab.env,
            &elab.globals,
            &mut elab.num_values,
            &elab.numeric_env,
            &elab.refinement_facts,
            format!("{}.initial", space.name),
        )
        .with_preconditions(&elab.preconditions, PremiseHoles::Reported);
        let cell_type = elab_type(&mut cx, &cell.ty)?;
        let cell_value = check(&mut cx, &cell.init, &cell_type, &cell.span)?;
        absorb_obligations(&mut cell_obligations, std::mem::take(&mut cx.obligations));
        cell_types.push(cx.metas.zonk_term(&cell_type));
        cell_values.push(cx.metas.zonk_term(&cell_value));
    }

    let state_body = build_space_state_type(&cell_types);
    let state_sort = kernel_infer_raw(&elab.env, &Context::new(), &state_body).map_err(|error| {
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
    absorb_obligations(&mut results[0].obligations, cell_obligations);

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
    let empty_type = Term::indformer(prelude.empty_id, vec![]);
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
        let inferred_row = infer_expr_row_type(
            &operation.body,
            &CheckedEffectRows::new(&elab.effect_rows, &elab.effect_rows_by_id),
            None,
        )
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
            &elab.refinement_facts,
            qualified_name.clone(),
        )
        .with_classes(
            &elab.class_env,
            &mut elab.resolution_provenance,
            &elab.standard_operators,
        )
        .with_preconditions(&elab.preconditions, PremiseHoles::Reported);
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
                kernel_check_raw(
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

                let closed = close_goal(&cx.ctx, &[], goal);
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
        absorb_obligations(&mut contract_obligations, std::mem::take(&mut cx.obligations));
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

/// The only admission route for a written callable signature. Its carrier
/// erasure and literal-parameter facts are produced in one scope: an eligible
/// spine domain cannot be admitted without collecting its predicate.
#[must_use]
fn elab_signature(
    cx: &mut ElabCtx<'_>, ty: &RType, result: bool,
) -> Result<(Term, Vec<Option<Term>>), ElabError> {
    let core = elab_type_in_slot(cx, ty, RefinementSlot::Signature { result })?;
    let core = cx.metas.zonk_term(&core);
    let params = collect_refined_params(cx, ty, &core)?;
    Ok((core, params))
}

fn record_refined_params(facts: &mut RefinementFacts, id: GlobalId, params: Vec<Option<Term>>) {
    if params.iter().any(Option::is_some) {
        facts.refined_params.insert(id, params);
    }
}

/// A source-refined parameter still has a carrier Π domain. Its direct
/// application carries the introduction obligation; callee recognition and
/// higher-order preservation belong to LANG-REFINED-PARAM-REQUIRES-DESUGAR.
#[inline(never)]
fn collect_refined_params(
    cx: &mut ElabCtx<'_>, declared: &RType, core: &Term,
) -> Result<Vec<Option<Term>>, ElabError> {
    let base = cx.ctx.len();
    let result = (|| {
        let mut params = Vec::new();
        // One entry per core Pi binder; only dependent Pi binds in the
        // resolver context. Never re-elaborate a signature domain here.
        let mut bound = Vec::new();
        let (mut current, mut telescope) = (declared, core);
        loop {
            let (domain, codomain, binds) = match current {
                RType::RPi(_, domain, codomain, _) => (domain.as_ref(), codomain.as_ref(), true),
                RType::RArr(domain, codomain, _) | RType::REffectArr(domain, _, codomain, _) =>
                    (domain.as_ref(), codomain.as_ref(), false),
                _ => break,
            };
            let Term::Pi(core_domain, core_codomain) = telescope else {
                return Err(ElabError::Internal(
                    "a signature's source spine and its core telescope disagree".into()));
            };
            let carrier = lower_over_anonymous(core_domain, &bound);
            let predicate = if let RType::RRefine(_, _, phi, _) = domain {
                cx.ctx.push(carrier.clone());
                let checked = elab_prop_at_omega(cx, phi, phi.span());
                cx.ctx.pop();
                Some(lift_over_anonymous(&Term::lam(carrier.clone(), checked?), &bound))
            } else { None };
            params.push(predicate);
            if binds { cx.ctx.push(carrier); }
            bound.push(binds);
            current = codomain;
            telescope = core_codomain;
        }
        Ok(params)
    })();
    cx.ctx.types.truncate(base);
    result
}

/// Re-scope a term written under resolver binders to the core telescope.
/// Each anonymous binder is inserted at its position, counting all inside it.
fn lift_over_anonymous(term: &Term, bound: &[bool]) -> Term {
    let mut lifted = term.clone();
    for (position, &binds) in bound.iter().rev().enumerate() {
        if !binds { lifted = shift(&lifted, 1, position) }
    }
    lifted
}

/// Inverse of the lift on terms that cannot mention anonymous binders.
fn lower_over_anonymous(term: &Term, bound: &[bool]) -> Term {
    let n = bound.len();
    let mut lowered = term.clone();
    for (index, &binds) in bound.iter().enumerate() {
        if !binds { lowered = shift(&lowered, -1, n - 1 - index) }
    }
    lowered
}

fn elaborate_v0(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &mut HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
    refinement_facts: &mut RefinementFacts,
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    declared_fixity: Option<(Fixity, Span)>,
) -> Result<ElabResult, ElabError> {
    // A self-recursive view/let (body mentions its own name) must be admitted
    // through the SCT gate with the name pre-bound, so the body's self-call
    // resolves — `declare_def` allocates the id only after the body is built,
    // which is too late for a self-reference. Route to the recursive path.
    if rexpr_mentions_name(&rdecl.body, &rdecl.name) {
        return elaborate_recursive_view(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            class_env,
            provenance,
            standard_operators,
            refinement_facts,
            fixities,
            fixity_spans,
            declared_fixity,
            rdecl,
        );
    }
    let (ty_core, body_core, body_obligations, params) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_classes(class_env, provenance, standard_operators)

            .with_local_dicts(local_dicts)
            .with_preconditions(preconditions, PremiseHoles::Reported);
        let (body_raw, ty_raw, params) = if let Some(ty) = &rdecl.ty {
            let (ty_c, params) = elab_signature(&mut cx, ty, false)?;
            let body_c = check(&mut cx, &rdecl.body, &ty_c, &rdecl.span)?;
            (body_c, ty_c, params)
        } else {
            let (body_c, ty_c) = infer(&mut cx, &rdecl.body)?;
            (body_c, ty_c, Vec::new())
        };
        let obligations = std::mem::take(&mut cx.obligations);
        (
            cx.metas.zonk_term(&ty_raw),
            cx.metas.zonk_term(&body_raw),
            obligations,
            params.into_iter().map(|p| p.map(|p| cx.metas.zonk_term(&p))).collect::<Vec<_>>(),
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
    record_refined_params(refinement_facts, id, params);
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
/// The kernel's `declare_def` already pre-admits an opaque and calls
/// `admit_bodies` to check and upgrade it — but it allocates the
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
///   4. Call `admit_bodies` on the singleton group: the kernel checks the
///      body, SCT, and paths through other transparent definitions before
///      upgrading it to transparent (δ-unfoldable, leaves `trusted_base`).
///   5. On rejection, roll back the pre-admission — the opaque
///      plus any literal postulates body elaboration added after it — and
///      unbind the name from `globals`.
///
/// **Contained vs deferred (K2c).** This elaborator path uses the checked
/// kernel `admit_bodies` gate; type checking, structural descent, and exclusion
/// of a cycle escaping through another transparent body all live in the kernel. The deferred sibling is **K2c general recursive δ** (`11
/// §4`): arbitrary recursive δ-unfolding in conversion. Here the recursive call
/// is to an *opaque* (δ blocks during checking); only after SCT acceptance does
/// it become transparent, and termination is by structural descent on an
/// inductive sub-term (SCT's `↓`) — not general δ. A recursive fn carrying
/// `requires` clauses (so the full type ≠ the carrier Pi-chain) is a tracked
/// follow-on; L3a's recursive views (`map`/`filter`/`fold`/`zip`/`unfoldUpTo`/
/// `sort`/`insert`) carry none.
// Kernel transaction rollback releases ids and the corresponding elaborator
// bindings together, including literals introduced after staging.
fn forget_rolled_back_decls(
    removed: Vec<Decl>,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
) {
    let ids = removed.iter().map(Decl::id).collect::<HashSet<_>>();
    for id in &ids {
        num_values.remove(id);
    }
    globals.retain(|_, id| !ids.contains(id));
}

fn rollback_elab_admission(
    env: &mut GlobalEnv,
    pending: ken_kernel::PendingAdmission,
    globals: &mut HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
) -> Result<(), ElabError> {
    let removed = ken_kernel::rollback_pending(env, pending)
        .map_err(|error| ElabError::Internal(format!("pending admission rollback: {error}")))?;
    forget_rolled_back_decls(removed, globals, num_values);
    Ok(())
}

fn elaborate_recursive_view(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    refinement_facts: &mut RefinementFacts,
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    declared_fixity: Option<(Fixity, Span)>,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    // 1. Elaborate the declared type (recursive views are annotated).
    let (ty_core, type_obligations, params) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_classes(class_env, provenance, standard_operators)

            .with_preconditions(preconditions, PremiseHoles::Reported);
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal("recursive declaration requires a type annotation".into())
        })?;
        let (ty_c, params) = elab_signature(&mut cx, ty, false)?;
        (
            cx.metas.zonk_term(&ty_c),
            std::mem::take(&mut cx.obligations),
            params.into_iter().map(|p| p.map(|p| cx.metas.zonk_term(&p))).collect(),
        )
    };

    // 2. Stage a checked opaque placeholder so the body can self-reference.
    let pending =
        ken_kernel::stage_placeholders(env, vec![(rdecl.name.clone(), vec![], ty_core.clone())])
            .map_err(|error| ElabError::KernelRejected {
                error,
                span: rdecl.span.clone(),
            })?;
    let id = pending.ids()[0];
    globals.insert(rdecl.name.clone(), id);
    let fixity_inserted =
        match install_fixity_binding(fixities, fixity_spans, id, &rdecl.name, declared_fixity) {
            Ok(inserted) => inserted,
            Err(error) => {
                rollback_elab_admission(env, pending, globals, num_values)?;
                return Err(error);
            }
        };

    // 3. Reassociate once, after predeclaration and before type-directed body
    // elaboration. The existing checker sees ordinary RApp/RBinOp only.
    let associated = match reassociate_rdecl(rdecl, globals, fixities, Some(standard_operators)) {
        Ok(associated) => associated,
        Err(error) => {
            rollback_elab_admission(env, pending, globals, num_values)?;
            if fixity_inserted {
                fixities.remove(&id);
                fixity_spans.remove(&id);
            }
            return Err(error);
        }
    };
    let associated = associated.as_deref().unwrap_or(rdecl);
    // Self-calls in the body must see this exact staged identity's facts.
    record_refined_params(refinement_facts, id, params);
    let body_result = (|| -> Result<(Term, Vec<Obligation>), ElabError> {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_classes(class_env, provenance, standard_operators)

            .with_preconditions(preconditions, PremiseHoles::Reported);
        let body_c = check(&mut cx, &associated.body, &ty_core, &rdecl.span)?;
        let obligations = std::mem::take(&mut cx.obligations);
        Ok((cx.metas.zonk_term(&body_c), obligations))
    })();
    let (body_core, body_obligations) = match body_result {
        Ok(body) => body,
        Err(error) => {
            rollback_elab_admission(env, pending, globals, num_values)?;
            refinement_facts.refined_params.remove(&id);
            if fixity_inserted {
                fixities.remove(&id);
                fixity_spans.remove(&id);
            }
            return Err(error);
        }
    };

    // 4. Check and install the entire singleton group in the kernel.
    let admit_result = ken_kernel::admit_pending(env, pending, vec![body_core]);

    match admit_result {
        Ok(_) => {
            let mut obligations = type_obligations;
            absorb_obligations(&mut obligations, body_obligations);
            Ok(ElabResult {
                name: rdecl.name.clone(),
                def_id: id,
                obligations,
                foreign_binding: None,
                temporal_obligations: vec![],
                effect_row_type: None,
            })
        }
        Err((error, removed)) => {
            forget_rolled_back_decls(removed, globals, num_values);
            refinement_facts.refined_params.remove(&id);
            if fixity_inserted {
                fixities.remove(&id);
                fixity_spans.remove(&id);
            }
            Err(ElabError::KernelRejected {
                error,
                span: rdecl.span.clone(),
            })
        }
    }
}

/// Elaborate a genuinely mutually-recursive group of `view`/`let` decls
/// (VAL2 #3) — `members.len() >= 2`, already confirmed to form one strongly-
/// connected call-graph component (`modules.rs`'s SCC pre-pass). Generalizes
/// `elaborate_recursive_view`'s singleton pattern (pre-admit as `Opaque`,
/// elaborate the body against that name-in-scope, call `admit_bodies`,
/// upgrade-or-rollback) to the whole group at once. The kernel checks SCT
/// on the WHOLE GROUP — no member escapes the termination check
/// (`[[sct-unapplied-self-reference-over-accepts]]`).
///
/// Each member requires an explicit type annotation (mirrors the existing
/// singleton recursive-const rule — a mutual group's forward references need
/// every member's *type* resolvable before any body is elaborated).
pub(crate) fn elaborate_mutual_group(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    fixities: &mut HashMap<GlobalId, Fixity>,
    fixity_spans: &mut HashMap<GlobalId, Span>,
    refinement_facts: &mut RefinementFacts,
    declared_fixities: &[Option<(Fixity, Span)>],
    members: &[RDecl],
) -> Result<Vec<ElabResult>, ElabError> {
    // 1. Elaborate every member's declared type FIRST (the signature
    // pre-pass) — none of these need a sibling's id, only their own params.
    let mut ty_cores: Vec<Term> = Vec::with_capacity(members.len());
    let mut parameter_facts: Vec<Vec<Option<Term>>> = Vec::with_capacity(members.len());
    for rdecl in members {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_preconditions(preconditions, PremiseHoles::Refused);
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal(format!(
                "mutually-recursive '{}' requires a type annotation",
                rdecl.name
            ))
        })?;
        let (ty_c, params) = elab_signature(&mut cx, ty, false)?;
        ty_cores.push(cx.metas.zonk_term(&ty_c));
        parameter_facts.push(params.into_iter().map(|p| p.map(|p| cx.metas.zonk_term(&p))).collect());
    }

    // 2. Pre-admit ALL members as Opaque, binding every name in `globals`
    // BEFORE any body is elaborated — this is what lets a forward/mutual
    // reference to any sibling resolve, exactly as the singleton case
    // pre-admits its own single name.
    let pending = ken_kernel::stage_placeholders(
        env,
        members
            .iter()
            .zip(&ty_cores)
            .map(|(rdecl, ty)| (rdecl.name.clone(), vec![], ty.clone()))
            .collect(),
    )
    .map_err(|error| ElabError::KernelRejected {
        error,
        span: members[0].span.clone(),
    })?;
    let ids = pending.ids().to_vec();
    for (rdecl, id) in members.iter().zip(&ids) {
        globals.insert(rdecl.name.clone(), *id);
    }

    if declared_fixities.len() != members.len() {
        rollback_elab_admission(env, pending, globals, num_values)?;
        return Err(ElabError::Internal(
            "mutual-group fixity metadata length does not match members".into(),
        ));
    }
    let mut inserted_fixity_ids = Vec::new();
    for ((rdecl, id), declared) in members.iter().zip(&ids).zip(declared_fixities) {
        match install_fixity_binding(fixities, fixity_spans, *id, &rdecl.name, declared.clone()) {
            Ok(true) => inserted_fixity_ids.push(*id),
            Ok(false) => {}
            Err(error) => {
                for inserted in &inserted_fixity_ids {
                    fixities.remove(inserted);
                    fixity_spans.remove(inserted);
                }
                rollback_elab_admission(env, pending, globals, num_values)?;
                return Err(error);
            }
        }
    }
    let associated_members = match members
        .iter()
        .map(|member| reassociate_rdecl(member, globals, fixities, Some(standard_operators)))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(members) => members,
        Err(error) => {
            for inserted in &inserted_fixity_ids {
                fixities.remove(inserted);
                fixity_spans.remove(inserted);
            }
            rollback_elab_admission(env, pending, globals, num_values)?;
            return Err(error);
        }
    };
    let members = associated_members
        .iter()
        .zip(members.iter())
        .map(|(associated, original)| associated.as_deref().unwrap_or(original))
        .collect::<Vec<_>>();

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
        rollback_elab_admission(env, pending, globals, num_values)?;
        for inserted in &inserted_fixity_ids {
            fixities.remove(inserted);
            fixity_spans.remove(inserted);
        }
        return Err(e);
    }

    // Every in-group call must see every sibling's exact staged identity.
    for (&id, params) in ids.iter().zip(parameter_facts) {
        record_refined_params(refinement_facts, id, params);
    }

    // 3. Elaborate each body checked against its own type (every sibling
    // name, including self, already resolves via `globals` from step 2).
    let recursive_group = ids.iter().copied().collect::<HashSet<_>>();
    let mut bodies: Vec<Term> = Vec::with_capacity(members.len());
    let mut all_obligations: Vec<Vec<Obligation>> = Vec::with_capacity(members.len());
    let elab_err = (|| -> Result<(), ElabError> {
        for (rdecl, ty_core) in members.iter().zip(&ty_cores) {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
                .with_classes(class_env, provenance, standard_operators)

                .with_preconditions(preconditions, PremiseHoles::Reported)
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
        rollback_elab_admission(env, pending, globals, num_values)?;
        for id in &ids { refinement_facts.refined_params.remove(id); }
        for inserted in &inserted_fixity_ids {
            fixities.remove(inserted);
            fixity_spans.remove(inserted);
        }
        return Err(e);
    }

    // 4. Admit the WHOLE GROUP in one kernel call. Each member remains
    // opaque during checking and SCT, and escape paths are checked before
    // any member is upgraded. A per-member call would wrongly reject an
    // honest mutual cycle through already-transparent siblings.
    let admit_result = ken_kernel::admit_pending(env, pending, bodies);

    match admit_result {
        Ok(_) => Ok(members
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
            .collect()),
        Err((error, removed)) => {
            forget_rolled_back_decls(removed, globals, num_values);
            for id in &ids { refinement_facts.refined_params.remove(id); }
            for inserted in &inserted_fixity_ids {
                fixities.remove(inserted);
                fixity_spans.remove(inserted);
            }
            Err(ElabError::KernelRejected {
                error,
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
        RExpr::RCheckedGlobal { .. } => false,
        RExpr::RVar(_, _, _)
        | RExpr::RPatternAlias(_, _, _)
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
        RExpr::RStandardOp { lhs, rhs, .. } => {
            rexpr_mentions_name(lhs, name) || rexpr_mentions_name(rhs, name)
        }
        RExpr::RBinOp(_, l, r, _) => rexpr_mentions_name(l, name) || rexpr_mentions_name(r, name),
        RExpr::RInfixSpine {
            operands,
            operators,
            ..
        } => {
            operands
                .iter()
                .any(|operand| rexpr_mentions_name(operand, name))
                || operators.iter().any(
                    |operator| matches!(operator, RInfixOperator::User(operator_name, _) if operator_name == name),
                )
        }
        RExpr::RMatch { scrut, arms, .. } => {
            rexpr_mentions_name(scrut, name)
                || arms.iter().any(|arm| {
                    arm.guard
                        .as_ref()
                        .is_some_and(|guard| rexpr_mentions_name(guard, name))
                        || rexpr_mentions_name(&arm.body, name)
                })
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
        RType::RCheckedGlobal { .. } => false,
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
        RType::RTrunc(inner, _) => rtype_mentions_name(inner, name),
        // MUST recurse into the base, exactly as `RRefine` does into its
        // predicate. This walk decides declaration dependency order, and the
        // projected object is an expression that can name a top-level binding
        // -- missing it would mis-order an SCC and the failure would surface
        // far from here as an unresolved name. The FIELD name is deliberately
        // not consulted: a class field is not a top-level binding.
        RType::RProj(base, _, _) => rexpr_mentions_name(base, name),
        RType::RUniv(_, _) | RType::RVarTy(_, _, _) | RType::RPatternAliasTy(_, _, _) => false,
    }
}

/// Elaborate all postconditions before checking a declared body. Each
/// predicate remains in the parameter, requirement, result telescope until
/// one result-position leaf consumes it (`22 §2.2`).
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn prepare_contract_ensures(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
    rdecl: &RDecl,
    ensures: &[&RExpr],
    param_types: &[Term],
    requires: &[Term],
    result_ty: &Term,
    recursive_id: Option<GlobalId>,
) -> Result<(Vec<ResultPredicate>, Vec<Obligation>), ElabError> {
    let mut param_ctx = Context::new();
    for param in param_types { param_ctx.push(param.clone()); }
    let mut ens_ctx = param_ctx.clone();
    for requirement in requires { ens_ctx.push(requirement.clone()); }
    let install_depth = ens_ctx.len();
    ens_ctx.push(result_ty.clone());
    let mut predicates = Vec::new();
    let mut obligations = Vec::new();
    for ens in ensures {
        let (psi, nested_obligations) = elab_in_ctx_at_omega(
            env, globals, num_values, numeric_env, refinement_facts, class_env, provenance,
            standard_operators, preconditions, local_dicts, &ens_ctx,
            requires, param_types.len(), ens, &rdecl.span, &rdecl.name,
        )?;
        absorb_obligations(&mut obligations, nested_obligations);
        predicates.push(ResultPredicate {
            predicate: Term::lam(result_ty.clone(), psi.clone()),
            install_depth,
            kind: ObligationKind::Ensures,
            recursive_self: recursive_id.map(|id| RecursiveSelf {
                id, params: param_types.len(), requires: requires.len(), psi,
            }),
        });
    }
    Ok((predicates, obligations))
}

/// Elaborate a `view` with `requires`/`ensures` clauses (`21 §6.3`).
fn elaborate_view_with_spec(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &mut HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
    refinement_facts: &mut RefinementFacts,
) -> Result<ElabResult, ElabError> {
    let mut pending: Option<ken_kernel::PendingAdmission> = None;
    let mut precondition_entry = None;
    let mut recorded_pending = None;
    let result = (|| -> Result<ElabResult, ElabError> {
        let is_recursive = rexpr_mentions_name(&rdecl.body, &rdecl.name);
        // Annotated contracts stage before checking their body so recursive
        // calls see the full type and all body holes share the admission rollback.
        let param_count = view_param_count(rdecl);
        let mut all_ensures: Vec<&RExpr> = rdecl.ensures.iter().collect();
        let literal_return = rdecl.ty.as_ref().and_then(innermost_refine_pred).is_some();
        if let Some(phi) = rdecl.ty.as_ref().and_then(|ty| innermost_refine_pred(ty)) {
            if rdecl.ty.as_ref().and_then(refine_return_depth) != Some(param_count) {
                return Err(ElabError::TypeMismatch {
                    span: rdecl.span.clone(),
                    reason: "a refinement under a function-valued return type is not supported yet"
                        .into(),
                });
            }
            all_ensures.push(phi);
        }
        let mut decl_obligations = Vec::new();
        let (
            full_body,
            _body_inner,
            _param_types,
            result_ty_under_requires,
            full_ty,
            pre_admit_id,
            req_cores,
        ) = if is_recursive || rdecl.ty.is_some() {
            let (carrier_ty, params, assumptions, req_cores) = {
                let mut cx =
                    ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
                        .with_classes(class_env, provenance, standard_operators)

                        .with_local_dicts(local_dicts)
                        .with_preconditions(preconditions, PremiseHoles::Reported);
                let ty = rdecl.ty.as_ref().ok_or_else(|| {
                    ElabError::Internal(
                        "recursive declaration with spec clauses requires a type annotation".into(),
                    )
                })?;
                let (carrier_ty, params) = elab_signature(&mut cx, ty, literal_return)?;
                let req_cores = install_requires_assumptions(
                    &mut cx,
                    &carrier_ty,
                    param_count,
                    &rdecl.requires,
                )?;
                let carrier_ty = cx.metas.zonk_term(&carrier_ty);
                let req_cores = req_cores
                    .iter()
                    .map(|requirement| cx.metas.zonk_term(requirement))
                    .collect::<Vec<_>>();
                let assumptions = cx
                    .assumptions
                    .iter()
                    .map(|assumption| Assumption {
                        prop: cx.metas.zonk_term(&assumption.prop),
                        depth: assumption.depth,
                    })
                    .collect::<Vec<_>>();
                let params = params.into_iter().map(|p| p.map(|p| cx.metas.zonk_term(&p))).collect();
                absorb_obligations(&mut decl_obligations, std::mem::take(&mut cx.obligations));
                (carrier_ty, params, assumptions, req_cores)
            };
            let (param_types, carrier_result, full_ty) =
                build_contract_type(&carrier_ty, param_count, &req_cores)?;
            let result_ty_under_requires = weaken(&carrier_result, req_cores.len() as i64);
            let staged = ken_kernel::stage_placeholders(
                env,
                vec![(rdecl.name.clone(), vec![], full_ty.clone())],
            )
            .map_err(|error| ElabError::KernelRejected {
                error,
                span: rdecl.span.clone(),
            })?;
            let id = staged.ids()[0];
            if !req_cores.is_empty() {
                preconditions.insert(id, (param_count, req_cores.len()));
                precondition_entry = Some(id);
            }
            pending = Some(staged);
            globals.insert(rdecl.name.clone(), id);
            // The checked signature and facts share this staged identity;
            // recursive calls in ensures or the body already see the params.
            record_refined_params(refinement_facts, id, params);
            recorded_pending = Some(id);
            let (predicates, psi_obligations) = prepare_contract_ensures(
                env, globals, num_values, numeric_env, &*refinement_facts, class_env, provenance,
                standard_operators, preconditions, local_dicts, rdecl,
                &all_ensures, &param_types, &req_cores, &result_ty_under_requires,
                is_recursive.then_some(id),
            )?;
            absorb_obligations(&mut decl_obligations, psi_obligations);
            let (full_body, body_inner) = {
                let mut cx =
                    ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
                        .with_classes(class_env, provenance, standard_operators)

                        .with_local_dicts(local_dicts)
                        .with_preconditions(preconditions, PremiseHoles::Reported);
                cx.assumptions = assumptions;
                let (full_body, body_inner) = check_contract_body(
                    &mut cx,
                    &rdecl.body,
                    &carrier_ty,
                    param_count,
                    &req_cores,
                    &rdecl.span,
                    &predicates,
                )?;
                absorb_obligations(&mut decl_obligations, std::mem::take(&mut cx.obligations));
                (
                    cx.metas.zonk_term(&full_body),
                    cx.metas.zonk_term(&body_inner),
                )
            };
            (
                full_body,
                body_inner,
                param_types,
                result_ty_under_requires,
                full_ty,
                Some(id),
                req_cores,
            )
        } else {
            let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
                .with_classes(class_env, provenance, standard_operators)

                .with_local_dicts(local_dicts)
                .with_preconditions(preconditions, PremiseHoles::Reported);
            let (full_body, body_inner, param_types, result_ty_under_requires, full_ty, req_cores) =
                if let Some(ty) = &rdecl.ty {
                    let carrier_ty = elab_type(&mut cx, ty)?;
                    let req_cores = install_requires_assumptions(
                        &mut cx,
                        &carrier_ty,
                        param_count,
                        &rdecl.requires,
                    )?;
                    let (param_types, carrier_result, full_ty) =
                        build_contract_type(&carrier_ty, param_count, &req_cores)?;
                    let result_ty_under_requires = weaken(&carrier_result, req_cores.len() as i64);
                    let (full_body, body_inner) = check_contract_body(
                        &mut cx,
                        &rdecl.body,
                        &carrier_ty,
                        param_count,
                        &req_cores,
                        &rdecl.span,
                        &[],
                    )?;
                    (
                        full_body,
                        body_inner,
                        param_types,
                        result_ty_under_requires,
                        full_ty,
                        req_cores,
                    )
                } else {
                    let req_cores = install_requires_assumptions(
                        &mut cx,
                        &Term::ty(Level::Zero),
                        0,
                        &rdecl.requires,
                    )?;
                    let context_base = cx.ctx.len();
                    let hidden_base = cx.hidden_positions.len();
                    for requirement in &req_cores {
                        let position = cx.ctx.len();
                        cx.ctx.push(requirement.clone());
                        cx.hidden_positions.push(position);
                    }
                    let inferred = infer(&mut cx, &rdecl.body);
                    let (body_inner, result_ty_under_requires) = match inferred {
                        Ok((body, result_ty)) => {
                            (cx.metas.zonk_term(&body), cx.metas.zonk_term(&result_ty))
                        }
                        Err(error) => {
                            while cx.ctx.len() > context_base {
                                cx.ctx.pop();
                            }
                            cx.hidden_positions.truncate(hidden_base);
                            return Err(error);
                        }
                    };
                    while cx.ctx.len() > context_base {
                        cx.ctx.pop();
                    }
                    cx.hidden_positions.truncate(hidden_base);
                    let mut full_body = body_inner.clone();
                    let mut full_ty = result_ty_under_requires.clone();
                    for requirement in req_cores.iter().rev() {
                        full_body = Term::lam(requirement.clone(), full_body);
                        full_ty = Term::pi(requirement.clone(), full_ty);
                    }
                    (
                        full_body,
                        body_inner,
                        Vec::new(),
                        result_ty_under_requires,
                        full_ty,
                        req_cores,
                    )
                };


            absorb_obligations(&mut decl_obligations, std::mem::take(&mut cx.obligations));
            (
                cx.metas.zonk_term(&full_body),
                cx.metas.zonk_term(&body_inner),
                param_types,
                cx.metas.zonk_term(&result_ty_under_requires),
                cx.metas.zonk_term(&full_ty),
                None,
                req_cores
                    .iter()
                    .map(|requirement| cx.metas.zonk_term(requirement))
                    .collect(),
            )
        };

        // A type-inferred declaration has no expected carrier until after its
        // body is inferred. Keep its existing straight-line fallback separate
        // from the checked contract path; no annotated contract uses it.
        if pre_admit_id.is_none() && !all_ensures.is_empty() {
            let mut ens_ctx = Context::new();
            for requirement in &req_cores { ens_ctx.push(requirement.clone()); }
            let mut result_ctx = ens_ctx.clone();
            result_ctx.push(result_ty_under_requires.clone());
            for ens in &all_ensures {
                let (psi, nested) = elab_in_ctx_at_omega(
                    env, globals, num_values, numeric_env, &*refinement_facts, class_env, provenance,
                    standard_operators, preconditions, local_dicts, &result_ctx,
                    &req_cores, 0, ens, &rdecl.span, &rdecl.name,
                )?;
                absorb_obligations(&mut decl_obligations, nested);
                let value = if matches!(
                    whnf(env, &ens_ctx, &result_ty_under_requires), Term::Pi(..)
                ) {
                    Term::Ascript(
                        Box::new(_body_inner.clone()),
                        Box::new(result_ty_under_requires.clone()),
                    )
                } else { _body_inner.clone() };
                let closed = close_goal(&ens_ctx, &[], subst0(&psi, &value));
                let hole_id = declare_postulate(env, rdecl.name.clone(), vec![], closed.clone())
                    .map_err(|error| ElabError::KernelRejected {
                        error, span: rdecl.span.clone(),
                    })?;
                decl_obligations.push(Obligation {
                    id: decl_obligations.len() as u32, hole_id, goal_closed: closed,
                    span: rdecl.span.clone(), kind: ObligationKind::Ensures,
                });
            }
        }

        // Phase 4's full contract type and body were built before staging.
        let id = if let Some(pre_id) = pre_admit_id {
            // The opaque was staged at the complete contract type, so recursive
            // calls and final admission see the same premise telescope.
            let staged = pending
                .take()
                .expect("recursive view staged its placeholder");
            match ken_kernel::admit_pending(env, staged, vec![full_body]) {
                Ok(_) => pre_id,
                Err((error, removed)) => {
                    forget_rolled_back_decls(removed, globals, num_values);
                    preconditions.remove(&pre_id);
                    precondition_entry = None;
                    return Err(ElabError::KernelRejected {
                        error,
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
            if !req_cores.is_empty() {
                preconditions.insert(id, (param_count, req_cores.len()));
            }
            id
        };
        Ok(ElabResult {
            name: rdecl.name.clone(),
            def_id: id,
            obligations: decl_obligations,
            foreign_binding: None,
            temporal_obligations: vec![],
            effect_row_type: None,
        })
    })();
    if result.is_err() {
        if let Some(staged) = pending {
            rollback_elab_admission(env, staged, globals, num_values)?;
        }
        if let Some(id) = recorded_pending {
            refinement_facts.refined_params.remove(&id);
        }
        if let Some(id) = precondition_entry {
            preconditions.remove(&id);
        }
    }
    result
}

/// Elaborate `prove name : φ` (`21 §6.3`, §3).
///
/// Declares `name` as a postulate of `φ`, emitting one obligation hole.
fn elaborate_prove(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    rdecl: &RDecl,
) -> Result<ElabResult, ElabError> {
    let (phi_core, mut obligations) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_preconditions(preconditions, PremiseHoles::Reported);
        let omega = Term::omega(Level::Zero);
        let (phi_raw, phi_ty_raw) = infer(&mut cx, &rdecl.body)?;
        // Check φ is Ω-typed
        unify_types(&mut cx.metas, &omega, &phi_ty_raw);
        (
            cx.metas.zonk_term(&phi_raw),
            std::mem::take(&mut cx.obligations),
        )
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
    obligations.push(Obligation {
        id: obligations.len() as u32,
        hole_id,
        goal_closed: phi_core,
        span: rdecl.span.clone(),
        kind: ObligationKind::Prove,
    });
    Ok(ElabResult {
        name: rdecl.name.clone(),
        def_id: hole_id,
        obligations,
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    })
}

fn elaborate_prop_decl(
    env: &mut GlobalEnv,
    globals: &mut HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &mut RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
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

    let (ty_core, body_core, params) = {
        // Carries the class env for the same reason the `fn`/`const` pre-pass
        // does: a `prop`'s telescope may be typed by a projection, and the
        // name-to-index lookup that resolves it is a `ClassEnv` fact.
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_classes(class_env, provenance, standard_operators)
            .with_preconditions(preconditions, PremiseHoles::Refused);
        let (ty, params) = elab_signature(&mut cx, prop_ty, false)?;
        let ty = cx.metas.zonk_term(&ty);
        let params = params.into_iter().map(|p| p.map(|p| cx.metas.zonk_term(&p))).collect();
        let body = top_body_for_prop_type(env, &ty, &rdecl.span)?;
        (ty, body, params)
    };

    let id = declare_def(env, vec![], ty_core.clone(), body_core).map_err(|e| {
        ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        }
    })?;
    globals.insert(rdecl.name.clone(), id);
    record_refined_params(refinement_facts, id, params);

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
            contains_infix_spine: false,
            kind: RDeclKind::Theorem,
        };
        let helper = elaborate_checked_theorem(
            env,
            globals,
            preconditions,
            num_values,
            numeric_env,
            refinement_facts,
            &ClassEnv::sentinel(),
            // A local sink on a sentinel path, for the same reason as
            // `elaborate_rdecl`'s: provenance lived in the throwaway registry
            // here and was dropped with it.
            &mut Vec::new(),
            // Sentinel path -- see `elaborate_rdecl_v1`'s call above.
            &HashMap::new(),
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
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &mut RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    rdecl: &RDecl,
    attached_subject: Option<&str>,
) -> Result<ElabResult, ElabError> {
    if globals.contains_key(&rdecl.name) {
        return Err(ElabError::TypeMismatch {
            span: rdecl.span.clone(),
            reason: format!("duplicate proof name '{}'", rdecl.name),
        });
    }

    let (ty_core, body_core, body_obligations, params) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, rdecl.name.clone())
            .with_classes(class_env, provenance, standard_operators)
            .with_preconditions(preconditions, PremiseHoles::Reported);
        let ty = rdecl.ty.as_ref().ok_or_else(|| {
            ElabError::Internal(format!("checked theorem '{}' has no type", rdecl.name))
        })?;
        let (ty_core, params) = elab_signature(&mut cx, ty, false)?;
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
            params.into_iter().map(|p| p.map(|p| cx.metas.zonk_term(&p))).collect(),
        )
    };
    let id =
        declare_def(env, vec![], ty_core, body_core).map_err(|e| ElabError::KernelRejected {
            error: e,
            span: rdecl.span.clone(),
        })?;
    globals.insert(rdecl.name.clone(), id);
    record_refined_params(refinement_facts, id, params);
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
    let sort = kernel_infer_raw(env, ctx, ty).map_err(|e| ElabError::KernelRejected {
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
    let sort = kernel_infer_raw(env, ctx, ty).map_err(|e| ElabError::KernelRejected {
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
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    rdecl: &RDecl,
    _param: String,
    fields: Vec<(String, RExpr)>,
) -> Result<ElabResult, ElabError> {
    let omega = Term::omega(Level::Zero);
    let mut obligations: Vec<Obligation> = Vec::new();

    // The param is pre-declared by the resolver; for each field φ, check at Ω
    // and emit an obligation hole.
    for (field_name, field_phi) in fields {
        let (phi_core, callsite_obligations) = {
            let mut cx = ElabCtx::new(
                env,
                globals,
                num_values,
                numeric_env,
                refinement_facts,
                format!("{}.{}", rdecl.name, field_name),
            )
            .with_preconditions(preconditions, PremiseHoles::Reported);
            // param is the law's `param` argument — it's in scope (resolver pushed it)
            // For elaboration, we need the param in scope. Since the resolver resolved
            // field_phi with param in scope at Var(0), we replicate that:
            // Note: we DON'T have a declared type for the param here. For V1, the param
            // is just a term variable whose type must be inferrable from the field props.
            // For test cases, params will always be globally declared.
            let (phi_raw, phi_ty_raw) = infer(&mut cx, &field_phi)?;
            unify_types(&mut cx.metas, &omega, &phi_ty_raw);
            (
                cx.metas.zonk_term(&phi_raw),
                std::mem::take(&mut cx.obligations),
            )
        };
        absorb_obligations(&mut obligations, callsite_obligations);
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
            id: obligations.len() as u32,
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
    refinement_facts: &RefinementFacts,
    class_env: &ClassEnv,
    provenance: &mut Vec<crate::classes::InstanceResolution>,
    standard_operators: &HashMap<StandardOperatorRole, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    local_dicts: &HashMap<String, (Term, Term, usize)>,
    ctx: &Context,
    requires: &[Term],
    parameter_depth: usize,
    expr: &RExpr,
    span: &Span,
    owner_label: &str,
) -> Result<(Term, Vec<Obligation>), ElabError> {
    let mut cx = ElabCtx::new(
        env,
        globals,
        num_values,
        numeric_env,
        refinement_facts,
        owner_label.to_string(),
    )
    .with_classes(class_env, provenance, standard_operators)
    .with_local_dicts(local_dicts)
    .with_preconditions(preconditions, PremiseHoles::Reported);
    // Populate cx.ctx from the snapshot. The requires binders are real entries
    // in this context; only their source-level visibility is hidden.
    for ty in &ctx.types {
        cx.ctx.push(ty.clone());
    }
    for (index, prop) in requires.iter().enumerate() {
        let depth = parameter_depth + index;
        cx.hidden_positions.push(depth);
        cx.assumptions.push(Assumption {
            prop: prop.clone(),
            depth,
        });
    }
    let core = elab_prop_at_omega(&mut cx, expr, span)?;
    Ok((core, std::mem::take(&mut cx.obligations)))
}

/// Split a declaration type after exactly `n` parameter binders: the
/// parameter domains and the declared return type, which may itself be a Pi.
fn split_params(ty: &Term, n: usize) -> Option<(Vec<Term>, Term)> {
    let mut domains = Vec::with_capacity(n);
    let mut current = ty;
    for _ in 0..n {
        let Term::Pi(domain, codomain) = current else {
            return None;
        };
        domains.push((**domain).clone());
        current = codomain;
    }
    Some((domains, current.clone()))
}

/// Number of arrows `innermost_refine_pred` crosses to reach its refinement.
fn refine_return_depth(ty: &RType) -> Option<usize> {
    match ty {
        RType::RPi(_, _, codomain, _)
        | RType::RArr(_, codomain, _)
        | RType::REffectArr(_, _, codomain, _) => {
            refine_return_depth(codomain).map(|depth| depth + 1)
        }
        RType::RRefine(..) => Some(0),
        _ => None,
    }
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

// ----- match elaboration -----

#[inline(never)]
fn infer_active_pattern_alias(
    cx: &mut ElabCtx,
    slot: usize,
    name: &str,
) -> Result<(Term, Term), ElabError> {
    let alias = cx
        .active_pattern_aliases
        .iter()
        .rev()
        .flat_map(|region| region.iter().rev())
        .find(|alias| alias.slot == slot && alias.name == name)
        .cloned()
        .ok_or_else(|| {
            ElabError::Internal(format!(
                "as-pattern alias '{}' has no active matrix-leaf occurrence",
                name
            ))
        })?;
    materialize_pattern_alias(cx, alias, name)
}

fn materialize_pattern_alias(
    cx: &ElabCtx<'_>,
    alias: ActivePatternAlias,
    name: &str,
) -> Result<(Term, Term), ElabError> {
    let growth = cx.ctx.len().checked_sub(alias.install_depth).ok_or_else(|| {
        ElabError::Internal(format!(
            "as-pattern alias '{}' escaped its installation context",
            name
        ))
    })?;
    let occurrence_growth = cx.ctx.len().checked_sub(alias.occurrence_depth)
        .ok_or_else(|| ElabError::Internal(format!(
            "as-pattern alias '{name}' escaped its occurrence context"
        )))?;
    let term = weaken(&alias.occurrence, occurrence_growth as i64);
    Ok((term, weaken(&alias.ty, growth as i64)))
}

/// Only a source index at a matrix split can take this route. Ordinary
/// context binders keep their positional resolution and refinements.
fn infer_virtual_pattern_alias(
    cx: &mut ElabCtx<'_>,
    index: usize,
    name: &str,
    span: &Span,
) -> Result<Option<(Term, Term)>, ElabError> {
    let Some(slot) = cx.virtual_surface_binding_slot(index) else {
        return Ok(None);
    };
    let alias = cx
        .active_pattern_aliases
        .iter()
        .rev()
        .flat_map(|region| region.iter().rev())
        .find(|alias| alias.virtual_slot == Some(slot) && alias.name == name)
        .cloned()
        .ok_or_else(|| ElabError::TypeMismatch {
            span: span.clone(),
            reason: format!(
                "split-column binder '{name}' is bound by a variable row while another field \
                 of the same constructor is also split; bind it by its constructor pattern \
                 in each arm or split it in a separate match"
            ),
        })?;
    materialize_pattern_alias(cx, alias, name).map(Some)
}

#[inline(never)]
fn finish_pattern_alias_frame(
    cx: &mut ElabCtx,
    raw_methods_result: Result<Vec<Option<Term>>, ElabError>,
) -> Result<Vec<Option<Term>>, ElabError> {
    let type_frame = cx.pattern_alias_type_frames.pop()
        .expect("infer_match alias-type frame must balance");
    if let Some(mismatch) = type_frame.type_mismatch {
        return Err(or_binder_type_error(mismatch));
    }
    raw_methods_result
}

#[inline(never)]
fn finish_pattern_alias_term_frame(
    cx: &mut ElabCtx,
    body_result: Result<Term, ElabError>,
) -> Result<Term, ElabError> {
    let type_frame = cx.pattern_alias_type_frames.pop()
        .expect("infer_match alias-type frame must balance");
    if let Some(mismatch) = type_frame.type_mismatch {
        return Err(or_binder_type_error(mismatch));
    }
    body_result
}

/// Elaborate `match scrut { C₁ x₁… => body₁ ; … }` (`34 §3`).
///
/// Compiles to `Term::Elim` with one method per constructor in declaration order.
/// Constant-motive variant: return type inferred from the first arm, checked
/// consistent across all arms by kernel type-checking the Elim.
struct MatrixEntry {
    root_frame_depth: usize,
    outer_ctx_len: usize,
    discovery: bool,
    rerun: bool,
    reused_leaf: bool,
    skipped_ih: Vec<usize>,
    first_leaf: Option<MatrixFirstLeaf>,
    literal_requests: HashMap<(usize, usize), usize>,
    literal_plans: HashMap<(usize, usize, usize), MatrixLiteralPlan>,
}

struct MatrixFirstLeaf {
    body: Term,
    result: Term,
    arm_idx: usize,
    context: Context,
    skipped_ih: Vec<usize>,
}

#[derive(Clone)]
struct MatrixLiteralPlan {
    literal: LiteralPat,
    column_type: Term,
    value: LiteralComparatorValue,
    plan: LiteralComparatorPlan,
}

impl MatrixEntry {
    fn new(root_frame_depth: usize, outer_ctx_len: usize) -> Self {
        Self {
            root_frame_depth,
            outer_ctx_len,
            discovery: false,
            rerun: false,
            reused_leaf: false,
            skipped_ih: Vec::new(),
            first_leaf: None,
            literal_requests: HashMap::new(),
            literal_plans: HashMap::new(),
        }
    }
}

struct IndexedMatchRootFrame {
    outer: Context,
    ind: InductiveDecl,
    family: GlobalId,
    level_args: Vec<Level>,
    params: Vec<Term>,
    scrut_indices: Vec<Term>,
    motive: Option<Box<Term>>,
}

/// A pending column in the pattern-matrix compiler (`34-data-match.md §3.1`):
/// either a genuine surface column (tracked per-row in `RowState::real_pats`)
/// or a synthetic induction-hypothesis slot the eliminator's method type
/// requires but no surface pattern ever names.
/// An IH column's domain comes from the owning eliminator's `method_type`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ColKind {
    Real,
    Ih,
}

/// The core occurrence of one pending pattern-matrix column.
///
/// A live occurrence is valid at the row's current emitted-core depth. A
/// pending occurrence is the `Var(0)` that will denote a constructor field
/// once that column's existing method binder is entered. Keeping this state
/// explicit prevents a future sibling field from being weakened before its
/// own binder exists, while every already-live occurrence is weakened under
/// each intervening real or IH binder.
#[derive(Clone, Debug, PartialEq, Eq)]
struct MatrixOccurrence {
    term: Term,
    live: bool,
    source_binding: bool,
    origin: MatchBinderOrigin,
    /// Whether an emitted core binder occupies a surface de Bruijn position.
    /// Record projection columns are continuation binders, not lexical ones.
    surface_binder: bool,
}

impl MatrixOccurrence {
    fn live(term: Term) -> Self {
        Self {
            term,
            live: true,
            source_binding: true,
            origin: MatchBinderOrigin::Scrutinee,
            surface_binder: true,
        }
    }

    fn pending_field(source_binding: bool, surface_binder: bool, origin: MatchBinderOrigin) -> Self {
        Self {
            term: Term::var(0),
            live: false,
            source_binding,
            origin,
            surface_binder,
        }
    }
}

#[cfg(test)]
#[derive(Default)]
struct MatchOccurrenceTrace {
    seeds: Vec<Term>,
    leaves: Vec<Vec<Option<Term>>>,
    nested_return_types: Vec<bool>,
}

#[cfg(test)]
thread_local! {
    static MATCH_OCCURRENCE_TRACE: std::cell::RefCell<Option<MatchOccurrenceTrace>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
fn begin_match_occurrence_trace() {
    MATCH_OCCURRENCE_TRACE.with(|trace| {
        *trace.borrow_mut() = Some(MatchOccurrenceTrace::default());
    });
}

#[cfg(test)]
fn take_match_occurrence_trace() -> MatchOccurrenceTrace {
    MATCH_OCCURRENCE_TRACE.with(|trace| trace.borrow_mut().take().unwrap_or_default())
}

/// One row of the pattern matrix: the still-unconsumed `Real` column
/// patterns and their aligned core occurrences, the occurrences already
/// supplied to source binding positions, plus the originating top-level arm
/// (for reachability bookkeeping across wildcard-row expansion, `§4.2`).
#[derive(Clone)]
struct RowState {
    real_pats: Vec<RPattern>,
    real_occurrences: Vec<MatrixOccurrence>,
    binding_occurrences: Vec<Option<Term>>,
    /// Source slots for split columns missing from `cx.ctx`, per arm. A
    /// constructor pattern does not create one; a wildcard/variable does.
    virtual_surface_positions: Vec<usize>,
    /// Split-column variables have a whole-value matrix occurrence, not a
    /// context binder. These entries align with the virtual slot above.
    virtual_aliases: Vec<MatrixVirtualAlias>,
    /// Emitted flat columns visible to another row but not bound by this
    /// row's source pattern. Installed only while elaborating this leaf.
    row_hidden_surface_positions: Vec<usize>,
    arm_idx: usize,
}

impl RowState {
    fn assert_occurrence_alignment(&self) {
        debug_assert_eq!(self.real_pats.len(), self.real_occurrences.len());
    }

    /// Enter one emitted-core binder. Occurrences that already denote live
    /// values move under it; future constructor-field binders do not exist yet
    /// and therefore remain pending at `Var(0)`.
    fn under_core_binder(mut self) -> Self {
        self.assert_occurrence_alignment();
        for occurrence in &mut self.real_occurrences {
            if occurrence.live {
                occurrence.term = weaken(&occurrence.term, 1);
            }
        }
        for occurrence in self.binding_occurrences.iter_mut().flatten() {
            *occurrence = weaken(occurrence, 1);
        }
        for alias in &mut self.virtual_aliases {
            alias.occurrence = weaken(&alias.occurrence, 1);
        }
        self
    }

    fn mark_current_real(mut self) -> Self {
        let current = self.real_occurrences.first_mut()
            .expect("a real matrix column has an aligned occurrence");
        current.term = Term::var(0);
        current.live = true;
        self
    }

    /// Enter an existing method binder for this real column.
    fn enter_current_real_binder(self) -> Self {
        self.under_core_binder().mark_current_real()
    }

    /// Supply the current value to a source binding position. Generated
    /// wildcard columns introduced while expanding a catch-all are not new
    /// source binders and therefore never enter this list.
    fn bind_current_occurrence(mut self) -> Self {
        let current = self
            .real_occurrences
            .first()
            .expect("a bound matrix column has an aligned occurrence");
        debug_assert!(current.live);
        if current.source_binding {
            self.binding_occurrences.push(Some(current.term.clone()));
        }
        self
    }

    fn bind_current_occurrence_at(mut self, slot: usize) -> Self {
        let current = self
            .real_occurrences
            .first()
            .expect("a bound matrix column has an aligned occurrence");
        debug_assert!(current.live);
        if self.binding_occurrences.len() <= slot {
            self.binding_occurrences.resize(slot + 1, None);
        }
        debug_assert!(self.binding_occurrences[slot].is_none());
        self.binding_occurrences[slot] = Some(current.term.clone());
        self
    }

    fn drop_current_column(mut self) -> Self {
        self.assert_occurrence_alignment();
        self.real_pats.remove(0);
        self.real_occurrences.remove(0);
        self
    }

    fn specialize_current_column(
        self,
        replacement_pats: Vec<RPattern>,
        replacement_source_bindings: bool,
    ) -> Self {
        let surface_binder = self.real_occurrences[0].surface_binder;
        let source_bindings = vec![replacement_source_bindings; replacement_pats.len()];
        self.specialize_current_columns(
            replacement_pats, source_bindings, surface_binder, MatchBinderOrigin::Field,
        )
    }

    fn specialize_projected_record_column(
        self,
        replacement_pats: Vec<RPattern>,
        replacement_source_bindings: Vec<bool>,
    ) -> Self {
        self.specialize_current_columns(
            replacement_pats, replacement_source_bindings, false, MatchBinderOrigin::UserLocal,
        )
    }

    fn specialize_current_columns(
        mut self,
        replacement_pats: Vec<RPattern>,
        replacement_source_bindings: Vec<bool>,
        replacement_surface_binder: bool,
        origin: MatchBinderOrigin,
    ) -> Self {
        self.assert_occurrence_alignment();
        debug_assert_eq!(replacement_pats.len(), replacement_source_bindings.len());
        self.real_pats.remove(0);
        self.real_occurrences.remove(0);

        let mut real_pats = replacement_pats;
        real_pats.append(&mut self.real_pats);
        self.real_pats = real_pats;

        let mut real_occurrences = replacement_source_bindings
            .into_iter()
            .map(|source_binding| {
                MatrixOccurrence::pending_field(source_binding, replacement_surface_binder, origin)
            })
            .collect::<Vec<_>>();
        real_occurrences.append(&mut self.real_occurrences);
        self.real_occurrences = real_occurrences;
        self.assert_occurrence_alignment();
        self
    }

    /// The occurrence vector supplied at the leaf, in resolver binding order.
    /// The as-pattern consumer will extend this vector with alias positions;
    /// keeping the accessor on the production leaf boundary prevents a second
    /// pattern walk from becoming a competing occurrence derivation.
    fn leaf_binding_occurrences(&self) -> &[Option<Term>] {
        self.assert_occurrence_alignment();
        debug_assert!(self.real_pats.is_empty());
        #[cfg(test)]
        MATCH_OCCURRENCE_TRACE.with(|trace| {
            if let Some(trace) = trace.borrow_mut().as_mut() {
                trace.leaves.push(self.binding_occurrences.clone());
            }
        });
        &self.binding_occurrences
    }
}

/// Expose lexical bindings whose value comes from the aligned occurrence.
/// Surface `as` keeps its inner matcher; record-contained variables become a
/// wildcard after recording their projected value because record columns are
/// declaration-ordered rather than lexical kernel binders.
fn install_matrix_alias_type(
    cx: &mut ElabCtx,
    arm_idx: usize,
    slot: usize,
    alias: MatrixAliasType,
    span: &Span,
) {
    let frame = cx
        .pattern_alias_type_frames
        .last()
        .expect("infer_match installs an alias-type frame before matrix compilation");
    let prior = frame.aliases.get(&(arm_idx, slot)).cloned();
    if let Some(prior) = prior {
        let is_or_slot = cx
            .pattern_alias_type_frames
            .last()
            .expect("alias-type frame remains installed")
            .or_slots
            .contains(&(arm_idx, slot));
        if !is_or_slot {
            cx.pattern_alias_type_frames
                .last_mut()
                .expect("alias-type frame remains installed")
                .aliases
                .insert((arm_idx, slot), alias);
            return;
        }
        let common_depth = *cx
            .pattern_alias_type_frames
            .last()
            .expect("alias-type frame remains installed")
            .or_common_depths
            .get(&(arm_idx, slot))
            .expect("an or-pattern slot is registered when its residual row expands");
        let prior_drop = prior.install_depth.checked_sub(common_depth);
        let alias_drop = alias.install_depth.checked_sub(common_depth);
        let prior_common =
            prior_drop.and_then(|drop| lower_pattern_type_to_common(&prior.ty, drop));
        let alias_common =
            alias_drop.and_then(|drop| lower_pattern_type_to_common(&alias.ty, drop));
        let equal = match (prior_common, alias_common) {
            (Some(prior_ty), Some(alias_ty)) => {
                let mut common_ctx = cx.ctx.clone();
                common_ctx.types.truncate(common_depth);
                let prior_ty = cx.metas.zonk_term(&prior_ty);
                let alias_ty = cx.metas.zonk_term(&alias_ty);
                convert_type(cx.env, &common_ctx, &prior_ty, &alias_ty)
            }
            _ => false,
        };
        if !equal {
            let frame = cx
                .pattern_alias_type_frames
                .last_mut()
                .expect("alias-type frame remains installed");
            frame.type_mismatch.get_or_insert(OrBinderTypeMismatch {
                name: alias.name,
                span: span.clone(),
            });
        }
        return;
    }
    cx.pattern_alias_type_frames
        .last_mut()
        .expect("alias-type frame remains installed")
        .aliases
        .insert((arm_idx, slot), alias);
}

#[inline(never)]
fn expose_current_pattern_aliases(
    cx: &mut ElabCtx,
    mut row: RowState,
    current_ty: &Term,
) -> RowState {
    loop {
        let span = row.real_pats[0].span.clone();
        let (inner, name, slot, occurrence_var) = match row.real_pats[0].kind.clone() {
            RPatKind::As(inner, name, slot) => (*inner, name, slot, false),
            RPatKind::Var(name, Some(slot)) => (
                RPattern {
                    kind: RPatKind::Wild,
                    span: span.clone(),
                },
                name,
                slot,
                true,
            ),
            _ => break,
        };
        debug_assert!(
            row.binding_occurrences
                .get(slot)
                .is_none_or(Option::is_none),
            "each resolver slot must receive exactly one matrix occurrence"
        );
        if occurrence_var {
            cx.pattern_alias_type_frames
                .last_mut()
                .expect("infer_match installs an alias-type frame before matrix compilation")
                .hidden_slots
                .insert((row.arm_idx, slot));
        }
        if name != "_" {
            install_matrix_alias_type(
                cx,
                row.arm_idx,
                slot,
                MatrixAliasType {
                    name,
                    ty: current_ty.clone(),
                    install_depth: cx.ctx.len(),
                },
                &span,
            );
        }
        row = row.bind_current_occurrence_at(slot);
        if occurrence_var {
            row.real_occurrences[0].source_binding = false;
        }
        row.real_pats[0] = inner;
    }
    row
}

fn collect_or_pattern_slots(pattern: &RPattern, inside_or: bool, slots: &mut HashSet<usize>) {
    match &pattern.kind {
        RPatKind::Var(_, Some(slot)) | RPatKind::Literal(_, Some(slot)) if inside_or => {
            slots.insert(*slot);
        }
        RPatKind::As(inner, _, slot) => {
            if inside_or {
                slots.insert(*slot);
            }
            collect_or_pattern_slots(inner, inside_or, slots);
        }
        RPatKind::Or(alternatives) => {
            for alternative in alternatives {
                collect_or_pattern_slots(alternative, true, slots);
            }
        }
        RPatKind::Ctor(_, fields)
        | RPatKind::CheckedCtor(_, _, fields)
        | RPatKind::Tuple(fields) => {
            for field in fields {
                collect_or_pattern_slots(field, inside_or, slots);
            }
        }
        RPatKind::Record(fields) => {
            for field in fields {
                collect_or_pattern_slots(&field.pattern, inside_or, slots);
            }
        }
        RPatKind::Wild | RPatKind::Var(_, _) | RPatKind::Literal(_, _) => {}
    }
}

#[inline(never)]
fn build_alias_rows(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    scrut_core: &Term,
    scrut_ty: &Term,
) -> Vec<RowState> {
    let mut or_slots = HashSet::new();
    for (arm_idx, arm) in arms.iter().enumerate() {
        let mut slots = HashSet::new();
        collect_or_pattern_slots(&arm.pat, false, &mut slots);
        or_slots.extend(slots.into_iter().map(|slot| (arm_idx, slot)));
    }
    cx.pattern_alias_type_frames.push(PatternAliasTypeFrame {
        aliases: HashMap::new(),
        or_slots,
        or_common_depths: HashMap::new(),
        type_mismatch: None,
        hidden_slots: HashSet::new(),
    });
    let mut rows = Vec::with_capacity(arms.len());
    for (i, arm) in arms.iter().enumerate() {
        let row = RowState {
            real_pats: vec![arm.pat.clone()],
            real_occurrences: vec![MatrixOccurrence::live(scrut_core.clone())],
            binding_occurrences: Vec::new(),
            virtual_surface_positions: Vec::new(),
            virtual_aliases: Vec::new(),
            row_hidden_surface_positions: Vec::new(),
            arm_idx: i,
        };
        rows.push(expose_current_pattern_aliases(cx, row, scrut_ty));
    }
    rows
}

struct PatternAliasLeafScope {
    hidden_base: usize,
    virtual_base: usize,
}

#[inline(never)]
fn enter_pattern_alias_leaf(
    cx: &mut ElabCtx,
    arm_idx: usize,
    binding_occurrences: &[Option<Term>],
    _real_depth: usize,
    virtual_surface_positions: &[usize],
    virtual_aliases: &[MatrixVirtualAlias],
    row_hidden_surface_positions: &[usize],
) -> PatternAliasLeafScope {
    let frame = cx
        .pattern_alias_type_frames
        .last()
        .expect("infer_match alias-type frame must span matrix compilation");
    let alias_types = frame
        .aliases
        .iter()
        .filter(|((candidate, _), _)| *candidate == arm_idx)
        .map(|((_, slot), alias)| (*slot, alias.clone()))
        .collect::<Vec<_>>();
    let hidden_slots = frame
        .hidden_slots
        .iter()
        .filter(|(candidate, _)| *candidate == arm_idx)
        .map(|(_, slot)| *slot)
        .collect::<Vec<_>>();
    let mut active_aliases = Vec::with_capacity(alias_types.len() + virtual_aliases.len());
    for (slot, alias) in alias_types {
        let occurrence = binding_occurrences
            .get(slot)
            .and_then(Clone::clone)
            .unwrap_or_else(|| {
                panic!("pattern slot {slot} is absent from its matrix-leaf occurrence vector")
            });
        active_aliases.push(ActivePatternAlias {
            slot,
            virtual_slot: None,
            name: alias.name,
            occurrence,
            occurrence_depth: cx.ctx.len(),
            ty: alias.ty,
            install_depth: alias.install_depth,
        });
    }
    let hidden_base = cx.hidden_positions.len();
    for slot in hidden_slots {
        let Some(Term::Var(index)) = binding_occurrences.get(slot).and_then(Option::as_ref) else {
            continue;
        };
        if let Some(position) = cx.ctx.len().checked_sub(index + 1) {
            if !cx.hidden_positions.contains(&position) {
                cx.hidden_positions.push(position);
            }
        }
    }
    for &position in row_hidden_surface_positions {
        debug_assert!(position < cx.ctx.len());
        if !cx.hidden_positions.contains(&position) {
            cx.hidden_positions.push(position);
        }
    }
    let virtual_base = cx.matrix_virtual_surface_positions.len();
    cx.matrix_virtual_surface_positions
        .extend_from_slice(virtual_surface_positions);
    for alias in virtual_aliases {
        debug_assert!(alias.slot < virtual_surface_positions.len());
        active_aliases.push(ActivePatternAlias {
            slot: alias.slot,
            virtual_slot: Some(virtual_base + alias.slot),
            name: alias.name.clone(),
            occurrence: alias.occurrence.clone(),
            occurrence_depth: cx.ctx.len(),
            ty: alias.ty.clone(),
            install_depth: alias.install_depth,
        });
    }
    cx.active_pattern_aliases.push(active_aliases);
    PatternAliasLeafScope { hidden_base, virtual_base }
}

fn leave_pattern_alias_leaf(cx: &mut ElabCtx, scope: PatternAliasLeafScope) {
    cx.active_pattern_aliases.pop();
    cx.hidden_positions.truncate(scope.hidden_base);
    cx.matrix_virtual_surface_positions.truncate(scope.virtual_base);
}

#[inline(never)]
fn infer_arm_at_matrix_leaf(
    cx: &mut ElabCtx,
    arm: &RMatchArm,
    arm_idx: usize,
    binding_occurrences: &[Option<Term>],
    real_depth: usize,
    virtual_surface_positions: &[usize],
    virtual_aliases: &[MatrixVirtualAlias],
    row_hidden_surface_positions: &[usize],
) -> Result<(Option<Term>, Term, Term), ElabError> {
    let scope = enter_pattern_alias_leaf(
        cx, arm_idx, binding_occurrences, real_depth, virtual_surface_positions,
        virtual_aliases, row_hidden_surface_positions,
    );
    let result = (|| {
        let guard_result = arm
            .guard
            .as_ref()
            .map(|guard| elaborate_if_condition(cx, guard))
            .transpose();
        let guard = guard_result?;
        let (body, body_ty) = infer(cx, &arm.body)?;
        Ok((guard, body, body_ty))
    })();
    leave_pattern_alias_leaf(cx, scope);
    result
}

#[inline(never)]
fn check_arm_at_matrix_leaf(
    cx: &mut ElabCtx,
    arm: &RMatchArm,
    arm_idx: usize,
    binding_occurrences: &[Option<Term>],
    real_depth: usize,
    virtual_surface_positions: &[usize],
    virtual_aliases: &[MatrixVirtualAlias],
    row_hidden_surface_positions: &[usize],
    expected: &Term,
) -> Result<(Option<Term>, Term), ElabError> {
    let scope = enter_pattern_alias_leaf(
        cx, arm_idx, binding_occurrences, real_depth, virtual_surface_positions,
        virtual_aliases, row_hidden_surface_positions,
    );
    let result = (|| {
        let guard_result = arm
            .guard
            .as_ref()
            .map(|guard| elaborate_if_condition(cx, guard))
            .transpose();
        let guard = guard_result?;
        let body = check_match_arm_result(cx, arm, expected, &arm.body.span())?;
        Ok((guard, body))
    })();
    leave_pattern_alias_leaf(cx, scope);
    result
}

fn pattern_without_aliases(mut pattern: &RPattern) -> &RPattern {
    while let RPatKind::As(inner, _, _) = &pattern.kind {
        pattern = inner;
    }
    pattern
}

fn guarded_leaf_missing_witness(pattern: &RPattern) -> MissingPatternWitness {
    match &pattern.kind {
        RPatKind::Ctor(name, fields) | RPatKind::CheckedCtor(name, _, fields) => MissingPatternWitness {
            constructor: name.clone(),
            arity: fields.len(),
        },
        RPatKind::As(inner, _, _) => guarded_leaf_missing_witness(inner),
        RPatKind::Or(alternatives) => alternatives
            .first()
            .map(guarded_leaf_missing_witness)
            .unwrap_or_else(|| MissingPatternWitness {
                constructor: "_".into(),
                arity: 0,
            }),
        RPatKind::Tuple(_)
        | RPatKind::Record(_)
        | RPatKind::Wild
        | RPatKind::Var(_, _)
        | RPatKind::Literal(_, _) => MissingPatternWitness {
            constructor: "_".into(),
            arity: 0,
        },
    }
}

/// Memoize the indexed root motive at the leaf that first determines its
/// result type. Kernel method types then supply the exact IH domains for the
/// matrix and the final root eliminator consumes this same motive term.
fn memoize_indexed_root_motive(
    cx: &mut ElabCtx,
    ret_ty: &Term,
    span: &Span,
    root_frame_depth: usize,
) -> Result<(), ElabError> {
    // An arm may contain its own match while the enclosing indexed root is
    // installed. Only the match that pushed this frame may determine its R.
    if cx.indexed_match_roots.len() != root_frame_depth + 1 {
        return Ok(());
    }
    let root = cx.indexed_match_roots.last().expect("owned root frame is installed");
    if root.motive.is_some() {
        return Ok(());
    }
    let (outer, ind, family, level_args, params, scrut_indices) = (
        root.outer.clone(),
        root.ind.clone(),
        root.family,
        root.level_args.clone(),
        root.params.clone(),
        root.scrut_indices.clone(),
    );
    let zonked_outer = Context {
        types: outer.types.iter().map(|ty| cx.metas.zonk_term(ty)).collect(),
    };
    let motive_ctx = motive_context_at(&zonked_outer, &ind, &params, &level_args);
    let motive = build_checked_dependent_motive(
        cx,
        &motive_ctx,
        &ind,
        family,
        &params,
        &scrut_indices,
        weaken(ret_ty, (ind.indices.len() + 1) as i64),
        false,
        RecursiveFieldIndexPath::CoupledRefinement,
        span,
    )?;
    let root = cx.indexed_match_roots.last_mut().expect("root frame remains installed");
    root.motive = Some(motive);
    Ok(())
}

/// Partition the pending telescope by free occurrences in its domains, not
/// by whether a column originated as a constructor field or an IH. A member
/// depending on the split variable, an index variable, or an earlier member
/// joins the reverted convoy. The result type is tested at the full depth.
fn nested_split_dependencies(
    codomain: &Term,
    tail_count: usize,
    indices_under_split: &[Term],
) -> Result<(Vec<usize>, bool), ElabError> {
    let mut cursor = codomain;
    let mut dependent = Vec::new();
    for position in 0..tail_count {
        let Term::Pi(domain, rest) = cursor else {
            return Err(ElabError::Internal(
                "nested continuation lost its pending telescope".into(),
            ));
        };
        let mentions_split = scrut_occurs(domain, &Term::var(position));
        let mentions_index = indices_under_split.iter().any(|index| {
            scrut_occurs(domain, &weaken(index, position as i64))
        });
        let mentions_prior = dependent.iter().any(|prior| {
            scrut_occurs(domain, &Term::var(position - prior - 1))
        });
        if mentions_split || mentions_index || mentions_prior {
            dependent.push(position);
        }
        cursor = rest;
    }
    let result_mentions_split = scrut_occurs(cursor, &Term::var(tail_count));
    Ok((dependent, result_mentions_split))
}

/// Nested indexed splits must not infer an equality convoy for a concrete
/// or repeated index. The caller has already checked constructor coverage;
/// this diagnostic cannot replace a nested omission's ExhaustivenessError.
fn check_nested_index_variables(
    cx: &ElabCtx<'_>,
    indices: &[Term],
    split_span: &Span,
) -> Result<(), ElabError> {
    let mut seen = HashSet::new();
    for index in indices {
        let Term::Var(index_var) = index else {
            return Err(ElabError::TypeMismatch {
                span: split_span.clone(),
                reason: "nested indexed split needs distinct variable indices; annotate the match result or use a separate match".into(),
            });
        };
        if !seen.insert(*index_var) {
            return Err(ElabError::TypeMismatch {
                span: split_span.clone(),
                reason: "nested indexed split repeats an index; annotate the match result or use a separate match".into(),
            });
        }
        // A captured ambient binder referring to this index cannot be
        // generalized here unless it travels in the pending convoy below.
        for binder in 0..cx.ctx.len() {
            if binder == *index_var {
                continue;
            }
            let Some(domain) = cx.ctx.lookup(binder) else { continue };
            let ambient = weaken(domain, (binder + 1) as i64);
            if scrut_occurs(&ambient, &Term::var(*index_var)) {
                return Err(ElabError::TypeMismatch {
                    span: split_span.clone(),
                    reason: "nested indexed split shares an index with an ambient binder; annotate the match result or use a separate match".into(),
                });
            }
        }
    }
    Ok(())
}

/// A split's motive must compute the Pi-chain owed by its pending columns.
/// Root IH domains come from the root eliminator's method type, not from R.
fn tail_codomain(
    cx: &ElabCtx<'_>,
    tail_col_types: &[Term],
    tail_col_kinds: &[ColKind],
    ret_ty_base: &Term,
    depth_before_tail: usize,
) -> Result<Term, ElabError> {
    if tail_col_types.is_empty() {
        return Ok(weaken(ret_ty_base, depth_before_tail as i64));
    }
    match tail_col_kinds[0] {
        ColKind::Ih => {
            let ih_ty = tail_col_types[0].clone();
            let rest = tail_codomain(
                cx,
                &tail_col_types[1..],
                &tail_col_kinds[1..],
                ret_ty_base,
                depth_before_tail + 1,
            )?;
            Ok(Term::pi(ih_ty, rest))
        }
        ColKind::Real => {
            let rest = tail_codomain(
                cx,
                &tail_col_types[1..],
                &tail_col_kinds[1..],
                ret_ty_base,
                depth_before_tail + 1,
            )?;
            Ok(Term::pi(tail_col_types[0].clone(), rest))
        }
    }
}

/// The closed, expected-carrier-derived realization of one literal comparison.
///
/// This is deliberately separate from `NumericEnv::eq_table`: `Direct` names a
/// real binary operation, while the other variants are finite compiler-owned
/// plans over already-landed lossless views. Every variant returns only `Bool`.
#[derive(Clone)]
enum LiteralComparatorPlan {
    Direct {
        comparator: GlobalId,
        literal: Term,
    },
    FixedWidth {
        view: GlobalId,
        eq_int: GlobalId,
        literal: Term,
    },
    String {
        view: GlobalId,
        eq_char: GlobalId,
        elements: Vec<Term>,
    },
    Bytes {
        view: GlobalId,
        uint8_to_int: GlobalId,
        eq_int: GlobalId,
        elements: Vec<Term>,
    },
}

/// Host-side key used only to put comparator-equal literal rows in one matrix
/// bucket. It mirrors the selected value comparator rather than source spelling.
#[derive(Clone)]
enum LiteralComparatorValue {
    Integer(num_bigint::BigInt),
    Float(f64),
    Float32(f32),
    String(crate::NfcString),
    Char(char),
    Bytes(Vec<u8>),
}

impl LiteralComparatorValue {
    fn same_value(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left == right,
            (Self::Float32(left), Self::Float32(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Char(left), Self::Char(right)) => left == right,
            (Self::Bytes(left), Self::Bytes(right)) => left == right,
            (
                Self::Integer(_)
                | Self::Float(_)
                | Self::Float32(_)
                | Self::String(_)
                | Self::Char(_)
                | Self::Bytes(_),
                _,
            ) => false,
        }
    }
}

#[derive(Clone, Copy)]
enum LiteralListElementComparator {
    Direct(GlobalId),
    ThroughIntView { view: GlobalId, eq_int: GlobalId },
}

fn literal_builtin(cx: &ElabCtx<'_>, name: &str) -> Result<GlobalId, ElabError> {
    cx.globals.get(name).copied().ok_or_else(|| {
        ElabError::Internal(format!("literal comparator builtin '{name}' is missing"))
    })
}

fn literal_expected_is(cx: &ElabCtx<'_>, expected: &Term, carrier: GlobalId) -> bool {
    let expected = cx.metas.zonk_term(expected);
    convert_type(cx.env, &cx.ctx, &expected, &Term::const_(carrier, vec![]))
}

fn literal_carrier_name(cx: &ElabCtx<'_>, expected: &Term) -> String {
    let expected = cx.metas.zonk_term(expected);
    let (head, _) = peel_app(&expected);
    match head {
        Term::Const { id, .. } | Term::IndFormer { id, .. } => type_name(cx, id),
        other => format!("{other:?}"),
    }
}

fn unsupported_literal_pattern(
    cx: &ElabCtx<'_>,
    literal: &LiteralPat,
    expected: &Term,
    span: &Span,
) -> ElabError {
    let row = match literal {
        LiteralPat::Numeric(NumLit::Decimal(_, _)) => "Decimal numeric",
        LiteralPat::Numeric(_) => "numeric",
        LiteralPat::String(_) => "String",
        LiteralPat::Char(_) => "Char",
        LiteralPat::Bytes(_) => "Bytes",
    };
    ElabError::TypeMismatch {
        span: span.clone(),
        reason: format!(
            "{row} literal-pattern row is unsupported for carrier '{}'",
            literal_carrier_name(cx, expected)
        ),
    }
}

/// Check the literal at the scrutinee's expected type, then select exactly one
/// closed realization plan from that carrier. No representation guess or
/// dictionary search participates in this selection.
#[inline(never)]
fn plan_literal_comparator(
    cx: &mut ElabCtx<'_>,
    literal: &LiteralPat,
    expected: &Term,
    span: &Span,
) -> Result<(LiteralComparatorValue, LiteralComparatorPlan), ElabError> {
    match literal {
        LiteralPat::Numeric(number) => {
            let literal_core = elab_num_lit_checked(cx, number, expected, span)?;
            match number {
                NumLit::Int(value) => {
                    if literal_expected_is(cx, expected, cx.numeric_env.int_id) {
                        let comparator = cx
                            .numeric_env
                            .classify_eq(&Term::const_(cx.numeric_env.int_id, vec![]))
                            .expect("Int has a registered value comparator")
                            .op_id;
                        return Ok((
                            LiteralComparatorValue::Integer(value.clone()),
                            LiteralComparatorPlan::Direct {
                                comparator,
                                literal: literal_core,
                            },
                        ));
                    }

                    let fixed_views = [
                        (cx.numeric_env.int8_id, "int8_to_int"),
                        (cx.numeric_env.int16_id, "int16_to_int"),
                        (cx.numeric_env.int32_id, "int32_to_int"),
                        (cx.numeric_env.int64_id, "int64_to_int"),
                        (cx.numeric_env.uint8_id, "uint8_to_int"),
                        (cx.numeric_env.uint16_id, "uint16_to_int"),
                        (cx.numeric_env.uint32_id, "uint32_to_int"),
                        (cx.numeric_env.uint64_id, "uint64_to_int"),
                    ];
                    for (carrier, view_name) in fixed_views {
                        if literal_expected_is(cx, expected, carrier) {
                            return Ok((
                                LiteralComparatorValue::Integer(value.clone()),
                                LiteralComparatorPlan::FixedWidth {
                                    view: literal_builtin(cx, view_name)?,
                                    eq_int: literal_builtin(cx, "eq_int")?,
                                    literal: literal_core,
                                },
                            ));
                        }
                    }
                }
                NumLit::Float(value)
                    if literal_expected_is(cx, expected, cx.numeric_env.float_id) =>
                {
                    let comparator = cx
                        .numeric_env
                        .classify_eq(&Term::const_(cx.numeric_env.float_id, vec![]))
                        .expect("Float has a registered value comparator")
                        .op_id;
                    return Ok((
                        LiteralComparatorValue::Float(*value),
                        LiteralComparatorPlan::Direct {
                            comparator,
                            literal: literal_core,
                        },
                    ));
                }
                NumLit::Float32(value)
                    if literal_expected_is(cx, expected, cx.numeric_env.float32_id) =>
                {
                    let comparator = cx
                        .numeric_env
                        .classify_eq(&Term::const_(cx.numeric_env.float32_id, vec![]))
                        .expect("Float32 has a registered value comparator")
                        .op_id;
                    return Ok((
                        LiteralComparatorValue::Float32(*value),
                        LiteralComparatorPlan::Direct {
                            comparator,
                            literal: literal_core,
                        },
                    ));
                }
                NumLit::Decimal(_, _) | NumLit::Float(_) | NumLit::Float32(_) => {}
            }
        }
        LiteralPat::String(value) => {
            let _checked = elab_str_lit(cx, value, Some(expected), span)?;
            let string_id = literal_builtin(cx, "String")?;
            if literal_expected_is(cx, expected, string_id) {
                let normalized = crate::NfcString::new(value);
                let mut elements = Vec::new();
                for scalar in normalized.chars() {
                    elements.push(elab_char_lit(cx, scalar, span)?.0);
                }
                return Ok((
                    LiteralComparatorValue::String(normalized),
                    LiteralComparatorPlan::String {
                        view: literal_builtin(cx, "string_to_list_char")?,
                        eq_char: literal_builtin(cx, "eqChar")?,
                        elements,
                    },
                ));
            }
        }
        LiteralPat::Char(value) => {
            let (literal_core, literal_ty) = elab_char_lit(cx, *value, span)?;
            unify_types(&mut cx.metas, expected, &literal_ty);
            if literal_expected_is(cx, expected, cx.numeric_env.char_id) {
                let comparator = cx
                    .numeric_env
                    .classify_eq(&Term::const_(cx.numeric_env.char_id, vec![]))
                    .expect("Char has a registered value comparator")
                    .op_id;
                return Ok((
                    LiteralComparatorValue::Char(*value),
                    LiteralComparatorPlan::Direct {
                        comparator,
                        literal: literal_core,
                    },
                ));
            }
        }
        LiteralPat::Bytes(value) => {
            let (checked, literal_ty) = elab_bytes_lit(cx, value, span)?;
            unify_types(&mut cx.metas, expected, &literal_ty);
            let bytes_id = literal_builtin(cx, "Bytes")?;
            if literal_expected_is(cx, expected, bytes_id) {
                let uint8_ty = Term::const_(cx.numeric_env.uint8_id, vec![]);
                let mut elements = Vec::with_capacity(value.len());
                for octet in value {
                    elements.push(elab_num_lit_checked(
                        cx,
                        &NumLit::Int((*octet).into()),
                        &uint8_ty,
                        span,
                    )?);
                }
                let _ = checked;
                return Ok((
                    LiteralComparatorValue::Bytes(value.clone()),
                    LiteralComparatorPlan::Bytes {
                        view: literal_builtin(cx, "bytes_to_list")?,
                        uint8_to_int: literal_builtin(cx, "uint8_to_int")?,
                        eq_int: literal_builtin(cx, "eq_int")?,
                        elements,
                    },
                ));
            }
        }
    }
    Err(unsupported_literal_pattern(cx, literal, expected, span))
}

/// Discovery's literal declarations stay in the same GlobalEnv instance.
/// Replaying its prefix returns the already checked plan, not a second mint.
fn matrix_literal_plan(
    cx: &mut ElabCtx<'_>,
    literal: &LiteralPat,
    expected: &Term,
    span: &Span,
    root_frame_depth: usize,
) -> Result<(LiteralComparatorValue, LiteralComparatorPlan), ElabError> {
    let frame = cx.matrix_entries.last_mut().ok_or_else(|| {
        ElabError::Internal("literal matrix has no owning entry".into())
    })?;
    if frame.root_frame_depth != root_frame_depth {
        return Err(ElabError::Internal("literal plan crossed its matrix owner".into()));
    }
    let ordinal = frame.literal_requests.entry((span.start, span.end)).or_default();
    let key = (span.start, span.end, *ordinal);
    *ordinal += 1;
    if frame.rerun {
        if let Some(saved) = frame.literal_plans.get(&key) {
            let same_literal = match (literal, &saved.literal) {
                (LiteralPat::Numeric(NumLit::Float(a)), LiteralPat::Numeric(NumLit::Float(b))) => {
                    a.to_bits() == b.to_bits()
                }
                (LiteralPat::Numeric(NumLit::Float32(a)), LiteralPat::Numeric(NumLit::Float32(b))) => {
                    a.to_bits() == b.to_bits()
                }
                _ => literal == &saved.literal,
            };
            if !same_literal || expected != &saved.column_type {
                return Err(ElabError::Internal(
                    "literal plan replay changed its literal value or column type".into(),
                ));
            }
            return Ok((saved.value.clone(), saved.plan.clone()));
        }
        if !frame.reused_leaf {
            return Err(ElabError::Internal("literal plan replay missed discovery prefix".into()));
        }
    }
    let (value, plan) = plan_literal_comparator(cx, literal, expected, span)?;
    let frame = cx.matrix_entries.last_mut().expect("literal owner is installed");
    frame.literal_plans.insert(key, MatrixLiteralPlan {
        literal: literal.clone(), column_type: expected.clone(),
        value: value.clone(), plan: plan.clone(),
    });
    Ok((value, plan))
}

fn literal_bool_value(cx: &ElabCtx<'_>, value: bool) -> Term {
    Term::constructor(
        if value {
            cx.numeric_env.bool_true_id
        } else {
            cx.numeric_env.bool_false_id
        },
        vec![],
    )
}

fn literal_bool_select(
    cx: &ElabCtx<'_>,
    condition: Term,
    then_branch: Term,
    else_branch: Term,
) -> Result<Term, ElabError> {
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    let motive = Term::Ascript(
        Box::new(Term::lam(bool_ty.clone(), weaken(&bool_ty, 1))),
        Box::new(Term::pi(bool_ty.clone(), Term::ty(Level::Zero))),
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
    debug_assert!(bool_decl.indices.is_empty());
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

fn compare_literal_list_element(
    comparator: LiteralListElementComparator,
    actual: Term,
    expected: Term,
) -> Term {
    match comparator {
        LiteralListElementComparator::Direct(op) => {
            apply_term_spine(Term::const_(op, vec![]), &[actual, expected])
        }
        LiteralListElementComparator::ThroughIntView { view, eq_int } => {
            let view = Term::const_(view, vec![]);
            apply_term_spine(
                Term::const_(eq_int, vec![]),
                &[Term::app(view.clone(), actual), Term::app(view, expected)],
            )
        }
    }
}

/// Compare a `List A` value to a finite literal sequence by unrolling exactly
/// that sequence and requiring `Nil` after its final element.
fn build_literal_list_test(
    cx: &ElabCtx<'_>,
    element_ty: &Term,
    elements: &[Term],
    comparator: LiteralListElementComparator,
    scrutinee: Term,
) -> Result<Term, ElabError> {
    let list_id = literal_builtin(cx, "List")?;
    let nil_id = literal_builtin(cx, "Nil")?;
    let cons_id = literal_builtin(cx, "Cons")?;
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    let list_ty = Term::app(Term::indformer(list_id, vec![]), element_ty.clone());
    let motive = Term::Ascript(
        Box::new(Term::lam(list_ty.clone(), weaken(&bool_ty, 1))),
        Box::new(Term::pi(list_ty.clone(), Term::ty(Level::Zero))),
    );

    let nil_method = literal_bool_value(cx, elements.is_empty());
    let cons_body = if elements.is_empty() {
        literal_bool_value(cx, false)
    } else {
        let actual_head = Term::var(2);
        let actual_tail = Term::var(1);
        let expected_head = weaken(&elements[0], 3);
        let condition = compare_literal_list_element(comparator, actual_head, expected_head);
        let nested_element_ty = weaken(element_ty, 3);
        let nested_elements = elements[1..]
            .iter()
            .map(|element| weaken(element, 3))
            .collect::<Vec<_>>();
        let then_branch = build_literal_list_test(
            cx,
            &nested_element_ty,
            &nested_elements,
            comparator,
            actual_tail,
        )?;
        literal_bool_select(cx, condition, then_branch, literal_bool_value(cx, false))?
    };
    let cons_method = Term::lam(
        element_ty.clone(),
        Term::lam(
            Term::app(Term::indformer(list_id, vec![]), weaken(element_ty, 1)),
            Term::lam(bool_ty, cons_body),
        ),
    );
    let list_decl = cx
        .env
        .inductive(list_id)
        .ok_or_else(|| ElabError::Internal("preregistered List is missing".into()))?;
    let methods = list_decl
        .constructors
        .iter()
        .map(|constructor| {
            if constructor.id == nil_id {
                Ok(nil_method.clone())
            } else if constructor.id == cons_id {
                Ok(cons_method.clone())
            } else {
                Err(ElabError::Internal(
                    "preregistered List has an unknown constructor identity".into(),
                ))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    debug_assert!(list_decl.indices.is_empty());
    Ok(Term::Elim {
        fam: list_id,
        level_args: vec![],
        params: vec![element_ty.clone()],
        motive: Box::new(motive),
        methods,
        indices: vec![],
        scrut: Box::new(scrutinee),
    })
}

impl LiteralComparatorPlan {
    fn condition(&self, cx: &ElabCtx<'_>, scrutinee: Term) -> Result<Term, ElabError> {
        match self {
            Self::Direct {
                comparator,
                literal,
            } => Ok(apply_term_spine(
                Term::const_(*comparator, vec![]),
                &[scrutinee, literal.clone()],
            )),
            Self::FixedWidth {
                view,
                eq_int,
                literal,
            } => {
                let view = Term::const_(*view, vec![]);
                Ok(apply_term_spine(
                    Term::const_(*eq_int, vec![]),
                    &[
                        Term::app(view.clone(), scrutinee),
                        Term::app(view, literal.clone()),
                    ],
                ))
            }
            Self::String {
                view,
                eq_char,
                elements,
            } => build_literal_list_test(
                cx,
                &Term::const_(cx.numeric_env.char_id, vec![]),
                elements,
                LiteralListElementComparator::Direct(*eq_char),
                Term::app(Term::const_(*view, vec![]), scrutinee),
            ),
            Self::Bytes {
                view,
                uint8_to_int,
                eq_int,
                elements,
            } => build_literal_list_test(
                cx,
                &Term::const_(cx.numeric_env.uint8_id, vec![]),
                elements,
                LiteralListElementComparator::ThroughIntView {
                    view: *uint8_to_int,
                    eq_int: *eq_int,
                },
                Term::app(Term::const_(*view, vec![]), scrutinee),
            ),
        }
    }
}

fn expose_current_literal_occurrence(cx: &mut ElabCtx<'_>, mut row: RowState) -> RowState {
    let RPatKind::Literal(literal, Some(slot)) = row.real_pats[0].kind.clone() else {
        return row;
    };
    cx.pattern_alias_type_frames
        .last_mut()
        .expect("literal matrix compilation occurs inside an alias frame")
        .hidden_slots
        .insert((row.arm_idx, slot));
    row = row.bind_current_occurrence_at(slot);
    row.real_occurrences[0].source_binding = false;
    row.real_pats[0].kind = RPatKind::Literal(literal, None);
    row
}

fn consume_literal_column(row: RowState) -> RowState {
    row.bind_current_occurrence().drop_current_column()
}

/// Compile one literal column as ordered value tests plus an unguarded residual
/// fallback. The fresh binder is an alignment device only: matching never adds
/// a proof or refinement to `cx`.
/// Boolean comparator decisions are the path facts of a literal matrix.
/// A later literal is reached only when all earlier comparisons were false.
fn push_literal_branch_condition(cx: &mut ElabCtx<'_>, condition: Term, truth: bool) {
    let chosen = if truth { cx.numeric_env.bool_true_id } else { cx.numeric_env.bool_false_id };
    cx.path_conditions.push((Term::Eq(
        Box::new(Term::indformer(cx.numeric_env.bool_id, vec![])),
        Box::new(condition), Box::new(Term::constructor(chosen, vec![])),
    ), cx.ctx.len()));
}

#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn compile_literal_column(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    col_types: &[Term],
    col_kinds: &[ColKind],
    rows: Vec<RowState>,
    real_depth_so_far: usize,
    top_span: &Span,
    root_frame_depth: usize,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Term, ElabError> {
    let current_is_live = rows[0].real_occurrences[0].live;
    let outer_occurrence = rows[0].real_occurrences[0].term.clone();
    let surface_binder = rows[0].real_occurrences[0].surface_binder;
    debug_assert!(rows.iter().all(|row| {
        row.real_occurrences[0].live == current_is_live
            && row.real_occurrences[0].surface_binder == surface_binder
    }));

    let current_ty = weaken(&col_types[0], 1);
    let mut entered_rows = Vec::with_capacity(rows.len());
    for row in rows {
        let row = row.enter_current_real_binder();
        let row = expose_current_pattern_aliases(cx, row, &current_ty);
        entered_rows.push(expose_current_literal_occurrence(cx, row));
    }
    let rows = entered_rows;

    cx.push_match_binder(col_types[0].clone(), MatchBinderOrigin::UserLocal);
    if !surface_binder {
        cx.hidden_positions.push(cx.ctx.len() - 1);
    }
    let result = (|| {
        let mut row_values = Vec::with_capacity(rows.len());
        let mut groups: Vec<(LiteralComparatorValue, LiteralComparatorPlan)> = Vec::new();
        for row in &rows {
            let value = match &row.real_pats[0].kind {
                RPatKind::Literal(literal, _) => {
                    let (value, plan) =
                        matrix_literal_plan(cx, literal, &current_ty, &row.real_pats[0].span, root_frame_depth)?;
                    if !groups
                        .iter()
                        .any(|(existing, _)| existing.same_value(&value))
                    {
                        groups.push((value.clone(), plan));
                    }
                    Some(value)
                }
                RPatKind::Wild | RPatKind::Var(_, _) => None,
                RPatKind::Ctor(_, _)
                | RPatKind::CheckedCtor(_, _, _)
                | RPatKind::Tuple(_)
                | RPatKind::Record(_) => {
                    return Err(ElabError::TypeMismatch {
                        span: row.real_pats[0].span.clone(),
                        reason: "literal column cannot mix value literals with structural patterns"
                            .into(),
                    })
                }
                RPatKind::As(_, _, _) | RPatKind::Or(_) => {
                    unreachable!("aliases and or-patterns are exposed before literal compilation")
                }
            };
            row_values.push(value);
        }

        let residual_rows = rows
            .iter()
            .zip(&row_values)
            .filter(|(_, value)| value.is_none())
            .map(|(row, _)| consume_literal_column(row.clone()))
            .collect::<Vec<_>>();
        if residual_rows.is_empty() {
            return Err(ElabError::ExhaustivenessError {
                missing: MissingPatternWitness {
                    constructor: "_".into(),
                    arity: 0,
                },
                span: top_span.clone(),
            });
        }

        let mut compiled = Vec::with_capacity(groups.len());
        let mut prior_conditions: Vec<Term> = Vec::new();
        for (value, plan) in &groups {
            let branch_rows = rows
                .iter()
                .zip(&row_values)
                .filter(|(_, candidate)| {
                    candidate
                        .as_ref()
                        .is_none_or(|candidate| candidate.same_value(value))
                })
                .map(|(row, _)| consume_literal_column(row.clone()))
                .collect::<Vec<_>>();
            let condition = plan.condition(cx, Term::var(0))?;
            let path_base = cx.path_conditions.len();
            for earlier in &prior_conditions {
                push_literal_branch_condition(cx, earlier.clone(), false);
            }
            push_literal_branch_condition(cx, condition.clone(), true);
            let checked = compile_match_matrix(
                cx, arms, &col_types[1..], &col_kinds[1..], branch_rows,
                real_depth_so_far + 1, top_span, root_frame_depth,
                ret_ty_slot, arm_used, subsumed_by,
            );
            cx.path_conditions.truncate(path_base);
            let body = checked?;
            compiled.push((condition.clone(), body));
            prior_conditions.push(condition);
        }
        let path_base = cx.path_conditions.len();
        for earlier in &prior_conditions {
            push_literal_branch_condition(cx, earlier.clone(), false);
        }
        let fallback = compile_match_matrix(
            cx, arms, &col_types[1..], &col_kinds[1..], residual_rows,
            real_depth_so_far + 1, top_span, root_frame_depth,
            ret_ty_slot, arm_used, subsumed_by,
        );
        cx.path_conditions.truncate(path_base);
        let mut body = fallback?;
        let ret_ty = ret_ty_slot
            .as_ref()
            .expect("literal compilation reaches a body leaf")
            .clone();
        let branch_ty = tail_codomain(
            cx,
            &col_types[1..],
            &col_kinds[1..],
            &ret_ty,
            real_depth_so_far + 1,
        )?;
        for (condition, then_branch) in compiled.into_iter().rev() {
            body = make_if_elim(cx, condition, then_branch, body, &branch_ty, top_span)?;
        }
        Ok((body, branch_ty))
    })();
    if !surface_binder {
        let hidden = cx.hidden_positions.pop();
        debug_assert_eq!(hidden, Some(cx.ctx.len() - 1));
    }
    cx.ctx.pop();
    let (body, branch_ty) = result?;
    let function = Term::lam(col_types[0].clone(), body);
    if current_is_live {
        Ok(Term::app(
            Term::Ascript(
                Box::new(function),
                Box::new(Term::pi(col_types[0].clone(), branch_ty)),
            ),
            outer_occurrence,
        ))
    } else {
        Ok(function)
    }
}

/// Split one negative Sigma column into its two projected components, compile
/// those components through the existing matrix continuation, then apply the
/// resulting two binders to the projections. Surface tuples with arity above
/// two remain right-nested in their second component and revisit this same
/// binary split on the next matrix step.
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn compile_tuple_column(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    col_types: &[Term],
    col_kinds: &[ColKind],
    mut rows: Vec<RowState>,
    real_depth_so_far: usize,
    top_span: &Span,
    root_frame_depth: usize,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Term, ElabError> {
    let current_ty = whnf(cx.env, &cx.ctx, &col_types[0]);
    let Term::Sigma(domain, codomain) = current_ty else {
        return Err(ElabError::TypeMismatch {
            span: top_span.clone(),
            reason: "tuple pattern requires a pair type".into(),
        });
    };

    let current_is_live = rows[0].real_occurrences[0].live;
    debug_assert!(rows
        .iter()
        .all(|row| row.real_occurrences[0].live == current_is_live));
    if !current_is_live {
        cx.push_match_binder(col_types[0].clone(), MatchBinderOrigin::UserLocal);
        cx.hidden_positions.push(cx.ctx.len() - 1);
        rows = rows
            .into_iter()
            .map(RowState::enter_current_real_binder)
            .map(|row| expose_current_pattern_aliases(cx, row, &col_types[0]))
            .collect();
    }
    let result = (|| {
    let pair_occurrence = rows[0].real_occurrences[0].term.clone();

    let mut component_rows = Vec::with_capacity(rows.len());
    for row in rows {
        let row = expose_current_pattern_aliases(cx, row, &col_types[0]);
        match row.real_pats[0].kind.clone() {
            RPatKind::Tuple(components) if components.len() >= 2 => {
                let first = components[0].clone();
                let second = if components.len() == 2 {
                    components[1].clone()
                } else {
                    RPattern {
                        kind: RPatKind::Tuple(components[1..].to_vec()),
                        span: row.real_pats[0].span.clone(),
                    }
                };
                component_rows.push(row.specialize_current_column(vec![first, second], true));
            }
            RPatKind::Tuple(_) => {
                return Err(ElabError::Internal(
                    "resolved tuple pattern must contain at least two components".into(),
                ));
            }
            RPatKind::Wild | RPatKind::Var(_, _) => {
                let span = row.real_pats[0].span.clone();
                let wild = || RPattern {
                    kind: RPatKind::Wild,
                    span: span.clone(),
                };
                component_rows.push(
                    row.bind_current_occurrence()
                        .specialize_current_column(vec![wild(), wild()], false),
                );
            }
            RPatKind::Ctor(_, _)
            | RPatKind::CheckedCtor(_, _, _)
            | RPatKind::Record(_)
            | RPatKind::Literal(_, _) => {
                return Err(ElabError::TypeMismatch {
                    span: row.real_pats[0].span.clone(),
                    reason: "non-tuple pattern cannot match a pair component".into(),
                });
            }
            RPatKind::As(_, _, _) => {
                unreachable!("current-column aliases are exposed before tuple splitting")
            }
            RPatKind::Or(_) => {
                unreachable!("current-column or-patterns are expanded before tuple splitting")
            }
        }
    }

    let mut component_types = if current_is_live {
        vec![*domain, *codomain]
    } else {
        vec![weaken(&domain, 1), shift(&codomain, 1, 1)]
    };
    component_types.extend_from_slice(&col_types[1..]);
    let mut component_kinds = vec![ColKind::Real, ColKind::Real];
    component_kinds.extend_from_slice(&col_kinds[1..]);
    let continuation = compile_match_matrix(
        cx,
        arms,
        &component_types,
        &component_kinds,
        component_rows,
        real_depth_so_far,
        top_span,
        root_frame_depth,
        ret_ty_slot,
        arm_used,
        subsumed_by,
    )?;
    let ret_ty = ret_ty_slot
        .as_ref()
        .expect("tuple component compilation reaches a body leaf")
        .clone();
    let continuation_ty = tail_codomain(
        cx,
        &component_types,
        &component_kinds,
        &ret_ty,
        real_depth_so_far + usize::from(!current_is_live),
    )?;
    let continuation = Term::Ascript(Box::new(continuation), Box::new(continuation_ty));
    let projected = Term::app(
        Term::app(continuation, Term::proj1(pair_occurrence.clone())),
        Term::proj2(pair_occurrence),
    );
    if current_is_live {
        Ok(projected)
    } else {
        Ok(Term::lam(col_types[0].clone(), projected))
    }
    })();
    if !current_is_live {
        cx.hidden_positions.pop();
        cx.ctx.pop();
    }
    result
}

struct RecordPatternProjection {
    owner_name: String,
    field_names: Vec<String>,
    field_types: Vec<Term>,
}

fn record_pattern_projection(
    cx: &ElabCtx<'_>,
    record_ty: &Term,
    span: &Span,
) -> Result<RecordPatternProjection, ElabError> {
    let (owner_id, head_arg) = match record_ty {
        Term::App(function, argument) => match function.as_ref() {
            Term::Const { id, .. } => (*id, Some((**argument).clone())),
            _ => {
                return Err(ElabError::TypeMismatch {
                    span: span.clone(),
                    reason: "record pattern requires a named record type".into(),
                })
            }
        },
        Term::Const { id, .. } => (*id, None),
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "record pattern requires a named record type".into(),
            })
        }
    };
    let class_env = cx.class_env.ok_or_else(|| ElabError::TypeMismatch {
        span: span.clone(),
        reason: "record patterns are unavailable in this elaboration context".into(),
    })?;
    let projection =
        class_env
            .projection_by_type_id(owner_id)
            .ok_or_else(|| ElabError::TypeMismatch {
                span: span.clone(),
                reason: "record pattern scrutinee is not a known named record type".into(),
            })?;
    let field_types = match (projection.head_param, head_arg) {
        (None, None) => projection.field_types.to_vec(),
        (Some(_), Some(head)) => projection
            .field_types
            .iter()
            .enumerate()
            .map(|(index, ty)| subst_outer(ty, 1, std::slice::from_ref(&head), index))
            .collect(),
        _ => {
            return Err(ElabError::TypeMismatch {
                span: span.clone(),
                reason: "record pattern scrutinee has an incomplete owner application".into(),
            })
        }
    };
    Ok(RecordPatternProjection {
        owner_name: projection.owner_name.to_string(),
        field_names: projection.field_names.to_vec(),
        field_types,
    })
}

fn normalize_record_row(
    fields: &[RRecordPatField],
    projection: &RecordPatternProjection,
    row_span: &Span,
) -> Result<(Vec<RPattern>, Vec<bool>), ElabError> {
    for field in fields {
        if !projection
            .field_names
            .iter()
            .any(|declared| declared == &field.label)
        {
            return Err(ElabError::UnresolvedCon {
                name: format!("{}.{}", projection.owner_name, field.label),
                span: field.label_span.clone(),
            });
        }
    }

    let mut patterns = Vec::with_capacity(projection.field_names.len());
    let mut source_bindings = Vec::with_capacity(projection.field_names.len());
    for name in &projection.field_names {
        if let Some(field) = fields.iter().find(|field| &field.label == name) {
            patterns.push(field.pattern.clone());
            source_bindings.push(true);
        } else {
            patterns.push(RPattern {
                kind: RPatKind::Wild,
                span: row_span.clone(),
            });
            source_bindings.push(false);
        }
    }
    Ok((patterns, source_bindings))
}

fn project_named_record_field(mut base: Term, index: usize) -> Term {
    for _ in 0..index {
        base = Term::proj2(base);
    }
    Term::proj1(base)
}

/// Project one named record column into declaration-order field columns and
/// compile those columns through the existing matrix continuation. Omitted
/// fields are generated wildcards; their continuation binders are hidden from
/// surface de Bruijn lookup and bind no source occurrence.
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn compile_record_column(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    col_types: &[Term],
    col_kinds: &[ColKind],
    mut rows: Vec<RowState>,
    real_depth_so_far: usize,
    top_span: &Span,
    root_frame_depth: usize,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Term, ElabError> {
    let projection = record_pattern_projection(cx, &col_types[0], top_span)?;
    let current_is_live = rows[0].real_occurrences[0].live;
    debug_assert!(rows
        .iter()
        .all(|row| row.real_occurrences[0].live == current_is_live));
    if !current_is_live {
        cx.push_match_binder(col_types[0].clone(), MatchBinderOrigin::UserLocal);
        cx.hidden_positions.push(cx.ctx.len() - 1);
        rows = rows
            .into_iter()
            .map(RowState::enter_current_real_binder)
            .map(|row| expose_current_pattern_aliases(cx, row, &col_types[0]))
            .collect();
    }
    let result = (|| {
    let record_occurrence = rows[0].real_occurrences[0].term.clone();

    let mut field_rows = Vec::with_capacity(rows.len());
    for row in rows {
        let row = expose_current_pattern_aliases(cx, row, &col_types[0]);
        match row.real_pats[0].kind.clone() {
            RPatKind::Record(fields) => {
                let (patterns, source_bindings) =
                    normalize_record_row(&fields, &projection, &row.real_pats[0].span)?;
                field_rows.push(row.specialize_projected_record_column(patterns, source_bindings));
            }
            RPatKind::Wild | RPatKind::Var(_, _) => {
                let span = row.real_pats[0].span.clone();
                let patterns = projection
                    .field_names
                    .iter()
                    .map(|_| RPattern {
                        kind: RPatKind::Wild,
                        span: span.clone(),
                    })
                    .collect::<Vec<_>>();
                let source_bindings = vec![false; patterns.len()];
                field_rows.push(
                    row.bind_current_occurrence()
                        .specialize_projected_record_column(patterns, source_bindings),
                );
            }
            RPatKind::Ctor(_, _)
            | RPatKind::CheckedCtor(_, _, _)
            | RPatKind::Tuple(_)
            | RPatKind::Literal(_, _) => {
                return Err(ElabError::TypeMismatch {
                    span: row.real_pats[0].span.clone(),
                    reason: "non-record pattern cannot match a named record component".into(),
                });
            }
            RPatKind::As(_, _, _) => {
                unreachable!("current-column aliases are exposed before record projection")
            }
            RPatKind::Or(_) => {
                unreachable!("current-column or-patterns are expanded before record projection")
            }
        }
    }

    let mut field_types = projection.field_types.iter().enumerate().map(|(index, ty)| {
        if current_is_live { ty.clone() } else { shift(ty, 1, index) }
    }).collect::<Vec<_>>();
    field_types.extend_from_slice(&col_types[1..]);
    let mut field_kinds = vec![ColKind::Real; projection.field_names.len()];
    field_kinds.extend_from_slice(&col_kinds[1..]);
    let continuation = compile_match_matrix(
        cx,
        arms,
        &field_types,
        &field_kinds,
        field_rows,
        real_depth_so_far,
        top_span,
        root_frame_depth,
        ret_ty_slot,
        arm_used,
        subsumed_by,
    )?;
    let ret_ty = ret_ty_slot
        .as_ref()
        .expect("record field compilation reaches a body leaf")
        .clone();
    let continuation_ty = tail_codomain(cx, &field_types, &field_kinds, &ret_ty,
        real_depth_so_far + usize::from(!current_is_live))?;
    let continuation = Term::Ascript(Box::new(continuation), Box::new(continuation_ty));
    let projected =
        projection
            .field_names
            .iter()
            .enumerate()
            .fold(continuation, |term, (index, _)| {
                Term::app(
                    term,
                    project_named_record_field(record_occurrence.clone(), index),
                )
            });
    if current_is_live {
        Ok(projected)
    } else {
        Ok(Term::lam(col_types[0].clone(), projected))
    }
    })();
    if !current_is_live {
        cx.hidden_positions.pop();
        cx.ctx.pop();
    }
    result
}

fn expand_top_or_pattern(pattern: RPattern) -> Vec<RPattern> {
    let span = pattern.span.clone();
    match pattern.kind {
        RPatKind::Or(alternatives) => alternatives
            .into_iter()
            .flat_map(expand_top_or_pattern)
            .collect(),
        RPatKind::As(inner, name, slot) => expand_top_or_pattern(*inner)
            .into_iter()
            .map(|alternative| RPattern {
                kind: RPatKind::As(Box::new(alternative), name.clone(), slot),
                span: span.clone(),
            })
            .collect(),
        kind => vec![RPattern { kind, span }],
    }
}

fn expanding_or_slots(pattern: &RPattern) -> Option<HashSet<usize>> {
    match &pattern.kind {
        RPatKind::Or(_) => {
            let mut slots = HashSet::new();
            collect_or_pattern_slots(pattern, false, &mut slots);
            Some(slots)
        }
        RPatKind::As(inner, _, _) => expanding_or_slots(inner),
        _ => None,
    }
}

#[inline(never)]
fn expand_current_or_rows(cx: &mut ElabCtx, rows: Vec<RowState>) -> Vec<RowState> {
    let mut expanded = Vec::new();
    for row in rows {
        if let Some(slots) = expanding_or_slots(&row.real_pats[0]) {
            let common_depth = cx.ctx.len();
            let frame = cx
                .pattern_alias_type_frames
                .last_mut()
                .expect("or-pattern expansion occurs inside an alias-type frame");
            for slot in slots {
                frame
                    .or_common_depths
                    .entry((row.arm_idx, slot))
                    .or_insert(common_depth);
            }
        }
        let alternatives = expand_top_or_pattern(row.real_pats[0].clone());
        for alternative in alternatives {
            let mut duplicate = row.clone();
            duplicate.real_pats[0] = alternative;
            expanded.push(duplicate);
        }
    }
    expanded
}

#[inline(never)]
fn prepare_current_or_rows(cx: &mut ElabCtx, rows: Vec<RowState>) -> Vec<RowState> {
    if rows
        .iter()
        .any(|row| expanding_or_slots(&row.real_pats[0]).is_some())
    {
        expand_current_or_rows(cx, rows)
    } else {
        rows
    }
}

/// Insert the discovery-skipped IH binders into one cached body, then check
/// the entire context inclusion before allowing a kernel-facing use.
fn reuse_matrix_first_leaf(
    cx: &mut ElabCtx<'_>,
    owner: usize,
    arm_idx: usize,
) -> Result<Term, ElabError> {
    let entry = cx.matrix_entries.get_mut(owner).ok_or_else(|| {
        ElabError::Internal("replayed leaf has no owning matrix frame".into())
    })?;
    let leaf = entry.first_leaf.take().ok_or_else(|| {
        ElabError::Internal("replayed leaf has no discovery body".into())
    })?;
    let prior_len = leaf.context.len();
    let replay_depths_match = entry.skipped_ih.len() == leaf.skipped_ih.len()
        && leaf.skipped_ih.iter().enumerate().all(|(ordinal, &depth)| {
            let earlier = leaf.skipped_ih[..ordinal].iter()
                .filter(|&&prior| prior <= depth).count();
            entry.skipped_ih[ordinal] == depth + earlier
        });
    if arm_idx != leaf.arm_idx
        || cx.ctx.len() != prior_len + leaf.skipped_ih.len()
        || !replay_depths_match
    {
        return Err(ElabError::Internal(
            "replayed leaf changed its arm or IH binder count".into(),
        ));
    }
    for (old_pos, old_domain) in leaf.context.types.iter().enumerate() {
        let mut thinned = old_domain.clone();
        for &depth in &leaf.skipped_ih {
            if depth <= old_pos {
                thinned = shift(&thinned, 1, old_pos - depth);
            }
        }
        let inserted = leaf.skipped_ih.iter().filter(|&&depth| depth <= old_pos).count();
        if cx.ctx.types.get(old_pos + inserted) != Some(&thinned) {
            return Err(ElabError::Internal(
                "replayed leaf changed a non-IH context entry".into(),
            ));
        }
    }
    let mut body = leaf.body;
    for &depth in &leaf.skipped_ih {
        if depth > prior_len {
            return Err(ElabError::Internal("recorded IH depth exceeds discovery context".into()));
        }
        body = shift(&body, 1, prior_len - depth);
    }
    entry.reused_leaf = true;
    Ok(body)
}

// An annotated postcondition checks the first leaf against its seeded motive.
// Keep this checked-only telescope work out of the ordinary inference leaf's
// stack frame; most nested matrix compilations carry no result predicate.
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn check_matrix_first_result_leaf(
    cx: &mut ElabCtx,
    first_arm: &RMatchArm,
    first_row: &RowState,
    first_occurrences: &[Option<Term>],
    real_depth_so_far: usize,
    owner: usize,
    ret_ty_slot: &Option<Term>,
) -> Result<(Option<Term>, Term, Term), ElabError> {
    let seed = ret_ty_slot.as_ref().ok_or_else(|| ElabError::Internal(
        "result predicate on an unseeded match leaf needs a checked motive".into(),
    ))?;
    let depth = matrix_telescope_depth(cx, cx.matrix_entries[owner].outer_ctx_len)?;
    let body_ty_ctx = weaken(seed, depth as i64);
    let (guard, body) = check_arm_at_matrix_leaf(
        cx, first_arm, first_row.arm_idx, first_occurrences, real_depth_so_far,
        &first_row.virtual_surface_positions, &first_row.virtual_aliases,
        &first_row.row_hidden_surface_positions, &body_ty_ctx,
    )?;
    Ok((guard, body, body_ty_ctx))
}

/// Compile one matrix leaf. Keeping guard-only vectors and conditionals in a
/// non-recursive frame preserves the existing recursive matrix stack budget.
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn compile_match_leaf(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    rows: &[RowState],
    real_depth_so_far: usize,
    top_span: &Span,
    root_frame_depth: usize,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Term, ElabError> {
    // Rows are in source order and every pattern matched this path. Guarded
    // rows may select but never close it. Or expansion can duplicate one
    // source arm at a leaf, so coalesce by arm id before building the chain.
    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    for row in rows {
        if seen.insert(row.arm_idx) {
            candidates.push(row);
        }
    }
    let fallback = candidates
        .iter()
        .position(|row| arms[row.arm_idx].guard.is_none())
        .ok_or_else(|| ElabError::ExhaustivenessError {
            missing: guarded_leaf_missing_witness(&arms[candidates[0].arm_idx].pat),
            span: top_span.clone(),
        })?;

    // Only the first unguarded fallback covers and subsumes. Every guarded
    // predecessor remains reachable because each may select its own body.
    for row in &candidates[..=fallback] {
        arm_used[row.arm_idx] = true;
    }
    let winner = candidates[fallback].arm_idx;
    for shadowed in &candidates[fallback + 1..] {
        if shadowed.arm_idx == winner {
            continue;
        }
        let entry = &mut subsumed_by[shadowed.arm_idx];
        if !entry.contains(&winner) {
            entry.push(winner);
        }
    }

    let first_row = candidates[0];
    let owner = cx.matrix_entries.len().checked_sub(1).ok_or_else(|| {
        ElabError::Internal("matrix leaf has no owning entry".into())
    })?;
    if cx.matrix_entries[owner].rerun && !cx.matrix_entries[owner].reused_leaf {
        if cx.matrix_entries[owner].root_frame_depth != root_frame_depth {
            return Err(ElabError::Internal("matrix leaf changed its owner".into()));
        }
        return reuse_matrix_first_leaf(cx, owner, first_row.arm_idx);
    }
    let first_occurrences = first_row.leaf_binding_occurrences().to_vec();
    let first_arm = &arms[first_row.arm_idx];
    let has_result_predicate = cx.match_frames.last()
        .is_some_and(|frame| !frame.result_predicates.is_empty());
    let (first_guard, first_body, body_ty_ctx) = if has_result_predicate {
        check_matrix_first_result_leaf(
            cx, first_arm, first_row, &first_occurrences, real_depth_so_far,
            owner, ret_ty_slot,
        )?
    } else {
        infer_arm_at_matrix_leaf(
            cx, first_arm, first_row.arm_idx, &first_occurrences, real_depth_so_far,
            &first_row.virtual_surface_positions, &first_row.virtual_aliases,
            &first_row.row_hidden_surface_positions,
        )?
    };
    let mut branches = vec![(first_row.arm_idx, first_guard, first_body)];
    for row in &candidates[1..=fallback] {
        let occurrences = row.leaf_binding_occurrences().to_vec();
        let (guard, body) = check_arm_at_matrix_leaf(
            cx,
            &arms[row.arm_idx],
            row.arm_idx,
            &occurrences,
            real_depth_so_far,
            &row.virtual_surface_positions,
            &row.virtual_aliases,
            &row.row_hidden_surface_positions,
            &body_ty_ctx,
        )?;
        branches.push((row.arm_idx, guard, body));
    }
    let (_, fallback_guard, mut body_core) = branches
        .pop()
        .expect("a covering leaf has an unguarded fallback");
    debug_assert!(fallback_guard.is_none());
    for (arm_idx, guard, body) in branches.into_iter().rev() {
        body_core = make_if_elim(
            cx,
            guard.expect("only the final covering branch is unguarded"),
            body,
            body_core,
            &body_ty_ctx,
            &arms[arm_idx].span,
        )?;
    }
    if ret_ty_slot.is_none() {
        let zonked = cx.metas.zonk_term(&body_ty_ctx);
        let derived_depth = matrix_telescope_depth(cx, cx.matrix_entries[owner].outer_ctx_len)?;
        let lowered = lower_by(&zonked, derived_depth).map_err(|index| {
            ElabError::InferredMatchResultEscapesPattern {
                match_span: top_span.clone(),
                arm_span: arms[first_row.arm_idx].span.clone(),
                escaping_binder: recover_escaping_pattern_binder(
                    &arms[first_row.arm_idx].pat,
                    &first_occurrences,
                    index,
                    derived_depth,
                ),
            }
        })?;
        if cx.matrix_entries[owner].discovery {
            let entry = &mut cx.matrix_entries[owner];
            if entry.root_frame_depth != root_frame_depth || entry.first_leaf.is_some() {
                return Err(ElabError::Internal("matrix discovery crossed its owner".into()));
            }
            entry.first_leaf = Some(MatrixFirstLeaf {
                body: body_core,
                result: lowered,
                arm_idx: first_row.arm_idx,
                context: cx.ctx.clone(),
                skipped_ih: entry.skipped_ih.clone(),
            });
            return Err(ElabError::MatrixResultDiscovered { owner });
        }
        memoize_indexed_root_motive(cx, &lowered, top_span, root_frame_depth)?;
        *ret_ty_slot = Some(lowered);
    }
    Ok(body_core)
}

/// Count only the live telescope of this matrix entry. Every binder introduced
/// while a match frame is active must have its origin and enclosing map at its
/// stable level; an absent entry is not repaired by guessing from the depth.
fn matrix_telescope_depth(cx: &ElabCtx<'_>, outer_len: usize) -> Result<usize, ElabError> {
    let end = cx.ctx.len();
    if outer_len > end {
        return Err(ElabError::Internal("matrix telescope escaped its entry context".into()));
    }
    for level in outer_len..end {
        cx.match_binder_origin(level)?;
    }
    Ok(end - outer_len)
}

/// Determine a nested eliminator's motive before opening its constructor
/// buckets. Its codomain is the pending continuation in the split binder's
/// context; `method_type` then provides every bucket's exact domains.
#[allow(clippy::too_many_arguments)]
fn nested_matrix_motive(
    cx: &ElabCtx<'_>,
    ind: &InductiveDecl,
    family: GlobalId,
    level_args: &[Level],
    params: &[Term],
    col_types: &[Term],
    col_kinds: &[ColKind],
    result: &Term,
    split_span: &Span,
    top_span: &Span,
) -> Result<(Term, bool), ElabError> {
    let entry = cx.matrix_entries.last().ok_or_else(|| {
        ElabError::Internal("nested motive has no owning matrix entry".into())
    })?;
    let derived_depth = matrix_telescope_depth(cx, entry.outer_ctx_len)?;
    let codomain = tail_codomain(
        cx, &col_types[1..], &col_kinds[1..], result, derived_depth + 1,
    )?;
    let indices_under_split = params[ind.params.len()..]
        .iter().map(|index| weaken(index, 1)).collect::<Vec<_>>();
    let (dependent_tail, result_mentions_split) = nested_split_dependencies(
        &codomain, col_types.len() - 1, &indices_under_split,
    )?;
    let needs_reverting = !dependent_tail.is_empty() || result_mentions_split;
    if !needs_reverting {
        debug_assert!(!scrut_occurs(&codomain, &Term::var(0)));
    } else {
        check_nested_index_variables(cx, &params[ind.params.len()..], split_span)?;
    }
    let ret_sort = if needs_reverting {
        let mut motive_ctx = Context {
            types: cx.ctx.types.iter().map(|ty| cx.metas.zonk_term(ty)).collect(),
        };
        motive_ctx.push(cx.metas.zonk_term(&col_types[0]));
        kernel_infer_in_context_current(cx, &motive_ctx, &cx.metas.zonk_term(&codomain))
    } else {
        kernel_infer_current(cx, &codomain)
    };
    let ret_level = match ret_sort {
        Ok(Term::Type(level)) => level,
        Ok(_) => Level::Zero,
        Err(CurrentKernelQueryError::View(error)) => return Err(error),
        Err(CurrentKernelQueryError::Kernel(_))
            if !needs_reverting && cx.active_index_premise_frames.is_empty() => Level::Zero,
        Err(CurrentKernelQueryError::Kernel(error)) => {
            return Err(ElabError::KernelRejected { error, span: top_span.clone() });
        }
    };
    let args = params.iter().map(|param| weaken(param, 1)).collect::<Vec<_>>();
    let scrut_ty_under_split = weaken(&col_types[0], 1);
    let elim = matrix_family_elim(
        ind, family, level_args, &args, &scrut_ty_under_split,
        shift(&codomain, 1, 1), ret_level, Vec::new(), Term::var(0),
        needs_reverting,
    );
    let Term::Elim { motive, .. } = elim else { unreachable!() };
    Ok((*motive, needs_reverting))
}

/// Compile the pattern matrix `col_types`/`col_kinds` (aligned; `Real`
/// columns are matched against `rows[_].real_pats`, `Ih` columns are
/// synthetic and never touch row patterns) down to a nested-`elim_D` method
/// term, per the standard column-by-column algorithm.
///
/// `real_depth_so_far` counts surface-bound flat `Real` columns, excluding
/// split and IH binders. All emitted matrix binders are real `cx.ctx` pushes;
/// split and IH entries are hidden from surface de Bruijn lookup. This count
/// tracks the resolver's flat binding order independently of context depth.
fn compile_match_matrix(
    cx: &mut ElabCtx,
    arms: &[RMatchArm],
    col_types: &[Term],
    col_kinds: &[ColKind],
    rows: Vec<RowState>,
    real_depth_so_far: usize,
    top_span: &Span,
    root_frame_depth: usize,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
) -> Result<Term, ElabError> {
    if col_types.is_empty() {
        return compile_match_leaf(
            cx,
            arms,
            &rows,
            real_depth_so_far,
            top_span,
            root_frame_depth,
            ret_ty_slot,
            arm_used,
            subsumed_by,
        );
    }

    match col_kinds[0] {
        ColKind::Ih => {
            if ret_ty_slot.is_none() {
                let entry = cx.matrix_entries.last_mut().ok_or_else(|| {
                    ElabError::Internal("IH discovery has no owning matrix".into())
                })?;
                if entry.root_frame_depth != root_frame_depth || entry.rerun {
                    return Err(ElabError::Internal("IH discovery changed its matrix owner".into()));
                }
                entry.discovery = true;
                entry.skipped_ih.push(cx.ctx.len());
                // The method telescope still includes this IH, but discovery
                // never pushes it. Remove its coordinate from each later
                // column relative to the columns preceding that column.
                let tail = col_types[1..]
                    .iter()
                    .enumerate()
                    .map(|(offset, ty)| lower_binders(ty, 1, offset))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| {
                        ElabError::Internal("a later matrix column depends on a skipped IH".into())
                    })?;
                return compile_match_matrix(
                    cx, arms, &tail, &col_kinds[1..], rows,
                    real_depth_so_far, top_span, root_frame_depth,
                    ret_ty_slot, arm_used, subsumed_by,
                );
            }
            // An IH is hidden from surface names but real in the method's
            // derived context. Its type was read from `method_type` before
            // any leaf in this bucket was elaborated.
            let ih_ty = col_types[0].clone();
            if let Some(entry) = cx.matrix_entries.last_mut() {
                if entry.rerun && !entry.reused_leaf {
                    entry.skipped_ih.push(cx.ctx.len());
                }
            }
            cx.push_match_binder(ih_ty.clone(), MatchBinderOrigin::Ih);
            cx.hidden_positions.push(cx.ctx.len() - 1);
            let rows = rows.into_iter().map(RowState::under_core_binder).collect();
            let inner = compile_match_matrix(
                cx, arms, &col_types[1..], &col_kinds[1..], rows,
                real_depth_so_far, top_span, root_frame_depth,
                ret_ty_slot, arm_used, subsumed_by,
            );
            let hidden = cx.hidden_positions.pop();
            debug_assert_eq!(hidden, Some(cx.ctx.len() - 1));
            cx.ctx.pop();
            Ok(Term::lam(ih_ty, inner?))
        }
        ColKind::Real => {
            // An or-pattern duplicates only its residual row. Every duplicate
            // retains the same arm id and the same aligned occurrence, so the
            // existing leaf winner accounting computes union coverage and
            // whole-arm reachability without a parallel matrix carrier.
            let rows = prepare_current_or_rows(cx, rows);
            let has_literal = rows.iter().any(|row| {
                matches!(
                    pattern_without_aliases(&row.real_pats[0]).kind,
                    RPatKind::Literal(_, _)
                )
            });
            if has_literal {
                return compile_literal_column(
                    cx,
                    arms,
                    col_types,
                    col_kinds,
                    rows,
                    real_depth_so_far,
                    top_span,
                    root_frame_depth,
                    ret_ty_slot,
                    arm_used,
                    subsumed_by,
                );
            }

            let has_record = rows.iter().any(|row| {
                matches!(
                    pattern_without_aliases(&row.real_pats[0]).kind,
                    RPatKind::Record(_)
                )
            });
            if has_record {
                return compile_record_column(
                    cx,
                    arms,
                    col_types,
                    col_kinds,
                    rows,
                    real_depth_so_far,
                    top_span,
                    root_frame_depth,
                    ret_ty_slot,
                    arm_used,
                    subsumed_by,
                );
            }

            let has_tuple = rows.iter().any(|row| {
                matches!(
                    pattern_without_aliases(&row.real_pats[0]).kind,
                    RPatKind::Tuple(_)
                )
            });
            if has_tuple {
                return compile_tuple_column(
                    cx,
                    arms,
                    col_types,
                    col_kinds,
                    rows,
                    real_depth_so_far,
                    top_span,
                    root_frame_depth,
                    ret_ty_slot,
                    arm_used,
                    subsumed_by,
                );
            }

            let all_flat = rows.iter().all(|row| {
                matches!(
                    pattern_without_aliases(&row.real_pats[0]).kind,
                    RPatKind::Wild | RPatKind::Var(_, _)
                )
            });
            if all_flat {
                // No constructor pattern in this column across any row: bind
                // it flatly (a real `cx.ctx` push), matching the resolver's
                // count exactly, and move on.
                let surface_binder = rows[0].real_occurrences[0].surface_binder;
                debug_assert!(rows
                    .iter()
                    .all(|row| row.real_occurrences[0].surface_binder == surface_binder));
                let origin = rows[0].real_occurrences[0].origin;
                if rows.iter().any(|row| row.real_occurrences[0].origin != origin) {
                    return Err(ElabError::Internal(
                        "matrix flat column has inconsistent binder origins".into(),
                    ));
                }
                cx.push_match_binder(col_types[0].clone(), origin);
                if !surface_binder {
                    cx.hidden_positions.push(cx.ctx.len() - 1);
                }
                let current_ty = weaken(&col_types[0], 1);
                let visible_position = cx.ctx.len() - 1;
                let new_rows: Vec<RowState> = rows
                    .into_iter()
                    .map(|mut row| {
                        if surface_binder && !row.real_occurrences[0].source_binding {
                            row.row_hidden_surface_positions.push(visible_position);
                        }
                        row.enter_current_real_binder()
                    })
                    .map(|row| expose_current_pattern_aliases(cx, row, &current_ty))
                    .map(|row| row.bind_current_occurrence().drop_current_column())
                    .collect();
                let inner = compile_match_matrix(
                    cx,
                    arms,
                    &col_types[1..],
                    &col_kinds[1..],
                    new_rows,
                    real_depth_so_far + 1,
                    top_span,
                    root_frame_depth,
                    ret_ty_slot,
                    arm_used,
                    subsumed_by,
                );
                if !surface_binder {
                    let hidden = cx.hidden_positions.pop();
                    debug_assert_eq!(hidden, Some(cx.ctx.len() - 1));
                }
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

            let split_span = rows
                .first()
                .map(|row| row.real_pats[0].span.clone())
                .unwrap_or_else(|| top_span.clone());
            let split_depth = cx.ctx.len();
            let rows: Vec<RowState> = rows
                .into_iter()
                .map(|mut row| {
                    let pattern = pattern_without_aliases(&row.real_pats[0]);
                    let split_name = match &pattern.kind {
                        RPatKind::Var(name, None) if name != "_" => Some(name.clone()),
                        _ => None,
                    };
                    let virtual_slot = if row.real_occurrences[0].surface_binder && matches!(
                        pattern.kind,
                        RPatKind::Wild | RPatKind::Var(_, _)
                    ) {
                        let slot = row.virtual_surface_positions.len();
                        row.virtual_surface_positions.push(split_depth);
                        Some(slot)
                    } else {
                        None
                    };
                    row = row.enter_current_real_binder();
                    if let (Some(slot), Some(name)) = (virtual_slot, split_name) {
                        row.virtual_aliases.push(MatrixVirtualAlias {
                            slot,
                            name,
                            occurrence: row.real_occurrences[0].term.clone(),
                            ty: col_types[0].clone(),
                            install_depth: split_depth,
                        });
                    }
                    row
                })
                .map(|row| expose_current_pattern_aliases(cx, row, &col_types[0]))
                .collect();
            #[cfg(test)]
            MATCH_OCCURRENCE_TRACE.with(|trace| {
                if let Some(trace) = trace.borrow_mut().as_mut() {
                    trace.nested_return_types.push(ret_ty_slot.is_some());
                }
            });
            // The split variable is a real, hidden context binder throughout
            // each derived method. Aborting discovery unwinds it before the
            // owning match decides whether to rerun.
            cx.push_match_binder(col_types[0].clone(), MatchBinderOrigin::Scrutinee);
            cx.hidden_positions.push(cx.ctx.len() - 1);
            let predicates = cx.match_frames.last()
                .map(|frame| frame.result_predicates.clone()).unwrap_or_default();
            let raw_methods_result = build_ctor_buckets(
                cx, arms, &ind0, d_id0, m0, &params0, rows,
                &col_types[1..], &col_kinds[1..], real_depth_so_far,
                top_span, root_frame_depth, ret_ty_slot, arm_used, subsumed_by,
                false, false, true,
                match &head {
                    Term::IndFormer { level_args, .. } => level_args,
                    _ => unreachable!("nested split head is an inductive former"),
                },
                Some(&col_types[0]), Some(&split_span), &predicates,
            );
            let hidden = cx.hidden_positions.pop();
            debug_assert_eq!(hidden, Some(cx.ctx.len() - 1));
            cx.ctx.pop();
            let raw_methods: Vec<Term> = raw_methods_result?
                .into_iter()
                .map(|method| method.expect("nested constructor coverage checked in bucket builder"))
                .collect();

            let ret_ty_base = ret_ty_slot.as_ref().ok_or_else(|| {
                ElabError::Internal("nested split finished without a result type".into())
            })?;
            let Term::IndFormer { level_args, .. } = head else {
                unreachable!("nested split has an inductive scrutinee")
            };
            let (motive, _needs_reverting) = nested_matrix_motive(
                cx, &ind0, d_id0, &level_args, &params0,
                col_types, col_kinds, ret_ty_base,
                &split_span, top_span,
            )?;
            let args = params0.iter().map(|p| weaken(p, 1)).collect::<Vec<_>>();
            let mut elim = Term::Elim {
                fam: d_id0,
                level_args: level_args.clone(),
                params: args[..m0].to_vec(),
                motive: Box::new(motive),
                methods: raw_methods,
                indices: args[m0..].to_vec(),
                scrut: Box::new(Term::var(0)),
            };
            {
                let Term::Elim { motive, methods, .. } = &mut elim else {
                    unreachable!("shared matrix constructor returns an eliminator")
                };
                let params = &args[..m0];
                let mut nested_ctx = Context {
                    types: cx.ctx.types.iter().map(|ty| cx.metas.zonk_term(ty)).collect(),
                };
                nested_ctx.push(cx.metas.zonk_term(&col_types[0]));
                for (ordinal, (ctor, method)) in ind0.constructors.iter()
                    .zip(methods.iter()).enumerate()
                {
                    let expected = method_type(
                        cx.env, &ind0, ordinal, motive, params, &level_args,
                    ).map_err(|error| ElabError::KernelRejected {
                        error,
                        span: split_span.clone(),
                    })?;
                    let ih_count = recursive_shapes(cx.env, ctor, d_id0, m0)
                        .map_err(|error| ElabError::KernelRejected {
                            error,
                            span: split_span.clone(),
                        })?
                        .len();
                    let binder_count = ctor.args.len() + ih_count + col_types.len() - 1;
                    let ih_positions = nested_method_ih_positions(&expected, motive, binder_count);
                    assert_nested_method_alignment(
                        &ind0,
                        ordinal,
                        &ih_positions,
                        ctor.args.len(),
                        ih_count,
                        col_types.len() - 1,
                        binder_count,
                    )?;
                    let checked = cx.metas.zonk_term(method);
                    let expected_checked = cx.metas.zonk_term(&expected);
                    kernel_check_in_context_current(cx, &nested_ctx, &checked, &expected_checked)
                        .map_err(|error| match error {
                            CurrentKernelQueryError::View(error) => error,
                            CurrentKernelQueryError::Kernel(error) => ElabError::KernelRejected {
                                error, span: split_span.clone(),
                            },
                        })?;
                }
            }
            Ok(Term::lam(col_types[0].clone(), elim))
        }
    }
}

// Build the per-bucket equation before descending into the recursive matrix.
// Its temporary constructor/type terms must not enlarge every live matrix
// frame on a deeply nested match when no result predicate is in scope.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn push_matrix_constructor_path_condition(
    cx: &mut ElabCtx,
    split_scrut: &Term,
    d_id0: GlobalId,
    split_level_args: &[Level],
    params0: &[Term],
    m0: usize,
    n_args0: usize,
    tail_under_split: bool,
    constructor_id: GlobalId,
) -> usize {
    let mut split_ty = Term::indformer(d_id0, split_level_args.to_vec());
    for arg in params0 {
        let arg = if tail_under_split { weaken(arg, 1) } else { arg.clone() };
        split_ty = Term::app(split_ty, arg);
    }
    let mut concrete = Term::constructor(constructor_id, split_level_args.to_vec());
    for param in params0.iter().take(m0) {
        concrete = Term::app(concrete,
            weaken(param, (n_args0 + usize::from(tail_under_split)) as i64));
    }
    for position in 0..n_args0 {
        concrete = Term::app(concrete, Term::var(n_args0 - 1 - position));
    }
    push_branch_path_condition(cx, &split_ty, split_scrut, &concrete, n_args0, n_args0)
}

/// Group `rows` (whose `real_pats[0]` matches the inductive `ind0`) into one
/// bucket per constructor — expanding a `Wild`/`Var` row into every
/// constructor (it matches all of them) — and recurse to build each
/// constructor's raw method term: `λ(fields). λ(IHs). <continuation>`,
/// where `<continuation>` threads through `tail_col_types`/`tail_col_kinds`
/// (the columns after this one). Nested buckets elaborate under the real
/// split binder, which their caller closes after checking the methods.
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
    root_frame_depth: usize,
    ret_ty_slot: &mut Option<Term>,
    arm_used: &mut [bool],
    subsumed_by: &mut [Vec<usize>],
    allow_index_omission: bool,
    indexed_root: bool,
    tail_under_split: bool,
    split_level_args: &[Level],
    split_column_type: Option<&Term>,
    split_span: Option<&Span>,
    predicates: &[ResultPredicate],
) -> Result<Vec<Option<Term>>, ElabError> {
    let mut methods: Vec<Option<Term>> = vec![None; ind0.constructors.len()];
    let mut nested_motive: Option<Term> = None;

    for (k0, c0) in ind0.constructors.iter().enumerate() {
        let mut bucket: Vec<RowState> = Vec::new();
        for r in &rows {
            r.assert_occurrence_alignment();
            debug_assert!(
                r.real_occurrences[0].live,
                "a constructor split must receive the current column's live occurrence"
            );
            match &r.real_pats[0].kind {
                RPatKind::Ctor(_, subs) | RPatKind::CheckedCtor(_, _, subs) => {
                    if pattern_ctor_id(cx, &r.real_pats[0].kind) == Some(c0.id) {
                        bucket.push(r.clone().specialize_current_column(subs.clone(), true));
                    }
                }
                RPatKind::Wild | RPatKind::Var(_, _) => {
                    let span = r.real_pats[0].span.clone();
                    let new_pats: Vec<RPattern> = (0..c0.args.len())
                        .map(|_| RPattern {
                            kind: RPatKind::Wild,
                            span: span.clone(),
                        })
                        .collect();
                    bucket.push(
                        r.clone()
                            .bind_current_occurrence()
                            .specialize_current_column(new_pats, false),
                    );
                }
                RPatKind::Tuple(_) | RPatKind::Record(_) => {
                    unreachable!("negative columns are projected before constructor bucketing")
                }
                RPatKind::Literal(_, _) => {
                    unreachable!("literal columns are compiled before constructor bucketing")
                }
                RPatKind::As(_, _, _) => {
                    unreachable!("current-column aliases are exposed before constructor bucketing")
                }
                RPatKind::Or(_) => {
                    unreachable!(
                        "current-column or-patterns are expanded before constructor bucketing"
                    )
                }
            }
        }

        let n_args0 = c0.args.len();
        if bucket.is_empty() {
            if allow_index_omission {
                continue;
            }
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

        // Every solved bucket reads its complete fields/IHs/tail telescope
        // from the eliminator's method type before any leaf is opened.
        // If the first bucket reaches a leaf before discovering R, its tail
        // still needs the index-specialized domains. A domain-only motive
        // derives those domains; it is never emitted or saved. The real
        // motive is constructed as soon as R is known.
        let mut domain_only_motive = None;
        if tail_under_split && nested_motive.is_none() {
            let split_ty = split_column_type.ok_or_else(|| {
                ElabError::Internal("nested bucket has no split column".into())
            })?;
            let split_span = split_span.ok_or_else(|| {
                ElabError::Internal("nested bucket has no split span".into())
            })?;
            let saved_len = cx.ctx.len();
            let saved_hidden = cx.hidden_positions.len();
            let hidden = cx.hidden_positions.pop();
            debug_assert_eq!(hidden, Some(cx.ctx.len() - 1));
            let binder = cx.ctx.pop().expect("nested split binder remains installed");
            let mut columns = vec![split_ty.clone()];
            columns.extend_from_slice(tail_col_types);
            let mut kinds = vec![ColKind::Real];
            kinds.extend_from_slice(tail_col_kinds);
            let domain_result = Term::ty(Level::Zero);
            let calculated = nested_matrix_motive(
                cx, ind0, d_id0, split_level_args, params0, &columns,
                &kinds, ret_ty_slot.as_ref().unwrap_or(&domain_result),
                split_span, top_span,
            );
            cx.push_match_binder(binder, MatchBinderOrigin::Scrutinee);
            cx.hidden_positions.push(cx.ctx.len() - 1);
            debug_assert_eq!(cx.ctx.len(), saved_len);
            debug_assert_eq!(cx.hidden_positions.len(), saved_hidden);
            debug_assert_eq!(cx.hidden_positions.last(), Some(&(saved_len - 1)));
            if ret_ty_slot.is_some() {
                nested_motive = Some(calculated?.0);
            } else {
                domain_only_motive = Some(calculated?.0);
            }
        }
        let motive = if let Some(motive) = nested_motive.as_ref().or(domain_only_motive.as_ref()) {
            Some(motive.clone())
        } else if indexed_root {
            cx.indexed_match_roots.last().and_then(|root| root.motive.as_deref()).cloned()
        } else if !tail_under_split {
            ret_ty_slot.as_ref().map(|ret| {
                let scrut_ty = params0.iter().cloned().fold(
                    Term::indformer(d_id0, split_level_args.to_vec()), Term::app,
                );
                let ret_level = match kernel_infer_current(cx, ret) {
                    Ok(Term::Type(level)) => level,
                    _ => Level::Zero,
                };
                let elim = matrix_family_elim(
                    ind0, d_id0, split_level_args, params0, &scrut_ty,
                    weaken(ret, 1), ret_level, Vec::new(), Term::var(0), false,
                );
                let Term::Elim { motive, .. } = elim else { unreachable!() };
                *motive
            })
        } else {
            None
        };
        let mut new_col_types = if let Some(motive) = motive.as_ref() {
            let params = if tail_under_split {
                params0[..m0].iter().map(|p| weaken(p, 1)).collect::<Vec<_>>()
            } else {
                params0[..m0].to_vec()
            };
            let mut method_ty = method_type(
                cx.env, ind0, k0, motive, &params, split_level_args,
            ).map_err(|error| ElabError::KernelRejected {
                error, span: top_span.clone(),
            })?;
            let binder_count = n_args0 + p_ihs0 + if tail_under_split { tail_col_types.len() } else { 0 };
            let mut domains = Vec::with_capacity(binder_count);
            let mut telescope_ctx = cx.ctx.clone();
            for _ in 0..binder_count {
                let Term::Pi(domain, rest) = whnf(cx.env, &telescope_ctx, &method_ty) else {
                    return Err(ElabError::Internal(
                        "derived method lost its field/IH telescope".into(),
                    ));
                };
                telescope_ctx.push((*domain).clone());
                domains.push(*domain);
                method_ty = *rest;
            }
            domains
        } else {
            if tail_under_split {
                return Err(ElabError::Internal("split tail has no derived telescope".into()));
            }
            let mut types = field_types0;
            types.extend(std::iter::repeat(Term::ty(Level::Zero)).take(p_ihs0));
            types
        };
        if !tail_under_split {
            new_col_types.extend_from_slice(tail_col_types);
        }
        let mut new_col_kinds: Vec<ColKind> = vec![ColKind::Real; n_args0];
        new_col_kinds.extend(std::iter::repeat(ColKind::Ih).take(p_ihs0));
        new_col_kinds.extend_from_slice(tail_col_kinds);

        let base = cx.ctx.len();
        cx.match_frames.push(MatchFrame::new(
            base, cx.match_frames.len(), ret_ty_slot.clone(), None,
        ));
        cx.match_frames.last_mut().expect("new matrix frame")
            .result_predicates.extend_from_slice(predicates);
        // The occurrence is the matched source value at the root, and the
        // installed split binder at a nested constructor column.
        let path_base = push_matrix_constructor_path_condition(
            cx, &rows[0].real_occurrences[0].term, d_id0, split_level_args,
            params0, m0, n_args0, tail_under_split, c0.id,
        );
        let result = compile_match_matrix(
            cx,
            arms,
            &new_col_types,
            &new_col_kinds,
            bucket,
            real_depth_so_far,
            top_span,
            root_frame_depth,
            ret_ty_slot,
            arm_used,
            subsumed_by,
        );
        cx.path_conditions.truncate(path_base);
        cx.match_frames.pop();
        let inner = result?;
        methods[k0] = Some(inner);
    }

    Ok(methods)
}

/// One constructor for every matrix-compiler family eliminator. Even a
/// constant motive has the full family-index telescope. A nested dependent
/// split instead abstracts its actual indices into the motive when requested.
/// `motive_body` lives under its scrutinee binder at Var(0): insert the
/// index binders above free variables, never above the scrutinee.
#[allow(clippy::too_many_arguments)]
fn matrix_family_elim(
    ind: &InductiveDecl,
    family: GlobalId,
    level_args: &[Level],
    args: &[Term],
    scrut_ty: &Term,
    motive_body: Term,
    level: Level,
    methods: Vec<Term>,
    scrut: Term,
    abstract_indices: bool,
) -> Term {
    let (params, indices) = args.split_at(ind.params.len());
    debug_assert_eq!(indices.len(), ind.indices.len());
    let sort = Term::ty(level);
    let motive = if indices.is_empty() {
        // This is exactly the previous non-indexed matrix motive.
        Term::Ascript(
            Box::new(Term::lam(scrut_ty.clone(), motive_body)),
            Box::new(Term::pi(scrut_ty.clone(), sort)),
        )
    } else {
        let mut body = shift(&motive_body, indices.len() as i64, 1);
        if abstract_indices {
            let substitutions: Vec<(Term, Term)> = indices
                .iter()
                .enumerate()
                .map(|(j, actual)| {
                    (
                        weaken(actual, (indices.len() + 1) as i64),
                        Term::var(indices.len() - j),
                    )
                })
                .collect();
            body = subst_term_generalize_many(&body, &substitutions);
        }
        Term::Ascript(
            Box::new(wrap_motive_lambdas_at(ind, family, params, body, level_args)),
            Box::new(motive_type_at(ind, family, params, &sort, level_args)),
        )
    };
    Term::Elim {
        fam: family,
        level_args: level_args.to_vec(),
        params: params.to_vec(),
        motive: Box::new(motive),
        methods,
        indices: indices.to_vec(),
        scrut: Box::new(scrut),
    }
}

/// Locate the IH domains in method_type's binder spine by the motive
/// application they contain. W-style and nested-positive lifts may carry
/// that application under Π or Σ binders; their containing outer binder is
/// still the IH position in the method's telescope.
fn nested_method_ih_positions(expected: &Term, motive: &Term, binder_count: usize) -> Vec<usize> {
    fn contains_motive(term: &Term, motive: &Term, depth: usize) -> bool {
        let (head, args) = peel_app(term);
        if !args.is_empty() && head == weaken(motive, depth as i64) {
            return true;
        }
        match term {
            Term::Pi(domain, body) | Term::Sigma(domain, body) | Term::Lam(domain, body) => {
                contains_motive(domain, motive, depth)
                    || contains_motive(body, motive, depth + 1)
            }
            Term::Let { ty, val, body } => {
                contains_motive(ty, motive, depth)
                    || contains_motive(val, motive, depth)
                    || contains_motive(body, motive, depth + 1)
            }
            _ => term.children().into_iter().any(|child| contains_motive(child, motive, depth)),
        }
    }

    let mut positions = Vec::new();
    let mut cursor = expected;
    for position in 0..binder_count {
        let Term::Pi(domain, rest) = cursor else { break };
        if contains_motive(domain, motive, position) {
            positions.push(position);
        }
        cursor = rest;
    }
    positions
}

fn assert_nested_method_alignment(
    ind: &InductiveDecl,
    ordinal: usize,
    ih_positions_in_method_type: &[usize],
    n_args0: usize,
    p_ihs0: usize,
    tail_len: usize,
    binder_count: usize,
) -> Result<(), ElabError> {
    let ctor = &ind.constructors[ordinal];
    let grouped: Vec<usize> = (ctor.args.len()..ctor.args.len() + p_ihs0).collect();
    if n_args0 != ctor.args.len()
        || p_ihs0 != ih_positions_in_method_type.len()
        || ih_positions_in_method_type != grouped.as_slice()
        || n_args0 + p_ihs0 + tail_len != binder_count
    {
        return Err(ElabError::Internal(format!(
            "nested method telescope misaligned for constructor {ordinal}: raw \
             fields/IHs/tail {n_args0}/{p_ihs0}/{tail_len}, method_type IH positions \
             {ih_positions_in_method_type:?}"
        )));
    }
    Ok(())
}

/// Close one root matrix method against the kernel's constructor method type.
/// Matrix leaves do not use the generated index evidence: only the outer
/// constructor method gains the premise lambdas. Reuse the kernel's own IH
/// domains, since the premise-carrying motive changes their result type too.
fn close_inferred_index_method(
    method: Option<Term>,
    method_ty: Term,
    premise_domains: &[Term],
    field_count: usize,
    ih_count: usize,
    sentinel_region: usize,
    _split_span: &Span,
) -> Result<(Option<Term>, Vec<Term>), ElabError> {
    let mut domains = Vec::with_capacity(field_count + ih_count);
    let mut tail_ty = method_ty;
    for _ in 0..field_count + ih_count {
        let Term::Pi(domain, codomain) = tail_ty else {
            return Err(ElabError::Internal(
                "inferred indexed constructor method lost a field/IH domain".into(),
            ));
        };
        domains.push(*domain);
        tail_ty = *codomain;
    }
    let method = if let Some(mut method) = method {
        let mut peeled = 0;
        while peeled < domains.len() {
            let Term::Lam(_, body) = method else { break };
            method = *body;
            peeled += 1;
        }
        // A nested split can leave an eliminator of the remaining Π type
        // rather than a syntactic lambda. Apply it to the method telescope's
        // remaining field/IH binders under those binders before closing the
        // generated index premises and re-wrapping the full telescope.
        let missing = domains.len() - peeled;
        let body = (0..missing).rev().fold(weaken(&method, missing as i64), |f, i| {
            Term::app(f, Term::var(i))
        });
        let premises_under_ih: Vec<_> = premise_domains
            .iter()
            .map(|premise| weaken(premise, ih_count as i64))
            .collect();
        let mut closed = wrap_premise_lams_finalized(body, &premises_under_ih, sentinel_region);
        for domain in domains.iter().rev() {
            closed = Term::lam(domain.clone(), closed);
        }
        Some(closed)
    } else {
        // The caller supplies the omitted constructor's Absurd method after
        // checking whether one of these very premises proves Bottom.
        None
    };
    Ok((method, domains))
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn finish_inferred_indexed_match(
    cx: &mut ElabCtx,
    ind: &InductiveDecl,
    family: GlobalId,
    level_args: &[Level],
    params: &[Term],
    scrut_indices: &[Term],
    raw_methods: Vec<Option<Term>>,
    result_ty: &Term,
    scrut_core: Term,
    span: &Span,
) -> Result<Term, ElabError> {
    let sentinel_region = cx.match_frames.len();
    let motive = cx
        .indexed_match_roots
        .last()
        .and_then(|root| root.motive.clone())
        .ok_or_else(|| ElabError::Internal(
            "indexed inferred match has no memoized root motive".into(),
        ))?;
    let mut methods = Vec::with_capacity(ind.constructors.len());
    for (ordinal, (ctor, raw)) in ind.constructors.iter().zip(raw_methods).enumerate() {
        let field_count = ctor.args.len();
        let ih_count = recursive_shapes(cx.env, ctor, family, ind.params.len())
            .map_err(|error| ElabError::KernelRejected {
                error,
                span: span.clone(),
            })?
            .len();
        let targets = ctor_target_indices(ctor, ind, params, level_args, field_count);
        let premises = method_index_premises(ind, params, &targets, scrut_indices, field_count);
        let expected_method = method_type(cx.env, ind, ordinal, &motive, params, level_args)
            .map_err(|error| ElabError::KernelRejected {
                error,
                span: span.clone(),
            })?;
        let (method, domains) = close_inferred_index_method(
            raw,
            expected_method,
            &premises,
            field_count,
            ih_count,
            sentinel_region,
            span,
        )?;
        if let Some(method) = method {
            methods.push(method);
            continue;
        }
        let old_depth = cx.ctx.len();
        cx.match_frames.push(MatchFrame::new(
            old_depth, sentinel_region, Some(result_ty.clone()), None,
        ));
        for (position, domain) in domains.iter().enumerate() {
            let origin = if position < field_count {
                MatchBinderOrigin::Field
            } else {
                MatchBinderOrigin::Ih
            };
            cx.push_match_binder(domain.clone(), origin);
        }
        let premises_under_ih: Vec<_> = premises
            .iter()
            .map(|premise| weaken(premise, ih_count as i64))
            .collect();
        let omitted = synthesize_omitted_index_method(
            cx,
            &premises_under_ih,
            &weaken(result_ty, (field_count + ih_count) as i64),
            sentinel_region,
            missing_pattern_witness(cx, ctor.id),
            span,
        );
        cx.ctx.types.truncate(old_depth);
        cx.match_frames.pop();
        let mut omitted = omitted?;
        for domain in domains.iter().rev() {
            omitted = Term::lam(domain.clone(), omitted);
        }
        methods.push(omitted);
    }
    let mut elim = Term::Elim {
        fam: family,
        level_args: level_args.to_vec(),
        params: params.to_vec(),
        motive,
        methods,
        indices: scrut_indices.to_vec(),
        scrut: Box::new(scrut_core),
    };
    let top_premises = method_index_premises(ind, params, scrut_indices, scrut_indices, 0);
    for premise in &top_premises {
        let proof = synth_generated_index_evidence(cx.env, &cx.ctx, premise, span)?;
        elim = Term::app(elim, proof);
    }
    Ok(elim)
}

/// Seed a check-mode matrix only when all levels in its goal are known.
/// Reading an unsolved level metavariable as Zero would prevent the first
/// leaf from solving it against the actual result type.
fn check_mode_result_seed(cx: &ElabCtx<'_>, expected: Option<&Term>) -> Option<Term> {
    let ty = expected?;
    cx.metas.defaulted.set(false);
    let zonked = cx.metas.zonk_term(ty);
    (!cx.metas.defaulted.replace(false)).then_some(zonked)
}

/// Run one matrix entry. Only its own first-leaf signal may restart the
/// descent; the leaf and literal plans remain in the same environment once.
#[inline(never)]
fn compile_result_matrix_entry<T>(
    cx: &mut ElabCtx<'_>,
    root_frame_depth: usize,
    ret_ty_slot: Option<Term>,
    arm_count: usize,
    predicates: &[ResultPredicate],
    build: impl FnMut(
        &mut ElabCtx<'_>, &mut Option<Term>, &mut [bool], &mut [Vec<usize>],
    ) -> Result<T, ElabError>,
) -> Result<(T, Option<Term>, Vec<bool>, Vec<Vec<usize>>), ElabError> {
    if predicates.is_empty() {
        return compile_matrix_entry(cx, root_frame_depth, ret_ty_slot, arm_count, build);
    }
    let base = cx.match_frames.len();
    let mut frame = MatchFrame::new(cx.ctx.len(), base, None, None);
    frame.result_predicates = predicates.to_vec();
    cx.match_frames.push(frame);
    let result = compile_matrix_entry(cx, root_frame_depth, ret_ty_slot, arm_count, build);
    cx.match_frames.truncate(base);
    result
}

fn compile_matrix_entry<T>(
    cx: &mut ElabCtx<'_>,
    root_frame_depth: usize,
    mut ret_ty_slot: Option<Term>,
    arm_count: usize,
    mut build: impl FnMut(
        &mut ElabCtx<'_>,
        &mut Option<Term>,
        &mut [bool],
        &mut [Vec<usize>],
    ) -> Result<T, ElabError>,
) -> Result<(T, Option<Term>, Vec<bool>, Vec<Vec<usize>>), ElabError> {
    let owner = cx.matrix_entries.len();
    cx.matrix_entries.push(MatrixEntry::new(root_frame_depth, cx.ctx.len()));
    let scope = (
        cx.ctx.len(), cx.hidden_positions.len(),
        cx.matrix_virtual_surface_positions.len(),
        cx.pattern_alias_type_frames.len(),
        cx.active_pattern_aliases.len(),
        cx.indexed_match_roots.len(),
        cx.match_frames.len(),
        cx.active_index_premise_frames.len(),
    );
    let result = loop {
        let mut arm_used = vec![false; arm_count];
        let mut subsumed_by = vec![Vec::new(); arm_count];
        match build(cx, &mut ret_ty_slot, &mut arm_used, &mut subsumed_by) {
            Err(ElabError::MatrixResultDiscovered { owner: discovered })
                if discovered == owner && !cx.matrix_entries[owner].rerun =>
            {
                let balanced = scope == (
                    cx.ctx.len(), cx.hidden_positions.len(),
                    cx.matrix_virtual_surface_positions.len(),
                    cx.pattern_alias_type_frames.len(),
                        cx.active_pattern_aliases.len(),
                    cx.indexed_match_roots.len(),
                    cx.match_frames.len(),
                    cx.active_index_premise_frames.len(),
                );
                if !balanced {
                    break Err(ElabError::Internal(
                        "matrix discovery left a scoped stack unbalanced".into(),
                    ));
                }
                let entry = &mut cx.matrix_entries[owner];
                let Some(first) = entry.first_leaf.as_ref() else {
                    break Err(ElabError::Internal(
                        "matrix discovery did not cache its first leaf".into(),
                    ));
                };
                ret_ty_slot = Some(first.result.clone());
                entry.rerun = true;
                entry.discovery = false;
                entry.skipped_ih.clear();
                entry.literal_requests.clear();
            }
            Err(ElabError::MatrixResultDiscovered { .. }) => {
                break Err(ElabError::Internal(
                    "matrix discovery escaped or repeated its owning entry".into(),
                ));
            }
            other => break other.map(|term| (term, ret_ty_slot, arm_used, subsumed_by)),
        }
    };
    cx.matrix_entries.pop();
    result
}

#[inline(never)]
fn infer_tuple_match(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
    expected: Option<&Term>,
    predicates: &[ResultPredicate],
) -> Result<(Term, Term), ElabError> {
    for arm in arms {
        if matches!(
            pattern_without_aliases(&arm.pat).kind,
            RPatKind::Wild | RPatKind::Var(_, _)
        ) {
            return Err(ElabError::Internal(
                "non-constructor pattern in match (wildcard/var not yet supported \
                 at top level; use constructor patterns)"
                    .into(),
            ));
        }
        if !matches!(pattern_without_aliases(&arm.pat).kind, RPatKind::Tuple(_)) {
            return Err(ElabError::TypeMismatch {
                span: arm.pat.span.clone(),
                reason: "tuple-pattern match arms must use tuple patterns at the top level".into(),
            });
        }
    }

    let (scrut_core, scrut_ty_raw) = infer(cx, scrut)?;
    let scrut_ty = whnf(cx.env, &cx.ctx, &scrut_ty_raw);
    if !matches!(scrut_ty, Term::Sigma(_, _)) {
        return Err(ElabError::TypeMismatch {
            span: span.clone(),
            reason: "tuple pattern requires a pair type".into(),
        });
    }

    let root_frame_depth = cx.indexed_match_roots.len();
    let seed = check_mode_result_seed(cx, expected);
    let (body_core, ret_ty_slot, arm_used, subsumed_by) = compile_result_matrix_entry(
        cx, root_frame_depth, seed, arms.len(), predicates,
        |cx, slot, used, subsumed| {
            let rows = build_alias_rows(cx, arms, &scrut_core, &scrut_ty);
            #[cfg(test)]
            MATCH_OCCURRENCE_TRACE.with(|trace| {
                if let Some(trace) = trace.borrow_mut().as_mut() {
                    trace.seeds.extend(rows.iter().map(|row| row.real_occurrences[0].term.clone()));
                }
            });
            let body = compile_match_matrix(
                cx, arms, std::slice::from_ref(&scrut_ty), &[ColKind::Real],
                rows, 0, span, root_frame_depth, slot, used, subsumed,
            );
            // Tuple matches require a Sigma scrutinee, not an indexed family.
            finish_pattern_alias_term_frame(cx, body)
        },
    )?;

    for (i, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[i].split_first() {
                Some((&first, rest)) => ArmDeadCause::Subsumed {
                    first: arms[first].span.clone(),
                    rest: rest
                        .iter()
                        .map(|&winner| arms[winner].span.clone())
                        .collect(),
                },
                None => ArmDeadCause::NoInhabitants,
            };
            return Err(ElabError::ReachabilityError {
                span: arms[i].span.clone(),
                cause,
            });
        }
    }

    Ok((
        body_core,
        ret_ty_slot.unwrap_or_else(|| Term::ty(Level::Zero)),
    ))
}

#[inline(never)]
fn infer_record_match(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
    expected: Option<&Term>,
    predicates: &[ResultPredicate],
) -> Result<(Term, Term), ElabError> {
    for arm in arms {
        if matches!(
            pattern_without_aliases(&arm.pat).kind,
            RPatKind::Wild | RPatKind::Var(_, _)
        ) {
            return Err(ElabError::Internal(
                "non-constructor pattern in match (wildcard/var not yet supported \
                 at top level; use constructor patterns)"
                    .into(),
            ));
        }
        if !matches!(pattern_without_aliases(&arm.pat).kind, RPatKind::Record(_)) {
            return Err(ElabError::TypeMismatch {
                span: arm.pat.span.clone(),
                reason: "record-pattern match arms must use record patterns at the top level"
                    .into(),
            });
        }
    }

    let (scrut_core, scrut_ty) = infer(cx, scrut)?;
    record_pattern_projection(cx, &scrut_ty, span)?;
    let root_frame_depth = cx.indexed_match_roots.len();
    let seed = check_mode_result_seed(cx, expected);
    let (body_core, ret_ty_slot, arm_used, subsumed_by) = compile_result_matrix_entry(
        cx, root_frame_depth, seed, arms.len(), predicates,
        |cx, slot, used, subsumed| {
            let rows = build_alias_rows(cx, arms, &scrut_core, &scrut_ty);
            #[cfg(test)]
            MATCH_OCCURRENCE_TRACE.with(|trace| {
                if let Some(trace) = trace.borrow_mut().as_mut() {
                    trace.seeds.extend(rows.iter().map(|row| row.real_occurrences[0].term.clone()));
                }
            });
            let body = compile_match_matrix(
                cx, arms, std::slice::from_ref(&scrut_ty), &[ColKind::Real],
                rows, 0, span, root_frame_depth, slot, used, subsumed,
            );
            // Record patterns require a named projection owner, not an indexed family.
            finish_pattern_alias_term_frame(cx, body)
        },
    )?;

    for (i, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[i].split_first() {
                Some((&first, rest)) => ArmDeadCause::Subsumed {
                    first: arms[first].span.clone(),
                    rest: rest
                        .iter()
                        .map(|&winner| arms[winner].span.clone())
                        .collect(),
                },
                None => ArmDeadCause::NoInhabitants,
            };
            return Err(ElabError::ReachabilityError {
                span: arms[i].span.clone(),
                cause,
            });
        }
    }

    Ok((
        body_core,
        ret_ty_slot.unwrap_or_else(|| Term::ty(Level::Zero)),
    ))
}

fn top_pattern_contains_or(pattern: &RPattern) -> bool {
    match &pattern.kind {
        RPatKind::Or(_) => true,
        RPatKind::As(inner, _, _) => top_pattern_contains_or(inner),
        _ => false,
    }
}

#[inline(never)]
fn arms_have_top_or(arms: &[RMatchArm]) -> bool {
    arms.iter().any(|arm| top_pattern_contains_or(&arm.pat))
}

fn top_pattern_is_catchall(pattern: &RPattern) -> bool {
    match &pattern.kind {
        RPatKind::Wild | RPatKind::Var(_, _) => true,
        RPatKind::As(inner, _, _) => top_pattern_is_catchall(inner),
        RPatKind::Or(alternatives) => alternatives.iter().any(top_pattern_is_catchall),
        RPatKind::Ctor(_, _)
        | RPatKind::CheckedCtor(_, _, _)
        | RPatKind::Tuple(_)
        | RPatKind::Record(_)
        | RPatKind::Literal(_, _) => false,
    }
}

fn ensure_top_pattern_ctors_belong_to_family(
    cx: &ElabCtx,
    pattern: &RPattern,
    ind: &InductiveDecl,
    d_id: GlobalId,
) -> Result<(), ElabError> {
    match &pattern.kind {
        RPatKind::Ctor(name, _) | RPatKind::CheckedCtor(name, _, _) => {
            let ctor_id = pattern_ctor_id(cx, &pattern.kind)
                .expect("constructor resolution precedes family validation");
            if !ind
                .constructors
                .iter()
                .any(|constructor| constructor.id == ctor_id)
            {
                return Err(ElabError::TypeMismatch {
                    span: pattern.span.clone(),
                    reason: format!(
                        "constructor '{}' is not a constructor of type '{}'",
                        name,
                        type_name(cx, d_id)
                    ),
                });
            }
        }
        RPatKind::As(inner, _, _) => {
            ensure_top_pattern_ctors_belong_to_family(cx, inner, ind, d_id)?;
        }
        RPatKind::Or(alternatives) => {
            for alternative in alternatives {
                ensure_top_pattern_ctors_belong_to_family(cx, alternative, ind, d_id)?;
            }
        }
        RPatKind::Wild
        | RPatKind::Var(_, _)
        | RPatKind::Tuple(_)
        | RPatKind::Record(_)
        | RPatKind::Literal(_, _) => {}
    }
    Ok(())
}

#[inline(never)]
fn infer_or_match(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
    expected: Option<&Term>,
    predicates: &[ResultPredicate],
) -> Result<(Term, Term), ElabError> {
    for arm in arms {
        if top_pattern_is_catchall(&arm.pat) {
            return Err(ElabError::Internal(
                "non-constructor pattern in match (wildcard/var not yet supported \
                 at top level; use constructor patterns)"
                    .into(),
            ));
        }
    }

    let (scrut_core, scrut_ty_raw) = infer(cx, scrut)?;
    let scrut_ty = whnf(cx.env, &cx.ctx, &scrut_ty_raw);
    let (head, _) = peel_app(&scrut_ty);
    let inductive = if let Term::IndFormer { id, .. } = head {
        let ind = cx
            .env
            .inductive(id)
            .ok_or_else(|| ElabError::Internal(format!("inductive {:?} not found", id)))?
            .clone();
        for arm in arms {
            ensure_top_pattern_ctors_belong_to_family(cx, &arm.pat, &ind, id)?;
        }
        Some(id)
    } else {
        None
    };

    let root_frame_depth = cx.indexed_match_roots.len();
    let seed = check_mode_result_seed(cx, expected);
    let (body_core, ret_ty_slot, arm_used, subsumed_by) = compile_result_matrix_entry(
        cx, root_frame_depth, seed, arms.len(), predicates,
        |cx, slot, used, subsumed| {
            let rows = build_alias_rows(cx, arms, &scrut_core, &scrut_ty);
            let body = compile_match_matrix(
                cx, arms, std::slice::from_ref(&scrut_ty), &[ColKind::Real],
                rows, 0, span, root_frame_depth, slot, used, subsumed,
            );
            finish_pattern_alias_term_frame(cx, body)
        },
    )?;

    for (i, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[i].split_first() {
                Some((&first, rest)) => ArmDeadCause::Subsumed {
                    first: arms[first].span.clone(),
                    rest: rest
                        .iter()
                        .map(|&winner| arms[winner].span.clone())
                        .collect(),
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
    let body_core = if inductive.is_some() {
        let function_ty = Term::pi(scrut_ty.clone(), weaken(&ret_ty, 1));
        Term::app(
            Term::Ascript(Box::new(body_core), Box::new(function_ty)),
            scrut_core,
        )
    } else {
        body_core
    };
    Ok((body_core, ret_ty))
}

fn top_pattern_contains_literal(pattern: &RPattern) -> bool {
    match &pattern.kind {
        RPatKind::Literal(_, _) => true,
        RPatKind::As(inner, _, _) => top_pattern_contains_literal(inner),
        RPatKind::Or(alternatives) => alternatives.iter().any(top_pattern_contains_literal),
        RPatKind::Wild
        | RPatKind::Var(_, _)
        | RPatKind::Ctor(_, _)
        | RPatKind::CheckedCtor(_, _, _)
        | RPatKind::Tuple(_)
        | RPatKind::Record(_) => false,
    }
}

fn top_pattern_is_literal_form(pattern: &RPattern) -> bool {
    match &pattern.kind {
        RPatKind::Literal(_, _) | RPatKind::Wild => true,
        // Preserve the landed refusal for top-level catchall aliases and
        // catchall or-alternatives. Only aliases/alternatives that actually
        // retain a literal test participate in this specialized entry path.
        RPatKind::As(inner, _, _) => {
            top_pattern_contains_literal(inner) && top_pattern_is_literal_form(inner)
        }
        RPatKind::Or(alternatives) => alternatives.iter().all(|alternative| {
            top_pattern_contains_literal(alternative)
                && top_pattern_is_literal_form(alternative)
        }),
        RPatKind::Var(_, _)
        | RPatKind::Ctor(_, _)
        | RPatKind::CheckedCtor(_, _, _)
        | RPatKind::Tuple(_)
        | RPatKind::Record(_) => false,
    }
}

/// Top-level literal matching is a deliberately separate entry path. It admits
/// the wildcard residual required by an open value column without lifting the
/// general top-level variable/wildcard rule for constructor matches.
#[inline(never)]
fn infer_literal_match(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
    expected: Option<&Term>,
    predicates: &[ResultPredicate],
) -> Result<(Term, Term), ElabError> {
    for arm in arms {
        if !top_pattern_is_literal_form(&arm.pat) {
            return Err(ElabError::TypeMismatch {
                span: arm.pat.span.clone(),
                reason: "literal-pattern match arms require literals and a wildcard residual"
                    .into(),
            });
        }
    }

    let (scrut_core, scrut_ty) = infer(cx, scrut)?;
    let root_frame_depth = cx.indexed_match_roots.len();
    let seed = check_mode_result_seed(cx, expected);
    let (body_core, ret_ty_slot, arm_used, subsumed_by) = compile_result_matrix_entry(
        cx, root_frame_depth, seed, arms.len(), predicates,
        |cx, slot, used, subsumed| {
            let rows = build_alias_rows(cx, arms, &scrut_core, &scrut_ty);
            let body = compile_match_matrix(
                cx, arms, std::slice::from_ref(&scrut_ty), &[ColKind::Real],
                rows, 0, span, root_frame_depth, slot, used, subsumed,
            );
            // Literal comparators can have indexed inductive scrutinees.
            finish_pattern_alias_term_frame(cx, body)
        },
    )?;

    for (index, used) in arm_used.iter().enumerate() {
        if !used {
            let cause = match subsumed_by[index].split_first() {
                Some((&first, rest)) => ArmDeadCause::Subsumed {
                    first: arms[first].span.clone(),
                    rest: rest
                        .iter()
                        .map(|&winner| arms[winner].span.clone())
                        .collect(),
                },
                None => ArmDeadCause::NoInhabitants,
            };
            return Err(ElabError::ReachabilityError {
                span: arms[index].span.clone(),
                cause,
            });
        }
    }

    Ok((
        body_core,
        ret_ty_slot.unwrap_or_else(|| Term::ty(Level::Zero)),
    ))
}

#[inline(always)]
fn infer_match(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
    expected: Option<&Term>,
) -> Result<(Term, Term), ElabError> {
    infer_match_with_predicates(cx, scrut, arms, span, expected, &[])
}

#[inline(never)]
fn infer_match_with_predicates(
    cx: &mut ElabCtx,
    scrut: &RExpr,
    arms: &[RMatchArm],
    span: &Span,
    expected: Option<&Term>,
    predicates: &[ResultPredicate],
) -> Result<(Term, Term), ElabError> {
    for arm in arms {
        ensure_pattern_constructors_resolve(cx, &arm.pat)?;
    }
    if arms.iter().any(|arm| top_pattern_contains_literal(&arm.pat)) {
        return infer_literal_match(cx, scrut, arms, span, expected, predicates);
    }
    if arms_have_top_or(arms) {
        return infer_or_match(cx, scrut, arms, span, expected, predicates);
    }
    if arms
        .iter()
        .any(|arm| matches!(pattern_without_aliases(&arm.pat).kind, RPatKind::Record(_)))
    {
        return infer_record_match(cx, scrut, arms, span, expected, predicates);
    }
    if arms
        .iter()
        .any(|arm| matches!(pattern_without_aliases(&arm.pat).kind, RPatKind::Tuple(_)))
    {
        return infer_tuple_match(cx, scrut, arms, span, expected, predicates);
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
        if matches!(
            pattern_without_aliases(&arm.pat).kind,
            RPatKind::Wild | RPatKind::Var(_, _)
        ) {
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
    // In check mode the goal is known before any bucket is compiled. Pure
    // inference discovers R at the first leaf, rerunning only if an IH
    // requires it first; the first leaf itself is reused exactly once.
    let seed = check_mode_result_seed(cx, expected);

    // Indexed families need the dependent motive even when every root
    // constructor is written. Only a missing root bucket uses the omission
    // permission, with the existing absurd-method proof as its authority.
    let indexed = !ind.indices.is_empty();
    let root_frame_depth = cx.indexed_match_roots.len();
    if indexed {
        let Term::IndFormer { level_args, .. } = &head else {
            unreachable!("inductive scrutinee head checked above")
        };
        let (params, scrut_indices) = params_terms.split_at(m);
        cx.indexed_match_roots.push(IndexedMatchRootFrame {
            outer: cx.ctx.clone(),
            ind: ind.clone(),
            family: d_id,
            level_args: level_args.clone(),
            params: params.to_vec(),
            scrut_indices: scrut_indices.to_vec(),
            motive: None,
        });
    }
    let result = (|| {
    let (raw_methods, ret_ty_slot, arm_used, subsumed_by) = compile_matrix_entry(
        cx, root_frame_depth, seed, arms.len(),
        |cx, slot, used, subsumed| {
            let rows = build_alias_rows(cx, arms, &scrut_core, &scrut_ty);
            #[cfg(test)]
            MATCH_OCCURRENCE_TRACE.with(|trace| {
                if let Some(trace) = trace.borrow_mut().as_mut() {
                    trace.seeds.extend(rows.iter().map(|row| row.real_occurrences[0].term.clone()));
                }
            });
            // The indexed root owns this motive. On a seeded rerun this is
            // the first producer; discovery never memoizes the outer root.
            let methods = (|| {
                if let Some(ret_ty) = slot.as_ref() {
                    memoize_indexed_root_motive(cx, ret_ty, span, root_frame_depth)?;
                }
                build_ctor_buckets(
                    cx, arms, &ind, d_id, m, &params_terms, rows, &[], &[],
                    0, span, root_frame_depth, slot, used, subsumed,
                    indexed, indexed, false,
                    match &head {
                        Term::IndFormer { level_args, .. } => level_args,
                        _ => unreachable!("match head is an inductive former"),
                    },
                    None,
                    None,
                    predicates,
                )
            })();
            finish_pattern_alias_frame(cx, methods)
        },
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
    if indexed {
        let Term::IndFormer { level_args, .. } = &head else {
            unreachable!("inductive scrutinee head checked above")
        };
        let (params, scrut_indices) = params_terms.split_at(m);
        let elim = finish_inferred_indexed_match(
            cx,
            &ind,
            d_id,
            level_args,
            params,
            scrut_indices,
            raw_methods,
            &ret_ty,
            scrut_core,
            span,
        )?;
        return Ok((elim, ret_ty));
    }
    let raw_methods: Vec<_> = raw_methods
        .into_iter()
        .map(|method| method.expect("complete inferred match has all constructor methods"))
        .collect();

    // 7. Build a constant motive over the family, ascribed at the return
    //    type's classifier. Indexed families were dispatched above; the
    //    shared builder also handles nested indexed matrix splits.
    let ret_level = match kernel_infer_current(cx, &ret_ty) {
        Ok(Term::Type(level)) => level,
        Ok(_) => Level::Zero,
        Err(CurrentKernelQueryError::View(error)) => return Err(error),
        Err(CurrentKernelQueryError::Kernel(_)) if cx.active_index_premise_frames.is_empty() => {
            Level::Zero
        }
        Err(CurrentKernelQueryError::Kernel(error)) => {
            return Err(ElabError::KernelRejected {
                error,
                span: span.clone(),
            })
        }
    };
    // 8. The top-level scrutinee is already a concrete elaborated value;
    //    unlike a nested split, it needs no extra enclosing binder.
    let Term::IndFormer { level_args, .. } = head else {
        unreachable!("inductive scrutinee head checked above")
    };
    let elim = matrix_family_elim(
        &ind,
        d_id,
        &level_args,
        &params_terms,
        &scrut_ty,
        weaken(&ret_ty, 1),
        ret_level,
        raw_methods,
        scrut_core,
        false,
    );

    Ok((elim, ret_ty))
    })();
    cx.indexed_match_roots.truncate(root_frame_depth);
    result
}

fn ensure_pattern_constructors_resolve(
    cx: &ElabCtx<'_>,
    pattern: &RPattern,
) -> Result<(), ElabError> {
    match &pattern.kind {
        RPatKind::Ctor(name, fields) | RPatKind::CheckedCtor(name, _, fields) => {
            if matches!(&pattern.kind, RPatKind::Ctor(_, _)) && !cx.globals.contains_key(name) {
                return Err(ElabError::UnresolvedCon {
                    name: name.clone(),
                    span: pattern.span.clone(),
                });
            }
            for field in fields {
                ensure_pattern_constructors_resolve(cx, field)?;
            }
        }
        RPatKind::Tuple(components) => {
            for component in components {
                ensure_pattern_constructors_resolve(cx, component)?;
            }
        }
        RPatKind::Record(fields) => {
            for field in fields {
                ensure_pattern_constructors_resolve(cx, &field.pattern)?;
            }
        }
        RPatKind::As(inner, _, _) => ensure_pattern_constructors_resolve(cx, inner)?,
        RPatKind::Or(alternatives) => {
            for alternative in alternatives {
                ensure_pattern_constructors_resolve(cx, alternative)?;
            }
        }
        RPatKind::Wild | RPatKind::Var(_, _) | RPatKind::Literal(_, _) => {}
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
        let head = &pattern_without_aliases(&arm.pat).kind;
        if let RPatKind::Ctor(name, _) | RPatKind::CheckedCtor(name, _, _) = head {
            let ctor_id = pattern_ctor_id(cx, head).expect(
                "ensure_pattern_constructors_resolve already validated every top-level \
                 arm pattern before this function runs",
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
fn lower_pattern_type_to_common(term: &Term, k: usize) -> Option<Term> {
    fn go(term: &Term, k: usize, cutoff: usize) -> Option<Term> {
        let child = |term: &Term, cutoff| go(term, k, cutoff);
        match term {
            Term::Var(index) if *index < cutoff => Some(Term::var(*index)),
            Term::Var(index) if *index < cutoff + k => None,
            Term::Var(index) => Some(Term::var(*index - k)),
            Term::Pi(domain, codomain) => Some(Term::pi(
                child(domain, cutoff)?,
                child(codomain, cutoff + 1)?,
            )),
            Term::Lam(domain, body) => {
                Some(Term::lam(child(domain, cutoff)?, child(body, cutoff + 1)?))
            }
            Term::Sigma(domain, codomain) => Some(Term::sigma(
                child(domain, cutoff)?,
                child(codomain, cutoff + 1)?,
            )),
            Term::Let { ty, val, body } => Some(Term::Let {
                ty: Box::new(child(ty, cutoff)?),
                val: Box::new(child(val, cutoff)?),
                body: Box::new(child(body, cutoff + 1)?),
            }),
            Term::App(function, argument) => Some(Term::app(
                child(function, cutoff)?,
                child(argument, cutoff)?,
            )),
            Term::Pair(first, second) => {
                Some(Term::pair(child(first, cutoff)?, child(second, cutoff)?))
            }
            Term::Proj1(pair) => Some(Term::proj1(child(pair, cutoff)?)),
            Term::Proj2(pair) => Some(Term::proj2(child(pair, cutoff)?)),
            Term::Ascript(value, ty) => Some(Term::Ascript(
                Box::new(child(value, cutoff)?),
                Box::new(child(ty, cutoff)?),
            )),
            Term::Eq(ty, left, right) => Some(Term::Eq(
                Box::new(child(ty, cutoff)?),
                Box::new(child(left, cutoff)?),
                Box::new(child(right, cutoff)?),
            )),
            Term::Cast(from, to, evidence, value) => Some(Term::Cast(
                Box::new(child(from, cutoff)?),
                Box::new(child(to, cutoff)?),
                Box::new(child(evidence, cutoff)?),
                Box::new(child(value, cutoff)?),
            )),
            Term::J(motive, base, evidence) => Some(Term::J(
                Box::new(child(motive, cutoff)?),
                Box::new(child(base, cutoff)?),
                Box::new(child(evidence, cutoff)?),
            )),
            Term::Quot(carrier, relation, equivalence) => Some(Term::Quot(
                Box::new(child(carrier, cutoff)?),
                Box::new(child(relation, cutoff)?),
                Box::new(child(equivalence, cutoff)?),
            )),
            Term::QuotClass(value) => Some(Term::QuotClass(Box::new(child(value, cutoff)?))),
            Term::Trunc(carrier) => Some(Term::Trunc(Box::new(child(carrier, cutoff)?))),
            Term::TruncProj(value) => Some(Term::TruncProj(Box::new(child(value, cutoff)?))),
            Term::Refl(value) => Some(Term::Refl(Box::new(child(value, cutoff)?))),
            Term::QuotElim {
                motive,
                method,
                respect,
                scrut,
            } => Some(Term::QuotElim {
                motive: Box::new(child(motive, cutoff)?),
                method: Box::new(child(method, cutoff)?),
                respect: Box::new(child(respect, cutoff)?),
                scrut: Box::new(child(scrut, cutoff)?),
            }),
            Term::Elim {
                fam,
                level_args,
                params,
                motive,
                methods,
                indices,
                scrut,
            } => Some(Term::Elim {
                fam: *fam,
                level_args: level_args.clone(),
                params: params
                    .iter()
                    .map(|term| child(term, cutoff))
                    .collect::<Option<Vec<_>>>()?,
                motive: Box::new(child(motive, cutoff)?),
                methods: methods
                    .iter()
                    .map(|term| child(term, cutoff))
                    .collect::<Option<Vec<_>>>()?,
                indices: indices
                    .iter()
                    .map(|term| child(term, cutoff))
                    .collect::<Option<Vec<_>>>()?,
                scrut: Box::new(child(scrut, cutoff)?),
            }),
            Term::Absurd(motive, proof) => Some(Term::Absurd(
                Box::new(child(motive, cutoff)?),
                Box::new(child(proof, cutoff)?),
            )),
            Term::Type(_)
            | Term::Omega(_)
            | Term::Const { .. }
            | Term::IndFormer { .. }
            | Term::Constructor { .. }
            | Term::IntLit(_) => Some(term.clone()),
        }
    }

    if k == 0 {
        Some(term.clone())
    } else {
        go(term, k, 0)
    }
}

/// Remove `k` binders starting at `cutoff`. `Err(j)` names a removed binder
/// (relative to `cutoff`) that `term` mentions. Use the kernel's total shift
/// over every term former, rather than a partial elaborator-side traversal.
fn lower_binders(term: &Term, k: usize, cutoff: usize) -> Result<Term, usize> {
    for j in 0..k {
        let at = cutoff + j;
        if shift(&shift(term, -1, at), 1, at) != *term {
            return Err(j);
        }
    }
    Ok(shift(term, -(k as i64), cutoff))
}

/// Project a leaf type to the match's outer scope, or name its first
/// escaping matrix binder (smallest de Bruijn index).
fn lower_by(term: &Term, k: usize) -> Result<Term, usize> {
    lower_binders(term, k, 0)
}

/// Resolve a failing core variable to its original pattern name only where
/// the matrix has an exact surviving occurrence; projection/alias paths may
/// not have one, and the surface diagnostic still applies without a name.
fn recover_escaping_pattern_binder(
    pattern: &RPattern,
    occurrences: &[Option<Term>],
    escaping_index: usize,
    real_depth_so_far: usize,
) -> Option<String> {
    fn names<'a>(pattern: &'a RPattern, out: &mut Vec<Option<&'a str>>) {
        match &pattern.kind {
            RPatKind::Var(name, _) if name != "_" => out.push(Some(name)),
            RPatKind::Var(_, _) | RPatKind::Wild | RPatKind::Literal(_, _) => out.push(None),
            RPatKind::Ctor(_, fields) | RPatKind::CheckedCtor(_, _, fields)
            | RPatKind::Tuple(fields) => {
                for field in fields { names(field, out); }
            }
            RPatKind::Record(fields) => {
                for field in fields { names(&field.pattern, out); }
            }
            RPatKind::As(inner, alias, _) => {
                out.push(Some(alias));
                names(inner, out);
            }
            // Or alternatives can reorder their canonical slots by name;
            // never report a guessed binder from one alternative.
            RPatKind::Or(_) => {}
        }
    }
    let mut binding_names = Vec::new();
    names(pattern, &mut binding_names);
    if binding_names.len() != occurrences.len() {
        return None;
    }
    // Both occurrences and the escaping index live in `cx.ctx`, including
    // hidden IH and split entries. `real_depth_so_far` counts only flat Real
    // source binders; compare their order, not raw de Bruijn indices. A
    // hidden IH below `m` changes its index without adding a source binder.
    let mut binders = binding_names
        .into_iter()
        .zip(occurrences)
        .map(|(name, occurrence)| match occurrence {
            Some(Term::Var(index)) => Some((*index, name)),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    if binders.len() != real_depth_so_far || escaping_index >= real_depth_so_far {
        return None;
    }
    binders.sort_by(|a, b| b.0.cmp(&a.0));
    if binders.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return None;
    }
    binders[real_depth_so_far - 1 - escaping_index]
        .1
        .map(str::to_string)
}

// ----- standalone expression elaboration -----

pub(crate) fn elaborate_rexpr(
    env: &mut GlobalEnv,
    globals: &HashMap<String, GlobalId>,
    preconditions: &HashMap<GlobalId, (usize, usize)>,
    num_values: &mut HashMap<GlobalId, NumericLitVal>,
    numeric_env: &NumericEnv,
    refinement_facts: &RefinementFacts,
    owner_label: impl Into<String>,
    rexpr: &RExpr,
) -> Result<(Term, Term), ElabError> {
    let (core, ty, expr_span) = {
        let mut cx = ElabCtx::new(env, globals, num_values, numeric_env, refinement_facts, owner_label)
            .with_preconditions(preconditions, PremiseHoles::Refused);
        let (core_raw, ty_raw) = infer(&mut cx, rexpr)?;
        let c = cx.metas.zonk_term(&core_raw);
        let t = cx.metas.zonk_term(&ty_raw);
        (c, t, rexpr.span().clone())
    };
    kernel_check_raw(env, &Context::new(), &core, &ty).map_err(|e| ElabError::KernelRejected {
        error: e,
        span: expr_span,
    })?;
    Ok((core, ty))
}

#[cfg(test)]
mod omega_index_refinement_tests {
    use crate::{ElabEnv, ElabError};
    use ken_kernel::{
        infer as kernel_infer_raw, whnf, ConstructorDecl, Context, GlobalId, InductiveDecl, Level, Term,
    };

    use super::{
        build_index_omega_transport, check_variable_with_index_views,
        classify_branch_goal_restoration, install_hidden_result_variable_refinements,
        install_index_refinements, subst_term_generalize, try_reindex_cast, weaken, ElabCtx,
        MatchFrame,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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
        // ascribed proof, or old-to-new orientation must make this control red.
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
        let expected_proof_type = Term::Eq(
            Box::new(idx_ty.clone()),
            Box::new(old_idx.clone()),
            Box::new(new_idx.clone()),
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
            Box::new(Term::Ascript(Box::new(h), Box::new(expected_proof_type))),
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
        let generalized = subst_term_generalize(&source_type, &old_index, &new_index);
        let plan = classify_branch_goal_restoration(
            &env.env,
            &Context::new(),
            &index_type,
            &old_index,
            &new_index,
            &source_type,
            &generalized,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "omega-prefilter-control",
        );
        cx.ctx.push(nat.clone());
        cx.ctx.push(Term::Eq(
            Box::new(nat.clone()),
            Box::new(Term::var(0)),
            Box::new(Term::var(0)),
        ));
        // These ambient binders precede the checked constructor arm. The
        // production seam now requires that arm's explicit owning frame.
        cx.match_frames
            .push(MatchFrame::new(cx.ctx.len(), 0, None, None));

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

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "omega-convoy-index-control",
        );
        cx.ctx.push(Term::ty(Level::Zero));
        cx.match_frames
            .push(MatchFrame::new(cx.ctx.len(), 0, None, None));

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
            &kernel_infer_raw(&env.env, &ctx, &cur_ty).expect("Zero infers at Nat"),
        );
        let error = try_reindex_cast(
            None,
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
#[path = "sibling_goal_refinement_tests.rs"]
mod sibling_goal_refinement_tests;

#[cfg(test)]
mod result_transport_control_flow_tests {
    use crate::{error::Span, resolve::RExpr, ElabEnv, ElabError};
    use ken_kernel::{convert_type, GlobalId, Level, Term};

    use super::{
        active_premise_kernel_view, index_refinement_sentinel, kernel_check_current,
        kernel_infer_current, transport_recursive_group_call_result, validate_large_convoy_base,
        ActiveIndexPremiseFrame, CurrentKernelQueryError, ElabCtx, ExpandedBindingSource,
        ResultRefinement,
    };

    fn app2(head: Term, first: Term, second: Term) -> Term {
        Term::app(Term::app(head, first), second)
    }

    fn refinement(
        index_ty: Term,
        concrete_index: Term,
        refined_index: Term,
        premise_slot: usize,
        install_depth: usize,
    ) -> ResultRefinement {
        ResultRefinement {
            index_ty,
            concrete_index,
            refined_index,
            premise_slot,
            sentinel_region: 0,
            install_depth,
        }
    }

    #[test]
    fn ordinary_kernel_queries_zonk_unconstrained_level_in_context_and_operand() {
        // Promise class: durable invariant (spec 39 §5.7). A query may
        // default a fresh declaration level for kernel admission without
        // committing that choice to the elaborator's metavariable store.
        // Both no-premise gateways must apply the same substitution.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "no-premise-level-query-control",
        );
        let level = cx.metas.fresh();
        cx.ctx.push(Term::ty(level.clone())); // A : Type ?u
        assert_eq!(
            kernel_infer_current(&cx, &Term::var(0)).expect("infer A"),
            Term::ty(Level::Zero),
            "kernel inference may not return an elaborator level meta"
        );
        assert_eq!(
            kernel_infer_current(&cx, &Term::ty(level.clone())).expect("infer Type ?u"),
            Term::ty(Level::Suc(Box::new(Level::Zero))),
            "the operand itself must be zonked before kernel inference"
        );
        kernel_check_current(&cx, &Term::var(0), &Term::ty(Level::Zero))
            .expect("A : Type 0 after contextual level zonking");
        kernel_check_current(
            &cx,
            &Term::ty(level),
            &Term::ty(Level::Suc(Box::new(Level::Zero))),
        )
        .expect("Type ?u : Type 1 after operand level zonking");
        assert!(
            cx.metas.metas[0].is_none(),
            "a read-only kernel query must not solve a declaration level meta"
        );
    }

    #[test]
    fn ordinary_kernel_check_zonks_expected_level_independently() {
        // Promise class: durable invariant (spec 39 §5.7).
        // MEASURED: a concrete `A : Type 0` checks against `Type ?u` while
        // that meta remains unsolved in the elaborator. CLAIMED: the expected
        // operand reaching the kernel has no declaration-level meta. THE GAP:
        // this private query pins the shared gateway; the source-level Vec
        // twins independently pin the production match-arm path.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "expected-only-level-control",
        );
        cx.ctx.push(Term::ty(Level::Zero)); // A : Type 0
        let expected_level = cx.metas.fresh();
        kernel_check_current(&cx, &Term::var(0), &Term::ty(expected_level))
            .expect("the expected Type ?u must be zonked without changing A");
        assert!(
            cx.metas.metas[0].is_none(),
            "query-local zonking must not solve the elaborator meta"
        );
    }

    /// MEASURED: map_free sees an Elim's variables in params, motive, methods,
    /// indices, and scrut order, and the first returned error stops traversal.
    /// CLAIMED: the outlined helpers preserve left-to-right callback effects
    /// and first-error identity. THE GAP: production caller suites cover real
    /// Elims; this fixture pins one populated shape of the Elim child lists.
    #[test]
    fn active_premise_elim_preserves_visit_order_and_first_error() {
        let term = Term::Elim {
            fam: GlobalId(0),
            level_args: vec![],
            params: vec![Term::var(10), Term::var(11)],
            motive: Box::new(Term::var(12)),
            methods: vec![Term::var(13), Term::var(14)],
            indices: vec![Term::var(15), Term::var(16)],
            scrut: Box::new(Term::var(17)),
        };
        let expected_order = vec![10, 11, 12, 13, 14, 15, 16, 17];
        let mut visited = Vec::new();
        let relocated = super::relocate_active_premise_term(&term, 0, &mut |index, _depth| {
            visited.push(index);
            Ok(Term::var(index))
        })
        .expect("identity relocation succeeds");
        assert_eq!(relocated, term);
        assert_eq!(visited, expected_order);

        let mut visited = Vec::new();
        let error = super::relocate_active_premise_term(&term, 0, &mut |index, _depth| {
            visited.push(index);
            if index == 13 {
                Err(ElabError::Internal("first relocation error".into()))
            } else {
                Ok(Term::var(index))
            }
        })
        .expect_err("the first map_free error is returned");
        assert!(matches!(error, ElabError::Internal(message)
            if message == "first relocation error"));
        assert_eq!(visited, vec![10, 11, 12, 13]);
    }

    /// MEASURED: a 64-level App chain relocates on a fixed 512 KiB child thread
    /// and yields a structurally equal term.
    /// CLAIMED: outlined traversal fits this controlled stack where the old
    /// dispatcher/closure cycle exceeds it.
    /// THE GAP: this pins the App recursion path; the sibling/call-site suites
    /// exercise production terms, other variants, and translation callers.
    /// Builder::stack_size sets this child stack directly, independent of the
    /// libtest parent stack, ambient ulimit, or RUST_MIN_STACK default. The old
    /// 45,152 B per-level cycle needs
    /// 64 × 45,152 = 2,889,728 B before caller overhead, above 524,288 B.
    #[test]
    fn active_premise_app_chain_64_fits_stated_512k_stack() {
        const DEPTH: usize = 64;
        const STACK_BYTES: usize = 512 * 1024;

        let input = (0..DEPTH).fold(Term::var(0), |subterm, _| Term::app(subterm, Term::var(0)));
        let expected = input.clone();
        let relocated = std::thread::Builder::new()
            .name("active-premise-app-chain-64".into())
            .stack_size(STACK_BYTES)
            .spawn(move || {
                super::relocate_active_premise_term(&input, 0, &mut |free_index, _depth| {
                    Ok(Term::var(free_index))
                })
            })
            .expect("spawn stated-stack relocation worker")
            .join()
            .expect("relocation worker does not panic")
            .expect("identity relocation succeeds");
        assert_eq!(relocated, expected);
    }

    #[test]
    fn active_premise_frame_authority_fails_closed() {
        // Promise class: durable invariant.
        // MEASURED: malformed private frame metadata is rejected before a
        // contextual kernel view is built. CLAIMED: only a uniquely owned,
        // live, complete premise telescope may authorize sentinel relocation. THE
        // GAP: ordinary construction makes these states unrepresentable; this
        // direct control pins the fail-closed boundary for future callers.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "frame-authority-control",
        );
        let term = Term::Type(Level::Zero);

        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 7,
                premise_domains: vec![],
                install_depth: 0,
            });
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 7,
                premise_domains: vec![],
                install_depth: 0,
            });
        let Err(duplicate) = active_premise_kernel_view(&cx) else {
            panic!("duplicate region ownership must reject");
        };
        assert!(matches!(duplicate, ElabError::Internal(ref reason)
            if reason.contains("duplicate active index-premise sentinel region 7")));

        cx.active_index_premise_frames.clear();
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 8,
                premise_domains: vec![],
                install_depth: 1,
            });
        let Err(escaped) = active_premise_kernel_view(&cx) else {
            panic!("a frame beyond the current context must reject");
        };
        assert!(matches!(escaped, ElabError::Internal(ref reason)
            if reason.contains("escaped its install context")));

        cx.active_index_premise_frames.clear();
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 9,
                premise_domains: vec![],
                install_depth: 0,
            });
        cx.result_refinements.push(ResultRefinement {
            index_ty: term.clone(),
            concrete_index: term.clone(),
            refined_index: term.clone(),
            premise_slot: 0,
            sentinel_region: 9,
            install_depth: 0,
        });
        let Err(outside) = active_premise_kernel_view(&cx) else {
            panic!("an out-of-telescope sentinel slot must reject");
        };
        assert!(matches!(outside, ElabError::Internal(ref reason)
            if reason.contains("sentinel slot 0 exceeds region 9 premise telescope")));
    }

    #[test]
    fn active_premise_embedding_rejects_forward_unknown_and_noninvertible_terms() {
        // Promise class: durable invariant.
        // MEASURED: the contextual view accepts an earlier-premise dependency
        // and exact owned sentinel, while refusing the same/later dependency,
        // unknown sentinel, ordinary out-of-scope variable, and an inferred
        // binding with no inverse source. CLAIMED: relocation authority is the
        // explicit bidirectional embedding, never numeric proximity. THE GAP:
        // frame-shape checks alone do not exercise term or context traversal.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "embedding-authority-control",
        );
        let type_zero = Term::Type(Level::Zero);
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 20,
                premise_domains: vec![type_zero.clone()],
                install_depth: 0,
            });
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 21,
                premise_domains: vec![index_refinement_sentinel(20, 0)],
                install_depth: 0,
            });
        let view = active_premise_kernel_view(&cx)
            .expect("valid dependent premise plan")
            .expect("active view");
        assert_eq!(
            view.embedding.expanded_sources,
            vec![
                ExpandedBindingSource::Premise {
                    sentinel_region: 20,
                    premise_slot: 0,
                },
                ExpandedBindingSource::Premise {
                    sentinel_region: 21,
                    premise_slot: 0,
                },
            ],
            "equal-depth frames retain outer-to-inner stack order"
        );
        assert_eq!(
            kernel_infer_current(&cx, &index_refinement_sentinel(20, 0))
                .expect("owned sentinel inference"),
            type_zero
        );
        kernel_check_current(
            &cx,
            &index_refinement_sentinel(20, 0),
            &Term::Type(Level::Zero),
        )
        .expect("owned sentinel checking");

        let unknown = kernel_infer_current(&cx, &index_refinement_sentinel(22, 0))
            .expect_err("unknown sentinel must reject");
        assert!(matches!(unknown, CurrentKernelQueryError::View(
            ElabError::Internal(ref reason)
        ) if reason.contains("unknown sentinel or ordinary out-of-scope")));
        let ordinary = kernel_infer_current(&cx, &Term::var(0))
            .expect_err("ordinary out-of-scope variable must reject");
        assert!(matches!(ordinary, CurrentKernelQueryError::View(
            ElabError::Internal(ref reason)
        ) if reason.contains("ordinary out-of-scope index 0")));

        let mut noninvertible = view.embedding.clone();
        noninvertible.expanded_sources.clear();
        let error = noninvertible
            .translate_to_original(&Term::var(0), 0, 2)
            .expect_err("an expanded binding without a source must reject");
        assert!(matches!(error, ElabError::Internal(ref reason)
            if reason.contains("has no source for expanded binding")));

        cx.active_index_premise_frames.clear();
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 30,
                premise_domains: vec![index_refinement_sentinel(31, 0)],
                install_depth: 0,
            });
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 31,
                premise_domains: vec![Term::Type(Level::Zero)],
                install_depth: 0,
            });
        let Err(forward) = active_premise_kernel_view(&cx) else {
            panic!("a later-frame dependency must reject")
        };
        assert!(matches!(forward, ElabError::Internal(ref reason)
            if reason.contains("forward dependency")));

        cx.active_index_premise_frames.clear();
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 32,
                premise_domains: vec![
                    index_refinement_sentinel(32, 1),
                    Term::Type(Level::Zero),
                ],
                install_depth: 0,
            });
        let Err(later_slot) = active_premise_kernel_view(&cx) else {
            panic!("a later-slot dependency must reject")
        };
        assert!(matches!(later_slot, ElabError::Internal(ref reason)
            if reason.contains("forward dependency")));
    }

    #[test]
    fn active_premise_view_relocates_context_entries_and_inferred_outputs() {
        // Promise class: durable invariant.
        // MEASURED: a lexical context-entry type containing an owned sentinel
        // becomes a dependency on the inserted premise, and inference maps that
        // dependency back to the owner-local sentinel at current growth.
        // CLAIMED: the gateway relocates the whole context and inverses inferred
        // types. THE GAP: translating only the named query leaves this context
        // entry invalid and cannot produce the asserted grown sentinel.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "context-relocation-control",
        );
        let region = 40;
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: region,
                premise_domains: vec![Term::Type(Level::Zero)],
                install_depth: 0,
            });
        cx.ctx.push(index_refinement_sentinel(region, 0));
        assert_eq!(
            kernel_infer_current(&cx, &Term::var(0)).expect("contextual inference"),
            index_refinement_sentinel(region, 1),
            "inverse translation must restore the exact owner-local growth"
        );

        cx.ctx.types[0] = index_refinement_sentinel(41, 0);
        let Err(unknown_context) = active_premise_kernel_view(&cx) else {
            panic!("an unknown sentinel in a context-entry type must reject")
        };
        assert!(matches!(unknown_context, ElabError::Internal(ref reason)
            if reason.contains("unknown sentinel or ordinary out-of-scope")));
    }

    #[test]
    fn current_inference_preserves_type_and_omega_classifier_kinds() {
        // Promise class: durable invariant.
        // MEASURED: the identical owned-sentinel query returns exact Type and
        // Omega classifiers, including their nonzero level. CLAIMED: contextual
        // inference preserves the kernel's classifier rather than guessing or
        // inferring a lambda/Pi surrogate. THE GAP: a Type-only positive cannot
        // discriminate a collapsed classifier kind or level.
        fn classify(domain: Term) -> Term {
            let mut env = ElabEnv::new().expect("base environment");

            let refinement_facts = super::RefinementFacts::default();
            let mut cx = ElabCtx::new(
                &mut env.env,
                &env.globals,
                &mut env.num_values,
                &env.numeric_env,
                &refinement_facts,
                "classifier-pair-control",
            );
            cx.active_index_premise_frames
                .push(ActiveIndexPremiseFrame {
                    sentinel_region: 50,
                    premise_domains: vec![domain],
                    install_depth: 0,
                });
            kernel_infer_current(&cx, &index_refinement_sentinel(50, 0))
                .expect("owned sentinel classifier")
        }

        let level = Level::Suc(Box::new(Level::Zero));
        assert_eq!(classify(Term::Type(level.clone())), Term::Type(level.clone()));
        assert_eq!(classify(Term::Omega(level.clone())), Term::Omega(level));
    }

    #[test]
    fn active_premise_frame_order_and_overflow_fail_closed() {
        // Promise class: durable invariant.
        // MEASURED: reversed install depths, an orphan result refinement, a
        // mismatched refinement depth, and an overflowing region all refuse,
        // while equal-depth outer-to-inner order is accepted above. CLAIMED:
        // the complete frame plan is validated before any query translation.
        // THE GAP: duplicate/escaped/slot checks exercise different metadata
        // axes and cannot catch these neighbours.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "frame-order-control",
        );
        cx.ctx.push(Term::Type(Level::Zero));
        cx.ctx.push(Term::Type(Level::Zero));
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 60,
                premise_domains: vec![],
                install_depth: 1,
            });
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 61,
                premise_domains: vec![],
                install_depth: 0,
            });
        let Err(nonnested) = active_premise_kernel_view(&cx) else {
            panic!("reversed install depths must reject")
        };
        assert!(matches!(nonnested, ElabError::Internal(ref reason)
            if reason.contains("not nested by install depth")));

        cx.active_index_premise_frames.clear();
        cx.result_refinements.push(ResultRefinement {
            index_ty: Term::Type(Level::Zero),
            concrete_index: Term::Type(Level::Zero),
            refined_index: Term::Type(Level::Zero),
            premise_slot: 0,
            sentinel_region: 62,
            install_depth: 0,
        });
        let Err(orphan) = active_premise_kernel_view(&cx) else {
            panic!("orphan result refinement must reject")
        };
        assert!(matches!(orphan, ElabError::Internal(ref reason)
            if reason.contains("has no active premise frame")));

        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 62,
                premise_domains: vec![Term::Type(Level::Zero)],
                install_depth: 1,
            });
        let Err(wrong_depth) = active_premise_kernel_view(&cx) else {
            panic!("mismatched refinement and frame depths must reject")
        };
        assert!(matches!(wrong_depth, ElabError::Internal(ref reason)
            if reason.contains("differs from its frame depth")));

        cx.result_refinements.clear();
        cx.active_index_premise_frames.clear();
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: usize::MAX,
                premise_domains: vec![Term::Type(Level::Zero)],
                install_depth: 0,
            });
        let Err(overflow) = active_premise_kernel_view(&cx) else {
            panic!("overflowing sentinel authority must reject")
        };
        assert!(matches!(overflow, ElabError::Internal(ref reason)
            if reason.contains("overflows the variable index")));

        cx.active_index_premise_frames.clear();
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 63,
                premise_domains: vec![Term::IntLit(0.into())],
                install_depth: 0,
            });
        let Err(ill_typed_domain) = active_premise_kernel_view(&cx) else {
            panic!("an ill-typed premise domain must reject")
        };
        assert!(matches!(ill_typed_domain, ElabError::Internal(ref reason)
            if reason.contains("classified by neither Type nor Omega")));
    }

    #[test]
    fn large_convoy_validation_returns_the_owner_preserving_base() {
        // Promise class: durable invariant.
        // MEASURED: large-convoy validation closes one active outer sentinel in
        // a kernel-checkable shadow but returns the zonked current-region base
        // with that sentinel unchanged. CLAIMED: this consumer never publishes
        // the disposable all-active wrapper. THE GAP: the surface large-convoy
        // grid has no nested outer result frame, so it cannot observe ownership.
        let mut env = ElabEnv::new().expect("base environment");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "large-convoy-return-control",
        );
        let region = 13;
        let domain = Term::Type(Level::Zero);
        let base = index_refinement_sentinel(region, 0);
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: region,
                premise_domains: vec![domain.clone()],
                install_depth: 0,
            });
        cx.result_refinements.push(ResultRefinement {
            index_ty: domain.clone(),
            concrete_index: domain.clone(),
            refined_index: domain.clone(),
            premise_slot: 0,
            sentinel_region: region,
            install_depth: 0,
        });

        let returned = validate_large_convoy_base(
            &cx,
            &base,
            &domain,
            &Span::new(0, 0),
        )
        .expect("the active-premise kernel view must validate");
        assert_eq!(returned, base, "the contextual view must never escape");
    }


    #[test]
    fn non_family_result_shape_still_uses_whole_index_fallback() {
        // Promise class: durable invariant.
        // MEASURED: transport changes a Sigma result whose first component is
        // indexed by the hidden equality, even though route (a) requires an
        // indexed IndFormer result head. CLAIMED: route-(a) inapplicability
        // falls immediately into the former whole-index transport. THE GAP:
        // the integration grid kernel-checks real dependent methods; this
        // focused pin isolates fallback reachability and its returned type.
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_decl(
            "data FallbackOut : Nat -> Type where { \
             FallbackMkOut : (index : Nat) -> FallbackOut index }",
        )
        .expect("FallbackOut");
        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let result_head = Term::IndFormer {
            id: env.globals["FallbackOut"],
            level_args: vec![],
        };
        let result_ctor = Term::Constructor {
            id: env.globals["FallbackMkOut"],
            level_args: vec![],
        };
        let recursive_id = env.globals["FallbackMkOut"];
        let old_index = Term::var(1);
        let new_index = Term::var(0);
        let inferred = Term::sigma(
            Term::app(result_head.clone(), old_index.clone()),
            nat.clone(),
        );
        let expected = Term::sigma(Term::app(result_head, new_index.clone()), nat.clone());
        let core = Term::pair(Term::app(result_ctor, old_index.clone()), old_index.clone());
        let span = Span::new(0, 0);
        let expression = RExpr::RCon("FallbackMkOut".into(), span);

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "fallback-control",
        );
        cx.ctx.push(nat.clone());
        cx.ctx.push(nat.clone());
        cx.recursive_group.insert(recursive_id);
        let premise = Term::Eq(
            Box::new(nat.clone()),
            Box::new(old_index.clone()),
            Box::new(new_index.clone()),
        );
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 0,
                premise_domains: vec![premise],
                install_depth: cx.ctx.len(),
            });
        cx.result_refinements
            .push(refinement(nat, old_index, new_index, 0, cx.ctx.len()));

        let (_, transported_ty) =
            transport_recursive_group_call_result(&cx, &expression, core, inferred, &expected)
                .expect("fallback transport must remain well-formed")
                .expect("non-route-(a) result must be transported by the fallback");
        assert!(convert_type(cx.env, &cx.ctx, &transported_ty, &expected));
    }

    #[test]
    fn two_active_result_refinements_compose_cumulatively() {
        // Promise class: durable invariant.
        // MEASURED: two active hidden equalities independently move the two
        // indices of one recursive result to its expected type. CLAIMED: each
        // successful row commits to cumulative transport state consumed by the
        // next row. THE GAP: each row alone leaves one index mismatched; the
        // final conversion assertion makes discarding either row observable.
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_decl(
            "data DoubleOut : Nat -> Nat -> Type where { \
             DoubleMkOut : (first : Nat) -> (second : Nat) \
               -> DoubleOut first second }",
        )
        .expect("DoubleOut");
        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let result_head = Term::IndFormer {
            id: env.globals["DoubleOut"],
            level_args: vec![],
        };
        let result_ctor = Term::Constructor {
            id: env.globals["DoubleMkOut"],
            level_args: vec![],
        };
        let recursive_id = env.globals["DoubleMkOut"];
        let first_old = Term::var(3);
        let first_new = Term::var(2);
        let second_old = Term::var(1);
        let second_new = Term::var(0);
        let inferred = app2(result_head.clone(), first_old.clone(), second_old.clone());
        let expected = app2(result_head, first_new.clone(), second_new.clone());
        let core = app2(result_ctor, first_old.clone(), second_old.clone());
        let span = Span::new(0, 0);
        let expression = RExpr::RCon("DoubleMkOut".into(), span);

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "cumulative-control",
        );
        for _ in 0..4 {
            cx.ctx.push(nat.clone());
        }
        cx.recursive_group.insert(recursive_id);
        let premises = vec![
            Term::Eq(
                Box::new(nat.clone()),
                Box::new(second_old.clone()),
                Box::new(second_new.clone()),
            ),
            Term::Eq(
                Box::new(nat.clone()),
                Box::new(first_old.clone()),
                Box::new(first_new.clone()),
            ),
        ];
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: 0,
                premise_domains: premises,
                install_depth: cx.ctx.len(),
            });
        cx.result_refinements.push(refinement(
            nat.clone(),
            second_old,
            second_new,
            0,
            cx.ctx.len(),
        ));
        cx.result_refinements
            .push(refinement(nat, first_old, first_new, 1, cx.ctx.len()));

        let (_, transported_ty) =
            transport_recursive_group_call_result(&cx, &expression, core, inferred, &expected)
                .expect("cumulative transport must remain well-formed")
                .expect("both active result refinements must compose");
        assert!(convert_type(cx.env, &cx.ctx, &transported_ty, &expected));
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
        check as kernel_check_raw, declare_postulate, infer as kernel_infer_raw, Context, KernelError,
        Level, Term,
    };

    use super::{
        check_match_with_lift, discharge_reflexive_recursive_ih_evidence, infer,
        install_lift_binding, lift_association_error, method_type, peel_pi,
        validate_lift_associations, whnf, ActiveIndexPremiseFrame, ElabCtx, ElabError,
        GlobalId, HashMap, LiftAssociationFailure, LiftBinding, ResultRefinement, Span,
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

    #[test]
    fn lifted_dispatch_rejects_a_foreign_constructor_before_reading_lift_evidence() {
        // Promise class: durable invariant. Intended extensions may add callers
        // or lift-evidence shapes; every direct lifted dispatch must still
        // reject an arm from a different family before reading its evidence.
        // MEASURED: a direct call reaches `check_match_with_lift` with a Bool
        // arm against the Nat host and no support binding, and reports the
        // family mismatch rather than the later missing-evidence error.
        // CLAIMED: the lifted dispatch guards its own complete arm population.
        // THE GAP: ordinary source reaches this dispatch through its currently
        // guarded sole caller; this direct control exists to cover future callers.
        let mut env = ElabEnv::new().expect("base environment");
        let host_id = env.globals["Nat"];
        let host = env.env.inductive(host_id).expect("Nat declaration").clone();
        let parsed = parse_expr("match Zero { True |-> Zero }").expect("match parses");
        let RExpr::RMatch { arms, span, .. } =
            resolve_expr_standalone(&parsed).expect("match resolves")
        else {
            panic!("expected a resolved match")
        };
        let expected = Term::IndFormer {
            id: host_id,
            level_args: vec![],
        };
        let scrutinee = Term::Constructor {
            id: env.globals["Zero"],
            level_args: vec![],
        };

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "lift-dispatch-self-guard-control",
        );

        let error = check_match_with_lift(
            &mut cx,
            &arms,
            &expected,
            &span,
            &scrutinee,
            &host,
            &[],
            &[],
            binding(0, None, None),
        )
        .expect_err("a Bool constructor must be foreign to the Nat host");

        match error {
            ElabError::TypeMismatch { reason, .. } => assert_eq!(
                reason,
                "constructor 'True' is not a constructor of type 'Nat'"
            ),
            other => panic!("expected the family-mismatch diagnostic, got {other:?}"),
        }
    }

    #[test]
    fn completed_lifted_eliminator_validation_closes_outer_result_premise() {
        // Promise class: durable invariant.
        // MEASURED: real lifted-match lowering checks generated methods whose
        // recursive-group calls are transported from `local` to `outer`; its
        // completed generated-All eliminator validates while that outer premise
        // remains active. CLAIMED: the second, completed-eliminator validation
        // uses the active-premise kernel view and returns the original
        // eliminator. THE GAP: the per-method checks already use contextual
        // views and therefore cannot detect omission of this distinct final
        // consumer.
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_decl(
            "data ShadowRose : Type where { \
               ShadowLeaf : ShadowRose; \
               ShadowNode : List ShadowRose -> ShadowRose \
             }",
        )
        .expect("ShadowRose");
        env.elaborate_decl(
            "data ShadowOut : Nat -> Type where { \
               ShadowMkOut : (index : Nat) -> ShadowOut index \
             }",
        )
        .expect("ShadowOut");
        env.elaborate_decl(
            "fn shadow_sibling (index : Nat) (rose : ShadowRose) \
               : ShadowOut index = ShadowMkOut index",
        )
        .expect("shadow_sibling");
        let sibling = env.globals["shadow_sibling"];
        env.globals.insert("ShadowSibling".into(), sibling);

        let parsed = parse_expr(
            "\\outer . \\local . \\members . match members { \
               Nil ↦ ShadowSibling local ShadowLeaf; \
               Cons member rest ↦ ShadowSibling local ShadowLeaf \
             }",
        )
        .expect("lifted body parses");
        let mut resolved = resolve_expr_standalone(&parsed).expect("lifted body resolves");
        for _ in 0..3 {
            let RExpr::RLam(_, body, _) = resolved else {
                panic!("expected three setup lambdas")
            };
            resolved = *body;
        }
        let RExpr::RMatch { arms, span, .. } = resolved else {
            panic!("setup lambdas must contain the lifted List match")
        };

        let nat = Term::IndFormer {
            id: env.globals["Nat"],
            level_args: vec![],
        };
        let rose = Term::IndFormer {
            id: env.globals["ShadowRose"],
            level_args: vec![],
        };
        let result = Term::IndFormer {
            id: env.globals["ShadowOut"],
            level_args: vec![],
        };
        let region = 17;
        let premise = Term::Eq(
            Box::new(nat.clone()),
            Box::new(Term::var(2)),
            Box::new(Term::var(3)),
        );

        // Reproduce the source-field and generated-evidence domains that the
        // enclosing ShadowNode method installs, but call the lifted List
        // consumer directly so this pin ends at its completed-eliminator seam.
        let rose_decl = env
            .env
            .inductive(env.globals["ShadowRose"])
            .expect("ShadowRose declaration")
            .clone();
        let zero = Term::Constructor {
            id: env.globals["Zero"],
            level_args: vec![],
        };
        let expected_at_install = Term::app(result.clone(), zero);
        let closed_motive = Term::Ascript(
            Box::new(Term::lam(
                rose.clone(),
                ken_kernel::subst::weaken(&expected_at_install, 1),
            )),
            Box::new(Term::pi(rose.clone(), Term::Type(Level::Zero))),
        );
        let node_method_ty = method_type(
            &env.env,
            &rose_decl,
            1,
            &closed_motive,
            &[],
            &[],
        )
        .expect("ShadowNode method type");
        let (node_domains, _) = peel_pi(&node_method_ty);
        assert_eq!(node_domains.len(), 2, "field plus generated evidence");

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "completed-lifted-shadow-control",
        );
        cx.ctx.push(nat.clone());
        cx.ctx.push(nat.clone());
        cx.recursive_group.insert(sibling);

        let base = cx.ctx.len();
        for (position, raw_domain) in node_domains.iter().enumerate() {
            let domain = whnf(cx.env, &cx.ctx, raw_domain);
            cx.ctx.push(domain);
            if position > 0 {
                cx.hidden_positions.push(base + position);
            }
        }
        cx.active_index_premise_frames
            .push(ActiveIndexPremiseFrame {
                sentinel_region: region,
                premise_domains: vec![premise],
                install_depth: cx.ctx.len(),
            });
        cx.result_refinements.push(ResultRefinement {
            index_ty: nat,
            concrete_index: Term::var(2),
            refined_index: Term::var(3),
            premise_slot: 0,
            sentinel_region: region,
            install_depth: cx.ctx.len(),
        });
        let installed = install_lift_binding(&mut cx, base, base + 1, None)
            .expect("generated lift binding");
        let members = cx
            .binding_term(base)
            .expect("members source binding")
            .0;
        let list = cx
            .env
            .inductive(cx.globals["List"])
            .expect("List declaration")
            .clone();
        let expected = Term::app(result, Term::var(3));

        let checked = check_match_with_lift(
            &mut cx,
            &arms,
            &expected,
            &span,
            &members,
            &list,
            &[],
            &[rose],
            installed,
        )
        .expect("the contextual view must validate the completed generated-All eliminator");
        assert!(
            matches!(checked, Term::Elim { .. }),
            "the lifted match must return its original eliminator"
        );
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
            kernel_infer_raw(&env.env, &ctx, &specialized).unwrap(),
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
            kernel_infer_raw(&env.env, &ctx, &specialized).unwrap(),
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

        let refinement_facts = super::RefinementFacts::default();
        let mut cx = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
            "recursiveOwner",
        );
        cx.ctx.push(nat.clone());
        cx.ctx.push(evidence_ty.clone());
        cx.hidden_positions.push(1);
        cx.lift_bindings
            .insert(0, binding(1, recursive_result_position, None));

        let (core, ty) = infer(&mut cx, &expression).unwrap();
        assert_eq!(ty, nat);
        assert_eq!(kernel_infer_raw(cx.env, &cx.ctx, &core).unwrap(), ty);
        let Term::App(specialized, ordinary_argument) = core else {
            panic!("ordinary source argument must remain the outer application");
        };
        assert_eq!(*ordinary_argument, zero.clone());
        let Term::App(raw_evidence, _) = *specialized else {
            panic!("generated equality evidence must be applied exactly once first");
        };
        assert_eq!(*raw_evidence, Term::var(0));

        assert!(
            kernel_infer_raw(cx.env, &cx.ctx, &Term::app(Term::var(0), zero),).is_err(),
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
            kernel_infer_raw(&env.env, &explicit_ctx, &explicitly_transported).unwrap(),
            nat,
            "neutral evidence must remain an explicit argument rather than becoming definitional",
        );
        assert!(
            kernel_check_raw(
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

        let refinement_facts = super::RefinementFacts::default();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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

        let refinement_facts = super::RefinementFacts::default();
        let mut selected = ElabCtx::new(
            &mut env.env,
            &env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &refinement_facts,
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

/// Grouped IH positions are derived from the kernel method type.
#[cfg(test)]
mod nested_method_alias_frame_tests {
    use super::{assert_nested_method_alignment, nested_method_ih_positions};
    use crate::{ElabEnv, ElabError};
    use ken_kernel::{inductive::method_type, Level, Term};

    #[test]
    fn grouped_recursive_ih_domains_match_nested_method_telescope() {
        // Durable invariant: grouped IH positions, not field arithmetic,
        // determine which method domains are IHs.
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_file(
            "data Branch : Type where { Leaf : Branch; Node : Branch → Nat → Branch → Branch }",
        ).expect("binary constructor");
        let branch = env.globals["Branch"];
        let ind = env.env.inductive(branch).expect("checked inductive");
        let motive = Term::lam(Term::indformer(branch, vec![]), Term::ty(Level::Zero));
        let method = method_type(&env.env, ind, 1, &motive, &[], &[])
            .expect("checked method type");
        let positions = nested_method_ih_positions(&method, &motive, 5);
        assert_eq!(positions, [3, 4]);
        assert!(assert_nested_method_alignment(ind, 1, &positions, 3, 2, 0, 5).is_ok());
        assert!(matches!(
            assert_nested_method_alignment(ind, 1, &[2, 4], 3, 2, 0, 5),
            Err(ElabError::Internal(reason)) if reason.contains("misaligned")
        ));
    }
}

/// `LANG-MATCH-MATRIX-OCCURRENCE-THREADING` white-box controls. These exercise
/// the occurrence carrier directly because the first black-box value-binding
/// consumer is the following as-pattern slice.
#[cfg(test)]
mod match_matrix_occurrence_tests {
    use super::{
        begin_match_occurrence_trace, take_match_occurrence_trace, MatchBinderOrigin,
        MatrixOccurrence, RowState,
    };
    use crate::{
        error::Span,
        resolve::{RPatKind, RPattern},
        ElabEnv,
    };
    use ken_kernel::Term;

    fn pat(kind: RPatKind) -> RPattern {
        RPattern {
            kind,
            span: Span::zero(),
        }
    }

    fn row(pattern: RPattern, occurrence: Term) -> RowState {
        RowState {
            real_pats: vec![pattern],
            real_occurrences: vec![MatrixOccurrence::live(occurrence)],
            binding_occurrences: Vec::new(),
            virtual_surface_positions: Vec::new(),
            virtual_aliases: Vec::new(),
            row_hidden_surface_positions: Vec::new(),
            arm_idx: 0,
        }
    }

    #[test]
    fn two_level_split_rebases_top_nested_child_and_tail_occurrences() {
        // Promise class: durable invariant. MEASURED: the production RowState
        // transitions carry four independently-originating occurrences under
        // exactly the binders introduced by a two-level split, including an
        // intervening synthetic IH. CLAIMED: each value-binding consumer sees
        // the core term for its own position at leaf depth. THE GAP: this is a
        // white-box carrier proof; the as-pattern consumer supplies the
        // independent black-box elaboration proof on the next node.
        let outer = pat(RPatKind::Ctor(
            "Outer".into(),
            vec![
                pat(RPatKind::Ctor(
                    "Inner".into(),
                    vec![pat(RPatKind::Var("child".into(), None))],
                )),
                pat(RPatKind::Var("tail".into(), None)),
            ],
        ));

        // Model a top-level alias by recording the already-live scrutinee
        // occurrence before stripping Outer. Outer contributes two pending
        // field columns; neither is live until its own method binder is
        // entered.
        let mut state = row(outer, Term::var(0)).bind_current_occurrence();
        let outer_subpatterns = match &state.real_pats[0].kind {
            RPatKind::Ctor(_, subpatterns) => subpatterns.clone(),
            _ => unreachable!(),
        };
        state = state.specialize_current_column(outer_subpatterns, true);
        assert!(state
            .real_occurrences
            .iter()
            .all(|occurrence| { occurrence.term == Term::var(0) && !occurrence.live }));

        // Enter the first outer field, whose pattern itself splits. Recording
        // it models an alias at this nested constructor position.
        state = state.enter_current_real_binder().bind_current_occurrence();
        let inner_subpatterns = match &state.real_pats[0].kind {
            RPatKind::Ctor(_, subpatterns) => subpatterns.clone(),
            _ => unreachable!(),
        };
        state = state.specialize_current_column(inner_subpatterns, true);

        // The nested child is an ordinary source binder.
        state = state
            .enter_current_real_binder()
            .bind_current_occurrence()
            .drop_current_column();
        // One synthetic IH intervenes between the nested child and the outer
        // sibling. It has no source pattern, but every live occurrence must
        // move under it.
        state = state.under_core_binder();
        // The outer sibling becomes the final fresh field binder.
        state = state
            .enter_current_real_binder()
            .bind_current_occurrence()
            .drop_current_column();

        assert!(state.real_pats.is_empty());
        assert_eq!(
            state.leaf_binding_occurrences(),
            &[
                Some(Term::var(4)),
                Some(Term::var(3)),
                Some(Term::var(2)),
                Some(Term::var(0)),
            ],
            "top scrutinee, nested constructor, nested child, and tail must each \
             name their own binder after two splits plus the intervening IH"
        );
    }

    #[test]
    fn production_two_level_recursive_split_rebases_binding_under_both_ihs() {
        // Promise class: durable invariant. MEASURED: a real general-matrix
        // compilation reaches the production leaf with `m`'s occurrence under
        // the nested and enclosing recursive IH binders. CLAIMED: D3 rebases
        // live occurrences at every actual matrix descent. THE GAP: tracing is
        // test-only observation of the production carrier; the returned value
        // independently proves the unchanged Var consumer still selects `m`.
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_decl("data NatL = LZero | LSucc NatL")
            .expect("recursive fixture type elaborates");
        let trusted_before = env.env.trusted_base();

        begin_match_occurrence_trace();
        env.elaborate_decl(
            "let result : NatL = match LSucc (LSucc LZero) { \
             LZero |-> LZero ; LSucc LZero |-> LZero ; LSucc (LSucc m) |-> m }",
        )
        .expect("two-level nested match elaborates through the general matrix");
        let trace = take_match_occurrence_trace();
        assert_eq!(
            env.env.trusted_base(),
            trusted_before,
            "occurrence bookkeeping must not add a trusted declaration"
        );

        let lzero = Term::Constructor {
            id: env.globals["LZero"],
            level_args: Vec::new(),
        };
        let lsucc = Term::Constructor {
            id: env.globals["LSucc"],
            level_args: Vec::new(),
        };
        let expected_scrutinee = Term::app(lsucc.clone(), Term::app(lsucc, lzero));
        assert_eq!(
            trace.seeds.len(),
            3,
            "the three source arms must each seed one aligned top-level occurrence"
        );
        assert!(
            trace.seeds.iter().all(|seed| seed == &expected_scrutinee),
            "every initial row must carry the inferred top-level scrutinee occurrence; \
             seeds were {:?}",
            trace.seeds
        );
        assert!(
            trace
                .leaves
                .iter()
                .any(|occurrences| occurrences == &[Some(Term::var(2))]),
            "the nested `m` binder must cross its own and its enclosing LSucc IH; \
             production leaf trace was {:?}",
            trace.leaves
        );
    }

    #[test]
    fn pending_sibling_is_not_weakened_before_its_binder_exists() {
        // Promise class: durable invariant. MEASURED: entering a binder weakens
        // a live occurrence and an already-supplied binding, while a future
        // sibling stays at its prospective Var(0). CLAIMED: occurrence
        // rebasing follows actual binder depth rather than eagerly counting a
        // constructor's whole field list. THE GAP: the two-level test above
        // proves the same distinction composes through a real split shape.
        let state = RowState {
            real_pats: vec![
                pat(RPatKind::Var("live".into(), None)),
                pat(RPatKind::Var("future".into(), None)),
            ],
            real_occurrences: vec![
                MatrixOccurrence::live(Term::var(2)),
                MatrixOccurrence::pending_field(true, true, MatchBinderOrigin::Field),
            ],
            binding_occurrences: vec![Some(Term::var(1))],
            virtual_surface_positions: Vec::new(),
            virtual_aliases: Vec::new(),
            row_hidden_surface_positions: Vec::new(),
            arm_idx: 0,
        }
        .under_core_binder();

        assert_eq!(state.real_occurrences[0].term, Term::var(3));
        assert_eq!(state.binding_occurrences, vec![Some(Term::var(2))]);
        assert_eq!(state.real_occurrences[1].term, Term::var(0));
        assert!(!state.real_occurrences[1].live);
    }
}

/// The known check-mode goal must reach the nested matrix before its first
/// leaf, while inference mode still discovers that goal from a leaf.
#[cfg(test)]
mod match_matrix_result_seed_tests {
    use super::{begin_match_occurrence_trace, take_match_occurrence_trace};
    use crate::ElabEnv;
    use ken_kernel::{normalize, Context, Term};

    #[test]
    fn checked_nested_split_is_seeded_before_bucket_but_inference_is_not() {
        // Promise class: durable invariant. MEASURED: the production matrix
        // sees Some before a nested split in check mode and None on another
        // nested split in inference mode. CLAIMED: check-mode R is available before
        // building nested buckets without inventing an inference-mode result.
        // THE GAP: this trace witnesses entry state, not the future derived
        // telescope; both declarations kernel-check: checked NatL returns
        // LZero, while inference-mode NatBox returns Zero : Nat.
        let mut env = ElabEnv::new().expect("prelude");
        // Put the nested constructor first: no earlier root method may infer
        // R before the split whose entry state this test measures.
        env.elaborate_decl("data NatL = LSucc NatL | LZero")
            .expect("recursive family");
        let trusted = env.env.trusted_base();
        let matrix = "match LSucc (LSucc LZero) { \
                      LZero |-> LZero; LSucc LZero |-> LZero; \
                      LSucc (LSucc m) |-> m }";

        begin_match_occurrence_trace();
        env.elaborate_decl(&format!("let checked_slot : NatL = {matrix}"))
            .expect("checked nested match");
        let checked = take_match_occurrence_trace();
        assert_eq!(checked.nested_return_types, [true]);

        begin_match_occurrence_trace();
        env.elaborate_file(
            "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
             fn inferred_slot (b : NatBox) : Nat = let r = match b { \
               BoxNat Zero ↦ Zero; BoxNat (Suc n) ↦ n \
             } in r",
        )
        .expect("inferred nested match");
        let inferred = take_match_occurrence_trace();
        assert_eq!(inferred.nested_return_types, [false]);

        env.elaborate_decl("const inferred_value : Nat = inferred_slot (BoxNat (Suc Zero))")
            .expect("apply inferred match");
        let normal = |name: &str| {
            let body = env.env.transparent_body(env.globals[name]).expect("body").1;
            normalize(&env.env, &Context::new(), &body)
        };
        let nat_zero = Term::Constructor {
            id: env.globals["Zero"],
            level_args: Vec::new(),
        };
        assert_eq!(normal("inferred_value"), nat_zero);
        assert_eq!(
            normal("checked_slot"),
            Term::Constructor {
                id: env.globals["LZero"],
                level_args: Vec::new(),
            }
        );
        assert_eq!(env.env.trusted_base(), trusted);
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

        let refinement_facts = super::RefinementFacts::default();
        let cx = ElabCtx::new(
            &mut elab.env,
            &elab.globals,
            &mut elab.num_values,
            &elab.numeric_env,
            &refinement_facts,
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

#[cfg(test)]
mod surf1_visits_row_production_path {
    //! SURF-1 D1's two production-path rows, RELOCATED FROM
    //! `tests/effects.rs` rather than rewritten (language-leader,
    //! `evt_66h8wqeaffnfg`).
    //!
    //! **They pin the `elaborate_rdecl_v1` hook deliberately** -- their own
    //! doc said "if the hook is removed, this fails with `None`" -- so the
    //! cheaper rewrite through a public `elaborate_file` path was refused: it
    //! would keep the assertion and drop the thing being asserted.
    //!
    //! They live here because that function is `pub(crate)`, and it is
    //! `pub(crate)` because its signature names `StandardOperatorRole`, which
    //! `lib.rs`'s `deny(private_interfaces)` keeps off the public surface for
    //! the membership track. An integration test cannot reach it. **In-crate
    //! they are compiled by `--lib`**, which runs every increment -- the
    //! version in `tests/` was not compiled by any targeted selection and sat
    //! broken, undelivered, for the branch's whole length.
    use super::*;
    use crate::effects::RowType;
    use crate::parser::parse_decls;
    use crate::resolve::resolve_decl;

    /// The certified standard operators these fixtures need: NONE -- CHOSEN
    /// AND STATED, not defaulted.
    ///
    /// **And the choice self-checks.** "These fixtures contain no comparison
    /// operator" is a property of the fixtures, not a fact about the world, so
    /// it is asserted rather than assumed: a fixture that later grows a `≤`
    /// reds here instead of being silently refused by an empty map.
    fn no_standard_operators(src: &str) -> HashMap<StandardOperatorRole, GlobalId> {
        for role in StandardOperatorRole::ALL {
            assert!(
                !src.contains(role.glyph()),
                "this fixture now contains `{}`, so an EMPTY certified map is \
                 no longer the right choice for it -- supply the role or split \
                 the fixture",
                role.glyph()
            );
        }
        HashMap::new()
    }

    /// SURF-1 D1 production path: real `RDeclKind::View` elaboration consumes
    /// the parsed concrete `visits` row and records the checked `RowType`. If
    /// the `elaborate_rdecl_v1` hook is removed, this fails with `None`.
    #[test]
    fn surf1_view_elaboration_consumes_visits_row() {
        let src = "proc surf1_visits (x : Nat) : Nat visits [Console] = x";
        let decls = parse_decls(src).expect("const with concrete visits row must parse");
        let rdecl = resolve_decl(&decls[0]).expect("const with concrete visits row must resolve");
        let mut env = crate::ElabEnv::new().expect("base env");
        let standard_operators = no_standard_operators(src);

        let result = elaborate_rdecl_v1(
            &mut env.env,
            &mut env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &mut env.class_env,
            &mut env.resolution_provenance,
            &standard_operators,
            &rdecl,
        )
        .expect("const with concrete D1 visits row must elaborate");

        let row = result
            .effect_row_type
            .expect("production const elaboration must expose checked visits row");
        assert_eq!(
            row,
            RowType::singleton("Console"),
            "written [Console] must reach production checking as a RowType"
        );
    }

    /// SURF-1 D1 production path: row variables fail closed unless the same
    /// variable was allocated from a HOF latent-row binding in the declaration
    /// type. A plain first-order const must not synthesize `e` from `visits`.
    #[test]
    fn surf1_view_elaboration_rejects_unbound_visits_row_var() {
        let src = "proc surf1_bad_visits (x : Nat) : Nat visits [Console | e] = x";
        let decls = parse_decls(src).expect("const with open visits row must parse");
        let rdecl = resolve_decl(&decls[0]).expect("const with open visits row must resolve");
        let mut env = crate::ElabEnv::new().expect("base env");
        let standard_operators = no_standard_operators(src);

        let err = elaborate_rdecl_v1(
            &mut env.env,
            &mut env.globals,
            &mut env.num_values,
            &env.numeric_env,
            &mut env.class_env,
            &mut env.resolution_provenance,
            &standard_operators,
            &rdecl,
        )
        .expect_err("unbound visits row variable must reject fail-closed");

        assert!(
            format!("{err:?}").contains("unknown row variable `e` in visits row"),
            "unexpected error for unbound row variable: {err:?}"
        );
    }
}
