//! V3 FO Kripke embedding, route (a) first vertical slice (`23-prover.md §4.5`,
//! `V3-FO-KRIPKE-SLICE`).
//!
//! Builds exactly the slice `§4.5` fixes and nothing wider: **one** rigid
//! object sort `A`, **one** unary uninterpreted predicate `P : A -> Omega`,
//! source forms `Bottom`/atom/`or`/`imp`/`forall`, and certificate rules
//! `init`/`imp-right`/`forall-right`. `IForm`/`Form`/`Cert` here are the
//! slice's CONSTRUCTOR SUBSET, not the full `§4.3` vocabulary (which has
//! `top`/`and`/`exists` on the source side and ~20 `Cert` rules) -- widening
//! either is explicitly out of this node's scope.
//!
//! **`IForm`/`Form`/`Cert`/`check_cert` are plain Rust data and a Rust-level
//! total function** (`23 §4.3`: "This is a Ken-level total function over
//! ordinary derived data, distinct from the kernel API `check`"), not Ken
//! kernel-checked terms. Their eventual kernel-checked home -- and the two
//! theorems `embedding_adequacy`/`checker_soundness` that would let route FO
//! return `Proved` -- is the reserved `§4.4` Architect/operator placement
//! decision. This module does not make that decision and does not return
//! `Proved` anywhere (`23 §4.4`, `V3-FO-KRIPKE-SLICE` `AC-5`/`AC-6`).
//!
//! `Carriers`/`AtomEnv`/`denote` DO reach into genuine Ken `Term`s: the one
//! sort and one predicate this slice quotes over are real closed rigid Ken
//! data, declared once per signature via [`declare_fo_slice_signature`].
//!
//! **Scope note on "or":** the prelude's own `Or`/`Inl`/`Inr` family
//! (`prelude.rs`) is registered only in `ElabEnv::globals`, not reachable
//! from a bare `GlobalEnv` (which is all this module and `prover.rs` are
//! plumbed with). This module therefore declares its own prover-owned
//! `Or`-shaped family per [`FoSliceSignature`] (same `declare_inductive`
//! shape as the prelude's), used consistently by both term construction and
//! `quote_fo`'s recognition within one signature. Recognizing the prelude's
//! own `Or` for arbitrary real obligations is later integration work, not
//! this slice's.

use ken_kernel::{
    check::{declare_inductive, CtorSpec, InductiveSpec},
    declare_postulate,
    subst::shift,
    GlobalEnv, GlobalId, Level, Term,
};

// ─── Signature: the slice's one-sort, one-predicate vocabulary (`23 §4.1`) ──

/// The slice's genuine Ken-level object sort `A`, predicate `P : A -> Omega`,
/// and this module's own `Or`-shaped family (see module doc). One value of
/// this type names one fixed signature; `quote_fo`/`embed`/term construction
/// must all be called against the SAME instance to agree on identities.
#[derive(Debug, Clone)]
pub struct FoSliceSignature {
    /// The closed, rigid Ken type for sort `A` -- an opaque postulate.
    pub sort_a: Term,
    /// `P : A -> Omega`, a postulate.
    pub pred_p: GlobalId,
    /// This module's own two-constructor `Or`-shaped family (`Inl`/`Inr`
    /// implicit as `constructors[0]`/`[1]`; the slice never constructs a
    /// value of it, only the type-former application quotation recognizes).
    pub or_id: GlobalId,
}

/// Declare a fresh [`FoSliceSignature`]: one rigid sort `A : Type 0`, one
/// unary predicate `P : A -> Omega 0`, and one prover-owned `Or`-shaped
/// family, all via `declare_postulate`/`declare_inductive` -- ordinary
/// kernel admissions, kernel-rechecked, zero `trusted_base()` delta beyond
/// the two postulates themselves (which is the cost of naming an abstract
/// sort/predicate at all, identical in kind to any other `postulate`).
///
/// No new kernel primitive, no trusted axiom (`23 §4.4`, `AC-5`): this is
/// the same `declare_postulate`/`declare_inductive` idiom `capabilities.rs`'s
/// `discharge_attenuation` and the prelude's own `Or`/`Perm_rel` already use
/// for internal, non-surface-syntax term construction.
pub fn declare_fo_slice_signature(env: &mut GlobalEnv) -> FoSliceSignature {
    let a_id = declare_postulate(env, "FO slice sort A".to_string(), vec![], Term::Type(Level::zero()))
        .expect("declare_postulate for FO slice sort A must succeed");
    let sort_a = Term::const_(a_id, vec![]);

    let pred_p = declare_postulate(
        env,
        "FO slice predicate P".to_string(),
        vec![],
        Term::pi(sort_a.clone(), Term::Omega(Level::zero())),
    )
    .expect("declare_postulate for FO slice predicate P must succeed");

    let omega0 = Term::Omega(Level::zero());
    let or_id = declare_inductive(env, |_or_id| InductiveSpec {
        level_params: vec![],
        // Params innermost-first: `a` = Var(1), `b` = Var(0), matching the
        // prelude's own `Or` declaration (`prelude.rs`) exactly.
        params: vec![omega0.clone(), omega0.clone()],
        indices: vec![],
        level: Level::Zero,
        constructors: vec![
            // Inl : a -> Or a b
            CtorSpec { args: vec![Term::Var(1)], target_indices: vec![] },
            // Inr : b -> Or a b
            CtorSpec { args: vec![Term::Var(0)], target_indices: vec![] },
        ],
    })
    .expect("declare_inductive for FO slice Or family must succeed");

    FoSliceSignature { sort_a, pred_p, or_id }
}

// ─── D0: quoted source data (`23 §4.3`, slice subset) ──────────────────────

/// A bound source object variable, de Bruijn (`23 §4.3` `IVar`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IVar(pub usize);

/// Quoted intuitionistic source formula -- SLICE CONSTRUCTOR SUBSET
/// (`23 §4.5`: `Bottom`, atom, `or`, `imp`, `forall`). The general `§4.3`
/// `IForm` also has `top`/`and`/`exists`; this slice's source grammar does
/// not include them (though `Form`, the classical target, needs `and` --
/// see below -- because `K(Sigma)`'s own axioms use it, independent of the
/// source fragment).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IForm {
    Bottom,
    /// `P x` for the slice's one, unary predicate.
    Atom(IVar),
    Or(Box<IForm>, Box<IForm>),
    Imp(Box<IForm>, Box<IForm>),
    /// `forall x : A. p` for the slice's one sort.
    Forall(Box<IForm>),
}

// ─── D0: quoted target data (`23 §4.3`, slice subset) ───────────────────────

/// A quoted-target variable, de Bruijn bound or a certificate-local free
/// eigenparameter (`23 §4.3` `QTerm`). The slice folds `QSort` (world vs.
/// object) into which `Form`/`Rule` variant is used rather than carrying it
/// on `QTerm` itself -- correct by construction here because every relation
/// (`Access`/`DomainA`/`ForcingP`) and quantifier (`ForallWorld`/`ForallObj`)
/// already fixes which sort each slot is; a general multi-sort `QSort` tag
/// is unneeded generality for a one-object-sort slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QTerm {
    Bound(usize),
    Parameter(usize),
}

/// Target classical formula -- SLICE CONSTRUCTOR SUBSET (`23 §4.5`):
/// `bottom`, relation, `and`, `or`, `imp`, `forall`. `and` is needed even
/// though the SOURCE slice has no `and`: `K(Sigma)`'s own axioms (`23 §4.2`)
/// use it in their antecedents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Form {
    Bottom,
    /// `Le w v` (`23 §4.2`'s `Le : World -> World -> Prop`).
    Access(QTerm, QTerm),
    /// `Dom_A w x`.
    DomainA(QTerm, QTerm),
    /// `Force_P w x` -- the slice's one, unary predicate's forcing relation.
    ForcingP(QTerm, QTerm),
    And(Box<Form>, Box<Form>),
    Or(Box<Form>, Box<Form>),
    Imp(Box<Form>, Box<Form>),
    /// `forall v : World. p`.
    ForallWorld(Box<Form>),
    /// `forall x : Obj(A). p`.
    ForallObj(Box<Form>),
}

/// A sequent `Gamma => Delta` (`23 §4.3`). The general contract uses
/// canonical MULTISETS (exchange is representation equality); the slice's
/// three rules never weaken/contract/exchange, so an insertion-order `Vec`
/// loses nothing here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequent {
    pub gamma: Vec<Form>,
    pub delta: Vec<Form>,
}

/// Certificate rule tags -- SLICE SUBSET ONLY: `init`, `imp-right`,
/// `forall-right` (`23 §4.5`: "The positive proof needs exactly [these]
/// certificate rules"). The general `§4.3` `Rule` has ~20 variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// Closes when the same formula occurs at both indices.
    Init { left: usize, right: usize },
    /// For right `p => q` at `right`, the sole child is `Gamma,p => Delta'`
    /// with `q` at `right`'s old position.
    ImpRight { right: usize },
    /// For right `forall S p` at `right`, the sole child instantiates `p`
    /// with a fresh eigenparameter, never a witness drawn from context --
    /// this is what stops the calculus inventing an object-sort inhabitant
    /// (`23 §4.3`).
    ForallRight { right: usize, eigen: QTerm },
}

/// A certificate proof-tree node (`23 §4.3` `Cert`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cert {
    pub conclusion: Sequent,
    pub rule: Rule,
    pub children: Vec<Cert>,
}

// ─── D0: Carriers / AtomEnv / denote (`23 §4.4`) ────────────────────────────

/// The slice's one-sort carrier family: the genuine closed rigid Ken type
/// for `A` (`23 §4.4` `Carriers`, restricted to the slice's one sort).
#[derive(Debug, Clone)]
pub struct Carriers {
    pub sort_a: Term,
}

/// The slice's one-predicate atom environment: `P`'s `GlobalId`, interpreted
/// as a genuine Ken proposition over `Carriers::sort_a` (`23 §4.4` `AtomEnv`,
/// restricted to the slice's one predicate).
#[derive(Debug, Clone)]
pub struct AtomEnv {
    pub pred_p: GlobalId,
}

/// `denote : Carriers -> AtomEnv -> IForm -> Omega` (`23 §4.4`): interpret
/// the slice's `IForm` constructors by Ken's own connectives (`16 §1.3`).
/// This is the STATED interpretation function D0 owes as data; proving it
/// agrees with quotation (`embedding_adequacy`) is the reserved boundary,
/// not built here.
pub fn denote(env: &GlobalEnv, sig: &FoSliceSignature, f: &IForm) -> Term {
    match f {
        IForm::Bottom => Term::const_(env.bottom_id(), vec![]),
        IForm::Atom(IVar(i)) => Term::app(Term::const_(sig.pred_p, vec![]), Term::Var(*i)),
        IForm::Or(p, q) => {
            let pd = denote(env, sig, p);
            let qd = denote(env, sig, q);
            Term::Trunc(Box::new(Term::app(
                Term::app(Term::indformer(sig.or_id, vec![]), pd),
                qd,
            )))
        }
        IForm::Imp(p, q) => {
            let pd = denote(env, sig, p);
            // `q` is quoted at the SAME object-scope depth as `p` (`imp`
            // introduces no object binder); wrapping it as a Pi codomain
            // introduces a phantom proof-binder slot it must not use, so its
            // free vars shift up by one to skip that slot -- the exact
            // inverse of `quote_fo`'s `shift(codomain, -1, 0)`.
            let qd = shift(&denote(env, sig, q), 1, 0);
            Term::pi(pd, qd)
        }
        IForm::Forall(p) => Term::pi(sig.sort_a.clone(), denote(env, sig, p)),
    }
}

// ─── D1: quote_fo -- total quotation (`23 §4.1`) ────────────────────────────

/// Why `quote_fo` refused an obligation (`23 §4.1` `FoBoundary`). The slice's
/// `quote_iform` exercises a subset of these categories (at minimum
/// `unsupported-term-shape`, `AC-3`); the rest are kept so the enum matches
/// the spec's own vocabulary rather than a slice-shrunk one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoBoundary {
    UnsupportedTermShape,
    UnsupportedAtomTheory,
    NonRigidSort,
    HigherOrderUse,
    DependentProofUse,
    IllScopedOrIllSorted,
}

/// The accepted quotation package (`23 §4.1` `FOProblem`, restricted to one
/// fixed [`FoSliceSignature`] rather than a general `Sigma`).
#[derive(Debug, Clone)]
pub struct FOProblem {
    pub carriers: Carriers,
    pub atoms: AtomEnv,
    pub f: IForm,
}

/// `quote_fo : Obligation -> Accepted FOProblem | Refused FoBoundary`
/// (`23 §4.1`), restricted to `sig`'s one sort and one predicate.
///
/// **Total by construction** (`AC-3`): every `Term` shape reaches either an
/// explicit accepting arm or the catch-all `Refused(UnsupportedTermShape)`
/// -- refusal is a direct return, never a fallthrough that happens to fail
/// some later check.
pub fn quote_fo(
    env: &GlobalEnv,
    sig: &FoSliceSignature,
    phi_closed: &Term,
) -> Result<FOProblem, FoBoundary> {
    let f = quote_iform(env, sig, phi_closed)?;
    Ok(FOProblem {
        carriers: Carriers { sort_a: sig.sort_a.clone() },
        atoms: AtomEnv { pred_p: sig.pred_p },
        f,
    })
}

/// Recursive quotation. Proof-hypothesis binders (`p => q` with the proof
/// var absent from `q`) are erased on the way down via `shift(codomain, -1,
/// 0)` rather than tracked in an explicit scope -- once every crossed
/// proof-hypothesis Pi is erased this way, the remaining term's own de
/// Bruijn indices count ONLY object binders, which is exactly `IVar`'s own
/// convention, so a leaf `Var(i)` maps to `IVar(i)` directly with no
/// separate lookup table.
fn quote_iform(env: &GlobalEnv, sig: &FoSliceSignature, term: &Term) -> Result<IForm, FoBoundary> {
    match term {
        Term::Const { id, .. } if *id == env.bottom_id() => Ok(IForm::Bottom),

        Term::App(f, a) => {
            if matches!(f.as_ref(), Term::Const { id, .. } if *id == sig.pred_p) {
                if let Term::Var(i) = a.as_ref() {
                    return Ok(IForm::Atom(IVar(*i)));
                }
                // `P` applied to something other than an in-scope object
                // Var -- not the accepted atom shape (`23 §4.1`: "every ti
                // is an in-scope object Var of the declared sort").
                return Err(FoBoundary::IllScopedOrIllSorted);
            }
            Err(FoBoundary::HigherOrderUse)
        }

        Term::Trunc(inner) => {
            // `or`: the canonical `Trunc` of this signature's two-ctor sum
            // (`23 §4.1`).
            if let Term::App(f1, q) = inner.as_ref() {
                if let Term::App(f0, p) = f1.as_ref() {
                    if matches!(f0.as_ref(), Term::IndFormer { id, .. } if *id == sig.or_id) {
                        let p_f = quote_iform(env, sig, p)?;
                        let q_f = quote_iform(env, sig, q)?;
                        return Ok(IForm::Or(Box::new(p_f), Box::new(q_f)));
                    }
                }
            }
            Err(FoBoundary::UnsupportedAtomTheory)
        }

        Term::Pi(domain, codomain) => {
            if domain.as_ref() == &sig.sort_a {
                // Object binder: `forall x : A. p`.
                let body = quote_iform(env, sig, codomain)?;
                return Ok(IForm::Forall(Box::new(body)));
            }
            // Proof-hypothesis binder: `p => q`, only if `q` never
            // references the (absent) proof var.
            let ante = quote_iform(env, sig, domain)?;
            if mentions_var0(codomain) {
                return Err(FoBoundary::DependentProofUse);
            }
            let erased = shift(codomain, -1, 0);
            let cons = quote_iform(env, sig, &erased)?;
            Ok(IForm::Imp(Box::new(ante), Box::new(cons)))
        }

        // Every remaining `Term` constructor is out of the slice's accepted
        // shapes by construction (`23 §4.1`'s refusal boundary: `Type`,
        // `Omega`, bare `Var`, `IntLit`, `IndFormer`/`Constructor` outside
        // the recognized `or`, `Elim`, `Lam`, `Sigma`/`Pair`/`Proj*`, `Let`,
        // `Ascript`, `Eq`, `Cast`, `J`, quotient forms, and any `Trunc` that
        // is not the recognized `or` expansion).
        _ => Err(FoBoundary::UnsupportedTermShape),
    }
}

/// Does `term` reference the OUTERMOST bound variable (de Bruijn index `0`
/// at the top call)? Used to enforce "the proof binder absent from `q`"
/// (`23 §4.1`) -- distinct from `prover.rs`'s own `has_free_vars`, which
/// checks for ANY free var at or above a depth, not this specific one.
fn mentions_var0(term: &Term) -> bool {
    fn go(term: &Term, depth: usize) -> bool {
        match term {
            Term::Var(i) => *i == depth,
            Term::Pi(a, b) | Term::Lam(a, b) | Term::Sigma(a, b) | Term::Pair(a, b) => {
                go(a, depth) || go(b, depth + 1)
            }
            Term::App(f, a) => go(f, depth) || go(a, depth),
            Term::Proj1(t) | Term::Proj2(t) | Term::Trunc(t) | Term::TruncProj(t) => go(t, depth),
            Term::Eq(a, t, u) => go(a, depth) || go(t, depth) || go(u, depth),
            _ => false,
        }
    }
    go(term, 0)
}

// ─── D2: embed -- the exact classical Kripke theory (`23 §4.2`) ────────────

/// `K(Sigma)`: the closed classical `World`/`Le`/`Dom_A`/`Force_P` theory's
/// five axioms, conjoined (`23 §4.2`). Retained in FULL, not stubbed
/// (`V3-FO-KRIPKE-SLICE` `D2`): the preorder (reflexive + transitive),
/// possibly-empty `Dom_A` with growth, and `Force_P` domain + persistence.
fn k_sigma() -> Form {
    // `forall w. Le w w`.
    let preorder_reflexive = Form::ForallWorld(Box::new(Form::Access(
        QTerm::Bound(0),
        QTerm::Bound(0),
    )));

    // `forall w v u. (Le w v and Le v u) => Le w u`.
    let preorder_transitive = Form::ForallWorld(Box::new(Form::ForallWorld(Box::new(
        Form::ForallWorld(Box::new(Form::Imp(
            Box::new(Form::And(
                Box::new(Form::Access(QTerm::Bound(2), QTerm::Bound(1))),
                Box::new(Form::Access(QTerm::Bound(1), QTerm::Bound(0))),
            )),
            Box::new(Form::Access(QTerm::Bound(2), QTerm::Bound(0))),
        ))),
    ))));

    // `forall w v x. (Le w v and Dom_A w x) => Dom_A v x`.
    let domain_growth_a = Form::ForallWorld(Box::new(Form::ForallWorld(Box::new(
        Form::ForallObj(Box::new(Form::Imp(
            Box::new(Form::And(
                Box::new(Form::Access(QTerm::Bound(2), QTerm::Bound(1))),
                Box::new(Form::DomainA(QTerm::Bound(2), QTerm::Bound(0))),
            )),
            Box::new(Form::DomainA(QTerm::Bound(1), QTerm::Bound(0))),
        ))),
    ))));

    // `forall w x. Force_P w x => Dom_A w x` (unary P: single conjunct, no
    // `and` needed -- `23 §4.2`'s general n-ary `atom-domain-P` collapses to
    // exactly this for arity 1).
    let atom_domain_p = Form::ForallWorld(Box::new(Form::ForallObj(Box::new(Form::Imp(
        Box::new(Form::ForcingP(QTerm::Bound(1), QTerm::Bound(0))),
        Box::new(Form::DomainA(QTerm::Bound(1), QTerm::Bound(0))),
    )))));

    // `forall w v x. (Le w v and Force_P w x) => Force_P v x`.
    let atom_persistence_p = Form::ForallWorld(Box::new(Form::ForallWorld(Box::new(
        Form::ForallObj(Box::new(Form::Imp(
            Box::new(Form::And(
                Box::new(Form::Access(QTerm::Bound(2), QTerm::Bound(1))),
                Box::new(Form::ForcingP(QTerm::Bound(2), QTerm::Bound(0))),
            )),
            Box::new(Form::ForcingP(QTerm::Bound(1), QTerm::Bound(0))),
        ))),
    ))));

    Form::And(
        Box::new(Form::And(
            Box::new(Form::And(
                Box::new(preorder_reflexive),
                Box::new(preorder_transitive),
            )),
            Box::new(domain_growth_a),
        )),
        Box::new(Form::And(Box::new(atom_domain_p), Box::new(atom_persistence_p))),
    )
}

/// `w |= f`, the forcing translation (`23 §4.2`), given the current world's
/// index and the object-variable environment (source `IVar` index -> its
/// current target index), both counted in the SAME unified de Bruijn space
/// `Form` uses for world and object binders alike.
fn w_forces(world: usize, object_env: &[usize], f: &IForm) -> Form {
    match f {
        IForm::Bottom => Form::Bottom,
        IForm::Atom(IVar(k)) => {
            Form::ForcingP(QTerm::Bound(world), QTerm::Bound(object_env[*k]))
        }
        IForm::Or(p, q) => Form::Or(
            Box::new(w_forces(world, object_env, p)),
            Box::new(w_forces(world, object_env, q)),
        ),
        IForm::Imp(p, q) => {
            // `forall v. Le w v => ((v|=p) => (v|=q))`.
            let shifted_world = world + 1;
            let shifted_env: Vec<usize> = object_env.iter().map(|i| i + 1).collect();
            let v = 0;
            Form::ForallWorld(Box::new(Form::Imp(
                Box::new(Form::Access(QTerm::Bound(shifted_world), QTerm::Bound(v))),
                Box::new(Form::Imp(
                    Box::new(w_forces(v, &shifted_env, p)),
                    Box::new(w_forces(v, &shifted_env, q)),
                )),
            )))
        }
        IForm::Forall(p) => {
            // `forall v. Le w v => forall x:Obj(A). Dom_A v x => (v|=p[x])`.
            let shifted_world = world + 1;
            let shifted_env: Vec<usize> = object_env.iter().map(|i| i + 1).collect();
            let v = 0;
            let v_at_obj = v + 1;
            let shifted_env_2: Vec<usize> = shifted_env.iter().map(|i| i + 1).collect();
            let x = 0;
            let mut new_object_env = vec![x];
            new_object_env.extend(shifted_env_2);
            Form::ForallWorld(Box::new(Form::Imp(
                Box::new(Form::Access(QTerm::Bound(shifted_world), QTerm::Bound(v))),
                Box::new(Form::ForallObj(Box::new(Form::Imp(
                    Box::new(Form::DomainA(QTerm::Bound(v_at_obj), QTerm::Bound(x))),
                    Box::new(w_forces(v_at_obj, &new_object_env, p)),
                )))),
            )))
        }
    }
}

/// `embed(Sigma, f) := K(Sigma) => forall w : World. w |= f` (`23 §4.2`,
/// `:335`). `K(Sigma)` is INSIDE the target (`AC-4`): no frame or forcing
/// premise is emitted outside `embed`'s own `Form`.
pub fn embed(f: &IForm) -> Form {
    let body = Form::ForallWorld(Box::new(w_forces(0, &[], f)));
    Form::Imp(Box::new(k_sigma()), Box::new(body))
}

// ─── D3: check_cert, computable (`23 §4.3`) ─────────────────────────────────

/// `check_cert : Form -> Cert -> Bool` (`23 §4.3`), restricted to the
/// slice's three rules. A Ken-level total function, distinct from the
/// kernel API `check` (`18 §4`).
pub fn check_cert(q: &Form, pi: &Cert) -> bool {
    let root = Sequent { gamma: vec![], delta: vec![q.clone()] };
    check_tree(&root, pi)
}

fn check_tree(expected_conclusion: &Sequent, node: &Cert) -> bool {
    if &node.conclusion != expected_conclusion {
        return false;
    }
    match &node.rule {
        Rule::Init { left, right } => {
            node.children.is_empty()
                && matches!(
                    (node.conclusion.gamma.get(*left), node.conclusion.delta.get(*right)),
                    (Some(g), Some(d)) if g == d
                )
        }
        Rule::ImpRight { right } => {
            let Some(Form::Imp(p, q)) = node.conclusion.delta.get(*right) else {
                return false;
            };
            let [child] = node.children.as_slice() else {
                return false;
            };
            let mut expected_gamma = node.conclusion.gamma.clone();
            expected_gamma.push((**p).clone());
            let mut expected_delta = node.conclusion.delta.clone();
            expected_delta[*right] = (**q).clone();
            check_tree(&Sequent { gamma: expected_gamma, delta: expected_delta }, child)
        }
        Rule::ForallRight { right, eigen } => {
            let Some(quantified) = node.conclusion.delta.get(*right) else {
                return false;
            };
            let body = match quantified {
                Form::ForallWorld(b) | Form::ForallObj(b) => b,
                _ => return false,
            };
            let [child] = node.children.as_slice() else {
                return false;
            };
            // Freshness: the eigenparameter must occur in neither the
            // conclusion sequent nor any earlier-recorded parameter
            // (`23 §4.3`). The slice's linear (non-branching) derivations
            // only ever introduce parameters in increasing order, so
            // freshness reduces to "not already present in the conclusion".
            if sequent_mentions_parameter(&node.conclusion, eigen) {
                return false;
            }
            let instantiated = subst0_form(body, eigen);
            let mut expected_delta = node.conclusion.delta.clone();
            expected_delta[*right] = instantiated;
            check_tree(
                &Sequent { gamma: node.conclusion.gamma.clone(), delta: expected_delta },
                child,
            )
        }
    }
}

fn sequent_mentions_parameter(sequent: &Sequent, target: &QTerm) -> bool {
    sequent.gamma.iter().any(|f| form_mentions_parameter(f, target))
        || sequent.delta.iter().any(|f| form_mentions_parameter(f, target))
}

fn form_mentions_parameter(form: &Form, target: &QTerm) -> bool {
    match form {
        Form::Bottom => false,
        Form::Access(a, b) | Form::DomainA(a, b) | Form::ForcingP(a, b) => a == target || b == target,
        Form::And(p, q) | Form::Or(p, q) | Form::Imp(p, q) => {
            form_mentions_parameter(p, target) || form_mentions_parameter(q, target)
        }
        Form::ForallWorld(b) | Form::ForallObj(b) => form_mentions_parameter(b, target),
    }
}

/// Substitute the outermost bound variable (index `0`, relative to `form`)
/// with `replacement`, shrinking every other bound index above it by one --
/// the same `subst0` convention `ken_kernel::subst` uses for `Term`, applied
/// to `Form`/`QTerm`. `replacement` is always a fresh `Parameter` at every
/// call site in this module, so it is invariant under the depth-tracking
/// shift performed as recursion crosses further binders (exactly like a
/// `Const` under `ken_kernel::subst::shift`).
fn subst0_form(form: &Form, replacement: &QTerm) -> Form {
    subst_form_at(form, 0, replacement)
}

fn subst_form_at(form: &Form, depth: usize, replacement: &QTerm) -> Form {
    match form {
        Form::Bottom => Form::Bottom,
        Form::Access(a, b) => Form::Access(
            subst_qterm_at(a, depth, replacement),
            subst_qterm_at(b, depth, replacement),
        ),
        Form::DomainA(a, b) => Form::DomainA(
            subst_qterm_at(a, depth, replacement),
            subst_qterm_at(b, depth, replacement),
        ),
        Form::ForcingP(a, b) => Form::ForcingP(
            subst_qterm_at(a, depth, replacement),
            subst_qterm_at(b, depth, replacement),
        ),
        Form::And(p, q) => Form::And(
            Box::new(subst_form_at(p, depth, replacement)),
            Box::new(subst_form_at(q, depth, replacement)),
        ),
        Form::Or(p, q) => Form::Or(
            Box::new(subst_form_at(p, depth, replacement)),
            Box::new(subst_form_at(q, depth, replacement)),
        ),
        Form::Imp(p, q) => Form::Imp(
            Box::new(subst_form_at(p, depth, replacement)),
            Box::new(subst_form_at(q, depth, replacement)),
        ),
        Form::ForallWorld(b) => Form::ForallWorld(Box::new(subst_form_at(b, depth + 1, replacement))),
        Form::ForallObj(b) => Form::ForallObj(Box::new(subst_form_at(b, depth + 1, replacement))),
    }
}

fn subst_qterm_at(q: &QTerm, depth: usize, replacement: &QTerm) -> QTerm {
    match q {
        QTerm::Bound(i) if *i == depth => *replacement,
        QTerm::Bound(i) if *i > depth => QTerm::Bound(i - 1),
        QTerm::Bound(i) => QTerm::Bound(*i),
        QTerm::Parameter(p) => QTerm::Parameter(*p),
    }
}

// ─── D4: bounded certificate search over the slice's three rules ───────────

/// Search for a [`Cert`] closing `f`'s `embed`-ded target, using only
/// `init`/`imp-right`/`forall-right`. Deterministic for this restricted,
/// non-branching rule set (no `and-left`/`or-right`/weaken/contract/cut):
/// at every step at most one rule can structurally apply to the sole
/// tracked-open right formula, so this is a decision procedure for the
/// slice's fragment, not a heuristic search -- exactly what lets `AC-1`'s
/// negative control be demonstrated by RUNNING this function rather than by
/// arguing the calculus cannot derive it.
pub fn find_certificate(f: &IForm) -> Option<Cert> {
    let target = embed(f);
    let root = Sequent { gamma: vec![], delta: vec![target] };
    let mut next_param = 0usize;
    search(&root, &mut next_param, 200)
}

fn search(sequent: &Sequent, next_param: &mut usize, fuel: usize) -> Option<Cert> {
    if fuel == 0 {
        return None;
    }
    for (i, g) in sequent.gamma.iter().enumerate() {
        for (j, d) in sequent.delta.iter().enumerate() {
            if g == d {
                return Some(Cert {
                    conclusion: sequent.clone(),
                    rule: Rule::Init { left: i, right: j },
                    children: vec![],
                });
            }
        }
    }
    for (j, d) in sequent.delta.iter().enumerate() {
        if let Form::Imp(p, q) = d {
            let mut gamma = sequent.gamma.clone();
            gamma.push((**p).clone());
            let mut delta = sequent.delta.clone();
            delta[j] = (**q).clone();
            if let Some(child) = search(&Sequent { gamma, delta }, next_param, fuel - 1) {
                return Some(Cert {
                    conclusion: sequent.clone(),
                    rule: Rule::ImpRight { right: j },
                    children: vec![child],
                });
            }
            // A stuck `Imp` cannot be revisited productively by any other
            // slice rule; do not fall through to try `forall-right` on it.
            return None;
        }
    }
    for (j, d) in sequent.delta.iter().enumerate() {
        let body = match d {
            Form::ForallWorld(b) | Form::ForallObj(b) => b,
            _ => continue,
        };
        let param = *next_param;
        *next_param += 1;
        let eigen = QTerm::Parameter(param);
        let instantiated = subst0_form(body, &eigen);
        let mut delta = sequent.delta.clone();
        delta[j] = instantiated;
        if let Some(child) = search(
            &Sequent { gamma: sequent.gamma.clone(), delta },
            next_param,
            fuel - 1,
        ) {
            return Some(Cert {
                conclusion: sequent.clone(),
                rule: Rule::ForallRight { right: j, eigen },
                children: vec![child],
            });
        }
        return None;
    }
    // Delta's sole tracked formula is neither `Imp` nor `Forall`, and no
    // `init` closed it -- e.g. an `Or` on the right, which needs `or-right`,
    // deliberately absent from the slice's rule set. Genuinely stuck.
    None
}

// ─── D4: the two fixed controls, as genuine Ken terms (`23 §4.5`) ─────────

/// The end-to-end positive control: the closed intuitionistic identity
/// `forall x : A. P x => P x`.
pub fn positive_control_term(sig: &FoSliceSignature) -> Term {
    let px = Term::app(Term::const_(sig.pred_p, vec![]), Term::Var(0));
    // The inner `Pi`'s codomain sits one binder deeper than its domain, so
    // the outer `x` reference shifts from `Var(0)` to `Var(1)` there.
    let px_at_codomain = shift(&px, 1, 0);
    Term::pi(sig.sort_a.clone(), Term::pi(px, px_at_codomain))
}

/// The classical-only negative control: `forall x : A. P x or not (P x)`.
/// `not p` is `p => Bottom` (`23 §4.1`); `Bottom` has no free vars so no
/// shift is needed for that codomain, unlike the positive control's own.
pub fn negative_control_term(env: &GlobalEnv, sig: &FoSliceSignature) -> Term {
    let px = Term::app(Term::const_(sig.pred_p, vec![]), Term::Var(0));
    let not_px = Term::pi(px.clone(), Term::const_(env.bottom_id(), vec![]));
    let or_term = Term::Trunc(Box::new(Term::app(
        Term::app(Term::indformer(sig.or_id, vec![]), px),
        not_px,
    )));
    Term::pi(sig.sort_a.clone(), or_term)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ken_kernel::GlobalEnv;

    /// `D1`/`AC-3`: both controls quote; an out-of-slice form is refused by
    /// construction, not by a fallthrough.
    #[test]
    fn quote_fo_accepts_both_controls_and_refuses_outside_the_slice() {
        let mut env = GlobalEnv::new();
        let sig = declare_fo_slice_signature(&mut env);

        let positive = positive_control_term(&sig);
        let negative = negative_control_term(&env, &sig);

        assert!(
            quote_fo(&env, &sig, &positive).is_ok(),
            "the positive control must quote"
        );
        assert!(
            quote_fo(&env, &sig, &negative).is_ok(),
            "the negative control must quote"
        );

        // Out-of-slice form: a bare `Type` is not an accepted proposition
        // shape anywhere in the grammar (`23 §4.1`'s refusal boundary lists
        // `Type` explicitly).
        let out_of_slice = Term::Type(Level::zero());
        assert_eq!(
            quote_fo(&env, &sig, &out_of_slice).err(),
            Some(FoBoundary::UnsupportedTermShape),
            "a bare Type must be refused, by construction"
        );
    }

    /// `D3`/`AC-2`: the positive certificate's `check_cert` COMPUTES to
    /// `true` -- run, not merely typed.
    #[test]
    fn positive_control_certificate_computes_true() {
        let mut env = GlobalEnv::new();
        let sig = declare_fo_slice_signature(&mut env);
        let positive = positive_control_term(&sig);
        let problem = quote_fo(&env, &sig, &positive).expect("positive control must quote");

        let cert = find_certificate(&problem.f)
            .expect("the slice's three rules must find a positive certificate");
        let target = embed(&problem.f);

        assert!(
            check_cert(&target, &cert),
            "the positive certificate must compute to True under check_cert"
        );
    }

    /// `D4`/`AC-1`: the negative control does NOT obtain an accepted
    /// certificate -- demonstrated by RUNNING the search, not by arguing the
    /// calculus cannot derive it.
    #[test]
    fn negative_control_obtains_no_certificate() {
        let mut env = GlobalEnv::new();
        let sig = declare_fo_slice_signature(&mut env);
        let negative = negative_control_term(&env, &sig);
        let problem = quote_fo(&env, &sig, &negative).expect("negative control must quote");

        assert!(
            find_certificate(&problem.f).is_none(),
            "the negative control must NOT obtain a certificate from the \
             slice's init/imp-right/forall-right rule set -- classical \
             reasoning (or-right, or excluded middle) is required and is \
             deliberately absent"
        );
    }

    /// `D2`/`AC-4`: `K(Sigma)` sits INSIDE `embed`'s target -- the top-level
    /// `Form` is `Imp(K(Sigma), forall w. w|=f)`, never a bare `forall w.
    /// w|=f` with the frame theory emitted separately.
    #[test]
    fn embed_places_k_sigma_inside_the_target() {
        let mut env = GlobalEnv::new();
        let sig = declare_fo_slice_signature(&mut env);
        let positive = positive_control_term(&sig);
        let problem = quote_fo(&env, &sig, &positive).expect("positive control must quote");
        let target = embed(&problem.f);
        match target {
            Form::Imp(k_sigma_form, world_quantified) => {
                assert!(
                    matches!(*k_sigma_form, Form::And(_, _)),
                    "K(Sigma) must be the conjoined five-axiom theory, got {k_sigma_form:?}"
                );
                assert!(
                    matches!(*world_quantified, Form::ForallWorld(_)),
                    "the consequent must be `forall w : World. w |= f`, got {world_quantified:?}"
                );
            }
            other => panic!("embed's target must be Imp(K(Sigma), forall w. w|=f), got {other:?}"),
        }
    }
}
